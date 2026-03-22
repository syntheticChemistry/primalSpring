# primalSpring — Ecosystem Leverage Guide

**Date**: March 22, 2026
**Version**: v0.7.0
**License**: AGPL-3.0-or-later

---

## What primalSpring Offers

primalSpring validates coordination itself. Its leverage surface is the
patterns, harnesses, and IPC infrastructure that all springs and primals
can absorb or compose against.

### Standalone Use

| Capability | How | When |
|------------|-----|------|
| **Coordination validation** | `cargo run --bin primalspring_primal -- server` | Validate atomic compositions live |
| **Deploy graph validation** | `graph.list` / `graph.validate` RPC | CI: ensure deploy TOMLs are structurally valid |
| **Health probing** | `health.liveness` / `health.readiness` | Kubernetes-style health checks |
| **MCP tool discovery** | `mcp.tools.list` | Squirrel AI routes coordination requests |
| **Meta-validation** | `cargo run --bin validate_all` | Run all 49 experiments in sequence |

### Library Patterns to Absorb

| Pattern | Module | Description |
|---------|--------|-------------|
| **ValidationResult** | `validation/mod.rs` | `check_bool`, `check_skip`, `check_or_skip`, `check_latency`, `check_minimum`, `check_count` + `finish()` + `exit_code()` |
| **Structured Provenance** | `validation/mod.rs` | `Provenance { source, baseline_date, description }` on validation results |
| **OrExit** | `validation/or_exit.rs` | `.or_exit("reason")` for zero-panic binary startup |
| **ValidationSink** | `validation/mod.rs` | `StdoutSink`, `NullSink` for pluggable output |
| **IPC resilience** | `ipc/resilience.rs` | `CircuitBreaker`, `RetryPolicy`, `resilient_call()` |
| **IpcError** | `ipc/error.rs` | 8 typed variants with `is_retriable()`, `is_connection_error()`, etc. |
| **IpcErrorPhase** | `ipc/error.rs` | Phase-aware error context: Connect/Serialize/Send/Receive/Parse |
| **PhasedIpcError** | `ipc/error.rs` | `error.in_phase(IpcErrorPhase::Receive)` for diagnostics |
| **DispatchOutcome** | `ipc/dispatch.rs` | Three-way: `Success(T)`, `ProtocolError`, `ApplicationError` |
| **extract_rpc_result** | `ipc/extract.rs` | Centralized JSON-RPC result extraction with typed errors |
| **4-format capability parsing** | `ipc/discover.rs` | Handles Format A/B/C/D wire formats from any primal |
| **5-tier discovery** | `ipc/discover.rs` | env/XDG/temp/manifest/socket-registry + Neural API |
| **MCP tool definitions** | `ipc/mcp.rs` | `McpTool { name, description, input_schema }` with JSON Schema |
| **Spring tool discovery** | `ipc/mcp.rs` | `discover_remote_tools(socket, primal)` to find other springs' MCP tools |
| **Safe cast** | `cast.rs` | Saturating numeric casts (`usize_u32`, `u64_usize`, `micros_u64`) |
| **Named tolerances** | `tolerances/mod.rs` | All latency/throughput bounds as named constants with provenance |
| **Capability registry** | `config/capability_registry.toml` | Single source of truth, sync-tested against code |

### Composition Patterns

| Layer | What primalSpring Validates |
|-------|-----------------------------|
| **Tower Atomic** | BearDog + Songbird discover, health, capabilities |
| **Node Atomic** | Tower + ToadStool compute |
| **Nest Atomic** | Tower + NestGate storage |
| **Full NUCLEUS** | All primals + Squirrel |
| **Graph Execution** | 5 coordination patterns (Sequential, Parallel, DAG, Pipeline, Continuous) |
| **Emergent Systems** | RootPulse, RPGPT, coralForge pipeline |
| **Bonding** | Covalent, Ionic, Plasmodium multi-gate |
| **Cross-Spring** | Data flow, provenance trio, fieldMouse, petalTongue, Squirrel AI |

### What primalSpring Absorbs From

| Source | What | Where in primalSpring |
|--------|------|----------------------|
| hotSpring | Provenance patterns, tolerance structure | `tolerances/`, `validation/mod.rs` |
| wetSpring | IPC resilience stack, cast module, MCP tools | `ipc/`, `cast.rs`, `ipc/mcp.rs` |
| airSpring | deny.toml, ecoBin enforcement | `deny.toml` |
| groundSpring | ValidationSink, typed errors, OrExit | `validation/`, `ipc/error.rs` |
| neuralSpring | Capability registry TOML, discovery module | `config/`, `ipc/discover.rs` |
| healthSpring | Proptest IPC fuzz, MCP tools, provenance registry | `ipc/extract.rs`, `ipc/mcp.rs` |
| ludoSpring | with_provenance(), #[expect(reason)] | `validation/mod.rs`, `Cargo.toml` |
| biomeOS v2.51 | IpcErrorPhase, manifest discovery, socket registry | `ipc/error.rs`, `ipc/discover.rs` |
| Squirrel alpha.13 | Spring tool discovery, socket registry | `ipc/mcp.rs`, `ipc/discover.rs` |

---

## How to Compose with primalSpring

### As a Primal Consumer (springs, primals)

1. Discover primalSpring: `discover_primal("primalspring")`
2. Check health: `health.liveness` → `health.readiness`
3. Request coordination: `coordination.validate_composition { "atomic": "Tower" }`
4. List deploy graphs: `graph.list`
5. Validate a graph: `graph.validate { "path": "...", "live": true }`

### As a Squirrel AI Consumer

1. Discover tools: `mcp.tools.list` → 8 typed tools with JSON Schema
2. Route requests: tool name maps 1:1 to JSON-RPC methods
3. Use `tool_to_method()` for name resolution

### As a biomeOS Graph Node

primalSpring ships 18 deploy graph TOMLs (all nodes declare `by_capability`).
biomeOS orchestrates the niche directly from these graphs. `topological_waves()`
computes startup ordering. primalSpring participates as a validator node that
probes other nodes by capability and reports composition health.

---

**License**: AGPL-3.0-or-later
