# primalSpring — Showcase Mining Report

**Date**: March 2, 2026 (mined), March 17, 2026 (updated), March 23, 2026 (expanded)  
**Source**: primal showcase directories (historically `phase1/`/`phase2/`, now `primals/`), multi-node bonding patterns  
**Purpose**: Extract early coordination patterns for primalSpring system tests

---

## Patterns Mined from phase1/ Showcases

### 1. ToadStool: Compute Triangle (`phase1/toadstool/showcase/02-compute-patterns/`)

**Pattern**: coralReef (compile) -> toadStool (orchestrate) -> barraCuda (execute)

**What to test in primalSpring**:
- Socket discovery at `$XDG_RUNTIME_DIR/biomeos/` for all primals
- `discovery.primals` and `discovery.topology` JSON-RPC calls
- Capability-based routing to the correct primal for each step
- Live compute triangle pipeline with a real WGSL shader

**Source files**:
- `01-capability-discovery/src/main.rs` — socket enumeration, topology query
- `03-deploy-graph/src/main.rs` — `deploy.capability_call`, `deploy.graph_status`
- `04-shader-to-gpu/src/main.rs` — full coralReef -> toadStool -> barraCuda pipeline

### 2. ToadStool: Ecosystem Integration (`phase1/toadstool/showcase/03-ecosystem-integration/`)

**Pattern**: Songbird registration, BearDog secured compute, NestGate artifact storage

**What to test**:
- `coordination.register` with capability advertisement
- Bearer token auth flow: `security.authenticate` -> `security.validate_token`
- `storage.artifact.store` / `storage.artifact.retrieve` round-trip

### 3. NestGate: Multi-Primal Startup (`phase1/nestgate/showcase/scripts/start_ecosystem.sh`)

**Pattern**: Start NestGate, Songbird, ToadStool with health checks, port management, log rotation

**What to test**:
- `wait_for_health()` pattern — repeated health probe with timeout
- Port conflict detection before startup
- Service dependency ordering (NestGate before Songbird before ToadStool)
- Graceful shutdown with PID tracking

### 4. NestGate: Songbird Coordination (`phase1/nestgate/showcase/04_inter_primal_mesh/`)

**Pattern**: Protocol negotiation (HTTP -> JSON-RPC -> tarpc), orchestrated workflows

**What to test**:
- Protocol escalation: HTTP REST -> JSON-RPC 2.0 -> tarpc binary
- Pool discovery and dataset creation via Songbird
- Three-primal workflow: Songbird + NestGate + ToadStool

### 5. BearDog: Multi-Primal Workflow (`phase1/beardog/showcase/04-advanced-features/01-multi-primal-workflow/`)

**Pattern**: 6-phase capability-based orchestration: discovery -> security -> AI -> storage -> compute -> orchestration -> lineage/audit

**What to test**:
- Capability-based endpoint discovery via `PRIMAL_{CAPABILITY}_ENDPOINT` env vars
- Multi-step workflow with handoff between 5+ services
- Lineage and audit trail after workflow completion

### 6. BearDog: Cross-Tower Federation (`phase1/beardog/showcase/04-advanced-features/09-cross-tower-federation/`)

**Pattern**: BYOB manifest for multi-tower deployment with dependency graph, health checks, coordination timeouts

**What to test**:
- Federation manifest parsing and dependency resolution
- Cross-tower service discovery
- Timeout handling and health check aggregation

---

## Patterns Mined from phase2/ Showcases

### 7. SweetGrass: RootPulse Emergence (`phase2/sweetGrass/showcase/ROOTPULSE_EMERGENCE_PLAN.md`)

**Pattern**: 7-step RootPulse workflow: changes -> rhizoCrypt staging -> sweetGrass semantic analysis -> braid creation -> attribution calculation -> dehydration -> LoamSpine commit

**What to test** (enhances existing exp020-022):
- Semantic tracking at module/feature/function levels
- Author/dependency/temporal braid formation
- Attribution proof generation and tamper detection
- Multi-agent concurrent contribution with fair attribution

### 8. SweetGrass: Integration Gaps (`phase2/sweetGrass/showcase/INTEGRATION_GAPS_REPORT.md`)

**Pattern**: 7-primal integration matrix with honest gap assessment

**What to test**:
- sweetGrass -> Songbird (discovery) — verified working
- sweetGrass -> NestGate (storage) — verified working
- sweetGrass -> BearDog (signing) — needs live verification
- sweetGrass -> ToadStool (compute provenance) — verified working
- sweetGrass -> Squirrel (AI attribution) — verified working
- sweetGrass -> LoamSpine (anchoring) — gap resolved (now production)
- sweetGrass -> rhizoCrypt (sessions) — gap resolved (now production)

### 9. biomeOS: Bonding Model Tests (`phase2/biomeOS/graphs/BONDING_TESTS_README.md`)

**Pattern**: 5 bonding test graphs with USB deployment, socket per family, validation criteria

**What to test** (enhances existing exp030-034):
- Covalent bonding: Family Alpha (5 primals, shared seed, BirdSong mesh)
- Covalent bonding: Family Beta (independent 5 primals)
- Ionic interaction: Alpha -> Beta cross-family storage request
- Weak forces: Zero information leakage with unknown primals
- Organo-metal-salt: Multi-modal bonding (covalent + ionic + metallic simultaneous)

### 10. biomeOS: Provenance Trio E2E (`phase2/biomeOS/scripts/test_provenance_trio_e2e.sh`)

**Pattern**: Full E2E test runner: start Tower -> check NestGate -> deploy Provenance Trio via graph -> run cargo tests -> cleanup

**What to test** (enhances existing exp020):
- Socket-based readiness polling with timeout
- Graph-based trio deployment via Neural API JSON-RPC
- Family-scoped socket naming (`{primal}-{family_id}.sock`)
- Cleanup of family-scoped sockets after test

### 11. petalTongue: Ecosystem Visualization (`phase2/petalTongue/showcase/03-inter-primal/`)

**Pattern**: biomeOS topology visualization, ecosystem health dashboard, multi-primal TUI

**What to test** (enhances existing exp043):
- `/api/v1/topology` endpoint from Neural API
- Health aggregation across all primals (healthy/degraded/unhealthy)
- Atomic deployment visualization (primals, edges, families, trust levels)

### 12. LoamSpine: Full Ecosystem Demo (`phase2/loamSpine/showcase/04-inter-primal/05-full-ecosystem/`)

**Pattern**: Research paper lifecycle across 6 primals: Songbird -> Squirrel -> NestGate -> LoamSpine -> BearDog -> ToadStool

**What to test**:
- Full 6-primal lifecycle workflow
- Content storage + metadata ledger + signing + compute in sequence
- No hardcoded endpoints — pure capability composition

### 13. rhizoCrypt: Complete Workflows (`phase2/rhizoCrypt/showcase/01-inter-primal-live/05-complete-workflows/`)

**Pattern**: Supply chain provenance (7-stage farm-to-table), document workflow, federated identity

**What to test**:
- Multi-agent DAG with per-agent BearDog signatures
- Document workflow across multiple agents
- Federated identity workflow

---

## New Experiments Recommended

Based on the mined patterns, these 10 experiments (exp050–059) extended primalSpring to 38 total.
Subsequent phases added experiments through exp084 (including cross-gate deployment in Track 10, gen4 deployment evolution in Track 11, deployment matrix in Track 12, and substrate stress in Track 13), bringing the total to **63 experiments**.

| Exp | Name | Source Pattern | What It Tests |
|-----|------|----------------|---------------|
| 050 | Compute triangle | ToadStool #1 | coralReef -> toadStool -> barraCuda live pipeline |
| 051 | Socket discovery sweep | ToadStool #1 | XDG_RUNTIME_DIR/biomeos/ enumeration for all primals |
| 052 | Protocol escalation | NestGate #4 | HTTP -> JSON-RPC -> tarpc negotiation |
| 053 | Multi-primal lifecycle | LoamSpine #12 | 6-primal research paper lifecycle |
| 054 | Bearer token auth flow | ToadStool #2 | BearDog authenticate -> validate -> compute.submit |
| 055 | Wait-for-health pattern | NestGate #3 | Repeated health probe with timeout and ordering |
| 056 | Cross-tower federation | BearDog #6 | BYOB manifest, cross-tower discovery, timeouts |
| 057 | Supply chain provenance | rhizoCrypt #13 | 7-stage DAG with per-agent signing |
| 058 | Semantic attribution | SweetGrass #7 | Module/feature/function level tracking + fair credit |
| 059 | Weak force isolation | biomeOS #9 | Zero leakage with unknown primals, read-only observation |

---

## Patterns from Multi-Node Bonding (Phase 12, March 23, 2026)

### 14. Basement HPC Covalent Mesh (`graphs/multi_node/basement_hpc_covalent.toml`)

**Pattern**: Deploy NUCLEUS on multiple LAN machines sharing a `.family.seed` for auto-trust via BirdSong encrypted beacons.

**What to test**:
- Covalent bonding with GeneticLineage trust model
- BirdSong mesh discovery across LAN
- Capability aggregation into a Plasmodium collective
- Family-scoped socket naming (`{primal}-{FAMILY_ID}.sock`)

### 15. Friend Remote Covalent with NAT Traversal (`graphs/multi_node/friend_remote_covalent.toml`)

**Pattern**: Remote machine joins covalent mesh through 4-tier STUN (Lineage → Self-hosted → Public → Rendezvous), hole-punch, and relay fallback. BondingPolicy enforces capability scope and time windows.

**What to test**:
- STUN sovereignty-first escalation (never skip to public before lineage/self-hosted)
- Hole-punch with relay fallback when direct connection fails
- BondingPolicy: `compute.*` only, `22:00-06:00` active window, 100 Mbps cap
- Dark forest discovery (encrypted beacons, zero metadata leakage)

### 16. Idle Compute Federation (`graphs/multi_node/idle_compute_federation.toml`)

**Pattern**: Orchestrate federated idle compute across bonded peers — discover idle nodes, dispatch workloads by capability, track provenance of remote compute results.

**What to test** (exp071):
- BondingConstraint capability filtering (`permits()` method)
- Time window scheduling for idle compute offers
- Bandwidth and concurrency limits enforcement
- Provenance trio tracking of federated compute results

### 17. Data Federation Cross-Site (`graphs/multi_node/data_federation_cross_site.toml`)

**Pattern**: 7-phase NestGate replication pipeline with full provenance trio tracking — list → replicate → DAG create → event append → attribute → federate → commit.

**What to test** (exp072):
- NestGate cross-site storage replication with conflict resolution
- rhizoCrypt DAG session per replication batch
- sweetGrass attribution of replicated data provenance
- loamSpine commit anchoring the federated state

### 18. BondType Full Taxonomy

**Pattern**: Chemistry-inspired bonding model covering all inter-node trust and interaction levels.

**Established types** (validated in exp030/032/033/071):
- **Covalent**: Shared `.family.seed`, mutual discovery, full in-mesh (basement HPC, friend clusters)
- **Metallic**: Electron-sea delocalized capabilities, specialization (compute-only racks, storage fleets)
- **Ionic**: Contract-based, metered, separate Towers (cloud burst GPU, external APIs)
- **Weak**: Minimal coupling, zero/low trust, read-only (public APIs, unknown beacons)
- **OrganoMetalSalt**: Mixed bond types in a single deployment

### 19. TrustModel + BondingPolicy

**Pattern**: Fine-grained access control for federated resource sharing.

**What to test**:
- GeneticLineage: `.family.seed` auto-trust, no credential exchange
- Contractual: Service agreements, metered billing
- Organizational: Certificate-based enterprise trust
- ZeroTrust: Challenge-response on every request
- BondingPolicy presets: idle_compute, covalent_full, ionic_contract

### 20. STUN Multi-Tier NAT Traversal (`config/stun/multi_tier.toml`)

**Pattern**: Sovereignty-first NAT traversal — never leak metadata to public infrastructure when private options exist.

**What to test** (stun_tiers.rs):
- Tier 1 (Lineage Relay): Family members relay for each other
- Tier 2 (Self-Hosted): User's own STUN infrastructure
- Tier 3 (Public): Community STUN servers, address discovery only
- Tier 4 (Rendezvous): Future gaming platform integration
- Sovereignty violations: parallel attempts, disabled lineage, corporate STUN

| Exp | Name | Source Pattern | What It Tests |
|-----|------|----------------|---------------|
| 071 | Idle compute policy | Pattern #16, #19 | BondingPolicy masks, time windows, bandwidth, graph validation |
| 072 | Data federation | Pattern #17 | NestGate replication + trio provenance, 7-phase pipeline |
