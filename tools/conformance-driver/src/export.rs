// SPDX-License-Identifier: MIT

mod shape;

#[cfg(test)]
mod tests;

use std::fs::{self, File};
use std::io::Read;
#[cfg(unix)]
use std::os::unix::fs::DirBuilderExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_BYTES: usize = 1024 * 1024;
const MAX_EVENTS: usize = 8192;
const MAX_INTEGER: u64 = 9_007_199_254_740_991;
static NEXT: AtomicU64 = AtomicU64::new(0);

/// One exclusively created private directory, containing only this attempt's output.
pub(super) struct Capture {
    directory: PathBuf,
    file: PathBuf,
}

impl Capture {
    pub(super) fn create() -> Result<Self, String> {
        Self::in_directory(&std::env::temp_dir())
    }

    fn in_directory(parent: &Path) -> Result<Self, String> {
        if !parent.is_absolute() {
            return Err("export_temporary_path".to_owned());
        }
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| "export_temporary_clock")?
            .as_nanos();
        let directory = parent.join(format!(
            "ascension-conformance-export-{}-{time}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let mut builder = fs::DirBuilder::new();
        #[cfg(unix)]
        builder.mode(0o700);
        builder
            .create(&directory)
            .map_err(|_| "export_temporary_create")?;
        Ok(Self {
            file: directory.join("export.json"),
            directory,
        })
    }

    pub(super) fn path(&self) -> &Path {
        &self.file
    }

    pub(super) fn argument(&self) -> Result<&str, &'static str> {
        self.path().to_str().ok_or("export_temporary_path")
    }

    pub(super) fn verify(&self, expected_run: &str) -> Result<(), String> {
        verify_file(&self.file, expected_run).map_err(str::to_owned)
    }
}

impl Drop for Capture {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.file);
        // Do not recursively delete unexpected files an upstream process may have left.
        let _ = fs::remove_dir(&self.directory);
    }
}

fn verify_file(path: &Path, expected_run: &str) -> Result<(), &'static str> {
    let metadata = fs::symlink_metadata(path).map_err(|_| "export_file_unavailable")?;
    if !metadata.is_file() {
        return Err("export_file_type");
    }
    let file = File::open(path).map_err(|_| "export_file_unavailable")?;
    let metadata = file.metadata().map_err(|_| "export_file_unavailable")?;
    if !metadata.is_file() || metadata.len() > MAX_BYTES as u64 {
        return Err("export_file_bound");
    }
    verify_bytes(&read_bounded(file)?, expected_run)
}

fn read_bounded(reader: impl Read) -> Result<Vec<u8>, &'static str> {
    let mut bytes = Vec::new();
    reader
        .take(MAX_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "export_file_read")?;
    if bytes.len() > MAX_BYTES {
        return Err("export_file_bound");
    }
    Ok(bytes)
}

fn verify_bytes(bytes: &[u8], expected_run: &str) -> Result<(), &'static str> {
    if bytes.len() > MAX_BYTES {
        return Err("export_file_bound");
    }
    // Typed, closed records reject duplicate/escaped duplicate fields at every level.
    // Errors deliberately omit decoder details, values and private file locations.
    let export: shape::Export =
        serde_json::from_slice(bytes).map_err(|_| "export_json_or_shape_invalid")?;
    let run = &export.run;
    if export.schema_version != "ascension.workflow-export/v1"
        || run.schema_version != "ascension.workflow-run/v1"
    {
        return Err("export_schema");
    }
    if !export.redacted {
        return Err("export_not_redacted");
    }
    if !identifier(expected_run)
        || export.workflow_run_id != expected_run
        || run.workflow_run_id != expected_run
    {
        return Err("export_run_identity");
    }
    if !digest(&run.definition_digest) {
        return Err("export_definition_identity");
    }
    if run.run_revision == 0
        || run.run_revision > MAX_INTEGER
        || ![
            "created",
            "validated",
            "running",
            "waiting_for_provider",
            "waiting_for_game",
            "pausing",
            "paused",
            "cancelling",
            "reconciling",
            "completed",
            "failed",
            "cancelled",
            "needs_operator",
        ]
        .contains(&run.status.as_str())
        || !["not_terminal", "victory", "defeat", "unknown"].contains(&run.game_outcome.as_str())
        || ![
            "not_started",
            "pending",
            "complete",
            "failed",
            "needs_operator",
        ]
        .contains(&run.cleanup.as_str())
        || [
            run.budget.provider_calls_consumed,
            run.budget.provider_calls_reserved,
            run.budget.node_steps_consumed,
            run.budget.replans_consumed,
        ]
        .iter()
        .any(|value| *value > MAX_INTEGER)
    {
        return Err("export_run_metadata");
    }
    if export.events.is_empty() || export.events.len() > MAX_EVENTS {
        return Err("export_event_count");
    }
    let mut previous = 0;
    for event in &export.events {
        if event.schema_version != "ascension.workflow-event/v1" {
            return Err("export_schema");
        }
        if event.workflow_run_id != expected_run {
            return Err("export_run_identity");
        }
        if event.definition_digest != run.definition_digest {
            return Err("export_definition_identity");
        }
        if event.sequence <= previous || event.sequence > MAX_INTEGER {
            return Err("export_event_order");
        }
        if event.run_revision == 0
            || event.run_revision > run.run_revision
            || ![
                "run_started",
                "node_entered",
                "node_completed",
                "plan_accepted",
                "plan_rejected",
                "operation_intent",
                "operation_unknown",
                "operation_settled",
                "command_requested",
                "command_applied",
                "run_terminal",
            ]
            .contains(&event.event_type.as_str())
            || event
                .payload
                .classification
                .as_deref()
                .is_some_and(|value| {
                    !["accepted", "unknown", "settled", "rejected", "cancelled"].contains(&value)
                })
        {
            return Err("export_event_metadata");
        }
        previous = event.sequence;
    }
    Ok(())
}

fn identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.as_bytes()[0].is_ascii_alphanumeric()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._:-".contains(&byte))
}

fn digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
