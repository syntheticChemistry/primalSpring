# primalSpring baseCamp — Coordination and Composition Validation

**Date**: March 18, 2026
**Status**: Phase 2 complete — 38 experiments, 157 tests, niche self-knowledge, deploy graph validation

---

## What This Is

Where baseCamp papers for other springs explore scientific questions using the
ecoPrimals infrastructure, primalSpring's baseCamp explores **the infrastructure
itself**. The "papers" are the atomics. The "experiments" are composition patterns.
The validation target is biomeOS and the Neural API.

## The Paper

See `ecoPrimals/whitePaper/gen3/baseCamp/23_primal_coordination.md` (Paper 23) for
the full baseCamp paper documenting primalSpring's validation of ecosystem coordination.

## Experiments by Track

| Track | Domain | Experiments | Key Question |
|-------|--------|-------------|--------------|
| 1 | Atomic Composition | exp001–006 | Do atomics deploy correctly? |
| 2 | Graph Execution | exp010–015 | Do all 5 coordination patterns work? |
| 3 | Emergent Systems | exp020–025 | Do Layer 3 systems emerge correctly? |
| 4 | Bonding & Plasmodium | exp030–034 | Does multi-gate coordination work? |
| 5 | coralForge | (exp025) | Does the neural object pipeline work? |
| 6 | Cross-Spring | exp040–044 | Do cross-spring data flows work? |
| 7 | Showcase-Mined | exp050–059 | Do mined phase1/phase2 coordination patterns hold? |

## Current State (v0.2.0)

| Metric | Value |
|--------|-------|
| Experiments | 38 (7 tracks) |
| Unit tests | 148 |
| Integration tests | 9 (real JSON-RPC round-trips against live server) |
| clippy (pedantic+nursery) | 0 warnings |
| cargo doc (-D warnings) | 0 warnings |
| `#[allow()]` in production | 0 |
| `#[expect()]` with reason | 3 (safe cast boundaries only) |
| unsafe_code | Workspace-level `forbid` |
| C dependencies | 0 (pure Rust, ecoBin compliant) |
| IPC client | Real Unix socket client with JSON-RPC 2.0 |
| IPC resilience | IpcError, CircuitBreaker, RetryPolicy, resilient_call, DispatchOutcome |
| Capability parsing | 4-format (A/B/C/D) |
| Discovery | Runtime `discover_primal()` via env/XDG/temp + Neural API |
| Niche self-knowledge | `niche.rs` — 21 capabilities, semantic mappings, cost estimates, registration |
| Deploy graph validation | `deploy.rs` — parse, structural validate, live probe all 6 TOMLs |
| Validation | `check_bool` (real) + `check_skip` (honest scaffolding) |
| Exit pattern | Uniform `finish()` + `exit_code()` with JSON output support |
| Dishonest scaffolding | 0 (all experiments use honest skip or real validation) |

## What Changed This Session (v0.2.0 evolution)

1. **Niche self-knowledge** (`niche.rs`) — 21 capabilities, semantic mappings,
   operation dependencies, cost estimates, `register_with_target()` following
   airSpring pattern
2. **Deploy graph validation** (`deploy.rs`) — parse all 6 TOMLs, structural
   checks (names, binaries, dependencies, ordering), live primal probing
3. **Integration tests** — 9 real JSON-RPC round-trip tests against live server
4. **validate_all meta-validator** — runs all 38 experiments in sequence
5. **Standardized exit patterns** — all 38 experiments use `finish()` + `exit_code()`
6. **Provenance on structural counts** — magic numbers replaced with API calls
7. **Tightened lints** — removed blanket `cast_possible_truncation` allow
8. **Zero doc warnings** — fixed broken intra-doc links, doc/code mismatches

## What Remains (Phase 3+)

- Tolerance calibration against live NUCLEUS deployment
- biomeOS graph executor integration (deploy graphs are validated but not executed)
- Songbird registration on startup (ecosystem-wide gap)
- MCP tool definitions for coordination capabilities
- `cargo llvm-cov` integration with coverage target (90%+)
