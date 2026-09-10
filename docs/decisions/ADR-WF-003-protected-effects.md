# ADR-WF-003: Protected action transaction and identity namespaces

## Context

External game mutations can be dispatched after a process crash or timeout.
Repeated node execution must not accidentally create a second game operation.

## Decision

The workflow identity, episode, plan, command, invocation, and game operation
namespaces remain distinct. A protected effect records intent durably before
dispatch, carries one stable operation identity, and reaches `settled` only
after correlated authority evidence. Unknown outcomes remain visible and block
unsafe continuation or retry.

## Alternatives and compatibility

Scheduler counters and model claims are insufficient evidence, so they cannot
settle an effect. Legacy gameplay operations are reused through owner-local
ports and retain their cleanup reporting.

## Failure and evidence

Intent/send, timeout, stale receipt, duplicate operation, and cleanup failures
are retained in the journal. The store and gateway candidate tests provide
component evidence; native settlement remains a separately authorized gate.

**Owner:** harness protected execution and gateway authority owners. **Status:** Phase 1 decision.
