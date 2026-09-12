# Workflow provider-session projection v1

This artifact defines the metadata-only provider-session list emitted by the
Harness for one authenticated, scoped workflow run. The Studio uses the
Harness management route directly:

```text
GET /v1/workflow-runs/{workflow_run_id}/provider-sessions
```

`value.run_id` is the authoritative workflow-management run identity. An
adapter must establish its relationship to any Context run, provider-session
scope, or native provider record explicitly; consumers must not join those
namespaces by equal strings or timeline adjacency.

The projection has no operation URLs, native thread identifiers, credentials,
raw RPC details, prepared content, provider payloads, effects, inference, or
resume authority. `game_dispatch_capability`, operation `game_effects`,
operation `auto_resume`, and envelope effects are all fixed to false or zero.
Mutation and provider execution remain Harness-only operations outside this
contract.
