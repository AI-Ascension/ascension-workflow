# Historical candidate 2 compatibility matrix

This is the preserved pre-review-wave snapshot. Its then-current role rows and
pending backend/rollback/review claims are historical, superseded by
`../recorded-run-matrix.md`. It is not the active acceptance ledger.

Status: candidate 2 actual Train interoperability and VM-origin deployed-browser
proof verified, 2026-09-11. All CLI interfaces resolved; actual and full synthetic
process plans executed. Independent protocol findings still hold admission open.
Verified deployment evidence URL: `http://192.168.1.146:4173/`.
Version `1.0.0-candidate.2`, schema SHA-256
`d5098e5f969d99707d3ad1d97acdbc803285b93f1eb1dcfe5dc3f63c534192af`,
whole-artifact inventory SHA-256
`41d760f8c41064c4e6b49a48dbe6e1a6c8f2a9958afbc50374986a54858fd598`.
These are proposed candidate pins, not release admission. Coordinator-exported
Train bundle byte digest `2575de7ba78baa30d1036233a0fc234a5b0e1e6d8988e10df62c139aaee37f8b`
passed the actual CLI and payload pipelines. Baseline revisions below identify inspected worktrees, not tested
recorded-run implementations. Source reports are in the workspace's
`recorded-run-integration/<repository>.md`; this matrix owns the delivery view.

| Repository | Inspected baseline | Role and current disposition | Next evidence |
| --- | --- | --- | --- |
| sts2-protocol | `0bc689eabc5542ede2b09b030d9ea32daa8a73e7` | Shared envelope owner; proposed candidate reviewed with changes | Candidate bytes, checksum inventory, validator CLI and vectors |
| sts2-harness | `4c5b1012efb0b8ffd0e95b76654678b170de4720` | Exporter; source-derived Train format/allowlist mapping | Export CLI, deterministic artifact, row reconciliation and finalization boundary |
| ascension-workflow-studio | `5faefecdb1bad62dd30763ca9d44758c556eac97` | Consumer implementation in progress; contrast fix preserved by owner | Import CLI/report adapter, real browser import and authoring proof |
| ai-agent-observability | `89539a6e7754b389f8eac148ba8a49c3892cddd8` | Independent consumer architecture/review | Import CLI, unknown accounting, durable duplicate/update and tracking persistence proof |
| ascension-workflow | `45341c7bdfa8d51eb6b52aec1ccc5b9f1a73a688` | Rust process runner and this matrix implemented locally | Real owner interfaces and same-bundle comparison |
| sts2-gateway | `f5582995da2ac9b4ba7f66602062a62c48648537` | Source-derived metadata compatibility; no change justified; owner reports policy/format/Clippy/tests passed | Revisit only if exporter identifies missing gateway-owned metadata |
| sts2-mcp-server | `a6b9215db1ddeeddabe4c111ed3b49476fb86e54` | Source-derived transport metadata mapping; no portable importer | Preserve MCP stream order/redaction and distinguish acknowledgement from settlement |
| sts2-game-mod | `d8b46bccbee9eff108efdab9c8fc9b27dbf2c034` | Source-derived authoritative receipt bindings; no code change justified; owner artifact checks/policy passed | Validate actual receipt bindings against adapter mapping |
| ascension-watchdog | `bc8ebf2054d2e3d2d74c1c7e5f775ae01952215d` | Unsupported/non-consumer baseline; no product importer or justified dependency | Revisit only with supervisory use case; history cannot trigger recovery |
| .github (org-governance) | `e0cf8065728ae427f21a2b277b0601108fc1899c` | Proposed ownership/admission guidance | Consume exact technical evidence and maintainer admission; owner tests report missing PyYAML |

## Acceptance ledger

| Gate | Current result | Owner |
| --- | --- | --- |
| RRI-01: pinned contract and conformance CLI | Finalized candidate 2 inventory pinned; actual Train validation passed; admission/review remain owner gates | Protocol |
| RRI-02: actual Train deterministic export and count reconciliation | Host export exit 0; 240 rows = 9 emitted + 231 filtered; all nine original fingerprint entries unchanged; full synthetic pipeline and repeat byte/semantic identity passed; repeated real export not independently exercised | Harness/coordinator |
| RRI-03: canonical validation before either consumer | Passed actual CLI ordering on Train ZIP and synthetic legacy-failed ZIP | Protocol/workflow |
| RRI-04: identical ZIP bytes through two consumers | Actual Train ZIP passed both consumer CLI/API checks unchanged; six goldens also passed | Workflow/consumers |
| RRI-05: common identities, counts, evidence and accounting agree | Actual Train CLI agrees on all eight summary fields; actual payload comparison agrees on accounting/identities/omissions with zero findings | Workflow/consumers |
| RRI-06: legacy, optional-stream and workflow-recording positives | Six protocol goldens include supported legacy, missing accounting, partial source, optional unknown profile and completed-gameplay cases; all consumers pass. Workflow recording profile remains unsupported | Harness/protocol |
| RRI-07: archive/version/tamper/truncation/bounds/redaction negatives | All 21 pinned invalid ZIPs rejected by all three implementations; broader bounds/adversarial review remains owner evidence | Protocol/consumers |
| RRI-08: repeat/update import and tracking persistence | Actual Train's exact 8 original events and 1 accounting record match stored revision after duplicate; one revision; owner synthetic Collector/MLflow revision/restart proof exists; actual Train backend delivery and Laminar unverified here | Observability |
| RRI-09: browser import, failure/re-import and authoring/contrast | Passed coordinator VM-origin Chromium against deployed Train listener: actual file import, numeric/ns/accounting, duplicate no-op, invalid import preserves prior, no live controls, six readable designer nodes; zero errors/unexpected requests | Studio/coordinator |
| RRI-10: LAN deployment, served asset digests, lifetime and rollback | Four served files exactly match tested build; enabled user service same PID 3826423 after fresh session, Linger=yes, LAN bind/firewall verified; rollback preserved/hash-verified but activation exercise and physical LAN-client proof remain open | Coordinator |
| RRI-11: independent review and repository checks | Local checks and integration reruns pass; independent protocol review has 1 high, 5 medium, 1 low finding; admission held for resolution/revalidation | Workflow/coordinator |

The earlier synthetic CLI proof used Studio `2959d09b8e58e92d9777907c60162a928ecca096`,
protocol base `0bc689eabc5542ede2b09b030d9ea32daa8a73e7`, and observability base
`89539a6e7754b389f8eac148ba8a49c3892cddd8`, with changed runtime files pinned
in `recorded-run-consume-plan.json`. ZIP SHA-256:
`a4f7b97df3ee55c35bb42c97ef57518e7a634c700c332025e08db4a3af785c1c`;
semantic digest: `de818d975e78dc7ac9bc235a7192abbfeef9859308bffb0dc34a54d12b198bc9`.
It has 8 events, 1 accounting record, process completed, episode_failed gameplay,
and unknown action/request/outcome evidence. It is a protocol synthetic fixture.
See `recorded-run-candidate2-results.json` for all vector outcomes and
`recorded-run-candidate2-pin.json` for per-file artifact pins.

## Actual Train evidence

The coordinator's host exporter exited 0 using binary SHA-256
`6c2cb070d1812133e3d136a151a32d98ade52ec20498b99f6b573430072d3a89`.
Its original build digest is separately recorded as
`c2121c4ab87d8238846a13c050da76f9a5b9b342d71f6bd2277d87a012faf0b6`.
Workflow verified the local transport binary matches the host-executed digest
and populated the source-export plan with it. The source directory stays on
Train; this thread consumes the coordinator export evidence and never accesses it.

Actual Train semantic digest:
`ebf13df215c02d1b8b31821e25a8696cb2901ca49cea413509be877ba8f36b60`.
Source format `seed-readiness-controller-release-v2`, producer/adapter versions
literally `0.0.0`, adapter source revision `unreleased-candidate`, no component
versions supplied. These provenance limits remain visible; binary integrity
does not establish a source-commit linkage or a released exporter version.

| Source stream | Input rows | Emitted rows/output records | Filtered | Unsupported/rejected |
| --- | ---: | ---: | ---: | ---: |
| trajectory | 6 | 6 | 0 | 0 |
| decisions | 1 | 1 | 0 | 0 |
| provider-accounting | 1 | 1 | 0 | 0 |
| result | 1 | 1 | 0 | 0 |
| manifest | 1 | 0 | 1 | 0 |
| mcp | 230 | 0 | 230 | 0 |
| Total | 240 | 9 | 231 | 0 |

Eight event records plus one accounting record. Filter reasons are
`private_source_metadata` and `raw_mcp_disallowed`. Completeness is partial and
source snapshot unverified, despite reconciled scanned-row counts. Process
failed, gameplay episode_failed, and action/request/outcome evidence unknown.
There are two unknown action outcomes and zero settled action outcomes.
Provider completion is independent of those failed process/episode states.

The first actual CLI proof used the Studio revision above, protocol base above, and
observability `9deae812443dd56e95788af657e06a1fb75b731e` plus pinned runtime bytes.
Reports: `integration/recorded-run-train-results.json`, with digests of coordinator
export/transport reports, all three source summaries, local CLI proof and the
independent payload comparison. Payload comparison had no standalone observation
records to compare in this artifact; it does not claim otherwise.

## Current execution and deployment evidence

The current actual Train rerun and full synthetic source-export plan passed with
observability commit `4c4486b5a6a691268c4f7e4b47c2be425082be85`; runtime byte pins
were unchanged. The six-golden/21-invalid matrix rerun also passed all three
implementations. `integration/recorded-run-current-execution.json` records
commands/stage outcomes and digests. The synthetic source directory contains six
data files plus provenance README: all seven fingerprints remained unchanged.
Its repeated ZIP digest is
`43f2911b4d7d348dce0f97ecd623402175d46398f194b0425704228a603788ab`,
semantic digest `632f7c9b2e1a0503098492e8eaba5c1932e5be3d8d86966bf1ce268f8082e8dc`.
Five events plus one accounting record; 8 input rows = 6 emitted + 2 filtered.
Private sentinels were absent. An earlier missing-counter fixture was rejected
before consumers; that negative outcome is retained, not represented as a pass.

Deployment release `4984edd81b7562bc561a2b592c95f8202f8e0cdbe59d708ae8b0307af3a68231`
matches all four served files. The user service is enabled with Linger=yes and
the same PID across independent Cockpit sessions. Coordinator firewall evidence
allows TCP 4173 from `192.168.1.0/24`; the listener binds `192.168.1.146:4173`.
Rollback release `6015d7eeb321666766b590ec90a1da80672d78e3a3026e4bb9df0c094d741699`
is preserved with five verified files. Actual rollback activation is untested.
The coordinator's operations guide supplies stop/restart/log and rollback steps.

`integration/recorded-run-deployment-results.json` binds source before/after,
cross-consumer persistence, browser JSON/PNGs, served inventory, service reconnect,
firewall and rollback evidence by hash. The workflow lead compared fingerprints,
stored original records and the build inventory, and inspected supplied accounting
and designer screenshots. Browser execution was coordinator-owned, VM-origin;
no independent physical LAN-client check is claimed.

Independent review is now concrete, not merely awaiting assignment. Open findings
cover process-result source-stream binding (high), required diagnostic digests,
identity separation, optional-profile grammar, disposition reason classes,
cross-file record bounds (medium), and pre-copy archive bounds (low). The existing
Train case and published vectors pass despite those uncovered gaps. Protocol and
affected consumers must resolve them with version/pin propagation and fresh
conformance. No owner admission or organization completion follows from this matrix.

The Train source format is `seed-readiness-controller-release-v2`; accounting
declares `sts2.provider-accounting-v1`. Sanitized coordinator probes show provider
execution `completed` and an `episode_failed` event. Episode failure is not an
authoritative game defeat; provider execution completion is not game success.
Exporter/source reconciliation must retain these scopes without inventing a
workflow identity, settled action, or completed gameplay.

Execute [the suite](../conformance/recorded-run.md) with the owner-populated
[plan](recorded-run-plan.json). New evidence may advance one gate without
advancing others. No schema validation, mock process, source report or single
consumer can establish organization-wide compatibility.
