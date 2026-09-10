# ADR-WF-009: First-party catalog and rollout boundary

## Context

Phase 1 needs substantive STS2 stage workflows and capability fixtures while
preserving the distinction between source/component completion and native game
compatibility.

## Decision

The delivery repository owns the immutable STS2 catalog, local capability
manifests, synthetic fixtures, contract consumption, and conformance evidence.
Definitions declare required and optional capabilities separately. Rollout is
explicit and can be disabled for new admissions while retaining journals and
uncertain operations; live migration and deployment are outside Phase 1.

## Alternatives and compatibility

A catalog of placeholders or native-only fixtures would not test admission or
recovery. The catalog therefore carries synthetic provenance and never embeds
provider credentials, scripts, URLs, saves, or native claims.

## Failure and evidence

Required capability absence rejects admission; optional analysis absence selects
the recorded fallback. Catalog compilation, digest checks, negative fixtures,
and exact candidate locks are the evidence.

**Owner:** delivery repository. **Status:** Phase 1 decision.
