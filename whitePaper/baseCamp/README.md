# primalSpring baseCamp — Coordination and Composition Validation

**Date**: March 22, 2026
**Status**: Phase 10 — GRAPH EXECUTION + PROVENANCE READINESS (87/87 gates), 49 experiments, 253+ tests

---

## What This Is

Where baseCamp papers for other springs explore scientific questions using the
ecoPrimals infrastructure, primalSpring's baseCamp explores **the infrastructure
itself**. The "papers" are the atomics. The "experiments" are composition patterns.
The validation target is biomeOS and the Neural API.

## The Paper

See `ecoPrimals/whitePaper/gen3/baseCamp/README.md` (Paper 23 section) for
the full baseCamp paper documenting primalSpring's validation of ecosystem coordination.

## Experiments by Track

| Track | Domain | Experiments | Key Question |
|-------|--------|-------------|--------------|
| 1 | Atomic Composition | exp001–006 | Do atomics deploy correctly? |
| 2 | Graph Execution | exp010–015 | Do all 5 coordination patterns work? (3/5 live) |
| 3 | Emergent Systems | exp020–025 | Do Layer 3 systems emerge correctly? |
| 4 | Bonding & Plasmodium | exp030–034 | Does multi-gate coordination work? |
| 5 | coralForge | (exp025) | Does the neural object pipeline work? |
| 6 | Cross-Spring | exp040–044 | Do cross-spring data flows work? |
| 7 | Showcase-Mined | exp050–059 | Do mined phase1/phase2 coordination patterns hold? |
| 8 | Live Composition | exp060–070 | Tower + Squirrel AI + Nest + Node + NUCLEUS + Graph Overlays + Cross-Primal Discovery |

## Current State (v0.7.0)

| Metric | Value |
|--------|-------|
| Experiments | 49 (8 tracks) |
| Total tests | **253+** (unit + integration + doc-tests, 42 ignored live) |
| Proptest fuzz tests | 15 (IPC protocol, extract, capability parsing) |
| clippy (pedantic+nursery) | 0 warnings |
| cargo doc | 0 warnings |
| `#[allow()]` in production | 0 |
| unsafe_code | Workspace-level `forbid` |
| C dependencies | 0 (pure Rust, ecoBin compliant, `deny.toml` enforced) |
| Deploy graphs | 18 TOMLs, all nodes `by_capability`, topologically validated |
| Discovery | Capability-first: 5-tier + Neural API + `discover_by_capability()` |
| RPC endpoints | 17 methods (including `graph.waves`, `graph.capabilities`) |
| Niche self-knowledge | `niche.rs` — 37 capabilities, semantic mappings, cost estimates |
| MCP tools | 8 typed tools via `mcp.tools.list` for Squirrel AI |
| Validation harness | `check_bool`, `check_skip`, `check_or_skip`, `run_experiment()`, `print_banner()` |
| Dishonest scaffolding | 0 (all experiments use honest skip or real validation) |
| Tower Atomic | **FULLY UTILIZED** — 41/41 gates (24 core + 17 full utilization) |
| Nest Atomic | **VALIDATED** — 8/8 gates (nestgate storage, model cache) |
| Node Atomic | **VALIDATED** — 5/5 gates (toadstool compute, dual-protocol) |
| NUCLEUS | **VALIDATED** — 58/58 base gates (Tower + Nest + Node) |
| Graph Overlays | **VALIDATED** — 14/14 gates (tier-independent primals via deploy graphs) |
| Squirrel Discovery | **VALIDATED** — 5/5 gates (cross-primal env_sockets, capability.discover) |
| Graph Execution | **LIVE** — 6/6 gates (3/5 coordination patterns live) |
| Provenance Readiness | **STRUCTURAL** — 4/4 gates (launch profiles + deploy graph) |
| Total Gates | **87/87** |
| Squirrel AI | Composition validated (Tower + Squirrel + Anthropic Claude) |
| petalTongue | v1.6.6 integrated, visualization.render.dashboard + grammar |

## What Changed (v0.6.0 -> v0.7.0)

### Graph-Driven Overlay Composition
1. **Tier-independent primals** — Squirrel, petalTongue, biomeOS compose at any atomic tier via deploy graphs
2. **`spawn` field on GraphNode** — distinguishes primal nodes from validation nodes
3. **5 new overlay deploy graphs** — tower_ai, tower_ai_viz, nest_viz, node_ai, full_overlay
4. **`merge_graphs()`** — merge base + overlay deploy graphs at runtime
5. **exp069** — end-to-end overlay validation (25/25 checks)

### Squirrel Cross-Primal Discovery
6. **9 env_sockets** — Squirrel discovers NestGate, ToadStool, Songbird, BearDog via explicit env vars
7. **full_overlay.toml** — Tower + Nest + Node + Squirrel (all capability domains)
8. **exp070** — cross-primal discovery validation
9. **4 new integration tests** — squirrel_discovers_sibling_primals, tool_list, context_create, ai_query

### Graph Execution Patterns (3/5 Live)
10. **exp010 Sequential** — live Tower composition with ordering verification
11. **exp011 Parallel** — live 4-primal burst (beardog+songbird+nestgate+toadstool)
12. **exp012 ConditionalDag** — live toadstool/CPU fallback branching
13. exp013/014 — awaiting provenance trio (sweetGrass, rhizoCrypt, loamSpine)

### Provenance Readiness
14. **Launch profiles** — sweetGrass, loamSpine, rhizoCrypt socket wiring
15. **provenance_overlay.toml** — Tower + RootPulse deploy graph
16. **Handoffs delivered** — provenance trio team + all teams

## What Changed (v0.5.0 -> v0.6.0)

### NUCLEUS Composition (v0.6.0)
1. **Nest Atomic** — nestgate storage primal integrated
2. **Node Atomic** — toadstool compute primal integrated
3. **NUCLEUS Composition** — all 3 atomic layers compose together (58/58 gates)
4. **3 new experiments** — exp066, exp067, exp068
5. **12 new integration tests** — 8 Nest + 4 Node

## Co-Evolution Strategy

| Phase | Focus | Partners | Gate Target | Status |
|---|---|---|---|---|
| Tower Stability | All 24 Tower gates | beardog, songbird, biomeOS | Gates 1–6 (24/24) | **DONE** |
| Tower + Squirrel | AI composition | + squirrel | AI gates | **DONE** |
| Tower Full Utilization | Subsystems + viz | + petalTongue | Gates 7–11 (41/41) | **DONE** |
| Nest Atomic | Storage gates | + nestgate | Gates 12–13 (8/8) | **DONE** |
| Node Atomic | Compute gates | + toadstool | Gates 14–15 (5/5) | **DONE** |
| NUCLEUS Composition | All layers compose | Tower + Nest + Node | Gate 16 (4/4) | **DONE** |
| Graph Overlays | Tier-independent primals | + squirrel, petalTongue | Gates 17–20 (14/14) | **DONE** |
| Squirrel Discovery | Cross-primal wiring | + all primals | Gate 21 (5/5) | **DONE** |
| Graph Execution | 3/5 patterns live | Tower + Nest + Node | Gate 22 (6/6) | **DONE** |
| Provenance Readiness | Structural prep | sweetGrass/loamSpine/rhizoCrypt | Gate 23 (4/4) | **DONE** |
| Provenance Live | Trio integration | + provenance trio | All gates | **NEXT** (awaiting provenance-trio-types) |

See `specs/TOWER_STABILITY.md` for the full 87-gate acceptance criteria.

## What Remains

- **Provenance trio binaries** — blocked on `provenance-trio-types` shared crate
- Pipeline + Continuous graph execution (exp013/014) — awaiting sweetGrass/rhizoCrypt
- Emergent system experiments (Track 3) — awaiting provenance trio
- Protocol escalation (JSON-RPC -> tarpc sidecar)
- biomeOS self-composition (biomeOS composes its own graphs at runtime)

---

**License**: AGPL-3.0-or-later
