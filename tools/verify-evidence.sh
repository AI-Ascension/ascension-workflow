#!/usr/bin/env bash
# Verifies that the current preflight and delivery ledgers preserve blocked gates.
set -euo pipefail

readonly root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

readonly preflight="docs/evidence/native-preflight.json"
readonly execution_state="execution-state.json"
readonly current_run="orchestration/current-run.json"
readonly tasks="orchestration/tasks.json"
readonly requirements="quality/requirements.json"

test -f "$preflight"
test -f "$execution_state"
test -f "$current_run"
test -f "$tasks"
test -f "$requirements"

# Parse every tracked JSON input before making semantic assertions. jq's parser
# is deliberately used here so malformed evidence cannot be treated as absent.
for json_root in catalog conformance contract-artifact docs/evidence integration orchestration quality schemas workflows; do
  if test -d "$json_root"; then
    find "$json_root" -type f -name '*.json' -print0 |
      xargs -0 -r -n1 jq --exit-status 'true' >/dev/null
  fi
done
jq --exit-status 'true' "$execution_state" "$current_run" "$tasks" "$requirements" >/dev/null

# The fresh preflight must remain a bounded, non-success record. Requested,
# effective and observed metadata are separate fields; unknown effective
# settings must not be silently upgraded to the requested values.
jq --exit-status '
  .schema_version == 1 and
  (.evidence_id | test("^native-preflight-[0-9]{4}-[0-9]{2}-[0-9]{2}$")) and
  (.captured_at | test("^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$")) and
  .evidence_level == "confirmed" and
  .target.repository == "AI-Ascension/ascension-workflow" and
  (.target.observed_head | test("^[0-9a-f]{40}$")) and
  .root.requested_model == "gpt-5.6-luna" and
  .root.requested_effort == "max" and
  .root.spawn_result == "confirmed" and
  .root.effective_model == "unverified" and
  .root.effective_effort == "unverified" and
  .d1.depth == 1 and
  .d1.tool_surface.native_collaboration == false and
  .d1.tool_surface.reported_by_child == true and
  .d1.tool_surface.independently_inspected == false and
  .d1.d2_spawn == "not_callable" and
  .d1.d3_spawn == "not_attempted" and
  .outcome.hierarchy_compliant == false and
  .outcome.independent_d3_review == false and
  .outcome.source_writes == false and
  .outcome.external_agent_processes == false and
  .outcome.status == "blocked" and
  ([.blockers[].id] | sort) == ["B-001", "B-005"] and
  (.blockers | all(.[]; .status == "active" and (.reason | length > 0))) and
  .privacy.contains_credentials == false and
  .privacy.contains_private_prompts == false and
  .privacy.contains_raw_provider_output == false and
  .privacy.contains_proprietary_game_data == false
' "$preflight" >/dev/null

# The execution state and current run must agree that the native hierarchy and
# independent review are unavailable. This prevents a green component check
# from becoming an end-to-end completion claim.
jq --exit-status '
  .status == "integrated_candidate_validation" and
  ([.blockers[] | select(.id == "B-001") | .status] == ["active_orchestration_blocker"]) and
  ([.blockers[] | select(.id == "B-005") | .status] == ["active_review_gate"])
' "$execution_state" >/dev/null
jq --exit-status '
  .observed_hierarchy.native_d1_d2_d3 == false and
  .observed_hierarchy.independent_d3 == false and
  .observed_hierarchy.subprocess_substitute == false
' "$current_run" >/dev/null

# Every blocked item needs a named blocker and a resumable next action. Keep
# statuses bounded until a future evidence update explicitly changes this
# verifier and the corresponding ledgers together.
jq --exit-status '
  .schema_version == 1 and
  ([.tasks[].id] | length == (unique | length)) and
  ([.tasks[].state] | all(.[]; IN("blocked", "in_progress", "verification"))) and
  ([.tasks[] | select(.state == "blocked") |
    ((.blocker_refs | length > 0) and (.next_safe_action | type == "string") and
     (.next_safe_action | length > 0))] | all)
' "$tasks" >/dev/null
jq --exit-status '
  .schema_version == 1 and
  ([.requirements[].id] | length == (unique | length)) and
  ([.requirements[].status] | all(.[]; IN("blocked", "in_progress", "verification"))) and
  ([.requirements[] | select(.status == "blocked") |
    ((.blocker_refs | length > 0) and (.implementation_status == "blocked_pending_required_gate") and
     (.next_safe_action | type == "string") and (.next_safe_action | length > 0))] | all)
' "$requirements" >/dev/null

# These three requirements are the direct machine-readable projection of the
# fresh D1 preflight. They must not be promoted independently of B-001/B-005.
jq --exit-status '
  ([.requirements[] | select(.id == "WF-001")] | length) == 1 and
  ([.requirements[] | select(.id == "WF-001") |
    .status == "blocked" and (.blocker_refs | sort) == ["B-001", "B-005"]] | all) and
  ([.requirements[] | select(.id == "WF-002")] | length) == 1 and
  ([.requirements[] | select(.id == "WF-002") |
    .status == "blocked" and (.blocker_refs | sort) == ["B-001", "B-005"]] | all) and
  ([.requirements[] | select(.id == "WF-003")] | length) == 1 and
  ([.requirements[] | select(.id == "WF-003") |
    .status == "blocked" and (.blocker_refs | sort) == ["B-001", "B-005"]] | all)
' "$requirements" >/dev/null

echo "delivery evidence gate: PASS (native/review blockers preserved)"
