# Delivery repository instructions

Follow the user's explicit task and the Phase 1 package preserved under `prompts/phase1/`.
Read `docs/evidence/EXECUTION_REPORT.md`, `execution-state.json`, `integration/source-lock.json`,
and the live requirement/task ledgers before resuming. Recheck remote and working-tree state.

This repository owns workflow catalog, pinned harness contract artifacts, conformance clients,
integration evidence, and documentation. The only workflow interpreter and scheduler belong in
`AI-Ascension/sts2-harness`; owner repositories retain their existing authority. No browser/UI,
game implementation, duplicate canonical contract owner, or sibling implementation dependency.

Use actual D0→D1→D2→D3 native delegation, all descendants on `gpt-5.6-luna` with effort `max`.
D3 must not spawn. Preserve requested, accepted, and observed metadata separately. A missing
recursive tool is a blocker; do not flatten the hierarchy or launch external process chains.
Apply the smaller of 12 live descendants and the runtime's limit, counting parked parents.

Preserve existing work. Use isolated branches/worktrees, bounded path leases, narrow staging,
and exact-commit verification. Root owns shared ledgers, manifests, locks, and GitHub delivery.
Read owner instructions and complete required source/standards discovery before source writes.

Use Rust for future executable conformance tooling after its standards profile is admitted.
No active Python product/tooling source. The user-supplied package archive is inert provenance,
including its offline Python archive validators; it is not a product standards exception.
Do not copy a sibling's implementation or add empty crates/checks. Use `apply_patch` for edits.

Retain no credentials, private prompts or raw provider responses, private reasoning, proprietary
game files, saves, personal filesystem paths, or unsanitized game text. Synthetic fixtures must
carry provenance. Preserve all WF/AT/FI IDs and distinguish unavailable checks from passing ones.

The initial bootstrap is documentation/data only. Check whitespace, local links, strict JSON,
ledger coverage/dependencies, archive bytes/provenance, and evidence digests. Rust product checks
remain blocked until a real implementation and toolchain exist; archive tests cannot replace them.
Task completion still requires a different D3 verifier and exact integrated validation.

Scoped commits, pushes, draft PRs, and private repository creation are authorized by the launch.
Merge, release, deployment, installation, host restart, live provider expenditure, game/profile
mutation, remote exposure, and global configuration changes require separate authorization.
Never reinterpret a tool permission change as product authorization.
\n## Workspace, branch, and artifact hygiene\n\nBefore creating an isolated checkout, declare the exact absolute worktree path and the exact branch name. Create it only with `git worktree add <absolute-path> -b <branch-name>` (or attach the explicitly named existing branch). Do not create branch copies, sibling checkouts, backup trees, archive trees, or `*-tmp*` directories as substitutes for a Git worktree; do not use generated or random paths for branch isolation.\n\nPerform edits and validation only in the declared checkout. Put build output, test fixtures, logs, and other derived artifacts in the repository's designated rebuildable output directory (such as `target/`) or a single declared task scratch path, never beside repositories or directly under `/home/agent`. Remove task scratch/output after it is no longer needed, and remove the worktree with `git worktree remove <the-same-absolute-path>` once its branch is integrated or abandoned. Preserve source, committed evidence, and any path explicitly retained by the task owner.\n