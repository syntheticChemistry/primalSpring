# primalSpring — Cross-Spring Evolution

**Date**: March 22, 2026
**Status**: Phase 10 — GRAPH EXECUTION + PROVENANCE READINESS (87/87 gates), 253+ tests, 49 experiments

---

## Overview

primalSpring is unique: cross-spring coordination is its core mission, not
an optional track. Every experiment involves multiple primals or springs.

## Cross-Spring Touchpoints

| Track | Springs/Primals Involved | Pattern |
|-------|--------------------------|---------|
| 1 (Atomic) | BearDog, Songbird, ToadStool, NestGate, Squirrel | Deploy + health check |
| 2 (Graph) | biomeOS, all primals | Graph execution |
| 3 (Emergent) | rhizoCrypt, LoamSpine, sweetGrass, ludoSpring, neuralSpring, wetSpring | Layer 3 systems |
| 4 (Bonding) | Songbird (mesh), BearDog (trust), all NUCLEUS | Multi-gate |
| 5 (coralForge) | neuralSpring, wetSpring, hotSpring, ToadStool, NestGate | Pipeline graph |
| 6 (Cross-Spring) | airSpring, wetSpring, neuralSpring, petalTongue, Squirrel | Data flow |
| 7 (Showcase) | coralReef, toadStool, barraCuda, BearDog, NestGate, sweetGrass, rhizoCrypt | Mined patterns |

## What primalSpring Learns from Each Spring

| Spring | Lesson | Absorbed in |
|--------|--------|-------------|
| hotSpring | Precision validation (170 tolerances, provenance) | v0.2.0 (pattern), v0.3.0 (provenance struct) |
| wetSpring | Deep IPC integration (354 bins, 214 tolerances, MCP tools) | v0.2.0 (resilience), v0.3.0 (MCP tools) |
| airSpring | NUCLEUS niche deployment (41 caps, deny.toml, MCP) | v0.2.0 (niche), v0.3.0 (deny.toml, MCP) |
| groundSpring | Typed errors, ValidationSink, 13-tier tol | v0.2.0 (ValidationSink, OrExit) |
| neuralSpring | Capability registry TOML, primal_names::display | v0.3.0 (capability_registry.toml) |
| ludoSpring | ValidationResult::with_provenance(), #[expect(reason)] | v0.3.0 (structured provenance) |
| healthSpring | Proptest IPC fuzz (18 tests), provenance completeness | v0.3.0 (proptest expansion) |

## What primalSpring Contributes Back

| Pattern | Absorbed By |
|---------|-------------|
| ValidationSink trait | groundSpring V116, rhizoCrypt v0.13 |
| check_skip / check_or_skip | Referenced by all spring experiment patterns |
| 4-format capability parsing | Converged implementation used as reference |
| Deploy graph structural + topological validation | 18 TOML graphs, all `by_capability`, topological waves |
| Graph-driven overlay composition | Tier-independent primals via deploy graph overlays |
| Graph merge/compose | Base + overlay graph merging for runtime composition |
| Squirrel cross-primal env_sockets wiring | Explicit `{CAPABILITY}_PROVIDER_SOCKET` env vars for fast discovery |
| full_overlay.toml | Tower + Nest + Node + Squirrel full-stack deploy graph |
| provenance_overlay.toml | Tower + RootPulse provenance trio deploy graph |
| Live graph execution patterns | Sequential, Parallel, ConditionalDag validated live |
| Provenance trio launch profiles | sweetGrass, loamSpine, rhizoCrypt socket wiring |
| Coordination experiment framework | 49 experiments across 8 tracks |
| MCP coordination tools | Available for Squirrel AI routing |
| Tower + Squirrel AI composition | Live demo: Tower + Squirrel + Anthropic Claude (exp061) |
| `passthrough_env` pattern | Secure env forwarding for API keys, GPU config vars |
| `PrimalProcess::from_parts()` | Custom spawn for primals with non-standard transports |
| Abstract socket integration | Squirrel Universal Transport on Linux abstract namespace |

## Ecosystem State (March 22, 2026)

| Spring | Version | Tests | Key Absorption |
|--------|---------|-------|----------------|
| hotSpring | v0.6.32 | 848 | Sovereign GPU, CoralCompiler IPC |
| groundSpring | V116 | 960+ | Typed errors, OnceLock GPU cache |
| neuralSpring | V118 | 1,304 | Capability registry, display names |
| wetSpring | V128 | 1,443+ | MCP tools, cast module, FMA |
| airSpring | v0.10.0 | 1,207+ | MCP tools, deny.toml, provenance registry |
| healthSpring | V37 | 706 | 18 proptest IPC fuzz, MCP tools |
| ludoSpring | V14 | 187 | with_provenance(), 93.2% coverage |
| primalSpring | v0.7.0 | 253+ | GRAPH EXECUTION + PROVENANCE READINESS 87/87, 3/5 patterns live, provenance trio structural, 49 experiments, 37 capabilities |

## Evolution Path

```
Phase 0 (done): Scaffolding (March 2, 2026)
  → 38 experiments scaffolded, workspace compiles

Phase 0→1 (done): Real Discovery (March 17, 2026)
  → IPC module evolved: discover + protocol + client
  → All experiments use real discover_primal() + honest check_skip

Phase 1 (done): Neural API + Deep Debt (March 17, 2026)
  → neural-api-client-sync, server mode, probe_primal(), 69 unit tests

Phase 2 (done): Ecosystem Absorption (March 18, 2026)
  → IPC resilience stack from 7 sibling springs
  → IpcError, CircuitBreaker, RetryPolicy, DispatchOutcome
  → 4-format capability parsing, health probes
  → safe_cast, OrExit, ValidationSink, proptest
  → 157 tests, zero warnings, v0.2.0

Phase 2→3 (done): Deep Debt + Cross-Ecosystem (March 18, 2026)
  → MCP tool definitions (8 tools with JSON Schema)
  → 5-tier discovery (manifest + socket-registry fallbacks)
  → Structured Provenance on ValidationResult
  → Capability registry TOML (sync-tested)
  → deny.toml (14-crate ecoBin ban)

Phase 3 (done): Capability-First Architecture (March 18, 2026)
  → All RPC handlers default to capability-based validation
  → discover_by_capability() replaces discover_primal() in core experiments
  → topological_waves() — Kahn's algorithm startup ordering
  → graph_required_capabilities() — graphs as source of truth
  → by_capability on all 11 deploy graph nodes (enforced by test)
  → New RPC: graph.waves, graph.capabilities, coordination.probe_capability
  → check_capability_health() for capability-based health probing
  → IpcErrorPhase + PhasedIpcError for phase-aware errors
  → discover_remote_tools() for cross-spring MCP tool discovery
  → 248 tests (233 unit + 13 integration + 2 doc-tests), 3 ignored (live atomic)

Phase 4 (done): Tower Stability Sprint (March 21, 2026)
  → 24/24 Tower Atomic gates STABLE with plasmidBin binaries
  → 11 live integration tests (beardog, songbird, Neural API)
  → exp060: biomeOS-orchestrated Tower deployment via bootstrap graph

Phase 4.5 (done): Squirrel AI Composition (March 21, 2026)
  → Tower + Squirrel (3-primal) composition validated
  → exp061: live ai.query via Anthropic Claude through Neural API
  → 2 Squirrel integration tests (ai_query, composition_health)
  → passthrough_env for API key forwarding, abstract socket support
  → 264 tests (239 unit + 23 integration + 2 doc-tests), 15 ignored (live)

Phase 5 (done): Tower Full Utilization (March 21, 2026)
  → 41/41 Tower gates (24 core + 17 full utilization)
  → songbird subsystems, Pixel rendezvous, internet reach, petalTongue viz
  → exp062-065, 270 tests, 44 experiments

Phase 6 (done): NUCLEUS Composition (March 22, 2026)
  → Nest Atomic: nestgate storage (8/8 gates, ZFS fallback, store/retrieve)
  → Node Atomic: toadstool compute (5/5 gates, dual-protocol, 4 workloads)
  → NUCLEUS: Tower + Nest + Node compose together (58/58 total)
  → exp066-068, 282 tests, 47 experiments, 31 integration tests in parallel
  → Harness: subcommand, jsonrpc_socket_suffix, env_sockets, remap()

Phase 7 (done): Graph-Driven Overlay Composition (March 22, 2026)
  → compute_spawn_order trusts graph (not enum filter)
  → spawn=true/false on GraphNode, graph_spawnable_primals(), graph_capability_map()
  → RunningAtomic overlay_capabilities, capability_to_primal overlay fallback
  → 4 overlay graphs: tower_ai, tower_ai_viz, nest_viz, node_ai
  → merge_graphs() for base + overlay composition
  → 11 new integration tests (4 structural + 7 live overlay)
  → exp069: 25/25 overlay checks pass
  → 87/87 total gates, 253+ tests, 49 experiments

Phase 8 (done): Squirrel Cross-Primal Discovery (March 22, 2026)
  → Squirrel env_sockets wiring: 9 capability provider socket mappings
  → full_overlay.toml: Tower + Nest + Node + Squirrel (all capability domains)
  → capability.discover, tool.list, context.create, ai.query via composition
  → exp070: structural + live cross-primal discovery validation
  → 4 new integration tests (squirrel_discovers_sibling_primals, tool_list, context, ai_query)
  → 77/77 total gates, 253+ tests, 49 experiments

Phase 9 (done): Graph Execution Patterns (March 22, 2026)
  → exp010 sequential: live Tower composition with ordering verification
  → exp011 parallel: live 4-primal burst (beardog+songbird+nestgate+toadstool)
  → exp012 conditional DAG: live toadstool/CPU fallback branching
  → exp013 pipeline: awaiting sweetGrass binary
  → exp014 continuous tick: awaiting provenance trio
  → 3/5 coordination patterns validated live

Phase 10 (done): Provenance Readiness (March 22, 2026)
  → Launch profiles for sweetGrass, loamSpine, rhizoCrypt
  → provenance_overlay.toml: Tower + RootPulse deploy graph
  → Handoff to provenance trio teams (PROVENANCE_TRIO_HANDOFF)
  → Handoff to all teams (V070_GRAPH_OVERLAY_HANDOFF)
  → Blocker: provenance-trio-types shared crate missing from disk
  → 87/87 total gates, 253+ tests, 49 experiments

Phase 11: Provenance Trio Live Integration
  → Awaiting provenance-trio-types resolution + binaries in plasmidBin
Phase 12: Emergent systems (Track 3)
  → RootPulse, coralForge pipeline validated
Phase 13: Bonding (Track 4)
  → Multi-gate coordination
Phase 14: Cross-spring (Track 6)
  → Full ecosystem integration
Phase 15: Showcase patterns (Track 7)
  → phase1/phase2 mined coordination patterns validated
Phase 16: biomeOS Self-Composition
  → biomeOS composes its own graphs at runtime
```
