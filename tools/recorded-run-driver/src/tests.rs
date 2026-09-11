use super::*;
use std::sync::atomic::{AtomicU64, Ordering};
static NEXT: AtomicU64 = AtomicU64::new(0);
const PENDING: &str =
    r#"{"provenance":"synthetic pending-plan test","contract":[],"stages":{},"comparison":null}"#;
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "recorded-driver-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn plan() -> Plan {
    serde_json::from_str(PENDING).unwrap()
}
#[test]
fn pending_is_not_success_and_preserves_existing_output() {
    let temp = Temp::new();
    let config = temp.0.join("plan.json");
    fs::write(&config, PENDING).unwrap();
    let out = temp.0.join("result");
    assert!(!run(&config, &temp.0.join("unavailable-source"), &out).unwrap());
    assert_eq!(
        read_json(&out.join("evidence.json")).unwrap()["status"],
        "pending"
    );
    assert!(run(&config, &temp.0, &out).is_err());
}
#[test]
fn comparison_preserves_unknown_and_namespaced_identity() {
    let mut p = plan();
    let mut pointers: BTreeMap<_, _> = FIELDS
        .iter()
        .map(|f| (f.to_string(), format!("/{f}")))
        .collect();
    pointers.insert("accounting".into(), "/accounting".into());
    p.comparison = Some(BTreeMap::from([
        ("studio".into(), pointers.clone()),
        ("observability".into(), pointers),
    ]));
    // Synthetic test report vocabulary only; this does not define bundle fields.
    let value = json!({"semantic_digest":"synthetic", "run_identity":{"namespace":"test","value":"one"},
        "event_records":1,"accounting_records":1,"unsupported_records":0,"completeness":{},"streams":[],
        "evidence":{"gameplay":"unknown"}, "accounting":{"tokens":{"status":"unknown"}}});
    let mut reports = BTreeMap::from([
        ("studio".into(), value.clone()),
        ("observability".into(), value),
    ]);
    compare(&p, &reports).unwrap();
    reports.get_mut("observability").unwrap()["accounting"]["tokens"] = json!(0);
    assert_eq!(
        compare(&p, &reports).unwrap_err(),
        "consumer disagreement: accounting"
    );
    reports.get_mut("studio").unwrap()["run_identity"] = Value::Null;
    assert!(
        compare(&p, &reports)
            .unwrap_err()
            .contains("missing or null")
    );
}
#[test]
fn pins_reject_tampering_and_symlinks() {
    let temp = Temp::new();
    let file = temp.0.join("artifact");
    fs::write(&file, b"synthetic").unwrap();
    let pin = Pin {
        path: file.clone(),
        sha256: digest(&file, 100).unwrap(),
    };
    check_pin(&pin).unwrap();
    fs::write(&file, b"tampered").unwrap();
    assert!(check_pin(&pin).is_err());
    #[cfg(unix)]
    {
        let link = temp.0.join("link");
        std::os::unix::fs::symlink(&file, &link).unwrap();
        assert!(digest(&link, 100).is_err());
    }
}
#[test]
#[cfg(unix)]
fn failed_validator_stops_both_consumers_and_writes_failure() {
    let temp = Temp::new();
    let repo = temp.0.join("repo");
    fs::create_dir(&repo).unwrap();
    for args in [
        vec!["init", "-q"],
        vec![
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
            "commit",
            "--allow-empty",
            "-qm",
            "synthetic",
        ],
    ] {
        assert!(
            Command::new("git")
                .args(args)
                .current_dir(&repo)
                .status()
                .unwrap()
                .success()
        );
    }
    let revision = String::from_utf8(
        Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(&repo)
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    let executable = fs::canonicalize("/bin/sh").unwrap();
    let pin = json!({"path":executable,"sha256":digest(&executable, 512*1024*1024).unwrap()});
    let candidate = repo.join("candidate.txt");
    fs::write(&candidate, b"synthetic candidate source").unwrap();
    let source_pin = json!({"path":candidate,"sha256":digest(&candidate,100).unwrap()});
    let stage = |script: &str| json!({"cwd":repo,"revision":revision.trim(),"executable":pin,"args":["-c",script,"test","{bundle}","{summary}"],"timeout_seconds":2,"stdout_json":true,"source_pins":[source_pin]});
    let config = temp.0.join("plan.json");
    let fixture = json!({"provenance":"synthetic process mechanics only","contract":[pin],
        "stages":{"export":stage("printf synthetic > \"$1\"; printf '{}'"),
        "validate":stage("exit 7"),"studio":stage("exit 88"),"observability":stage("exit 88")},
        "comparison":{}});
    fs::write(&config, serde_json::to_vec(&fixture).unwrap()).unwrap();
    let source = temp.0.join("source");
    fs::create_dir(&source).unwrap();
    let out = temp.0.join("result");
    assert!(
        run(&config, &source, &out)
            .unwrap_err()
            .contains("stage exit")
    );
    let evidence = read_json(&out.join("evidence.json")).unwrap();
    assert_eq!(evidence["status"], "failed");
    assert_eq!(evidence["stages"].as_array().unwrap().len(), 2);
    assert!(!out.join("studio.json").exists());
    assert_eq!(read_json(&out.join("export.json")).unwrap(), json!({}));
    // A stage may leave its own runtime pins unchanged while mutating the shared
    // contract. It must fail before any consumer and cannot declare comparison pass.
    let contract = temp.0.join("contract.json");
    fs::write(&contract, b"original contract").unwrap();
    let mut mutation = fixture.clone();
    mutation["contract"] = json!([{"path":contract,"sha256":digest(&contract,100).unwrap()}]);
    mutation["stages"]["export"]["args"] = json!([
        "-c",
        "printf changed > \"$3\"; printf synthetic > \"$1\"; printf '{}'",
        "test",
        "{bundle}",
        "{summary}",
        contract
    ]);
    fs::write(&config, serde_json::to_vec(&mutation).unwrap()).unwrap();
    let mutated_out = temp.0.join("mutated-result");
    assert_eq!(
        run(&config, &source, &mutated_out).unwrap_err(),
        "pinned file digest mismatch"
    );
    let rejected = read_json(&mutated_out.join("evidence.json")).unwrap();
    assert_eq!(rejected["status"], "failed");
    assert_eq!(rejected["stages"].as_array().unwrap().len(), 1);
    assert!(!mutated_out.join("validate.json").exists());
    let unpinned: Stage = serde_json::from_value(stage("exit 0")).unwrap();
    fs::write(candidate, b"changed after pinning").unwrap();
    assert!(check_stage(&unpinned).is_err());
}
