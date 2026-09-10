# Resume Phase 1

The current run has local harness and gateway candidates plus a delivery branch. Preserve the
original archive, stable IDs, dirty work and exact locks. Start with
[`execution-state.json`](../../execution-state.json),
[`orchestration/current-run.json`](../../orchestration/current-run.json), and
[`integration/candidate-lock.json`](../../integration/candidate-lock.json).

The harness candidate `623848ed6dc8b2bbfa07cd287df7aa4feba18c87` passed format, strict policy,
Clippy and the full workspace suite. The delivery Rust process driver passed authenticated
loopback catalog and control conformance. Re-run the commands and driver from the current locks
before changing the candidate. The gateway candidate is separate and is not on its default branch.

The native D0→D1→D2→D3 hierarchy was not observed: D1 environments had no callable native
collaboration tools. Do not flatten the hierarchy or substitute subprocesses. Keep B-001 and B-005
active until a real independent review is available. Rust 1.97.1, rustfmt and Clippy are already
installed in the workspace-local toolchain; B-002 is resolved.

Remaining work is the cross-repository MCP/protocol/watchdog conformance, full crash/privacy/
telemetry/replay evidence, independent review, legacy differential measurements and the separately
authorized native gate. Do not mark a requirement complete from a file existing or a unit test
alone. The current requirement ledger distinguishes `verification`, `blocked` and `in_progress`.

The package archive contains provenance checks only. Keep product conformance in the admitted Rust
driver and invoke the built harness binary; do not add a second graph interpreter or scheduler.
No gameplay operation, valued save, live provider call or unresolved game effect is pending.
