// SPDX-License-Identifier: MIT
//! Synthetic receipts only: compiled subprocess checks of the matrix acceptance boundary.
#[path = "../src/bin/candidate_matrix/receipt.rs"]
mod receipt;
use serde_json::{Value, json};
use std::process::Command;

fn reports() -> Vec<Value> {
    receipt::ROLES.iter().map(|role| {
        let summary = json!({
            "digest":"opaque digest", "identity":"owner:any punctuation!",
            "identities":{}, "counts":{}, "evidence":{}, "accounting":[],
            "omissions":{}, "observations":[], "events":[], "provenance":{},
            "completeness":{}, "original_events":[], "original_accounting":[],
            "persistence":{"first":"imported","retry":"duplicate","revisions":1,"delivery":"local_only"}
        });
        json!({"role":role,"results":[
            {"path":"good.zip","expected":true,"accepted":true,"summary":summary},
            {"path":"bad.zip","expected":false,"accepted":false}
        ]})
    }).collect()
}

#[test]
fn receipt_child() {
    let Ok(mode) = std::env::var("SYNTHETIC_MATRIX_CASE") else {
        return;
    };
    let expected = receipt::expectations(&json!({"cases":[
        {"path":"good.zip","valid":true},{"path":"bad.zip","valid":false}
    ]}))
    .unwrap();
    let mut r = reports();
    match mode.as_str() {
        "control" => {}
        "reordered" => r[1]["results"].as_array_mut().unwrap().reverse(),
        "empty" => r[0]["results"] = json!([]),
        "missing" => {
            r[1]["results"].as_array_mut().unwrap().pop();
        }
        "duplicate" => r[1]["results"][1] = r[1]["results"][0].clone(),
        "extra" => r[0]["results"]
            .as_array_mut()
            .unwrap()
            .push(json!({"path":"extra"})),
        "wrong_role" => r[2]["role"] = json!("studio"),
        "missing_owner" => {
            r.pop();
        }
        "forged_expected" => {
            r[0]["results"][1]["expected"] = json!(true);
            r[0]["results"][1]["accepted"] = json!(true);
        }
        "null_verdict" => {
            r[0]["results"][0]["expected"] = Value::Null;
            r[0]["results"][0]["accepted"] = Value::Null;
        }
        "string_verdict" => r[0]["results"][0]["accepted"] = json!("true"),
        "missing_summary" => {
            r[1]["results"][0]
                .as_object_mut()
                .unwrap()
                .remove("summary");
        }
        "missing_projection" => {
            for owner in &mut r {
                owner["results"][0]["summary"]
                    .as_object_mut()
                    .unwrap()
                    .remove("digest");
            }
        }
        "disagreement" => r[1]["results"][0]["summary"]["identity"] = json!("different"),
        "original_events" => r[2]["results"][0]["summary"]["original_events"] = json!([{}]),
        "first" => r[2]["results"][0]["summary"]["persistence"]["first"] = json!("duplicate"),
        "retry" => r[2]["results"][0]["summary"]["persistence"]["retry"] = json!("imported"),
        "revisions" => r[2]["results"][0]["summary"]["persistence"]["revisions"] = json!(2),
        "delivery" => r[2]["results"][0]["summary"]["persistence"]["delivery"] = json!("remote"),
        _ => panic!("unknown synthetic case"),
    }
    let findings = receipt::validate(&r, &expected);
    assert!(findings.is_empty(), "{findings:?}");
}

#[test]
fn compiled_receipt_regressions() {
    for mode in [
        "control",
        "reordered",
        "empty",
        "missing",
        "duplicate",
        "extra",
        "wrong_role",
        "missing_owner",
        "forged_expected",
        "null_verdict",
        "string_verdict",
        "missing_summary",
        "missing_projection",
        "disagreement",
        "original_events",
        "first",
        "retry",
        "revisions",
        "delivery",
    ] {
        let output = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "receipt_child", "--nocapture"])
            .env("SYNTHETIC_MATRIX_CASE", mode)
            .output()
            .unwrap();
        assert_eq!(
            output.status.success(),
            matches!(mode, "control" | "reordered"),
            "{mode}: {}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn malformed_inventory_fails_closed() {
    for input in [
        json!({"cases":[]}),
        json!({"cases":[{"path":"a","valid":null}]}),
        json!({"cases":[{"path":"a","valid":true},{"path":"a","valid":false}]}),
    ] {
        assert!(receipt::expectations(&input).is_err());
    }
}

fn private_sentinel() -> String {
    format!(
        "UNTRUSTED_RECEIPT_SENTINEL\n{}",
        "private-data".repeat(4096)
    )
}

#[test]
fn findings_never_echo_foreign_role_or_paths() {
    let expected = receipt::expectations(&json!({"cases":[
        {"path":"good.zip","valid":true},{"path":"bad.zip","valid":false}
    ]}))
    .unwrap();
    let sentinel = private_sentinel();
    let mut owners = reports();
    owners[0]["role"] = json!(sentinel);
    owners[1]["results"] = json!([{"path":sentinel}, {"path":sentinel}]);
    let findings = receipt::validate(&owners, &expected);
    assert!(
        findings
            .iter()
            .any(|f| f == "unexpected or duplicate owner role")
    );
    assert!(findings.iter().any(|f| f == "unexpected case path"));
    assert!(findings.iter().any(|f| f == "duplicate case path"));
    for finding in findings {
        assert!(!finding.contains("UNTRUSTED_RECEIPT_SENTINEL"));
        assert!(!finding.contains("private-data"));
        assert!(!finding.contains('\n'));
        assert!(finding.len() < 200);
    }
}

/// Requires the unchanged published candidate artifact, not an invented pin.
#[test]
fn compiled_candidate_command() {
    use std::{fs, os::unix::fs::PermissionsExt, path::PathBuf};
    let artifact = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/candidate3");
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/candidate-matrix-cli")
        .join(std::process::id().to_string());
    fs::create_dir_all(base.join("artifacts")).unwrap();
    assert!(
        Command::new("cp")
            .arg("-R")
            .arg(&artifact)
            .arg(base.join("artifacts/recorded-run-bundle-v1-candidate3"))
            .status()
            .unwrap()
            .success()
    );
    let inventory: Value =
        serde_json::from_slice(&fs::read(artifact.join("conformance.json")).unwrap()).unwrap();
    let node = base.join("node");
    fs::write(
        &node,
        "#!/bin/sh\ncat \"$SYNTHETIC_MATRIX_REPORTS/$4.json\"\n",
    )
    .unwrap();
    fs::set_permissions(&node, fs::Permissions::from_mode(0o700)).unwrap();
    for mode in [
        "control",
        "empty",
        "missing",
        "duplicate",
        "wrong_role",
        "null_verdict",
        "missing_summary",
        "persistence",
        "accepted",
        "expected",
        "original_accounting",
        "setup_error",
        "malformed_results",
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
        "foreign_role",
        "foreign_duplicate_path",
        "all_omit_same_vector",
        "provided_control",
        "provided_missing",
        "provided_foreign",
    ] {
        let provided = mode.starts_with("provided_");
        let mut owners = reports();
        for owner in &mut owners {
            let summary = owner["results"][0]["summary"].clone();
            owner["results"] = Value::Array(
                inventory["cases"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|c| {
                        let mut row =
                            json!({"path":c["path"],"expected":c["valid"],"accepted":c["valid"]});
                        if c["valid"] == true {
                            row["summary"] = summary.clone();
                        }
                        row
                    })
                    .collect(),
            );
            if provided {
                owner["results"] = json!([{
                    "path":"provided.zip", "expected":true, "accepted":true, "summary":summary
                }]);
            }
        }
        match mode {
            "all_omit_same_vector" => {
                for owner in &mut owners {
                    owner["results"].as_array_mut().unwrap().pop();
                }
            }
            "provided_missing" => {
                for owner in &mut owners {
                    owner["results"] = json!([]);
                }
            }
            "provided_foreign" => {
                for owner in &mut owners {
                    owner["results"][0]["path"] = json!("foreign.zip");
                }
            }
            "foreign_role" => owners[0]["role"] = json!(private_sentinel()),
            "foreign_duplicate_path" => {
                owners[1]["results"] = json!([
                    {"path":private_sentinel()}, {"path":private_sentinel()}
                ])
            }
            "empty" => owners[0]["results"] = json!([]),
            "missing" => {
                owners[1]["results"].as_array_mut().unwrap().pop();
            }
            "duplicate" => owners[1]["results"][1] = owners[1]["results"][0].clone(),
            "wrong_role" => owners[2]["role"] = json!("studio"),
            "null_verdict" => {
                owners[0]["results"][0]["expected"] = Value::Null;
                owners[0]["results"][0]["accepted"] = Value::Null;
            }
            "missing_summary" => {
                owners[1]["results"][0]
                    .as_object_mut()
                    .unwrap()
                    .remove("summary");
            }
            "persistence" => {
                owners[2]["results"][0]["summary"]["persistence"]["first"] = json!("duplicate")
            }
            "accepted" | "expected" => {
                owners[0]["results"][0]
                    .as_object_mut()
                    .unwrap()
                    .remove(mode);
            }
            "original_accounting" => owners[2]["results"][0]["summary"][mode] = json!([{}]),
            "setup_error" => owners[1]["setup_error"] = json!("failed"),
            "malformed_results" => owners[1]["results"] = json!({}),
            "control" | "provided_control" => {}
            key => {
                owners[1]["results"][0]["summary"]
                    .as_object_mut()
                    .unwrap()
                    .remove(key);
            }
        }
        for (role, owner) in receipt::ROLES.iter().zip(owners) {
            fs::write(
                base.join(format!("{role}.json")),
                serde_json::to_vec(&owner).unwrap(),
            )
            .unwrap();
        }
        let out = base.join(mode);
        let mut command = Command::new(env!("CARGO_BIN_EXE_candidate-matrix"));
        command.args([&base, &base, &base, &out]);
        if provided {
            command.arg(artifact.join("golden/legacy-failed.zip"));
        }
        let output = command
            .env(
                "PATH",
                format!(
                    "{}:{}",
                    base.display(),
                    std::env::var("PATH").unwrap_or_default()
                ),
            )
            .env("SYNTHETIC_MATRIX_REPORTS", &base)
            .output()
            .unwrap();
        assert_eq!(
            output.status.success(),
            matches!(mode, "control" | "provided_control"),
            "{mode}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let evidence: Value =
            serde_json::from_slice(&fs::read(out.join("matrix.json")).unwrap()).unwrap();
        let findings = serde_json::to_string(&evidence["findings"]).unwrap();
        assert!(!findings.contains("UNTRUSTED_RECEIPT_SENTINEL"));
        assert!(!findings.contains("private-data"));
        let expected_status = match mode {
            "control" => "synthetic_matrix_passed",
            "provided_control" => "provided_bundle_matrix_passed",
            _ => "failed",
        };
        assert_eq!(evidence["status"], expected_status);
    }
    fs::remove_dir_all(base).unwrap();
}
