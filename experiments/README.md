# primalSpring Experiments

**51 experiments across 9 tracks** validating coordination, composition, and emergent behavior in the ecoPrimals ecosystem.

---

## Overview

Each experiment is a standalone Rust binary in its own crate under `experiments/`.
Every experiment uses the shared validation harness (`check_bool`, `check_skip`,
`check_or_skip`) with uniform `finish()` + `exit_code()` and optional JSON output
(`PRIMALSPRING_JSON=1`).

All experiments use **honest scaffolding**: when a primal isn't running, the
experiment reports `check_skip` (not a fake pass). Zero dishonest scaffolding
across all 51 experiments.

## Running

```bash
# Run all 51 experiments via meta-validator
cargo run --release --bin validate_all

# Run a single experiment
cargo run --release --bin exp001

# Run with JSON output (for CI)
PRIMALSPRING_JSON=1 cargo run --release --bin exp001
```

## Track Summary

| Track | Domain | Experiments | Status |
|-------|--------|-------------|--------|
| 1 | Atomic Composition | exp001–006 | IPC wired, discovery wired |
| 2 | Graph Execution | exp010–015 | **3/5 live** (Sequential, Parallel, ConditionalDag), 2 awaiting provenance trio |
| 3 | Emergent Systems | exp020–025 | Discovery wired |
| 4 | Bonding & Plasmodium | exp030–034 | Discovery wired |
| 5 | coralForge | (exp025) | Discovery wired |
| 6 | Cross-Spring | exp040–044 | Discovery wired |
| 7 | Showcase-Mined | exp050–059 | Discovery wired |
| 8 | Live Composition | exp060–070 | **Live validated** (Tower + Squirrel AI + Nest + Node + NUCLEUS + Graph Overlays + Cross-Primal Discovery) |
| 9 | Multi-Node Bonding | exp071–072 | **Structural** (bonding policy, data federation, graph metadata) |

## Experiment Status Key

- **IPC wired**: Uses real `probe_primal()` with Unix socket JSON-RPC 2.0
- **Discovery wired**: Uses `discover_primal()` with 5-tier fallback; skips honestly if primal unavailable

## Phase Progression

| Phase | What | Status |
|-------|------|--------|
| 0 | Scaffolding (38 experiments compile) | Done |
| 1 | Real discovery + honest skip | Done |
| 2 | IPC resilience + niche + deploy | Done (v0.2.0) |
| 2→3 | Deep debt + MCP + provenance | Done (v0.3.0-dev) |
| 3 | Capability-first architecture | Done (v0.3.0) |
| 4 | Tower Atomic + Squirrel AI live | Done (v0.4.0) |
| 5 | Tower Full Utilization | Done (v0.5.0) |
| 6 | Nest Atomic + Node Atomic + NUCLEUS | Done (v0.6.0) |
| 7 | Graph-Driven Overlay Composition | Done (v0.7.0) |
| 8 | Squirrel Cross-Primal Discovery | Done (v0.7.0) |
| 9 | Graph Execution Patterns (3/5 live) | Done (v0.7.0) |
| 10 | Provenance Readiness (structural) | Done (v0.7.0) |
| 11 | Provenance Trio Neural API Integration | **Done** (trio inlined types, ipc::provenance wired) |
| 12 | Multi-Node Bonding + Federation | **Done** (BondType 5 variants, BondingPolicy, 4 multi-node graphs, STUN tiers) |
| 13+ | Emergent E2E, live multi-node, bonding coordination | Awaiting live multi-machine deployment |

## Validation Harness

All experiments share the `ecoPrimal` library crate's validation module:

- `check_bool(name, actual, expected)` — strict equality check
- `check_skip(name, reason)` — honest skip when dependency unavailable
- `check_or_skip(name, result)` — check if available, skip otherwise
- `ValidationResult::finish()` — summary with pass/fail/skip counts
- `ValidationResult::exit_code()` — 0 if all checks pass (at least one required), 1 if any fail or none pass
- `ValidationResult::exit_code_skip_aware()` — 0=pass, 1=fail, 2=all-skipped (skip ≠ fail in CI)
- `ValidationResult::with_provenance(source, date)` — structured provenance metadata
- `ValidationResult::section(name)` — begin a named section of checks (groundSpring V120)

## Crate Structure

Each experiment crate has:
```
experiments/expNNN/
├── Cargo.toml    # version 0.7.0, depends on ecoPrimal
└── src/
    └── main.rs   # experiment binary
```

All experiment crates inherit workspace lints (clippy pedantic+nursery, `forbid(unsafe_code)`).

---

**License**: AGPL-3.0-or-later
