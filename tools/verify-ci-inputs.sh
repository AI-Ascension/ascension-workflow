#!/usr/bin/env bash
# Verifies delivery data before the process conformance lane consumes it.
set -euo pipefail

readonly candidate_harness_commit="6e0272511997165ebba337a15d22f73acd36a4c4"
readonly refreshed_harness_commit="e70fce12afebb38503a450c8bd242bd8ad532817"
readonly workflow_schema_sha256="f8f79e115cd3bd736125ad17e33676bfe110f8a1b2673577cbbe93f1d5cc937d"
readonly workflow_artifact_inventory_sha256="bf31d36d70f9580a41577e95c6fb4e94b9cda9e3f693c303ab1701a1faaf5b84"

find catalog conformance contract-artifact integration orchestration quality schemas workflows \
  -type f -name '*.json' -print0 | xargs -0 -r -n1 jq --exit-status 'true' >/dev/null
test "$(sha256sum contract-artifact/workflow-v1/SHA256SUMS | awk '{print $1}')" = "$workflow_artifact_inventory_sha256"
mapfile -t artifact_inventory_paths < <(awk '{print $2}' contract-artifact/workflow-v1/SHA256SUMS)
test "${#artifact_inventory_paths[@]}" = 4
test "${artifact_inventory_paths[*]}" = "../../schemas/workflow-v1.schema.json ../../conformance/cases/workflow-v1.json canonical-vectors.json manifest.json"
(
  cd contract-artifact/workflow-v1
  sha256sum --strict --check SHA256SUMS
)

# Preserve the different claims in the historical candidate and refreshed-default locks.
jq --exit-status \
  --arg candidate "$candidate_harness_commit" \
  --arg digest "$workflow_schema_sha256" \
  '.status == "local_candidates_validated" and
   ([.candidate_revisions[] | select(.repository == "AI-Ascension/sts2-harness")] as $harness |
    ($harness | length) == 1 and $harness[0].commit == $candidate and
    ($harness[0].publication | contains("not merged"))) and
   .authority.harness_default_branch == "33437ddb18f69f68d88521d947efa3568a32a3bf" and
   ([.accepted_contract_artifacts[] | select(.path == "contract-artifact/workflow-v1/manifest.json")]
    | length == 1 and .[0].producer_commit == $candidate and .[0].schema_sha256 == $digest)' \
  integration/candidate-lock.json >/dev/null
jq --exit-status \
  --arg refreshed "$refreshed_harness_commit" \
  '.status == "refreshed_default_heads" and
   ([.repositories[] | select(.repository == "AI-Ascension/sts2-harness")] as $harness |
    ($harness | length) == 1 and $harness[0].default_branch == "main" and
    $harness[0].commit == $refreshed)' \
  integration/source-lock-20260911.json >/dev/null

# The accepted artifact is tied to the candidate without treating it as a default-branch lock.
test "$(sha256sum schemas/workflow-v1.schema.json | awk '{print $1}')" = "$workflow_schema_sha256"
jq --exit-status \
  --arg digest "$workflow_schema_sha256" \
  '.artifact == "sts2-harness/workflow-v1" and
   .contract_version == "ascension.workflow/v1" and
   .schema == "../../schemas/workflow-v1.schema.json" and
   .conformance == ["conformance/cases/workflow-v1.json"] and
   .canonical_vectors == ["canonical-vectors.json"] and
   (.semantic_hash.algorithm == "sha-256")' \
  contract-artifact/workflow-v1/manifest.json >/dev/null

# Keep the catalog gate explicit; semantic validation is then performed by the harness binary.
jq --exit-status \
  '.schema_version == "ascension.conformance/catalog-v1" and
   .positive.directory == "workflows/sts2" and
   .positive.expected_definitions == 13 and
   .positive.capabilities == "catalog/capabilities/synthetic-full.json" and
   (.negative | length == 1) and
   .negative[0].definition == "workflows/sts2/map.strict.json" and
   .negative[0].capabilities == "catalog/capabilities/synthetic-missing-map.json" and
   .negative[0].error_class == "capability" and
   .process_driver == "tools/conformance-driver"' conformance/cases/catalog-v1.json >/dev/null
test "$(find workflows/sts2 -maxdepth 1 -type f -name '*.json' | wc -l | tr -d ' ')" = "13"
jq --exit-status \
  '.schema_version == "ascension.sts2.catalog/v1" and
   .catalog_id == "sts2.first-party" and .workflow_schema == "ascension.workflow/v1" and
   .local_only == true and .remote_refs == [] and .scripts == false and
   .provider_credentials == false and .native_game_calls == false' catalog/registry.json >/dev/null
jq --exit-status \
  '.schema_version == "ascension.sts2.fixtures/v1" and .suite_id == "sts2.catalog.phase1" and
   .provenance.kind == "synthetic" and .provenance.claims_live_observation == false and
   .positive == ["synthetic/stage-suite.json"] and .negative == ["negative/admission-and-boundary.json"]' \
  catalog/fixtures/index.json >/dev/null
jq --exit-status \
  '.schema_version == "ascension.sts2.fixture/v1" and .suite_id == "sts2.synthetic.stage-suite" and
   .provenance.kind == "synthetic" and .provenance.claims_live_observation == false and
   (.cases | length == 14) and ([.cases[].fixture_id] | length == (unique | length)) and
   (.cases | all(.[]; (.fixture_id | test("^sts2\\.synthetic\\.[a-z0-9.]+$")) and
    (.workflow_id | test("^sts2\\.(campaign|setup|combat|map|reward|shop|event|rest|selection|recovery|terminal)\\.(strict|dynamic)$")) and
    (.capability_manifest | IN("catalog/capabilities/synthetic-full.json", "catalog/capabilities/synthetic-terminal-only.json"))))' catalog/fixtures/synthetic/stage-suite.json >/dev/null
jq --exit-status \
  '.schema_version == "ascension.sts2.fixture/v1" and .suite_id == "sts2.synthetic.negative-admission-boundary" and
   .provenance.kind == "synthetic" and .provenance.claims_live_observation == false and
   (.cases | length == 9) and ([.cases[].fixture_id] | length == (unique | length)) and
   (.cases | all(.[]; (.fixture_id | test("^sts2\\.negative\\.[a-z0-9.-]+$")) and
    (.workflow_id | test("^sts2\\.(campaign|setup|combat|map|reward|shop|event|rest|selection|recovery|terminal)\\.(strict|dynamic)$")) and
    (.capability_manifest | IN("catalog/capabilities/synthetic-full.json", "catalog/capabilities/synthetic-terminal-only.json", "catalog/capabilities/synthetic-missing-map.json", "catalog/capabilities/synthetic-baseline-no-analysis.json"))))' catalog/fixtures/negative/admission-and-boundary.json >/dev/null
for fixture in catalog/fixtures/synthetic/stage-suite.json catalog/fixtures/negative/admission-and-boundary.json; do
  while IFS=$'\t' read -r workflow capability; do
    test -f "workflows/sts2/${workflow#sts2.}.json"
    test -f "$capability"
  done < <(jq -r '.cases[] | [.workflow_id, .capability_manifest] | @tsv' "$fixture")
done

echo "delivery CI inputs: PASS"
