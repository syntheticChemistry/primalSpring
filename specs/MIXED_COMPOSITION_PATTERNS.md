# Mixed Composition Patterns — Particle Model & Layered Validation

**Date**: April 7, 2026
**Phase**: 25+ (evolution)
**Purpose**: Define generic bonding patterns that springs can specialize, grounded in the particle model from Paper 23 (mass-energy-information equivalence).

---

## 1. Particle Model

The mapping derives from `gen3/baseCamp/23_mass_energy_information_equivalence.md`, which establishes a three-way equivalence: mass (data at rest), energy (data in transit / computation), and information (the structural organization that makes the conversion functional).

### 1.1 Atomic-to-Particle Mapping

| Atomic | Particle | Physical Property | System Property | Fungibility |
|--------|----------|-------------------|-----------------|-------------|
| **Tower** | Electron | Light, orbits nucleus, mediates bonding | BearDog + Songbird: trust boundary, all inter-gate interaction flows through Tower | n/a (bonding medium) |
| **Node** | Proton | Massive, positive charge, defines element identity (atomic number) | ToadStool + BarraCuda: compute substrate. Defines what the gate *is* (GPU node, CPU node, NPU node) | **Fungible** — a TFLOP is a TFLOP regardless of source |
| **Nest** | Neutron | Massive, neutral, stabilizes nucleus | NestGate + Squirrel: content-addressed storage, AI coordination. Data at rest. | **Non-fungible** — content-addressed (BLAKE3), cryptographically unique |
| **NUCLEUS** | Atom | Complete assembly: protons + neutrons + electrons | Full composition: Tower + Node + Nest + provenance trio | — |

### 1.2 Why This Assignment

**Node = proton (compute = energy, fungible)**

The proton defines the element's identity via atomic number. What compute substrate a gate specializes in defines its identity in the mesh: Northgate (RTX 5090, AI-heavy) is a different element than Strandgate (dual EPYC, bio-parallel). But compute itself is fungible — any energy source that provides the required TFLOPS works. A proton is a proton.

**Nest = neutron (data = energy at rest, non-fungible)**

Neutrons are massive, uncharged, and stabilize the nucleus. Neutron count determines the isotope — the specific variant of an element. Data stored in NestGate is content-addressed by BLAKE3 hash: each piece is unique, irreplaceable, cryptographically bound to its content. The specific data profile of a gate (76TB ZFS on Westgate vs. fast NVMe on Eastgate) defines its isotope. Data is non-fungible mass.

**Tower = electron (bonding medium)**

Electrons mediate all inter-atomic bonding. Covalent bonds share electrons. Metallic bonds delocalize them into an electron sea. Ionic bonds transfer them. Tower Atomic (BearDog + Songbird) is exactly what gates use to interact: TLS 1.3, BirdSong mesh discovery, capability routing. The existing code confirms this — `BondType::shares_electrons()` in `ecoPrimal/src/bonding/mod.rs` returns true for Covalent and Metallic bonds.

**Information = biomeOS + sweetGrass**

Paper 23's third leg — information as structural organization — maps to biomeOS orchestration (Neural API, capability routing, graph execution) and sweetGrass provenance (attribution braids, PROV-O). Information is what makes the mass-energy conversion *functional*: without orchestration, protons and neutrons are unbound particles.

### 1.3 Paper 23 Connection

Paper 23 frames computing architecture evolution as the progressive dissolution of the mass-energy boundary:

| Stage | Mass-Energy Relationship | ecoPrimals Mapping |
|-------|------------------------|--------------------|
| Von Neumann | Strictly separated: memory (mass) vs ALU (energy) | Nest (storage) vs Node (compute), Tower (bus) mediates |
| GPU | Partially merged: local VRAM | ToadStool persistent compute buffers |
| Neuromorphic | Weights ARE computation | AKD1000 NPU — no fetch cycle |
| Biological | Mass, energy, information indistinguishable | Target: bonding patterns that blur the boundary |

The bonding patterns below are steps along this evolution. L0-L1 validate the Von Neumann stage (clear separation). L2-L3 explore compositions where the boundary begins to blur (dual Towers, enclaved Nests, cross-gate sharding).

---

## 2. Layered Validation Model

Validation proceeds in four layers, each building on the previous. Springs consume these layers to model their domain-specific compositions.

### L0: biomeOS + Any Primal (Individual Routing)

Validate that biomeOS correctly routes `capability.call` to each of the 10 primal capability domains independently. One node per domain, one assertion per node.

**Graph**: `graphs/sketches/validation/primal_routing_matrix.toml`
**Experiment**: `exp091_primal_routing_matrix`

Domains tested:

| # | Domain | Provider | Method |
|---|--------|----------|--------|
| 1 | security | BearDog | `crypto.sign_ed25519` |
| 2 | discovery | Songbird | `discovery.find_primals` |
| 3 | compute | ToadStool | `compute.submit` |
| 4 | storage | NestGate | `storage.put` |
| 5 | ai | Squirrel | `ai.query` |
| 6 | dag | rhizoCrypt | `dag.session.create` |
| 7 | spine | loamSpine | `spine.create` |
| 8 | braid | sweetGrass | `braid.create` |
| 9 | http | Songbird (via Tower) | `http.get` |
| 10 | mesh | Songbird | `mesh.peers` |

### L1: Each Atomic (Composition Validation)

Validate Tower, Node, and Nest as cohesive compositions. These graphs already exist:

| Atomic | Particle | Graph | What It Proves |
|--------|----------|-------|----------------|
| Tower | Electron | `nest-deploy.toml` phases 1-2 | BearDog + Songbird compose; TLS 1.3 works |
| Node | Proton | `node_atomic_compute.toml` | Tower + ToadStool; GPU compute available |
| Nest | Neutron | `nest-deploy.toml` phases 1-4 | Tower + NestGate + Squirrel; storage round-trip |
| NUCLEUS | Atom | `nucleus_complete.toml` | All particles bound; full capability routing |

No new sketches needed for L1.

### L2: Mixed Atomics (Multi-Instance Compositions)

Compositions with multiple atomic instances on the same host or across hosts. These test what happens when you have more than one of a particle type.

| Sketch | Pattern | Physics Analogy |
|--------|---------|-----------------|
| `dual_tower_ionic.toml` | Two Tower Atomics, different FAMILY_IDs, ionic bond between them | Two electron shells from different atoms — ionic bonding |
| `node_with_dedicated_tower.toml` | Node Atomic with its own Tower separate from the composition's main Tower | Proton with a dedicated electron for compute-facing network |
| `nest_enclave.toml` | Nest Atomic behind BondingPolicy fence | Neutron-heavy isotope — extra data mass behind policy boundary |

**Experiment**: `exp092_dual_tower_ionic`

### L3: Bonding Patterns on Top of Atomics

Inter-atomic bonding patterns that springs can specialize. Tower is the trust boundary for all bonding — full bonding mixing across Towers is late-stage.

| Sketch | Pattern | Physics Analogy | Use Case |
|--------|---------|-----------------|----------|
| `covalent_mesh_backup.toml` | Shared electrons across 3+ gates; Nest data sharded and replicated | Covalent molecule — shared electron cloud | Player-owned Steam: friend mesh backup |
| `ionic_capability_lease.toml` | Tower transfers scoped capability to external gate; metered, time-bounded | Ionic bond — electron transfer | Cloud burst GPU rental |
| `organo_metal_salt.toml` | Covalent internal + ionic edge + weak public | Organo-metal-salt compound — multiple bond types | Full basement-to-cloud production deployment |

**Experiment**: `exp093_covalent_mesh_backup`

---

## 3. Gap Inventory

Patterns identified but not yet implemented. Each gap has a severity and the layer at which it blocks progress.

### 3.1 Blocking Gaps

| Gap | Layer | Description | What Exists | What's Missing |
|-----|-------|-------------|-------------|----------------|
| **Dual-Tower coexistence** | L2 | Two Tower Atomics on same host with different FAMILY_IDs | `AtomicHarness` can run compositions sequentially | Simultaneous multi-Tower, ionic bridge between them |
| **Client-side shard encryption** | L3 | BearDog encrypts data shards before distribution | BearDog `crypto.encrypt` (AES-256-GCM) | Shard-level key management, per-shard metadata |
| **Erasure coding** | L3 | Reed-Solomon or similar for redundant sharding | None | Entire subsystem — could be a barraCuda primitive |

### 3.2 Non-Blocking Gaps

| Gap | Layer | Description | What Exists | What's Missing |
|-----|-------|-------------|-------------|----------------|
| **Enclave enforcement** | L2 | Hardware enclaves (SGX, TrustZone) for compute/data isolation | `BondingPolicy` fence (software) | Hardware attestation, enclave runtime |
| **Friend-vault mesh** | L3 | Distributed encrypted backup across trusted friends | `BondingPolicy`, covalent trust, `storage.*` | Shard distribution logic, recovery protocol, quorum |
| **Cross-tower bonding mixer** | L3+ | Full bonding pattern mixing across Tower boundaries | Single Tower per composition | Late-stage: requires dual-Tower L2 first |
| **Per-primal routing matrix** | L0 | Systematic test of all 10 primal domains | Individual experiments exist (exp060-070, exp075-080) | Unified single-graph sweep |

### 3.3 Future Considerations

- **Isotope stability**: some neutron-heavy compositions (large data, small compute) may be unstable — graceful degradation must handle this
- **Beta decay analog**: a Node-heavy gate losing compute capability should shed work to peers (proton→neutron + positron in physics; compute→data + notification in the system)
- **Fusion/fission**: merging or splitting NUCLEUS instances at runtime (gate joins mesh = fusion, gate departs = fission)
- **Photon analog**: the actual data packets flowing between gates. Songbird HTTP requests and BirdSong multicast beacons are the photons — carriers of the electromagnetic force (Tower/electron interaction)

---

## 4. Tower as Trust Boundary

All bonding patterns flow through Tower (electron). This is a deliberate architectural constraint, not a limitation:

1. **Covalent**: shared family seed verified by BearDog (Tower). Songbird (Tower) discovers peers via BirdSong.
2. **Ionic**: contract negotiated and enforced by BearDog (Tower). Metering tracked through Songbird (Tower).
3. **Metallic**: electron sea = pool of Tower Atomics providing delocalized discovery and relay.
4. **Weak**: all untrusted interaction enters through Tower and stays at Tower level until trust escalates.

Full bonding mixing across Tower boundaries (e.g., a Node on gate A directly accessing a Nest on gate B without going through either Tower) is explicitly late-stage. The current architecture requires all cross-gate traffic to transit Tower. This matches physics: inter-atomic forces are mediated by electron interactions, not by direct proton-neutron contact.

---

## 5. Spring Specialization Guide

Each spring inherits the generic patterns and specializes by adjusting the particle ratios:

| Spring | Primary Particle | Isotope Profile | Key Pattern |
|--------|-----------------|-----------------|-------------|
| hotSpring | Proton-heavy | GPU-dense Node, minimal Nest | L3 metallic (compute fleet) |
| wetSpring | Balanced | Genomic data Nest + bio compute Node | L3 covalent mesh backup |
| healthSpring | Neutron-heavy | HIPAA-scoped Nest enclave + zero-knowledge Tower | L2 dual tower + nest enclave |
| ludoSpring | Proton-heavy | Real-time compute Node + game state Nest | L2 node with dedicated Tower |
| airSpring | Balanced | Sensor data Nest + simulation Node | L3 covalent mesh (field stations) |
| groundSpring | Proton-heavy | Uncertainty compute Node | L3 ionic lease (burst compute) |
| neuralSpring | Balanced | Model weight Nest + training Node | L2 nest enclave (model IP) |

---

## 6. Cross-References

| Document | Relationship |
|----------|-------------|
| `gen3/baseCamp/23_mass_energy_information_equivalence.md` | Paper 23: theoretical foundation for particle model |
| `gen3/ECOSYSTEM_ARCHITECTURE.md` §3-4 | Atomic composition and bonding model definitions |
| `gen3/atlasHugged/07_THE_MOBILITY_EDGE.md` | Anderson localization as bonding/isolation phase transition |
| `ecoPrimal/src/bonding/mod.rs` | `BondType`, `TrustModel`, `BondingConstraint`, `BondingPolicy` |
| `ecoPrimal/src/coordination/mod.rs` | `AtomicType` (Tower, Node, Nest, FullNucleus) |
| `specs/NUCLEUS_VALIDATION_MATRIX.md` | Columns K-O reference this document's layered model |
| `graphs/sketches/mixed_atomics/` | L2 graph sketches |
| `graphs/sketches/bonding_patterns/` | L3 graph sketches |
