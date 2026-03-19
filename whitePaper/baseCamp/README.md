# primalSpring baseCamp — Coordination and Composition Validation

**Date**: March 18, 2026
**Status**: Phase 3 — capability-first architecture, live Tower + Neural API, NUCLEUS audit + handoffs, 38 experiments, 251 tests

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
| Unit tests | 233 |
| Integration tests | 16 (10 JSON-RPC + 3 live atomic + 3 neural API) |
| Doc-tests | 2 |
| Total tests | **251** (245 auto + 6 ignored live) |
| Proptest fuzz tests | 15 (IPC protocol, extract, capability parsing) |
| clippy (pedantic+nursery) | 0 warnings |
| cargo doc | 0 warnings |
| `#[allow()]` in production | 0 |
| unsafe_code | Workspace-level `forbid` |
| C dependencies | 0 (pure Rust, ecoBin compliant, `deny.toml` enforced) |
| Deploy graphs | 11 TOMLs, all nodes have `by_capability`, topologically validated |
| Discovery | Capability-first: 5-tier + Neural API + `discover_by_capability()` |
| RPC endpoints | 17 methods (including `graph.waves`, `graph.capabilities`) |
| Niche self-knowledge | `niche.rs` — 22 capabilities, semantic mappings, cost estimates |
| MCP tools | 8 typed tools via `mcp.tools.list` for Squirrel AI |
| Validation harness | `check_bool`, `check_skip`, `check_or_skip`, `run_experiment()`, `print_banner()` |
| Dishonest scaffolding | 0 (all experiments use honest skip or real validation) |

## What Changed (v0.2.0 → v0.3.0-dev)

### Live Atomic Harness (latest sprint)
1. **Absorbed biomeOS primal coordination** — `launcher/` module (binary discovery, socket nucleation, process spawn, RAII lifecycle), `harness/` module (composition startup, health checks, capability validation, RAII teardown). Pure sync Rust, no tokio.
2. **Harvested stable binaries** — beardog, songbird, nestgate, toadstool, squirrel to root `plasmidBin/primals/`
3. **3 live integration tests** — `tower_atomic_live_*` (health, capabilities, validation), `#[ignore]` guarded
4. **Neural API integration** — `spawn_neural_api()`, `start_with_neural_api()`, `RunningAtomic::neural_bridge()` — live NeuralBridge health checks
5. **exp001 harness + Neural API** — optionally spawns live Tower + Neural API, validates via NeuralBridge
6. **Launch profiles** — `config/primal_launch_profiles.toml`, data-driven socket config per primal
7. **251 tests** — up from 236 (+15); 16 integration tests (up from 10), 6 ignored (live)

### Capability-First Architecture
1. **Capability-based RPC handlers** — all coordination handlers default to capability-based validation; identity-based retained as `mode: "identity"` fallback
2. **`topological_waves()`** — Kahn's algorithm startup wave computation from deploy graph DAGs
3. **`graph_required_capabilities()`** — graphs as authoritative source of truth for capability rosters
4. **`by_capability` on all graph nodes** — 11 TOML graphs, enforced by test
5. **New RPC endpoints** — `graph.waves`, `graph.capabilities`, `coordination.probe_capability`, `coordination.validate_composition_by_capability`
6. **`check_capability_health()`** — capability-based analog of `check_primal_health()`
7. **Experiments evolved** — exp001-004, exp006, exp051 migrated from identity-based to capability-based discovery
8. **`IpcErrorPhase` + `PhasedIpcError`** — phase-aware IPC error context
9. **`discover_remote_tools()`** — spring tool discovery via `mcp.tools.list`
10. **248 tests** — up from 195 (+53); 13 integration tests (up from 9)

### Earlier v0.3.0 Changes
11. **MCP tool definitions** — 8 typed tools with JSON Schema for Squirrel AI discovery
12. **5-tier discovery** — Manifest files + socket registry fallbacks
13. **Structured provenance** — `Provenance { source, baseline_date, description }`
14. **`deny.toml`** — 14-crate ecoBin C-dep ban
15. **Proptest expansion** — 15 fuzz tests
16. **`deploy.rs` TOCTOU fix** — graceful `Result` propagation

## NUCLEUS Capability Audit (v0.3.5 — Latest Sprint)

Full audit of all 5 NUCLEUS primals + biomeOS for capability-based compliance.  
Handoff documents delivered to `wateringHole/handoffs/`.

| Primal | Severity | `health.liveness` | `capabilities.list` | 5-tier discovery | Coupling sites |
|---|---|---|---|---|---|
| BearDog | Medium | Missing | Missing | No (3-tier server / 4-tier client) | ~5 songbird/nestgate refs |
| Songbird | High | Missing | Missing | No (7-tier mixed) | ~15 beardog direct calls |
| NestGate | Medium | Missing | Missing | No (4-tier) | ~6 beardog/songbird refs |
| ToadStool | Low | Missing | Missing | **Yes** | ~10 (showcase examples) |
| Squirrel | Low | **Yes** | **Yes** | **Yes** | ~3 beardog auth fallbacks |
| biomeOS | High | N/A | N/A | N/A | ~12 Neural API bypasses |

**Total hardcoded sites traced**: 53+ (see `specs/CAPABILITY_ROUTING_TRACE.md`)

## What Remains (Phase 4+)

- Expand live harness beyond Tower (Node, Nest, Full NUCLEUS)
- biomeOS graph executor integration (graphs validated + topologically sorted but not executed)
- Beacon coordination validation (generate → encrypt → exchange → decrypt chain)
- Protocol escalation (JSON-RPC → tarpc sidecar from biomeOS v2.50)
- `cargo llvm-cov` CI integration with 90% floor
- Full experiment migration — remaining 32 experiments still use `discover_primal()` where applicable
- Implement primalSpring validation role: method availability audit, socket bypass detection, capability translation coverage

---

**License**: AGPL-3.0-or-later
