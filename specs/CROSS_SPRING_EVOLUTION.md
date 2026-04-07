# primalSpring — Cross-Spring Evolution

**Date**: April 7, 2026
**Status**: Phase 23c+ — trio witness harvest, garden acceleration (87/87 gates), 402+ tests, 89 experiments, 89 deploy graphs, trio binaries in plasmidBin

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

## Modernization Sweep (April 7, 2026)

### What Changed (April 7 — Pattern Modernization)

- **Capability naming cleanup (NA-009 resolved)**: `dag.dehydrate` → `dag.dehydration.trigger`
  across `capability_registry.toml`, `niche.rs`, and 17 graph files. Also fixed stale
  `dag.create_session`/`dag.append_event`/`dag.merkle_root` → dotted canonical names in
  `primalspring_deploy`, `nucleus_complete`, `continuous_tick`. `commit.session` →
  `session.commit`, `commit.entry` → `entry.append` in all loamSpine capability lists.
- **HTTP health probe deprecated**: `http_health_probe` in `tcp.rs` marked `#[deprecated]`
  with guidance to use TCP JSON-RPC `health.liveness`. All experiments (exp073, exp074,
  exp076, exp081) updated to use `tcp_rpc` instead. `ProbeProtocol::Http` variant removed
  from exp074 and exp081. Songbird no longer exposes HTTP /health on a port — Tower Atomic
  owns all HTTP.
- **Graph format unified (NA-016 resolved)**: primalSpring deploy parser now accepts three
  TOML formats: `[[graph.node]]` (legacy), `[[graph.nodes]]` (biomeOS native), and
  top-level `[[nodes]]` (multi-node shorthand). Implemented via `#[serde(alias = "nodes")]`
  on `GraphMeta.node` and a top-level `nodes` merge in `load_graph`. `GraphMeta` gains
  optional `id: Option<String>` for biomeOS `GraphId` support. All 87+ graphs migrated to
  `[[graph.nodes]]`. Multi-node files converted from `[[nodes]]` to `[[graph.nodes]]`.
- **nest-deploy.toml v4.0 (gold standard)**: Added `id = "nest-deploy"`, mesh capabilities
  (`mesh.init`, `mesh.auto_discover`, `mesh.peers`), HTTPS validation phase (Phase 5),
  and renumbered composition validation to Phase 6. Serves as canonical reference for all
  deployment graphs.
- **exp090 Tower Atomic LAN Probe**: New experiment for LAN discovery — mesh.init +
  mesh.auto_discover via BirdSong, peer capability enumeration, HTTPS through Tower,
  STUN/NAT discovery. Topology summary reports gate inventory.
- **exp073 modernized for covalent bonding**: Added neural-api `capability.call` routing
  validation, genetic lineage (`FAMILY_ID`) verification via BearDog, and HTTPS validation
  through remote Tower Atomic.
- **basement_hpc_covalent.toml updated**: Capability names modernized to dotted canonical
  form (`crypto.sign_ed25519`, `http.request`, `storage.fetch_external`, etc.). HTTPS
  validation phase added between gate validation and capability announcement.

## Garden Acceleration State (April 7, 2026)

### What Changed (April 6-7)

- **Nest Atomic composition validated live (4 primals)**: NestGate
  `storage.fetch_external` refactored to delegate HTTPS fetch via biomeOS
  `capability.call` → Songbird `http.request` → BearDog TLS. NestGate no
  longer uses `reqwest`/`rustls` directly; Tower Atomic is the single TLS
  boundary. Full e2e: NestGate → neural-api → Songbird → HTTP → BLAKE3 →
  cache → provenance. Squirrel (AI orchestration) also validated through
  the same `capability.call` routing: `context.create`, `tool.list`,
  `ai.list_providers` all routed through neural-api to Squirrel.
- **Nest deploy graph v3.1** (`graphs/nest-deploy.toml`): BearDog → Songbird →
  NestGate → Squirrel → validation. Validates via `biomeos deploy --validate-only`
  and `--dry-run`. Deployed live via neural-api `graph.execute`.
- **Tower Atomic HTTPS fixed**: Songbird TLS 1.3 client was sending empty
  `legacy_session_id` (rejected by CDN middleboxes) and using non-CSPRNG
  random. Fixed with 32-byte random session ID per RFC 8446 Appendix D.4,
  `getrandom` CSPRNG, and P-256 fallback in `supported_groups`. HTTPS
  validated against api.github.com, ifconfig.me, jsonplaceholder.typicode.com.
  Full Nest Atomic e2e: NestGate `storage.fetch_external` → biomeOS → Songbird
  TLS 1.3 → BearDog X25519/HKDF/AEAD → HTTPS 200 + BLAKE3 hash + cache.
- **16 Nest Atomic gaps documented** (NA-001 through NA-016) with severity,
  including Squirrel abstract socket transport (NA-001), graph format
  divergence (NA-016), and Node Atomic (ToadStool GPU) noted as next step.
- **Trio witness evolution**: `WireAttestationRef` -> `WireWitnessRef` across
  rhizoCrypt, loamSpine, sweetGrass. Self-describing `kind`/`encoding`/
  `algorithm`/`tier`/`context` fields. Trio is now algo-agnostic — tracks
  crypto signatures, hash observations, game checkpoints, conversation markers.
- **plasmidBin harvest**: Trio glibc binaries built, checksums updated,
  manifest versions bumped, doctor.sh + deploy_gate.sh updated.
- **exp089**: BearDog witness round-trip validation (offline + live crypto).
- **26 primal gaps resolved**, 5 remain (all low). BD-01 (BearDog encoding
  hint) added.
- **biomeOS Neural API handoff**: GAP-017 (benchScale ZOMBIE) + GAP-018
  (executor RPCs — partially resolved: pipeline + continuous now on JSON-RPC).

### Nest Atomic — Gaps, Debt, and Inconsistencies

Tracked during live Nest Atomic validation (April 7, 2026). These are
actionable items for primal teams to address before garden production use.

**Architecture / Routing**

| ID | Primal | Severity | Description |
|----|--------|----------|-------------|
| NA-001 | Squirrel | **HIGH** | Socket transport uses abstract namespace (`@squirrel`) instead of filesystem UDS. `--socket` CLI arg is logged but not honored for binding. biomeOS `capability.call` cannot forward to abstract sockets without a socat bridge. Squirrel's `universal_transport` layer needs a filesystem socket mode. |
| NA-002 | biomeOS | MEDIUM | `capability.call` forwarding intermittently fails on first request after registration. Retry succeeds. Possible race between registration confirmation and router table update. |
| NA-003 | biomeOS | LOW | `capability.register` accepts any socket path without verifying the socket exists or is connectable. Should probe health before confirming. |
| NA-004 | NestGate | MEDIUM | `fetch_external` falls back to `status` field from `status_code` in Songbird response. Should be a single canonical field name — coordinate with Songbird team. |

**Build / Binary**

| ID | Primal | Severity | Description |
|----|--------|----------|-------------|
| NA-005 | loamSpine | MEDIUM | `plasmidBin` binary is dynamically linked (`glibc`). Should be `musl-static` per ecoBin standard. |
| NA-006 | sweetGrass | MEDIUM | Same as NA-005: dynamically linked, pending musl-static cross-compile. |
| NA-007 | NestGate | LOW | Binary crate is `nestgate-bin`, not `nestgate`. Inconsistent with other primals where `cargo build -p <name>` yields the binary. |

**Schema / Wire Types**

| ID | Primal | Severity | Description |
|----|--------|----------|-------------|
| NA-008 | sweetGrass | LOW | Internal `Attestation` struct still uses `attested_at` field, while wire type `WireWitnessRef` uses `witnessed_at`. Field rename needed for consistency. |
| NA-009 | rhizoCrypt | **RESOLVED** | `capability_registry.toml` listed `dag.dehydrate` but the actual RPC method is `dag.dehydration.trigger`. Fixed: registry, niche.rs, and all 17 graph files updated to `dag.dehydration.trigger`. Also fixed stale `dag.create_session`/`dag.append_event`/`dag.merkle_root`/`commit.session`/`commit.entry` → canonical names across `primalspring_deploy`, `nucleus_complete`, `continuous_tick`, and `data_federation_cross_site` graphs. |
| NA-010 | Squirrel | LOW | `discovery.list` is implemented in the dispatcher but absent from `niche::CAPABILITIES`. Niche-vs-code drift not caught by tests. |
| NA-011 | Squirrel | LOW | `capability_registry.toml` not found at runtime ("No such file or directory"). Falls back to embedded defaults — registry file location needs alignment with CWD or env var. |
| NA-012 | Songbird | LOW | HTTP response uses both `status_code` and `status` in different code paths. Should standardize to one field (prefer `status_code` per HTTP semantics). |

**AI Provider Plumbing (Squirrel)**

| ID | Primal | Severity | Description |
|----|--------|----------|-------------|
| NA-013 | Squirrel | MEDIUM | No AI providers discovered at startup. Squirrel needs `http.request` capability from Songbird to route cloud API calls (Anthropic, OpenAI) through the Tower Atomic TLS boundary, but has no auto-discovery path for this. |
| NA-014 | Squirrel | LOW | `AI_HTTP_PROVIDERS` env var and `deprecated-adapters` feature gate create two disjoint provider init paths. Should converge on capability-based routing through Tower. |

**TLS**

| ID | Primal | Severity | Description |
|----|--------|----------|-------------|
| NA-015 | Songbird | **RESOLVED** | TLS 1.3 ClientHello was missing 32-byte `legacy_session_id` (RFC 8446 Appendix D.4 middlebox compat) and used non-CSPRNG random. Fixed: CSPRNG via `getrandom`, 32-byte session ID, P-256 fallback group. HTTPS now works against GitHub, Google, Cloudflare-fronted services. httpbin.org specifically rejects (Cloudflare WAF fingerprint issue, not a protocol failure). |

**Graph Deployment**

| ID | Primal | Severity | Description |
|----|--------|----------|-------------|
| NA-016 | primalSpring / biomeOS | **RESOLVED** | Graph format divergence eliminated. primalSpring parser now accepts `[[graph.node]]` (legacy), `[[graph.nodes]]` (biomeOS native), and top-level `[[nodes]]` via serde alias + merge. `GraphMeta` gains optional `id` field. All 87+ graphs migrated from `[[graph.node]]` to `[[graph.nodes]]`. Multi-node graphs converted from `[[nodes]]` to `[[graph.nodes]]` with `[graph.nodes.*]` subsections. `nest-deploy.toml` v4.0 established as gold standard. |

**Next Atomic: Node Atomic (ToadStool GPU + Local AI)**

The same composition pattern (graph-deployed, capability.call routed) extends
to local AI inference:

- **ToadStool** (GPU compute primal) provides `compute.execute` / `compute.submit`
- Squirrel routes `ai.query` to ToadStool when `AI_PROVIDER_SOCKETS` or biomeOS
  socket scan finds it (already coded in `router.rs`)
- Node Atomic = Nest Atomic + ToadStool: full local-first AI stack with no
  external API dependency
- Composition: BearDog → Songbird → NestGate → Squirrel → ToadStool
- ToadStool is `required = false` in Squirrel's niche graph; Squirrel degrades
  gracefully to cloud providers when ToadStool is absent

### Spring-to-Garden Acceleration Assignments

esotericWebb (first gen4 garden) has 11 open `EVOLUTION_GAPS.md` entries.
Each spring accelerates specific primal interactions that the garden needs.

| Webb Gap | Spring | Primal Interaction |
|----------|--------|--------------------|
| GAP-004 (trio E2E) | wetSpring | rhizoCrypt -> loamSpine -> sweetGrass pipeline with Anderson QS data |
| GAP-019 (BearDog crypto bridge) | primalSpring | BearDog sign/verify -> WireWitnessRef -> trio (exp089 validates) |
| GAP-010 (plasmidBin deploy) | primalSpring | deploy_gate.sh + doctor.sh + compositions |
| GAP-017 (biomeOS ZOMBIE) | primalSpring -> biomeOS | Neural API startup health in benchScale |
| GAP-018 (executor RPCs) | primalSpring + neuralSpring | Pipeline + Continuous now exposed; ConditionalDag implicit |
| GAP-002 (dialogue scene) | neuralSpring | petalTongue visualization.render.scene for CRPG |
| GAP-003 (cert-gated AI) | ludoSpring + primalSpring | Squirrel ai.query with loamSpine certificate constraints |
| GAP-006 (filtered discovery) | primalSpring | Songbird discovery.query with capability filter |
| GAP-021 (game primal) | ludoSpring | game.* RPCs as deployable primal binary |
| GAP-007 (voice preview) | ludoSpring + neuralSpring | Squirrel surrogate for offline AI |
| GAP-009 (RulesetCert) | ludoSpring | loamSpine certificate YAML spec |

### Cross-Spring Data Flow with Provenance

The trio witness wire type (`WireWitnessRef`) enables cross-spring data flow
with provenance tracking. Each spring emits witnesses appropriate to its domain:

| Spring | Witness Kind | Encoding | Use |
|--------|-------------|----------|-----|
| wetSpring | `"hash"` | hex | Data ingest content hash |
| wetSpring | `"signature"` | base64 | BearDog-signed pipeline result |
| wetSpring | `"checkpoint"` | none | Anderson QS pipeline step |
| ludoSpring | `"checkpoint"` | none | Game session tick / action |
| ludoSpring | `"signature"` | base64 | BearDog-signed sovereign save |
| hotSpring | `"hash"` | hex | GPU compute output hash |
| groundSpring | `"hash"` | hex | Measurement data hash |
| airSpring | `"checkpoint"` | none | ET0 pipeline step |
| neuralSpring | `"checkpoint"` | none | CoralForge pipeline stage |
| healthSpring | `"hash"` | hex | PK/PD result hash |

### helixVision (Second Garden, Planned)

helixVision composes wetSpring genomics + coralForge structure prediction
into a sovereign genomics platform. Same PrimalBridge, same deploy graphs,
same degradation patterns as esotericWebb. When wetSpring validates the
Anderson QS provenance pipeline E2E, the same pattern applies to helixVision's
sample-to-publication pipeline.

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

---

## Live Validation Matrix — April 7, 2026

### L0: Individual Primal Routing (Direct IPC)

| # | Domain | Provider | Direct Probe | Neural API Route (v2.81) | Neural API Route (v2.92) | Status |
|---|--------|----------|-------------|-------------------------|-------------------------|--------|
| 1 | security | BearDog | **PASS** (health, sign, hash) | **FAIL** (0 caps) | **FAIL** (Format E unrecognized) | GAP-MATRIX-01b: BearDog wire format |
| 2 | discovery | Songbird | **PASS** (health, find_primals) | **FAIL** (0 caps) | **PARTIAL** (14 caps registered, forwarding fails) | GAP-MATRIX-07: proxy forwarding |
| 3 | compute | ToadStool | NOT RUNNING | — | — | Need: start ToadStool process |
| 4 | storage | NestGate | NOT RUNNING | — | — | Need: start NestGate process |
| 5 | ai | Squirrel | NOT RUNNING | — | — | Need: start Squirrel process |
| 6 | dag | rhizoCrypt | NOT RUNNING | — | — | Need: start rhizoCrypt process |
| 7 | spine | loamSpine | NOT RUNNING | — | — | Need: start loamSpine process |
| 8 | braid | sweetGrass | NOT RUNNING | — | — | Need: start sweetGrass process |
| 9 | http | Songbird (Tower) | **PASS** (ifconfig.me HTTP 200) | **FAIL** (0 caps) | **PARTIAL** (crypto.delegate registered, forwarding fails) | GAP-MATRIX-07 |
| 10 | mesh | Songbird (BirdSong) | **FAIL** (mesh not initialized) | — | — | Expected: mesh.init required first |

### L1: Tower Atomic Composition

| Check | Result | Detail |
|-------|--------|--------|
| BearDog health.liveness | **PASS** | v0.9.0, 9 capability groups |
| crypto.sign_ed25519 | **PASS** | Ed25519 signature, 88-char base64 |
| crypto.blake3_hash | **PASS** | BLAKE3 hash of test data |
| Songbird health.liveness | **PASS** | Healthy |
| HTTPS via Tower (ifconfig.me) | **PASS** | HTTP 200, public IP obtained |
| HTTPS via Tower (httpbin.org) | **FAIL** | TLS handshake failure (cipher suite gap) |
| discovery.find_primals | **PASS** | Discovery operational |
| BearDog → Songbird composition | **PASS** | Songbird uses BearDog crypto for TLS |

### Critical Gaps Found

**GAP-MATRIX-01: Neural API capability registration — PARTIALLY RESOLVED (biomeOS v2.92)**

biomeOS v2.92 (commits `489f8d66` + `3cfeeecf`, April 7) added real JSON-RPC probing (`identity.get` + `capabilities.list`), 4-format capability parsing, and domain prefix matching (GAP-019).

**Result**: Songbird now discovered with **14 capabilities** (was 0). `capability.discover("network")` correctly routes to Songbird. BearDog still reports 0 capabilities — its `provided_capabilities` wire format (`[{type: "security", methods: [...]}]`) is Format E, not handled by the A-D parser. biomeOS logs: `Unrecognized capabilities.list response shape`.

**GAP-MATRIX-01b (NEW)**: BearDog Format E wire format unrecognized. biomeOS team needs to add Format E parsing: `result.provided_capabilities: [{type, methods, description?, version?}]`.

Severity: **Medium** (downgraded from Critical) — Songbird routing works, BearDog still blocked.

**GAP-MATRIX-07 (NEW): Neural API proxy forwarding fails after discovery**

biomeOS discovers the correct provider and endpoint, but `capability.call` forwarding fails with "Failed to forward {method} to unix:///...". The proxy layer cannot connect to primal sockets despite `capability.discover` correctly returning the endpoint. Likely a URI scheme handling issue (`unix:///path` vs bare path in the `AtomicClient`).

Impact: All `capability.call` forwarding fails even for correctly discovered primals.

Severity: **Critical** — blocks the `capability.call` path that all springs rely on. Direct IPC works as workaround.

**GAP-MATRIX-02: tower_atomic_bootstrap.toml fails to parse in biomeOS**

biomeOS logs: `Failed to parse TOML from: tower_atomic_bootstrap.toml`. The file is valid TOML (Python parser confirms). biomeOS's internal graph parser may require fields not present (e.g., `id`, or biomeOS-specific node structure).

Impact: biomeOS cannot load semantic translations from the Tower graph. Falls back to internal defaults.

Severity: **Medium** — degraded routing, workaround available (direct IPC).

**GAP-MATRIX-03: Songbird TLS cipher suite compatibility**

Some HTTPS targets fail TLS handshake (httpbin.org) while others succeed (ifconfig.me). This suggests Songbird's TLS 1.3 implementation doesn't support all cipher suites the targets require.

Impact: Some HTTPS endpoints unreachable through Tower Atomic.

Severity: **Low** — most targets work. Songbird team can expand cipher suite support.

**GAP-MATRIX-04: NestGate CLI instability**

NestGate's `--help` historically segfaults. The `daemon --socket-only --dev` mode is inferred from binary strings, not documented. NestGate uses HTTP REST, not JSON-RPC over UDS — different IPC pattern from other primals.

Impact: NestGate integration requires HTTP bridge or NestGate evolution to JSON-RPC.

Severity: **Medium** — workaround via HTTP port, but breaks the uniform JSON-RPC IPC model.

**GAP-MATRIX-05: Provenance trio + Squirrel + ToadStool not tested live**

rhizoCrypt, loamSpine, sweetGrass, Squirrel, and ToadStool were not running during validation. Their individual L0 routing and L1 composition patterns remain structural-only (validated by primalSpring Rust code, not by live IPC).

Impact: L0 domains 3-8 are structural-only PASS, not live PASS.

Severity: **Medium** — requires starting each primal, which needs plasmidBin binaries and correct CLI flags.

**GAP-MATRIX-06: plasmidBin binary freshness**

`primalspring_primal` was from March 27 (pre-Phase 25). Updated to April 7 during this session. Other binaries: BearDog (Mar 27), Songbird (Mar 27 plasmidBin, Apr 7 from-source), NestGate (Mar 28), Squirrel (Mar 27), ToadStool (Mar 27). The provenance trio was updated Apr 7 (rhizoCrypt, loamSpine, sweetGrass).

Impact: Some binaries may not reflect latest primal evolution.

Severity: **Low** — rebuild from source when needed. Manifest tracks versions.
