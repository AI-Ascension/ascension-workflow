// SPDX-License-Identifier: MIT

//! Original synthetic process fixtures. These invoke the compiled delivery runner,
//! not owner implementations, and prove only its plan/report acceptance boundary.

#![cfg(unix)]

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);
const ROLES: [&str; 3] = ["validate", "studio", "observability"];

struct Fixture {
    root: PathBuf,
    plan: Value,
}

impl Fixture {
    fn new() -> Self {
        let parent =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/recorded-json-process-tests");
        fs::create_dir_all(&parent).unwrap();
        let root = parent.join(format!(
            "{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let repo = root.join("synthetic-stage-repository");
        fs::create_dir(&repo).unwrap();
        for args in [
            vec!["init", "-q"],
            vec![
                "-c",
                "user.name=Synthetic Test",
                "-c",
                "user.email=test@example.invalid",
                "commit",
                "--allow-empty",
                "-qm",
                "synthetic process metadata",
            ],
        ] {
            assert!(
                Command::new("git")
                    .args(args)
                    .current_dir(&repo)
                    .stdin(Stdio::null())
                    .status()
                    .unwrap()
                    .success()
            );
        }
        let revision = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(&repo)
            .output()
            .unwrap();
        assert!(revision.status.success());
        let revision = String::from_utf8(revision.stdout).unwrap();
        let executable = fs::canonicalize("/bin/sh").unwrap();
        let executable_pin = json!({
            "path": executable,
            "sha256": format!("{:x}", Sha256::digest(fs::read(&executable).unwrap()))
        });
        let contract = root.join("synthetic-contract");
        fs::write(&contract, "synthetic process boundary only").unwrap();
        fs::write(
            root.join("synthetic-input.zip"),
            "synthetic bytes; not an owner bundle",
        )
        .unwrap();
        let mut stages = json!({"export": null});
        for role in ROLES {
            let report = root.join(format!("{role}-input.json"));
            fs::write(&report, serde_json::to_vec(&valid_report()).unwrap()).unwrap();
            stages[role] = json!({
                "cwd": repo, "revision": revision.trim(), "executable": executable_pin,
                "args": ["-c", "printf executed > \"$2\"; cat \"$1\"", "synthetic-stage",
                    report, root.join(format!("{role}.executed"))],
                "timeout_seconds": 2, "stdout_json": true
            });
        }
        let pointers = json!({
            "semantic_digest": "/semantic_digest", "run_identity": "/recording_identity",
            "event_records": "/event_records", "accounting_records": "/accounting_records",
            "unsupported_records": "/unsupported_records", "evidence": "/evidence",
            "completeness": "/completeness", "streams": "/streams"
        });
        let plan = json!({
            "provenance": "Original synthetic delivery parser/process fixture; no owner acceptance",
            "contract": [{"path": contract,
                "sha256": format!("{:x}", Sha256::digest(fs::read(&contract).unwrap()))}],
            "stages": stages, "existing_bundle": true,
            "comparison": {"validate": pointers, "studio": pointers, "observability": pointers}
        });
        Self { root, plan }
    }

    fn execute(&self, plan: &[u8]) -> Output {
        let path = self.root.join("plan.json");
        fs::write(&path, plan).unwrap();
        Command::new(env!("CARGO_BIN_EXE_recorded-run-driver"))
            .arg(path)
            .arg(self.root.join("synthetic-input.zip"))
            .arg(self.root.join("result"))
            .output()
            .expect("compiled recorded-run driver")
    }

    fn report(&self, role: &str, bytes: &str) {
        fs::write(self.root.join(format!("{role}-input.json")), bytes).unwrap();
    }

    fn executed(&self, role: &str) -> bool {
        self.root.join(format!("{role}.executed")).exists()
    }

    fn evidence(&self) -> Value {
        serde_json::from_slice(&fs::read(self.root.join("result/evidence.json")).unwrap()).unwrap()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).expect("only this owned synthetic fixture child");
    }
}

fn valid_report() -> Value {
    json!({
        "valid": true, "semantic_digest": "synthetic",
        "recording_identity": {"namespace": "synthetic", "value": "one"},
        "event_records": 0, "accounting_records": 0, "unsupported_records": 0,
        "evidence": {"gameplay": "unknown"}, "completeness": {}, "streams": []
    })
}

#[test]
fn compiled_runner_accepts_matching_unambiguous_synthetic_reports() {
    let fixture = Fixture::new();
    let output = fixture.execute(&serde_json::to_vec(&fixture.plan).unwrap());
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(fixture.evidence()["status"], "process_comparison_passed");
    assert!(ROLES.into_iter().all(|role| fixture.executed(role)));
}

#[test]
fn duplicate_validation_flag_cannot_overwrite_a_failed_verdict() {
    let fixture = Fixture::new();
    let report = serde_json::to_string(&valid_report())
        .unwrap()
        .replace("\"valid\":true", "\"valid\":false,\"valid\":true");
    fixture.report("validate", &report);
    let output = fixture.execute(&serde_json::to_vec(&fixture.plan).unwrap());
    assert!(
        !output.status.success(),
        "ambiguous validation report qualified as success"
    );
    assert!(fixture.executed("validate"));
    assert!(!fixture.executed("studio"));
    assert!(!fixture.executed("observability"));
    assert_eq!(fixture.evidence()["status"], "failed");
}

#[test]
fn escaped_duplicate_identity_cannot_hide_a_different_consumer_result() {
    let fixture = Fixture::new();
    let report = serde_json::to_string(&valid_report()).unwrap().replace(
        "\"value\":\"one\"",
        "\"value\":\"SYNTHETIC_PRIVATE_DUPLICATE_ID\",\"v\\u0061lue\":\"one\"",
    );
    fixture.report("studio", &report);
    let output = fixture.execute(&serde_json::to_vec(&fixture.plan).unwrap());
    assert!(
        !output.status.success(),
        "ambiguous identity qualified as consumer agreement"
    );
    assert!(fixture.executed("validate") && fixture.executed("studio"));
    assert!(!fixture.executed("observability"));
    assert_eq!(fixture.evidence()["status"], "failed");
    assert!(!String::from_utf8_lossy(&output.stderr).contains("SYNTHETIC_PRIVATE_DUPLICATE_ID"));
}

#[test]
fn duplicate_stage_keys_are_refused_before_output_or_execution() {
    let fixture = Fixture::new();
    let stage = serde_json::to_string(&fixture.plan["stages"]["validate"]).unwrap();
    let plan = serde_json::to_string(&fixture.plan).unwrap().replace(
        &format!("\"validate\":{stage}"),
        &format!("\"validate\":null,\"val\\u0069date\":{stage}"),
    );
    let output = fixture.execute(plan.as_bytes());
    assert!(
        !output.status.success(),
        "ambiguous stage selection qualified as a plan"
    );
    assert!(!fixture.root.join("result").exists());
    assert!(ROLES.into_iter().all(|role| !fixture.executed(role)));
}

#[test]
fn duplicate_comparison_pointer_cannot_silently_change_the_reviewed_plan() {
    let fixture = Fixture::new();
    let plan = serde_json::to_string(&fixture.plan).unwrap().replacen(
        "\"semantic_digest\":\"/semantic_digest\"",
        "\"semantic_digest\":\"/unreviewed\",\"semantic_dige\\u0073t\":\"/semantic_digest\"",
        1,
    );
    let output = fixture.execute(plan.as_bytes());
    assert!(
        !output.status.success(),
        "ambiguous comparison mapping qualified as a plan"
    );
    assert!(!fixture.root.join("result").exists());
    assert!(ROLES.into_iter().all(|role| !fixture.executed(role)));
}
