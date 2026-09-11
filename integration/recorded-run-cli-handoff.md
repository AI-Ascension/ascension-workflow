# Recorded-run CLI handoff

All CLI interfaces are concrete; no consumer interface question is pending.
The current plans pin candidate 3, Studio `ad9f764c5caf6a7208b55d380e4f79f7ad8e6455`,
and observability `f6861dffb0f45cdc01f122c027f219437f4773fd` (including consumer fixes `6c5ee3f` and independently closed metadata rollback fix).
Protocol final commit is `6cdcf0995b25185e59c57d21b1e7a23c2d9b5f8a`.
Its artifact inventory is
`580c1cf3be4bb3e4eb37b9acd9166808b7386b0eb84286cc0798a0d88e35bb35`;
schema is `a6c32127290f4d5e670d8863f97a74a7b8e3e411e735d81394b51fe1578b4eb6`.

| Stage | Exact interface | Report |
| --- | --- | --- |
| Export | `sts2-recorded-run-export export SOURCE --output BUNDLE.zip` | Review2 immutable transport 48a4ff28…; JSON stdout; source commit a065fe51 |
| Validate | `node tools/recorded-run/validate.mjs BUNDLE.zip` | Canonical JSON summary; candidate 3 only |
| Studio | `node tools/inspect-recording.mjs BUNDLE.zip` | Independent importer's protocol-shaped JSON |
| Observability | `node deploy/recorded-run/import.mjs import BUNDLE.zip DB`, then `inspect DB RUN_KEY` | Import/revision state, local storage unless explicitly configured otherwise |
| Workflow bridge | `observability-report OBS_ROOT BUNDLE.zip DB` | Same owner APIs, duplicate retry, exact stored summary; no endpoints |

Pointers for all three reports: `/semantic_digest`, `/recording_identity`,
`/event_records`, `/accounting_records`, `/unsupported_records`,
`/evidence`, `/completeness`, `/streams`.

[Candidate 3 consume plan](recorded-run-candidate3-consume-plan.json) is populated.
[Full candidate 3 plan](recorded-run-candidate3-plan.json) is populated and executed.
Build/source linkage is recorded in the [exporter pin](recorded-run-candidate3-exporter-pin.json).
Observability's three edge cases and metadata rollback have independent closure.

Executed candidate 3 proofs: all 8 valid/25 invalid vectors through all three
implementations with zero findings; actual Train ordered CLI/deep comparison passed;
full synthetic source pipeline and six source probes passed, with all four positive
probe shapes independently consumed. Exact evidence is
[review2 Train](recorded-run-candidate3-train-review2.json).
The legacy golden is synthetic, process completed/gameplay episode_failed/actions
unknown. Actual review2 Train has process failed/gameplay episode_failed/two unknown
action outcomes, with seed-start accepted/settled/observed. Independent seed-source
review is closed; that start does not imply successful gameplay.

Candidate 2 original plans and evidence remain historical. Actual candidate 2
Train CLI/payload and Collector/MLflow proofs, privacy-patched LAN deployment and
exercised rollback are recorded in the [current matrix](recorded-run-matrix.md).
Do not feed old ZIPs to the active candidate 3 validator or silently relabel evidence.
Use [the suite](../conformance/recorded-run.md) for exact commands and process limits.
