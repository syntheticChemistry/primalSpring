# primalSpring v0.3.0 → toadStool/barraCuda Evolution Handoff

**Date:** March 18, 2026
**From:** primalSpring v0.3.0-dev
**To:** toadStool team, barraCuda team
**Supersedes:** PRIMALSPRING_V020_TOADSTOOL_BARRACUDA_EVOLUTION_HANDOFF_MAR18_2026.md,
PRIMALSPRING_V020_TOADSTOOL_BARRACUDA_COMPUTE_TRIANGLE_HANDOFF_MAR18_2026.md
**License:** AGPL-3.0-or-later

---

## Executive Summary

primalSpring validates coordination — it does not consume barraCuda math directly.
Its relationship to toadStool and barraCuda is indirect: primalSpring validates
the coordination layer through which springs invoke toadStool compute dispatch and
barraCuda math primitives. This handoff documents what primalSpring has learned
about coordination patterns that are relevant to toadStool/barraCuda evolution.

---

## 1. primalSpring's Relationship to the Compute Triangle

```
coralReef (compile) → toadStool (dispatch) → barraCuda (math)
         ↑                    ↑                     ↑
    primalSpring validates the coordination between all three
```

primalSpring does NOT:
- Import barraCuda crates
- Execute WGSL shaders
- Call toadStool `compute.dispatch.*` methods directly

primalSpring DOES:
- Discover toadStool, coralReef, and barraCuda primals via IPC
- Probe their health (`health.liveness`, `health.readiness`)
- Parse their capabilities (4-format: A/B/C/D wire formats)
- Validate their participation in deploy graph compositions
- Expose MCP coordination tools that Squirrel AI uses to route compute requests

## 2. Coordination Intelligence for toadStool

### 2.1 Discovery Patterns

primalSpring's 5-tier discovery model is the reference implementation for finding
any primal socket. toadStool should align with these tiers:

1. `TOADSTOOL_SOCKET` env override (already supported)
2. `$XDG_RUNTIME_DIR/biomeos/toadstool-{family}.sock`
3. `{temp_dir}/biomeos/toadstool-{family}.sock`
4. **NEW**: Manifest: `$XDG_RUNTIME_DIR/ecoPrimals/manifests/toadstool.json`
5. **NEW**: Socket registry: `$XDG_RUNTIME_DIR/biomeos/socket-registry.json`

Tiers 4-5 absorbed from biomeOS v2.50 and Squirrel alpha.12. Recommend toadStool
absorb the same manifest/registry discovery for cross-primal coordination.

### 2.2 MCP Tool Pattern

primalSpring exposes 8 MCP tools via `mcp.tools.list` with JSON Schema inputs.
toadStool should expose its compute dispatch capabilities as MCP tools for Squirrel:

| Recommended Tool | Maps To |
|-----------------|---------|
| `toadstool_compute_dispatch` | `compute.dispatch.*` with workload routing |
| `toadstool_gpu_status` | GPU device inventory + health |
| `toadstool_shader_compile` | coralReef compile delegation |
| `toadstool_precision_route` | PrecisionBrain domain routing |
| `toadstool_pipeline_status` | Active pipeline monitoring |

Each tool should have a JSON Schema input definition following the ecosystem MCP pattern.

### 2.3 Capability Wire Format

primalSpring parses 4 capability response formats from the ecosystem. toadStool
currently uses Format A (string array). For richer Squirrel AI routing, consider
evolving to Format B (object array with descriptions) or C (method_info with
detailed metadata):

```
Format A: ["compute.dispatch.gemm", "compute.dispatch.variance", ...]
Format B: [{"name": "compute.dispatch.gemm", "description": "..."}, ...]
Format C: {"methods": [{"method": "compute.dispatch.gemm", "schema": {...}}]}
```

### 2.4 Health Probe Convention

primalSpring validates health via two Kubernetes-style probes:
- `health.liveness` — "are you running?" (boolean)
- `health.readiness` — "can you accept work?" (boolean + metadata)

toadStool already supports these. Ensure readiness includes GPU device availability,
active shader count, and pipeline state.

### 2.5 Deploy Graph Participation

primalSpring validates 6 biomeOS deploy graphs. toadStool appears in:
- `primalspring_deploy.toml` — Full NUCLEUS (order 4)
- `coralforge_pipeline.toml` — Pipeline (compute node)
- `conditional_fallback.toml` — GPU → CPU fallback (primary compute)

Ensure toadStool's binary name in deploy graphs matches its actual binary
(`toadstool_primal` or equivalent) and that `capabilities` listed in the graph
match its `capabilities.list` response.

## 3. Coordination Intelligence for barraCuda

### 3.1 Zero Direct Dependency

primalSpring has zero barraCuda imports. This is correct — primalSpring validates
coordination, not math. Springs that consume barraCuda math (wetSpring, hotSpring,
airSpring, groundSpring, healthSpring, ludoSpring, neuralSpring) coordinate through
toadStool dispatch. primalSpring validates that coordination.

### 3.2 IPC Resilience Patterns

primalSpring's IPC resilience stack (absorbed from 7 springs) is relevant to
barraCuda's daemon mode if/when it exposes JSON-RPC capabilities:

- `CircuitBreaker` (threshold: 3, timeout: 10s) — prevents cascading failures
- `RetryPolicy` (max: 2, base delay: 50ms, max delay: 500ms) — transient error recovery
- `resilient_call()` — combines circuit breaker + retry into a single ergonomic call

These constants are centralized in `tolerances/mod.rs` and should be ecosystem-wide defaults.

### 3.3 Structured Provenance

primalSpring's `Provenance { source, baseline_date, description }` on `ValidationResult`
models how springs trace their math back to Python baselines. barraCuda's ops should
carry provenance metadata (original paper, validated precision, tolerance justification)
that springs can propagate through their validation results.

## 4. Cross-Spring Compute Patterns Observed

primalSpring's 38 experiments cover coordination patterns that involve toadStool/barraCuda:

| Experiment | Pattern | toadStool/barraCuda Role |
|------------|---------|-------------------------|
| exp050 | Compute triangle | coralReef → toadStool → barraCuda pipeline |
| exp002 | Node Atomic | Tower + toadStool (compute layer) |
| exp004 | Full NUCLEUS | toadStool as compute node in full composition |
| exp005 | Atomic subtraction | Graceful degradation when toadStool absent |
| exp025 | coralForge pipeline | neuralSpring → toadStool → NestGate pipeline |
| exp053 | Multi-primal lifecycle | toadStool in 6-primal research paper lifecycle |

### Pattern: Graceful Degradation When Compute Unavailable

exp005 validates that the ecosystem degrades gracefully when toadStool is absent.
This is the pattern all springs should follow — `check_or_skip` when toadStool is
not reachable, never panic, never fake results.

### Pattern: Compute Triangle Discovery

exp050 discovers toadStool, coralReef, and barraCuda independently via IPC.
It probes each for health, checks latency (< 500ms), and validates at least
one capability. This is the coordination health check for the entire compute stack.

## 5. Recommended Actions

### For toadStool

| Priority | Action |
|----------|--------|
| P1 | Absorb manifest + socket-registry discovery (tiers 4-5) |
| P1 | Expose compute capabilities as MCP tools (5+ tools with JSON Schema) |
| P2 | Evolve capability response to Format B or C (richer metadata for Squirrel AI) |
| P2 | Ensure deploy graph `capabilities` match `capabilities.list` response |
| P3 | Add `deny.toml` if not already present (14-crate ecoBin ban) |

### For barraCuda

| Priority | Action |
|----------|--------|
| P2 | Add provenance metadata to ops (source paper, precision, tolerance) |
| P3 | Consider JSON-RPC daemon mode with IPC resilience patterns |
| P3 | Add `deny.toml` if not already present |

## 6. Metrics

| Metric | Value |
|--------|-------|
| primalSpring tests | 195 |
| primalSpring line coverage | 89.8% |
| Compute triangle experiments | 1 (exp050) |
| toadStool-involving experiments | 6 |
| barraCuda direct dependency | None (correct) |
| MCP tools exposed | 8 |
| Discovery tiers | 5 |

---

## 7. References

- `experiments/exp050_compute_triangle/` — compute triangle validation
- `ecoPrimal/src/coordination/mod.rs` — AtomicType definitions
- `ecoPrimal/src/ipc/mcp.rs` — MCP tool definitions
- `ecoPrimal/src/ipc/discover.rs` — 5-tier discovery implementation
- `config/capability_registry.toml` — capability registry
- `graphs/` — 6 deploy graph TOMLs

---

**License**: AGPL-3.0-or-later
