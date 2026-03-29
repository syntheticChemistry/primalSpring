# primalSpring — Ecosystem Leverage Guide

**Date**: March 23, 2026
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
| **Meta-validation** | `cargo run --bin validate_all` | Run all 67 experiments in sequence |
| **Remote gate probe** | `./scripts/validate_remote_gate.sh <host>` | Per-primal TCP health check on any gate |
| **Musl build** | `./scripts/build_ecosystem_musl.sh` | Static x86_64+aarch64 binaries for deployment |
| **Spore prep** | `./scripts/prepare_spore_payload.sh <dir>` | USB payload assembly |

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
| **Primal display names** | `primal_names.rs` | `display_name()` / `discovery_slug()` round-trip (neuralSpring pattern) |
| **Skip-aware exit** | `validation/mod.rs` | `exit_code_skip_aware()`: 0=pass, 1=fail, 2=all-skipped (wetSpring pattern) |
| **Provenance resilience** | `ipc/provenance.rs` | Epoch-based circuit breaker + exponential backoff for trio calls |
| **Cross-cutting proptest** | `ipc/proptest_ipc.rs` | Pipeline-spanning property tests (healthSpring pattern) |
| **normalize_method()** | `ipc/mod.rs` | Ecosystem-wide JSON-RPC method normalization — strips legacy prefixes |
| **check_relative()** | `validation/mod.rs` | Relative-tolerance numeric validation (groundSpring/healthSpring pattern) |
| **check_abs_or_rel()** | `validation/mod.rs` | Combined absolute-or-relative tolerance (avoids false negatives near zero) |
| **NdjsonSink** | `validation/mod.rs` | Streaming NDJSON validation output for CI/log aggregation |
| **is_recoverable()** | `ipc/error.rs` | Broader recovery classification: retriable + server-recoverable errors |
| **Transport** | `ipc/transport.rs` | Unified Unix+Tcp transport with `connect_transport()` address parsing |
| **OnceLock probes** | `ipc/probes.rs` | Cached runtime resource probes for parallel test execution |
| **Release gate** | `scripts/validate_release.sh` | fmt + clippy + deny + test floor + docs CI gate |

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
| wetSpring | IPC resilience stack, cast module, MCP tools, skip_with_code | `ipc/`, `cast.rs`, `ipc/mcp.rs`, `validation/mod.rs` |
| airSpring | deny.toml merged bans, ecoBin enforcement, cast lints | `deny.toml`, `Cargo.toml` |
| groundSpring V120 | ValidationSink (section + write_summary), typed errors, OrExit, deny.toml merge | `validation/`, `ipc/error.rs`, `deny.toml` |
| neuralSpring S170 | Capability registry TOML, discovery module, primal_names::display, cast lint policy | `config/`, `ipc/discover.rs`, `primal_names.rs`, `Cargo.toml` |
| healthSpring V41 | Proptest IPC consolidated module, provenance circuit breaker, MCP tools | `ipc/proptest_ipc.rs`, `ipc/provenance.rs`, `ipc/mcp.rs` |
| ludoSpring V29 | with_provenance(), #[expect(reason)], XDG sockets | `validation/mod.rs`, `Cargo.toml` |
| biomeOS v2.66 | IpcErrorPhase, manifest discovery, socket registry, aligned 5-tier | `ipc/error.rs`, `ipc/discover.rs` |
| Squirrel alpha.21 | Spring tool discovery, socket registry | `ipc/mcp.rs`, `ipc/discover.rs` |

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

## Cross-Architecture Leverage

primalSpring is the reference implementation for cross-architecture deployment.
Every pattern above works identically on `x86_64` and `aarch64`.

### What primalSpring Proves

| Capability | Proven |
|------------|--------|
| `aarch64-unknown-linux-musl` cross-compile | 2.99 MB static, runs on Pixel 8a |
| Full workspace cross-compile (67 experiments + server) | 1.27s incremental |
| JSON-RPC coordination over abstract sockets | Same protocol, different transport |
| Zero architecture-specific code | All arch concerns in env/transport config |

### What Other Primals Should Absorb

1. **musl target**: `cargo build --release --target aarch64-unknown-linux-musl`
2. **Release profile**: `strip = true`, `lto = true` in `[profile.release]`
3. **Abstract socket support**: `@biomeos/{primal}` for Android deployment
4. **Env-first config**: `FAMILY_ID`, `NODE_ID`, `{PRIMAL}_SOCKET` — no filesystem assumptions

### Reference Documents

- `PRIMAL_CAPABILITY_STATUS_MAR22_2026.md` — per-primal open items and compliance
- `ECOBIN_GENOMEBIN_EVOLUTION_GUIDANCE_MAR22_2026.md` — ecoBin/genomeBin evolution roadmap
- `PRIMALSPRING_V070_HARDWARE_VALIDATION_HANDOFF_MAR22_2026.md` — full hardware audit

---

**License**: AGPL-3.0-or-later
