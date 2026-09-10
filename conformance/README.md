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
