//! Process-only bridge to owner import APIs; no bundle parser or validator here.
use recorded_run_driver::{REPORT_LIMIT, execute_bounded, read_bounded};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::Duration,
};
const INVENTORY: &str = "580c1cf3be4bb3e4eb37b9acd9166808b7386b0eb84286cc0798a0d88e35bb35";
const SCHEMA: &str = "a6c32127290f4d5e670d8863f97a74a7b8e3e411e735d81394b51fe1578b4eb6";
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Minimal Node bridge invokes the owners' public importer functions. Vite is
// needed for Studio's TypeScript and ?raw imports; it never listens on a port.
const BRIDGE: &str = r#"
import {readFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
import {resolve} from 'node:path';
const [role,protocol,studio,obs,out,provided]=process.argv.slice(1);
const artifact=resolve(protocol,'artifacts/recorded-run-bundle-v1-candidate3');
const cases=provided?[{path:'provided.zip',valid:true}]:JSON.parse(readFileSync(resolve(artifact,'conformance.json'))).cases;
const load=p=>import(pathToFileURL(p).href);
const record=r=>({key:r.key??`${r.source.stream}:${r.source.record_ordinal}:${r.source.subrecord_ordinal}`,
 stream:r.source?.stream??r.stream,ordinal:r.source?.record_ordinal??r.ordinal,
 sequence:r.source?.stream_sequence??r.sequence??null,timestamp:r.time?.unix_ns??r.timestamp??null,
 identities:r.identities,evidence:r.evidence,payload:r.payload});
const provenance=(producer,adapter,versions,format_version,contract_schema_sha256,fixture_provenance)=>
 ({producer,adapter,versions,format_version,contract_schema_sha256,fixture_provenance});
const normalize=(digest,identity,events,accounting,evidence,identities,omissions,origin,completeness)=>({digest,identity,identities,omissions,
 provenance:origin,completeness,events:events.map(record),
 original_events:events.every(r=>r.source)?events:undefined,
 original_accounting:accounting.every(r=>r.source)?accounting:undefined,
 counts:{event_records:events.length,accounting_records:accounting.length,
  unsupported_records:events.filter(r=>r.payload.kind==='opaque').length,
  unknown_action_outcomes:events.filter(r=>r.payload.kind==='action_outcome'&&r.evidence.action==='unknown').length,
  settled_action_outcomes:events.filter(r=>r.payload.kind==='action_outcome'&&r.evidence.action==='settled').length},evidence,
 observations:events.filter(r=>r.payload.kind==='observation_summary').map(r=>r.payload),
 accounting:accounting.map(record)});
let server, consume;
try {
 if(role==='protocol') {
  const {validateBundle}=await load(resolve(protocol,'tools/recorded-run/validate.mjs'));
  consume=async(path)=>{const b=validateBundle(readFileSync(path));return normalize(
   b.summary.semantic_digest,b.manifest.recording.identity.namespace+':'+b.manifest.recording.identity.value,
   b.events,b.accounting,b.manifest.evidence,b.manifest.recording.identities,b.omissions,
   provenance(b.manifest.producer,b.manifest.adapter,b.manifest.versions,b.manifest.format_version,
    b.manifest.contract_schema_sha256,b.omissions.fixture_provenance),b.manifest.completeness);};
 } else if(role==='studio') {
  const {createServer}=await load(resolve(studio,'node_modules/vite/dist/node/index.js'));
  server=await createServer({root:studio,configFile:false,logLevel:'silent',cacheDir:resolve(out,'vite-cache'),
   resolve:{alias:{'@studio/document':resolve(studio,'packages/document/src/index.ts'),
    '@studio/contracts':resolve(studio,'packages/contracts/src/index.ts')}},
   server:{middlewareMode:true,hmr:false,watch:null},appType:'custom'});
  const {importRecording}=await server.ssrLoadModule('/packages/recording/src/import.ts');
  consume=async(path)=>{const b=readFileSync(path),r=await importRecording(b.buffer.slice(b.byteOffset,b.byteOffset+b.byteLength));
   const {streams,...completeness}=r.completeness;
   const p=r.provenance;
   return normalize(r.digest,r.bundleIdentity,r.records,r.accounting,r.evidence,r.identities,r.omissions,
    provenance(p.producer,p.adapter,p.versions,p.format_version,p.contract_schema_sha256,p.fixture_provenance),completeness);};
 } else {
  const {importBundle}=await load(resolve(obs,'deploy/recorded-run/import.mjs'));
  const {ImportStore}=await load(resolve(obs,'deploy/recorded-run/store.mjs'));
  consume=async(path,index)=>{
   const db=resolve(out,'observability-'+index+'.sqlite3');
   const first=importBundle(path,db),retry=importBundle(path,db);
   const store=new ImportStore(db);
   try {const revisions=store.inspect(first.runId);
    const r=revisions.find(r=>r.semanticDigest===first.semanticDigest);
    if(!r)throw Error('stored_revision_missing');
    return {...normalize(r.semanticDigest,r.identity.namespace+':'+r.identity.value,r.records,r.accounting,r.evidence,r.identities,r.omissions,
     provenance(r.producer,r.sourceAdapter,r.versions,r.wireVersion,r.contractSchemaDigest,r.omissions.fixture_provenance),r.completeness),
     persistence:{first:first.disposition,retry:retry.disposition,revisions:revisions.length,delivery:'local_only'}};
   }finally{store.close();}
  };
 }
 const results=[];
 for(const [index,c]of cases.entries()) {
  try {results.push({path:c.path,expected:c.valid,accepted:true,summary:await consume(provided||resolve(artifact,c.path),index)});}
  catch(e){results.push({path:c.path,expected:c.valid,accepted:false,
   code:/^[a-zA-Z0-9_]{1,100}$/.test(e.code??e.message)?(e.code??e.message):'consumer_rejected'});}
 }
 console.log(JSON.stringify({role,results}));
}catch(e){console.log(JSON.stringify({role,setup_error:/^[a-zA-Z0-9_]{1,100}$/.test(e.code??e.message)?(e.code??e.message):'consumer_setup_failed'}));process.exitCode=1;}
finally{if(server)await server.close();}
"#;

fn hash(path: &Path) -> Result<String> {
    Ok(format!(
        "{:x}",
        Sha256::digest(read_bounded(path, 512 * 1024 * 1024)?)
    ))
}
fn snapshot(root: &Path, paths: &[&str]) -> Result<Value> {
    fn visit(root: &Path, path: &Path, entries: &mut serde_json::Map<String, Value>) -> Result<()> {
        if path.is_dir() {
            for item in fs::read_dir(path)? {
                visit(root, &item?.path(), entries)?;
            }
        } else if path.is_file() {
            entries.insert(
                path.strip_prefix(root)?.to_string_lossy().into_owned(),
                json!(hash(path)?),
            );
        }
        Ok(())
    }
    let mut entries = serde_json::Map::new();
    for p in paths {
        visit(root, &root.join(p), &mut entries)?;
    }
    let revision = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root)
        .output()?;
    Ok(json!({"revision":String::from_utf8(revision.stdout)?.trim(),"files":entries}))
}
fn verify_inventory(artifact: &Path) -> Result<Value> {
    if hash(&artifact.join("SHA256SUMS"))? != INVENTORY
        || hash(&artifact.join("schema.json"))? != SCHEMA
    {
        return Err("published candidate pin mismatch".into());
    }
    let manifest: Value = serde_json::from_slice(&read_bounded(
        &artifact.join("manifest.json"),
        REPORT_LIMIT,
    )?)?;
    let schema: Value =
        serde_json::from_slice(&read_bounded(&artifact.join("schema.json"), REPORT_LIMIT)?)?;
    if manifest["version"] != "1.0.0-candidate.3"
        || schema.pointer("/$defs/manifest/properties/format_version/const")
            != Some(&json!("1.0.0-candidate.3"))
    {
        return Err("candidate manifest/schema version mismatch".into());
    }
    let mut pins = serde_json::Map::new();
    pins.insert("SHA256SUMS".into(), json!(INVENTORY));
    for line in
        std::str::from_utf8(&read_bounded(&artifact.join("SHA256SUMS"), REPORT_LIMIT)?)?.lines()
    {
        let (expected, name) = line.split_once("  ").ok_or("malformed inventory")?;
        if name.starts_with('/') || name.split('/').any(|p| p == ".." || p.is_empty()) {
            return Err("unsafe inventory path".into());
        }
        if hash(&artifact.join(name))? != expected {
            return Err("inventory entry mismatch".into());
        }
        pins.insert(name.into(), json!(expected));
    }
    Ok(json!(pins))
}
fn launch(role: &str, roots: &[PathBuf], out: &Path, provided: Option<&Path>) -> Result<Value> {
    let output = out.join(format!("{role}.json"));
    let file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&output)?;
    execute_bounded(
        Command::new("node")
            .args(["--input-type=module", "-e", BRIDGE, role])
            .args(roots)
            .arg(out)
            .arg(provided.unwrap_or(Path::new("")))
            .stdin(Stdio::null())
            .stdout(file)
            .stderr(Stdio::null()),
        &output,
        Duration::from_secs(120),
    )?;
    Ok(serde_json::from_slice(&read_bounded(
        &output,
        REPORT_LIMIT,
    )?)?)
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 4 && args.len() != 5 {
        return Err(
            "usage: candidate-matrix PROTOCOL_ROOT STUDIO_ROOT OBS_ROOT NEW_OUTPUT [BUNDLE]".into(),
        );
    }
    let roots: Vec<_> = args[..3]
        .iter()
        .map(fs::canonicalize)
        .collect::<std::io::Result<_>>()?;
    let artifact = roots[0].join("artifacts/recorded-run-bundle-v1-candidate3");
    let pins = verify_inventory(&artifact)?;
    fs::create_dir(&args[3])?;
    let out = fs::canonicalize(&args[3])?;
    let provided = if args.len() == 5 {
        let input = fs::canonicalize(&args[4])?;
        if !input.is_file() || fs::metadata(&input)?.len() > 16 * 1024 * 1024 {
            return Err("bundle file/size rejected".into());
        }
        let expected = hash(&input)?;
        let copy = out.join("provided.zip");
        fs::write(&copy, read_bounded(&input, 16 * 1024 * 1024)?)?;
        if hash(&copy)? != expected {
            return Err("bundle changed during copy".into());
        }
        Some((copy, expected))
    } else {
        None
    };
    let paths: [&[&str]; 3] = [
        &[
            "tools/recorded-run",
            "artifacts/recorded-run-bundle-v1-candidate3",
        ],
        &[
            "packages/recording",
            "packages/document",
            "packages/contracts",
            "contracts/recorded-run-candidate",
            "package.json",
            "package-lock.json",
        ],
        &[
            "deploy/recorded-run",
            "contract/recorded-run-bundle-v1-candidate3",
        ],
    ];
    let before: Vec<_> = roots
        .iter()
        .zip(paths)
        .map(|(r, p)| snapshot(r, p))
        .collect::<Result<_>>()?;
    let mut reports = Vec::new();
    for role in ["protocol", "studio", "observability"] {
        reports.push(launch(
            role,
            &roots,
            &out,
            provided.as_ref().map(|(p, _)| p.as_path()),
        )?);
    }
    let after: Vec<_> = roots
        .iter()
        .zip(paths)
        .map(|(r, p)| snapshot(r, p))
        .collect::<Result<_>>()?;
    let mut findings = Vec::new();
    if let Some((file, expected)) = &provided
        && &hash(file)? != expected
    {
        findings.push("provided_bundle_changed".into());
    }
    if before != after {
        findings.push("owner_source_changed_during_run".to_owned());
    }
    if verify_inventory(&artifact)? != pins {
        return Err("contract changed during stages".into());
    }
    for r in &reports {
        if r.get("setup_error").is_some() {
            findings.push(format!("{} setup failed", r["role"]));
            continue;
        }
        for case in r["results"].as_array().ok_or("missing results")? {
            if case["accepted"] != case["expected"] {
                findings.push(format!(
                    "{} {} acceptance mismatch",
                    r["role"], case["path"]
                ));
            }
        }
    }
    if let Some(cases) = reports[0]["results"].as_array() {
        for (index, case) in cases
            .iter()
            .enumerate()
            .filter(|(_, c)| c["expected"] == true)
        {
            for consumer in reports.iter().skip(1) {
                if let Some(summary) = consumer.pointer(&format!("/results/{index}/summary")) {
                    for key in [
                        "digest",
                        "identity",
                        "identities",
                        "counts",
                        "evidence",
                        "accounting",
                        "omissions",
                        "observations",
                        "events",
                        "provenance",
                        "completeness",
                    ] {
                        if summary[key] != case["summary"][key] {
                            findings.push(format!(
                                "{} {} {key} disagreement",
                                consumer["role"], case["path"]
                            ));
                        }
                    }
                    if consumer["role"] == "observability" {
                        for key in ["original_events", "original_accounting"] {
                            if summary[key] != case["summary"][key] {
                                findings.push(format!(
                                    "observability {} {key} persistence disagreement",
                                    case["path"]
                                ));
                            }
                        }
                    }
                    if consumer["role"] == "observability"
                        && (summary["persistence"]["retry"] != "duplicate"
                            || summary["persistence"]["revisions"] != 1)
                    {
                        findings.push(format!(
                            "observability {} duplicate persistence failed",
                            case["path"]
                        ));
                    }
                }
            }
        }
    }
    let passed = if provided.is_some() {
        "provided_bundle_matrix_passed"
    } else {
        "synthetic_matrix_passed"
    };
    let scope = if provided.is_some() {
        "supplied existing export; actual owner imports and local SQLite; no source re-export, browser or deployment"
    } else {
        "synthetic protocol vectors; actual owner import code; local SQLite only; no browser or Train proof"
    };
    let evidence = json!({"status":if findings.is_empty(){passed}else{"failed"}, "scope":scope,
      "provided_bundle_sha256":provided.as_ref().map(|(_,digest)|digest),
      "wire_version":"1.0.0-candidate.3","schema_sha256":SCHEMA,"artifact_inventory_sha256":INVENTORY,
      "artifact_pins":pins,"owner_sources_before":before,"owner_sources_after":after,
      "driver_bridge_sha256":format!("{:x}",Sha256::digest(BRIDGE)),"findings":findings,"reports":reports});
    fs::write(
        out.join("matrix.json"),
        serde_json::to_vec_pretty(&evidence)?,
    )?;
    println!(
        "{} findings; evidence {}",
        findings.len(),
        out.join("matrix.json").display()
    );
    if !findings.is_empty() {
        std::process::exit(1);
    }
    Ok(())
}
