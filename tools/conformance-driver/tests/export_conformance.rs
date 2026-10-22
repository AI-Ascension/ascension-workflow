// SPDX-License-Identifier: MIT

//! Original synthetic writer/health fixture. This invokes the compiled delivery driver,
//! not a Harness owner, and establishes only rejection of invalid export evidence.

#![cfg(unix)]

use serde_json::{Value, json};
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let root =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/export-conformance-tests");
        fs::create_dir_all(&root).expect("owned fixture root");
        let path = root.join(format!(
            "{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::DirBuilder::new().mode(0o700).create(&path).unwrap();
        Self(path)
    }

    fn exercise(&self, export: &[u8]) -> Output {
        self.exercise_with_script(export, SCRIPT)
    }

    fn exercise_with_script(&self, export: &[u8], script_body: &str) -> Output {
        let script = self.0.join("synthetic-writer");
        fs::write(&script, script_body).unwrap();
        fs::set_permissions(&script, fs::Permissions::from_mode(0o700)).unwrap();
        let fixture = self.0.join("export-fixture.json");
        fs::write(&fixture, export).unwrap();
        let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        Command::new(env!("CARGO_BIN_EXE_ascension-conformance-driver"))
            .arg("--binary")
            .arg(script)
            .arg("--catalog")
            .arg(repository.join("workflows/sts2"))
            .arg("--capabilities")
            .arg(repository.join("catalog/capabilities/synthetic-full.json"))
            .arg("--missing-capabilities")
            .arg(repository.join("catalog/capabilities/synthetic-missing-map.json"))
            .arg("--negative")
            .arg(repository.join("workflows/sts2/map.strict.json"))
            .env("WORKFLOW_EXPORT_FIXTURE", fixture)
            .env(
                "WORKFLOW_EXPORT_TEST_EXECUTABLE",
                std::env::current_exe().unwrap(),
            )
            .env("TMPDIR", &self.0)
            .output()
            .expect("compiled conformance driver")
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).expect("remove only the owned fixture child");
    }
}

const SCRIPT: &str = r#"#!/bin/sh
case "$1" in
  serve)
    export WORKFLOW_EXPORT_TEST_ADDRESS="$3"
    exec "$WORKFLOW_EXPORT_TEST_EXECUTABLE" --ignored --exact synthetic_health_server --nocapture
    ;;
  validate)
    case "$*" in
      *synthetic-missing-map*) printf '{"valid":false,"code":"capability_unavailable"}' ;;
      *) printf '{"valid":true}' ;;
    esac
    ;;
  run) printf '{"workflow_run_id":"run.export-fixture"}' ;;
  status) printf '{"recovery_admission":{"kind":"safely_resumable","capability":"synthetic.workflow.resume.v1"}}' ;;
  export)
    shift
    while [ "$#" -gt 1 ]; do
      if [ "$1" = "--output" ]; then cp "$WORKFLOW_EXPORT_FIXTURE" "$2"; exit "$?"; fi
      shift
    done
    exit 2
    ;;
  *) printf '{}' ;;
esac
"#;

#[test]
#[ignore = "owned child-process entry; invoked by the compiled-driver tests"]
fn synthetic_health_server() {
    let address = std::env::var("WORKFLOW_EXPORT_TEST_ADDRESS").unwrap();
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let mut request = [0_u8; 4096];
        let _ = stream.read(&mut request);
        stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}")
            .unwrap();
    }
}

fn redacted_export() -> Value {
    serde_json::from_slice(include_bytes!("fixtures/redacted-export.json")).unwrap()
}

#[test]
fn compiled_driver_accepts_the_exact_redacted_contract() {
    let result = Fixture::new().exercise(&serde_json::to_vec_pretty(&redacted_export()).unwrap());
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(String::from_utf8_lossy(&result.stdout).contains("conformance: PASS"));
}

#[test]
fn compiled_driver_refuses_a_pretty_printed_private_operation() {
    let mut export = redacted_export();
    export["events"][0]["payload"]["operation_id"] = json!("private-operation-probe");
    let result = Fixture::new().exercise(&serde_json::to_vec_pretty(&export).unwrap());
    assert!(
        !result.status.success(),
        "a leaking export must not yield conformance: PASS"
    );
    assert!(!String::from_utf8_lossy(&result.stderr).contains("private-operation-probe"));
}

#[test]
fn compiled_driver_refuses_an_export_for_a_different_run() {
    let mut export = redacted_export();
    export["workflow_run_id"] = json!("run.foreign");
    let result = Fixture::new().exercise(&serde_json::to_vec(&export).unwrap());
    assert!(
        !result.status.success(),
        "foreign export evidence must not qualify this run"
    );
}

#[test]
fn compiled_driver_refuses_an_escaped_private_operation_field() {
    let mut export = redacted_export();
    export["events"][0]["payload"]["operation_id"] = json!("private-operation-probe");
    let bytes = serde_json::to_string(&export)
        .unwrap()
        .replace("\"operation_id\"", "\"oper\\u0061tion_id\"");
    let result = Fixture::new().exercise(bytes.as_bytes());
    assert!(!result.status.success());
    assert!(!String::from_utf8_lossy(&result.stderr).contains("private-operation-probe"));
}

#[test]
fn failed_exporter_cannot_echo_its_unverified_private_output() {
    let script = SCRIPT.replace(
        "  export)\n",
        concat!(
            "  export)\n",
            "    printf private-operation-probe\n",
            "    printf private-operation-probe >&2\n",
            "    exit 7\n"
        ),
    );
    let result = Fixture::new().exercise_with_script(b"", &script);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("export_command_failed"));
    for output in [&result.stdout, &result.stderr] {
        assert!(!String::from_utf8_lossy(output).contains("private-operation-probe"));
    }
}

#[test]
fn successful_export_command_cannot_substitute_for_an_output_file() {
    let script = SCRIPT.replace("  export)\n", "  export)\n    exit 0\n");
    let result = Fixture::new().exercise_with_script(b"", &script);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("export_file_unavailable"));
}
