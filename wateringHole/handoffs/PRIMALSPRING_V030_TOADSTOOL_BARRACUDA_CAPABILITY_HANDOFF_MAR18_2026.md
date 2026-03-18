# primalSpring v0.3.0 → toadStool/barraCuda Capability-First Handoff

**Date:** March 18, 2026
**From:** primalSpring v0.3.0-dev
**To:** toadStool team, barraCuda team
**Supersedes:** PRIMALSPRING_V030_TOADSTOOL_BARRACUDA_EVOLUTION_HANDOFF_MAR18_2026.md (archived)
**License:** AGPL-3.0-or-later

---

## Executive Summary

primalSpring has completed its migration to capability-first architecture. This
handoff documents the evolution's impact on toadStool/barraCuda and what
coordination intelligence is available for absorption.

Key change: primalSpring no longer hardcodes `"toadstool"` or `"barracuda"` in
its RPC handlers. Instead, it discovers **whoever provides the `compute`
capability** at runtime. Deploy graphs declare `by_capability = "compute"` on the
toadStool node — biomeOS resolves the provider. This is the loose coupling path
all primals should follow.

---

## 1. What Changed for toadStool

### 1.1 Capability-Based Discovery

primalSpring's coordination handlers no longer call `discover_primal("toadstool")`.
They call `discover_by_capability("compute")`. This means:

- toadStool does NOT need to be named "toadstool" to be discovered
- Any primal advertising the `compute` capability will be found
- The Neural API, capability-named sockets (`compute.sock`), and socket registry
  are all valid discovery paths

**Action**: toadStool should register its `compute` capability in its manifest
and/or the socket registry so primalSpring (and all other springs) can find it
by capability.

### 1.2 Deploy Graph Convention

All 11 primalSpring deploy graphs now have `by_capability` on every node.
toadStool appears with `by_capability = "compute"` in:

| Graph | Role | Order |
|-------|------|-------|
| `primalspring_deploy.toml` | Full NUCLEUS compute | 7 |
| `node_atomic_compute.toml` | Node = Tower + compute | 3 |
| `nucleus_complete.toml` | All capabilities | 7 |
| `coralforge_pipeline.toml` | Pipeline GPU dispatch | 6 |
| `conditional_fallback.toml` | Primary compute (GPU) | 3 |
| `continuous_tick.toml` | Health poll target | 7 |
| `parallel_capability_burst.toml` | Parallel compute burst | 4 |

### 1.3 Topological Waves

`topological_waves()` computes startup ordering from graph dependencies.
toadStool always starts AFTER beardog (security) and songbird (discovery),
typically in wave 2 or 3. This is now validated at test time for all graphs.

### 1.4 `graph.capabilities` Endpoint

New `graph.capabilities` RPC returns the capabilities required by a deploy graph.
toadStool can query this to understand what a composition needs without parsing
TOML directly.

---

## 2. What Changed for barraCuda

### 2.1 Zero Direct Dependency (unchanged)

primalSpring has zero barraCuda imports. This remains correct. The relationship
is indirect: primalSpring validates the coordination layer through which springs
invoke compute dispatch, which toadStool routes to barraCuda.

### 2.2 Phase-Aware IPC Errors

New `IpcErrorPhase` enum (Connect, Serialize, Send, Receive, Parse) and
`PhasedIpcError` struct provide phase-aware error context. If barraCuda evolves
toward daemon-mode JSON-RPC, this error model should be adopted for structured
error reporting.

### 2.3 Cross-Spring MCP Tool Discovery

New `discover_remote_tools()` function enumerates MCP tools from a remote primal
via `mcp.tools.list`. This enables Squirrel AI to discover barraCuda's math
capabilities through toadStool's tool roster — the discovery chain is:

```
Squirrel → mcp.tools.list → toadStool → compute.dispatch → barraCuda
```

barraCuda's ops should be surfaced as MCP tools through toadStool.

---

## 3. Patterns Available for Absorption

### For toadStool

| Priority | Pattern | Where |
|----------|---------|-------|
| P1 | `by_capability = "compute"` convention | All deploy graphs |
| P1 | Manifest + socket-registry discovery (tiers 4-5) | `ipc/discover.rs` |
| P1 | MCP tool definitions with JSON Schema | `ipc/mcp.rs` |
| P2 | Phase-aware IPC errors | `ipc/error.rs` |
| P2 | `check_capability_health()` pattern | `coordination/mod.rs` |
| P3 | Topological wave validation | `deploy.rs` |

### For barraCuda

| Priority | Pattern | Where |
|----------|---------|-------|
| P2 | Provenance metadata on ops | `validation/mod.rs` (`Provenance`) |
| P2 | Phase-aware IPC errors | `ipc/error.rs` |
| P3 | `deny.toml` 14-crate C-dep ban | `deny.toml` |

---

## 4. Metrics

| Metric | Value |
|--------|-------|
| primalSpring tests | 236 |
| Deploy graphs with toadStool | 7 |
| All nodes have `by_capability` | Yes (enforced by test) |
| toadStool discovered by | `discover_by_capability("compute")` |
| barraCuda direct dependency | None (correct) |
| MCP tools exposed | 8 |
| RPC endpoints | 17 |

---

## 5. References

- `experiments/exp050_compute_triangle/` — compute triangle validation
- `experiments/exp002_node_atomic/` — Node Atomic (capability-based)
- `ecoPrimal/src/coordination/mod.rs` — `check_capability_health()`
- `ecoPrimal/src/deploy.rs` — `topological_waves()`, `graph_required_capabilities()`
- `ecoPrimal/src/ipc/error.rs` — `IpcErrorPhase`, `PhasedIpcError`
- `ecoPrimal/src/ipc/mcp.rs` — `discover_remote_tools()`
- `graphs/*.toml` — 11 deploy graphs, all `by_capability`

---

**License**: AGPL-3.0-or-later
