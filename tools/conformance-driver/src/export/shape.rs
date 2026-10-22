// SPDX-License-Identifier: MIT

//! Read-only export projection for the existing pinned Harness management contract.
//! These consumer records contain no workflow interpretation or mutation authority.

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Export {
    pub schema_version: String,
    pub workflow_run_id: String,
    pub redacted: bool,
    pub run: Run,
    pub events: Vec<Event>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Run {
    pub schema_version: String,
    pub workflow_run_id: String,
    pub definition_digest: String,
    pub run_revision: u64,
    pub status: String,
    pub game_outcome: String,
    #[serde(rename = "cursor")]
    _cursor: Cursor,
    // Required JSON null: an object, string, or omitted field is not redaction.
    #[serde(rename = "pending_operation")]
    _pending_operation: (),
    pub budget: Budget,
    pub cleanup: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Cursor {
    #[serde(rename = "graph_id")]
    _graph_id: Redacted,
    #[serde(rename = "node_id")]
    _node_id: Redacted,
    #[serde(rename = "node_execution_id")]
    _node_execution_id: Redacted,
}

#[derive(Deserialize)]
#[serde(try_from = "String")]
struct Redacted;

impl TryFrom<String> for Redacted {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "[redacted]" {
            Ok(Self)
        } else {
            Err("expected redaction marker")
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Budget {
    pub provider_calls_consumed: u64,
    pub provider_calls_reserved: u64,
    pub node_steps_consumed: u64,
    pub replans_consumed: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Event {
    pub schema_version: String,
    pub workflow_run_id: String,
    pub sequence: u64,
    pub run_revision: u64,
    pub event_type: String,
    pub definition_digest: String,
    #[serde(rename = "node_execution_id")]
    _node_execution_id: Redacted,
    pub payload: Payload,
    // The producer omits this redacted field; explicit null is also harmless.
    #[serde(default, rename = "integrity_digest")]
    _integrity_digest: (),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Payload {
    #[serde(rename = "operation_id")]
    _operation_id: (),
    #[serde(deserialize_with = "Option::deserialize")]
    pub classification: Option<String>,
    #[serde(rename = "reason_code")]
    _reason_code: RedactedReason,
}

#[derive(Deserialize)]
#[serde(try_from = "String")]
struct RedactedReason;

impl TryFrom<String> for RedactedReason {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "event" {
            Ok(Self)
        } else {
            Err("expected redacted event reason")
        }
    }
}
