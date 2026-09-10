# Resume Phase 1

The current run has local harness, gateway and MCP candidates plus a delivery branch. Preserve the
original archive, stable IDs, dirty work and exact locks. Start with
[`execution-state.json`](../../execution-state.json),
[`orchestration/current-run.json`](../../orchestration/current-run.json), and
[`integration/candidate-lock.json`](../../integration/candidate-lock.json).

The harness candidate `9c281717ed257476b179f186ee3d174b8a5cf0f2` passed format, strict policy,
Clippy and the full workspace suite. The delivery Rust process driver passed authenticated
loopback catalog and control conformance, including positive validation assertions and redacted
export checks; the SQLite management target also passed its restart and tamper tests. Re-run the
commands and driver from the current locks before changing the candidate. The gateway candidate is
separate and is not on its default branch.

The native D0→D1→D2→D3 hierarchy was not observed: D1 environments had no callable native
collaboration tools. Do not flatten the hierarchy or substitute subprocesses. Keep B-001 and B-005
active until a real independent review is available. Rust 1.97.1, rustfmt and Clippy are already
installed in the workspace-local toolchain; B-002 is resolved.

The MCP candidate `16ca0cb06dc93564c14963bc544bef282b38d26d` forwards the optional bounded
`workflow_boot_epoch` authority header through the Runtime-v2 mapping. Its isolated workspace checks
passed; live cross-repository MCP-to-gateway conformance and owner-issued authority remain unverified.

Remaining work is the cross-repository protocol/watchdog conformance, full crash/privacy/
telemetry/replay evidence, independent review, legacy differential measurements and the separately
authorized native gate. Do not mark a requirement complete from a file existing or a unit test
alone. The current requirement ledger distinguishes `verification`, `blocked` and `in_progress`.

The package archive contains provenance checks only. Keep product conformance in the admitted Rust
driver and invoke the built harness binary; do not add a second graph interpreter or scheduler.
No gameplay operation, valued save, live provider call or unresolved game effect is pending.
