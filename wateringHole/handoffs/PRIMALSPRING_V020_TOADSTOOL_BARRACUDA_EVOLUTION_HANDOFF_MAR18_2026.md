# primalSpring v0.2.0 → toadStool/barraCuda Evolution Handoff

**Date:** 2026-03-18
**From:** primalSpring v0.2.0 (38 experiments, 157 tests, 2 binaries)
**To:** toadStool team, barraCuda team
**License:** AGPL-3.0-or-later

---

## Executive Summary

primalSpring v0.2.0 has completed its Phase 2 audit and evolution. This handoff
documents what primalSpring learned about the coordination layer that is relevant
to toadStool/barraCuda evolution, including IPC patterns for upstream absorption,
capability exposure requirements, niche modeling for biomeOS BYOB consumption,
and deploy graph validation results.

primalSpring has **zero** barraCuda code dependencies — its relationship is
IPC-only, validating coordination rather than math. The value to the toadStool/
barraCuda team is coordination intelligence: what patterns emerged across 7+
sibling springs that toadStool should absorb, and what primalSpring's niche
model tells biomeOS about how to compose primals.

---

## 1. IPC Patterns for Upstream Absorption

### 1.1 Typed `IpcError` (Converged from 7 Springs)

primalSpring absorbed and documented the converged `IpcError` pattern from
wetSpring, healthSpring, groundSpring, airSpring, neuralSpring, ludoSpring,
and hotSpring. All springs now use 8 typed variants with query methods:

```rust
pub enum IpcError {
    SocketNotFound(String),
    ConnectionRefused(String),
    ConnectionReset(String),
    Timeout(String),
    ProtocolError(String),
    MethodNotFound(String),
    ApplicationError { code: i64, message: String },
    SerializationError(String),
}

impl IpcError {
    pub const fn is_retriable(&self) -> bool;
    pub fn is_timeout_likely(&self) -> bool;
    pub const fn is_method_not_found(&self) -> bool;
    pub const fn is_connection_error(&self) -> bool;
}
```

**toadStool action:** Adopt this enum (or equivalent) for toadStool's own IPC
error handling. Return standard JSON-RPC error codes (`-32601` method not found,
`-32602` invalid params, `-32603` internal error) so all downstream clients
classify correctly.

### 1.2 `DispatchOutcome<T>` (Converged from 3 Springs)

```rust
pub enum DispatchOutcome<T> {
    Success(T),
    Protocol(IpcError),
    Application { code: i64, message: String },
}
```

Enables three-way classification: transport failure (retry), application
rejection (report), success (proceed). Absorbed from loamSpine, airSpring,
healthSpring.

**toadStool action:** Use this pattern in toadStool's dispatch client. Separate
"toadStool is down" from "toadStool rejected the workload."

### 1.3 `CircuitBreaker` + `RetryPolicy`

primalSpring implements a full circuit breaker (closed → open → half-open) with
exponential backoff retry policy. All IPC calls to toadStool/barraCuda go through
`resilient_call()` which wraps both.

**toadStool action:** If toadStool has its own IPC clients (for barraCuda, coralReef),
adopt the same pattern for consistent resilience across the ecosystem.

### 1.4 4-Format Capability Parsing

primalSpring's `extract_capability_names()` handles all 4 wire formats discovered
across the ecosystem:

- **Format A** — `["compute.dispatch", "compute.status"]` (flat string array)
- **Format B** — `[{"method": "compute.dispatch"}]` (object array)
- **Format C** — `{"method_info": [{"name": "compute.dispatch"}]}` (nested)
- **Format D** — `{"semantic_mappings": {"compute": {"dispatch": {}}}}` (double-nested)

**toadStool action:** Any format works. Format A is simplest. The ecosystem handles
all four, so toadStool can choose based on what makes sense for its capability model.

### 1.5 Health Probes

| Method | Purpose | Expected Response |
|--------|---------|-------------------|
| `health.liveness` | Process alive? | `{ "healthy": true }` |
| `health.readiness` | Ready to serve? | `{ "ready": true, "details": { ... } }` |

**toadStool action:** Implement both. `health.readiness` should report GPU
availability, shader compilation status, loaded firmware state, and any blocking
conditions. biomeOS polls liveness cheaply, readiness for routing decisions.

---

## 2. Niche Self-Knowledge Model

primalSpring now exposes a structured niche model via `niche.rs` following the
pattern established by airSpring. This is consumed by biomeOS for BYOB (Build
Your Own Biome) scheduling.

### 2.1 Capabilities (21 total)

```
coordination.validate_composition    coordination.discovery_sweep
coordination.neural_api_status       coordination.probe_primal
health.check                         health.liveness
health.readiness                     capabilities.list
lifecycle.status                     graph.list
graph.validate                       graph.deploy
bonding.validate                     bonding.plasmodium_check
emergent.rootpulse_validate          emergent.rpgpt_session
emergent.coralforge_pipeline         emergent.cross_spring_ecology
tolerance.calibrate                  tolerance.list
niche.register
```

### 2.2 Semantic Mappings

```
coordination → [validate_composition, discovery_sweep, probe_primal, neural_api_status]
health       → [check, liveness, readiness]
graph        → [list, validate, deploy]
bonding      → [validate, plasmodium_check]
emergent     → [rootpulse_validate, rpgpt_session, coralforge_pipeline, cross_spring_ecology]
tolerance    → [calibrate, list]
lifecycle    → [status]
niche        → [register]
```

### 2.3 Operation Dependencies

Operations declare what they need from the ecosystem:

| Operation | Dependencies |
|-----------|-------------|
| `validate_composition` | `neural_api`, `target_primals_running` |
| `discovery_sweep` | `neural_api` |
| `graph.deploy` | `biomeos_executor`, `target_primals_running` |
| `rootpulse_validate` | `rhizocrypt`, `loamspine`, `sweetgrass` |
| `coralforge_pipeline` | `neuralspring`, `hotspring`, `wetspring`, `toadstool`, `nestgate` |

### 2.4 Cost Estimates

| Operation | Latency | CPU | Memory |
|-----------|---------|-----|--------|
| `health.liveness` | <1ms | minimal | <1KB |
| `validate_composition` | 50–500ms | low | <64KB |
| `graph.validate` | 100–2000ms | low | <256KB |
| `graph.deploy` | 1–30s | medium | <1MB |

**toadStool action:** Consider exposing a similar niche model. biomeOS needs to
know what toadStool can do, what it needs, and what it costs — for scheduling
graph execution across heterogeneous deployments.

---

## 3. Deploy Graph Validation

primalSpring ships 6 biomeOS deploy graph TOMLs and validates them both
structurally and (when primals are running) live:

| Graph | Pattern | Nodes | Structural | Live |
|-------|---------|-------|------------|------|
| `primalspring_deploy.toml` | Sequential | 9 | PASS | Requires NUCLEUS |
| `coralforge_pipeline.toml` | Pipeline | 7 | PASS | Requires 5 springs |
| `streaming_pipeline.toml` | Pipeline | 4 | PASS | Requires 3 primals |
| `continuous_tick.toml` | Continuous | 8 | PASS | Requires full mesh |
| `conditional_fallback.toml` | ConditionalDag | 4 | PASS | Requires toadStool |
| `parallel_capability_burst.toml` | Parallel | 4 | PASS | Requires 4 primals |

Structural validation checks: graph name, node binary names, dependency
references, node ordering, and pattern-specific constraints (e.g., continuous
graphs must have tick intervals).

**toadStool action:** `conditional_fallback.toml` defines a GPU → CPU fallback
pattern. If toadStool implements `compute.dispatch.status`, primalSpring can
validate the fallback path end-to-end.

---

## 4. barraCuda Relationship

primalSpring has **zero** barraCuda code dependencies. This is by design:

| Aspect | primalSpring | Domain Springs |
|--------|-------------|----------------|
| barraCuda import | None | Direct crate dependency |
| WGSL shaders | None | 767+ across ecosystem |
| Math primitives | None | stats, linalg, ops, spectral, nautilus |
| Compute dispatch | IPC via `probe_primal("barracuda")` | Direct `compute.dispatch.*` |
| Absorption candidates | None | Hundreds (Write → Absorb → Lean) |

primalSpring contributes coordination intelligence, not math. When primalSpring
validates that a NUCLEUS composition works, it is indirectly validating that
barraCuda's math primitives are accessible through the toadStool dispatch layer.

---

## 5. What primalSpring Needs from toadStool/barraCuda

### 5.1 High Priority (Blocks Phase 3)

1. **`health.liveness` + `health.readiness`** on toadStool and barraCuda — primary
   blocker for live primal integration in exp050 (Compute Triangle)
2. **`capabilities.list`** returning a parseable response (any of 4 formats)
3. **Standard socket paths** (`$TOADSTOOL_SOCKET` or `$XDG_RUNTIME_DIR/biomeos/toadstool.sock`)

### 5.2 Medium Priority

4. **Standard JSON-RPC error codes** (-32601, -32602, -32603)
5. **`compute.dispatch.status`** — for graph fallback path validation
6. **`FAMILY_ID` in socket paths** — for multi-tower isolation

### 5.3 Low Priority (Future)

7. **Shader compilation status** via `health.readiness` details
8. **Protocol escalation** (HTTP → JSON-RPC → tarpc) for exp052
9. **Niche model** — toadStool and barraCuda should expose their own capabilities,
   dependencies, and costs for biomeOS scheduling

---

## 6. Ecosystem Intelligence Gathered

### 6.1 Patterns Converging Across All Springs

| Pattern | Adopted By | Status |
|---------|-----------|--------|
| Typed `IpcError` | 8 springs | Converged |
| `DispatchOutcome<T>` | 4 springs | Converging |
| `health.liveness`/`health.readiness` | 6 springs | Standard |
| 4-format capability parsing | 5 springs | Standard |
| `PRIMAL_NAME`/`PRIMAL_DOMAIN` constants | All springs | Standard |
| `#[expect(reason)]` over `#[allow()]` | All springs | Standard |
| `OrExit<T>` for validation binaries | 5 springs | Converging |
| `safe_cast` module | 4 springs | Converging |
| Named tolerance constants | All domain springs | Standard |
| `ValidationSink` trait | 3 springs | Converging |
| proptest for IPC protocol | 3 springs | Emerging |

### 6.2 Domain Spring barraCuda Consumption (Current State)

Based on cross-spring survey from primalSpring v0.2.0 audit:

| Spring | barraCuda Primitives | GPU Modules | CPU Modules | Local WGSL | Phase |
|--------|---------------------|-------------|-------------|------------|-------|
| wetSpring V125 | 150+ | 42 | 41 | 0 | All Lean |
| hotSpring v0.6.31 | 85+ | 85 shaders | — | 0 | All Lean |
| airSpring v0.8.7 | 30+ | 25 | — | 6 (f64-canonical) | 3 local absorbing |
| groundSpring V110 | 60+ | 41 GPU | 61 CPU | 2 | 102 delegations |
| neuralSpring S166 | 40+ | — | — | 0 | All Lean |
| healthSpring V30 | 20+ | 6 shaders | — | 0 | 3 ODE→WGSL codegen |
| ludoSpring V22 | 3 (sigmoid, dot, lcg_step) | 11 shaders | — | 0 | Lean + dispatch |

### 6.3 airSpring Local WGSL Ready for Absorption

airSpring v0.8.7 has 6 f64-canonical local WGSL shaders compiled via
`compile_shader_universal()`, ready for toadStool absorption as ops 14–19:

| Op | Shader | Domain |
|----|--------|--------|
| 14 | SCS-CN runoff | Hydrology |
| 15 | Stewart yield | Crop science |
| 16 | Makkink ET₀ | Evapotranspiration (absorbed) |
| 17 | Turc ET₀ | Evapotranspiration (absorbed) |
| 18 | Hamon PET | Evapotranspiration (absorbed) |
| 19 | Blaney-Criddle | Evapotranspiration |

3 of 6 already absorbed upstream. Remaining 3 documented in airSpring's V051
handoff for toadStool.

---

## 7. Recommended Upstream Actions

### For toadStool Team

1. **Absorb `health.liveness`/`health.readiness`** as standard endpoints — all
   springs use them, biomeOS needs them for routing
2. **Absorb `DispatchOutcome<T>`** pattern into dispatch API for three-way error
   classification
3. **Absorb `IpcError` query helpers** for circuit-breaker logic
4. **Expose a niche model** — capabilities, dependencies, costs for biomeOS BYOB
   scheduling (follow primalSpring `niche.rs` / airSpring `niche.rs` pattern)
5. **Implement `compute.dispatch.status`** — enables conditional fallback graph
   validation (GPU → CPU path)
6. **Enforce `{PRIMAL_UPPER}_SOCKET` discovery convention** in PCIe topology routing
7. **Absorb remaining airSpring local WGSL** (ops 14–19) to complete the
   Write → Absorb → Lean cycle

### For barraCuda Team

1. **Stabilize `SparseGemmF64`, `TranseScoreF64`, `TopK`** — wetSpring Track 3 needs them
2. **Absorb Perlin 3D / fBm 3D** from ludoSpring (2D already absorbed)
3. **Consider `IpcError`-style enum** for barraCuda's own error types
4. **Consider niche model exposure** for biomeOS to understand math primitive availability

### For Ecosystem (biomeOS/sweetGrass)

1. **biomeOS:** Poll `health.liveness` for keep-alive, `health.readiness` for routing
2. **biomeOS:** Consume niche models from all primals for BYOB graph optimization
3. **sweetGrass:** Wire `IpcError::is_retriable()` into provenance trio circuit breaker
4. **All primals:** Converge on `DispatchOutcome` for toadStool dispatch clients

---

## 8. Metrics

| Metric | Value |
|--------|-------|
| Tests | 157 (148 unit + 9 integration) |
| Experiments | 38 (7 tracks) |
| Deploy graphs | 6 (all structurally validated) |
| Niche capabilities | 21 |
| IPC modules | 7 |
| Clippy warnings | 0 (pedantic + nursery) |
| Unsafe code | 0 (workspace forbid) |
| C dependencies | 0 |
| `#[allow()]` in production | 0 |
| `#[expect()]` with reason | 3 (safe cast boundaries) |

---

## References

- `primalSpring/ecoPrimal/src/niche.rs` — niche self-knowledge module
- `primalSpring/ecoPrimal/src/deploy.rs` — deploy graph validation
- `primalSpring/experiments/exp050_compute_triangle/` — Sovereign Compute Triangle validation
- `primalSpring/graphs/` — 6 biomeOS deploy graph TOMLs
- `primalSpring/specs/BARRACUDA_REQUIREMENTS.md` — barraCuda relationship (indirect only)
- `primalSpring/wateringHole/PRIMALSPRING_COMPOSITION_GUIDANCE.md` — composition patterns
- `wetSpring/wateringHole/handoffs/WETSPRING_V126_TOADSTOOL_BARRACUDA_EVOLUTION_HANDOFF_MAR16_2026.md` — sibling handoff
- `airSpring/wateringHole/handoffs/` — airSpring absorption handoffs (ops 14–19)
