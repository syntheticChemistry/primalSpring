# primalSpring v0.9.4 — BTSP, Inference Abstraction, and Proto-Nucleate Graphs Handoff

**Date**: April 10, 2026  
**From**: primalSpring (coordination spring)  
**To**: All primal teams, downstream springs (neuralSpring, hotSpring, healthSpring, wetSpring, groundSpring, airSpring), sporeGarden  
**Phase**: 28 — BTSP Phase 2 + Inference Abstraction + Proto-Nucleate Graphs

---

## Summary

primalSpring v0.9.4 delivers three major architectural advances:

1. **BTSP Phase 2 Cascade** — Secure-by-default authentication across 11/13 primals. Zero TCP ports. All 107 deploy graphs carry `secure_by_default = true` metadata.
2. **Inference Provider Abstraction** — Vendor-agnostic `inference.complete`/`embed`/`models` wire standard in ecoPrimal. Squirrel bridges to `AiRouter` (Ollama as OpenAI-compatible endpoint). No CUDA or Ollama lock-in.
3. **Proto-Nucleate Graphs** — 5 downstream composition graphs + 3 pipeline graphs defining how neuralSpring, hotSpring, and healthSpring should compose NUCLEUS primals for their domains.

The unifying insight: **ML inference, QCD physics, and biology are all compositions of the same barraCuda WGSL shaders**, compiled by coralReef, dispatched by toadStool. Springs are application layers, not standalone compute engines.

---

## For Primal Teams

### BTSP Phase 2 Status

| Primal | BTSP Phase 2 | Notes |
|--------|-------------|-------|
| BearDog | **Enforces** | Core BTSP authority, provides `btsp.handshake` |
| Songbird | **Enforces** | Network layer authenticates peers |
| NestGate | **Enforces** | Storage layer, BTSP on all socket connections |
| Squirrel | **Enforces** | AI/MCP hub, BTSP + inference bridge |
| ToadStool | **Enforces** | Compute dispatch, BTSP on shader submission |
| biomeOS | **Enforces** | Orchestration substrate, BTSP on Neural API |
| barraCuda | **Enforces** | GPU compute, BTSP on tensor operations |
| coralReef | **Enforces** | Shader compiler, BTSP on compile requests |
| petalTongue | **Enforces** | Rendering, BTSP on scene operations |
| loamSpine | **Enforces** | Ledger, BTSP on append/query |
| rhizoCrypt | **Enforces** | DAG, BTSP on commit/branch operations |
| sweetGrass | Not yet | Attribution — BTSP integration pending |
| rustChip | N/A | CLI tool, no server mode |

**Action for primal teams**: If your primal does not yet enforce BTSP Phase 2, add `btsp.handshake` as the first method call after socket accept. See `ecoPrimal/src/ipc/btsp_handshake.rs` for the client-side reference implementation (FAMILY_ID + nonce + HMAC pattern).

### Inference Wire Standard

All primals that route or process AI/ML requests should understand the `inference.*` wire:

| Method | Purpose | Wire Types |
|--------|---------|-----------|
| `inference.complete` | Text generation (chat/completion) | `CompleteRequest` → `CompleteResponse` |
| `inference.embed` | Vector embedding | `EmbedRequest` → `EmbedResponse` |
| `inference.models` | List available models + providers | `()` → `ModelsResponse` |

Wire types are defined in `ecoPrimal/src/inference/types.rs`. The `InferenceClient` in `ecoPrimal/src/inference/mod.rs` handles socket discovery and RPC serialization.

**Squirrel team**: Squirrel is the current inference bridge. `handlers_inference.rs` bridges ecoPrimal wire types to `AiRouter`. As new providers come online (native WGSL-based inference via neuralSpring evolution), Squirrel routes transparently.

### Deploy Graph Metadata

All 107 deploy graphs now carry:

```toml
[graph.metadata]
secure_by_default = true
btsp_phase = 2
```

**biomeOS team**: Consider parsing `btsp_phase` metadata during graph loading to enforce minimum security posture for compositions.

---

## For neuralSpring

### What to Absorb

**Proto-nucleate graph**: `graphs/downstream/neuralspring_inference_proto_nucleate.toml`  
**Pipeline graph**: `graphs/neuralspring_inference_pipeline.toml`  
**Deploy graph** (updated): `graphs/spring_deploy/neuralspring_deploy.toml`

### Architecture

neuralSpring's ML inference is a **composition of existing WGSL shader primals**, not a standalone inference engine:

```
neuralSpring (application layer — composes the pipeline)
    → coralReef (compiles WGSL tokenizer kernels + attention programs)
    → toadStool (dispatches compiled workloads to GPU/CPU substrate)
    → barraCuda (executes 826 WGSL compute shaders: matmul, attention, FFT, softmax)
    → NestGate (caches model weights, KV cache, inference results)
    → Squirrel (routes inference requests, fallback to Ollama during transition)
```

### Evolution Targets

1. **Tokenization as shader operation** — Evolve tokenizer from CPU Rust code to a coralReef-compiled WGSL kernel running on barraCuda. BPE merge tables as GPU-side lookup textures.
2. **Forward pass pipelines** — Compose transformer attention, FFN, and layer norm as a `PipelineGraph` of barraCuda shader dispatches. Each layer is a graph stage.
3. **Model weight format** — Define a NestGate-native weight storage format that toadStool can dispatch directly to barraCuda without host-side copies.
4. **Multi-head attention composition** — barraCuda already has `attention_*` shaders. neuralSpring composes them into multi-head patterns via coralReef programs.
5. **Inference as primal composition validation** — Each ML inference run is a composition that primalSpring can validate: correct shader dispatch, proper BTSP authentication, provenance of model weights.

### Key Dependencies

| Primal | Why Required |
|--------|-------------|
| coralReef | Compiles WGSL programs (tokenizer kernels, attention compositions) |
| toadStool | Dispatches compiled workloads to GPU or CPU fallback |
| barraCuda | Provides 826 WGSL compute shaders (the actual math) |
| NestGate | Model weight cache, KV cache, inference result storage |
| Squirrel | Routes `inference.*` requests, bridges Ollama during transition |

---

## For hotSpring

### What to Absorb

**Proto-nucleate graph**: `graphs/downstream/hotspring_qcd_proto_nucleate.toml`  
**Pipeline graph**: `graphs/hotspring_qcd_pipeline.toml`  
**Deploy graph** (updated): `graphs/spring_deploy/hotspring_deploy.toml`

### Architecture

hotSpring's Lattice QCD is a **proton-heavy** composition — compute-dominated, requiring metallic GPU pool bonding:

```
hotSpring (physics application — defines lattice + HMC trajectories)
    → coralReef (compiles QCD WGSL operators: Wilson-Dirac, plaquette, HMC)
    → toadStool (metallic bond — dispatches across GPU fleet, electron sea pattern)
    → barraCuda (df64 tensor execution — double-float-64 emulation on GPU)
    → NestGate (gauge configuration cache between HMC trajectories)
    → sweetGrass (provenance for reproducibility — every trajectory is attributed)
```

### CERN-Level Cloud Deployment

The proto-nucleate graph includes bonding policy for institutional-grade deployment:

- **Metallic bond**: Multiple toadStool instances form a GPU pool. Shared compute fleet with dynamic work-stealing.
- **Ionic lease**: CERN cloud resources are accessed via cross-family ionic bonds with capability masks — hotSpring leases GPU time without exposing internal gauge configurations.
- **df64 precision**: barraCuda's double-float-64 emulation is mandatory for QCD. Standard f32 is insufficient for Wilson-Dirac operator precision.
- **Provenance trio**: Every HMC trajectory, gauge configuration, and measurement carries full provenance (rhizoCrypt DAG + loamSpine ledger + sweetGrass attribution). Required for reproducibility in computational physics.

### Evolution Targets

1. **Wilson-Dirac operator as coralReef program** — Compose barraCuda df64 shaders into the Wilson-Dirac operator via coralReef WGSL compilation.
2. **HMC trajectory pipeline** — Each molecular dynamics trajectory is a `PipelineGraph` stage: gauge update → force calculation → leapfrog integration → accept/reject.
3. **Multi-GPU domain decomposition** — Lattice sites distributed across GPU fleet via toadStool metallic dispatch. Communication via NestGate boundary exchange.
4. **Gauge configuration storage** — NestGate as thermalized gauge config repository with sweetGrass provenance for each configuration.

---

## For healthSpring

### What to Absorb

**Proto-nucleate graph**: `graphs/downstream/healthspring_enclave_proto_nucleate.toml`  
**Pipeline graph**: `graphs/healthspring_clinical_pipeline.toml`  
**Deploy graph** (updated): `graphs/spring_deploy/healthspring_deploy.toml`

### Architecture

healthSpring is **neutron-heavy** — data-dominated, requiring dual-tower enclave security:

```
Tower A (Patient Data Enclave):
    healthSpring (ingest + de-identify)
    → NestGate-A (enforces BondingPolicy egress fence — no raw patient data leaves)
    → rhizoCrypt-A + loamSpine-A + sweetGrass-A (provenance for audit trail)

    ═══ ionic bridge (aggregates only, capabilities_denied: storage.*, dag.*) ═══

Tower B (Analytics Tower):
    Squirrel (clinical AI inference on de-identified data)
    → NestGate-B (model weight cache, inference results)
    → rhizoCrypt-B + loamSpine-B + sweetGrass-B (analytics provenance)
```

### Security Model

- **Ionic bond** between towers: Different `FAMILY_ID` trust domains. Only de-identified statistical aggregates cross the bridge. `capabilities_denied = ["storage.*", "dag.*"]` prevents raw data access from Tower B.
- **BondingPolicy egress fence**: NestGate-A enforces a strict egress policy — patient records cannot be retrieved by any entity outside Tower A's trust domain.
- **Regulatory provenance**: Both towers carry full provenance trios. Every data access, transformation, and inference is attributed and immutable. HIPAA/GDPR audit trail is a composition property.

### Evolution Targets

1. **De-identification as shader composition** — Evolve PII scrubbing from CPU regex to coralReef-compiled WGSL text processing kernels on barraCuda.
2. **Clinical AI via inference wire** — healthSpring's clinical models use `inference.complete` routed through the ionic bridge to Squirrel in Tower B. No model weights touch patient data.
3. **Encrypted aggregation** — NestGate-A computes encrypted aggregates (homomorphic or MPC) that cross the ionic bridge without exposing individual records.
4. **BondingPolicy enforcement validation** — primalSpring can validate that healthSpring's composition correctly prevents data leakage: deploy graph structural analysis + live probing.

---

## For wetSpring / groundSpring / airSpring

Your deploy graphs exist in `graphs/spring_deploy/` but do not yet have proto-nucleate compositions. Based on the patterns from neuralSpring/hotSpring/healthSpring, here is guidance:

### wetSpring (Biology)

- **Particle model**: Balanced (compute + data)
- **Priority**: Biology shader compositions — protein folding energy minimization, molecular dynamics, sequence alignment as barraCuda shader pipelines
- **Pattern to adopt**: neuralSpring's WGSL shader composition model. AlphaFold-like structure prediction = same barraCuda attention shaders + different training data
- **Proto-nucleate template**: `neuralspring_inference_proto_nucleate.toml` with biology-specific capabilities

### groundSpring (Earth Science)

- **Particle model**: Balanced
- **Priority**: Uncertainty quantification — Monte Carlo sampling as barraCuda shader dispatch, ensemble forecasting as parallel pipeline stages
- **Pattern to adopt**: hotSpring's metallic GPU pool for ensemble runs + provenance trio for measurement reproducibility
- **Proto-nucleate template**: `hotspring_qcd_proto_nucleate.toml` adapted for Monte Carlo rather than HMC

### airSpring (Atmospheric / Ag IoT)

- **Particle model**: Neutron-leaning (data-heavy IoT ingest)
- **Priority**: Weather model inference + agricultural sensor data pipelines
- **Pattern to adopt**: healthSpring's dual-tower enclave if handling proprietary sensor data; neuralSpring's inference pipeline for weather model serving
- **Proto-nucleate template**: Hybrid of `healthspring_enclave_proto_nucleate.toml` (data security) + `neuralspring_inference_proto_nucleate.toml` (model serving)

---

## Cross-Spring Evolution Model

Each spring solving its domain unlocks patterns for all others:

```
hotSpring GPU work        → coralReef evolved (WGSL compiler improvements)
                          → barraCuda df64 shaders became ecosystem-wide

neuralSpring ML inference → tokenization as shader ops (flows to all springs)
                          → attention composition (AlphaFold for physics = same pattern)
                          → inference wire standard (every spring can serve models)

healthSpring enclave      → ionic bond enforcement (any spring handling sensitive data)
                          → dual-tower pattern (regulatory compliance as composition)
                          → egress fence validation (primalSpring can audit any spring)

wetSpring biology         → (future) protein shaders → barraCuda evolution
                          → molecular dynamics patterns → hotSpring cross-pollination

groundSpring earth sci    → (future) uncertainty quantification → all springs benefit
                          → ensemble patterns → metallic bond evolution
```

The feedback loop: springs absorb proto-nucleate graphs → evolve against them → pass patterns back to primalSpring → primalSpring refines and passes up to primals as needed.

---

## What Flows Back Up to Primals

Gaps and patterns identified during this sprint that need primal-level evolution:

| Gap | Primal | Priority | Description |
|-----|--------|----------|-------------|
| BondingPolicy enforcement at runtime | NestGate | Medium | Deploy graph declares egress fence; NestGate should enforce it at storage level |
| Ionic bridge metering | biomeOS | Medium | Track data volume crossing ionic bonds for compliance auditing |
| Multi-GPU dispatch improvements | toadStool | Medium | Metallic bond GPU pool needs work-stealing and domain decomposition support |
| NestGate enclave mode | NestGate | Medium | Dedicated mode where NestGate enforces strict data residency within a Tower |
| sweetGrass BTSP Phase 2 | sweetGrass | Low | Last primal to enforce BTSP handshake on connections |
| coralReef tokenizer kernel support | coralReef | Medium | WGSL compiler needs text-processing kernel support for tokenization evolution |
| barraCuda df64 documentation | barraCuda | Low | Document df64 precision guarantees for downstream springs |

---

## Artifacts

| Artifact | Path | Description |
|----------|------|-------------|
| BTSP handshake client | `ecoPrimal/src/ipc/btsp_handshake.rs` | Client-side BTSP authentication module |
| Inference wire types | `ecoPrimal/src/inference/types.rs` | Vendor-agnostic `CompleteRequest`, `EmbedRequest`, `ModelsResponse` |
| Inference client | `ecoPrimal/src/inference/mod.rs` | `InferenceClient` with socket discovery |
| Squirrel inference bridge | `squirrel/crates/main/src/rpc/handlers_inference.rs` | `inference.*` dispatch handlers |
| neuralSpring proto-nucleate | `graphs/downstream/neuralspring_inference_proto_nucleate.toml` | ML inference composition |
| hotSpring proto-nucleate | `graphs/downstream/hotspring_qcd_proto_nucleate.toml` | QCD physics composition |
| healthSpring proto-nucleate | `graphs/downstream/healthspring_enclave_proto_nucleate.toml` | Dual-tower enclave |
| neuralSpring pipeline | `graphs/neuralspring_inference_pipeline.toml` | ML inference data flow |
| hotSpring pipeline | `graphs/hotspring_qcd_pipeline.toml` | QCD simulation data flow |
| healthSpring pipeline | `graphs/healthspring_clinical_pipeline.toml` | Clinical data flow |
| Gap registry | `docs/PRIMAL_GAPS.md` | Updated with inference + nucleation sections |

---

## How to Absorb This Handoff

1. **Read your proto-nucleate graph** — It defines the target composition for your spring
2. **Read your pipeline graph** — It models the data flow through primal compositions
3. **Read your updated deploy graph** — It has the latest capabilities and dependencies
4. **Start composing** — Wire your application logic to call primals via ecoPrimal's IPC client
5. **Validate** — Use primalSpring's experiment harness to validate your compositions
6. **Pass patterns back** — When you discover gaps or improvements, document them and hand back to primalSpring

---

**License**: AGPL-3.0-or-later
