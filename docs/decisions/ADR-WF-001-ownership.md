# ADR-WF-001: One authoritative workflow runtime

## Context

The delivery repository publishes STS2 definitions and conformance inputs. The
harness owns decoding, compilation, scheduling, protected effect handling, and
durable recovery. Gateway, MCP, protocol, and watchdog repositories retain their
existing authority boundaries.

## Decision

`AI-Ascension/sts2-harness` is the only workflow interpreter and scheduler.
`ascension-workflow` consumes versioned artifacts and drives public harness
interfaces. A gateway remains authoritative for game instances and the harness
owns only declared child processes.

## Alternatives and compatibility

A delivery-side interpreter would duplicate graph semantics and make exact-head
conformance meaningless, so it is rejected. Existing legacy runners remain
available behind owner-local ports until parity evidence supports a release
decision.

## Failure and evidence

An unavailable owner port fails closed as `Unavailable` or `NeedsOperator`; it
does not create a fallback scheduler. Evidence is the harness candidate lock,
process conformance driver, and cross-repository authority contract.

**Owner:** harness and delivery maintainers. **Status:** Phase 1 decision.
