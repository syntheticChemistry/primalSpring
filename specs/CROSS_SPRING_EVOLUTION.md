# primalSpring — Cross-Spring Evolution

**Date**: March 18, 2026  
**Status**: Phase 2 — Ecosystem absorption, IPC resilience stack, 132 tests

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

| Spring | Lesson |
|--------|--------|
| hotSpring | Precision validation patterns (f64 tolerance tiers) |
| wetSpring | Deep integration patterns (354 binaries, 214 named tolerances) |
| airSpring | NUCLEUS niche deployment (30 capabilities, 4 deploy graphs) |
| groundSpring | Uncertainty quantification (tolerance provenance system) |
| neuralSpring | Graph execution validation (`validate_biomeos_graph`) |
| ludoSpring | Cross-spring experiment patterns (exp041–044) |
| healthSpring | Provenance trio resilience (circuit breaker, graceful degradation) |

## Evolution Path

```
Phase 0 (done): Scaffolding (March 2, 2026)
  → 38 experiments scaffolded, workspace compiles

Phase 0→1 (done): Real Discovery (March 17, 2026)
  → Crate rename (primalspring-barracuda → primalspring)
  → IPC module evolved: discover + protocol + client (3 modules)
  → All experiments use real discover_primal() + honest check_skip

Phase 1 (done): Neural API + Deep Debt (March 17, 2026)
  → neural-api-client-sync integrated (biomeOS path dep)
  → KNOWN_PRIMALS removed — sovereignty fix
  → Discovery evolved: composition-driven + Neural API
  → Server mode: JSON-RPC 2.0 over Unix socket
  → probe_primal(), validate_composition(), health_check()
  → validation: check_or_skip(), JSON output, exit_code()
  → Workspace lints centralized, 69 unit tests
  → exp001 + exp004 IPC-wired with graceful degradation
  → Zero warnings: check, clippy (pedantic+nursery), doc, fmt

Phase 2 (done): Ecosystem Absorption (March 18, 2026)
  → Absorbed IPC resilience from 7 sibling springs
  → IpcError (8 typed variants), CircuitBreaker, RetryPolicy, resilient_call()
  → DispatchOutcome<T>, extract_rpc_result<T>(), extract_rpc_dispatch<T>()
  → 4-format capability parsing (Formats A–D)
  → health.liveness / health.readiness (Kubernetes-style probes)
  → safe_cast module, OrExit<T> trait, ValidationSink trait
  → PRIMAL_NAME / PRIMAL_DOMAIN constants
  → All 38 experiments evolved with real probe patterns
  → FAMILY_ID-aware discovery, Neural API health checks
  → proptest for IPC protocol fuzzing
  → 132 unit tests, zero warnings, v0.2.0

Phase 3: Live Primals — Tower Atomic (Track 1, exp001–002)
  → BearDog + Songbird real IPC validation
Phase 4: Full NUCLEUS (Track 1, exp004)
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
