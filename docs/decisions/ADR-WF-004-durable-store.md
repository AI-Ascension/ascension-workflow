# ADR-WF-004: Durable store and crash classification

## Context

Workflow progress, command acceptance, provider cost, and external effect
uncertainty must survive restart without holding a database transaction across
network or game work.

## Decision

The owner store uses transactional SQLite records for immutable definitions and
plans, monotonic workflow events, compare-and-swap revisions, invocation
intents, send markers, and conservative replay projections. Transactions end
before external dispatch. Corrupt, newer, incomplete, or incompatible stores
fail closed and retain the original evidence.

## Alternatives and compatibility

An in-memory scheduler or an auto-migrating JSON state file cannot provide the
required crash boundary, so they are not authoritative. The management adapter
may use a bounded local file store for its wire contract, while the harness
execution store remains the recovery authority.

## Failure and evidence

Crash windows classify missing send evidence as possibly dispatched. Corruption,
disk errors, and schema mismatches become explicit store failures. Evidence is
the SQLite migration/replay suite, management persistence tests, and recovery
fault matrix.

**Owner:** `sts2-harness` execution store. **Status:** Phase 1 decision.
