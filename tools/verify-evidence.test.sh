#!/usr/bin/env bash
# Regression tests for the failure-closed evidence gate.
set -euo pipefail

readonly root="$(cd "$(dirname "$0")/.." && pwd)"
readonly fixture_root="$(mktemp -d)"
trap 'rm -rf "$fixture_root"' EXIT

mkdir -p "$fixture_root/docs/evidence" "$fixture_root/orchestration" \
  "$fixture_root/quality" "$fixture_root/tools"
cp "$root/docs/evidence/native-preflight.json" "$fixture_root/docs/evidence/"
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
  "$fixture_root/docs/evidence/native-preflight.json" \
  > "$fixture_root/docs/evidence/native-preflight.json.next"
mv "$fixture_root/docs/evidence/native-preflight.json.next" \
  "$fixture_root/docs/evidence/native-preflight.json"
expect_failure preflight-success-claim bash "$fixture_root/tools/verify-evidence.sh"
cp "$root/docs/evidence/native-preflight.json" \
  "$fixture_root/docs/evidence/native-preflight.json"

jq '.d1.tool_surface.native_collaboration = true' \
  "$fixture_root/docs/evidence/native-preflight.json" \
  > "$fixture_root/docs/evidence/native-preflight.json.next"
mv "$fixture_root/docs/evidence/native-preflight.json.next" \
  "$fixture_root/docs/evidence/native-preflight.json"
expect_failure native-tool-claim bash "$fixture_root/tools/verify-evidence.sh"
cp "$root/docs/evidence/native-preflight.json" \
  "$fixture_root/docs/evidence/native-preflight.json"

jq '.requirements |= map(if .id == "WF-001" then .status = "verification" else . end)' \
  "$fixture_root/quality/requirements.json" \
  > "$fixture_root/quality/requirements.json.next"
mv "$fixture_root/quality/requirements.json.next" \
  "$fixture_root/quality/requirements.json"
expect_failure requirement-promotion bash "$fixture_root/tools/verify-evidence.sh"

echo "delivery evidence regressions: PASS"
