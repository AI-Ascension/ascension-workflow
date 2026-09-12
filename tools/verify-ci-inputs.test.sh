#!/usr/bin/env bash
# Regression tests for delivery CI's failure-closed data gate.
set -euo pipefail

readonly root="$(cd "$(dirname "$0")/.." && pwd)"
readonly fixture_root="$(mktemp -d)"
trap 'rm -rf "$fixture_root"' EXIT

readonly fixture_repo="$fixture_root/repo"
mkdir "$fixture_repo"
for path in catalog conformance contract-artifact integration orchestration quality schemas workflows; do
  cp -a "$root/$path" "$fixture_repo/$path"
done
mkdir "$fixture_repo/tools"
cp "$root/tools/verify-ci-inputs.sh" "$fixture_repo/tools/verify-ci-inputs.sh"
cd "$fixture_repo"
bash tools/verify-ci-inputs.sh

expect_failure() {
  local name="$1"
  shift
  if "$@" >/dev/null 2>&1; then
    echo "negative case unexpectedly passed: $name" >&2
    exit 1
  fi
  echo "negative case rejected: $name"
}

# An unmerged candidate cannot be silently replaced by an arbitrary revision.
jq '.candidate_revisions[0].commit = "0000000000000000000000000000000000000000"' \
  integration/candidate-lock.json > integration/candidate-lock.json.next
mv integration/candidate-lock.json.next integration/candidate-lock.json
expect_failure candidate-lock-tamper bash tools/verify-ci-inputs.sh
cp "$root/integration/candidate-lock.json" integration/candidate-lock.json

jq '.accepted_contract_artifacts[0].producer_commit = "0000000000000000000000000000000000000000"' \
  integration/candidate-lock.json > integration/candidate-lock.json.next
mv integration/candidate-lock.json.next integration/candidate-lock.json
expect_failure accepted-producer-commit-tamper bash tools/verify-ci-inputs.sh
cp "$root/integration/candidate-lock.json" integration/candidate-lock.json

# The catalog declaration must retain the 13-definition process contract.
jq '.positive.expected_definitions = 12' conformance/cases/catalog-v1.json > conformance/cases/catalog-v1.json.next
mv conformance/cases/catalog-v1.json.next conformance/cases/catalog-v1.json
expect_failure catalog-count-tamper bash tools/verify-ci-inputs.sh
cp "$root/conformance/cases/catalog-v1.json" conformance/cases/catalog-v1.json

jq '.cases[0].capability_manifest = "../../outside.json"' catalog/fixtures/synthetic/stage-suite.json \
  > catalog/fixtures/synthetic/stage-suite.json.next
mv catalog/fixtures/synthetic/stage-suite.json.next catalog/fixtures/synthetic/stage-suite.json
expect_failure fixture-reference-tamper bash tools/verify-ci-inputs.sh
cp "$root/catalog/fixtures/synthetic/stage-suite.json" catalog/fixtures/synthetic/stage-suite.json

jq '.cases[1].fixture_id = .cases[0].fixture_id' catalog/fixtures/synthetic/stage-suite.json \
  > catalog/fixtures/synthetic/stage-suite.json.next
mv catalog/fixtures/synthetic/stage-suite.json.next catalog/fixtures/synthetic/stage-suite.json
expect_failure duplicate-fixture-id bash tools/verify-ci-inputs.sh
cp "$root/catalog/fixtures/synthetic/stage-suite.json" catalog/fixtures/synthetic/stage-suite.json

# The committed artifact inventory covers canonical vectors and the conformance fixture.
printf '\n' >> contract-artifact/workflow-v1/canonical-vectors.json
expect_failure canonical-vector-checksum-tamper bash tools/verify-ci-inputs.sh
cp "$root/contract-artifact/workflow-v1/canonical-vectors.json" contract-artifact/workflow-v1/canonical-vectors.json

# The inventory itself is pinned and may contain only its declared references.
printf '\n' >> contract-artifact/workflow-v1/SHA256SUMS
expect_failure artifact-inventory-tamper bash tools/verify-ci-inputs.sh

echo "delivery CI input regressions: PASS"
