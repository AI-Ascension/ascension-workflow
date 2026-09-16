# Process conformance

The delivery repository supplies catalog inputs and invokes the built
`sts2-workflow` binary. It does not decode workflow graphs or implement a
second scheduler. The driver starts an authenticated loopback service with a
synthetic capability manifest, validates every published definition, exercises
one durable run/status/events path, and checks a required-capability negative
case.

Run it from this repository after building the harness candidate:

```text
cargo run --manifest-path tools/conformance-driver/Cargo.toml -- \
  --binary /path/to/sts2-workflow \
  --catalog workflows/sts2 \
  --capabilities catalog/capabilities/synthetic-full.json \
  --missing-capabilities catalog/capabilities/synthetic-missing-map.json \
  --negative workflows/sts2/map.strict.json
```

All service traffic is synthetic and loopback-only. The driver leaves no store
or credentials in Git; temporary files are removed on exit.

The consumer fixtures also cover the metadata-only context association,
context-binding catalog and provider-session projection contracts. Their
producer revisions and exact bytes are recorded in the per-artifact manifests
and `SHA256SUMS` files. See the
[`Studio acceptance matrix`](../docs/evidence/studio-acceptance-matrix-20260913.json)
and [`current source lock`](../integration/source-lock-20260913.json) for the
cross-repository pin audit. A passing synthetic driver run does not close the
live executor, readiness, provider or native-game gates.

## Served live admission contract

The separate [`serve-workflow` case](cases/serve-workflow-v1.json) pins the
merged Harness #220 producer in
[`served-live-source-lock.json`](../integration/served-live-source-lock.json).
Its process driver starts that producer's `serve-workflow`, compares the
reported `/v1/workflow-targets` response and `/v1/capabilities` manifest to
separate checked-in expectations, then invokes the pinned `sts2-workflow
validate` command with the manifest fetched from the running service. The live
graph is the same strict observe/decide/action/terminal graph used by Harness's
served process tests.

This check verifies served catalog and validation-contract alignment. It does
not launch Gateway or MCP; their separate pinned process acceptance remains
recorded by the Harness peer-contract workflow and the source lock. The
provider-capabilities fixture is only service startup input and causes no
provider call. Neither check establishes native game or aggregate issue
acceptance.

The authored composed failure fixture is
[`studio-acceptance-matrix-v1.json`](cases/studio-acceptance-matrix-v1.json).
It records exact-pin, unauthorized, unavailable, stale, oversized,
cancellation, restart and unknown-outcome expectations as `unverified` until
the owner boundaries and independent review are available.
