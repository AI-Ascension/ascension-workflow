# Synthetic source directory for the four-stage integration plan

All values in this directory are synthetic, original conformance data; there are
no Train recording bytes, credentials, game assets, save files or provider calls.
The input layout/field types come from the harness owner's
`recorded_run_tests.rs` fixture and source adapter. Sentinels deliberately exercise
the existing redaction allowlist. This is test input, not a portable schema.

Use the pinned source plan with this directory as SOURCE. Expected output:
5 events, 1 accounting record; 8 source rows = 6 emitted + 2 filtered.
Process and episode failed; action outcomes remain unknown. Model-execution
namespaces remain distinct, exact nanoseconds exceed JavaScript safe integer
precision. All five counters have explicit synthetic values. A preliminary fixture
omitting three counters was rejected by the current legacy source adapter; its
failure remains recorded separately. Missing source counters were not coerced to zero.

The adapter currently labels its fixed omission provenance `sanitized_source_derived`.
That label must not turn this synthetic integration test into real Train evidence.
The integration report explicitly identifies the fixture as synthetic. Repeated
exports should preserve identical semantic and ZIP digests and source fingerprints.
