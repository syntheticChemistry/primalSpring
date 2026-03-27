# primalSpring v0.7.0 Phase 16 — Deep Debt Audit + Centralized Tolerances

**Date**: March 24, 2026  
**From**: primalSpring coordination team  
**To**: All primal teams + all spring teams  
**Supersedes**: Phase 15 handoff (patterns still valid; this adds debt resolution)

---

## Summary

Comprehensive audit of primalSpring against all ecosystem standards (wateringHole/),
sibling spring conventions, and the ecoBin/scyBorg/sovereignty requirements. All
critical debt resolved. Hardcoded values evolved to centralized, capability-based
patterns. Coverage baseline measured.

## Metrics

| Metric | Before | After |
|--------|--------|-------|
| Tests | 361 | **364** |
| Tolerance constants with "pending calibration" | 7 | **0** |
| Local magic numbers (ipc/provenance.rs) | 2 (threshold + delay) | **0** (centralized) |
| Inline port defaults (exp073/074) | 7 | **0** (centralized) |
| String literal primal names in tests | 6 | **0** (slug constants) |
| Hardcoded description match (exp010) | 1 | **0** (semantic check) |
| Stale doc comments | 2 | **0** |
| `cargo clippy` warnings | 0 | 0 |
| `cargo deny` issues | 0 | 0 |
| `cargo fmt` diff (primalspring crates) | 0 | 0 |

## Changes

### 1. Centralized Tolerance Constants (`tolerances/mod.rs`)

**What**: Provenance trio resilience params and remote gate TCP port defaults moved
from scattered inline constants to `tolerances/mod.rs` — single source of truth.

**New constants**:
- `TRIO_RETRY_ATTEMPTS` (2) — retry count for trio capability calls
- `TRIO_RETRY_BASE_DELAY_MS` (100) — exponential backoff base
- `DEFAULT_BEARDOG_PORT` (9100) through `DEFAULT_SQUIRREL_PORT` (9500)

**Pattern for all teams**: If you have resilience parameters (circuit breaker
thresholds, retry counts, backoff delays) or default port assignments scattered
across modules, centralize them in your tolerance/config module. Name them, document
their source, and test their bounds.

### 2. Tolerance Calibration Notes Updated

**What**: All 7 latency/throughput tolerance constants had "Calibration: pending
Phase N measurement" notes. Updated to document Phase 15 operational validation.

**Pattern for all teams**: Review your tolerance constants. If they say "pending"
but you've been validating successfully for weeks, update the provenance to reflect
actual operational experience. Stale "pending" notes undermine confidence in values
that are actually well-validated.

### 3. Deduplicated Capability Parsing

**What**: `coordination/mod.rs` had a local `extract_capability_names` that handled
2 JSON formats (array + object keys). Replaced with delegation to
`ipc::discover::extract_capability_names` which handles all 4 ecosystem wire formats
(A: flat array, B: object array, C: method_info, D: semantic_mappings).

**Pattern for all teams**: If you parse capability responses, use the 4-format
parser. Primals in the wild return different shapes — a 2-format parser will silently
drop capabilities from primals using formats C or D.

### 4. Hardcoding → Capability-Based Evolution

**exp010**: Hardcoded expected description string (`"Nodes in dependency order (A -> B -> C)"`)
replaced with semantic assertion (description > 10 chars and contains "order").
The check now survives description text evolution.

**exp073/074**: Inline port numbers (`9100`, `9200`, etc.) replaced with
`tolerances::DEFAULT_*_PORT` constants. All ports remain env-overridable.

**exp074**: String literal primal names in `PrimalProbe` struct replaced with
`primal_names::*` slug constants.

**coordination tests**: `"beardog"`, `"songbird"`, `"toadstool"`, `"nestgate"` string
literals in test assertions replaced with `primal_names::BEARDOG` etc.

**Pattern for all teams**: Audit your test files for hardcoded primal names.
If `primal_names` exists in your crate (or you can reference the ecosystem slug
constants), use them. Raw strings drift silently when names change.

### 5. Coverage Baseline (cargo llvm-cov)

First measured coverage baseline for primalSpring:

| Module tier | Line coverage |
|-------------|---------------|
| Pure logic (cast, dispatch, error, protocol, tolerances, graphs, emergent) | 95–100% |
| IPC resilience (resilience, extract, mcp, probes, proptest_ipc) | 89–100% |
| Validation/bonding/deploy | 75–92% |
| Discovery/coordination/provenance/niche | 66–74% |
| Live-primal-dependent (client, harness) | 50–54% |
| Binary launcher (requires live sockets) | 21% |

The launcher/harness/client modules are tested by 42 `#[ignore]` integration tests
that require live primals. This is correct architecture for a coordination spring —
offline unit tests cover logic; live integration tests cover IPC.

## Patterns for Primal Teams to Absorb

### BearDog
- **Abstract socket regression** remains the #1 blocker for Pixel deployment
- primalSpring's `ipc::transport::Transport` enum cleanly handles Unix + TCP fallback —
  consider adopting for BearDog's listener if abstract sockets are unreliable on Android

### Songbird
- primalSpring centralizes all remote Songbird TCP port defaults in `tolerances/`
- BirdSong beacon exchange (exp073) is validated structurally; needs live LAN for Phase 18

### ToadStool
- exp067 validates dual-protocol (tarpc + JSON-RPC) — `jsonrpc_socket_suffix` pattern
  stable through Phase 16
- `DEFAULT_TOADSTOOL_PORT` (9400) now centralized for cross-gate probing

### NestGate
- exp066/072 validate storage and federation patterns
- `DEFAULT_NESTGATE_PORT` (9300) centralized
- USB build corruption (segfault) still unresolved from Phase 14 hardware audit

### Squirrel
- exp070 cross-primal discovery validated through Phase 16
- `DEFAULT_SQUIRREL_PORT` (9500) centralized
- env_sockets pattern stable: `{CAPABILITY}_PROVIDER_SOCKET` for all 9 providers

### Provenance Trio (sweetGrass, rhizoCrypt, LoamSpine)
- `ipc/provenance.rs` trio circuit breaker now uses centralized `CIRCUIT_BREAKER_THRESHOLD`
  and `TRIO_RETRY_ATTEMPTS` / `TRIO_RETRY_BASE_DELAY_MS` from `tolerances/`
- rhizoCrypt sled→redb migration documented in provenance module docs
- loamSpine `block_on` panic (Phase 11.1) still unresolved — needs nested runtime fix

## Patterns for Spring Teams to Absorb

### All Springs
1. **Centralize your tolerances**: If you have scattered `const THRESHOLD: u32 = 3;`
   in various modules, pull them into one place. primalSpring's `tolerances/mod.rs`
   pattern (named, documented, tested for bounds) works at any scale.
2. **Audit "pending calibration" notes**: If your tolerances have been passing for
   multiple phases, update the docs to say so.
3. **Use 4-format capability parsing**: `extract_capability_names` handles all
   ecosystem wire formats. If you wrote a simpler parser, consider upgrading.
4. **Replace string literal primal names**: Use `primal_names::*` slug constants in
   tests and production code. Round-trip testing (`display_name` ↔ `discovery_slug`)
   catches name drift.

### hotSpring / groundSpring / neuralSpring
- primalSpring's `OnceLock` probe pattern (absorbed from your springs) is stable
  through Phase 16 and used across 12+ test files

### wetSpring / airSpring
- `NdjsonSink`, `exit_code_skip_aware`, and `Transport` enum — all stable through
  Phase 16, actively used in server and experiments

### healthSpring
- Epoch-based circuit breaker (absorbed from your V41) is now centralized through
  `tolerances::CIRCUIT_BREAKER_THRESHOLD` — your pattern works ecosystem-wide

## What Remains

- **Phase 17**: gen4 Composition Bridge (Webb composition health, capability drift, TCP transport)
- **Phase 18**: LAN covalent deployment (multi-gate NUCLEUS, BirdSong exchange)
- **Phase 18+**: Live multi-node validation, emergent E2E, bonding coordination
- **Coverage improvement**: launcher module (21%) needs live primal integration test CI
- **ecoBin compliance**: rebuild all primals as static musl for x86_64 + aarch64
- **genomeBin packaging**: sourDough .genome self-extractors not yet built

## Files Changed

```
ecoPrimal/src/tolerances/mod.rs          — +40 lines (trio params, port defaults, tests)
ecoPrimal/src/ipc/provenance.rs          — TRIO_CIRCUIT_THRESHOLD removed, uses tolerances
ecoPrimal/src/coordination/mod.rs        — extract_capability_names delegates to ipc::discover; tests use primal_names
ecoPrimal/src/bin/validate_all/main.rs   — doc comment corrected
experiments/exp010_*/src/main.rs         — semantic check, primal_names constants
experiments/exp064_*/src/main.rs         — env var documented
experiments/exp073_*/src/main.rs         — tolerances port constants
experiments/exp074_*/src/main.rs         — tolerances port constants, primal_names
scripts/validate_release.sh             — MIN_TESTS 361 → 364
scripts/validate_remote_gate.sh         — usage comment fixed
niches/primalspring-coordination.yaml   — version 0.2.0 → 0.7.0
README.md                               — test count 361 → 364
CHANGELOG.md                            — Phase 16 entry
specs/CROSS_SPRING_EVOLUTION.md          — Phase 16 added, primalSpring row updated
whitePaper/baseCamp/README.md            — Phase 16 section, test count
wateringHole/README.md                   — version/test count
experiments/README.md                    — Phase 16 line
```

## Gates

All gates pass:
- `cargo fmt` — 0 diff (primalspring + experiment crates)
- `cargo clippy --workspace --all-targets -- -D warnings` — 0 warnings
- `cargo deny check` — advisories ok, bans ok, licenses ok, sources ok
- `cargo test --workspace` — 364 passed, 0 failed, 42 ignored

---

**License**: AGPL-3.0-or-later
