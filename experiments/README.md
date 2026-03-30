# primalSpring Experiments

**67 experiments across 14 tracks** validating coordination, composition, and emergent behavior in the ecoPrimals ecosystem.

---

## Overview

Each experiment is a standalone Rust binary in its own crate under `experiments/`.
Every experiment uses the builder-pattern validation harness:

```rust
ValidationResult::new("Experiment Title")
    .with_provenance("exp_crate_name", "2026-03-24")
    .run("subtitle", |v| {
        v.check_bool("name", actual, expected);
        v.check_skip("name", "reason");
    });
```

The `.run()` method prints the banner, executes checks, prints the summary,
and exits with the appropriate code (0 = pass, 1 = fail). All 67 experiments
carry structured provenance via `with_provenance()`.

All experiments use **honest scaffolding**: when a primal isn't running, the
experiment reports `check_skip` (not a fake pass). Zero dishonest scaffolding
across all 67 experiments. All 67 use centralized library helpers for TCP RPC,
method name constants (`ipc::methods`), and primal name constants (`primal_names`).

## Running

```bash
# Run all 67 experiments via meta-validator
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
| 10 | Cross-Gate Deployment | exp073–074 | **Structural** (LAN covalent mesh, remote NUCLEUS health via TCP) |
| 11 | gen4 Deployment Evolution | exp075–080 | **Live validated** (biomeOS substrate, cross-gate routing, Squirrel AI, petalTongue, spring sweep, cross-spring ecology) |
| 12 | Deployment Matrix | exp081 | **Structural** (43-cell deployment matrix sweep across arch × topology × preset × transport) |
| 13 | Substrate Stress | exp082–084 | **Structural** (chaos substrate, federation edge cases, provenance adversarial) |
| 14 | E2E Composition | exp085–088 | **E2E composition** (BearDog crypto lifecycle, genetic identity, Neural API routing, storytelling composition) |

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
| 12.1 | Ecosystem Absorption Wave 1 | **Done** (deny.toml, cast lints, ValidationSink, exit_code_skip_aware, proptest_ipc, primal_names, circuit breaker — 303 tests) |
| 12.2 | Ecosystem Absorption Wave 2 | **Done** (normalize_method, check_relative, NdjsonSink, is_recoverable, Transport, OnceLock probes, missing_docs deny, release gate — 360 tests) |
| 13 | Cross-Gate Deployment Tooling | **Done** (build_ecosystem_musl.sh, prepare_spore_payload.sh, validate_remote_gate.sh, exp073, exp074, exp063 cross-device) |
| 14 | Deep Debt + Builder Pattern + Full Provenance | **Done** (builder `.run()`, all experiments with `with_provenance()`, validation/tests.rs extracted, zero `#[allow()]`, zero `.unwrap()` in experiments) |
| 15 | Cross-Ecosystem Absorption | **Done** (slug constants, unwrap/expect discipline, launcher smart refactor, CONTRIBUTING.md, SECURITY.md, capability-based env naming) |
| 16 | Deep Debt Audit + Centralized Tolerances | **Done** (comprehensive audit, centralized trio/port tolerances, deduplicated capability parsing, hardcoding→capability-based evolution) |
| 17 | gen4 Deployment Evolution | **Done** (biomeOS substrate validation, cross-gate Pixel routing, Squirrel AI bridge, spring deploy sweep, gen4 prototype graphs, 59 experiments, 385 tests) |
| 18 | LAN Covalent Deployment | Planned — live multi-gate NUCLEUS, biomeOS aarch64 cross-compile for Pixel substrate |
| 19 | Gen4 Spring Scaffolding | **Done** (5 spring primal binaries built, 7 validation graphs, launch profiles) |
| 20 | Deployment Matrix + Substrate Validation | **Done** (43-cell matrix, benchScale topologies, chaos/federation/provenance experiments, agentic trio, storytelling stack, showcase mining, 67 experiments, 59 graphs) |
| 21 | Deep Ecosystem Audit + Library Consolidation | **Done** (ipc::tcp + ipc::methods library modules, launcher 4-module refactor, provenance circuit breaker half-open, tracing migration, 8 experiments consolidated to library helpers, 413 tests, zero clippy/fmt/doc warnings) |
| 22 | Track 14 — exp085–exp088 — E2E composition: BearDog crypto lifecycle, genetic identity, Neural API routing, storytelling composition | **Done** |
| 23 | Ecosystem debt resolution — gap map, crypto negative validation graph, exp086 generate-then-verify, per-primal handoffs, composition standards | **Done** (v0.8.0) |
| 23b | biomeOS v2.78 rewire — 20 method constants, NeuralBridge graph lifecycle + discover_domain, rollback + federation validation graphs | **Done** (v0.8.0b) |

## Validation Harness

All experiments share the `ecoPrimal` library crate's validation module:

**Builder API** (preferred — all 67 experiments use this):
- `ValidationResult::new(title)` — create a harness with title
- `.with_provenance(source, date)` — attach structured provenance metadata
- `.run(subtitle, |v| { ... })` — print banner, execute checks, print summary, exit

**Check methods** (called on `&mut ValidationResult` inside `.run()`):
- `check_bool(name, actual, expected)` — strict equality check
- `check_skip(name, reason)` — honest skip when dependency unavailable
- `check_or_skip(name, result)` — check if available, skip otherwise
- `check_relative(name, actual, expected, rel_tol)` — relative tolerance for floating-point validation
- `check_abs_or_rel(name, actual, expected, abs_tol, rel_tol)` — combined absolute-or-relative tolerance
- `check_latency(name, actual_us, max_us)` — latency bound check
- `check_count(name, actual, expected)` — exact count match
- `check_minimum(name, actual, minimum)` — minimum threshold check
- `exit_code_skip_aware()` — 0=pass, 1=fail, 2=all-skipped (skip ≠ fail in CI)
- `section(name)` — begin a named section of checks (groundSpring V120)

**Output sinks**:
- `NdjsonSink` — streaming NDJSON output for CI/log aggregation
- `StdoutSink` / `NullSink` — pluggable output sinks

## Crate Structure

Each experiment crate has:
```
experiments/expNNN/
├── Cargo.toml    # version 0.8.0, depends on ecoPrimal
└── src/
    └── main.rs   # experiment binary
```

All experiment crates inherit workspace lints (clippy pedantic+nursery, `forbid(unsafe_code)`).

---

**License**: AGPL-3.0-or-later
