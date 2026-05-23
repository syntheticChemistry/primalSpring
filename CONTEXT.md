# SPDX-License-Identifier: AGPL-3.0-or-later

# primalSpring — Context

## What

primalSpring is the coordination and composition validation spring for
the ecoPrimals ecosystem. Its domain IS the ecosystem itself: atomic
composition (Tower, Node, Nest, Full NUCLEUS), graph execution patterns,
emergent systems, multi-node bonding, and cross-spring interaction.

## Role

primalSpring sits upstream of all springs and gardens but downstream of
primals. Where other springs validate domain science (hotSpring → physics,
wetSpring → biology), primalSpring validates the coordination layer
that biomeOS and the Neural API produce when primals work together.
It has self-knowledge of coordination patterns and discovers other
primals at runtime via capability-based discovery.

Downstream tributaries (springs, gardens) consume our patterns from
`wateringHole/`. As they validate, they expose new gaps that flow
back upstream to primals and primalSpring.

## Architecture

- **ecoPrimal/** — library crate (`primalspring`) + 2 binaries:
  `primalspring_unibin` (eukaryotic UniBin: certify + validate + serve + status + version),
  `primalspring_primal` (JSON-RPC IPC server / cell membrane)
- **experiments/** — 89 validation binaries covering 20 tracks
- **graphs/** — 80 deploy graph TOMLs + 14 atomic signal graphs using fragment-first
  composition (14 root + 9 profiles + 6 fragments + 9 spring validation + 5 multi-node +
  5 bonding + 4 patterns + 4 desktop + 3 downstream + 2 spring deploy + 2 chaos +
  2 cross-spring + 1 federation + 1 composition + 12 cell graphs + `signals/` tier)
- **docs/** — structured gap registry (`PRIMAL_GAPS.md`), wire contracts (discovery, storage, crypto), migration guides
- **tools/** — desktop NUCLEUS launcher, nucleus launcher, composition library + template, TTT reference implementation, Godot bridge, thin WS gateway, composition validator
- **config/** — capability registry, launch profiles
- **niches/** — BYOB niche YAML for biomeOS scheduling
- **specs/** — architecture and evolution specs

## Key Modules

| Module | Purpose |
|--------|---------|
| `coordination` | Atomic composition definitions, health probing |
| `deploy` | Deploy graph parsing, structural + live validation |
| `ipc` | JSON-RPC 2.0 client, Neural API bridge, socket discovery, BTSP handshake |
| `launcher` | Binary discovery, process spawn, socket nucleation |
| `harness` | Spawn compositions, validate, RAII teardown |
| `bonding` | Multi-gate bonding models + STUN tiers + ionic RPC + content distribution |
| `btsp` | BTSP Phase 1–3: handshake, cipher negotiation, encrypted channels |
| `validation` | Experiment harness with structured output (`ValidationResult`, `ValidationSink`) |
| `validation/helpers` | Shared graph parsing, Dark Forest, capability cross-ref helpers |
| `validation/scenarios` | 49 absorbed experiment scenarios (10 tracks, 3 tiers: Rust/Live/Both) |
| `composition/neural_routing` | NeuralRoutingTable — static model of 458-method routing surface |
| `composition/neural_dispatch` | NeuralDispatcher — dispatch surface with metrics + bridge outcome ingestion |
| `tolerances` | Named latency and throughput bounds |
| `niche` | Capability table, semantic mappings, registration |

## Boundaries

- **No barraCuda dependency** — coordination, not compute
- **No WGSL shaders** — GPU work stays in domain springs
- **No cross-spring imports** — discovers primals via IPC at runtime
- **Pure Rust** — zero C dependencies (ecoBin compliant)

## IPC

JSON-RPC 2.0 over Unix domain sockets (TCP fallback) and HTTP POST.
`tcp_rpc_multi_protocol` tries raw TCP then HTTP for transport-agnostic probing.
Method constants across 20+ primal domains. MCP tool surface for Squirrel AI.
Capability-based discovery via Neural API or 6-tier filesystem probing.

## Status

v0.9.26 Wave 45 (May 23, 2026) — 49 scenarios (10 tracks, 3 tiers),
458 registered capability methods (322 exercised, 70% coverage),
89 experiments (20 tracks), 94 deploy graphs (80 deploy + 14 signal),
44-cell deployment matrix. 784 lib tests (all passing). 12/12 primal.announce
compliant. All upstream Neural API blockers resolved.

**Wave 42: Full Neural API Deployment** — NeuralBridge feedback loop
(`capability_call_instrumented`, `record_bridge_outcome`, `dispatch_instrumented`).
biomeOS v3.70: persistent routing weights (redb-backed `RoutingWeightTable`), weight health introspection,
capability utilization tracking (`neural_api.utilization`). Team restructuring
documented (`TEAM_OWNERSHIP_MATRIX.md`). Sovereignty infrastructure status
(`SOVEREIGNTY_INFRASTRUCTURE_STATUS.md`). Neural API deployment guide
for all 13 primal teams (fossilized to archive). biomeOS v3.70: weight health
introspection, attestation verification via BearDog, persistent weights on startup.
Live validation scenarios S47-S49 (neural dispatch, observatory parity, feedback loop).
1311 biomeOS tests.

**Wave 41: Observatory Posture** — biomeOS v3.68 composition intelligence
abstracted from primalSpring's exploratory work. `CompositionTier`, `CompositionPatternRegistry`,
`plan_tier()` now live in biomeOS runtime. NeuralBridge observatory methods
let primalSpring consume biomeOS's adaptive routing intelligence. 1303 biomeOS tests.

**Wave 40: Neural Routing Layer** — `NeuralRoutingTable` (static model of the
full method surface), `NeuralDispatcher` (dispatch + metrics), S46 scenario
(17 structural checks). biomeOS v3.67 adaptive routing weights (`RoutingWeightTable`,
EWMA latency/error, circuit breaker, `primal.announce` cost hints).

**Wave 39: Upstream Absorption** — bearDog Wave 109, songbird capability.call
TURN relay, biomeOS v3.66 cross-gate, toadStool S269, nestgate SP-4 compat,
healthSpring V64z, neuralSpring V170. Neural API Evolution Spec published.

**Waves 1-45 complete**. Zero DEBT markers, zero unsafe blocks, zero panics, zero warnings.
Security gate: MethodGate 13/13, BTSP AEAD 13/13, Edition 2024 13/13.
`Vec<&String>` → `Vec<&str>`. `JsonRpcError`/`UnknownPrimal` → `thiserror` derives.
`DeployError::Parse` now wraps `toml::de::Error` source for error chains. Deprecated
`family_seed_from_env()` → `mito_beacon_from_env().key_bytes()`. Hardcoded arch
triple → `current_target_triple()` compile-time dispatch.
**Zero-port Tower Atomic standard**: Tier 5 TCP discovery gated behind
`PRIMALSPRING_TCP_TIER5=1` (opt-in). UDS-only is the default. NestGate/Squirrel
port-swap bug fixed in scripts and `ports.env`.
See `fossilRecord/springs/primalSpring/docs_wave35_may2026/TEMPORAL_ECOSYSTEM_REVIEW_MAY12_2026.md` for the archived ecosystem audit.
**plasmidBin decoupled** — all direct filesystem coupling to `../plasmidBin` removed (20 files).
Binary discovery standardized: `$ECOPRIMALS_PLASMID_BIN` → `$XDG_DATA_HOME/ecoPrimals/plasmidBin`.
`tools/fetch_primals.sh` bootstraps binaries from GitHub Releases. plasmidBin CI/CD
auto-harvests on primal push via `repository_dispatch`. GAP-27 (stale biomeOS) resolved.
**genomeBin v5.1** — 46 cross-architecture binaries across 6 target triples (Tier 1: 39/39),
`build_ecosystem_genomeBin.sh` replaces musl-only script with full 9-target matrix.

Live validation: **13/13 primals ALIVE** (plasmidBin v2026.05.03 on eastGate),
**guidestone Level 8** (BTSP alias routing + flex key fixes shipped Phase 58; 13 failures resolved),
**13/13 BTSP authenticated**, 8 cellular graphs BTSP-enforced, bonding model ALL PASS,
**19/19 exp094 composition parity**, **12/12 exp091 routing PASS**, **14/15 exp096 cross-arch**
(HSM cfg-gated). ludoSpring parity: exp068 **6/6**, exp067 **18/19**, exp072 **24/31**.
Full NUCLEUS validated across all 3 atomics (Tower + Node + Nest) + cross-atomic pipeline.
benchScale Docker lab: 13 binaries deployed and version-verified.
biomeOS substrate: Neural API liveness and graph executor validated via guidestone Layer 1.5.
**Neural API evolution** (biomeOS v3.55–v3.57): `signal.dispatch` as preferred atomic
signal dispatch path (composition collapse), `capability.call` signal-tier interception,
`primal.announce` atomic self-registration protocol, metrics tagging with signal
namespaces, Squirrel `signal_plan` mode for intent-to-signal decomposition. Tier 2
validation dynamically checks `signal.list` counts and `signal.schema` tool definitions.
See `wateringHole/PRIMAL_ANNOUNCE_PROTOCOL.md`.

Multi-tier genetics identity system: Mitochondrial (Mito-Beacon for discovery
and NAT negotiation), Nuclear (lineage DNA for non-fungible permissions with
generational mixing), Tags (open participation from plaintext seed heritage).
Three-tier BTSP: Phase 1 (FAMILY_SEED auth), Phase 2 (secure-by-default
cascade across 13/13 primals), Phase 3 (ChaCha20-Poly1305 encrypted channel —
13/13 FULL AEAD, converged May 2, 2026).
BtspEnforcer with explicit deny semantics per TrustModel.

Cross-architecture deployment: plasmidBin serves as genomeBin depot per ecoBin
Architecture Standard v3.0. Tier 1 MUST: x86_64 + aarch64 + armv7 musl-static.
Tier 2 SHOULD: Windows (barraCuda), Android (5 primals), macOS (8/14 check-pass).
Tier 3 NICE: RISC-V (all cargo-check pass, primalSpring itself linked).
14/15 cross-arch checks pass (beardog HSM cfg-gated in upstream Session 43).

Particle model adopted: Tower = electron, Node = proton, Nest = neutron,
NUCLEUS = atom. Layered validation: L0 (primal routing) → L1 (atomic) →
L2 (mixed atomics) → L3 (bonding patterns).

guideStone composition certification: `primalspring certify` (UniBin subcommand,
formerly standalone `primalspring_guidestone` — removed Wave 32) validates
NUCLEUS composition correctness across 9 layers (bare properties, seed provenance,
discovery, BTSP escalation, atomic health, capability parity, cross-atomic pipeline,
bonding, BTSP/crypto, cellular deployment). Layer 1.5 reports per-atomic security
posture (BTSP default on all tiers — cleartext is FAIL). biomeOS substrate
health (neural-api liveness + graph.list) validated as first-class check.
Domain guideStones (hotSpring, healthSpring, etc.) inherit this base certification
and only validate their own science. See [fossilRecord](https://github.com/ecoPrimals/fossilRecord) → `springs/primalSpring/wateringHole_phase56_apr2026/GUIDESTONE_COMPOSITION_STANDARD.md`.

BTSP convergence achieved: 13/13 capabilities BTSP-authenticated across all NUCLEUS
tiers. `upgrade_btsp_clients()` uses a two-pass strategy — cleartext probe first,
then BTSP-first for enforcing primals that reject cleartext. Published seed fingerprints
prove binary authenticity at Layer 0.5. All upstream primals now implement the 4-step
handshake server protocol. Key convergence fixes: Songbird `SecurityRpcClient::new_direct()`
(Wave 169), ToadStool post-handshake connection persistence, loamSpine `btsp.negotiate`
non-fatal fallback, petalTongue BearDog field alignment. `nucleus_launcher.sh` starts
biomeOS with `BIOMEOS_BTSP_ENFORCE=0` (cleartext bootstrap before Tower is alive).

Bonding models validated (structural): Covalent, Ionic, Metallic, Weak,
OrganoMetalSalt. Content distribution federation graph with 4 bonding tiers.
Ionic bond protocol RPC wiring for cross-family capability sharing.

## Shell Composition Library

`tools/nucleus_composition_lib.sh` — 41 reusable bash functions for interactive
NUCLEUS composition via IPC. Covers capability discovery, JSON-RPC transport,
petalTongue motor/scene/interaction/proprioception, rhizoCrypt DAG, loamSpine
ledger, sweetGrass braids, discrete sensor event isolation (click vs hover vs
keypress), and startup/teardown lifecycle. Springs source this library and
implement domain hooks. `tools/composition_template.sh` is the minimal starter,
`tools/ttt_composition.sh` is the reference implementation with branching game
states, and `tools/composition_nucleus.sh` is the parameterized NUCLEUS launcher.

## Development Systems

| Gate | Role | Springs/Primals |
|------|------|-----------------|
| **eastGate** | Primary dev — primalSpring, plasmidBin, upstream coordination | Most complex subprojects, BTSP convergence, assists all remote teams |
| **ironGate** | Sister dev — clean deployment validation, composition testing | primalSpring (shared), ludoSpring, groundSpring |

primalSpring development is shared between eastGate and ironGate. ironGate
provides clean deployment validation — fresh NUCLEUS bootstraps via
`fetch_primals.sh` without pre-existing state. eastGate handles the
bottleneck work: BTSP convergence, plasmidBin CI/CD, upstream primal
coordination, and ecosystem-wide Phase 3 rollout.

See `infra/whitePaper/gen3/about/HARDWARE.md` for full cluster topology
(11 towers, 4 HBM2 cards, 3 NPUs, ~1 TB aggregate RAM).

## Ecosystem Position

primalSpring validates biomeOS composition patterns so that other
springs and gen4 products can trust the coordination layer. It
contributes ValidationSink, deploy graphs, overlays, MCP tools,
bonding metadata, STUN tier definitions, pure-primal proto-nucleate
graphs, and the shell composition library back to the ecosystem.
Downstream tributaries reference `wateringHole/` for patterns and
standards. Per-spring exploration lanes guide convergent evolution:
ludoSpring (interaction fidelity), hotSpring (async compute/DAG
memoization), wetSpring (data visualization), neuralSpring (agentic
composition).

## Glacial Checkpoint — Current and Remaining (May 20, 2026)

### Completed
- **Wave 22 stadial gate absorbed**: 13/13 primals evolved, plasmidBin v5.5.0
- **Fossilization pass**: 45+ handoffs archived, infra handoffs 100% archived, primalSpring 3 living (1 catalogue + 2 pattern standards)
- **Waves 23–37 complete**: E2E study, shadow runs, upstream ingestion, pattern dissemination, showcase fossilization, ionic bond runtime (WS-1), sovereign publish pipeline (SP-4), ecosystem debt sweep
- All 8 springs at Wave 20 — canonical capability.list envelope, debt resolved
- 458-method registry (real methods, post-Wave 32 recount excluding test fixtures), zero drift
- 49 validation scenarios (10 tracks), 80 deploy graphs, 14 signal graphs
- 10/10 foundation threads active (Thread 4 now active)
- lithoSpore v1.0.0 released — 7/7 modules Tier 2 PASS (75/75 checks)
- All deprecated `probe_primal` callers removed from primalSpring
- CATHEDRAL split documented, garden evolution blurbs published
- Zero local debt across all springs (deep audit confirmed)
- 13/13 primals in plasmidBin, all BTSP AEAD authenticated
- All upstream blockers SHIPPED (UB-1 through UB-4)
- **Wave 20**: `primal.list` + `capability.list` canonical schemas defined and validated
- **Wave 20**: `nest.commit` live probe scenario (skip-tolerant for pre-v3.57 biomeOS)
- **Wave 20**: Thread 10 workload wired (`--provenance-dir` flag, `thread10_provenance.sh`)
- **Wave 20**: Primal-blocked gaps documented as upstream asks
- **Wave 20**: LTEE paper queue tracker — 8 papers, 4 springs, 7 lithoSpore modules
- **Wave 20 PM**: lithoSpore audit absorption — R1 (degradation behavior documented in CompositionContext), R2 (stability tiers in capability_registry.toml), R3 (trio transaction semantics in PROVENANCE_TRIO_INTEGRATION_GUIDE), R4 (UDS socket ownership in CAPABILITY_BASED_DISCOVERY_STANDARD)
- **Wave 20 PM**: Cross-tier parity pattern absorbed into VALIDATION_TIERS.md (Tier 3 + parity sections)
- **Wave 20 PM**: Ferment transcript pattern cross-referenced in DOWNSTREAM_PATTERN_GUIDE.md
- **Wave 20 PM (post-absorption)**: All 7 delta springs absorbed lithoSpore audit blurb — stability tiers annotated, degradation behavior documented, trio transaction semantics aligned, cross-tier parity pattern adopted
- **Wave 20 PM**: wetSpring V177 Exp381 breseq pipeline executing on Barrick 2009 (3/7 clones done, first ferment transcript braid exported, mutation accumulation trend confirmed)
- **Wave 20 PM**: airSpring v0.10.0 — 3 new cross-tier parity validators (autocorrelation, gamma_cdf, soil_moisture_topp), 57 capabilities (53 stable, 4 evolving), trio bug fix (`primals_reached`)
- **Wave 20 PM**: hotSpring — 6 new experiments (199-204) including VBIOS interpreter live HW validation, oracle data cleaned, handoff archival
- **Wave 20 PM**: ludoSpring V76 — Schell Lenses + CPU/GPU parity, 982 tests, cross-tier parity doc
- **Wave 20 PM**: groundSpring V145 — degradation behavior doc, niche metadata fix
- **Wave 21**: All downstream gardens absorbed Wave 20 patterns:
  - lithoSpore: `PARITY_REPORT_SCHEMA.md` (ecosystem standard), `DEGRADATION_BEHAVIOR.md`, `provenance/braids/` for ferment ingestion, stability tiers, partial trio semantics
  - projectFOUNDATION: `DEGRADATION_BEHAVIOR.md`, `validation/wetSpring/braids/` for Thread 5, BLAKE3 backfill documented, composition gaps marked resolved, stability tiers on workloads
  - esotericWebb V9: canonical capability.list envelope, stability tiers, degradation doc, `primals_reached` on WorldState, GAP-026–030 resolved
  - projectNUCLEUS: path reorganization, local hardcoding eliminated, cellMembrane owns fieldMouse Tower

### Garden Evolution (May 17, 2026)
- **lithoSpore** (latest): Tier 3 provenance trio wired via JSON-RPC (dag/spine/braid),
  cross-tier parity (`litho parity` — 7/7 modules MATCH), two-tier data model formalized,
  ferment transcript upstream braid handoff, 117 tests, 15 CLI subcommands, zero clippy,
  `#![forbid(unsafe_code)]` workspace-wide, `--provenance-dir` for Thread 10
- **lithoSpore v1.0.0**: ScopeManifest, liveSpore.json provenance journal,
  capability-first discovery, scope-driven validation, 6 THREAD_INDEX entries,
  sporePrint dispatch CI, CLI integration test harness
- **projectNUCLEUS V3**: 55 Rust tests (darkforest 34, tunnelKeeper 21),
  discovery cascade (primal.list → env → defaults), 7 gate TOMLs with
  [science] dispatch metadata, signal_executor.sh (Squirrel → signal.dispatch),
  tower_agent.toml (agentic graph), VALIDATION_PLAYBOOK + FUZZ_EVOLUTION +
  SCIENCE_DISPATCH_MAP + TIER2_CEREMONY_DESIGN specs, FAMILY_HPC_MODEL
- **esotericWebb V8**: 357 tests, 24 capabilities, signal-first provenance
  (nest_store/nest_commit bridge), startup primal.announce, lifecycle handlers
  (health.version, health.drain, primal.info), test extraction pattern
  (#[path = "tests.rs"]), capability registry ↔ niche cross-validation test
- **projectFOUNDATION**: 184 targets (146 validated), 29 workloads across
  all 10 threads, per-spring validation folders + provenance convention,
  primal_ipc.sh/target_compare.sh libs, 6 barraCuda CPU parity benchmarks,
  CI with graph/schema/workload/benchmark gates, ag-guidestone proposal,
  FOUNDATION_VALIDATE elevation review targeting CompositionContext

### Infrastructure Review (May 17, 2026)
- Stale `gardens/sporeGarden/` clone removed (duplicate of projectNUCLEUS)
- `.env` audit complete — all sensitive files gitignored, zero contamination risk
- `REPO_MEMBRANE_BOUNDARY.md` published — inner/outer repo classification standard
- Validation Gate Matrix in `EVOLUTION_GAPS.md` — 11 validation systems mapped to sovereignty transitions
- cellMembrane: recommend Forgejo-only when operationally stable

### wateringHole Audit and Fossilization (May 17, 2026)
- Fossilized 6 primalSpring handoffs + 2 infra wateringHole docs (superseded by Wave 20/21)
- Fixed numeric drift: 427/418→452 methods, 12→13 primals across 10 infra docs
- Reconciled Forgejo/GitHub posture in SOVEREIGNTY_STANDARDS (operational reality vs aspirational)
- GLOSSARY updated: cellMembrane entry, dual Git host framing, sporeGarden products roster
- INFRASTRUCTURE_RECREATION_AND_OUTAGE_PLAYBOOK.md created in gen4/architecture
- gen4 architecture indexes updated (18 docs organized into 3 clusters)
- Glacial status evolved: ECOSYSTEM_EVOLUTION_CYCLE v1.8.0, INTERSTADIAL_EXIT_CRITERIA
  expanded with outage simulation readiness and infrastructure sovereignty posture tables

### Wave 22: Upstream Primal Evolution (May 17-18, 2026)
- Removed duplicate `primals/beardog/` and `primals/nestGate/` stub
- Audited all 13 primals + sourDough + bingoCube for deployment-valid standard compliance
- Created upstream evolution blurb with per-primal action items and stadial pairing preview
- Registered 4 new methods: `dag.partial_dehydrate`, `braid.partial_update`,
  `braid.complete`, `compute.fan_out` — from downstream-validated upstream asks
- **Stadial gate push absorbed (May 18)**: all primals responded to blurb
  - All 3 wetSpring upstream asks IMPLEMENTED by primal teams
  - 5/7 composition gaps RESOLVED (toadStool 3, sweetGrass TCP/BTSP, hex acceptance)
  - Major bumps: toadStool 0.2.0, coralReef 0.2.0, skunkBat 0.2.0, sweetGrass 0.7.36,
    biomeOS v3.60, barraCuda Sprint 70 (75 methods)
  - All primals now: stability tiers, primal.announce, btsp.capabilities, canonical envelope
  - bearDog Wave 105: ring/rustls policy reconciled, ACME design doc authored
  - songbird Wave 207-208: btsp.capabilities, primal.announce, aws-lc-sys banned, test splits
  - biomeOS v3.61: composition.status pipelines, enrichment module, NucleusMode::Full,
    braid signals fully wired, spore.instantiate route
  - plasmidBin manifest v5.5.0: toadStool 0.2.0, coralReef 0.2.0 (A++), sweetGrass 0.7.36
  - **All 13/13 primals stadial-gate absorbed**
  - Remaining: sourDough version drift, 2 composition gaps (GPU API alignment, cross-gate dispatch)

### Remaining (updated May 23, 2026)

**Resolved since last review:**
- ~~**Thread 4**~~ now active (expression + data sources in projectFOUNDATION)
- ~~**biomeOS primal.list implementation**~~ **RESOLVED** — shipped in biomeOS v3.65 (Wave 33)
- ~~**biomeOS nest.store signal dispatch**~~ **RESOLVED** — R5 promoted in biomeOS v3.63
- ~~**biomeOS spore.instantiate**~~ **DEFERRED-TO-STADIAL** — R7 in biomeOS v3.63
- ~~**Primal-blocked gaps**~~ (toadStool sandbox, barraCuda/coralReef, ionic bridge, sweetGrass TCP) — **RESOLVED** Wave 22 stadial gate + Wave 44 fixes

**Neural API (upstream — ALL RESOLVED):**
- ~~**songbird outbound announce**~~ — **RESOLVED** commit `4a8f4cdc`: outbound push + capability alignment shipped
- ~~**bearDog attestation field rename**~~ — **RESOLVED** commit `2a94f2d6d` (Wave 111): field renamed, verified by biomeOS v3.70

**Sovereignty infrastructure (cellMembrane team):**
- **Sovereign DNS** (knot-dns, H2-17 through H2-20) — cellMembrane owns
- **Forgejo Actions CI** porting for projectNUCLEUS sovereignty
- **Forgejo Releases** as sovereign binary channel (S5)
- **sporePrint living content** evolution (S6)

**Downstream / cross-spring (async):**
- **ludoSpring 6-method IPC** expansion for esotericWebb
- **esotericWebb provenance E2E** on biomeOS (GAP-024)
- **lithoSpore TURN wiring** (songbird-turn-client integration pending)
- **petalTongue dialogue scenes** for esotericWebb narrative UI
- **Foundation validate elevation** to CompositionContext + Rust crates
- **Thread 1 WCM RPC** stack (0/24 blocked)

**Data / science (springs own):**
- **BLAKE3 backfill** — projectFOUNDATION has 165 empty `blake3` fields across 11 source TOMLs
- **LTEE enrichments** (B5 lithoSpore promotion, B7 Tier 3, B1-ML Rust elevation)
- **wetSpring ferment transcripts** — Barrick 2009 SEALED (7/7), Tenaillon 2016 batch 0 COMPLETE (5/5 clones)
- **plasmidBin convergence** — all gardens converging on plasmidBin as central deployment depot
