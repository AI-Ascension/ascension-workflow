<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/AI-Ascension/.github/main/profile/assets/banner-dark.svg">
  <img alt="AI-Ascension — Inspect how AI requests to a game get fenced, one Rust contract at a time. Bounded runtime host trace confirmed. Deterministic tests: confirmed." src="https://raw.githubusercontent.com/AI-Ascension/.github/main/profile/assets/banner-light.svg" width="100%">
</picture>

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
- [recorded-run integration suite](conformance/recorded-run.md) and
  [compatibility matrix](integration/recorded-run-matrix.md) (implementation in progress)
- [workflow-v1 contract artifact](contract-artifact/workflow-v1/manifest.json)
- [Studio acceptance matrix](docs/evidence/studio-acceptance-matrix-20260913.json) and
  [current source lock](integration/source-lock-20260913.json)
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

## Status

The catalog, conformance driver and evidence ledgers are in place and pass the
repository's CI checks. The conformance run uses synthetic fixtures against a
locally built harness binary; the live executor, provider, native game,
release and deployment gates remain unverified, and nothing here is live.

## Local validation

The CI workflow in `.github/workflows/ci.yml` runs these checks. The shell
checks need `bash`, `jq` and `sha256sum`; the Rust tools are built with the
pinned 1.97.1 toolchain.

```text
bash tools/verify-ci-inputs.sh
bash tools/verify-ci-inputs.test.sh
bash tools/verify-evidence.sh
cargo +1.97.1 test --locked --manifest-path tools/conformance-driver/Cargo.toml
cargo +1.97.1 test --locked --manifest-path tools/recorded-run-driver/Cargo.toml
```

Running the conformance driver against a built `sts2-workflow` binary is
described in [conformance/README.md](conformance/README.md).

## License

MIT licensed. This project does not distribute game files or grant rights to
them. AI-Ascension is an independent project. It is not affiliated with or
endorsed by Mega Crit or Valve.
