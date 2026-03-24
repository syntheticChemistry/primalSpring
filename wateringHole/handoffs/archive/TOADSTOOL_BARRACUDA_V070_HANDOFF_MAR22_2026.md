# toadStool / barraCuda — v0.7.0 Evolution Handoff

**Date:** March 22, 2026
**From:** primalSpring v0.7.0
**To:** toadStool team, barraCuda team
**Supersedes:** `archive/PRIMALSPRING_V040_TOADSTOOL_BARRACUDA_EVOLUTION_HANDOFF_MAR21_2026.md`
**License:** AGPL-3.0-or-later

---

## Executive Summary

primalSpring has validated toadStool through **3 composition tiers** and
**5 coordination patterns**. barraCuda remains **documentation + IPC probe
only** in primalSpring (zero WGSL, zero math — by design). This handoff
documents what primalSpring learned, what the toadStool team should absorb,
and how barraCuda budding should proceed.

---

## 1. What primalSpring Validated for toadStool

### Compositions toadStool Participates In (all PASS)

| Composition | Graph | Gate |
|-------------|-------|------|
| Node Atomic | `node_atomic_compute.toml` | Gates 14-15 (5/5) |
| Full NUCLEUS | `nucleus_complete.toml` | Gate 16 (4/4) |
| Parallel burst | `parallel_capability_burst.toml` | Gate 22 (exp011 live) |
| Conditional DAG | `conditional_fallback.toml` | Gate 22 (exp012 live) |
| Full Overlay | `full_overlay.toml` | Gate 21 (Squirrel discovery) |
| Node + AI | `node_ai.toml` | Gate 19 (overlay) |

### Integration Tests Exercising toadStool

- `node_atomic_live_health_check` — toadstool starts, socket nucleates
- `node_atomic_live_validation` — full Node tier validates
- `node_toadstool_health` — `toadstool.health` direct call
- `node_toadstool_capabilities` — `toadstool.query_capabilities` (4 workload types, 24 cores)
- `overlay_node_ai_spawn_order` — toadstool in graph overlay
- `overlay_node_ai_validation` — Node + Squirrel validates

### Experiments Exercising toadStool

| Experiment | What |
|-----------|------|
| exp011 | **Live** parallel burst: toadstool alongside beardog+songbird+nestgate |
| exp012 | **Live** conditional DAG: toadstool as GPU primary path, CPU fallback |
| exp050 | Compute triangle probe: toadstool + coralReef + barracuda (IPC only) |
| exp067 | Node Atomic validation (13/13 checks) |
| exp068 | Full NUCLEUS (16/16 checks) |

---

## 2. What primalSpring Learned About toadStool

### IPC Quirks the Harness Accommodates

| Issue | primalSpring Adaptation |
|-------|------------------------|
| **Dual sockets** | toadstool creates both tarpc (`.sock`) and JSON-RPC (`.jsonrpc.sock`) — harness uses `jsonrpc_socket_suffix` to connect to the right one |
| **Ignores `--socket`** | toadstool reads `TOADSTOOL_SOCKET` env var, not CLI flag — harness sets `socket_flag = "__skip__"` and uses `env_sockets` |
| **Prefixed methods** | `toadstool.health` not `health.liveness` — harness has 4-deep fallback chain |
| **Security warning** | Requires `TOADSTOOL_SECURITY_WARNING_ACKNOWLEDGED=1` env var |

### What Works Well

- JSON-RPC is clean and responsive
- `toadstool.query_capabilities` returns structured data (4 workload types, 24 CPU cores)
- Plays well in parallel compositions (no socket contention)
- Conditional DAG graph works: when toadstool is up, it's the primary path; when down, CPU fallback activates

### Recommendations for toadStool Team

1. **Add `health.liveness`** as an alias for `toadstool.health` — the ecosystem standard
2. **Add `capabilities.list`** as an alias for `toadstool.query_capabilities`
3. **Accept `--socket PATH`** CLI flag alongside `TOADSTOOL_SOCKET` env var
4. **Single socket mode**: option to serve JSON-RPC on the primary socket (skip tarpc)
   — simplifies primalSpring harness and other coordination consumers

---

## 3. barraCuda Status in primalSpring

### What Exists

- **`specs/BARRACUDA_REQUIREMENTS.md`** — defines the relationship (indirect only)
- **`exp050_compute_triangle`** — probes barracuda IPC socket if running
- **`plasmidBin/manifest.toml`** — declares barracuda primal with capabilities
- **Zero deploy graphs** reference barracuda by name (capability-based)

### What Does NOT Exist

- No barracuda binary in `plasmidBin/primals/`
- No barracuda Cargo dependency
- No WGSL or math code (by design — primalSpring validates coordination, not math)
- No barracuda launch profile (not yet needed)

### The Compute Triangle

```
coralReef (compile) → toadStool (dispatch) → barraCuda (execute)
```

primalSpring validates the **coordination pattern**, not the math. Currently:
- **toadStool**: fully integrated and validated in 6 compositions
- **coralReef**: referenced in exp050, awaiting binary
- **barraCuda**: referenced in exp050, awaiting binary + budding from toadStool

---

## 4. barraCuda Budding Path

Based on `phase1/toadstool/specs/BARRACUDA_PRIMAL_BUDDING.md`:

### Current State (toadStool code)

`phase1/toadstool/crates/server/src/pure_jsonrpc/handler/science/barracuda.rs`
exposes science capability metadata (activations, RNG, special functions) as
JSON-RPC handlers inside toadStool. This is the **budding surface**.

### What primalSpring Needs From barraCuda (When Ready)

1. **Binary** in `plasmidBin/primals/barracuda`
2. **Unix socket JSON-RPC** (line-delimited NDJSON)
3. **`health.liveness`** method
4. **`capability.list`** method returning at minimum:
   - `compute.gpu` — GPU compute dispatch
   - `compute.wgsl` — WGSL shader execution
   - `compute.tensor` — tensor operations
5. **Accept** `BARRACUDA_SOCKET` env var or `--socket` CLI flag

### primalSpring Will Then

- Add barracuda launch profile to `config/primal_launch_profiles.toml`
- Create `graphs/compute_triangle.toml` (coralReef → toadStool → barraCuda)
- Create exp071+ for compute triangle validation
- Wire into Squirrel's `compute.*` capabilities
- Add integration tests

---

## 5. Deploy Graphs Relevant to toadStool

| Graph | toadStool Role |
|-------|---------------|
| `node_atomic_compute.toml` | Primary compute node |
| `nucleus_complete.toml` | Part of full NUCLEUS |
| `parallel_capability_burst.toml` | Parallel health target |
| `conditional_fallback.toml` | GPU primary path (fallback: CPU) |
| `full_overlay.toml` | Part of full overlay with Squirrel |
| `node_ai.toml` | Node tier + Squirrel AI |
| `continuous_tick.toml` | 60Hz health target (awaiting provenance trio) |
| `coralforge_pipeline.toml` | Pipeline stage |

---

## 6. Reference Material

| Document | What It Shows |
|----------|--------------|
| `specs/TOWER_STABILITY.md` | 87/87 gates, toadStool in Gates 14-15, 19, 22 |
| `specs/BARRACUDA_REQUIREMENTS.md` | barraCuda relationship (indirect only) |
| `config/primal_launch_profiles.toml` | toadStool launch profile with env_sockets |
| `ecoPrimal/tests/server_ecosystem_compose.rs` | 6 tests exercising toadStool |
| `experiments/exp050_compute_triangle/` | Compute triangle IPC probe |
| `experiments/exp067_node_atomic/` | Node Atomic validation (13/13) |

---

**License**: AGPL-3.0-or-later
