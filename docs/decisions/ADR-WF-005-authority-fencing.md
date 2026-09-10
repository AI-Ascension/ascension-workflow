# ADR-WF-005: Authority fencing and receipt recovery

## Context

Recovery is safe only when the current owner, instance generation, and receipt
retention scope are known. A stale worker must not dispatch after ownership has
changed.

## Decision

Every protected action is admitted against the current authority and generation.
Receipts are scoped to recovery, operation identity, and retention policy.
Changed epochs, missing receipts, expired evidence, and conflicting owners map
to `NeedsOperator` unless a declared authority port can reconcile them.

## Alternatives and compatibility

Blind retry, a watchdog-selected action, or a model-generated approval phrase is
not authority. The watchdog can report liveness but cannot choose actions or own
the workflow cursor.

## Failure and evidence

Stale owners and simultaneous writers are negative cases in the integration
matrix. Cross-repository evidence is pinned to exact candidate heads; no native
claim is made from source-only checks.

**Owner:** gateway authority with harness admission. **Status:** Phase 1 decision.
