//! Synthetic source probes against pinned exporter and canonical owner APIs.
use recorded_run_driver::{REPORT_LIMIT, execute_bounded, read_bounded};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
    time::Duration,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const ORACLE: &str = r#"
import {readFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
const [validator,bundle]=process.argv.slice(2);
const {validateBundle}=await import(pathToFileURL(validator));
console.log(JSON.stringify(validateBundle(readFileSync(bundle))));
"#;
fn hash(path: &Path) -> Result<String> {
    Ok(format!(
        "{:x}",
        Sha256::digest(read_bounded(path, 512 * 1024 * 1024)?)
    ))
}
fn text(value: &Value) -> Result<&str> {
    value.as_str().ok_or("missing string".into())
}
fn pins(plan: &Value) -> Result<()> {
    let mut all: Vec<&Value> = plan["contract"]
        .as_array()
        .ok_or("missing contract")?
        .iter()
        .collect();
    for name in ["export", "validate"] {
        let s = &plan["stages"][name];
        all.push(&s["executable"]);
        all.extend(s["source_pins"].as_array().ok_or("missing runtime pins")?);
        let git = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(text(&s["cwd"])?)
            .output()?;
        if !git.status.success() || String::from_utf8(git.stdout)?.trim() != text(&s["revision"])? {
            return Err("owner revision mismatch".into());
        }
    }
    for pin in all {
        if hash(Path::new(text(&pin["path"])?))? != text(&pin["sha256"])? {
            return Err("source probe pin mismatch".into());
        }
    }
    Ok(())
}
fn snapshot(root: &Path) -> Result<Value> {
    let mut entries = BTreeMap::new();
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        let meta = fs::symlink_metadata(&path)?;
        let value = if meta.file_type().is_symlink() {
            json!({"link":fs::read_link(&path)?})
        } else {
            json!({"sha256":hash(&path)?,"bytes":meta.len(),
                "modified_ns":meta.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_nanos().to_string()})
        };
        entries.insert(
            path.file_name()
                .ok_or("missing name")?
                .to_string_lossy()
                .into_owned(),
            value,
        );
    }
    Ok(json!(entries))
}
fn capture(command: &mut Command, output: &Path) -> Result<()> {
    execute_bounded(
        command.stdin(Stdio::null()).stderr(Stdio::null()).stdout(
            fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(output)?,
        ),
        output,
        Duration::from_secs(120),
    )?;
    Ok(())
}
fn stream<'a>(b: &'a Value, name: &str) -> Result<&'a Value> {
    b["omissions"]["streams"]
        .as_array()
        .ok_or("missing streams")?
        .iter()
        .find(|s| s["stream"] == name)
        .ok_or("missing stream".into())
}
fn require(ok: bool, message: &'static str) -> Result<()> {
    if ok { Ok(()) } else { Err(message.into()) }
}
fn probe(plan: &Value, original: &Path, root: &Path, name: &str) -> Result<Value> {
    let dir = root.join(name);
    fs::create_dir(&dir)?;
    // The producer derives recording identity from the source directory basename.
    // Preserve that owner-defined input when copying a fixture into a probe.
    let source = dir.join(original.file_name().ok_or("missing source basename")?);
    fs::create_dir(&source)?;
    for entry in fs::read_dir(original)? {
        let p = entry?.path();
        fs::write(
            source.join(p.file_name().ok_or("missing name")?),
            read_bounded(&p, REPORT_LIMIT)?,
        )?;
    }
    let accounting = source.join("provider-accounting.jsonl");
    match name {
        "missing-accounting" => fs::remove_file(&accounting)?,
        "partial-tail" => fs::OpenOptions::new()
            .append(true)
            .open(source.join("trajectory.jsonl"))?
            .write_all(b"{\"event\":")?,
        "unsupported-status" => {
            let bytes = read_bounded(&accounting, REPORT_LIMIT)?;
            let mut rows: Vec<Value> = std::str::from_utf8(&bytes)?
                .lines()
                .map(serde_json::from_str)
                .collect::<std::result::Result<_, _>>()?;
            rows[0]["execution_status"] = json!("synthetic_unsupported_status");
            let mut file = fs::File::create(&accounting)?;
            for row in rows {
                writeln!(file, "{}", serde_json::to_string(&row)?)?;
            }
        }
        "symlink-child" => {
            fs::remove_file(source.join("mcp.jsonl"))?;
            std::os::unix::fs::symlink(original.join("mcp.jsonl"), source.join("mcp.jsonl"))?;
        }
        _ => {}
    }
    let before = snapshot(&source)?;
    let bundle = dir.join("recording.zip");
    let output = dir.join("export.json");
    let s = &plan["stages"]["export"];
    let export = capture(
        Command::new(text(&s["executable"]["path"])?)
            .current_dir(text(&s["cwd"])?)
            .arg("export")
            .arg(&source)
            .arg("--output")
            .arg(&bundle),
        &output,
    );
    require(snapshot(&source)? == before, "source changed during export")?;
    if name == "symlink-child" {
        require(
            export
                .as_ref()
                .err()
                .is_some_and(|e| e.to_string() == "stage exit Some(2)")
                && !bundle.exists(),
            "symlink source was not rejected without output",
        )?;
        return Ok(
            json!({"case":name,"result":"passed","export":"rejected_exit_2","source_unchanged":true,"source_before":before}),
        );
    }
    export?;
    let report = dir.join("canonical.json");
    let validator = &plan["stages"]["validate"];
    capture(
        Command::new(text(&validator["executable"]["path"])?)
            .args(["--input-type=module", "-e", ORACLE, "source-probe"])
            .arg(Path::new(text(&validator["cwd"])?).join("tools/recorded-run/validate.mjs"))
            .arg(&bundle),
        &report,
    )?;
    let bytes = read_bounded(&report, REPORT_LIMIT)?;
    let b: Value = serde_json::from_slice(&bytes)?;
    require(
        !String::from_utf8_lossy(&bytes).contains("SYNTHETIC_PRIVATE_"),
        "private source sentinel survived",
    )?;
    match name {
        "baseline" => {
            require(
                b["accounting"].as_array().is_some_and(|a| a.len() == 2),
                "accounting count",
            )?;
            let reported = &b["accounting"][0]["payload"]["value"]["usage"];
            let unknown = &b["accounting"][1]["payload"]["value"]["usage"];
            require(
                reported["input_tokens"]["value"] == "3"
                    && reported["output_tokens"]["value"] == "4",
                "reported usage changed",
            )?;
            require(
                unknown["input_tokens"]["value"].is_null()
                    && unknown["input_tokens"]["value_status"] == "unknown"
                    && unknown["output_tokens"]["value"].is_null()
                    && unknown.get("cached_input_tokens").is_none(),
                "unknown/absent usage fabricated",
            )?;
            require(
                stream(&b, "trajectory")?["field_omissions"]
                    .as_array()
                    .is_some_and(|a| !a.is_empty()),
                "missing trajectory field omissions",
            )?;
        }
        "missing-accounting" => require(
            b["accounting"].as_array().is_some_and(Vec::is_empty)
                && stream(&b, "provider-accounting")?["input_records"].is_null(),
            "missing accounting not preserved",
        )?,
        "partial-tail" => require(
            stream(&b, "trajectory")?["state"] == "interrupted"
                && stream(&b, "trajectory")?["rejected_rows"] == 1,
            "partial tail not reconciled",
        )?,
        "unsupported-status" => require(
            stream(&b, "provider-accounting")?["unsupported_rows"] == 1
                && b["accounting"].as_array().is_some_and(|a| a.len() == 1),
            "unsupported accounting fabricated",
        )?,
        _ => {}
    }
    Ok(
        json!({"case":name,"result":"passed","bundle_sha256":hash(&bundle)?,"canonical_report_sha256":hash(&report)?,
        "semantic_digest":b["summary"]["semantic_digest"],"summary":b["summary"],"source_unchanged":true,"source_before":before}),
    )
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 3 {
        return Err("usage: source-probes FULL_PLAN SOURCE_FIXTURE NEW_OUTPUT".into());
    }
    let plan_bytes = read_bounded(Path::new(&args[0]), REPORT_LIMIT)?;
    let plan: Value = serde_json::from_slice(&plan_bytes)?;
    let source = fs::canonicalize(&args[1])?;
    let parent = Path::new(&args[2])
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    require(
        !fs::canonicalize(parent)?.starts_with(&source),
        "output must be outside source",
    )?;
    fs::create_dir(&args[2])?;
    let out = fs::canonicalize(&args[2])?;
    require(!out.starts_with(&source), "output must be outside source")?;
    pins(&plan)?;
    let original = snapshot(&source)?;
    let mut results = Vec::new();
    let mut failure = None;
    for name in [
        "baseline",
        "repeat",
        "missing-accounting",
        "partial-tail",
        "unsupported-status",
        "symlink-child",
    ] {
        match probe(&plan, &source, &out, name) {
            Ok(result) => results.push(result),
            Err(error) => {
                failure = Some(format!("{name}: {error}"));
                break;
            }
        }
    }
    if failure.is_none() && results[0]["bundle_sha256"] != results[1]["bundle_sha256"] {
        failure = Some("repeated export bytes differ".into());
    }
    if snapshot(&source)? != original {
        failure = Some("original fixture changed".into());
    }
    if let Err(e) = pins(&plan) {
        failure = Some(e.to_string());
    }
    let evidence = json!({"status":if failure.is_none(){"passed"}else{"failed"},"scope":"synthetic source mutations; pinned exporter and canonical validator only",
        "plan_sha256":format!("{:x}",Sha256::digest(plan_bytes)),"source_original":original,"failure":failure,"results":results});
    fs::write(
        out.join("probes.json"),
        serde_json::to_vec_pretty(&evidence)?,
    )?;
    require(failure.is_none(), "source probes failed; see probes.json")?;
    println!(
        "6 source probes passed; {}",
        out.join("probes.json").display()
    );
    Ok(())
}
