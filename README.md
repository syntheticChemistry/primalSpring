# primalSpring

**Coordination and composition validation spring for the ecoPrimals ecosystem.**

| | |
|-|-|
| **Domain** | Primal coordination, atomic composition, graph execution, emergent systems, bonding |
| **Version** | 0.3.0 (unreleased) |
| **Edition** | Rust 2024 (1.87+) |
| **License** | AGPL-3.0-or-later |
| **Tests** | 236 (225 unit + 10 integration + 1 doc-test) |
| **Experiments** | 38 (7 tracks) |
| **Unsafe** | Workspace-level `forbid` via `[workspace.lints.rust]` |
| **C deps** | Zero (ecoBin compliant, `deny.toml` enforced) |

---

## What Is primalSpring?

primalSpring is the spring whose domain IS coordination. Where other springs
validate domain science (hotSpring validates physics, wetSpring validates
biology), primalSpring validates the ecosystem itself — the coordination,
composition, and emergent behavior that biomeOS and the Neural API produce
when primals work together.

Its "papers" are the atomics. Its "experiments" are composition patterns.
Its validation target is biomeOS itself.

## Architecture

```
primalSpring/
├── ecoPrimal/                     # Library crate + UniBin binaries
│   ├── src/
│   │   ├── lib.rs                 # Library root (unsafe_code = forbid)
│   │   ├── cast.rs                # Safe numeric casts (saturating boundary)
│   │   ├── coordination/          # Atomic composition, health probing, composition validation
│   │   ├── deploy.rs              # Deploy graph parsing, structural + live validation
│   │   ├── graphs/                # Graph execution pattern types (5 patterns)
│   │   ├── emergent/              # Emergent system validation (RootPulse, RPGPT, CoralForge)
│   │   ├── bonding/               # Multi-gate bonding models (Covalent, Ionic, Weak, OMS)
│   │   ├── ipc/                   # JSON-RPC 2.0 client, discovery, error, dispatch, extract, resilience
│   │   ├── niche.rs               # BYOB niche self-knowledge (capabilities, semantic mappings, registration)
│   │   ├── validation/            # Experiment harness (check_bool, check_skip, OrExit, ValidationSink)
│   │   └── tolerances/            # Named latency and throughput bounds
│   ├── src/bin/
│   │   ├── primalspring_primal/   # UniBin: JSON-RPC 2.0 server with niche registration
│   │   └── validate_all/          # Meta-validator: runs all 38 experiments
│   └── tests/
│       └── server_integration.rs  # 10 real IPC round-trip tests
├── experiments/                   # 38 validation experiments (7 tracks)
├── graphs/                        # 11 biomeOS deploy graph TOMLs (all by_capability)
├── niches/                        # BYOB niche deployment YAML
├── specs/                         # Architecture specs
└── wateringHole/                  # Docs and handoffs
```

## Key Design Principles

- **Capability-first discovery**: No hardcoded primal rosters. Discovery is
  capability-based (via `discover_by_capability()`) or Neural API-driven
  (via `neural-api-client-sync`). Deploy graphs use `by_capability` for
  loose coupling — callers ask for capabilities, not primal identities.
- **Graphs as source of truth**: Deploy graph TOMLs define what capabilities
  a composition needs. `topological_waves()` computes startup ordering from
  dependency edges. `graph_required_capabilities()` extracts the capability
  roster from graph nodes.
- **Graceful degradation**: Experiments honestly skip checks when providers
  aren't running — never fake a pass.
- **Zero domain knowledge**: primalSpring validates coordination, not math.
  Zero barraCuda, zero WGSL, zero domain science.
- **BYOB niche model**: Full self-knowledge via `niche.rs` — capabilities,
  semantic mappings, operation dependencies, cost estimates, and runtime
  registration with biomeOS.

## Quick Start

```bash
# Build everything
cargo build --workspace

# Run all 236 tests
cargo test --workspace

# Run all 38 experiments (meta-validator)
cargo run --release --bin validate_all

# Start the primalSpring JSON-RPC server
cargo run --bin primalspring_primal -- server

# Show ecosystem status
cargo run --bin primalspring_primal -- status
```

## Server Mode

The `primalspring_primal` binary exposes coordination capabilities via JSON-RPC 2.0:

| Method | Description |
|--------|-------------|
| `health.check` | Self health status |
| `health.liveness` | Kubernetes-style liveness probe |
| `health.readiness` | Readiness probe (Neural API + discovered primals) |
| `capabilities.list` | Niche capabilities + semantic mappings + cost estimates |
| `coordination.validate_composition` | Validate an atomic composition (capability-based by default) |
| `coordination.validate_composition_by_capability` | Explicitly capability-based validation |
| `coordination.discovery_sweep` | Enumerate capabilities in a composition |
| `coordination.probe_capability` | Probe a single capability provider |
| `coordination.neural_api_status` | Neural API reachability |
| `graph.list` | Structurally validate all deploy graphs |
| `graph.validate` | Validate a specific graph (structural or live) |
| `graph.waves` | Compute topological startup waves from a deploy graph |
| `graph.capabilities` | Extract required capabilities from a deploy graph |
| `lifecycle.status` | Primal status report |
| `mcp.tools.list` | MCP tool definitions for Squirrel AI |

## Deploy Graphs

primalSpring ships 11 biomeOS deploy graph TOMLs (all nodes declare `by_capability`):

| Graph | Pattern | Nodes |
|-------|---------|-------|
| `primalspring_deploy.toml` | Sequential | 9 (full NUCLEUS + primalSpring) |
| `coralforge_pipeline.toml` | Pipeline | 7 (neuralSpring → hotSpring → wetSpring → toadStool → NestGate) |
| `streaming_pipeline.toml` | Pipeline | 4 (NestGate → primalSpring → sweetGrass) |
| `continuous_tick.toml` | Continuous | 8 (60 Hz health poll loop) |
| `conditional_fallback.toml` | ConditionalDag | 4 (GPU → CPU fallback) |
| `parallel_capability_burst.toml` | Parallel | 4 (crypto + net + storage + compute) |
| `tower_atomic_bootstrap.toml` | Sequential | 3 (security + discovery + validation) |
| `node_atomic_compute.toml` | Sequential | 4 (Tower + compute + validation) |
| `nest_deploy.toml` | Sequential | 4 (Tower + storage + validation) |
| `nucleus_complete.toml` | Sequential | 9 (all capabilities + coordination) |
| `spring_byob_template.toml` | Sequential | Template for new springs |

All graphs have `by_capability` on every node and are structurally validated +
topologically sorted at test time. `topological_waves()` computes startup wave
ordering from dependency edges via Kahn's algorithm.

## IPC Resilience

Converged IPC resilience stack absorbed from 7 sibling springs: `IpcError` (8
typed variants with `is_retriable()`, `is_timeout_likely()`, etc.),
`CircuitBreaker`, `RetryPolicy`, `resilient_call()`, `DispatchOutcome<T>`, and
centralized `extract_rpc_result`/`extract_rpc_dispatch` for JSON-RPC result
extraction. Capability parsing handles all 4 ecosystem wire formats (A/B/C/D).

## Discovery (5-Tier)

Discovery walks 5 tiers in priority order:
1. `{PRIMAL}_SOCKET` env override
2. `$XDG_RUNTIME_DIR/biomeos/{primal}-{family}.sock` (XDG convention)
3. `{temp_dir}/biomeos/{primal}-{family}.sock` (fallback)
4. Primal manifest: `$XDG_RUNTIME_DIR/ecoPrimals/manifests/{primal}.json`
5. Socket registry: `$XDG_RUNTIME_DIR/biomeos/socket-registry.json`

Tiers 4–5 absorbed from biomeOS v2.50 and Squirrel alpha.12.

## MCP Tools

primalSpring exposes 8 typed MCP tools via `mcp.tools.list` for Squirrel AI
coordination tool discovery. Each tool has a JSON Schema input definition.

## Docs

- `wateringHole/README.md` — Track structure and cross-spring context
- `wateringHole/PRIMALSPRING_COMPOSITION_GUIDANCE.md` — Composition guidance
- `wateringHole/handoffs/` — Active + archived evolution handoffs
- `specs/CROSS_SPRING_EVOLUTION.md` — Evolution path (Phase 0–5+)
- `specs/BARRACUDA_REQUIREMENTS.md` — barraCuda relationship (indirect only)
- `whitePaper/baseCamp/README.md` — baseCamp paper pointer

---

**License**: AGPL-3.0-or-later
