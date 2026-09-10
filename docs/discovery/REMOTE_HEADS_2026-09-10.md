# Refreshed remote heads

Captured 2026-09-10 from the configured `origin` remotes with
`git ls-remote --heads origin main` (the watchdog repository uses `bootstrap` as its
default branch). These are source-refresh evidence, not integrated candidate revisions.

| Repository | Default ref | Remote head |
|---|---|---|
| `AI-Ascension/sts2-harness` | `main` | `33437ddb18f69f68d88521d947efa3568a32a3bf` |
| `AI-Ascension/sts2-gateway` | `main` | `6b6c7f2fac67de22fdf78c9fd818c6781f689ba0` |
| `AI-Ascension/sts2-game-mod` | `main` | `b9754b803cbff79836143d5c115146f65b55184c` |
| `AI-Ascension/sts2-mcp-server` | `main` | `73e777b96700917cca5ff8f6ce0f5a72009384bc` |
| `AI-Ascension/sts2-protocol` | `main` | `e5e545c2ff7166e073f6d44256016f85d7ea7e83` |
| `AI-Ascension/.github` | `main` | `92ab3ed900272703dbbe892bd333ead2a6cf1a86` |
| `AI-Ascension/ai-agent-observability` | `main` | `28a48590afb75b07590e1b78ea47dae08f5c3ade` |
| `AI-Ascension/ascension-watchdog` | `bootstrap` | `bc8ebf2054d2e3d2d74c1c7e5f775ae01952215d` |
| `AI-Ascension/ascension-workflow` | `main` | `20bb8fe06708ae70666711fc975baf475d827c2a` |

The refreshed heads advance the prior Phase 1 lock for harness, gateway, game-mod, MCP,
protocol, and organization governance. Existing isolated candidate branches remain based on
their recorded bases until the integration owner rebases or cherry-picks them with conflict
review; no remote change is silently treated as compatible.
