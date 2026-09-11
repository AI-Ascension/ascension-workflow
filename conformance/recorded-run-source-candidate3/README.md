# Synthetic source for candidate 3 integration

All values are invented integration fixtures; this is not Train source data.
Original candidate 2 fixture and its executed proofs remain unchanged in the
adjacent `recorded-run-source` directory.

This fixture follows the harness owner's current source adapter
`crates/harness/src/recorded_run_accounting.rs`: explicit source schema,
execution/decision/usage status, optional provider request identity, and optional
usage counters. The first row reports known counters; the second has unknown
usage with one supplied number, one null and three absent counters. Export must
retain unknown/null and absence, never invent reported usage or zero. This is
an integration assertion to verify once the corrected exporter is published.

Trajectory, decisions, MCP and result retain the original synthetic sentinels,
precise nanosecond time and numeric observation. The raw request/action/state,
observation, rationale, provider-process, error and private manifest/guest fields
must be omitted or digest-transformed with the owner's field-omission rules.
MCP and manifest rows retain their distinct whole-row filter reasons.

Execution is recorded separately from this fixture. The command suite's
`source-probes` executable checks actual output counts, source preservation,
omission rules, private sentinels, unknown/absent usage, and deterministic repeat.
Use the active candidate3 full plan and retain each immutable exporter revision's
results separately. This fixture contains no source seed-start receipt; actual
Train seed evidence and the harness owner's source-schema probes cover that case.
