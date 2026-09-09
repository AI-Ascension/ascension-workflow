# Ascension Workflow

Phase 1 is **blocked before runtime implementation**. This repository preserves the
instruction package, current source inventory, requirement and task ledgers, and the
execution handoff. It does not yet provide a workflow engine or runnable workflow catalog.

The authoritative interpreter, scheduler, protected gameplay operations, storage adapters,
and CLI/API belong in `AI-Ascension/sts2-harness`. This repository owns first-party workflow
definitions, pinned contract consumption, conformance, integration evidence, and operating docs.
There must be one gameplay runtime.

Read the [execution report](docs/evidence/EXECUTION_REPORT.md),
[source discovery](docs/discovery/BASELINE.md), and
[resume instructions](docs/operations/RESUME.md). The original package is retained as an
[inert archive](prompts/phase1/README.md); its examples are proposed seeds, not implemented behavior.

The runtime preflight verified one D1 worker on `gpt-5.6-luna` with `max` effort using structured
runtime metadata. That worker had no callable native collaboration tools, preventing D2/D3
creation. A fresh D1 recheck after the permission change reported the same limitation.
No flat or subprocess substitute was used. Rust 1.97.1 is also unavailable locally.

No Phase 1 product acceptance test has passed in this execution. Package integrity and its
12 unit tests passed; these do not establish gameplay, durability, or CLI/API behavior.
No merge, release, deployment, game launch, or gameplay provider call occurred.

Original repository material is MIT licensed. Imported instruction assets retain their own
provenance and do not acquire a new license by being stored here.
