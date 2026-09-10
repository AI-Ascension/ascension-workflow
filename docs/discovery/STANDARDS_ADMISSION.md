# Phase 1 delivery standards admission

The task authorizes owner-local Rust implementation and conformance tooling. The
[profile](../../standards-profile.toml) applies the current harness Rust 1.97.1/edition 2024,
format/lint and file budgets to this delivery repository without importing harness implementation
or pretending an absent organization standards bundle exists.

Organization `GOVERNANCE.md` and `CONTRIBUTING.md` at
`dbdc1f19b623a5bfe67ab86a8e986b07201c85ca` establish owner authority, MIT provenance and
review/evidence rules. Harness standards at `fc44d3ef65fefa6d13ecd5f690e5335a6ef60080`
supply the admitted engineering limits. This is an original delivery policy declaration,
not a generated organization artifact or a change to public organization metadata.

Executable tooling may check artifacts, invoke the built harness and verify process results.
It must not parse graphs into a competing execution model or depend on sibling source paths.
Canonical workflow schemas come only from a pinned harness export. The original package ZIP
remains inert provenance, including its Python archive checks.

Root admits this profile within the user's existing scope. The dependency-free Rust
`tools/conformance-driver` implements the admitted process boundary: it starts the authenticated
loopback service, validates the 13 catalog definitions, checks a required-capability rejection,
and exercises durable run/control/replay routes. It does not interpret workflow graphs.
Independent D3/G6 review and the remaining cross-repository gates are still required.
