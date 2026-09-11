# Recorded-run compatibility matrix

Current state — 2026-09-11: **actual candidate 3 review-2 export, ordered CLI,
deep consumer comparisons, local MLflow persistence and deployed LAN browser pass.**
Seed-start mapping is corrected and independently closed: accepted/settled/observed
is preserved separately from episode_failed and two unknown action outcomes. Harness's
final workspace passes 656 tests (0 failed, 5 ignored). Formal admission remains a
separate maintainer/coordinator decision.

LAN: `http://192.168.1.146:4173/`, candidate 3 Studio release
`52891bfb9614672cba058e1527b5c653b55e0c321f7a5aef6f8461569c77a821`,
enabled user service PID 288479. Coordinator owns activation and browser/backend runs.

## Exact candidate and implementation pins

Proposed wire `1.0.0-candidate.3`, schema
`a6c32127290f4d5e670d8863f97a74a7b8e3e411e735d81394b51fe1578b4eb6`,
inventory `580c1cf3be4bb3e4eb37b9acd9166808b7386b0eb84286cc0798a0d88e35bb35`.
[All 43 artifact files](recorded-run-candidate3-pin.json) are pinned.

Actual review-2 ZIP: 11,506 bytes, SHA256
`fe10fe2d9674493469f17a51f59d2b01f07b4cbd1c5179e061148e97bf476317`;
semantic digest
`e5bd1aaac5209573192861c81f3bbac6b6356c24f48c129b52e93e508454b686`.
[Complete review-2 evidence](recorded-run-candidate3-train-review2.json) records
every admitted event/accounting record, omissions, provenance, source fingerprints,
both host exports, local CLI/API results and coordinator backend/browser reports.

| Repository | Current implementation/disposition | Remaining gate |
| --- | --- | --- |
| sts2-protocol | Commit `6cdcf0995b25185e59c57d21b1e7a23c2d9b5f8a`; exact candidate 3 oracle/artifacts; seven original findings independently closed. | Formal owner admission per docs/recorded-run-admission.md |
| sts2-harness | Review-2 commit `a065fe5187afefa9deccba2355ed5c1f00ba20ac`; immutable exporter 4d7322a0… and transport 48a4ff28… verified; actual double export, 656-test workspace and independent review closure pass. | Formal admission |
| ascension-workflow-studio | `ad9f764c5caf6a7208b55d380e4f79f7ad8e6455`; independent privacy closure, owner 98 unit/8 Chromium tests, actual import and deployed numeric/accounting/contrast proof pass. | Review2 actual browser/import passed; formal admission |
| ai-agent-observability | `f6861dffb0f45cdc01f122c027f219437f4773fd`, consumer fixes `6c5ee3f`; 60 owner tests, actual candidate3 Collector/MLflow and independent consumer/metadata-rollback closure pass. | Review2 actual backend passed; Laminar and Dockerfile build unverified |
| ascension-workflow | Isolated `codex/recorded-run-ascension-workflow`, base `45341c7bdfa8d51eb6b52aec1ccc5b9f1a73a688`; bounded Rust command suite, 10 tests, six source probes, actual/deep synthetic comparisons pass; timeout finding independently closed. | Reviewable local commit and coordinator handoff; no push/merge |
| sts2-gateway | Compatible lifecycle/lease/fencing metadata source; owner source/policy/test evidence supports no change. | Revisit only for demonstrated producer metadata gap |
| sts2-mcp-server | Compatible ordered/redacted transport source; 230 actual MCP rows filtered in one explicit range. No importer needed. | Acknowledgement must never become action settlement |
| sts2-game-mod | Current upstream `bd8e90542dfc89366f820150c5c755e32716b1b0`, 114 commits past discovery; owner 9 inventories, strict policy/fmt/10 focused tests pass; no change justified. | Source-owned seed grammar verified; no additional game-mod changes |
| ascension-watchdog | Justified non-consumer; historical imports cannot trigger recovery. | No dependency absent a supervisory use case |
| .github / org-governance | Ownership/admission guidance consumes technical evidence without owning the schema. | Exact final review evidence and maintainer admission |

The [exporter pin](recorded-run-candidate3-exporter-pin.json) distinguishes:
immutable build SHA `4d7322a0f43fdd624644729504771e1c72e57b9a239bfe34ac0a1581bcdf4c11`;
executed transport SHA `48a4ff28886dfbeaba9de0bcb03f1325434952023e9a775eec9b9b6dbbc14fb3`;
embedded adapter source SHA
`2754dc1afd12f8e983103ed170c6d65494575492f69dfd3a1bd3e67f9f82beab`.
Workflow recomputed all 15 embedded source inputs and verified them against
commit a065fe51. Stripped/original fixture bytes matched in coordinator evidence.
Later mutable source is not a runtime input to this immutable binary.

## Current acceptance ledger

| Gate | Concrete result and boundary |
| --- | --- |
| RRI-01 contract | Exact candidate3 schema/manifest const/inventory agree; pins rechecked after stages and before comparison. |
| RRI-02 deterministic export | Two host exports exit0, identical 11,506-byte ZIP and semantic digest. All 9 source bytes/hash/mtime entries unchanged. 240 input = 9 emitted + 231 filtered; no unsupported/rejected source rows. |
| RRI-03 ordered CLI | Actual review2 ZIP passes canonical → Studio → observability; eight summary pointers agree. Full four-stage synthetic source pipeline also passes. |
| RRI-04 same bytes | Identical validated ZIP consumed independently by Studio and observability; source/runtime snapshots unchanged during API comparison. All 8 valid/25 invalid protocol vectors match all three implementations. |
| RRI-05 deep agreement | Every presented event/accounting payload, source key, sequence, decimal timestamp, identity and evidence agrees; common producer/adapter/version/completeness/omissions agree. Exact original events/accounting persisted in SQLite; duplicate retains one revision. |
| RRI-06 producer positives | Six reproducible source probes pass: baseline, deterministic repeat, absent accounting, partial final trajectory, unsupported accounting status, symlink rejection. All four valid scenario shapes pass deep comparisons in both consumers. Workflow recording profile remains unsupported. |
| RRI-07 negative/privacy/bounds | All 25 shared invalid ZIPs rejected, six deployed identity-privacy probes rejected without losing prior recording. Source symlink rejects exit2/no bundle; sentinels absent; unknown/null/absent usage remains distinct. Owner JCS/resource regressions pass. |
| RRI-08 actual backend | Actual candidate3 review2 passes disposable Collector 0.160.0 → MLflow 3.16.0: 19 spans/1 revision, exact fields/scopes, duplicate unchanged, persists after restart, input artifact unchanged. Laminar remains unverified. |
| RRI-09 browser/authoring | VM-origin Chromium to active candidate3 LAN release: 9 rows, counters 80/80/0/99/1, reported usage 11428/44, duplicate no-op, six privacy failures retain prior recording, six designer nodes with exact text/background colors, zero errors/outgoing requests. |
| RRI-10 deployment/lifetime | All four served hashes match Studio's tested candidate3 manifest. Enabled active service PID288479; prior f572… available rollback. Earlier f572→4984→f572 rollback/lifetime exercise is historical evidence of the service procedure; no claim of a new candidate3 rollback exercise or physical LAN client. |
| RRI-11 review/checks | Protocol, Studio, observability consumer/metadata-rollback and concrete workflow timeout findings independently closed. Harness review2 seed/accounting findings independently closed, final workspace passes 656/0/5; our scoped tests/Clippy/fmt pass. |

## Source semantics and omissions

Actual producer is `seed-readiness-controller`, version `unknown`, source format
`seed-readiness-controller-release-v2`. Adapter version `0.0.0` is independent
of its exact source digest. Recorded component revisions are independent of the
current implementation checkouts:

| Recorded component | Source revision |
| --- | --- |
| protocol | `ed8626c2cf30089b4bdf214a2fdcb09b3eca3d29` |
| runtime | `682c2b5ba38010e16d43b04c43d40184bda70106` |
| game_mod | `70c56c3f5bb179b32b3e919814c43d977af2b725` |

Events: 6 trajectory + 1 decision + 1 result; separate accounting: 1. Whole-row
filters: 230 MCP (`raw_mcp_disallowed`) + 1 manifest
(`private_source_metadata`). Field omissions count affected source rows separately
and are not added to the row equation:

| Stream | Rules and affected rows |
| --- | --- |
| decisions | identity transformation 1; private metadata 1; raw decision text 1 |
| provider-accounting | private metadata 1; provider request identity 1 |
| result | private metadata 1 |
| trajectory | identity transformation 4; private metadata 1; raw error 1; raw observation 3; raw seed 3 |

Both action outcomes carry the same admitted hashed action identity as the
trajectory decision and preserve separate operation identities/source ordinals.
Their evidence stays unknown; process failed and gameplay episode_failed remain
independent of provider completion. Recorded model execution values `1` and
`model-execution-1` stay in separate namespaces without an inferred join.
Completeness remains partial/source snapshot unverified despite before/after equality.

## Review dispositions

| Owner/finding | Disposition |
| --- | --- |
| Protocol six validator gaps | Implemented and independently closed; candidate3 shared regressions pass. |
| Protocol tuple equality | Closed by clarification: equality is valid across distinct semantic roles; no fabricated join. |
| Studio manifest/common/opaque privacy | Both high findings independently closed; six actual worker/browser negatives pass. |
| Observability outbox, empty collector, creating inspect DB | Three original findings independently closed at corrected consumer code. |
| Observability archived-xattr rollback | Independently closed at f6861dff using verified descriptors; Linux user xattr/POSIX ACL scope is explicit. Privileged metadata/other filesystems remain unverified. |
| Observability false HIGH CI glob | Withdrawn: *.test.mjs excludes runtime.mjs. |
| Harness empty field omissions | Corrected in review1; actual affected-row counts and synthetic sentinel/omission probes agree. |
| Harness accounting constants/missing usage | Corrected; known/unknown/null/absent usage and unsupported status probes pass. |
| Harness lost action identity | Corrected; both actual outcomes retain action digest while remaining unknown. |
| Harness missing optional accounting | Corrected; export without accounting validates and both consumers agree. |
| Harness final tail/ordinals | Corrected for admitted streams; valid prefix/one rejected final row/interrupted state pass both consumers. Malformed MCP tail remains a fail-closed format limit. |
| Harness symlink/snapshot/bounds | Immutable binary rejects symlink/no output; source fingerprints unchanged; owner bounds/change regressions pass. Snapshot does not lock writers. |
| Harness provenance | Adapter content identity and independently recorded protocol/runtime/game-mod revisions retained. Extra gateway/MCP/trees/controller scopes need protocol-owned design; optional provider/model labels withheld pending public allowlist. |
| Harness JCS | Owner immutable-binary UTF-16 supplementary-key digest matched oracle; duplicate/number/bound tests passed. |
| Harness actual seed-start downgrade | Independently closed at a065fe51. Exact source identity grammar accepts the legitimate slash/leading punctuation; review2 actual record is accepted/settled/observed and raw-seed omission affected_rows=3. Canonical, both consumers, real backend and executed LAN seed assertion pass. Overall failure/two unknown actions remain unchanged. |
| Harness final workspace check | Final review2 full workspace: 656 passed, 0 failed, 5 ignored; 17 focused tests and fmt/Clippy/policy/build pass. Earlier temporary-storage failures are historical, not a current blocker. |
| Workflow process tree/bounded reads/pins | Fixed; 10 tests and coordinator timeout re-probe pass. Group cleanup covers trusted non-daemonizing descendants, not an arbitrary sandbox. |

## Commands and history

Both [full export plan](recorded-run-candidate3-plan.json) and
[consume plan](recorded-run-candidate3-consume-plan.json) are populated and executed.
[Suite commands](../conformance/recorded-run.md) reproduce source probes, ordered
imports and deep comparisons with new output directories. Local SQLite proof,
coordinator real MLflow, and coordinator LAN browser evidence have separate scopes.

Candidate2 JSON proofs and [its matrix snapshot](history/recorded-run-candidate2-matrix.md)
remain historical and were not rerun for this handoff. Earlier candidate3 synthetic
review-wave proof remains in `recorded-run-review-wave-evidence.json`. Review1 actual
proof is preserved in `recorded-run-candidate3-train-review1.json` and its original
plans/pins in `integration/history/`; it is not relabeled as review2.

Next: coordinator/maintainers consume this exact review2 evidence for formal
admission. The same workflow thread remains available for subsequent compatibility
or integration work. No game/provider call, host change, push, merge or admission by this lead.
