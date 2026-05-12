# Compute Trio Evolution — Node Atomic Domain Split + Absorption

**Status**: SKETCH (May 11, 2026)
**Owner**: primalSpring (L2 gate) — defines contracts, hands upstream to primal teams
**Phase**: Interstadial — pre-wiring sovereign compute composition

---

## Strategic Frame

The ecosystem's atomic patterns are maturing into well-defined composition tiers:

| Atomic | Particle | Primals | Capabilities |
|--------|----------|---------|-------------|
| **Tower** | Electron | bearDog + songbird + skunkBat (3) | security, discovery, defense |
| **Node** | Proton | Tower + toadStool + barraCuda + coralReef (6) | + compute, tensor, shader |
| **Nest** | Neutron | Tower + NestGate + provenance trio (7) | + storage, dag, ledger, attribution |
| **NUCLEUS** | Atom | Tower + Node + Nest (10 core) + meta-tier (3) = 13 | all 13 domains |

The **provenance trio** (rhizoCrypt + loamSpine + sweetGrass) recently shipped
composition readiness (S67, v0.7.34, JH-5 Phase 3). The **compute trio**
(toadStool + barraCuda + coralReef) has the infrastructure but the E2E
sovereign dispatch path is not yet wired or tested in composition.

The hotSpring team's sovereign compute breakthrough (3 GPUs, warm-catch
pipeline, pure Rust ELF patching) and the wateringHole
`SOVEREIGN_COMPUTE_THREE_GPU_WARM_CATCH_HANDOFF_MAY11_2026.md` document the
domain split clearly. This document defines the primalSpring contracts.

---

## The HOW / WHERE / WHAT Domain Split

Following the Nest atomic precedent (NestGate does not embed BearDog's
crypto — it calls `crypto.sign` via IPC), the compute trio splits into
three domains. **Neither primal links the other's crate at compile time.**
All composition is JSON-RPC over IPC.

### coralReef — HOW (compiler domain)

**Owns**: compilation of compute kernels into target-specific machine code.

| Component | What |
|-----------|------|
| `coral-reef` | WGSL → naga IR → SASS/ISA compiler (SM35, SM70, SM120, GFX10) |
| Shader encoders | PTX emit, SASS instruction encoders, QMD struct generation |
| `naga_translate` | IR construction, optimization passes, register allocation |
| `coral-reef-stubs` | CPU simulation stubs for offline validation |
| ELF patcher | Pure Rust kernel module binary patching (warm-catch tooling) |

**IPC surface**: `shader.compile.wgsl`, `shader.compile.spirv`,
`shader.compile.capabilities`, `shader.compile.wgsl.multi`, `shader.health`,
`shader.validate`, `shader.dispatch`, `shader.bio`

**Key principle**: coralReef outputs a **compiled binary blob + ShaderInfo**
(GPRs, shared memory, barriers, workgroup size, wave size, local mem). It
does not know which GPU will execute the blob.

### toadStool — WHERE (hardware domain)

**Owns**: hardware lifecycle, device management, and compute dispatch.

| Component | What |
|-----------|------|
| BAR0/MMIO | Register access to GPU/NPU hardware |
| VFIO channel | Channel creation, GPFIFO/pushbuf submission, semaphore fence |
| Sovereign init | Device boot sequence, memory training, firmware management |
| Cylinder | Per-device subprocess isolation (GPU, NPU, USB, HSM) |
| ember (absorbing) | Device hold/release/reacquire, swap journal, metadata store |
| glowplug (absorbing) | Fleet orchestration, personality detection, health probes |

**IPC surface**: `compute.dispatch`, `compute.dispatch.submit`,
`compute.dispatch.result`, `compute.execute`, `compute.capabilities`,
`compute.status`, `compute.health`, `compute.gpu`, `compute.offload`,
`compute.pool`, `compute.precision`

**Key principle**: toadStool receives a **compiled binary blob** from
coralReef (or from barraCuda's SovereignDevice client) and dispatches it to
the appropriate hardware target. toadStool owns the driver stack.

### barraCuda — WHAT (math/physics domain)

**Owns**: mathematical and scientific computation — the workload itself.

| Component | What |
|-----------|------|
| WGSL math | 826+ compute shaders: matmul, attention, FFT, df64, Perlin, WFC, diversity, Anderson spectral |
| Tensor store | In-memory tensor handles with GPU/CPU/IPC dispatch tiers |
| SovereignDevice | IPC client for sovereign dispatch via trio |
| 4-tier fallback | GPU (wgpu) → CPU (lavapipe/cpu-shader) → IPC (sovereign) → scalar |

**IPC surface**: `tensor.*`, `stats.*`, `math.*`, `noise.*`, `linalg.*`,
`spectral.*`, `activation.*`, `fhe.*`, `compute.dispatch` (local tensor ops)

**Key principle**: barraCuda is a **math library** that delegates compilation
to coralReef and dispatch to toadStool when sovereign mode is active. It
never touches VFIO, DRM, or GPU hardware directly.

---

## Sovereign Dispatch E2E Path

```
barraCuda (WGSL math workload)
  │
  ├─── shader.compile.wgsl ──→ coralReef (compile WGSL → native binary)
  │                              │
  │                              └── returns: { binary: [u8], shader_info: ShaderInfo }
  │
  └─── compute.dispatch.submit ──→ toadStool (dispatch binary → hardware)
                                     │
                                     ├── VFIO: BAR0 → GPFIFO → pushbuf → semaphore → readback
                                     ├── DRM: nouveau EXEC / amdgpu CS
                                     └── NPU: Akida backend → inference
                                     │
                                     └── returns: { result: [u8], dispatch_id, timing }
```

### IPC Contract: shader.compile.wgsl

```json
{
  "method": "shader.compile.wgsl",
  "params": {
    "source": "<WGSL source text>",
    "target": "sm70",
    "entry_point": "main",
    "workgroup_size": [256, 1, 1]
  }
}
```

Response:

```json
{
  "binary_b64": "<base64-encoded SASS/ISA binary>",
  "shader_info": {
    "gprs": 32,
    "shared_memory": 0,
    "barriers": 0,
    "workgroup": [256, 1, 1],
    "wave_size": 32,
    "local_memory": 0
  },
  "target": "sm70",
  "compile_time_ms": 42
}
```

### IPC Contract: compute.dispatch.submit

```json
{
  "method": "compute.dispatch.submit",
  "params": {
    "binary_b64": "<base64-encoded compiled binary>",
    "shader_info": { "gprs": 32, "shared_memory": 0, ... },
    "dispatch_dims": [1, 1, 1],
    "buffers": [
      { "binding": 0, "data_b64": "<base64 input data>", "size": 1024, "usage": "storage" }
    ],
    "target_bdf": "optional PCI BDF for specific device"
  }
}
```

Response:

```json
{
  "dispatch_id": "uuid",
  "status": "completed",
  "buffers": [
    { "binding": 0, "data_b64": "<base64 output data>" }
  ],
  "timing": {
    "dispatch_ms": 1.2,
    "readback_ms": 0.3
  }
}
```

---

## Ember/Glowplug Absorption into toadStool

The wateringHole handoff documents a 6-phase absorption path. toadStool
already has the trait surface waiting — these are the implementations
that fill the traits.

### Phase 1: Absorb coral-ember implementations

coral-ember (228 tests, ~9-12k LOC) provides:

| coral-ember Type | toadStool Trait Target | Notes |
|-----------------|----------------------|-------|
| `HeldDevice` | `ResourceHandle` impl | First production `ResourceHandle` — wraps `VfioDevice` + ring metadata |
| `SwapObservation` | `SwapJournal` integration | Journal entries for device swaps |
| `VendorLifecycle` | `DevicePersonality` integration | NVIDIA/AMD/Intel lifecycle detection |
| `EmberConfig` | Integration with toadStool config system | Socket paths, hold policies |

coral-ember RPC methods that move to toadStool:
`ember.vfio_fds`, `ember.list`, `ember.release`, `ember.reacquire`,
`ember.swap`, `ember.device_reset`, `ember.status`, `ember.journal.*`,
`ember.ring_meta.*`, `mmio.*`, `ember.sovereign.init`, `ember.devinit.*`,
`ember.vbios.read`, `ember.kmod.*`

### Phase 2: Absorb coral-glowplug implementations

coral-glowplug (436 tests, ~12-18k LOC) provides:

| coral-glowplug Type | toadStool Trait Target | Notes |
|--------------------|----------------------|-------|
| `DeviceSlot` (glowplug) | `DeviceSlot` (toadStool) | Same concept, different implementations |
| `Personality` | `DevicePersonality` impl | GPU personality detection |
| `sovereign_boot` | `SwapOrchestrator` fill | Completes the 7-step flow (currently doc-only in toadStool) |
| `EmberClient` | Internal IPC bridge | Becomes toadStool-internal after ember absorption |

coral-glowplug RPC methods that become toadStool's hardware surface:
`device.list`, `device.get`, `device.swap`, `device.health`,
`device.register_dump`, `device.dispatch`, `device.compute_info`,
`device.quota`, `device.lend`, `device.reclaim`, `mailbox.*`, `ring.*`

### Phase 3: Absorb coral-driver hardware access

coral-driver (~50k+ LOC) hardware layer:
- BAR0/MMIO register access → toadStool driver layer
- VFIO channel creation, GPFIFO/pushbuf submission → toadStool dispatch
- Sovereign init stages → toadStool device lifecycle
- DRM ioctl wrappers (nouveau EXEC, amdgpu CS) → toadStool driver layer

### Phase 4: Validate with Akida NPU + AMD

toadStool validates that absorbed ember/glowplug patterns generalize:
- Akida NPU: device hold/swap/health/dispatch on non-GPU silicon
- AMD GPUs: full lifecycle + dispatch on RDNA hardware
- Intel Arc: xe driver personality + dispatch validation

### Phase 5: Generalize cylinder

Cylinder architecture (per-device subprocess isolation) generalizes from
"per-GPU subprocess" to "per-device subprocess" — GPU + NPU + USB + HSM.

### Phase 6: Serve compute.dispatch.execute

toadStool receives compiled binaries from coralReef and dispatches to any
hardware. The `compute.dispatch.execute` method becomes the universal
dispatch entry point.

**Known issue**: BrainChip vendor ID mismatch between `toadstool-common`
(0x1e96) and `akida-driver` (0x1E7C) — reconcile during Phase 4.

---

## Comparison with Provenance Trio (Nest Atomic)

| Aspect | Provenance Trio (Nest) | Compute Trio (Node) |
|--------|----------------------|-------------------|
| **Pattern** | Event → Certificate → Attribution | Compile → Dispatch → Compute |
| **Primals** | rhizoCrypt + loamSpine + sweetGrass | coralReef + toadStool + barraCuda |
| **Data flow** | `dag.event.append` → `session.commit` → `braid.create` | `shader.compile.wgsl` → `compute.dispatch.submit` → `tensor.*` results |
| **Composition status** | Shipped (S67 + v0.7.34 + JH-5 Phase 3) | Infrastructure exists, E2E not wired |
| **Domain split** | WHO (rhizoCrypt: events) / WHAT (loamSpine: certificates) / WHY (sweetGrass: attribution) | HOW (coralReef: compile) / WHERE (toadStool: hardware) / WHAT (barraCuda: math) |
| **Absorption** | None needed (clean domain boundaries) | ember/glowplug → toadStool (hardware migration) |

---

## Degradation Tiers (PrimalBridge pattern)

Following `COMPOSITION_PATTERNS.md` degradation model:

| Tier | Compute Trio State | barraCuda Behavior |
|------|-------------------|-------------------|
| **Full** | coralReef + toadStool + GPU hardware | Sovereign dispatch: WGSL → native SASS → VFIO dispatch → GPU results |
| **Partial** | coralReef absent, wgpu available | Local wgpu: WGSL → Vulkan/Metal pipeline → GPU results (vendor driver) |
| **Minimal** | No GPU, CPU only | CPU shader: WGSL → cpu-shader or lavapipe → CPU results |
| **Standalone** | No IPC, no GPU, no CPU shader | Pure Rust scalar: inline `stats.mean`, `math.sigmoid`, etc. |

Each tier produces correct results. Performance degrades; correctness is
invariant. The `SovereignDevice` in barraCuda implements the Full tier via
IPC to the trio. `Auto::new()` tries wgpu → cpu → sovereign in order.

---

## primalSpring Validation Surface

### Routing (current)

```
capability_to_primal("compute") → toadstool
capability_to_primal("tensor")  → barracuda
capability_to_primal("shader")  → coralreef
```

### Node Atomic Definition

```rust
AtomicType::Node.required_capabilities() = ["security", "discovery", "compute", "tensor", "shader"]
AtomicType::Node.required_primals()      = ["beardog", "songbird", "toadstool", "barracuda", "coralreef"]
```

### Registry Coverage (413 methods)

| Domain | Owner | Registered Methods | Exercised in Tests |
|--------|-------|-------------------|--------------------|
| compute | toadstool | 19 | Partial (capabilities, health) |
| tensor | barracuda | 15+ | Partial (create, matmul, mean) |
| shader | coralreef | 8 | Partial (compile.capabilities) |
| stats | barracuda | 6+ | Partial (mean, std_dev) |

Key gap: `compute.dispatch.submit` → `compute.dispatch.result` E2E path
is **registered but never exercised** — the exact gap class Wave 7 closes
for content.

### Validation Scenarios

| Scenario | ID | What It Tests | Status |
|----------|----|----|--------|
| `s_compute_triangle` | compute-triangle | Discovery + health for 3 capabilities | **LIVE** (compile+dispatch is SKIP placeholder) |
| `s_node_atomic` | node-atomic | Structural + discovery + health for Node | **LIVE** |
| `s_composition_parity` | composition-parity | Cross-atomic pipeline (tensor.stats.mean) | **LIVE** |

**Missing**: Sovereign dispatch contract test (Wave 8 target).

---

## Upstream Handoff Matrix

| Team | primalSpring Provides | Team Action |
|------|----------------------|-------------|
| **toadStool** | Architecture doc, IPC contracts, gate tests, deploy graph with `compute.dispatch.execute` | Absorb ember/glowplug (Phases 1-3), wire `compute.dispatch.execute`, validate on Akida/AMD |
| **coralReef** | Domain split boundary, `shader.compile.*` contract shape expectations | Keep compiler domain, extract hardware code, serve `shader.compile.*` only |
| **barraCuda** | Sovereign dispatch E2E contract, `stats.mean` gate test expectations | Absorb bearDog crypto IPC (Wave 101), wire sovereign dispatch E2E through trio |
| **hotSpring** | Compute trio smoke graph, validation scenarios | Continue dispatch validation (Titan V, K80), exercise `sovereign-dispatch` on warm GPUs |

---

## References

- `ecoPrimal/src/coordination/mod.rs` — `AtomicType::Node`
- `ecoPrimal/src/composition/routing.rs` — compute/tensor/shader routing
- `graphs/fragments/node_atomic.toml` — Node atomic deploy graph
- `config/capability_registry.toml` — compute/tensor/shader method registrations
- `infra/wateringHole/handoffs/SOVEREIGN_COMPUTE_THREE_GPU_WARM_CATCH_HANDOFF_MAY11_2026.md`
- `springs/hotSpring/wateringHole/handoffs/HOTSPRING_SOVEREIGN_RUST_EVOLUTION_HANDOFF_MAY11_2026.md`
- `whitePaper/gen4/architecture/COMPOSITION_PATTERNS.md` — PrimalBridge degradation tiers
