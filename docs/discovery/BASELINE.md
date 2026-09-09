# Source discovery, 2026-09-09

Evidence: current Git/API metadata and bounded source inspection. Full source admission is
incomplete. Exact repository IDs, heads, branch names, visibility, open issues/PRs, toolchain pins,
tracked policy digests, and artifact inventory digests are in
[`integration/source-lock.json`](../../integration/source-lock.json).

`sts2-harness` main is `fc44d3ef65fefa6d13ecd5f690e5335a6ef60080`, identical to the package baseline.
The fresh local main checkout is clean. This does not mean its open feature branches are absent
or accepted. No existing owner repository was edited by this execution.

## Changes that must be assessed before implementation

| Candidate | Observed exact head | Relevant overlap |
| --- | --- | --- |
| [Harness PR 40](https://github.com/AI-Ascension/sts2-harness/pull/40) | `baabf9767264de11e09cd65cc7262ba36664ec2d` | Durable execution, SQLite dependency, authenticated worker commands, cancellation, recovery, allocation context |
| [Harness PR 41](https://github.com/AI-Ascension/sts2-harness/pull/41) | `d549027ad15abc66c901f1e9c4a99e0c53122447` | Map context, expert rest/selection, retained receipt identities, replay, runner changes |
| [Harness PR 45](https://github.com/AI-Ascension/sts2-harness/pull/45) | See machine-readable lock | Explicit seeded setup/campaign invocation |

These are unmerged dependency candidates, not an integrated baseline. PR 40's body cites
`ffa3e61a...`, while the fetched head is `baabf976...`; use current exact-head source and checks,
not the stale body claim. PR descriptions and reported CI results are source-derived evidence
until independently reproduced. No candidate was cherry-picked or merged.

## Inspected harness seams

The full current `episode/runner.rs`, `runner_impl.rs`, and `runner_actions.rs` were read.
`run_inner` constructs an in-memory machine/ledger per invocation. `prepare_action` derives
operation identity from generation and step; durable multi-run identity needs explicit analysis.
`run` currently returns a shutdown error when cleanup fails, replacing an earlier outcome; the
Phase 1 requirement to preserve primary and cleanup failures needs a regression and scoped change.
These are source observations, not executed defect tests.

`EpisodeRuntimePort` owns launch, observation, legal catalog, and dispatch seams and extends
barrier/recovery/shutdown ports. Extraction must retain that boundary. The 4,096-step cap remains
in current source. No reusable workflow kernel, immutable definition registry, or workflow CLI
was implemented during this execution.

Before write admission, read all construction sites and the complete runner steps/recovery,
policy router, observation/catalog, action-plan, state machine, postconditions, idempotency,
stability barrier, adapter implementations, and matching tests at the selected candidate.
Inspect PR 40's durable modules and PR 41's changes through their actual dependencies; do not
create a competing durable runtime from this incomplete discovery.

## Other owners and standards

Gateway's current architecture/compatibility documents still describe the attached executable's
lack of TTL/renewal and durable boot epoch, separately from library seams. This is source-derived
and applies to the pinned default branch only; open recovery PRs require inspection. A workflow
checkpoint cannot establish host receipt persistence or safe authority transfer.

Watchdog bootstrap documents describe separate watchdog/gateway/harness databases and label
their architecture proposed. Its standards profile pins an older `.github` source bundle.
That historical profile is not automatically the new repository's admitted standards profile.
Observability's AGENTS file keeps deployment configuration separate from harness authority.

The current organization registry is `metadata/repositories.yml`; it is a metadata rollout
registry. No `.github/standards/repositories.yaml` was invented, no registry was edited, and no
historical standards tool was copied from a sibling implementation. Current organization
contribution and governance docs provide MIT licensing, PR-based changes, evidence vocabulary,
and owner language boundaries. A concrete delivery tooling profile still needs admission in T02.

Harness policy pins Rust 1.97.1, edition 2024, rustfmt width 100, strict Clippy, and file budgets.
The installed environment has Git 2.39.5, GitHub CLI 2.23.0, and Codex CLI 0.153.4, but no
`cargo`, `rustc`, or `rustup` on PATH. No toolchain was silently downgraded or installed.

The source lock inventories eight relevant repositories. Core and map visualizer were not
cloned because no domain change or visual implementation is proposed. No full cross-owner
source, artifact-conformance, or native compatibility certification is claimed.
