# primalSpring v0.4.0 → toadStool/barraCuda Evolution Handoff

**Date:** March 21, 2026
**From:** primalSpring v0.4.0
**To:** toadStool team, barraCuda team
**Supersedes:** PRIMALSPRING_V030_TOADSTOOL_BARRACUDA_CAPABILITY_HANDOFF_MAR18_2026.md
**License:** AGPL-3.0-or-later

---

## Executive Summary

primalSpring v0.4.0 achieved **Tower STABLE** (24/24 gates) and validated a
**Tower + Squirrel AI composition** with live Anthropic Claude inference. This
handoff documents what the toadStool and barraCuda teams should absorb from this
sprint, what coordination infrastructure is now available, and what the evolution
path looks like for Node Atomic (Tower + toadStool compute).

The key takeaway: primalSpring has proven the full composition lifecycle —
binary discovery, socket nucleation, topological startup, capability-based
health validation, and multi-primal coordination with real IPC — all works at
the Tower level. Node Atomic is next, and toadStool is the critical addition.

---

## 1. What primalSpring Now Provides

### 1.1 Live Atomic Harness (Stable)

The `AtomicHarness` + `RunningAtomic` framework is production-proven:

- **Binary discovery**: resolves binaries from `plasmidBin/primals/`, `PATH`, or build artifacts
- **Socket nucleation**: deterministic `{base}/{family_id}/{primal}.sock` paths
- **Topological startup**: `topological_waves()` (Kahn's algorithm) computes startup ordering from deploy graph DAGs
- **Health gating**: each primal's `health.liveness` must respond before the next wave starts
- **RAII lifecycle**: `RunningAtomic` kills all child processes on drop
- **Capability-based lookups**: `socket_for("compute")`, `client_for("compute")` resolve providers at runtime

**For toadStool**: this harness will spawn toadStool alongside beardog and songbird when Node Atomic testing begins. toadStool needs `health.liveness` and `capabilities.list` RPC methods to integrate.

### 1.2 `PrimalProcess::from_parts()`

New constructor allows custom spawn logic for primals with non-standard startup:

```rust
PrimalProcess::from_parts(name, socket_path, child)
```

**For toadStool**: if toadStool needs GPU device probing, WGPU backend selection, or other custom initialization before becoming ready, `from_parts()` supports wrapping that logic while still integrating with the harness lifecycle.

### 1.3 `LaunchProfile::passthrough_env`

New data-driven environment forwarding from `primal_launch_profiles.toml`:

```toml
[profiles.toadstool.passthrough_env]
CUDA_VISIBLE_DEVICES = true
WGPU_BACKEND = true
WGPU_ADAPTER_NAME = true
```

This securely forwards parent environment variables to child processes without `unsafe` global `set_var`. Already used for Squirrel's `ANTHROPIC_API_KEY` forwarding.

**For toadStool**: GPU-specific environment variables (device selection, backend, adapter) should be declared in the passthrough table so primalSpring tests can control GPU behavior without hardcoding.

### 1.4 Deploy Graph Conventions (Unchanged + Validated)

All 11 deploy graphs have `by_capability` on every node, enforced by test.
toadStool appears with `by_capability = "compute"` in 7 graphs. The convention is now proven end-to-end with live primals (exp060 uses `neural-api-server` to execute the Tower bootstrap graph).

### 1.5 25 Registered Capabilities

The capability registry (`config/capability_registry.toml`) now includes 25 capabilities. New since v0.3.0:

| Capability | Provider | Description |
|---|---|---|
| `ai.query` | squirrel | Route AI inference queries |
| `ai.health` | squirrel | AI provider health check |
| `composition.tower_squirrel_health` | squirrel | Composition health for Tower+Squirrel |

**For toadStool**: when toadStool registers its compute capabilities, they should follow this same registry pattern with `by_capability` provider declarations.

---

## 2. What toadStool Should Absorb

### P0: Standard RPC Methods

toadStool currently lacks the two ecosystem-standard methods:

| Method | Purpose | Reference Implementation |
|---|---|---|
| `health.liveness` | Health check returning `{"status": "alive"}` | Squirrel (gold standard) |
| `capabilities.list` | Return capability array in Format A, B, C, or D | Squirrel, primalSpring |

These are required for integration with the AtomicHarness health gate.

### P0: 5-Tier Socket Discovery

toadStool already has 5-tier socket discovery (confirmed in NUCLEUS audit). Continue following this pattern:

```
Tier 1: $TOADSTOOL_SOCKET environment variable
Tier 2: $XDG_RUNTIME_DIR/biomeos/{family_id}/toadstool.sock
Tier 3: /tmp/biomeos/{family_id}/toadstool.sock
Tier 4: Manifest file lookup
Tier 5: Socket registry
```

### P1: Neural API Capability Registration

When biomeOS's graph executor spawns toadStool, it should auto-register toadStool's capabilities with the Neural API's capability registry. The pattern from the Tower sprint:

1. biomeOS spawns toadStool
2. biomeOS calls `capabilities.list` on toadStool
3. biomeOS registers each capability in the Neural API's runtime registry
4. Other primals discover toadStool via `capability.call("compute", ...)`

### P1: `passthrough_env` for GPU Configuration

Add GPU-specific environment variables to the launch profile:

```toml
[profiles.toadstool.passthrough_env]
CUDA_VISIBLE_DEVICES = true
WGPU_BACKEND = true
WGPU_ADAPTER_NAME = true
VK_ICD_FILENAMES = true
```

This enables primalSpring integration tests to control which GPU adapter toadStool uses without hardcoding device indices.

### P2: Deploy Graph Node Participation

Verify toadStool appears correctly in all graphs where it has `by_capability = "compute"`:

| Graph | toadStool Wave | Dependencies |
|---|---|---|
| `node_atomic_compute.toml` | Wave 2 | beardog, songbird |
| `nucleus_complete.toml` | Wave 3 | beardog, songbird |
| `conditional_fallback.toml` | Wave 2 | beardog, songbird |
| `parallel_capability_burst.toml` | Wave 2 | beardog |
| `coralforge_pipeline.toml` | Wave 3 | beardog, songbird, squirrel |

---

## 3. What barraCuda Should Absorb

### 3.1 Zero Direct Dependency (Still Correct)

primalSpring has zero barraCuda imports and will not contribute math or shaders. The relationship remains indirect: primalSpring validates the coordination layer, toadStool dispatches to barraCuda for compute.

### 3.2 MCP Tool Surface Area

The AI composition pattern (exp061) demonstrated:

```
User → Squirrel → ai.query → Claude API → response
```

The future compute pattern extends this:

```
User → Squirrel → compute.dispatch → Neural API → toadStool → barraCuda → result
```

barraCuda's ops should be surfaced as MCP tools through toadStool, so Squirrel AI can discover and invoke compute capabilities naturally.

### 3.3 Provenance for Compute Results

The `Provenance { source, baseline_date, description }` struct is available for
tagging compute results with lineage. When barraCuda produces a numerical result,
it should carry provenance (which shader, which backend, which precision mode).

---

## 4. Learnings from the Tower + Squirrel Sprint

These are cross-cutting insights relevant to all primal teams:

### 4.1 Abstract vs. Filesystem Sockets

Squirrel's `UniversalListener` binds to Linux abstract sockets (`\0squirrel`)
by default, ignoring the `--socket` CLI argument. This created friction in
integration testing because:

- Abstract sockets don't appear in the filesystem (can't poll with `Path::exists()`)
- Only one instance can bind `\0squirrel` (singleton semantics)
- Parallel test runs conflict on the same abstract name

**Recommendation for toadStool**: use **filesystem sockets** that respect the
`--socket` argument and the 5-tier discovery convention. Abstract sockets are
appropriate for single-instance system services but problematic for
multi-instance testing.

### 4.2 TCP Port Conflicts in Parallel Tests

Songbird's HTTP listener (default port 8080-8090) caused "Address in use" errors
when multiple test instances ran in parallel. Squirrel had the same issue with its
default service mesh port (9010).

**Recommendation**: all primals that bind TCP ports should accept `--port 0` for
ephemeral port binding and expose the actual port via their `health.liveness` or
`capabilities.list` response. Environment variable overrides (e.g.,
`SERVICE_MESH_PORT=0`) are also valuable.

### 4.3 `passthrough_env` Pattern

Globally setting environment variables (`std::env::set_var`) is `unsafe` in Rust 2024
and creates races in parallel tests. The `passthrough_env` pattern in
`primal_launch_profiles.toml` provides a safe, data-driven alternative.

**For toadStool**: if toadStool needs `WGPU_BACKEND`, `CUDA_VISIBLE_DEVICES`, or
similar GPU config, declare them in `passthrough_env` rather than expecting the
parent test process to globally `set_var`.

### 4.4 Health-Gated Startup

The `AtomicHarness` waits for each primal's `health.liveness` before proceeding
to the next topological wave. Primals that start slowly (GPU initialization,
shader compilation) should ensure `health.liveness` responds only when truly ready.
The harness has a configurable 30-second timeout per wave.

---

## 5. Evolution Path: Tower → Node Atomic

```
Tower Atomic (beardog + songbird + biomeOS)     ← STABLE (24/24 gates)
    ▼
Node Atomic (Tower + toadStool)                 ← NEXT
    Requires: toadStool health.liveness + capabilities.list
    Requires: compute capability registration in Neural API
    Validates: exp002_node_atomic with live primals
    New gates: compute dispatch, GPU health, shader availability
    ▼
Nest Atomic (Node + nestgate)
    ▼
Full NUCLEUS (all primals)
```

### Node Atomic Gate Candidates

| Gate | Description | Validated By |
|---|---|---|
| N1 | toadStool starts and responds to `health.liveness` | Integration test |
| N2 | toadStool's `capabilities.list` returns `compute` domain | Integration test |
| N3 | Neural API discovers toadStool via `by_capability = "compute"` | exp002 |
| N4 | `compute.dispatch` routes through Neural API to toadStool | New experiment |
| N5 | toadStool → barraCuda delegation works via JSON-RPC | New experiment |
| N6 | GPU device enumeration via toadStool | New experiment |

---

## 6. Metrics

| Metric | Value |
|--------|-------|
| primalSpring version | v0.4.0 |
| Total tests | 264 (239 unit + 23 integration + 2 doc-tests) |
| Integration tests (live) | 15 ignored (require plasmidBin binaries) |
| Experiments | 40 (8 tracks) |
| Deploy graphs with toadStool | 7 |
| Registered capabilities | 25 |
| Tower Atomic gates | 24/24 STABLE |
| Node Atomic gates | 0/6 (not yet started) |

---

## 7. References

- `experiments/exp050_compute_triangle/` — compute coordination template for toadStool + barraCuda + coralReef
- `experiments/exp002_node_atomic/` — Node Atomic composition (capability-based, not yet live-validated)
- `experiments/exp060_biomeos_tower_deploy/` — reference for biomeOS graph-driven deployment
- `experiments/exp061_squirrel_ai_composition/` — reference for multi-primal composition with env passthrough
- `config/primal_launch_profiles.toml` — `passthrough_env` convention
- `config/capability_registry.toml` — 25 capabilities, `by_capability` provider declarations
- `specs/TOWER_STABILITY.md` — 24/24 gate definitions and evidence
- `specs/BARRACUDA_REQUIREMENTS.md` — confirms zero direct dependency

---

**License**: AGPL-3.0-or-later
