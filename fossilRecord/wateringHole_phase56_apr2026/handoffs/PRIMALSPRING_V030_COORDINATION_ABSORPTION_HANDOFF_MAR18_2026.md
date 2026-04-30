# primalSpring v0.3.0 — Coordination Absorption Handoff

**Date**: March 18, 2026
**From**: primalSpring
**To**: biomeOS, ecosystem

---

## Summary

primalSpring has absorbed primal coordination responsibility from biomeOS.
The following biomeOS modules have been ported to pure synchronous Rust
(no tokio) and integrated into primalSpring's validation domain:

| biomeOS Module | primalSpring Module | LOC |
|----------------|---------------------|-----|
| `primal_spawner.rs` (binary discovery, spawn, socket wait) | `launcher/mod.rs` | ~350 |
| `nucleation.rs` (deterministic socket assignment) | `launcher/mod.rs` (SocketNucleation) | — |
| `health_check.rs` (liveness probing) | `harness/mod.rs` (RunningAtomic) | — |
| `primal_launch_profiles.toml` | `config/primal_launch_profiles.toml` | 45 |
| `concurrent_startup.rs` (wave-based startup) | Existing `topological_waves()` | — |

## What primalSpring Now Provides

### `launcher/` Module
- `discover_binary(primal)` — 5-tier base directory search, 6 binary-name patterns
- `spawn_primal(primal, family_id, nucleation)` — std::process::Command, socket wait, RAII cleanup
- `spawn_neural_api(family_id, nucleation, graphs_dir)` — dedicated Neural API server launcher
- `wait_for_socket(path, timeout)` — polling loop (100ms interval)
- `SocketNucleation` — deterministic socket assignment (`$XDG_RUNTIME_DIR/biomeos/{primal}-{family}.sock`)
- `LaunchProfile` — data-driven TOML per-primal socket configuration
- `PrimalProcess` — RAII wrapper: Drop sends SIGTERM, waits, removes socket
- `LaunchError` — typed: BinaryNotFound, SpawnFailed, SocketTimeout, HealthCheckFailed, ProfileParseError

### `harness/` Module
- `AtomicHarness::new(atomic)` — construct harness with static primal ordering
- `AtomicHarness::with_graph(atomic, path)` — construct with graph-driven topological wave ordering
- `AtomicHarness::start(family_id)` — spawns composition, uses `topological_waves()` when graph is set
- `AtomicHarness::start_with_neural_api(family_id, graphs_dir)` — spawns primals + Neural API server
- `RunningAtomic` — RAII lifecycle:
  - `socket_for(capability)` — capability-based socket lookup (security → beardog)
  - `client_for(capability)` — capability-based PrimalClient connection
  - `socket_for_primal(name)` / `client_for_primal(name)` — identity-based fallbacks
  - `health_check_all()`, `capabilities_all()`, `validate()`, `neural_bridge()`
- Drop tears down in reverse order and removes the runtime directory

### Binary Harvest
- Stable binaries copied from `biomeOS/plasmidBin/` to `ecoPrimals/plasmidBin/primals/`:
  beardog (v0.9.0), songbird (v3.33.0), nestgate (v2.1.0), toadstool (v0.1.0), squirrel (v0.1.0)
- `primalspring_primal` (v0.2.0) installed to `plasmidBin/springs/`

### Integration Tests (6 live, all `#[ignore]`)
- `tower_atomic_live_health_check` — spawns Tower, verifies all primals live
- `tower_atomic_live_capabilities` — spawns Tower, verifies capability reporting
- `tower_atomic_live_validation_result` — spawns Tower, runs full validation
- `tower_neural_api_health` — spawns Tower + Neural API, verifies NeuralBridge health
- `tower_neural_api_capability_discovery` — verifies ecosystem.coordination via NeuralBridge
- `tower_neural_api_full_validation` — full validation including Neural API health check
- All require `ECOPRIMALS_PLASMID_BIN` pointing at `ecoPrimals/plasmidBin/`

### Experiment Evolution
- exp001 now optionally spawns live primals via `AtomicHarness` when `ECOPRIMALS_PLASMID_BIN` is set

## What This Means for biomeOS

biomeOS can focus entirely on its evolution as a primal (the NUCLEUS
orchestrator). Primal coordination validation — binary discovery, socket
nucleation, wave-based startup, health checking — is now primalSpring's
domain.

biomeOS no longer needs to maintain test harnesses for primal spawning.
primalSpring validates that the coordination layer works correctly using
live binaries from `ecoPrimals/plasmidBin/`.

## Patterns Available for Ecosystem Absorption

| Pattern | Module | Reusable? |
|---------|--------|-----------|
| 5-tier binary discovery | `launcher/discover_binary` | Yes — any spring needing primal binaries |
| Deterministic socket nucleation | `launcher/SocketNucleation` | Yes — prevents socket collision |
| Data-driven launch profiles | `config/primal_launch_profiles.toml` | Yes — extend with new primals |
| RAII process lifecycle | `launcher/PrimalProcess` | Yes — Drop-safe child management |
| Composition test harness | `harness/AtomicHarness` | Yes — any spring validating compositions |

## Test Metrics After Absorption

| Metric | Before | After |
|--------|--------|-------|
| Unit tests | 225 | 239 |
| Integration tests | 10 | 16 (10 + 6 live) |
| Doc-tests | 1 | 2 |
| Total | 236 | 257 |
| Clippy (pedantic+nursery) | 0 warnings | 0 warnings |
| Unsafe | forbid | forbid |

---

**License**: AGPL-3.0-or-later
