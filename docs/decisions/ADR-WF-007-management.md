# ADR-WF-007: Authenticated headless management

## Context

Operators and automation need a bounded CLI and loopback API that can reconnect
to accepted work, inspect durable events, and issue safe commands.

## Decision

`sts2-workflow` uses the versioned management contract and the harness-owned
server. The default bind is numeric loopback and every route except health is
authenticated. Writes require a unique command ID, actor scope, and expected
revision. Read, validate, diff, and offline replay routes have no provider or
game side effects. Event pages use durable cursors and report retention gaps.

## Alternatives and compatibility

There is no browser client, wildcard CORS, unauthenticated socket, remote bind,
or API-side scheduler in Phase 1. The service owns only its declared child
processes and does not take over gateway lifecycle.

## Failure and evidence

Malformed bounded requests, auth failures, duplicate/conflicting commands,
stale revisions, slow clients, and unavailable ports produce stable error
classes. CLI and loopback process tests are required evidence.

**Owner:** harness management adapter. **Status:** Phase 1 decision.
