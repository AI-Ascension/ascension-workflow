# Phase 1 execution report

## Current continuation — 2026-09-10

The implementation is split across an exact local harness candidate, a
separate gateway authority candidate and an MCP mapping candidate. The delivery branch contains the catalog,
contract artifact, conformance inputs, process driver, ADR set and evidence
ledgers. The exact revisions and publication limits are recorded in
[`integration/candidate-lock.json`](../../integration/candidate-lock.json) and
[`integration/source-lock-20260910.json`](../../integration/source-lock-20260910.json).

The harness candidate is `9c281717ed257476b179f186ee3d174b8a5cf0f2`, based on
refreshed default head `33437ddb18f69f68d88521d947efa3568a32a3bf`. It contains
the current owner recovery and telemetry implementation integrated with the
workflow-v1 decoder, typed definitions, canonical compiler/artifact boundary,
three-valued guards, strict reducer, bounded dynamic plan registry/runtime,
provider and budget boundaries, durable workflow event/invocation storage,
protected episode ports, authenticated loopback management API, `sts2-workflow`
CLI, and the synthetic management adapter. Served management now uses a
transactional SQLite store with a durable synthetic runtime snapshot; persisted
events carry integrity seals and offline replay rejects tampering. The management
export applies a bounded redaction projection and has sentinel coverage.

The gateway authority candidate is `1a29e425eca96eba6d88caf85c1b2dc71c4b072c`,
based on `6b6c7f2fac67de22fdf78c9fd818c6781f689ba0`. It passed its owner policy,
format, Clippy and full test suite with the new authority-route test included.
The harness and gateway candidates are not merged to their default branches.

The MCP candidate is `16ca0cb06dc93564c14963bc544bef282b38d26d`, based on
`73e777b96700917cca5ff8f6ce0f5a72009384bc`. It keeps Runtime-v2 mapping thin,
advertises the optional bounded `workflow_boot_epoch` on the three Runtime-v2
tools, forwards it as `x-sts2-workflow-boot-epoch`, and rejects malformed values
before gateway access. It passed strict policy, format, Clippy, and the full
workspace suite. This is an isolated boundary candidate; it does not prove a
live MCP-to-gateway process or owner-issued authority flow.

The following harness checks passed on the integrated candidate:

- `cargo fmt --all --check`
- `cargo run --locked --package repo-policy -- --strict` — 644 sized files, zero warnings/errors
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `cargo test --workspace --all-targets --all-features --locked` — all workspace
  targets passed, including 167 runtime tests; repository operator-only tests
  remain explicitly ignored; the SQLite management integration target passed its
  4 tests
- the Rust delivery conformance driver required `valid:true` for all 13 catalog
  definitions, rejected the missing-map capability case with a structured
  diagnostic, and exercised authenticated run/status/events, pause/resume/step,
  offline replay with tamper rejection, durable served-management restart, and the
  redacted export path

The integrated full-suite run passed after clearing stale task-generated temporary
build artifacts that had filled `/tmp` and caused two 16 MiB corruption fixtures
to report SQLite `disk full`. The final run is the evidence cited above.

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
Three fresh D1 review lanes independently held acceptance for the integrated
candidate. Their findings and the unavailable-recursion evidence are recorded in
[`review-wave-20260910.json`](review-wave-20260910.json); those reviews do not
close the required D3/G6 gate.

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
- Cross-repository MCP/protocol/watchdog conformance is not complete; the
  harness, gateway and isolated MCP mapping candidates were built and tested,
  while protocol/watchdog integration remains open.
- Full crash/fault matrix, independent adversarial review, legacy differential
  measurements, redacted telemetry integration and complete offline replay
  artifact retention still require additional evidence.
- Native compatibility and unattended live recovery remain separately gated by
  authorization and environment.

Resume from [the operations guide](../operations/RESUME.md), preserve the exact
candidate locks, and update the live requirement/task ledgers only with new
tool-derived evidence. Do not claim a default-branch merge from a local
candidate.
