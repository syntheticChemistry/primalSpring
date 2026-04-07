# NUCLEUS Validation Matrix

**Date**: April 7, 2026  
**Phase**: 25+ (planning)  
**Purpose**: Define the validation matrix for NUCLEUS composition patterns across downstream springs and sporeGarden products, based on gen4 architecture (`infra/whitePaper/gen4/architecture/COMPOSITION_PATTERNS.md`) and primalSpring's Phase 25 modernization results.

---

## Context

primalSpring validates the coordination layer. Phase 25 cleaned all legacy patterns and confirmed Tower Atomic HTTPS works end-to-end. The next evolution step is validating that these patterns compose correctly in downstream springs and gen4 products (sporeGarden).

This matrix defines what each spring and product must demonstrate to confirm NUCLEUS readiness.

---

## Meta-Patterns to Nucleate

These are the composition patterns proven in primalSpring that downstream systems must absorb:

| Pattern | primalSpring Reference | What It Proves |
|---------|----------------------|----------------|
| **Tower Atomic** | `nest-deploy.toml` Phase 1-2 | BearDog + Songbird compose; TLS 1.3 works |
| **Nest Atomic** | `nest-deploy.toml` Phase 1-4 | Tower + NestGate + Squirrel; storage + AI |
| **Node Atomic** | `node_atomic_compute.toml` | Tower + ToadStool; GPU compute |
| **Full NUCLEUS** | `nucleus_complete.toml` | Tower + Nest + Node; all capability domains |
| **Graph-Deployed Composition** | `[[graph.nodes]]` format | biomeOS deploys primals via TOML graphs |
| **Capability Routing** | `capability.call` via Neural API | biomeOS routes method calls to correct primal |
| **HTTPS Through Tower** | `validate_https` node | End-to-end TLS via BearDog→Songbird, no external TLS |
| **Covalent Bonding** | `basement_hpc_covalent.toml` | Multi-node with shared `FAMILY_ID`, mesh discovery |
| **Graceful Degradation** | gen4 COMPOSITION_PATTERNS §III | Product runs even if primals absent |
| **health.liveness** | All primals | Universal JSON-RPC health check (no HTTP) |

---

## Validation Matrix

### Columns

| Column | What to Validate | Method |
|--------|-----------------|--------|
| **A: Graph Format** | Uses `[[graph.nodes]]` with `id` field | Structural parse |
| **B: Capability Names** | All methods use canonical dotted names | Registry cross-check |
| **C: health.liveness** | All primals respond to `health.liveness` | JSON-RPC probe |
| **D: HTTPS Validation** | HTTPS through Tower Atomic works | `http.get` via Neural API |
| **E: Nest Atomic** | NestGate storage round-trip | `storage.store` + `storage.retrieve` |
| **F: Node Atomic** | ToadStool compute available | `compute.submit` |
| **G: AI Routing** | Squirrel `ai.query` via Neural API | `capability.call` |
| **H: Covalent Ready** | Multi-node graph, `FAMILY_ID`, mesh | Graph structure + exp073 pattern |
| **I: Graceful Degradation** | Product runs standalone (no primals) | Launch without stack |
| **J: sporeGarden Deploy** | BYOB deploy graph, plasmidBin binaries | `prepare_spore_payload.sh` |

### Rows (Springs)

| Spring | Domain | A | B | C | D | E | F | G | H | I | J |
|--------|--------|---|---|---|---|---|---|---|---|---|---|
| **primalSpring** | Coordination | **PASS** | **PASS** | **LIVE** (Tower) | **LIVE** (ifconfig.me 200) | structural | structural | structural | structural | n/a | structural |
| **wetSpring** | Biology | pending | pending | pending | pending | pending | pending | pending | pending | n/a | pending |
| **hotSpring** | Physics | pending | pending | pending | pending | pending | **likely** | pending | pending | n/a | pending |
| **airSpring** | Agriculture | pending | pending | pending | pending | pending | pending | pending | pending | n/a | pending |
| **groundSpring** | Uncertainty | pending | pending | pending | pending | pending | pending | pending | pending | n/a | pending |
| **neuralSpring** | ML/Neural | pending | pending | pending | pending | pending | pending | pending | pending | n/a | pending |
| **healthSpring** | Health | pending | pending | pending | pending | pending | pending | pending | pending | n/a | pending |
| **ludoSpring** | Game Science | pending | pending | pending | pending | pending | pending | pending | pending | n/a | pending |

### Rows (sporeGarden Products)

| Product | Domain | A | B | C | D | E | F | G | H | I | J |
|---------|--------|---|---|---|---|---|---|---|---|---|---|
| **esotericWebb** | CRPG Engine | pending | pending | pending | pending | pending | pending | pending | pending | **required** | pending |
| **helixVision** | Genomics | planned | planned | planned | planned | planned | planned | planned | planned | **required** | planned |

### Extended Columns (Particle Model & Mixed Composition)

| Column | What to Validate | Method |
|--------|-----------------|--------|
| **K: Particle Profile** | Which particle the spring primarily exercises (proton-heavy, neutron-heavy, balanced) | Architectural analysis per `MIXED_COMPOSITION_PATTERNS.md` §5 |
| **L: Mixed Atomic** | Can the spring deploy L2 patterns (dual tower, dedicated tower, enclave) | Graph sketch structural validation |
| **M: Bonding Patterns** | Which L3 bonding patterns the spring requires | Domain analysis |
| **N: Sharding Ready** | Covalent mesh backup applicable (L3 `covalent_mesh_backup.toml`) | Structural + BondingPolicy validation |
| **O: Enclave Ready** | BondingPolicy data egress fence applicable (L2 `nest_enclave.toml`) | BondingPolicy structural validation |

### Extended Rows (Springs)

| Spring | Domain | K: Particle | L: Mixed Atomic | M: Bonding | N: Sharding | O: Enclave |
|--------|--------|-------------|-----------------|------------|-------------|------------|
| **primalSpring** | Coordination | balanced | structural | all (test arena) | structural | structural |
| **wetSpring** | Biology | balanced | nest enclave | covalent mesh | planned | planned |
| **hotSpring** | Physics | proton-heavy | node+dedicated tower | metallic, ionic lease | n/a | n/a |
| **airSpring** | Agriculture | balanced | nest enclave | covalent mesh | planned | planned |
| **groundSpring** | Uncertainty | proton-heavy | node+dedicated tower | ionic lease | n/a | n/a |
| **neuralSpring** | ML/Neural | balanced | nest enclave | ionic lease | n/a | **required** |
| **healthSpring** | Health | neutron-heavy | dual tower + enclave | covalent mesh | **required** | **required** |
| **ludoSpring** | Game Science | proton-heavy | node+dedicated tower | organo-metal-salt | planned | n/a |

### Extended Rows (sporeGarden Products)

| Product | Domain | K: Particle | L: Mixed Atomic | M: Bonding | N: Sharding | O: Enclave |
|---------|--------|-------------|-----------------|------------|-------------|------------|
| **esotericWebb** | CRPG Engine | proton-heavy | node+dedicated tower | covalent, ionic | planned | n/a |
| **helixVision** | Genomics | neutron-heavy | dual tower + enclave | covalent mesh | **required** | **required** |

---

## Layered Validation Model

The matrix columns map to a four-layer validation model defined in `specs/MIXED_COMPOSITION_PATTERNS.md`:

| Layer | Scope | Matrix Columns | Key Graph |
|-------|-------|---------------|-----------|
| **L0** | biomeOS + any primal | A, B, C | `graphs/sketches/validation/primal_routing_matrix.toml` |
| **L1** | Each atomic (Tower/Node/Nest) | C, D, E, F, G | Existing: `nest-deploy.toml`, `node_atomic_compute.toml` |
| **L2** | Mixed atomics | L, O | `graphs/sketches/mixed_atomics/*.toml` |
| **L3** | Bonding patterns | H, M, N | `graphs/sketches/bonding_patterns/*.toml` |

### Particle Model Reference

The atomic-to-particle mapping (Paper 23, `gen3/baseCamp/23_mass_energy_information_equivalence.md`):

| Atomic | Particle | Property | Fungibility |
|--------|----------|----------|-------------|
| **Tower** | Electron | Trust boundary, mediates all bonding | n/a (medium) |
| **Node** | Proton | Compute = energy | Fungible |
| **Nest** | Neutron | Data = energy at rest | Non-fungible |
| **NUCLEUS** | Atom | Complete composition | — |

Column K (Particle Profile) characterizes each spring's emphasis:
- **Proton-heavy**: compute-dominated (hotSpring, groundSpring, ludoSpring)
- **Neutron-heavy**: data-dominated (healthSpring, helixVision)
- **Balanced**: equal compute + data emphasis (wetSpring, airSpring, neuralSpring, primalSpring)

---

## Validation Approach per Spring

Each spring has a `validate_nucleus_*` binary or equivalent. The matrix cells are validated by:

1. **Structural**: Spring has a biomeOS deploy graph in `graphs/spring_deploy/` (primalSpring has these for all 7). Validate it parses with `[[graph.nodes]]` format.
2. **Live**: Spring's deploy graph is executed on Eastgate with live primals. The spring primal starts, discovers NUCLEUS primals, and performs its domain validation.
3. **Product**: For sporeGarden products, the full composition pipeline runs — PrimalBridge connects to all required primals, graceful degradation works, standalone mode functional.

---

## Priority Order

### Phase A: Graph Format Compliance (columns A + B)

All springs already have nucleated deploy graphs in `primalSpring/graphs/spring_deploy/`. Validate these use canonical `[[graph.nodes]]` format and capability names. This is already done — they were migrated in Phase 25.

Action: Each spring team should verify their local graph files (if any) match `[[graph.nodes]]` format.

### Phase B: Health + HTTPS Validation (columns C + D)

Deploy each spring's NUCLEUS graph and validate:
- All primals respond to `health.liveness`
- HTTPS through Tower Atomic returns a valid response

This requires live primals. primalSpring's `AtomicHarness` can drive this.

### Phase C: Storage + Compute + AI (columns E + F + G)

Validate domain-specific primal interactions:
- NestGate `storage.store`/`storage.retrieve` for experiment data
- ToadStool `compute.submit` for GPU workloads (hotSpring, airSpring, groundSpring)
- Squirrel `ai.query` for AI-assisted analysis

### Phase D: Multi-Node + Covalent (column H)

Validate covalent bonding readiness:
- Each spring's graph can extend to multi-node deployment
- `FAMILY_ID` propagation works across gates
- BirdSong mesh discovery finds peer spring instances

### Phase E: sporeGarden Deployment (columns I + J)

For products:
- Graceful degradation validated (standalone mode works)
- BYOB deploy graph defines full primal topology
- `prepare_spore_payload.sh` produces deployable payload

---

## Integration with Existing Infrastructure

| Component | Role in Matrix |
|-----------|---------------|
| `primalSpring/graphs/spring_deploy/*.toml` | Nucleated deploy graphs for all 7 springs |
| `primalSpring/config/deployment_matrix.toml` | 43-cell deployment matrix (arch × topology × preset × transport) |
| `primalSpring/scripts/validate_deployment_matrix.sh` | Matrix runner |
| `primalSpring/scripts/validate_remote_gate.sh` | Remote gate NUCLEUS health probe |
| `primalSpring/scripts/prepare_spore_payload.sh` | USB spore payload assembly |
| `primalSpring/ecoPrimal/src/harness/` | `AtomicHarness` for live composition |
| `primalSpring/ecoPrimal/src/bonding/` | `BondType`, `BondingPolicy`, `BondingConstraint` |
| `primalSpring/experiments/exp090_tower_atomic_lan_probe/` | LAN discovery validation |
| `primalSpring/experiments/exp091_primal_routing_matrix/` | L0 routing matrix validation |
| `primalSpring/experiments/exp092_dual_tower_ionic/` | L2 dual tower + ionic validation |
| `primalSpring/experiments/exp093_covalent_mesh_backup/` | L3 covalent mesh backup validation |
| `primalSpring/specs/MIXED_COMPOSITION_PATTERNS.md` | Particle model, layered validation, gap inventory |
| `primalSpring/graphs/sketches/mixed_atomics/` | L2 mixed atomic graph sketches |
| `primalSpring/graphs/sketches/bonding_patterns/` | L3 bonding pattern graph sketches |
| `infra/whitePaper/gen4/architecture/COMPOSITION_PATTERNS.md` | gen4 patterns (PrimalBridge, graceful degradation, deploy graphs) |
| `infra/whitePaper/gen3/baseCamp/23_mass_energy_information_equivalence.md` | Paper 23: particle model theoretical foundation |

---

## Relationship to gen4

The gen4 vision (`COMPOSITION_PATTERNS.md`) introduces two patterns that extend the NUCLEUS matrix:

1. **Dual Surface** (Creator + Developer): The Creator surface (YAML/CLI) requires graceful degradation (column I). The Developer surface (Rust/PrimalBridge) requires all columns A-H.

2. **PrimalBridge**: Each gen4 product has a bridge that connects to 8+ primal domains. The matrix validates that these domains are reachable through NUCLEUS composition.

The NUCLEUS validation matrix is the gen3→gen4 bridge checkpoint: when all springs pass columns A-H, products can trust the composition layer.

---

## Live Validation Results (April 7, 2026)

### Run 1: biomeOS v2.81 (pre-fix)

Tower Atomic (BearDog + Songbird) validated live on Eastgate. Neural API running but capability registration gap identified — 0 capabilities discovered from running primals.

### Run 2: biomeOS v2.92 (post-fix — probe_endpoint + prefix matching)

biomeOS v2.92 includes real JSON-RPC probing (`identity.get` + `capabilities.list`), 4-format capability parser alignment, and domain prefix matching (GAP-019). Rebuilt from source and validated:

| Probe | Result | Notes |
|-------|--------|-------|
| BearDog health.liveness (direct) | **LIVE PASS** | v0.9.0, 9 capability groups via `capabilities.list` |
| BearDog crypto.sign_ed25519 (direct) | **LIVE PASS** | Ed25519 signature generated |
| BearDog crypto.blake3_hash (direct) | **LIVE PASS** | BLAKE3 hash confirmed |
| Songbird health.liveness (direct) | **LIVE PASS** | Healthy |
| Songbird HTTPS ifconfig.me (direct) | **LIVE PASS** | HTTP 200 via Tower crypto, 300ms |
| Neural API health.liveness | **LIVE PASS** | v0.1.0, alive |
| Neural API → Songbird discovery | **PARTIAL** | 14 Songbird capabilities registered (was 0). Routing finds Songbird. Proxy forwarding fails. |
| Neural API → BearDog capabilities | **FAIL** | BearDog `provided_capabilities` wire format (`[{type, methods}]`) unrecognized — 0 BearDog capabilities registered. **GAP-MATRIX-01b** |
| Neural API capability.call forwarding | **FAIL** | Discovers correct provider, but proxy connection to primal socket fails. **GAP-MATRIX-07** |
| Neural API capability.discover | **LIVE PASS** | Returns correct Songbird endpoint for `network.*` domains |
| NestGate | NOT STARTED | GAP-MATRIX-04: CLI instability |
| ToadStool, Squirrel, Trio | NOT STARTED | GAP-MATRIX-05: require manual process launch |

### What biomeOS v2.92 Fixed

- **Songbird capabilities discovered**: 14 capabilities registered (network.discovery, network.federation, network.relay, network.stun, network.igd, network.quic, network.tls, network.tor, network.onion, ipc.jsonrpc, ipc.tarpc, crypto.delegate, nfc.genesis, bluetooth.pair). Was 0 in v2.81.
- **Prefix routing works**: `capability.discover("network")` correctly finds Songbird via `network.*` prefix scan.
- **Domain fallback table expanded**: "registry" added as CapabilityTaxonomy alias for Discovery→songbird.

### What Remains

- **GAP-MATRIX-01b**: BearDog uses Format E wire format (`provided_capabilities: [{type, methods}]`). biomeOS v2.92 parses Formats A-D but not E. BearDog capabilities still not registered.
- **GAP-MATRIX-07**: biomeOS proxy forwarding to primal sockets fails after discovery. Provider is found, but the actual JSON-RPC forwarding connection fails. Likely a URI handling issue (`unix:///path` vs bare path).
- **GAP-MATRIX-02**: `tower_atomic_bootstrap.toml` still fails biomeOS graph parser.

Critical path: GAP-MATRIX-07 (proxy forwarding) blocks all `capability.call` even for correctly discovered primals.

---

## New Sketches & Experiments (Phase 25+)

| Artifact | Layer | Purpose |
|----------|-------|---------|
| `graphs/sketches/validation/primal_routing_matrix.toml` | L0 | 10-domain Neural API routing sweep |
| `graphs/sketches/mixed_atomics/dual_tower_ionic.toml` | L2 | Two electron shells, ionic bridge |
| `graphs/sketches/mixed_atomics/node_with_dedicated_tower.toml` | L2 | Proton with dedicated electron |
| `graphs/sketches/mixed_atomics/nest_enclave.toml` | L2 | Neutron-heavy isotope, policy fence |
| `graphs/sketches/bonding_patterns/covalent_mesh_backup.toml` | L3 | Sharded encrypted backup across peers |
| `graphs/sketches/bonding_patterns/ionic_capability_lease.toml` | L3 | Metered electron transfer |
| `graphs/sketches/bonding_patterns/organo_metal_salt.toml` | L3 | Multi-bond compound (covalent + ionic + weak) |
| `experiments/exp091_primal_routing_matrix/` | L0 | Drives routing matrix graph |
| `experiments/exp092_dual_tower_ionic/` | L2 | Structural dual-tower + ionic bond validation |
| `experiments/exp093_covalent_mesh_backup/` | L3 | Structural shard model + covalent policy validation |
| `specs/MIXED_COMPOSITION_PATTERNS.md` | All | Particle model, layered validation, gap inventory |

---

## Next Steps

1. **Immediate**: Verify Phase 25 graph migration covers all spring deploy graphs (already done).
2. **Short-term**: Run live Phase B validation on Eastgate for each spring's deploy graph.
3. **Medium-term**: Extend exp090 pattern to validate each spring's NUCLEUS health remotely.
4. **Medium-term**: Run exp091 (L0 routing matrix) to validate all 10 primal domains.
5. **Medium-term**: Implement dual-Tower coexistence in AtomicHarness (L2 gap).
6. **Long-term**: Implement erasure coding as barraCuda primitive for L3 sharding.
7. **Long-term**: Automate the full matrix (columns A-O) as a CI pipeline per spring.
