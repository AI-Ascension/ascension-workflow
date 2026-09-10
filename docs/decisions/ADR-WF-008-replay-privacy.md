# ADR-WF-008: Structured replay and privacy boundary

## Context

Offline replay must reproduce recorded workflow decisions without calling a
provider or mutating a game. Logs and exports must remain safe when inputs are
untrusted or sensitive.

## Decision

Replay consumes the exact definition/compiler/registry/policy digests and
recorded normalized inputs, plans, typed provider outcomes, effect results,
commands, and clock events. It reports the first stable divergence. Retained
decisions are structured projections; raw prompts, chain-of-thought, credentials,
private game bytes, and unsanitized game text are excluded. Exports are bounded
and redacted.

## Alternatives and compatibility

Regenerating a plan or calling a provider during replay would make replay
non-deterministic and unsafe. A workflow checkpoint is not a game save or
rollback point. Fresh-game replay remains a separately authorized operation.

## Failure and evidence

Missing privacy-approved material marks a run not fully replayable. Redaction,
malicious-text, no-side-effect replay, and divergence tests provide evidence.

**Owner:** harness journal/replay and delivery evidence owners. **Status:** Phase 1 decision.
