#!/usr/bin/env bash
# Regression tests for the failure-closed evidence gate.
set -euo pipefail

readonly root="$(cd "$(dirname "$0")/.." && pwd)"
readonly fixture_root="$(mktemp -d)"
trap 'rm -rf "$fixture_root"' EXIT

mkdir -p "$fixture_root/docs/evidence" "$fixture_root/orchestration" \
  "$fixture_root/quality" "$fixture_root/tools"
cp "$root/docs/evidence/native-preflight-20260913.json" "$fixture_root/docs/evidence/"
for evidence in studio-acceptance-current-state-20261009 studio-acceptance-matrix-20260913 \
  studio-acceptance-reconciliation-20260915 studio-acceptance-journey-20260913; do
  cp "$root/docs/evidence/$evidence.json" "$fixture_root/docs/evidence/"
done
cp "$root/execution-state.json" "$fixture_root/"
cp "$root/orchestration/current-run.json" "$fixture_root/orchestration/"
cp "$root/orchestration/tasks.json" "$fixture_root/orchestration/"
cp "$root/quality/requirements.json" "$fixture_root/quality/"
cp "$root/tools/verify-evidence.sh" "$fixture_root/tools/"

(
  cd "$fixture_root"
  bash tools/verify-evidence.sh
)

expect_failure() {
  local name="$1"
  shift
  if "$@" >/dev/null 2>&1; then
    echo "negative case unexpectedly passed: $name" >&2
    exit 1
  fi
  echo "negative case rejected: $name"
}

jq '.outcome.status = "complete"' \
  "$fixture_root/docs/evidence/native-preflight-20260913.json" \
  > "$fixture_root/docs/evidence/native-preflight-20260913.json.next"
mv "$fixture_root/docs/evidence/native-preflight-20260913.json.next" \
  "$fixture_root/docs/evidence/native-preflight-20260913.json"
expect_failure preflight-success-claim bash "$fixture_root/tools/verify-evidence.sh"
cp "$root/docs/evidence/native-preflight-20260913.json" \
  "$fixture_root/docs/evidence/native-preflight-20260913.json"

jq '.outcome.hierarchy_compliant = false' \
  "$fixture_root/docs/evidence/native-preflight-20260913.json" \
  > "$fixture_root/docs/evidence/native-preflight-20260913.json.next"
mv "$fixture_root/docs/evidence/native-preflight-20260913.json.next" \
  "$fixture_root/docs/evidence/native-preflight-20260913.json"
expect_failure hierarchy-regression bash "$fixture_root/tools/verify-evidence.sh"
cp "$root/docs/evidence/native-preflight-20260913.json" \
  "$fixture_root/docs/evidence/native-preflight-20260913.json"

jq '.blockers |= map(if .id == "B-001" then .status = "active" else . end)' \
  "$fixture_root/docs/evidence/native-preflight-20260913.json" \
  > "$fixture_root/docs/evidence/native-preflight-20260913.json.next"
mv "$fixture_root/docs/evidence/native-preflight-20260913.json.next" \
  "$fixture_root/docs/evidence/native-preflight-20260913.json"
expect_failure stale-orchestration-blocker bash "$fixture_root/tools/verify-evidence.sh"
cp "$root/docs/evidence/native-preflight-20260913.json" \
  "$fixture_root/docs/evidence/native-preflight-20260913.json"

jq 'del(.d1.observed.parent_agent_path)' \
  "$fixture_root/docs/evidence/native-preflight-20260913.json" \
  > "$fixture_root/docs/evidence/native-preflight-20260913.json.next"
mv "$fixture_root/docs/evidence/native-preflight-20260913.json.next" \
  "$fixture_root/docs/evidence/native-preflight-20260913.json"
expect_failure missing-observed-parentage bash "$fixture_root/tools/verify-evidence.sh"
cp "$root/docs/evidence/native-preflight-20260913.json" \
  "$fixture_root/docs/evidence/native-preflight-20260913.json"

jq '.d2.observed.parent_agent_path = "/root/foreign"' \
  "$fixture_root/docs/evidence/native-preflight-20260913.json" \
  > "$fixture_root/docs/evidence/native-preflight-20260913.json.next"
mv "$fixture_root/docs/evidence/native-preflight-20260913.json.next" \
  "$fixture_root/docs/evidence/native-preflight-20260913.json"
expect_failure contradictory-observed-parentage bash "$fixture_root/tools/verify-evidence.sh"
cp "$root/docs/evidence/native-preflight-20260913.json" \
  "$fixture_root/docs/evidence/native-preflight-20260913.json"

jq '.d2.accepted.spawn_result.task_name = "/root/workflow_native_d1_20260913/workflow_native_d2_20260913" | .d2.accepted.spawn_result.nickname = "Hume"' \
  "$fixture_root/docs/evidence/native-preflight-20260913.json" \
  > "$fixture_root/docs/evidence/native-preflight-20260913.json.next"
mv "$fixture_root/docs/evidence/native-preflight-20260913.json.next" \
  "$fixture_root/docs/evidence/native-preflight-20260913.json"
expect_failure mismatched-accepted-spawn bash "$fixture_root/tools/verify-evidence.sh"
cp "$root/docs/evidence/native-preflight-20260913.json" \
  "$fixture_root/docs/evidence/native-preflight-20260913.json"

jq '.requirements |= map(select(.id != "WF-001"))' \
  "$fixture_root/quality/requirements.json" \
  > "$fixture_root/quality/requirements.json.next"
mv "$fixture_root/quality/requirements.json.next" \
  "$fixture_root/quality/requirements.json"
expect_failure requirement-deletion bash "$fixture_root/tools/verify-evidence.sh"
cp "$root/quality/requirements.json" \
  "$fixture_root/quality/requirements.json"

jq '.requirements |= map(if .id == "WF-001" then .id = "WF-001-renamed" else . end)' \
  "$fixture_root/quality/requirements.json" \
  > "$fixture_root/quality/requirements.json.next"
mv "$fixture_root/quality/requirements.json.next" \
  "$fixture_root/quality/requirements.json"
expect_failure requirement-rename bash "$fixture_root/tools/verify-evidence.sh"
cp "$root/quality/requirements.json" \
  "$fixture_root/quality/requirements.json"

jq '.requirements |= map(if .id == "WF-001" then .status = "complete" else . end)' \
  "$fixture_root/quality/requirements.json" \
  > "$fixture_root/quality/requirements.json.next"
mv "$fixture_root/quality/requirements.json.next" \
  "$fixture_root/quality/requirements.json"
expect_failure requirement-promotion bash "$fixture_root/tools/verify-evidence.sh"
cp "$root/quality/requirements.json" "$fixture_root/quality/requirements.json"

# Each overlay mutation starts from the same valid record. Historical source
# pins are not hashes of today's edited validator scripts.
readonly overlay="docs/evidence/studio-acceptance-current-state-20261009.json"
reject_overlay() {
  local name="$1" filter="$2"
  jq "$filter" "$root/$overlay" > "$fixture_root/$overlay"
  expect_failure "$name" bash "$fixture_root/tools/verify-evidence.sh"
  cp "$root/$overlay" "$fixture_root/$overlay"
}
reject_overlay feature-count '.counts.features.total = 21'
reject_overlay duplicate-feature '.feature_issue_state[1] = .feature_issue_state[0]'
reject_overlay unmapped-feature '.feature_issue_state[0].feature_issue = "AI-Ascension/sts2-harness#999"'
reject_overlay changed-outcome '.feature_issue_state[0].matrix_outcome_preserved = "complete"'
reject_overlay removed-gate '.feature_issue_state[0].remaining_acceptance_gates_at_matrix = []'
reject_overlay acceptance-promotion '.feature_issue_state[0].acceptance_promoted_by_overlay = true'
reject_overlay duplicate-child '.backend_child_state[1] = .backend_child_state[0]'
reject_overlay child-closure-credit '.backend_child_state[0].closed_issue_state_is_component_disposition_only = false'
reject_overlay child-state-change '.backend_child_state[0].issue_state_change = "same_as_matrix"'
reject_overlay newly-closed-list '.counts.backend_children.newly_closed_refs |= .[1:]'
reject_overlay changed-outcome-count '.outcome_preservation.partial_component_only = 16'
reject_overlay changed-historical-gate '.matrix_gates_at_capture[0].status_at_matrix_capture = "complete"'
reject_overlay duplicate-graph-edge '.current_dependency_graph.edges[1] = .current_dependency_graph.edges[0]'
reject_overlay unmapped-graph-edge '.current_dependency_graph.edges[0].issue_url = "https://github.com/AI-Ascension/sts2-harness/issues/999"'
reject_overlay graph-state-mismatch '.current_dependency_graph.edges[0].rest_issue_state = "closed"'
reject_overlay graph-count '.current_dependency_graph.open_count = 30'
reject_overlay graph-open-set '.current_dependency_graph.unresolved_dependency_urls |= .[1:]'
reject_overlay closed-parent '.parent_issue_state.state = "closed"'
reject_overlay started-deferred-work '.acceptance_boundary.this_overlay_started_122 = true'
reject_overlay upgraded-evidence-lane '.evidence_lanes_preserved.provider.status = "pass"'
reject_overlay missing-provenance 'del(.provenance.review_verification)'
reject_overlay invented-capture-verification '.provenance.review_verification.capture_input_hashes_recomputed = true'
reject_overlay malformed-input-hash '.provenance.input_hashes[0].sha256 = ""'
reject_overlay duplicate-source-pin '.source_revision.source_file_hashes[1] = .source_revision.source_file_hashes[0]'
reject_overlay escaping-source-pin '.source_revision.source_file_hashes[0].path = "../outside.json"'
reject_overlay missing-history-target '.historical_baseline.matrix_path = "docs/evidence/missing.json"'
reject_overlay absent-history-digest 'del(.historical_baseline.matrix_sha256)'
printf '\n' >> "$fixture_root/docs/evidence/studio-acceptance-matrix-20260913.json"
expect_failure historical-artifact-byte-drift bash "$fixture_root/tools/verify-evidence.sh"
cp "$root/docs/evidence/studio-acceptance-matrix-20260913.json" "$fixture_root/docs/evidence/"
bash "$fixture_root/tools/verify-evidence.sh"

echo "delivery evidence regressions: PASS"
