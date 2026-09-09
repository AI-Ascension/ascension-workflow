# Resume source review

Evidence level: source-derived inspection, not executed behavior or a completed T01 gate.
The native read-only chain `/root/preflight` → `/root/preflight/d2` →
`/root/preflight/d2/d3` completed this bounded review. No descendant wrote files or ran tests.
Requested Luna/Max settings remain configuration-only evidence.

## Inspected baseline and interfaces

Harness main: `fc44d3ef65fefa6d13ecd5f690e5335a6ef60080`, clean checkout.
Paths below are relative to the harness repository.

| Path | Current seam and integration consequence |
| --- | --- |
| `crates/harness/src/episode/runner.rs` | `EpisodeRuntimePort` extends barrier, recovery and shutdown ports; launch, observe, legal catalog and dispatch remain the owner-local boundary. |
| `crates/harness/src/episode/runner_impl.rs` | Each run constructs an in-memory machine and ledger. Workflow resume needs explicit persisted state. |
| `crates/harness/src/episode/runner_steps.rs` | Observation/actionability, policy routing and bounded catalog refresh must survive extraction. |
| `crates/harness/src/episode/runner_actions.rs` | Generation/step-derived operation identity, admission, barrier and settlement/reconcile need a durable composite boundary. Dispatch error handling needs a regression before changing it. |
| `crates/harness/src/episode/state_machine.rs` | Observation can overwrite a pending phase; a workflow cursor must never invoke that transition while mutation uncertainty remains. |
| `crates/harness/src/episode/recovery.rs` | Bounded reobserve/reconcile/release/stop port; receipt and no-effect evidence need explicit durable admission. |
| `crates/harness/src/bin/runtime_support/runtime_v3_ledger.rs` | Adapter dispatch binds instance, MCP session, lease, epoch, generation, state, operation and action. Preserve authoritative boundary checks. |
| `crates/harness/src/bin/runtime_support/runtime_v3_recovery.rs` | Replacement MCP processes serve recovery reads and original-operation reconciliation. This is not durable host receipt evidence. |
| `crates/harness/src/bin/runtime_support/runtime_v3_combat_demo.rs` | Separate demonstration dispatch loop must not become a workflow mutation route. |

The core action identity and receipt types do not carry every run/instance/session/authority
namespace used by the adapter. This does not establish an exploitable boundary defect; it means
workflow persistence must preserve the complete binding and validate it during reconciliation.
Accepted receipts remain distinct from independently witnessed settlement.

The existing runtime-v2 coordinator supplies bounded multi-instance and serial-mutation
conformance ideas, while record/replay/memory ports provide owner-local interfaces. Their
library existence does not establish assembly into runtime-v3 or durable workflow behavior.

## Candidate overlap

PR 40 at `baabf9767264de11e09cd65cc7262ba36664ec2d` already contains a SQLite
`ExecutionStore` and authenticated worker controls. Root inspected its manifest and store opening
surface, including read-only opening without creation/migration. Full candidate source,
transaction, migration and compatibility review remains necessary before adoption.

PR 41 at `30eaf19ccb5b2ed118b5cf1baaef3793a408f499` adds bounded reobservation for
forward drift between normal and expert MCP snapshots. Preserve its two-reobserve limit and
failure on identity regression/exhaustion. Its newly added composition/exhaustion tests were
inspected as source, not executed.

Neither candidate was integrated. The source lock records them separately from main.

## Next contract decisions

Retain the legacy runner as a compatibility wrapper around a harness-owned protected operation.
Keep the store adapter outside pure orchestration. Atomically persist intent and the pending
mutation slot with full identity binding before dispatch; reconcile the original operation after
uncertainty. The fixed outer workflow owns execute-action. Unsupported authority/receipt retention
must yield `NeedsOperator`.

Complete candidate review and ADR-WF-001 through ADR-WF-009 before write leases depend on these
proposals. Rust installation authorization remains pending; no policy, compiler, process,
store-fault or native validation is inferred from this review.
