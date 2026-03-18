# primalSpring — Coordination and Composition Spring

**Domain**: Primal coordination, atomic composition, graph execution, emergent systems, bonding  
**Version**: 0.2.0 (Phase 2 — niche self-knowledge, deploy graph validation, IPC resilience, 157 tests)  
**License**: AGPL-3.0-or-later  
**Last Updated**: March 18, 2026

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
| Tests | **157** (148 unit + 9 integration) |
| Experiments | 38 (7 tracks) |
| Clippy (pedantic + nursery) | **0 warnings** |
| `cargo fmt` | **clean** |
| `#![forbid(unsafe_code)]` | all files |
| C dependencies | 0 |
| Files over 1000 LOC | 0 |
| Neural API integration | `neural-api-client-sync` |
| Server mode | JSON-RPC 2.0 over Unix socket |
| Niche self-knowledge | `niche.rs` — 21 capabilities, semantic mappings, cost estimates |
| Deploy graph validation | `deploy.rs` — 6 biomeOS TOMLs (structural + live) |
| Meta-validator | `validate_all` binary — runs all 38 experiments |
| Workspace lints | centralized in `Cargo.toml` |

### Key Capabilities

- **Real IPC**: `probe_primal()`, `validate_composition()`, `health_check()` connect
  to live primals via Unix socket JSON-RPC 2.0
- **IPC resilience stack**: `IpcError` (8 typed variants), `CircuitBreaker`,
  `RetryPolicy`, `resilient_call()`, `DispatchOutcome<T>`, `extract_rpc_result`/`extract_rpc_dispatch`
- **4-format capability parsing**: Handles Format A (string array), B (object array),
  C (method_info nested), D (semantic_mappings double-nested)
- **Health probes**: `health.liveness`, `health.readiness` (Kubernetes-style) in server and client
- **Neural API bridge**: `NeuralBridge` from `neural-api-client-sync` for ecosystem
  discovery — no hardcoded primal rosters
- **Graceful degradation**: `check_skip()` and `check_or_skip()` for honest reporting
  when primals are not running
- **Validation harness**: `OrExit<T>`, `ValidationSink` (StdoutSink, NullSink), `safe_cast` module
- **JSON output**: `PRIMALSPRING_JSON=1` for CI pipeline integration
- **Server mode**: `primalspring_primal server` exposes `coordination.validate_composition`,
  `coordination.discovery_sweep`, `coordination.neural_api_status`, `graph.list`, and `graph.validate`
- **Niche self-knowledge**: `niche.rs` defines 21 capabilities, semantic mappings, operation
  dependencies, cost estimates, and `register_with_target()` for biomeOS registration
- **Deploy graph validation**: `deploy.rs` parses biomeOS TOML graphs, validates structure
  (names, binaries, dependencies, ordering), and probes running primals

---

## Track Structure (7 Tracks, 38 Experiments)

### Track 1: Atomic Composition (exp001–006)

Deploy each atomic layer, validate every primal starts, discovers peers,
and responds to capability calls.

| Exp | What | Primals | Status |
|-----|------|---------|--------|
| 001 | Tower Atomic bootstrap | BearDog + Songbird | **IPC wired** |
| 002 | Node Atomic compute | Tower + ToadStool | Discovery wired |
| 003 | Nest Atomic storage | Tower + NestGate | Discovery wired |
| 004 | Full NUCLEUS | All primals + Squirrel | **IPC wired** |
| 005 | Atomic subtraction | Graceful degradation | Discovery wired |
| 006 | Startup ordering | Dependency resolution | Discovery wired |

### Track 2: Graph Execution (exp010–015)

Validate all 5 coordination patterns with real primals.

| Exp | Pattern | Graph |
|-----|---------|-------|
| 010 | Sequential | rootpulse_commit.toml |
| 011 | Parallel | parallel_capability_burst.toml |
| 012 | ConditionalDag | conditional_fallback.toml |
| 013 | Pipeline | streaming_pipeline.toml |
| 014 | Continuous | continuous_tick.toml |
| 015 | PathwayLearner | Metrics + optimization |

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
| 051 | Socket discovery sweep | Composition-driven XDG enumeration |
| 052 | Protocol escalation | HTTP -> JSON-RPC -> tarpc negotiation |
| 053 | Multi-primal lifecycle | 6-primal research paper lifecycle |
| 054 | Bearer token auth | BearDog authenticate -> validate -> compute |
| 055 | Wait-for-health | Repeated health probes with timeout and ordering |
| 056 | Cross-tower federation | BYOB manifest, cross-tower discovery |
| 057 | Supply chain provenance | 7-stage DAG with per-agent signing |
| 058 | Semantic attribution | Module/feature/function tracking + fair credit |
| 059 | Weak force isolation | Zero leakage with unknown primals |

---

## Capability Domain

```
coordination.validate_composition   — Validate atomic compositions
coordination.discovery_sweep        — Discover all primals in a composition
coordination.neural_api_status     — Neural API health
health.check                       — Full self health status
health.liveness                    — Kubernetes-style liveness probe
health.readiness                   — Kubernetes-style readiness probe
capabilities.list                  — Niche capabilities + semantic mappings + cost estimates
graph.list                         — Structurally validate all deploy graphs
graph.validate                     — Validate a specific graph (structural or live)
lifecycle.status                   — Primal status report
```

---

## Active Handoffs

| Version | File | Date | Scope |
|---------|------|------|-------|
| v0.2.0 | `PRIMALSPRING_V020_ECOSYSTEM_ABSORPTION_EVOLUTION_HANDOFF_MAR18_2026.md` | Mar 18 | Ecosystem absorption: IPC resilience, niche, deploy, 157 tests |
| v0.2.0 | `PRIMALSPRING_V020_TOADSTOOL_BARRACUDA_COMPUTE_TRIANGLE_HANDOFF_MAR18_2026.md` | Mar 18 | Compute triangle coordination intelligence for toadStool/barraCuda team |
| v0.2.0 | `PRIMALSPRING_V020_TOADSTOOL_BARRACUDA_EVOLUTION_HANDOFF_MAR18_2026.md` | Mar 18 | barraCuda evolution + absorption recommendations for toadStool team |

## Archived Handoffs

| Version | File | Date | Scope |
|---------|------|------|-------|
| v0.1.1 | `archive/PRIMALSPRING_V011_CROSS_ECOSYSTEM_ABSORPTION_HANDOFF_MAR18_2026.md` | Mar 18 | IPC resilience absorption (superseded by V020) |
| v0.1.0 | `archive/PRIMALSPRING_V010_NEURAL_API_EVOLUTION_HANDOFF_MAR17_2026.md` | Mar 17 | Neural API integration + server mode |
| v0.1.0 | `archive/PRIMALSPRING_V010_DEEP_DEBT_AUDIT_EVOLUTION_HANDOFF_MAR17_2026.md` | Mar 17 | Deep debt audit + workspace lint consolidation |
| v0.1.0 | `archive/PRIMALSPRING_V010_COMPREHENSIVE_AUDIT_EVOLUTION_HANDOFF_MAR17_2026.md` | Mar 17 | Initial audit + deep debt evolution |
| v0.1.0 | `archive/PRIMALSPRING_V010_TOADSTOOL_BARRACUDA_COORDINATION_HANDOFF_MAR17_2026.md` | Mar 17 | toadStool/barraCuda coordination intelligence |

## Convention

**Naming**: `PRIMALSPRING_V{VERSION}_{TOPIC}_HANDOFF_{DATE}.md`

**Flow**: primalSpring → biomeOS (composition), primalSpring → toadStool/barraCuda (coordination intelligence). No reverse dependencies.

---

## Cross-Spring Context

primalSpring is unique: cross-spring coordination is its core mission.
Every experiment involves multiple primals or springs.

| Spring | What primalSpring Learns |
|--------|-------------------------|
| hotSpring | Precision validation patterns (f64 tolerance tiers) |
| wetSpring | Deep integration patterns (354 binaries, 214 named tolerances) |
| airSpring | NUCLEUS niche deployment (30 capabilities, 4 deploy graphs) |
| groundSpring | Uncertainty quantification (tolerance provenance system) |
| neuralSpring | Graph execution validation (`validate_biomeos_graph`) |
| ludoSpring | Cross-spring experiment patterns, session decomposition |
| healthSpring | Provenance trio resilience (circuit breaker, graceful degradation) |

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
