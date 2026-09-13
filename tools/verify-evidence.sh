#!/usr/bin/env bash
# Verifies that the current preflight and delivery ledgers preserve blocked gates.
set -euo pipefail

readonly root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

readonly preflight="docs/evidence/native-preflight-20260913.json"
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

# The fresh preflight must record the successful native hierarchy while
# remaining a bounded, non-success record until independent review exists.
# Requested, accepted and observed metadata are separate fields; unknown
# effective settings must not be silently upgraded to the requested values.
jq --exit-status '
  .schema_version == 1 and
  .evidence_id == "native-preflight-2026-09-13-rerun" and
  (.captured_at | test("^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$")) and
  .evidence_level == "confirmed" and
  .target.repository == "AI-Ascension/ascension-workflow" and
  (.target.observed_head | test("^[0-9a-f]{40}$")) and
  .root.agent_path == "/root" and
  .root.requested_model == "gpt-5.6-luna" and
  .root.requested_effort == "max" and
  .root.spawn_result == "confirmed" and
  .root.accepted.spawn_result.task_name == "/root/workflow_native_d1_20260913" and
  .root.accepted.spawn_result.nickname == "Godel" and
  .root.accepted.model == null and
  .root.accepted.effort == null and
  .root.observed.agent_path == "/root" and
  .root.observed.depth == 0 and
  .root.observed.child_status == "completed" and
  .root.observed.effective_model == "unverified" and
  .root.observed.effective_effort == "unverified" and
  .root.effective_model == "unverified" and
  .root.effective_effort == "unverified" and
  .d1.agent_path == "/root/workflow_native_d1_20260913" and
  .d1.parent_agent_path == "/root" and
  .d1.depth == 1 and
  .d1.requested.model == "gpt-5.6-luna" and
  .d1.requested.effort == "max" and
  .d1.accepted.spawn_result.task_name == "/root/workflow_native_d1_20260913/workflow_native_d2_20260913" and
  .d1.accepted.spawn_result.nickname == "Hume" and
  .d1.accepted.model == null and
  .d1.accepted.effort == null and
  .d1.observed.agent_path == "/root/workflow_native_d1_20260913" and
  .d1.observed.parent_agent_path == "/root" and
  .d1.observed.depth == 1 and
  .d1.observed.effective_model == "unverified" and
  .d1.observed.effective_effort == "unverified" and
  .d1.observed.status == "completed" and
  .d1.observed.next_depth_spawn.callable == true and
  .d1.observed.next_depth_spawn.attempted == true and
  .d1.observed.next_depth_spawn.result == "confirmed" and
  .d1.tool_surface.native_collaboration == true and
  (.d1.tool_surface.reported_namespaces | index("collaboration") != null) and
  (.d1.tool_surface.reported_tools | index("spawn_agent") != null) and
  .d1.tool_surface.reported_by_child == true and
  .d1.tool_surface.independently_inspected == false and
  .d1.d2_spawn == "confirmed" and
  .d1.d3_spawn == "delegated_by_d2" and
  .d2.agent_path == "/root/workflow_native_d1_20260913/workflow_native_d2_20260913" and
  .d2.parent_agent_path == "/root/workflow_native_d1_20260913" and
  .d2.depth == 2 and
  .d2.requested.model == "gpt-5.6-luna" and
  .d2.requested.effort == "max" and
  .d2.accepted.spawn_result.task_name == "/root/workflow_native_d1_20260913/workflow_native_d2_20260913" and
  .d2.accepted.spawn_result.nickname == "Hume" and
  .d2.accepted.model == null and
  .d2.accepted.effort == null and
  .d2.observed.agent_path == "/root/workflow_native_d1_20260913/workflow_native_d2_20260913" and
  .d2.observed.parent_agent_path == "/root/workflow_native_d1_20260913" and
  .d2.observed.depth == 2 and
  .d2.observed.effective_model == "unverified" and
  .d2.observed.effective_effort == "unverified" and
  .d2.observed.status == "completed" and
  .d2.observed.next_depth_spawn.callable == true and
  .d2.observed.next_depth_spawn.attempted == true and
  .d2.observed.next_depth_spawn.result == "confirmed" and
  .d2.tool_surface.native_collaboration == true and
  (.d2.tool_surface.reported_namespaces | index("collaboration") != null) and
  (.d2.tool_surface.reported_tools | index("spawn_agent") != null) and
  .d2.tool_surface.reported_by_child == true and
  .d2.tool_surface.independently_inspected == false and
  .d2.d3_spawn == "confirmed" and
  .d3.agent_path == "/root/workflow_native_d1_20260913/workflow_native_d2_20260913/workflow_native_d3_20260913" and
  .d3.parent_agent_path == "/root/workflow_native_d1_20260913/workflow_native_d2_20260913" and
  .d3.depth == 3 and
  .d3.requested.model == "gpt-5.6-luna" and
  .d3.requested.effort == "max" and
  .d3.accepted.spawn_result.task_name == "/root/workflow_native_d1_20260913/workflow_native_d2_20260913/workflow_native_d3_20260913" and
  .d3.accepted.spawn_result.nickname == "Gauss" and
  .d3.accepted.model == null and
  .d3.accepted.effort == null and
  .d3.observed.agent_path == "/root/workflow_native_d1_20260913/workflow_native_d2_20260913/workflow_native_d3_20260913" and
  .d3.observed.parent_agent_path == "/root/workflow_native_d1_20260913/workflow_native_d2_20260913" and
  .d3.observed.depth == 3 and
  .d3.observed.effective_model == "unverified" and
  .d3.observed.effective_effort == "unverified" and
  .d3.observed.status == "completed" and
  .d3.observed.next_depth_spawn.callable == "unverified" and
  .d3.observed.next_depth_spawn.attempted == false and
  .d3.observed.next_depth_spawn.prohibited == true and
  .d3.observed.next_depth_spawn.result == "not_attempted" and
  .d3.tool_surface.native_collaboration == true and
  (.d3.tool_surface.reported_namespaces | index("collaboration") != null) and
  (.d3.tool_surface.reported_tools | index("spawn_agent") != null) and
  .d3.tool_surface.reported_by_child == true and
  .d3.tool_surface.independently_inspected == false and
  .d3.d4_spawn == "prohibited_not_attempted" and
  .hierarchy.path == [
    "/root",
    "/root/workflow_native_d1_20260913",
    "/root/workflow_native_d1_20260913/workflow_native_d2_20260913",
    "/root/workflow_native_d1_20260913/workflow_native_d2_20260913/workflow_native_d3_20260913"
  ] and
  .hierarchy.depths == [0, 1, 2, 3] and
  [.d1.observed.agent_path, .d2.observed.agent_path, .d3.observed.agent_path] == .hierarchy.path[1:] and
  [.root.observed.depth, .d1.observed.depth, .d2.observed.depth, .d3.observed.depth] == .hierarchy.depths and
  .hierarchy.peak_observed_live_descendants == 3 and
  .hierarchy.global_descendant_ceiling == 12 and
  .hierarchy.d4_spawned == false and
  .outcome.hierarchy_compliant == true and
  .outcome.independent_d3_review == false and
  .outcome.source_writes == false and
  .outcome.external_agent_processes == false and
  .outcome.status == "blocked" and
  ([.blockers[] | select(.id == "B-001") | .status] == ["resolved"]) and
  ([.blockers[] | select(.id == "B-005") | .status] == ["active"]) and
  ([.blockers[].id] | sort) == ["B-001", "B-005"] and
  (.blockers | all(.[]; (.status | IN("active", "resolved")) and (.reason | length > 0))) and
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
  ([.blockers[] | select(.id == "B-001") | .status] == ["resolved"]) and
  ([.blockers[] | select(.id == "B-005") | .status] == ["active_review_gate"])
' "$execution_state" >/dev/null
jq --exit-status '
  .observed_hierarchy.native_d1_d2_d3 == true and
  .observed_hierarchy.independent_d3 == false and
  .observed_hierarchy.subprocess_substitute == false and
  .observed_hierarchy.peak_observed_live_descendants == 3
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
# fresh native preflight. Hierarchy evidence may move them to verification, but
# none may be promoted to completion while B-005 remains active.
jq --exit-status '
  ([.requirements[] | select(.id == "WF-001")] | length) == 1 and
  ([.requirements[] | select(.id == "WF-001") |
    .status == "verification" and
    (.blocker_refs | sort) == ["B-005"] and
    .implementation_status == "native_hierarchy_verified_pending_independent_review"] | all) and
  ([.requirements[] | select(.id == "WF-002")] | length) == 1 and
  ([.requirements[] | select(.id == "WF-002") |
    .status == "verification" and
    (.blocker_refs | sort) == ["B-005"] and
    .implementation_status == "native_settings_explicitly_qualified_pending_independent_review"] | all) and
  ([.requirements[] | select(.id == "WF-003")] | length) == 1 and
  ([.requirements[] | select(.id == "WF-003") |
    .status == "verification" and
    (.blocker_refs | sort) == ["B-005"] and
    .implementation_status == "scope_recorded_pending_independent_review"] | all)
' "$requirements" >/dev/null

echo "delivery evidence gate: PASS (native/review blockers preserved)"
