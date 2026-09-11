use recorded_run_driver::{REPORT_LIMIT, execute_bounded, read_bounded};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::Duration,
};

const STAGES: [&str; 4] = ["export", "validate", "studio", "observability"];
const FIELDS: [&str; 8] = [
    "semantic_digest",
    "run_identity",
    "event_records",
    "accounting_records",
    "unsupported_records",
    "evidence",
    "completeness",
    "streams",
];
type Result<T> = std::result::Result<T, String>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Plan {
    provenance: String,
    contract: Vec<Pin>,
    stages: BTreeMap<String, Option<Stage>>,
    comparison: Option<BTreeMap<String, BTreeMap<String, String>>>,
    #[serde(default)]
    existing_bundle: bool,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Pin {
    path: PathBuf,
    sha256: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Stage {
    cwd: PathBuf,
    revision: String,
    executable: Pin,
    args: Vec<String>,
    timeout_seconds: u64,
    #[serde(default)]
    stdout_json: bool,
    #[serde(default)]
    source_pins: Vec<Pin>,
    #[serde(default)]
    source_roots: Vec<String>,
}

fn main() {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 3 {
        eprintln!(
            "usage: recorded-run-driver PLAN.json SOURCE_DIRECTORY_OR_BUNDLE NEW_OUTPUT_DIRECTORY"
        );
        std::process::exit(2);
    }
    let result = run(
        Path::new(&args[0]),
        Path::new(&args[1]),
        Path::new(&args[2]),
    );
    match result {
        Ok(true) => {
            println!("Process comparison passed; see evidence.json for scope and remaining gates.")
        }
        Ok(false) => {
            eprintln!("Pending owner interfaces; see evidence.json.");
            std::process::exit(2);
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

fn digest(path: &Path, bound: u64) -> Result<String> {
    let bytes = read_bounded(path, bound).map_err(|e| e.to_string())?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}
#[cfg(test)]
fn read_json(path: &Path) -> Result<Value> {
    let bytes = read_bounded(path, REPORT_LIMIT).map_err(|e| e.to_string())?;
    serde_json::from_slice(&bytes).map_err(|_| "invalid JSON report or plan".into())
}
fn pending(plan: &Plan) -> Vec<String> {
    let mut missing = Vec::new();
    if plan.contract.is_empty() {
        missing.push("protocol candidate byte pins".into());
    }
    for stage in STAGES {
        if plan.existing_bundle && stage == "export" {
            continue;
        }
        if plan.stages.get(stage).and_then(Option::as_ref).is_none() {
            missing.push(format!("{stage} command, executable digest and revision"));
        }
    }
    if plan.comparison.is_none() {
        missing.push("owner-reviewed comparison JSON pointers".into());
    }
    missing
}
fn check_pin(pin: &Pin) -> Result<()> {
    if !pin.path.is_absolute() || pin.sha256.len() != 64 {
        return Err("pins require absolute paths and SHA-256".into());
    }
    if digest(&pin.path, 512 * 1024 * 1024)? != pin.sha256 {
        return Err("pinned file digest mismatch".into());
    }
    Ok(())
}
fn git(stage: &Stage, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(&stage.cwd)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map_err(|_| "git unavailable")?;
    if !out.status.success() {
        return Err("repository inspection failed".into());
    }
    String::from_utf8(out.stdout).map_err(|_| "invalid repository metadata".into())
}
fn check_stage(stage: &Stage) -> Result<()> {
    check_pin(&stage.executable)?;
    if !stage.cwd.is_absolute() || !(1..=600).contains(&stage.timeout_seconds) {
        return Err("stage needs absolute cwd and timeout 1..600 seconds".into());
    }
    if git(stage, &["rev-parse", "HEAD"])?.trim() != stage.revision {
        return Err("stage repository revision mismatch".into());
    }
    for pin in &stage.source_pins {
        check_pin(pin)?;
    }
    // Candidate work may run before commit; every changed runtime input must be pinned.
    let changed = git(stage, &["diff", "HEAD", "--name-only", "-z"])?
        + &git(stage, &["ls-files", "--others", "--exclude-standard", "-z"])?;
    for name in changed.split('\0').filter(|s| !s.is_empty()) {
        if !stage.source_roots.is_empty()
            && !stage
                .source_roots
                .iter()
                .any(|root| name == root || name.starts_with(&format!("{root}/")))
        {
            continue;
        }
        if !stage
            .source_pins
            .iter()
            .any(|p| p.path == stage.cwd.join(name))
        {
            return Err("stage repository has unpinned changes".into());
        }
    }
    Ok(())
}
fn execute(stage: &Stage, source: &Path, bundle: &Path, summary: &Path) -> Result<()> {
    check_stage(stage)?;
    let args: Vec<_> = stage
        .args
        .iter()
        .map(|arg| {
            arg.replace("{source}", &source.to_string_lossy())
                .replace("{bundle}", &bundle.to_string_lossy())
                .replace("{summary}", &summary.to_string_lossy())
        })
        .collect();
    // Only owner-confirmed structured JSON stdout may be captured.
    let stdout = if stage.stdout_json {
        Stdio::from(
            fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(summary)
                .map_err(|_| "summary creation failed")?,
        )
    } else {
        Stdio::null()
    };
    execute_bounded(
        Command::new(&stage.executable.path)
            .args(args)
            .current_dir(&stage.cwd)
            .stdin(Stdio::null())
            .stdout(stdout)
            .stderr(Stdio::null()),
        summary,
        Duration::from_secs(stage.timeout_seconds),
    )
    .map_err(|e| e.to_string())
}
fn compare(plan: &Plan, reports: &BTreeMap<String, Value>) -> Result<()> {
    let mapping = plan
        .comparison
        .as_ref()
        .ok_or("comparison mapping missing")?;
    let mut fields = FIELDS.to_vec();
    if let Some(studio) = mapping.get("studio") {
        fields.extend(
            studio
                .keys()
                .map(String::as_str)
                .filter(|f| !FIELDS.contains(f)),
        );
    }
    for field in fields {
        let mut values = Vec::new();
        for consumer in ["studio", "observability", "validate"] {
            if consumer == "validate" && !mapping.contains_key("validate") {
                continue;
            }
            let pointer = mapping
                .get(consumer)
                .and_then(|m| m.get(field))
                .ok_or("comparison mapping incomplete")?;
            if !pointer.starts_with('/') {
                return Err("comparison needs non-root JSON pointer".into());
            }
            let value = reports
                .get(consumer)
                .and_then(|r| r.pointer(pointer))
                .filter(|v| !v.is_null())
                .ok_or("comparison value missing or null")?;
            values.push(value);
        }
        if values.iter().any(|v| *v != values[0]) {
            return Err(format!("consumer disagreement: {field}"));
        }
    }
    Ok(())
}
fn write_evidence(out: &Path, evidence: &Value) -> Result<()> {
    fs::write(
        out.join("evidence.json"),
        serde_json::to_vec_pretty(evidence).unwrap(),
    )
    .map_err(|_| "evidence write failed".into())
}
fn run(plan_path: &Path, source: &Path, out: &Path) -> Result<bool> {
    let plan_bytes = read_bounded(plan_path, REPORT_LIMIT).map_err(|e| e.to_string())?;
    let plan: Plan = serde_json::from_slice(&plan_bytes).map_err(|_| "invalid integration plan")?;
    if let Ok(source) = fs::canonicalize(source) {
        let parent = out
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        let parent = fs::canonicalize(parent).map_err(|_| "output parent unavailable")?;
        if parent.starts_with(source) {
            return Err("output must be outside source directory".into());
        }
    }
    fs::create_dir(out).map_err(|_| "output must be a new directory with existing parent")?;
    let out = fs::canonicalize(out).map_err(|_| "output resolution failed")?;
    let missing = pending(&plan);
    let mut evidence = json!({"status":"pending", "provenance":plan.provenance,
        "plan_sha256":format!("{:x}", Sha256::digest(&plan_bytes)), "pending":missing,
        "stages":[], "scope":"local process integration only; no admission, browser, tracking persistence or deployment claim"});
    write_evidence(&out, &evidence)?;
    if !missing.is_empty() {
        return Ok(false);
    }
    let result = run_ready(&plan, source, &out, &mut evidence);
    evidence["status"] = json!(if result.is_ok() {
        "process_comparison_passed"
    } else {
        "failed"
    });
    if let Err(ref error) = result {
        evidence["failure"] = json!(error);
        if let Some(row) = evidence["stages"].as_array_mut().unwrap().last_mut()
            && row["status"] == "running"
        {
            row["status"] = json!("failed");
        }
    }
    write_evidence(&out, &evidence)?;
    result.map(|_| true)
}
fn run_ready(plan: &Plan, source: &Path, out: &Path, evidence: &mut Value) -> Result<()> {
    for pin in &plan.contract {
        check_pin(pin)?;
    }
    for stage in STAGES {
        if plan.existing_bundle && stage == "export" {
            continue;
        }
        check_stage(plan.stages[stage].as_ref().unwrap())?;
    }
    let source = fs::canonicalize(source).map_err(|_| "source unavailable")?;
    if (!plan.existing_bundle && !source.is_dir()) || out.starts_with(&source) {
        return Err("output must be outside source directory".into());
    }
    let bundle = out.join("recording.zip");
    let mut bundle_digest = None;
    if plan.existing_bundle {
        let expected = digest(&source, 32 * 1024 * 1024)?;
        fs::write(
            &bundle,
            read_bounded(&source, 32 * 1024 * 1024).map_err(|e| e.to_string())?,
        )
        .map_err(|_| "bundle copy failed")?;
        if digest(&bundle, 32 * 1024 * 1024)? != expected {
            return Err("bundle changed during copy".into());
        }
        evidence["bundle_sha256"] = json!(expected);
        evidence["input_mode"] = json!("existing_export; exporter not executed");
        bundle_digest = Some(expected);
    }
    let mut reports = BTreeMap::new();
    for name in STAGES {
        if plan.existing_bundle && name == "export" {
            continue;
        }
        if let Some(ref expected) = bundle_digest
            && &digest(&bundle, 32 * 1024 * 1024)? != expected
        {
            return Err("bundle changed before stage".into());
        }
        let summary = out.join(format!("{name}.json"));
        let stage = plan.stages[name].as_ref().unwrap();
        evidence["stages"].as_array_mut().unwrap().push(json!({"name":name,"status":"running",
            "revision":stage.revision,"executable_sha256":stage.executable.sha256,"args":stage.args}));
        write_evidence(out, evidence)?;
        execute(stage, &source, &bundle, &summary)?;
        for pin in &plan.contract {
            check_pin(pin)?;
        }
        check_stage(stage)?;
        let current = digest(&bundle, 32 * 1024 * 1024)?;
        if let Some(ref expected) = bundle_digest {
            if &current != expected {
                return Err("bundle changed during stage".into());
            }
        } else {
            bundle_digest = Some(current.clone());
            evidence["bundle_sha256"] = json!(current);
        }
        let report_bytes = read_bounded(&summary, REPORT_LIMIT).map_err(|e| e.to_string())?;
        let report: Value =
            serde_json::from_slice(&report_bytes).map_err(|_| "invalid JSON report")?;
        if name != "export" && report["valid"] != true {
            return Err("stage did not report valid true".into());
        }
        reports.insert(name.to_owned(), report);
        let row = evidence["stages"]
            .as_array_mut()
            .unwrap()
            .last_mut()
            .unwrap();
        row["status"] = json!("process_passed");
        row["report_sha256"] = json!(format!("{:x}", Sha256::digest(&report_bytes)));
        write_evidence(out, evidence)?;
    }
    for pin in &plan.contract {
        check_pin(pin)?;
    }
    compare(plan, &reports)
}

#[cfg(test)]
mod tests;
