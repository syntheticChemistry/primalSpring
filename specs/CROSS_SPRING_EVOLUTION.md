# primalSpring — Cross-Spring Evolution

**Date**: March 28, 2026
**Status**: Phase 21 — Deep Ecosystem Audit + Library Consolidation (87/87 gates), 411 tests, 63 experiments, 59 deploy graphs, 5 spring primal binaries in plasmidBin

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
| 4 (Bonding) | Songbird (mesh), BearDog (trust), all NUCLEUS | Multi-gate bonding (Covalent, Metallic, Ionic, Weak, OrganoMetalSalt) |
| 5 (coralForge) | neuralSpring, wetSpring, hotSpring, ToadStool, NestGate | Pipeline graph |
| 6 (Cross-Spring) | airSpring, wetSpring, neuralSpring, petalTongue, Squirrel | Data flow |
| 7 (Showcase) | coralReef, toadStool, barraCuda, BearDog, NestGate, sweetGrass, rhizoCrypt | Mined patterns |
| 8 (Multi-Node) | Songbird (mesh+STUN), BearDog (lineage), NestGate (replication), Trio | Federation, idle compute, data sync |
| 9 (gen4 Bridge) | esotericWebb, ludoSpring, all NUCLEUS primals | Product composition health, session pipeline ordering, capability drift detection |

## What primalSpring Learns from Each Spring

| Spring | Lesson | Absorbed in |
|--------|--------|-------------|
| hotSpring V0.6.32 | Precision validation (170 tolerances, provenance), OnceLock GPU probes | v0.2.0 (pattern), v0.3.0 (provenance), v0.7.0-12.2 (OnceLock probes) |
| wetSpring V133 | Deep IPC (354 bins, 214 tolerances, MCP, skip_with_code, NdjsonSink, is_recoverable) | v0.2.0 (resilience), v0.3.0 (MCP), v0.7.0-12.1 (exit_code_skip_aware), v0.7.0-12.2 (NdjsonSink, is_recoverable) |
| airSpring V010 | NUCLEUS niche deployment (41 caps, deny.toml, MCP, Transport enum) | v0.2.0 (niche), v0.3.0 (deny.toml, MCP), v0.7.0-12.1 (cast lints), v0.7.0-12.2 (Transport) |
| groundSpring V121 | Typed errors, ValidationSink, check_relative, normalize_method, NdjsonSink | v0.2.0 (ValidationSink, OrExit), v0.7.0-12.1 (section, write_summary), v0.7.0-12.2 (check_relative, normalize_method, NdjsonSink) |
| neuralSpring V122 | Capability registry, primal_names, cast lints, is_recoverable, OnceLock | v0.3.0 (capability_registry), v0.7.0-12.1 (primal_names, cast lints), v0.7.0-12.2 (is_recoverable, OnceLock) |
| ludoSpring V14 | ValidationResult::with_provenance(), #[expect(reason)] | v0.3.0 (structured provenance) |
| healthSpring V42 | Proptest IPC fuzz, epoch-based circuit breaker, check_abs_or_rel, Transport | v0.3.0 (proptest), v0.7.0-12.1 (proptest_ipc, circuit breaker), v0.7.0-12.2 (check_abs_or_rel, Transport) |

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
| Coordination experiment framework | 63 experiments across 13 tracks |
| MCP coordination tools | Available for Squirrel AI routing |
| Tower + Squirrel AI composition | Live demo: Tower + Squirrel + Anthropic Claude (exp061) |
| `passthrough_env` pattern | Secure env forwarding for API keys, GPU config vars |
| `PrimalProcess::from_parts()` | Custom spawn for primals with non-standard transports |
| Abstract socket integration | Squirrel Universal Transport on Linux abstract namespace |
| BondType full taxonomy | Covalent, Metallic, Ionic, Weak, OrganoMetalSalt — chemistry-inspired bonding |
| BondingConstraint + BondingPolicy | Capability-scoped permissions, bandwidth limits, time windows for federated sharing |
| Multi-node deploy graph templates | basement_hpc, friend_remote, idle_compute, data_federation TOML graphs |
| Graph bonding metadata validation | Parses [graph.metadata] + [graph.bonding_policy] for structural consistency |
| STUN tier config + sovereignty-first | 4-tier NAT traversal: Lineage → Self-hosted → Public → Rendezvous |
| TrustModel taxonomy | GeneticLineage, Contractual, Organizational, ZeroTrust |
| Idle compute policy validation | BondingPolicy presets for time-windowed, capability-scoped compute sharing |
| Data federation pipeline | 7-phase NestGate replication with provenance trio tracking |

## gen4 Bridge Role

primalSpring is uniquely positioned to bridge gen3→gen4. It already validates
that primals compose — gen4 extends this to "primals compose into products."

### esotericWebb Composition Contract

Esoteric Webb (sporeGarden/esotericWebb) deploy graphs declare `primalspring_primal`
as a post-deploy validation node. Six composition health capabilities are expected:

| Capability | Webb Graph | Primal Stack |
|------------|-----------|--------------|
| `composition.webb_tower_health` | `webb_tower.toml` | BearDog + Songbird |
| `composition.webb_node_health` | `webb_node.toml` | Tower + ToadStool |
| `composition.webb_nest_health` | `webb_nest.toml` | Tower + NestGate |
| `composition.webb_ai_viz_health` | `webb_ai_viz.toml` | Tower + Squirrel + PetalTongue |
| `composition.webb_provenance_health` | `webb_provenance.toml` | Nest + Provenance Trio |
| `composition.webb_full_health` | `webb_full.toml` | All 8 domains |

These map to primalSpring's existing atomic tiers (Tower/Node/Nest/NUCLEUS)
but add product-specific assertions: Webb's `PrimalBridge` uses TCP-first
discovery, `resilient_call` with circuit breakers, and four degradation
patterns (`call_or_default`, `call_fire`, `call_extract_id`, `call_passthrough`).

### What primalSpring Must Validate for gen4

1. **Composition health endpoints** — `composition.webb_*_health` RPCs that
   run the same stack validation Webb's deploy graphs expect
2. **Capability string consistency** — detect drift between Webb's capability
   registry (`webb/capability_registry.toml`), bridge method constants
   (`webb/src/ipc/mod.rs`), deploy graph capabilities, and niche YAML
3. **Transport priority** — TCP-first, UDS fallback (matching `PrimalBridge::discover`)
4. **Resilience semantics** — circuit breaker open/short-circuit, retry with
   exponential backoff, `is_recoverable` classification
5. **Session pipeline ordering** — narrate → dialogue → flow → render → DAG
   append → complete (6 sequential phases)
6. **Degradation correctness** — when primals are absent, sensible defaults,
   never panics

### ludoSpring Co-Evolution

ludoSpring validates game science; primalSpring validates the IPC and composition
that makes those models available to gen4 products. Together:

- ludoSpring proves `game.*` RPCs return correct game science results
- primalSpring proves `game.*` RPCs arrive reliably via IPC composition
- Webb's `GameSession::act()` calls both in sequence — mechanical resolution
  (ludoSpring) then enrichment pipeline (all primals)

### helixVision — Second gen4 Product

helixVision (sporeGarden, planned) is the second gen4 product primalSpring
validates. It composes wetSpring genomics (16S pipeline, microbiome analytics)
and coralForge structure prediction (AlphaFold primitives) into a sovereign
genomics discovery platform.

helixVision proves the sporeGarden pattern works for science — same
PrimalBridge, same deploy graphs, same degradation — different domain. Where
Webb composes `game.*` and `ai.*` RPCs into a narrative pipeline, helixVision
composes `compute.*`, `storage.*`, and provenance RPCs into a sample-to-
publication pipeline.

primalSpring's composition health endpoints (`composition.webb_*_health`) are
Webb-specific, but the underlying validation — can these primals compose via
TCP IPC with graceful degradation? — applies directly to helixVision's
Sequence/Provenance/Field/Full deploy graph tiers.

## Ecosystem State (March 23, 2026)

| Spring | Version | Tests | Key Absorption |
|--------|---------|-------|----------------|
| hotSpring | v0.6.32 | 848 | Sovereign GPU, CoralCompiler IPC |
| groundSpring | V120 | 960+ | Typed errors, 13-tier tolerances, OnceLock GPU cache |
| neuralSpring | S170 | 1,320+ | Capability registry, display names, cast lints |
| wetSpring | V132 | 1,443+ | MCP tools, cast module, 214 tolerances, FMA |
| airSpring | v0.10.0 | 1,207+ | MCP tools, deny.toml, 58 tolerances, NUCLEUS niche TOML |
| healthSpring | V41 | 719 | proptest IPC fuzz, circuit breaker, tracing, DOI provenance |
| ludoSpring | V29 | 187 | with_provenance(), XDG sockets, 93.2% coverage |
| primalSpring | v0.7.0 | 411 | Phase 21 — 87/87 gates, deep ecosystem audit + library consolidation, 63 experiments, 59 deploy graphs |

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
  → 87/87 total gates, 253+ tests, 49 experiments

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
  → Resolved: trio teams inlined types, provenance-trio-types shim deleted (Mar 22)
  → 87/87 total gates, 253+ tests, 49 experiments

Phase 11 (done): Provenance Trio Neural API Integration (March 22, 2026)
  → ipc::provenance module: begin_session, record_step, complete_experiment
  → Full RootPulse pipeline: dehydrate → commit → attribute via capability.call
  → rootpulse_branch, rootpulse_merge, rootpulse_diff, rootpulse_federate
  → exp020 evolved: 6-phase commit via Neural API (graceful degradation)
  → exp021 evolved: branch/merge operations via capability.call
  → exp022 evolved: diff/federate operations via capability.call
  → exp041 evolved: E2E provenance chain (session → steps → pipeline)
  → Zero compile-time coupling to trio crates (all via capability.call)
  → Trio teams inlined types — no provenance-trio-types dependency anywhere
  → Release binaries built and in plasmidBin/primals/
  → Live probing revealed 3 gaps + 2 working primals (see Phase 11.1)

Phase 11.1 (done): Live Trio Probing (March 23, 2026)
  → sweetGrass: LIVE — Unix socket + HTTP JSON-RPC, 24 methods, 9 domains, PROV-O response
  → rhizoCrypt: LIVE (TCP only) — HTTP JSON-RPC on :9401/rpc, full DAG lifecycle works
  → loamSpine: BROKEN — panic in infant_discovery (nested runtime block_on)
  → Gap 1: rhizoCrypt TCP-only (no Unix socket, ignores RHIZOCRYPT_SOCKET env var)
  → Gap 2: loamSpine runtime panic (cannot block_on inside async runtime)
  → Gap 3: Event type wire format mismatch (struct variants, not strings)
  → Gap 4: braid.create/pipeline.attribute param schemas differ from ipc::provenance
  → Validated: DAG session lifecycle (create → ExperimentStart → Observation → merkle root)
  → Validated: sweetGrass braid.create (returns JSON-LD PROV-O with DID attribution)
  → Validated: sweetGrass pipeline.attribute (returns commit_ref + merkle_root)
  → sweetGrass capability.list reports consumed_capabilities matching primalSpring graph

Phase 12 (done): Multi-Node Bonding + Federation (March 23, 2026)
  → BondType expanded: Covalent, Metallic, Ionic, Weak, OrganoMetalSalt (5 variants)
  → TrustModel: GeneticLineage, Contractual, Organizational, ZeroTrust
  → BondingConstraint: capability allow/deny lists, bandwidth, concurrency limits
  → BondingPolicy: bond type + trust + constraints + time windows + relay offer
  → 4 multi-node deploy graphs: basement_hpc, friend_remote, idle_compute, data_federation
  → graph_metadata.rs: parse + validate [graph.metadata] and [graph.bonding_policy] from TOML
  → stun_tiers.rs: 4-tier STUN config parser, sovereignty-first escalation validation
  → exp071: idle compute policy (capability masks, time windows, bandwidth, graph metadata)
  → exp072: data federation (NestGate replication, trio provenance, 7-phase pipeline)
  → Evolved exp030 (covalent), exp032 (plasmodium+Metallic), exp056 (cross-tower+3 graphs)
  → 303 tests (incl. 12 bonding, 6 graph metadata, 6 STUN tier, 7 cross-cutting proptest), 51 experiments, 22 graphs (at time of Phase 12)

Phase 12.1 (done): Ecosystem Absorption Wave 1 (March 23, 2026)
  → deny.toml convergence: merged groundSpring V121 + wetSpring V133 C-dep bans
  → Cast discipline lints: neuralSpring S170 + airSpring V010 clippy cast_* workspace-wide
  → ValidationSink enrichment: section() + write_summary() from groundSpring V121
  → exit_code_skip_aware(): 3-way exit from wetSpring V133 (0=pass, 1=fail, 2=all-skipped)
  → proptest_ipc module: 7 cross-cutting property tests fuzzing IPC pipeline (healthSpring V42)
  → primal_names module: canonical display↔slug mapping for 23 primals/springs
  → Provenance circuit breaker: epoch-based + exponential backoff (healthSpring V42)
  → 303 tests (up from 280)

Phase 12.2 (done): Deep Ecosystem Absorption Wave 2 (March 23, 2026)
  → normalize_method(): ecosystem-wide JSON-RPC prefix-agnostic dispatch
  → check_relative() + check_abs_or_rel(): robust numeric tolerance validation
  → NdjsonSink: streaming newline-delimited JSON validation output
  → IpcError::is_recoverable(): broader recovery classification beyond is_retriable()
  → Transport enum (Unix + Tcp): cross-platform IPC with connect_transport() address parsing
  → ipc::probes: OnceLock-cached runtime resource probes for test parallelism
  → missing_docs → deny: all public items fully documented, lint upgraded
  → validate_release.sh: fmt + clippy + deny + test floor (320) + docs quality gate
  → Server dispatch wired through normalize_method()
  → 360 tests (up from 303), zero clippy, zero missing docs, zero unsafe, zero C deps

Phase 13 (done): Cross-Gate Deployment Tooling (March 23, 2026)
  → build_ecosystem_musl.sh: x86_64 + aarch64 musl static builds for all primals
  → prepare_spore_payload.sh: USB spore assembly (binaries + graphs + genetics)
  → validate_remote_gate.sh: remote gate NUCLEUS health via TCP JSON-RPC
  → exp073: LAN covalent mesh (remote Songbird mesh + BirdSong beacon exchange)
  → exp074: cross-gate health (per-primal TCP health + capabilities + composition)
  → exp063 evolved: cross-device Pixel beacon exchange via TCP
  → basement_hpc_covalent.toml: annotated with full gate inventory from HARDWARE.md
  → LAN_COVALENT_DEPLOYMENT_GUIDE handoff for all gate operators
  → 53 experiments (up from 51), 10 tracks (up from 9)

Phase 14: Deep Debt + Builder Pattern + Full Provenance (March 24, 2026)
  → Builder-pattern ValidationResult::run() on all 53 experiments
  → 100% structured provenance via with_provenance() on all experiments
  → Smart module extraction: validation/tests.rs (1016 → 540+493 LOC)
  → Zero .unwrap() in experiment binaries (all .or_exit())
  → Zero #[allow()] in production (all #[expect(reason)])
  → 361 tests, 0 clippy/doc/fmt warnings

Phase 15: Cross-Ecosystem Absorption (March 24, 2026)
  → primal_names slug constants (BEARDOG, SONGBIRD, etc.) — zero hardcoded names
  → unwrap_used/expect_used = warn workspace-wide (healthSpring V42 / wetSpring V135)
  → launcher/mod.rs smart refactored (802 → 699 LOC, tests extracted)
  → CONTRIBUTING.md + SECURITY.md (neuralSpring V124 ecosystem standard)
  → ipc::provenance docs updated for rhizoCrypt sled→redb migration
  → Zero clippy warnings on --all-targets including unwrap/expect discipline

Phase 16 (done): Deep Debt Audit + Centralized Tolerances (March 24, 2026)
  → Comprehensive audit against ecosystem standards (zero debt found in critical paths)
  → TRIO_CIRCUIT_THRESHOLD centralized to tolerances (was local const in provenance)
  → Provenance trio retry params centralized (TRIO_RETRY_ATTEMPTS, TRIO_RETRY_BASE_DELAY_MS)
  → Remote gate TCP port defaults centralized (DEFAULT_{PRIMAL}_PORT constants)
  → All tolerance calibration notes updated from "pending" to Phase 15 operational data
  → extract_capability_names deduplicated (coordination delegates to ipc::discover 4-format parser)
  → exp010 hardcoded description → capability-based semantic check
  → exp073/074 inline ports → tolerances constants + env override
  → coordination tests → primal_names slug constants (zero string literals)
  → validate_all doc comment corrected (not build-time discovery)
  → Coverage baseline measured (cargo llvm-cov)
  → 364 tests (up from 361), zero clippy, zero fmt diff, zero deny issues

Phase 17: gen4 Deployment Evolution — biomeOS Substrate Validation
  → biomeOS neural-api running on Eastgate in coordinated mode (24 capability domains, 39 graphs)
  → Capability routing validated: crypto.generate_keypair, beacon.generate via biomeOS → BearDog
  → Cross-gate routing: Eastgate biomeOS orchestrates Pixel Tower via TCP (ADB-forwarded)
  → Squirrel AI primal validated via abstract socket @squirrel + biomeOS ai.* domain registered
  → petalTongue ui_atomic graph loaded in biomeOS, structural validation passing
  → Birdsong encrypted beacon generation validated (direct Songbird + biomeOS capability routing)
  → Spring deploy sweep: all 7 spring + 4 pipeline graphs loaded in biomeOS (39 total)
  → Cross-spring ecology graph validated: 9 nodes, ET₀ → diversity → spectral pipeline
  → New experiments: exp075 (biomeOS live), exp076 (cross-gate), exp077 (Squirrel bridge),
    exp078 (petalTongue viz), exp079 (spring sweep), exp080 (cross-spring ecology)
  → New graph templates: graphs/spring_validation/ (7), graphs/cross_spring/ (2), graphs/gen4/ (4)
  → gen4 prototypes: sovereign tower, science substrate, agentic tower, interactive substrate
  → Known gaps: Squirrel uses abstract sockets (biomeOS routes to filesystem sockets),
    no aarch64 biomeOS binary for Pixel substrate deployment (biomeOS-scope work)

Phase 18: Live NUCLEUS + Cross-Gate Federation (March 28, 2026) ✅
  → Full NUCLEUS on Eastgate: biomeOS + BearDog + Songbird + NestGate + Squirrel running concurrently
  → FAMILY_ID reconciliation: all primals use seed-derived 8ff3b864a4bc589a (matching biomeOS internal routing)
  → biomeOS capability.call validated: crypto (BearDog), storage (NestGate), AI (Squirrel)
  → Cross-gate: Pixel Songbird TCP (v0.1.0) → ADB forward → Eastgate biomeOS route.register (gate: pixel8a)
  → Mesh init: both Eastgate and Pixel mesh networks initialized, announce operational
  → STUN: Eastgate public address 162.226.225.148 via racing 3 servers
  → GrapheneOS SELinux gap: sock_file creation denied for shell context — blocks BearDog, biomeOS, NestGate on Android
  → Songbird only primal with --listen TCP IPC mode for mobile; BearDog/biomeOS need TCP-only server mode
  → biomeOS capability.call lacks gate-aware routing (ignores gate param, always uses primary endpoint)
  → Handoff: CROSS_GATE_MOBILE_TCP_TRANSPORT_GAP_HANDOFF_MAR28_2026.md

Phase 19: Gen4 Spring Scaffolding (March 28, 2026) ✅
  → Resolved broken path deps across 7 springs via symlinks (barraCuda, bingoCube, toadStool, coralReef, loamSpine, rhizoCrypt, sweetGrass)
  → Patched barraCuda: version 0.3.5→0.3.7, F16 precision variant, GPU feature-gating, missing DeviceCapabilities methods, rel_tolerance on Check, PrecisionRoutingAdvice re-export
  → Patched bingoCube/nautilus: json feature gate, input_dim on ShellConfig
  → Built 5/6 spring primal binaries (groundspring, healthspring_primal, ludospring, neuralspring, wetspring)
  → airspring_primal BLOCKED: internal data::Provider / data::NestGateProvider API drift
  → Binaries stripped + deployed to plasmidBin/springs/, blake3 checksums recorded
  → plasmidBin manifest.toml, sources.toml, checksums.toml, doctor.sh updated
  → gen4_spring_composition.toml: master graph (Tower + biomeOS + 5 springs + cross-spring validation)
  → All 7 spring validation graphs updated with biomeOS substrate node (start_biomeos, order 2)
  → Launch profiles added for all 6 springs in primal_launch_profiles.toml
  → 59 deploy graphs, 5 spring binaries in plasmidBin/springs/

Phase 20: LAN Covalent Deployment
  → Live multi-gate NUCLEUS deployment with BirdSong beacon exchange
  → BearDog + biomeOS TCP-only mode for mobile (unblocks full Pixel NUCLEUS)
  → 10G mesh backbone validation

Phase 21: Live Multi-Node Validation (Track 8)
  → Basement HPC: deploy NUCLEUS on 2+ LAN machines, validate covalent mesh formation
  → Friend remote: NAT traversal via STUN tiers, hole-punch, relay fallback
  → Idle compute: validate BondingPolicy enforcement (time windows, capability scope)
  → Data federation: NestGate cross-site replication with trio provenance chain
  → Plasmodium: decentralized capability aggregation across covalently bonded nodes

Phase 22: Emergent Systems End-to-End (Track 3)
  → RootPulse commit/branch/merge/diff/federate with live trio (ipc::provenance wired)
  → coralForge pipeline streaming (exp013) — needs sweetGrass running
  → Continuous 60Hz tick (exp014) — needs provenance trio running
  → RPGPT session management with provenance tracking

Phase 23: Bonding Live Coordination (Track 4)
  → Multi-gate covalent mesh with BirdSong encrypted beacons
  → Ionic contract-based metered bonds (cloud burst, external APIs)
  → Metallic electron-sea: homogeneous fleet specialization (compute-only racks)
  → Weak force: zero-trust read-only bonds to unknown primals
  → OrganoMetalSalt: mixed bond types in a single deployment

Phase 24: Cross-Spring Integration (Track 6)
  → Full ecosystem data flow: airSpring, wetSpring, neuralSpring, petalTongue, Squirrel
  → wetSpring uses provenance trio to track genetic data lineage
  → Cross-spring BYOB composition: primals as DAG execution nodes

Phase 25: Showcase Patterns (Track 7)
  → phase1/phase2 mined coordination patterns validated end-to-end
  → Compute triangle, bearer token auth, supply chain provenance, semantic attribution

Phase 26: Anchoring + Economics
  → sweetGrass anchoring.anchor → BTC/ETH (hash attestation, not currency)
  → loamSpine certificates as Novel Ferment Transcripts (NFTs)
  → sunCloud radiating attribution via sweetGrass braids

Phase 27: biomeOS Self-Composition
  → biomeOS composes its own graphs at runtime
  → Dynamic capability negotiation for BYOB niche creation
```
