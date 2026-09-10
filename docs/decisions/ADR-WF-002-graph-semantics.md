# ADR-WF-002: Typed graph semantics and immutable revisions

## Context

Workflow definitions need stable identity, typed node ports, deterministic
guards, bounded control flow, and a digest that is independent of cosmetic
annotations.

## Decision

The `ascension.workflow/v1` contract uses strict node-specific records. The
harness rejects duplicate keys, unknown fields, unsafe numbers, unresolved
references, unrestricted strict cycles, and invalid dependency graphs. Guards
use explicit three-valued logic. A compiled definition is bound to its semantic
SHA-256 digest; revisions are immutable and a changed digest cannot reuse a
previous runtime snapshot.

## Alternatives and compatibility

An untyped map-based definition or a general expression interpreter would permit
schema smuggling and unbounded effects, so neither is admitted. Cosmetic
annotations remain available but are excluded from semantic hashing.

## Failure and evidence

Decode, validation, canonicalization, and snapshot-digest failures stop
admission. Evidence is the harness contract artifact, canonical vectors, strict
decoder tests, compiler tests, and negative catalog fixtures.

**Owner:** `sts2-harness` workflow package. **Status:** Phase 1 decision.
