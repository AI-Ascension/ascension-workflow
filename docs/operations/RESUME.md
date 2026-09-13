# Resume Phase 1

The current run has local harness, gateway and MCP candidates plus a delivery branch. Preserve the
original archive, stable IDs, dirty work and exact locks. Start with
[`execution-state.json`](../../execution-state.json),
[`orchestration/current-run.json`](../../orchestration/current-run.json), and
[`integration/candidate-lock.json`](../../integration/candidate-lock.json).

The harness candidate `6e0272511997165ebba337a15d22f73acd36a4c4` passed format, strict policy,
Clippy and the full workspace suite. The delivery Rust process driver passed authenticated
loopback catalog and control conformance, including positive validation assertions, typed
safely-resumable recovery admission and redacted export checks; the SQLite management target also
passed its restart, tamper and independent-connection command-admission tests. Re-run the
commands and driver from the current locks before changing the candidate. The gateway candidate is
separate and is not on its default branch.

The explicitly built gateway/MCP/harness composition checks also pass: one standard expert path and
two REST selector paths, covering synthetic and native-shaped selector IDs. The recovery adapter
accepts authoritative replacement of provisional transport evidence and treats a direct settled
receipt as already reconciled. These fixtures remain synthetic boundary evidence and do not close
the native compatibility or full fault and authority gates.

The fresh native D0→D1→D2→D3 hierarchy is recorded in
[`native-preflight-20260913.json`](../evidence/native-preflight-20260913.json): D1 spawned D2,
and D2 spawned D3; D3 did not spawn D4. Do not flatten the hierarchy or substitute subprocesses.
The fresh run resolves B-001, while B-005 remains active until a real independent review is
available. Requested Luna/Max settings remain separate from unverified effective settings.
Rust 1.97.1, rustfmt and Clippy are already installed in the workspace-local toolchain; B-002 is
resolved.

The MCP candidate `16ca0cb06dc93564c14963bc544bef282b38d26d` forwards the optional bounded
`workflow_boot_epoch` authority header through the Runtime-v2 mapping. Its isolated workspace checks
passed, and the executable composition checks exercise the candidate MCP and gateway child
processes over synthetic loopback. Owner-issued authority and protocol/watchdog integration remain
unverified.

Remaining work is the cross-repository protocol/watchdog conformance, full crash/privacy/
telemetry/replay evidence, independent review, legacy differential measurements and the separately
authorized native gate. Do not mark a requirement complete from a file existing or a unit test
alone. The current requirement ledger distinguishes `verification`, `blocked` and `in_progress`.

The package archive contains provenance checks only. Keep product conformance in the admitted Rust
driver and invoke the built harness binary; do not add a second graph interpreter or scheduler.
No gameplay operation, valued save, live provider call or unresolved game effect is pending.
