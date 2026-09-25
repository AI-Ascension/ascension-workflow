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

# `artifact-inventory-tamper` above is the last case of the original suite and
# leaves the appended inventory line in place; restore it so the cases below fail
# only for their own reason. (Without this the two digest-vacuity cases below
# "pass" on a pre-fix tool simply because the inventory is still corrupt.)
cp "$root/contract-artifact/workflow-v1/SHA256SUMS" contract-artifact/workflow-v1/SHA256SUMS

# A row that is unverifiable in both directions must not count as verified: an
# emptied digest comparing "" = "" against a path sha256sum cannot read passes
# the bare `test` regardless of pipefail. Mutate the digest and its path together
# so the row drifts into that state while the artifact list stays consistent.
jq '(.artifact_digests[3].sha256) = ""
    | (.artifact_digests[3].path) = "ghost-served-live.bin"
    | (.artifacts[3]) = "ghost-served-live.bin"' \
  integration/served-live-source-lock.json > integration/served-live-source-lock.json.next
mv integration/served-live-source-lock.json.next integration/served-live-source-lock.json
expect_failure served-live-digest-vacuity bash tools/verify-ci-inputs.sh
cp "$root/integration/served-live-source-lock.json" integration/served-live-source-lock.json

# The same vacuity with the row's `sha256` key removed outright rather than
# emptied -- `jq -r '@tsv'` emits an empty field for both shapes.
jq '(.artifact_digests[3].path) = "ghost-served-live.bin"
    | (.artifact_digests[3] |= del(.sha256))
    | (.artifacts[3]) = "ghost-served-live.bin"' \
  integration/served-live-source-lock.json > integration/served-live-source-lock.json.next
mv integration/served-live-source-lock.json.next integration/served-live-source-lock.json
expect_failure served-live-digest-key-absent bash tools/verify-ci-inputs.sh
cp "$root/integration/served-live-source-lock.json" integration/served-live-source-lock.json

echo "delivery CI input regressions: PASS"
