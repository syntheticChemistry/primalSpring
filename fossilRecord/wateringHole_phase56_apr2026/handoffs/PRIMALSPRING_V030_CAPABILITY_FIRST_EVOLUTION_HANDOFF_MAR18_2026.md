# primalSpring v0.3.0 — Capability-First Evolution Handoff

**Date:** March 18, 2026
**From:** primalSpring v0.3.0-dev
**To:** Ecosystem (biomeOS, all springs, all primals)
**Supersedes:** PRIMALSPRING_V030_EVOLUTION_HANDOFF_MAR18_2026.md (archived)
**License:** AGPL-3.0-or-later

---

## Executive Summary

primalSpring v0.3.0 completes the migration from identity-based coordination to
**capability-first architecture**. All RPC handlers, discovery sweeps, and core
experiments now default to capability-based resolution. Deploy graphs are the
source of truth for composition definitions — `by_capability` on every node,
`topological_waves()` for startup ordering, `graph_required_capabilities()` for
capability roster extraction. 236 tests (225 unit + 10 integration + 1 doc-test),
all green. Zero clippy warnings, zero unsafe, zero C deps.

---

## 1. What Changed

### 1.1 Capability-First RPC Handlers

All coordination handlers default to capability-based validation:

| Handler | Before | After |
|---------|--------|-------|
| `coordination.validate_composition` | `validate_composition()` (identity) | `validate_composition_by_capability()` (default) |
| `coordination.discovery_sweep` | `discover_for(primals)` | `discover_capabilities_for(caps)` |
| `coordination.deploy_atomic` | identity-based | capability-based |
| `coordination.bonding_test` | `discover_for(required_primals)` | `discover_capabilities_for(capabilities)` |
| `nucleus.start/stop` | `required_primals` in response | `required_capabilities` in response |
| `print_status` | primal names | capability domains + provider names |

Identity-based fallback retained via `mode: "identity"` parameter.

### 1.2 Topological Startup Waves

New `topological_waves()` in `deploy.rs` — Kahn's algorithm computes startup
wave ordering from deploy graph dependency edges:

- Wave 0: root nodes (no dependencies)
- Wave 1: nodes whose deps are all in wave 0
- ...and so on

Detects cycles, missing dependencies, validates all 11 graphs are acyclic.
New `graph.waves` RPC endpoint exposes this for biomeOS consumption.

### 1.3 Graphs as Source of Truth

- `graph_required_capabilities()` extracts capability roster from `by_capability` fields
- `validate_live_by_capability()` probes nodes using capability-first discovery
- `graph.capabilities` RPC endpoint returns graph-derived capability requirements
- All 11 deploy graph TOMLs have `by_capability` on every node (enforced by test)

### 1.4 New RPC Endpoints

| Method | Description |
|--------|-------------|
| `graph.waves` | Topological startup wave ordering from deploy graph |
| `graph.capabilities` | Required capabilities extracted from graph nodes |
| `coordination.probe_capability` | Probe a single capability provider |
| `coordination.validate_composition_by_capability` | Explicit capability-based validation |

### 1.5 Experiment Evolution

| Experiment | Before | After |
|------------|--------|-------|
| exp001 (Tower) | `discover_primal("beardog")` | `discover_by_capability("security")` |
| exp002 (Node) | `discover_primal("toadstool")` | `discover_by_capability("compute")` |
| exp003 (Nest) | `discover_primal("nestgate")` | `discover_by_capability("storage")` |
| exp004 (Full) | `probe_primal(name)` for each | `check_capability_health(cap)` for each |
| exp006 (Startup) | primal subset checks | `topological_waves()` from real graphs |
| exp051 (Sweep) | `discover_for(primals)` | `discover_capabilities_for(caps)` |

### 1.6 New Library Modules

- `coordination::check_capability_health()` — capability-based health probe helper
- `deploy::topological_waves()` — startup wave computation
- `deploy::graph_required_capabilities()` — graph capability extraction
- `deploy::validate_live_by_capability()` — capability-first live validation
- `ipc::error::IpcErrorPhase` / `PhasedIpcError` — phase-aware error context
- `ipc::mcp::discover_remote_tools()` — cross-spring MCP tool discovery

---

## 2. Metrics

| Metric | v0.2.0 | v0.3.0-dev |
|--------|--------|------------|
| Tests | 157 | **236** |
| Integration tests | 9 | **10** |
| Deploy graphs | 6 | **11** |
| Nodes with `by_capability` | partial | **100%** |
| Capability-based experiments | 0 | **6** |
| RPC endpoints | 11 | **17** |
| Clippy warnings | 0 | 0 |
| Unsafe code | forbid | forbid |

---

## 3. Patterns Available for Ecosystem Absorption

| Pattern | Where | Useful For |
|---------|-------|------------|
| Capability-first discovery + validation | `coordination/`, `ipc/discover.rs` | All primals and springs |
| Topological wave ordering (Kahn's) | `deploy.rs` | biomeOS graph executor |
| Graph-as-source-of-truth capabilities | `deploy.rs` | Any spring with deploy graphs |
| `by_capability` deploy graph convention | `graphs/*.toml` | All BYOB deploy graphs |
| `check_capability_health()` helper | `coordination/mod.rs` | Experiment harnesses |
| Phase-aware IPC errors | `ipc/error.rs` | All IPC consumers |
| Cross-spring MCP tool discovery | `ipc/mcp.rs` | Squirrel AI, any MCP consumer |
| Identity → capability migration pattern | `main.rs` handlers | Any spring with hardcoded primal names |

---

## 4. What Blocks Phase 4

| Blocker | Severity | Notes |
|---------|----------|-------|
| Live NUCLEUS deployment | P0 | Capability providers must actually be running |
| biomeOS graph executor | P1 | Topological waves computed but not executed |
| Beacon coordination validation | P2 | generate → encrypt → exchange → decrypt chain |
| Remaining experiment migration | P3 | 32 experiments still identity-based where applicable |

---

**License**: AGPL-3.0-or-later
