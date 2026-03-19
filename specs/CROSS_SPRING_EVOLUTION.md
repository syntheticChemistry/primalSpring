# primalSpring — Cross-Spring Evolution

**Date**: March 18, 2026
**Status**: Phase 3 — Live atomic harness, capability-first architecture, 248 tests

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
| Deploy graph structural + topological validation | 11 TOML graphs, all `by_capability`, topological waves |
| Coordination experiment framework | 38 experiments across 7 tracks |
| MCP coordination tools | Available for Squirrel AI routing |

## Ecosystem State (March 18, 2026)

| Spring | Version | Tests | Key Absorption |
|--------|---------|-------|----------------|
| hotSpring | v0.6.32 | 848 | Sovereign GPU, CoralCompiler IPC |
| groundSpring | V116 | 960+ | Typed errors, OnceLock GPU cache |
| neuralSpring | V118 | 1,304 | Capability registry, display names |
| wetSpring | V128 | 1,443+ | MCP tools, cast module, FMA |
| airSpring | v0.10.0 | 1,207+ | MCP tools, deny.toml, provenance registry |
| healthSpring | V37 | 706 | 18 proptest IPC fuzz, MCP tools |
| ludoSpring | V14 | 187 | with_provenance(), 93.2% coverage |
| primalSpring | v0.3.0-dev | 248 | Live atomic harness, capability-first, 17 RPC endpoints |

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

Phase 4: Live Primals — Tower Atomic (Track 1, exp001–002)
  → Live security + discovery capability providers
Phase 5: Full NUCLEUS (Track 1, exp004)
  → All primals deployed and validated
Phase 5: Graph execution (Track 2)
  → All 5 coordination patterns with real primals
Phase 6: Emergent systems (Track 3)
  → RootPulse, coralForge pipeline validated
Phase 7: Bonding (Track 4)
  → Multi-gate coordination
Phase 8: Cross-spring (Track 6)
  → Full ecosystem integration
Phase 9: Showcase patterns (Track 7)
  → phase1/phase2 mined coordination patterns validated
```
