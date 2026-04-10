# NUCLEUS Spring Alignment — Phase 33 (local copy)

> **Canonical version**: `ecoPrimals/infra/wateringHole/NUCLEUS_SPRING_ALIGNMENT.md`
> This local copy is for primalSpring context. The infra version is the
> inter-spring reference.

**Date**: April 10, 2026
**From**: primalSpring v0.9.9
**License**: AGPL-3.0-or-later

---

## The Atomic Model

Every spring composes from the same NUCLEUS atomics. Each spring
stresses different portions based on its domain. As springs evolve,
they harden the primals they depend on — and those improvements
propagate to every other spring in the ecosystem.

| Atomic | Particle | Primals | Fragment |
|--------|----------|---------|----------|
| Tower | Electron | BearDog + Songbird | `tower_atomic` |
| Node | Proton | Tower + ToadStool + barraCuda + coralReef | `node_atomic` |
| Nest | Neutron | Tower + NestGate + rhizoCrypt + loamSpine + sweetGrass | `nest_atomic` |
| NUCLEUS | Atom | Tower + Node + Nest (9 unique primals) | `nucleus` |
| Meta-tier | — | biomeOS + Squirrel + petalTongue | `meta_tier` |

---

## Spring × Atomic Alignment Matrix

Each spring's proto-nucleate graph lives in `graphs/downstream/`. The matrix
below shows which atomics each spring exercises and what it evolves.

| Spring | Version | Tests | Primary Atomics | Proto-Nucleate | Particle Profile |
|--------|---------|-------|-----------------|----------------|------------------|
| **hotSpring** | 0.6.32 | ~870 | **Node** (proton-heavy) + Nest | `hotspring_qcd_proto_nucleate.toml` | proton_heavy |
| **neuralSpring** | 0.1.0 | 1,403 | **Node** + Meta | `neuralspring_inference_proto_nucleate.toml` | balanced |
| **wetSpring** | 0.1.0 | 1,902 | Node + **Nest** + Meta | `wetspring_lifescience_proto_nucleate.toml` | balanced |
| **airSpring** | 0.10.0 | 986 | Node + **Nest** | `airspring_ecology_proto_nucleate.toml` | balanced |
| **groundSpring** | 0.1.0 | 1,050+ | Node + **Nest** | `groundspring_geoscience_proto_nucleate.toml` | balanced |
| **healthSpring** | 0.1.0 | 928 | **Nest** (neutron-heavy) + Meta | `healthspring_enclave_proto_nucleate.toml` | neutron_heavy |
| **ludoSpring** | 0.1.0 | 222 | Node + **Meta** | `ludospring_proto_nucleate.toml` | balanced |

### Key

- **Bold atomic** = primary domain stress point
- Particle profile: `proton_heavy` = compute-dominated, `neutron_heavy` = storage/compliance-dominated, `balanced` = even mix

---

## neuralSpring: AI Provider for the Ecosystem

neuralSpring has a unique cross-cutting role: as it evolves the WGSL shader
composition for ML inference, **every other spring gains AI capabilities**.

```
neuralSpring evolves inference.complete / inference.embed / inference.models
    ↓ registers as Squirrel provider
Squirrel discovers neuralSpring (or falls back to Ollama)
    ↓ ai.query / ai.complete / inference.*
Every spring with Squirrel in its composition gets AI
```

### What Each Spring Gains from neuralSpring

| Spring | AI Capability | Use Case |
|--------|--------------|----------|
| **hotSpring** | `inference.complete` | AI-guided simulation parameter selection, anomaly detection in QCD measurements |
| **wetSpring** | `inference.complete` + `inference.embed` | AI sample triage, specimen classification, anomaly detection in sensor streams |
| **airSpring** | `inference.complete` + `inference.embed` | Ecological prediction, sensor anomaly detection, crop stress classification |
| **groundSpring** | `inference.complete` | AI-guided calibration, inverse problem parameter estimation |
| **healthSpring** | `inference.complete` + `inference.embed` | Clinical decision support, drug interaction classification, biosignal analysis |
| **ludoSpring** | `inference.complete` | AI Dungeon Master narration, NPC dialogue, game-science optimization |
| **esotericWebb** | `inference.complete` | Narrative generation, session context, AI-driven world building |

### The Inference Evolution Path

```
Phase 1 (now):   Squirrel → Ollama (external vendor, HTTP)
Phase 2 (next):  Squirrel → neuralSpring → WGSL shader composition
                 (tokenization + attention + FFN as barraCuda shaders)
Phase 3 (later): Squirrel → neuralSpring → domain-specific models
                 (each spring contributes domain training data)
```

Every spring that adds Squirrel to its composition immediately benefits
from neuralSpring's inference evolution — without any code changes.

---

## Per-Spring NUCLEUS Composition Detail

### hotSpring — Lattice QCD / HPC Physics

**Atomics**: Tower + **Node** (proton-heavy) + Nest

```
hotSpring domain layer
    ├── coralReef: QCD-specific WGSL (gauge update, Wilson/Dirac, HMC)
    ├── toadStool: metallic GPU fleet dispatch (lattice partitioning)
    ├── barraCuda: df64 tensor shaders (SU(3) matmul, FFT, CG solver)
    ├── NestGate: gauge configuration cache
    └── Provenance trio: reproducibility witness per configuration
```

**What hotSpring evolves for the ecosystem**:
- df64 double-precision GPU emulation → benefits any spring needing high precision
- Multi-GPU metallic dispatch → benefits any spring needing fleet compute
- Shader pipeline scaling → benefits neuralSpring's multi-stage inference
- HPC deployment patterns → benefits CERN/cloud-scale compositions

---

### neuralSpring — ML / AI Inference

**Atomics**: Tower + **Node** + Meta (Squirrel)

```
neuralSpring domain layer
    ├── coralReef: ML-specific WGSL (tokenizer, attention, KV-cache)
    ├── toadStool: inference pipeline scheduling
    ├── barraCuda: transformer shaders (matmul, attention, softmax, gelu)
    ├── Squirrel: inference routing (registers as provider)
    └── NestGate (optional): model weight cache
```

**What neuralSpring evolves for the ecosystem**:
- Tokenization as WGSL shader → vendor-free tokenization for all springs
- Attention/FFN forward pass → native inference without Ollama/CUDA
- `inference.*` wire standard → unified AI interface for all compositions
- Model routing → Squirrel discovers best provider per-request

---

### wetSpring — Life Science & Analytical Chemistry

**Atomics**: Tower + Node + **Nest** + Meta (Squirrel + petalTongue)

```
wetSpring domain layer
    ├── coralReef: domain WGSL (spectral deconvolution, phylogenetics)
    ├── toadStool: GPU/NPU dispatch (Akida edge classification)
    ├── barraCuda: spectral analysis, peak detection, statistical clustering
    ├── NestGate: specimen/sensor time-series storage
    ├── Provenance trio: sample chain-of-custody
    ├── Squirrel: AI-driven triage and anomaly detection
    └── petalTongue: real-time lab monitoring dashboards
```

**What wetSpring evolves for the ecosystem**:
- Time-series storage patterns → benefits any spring with sensor data
- Streaming pipeline composition → benefits real-time processing springs
- NPU/edge dispatch (Akida) → benefits fieldMouse deployments
- Biodiversity attribution → enriches provenance trio patterns

---

### airSpring — Ecological & Agricultural Science

**Atomics**: Tower + Node + **Nest**

```
airSpring domain layer
    ├── coralReef: ecology WGSL (ET₀, soil moisture, canopy resistance)
    ├── toadStool: GPU/NPU dispatch (edge sensor nodes)
    ├── barraCuda: PDE solvers, FFT, statistical analysis
    ├── NestGate: IoT sensor time-series + model outputs
    └── Provenance trio: measurement attribution for compliance
```

**What airSpring evolves for the ecosystem**:
- PDE solver shaders → benefits physics and biology springs
- IoT sensor ingestion patterns → benefits fieldMouse and edge deployments
- NPU dispatch for edge → validates Akida/Coral composition paths
- Environmental compliance attribution → enriches provenance patterns

---

### groundSpring — Geoscience & Measurement Science

**Atomics**: Tower + Node + **Nest**

```
groundSpring domain layer
    ├── coralReef: geology WGSL (noise filters, inverse solvers)
    ├── toadStool: compute dispatch
    ├── barraCuda: FFT, matrix decomposition, Anderson-Darling, WDM
    ├── NestGate: geospatial data + calibration records
    └── Provenance trio: calibration audit trails
```

**What groundSpring evolves for the ecosystem**:
- Statistical shader library → benefits any spring needing data quality checks
- Inverse problem solvers → benefits physics and signal processing
- Long-duration storage patterns → benefits any spring with archival needs
- Calibration traceability → enriches provenance trio for metrology

---

### healthSpring — Clinical / Compliance

**Atomics**: **Tower** (dual-tower) + **Nest** (neutron-heavy) + Meta

```
healthSpring domain layer
    ├── Tower A (data custody): NestGate-A + Provenance Trio A
    │   └── ionic fence: data cannot leave Tower A as raw
    ├── Tower B (analytics): Squirrel + NestGate-B (model cache)
    │   └── ionic bridge: receives only de-identified aggregates
    └── BearDog: cross-family ionic bond enforcement
```

**What healthSpring evolves for the ecosystem**:
- Ionic bond runtime enforcement → benefits any spring with trust boundaries
- Data egress fences → benefits any composition handling sensitive data
- Dual-tower enclave pattern → benefits financial, regulatory, government
- `crypto.sign_contract` capability → enables metered capability sharing
- HIPAA audit trail patterns → enriches provenance trio for compliance

---

### ludoSpring — Game Science / HCI

**Atomics**: Tower + Node + **Meta** (Squirrel + petalTongue)

```
ludoSpring composition (pure — no ludospring binary)
    ├── coralReef: game WGSL (Fitts, Perlin, WFC)
    ├── toadStool: 60Hz tick-budget dispatch
    ├── barraCuda: game math shaders (noise, procedural, physics)
    ├── Squirrel: AI Dungeon Master (narration, NPC dialogue)
    ├── petalTongue: scene rendering, TUI
    └── NestGate: session persistence
```

**What ludoSpring evolves for the ecosystem**:
- 60Hz composition budget → tests graph execution latency limits
- Pure composition proof → validates graph-as-product model
- AI narration under latency → tests Squirrel real-time performance
- Session lifecycle (create/save/restore/fork) → benefits any stateful composition

---

## Cross-Pollination Network

```
hotSpring ──df64/GPU fleet──→ barraCuda/coralReef ←──ML shaders── neuralSpring
    │                              ↕                                    │
    │                         toadStool                                 │
    │                              ↕                                    ↓
    │              ┌── airSpring (PDE/IoT)                         Squirrel
    │              ├── groundSpring (stats/geo)          (inference provider)
    │              ├── wetSpring (spectral/bio)                        ↓
    │              └── ludoSpring (game math)             ALL springs get AI
    │                              ↕
    └──metallic fleet──→ healthSpring ──ionic bonds──→ BearDog
                              ↕
                         NestGate / Provenance trio
                    (every spring benefits from
                     audit, storage, attribution)
```

Each arrow represents a pattern that flows from one spring's domain work
to harden a shared primal. The network is not hierarchical — it's a
feedback web where every spring solving its problem makes every other
spring's composition more capable.

---

## Getting Started (for any spring)

1. **Read your proto-nucleate**: `graphs/downstream/{yourspring}_*_proto_nucleate.toml`
2. **Check atomics**: which fragments does your proto-nucleate declare?
3. **Wire IPC**: call primals by capability, not identity
4. **Validate**: run primalSpring experiments for your composition
5. **Evolve**: push domain-specific WGSL through coralReef, compute through toadStool
6. **Add Squirrel**: when ready for AI, add `squirrel` to your composition — neuralSpring's inference is immediately available
7. **Hand back**: document gaps and patterns → primalSpring → primal teams

---

**License**: AGPL-3.0-or-later
