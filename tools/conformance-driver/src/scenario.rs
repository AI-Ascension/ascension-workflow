// SPDX-License-Identifier: MIT

use super::{
    Arguments, assert_safely_resumable_status, definition_to_str, export, json_bool, json_string,
    path_to_str, run_cli, run_export_cli, start_server, wait_for_health,
};
use std::path::{Path, PathBuf};
use std::process::Child;

pub(super) fn exercise_process(
    arguments: &Arguments,
    address: &str,
    store: &Path,
    definitions: &[PathBuf],
    server: &mut Child,
) -> Result<(), String> {
    wait_for_health(address, server)?;
    for definition in definitions {
        let validation = run_cli(
            &arguments.binary,
            &[
                "validate",
                definition_to_str(definition)?,
                "--capabilities",
                path_to_str(&arguments.capabilities)?,
                "--format",
                "json",
                "--listen",
                address,
                "--auth-profile",
                "synthetic",
            ],
        )?;
        if !json_bool(&validation, "valid")? {
            return Err(format!(
                "catalog definition {} did not validate",
                definition.display()
            ));
        }
    }
    let negative = run_cli(
        &arguments.binary,
        &[
            "validate",
            path_to_str(&arguments.negative)?,
            "--capabilities",
            path_to_str(&arguments.missing_capabilities)?,
            "--format",
            "json",
            "--listen",
            address,
            "--auth-profile",
            "synthetic",
        ],
    )?;
    if !negative.contains("\"valid\":false") || !negative.contains("capability_unavailable") {
        return Err("negative capability case did not report a capability error".to_owned());
    }
    let map_definition = definitions
        .iter()
        .find(|path| path.file_name().and_then(|name| name.to_str()) == Some("map.strict.json"))
        .ok_or("catalog is missing map.strict.json")?;
    let run_response = run_cli(
        &arguments.binary,
        &[
            "run",
            definition_to_str(map_definition)?,
            "--instance",
            "instance.synthetic.full",
            "--profile",
            "synthetic",
            "--format",
            "json",
            "--listen",
            address,
            "--auth-profile",
            "synthetic",
        ],
    )?;
    let run_id = json_string(&run_response, "workflow_run_id")?;
    let status = run_cli(
        &arguments.binary,
        &[
            "status",
            &run_id,
            "--format",
            "json",
            "--listen",
            address,
            "--auth-profile",
            "synthetic",
        ],
    )?;
    assert_safely_resumable_status(&status)?;
    let _ = run_cli(
        &arguments.binary,
        &[
            "events",
            &run_id,
            "--after-sequence",
            "0",
            "--limit",
            "32",
            "--format",
            "json",
            "--listen",
            address,
            "--auth-profile",
            "synthetic",
        ],
    )?;
    for (command, revision) in [("pause", "1"), ("resume", "2"), ("step", "3")] {
        let _ = run_cli(
            &arguments.binary,
            &[
                command,
                &run_id,
                "--expected-revision",
                revision,
                "--format",
                "json",
                "--listen",
                address,
                "--auth-profile",
                "synthetic",
            ],
        )?;
    }
    server
        .kill()
        .map_err(|error| format!("stop service for restart: {error}"))?;
    server
        .wait()
        .map_err(|error| format!("wait for service restart: {error}"))?;
    *server = start_server(&arguments.binary, address, store)?;
    wait_for_health(address, server)?;
    let status = run_cli(
        &arguments.binary,
        &[
            "status",
            &run_id,
            "--format",
            "json",
            "--listen",
            address,
            "--auth-profile",
            "synthetic",
        ],
    )?;
    assert_safely_resumable_status(&status)?;
    let _ = run_cli(
        &arguments.binary,
        &[
            "events",
            &run_id,
            "--after-sequence",
            "0",
            "--limit",
            "64",
            "--format",
            "json",
            "--listen",
            address,
            "--auth-profile",
            "synthetic",
        ],
    )?;
    let _ = run_cli(
        &arguments.binary,
        &[
            "replay",
            &run_id,
            "--offline",
            "--format",
            "json",
            "--listen",
            address,
            "--auth-profile",
            "synthetic",
        ],
    )?;
    let export = export::Capture::create()?;
    run_export_cli(
        &arguments.binary,
        &[
            "export",
            &run_id,
            "--redacted",
            "--output",
            export.argument()?,
            "--format",
            "json",
            "--listen",
            address,
            "--auth-profile",
            "synthetic",
        ],
    )?;
    export.verify(&run_id)
}
