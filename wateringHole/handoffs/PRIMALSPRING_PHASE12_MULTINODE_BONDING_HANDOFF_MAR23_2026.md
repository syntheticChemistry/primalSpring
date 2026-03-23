# primalSpring Phase 12 — Multi-Node Bonding + Federation Handoff

**Date**: March 23, 2026
**From**: primalSpring v0.7.0, Phase 12
**To**: All primal teams, spring teams, biomeOS
**License**: AGPL-3.0-or-later

---

## Executive Summary

primalSpring Phase 12 extends the ecosystem from single-machine NUCLEUS composition
to multi-node bonding and federation. This handoff documents what was built,
what each team can absorb, and what patterns are now available for cross-spring evolution.

**Key numbers**: 280+ tests, 51 experiments, 22 deploy graphs, 87/87 gates, 9 tracks.

---

## What Was Built

### 1. BondType Full Taxonomy (5 Variants)

The bonding model now covers all chemistry-inspired interaction patterns:

| BondType | Metaphor | Use Case | Properties |
|----------|----------|----------|------------|
| **Covalent** | Shared electrons | Basement HPC, family clusters | `shares_electrons=true`, `is_metered=false` |
| **Metallic** | Electron sea | Compute-only racks, storage fleets | `shares_electrons=true`, `is_metered=false` |
| **Ionic** | Electron transfer | Cloud burst GPU, external APIs | `shares_electrons=false`, `is_metered=true` |
| **Weak** | Van der Waals | Public APIs, unknown beacons | `shares_electrons=false`, `is_metered=false` |
| **OrganoMetalSalt** | Mixed | Simultaneous bond types | `shares_electrons=true`, `is_metered=true` |

**Code**: `ecoPrimal/src/bonding/mod.rs` — `BondType`, `BondType::all()`, `shares_electrons()`, `is_metered()`

### 2. TrustModel Taxonomy

| TrustModel | Authentication | Use Case |
|------------|---------------|----------|
| **GeneticLineage** | `.family.seed` auto-trust | Basement HPC, friend clusters |
| **Contractual** | Service agreements | Cloud providers, metered APIs |
| **Organizational** | Certificate-based | Enterprise deployments |
| **ZeroTrust** | Challenge-response per request | Unknown peers, public APIs |

### 3. BondingConstraint + BondingPolicy

Fine-grained access control for federated resource sharing:

```rust
BondingConstraint {
    capability_allow: Vec<String>,  // "compute.*" — glob patterns
    capability_deny: Vec<String>,   // "storage.*", "ai.*"
    bandwidth_limit_mbps: u64,      // 0 = unlimited
    max_concurrent_requests: u32,
}

BondingPolicy {
    bond_type: BondType,
    trust_model: TrustModel,
    constraints: BondingConstraint,
    active_windows: Vec<String>,    // "22:00-06:00" — idle hours
    offer_relay: bool,              // relay for other bonded peers
    label: String,
}
```

**Presets**: `BondingPolicy::covalent_full()`, `::idle_compute(windows, bandwidth)`, `::ionic_contract(label)`

### 4. Multi-Node Deploy Graph Templates

Four TOML deploy graphs in `graphs/multi_node/`:

| Graph | Scenario | Key Features |
|-------|----------|-------------|
| `basement_hpc_covalent.toml` | LAN HPC mesh | Covalent + GeneticLineage, BirdSong mesh, no NAT |
| `friend_remote_covalent.toml` | Remote friend's PC | STUN 4-tier NAT traversal, BondingPolicy with time windows |
| `idle_compute_federation.toml` | Federated idle compute | Capability-scoped sharing, provenance tracking |
| `data_federation_cross_site.toml` | NestGate cross-site replication | 7-phase pipeline with trio provenance |

Each graph includes `[graph.metadata]` (bond type, trust model) and `[graph.bonding_policy]`
(constraints, time windows, relay offer) validated by `graph_metadata.rs`.

### 5. Graph Bonding Metadata Validation

`ecoPrimal/src/bonding/graph_metadata.rs`:
- Parses `[graph.metadata]` and `[graph.bonding_policy]` from any biomeOS deploy TOML
- Validates consistency: bond type alignment, trust model presence, constraint validity
- `validate_graph_bonding(path)` → `GraphBondingMetadata` with issues list
- `validate_all_graph_bonding(dir)` → batch validation of all graphs in a directory

### 6. STUN Multi-Tier NAT Traversal

`ecoPrimal/src/bonding/stun_tiers.rs`:
- Parses `config/stun/multi_tier.toml` (biomeOS convention)
- 4-tier sovereignty-first escalation: Lineage → Self-hosted → Public → Rendezvous
- `validate_sovereignty_first()` catches violations: parallel attempts, disabled lineage, corporate STUN
- `escalation_order()` returns the tier sequence

### 7. New Experiments

| Exp | Name | What It Validates |
|-----|------|-------------------|
| 071 | Idle compute policy | BondingConstraint.permits(), policy presets, graph metadata for idle compute |
| 072 | Data federation | NestGate replication pipeline, trio provenance discovery, 7-phase structure |

### 8. Evolved Experiments

| Exp | What Changed |
|-----|-------------|
| 030 | + BondType properties, BondingPolicy validation, HPC graph metadata |
| 032 | + Metallic variant, BOND_TYPE_COUNT=5, HPC graph metadata |
| 033 | + BOND_TYPE_COUNT=5, Metallic in variant array |
| 056 | + Bonding metadata for friend_remote, idle_compute, data_federation graphs |

---

## What Each Team Should Absorb

### biomeOS

| Item | Why | How |
|------|-----|-----|
| `[graph.bonding_policy]` TOML section | Deploy graphs now carry bonding intent | Parse `bond_type`, `trust_model`, `constraints` from graph TOML during deployment |
| BondingPolicy enforcement at bond layer | Capability filtering for bonded nodes | When routing `capability.call` across bonds, check `BondingConstraint.permits()` |
| STUN tier escalation | Sovereignty-first NAT traversal | Songbird should escalate Lineage → Self → Public → Rendezvous (never skip) |
| Multi-node graph execution | Basement HPC, friend remote | Graph executor deploys to remote NUCLEUS via bonded Songbird mesh |

### Songbird

| Item | Why | How |
|------|-----|-----|
| BirdSong encrypted beacon mesh | Multi-machine covalent discovery | Already exists; validate mesh formation across LAN |
| STUN sovereignty-first | NAT traversal for remote friends | 4-tier escalation with lineage relay preferred |
| Dark forest discovery | Zero-metadata leakage | Encrypted beacons + challenge-response for unknown peers |
| Hole-punch + relay fallback | When direct connection fails | Relay through bonded peers when hole-punch unavailable |

### BearDog

| Item | Why | How |
|------|-----|-----|
| `.family.seed` for GeneticLineage trust | Auto-trust within a family | Already exists; validate `FAMILY_ID` scoping in multi-node |
| BondingPolicy signing | Cryptographic attestation of bonding contracts | Sign BondingPolicy structs for ionic/contractual bonds |
| Per-bond capability filtering | Security enforcement | Validate that `BondingConstraint` aligns with signed policies |

### NestGate

| Item | Why | How |
|------|-----|-----|
| Cross-site replication | `data_federation_cross_site.toml` assumes `storage.replicate` | Implement NestGate-to-NestGate replication with conflict resolution |
| Federation state tracking | Provenance trio tracks replication | Emit events to rhizoCrypt DAG, attribute via sweetGrass |
| Bonded storage scope | BondingPolicy may deny `storage.*` | Respect capability filtering from BondingConstraint |

### Provenance Trio (sweetGrass, rhizoCrypt, loamSpine)

| Item | Why | How |
|------|-----|-----|
| Federation provenance | Every cross-site operation should be tracked | `ipc::provenance` already wired; fix wire format gaps (see PROVENANCE_TRIO_LIVE_PROBING handoff) |
| loamSpine runtime panic | Blocks live federation validation | Fix `block_on` inside async runtime (refactor to `await` or spawn dedicated thread) |
| rhizoCrypt Unix socket | Currently TCP-only on :9401 | Add Unix socket listener alongside HTTP for ecosystem consistency |
| Event type struct variants | Wire format mismatch | Document/stabilize the externally-tagged enum format for `dag.event.append` |

### ToadStool

| Item | Why | How |
|------|-----|-----|
| Remote compute dispatch | Idle compute federation routes work to remote ToadStool | Validate `compute.submit` works across bonded Songbird mesh |
| Precision-aware routing | Basement HPC may have mixed GPU precision tiers | Route workloads based on `precision_tier` from hardware profile |

### Spring Teams (wetSpring, hotSpring, airSpring, neuralSpring, ludoSpring, healthSpring)

| Pattern | What It Enables |
|---------|----------------|
| BondingPolicy for cross-spring data | Each spring can define capability-scoped sharing policies for its data |
| Provenance trio for domain lineage | wetSpring genetic data, hotSpring simulation runs, neuralSpring model training — all trackable via `capability.call("provenance.*")` |
| Multi-node deploy graphs | Springs can define BYOB niches that span multiple machines |
| STUN sovereignty-first | Remote spring instances discover each other without leaking metadata |
| Idle compute sharing | hotSpring GPU workloads can spill to friend's idle GPU via BondingPolicy |

---

## Established Patterns

These patterns are now validated and available for ecosystem use:

1. **Chemistry-inspired bonding** — 5 bond types model all trust/interaction levels
2. **BondingPolicy as capability firewall** — `BondingConstraint.permits(capability)` gates access at bond layer
3. **Graph bonding metadata** — deploy graphs carry bonding intent in `[graph.metadata]`
4. **Sovereignty-first NAT** — never leak metadata to public infrastructure when private options exist
5. **Capability.call across bonds** — same `capability.call` interface, BondingPolicy filters at bond layer
6. **Zero compile-time coupling** — provenance trio, bonding, federation all via `capability.call`
7. **Graceful degradation** — all multi-node experiments use `check_or_skip` for honest reporting
8. **7-phase data federation** — list → replicate → DAG create → event append → attribute → federate → commit

## Future Patterns (Emerging)

1. **Plasmodium** — decentralized capability aggregation across covalently bonded NUCLEUS instances
2. **BYOB primal DAG execution** — primals as complexity-focused DAG nodes for custom compositions
3. **BTC/ETH provenance anchoring** — sweetGrass `anchoring.anchor` publishes hash attestations to external chains
4. **Novel Ferment Transcript** — loamSpine cert + rhizoCrypt DAG + sweetGrass braid + BearDog sig + optional anchor
5. **sunCloud radiating attribution** — sweetGrass braids distribute value attribution across contributors
6. **Metallic fleet specialization** — compute-only or storage-only racks with delocalized capabilities
7. **Ionic metered bonds** — cloud burst GPU, external API contracts with per-request billing

---

## Files Changed (Phase 12)

| File | Change |
|------|--------|
| `ecoPrimal/src/bonding/mod.rs` | BondType::Metallic, TrustModel, BondingConstraint, BondingPolicy, presets |
| `ecoPrimal/src/bonding/graph_metadata.rs` | NEW — graph bonding metadata parser + validator |
| `ecoPrimal/src/bonding/stun_tiers.rs` | NEW — STUN tier config parser + sovereignty-first validation |
| `ecoPrimal/src/lib.rs` | Updated bonding module docs |
| `ecoPrimal/src/bin/primalspring_primal/main.rs` | Metallic match arm in bonding_test handler |
| `graphs/multi_node/*.toml` | NEW — 4 multi-node deploy graph templates |
| `experiments/exp071_idle_compute_policy/` | NEW — bonding policy experiment |
| `experiments/exp072_data_federation/` | NEW — data federation experiment |
| `experiments/exp030/src/main.rs` | + BondType properties, policy, graph metadata checks |
| `experiments/exp032/src/main.rs` | + Metallic, BOND_TYPE_COUNT=5, graph metadata |
| `experiments/exp033/src/main.rs` | + BOND_TYPE_COUNT=5, Metallic in variant array |
| `experiments/exp056/src/main.rs` | + 3 multi-node graph metadata validations |
| `specs/*` | Updated to Phase 12, future roadmap Phases 13–19 |

---

**License**: AGPL-3.0-or-later
