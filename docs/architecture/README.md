# Phase 1 architecture

```mermaid
flowchart LR
  C[Catalog definitions] --> D[sts2-workflow management adapter]
  D --> H[sts2-harness compiler and reducer]
  H --> J[Durable journal and SQLite store]
  H --> P[Declared provider port]
  H --> G[Gateway authority port]
  G --> M[Game instance]
  J --> R[Offline replay and redacted export]
```

The delivery repository supplies definitions, capability manifests and process
cases. The harness decodes and validates a definition, computes its semantic
digest, compiles one immutable graph, and advances the reducer through bounded
steps. Dynamic regions can produce only registered analysis/decision topology;
mutation still returns to the fixed outer action boundary.

The journal is authoritative. Telemetry and exported evidence are bounded
projections. A provider or gateway outage cannot erase an intent already
committed to the journal. An uncertain external effect stays unresolved until a
declared authority port supplies correlated evidence.

The current delivery process profile is synthetic and loopback-only. It has no
native game, provider credential, remote bind or deployment behavior.
