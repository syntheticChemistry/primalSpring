# primalSpring baseCamp — Coordination and Composition Validation

**Date**: March 18, 2026
**Status**: Phase 2→3 evolution — 38 experiments, 195 tests, 89.8% coverage, MCP tools, 5-tier discovery

---

## What This Is

Where baseCamp papers for other springs explore scientific questions using the
ecoPrimals infrastructure, primalSpring's baseCamp explores **the infrastructure
itself**. The "papers" are the atomics. The "experiments" are composition patterns.
The validation target is biomeOS and the Neural API.

## The Paper

See `ecoPrimals/whitePaper/gen3/baseCamp/23_primal_coordination.md` (Paper 23) for
the full baseCamp paper documenting primalSpring's validation of ecosystem coordination.

## Experiments by Track

| Track | Domain | Experiments | Key Question |
|-------|--------|-------------|--------------|
| 1 | Atomic Composition | exp001–006 | Do atomics deploy correctly? |
| 2 | Graph Execution | exp010–015 | Do all 5 coordination patterns work? |
| 3 | Emergent Systems | exp020–025 | Do Layer 3 systems emerge correctly? |
| 4 | Bonding & Plasmodium | exp030–034 | Does multi-gate coordination work? |
| 5 | coralForge | (exp025) | Does the neural object pipeline work? |
| 6 | Cross-Spring | exp040–044 | Do cross-spring data flows work? |
| 7 | Showcase-Mined | exp050–059 | Do mined phase1/phase2 coordination patterns hold? |

## Current State (v0.3.0-dev)

| Metric | Value |
|--------|-------|
| Experiments | 38 (7 tracks) |
| Unit tests | 186 |
| Integration tests | 9 (real JSON-RPC round-trips against live server) |
| Line coverage | 89.8% (llvm-cov) |
| Function coverage | 92.8% (llvm-cov) |
| Proptest fuzz tests | 15 (IPC protocol, extract, capability parsing) |
| clippy (pedantic+nursery) | 0 warnings |
| cargo doc | 0 warnings |
| `#[allow()]` in production | 0 |
| `#[expect()]` with reason | 3 (safe cast boundaries only) |
| unsafe_code | Workspace-level `forbid` |
| C dependencies | 0 (pure Rust, ecoBin compliant, `deny.toml` enforced) |
| IPC client | Real Unix socket client with JSON-RPC 2.0 |
| IPC resilience | IpcError, CircuitBreaker, RetryPolicy, resilient_call, DispatchOutcome |
| Capability parsing | 4-format (A/B/C/D) |
| Discovery | 5-tier: env/XDG/temp/manifest/socket-registry + Neural API |
| Niche self-knowledge | `niche.rs` — 22 capabilities, semantic mappings, cost estimates, registration |
| Capability registry | `config/capability_registry.toml` sync-tested against code |
| MCP tools | 8 typed tools via `mcp.tools.list` for Squirrel AI |
| Deploy graph validation | `deploy.rs` — parse, structural validate, live probe all 6 TOMLs |
| Validation | `check_bool`, `check_skip`, `check_or_skip` + structured `Provenance` metadata |
| Exit pattern | Uniform `finish()` + `exit_code()` with JSON output support |
| Dishonest scaffolding | 0 (all experiments use honest skip or real validation) |

## What Changed (v0.2.0 → v0.3.0-dev)

1. **MCP tool definitions** — 8 typed tools with JSON Schema for Squirrel AI discovery
2. **5-tier discovery** — Manifest files + socket registry fallbacks (from biomeOS v2.50, Squirrel alpha.12)
3. **Structured provenance** — `Provenance { source, baseline_date, description }` on `ValidationResult`
4. **Capability registry TOML** — `config/capability_registry.toml` sync-tested against `niche::CAPABILITIES`
5. **`deny.toml`** — 14-crate ecoBin C-dep ban
6. **Proptest expansion** — 15 fuzz tests (up from 5): extract, dispatch, capability parsing
7. **Coverage** — 86.0% → 89.8% line, 89.9% → 92.8% function
8. **`deploy.rs` TOCTOU fix** — `.expect()` replaced with graceful `Result` propagation
9. **Resilience constants** — Circuit breaker and retry parameters extracted to `tolerances/mod.rs`
10. **`JSONRPC_VERSION` constant** — Eliminates `"2.0"` repetition across 9 sites
11. **Graphs path** — Runtime-resolvable via `PRIMALSPRING_GRAPHS_DIR` env
12. **LICENSE** file + **CHANGELOG.md** added

## What Remains (Phase 3+)

- Tolerance calibration against live NUCLEUS deployment
- biomeOS graph executor integration (deploy graphs are validated but not executed)
- Songbird registration on startup (ecosystem-wide gap)
- Live primal validation (Phase 3: Tower Atomic → Phase 4: Full NUCLEUS)
- `cargo llvm-cov` CI integration with 90% floor
- Protocol escalation (JSON-RPC → tarpc sidecar from biomeOS v2.50)

---

**License**: AGPL-3.0-or-later
