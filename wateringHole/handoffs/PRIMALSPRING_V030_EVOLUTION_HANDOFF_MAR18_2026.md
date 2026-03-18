# primalSpring v0.3.0 Evolution Handoff

**Date:** March 18, 2026
**From:** primalSpring v0.3.0-dev
**To:** Ecosystem (biomeOS, all springs, all primals)
**Supersedes:** PRIMALSPRING_V020_ECOSYSTEM_ABSORPTION_EVOLUTION_HANDOFF_MAR18_2026.md
**License:** AGPL-3.0-or-later

---

## Executive Summary

primalSpring v0.3.0 completes a deep debt resolution and cross-ecosystem evolution
sprint. 195 tests (186 unit + 9 integration), 89.8% line coverage, 92.8% function
coverage. New: MCP tool definitions (8 tools for Squirrel AI), 5-tier discovery
(manifest + socket-registry fallbacks), structured `Provenance` metadata, capability
registry TOML (sync-tested), `deny.toml` (14-crate ecoBin ban), TOCTOU fix in
deploy.rs, and 15 proptest fuzz tests. Zero clippy warnings, zero unsafe, zero C deps,
zero `#[allow()]` in production.

---

## 1. What Changed (v0.2.0 → v0.3.0)

### 1.1 MCP Tool Definitions

New `ipc/mcp.rs` module exposes 8 typed Machine-Callable Protocol tools with JSON Schema
input definitions. These are discoverable by Squirrel AI via `mcp.tools.list`:

- `primalspring_validate_composition` — validate an atomic composition
- `primalspring_discovery_sweep` — discover primals in a composition
- `primalspring_neural_api_status` — Neural API reachability
- `primalspring_health_check` — full self health
- `primalspring_graph_list` — list + validate all deploy graphs
- `primalspring_graph_validate` — validate a specific graph
- `primalspring_lifecycle_status` — primal status report
- `primalspring_capabilities` — niche capabilities + mappings + costs

Pattern absorbed from: wetSpring V128 (8 tools), airSpring v0.9.0 (MCP), healthSpring V37.

### 1.2 5-Tier Discovery

Discovery expanded from 3-tier (env/XDG/temp) to 5-tier:

1. `{PRIMAL}_SOCKET` env override
2. `$XDG_RUNTIME_DIR/biomeos/{primal}-{family}.sock`
3. `{temp_dir}/biomeos/{primal}-{family}.sock`
4. Manifest: `$XDG_RUNTIME_DIR/ecoPrimals/manifests/{primal}.json`
5. Socket registry: `$XDG_RUNTIME_DIR/biomeos/socket-registry.json`

`DiscoverySource` enum extended with `Manifest` and `SocketRegistry` variants.
Pattern absorbed from: biomeOS v2.50 (manifest), Squirrel alpha.12 (socket-registry).

### 1.3 Structured Provenance

`ValidationResult::provenance` evolved from `Option<String>` to `Option<Provenance>`:

```rust
pub struct Provenance {
    pub source: String,
    pub baseline_date: Option<String>,
    pub description: Option<String>,
}
```

New constructors: `with_provenance(source, date)`, `with_provenance_full(source, date, description)`.
Pattern absorbed from: ludoSpring V14 (`with_provenance()`), hotSpring (structured provenance).

### 1.4 Capability Registry

`config/capability_registry.toml` — single source of truth for all 22 primalSpring
capabilities with metadata (description, domain). Sync-tested against `niche::CAPABILITIES`
in unit test `capabilities_match_registry_toml()`.

Pattern absorbed from: neuralSpring V118 (`capability_registry.toml`).

### 1.5 ecoBin Enforcement

`deny.toml` bans 14 known C-dependency crates (openssl-sys, libz-sys, ring, etc.).
Pattern absorbed from: airSpring v0.9.0, wetSpring V124, groundSpring V110.

### 1.6 Code Quality

- **TOCTOU fix**: `deploy::validate_live()` `.expect()` replaced with graceful `Result`
  propagation — filesystem race between parse passes no longer panics
- **`JSONRPC_VERSION` const**: Eliminates `"2.0"` string repetition across 9 sites
- **Resilience constants**: Circuit breaker and retry parameters extracted to `tolerances/mod.rs`
- **Runtime graph path**: `PRIMALSPRING_GRAPHS_DIR` env var for agnostic resolution
- **Workspace lints**: `assertions_on_constants` and `doc_markdown` evolved to
  `level = "allow", reason = "..."` for explicit justification

### 1.7 Coverage Expansion

- 157 → 195 tests (+38)
- 86.0% → 89.8% line coverage (+3.8pp)
- 89.9% → 92.8% function coverage (+2.9pp)
- 5 → 15 proptest fuzz tests (extract, dispatch, capability parsing)
- New test coverage: `or_exit` (0% → tested), `deploy.rs` degradation paths, `error.rs` variants

---

## 2. Metrics

| Metric | v0.2.0 | v0.3.0-dev |
|--------|--------|------------|
| Tests | 157 | 195 |
| Line coverage | 86.0% | 89.8% |
| Function coverage | 89.9% | 92.8% |
| Proptest fuzz | 5 | 15 |
| Capabilities | 21 | 22 |
| MCP tools | 0 | 8 |
| Discovery tiers | 3 | 5 |
| C-dep ban crates | 0 | 14 |
| Clippy warnings | 0 | 0 |
| Unsafe code | forbid | forbid |
| `#[allow()]` in prod | 0 | 0 |

---

## 3. What Blocks Phase 3

| Blocker | Severity | Notes |
|---------|----------|-------|
| Live BearDog + Songbird | P0 | Tower Atomic exp001/002 need real sockets |
| Live NUCLEUS deployment | P0 | Full NUCLEUS exp004 needs all primals |
| biomeOS graph executor | P1 | Deploy graphs validated but not executed |
| Songbird registration | P1 | Ecosystem-wide gap (no spring registers yet) |
| `cargo llvm-cov` CI | P2 | Coverage measured but not enforced in CI |

---

## 4. Patterns Available for Ecosystem Absorption

| Pattern | Where | Useful For |
|---------|-------|------------|
| MCP tool definitions with JSON Schema | `ipc/mcp.rs` | Any primal exposing tools to Squirrel |
| 5-tier discovery (manifest + socket-registry) | `ipc/discover.rs` | All primals and springs |
| Structured Provenance | `validation/mod.rs` | Any spring with validation experiments |
| Capability registry TOML (sync-tested) | `config/capability_registry.toml` | Primals with many capabilities |
| deny.toml 14-crate ban | `deny.toml` | All crates in ecosystem |
| TOCTOU-safe file reparse | `deploy.rs` | Any code that validates files twice |
| Resilience constants in tolerances/ | `tolerances/mod.rs` | Springs with IPC resilience |

---

## 5. References

- `CHANGELOG.md` — full change log
- `README.md` — updated architecture and capabilities
- `wateringHole/README.md` — updated track structure and handoff index
- `whitePaper/baseCamp/README.md` — updated baseCamp state
- `specs/CROSS_SPRING_EVOLUTION.md` — updated evolution path

---

**License**: AGPL-3.0-or-later
