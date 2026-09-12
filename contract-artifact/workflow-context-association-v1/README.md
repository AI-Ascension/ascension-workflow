# Workflow-context association v1

This artifact defines the redacted association emitted by the harness for the
current workflow cursor. It allows Studio and Context Console to establish that
a context record belongs to a particular workflow invocation without merging
their independent identity namespaces.

The producer is `sts2-harness`. Studio presents the association through its
same-origin management adapter. Context Console decodes it as a bounded,
read-only artifact before using the association to navigate already-authorized
context evidence.

The artifact contains no retained component content, provider credentials, raw
provider payload, or generic URL. A context association is either fully bound
(`available`) or carries no context identities (`unavailable` or
`not_applicable`). Consumers must not fill missing identifiers from matching
run IDs, timestamps, or timeline position.

Capture state is evidence of a local lifecycle boundary only. `prepared`,
`input_write_completed`, `provider_receipt_reported`, and `unknown` have
different meanings. A consumer must not infer provider receipt, game action, or
context-control authority from this response.

The management endpoint proposed by the producer is:

```text
GET /v1/workflow-runs/{workflow_run_id}/context
```

It is authenticated, scoped to the workflow run, and read-only. Context
editing, retained-content access, memory retrieval, session inspection, pause,
commit, and resume retain their separate owner capabilities.
