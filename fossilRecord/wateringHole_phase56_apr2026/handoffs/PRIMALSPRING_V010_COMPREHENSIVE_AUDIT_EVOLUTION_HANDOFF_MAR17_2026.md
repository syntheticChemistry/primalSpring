# Handoff: primalSpring v0.1.0 — Comprehensive Audit + Deep Debt Evolution

**Date:** March 17, 2026  
**From:** primalSpring (Phase 0 scaffolding → Phase 0→1 real discovery)  
**To:** biomeOS, all NUCLEUS primals, ecosystem  
**License:** AGPL-3.0-or-later  
**Covers:** primalSpring v0.1.0, all 38 experiments, 7 tracks

---

## Executive Summary

primalSpring underwent a comprehensive audit and immediate deep-debt evolution
in a single session. The project went from 100% vacuous scaffolding (38
experiments all hardcoding `true`) to honest validation with real runtime
discovery, 55 unit tests, zero clippy warnings (pedantic+nursery), and a
genomeBin pinned in `plasmidBin/`.

---

## What Changed

### Code Quality (was: 55+ warnings, format failures)

| Metric | Before | After |
|--------|--------|-------|
| clippy warnings (pedantic+nursery) | 55 | **0** |
| `cargo fmt --check` | FAIL (11 diffs) | **PASS** |
| `#[allow()]` in production | 1 (`module_name_repetitions`) | **0** |
| `#![forbid(unsafe_code)]` | All files | All files (unchanged) |
| C dependencies | 0 | **0** |
| Files over 1000 LOC | 0 | **0** |
| Unit tests | 0 | **55** |

### Crate Rename

`primalspring-barracuda` → `primalspring`. The old name falsely implied a
barraCuda dependency. primalSpring validates coordination, not math — it has
zero barraCuda dependency by design.

### IPC Module Evolution (was: stub-only)

The IPC module was split from a single `mod.rs` into three focused modules:

| Module | Purpose | Status |
|--------|---------|--------|
| `ipc::discover` | Runtime socket discovery (env → XDG → temp_dir) | **Complete** with 8 tests |
| `ipc::protocol` | JSON-RPC 2.0 types (Request, Response, Error) | **Complete** with 7 tests |
| `ipc::client` | Unix socket client (connect, call, health_check) | **Complete** with 3 tests |

Key improvements:
- `JsonRpcRequest.id` now uses `AtomicU64` auto-increment (was hardcoded `1`)
- `JsonRpcResponse` and `JsonRpcError` types added (were missing)
- `socket_path()` uses `std::env::temp_dir()` (was hardcoded `/tmp`)
- `discover_primal()` returns structured `DiscoveryResult` with source info
- `discover_all()` and `discover_reachable()` sweep functions added
- `connect_primal()` combines discovery + connection in one call
- `PrimalClient` provides typed methods: `call()`, `health_check()`, `capabilities()`

### Validation Module Evolution

- Added `CheckOutcome` enum: `Pass`, `Fail`, `Skip`
- Added `check_skip()` for honest scaffolding (replaces fake `check_bool(_, true, _)`)
- Added `skipped` counter and `evaluated()` method
- Summary now shows skip count
- 15 unit tests covering all check types

### Experiment Evolution (was: all vacuous)

All 38 experiments evolved from hardcoded `true` to:
- **Real checks**: call `discover_primal()`, validate type hierarchies, test
  serialization, verify tolerance constants, check socket path formats
- **Honest skips**: IPC-dependent checks use `check_skip()` with clear reason
- Exit code based on real pass/fail (never fakes success)

### Ecosystem Updates

- `plasmidBin/manifest.toml` — added `[springs.primalspring]` entry
- `plasmidBin/sources.toml` — added `[sources.primalspring]` entry
- `plasmidBin/springs/primalspring_primal` — release binary pinned (2.1 MB)
- `wateringHole/STANDARDS_AND_EXPECTATIONS.md` — added genome pinning standard
- `wateringHole/PRIMAL_REGISTRY.md` — updated primalSpring status

---

## Test Results

```
55 passed, 0 failed, 0 ignored
```

Tests cover: `validation` (15), `ipc::discover` (8), `ipc::protocol` (7),
`ipc::client` (3), `coordination` (8), `graphs` (3), `emergent` (5),
`bonding` (3), `tolerances` (3).

---

## What Blocks Phase 1 (Live IPC)

1. **Live primals needed** — experiments need BearDog + Songbird running on
   Unix sockets to move from `check_skip` to `check_bool` with real IPC
2. **biomeOS graph executor** — Track 2 experiments need biomeOS to parse
   and execute the graph TOML files
3. **Provenance Trio deployment** — Track 3 (RootPulse) needs rhizoCrypt +
   LoamSpine + sweetGrass running for the 6-phase commit flow

---

## What Each Consumer Should Know

### biomeOS
- primalSpring's `graphs/primalspring_deploy.toml` is ready for biomeOS to
  consume via `biomeos deploy`
- The niche YAML (`niches/primalspring-coordination.yaml`) defines the full
  deployment

### BearDog + Songbird
- exp001 (Tower Atomic) is the first experiment that will go fully live
- It calls `discover_primal("beardog")` and `discover_primal("songbird")`
- Phase 1 will add `PrimalClient::connect()` + `health_check()`

### All Primals
- primalSpring's `KNOWN_PRIMALS` list includes all 10 known primals
- Discovery uses the standard `$XDG_RUNTIME_DIR/biomeos/{primal}-{family}.sock`
  convention and `{PRIMAL}_SOCKET` env override

### Springs
- primalSpring genomeBin is pinned in `plasmidBin/springs/primalspring_primal`
- Springs can use it for coordination validation in their own niche deployments
