# Phase 1 source refresh — 2026-09-11

This continuation captures current default-branch provenance after the earlier
candidate locks. The exact heads are recorded in
[`integration/source-lock-20260911.json`](../../integration/source-lock-20260911.json).
It preserves the historical candidate record and does not turn owner-repository
updates into a combined release claim.

## Source observations

The refreshed harness default head is
`e70fce12afebb38503a450c8bd242bd8ad532817`. The refreshed gateway, MCP,
game-mod, protocol, observability, Studio and delivery heads are all listed in
the source lock. The owner repositories contain changes beyond the prior
candidate baseline, including recovery/replay, peer authority, native mapping,
game integration hardening and observability persistence work. Their presence
was recorded as source provenance only; their combined contract has not been
independently exercised here.

## Executed validation

The following commands completed successfully against the refreshed local
checkouts:

- `cargo test --locked --workspace --all-targets --all-features --manifest-path repositories/sts2-harness/Cargo.toml`
  at `AI-Ascension/sts2-harness@e70fce12afebb38503a450c8bd242bd8ad532817`.
  Cargo reported 86 passing test targets and no failures.
- `cargo test --locked --manifest-path tools/recorded-run-driver/Cargo.toml`
  at `AI-Ascension/ascension-workflow@32c1d5b46d2ac62f8df9ddd2020308df78bfe8e2`.
  The driver reported 10 passing tests and no failures.
- `cargo test --locked --workspace --all-targets --all-features`
  at `AI-Ascension/sts2-gateway@f4d14091ce1f3b5327925a7a536e2c7bf7b0c56b`.
  The full workspace completed with exit code 0.
- `cargo test --locked --workspace --all-targets --all-features`
  at `AI-Ascension/sts2-mcp-server@30398bcb7a066e3641e5baaa41cb48dcaefaf65b`.
  The full workspace completed with exit code 0.
- `cargo test --locked --workspace --all-targets --all-features`
  at `AI-Ascension/sts2-protocol@58a158c7c4a722f33b0d11d8760d4a6eec03dc98`.
  The full workspace completed with exit code 0.
- `node --test tests/recorded-run-*.test.mjs`
  at `AI-Ascension/ai-agent-observability@bb04e1cd7ca41330475161fd29bcefc8cc217247`
  on Node `v24.16.0`. The recorded-run consumer reported 60 passing tests and
  no failures.

The harness test log is a local transient artifact at
`/tmp/phase1-harness-refresh-20260911.log`; this document records its command,
source revision and outcome rather than treating that local path as durable
repository evidence.

## Gates that remain open

The required D0→D1→D2→D3 native hierarchy was still unavailable in this
environment. Independent D3 review, cross-repository protocol/watchdog and
owner-authority conformance, native game compatibility and the full fault matrix
remain open. These test results therefore support refreshed component evidence
only.
