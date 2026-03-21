# primalSpring Experiments

**40 experiments across 8 tracks** validating coordination, composition, and emergent behavior in the ecoPrimals ecosystem.

---

## Overview

Each experiment is a standalone Rust binary in its own crate under `experiments/`.
Every experiment uses the shared validation harness (`check_bool`, `check_skip`,
`check_or_skip`) with uniform `finish()` + `exit_code()` and optional JSON output
(`PRIMALSPRING_JSON=1`).

All experiments use **honest scaffolding**: when a primal isn't running, the
experiment reports `check_skip` (not a fake pass). Zero dishonest scaffolding
across all 40 experiments.

## Running

```bash
# Run all 40 experiments via meta-validator
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
| 2 | Graph Execution | exp010–015 | Discovery wired |
| 3 | Emergent Systems | exp020–025 | Discovery wired |
| 4 | Bonding & Plasmodium | exp030–034 | Discovery wired |
| 5 | coralForge | (exp025) | Discovery wired |
| 6 | Cross-Spring | exp040–044 | Discovery wired |
| 7 | Showcase-Mined | exp050–059 | Discovery wired |
| 8 | Live Composition | exp060–061 | **Live validated** (Tower + Squirrel AI) |

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
| 5 | Nest Atomic + Node Atomic | Next |
| 6+ | Full NUCLEUS, graph execution, emergent, bonding | Planned |

## Validation Harness

All experiments share the `ecoPrimal` library crate's validation module:

- `check_bool(name, actual, expected)` — strict equality check
- `check_skip(name, reason)` — honest skip when dependency unavailable
- `check_or_skip(name, result)` — check if available, skip otherwise
- `ValidationResult::finish()` — summary with pass/fail/skip counts
- `ValidationResult::exit_code()` — 0 if all pass/skip, 1 if any fail
- `ValidationResult::with_provenance(source, date)` — structured provenance metadata

## Crate Structure

Each experiment crate has:
```
experiments/expNNN/
├── Cargo.toml    # version 0.4.0, depends on ecoPrimal
└── src/
    └── main.rs   # experiment binary
```

All experiment crates inherit workspace lints (clippy pedantic+nursery, `forbid(unsafe_code)`).

---

**License**: AGPL-3.0-or-later
