# Phase 1 operator runbook

Build the pinned harness candidate with Rust 1.97.1, then start the authenticated
loopback service with a new local store path:

```text
STS2_WORKFLOW_TOKEN_SYNTHETIC=<local-secret> \
  sts2-workflow serve --listen 127.0.0.1:8787 \
  --store /approved/path/workflow-store.json --auth-profile synthetic
```

Use `validate`, `inspect` and `diff` before `run`. These commands use the
compiler boundary and do not launch a provider or game. Record the returned run
ID, then use `status` and bounded `events` pages to reconnect. Every control
command supplies the observed `--expected-revision`; a stale revision is a
conflict and should be reread rather than forced.

Pause before planned shutdown. If a run reports `NeedsOperator`, preserve the
store and event export and investigate the operation identity and authority
generation. Do not retry an unknown mutation or delete its journal entry.

Back up the store only while the service is stopped or through the owner’s
consistent backup procedure. Retain the journal for the documented retention
period and delete exported redacted files according to local policy. A corrupt,
newer or incompatible store fails closed; restore a verified backup or inspect
the evidence before admitting new work.

Rollback disables new admissions while retaining the journal and uncertain
operations. It never destructively downgrades a newer schema and never treats a
workflow checkpoint as a game save.
