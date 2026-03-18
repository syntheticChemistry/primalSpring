# primalSpring

**Coordination and composition validation spring for the ecoPrimals ecosystem.**

| | |
|-|-|
| **Domain** | Primal coordination, atomic composition, graph execution, emergent systems, bonding |
| **Version** | 0.2.0 |
| **Edition** | Rust 2024 (1.87+) |
| **License** | AGPL-3.0-or-later |
| **Tests** | 157 (148 unit + 9 integration) |
| **Experiments** | 38 (7 tracks) |
| **Unsafe** | Workspace-level `forbid` via `[workspace.lints.rust]` |
| **C deps** | Zero (ecoBin compliant) |

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
│       └── server_integration.rs  # 9 real IPC round-trip tests
├── experiments/                   # 38 validation experiments (7 tracks)
├── graphs/                        # 6 biomeOS deploy graph TOMLs
├── niches/                        # BYOB niche deployment YAML
├── specs/                         # Architecture specs
└── wateringHole/                  # Docs and handoffs
```

## Key Design Principles

- **Sovereign discovery**: No hardcoded primal rosters. Discovery is
  composition-driven (via `AtomicType::required_primals()`) or Neural
  API-driven (via `neural-api-client-sync`).
- **Graceful degradation**: Experiments honestly skip checks when primals
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

# Run all 157 tests
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
| `coordination.validate_composition` | Validate an atomic composition (Tower/Node/Nest/FullNucleus) |
| `coordination.discovery_sweep` | Enumerate primals in a composition |
| `coordination.neural_api_status` | Neural API reachability |
| `graph.list` | Structurally validate all deploy graphs |
| `graph.validate` | Validate a specific graph (structural or live) |
| `lifecycle.status` | Primal status report |

## Deploy Graphs

primalSpring ships 6 biomeOS deploy graph TOMLs:

| Graph | Pattern | Nodes |
|-------|---------|-------|
| `primalspring_deploy.toml` | Sequential | 9 (full NUCLEUS + primalSpring) |
| `coralforge_pipeline.toml` | Pipeline | 7 (neuralSpring → hotSpring → wetSpring → toadStool → NestGate) |
| `streaming_pipeline.toml` | Pipeline | 4 (NestGate → primalSpring → sweetGrass) |
| `continuous_tick.toml` | Continuous | 8 (60 Hz health poll loop) |
| `conditional_fallback.toml` | ConditionalDag | 4 (GPU → CPU fallback) |
| `parallel_capability_burst.toml` | Parallel | 4 (crypto + net + storage + compute) |

All graphs are parsed and structurally validated by `deploy.rs` at test time.

## IPC Resilience

Converged IPC resilience stack absorbed from 7 sibling springs: `IpcError` (8
typed variants with `is_retriable()`, `is_timeout_likely()`, etc.),
`CircuitBreaker`, `RetryPolicy`, `resilient_call()`, `DispatchOutcome<T>`, and
centralized `extract_rpc_result`/`extract_rpc_dispatch` for JSON-RPC result
extraction. Capability parsing handles all 4 ecosystem wire formats (A/B/C/D).

## Docs

- `wateringHole/README.md` — Track structure and cross-spring context
- `wateringHole/PRIMALSPRING_COMPOSITION_GUIDANCE.md` — Composition guidance
- `specs/CROSS_SPRING_EVOLUTION.md` — Evolution path (Phase 0–9)
- `specs/BARRACUDA_REQUIREMENTS.md` — barraCuda relationship (indirect only)
- `whitePaper/baseCamp/README.md` — baseCamp paper pointer

---

**License**: AGPL-3.0-or-later
