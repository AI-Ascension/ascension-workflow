# Recorded-run process integration

The Rust runner invokes the owner's exporter, canonical validator and two independent
consumer implementations. It implements no shared schema, importer or scheduler.
Active proposed contract is `1.0.0-candidate.3`; complete byte pins are in
[integration/recorded-run-candidate3-pin.json](../integration/recorded-run-candidate3-pin.json).

## Reproduce the candidate 3 checks

Run from this repository with a new output directory each time:

```sh
mkdir -p .local
cargo build --locked --manifest-path tools/recorded-run-driver/Cargo.toml --bins
cargo run --locked --manifest-path tools/recorded-run-driver/Cargo.toml --bin candidate-matrix -- \
  ../recorded-run-sts2-protocol ../recorded-run-ascension-workflow-studio \
  ../recorded-run-ai-agent-observability .local/candidate3-vectors-new
cargo run --locked --manifest-path tools/recorded-run-driver/Cargo.toml -- \
  integration/recorded-run-candidate3-consume-plan.json \
  ../recorded-run-sts2-protocol/artifacts/recorded-run-bundle-v1-candidate3/golden/legacy-failed.zip \
  .local/candidate3-golden-new
```

The vector run accepted all 8 valid cases and rejected all 25 invalid cases in all
three implementations, with zero comparison findings. The golden ordered CLI run
also passed. These are synthetic proofs. Substitute a fresh coordinator-exported
candidate 3 ZIP for the golden to run the actual artifact; adding that ZIP as the
last argument to `candidate-matrix` runs the deeper payload/persistence comparison.

The ordered runner copies the existing artifact, checks byte stability, runs
`node tools/recorded-run/validate.mjs BUNDLE`, then Studio's exact
`node tools/inspect-recording.mjs BUNDLE`, then `observability-report`.
The Rust report bridge invokes observability's `importBundle` and
`ImportStore.inspect`, retries the same digest and compares the persisted revision.
Its equivalent public commands are `node deploy/recorded-run/import.mjs import BUNDLE DB`
and `node deploy/recorded-run/import.mjs inspect DB RUN_KEY`. No collector endpoint
is supplied. Local SQLite stays in the output directory.

## Full export scenario

The source-directory command is already established:

```sh
cargo run --locked --manifest-path tools/recorded-run-driver/Cargo.toml -- \
  integration/recorded-run-candidate3-plan.json conformance/recorded-run-source-candidate3 \
  .local/candidate3-source-new
```

The full plan now pins review-2 transport exporter SHA256
`48a4ff28886dfbeaba9de0bcb03f1325434952023e9a775eec9b9b6dbbc14fb3`.
Its immutable build/source commit is `a065fe5187afefa9deccba2355ed5c1f00ba20ac`;
all 15 embedded source inputs were checked against that commit. Later mutable
source edits do not change this binary. Runtime pins bind the binary and immutable
handoff inventory; build inputs are recorded separately in
`integration/recorded-run-candidate3-exporter-pin.json`.

The four-stage source command passed with 5 events and 2 accounting records.
The coordinator alone exports the actual Train source; never rerun the game.
The actual review2 ZIP passed ordered/deep comparisons and preserves seed-start
accepted/settled/observed independently of the failed episode and two unknown
action outcomes. Review1 bytes/results remain historical and unchanged.

Reproduce the synthetic source probes:

```sh
cargo run --locked --manifest-path tools/recorded-run-driver/Cargo.toml --bin source-probes -- \
  integration/recorded-run-candidate3-plan.json conformance/recorded-run-source-candidate3 \
  .local/source-probes-new
```

Six probes passed: baseline, byte-identical repeat, missing accounting, partial
final trajectory, unsupported accounting status and symlink-child rejection.
Known values remain reported; unknown/null/absent values never become zero.
The original fixture and each probe input remain unchanged during export, and no
private sentinel survives. Each accepted bundle receives canonical validation.
Pass each `recording.zip` from baseline/missing-accounting/partial-tail/unsupported-status
to the optional final argument of `candidate-matrix` for independent deep consumer
proof; all four have passed. Probes operate entirely under the new output directory.

## Bounds, lifecycle and evidence

Plans pin absolute executable paths and SHA256, repository revision, runtime
source files, contract inventory and exact source roots. Source roots cover the
runtime input set so unrelated documentation changes do not invalidate a run.
Pins are checked before execution, runtime pins again after each stage, and
contract pins after every stage and immediately before comparison.

Every stage receives a separate Unix process group. Timeout, report overflow,
nonzero exit, polling failure and normal parent completion terminate inherited
descendants and reap the direct child before consuming reports. Trusted tools
must not daemonize into a different session; this is lifecycle cleanup, not a
sandbox. Output files are polled at a 4 MiB limit. Parsing reads a single
descriptor with a bounded reader, never an unbounded reopen; plan/report hashes
use those exact parsed bytes. Bundle copies and artifact digests are bounded too.

Exit 2 is pending required pins, exit 1 a failed gate, and exit 0 a completed
process comparison. Evidence records the plan hash, revisions, executable and
report hashes, artifact digest, each stage and precise scope. Existing-export
mode explicitly skips export. No shell string is evaluated by the runner;
arguments are structured with `{source}`, `{bundle}`, `{summary}` substitution.
Source fixtures and test shells are synthetic.

The eight summary pointers are `/semantic_digest`, `/recording_identity`,
`/event_records`, `/accounting_records`, `/unsupported_records`,
`/evidence`, `/completeness`, `/streams`. Optional explicitly mapped fields
must also agree. API matrix checks accounting payloads, source keys, identity
maps, every presented event's payload/evidence/sequence/timestamp, provenance,
completeness, omissions and duplicate revision counts. Protocol and observability
also compare exact original event/accounting envelopes, not just projections.

```sh
cargo test --locked --manifest-path tools/recorded-run-driver/Cargo.toml
cargo clippy --locked --manifest-path tools/recorded-run-driver/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path tools/recorded-run-driver/Cargo.toml --check
git diff --check
```

If shared `/tmp` is full, create `.local/test-tmp` and prefix the test command with
`TMPDIR="$PWD/.local/test-tmp"`. Do not delete another thread's temporary evidence.

Ten tests include four descendant lifecycle paths, bounded/growing descriptor reads,
contract tampering, early failure, missing pins and semantic comparison.
The coordinator's separate delayed-write timeout probe was rerun successfully:
expected exit 1 and no descendant marker after an additional 2.3 seconds.

## Historical candidate 2 proof and current gates

Candidate 2 JSON results and old plans are preserved as historical exact inputs.
Their executable/runtime revisions may no longer match the active worktrees;
they must not be silently repinned or run against the active candidate 3 oracle.
Protocol preserves its original validator at
`history/recorded-run-candidate2/tools/recorded-run/validate.mjs`.

Historical full synthetic export passed 5 events/1 accounting, repeat byte and
semantic equality, preserved source fingerprints and omitted sentinels. Historical
actual Train candidate 2 passed the ordered/payload suite, 8 events/1 accounting,
one revision after duplicate and exact persisted records. Owner actual Train
Collector→MLflow proof also passed 19 spans and restart persistence. Those results
are not new candidate 3 Train proof.

See [the current matrix](../integration/recorded-run-matrix.md) for independent
review dispositions, actual backend evidence and coordinator browser/deployment/
rollback proof. Actual candidate3 review2 export, local real backend and LAN browser
pass. Seed-source semantics and observability edge/metadata-rollback findings are
independently closed. Formal admission remains a separate owner decision.
