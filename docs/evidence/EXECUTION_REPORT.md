# Phase 1 execution report

## Studio acceptance matrix — 2026-09-13

Issue #11 is recorded as a blocked, additive delivery gate in
[`studio-acceptance-matrix-20260913.json`](studio-acceptance-matrix-20260913.json).
The matrix covers all 22 Studio predecessor features and all 21 required
backend children, with issue state, owner-child mapping, required companion PR
disposition, exact merged head/merge pins, contract versions, repository
commands and remaining gates. The deliberately deferred Studio #122 feature is
downstream of this matrix and was not started.

The current default-branch snapshot and copied-artifact digests are locked in
[`integration/source-lock-20260913.json`](../../integration/source-lock-20260913.json).
The context association, binding-catalog and provider-session projection
manifests now carry their producer revision and schema/conformance digests;
each has a checked-in `SHA256SUMS`. The catalog registry is revision `0.1.1`
while retaining `0.1.0` as a supported prior pin. Canonical workflow
interpretation and scheduling remain owned by `sts2-harness`.

The audit found five rows with merged but explicitly partial component pins:
harness #95 (PRs #131 and #133, used by two budget/limits rows), Studio #110
(PRs #124 and #125), game-mod #78 (PR #118), and the Studio seed row's
game-mod probe (PR #113). Harness #94's live-executor PR #134 and Studio
#110's PR #126 are still open. The
other required owner children have no merged companion PR observed. Game-mod
#79 is API-closed, but its source-only PR #113 states that native
authorization, live-host settlement and failure recovery remain unverified;
the matrix therefore keeps that acceptance gate open.

The authored
[`studio-acceptance-journey-20260913.json`](studio-acceptance-journey-20260913.json)
describes start, wait, profile, seed, observe, decide, action, conditional
context inclusion, projection, lifetime, latest-only, persistent pin, budget,
held preview and history-policy steps, plus unauthorized, unavailable, stale,
oversized, cancellation, restart and outcome-unknown cases. It is an
authored synthetic specification: zero journey or failure cases were executed
in this documentation change. Existing repository JSON, evidence, CI-input
and checksum checks remain the passing synthetic/component lane. Real-process
cross-repository, provider, native-game and independent D3/G6 lanes remain
`unverified` and are listed as external gates in the matrix.

## Fresh native preflight — 2026-09-13

The fresh D1 native capability check is recorded in
[`native-preflight.json`](native-preflight.json). Native spawn returned the
canonical D1 task name, but the child reported no callable native collaboration
tool, so no D2 or D3 chain could be created. Requested Luna/Max settings remain
separate from unverified effective settings. The hierarchy and independent D3/G6
review gates therefore remain blocked; this evidence does not claim cross-
repository authority, fault/privacy/replay, native-game or unattended-recovery
acceptance.

`tools/verify-evidence.sh` enforces this failure-closed projection in CI: active
B-001/B-005 blockers, blocked WF-001/WF-002/WF-003 requirements, and the
non-success preflight outcome must remain present until a future independently
reviewed evidence update changes them together.

## Source refresh — 2026-09-11

A post-candidate source refresh is recorded in
[`SOURCE_REFRESH_2026-09-11.md`](SOURCE_REFRESH_2026-09-11.md) with exact
default heads in
[`integration/source-lock-20260911.json`](../../integration/source-lock-20260911.json).
It is additive evidence: the candidate snapshot and its remaining gates below
remain historical and authoritative for their stated scope.

## Current verification — 2026-09-11

The delivery package verifier passed under Python 3.11.2, followed by all 12
package checker tests. The package check covers archive integrity, task DAG,
traceability, contracts, and synthetic fixtures only; it does not replace
product or native-host evidence.

The merged delivery commit `7254e35` makes the Rust process-conformance driver
retry isolated loopback startup and include bounded harness stderr when a child
process fails. The driver compiled and passed against the exact pinned harness
candidate `6e0272511997165ebba337a15d22f73acd36a4c4`: all 13 catalog definitions
validated, the required map capability was rejected against the missing-map
manifest, and the synthetic authenticated run, restart, replay, and redacted
export checks completed. A workspace-local temporary directory was used because
the system `/tmp` tmpfs was full; no unrelated temporary data was removed.

## Current continuation — 2026-09-10

The implementation is split across an exact local harness candidate, a
separate gateway authority candidate and an MCP mapping candidate. The delivery branch contains the catalog,
contract artifact, conformance inputs, process driver, ADR set and evidence
ledgers. The exact revisions and publication limits are recorded in
[`integration/candidate-lock.json`](../../integration/candidate-lock.json) and
[`integration/source-lock-20260910.json`](../../integration/source-lock-20260910.json).

The harness candidate is `6e0272511997165ebba337a15d22f73acd36a4c4`, based on
refreshed default head `33437ddb18f69f68d88521d947efa3568a32a3bf`. It contains
the current owner recovery and telemetry implementation integrated with the
workflow-v1 decoder, typed definitions, canonical compiler/artifact boundary,
three-valued guards, strict reducer, bounded dynamic plan registry/runtime,
provider and budget boundaries, durable workflow event/invocation storage,
protected episode ports, authenticated loopback management API, `sts2-workflow`
CLI, the synthetic management adapter, and a typed recovery-admission status
contract for supervision consumers. Served management now uses a
transactional SQLite store with a durable synthetic runtime snapshot and
per-run command admission that returns typed pending while another command is
unresolved; persisted events carry integrity seals and offline replay rejects
tampering. The management export applies a bounded redaction projection and has
sentinel coverage. Authoritative reconciliation can replace provisional
Accepted/Unknown transport evidence for the same operation, and a direct
Settled receipt is idempotent; the execution-store and executable composition
checks cover those recovery boundaries.

The gateway authority candidate is `1a29e425eca96eba6d88caf85c1b2dc71c4b072c`,
based on `6b6c7f2fac67de22fdf78c9fd818c6781f689ba0`. It passed its owner policy,
format, Clippy and full test suite with the new authority-route test included.
The harness and gateway candidates are not merged to their default branches.

The MCP candidate is `16ca0cb06dc93564c14963bc544bef282b38d26d`, based on
`73e777b96700917cca5ff8f6ce0f5a72009384bc`. It keeps Runtime-v2 mapping thin,
advertises the optional bounded `workflow_boot_epoch` on the three Runtime-v2
tools, forwards it as `x-sts2-workflow-boot-epoch`, and rejects malformed values
before gateway access. It passed strict policy, format, Clippy, and the full
workspace suite. The explicitly built composition checks below also run the
candidate MCP server and gateway as child processes over synthetic loopback; they
do not prove owner-issued authority flow or protocol/watchdog integration.

The following harness checks passed on the integrated candidate:

- `cargo fmt --all --check`
- `cargo run --locked --package repo-policy -- --strict` — 644 sized files, zero warnings/errors
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `cargo test --workspace --all-targets --all-features --locked` — all workspace
  targets passed, including 167 runtime tests with 1 operator-only runtime test
  explicitly ignored; the SQLite management integration target passed its 5 tests
- the targeted management suites passed: 6 management tests and 5 SQLite
  management tests, including independent-connection serialization of an
  unresolved command
- the explicitly built gateway/MCP/harness child-process composition tests
  passed: 1 standard expert composition test, 1 synthetic REST selector chain
  and 1 native-shaped REST selector chain
- the Rust delivery conformance driver required `valid:true` for all 13 catalog
  definitions, rejected the missing-map capability case with a structured
  diagnostic, and exercised authenticated run/status/events, pause/resume/step,
  typed safely-resumable recovery admission, offline replay with tamper rejection,
  durable served-management restart, and the redacted export path. The recovery
  admission object exposes only a bounded kind/capability result and carries no
  workflow cursor, action identity or provider output.

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
- Cross-repository MCP/protocol/watchdog conformance is not complete; synthetic
  MCP-to-gateway child-process composition passed, while owner-issued authority
  and protocol/watchdog integration remain open.
- Full crash/fault matrix, independent adversarial review, legacy differential
  measurements, redacted telemetry integration and complete offline replay
  artifact retention still require additional evidence.
- Native compatibility and unattended live recovery remain separately gated by
  authorization and environment.

Resume from [the operations guide](../operations/RESUME.md), preserve the exact
candidate locks, and update the live requirement/task ledgers only with new
tool-derived evidence. Do not claim a default-branch merge from a local
candidate.
