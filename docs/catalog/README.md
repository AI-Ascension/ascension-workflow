# STS2 first-party workflow catalog

This directory publishes the Phase 1 STS2 workflow definitions and synthetic
conformance inputs. The catalog owns reusable definitions and their admission
fixtures; `sts2-harness` owns decoding, compilation, scheduling, protected
execution, durability, and recovery.

Every definition uses the closed `ascension.workflow/v1` shape. References in a
definition are opaque IDs resolved by the local registry at
[`catalog/registry.json`](../../catalog/registry.json). There are no URLs,
scripts, provider credentials, arbitrary tool calls, or native game calls in a
definition. The registry's `local_only`, `scripts`, `provider_credentials`, and
`native_game_calls` fields are fixture assertions for conformance.

The catalog follows the package's first-party set:

| Definition family | Published entries |
| --- | --- |
| Campaign | `sts2.campaign.strict` |
| Setup | `sts2.setup.strict` |
| Combat | `sts2.combat.strict`, `sts2.combat.dynamic` |
| Map | `sts2.map.strict`, `sts2.map.dynamic` |
| Reward | `sts2.reward.strict` |
| Shop | `sts2.shop.strict` |
| Event | `sts2.event.strict` |
| Rest | `sts2.rest.strict` |
| Selection | `sts2.selection.strict` |
| Recovery | `sts2.recovery.strict` |
| Terminal | `sts2.terminal.strict` |

Strict definitions have a fixed acyclic graph. The campaign definition uses a
bounded loop with a local exit guard. Dynamic combat and map definitions add a
bounded adaptive analysis region, but both route every mutation through the
same fixed `execute_action` node as their strict counterparts. An unavailable
optional analysis capability is represented in the registered operation set for
a later provider-free fallback; an analysis error or unavailable region is an
explicit operator path in this contract seed.

Required and optional capability sets are deliberately different. Required
capabilities reject admission when absent. Optional analysis capabilities may
be absent and select the fallback recorded in the dynamic fixtures. No fixture
claims native support, a target build, a real save, a live provider, or a
model-played victory.

The fixture suite is under [`catalog/fixtures`](../../catalog/fixtures). Its
positive cases cover every published stage family. Its negative cases cover
required capability absence, optional fallback, stale generation, hidden-state
input, blocked modal mutation, provider failure, and terminal provider calls.

The checked-in workflow-v1 contract artifact remains tied to the historical
candidate revision recorded in `integration/candidate-lock.json`; that
historical pin is preserved. The context association, binding-catalog and
provider-session projection artifacts carry their own producer pins and
`SHA256SUMS` files. The current cross-repository Studio gate, exact default
heads, merged pins and explicit open gates are recorded in
[`studio-acceptance-matrix-20260913.json`](../evidence/studio-acceptance-matrix-20260913.json)
and [`integration/source-lock-20260913.json`](../../integration/source-lock-20260913.json).

The Rust process driver validates every published definition against the built
harness binary and records the required-capability negative case; it does not
copy or reinterpret the harness implementation. The driver result cited by
the current matrix is historical synthetic/component evidence, not live,
provider or native-game acceptance.
