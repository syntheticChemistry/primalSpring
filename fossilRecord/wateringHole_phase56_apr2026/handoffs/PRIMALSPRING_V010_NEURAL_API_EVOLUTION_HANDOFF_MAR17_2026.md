# Handoff: primalSpring v0.1.0 — Neural API Integration + Server Mode + Sovereignty

**Date:** March 17, 2026  
**From:** primalSpring (Phase 0→1 — real IPC evolution)  
**To:** biomeOS, all NUCLEUS primals, ecosystem  
**License:** AGPL-3.0-or-later  
**Supersedes:** `archive/PRIMALSPRING_V010_COMPREHENSIVE_AUDIT_EVOLUTION_HANDOFF_MAR17_2026.md`

---

## Executive Summary

primalSpring evolved from scaffolded validation to real IPC integration in a
single session. Key changes: Neural API integration via `neural-api-client-sync`,
sovereignty fix (removed hardcoded primal roster), server mode implementation,
and composition-driven discovery. Tests grew from 55 to 69, all clippy
pedantic+nursery warnings resolved, workspace lints centralized.

---

## What Changed

### Neural API Integration

| Before | After |
|--------|-------|
| No biomeOS dependency | `neural-api-client-sync` path dep |
| `KNOWN_PRIMALS` hardcoded roster | Composition-driven + Neural API discovery |
| No ecosystem awareness | `NeuralBridge::discover()`, `health_check()`, `discover_capability()` |

New discovery functions in `ipc::discover`:
- `neural_bridge()` — connect to Neural API via 5-tier resolution
- `neural_api_healthy()` — check if biomeOS is running
- `discover_capabilities(capability)` — query registered capabilities
- `discover_for(primals)` — probe a caller-provided set of primal names
- `discover_reachable_for(primals)` — filter to reachable sockets

### Sovereignty Fix

**Removed**: `KNOWN_PRIMALS` constant — a hardcoded list of 10 primal names.
This violated sovereignty (primalSpring should not maintain a static roster
of the entire ecosystem).

**Replaced with**: Two sovereign patterns:
1. **Composition-driven**: `AtomicType::required_primals()` provides the
   primal list for each atomic composition (Tower = beardog+songbird, etc.)
2. **Neural API-driven**: `NeuralBridge::discover_capability()` queries
   what's actually registered at runtime

All 38 experiments updated. Zero references to `KNOWN_PRIMALS` remain.

### Coordination Module Evolution

New real-IPC functions in `coordination`:
- `probe_primal(name)` — discover socket, connect, health check, list caps, measure latency
- `validate_composition(atomic)` — probe all required primals, aggregate results
- `health_check(primal)` — connect and return latency
- `health_check_within_tolerance(primal)` — latency vs tolerance bound

### Server Mode

`primalspring_primal server` now implements a full JSON-RPC 2.0 server:

| Method | Description |
|--------|-------------|
| `health.check` | Self health status |
| `capabilities.list` | Coordination capabilities |
| `coordination.validate_composition` | Validate atomic composition (Tower/Node/Nest/FullNucleus) |
| `coordination.discovery_sweep` | Enumerate primals in a composition |
| `coordination.neural_api_status` | Neural API reachability |
| `lifecycle.status` | Primal status report |

Status command now shows live Neural API status and discovered primal count.

### Validation Harness Evolution

- `check_or_skip()` — conditional evaluation: run check if prerequisite present, honest skip if not
- `to_json()` — structured JSON output for CI pipelines
- `finish()` — auto-selects JSON or human format via `PRIMALSPRING_JSON=1`
- `exit_code()` — 0 on all-pass, 1 otherwise
- `CheckOutcome`, `CheckResult`, `ValidationResult` now `Serialize + Deserialize`

### Experiment Evolution

Experiments evolved from scaffold to real IPC:

| Experiment | Before | After |
|-----------|--------|-------|
| exp001 (Tower) | `check_bool(true)` stubs | Real `probe_primal()` + Neural API + `check_or_skip` |
| exp004 (NUCLEUS) | `KNOWN_PRIMALS` count check | Per-primal health probing with latency |
| exp034 (Aggregation) | `discover_all()` stub | Composition-driven `discover_for()` |
| exp040 (Cross-spring) | `KNOWN_PRIMALS` stub | Neural API + `discover_for()` |
| exp051 (Discovery) | `KNOWN_PRIMALS` sweep | Composition-driven `discover_for()` |
| exp053 (Lifecycle) | `KNOWN_PRIMALS` stub | `validate_composition()` with participant threshold |

### Workspace Hygiene

| Metric | Before | After |
|--------|--------|-------|
| Unit tests | 55 | **69** |
| Clippy warnings | 0 | **0** (with workspace lints) |
| Workspace deps | None | `serde`, `serde_json`, `tracing`, `clap`, `neural-api-client-sync` |
| Workspace lints | Inline per-file | Centralized `[workspace.lints.clippy]` |
| Experiment Cargo.tomls | Inline deps + lints | `workspace = true` everywhere |
| Root README | None | Created |

---

## Test Results

```
69 passed, 0 failed, 0 ignored
```

Tests cover: `validation` (17), `ipc::discover` (11), `ipc::protocol` (7),
`ipc::client` (3), `coordination` (14), `graphs` (3), `emergent` (5),
`bonding` (3), `tolerances` (3), `lib` doc-tests.

---

## What Consumers Should Know

### biomeOS

- primalSpring now depends on `neural-api-client-sync` (path dep)
- Server mode listens at `$XDG_RUNTIME_DIR/biomeos/primalspring-{family}.sock`
- `coordination.validate_composition` is callable via Neural API routing

### All Primals

- `KNOWN_PRIMALS` is gone. primalSpring discovers via `AtomicType::required_primals()`
  or Neural API
- Each primal can be independently probed via `probe_primal(name)`
- Socket convention unchanged: `$XDG_RUNTIME_DIR/biomeos/{primal}-{family}.sock`

### Springs

- JSON output via `PRIMALSPRING_JSON=1` enables CI pipeline integration
- `validate_composition()` returns structured `CompositionResult` (serializable)

---

## What Blocks Phase 2

1. **Live primals** — experiments need BearDog + Songbird running on
   Unix sockets to move from `check_skip` to real validation
2. **biomeOS graph executor** — Track 2 experiments need biomeOS to parse
   and execute graph TOML files
3. **Provenance Trio deployment** — Track 3 needs rhizoCrypt + LoamSpine +
   sweetGrass running for the 6-phase commit flow

---

*primalSpring v0.1.0: 38 experiments, 7 tracks, 69 unit tests, Neural API
integrated, server mode running, sovereign discovery. The spring that validates
the coordination layer.*
