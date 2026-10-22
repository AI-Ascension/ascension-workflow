# Redacted workflow export evidence

The process driver must reject a leaking or unrelated export before it reports
`conformance: PASS`. This is a bounded consumer check for WF-071 and the export
privacy portion of WF-070. It does not interpret workflow graphs or assume
Harness's validation, scheduling, replay, or effect-settlement authority.

## Producer contract and evidence class

The existing CI management candidate remains
`AI-Ascension/sts2-harness@6e0272511997165ebba337a15d22f73acd36a4c4`, recorded
in [`candidate-lock.json`](../integration/candidate-lock.json). It is a
historical candidate pin, not a claim about the current Harness default branch.
At that exact revision, the producer's
[`store_sqlite_export.rs`](https://github.com/AI-Ascension/sts2-harness/blob/6e0272511997165ebba337a15d22f73acd36a4c4/crates/harness/src/management/store_sqlite_export.rs)
replaces cursor and event execution identifiers with `[redacted]`, clears
pending operations and event operation identifiers, replaces event reasons
with `event`, and omits integrity digests. Its
[`contract_types.rs`](https://github.com/AI-Ascension/sts2-harness/blob/6e0272511997165ebba337a15d22f73acd36a4c4/crates/harness/src/management/contract_types.rs)
defines the run, event, budget, and closed enum vocabulary.

The verifier contains an original read-only projection of that published wire
shape. It has no sibling source dependency or copied owner implementation.
Future producer field additions or schema changes require deliberate consumer
review; they cannot silently qualify as verified evidence.

The checked-in JSON fixture and synthetic writer/health server are original MIT
test inputs. They prove consumer acceptance and rejection behavior by invoking
the compiled driver. They are not records of an actual Harness run. The
existing CI job separately builds the exact pinned Harness management binary
and runs the full process driver against it. The separate served-live producer
pin and its catalog check are unchanged.

## Checks before success

The verifier requires a regular output file and reads at most 1 MiB plus one
sentinel byte, including when a file grows after its metadata was inspected.
It rejects oversized, unavailable, symlink, and nonregular output. A bounded,
typed JSON decoder then rejects malformed documents, trailing data, duplicate
keys (including escaped duplicates), missing required fields, and unknown
fields at every record level. Formatting and legitimate JSON escapes do not
change the result.

The root, run snapshot, and every event must identify the requested run. The
snapshot's definition digest must be a lower-case SHA-256 representation and
every event must carry the same digest. Schema versions, state vocabulary,
safe integer bounds, nonempty event evidence, and increasing event sequences
must match the reviewed projection. These consistency checks do not prove
that events are complete or that replay is equivalent to a live execution.

The cursor's three fields and each event execution identifier must contain the
exact redaction marker. Pending operations and event operation identifiers
must be explicit JSON null. Event reasons must be `event`; an event integrity
digest may be omitted or null. The required classification field may be null
or one of the producer's closed classification values. An asserted
`redacted: true` flag alone is insufficient.

Each export attempt creates an exclusive directory below the process's
temporary directory, with mode `0700` on Unix. The output name is fixed inside
that directory and does not incorporate an unverified run identifier. Cleanup
removes only that output and the now-empty directory. It does not recursively
delete unexpected files. The surrounding temporary directory and the executing
user remain trusted; this check does not defend against another process with
the same user identity, and it does not specify Windows ACL isolation.

The export subprocess's stdout and stderr go directly to the null device. The
private file is its only evidence input, so unverified export bytes cannot be
duplicated into command output buffers or echoed on exporter failure. File,
decoder, and exporter errors use fixed diagnostic codes without raw values or
private paths. This property is specific to the export boundary: the driver's
other command and service diagnostics retain their existing behavior, as does
the existing subprocess lifetime policy.

## Verification and remaining acceptance

Run the consumer checks with the admitted toolchain:

```text
cargo +1.97.1 fmt --manifest-path tools/conformance-driver/Cargo.toml --check
cargo +1.97.1 clippy --locked --manifest-path tools/conformance-driver/Cargo.toml --all-targets --all-features -- -D warnings
cargo +1.97.1 test --locked --manifest-path tools/conformance-driver/Cargo.toml
cargo +1.97.1 build --locked --manifest-path tools/conformance-driver/Cargo.toml
```

The compiled-driver regression tests require Unix shell utilities and an owned
loopback listener, matching the existing Ubuntu CI runner. Unit tests cover
structural privacy, identity, bounds, file handling, and cleanup. A passing
fixture suite does not close Workflow #1 or #11, cross-owner B003 acceptance,
native/provider B004 acceptance, or independent-review B005 acceptance. Each
still requires its own exact-source evidence and review.
