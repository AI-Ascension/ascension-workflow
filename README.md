# Ascension Workflow

This repository is the Phase 1 delivery surface for the STS2 workflow package.
It owns first-party workflow definitions, capability manifests, pinned contract
artifacts, conformance inputs and process evidence. The authoritative compiler,
runtime, management server, protected effect boundary and durable store remain
in `AI-Ascension/sts2-harness`; gateway remains the game-instance authority.

The catalog contains 13 substantive strict and dynamic definitions covering the
campaign, setup, combat, map, reward, shop, event, rest, selection, recovery and
terminal stages. The synthetic Rust conformance driver invokes the built
`sts2-workflow` binary over authenticated loopback, validates the catalog,
checks required-capability rejection, and exercises durable run/status/events,
control, and offline replay behavior.

Useful entry points:

- [catalog documentation](docs/catalog/README.md)
- [conformance instructions](conformance/README.md)
- [workflow-v1 contract artifact](contract-artifact/workflow-v1/manifest.json)
- [execution report](docs/evidence/EXECUTION_REPORT.md)
- [resume instructions](docs/operations/RESUME.md)

The current harness implementation candidate is locally integrated and tested;
its exact commit and base are recorded in [the candidate lock](integration/candidate-lock.json).
The gateway authority work is a separate tested candidate and is recorded in the
same lock without claiming that it is present on its default branch.

The archive under `prompts/phase1/` is retained as inert provenance. D2/D3
native recursive delegation was unavailable in this execution and is recorded
explicitly in the orchestration evidence. Native game compatibility, live
provider use, merge, release, deployment and installation were not performed.

All fixtures are synthetic and carry provenance. They contain no credentials,
private prompts, raw provider output, proprietary game data or saves.
