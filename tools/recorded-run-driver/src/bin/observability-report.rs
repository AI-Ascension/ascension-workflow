//! Local-only adapter around the owner's durable importer and inspection APIs.
use std::process::{Command, Stdio};
const BRIDGE: &str = r#"
import {pathToFileURL} from 'node:url';
import {resolve} from 'node:path';
const [root,bundle,db]=process.argv.slice(1);
try {
 const {importBundle}=await import(pathToFileURL(resolve(root,'deploy/recorded-run/import.mjs')));
 const {ImportStore}=await import(pathToFileURL(resolve(root,'deploy/recorded-run/store.mjs')));
 const first=importBundle(bundle,db), retry=importBundle(bundle,db);
 const store=new ImportStore(db);
 try {
  const revisions=store.inspect(first.runId),r=revisions.find(r=>r.semanticDigest===first.semanticDigest);
  if(!r||retry.disposition!=='duplicate'||revisions.length!==1)throw Error('revision_or_retry_mismatch');
  console.log(JSON.stringify({valid:true,semantic_digest:r.semanticDigest,recording_identity:r.identity,
   evidence:r.evidence,completeness:r.completeness,streams:r.omissions.streams,
   event_records:r.counts.event_records,accounting_records:r.counts.accounting_records,
   unsupported_records:r.counts.unsupported_records,
   accounting:r.accounting,local_tracking:{first:first.disposition,retry:retry.disposition,revisions:revisions.length,delivery:'local_only'}}));
 }finally{store.close();}
}catch{console.error('observability_report_failed');process.exitCode=1;}
"#;
fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 3 {
        eprintln!("usage: observability-report OBS_ROOT BUNDLE NEW_DATABASE");
        std::process::exit(2);
    }
    let status = Command::new("node")
        .args(["--input-type=module", "-e", BRIDGE])
        .args(args)
        .stdin(Stdio::null())
        .status()?;
    std::process::exit(status.code().unwrap_or(1));
}
