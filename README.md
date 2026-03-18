# primalSpring

**Coordination and composition validation spring for the ecoPrimals ecosystem.**

| | |
|-|-|
| **Domain** | Primal coordination, atomic composition, graph execution, emergent systems, bonding |
| **Version** | 0.1.0 |
| **Edition** | Rust 2024 (1.87+) |
| **License** | AGPL-3.0-or-later |
| **Tests** | 69 unit tests |
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
│   │   ├── coordination/   # Atomic composition, health probing, composition validation
│   │   ├── graphs/         # Graph execution pattern types
│   │   ├── emergent/       # Emergent system validation types
│   │   ├── bonding/        # Multi-gate bonding model types
│   │   ├── ipc/            # JSON-RPC 2.0 client + Neural API bridge + socket discovery
│   │   ├── validation/     # Experiment harness (check_bool, check_skip, JSON output)
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
| `capabilities.list` | Available coordination capabilities |
| `coordination.validate_composition` | Validate an atomic composition |
| `coordination.discovery_sweep` | Enumerate primals in a composition |
| `coordination.neural_api_status` | Neural API reachability |
| `lifecycle.status` | Primal status report |

## Docs

- `wateringHole/README.md` — Track structure and cross-spring context
- `specs/CROSS_SPRING_EVOLUTION.md` — Evolution path
- `specs/PAPER_REVIEW_QUEUE.md` — Experiment priority queue
- `specs/SHOWCASE_MINING_REPORT.md` — Patterns mined from phase1/phase2

---

**License**: AGPL-3.0-or-later
