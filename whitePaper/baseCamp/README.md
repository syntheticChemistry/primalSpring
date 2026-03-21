# primalSpring baseCamp — Coordination and Composition Validation

**Date**: March 21, 2026
**Status**: Phase 4 — Tower STABLE (24/24 gates), Squirrel AI composition validated, 40 experiments, 264 tests

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
| 8 | Live Composition | exp060–061 | biomeOS Tower deploy + Squirrel AI composition |

## Current State (v0.4.0)

| Metric | Value |
|--------|-------|
| Experiments | 40 (8 tracks) |
| Unit tests | 239 |
| Integration tests | 23 (10 JSON-RPC + 11 live Tower + 2 Squirrel AI) |
| Doc-tests | 2 |
| Total tests | **264** (249 auto + 15 ignored live) |
| Proptest fuzz tests | 15 (IPC protocol, extract, capability parsing) |
| clippy (pedantic+nursery) | 0 warnings |
| cargo doc | 0 warnings |
| `#[allow()]` in production | 0 |
| unsafe_code | Workspace-level `forbid` |
| C dependencies | 0 (pure Rust, ecoBin compliant, `deny.toml` enforced) |
| Deploy graphs | 11 TOMLs, all nodes have `by_capability`, topologically validated |
| Discovery | Capability-first: 5-tier + Neural API + `discover_by_capability()` |
| RPC endpoints | 17 methods (including `graph.waves`, `graph.capabilities`) |
| Niche self-knowledge | `niche.rs` — 25 capabilities, semantic mappings, cost estimates |
| MCP tools | 8 typed tools via `mcp.tools.list` for Squirrel AI |
| Validation harness | `check_bool`, `check_skip`, `check_or_skip`, `run_experiment()`, `print_banner()` |
| Dishonest scaffolding | 0 (all experiments use honest skip or real validation) |
| Tower Atomic | **STABLE** — 24/24 gates passing with plasmidBin binaries |
| Squirrel AI | Composition validated (Tower + Squirrel + Anthropic Claude) |

## What Changed (v0.3.0 -> v0.4.0)

### Tower Stability Sprint (v0.4.0)
1. **All 24 Tower Atomic gates pass** — cross-primal changes to beardog (5-tier socket discovery), songbird (`songbird-crypto-provider` crate, Neural API routing), biomeOS (capability-based enrollment, graph executor, federation)
2. **Squirrel AI Composition** — Tower + Squirrel (beardog + songbird + squirrel) with live Anthropic Claude inference via Neural API capability routing
3. **exp060_biomeos_tower_deploy** — biomeOS-orchestrated Tower deployment via `neural-api-server` + bootstrap graph
4. **exp061_squirrel_ai_composition** — 3-primal composition with live AI `ai.query` calls, API key passthrough, Tower post-query health validation
5. **7 new integration tests** — `tower_zombie_check`, `tower_discovery_peer_list`, `tower_tls_handshake`, `tower_tls_internet_reach`, `tower_tls_routing_audit`, `tower_squirrel_ai_query`, `tower_squirrel_composition_health`
6. **`PrimalProcess::from_parts()`** — construct from pre-spawned components for custom spawn logic
7. **`LaunchProfile::passthrough_env`** — forward parent env vars (API keys, GPU config) to child processes
8. **Abstract socket discovery** — Squirrel Universal Transport integration via Linux abstract namespace
9. **264 tests** — up from 251 (+13); 23 integration tests, 15 ignored (live)

### Earlier (v0.2.0 -> v0.3.0)

See CHANGELOG.md for the full v0.3.0 evolution: capability-first architecture, live atomic harness, topological waves, 5-tier discovery, MCP tools, IPC resilience stack, proptest expansion.

## NUCLEUS Capability Audit (v0.3.5)

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

## Co-Evolution Strategy

| Phase | Focus | Partners | Gate Target | Status |
|---|---|---|---|---|
| Tower Stability | All 24 Tower gates | beardog, songbird, biomeOS | Gates 1–6 (24/24) | **DONE** |
| Tower + Squirrel | AI composition | + squirrel | AI gates | **DONE** |
| Nest Atomic | Storage gates | + nestgate | Storage gates | **NEXT** |
| Node Atomic | Compute gates | + toadstool | Compute gates | Planned |
| Full NUCLEUS | All gates | + all primals | All gates | Future |

See `specs/TOWER_STABILITY.md` for the full 24-gate acceptance criteria.

## What Remains (Phase 5+)

- Nest Atomic gate definition and nestgate integration
- Node Atomic with toadStool compute capability registration
- Full NUCLEUS composition (all primals + all gates)
- Beacon coordination validation (generate -> encrypt -> exchange -> decrypt chain)
- Protocol escalation (JSON-RPC -> tarpc sidecar from biomeOS v2.50)
- `cargo llvm-cov` CI integration with 90% floor
- Neural API auto-registration for Squirrel `ai` capabilities

---

**License**: AGPL-3.0-or-later
