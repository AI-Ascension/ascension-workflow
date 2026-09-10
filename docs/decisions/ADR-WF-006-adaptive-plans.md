# ADR-WF-006: Bounded adaptive planning

## Context

Dynamic regions need topology variation without allowing a provider or model to
inject mutation, script, policy, or authority nodes into the workflow.

## Decision

Adaptive plans contain only registered analysis and decision nodes, typed edges,
bounded depth and node counts, a base definition revision, and dependency
digests. Plan bytes and acceptance are recorded before execution. Dependency
changes, supersession, stale revisions, budget exhaustion, and no-progress
limits invalidate a plan.

## Alternatives and compatibility

Free-form executable plans and model-owned scheduling are rejected. Adaptive
outputs return proposals to the fixed outer graph and use the same action
boundary as strict workflows.

## Failure and evidence

Unknown operations, cycles, mutation nodes, stale dependencies, and invalid
budgets fail closed. Dynamic plan validation, registry, runtime, and catalog
fixtures are the primary evidence.

**Owner:** harness workflow package. **Status:** Phase 1 decision.
