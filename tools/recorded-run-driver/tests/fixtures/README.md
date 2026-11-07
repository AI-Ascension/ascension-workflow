# Candidate matrix synthetic fixture provenance

`candidate3/` copies the 42 checksum-listed files and unchanged `SHA256SUMS`
from `AI-Ascension/ascension-workflow-studio` revision
`2636aefec997a7c1266683f56cb503cbf37d6e47`, path
`contracts/recorded-run-candidate/`. The canonical artifact owner is
`AI-Ascension/sts2-protocol`; its manifest identifies original hand-authored
schema and synthetic fixtures under MIT. No implementation source is copied.

Inventory SHA-256:
`580c1cf3be4bb3e4eb37b9acd9166808b7386b0eb84286cc0798a0d88e35bb35`.
The compiled candidate command verifies all copied bytes before running.
The regression replaces Node with synthetic receipts. Passing it establishes
the delivery command's receipt checks, not owner import, native, browser,
provider, or deployment acceptance.
