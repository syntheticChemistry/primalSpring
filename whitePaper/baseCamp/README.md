# primalSpring baseCamp — Coordination and Composition Validation

**Date**: March 22, 2026
**Status**: Phase 6 — NUCLEUS COMPOSITION VALIDATED (58/58 gates), Tower + Nest + Node, 47 experiments, 282 tests

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
| 2 | Graph Execution | exp010–015 | Do all 5 coordination patterns work? |
| 3 | Emergent Systems | exp020–025 | Do Layer 3 systems emerge correctly? |
| 4 | Bonding & Plasmodium | exp030–034 | Does multi-gate coordination work? |
| 5 | coralForge | (exp025) | Does the neural object pipeline work? |
| 6 | Cross-Spring | exp040–044 | Do cross-spring data flows work? |
| 7 | Showcase-Mined | exp050–059 | Do mined phase1/phase2 coordination patterns hold? |
| 8 | Live Composition | exp060–068 | Tower + Squirrel AI + subsystems + Nest + Node + NUCLEUS |

## Current State (v0.6.0)

| Metric | Value |
|--------|-------|
| Experiments | 47 (8 tracks) |
| Unit tests | 239 |
| Integration tests | 31 (10 JSON-RPC + 19 live Tower + 8 Nest + 4 Node) |
| Doc-tests | 2 |
| Total tests | **282** (251 auto + 31 ignored live) |
| Proptest fuzz tests | 15 (IPC protocol, extract, capability parsing) |
| clippy (pedantic+nursery) | 0 warnings |
| cargo doc | 0 warnings |
| `#[allow()]` in production | 0 |
| unsafe_code | Workspace-level `forbid` |
| C dependencies | 0 (pure Rust, ecoBin compliant, `deny.toml` enforced) |
| Deploy graphs | 12 TOMLs (incl. `tower_full_capability.toml`), all nodes `by_capability` |
| Discovery | Capability-first: 5-tier + Neural API + `discover_by_capability()` |
| RPC endpoints | 17 methods (including `graph.waves`, `graph.capabilities`) |
| Niche self-knowledge | `niche.rs` — 37 capabilities, semantic mappings, cost estimates |
| MCP tools | 8 typed tools via `mcp.tools.list` for Squirrel AI |
| Validation harness | `check_bool`, `check_skip`, `check_or_skip`, `run_experiment()`, `print_banner()` |
| Dishonest scaffolding | 0 (all experiments use honest skip or real validation) |
| Tower Atomic | **FULLY UTILIZED** — 41/41 gates (24 core + 17 full utilization) |
| Nest Atomic | **VALIDATED** — 8/8 gates (nestgate storage, model cache) |
| Node Atomic | **VALIDATED** — 5/5 gates (toadstool compute, dual-protocol) |
| NUCLEUS | **VALIDATED** — 58/58 total gates (Tower + Nest + Node) |
| Squirrel AI | Composition validated (Tower + Squirrel + Anthropic Claude) |
| petalTongue | v1.6.6 integrated, visualization.render.dashboard + grammar |

## What Changed (v0.5.0 -> v0.6.0)

### NUCLEUS Composition (v0.6.0)
1. **Nest Atomic** — nestgate storage primal integrated: socket-only mode (no ZFS required), storage.store/retrieve round-trip, model.register/locate, discover_capabilities
2. **Node Atomic** — toadstool compute primal integrated: dual-protocol socket (tarpc + JSON-RPC), toadstool.health, toadstool.query_capabilities (4 workload types, 24 CPU cores)
3. **NUCLEUS Composition** — all 3 atomic layers (Tower + Nest + Node) compose and validate together. 58/58 total gates passing
4. **3 new experiments** — exp066 (Nest 13/13), exp067 (Node 13/13), exp068 (NUCLEUS 16/16)
5. **12 new integration tests** — 8 Nest + 4 Node, all passing in parallel with Tower tests
6. **Harness enhancements** — `subcommand` override, `jsonrpc_socket_suffix` for dual-protocol primals, `SocketNucleation::remap()`, health liveness fallback chain
7. **282 tests** — up from 270 (+12); 31 integration tests, 31 ignored (live)

## What Changed (v0.4.0 -> v0.5.0)

### Tower Full Utilization (v0.5.0)
1. **Gates 7-11 added** — songbird subsystem health, beacon round-trip, Pixel rendezvous, internet reach, petalTongue visualization (24->41 total gates)
2. **4 new experiments** — exp062 (subsystem sweep), exp063 (Pixel rendezvous), exp064 (internet reach), exp065 (petalTongue dashboard)
3. **6 new songbird subsystem integration tests** — discovery, STUN, BirdSong, onion, Tor, federation
4. **petalTongue v1.6.6** — built, deployed to plasmidBin, launch profile + capability registry wired
5. **`tower_full_capability.toml`** — complete Tower deploy graph with all subsystem nodes
6. **12 new capabilities** — songbird subsystems + petalTongue visualization (25->37 total)
7. **270 tests** — up from 264 (+6); 29 integration tests, 21 ignored (live)

## What Changed (v0.3.0 -> v0.4.0)

### Tower Stability Sprint (v0.4.0)
1. **All 24 Tower Atomic gates pass** — cross-primal changes to beardog, songbird, biomeOS
2. **Squirrel AI Composition** — Tower + Squirrel with live Anthropic Claude inference
3. **exp060 + exp061** — biomeOS Tower deploy and Squirrel AI composition experiments
4. **264 tests** — up from 251 (+13); 23 integration tests, 15 ignored (live)

See CHANGELOG.md for earlier evolution history (v0.1.0 -> v0.3.0).

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
| Tower Full Utilization | Subsystems + viz | + petalTongue | Gates 7–11 (41/41) | **DONE** |
| Nest Atomic | Storage gates | + nestgate | Gates 12–13 (8/8) | **DONE** |
| Node Atomic | Compute gates | + toadstool | Gates 14–15 (5/5) | **DONE** |
| NUCLEUS Composition | All layers compose | Tower + Nest + Node | Gate 16 (4/4) | **DONE** |
| Full NUCLEUS | + provenance trio | + squirrel + provenance | All gates | **NEXT** |

See `specs/TOWER_STABILITY.md` for the full 58-gate acceptance criteria.

## What Remains (Phase 7+)

- Full NUCLEUS with Squirrel AI + provenance trio (sourDough, sweetGrass, fieldMouse)
- Beacon coordination validation (generate -> encrypt -> exchange -> decrypt chain)
- Protocol escalation (JSON-RPC -> tarpc sidecar from biomeOS v2.50)
- `cargo llvm-cov` CI integration with 90% floor
- Neural API auto-registration for Squirrel `ai` capabilities
- Graph execution experiments (Track 2) live with real primals
- Emergent system experiments (Track 3) live validation

---

**License**: AGPL-3.0-or-later
