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
  .d2.accepted.spawn_result.task_name == "/root/workflow_native_d1_20260913/workflow_native_d2_20260913/workflow_native_d3_20260913" and
  .d2.accepted.spawn_result.nickname == "Gauss" and
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
  .root.accepted.spawn_result.task_name == .d1.agent_path and
  .d1.accepted.spawn_result.task_name == .d2.agent_path and
  .d2.accepted.spawn_result.task_name == .d3.agent_path and
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

# Validate the dated overlay against retained historical evidence, without
# claiming that absent capture bytes or old source-file hashes were rechecked.
readonly overlay="docs/evidence/studio-acceptance-current-state-20261009.json"
readonly matrix="docs/evidence/studio-acceptance-matrix-20260913.json"
jq --exit-status --slurpfile historical "$matrix" '
  def child_ref: .repository + "#" + (.issue | tostring);
  def state_counts: {total:length, open:map(select(.snapshot_issue_state == "open"))|length,
    closed:map(select(.snapshot_issue_state == "closed"))|length, missing:map(select(.issue_record_found != true))|length};
  def pins($count): length == $count and ([.[].path] | length == (unique|length)) and
    all(.[]; (.sha256|test("^[0-9a-f]{64}$")) and (.path|test("^[a-zA-Z0-9_.-]+(/[a-zA-Z0-9_.-]+)*$")) and
      (.path|split("/")|index("..") == null));
  $historical[0] as $m | .feature_issue_state as $f | .backend_child_state as $c |
  .current_dependency_graph as $g | ($f + $c) as $rows |
  .schema_version == "ascension.workflow.studio-acceptance-current-state/v1" and
  .evidence_id == "studio-acceptance-current-state-20261009" and
  .source_issue == ($m.source_issue.repository + "#" + ($m.source_issue.number|tostring)) and
  (.provenance | [.is_live_github_observation,.source_or_issue_mutation,.runtime_test_or_execution,
    .acceptance_or_issue_closure] | all(.[]; . == false)) and
  (.provenance.review_verification | .capture_inputs_retained == false and .capture_input_hashes_recomputed == false and
    .snapshot_claims == "author_recorded_unverified" and .snapshot_claim_scope ==
      ["input_hashes","coverage","issue_metadata","graph_to_rest_joins"] and (.limitation|length > 0)) and
  (.provenance.input_hashes|pins(19)) and (.source_revision.source_file_hashes|pins(16)) and
  ($f|length == 22) and ($c|length == 21) and
  ([$f[].row_id]|length == (unique|length)) and ([$c[]|child_ref]|length == (unique|length)) and
  ([$f[]|[.row_id,.feature_issue,.matrix_issue_state,.matrix_outcome_preserved,.remaining_acceptance_gates_at_matrix]]|sort) ==
    ([$m.features[]|[.row_id,.feature_issue,.issue_state,.outcome,.remaining_acceptance_gates]]|sort) and
  ([$c[]|[child_ref,.matrix_issue_state,.historical_matrix_acceptance_state,.historical_required_pr]]|sort) ==
    ([$m.backend_children[]|[child_ref,.state,.acceptance_state,.required_pr]]|sort) and
  ($f|all(.[]; .issue_url == ("https://github.com/" + (.feature_issue|sub("#";"/issues/"))) and
    .acceptance_promoted_by_overlay == false)) and
  ($c|all(.[]; .issue_url == ("https://github.com/" + .repository + "/issues/" + (.issue|tostring)) and
    .aggregate_acceptance_promoted == false and .closed_issue_state_is_component_disposition_only == (.snapshot_issue_state == "closed") and
    .issue_state_change == (if .matrix_issue_state == .snapshot_issue_state then "same_as_matrix" else "changed_since_matrix" end))) and
  ($rows|all(.[]; .issue_record_found == true and (.snapshot_issue_state|IN("open","closed")))) and
  .counts.features == ($f|state_counts) and
  (.counts.backend_children|{total,open,closed,missing}) == ($c|state_counts) and
  .counts.backend_children.changed_since_matrix == ([$c[]|select(.matrix_issue_state != .snapshot_issue_state)]|length) and
  (.counts.backend_children.newly_closed_refs|sort) == ([$c[]|select(.matrix_issue_state == "open" and .snapshot_issue_state == "closed")|child_ref]|sort) and
  (.counts.backend_children.newly_closed_refs|length) == 8 and
  (.counts.backend_children.already_closed_in_matrix_refs|sort) == ([$c[]|select(.matrix_issue_state == "closed")|child_ref]|sort) and
  (.counts.backend_children.already_closed_in_matrix_refs|length) == 2 and
  [.outcome_preservation.blocked,.outcome_preservation.partial_component_only,.outcome_preservation.complete] == [7,15,0] and
  .outcome_preservation.blocked == ([$f[]|select(.matrix_outcome_preserved == "blocked")]|length) and
  .outcome_preservation.partial_component_only == ([$f[]|select(.matrix_outcome_preserved == "partial_component_only")]|length) and
  .outcome_preservation.complete == ([$f[]|select(.matrix_outcome_preserved == "complete")]|length) and
  .outcome_preservation.backend_children_open_or_unverified == $m.matrix_result.backend_children_open_or_unverified and
  .historical_baseline.matrix_result == $m.matrix_result and .evidence_lanes_preserved == $m.evidence_lanes and
  [.matrix_gates_at_capture[]|{id,status:.status_at_matrix_capture,scope,precondition}] == $m.external_gates and
  $g.blocked_by_total == 35 and $g.returned_rows == ($g.edges|length) and $g.returned_rows == 35 and
  ([$g.edges[].issue_url]|length == (unique|length)) and $g.connection_complete == true and
  $g.open_count == ([$g.edges[]|select(.rest_issue_state == "open")]|length) and
  $g.closed_count == ([$g.edges[]|select(.rest_issue_state == "closed")]|length) and
  $g.open_count + $g.closed_count == 35 and $g.missing_rest_records == 0 and $g.graph_rest_state_mismatches == 0 and
  ($g.edges|all(.[]; . as $edge | .state_matches_rest_record == true and .rest_record_found == true and
    .state_in_captured_graph == (.rest_issue_state|ascii_upcase) and
    .issue_url == ("https://github.com/" + .repository + "/issues/" + (.issue|tostring)) and
    ([$rows[]|select(.issue_url == $edge.issue_url)|.snapshot_issue_state] == [$edge.rest_issue_state]))) and
  ($g.unresolved_dependency_urls|sort) == ([$g.edges[]|select(.rest_issue_state == "open")|.issue_url]|sort) and
  .issue_11.state == "open" and .parent_issue_state.state == "open" and .parent_issue_state.state_reason == "reopened" and
  .deferred_122.snapshot_state == "open" and .deferred_122.matrix_disposition ==
    "deliberately_deferred_until_this_matrix_and_all_predecessors_are_accepted" and
  $g.blocking_count == 1 and $g.blocking_rows == [{number:122,url:.deferred_122.issue_url,state:"open"}] and
  .acceptance_boundary == {issue_11_open_reopened:true,features_complete:0,real_process_cross_repository:"unverified",
    provider:"unverified",native_game:"unverified",independent_d3:"unverified",authored_journey_executed:false,
    new_process_provider_native_or_browser_check_run:false,this_overlay_started_122:false,source_contract_or_schema_changed:false,remote_mutation:false} and
  .historical_baseline.matrix_path == "docs/evidence/studio-acceptance-matrix-20260913.json" and
  .historical_baseline.reconciliation_path == "docs/evidence/studio-acceptance-reconciliation-20260915.json" and
  .historical_baseline.journey_path == "docs/evidence/studio-acceptance-journey-20260913.json"
' "$overlay" >/dev/null
for artifact in matrix reconciliation journey; do
  path=$(jq -r --arg kind "$artifact" '.historical_baseline[$kind + "_path"]' "$overlay")
  expected=$(jq -r --arg kind "$artifact" '.historical_baseline[$kind + "_sha256"]' "$overlay")
  [[ "$expected" =~ ^[0-9a-f]{64}$ ]]
  test "$(sha256sum "$path" | awk '{print $1}')" = "$expected"
done

echo "delivery evidence gate: PASS (historical limits and native/review blockers preserved)"
