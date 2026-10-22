// SPDX-License-Identifier: MIT

//! Original synthetic consumer fixtures; these are not owner execution receipts.

use super::*;
use serde_json::{Value, json};
#[cfg(unix)]
use std::os::unix::fs::{PermissionsExt, symlink};

const RUN: &str = "run.export-fixture";
const FIXTURE: &[u8] = include_bytes!("../../tests/fixtures/redacted-export.json");

fn fixture() -> Value {
    serde_json::from_slice(FIXTURE).unwrap()
}

fn reject_change(pointer: &str, value: Value) {
    let mut export = fixture();
    *export.pointer_mut(pointer).expect("fixture pointer") = value;
    assert!(
        verify_bytes(&serde_json::to_vec_pretty(&export).unwrap(), RUN).is_err(),
        "accepted invalid field {pointer}"
    );
}

#[test]
fn accepts_only_the_redacted_projection_independent_of_json_formatting() {
    assert_eq!(verify_bytes(FIXTURE, RUN), Ok(()));
    assert_eq!(
        verify_bytes(&serde_json::to_vec(&fixture()).unwrap(), RUN),
        Ok(())
    );
    let escaped = String::from_utf8(FIXTURE.to_vec())
        .unwrap()
        .replace("[redacted]", "[red\\u0061cted]")
        .replace("\"operation_id\"", "\"oper\\u0061tion_id\"");
    assert_eq!(verify_bytes(escaped.as_bytes(), RUN), Ok(()));
    let mut explicit_null = fixture();
    explicit_null["events"][0]["integrity_digest"] = Value::Null;
    for classification in ["accepted", "unknown", "settled", "rejected", "cancelled"] {
        explicit_null["events"][0]["payload"]["classification"] = json!(classification);
        assert_eq!(
            verify_bytes(&serde_json::to_vec(&explicit_null).unwrap(), RUN),
            Ok(())
        );
    }
}

#[test]
fn refuses_every_nonredacted_cursor_operation_and_event_field() {
    for path in [
        "/run/cursor/graph_id",
        "/run/cursor/node_id",
        "/run/cursor/node_execution_id",
        "/run/pending_operation",
        "/events/0/node_execution_id",
        "/events/0/payload/operation_id",
        "/events/0/payload/reason_code",
    ] {
        reject_change(path, json!("private-probe"));
    }
    reject_change(
        "/run/pending_operation",
        json!({"operation_id": "private-probe"}),
    );
    reject_change("/events/0/node_execution_id", json!({"[redacted]": null}));
    reject_change("/events/0/payload/reason_code", json!({"event": null}));
    let mut export = fixture();
    export["events"][0]["integrity_digest"] = json!("b".repeat(64));
    assert!(verify_bytes(&serde_json::to_vec(&export).unwrap(), RUN).is_err());
}

#[test]
fn refuses_missing_and_unknown_fields_in_every_closed_record() {
    for pointer in [
        "",
        "/run",
        "/run/cursor",
        "/run/budget",
        "/events/0",
        "/events/0/payload",
    ] {
        let object = fixture()
            .pointer(pointer)
            .unwrap()
            .as_object()
            .unwrap()
            .clone();
        for key in object.keys() {
            let mut missing = fixture();
            missing
                .pointer_mut(pointer)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .remove(key);
            assert!(
                verify_bytes(&serde_json::to_vec(&missing).unwrap(), RUN).is_err(),
                "accepted omitted {pointer}/{key}"
            );
        }
        let mut unknown = fixture();
        unknown
            .pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("unreviewed_private_data".to_owned(), json!("private-probe"));
        assert!(verify_bytes(&serde_json::to_vec(&unknown).unwrap(), RUN).is_err());
    }
}

#[test]
fn refuses_duplicate_and_escaped_duplicate_keys_in_every_record() {
    let compact = serde_json::to_string(&fixture()).unwrap();
    for (field, value, escaped) in [
        ("redacted", "true", "red\\u0061cted"),
        ("status", "\"paused\"", "st\\u0061tus"),
        ("graph_id", "\"[redacted]\"", "gr\\u0061ph_id"),
        ("node_steps_consumed", "1", "node_steps_consum\\u0065d"),
        ("sequence", "1", "sequ\\u0065nce"),
        ("classification", "null", "cl\\u0061ssification"),
    ] {
        let needle = format!("\"{field}\":{value}");
        for duplicate in [field, escaped] {
            let replacement = format!("{needle},\"{duplicate}\":{value}");
            let bytes = compact.replacen(&needle, &replacement, 1);
            assert_ne!(bytes, compact, "fixture match for {field}");
            assert_eq!(
                verify_bytes(bytes.as_bytes(), RUN),
                Err("export_json_or_shape_invalid")
            );
        }
    }
}

#[test]
fn refuses_foreign_run_and_definition_evidence_at_every_level() {
    for path in [
        "/workflow_run_id",
        "/run/workflow_run_id",
        "/events/0/workflow_run_id",
    ] {
        reject_change(path, json!("run.foreign"));
    }
    reject_change("/events/0/definition_digest", json!("b".repeat(64)));
    for digest in ["A".repeat(64), "a".repeat(63), "g".repeat(64)] {
        reject_change("/run/definition_digest", json!(digest));
    }
    for run in ["", "run.foreign", "../run.export-fixture", "[redacted]"] {
        assert_eq!(verify_bytes(FIXTURE, run), Err("export_run_identity"));
    }
}

#[test]
fn refuses_unreviewed_versions_flags_states_and_unsafe_numbers() {
    for path in [
        "/schema_version",
        "/run/schema_version",
        "/events/0/schema_version",
    ] {
        reject_change(path, json!("unreviewed/v2"));
    }
    reject_change("/redacted", json!(false));
    for path in [
        "/run/status",
        "/run/game_outcome",
        "/run/cleanup",
        "/events/0/event_type",
        "/events/0/payload/classification",
    ] {
        reject_change(path, json!("private-probe"));
    }
    for path in [
        "/run/run_revision",
        "/events/0/sequence",
        "/events/0/run_revision",
        "/run/budget/provider_calls_consumed",
        "/run/budget/provider_calls_reserved",
        "/run/budget/node_steps_consumed",
        "/run/budget/replans_consumed",
    ] {
        reject_change(path, json!(MAX_INTEGER + 1));
        reject_change(path, json!(-1));
        reject_change(path, json!(0.5));
    }
    for path in [
        "/run/run_revision",
        "/events/0/sequence",
        "/events/0/run_revision",
    ] {
        reject_change(path, json!(0));
    }
    reject_change("/events/0/run_revision", json!(5));
}

#[test]
fn requires_nonempty_ordered_events_without_inventing_replay_semantics() {
    reject_change("/events", json!([]));
    let mut export = fixture();
    let event = export["events"][0].clone();
    export["events"].as_array_mut().unwrap().push(event);
    assert_eq!(
        verify_bytes(&serde_json::to_vec(&export).unwrap(), RUN),
        Err("export_event_order")
    );
    export["events"][1]["sequence"] = json!(3);
    assert_eq!(
        verify_bytes(&serde_json::to_vec(&export).unwrap(), RUN),
        Ok(())
    );
    export["events"][0]["sequence"] = json!(4);
    assert_eq!(
        verify_bytes(&serde_json::to_vec(&export).unwrap(), RUN),
        Err("export_event_order")
    );
}

#[test]
fn malformed_documents_and_errors_never_echo_unverified_values() {
    for bytes in [
        b"null".as_slice(),
        b"[]",
        b"{}",
        b"\xff",
        b"{\"private-probe\":",
    ] {
        assert_eq!(
            verify_bytes(bytes, RUN),
            Err("export_json_or_shape_invalid")
        );
    }
    let mut trailing = FIXTURE.to_vec();
    trailing.extend_from_slice(b"{}");
    assert_eq!(
        verify_bytes(&trailing, RUN),
        Err("export_json_or_shape_invalid")
    );
    let mut export = fixture();
    export["events"][0]["payload"]["operation_id"] = json!("private-probe");
    let bytes = serde_json::to_vec(&export).unwrap();
    let error = verify_bytes(&bytes, RUN).unwrap_err();
    assert_eq!(error, "export_json_or_shape_invalid");
    assert!(!error.contains("private-probe"));
}

#[test]
fn read_limit_holds_for_a_stream_that_exceeds_its_initial_metadata() {
    let mut reader = std::io::repeat(b' ').take(MAX_BYTES as u64 + 100);
    assert_eq!(read_bounded(&mut reader), Err("export_file_bound"));
    assert_eq!(
        reader.limit(),
        99,
        "read at most the byte limit plus one sentinel"
    );
    assert_eq!(
        read_bounded(std::io::repeat(b' ').take(MAX_BYTES as u64))
            .unwrap()
            .len(),
        MAX_BYTES
    );
    assert_eq!(
        verify_bytes(&vec![b' '; MAX_BYTES + 1], RUN),
        Err("export_file_bound")
    );
}

struct Root(PathBuf);

impl Root {
    fn new() -> Self {
        let parent = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/export-unit-tests");
        fs::create_dir_all(&parent).unwrap();
        let child = parent.join(format!(
            "{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&child).unwrap();
        Self(child)
    }
}

impl Drop for Root {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn capture_is_exclusive_private_and_cleans_only_its_known_output() {
    let root = Root::new();
    let first = Capture::in_directory(&root.0).unwrap();
    let second = Capture::in_directory(&root.0).unwrap();
    assert_ne!(first.path(), second.path());
    #[cfg(unix)]
    assert_eq!(
        fs::metadata(&first.directory).unwrap().permissions().mode() & 0o777,
        0o700
    );
    fs::write(first.path(), FIXTURE).unwrap();
    assert_eq!(first.verify(RUN), Ok(()));
    let directory = first.directory.clone();
    drop(first);
    assert!(!directory.exists());
    let unexpected = second.directory.join("unrelated");
    fs::write(&unexpected, "keep").unwrap();
    fs::write(second.path(), FIXTURE).unwrap();
    let file = second.file.clone();
    drop(second);
    assert!(!file.exists());
    assert_eq!(fs::read_to_string(unexpected).unwrap(), "keep");
    assert!(Capture::in_directory(Path::new("relative")).is_err());
}

#[test]
fn refuses_unavailable_nonregular_and_oversized_files_before_decoding() {
    let root = Root::new();
    let path = root.0.join("export.json");
    assert_eq!(verify_file(&path, RUN), Err("export_file_unavailable"));
    assert_eq!(verify_file(&root.0, RUN), Err("export_file_type"));
    let file = File::create(&path).unwrap();
    file.set_len(MAX_BYTES as u64 + 1).unwrap();
    assert_eq!(verify_file(&path, RUN), Err("export_file_bound"));
    #[cfg(unix)]
    {
        let link = root.0.join("symlink");
        symlink(&path, &link).unwrap();
        assert_eq!(verify_file(&link, RUN), Err("export_file_type"));
    }
}

#[cfg(unix)]
#[test]
fn unrepresentable_export_path_has_a_fixed_private_diagnostic() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    let root = Root::new();
    let parent = root
        .0
        .join(OsString::from_vec(b"private-path-probe-\xff".to_vec()));
    fs::create_dir(&parent).unwrap();
    let capture = Capture::in_directory(&parent).unwrap();
    assert_eq!(capture.argument(), Err("export_temporary_path"));
}
