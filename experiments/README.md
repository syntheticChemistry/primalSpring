# primalSpring Experiments

**74 experiments across 17 tracks** validating coordination, composition, and emergent behavior in the ecoPrimals ecosystem.

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
and exits with the appropriate code (0 = pass, 1 = fail). All 74 experiments
carry structured provenance via `with_provenance()`.

All experiments use **honest scaffolding**: when a primal isn't running, the
experiment reports `check_skip` (not a fake pass). Zero dishonest scaffolding
across all 74 experiments. All 74 use centralized library helpers for TCP RPC,
method name constants (`ipc::methods`), and primal name constants (`primal_names`).

## Running

```bash
# Run all 74 experiments via meta-validator
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
| 15 | LAN/Covalent + Mixed Composition | exp089–093 | **Structural** (deployment graph sweep, Tower Atomic LAN probe, L0 primal routing matrix, L2 dual-tower ionic, L3 covalent mesh backup) |
| 16 | Composition Parity | exp094 | Full NUCLEUS composition parity: 19/19 checks via live IPC |
| 17 | Cross-Architecture Deployment | exp095–096 | **Live validated** (proto-nucleate template, Pixel cross-arch bonding via biomeOS Neural API `--tcp-only`) |

## Experiment Status Key

- **IPC wired**: Uses real `probe_primal()` with Unix socket JSON-RPC 2.0
- **Discovery wired**: Uses `discover_primal()` with 6-tier fallback; skips honestly if primal unavailable
- **Structural**: Validates graph structure, deploy patterns, or composition logic without requiring live primals

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
| 18 | LAN Covalent Deployment | Planned — live multi-gate NUCLEUS, biomeOS aarch64 cross-compile for Pixel substrate (blocked on GAP-MATRIX-05) |
| 19 | Gen4 Spring Scaffolding | **Done** (5 spring primal binaries built, 7 validation graphs, launch profiles) |
| 20 | Deployment Matrix + Substrate Validation | **Done** (43-cell matrix, benchScale topologies, chaos/federation/provenance experiments, agentic trio, storytelling stack, showcase mining, 67 experiments, 59 graphs) |
| 21 | Deep Ecosystem Audit + Library Consolidation | **Done** (ipc::tcp + ipc::methods library modules, launcher 4-module refactor, provenance circuit breaker half-open, tracing migration, 8 experiments consolidated to library helpers, 413 tests, zero clippy/fmt/doc warnings) |
| 22 | Track 14 — exp085–exp088 — E2E composition: BearDog crypto lifecycle, genetic identity, Neural API routing, storytelling composition | **Done** |
| 23 | Ecosystem debt resolution — gap map, crypto negative validation graph, exp086 generate-then-verify, per-primal handoffs, composition standards | **Done** (v0.8.0) |
| 23b | biomeOS v2.78 rewire — 20 method constants, NeuralBridge graph lifecycle + discover_domain, rollback + federation validation graphs | **Done** (v0.8.0b) |
| 23c | NUCLEUS atomics + biomeOS substrate — SubstrateHealth, Nest+Squirrel, all deploy graphs carry Phase 0 biomeos node, composition.tower_squirrel_health wired, 402 tests | **Done** (v0.8.0c) |
| 23d | Absorb toadStool S168 + esotericWebb V6 + ludoSpring V32 — 16 new method constants (compute, shader, webb, session, game), gen4 storytelling graphs v2.0 | **Done** (v0.8.0d) |
| 23e | Live composition — esotericWebb as ecoPrimals product: 3 new deploy graphs, 3 validation graphs, exp088 UDS rewrite, capability discovery fix, NeuralBridge health fallback, Tower 13/13, Neural API 12/12, Storytelling 16/16 | **Done** (v0.8.0e) |
| 23f | Composition decomposition — 7 subsystem compositions (C1-C7), 7 composition validation graphs, PRIMAL_GAPS.md (22 gaps), thin gateway bridge, composition monitor, live subsystem validation 34/43 (79%) | **Done** (v0.8.0f) |
| 23g | Primal rewiring + gap cleanup — methods.rs/neural_bridge.rs/discover.rs rewired, 5 gaps resolved, gap registry scoped to primals only, 6 nucleated spring deploy graphs, 43/44 (98%) live validation | **Done** (v0.8.0g) |
| 24 | Deep debt resolution + public readiness sprint — bingoCube, benchScale, agentReagents, rustChip graded A | **Done** (v0.9.0–v0.9.1) |
| 25 | Modernization sweep — capability naming cleanup, `[[graph.nodes]]` unification, `http_health_probe` deprecated, `nest-deploy.toml` v4.0, exp089+090, Tower Atomic HTTPS validated | **Done** (v0.9.2) |
| 26 | Mixed composition + live validation — particle model (Tower=electron, Node=proton, Nest=neutron), 17 sketch graphs, exp091-093, live Tower Atomic probes, 6 GAP-MATRIX items documented | **Done** (v0.9.3) |
| 27 | BTSP Phase 2 cascade — secure-by-default across 11/13 primals, `ipc::btsp_handshake` module; 100 deploy graphs + 4 fragments; all deploy graphs carry `secure_by_default` metadata | **Done** (v0.9.14) |
| 28 | Inference abstraction + proto-nucleate graphs — vendor-agnostic `inference.*` wire standard, Squirrel bridge, WGSL shader composition model, 5 downstream proto-nucleate + 3 pipeline graphs (neuralSpring/hotSpring/healthSpring) | **Done** (v0.9.14) |
| 39 | NUCLEUS Composition PASS | 17/17 exp094, 441 tests |
| 40 | NUCLEUS Complete | 12/12 ALIVE, 19/19 exp094 PASS, 455 tests |
| 41 | Pre-Downstream Gap Resolution | 13 FullNucleus caps, 443 tests, gap resolution |
| 42 | Multi-Tier Genetics + BTSP Phase 3 | Mito-Beacon / Nuclear lineage / Tags architecture, ChaCha20-Poly1305 encrypted channels, BtspEnforcer deny semantics, ionic bond RPC, content distribution federation |
| 43 | Cross-Architecture Deployment | biomeOS Tower bootstrap on Pixel (aarch64 + GrapheneOS), `tcp_rpc_multi_protocol`, exp096 6/9 cross-arch checks (3 blocked on upstream biomeOS TCP propagation + graph env substitution) |

## Validation Harness

All experiments share the `ecoPrimal` library crate's validation module:

**Builder API** (preferred — all 74 experiments use this):
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
├── Cargo.toml    # version 0.9.14 (workspace), depends on ecoPrimal
└── src/
    └── main.rs   # experiment binary
```

All experiment crates inherit workspace lints (clippy pedantic+nursery, `forbid(unsafe_code)`).

---

**License**: AGPL-3.0-or-later
