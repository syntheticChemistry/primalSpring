# primalSpring

**Coordination and composition validation spring for the ecoPrimals ecosystem.**

| | |
|-|-|
| **Domain** | Primal coordination, atomic composition, graph execution, emergent systems, bonding |
| **Version** | 0.2.0 |
| **Edition** | Rust 2024 (1.87+) |
| **License** | AGPL-3.0-or-later |
| **Tests** | 127 unit tests |
| **Experiments** | 38 (7 tracks) |
| **Unsafe** | Workspace-level `forbid` via `[workspace.lints.rust]` |
| **C deps** | Zero |

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
├── ecoPrimal/              # Library crate + UniBin binary
│   ├── src/
│   │   ├── lib.rs          # Library root (unsafe_code = forbid via workspace lint)
│   │   ├── cast.rs         # Safe numeric casts (micros_u64, u128_to_u64, etc.)
│   │   ├── coordination/   # Atomic composition, health probing, composition validation
│   │   ├── graphs/         # Graph execution pattern types
│   │   ├── emergent/       # Emergent system validation types
│   │   ├── bonding/        # Multi-gate bonding model types
│   │   ├── ipc/            # JSON-RPC 2.0 client, discovery, error, dispatch, extract, resilience
│   │   ├── validation/     # Experiment harness (check_bool, check_skip, OrExit, ValidationSink)
│   │   └── tolerances/     # Named latency and throughput bounds
│   └── src/bin/
│       └── primalspring_primal/  # UniBin: JSON-RPC 2.0 server
├── experiments/            # 38 validation experiments (7 tracks)
├── graphs/                 # Deploy graph TOML files
├── niches/                 # BYOB niche YAML
├── specs/                  # Architecture specs
└── wateringHole/           # Docs and handoffs
```

## Key Design Principles

- **Sovereign discovery**: No hardcoded primal rosters. Discovery is
  composition-driven (via `AtomicType::required_primals()`) or Neural
  API-driven (via `neural-api-client-sync`).
- **Graceful degradation**: Experiments honestly skip checks when primals
  aren't running — never fake a pass.
- **Zero domain knowledge**: primalSpring validates coordination, not math.
  Zero barraCuda, zero WGSL, zero domain science.

## Quick Start

```bash
# Build everything
cargo build

# Run all tests
cargo test

# Check a specific experiment (requires live primals for full validation)
cargo run --bin exp001_tower_atomic

# Start the primalSpring JSON-RPC server
cargo run --bin primalspring_primal -- server

# Show ecosystem status
cargo run --bin primalspring_primal -- status
```

## IPC Integration

primalSpring integrates with biomeOS via `neural-api-client-sync`:

```rust
use primalspring::ipc::NeuralBridge;
use primalspring::ipc::discover::{discover_primal, neural_api_healthy};
use primalspring::coordination::{probe_primal, validate_composition, AtomicType};

// Check Neural API
if neural_api_healthy() {
    // Validate full NUCLEUS composition
    let result = validate_composition(AtomicType::FullNucleus);
    println!("all healthy: {}", result.all_healthy);
}

// Probe a specific primal
let health = probe_primal("beardog");
println!("{}: socket={}, healthy={}", health.name, health.socket_found, health.health_ok);
```

## Server Mode

The `primalspring_primal` binary exposes coordination capabilities via JSON-RPC 2.0:

| Method | Description |
|--------|-------------|
| `health.check` | Self health status |
| `health.liveness` | Kubernetes-style liveness probe |
| `health.readiness` | Kubernetes-style readiness probe |
| `capabilities.list` | Available coordination capabilities |
| `coordination.validate_composition` | Validate an atomic composition |
| `coordination.discovery_sweep` | Enumerate primals in a composition |
| `coordination.neural_api_status` | Neural API reachability |
| `lifecycle.status` | Primal status report |

## IPC Resilience

primalSpring provides a converged IPC resilience stack: `IpcError` (8 typed
variants with `is_retriable()`, `is_timeout_likely()`, etc.), `CircuitBreaker`,
`RetryPolicy`, `resilient_call()`, `DispatchOutcome<T>`, and centralized
`extract_rpc_result`/`extract_rpc_dispatch` for JSON-RPC result extraction.
Capability parsing handles 4 wire formats (A/B/C/D) across the ecosystem.

## Docs

- `wateringHole/README.md` — Track structure and cross-spring context
- `wateringHole/PRIMALSPRING_COMPOSITION_GUIDANCE.md` — Composition guidance (standalone, Tower, Node, Nest, NUCLEUS, Provenance Trio, cross-spring)
- `specs/CROSS_SPRING_EVOLUTION.md` — Evolution path
- `specs/PAPER_REVIEW_QUEUE.md` — Experiment priority queue
- `specs/SHOWCASE_MINING_REPORT.md` — Patterns mined from phase1/phase2

---

**License**: AGPL-3.0-or-later
