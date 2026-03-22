# primalSpring — Coordination and Composition Spring

**Domain**: Primal coordination, atomic composition, graph execution, emergent systems, bonding  
**Version**: 0.7.0 (Phase 10 — GRAPH EXECUTION + PROVENANCE READINESS 87/87, 253+ tests, 49 experiments)  
**License**: AGPL-3.0-or-later  
**Last Updated**: March 22, 2026

---

## What Is primalSpring?

primalSpring is the spring whose domain IS coordination. Where other springs validate
domain science (hotSpring validates physics, wetSpring validates biology), primalSpring
validates the ecosystem itself — the coordination, composition, and emergent behavior
that biomeOS and the Neural API produce when primals work together.

Its "papers" are the atomics. Its "experiments" are composition patterns. Its validation
target is biomeOS itself.

---

## Why It Exists

Existing NUCLEUS testing is fragmented:

- biomeOS has graph parsing tests but no end-to-end multi-primal composition tests
- Individual springs have `validate_nucleus_*` binaries, each testing from its own narrow domain
- Bonding model has 5 test graphs but zero automated validation
- Plasmodium has specs and unit tests but no multi-gate integration
- RootPulse is validated via ludoSpring exp052 (37 checks) but not systematically

primalSpring fills the gap.

---

## Current State

| Metric | Value |
|--------|-------|
| Tests | **253+** (unit + integration + doc-tests, 42 ignored live) |
| Experiments | 49 (8 tracks) |
| Proptest fuzz tests | 15 |
| Clippy (pedantic + nursery) | **0 warnings** |
| `cargo fmt` | **clean** |
| `#![forbid(unsafe_code)]` | workspace-level |
| C dependencies | 0 (`deny.toml` enforced) |
| Files over 1000 LOC | 0 |
| Deploy graphs | 18 TOMLs, all `by_capability`, topologically validated |
| RPC endpoints | 17 methods |
| Discovery | **Capability-first**: `discover_by_capability()` + 5-tier + Neural API |
| Server mode | JSON-RPC 2.0 over Unix socket |
| MCP tools | 8 typed tools via `mcp.tools.list` |
| Niche self-knowledge | `niche.rs` — 37 capabilities, semantic mappings, cost estimates |
| Meta-validator | `validate_all` binary — runs all 49 experiments |
| Tower Atomic | **STABLE** — 41/41 gates passing (core + full utilization) |
| Nest Atomic | **VALIDATED** — nestgate storage (8/8 gates) |
| Node Atomic | **VALIDATED** — toadstool compute (5/5 gates) |
| NUCLEUS | **VALIDATED** — Tower + Nest + Node (58/58 base gates) |
| Graph Overlays | **VALIDATED** — tier-independent primals via deploy graphs (14/14) |
| Squirrel Discovery | **VALIDATED** — cross-primal env_sockets wiring (5/5) |
| Graph Execution | **LIVE** — 3/5 coordination patterns validated live (6/6) |
| Provenance Readiness | **STRUCTURAL** — launch profiles + deploy graph ready (4/4) |
| Total Gates | **87/87** |

### Key Capabilities

- **Capability-first discovery**: `discover_by_capability()` resolves providers by what
  they offer, not who they are. All RPC handlers default to capability-based validation.
- **Graphs as source of truth**: `topological_waves()` computes startup ordering via
  Kahn's algorithm. `graph_required_capabilities()` extracts capability rosters from
  graph nodes. All 18 graphs have `by_capability` on every node (enforced by test).
- **Real IPC**: `probe_primal()`, `validate_composition_by_capability()`, `health_check()`
  connect to live providers via Unix socket JSON-RPC 2.0
- **IPC resilience stack**: `IpcError` (8 typed variants + `IpcErrorPhase`), `CircuitBreaker`,
  `RetryPolicy`, `resilient_call()`, `DispatchOutcome<T>`
- **4-format capability parsing**: Handles Format A (string array), B (object array),
  C (method_info nested), D (semantic_mappings double-nested)
- **MCP tool definitions**: 8 typed tools with JSON Schema for Squirrel AI discovery +
  `discover_remote_tools()` for cross-spring tool enumeration
- **5-tier discovery**: env → XDG → temp → manifest → socket-registry (+ Neural API)
- **Capability-based health probing**: `check_capability_health()` discovers providers
  at runtime and records health, latency, and capabilities checks
- **Graceful degradation**: `check_skip()` and `check_or_skip()` for honest reporting
  when providers are not running
- **Server mode**: `primalspring_primal server` exposes 17 methods including `graph.waves`,
  `graph.capabilities`, `coordination.probe_capability`
- **Deploy graph validation**: `deploy/` module parses, structurally validates, topologically
  sorts, and live-probes biomeOS TOML graphs

---

## Track Structure (8 Tracks, 49 Experiments)

### Track 1: Atomic Composition (exp001–006)

Deploy each atomic layer, validate every primal starts, discovers peers,
and responds to capability calls.

| Exp | What | Primals | Status |
|-----|------|---------|--------|
| 001 | Tower Atomic bootstrap | security + discovery | **Capability-based** |
| 002 | Node Atomic compute | security + discovery + compute | **Capability-based** |
| 003 | Nest Atomic storage | security + discovery + storage | **Capability-based** |
| 004 | Full NUCLEUS | All 8 capability domains | **Capability-based** |
| 005 | Atomic subtraction | Graceful degradation | Discovery wired |
| 006 | Startup ordering | Topological waves from graphs | **Graph-driven** |

### Track 2: Graph Execution (exp010–015)

Validate all 5 coordination patterns with real primals.

| Exp | Pattern | Graph |
|-----|---------|-------|
| 010 | Sequential | tower_atomic_bootstrap.toml | **Live validated** |
| 011 | Parallel | parallel_capability_burst.toml | **Live validated** |
| 012 | ConditionalDag | conditional_fallback.toml | **Live validated** |
| 013 | Pipeline | streaming_pipeline.toml | Awaiting sweetGrass |
| 014 | Continuous | continuous_tick.toml | Awaiting provenance trio |
| 015 | PathwayLearner | Metrics + optimization | Discovery wired |

### Track 3: Emergent Systems (exp020–025)

Validate Layer 3 systems that emerge from graph execution.

| Exp | System | Validates |
|-----|--------|-----------|
| 020 | RootPulse commit | 6-phase provenance trio |
| 021 | RootPulse branch + merge | Branch, merge, seal |
| 022 | RootPulse diff + federate | Merkle comparison |
| 023 | RPGPT session | 60 Hz tick + provenance |
| 024 | Cross-spring ecology | airSpring + wetSpring pipeline |
| 025 | coralForge pipeline | Neural object (structure prediction) |

### Track 4: Bonding and Plasmodium (exp030–034)

Multi-gate coordination.

| Exp | What | Validates |
|-----|------|-----------|
| 030 | Covalent bond | Shared family seed discovery |
| 031 | Ionic bond | Cross-family capability sharing |
| 032 | Plasmodium formation | query_collective() |
| 033 | Gate failure | Graceful degradation |
| 034 | Capability aggregation | Best-gate routing |

### Track 5: coralForge Redefinition

coralForge is no longer a module inside neuralSpring. It is an emergent
neural object — a Pipeline graph composed via biomeOS over neuralSpring +
wetSpring + hotSpring + toadStool + NestGate. The math stays in neuralSpring.
The composition becomes `coralforge_pipeline.toml`. primalSpring exp025
validates the pipeline end-to-end.

### Track 6: Cross-Spring Coordination (exp040–044)

| Exp | What | Springs |
|-----|------|---------|
| 040 | Cross-spring data flow | Capability-routed ecology pipeline (petalTongue, Squirrel) |
| 041 | Provenance trio for science | Any spring -> provenance trio |
| 042 | fieldMouse ingestion | fieldMouse frames -> NestGate -> sweetGrass |
| 043 | petalTongue visualization | biomeOS SSE -> petalTongue |
| 044 | Squirrel AI coordination | Multi-MCP via Squirrel |

### Track 7: Showcase-Mined Patterns (exp050–059)

Early coordination patterns extracted from phase1/ and phase2/ primal showcases.

| Exp | What | Source |
|-----|------|--------|
| 050 | Compute triangle | coralReef -> toadStool -> barraCuda pipeline |
| 051 | Socket discovery sweep | Capability-based enumeration |
| 052 | Protocol escalation | HTTP -> JSON-RPC -> tarpc negotiation |
| 053 | Multi-primal lifecycle | 6-primal research paper lifecycle |
| 054 | Bearer token auth | BearDog authenticate -> validate -> compute |
| 055 | Wait-for-health | Repeated health probes with timeout and ordering |
| 056 | Cross-tower federation | BYOB manifest, cross-tower discovery |
| 057 | Supply chain provenance | 7-stage DAG with per-agent signing |
| 058 | Semantic attribution | Module/feature/function tracking + fair credit |
| 059 | Weak force isolation | Zero leakage with unknown primals |

### Track 8: Live Composition (exp060–070)

Live multi-primal composition with real primals from `plasmidBin`.

| Exp | What | Primals | Status |
|-----|------|---------|--------|
| 060 | biomeOS Tower deploy | beardog + songbird via neural-api-server bootstrap graph | **Live validated** |
| 061 | Squirrel AI composition | Tower + Squirrel + Anthropic Claude `ai.query` | **Live validated** |
| 062 | Subsystem sweep | songbird JSON-RPC subsystems (11/12 UP) | **Live validated** |
| 063 | Pixel rendezvous | BirdSong beacon encrypt/decrypt round-trip | **Live validated** |
| 064 | Internet reach | STUN, Onion, Tor paths | **Live validated** |
| 065 | petalTongue dashboard | Dashboard + Grammar of Graphics rendering | **Live validated** |
| 066 | Nest Atomic | nestgate storage: store/retrieve/list/model cache (13/13) | **Live validated** |
| 067 | Node Atomic | toadstool compute: health, capabilities, version (13/13) | **Live validated** |
| 068 | Full NUCLEUS | Tower + Nest + Node composing together (16/16) | **Live validated** |
| 069 | Graph overlay composition | Overlay structural + merge + live (25/25) | **Live validated** |
| 070 | Squirrel cross-primal discovery | Cross-primal env_sockets + capability.discover | **Live validated** |

---

## Capability Domain

```
coordination.validate_composition              — Validate composition (capability-based by default)
coordination.validate_composition_by_capability — Explicitly capability-based validation
coordination.discovery_sweep                    — Discover capabilities in a composition
coordination.probe_primal                       — Probe a single primal by name
coordination.probe_capability                   — Probe a single capability provider
coordination.deploy_atomic                      — Deploy an atomic via graph
coordination.bonding_test                       — Test bonding readiness
coordination.neural_api_status                  — Neural API health
composition.tower_health                        — Tower composition health (capability-based)
composition.node_health                         — Node composition health
composition.nest_health                         — Nest composition health
composition.nucleus_health                      — Full NUCLEUS health
health.check / health.liveness                  — Self health + liveness probe
health.readiness                                — Readiness (capabilities + Neural API)
identity.get                                    — sourDough identity compliance
capabilities.list                               — Niche capabilities + mappings + costs
graph.list                                      — Validate all deploy graphs
graph.validate                                  — Validate a specific graph
graph.waves                                     — Topological startup wave ordering
graph.capabilities                              — Required capabilities from graph
nucleus.start / nucleus.stop                    — Lifecycle management
lifecycle.status                                — Primal status report
mcp.tools.list                                  — MCP tool definitions for Squirrel AI
ai.query                                        — Route AI inference queries (via Squirrel)
ai.health                                       — AI provider health check (via Squirrel)
composition.tower_squirrel_health               — Tower + Squirrel composition health
```

---

## Active Handoffs

| Version | File | Date | Scope |
|---------|------|------|-------|
| v0.7.0 | `PRIMALSPRING_V070_GRAPH_OVERLAY_HANDOFF_MAR22_2026.md` | Mar 22 | Graph overlays, Squirrel discovery, graph execution, integration guide for all teams |
| v0.7.0 | `PROVENANCE_TRIO_HANDOFF_MAR22_2026.md` | Mar 22 | sweetGrass/loamSpine/rhizoCrypt: blocker, required methods, integration path |
| v0.6.0 | `PRIMALSPRING_V060_NUCLEUS_COMPOSITION_HANDOFF_MAR22_2026.md` | Mar 22 | NUCLEUS validated: Nest + Node atomic layers, 58/58 gates, toadstool/barracuda evolution |
| v0.5.0 | `TOWER_FULL_UTILIZATION_VALIDATED_MAR21_2026.md` | Mar 21 | Tower 41/41, subsystems, petalTongue, parallel tests |
| v0.4.0 | `TOWER_STABILITY_SPRINT_MAR21_2026.md` | Mar 21 | Tower 24/24 STABLE, 7 new integration tests, cross-primal capability alignment |
| v0.4.0 | `TOWER_SQUIRREL_COMPOSITION_MAR21_2026.md` | Mar 21 | Tower + Squirrel AI composition, exp060/061, abstract socket integration |
| v0.3.7 | `DEPRECATION_CLEANUP_MAR20_2026.md` | Mar 20 | Cross-repo dead code removal: 6,478 lines across beardog, songbird, biomeOS |
| v0.3.6 | `TOWER_COEVOLUTION_GUIDE.md` | Mar 18 | Tower co-evolution contract: sprint loop, per-team quick wins, timeline |
| v0.3.5 | Capability audits (6 files) | Mar 18 | BearDog, Songbird, NestGate, ToadStool, Squirrel, biomeOS |

## Archived Handoffs

| Version | File | Date | Scope |
|---------|------|------|-------|
| v0.4.0 | `archive/PRIMALSPRING_V040_TOADSTOOL_BARRACUDA_EVOLUTION_HANDOFF_MAR21_2026.md` | Mar 21 | Superseded by v0.6.0 NUCLEUS handoff |
| v0.3.0 | `archive/PRIMALSPRING_V030_COORDINATION_ABSORPTION_HANDOFF_MAR18_2026.md` | Mar 18 | biomeOS coordination absorption |
| v0.3.0 | `archive/PRIMALSPRING_V030_CAPABILITY_FIRST_EVOLUTION_HANDOFF_MAR18_2026.md` | Mar 18 | Capability-first architecture |
| v0.3.0 | `archive/PRIMALSPRING_V030_TOADSTOOL_BARRACUDA_CAPABILITY_HANDOFF_MAR18_2026.md` | Mar 18 | toadStool/barraCuda: capability-based discovery |
| v0.3.0 | `archive/PRIMALSPRING_V030_EVOLUTION_HANDOFF_MAR18_2026.md` | Mar 18 | Pre-capability evolution |
| v0.3.0 | `archive/PRIMALSPRING_V030_TOADSTOOL_BARRACUDA_EVOLUTION_HANDOFF_MAR18_2026.md` | Mar 18 | Pre-capability barraCuda handoff |
| v0.2.0 | `archive/PRIMALSPRING_V020_*` (3 files) | Mar 18 | Ecosystem absorption, compute triangle, barraCuda evolution |
| v0.1.x | `archive/PRIMALSPRING_V01*` (4 files) | Mar 17–18 | Initial audit, Neural API, deep debt, toadStool coordination |

## Convention

**Naming**: `PRIMALSPRING_V{VERSION}_{TOPIC}_HANDOFF_{DATE}.md`

**Flow**: primalSpring → biomeOS (composition), primalSpring → toadStool/barraCuda (coordination intelligence). No reverse dependencies.

---

## Cross-Spring Context

primalSpring is unique: cross-spring coordination is its core mission.
Every experiment involves multiple primals or springs.

| Spring | What primalSpring Learns |
|--------|-------------------------|
| hotSpring | Precision validation patterns (170 tolerances, structured provenance) |
| wetSpring | Deep IPC integration (354 bins, 214 tolerances, MCP tools) |
| airSpring | NUCLEUS niche deployment (41 caps, deny.toml, MCP) |
| groundSpring | Typed errors, ValidationSink, 13-tier tolerance provenance |
| neuralSpring | Capability registry TOML, primal_names::display, MCP tools |
| ludoSpring | ValidationResult::with_provenance(), structured provenance |
| healthSpring | Proptest IPC fuzz (18 tests), circuit breaker patterns |

---

## Key Differences from Other Springs

| Property | Other Springs | primalSpring |
|----------|--------------|-------------|
| Domain | Science | Coordination itself |
| "Papers" | Published papers | Atomics, emergent systems |
| Validation target | barraCuda math | biomeOS orchestration |
| biomeOS role | Deploys the spring | IS the subject under test |
| Cross-spring | Optional | Core mission |
| barraCuda consumption | Domain-specific | None (IPC only) |

---

**License**: AGPL-3.0-or-later
