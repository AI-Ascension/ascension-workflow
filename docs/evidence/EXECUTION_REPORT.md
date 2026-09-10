# Phase 1 execution report

## Current continuation — 2026-09-10

The implementation is split across an exact local harness candidate and a
separate gateway authority candidate. The delivery branch contains the catalog,
contract artifact, conformance inputs, process driver, ADR set and evidence
ledgers. The exact revisions and publication limits are recorded in
[`integration/candidate-lock.json`](../../integration/candidate-lock.json) and
[`integration/source-lock-20260910.json`](../../integration/source-lock-20260910.json).

The harness candidate is `623848ed6dc8b2bbfa07cd287df7aa4feba18c87`, based on
default head `68e4f935f251c5e20d07b929c6b1c096d0b7b183`. It contains the
workflow-v1 decoder, typed definitions, canonical compiler/artifact boundary,
three-valued guards, strict reducer, bounded dynamic plan registry/runtime,
provider and budget boundaries, durable workflow event/invocation storage,
protected episode ports, authenticated loopback management API, `sts2-workflow`
CLI, and a synthetic management adapter that invokes the shared runtime.

The gateway authority candidate is `e5543403e7ac0fa13929c296adfab877bea4212a`,
based on `6b6c7f2fac67de22fdf78c9fd818c6781f689ba0`. It passed its owner policy,
format, Clippy and full test suite with 147 passing tests. Neither candidate is
merged to its default branch.

The following harness checks passed on the integrated candidate:

- `cargo fmt --all --check`
- `cargo run -p repo-policy -- --strict` — 560 sized files, zero warnings/errors
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `cargo test --workspace --all-targets --all-features --locked` — 128 runtime
  tests passed in the final successful run; repository operator-only tests remain
  explicitly ignored
- the Rust delivery conformance driver validated all 13 catalog definitions,
  rejected the missing-map capability case with a structured diagnostic, and
  exercised authenticated run/status/events, pause/resume/step and offline replay

One full-suite attempt had a transient existing `MCP process failed to start`
failure in an operator lifecycle test. The exact test passed on immediate rerun;
the final full-suite run is the evidence cited above. No code change was made for
that environment race.

The implementation currently proves deterministic synthetic source/component
behavior. The default synthetic management profile has no provider or game
authority and produces only bounded fixture outcomes through the shared runtime.
Live profiles remain capability- and scope-gated. Native game compatibility,
provider expenditure, host restart, deployment, installation, merge and release
were not performed.

## Archive, standards and orchestration

The package inventory, SHA-256, JSON/TOML/DAG/traceability/link checks and its 12
unit tests passed. Optional Python `jsonschema` validation remained skipped because
that development dependency was absent; it was not installed. The package is
retained byte-for-byte under `prompts/phase1/`.

Rust 1.97.1, rustfmt and Clippy are available from the workspace-local toolchain.
The delivery standards profile is recorded in `standards-profile.toml`; the
delivery driver invokes the built harness binary and contains no graph
interpreter, scheduler, sibling-source dependency or native game call.

The requested D0→D1→D2→D3 native hierarchy was not observed. The D1 workers used
`gpt-5.6-luna` with `max` effort as requested, but their environments exposed no
callable native collaboration tools, so D2/D3 could not be created. No flat or
subprocess substitute was used. This is an orchestration evidence limitation,
separate from the component tests above; it prevents a complete Phase 1 claim.

Scoped branches, commits, pushes, private repository bootstrap, issue and draft
PR preparation were authorized by the launch package. No merge, release,
deployment, installation, remote exposure, native game launch or live provider
call occurred.

## Historical preparation checkpoint

The earlier 2026-09-09 checkpoint was blocked before product implementation. Its
source locks, preflight attestations and handoff remain in Git history and the
original ledgers. They are historical evidence only; the current continuation
above supersedes its “not implemented” product table and missing-toolchain note.

## Remaining gates

- D2/D3 independent review and exact hierarchy evidence remain unavailable.
- Cross-repository MCP/protocol/watchdog conformance is not complete; only the
  harness and gateway candidates were built and tested in this continuation.
- Full crash/fault matrix, independent adversarial review, legacy differential
  measurements, redacted telemetry integration and complete offline replay
  artifact retention still require additional evidence.
- Native compatibility and unattended live recovery remain separately gated by
  authorization and environment.

Resume from [the operations guide](../operations/RESUME.md), preserve the exact
candidate locks, and update the live requirement/task ledgers only with new
tool-derived evidence. Do not claim a default-branch merge from a local
candidate.
