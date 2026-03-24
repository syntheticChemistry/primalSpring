# primalSpring v0.7.0 Phase 12.2 — Deep Ecosystem Absorption Handoff

**Date**: March 23, 2026  
**Version**: 0.7.0 (360 tests, 51 experiments)  
**Scope**: Deep absorption from all 7 sibling springs + primals into primalSpring core

---

## What Was Absorbed

### 1. `normalize_method()` — Ecosystem-Wide JSON-RPC Dispatch Standard

**Sources**: groundSpring V121, neuralSpring V122, wetSpring V133, healthSpring V42, BearDog Waves 9-12, Songbird v0.2.1

Strips legacy prefixes (`primalspring.`, `barracuda.`, `biomeos.`, etc.) so
`primalspring.health.check` and `health.check` resolve identically. Also
handles `capability.list` → `capabilities.list` plural alias.

**Location**: `ecoPrimal/src/ipc/mod.rs::normalize_method()`

**For all teams**: Wire `normalize_method()` into your server dispatch to
accept prefixed method calls from any ecosystem caller.

### 2. `check_relative()` + `check_abs_or_rel()` — Robust Numeric Validation

**Sources**: groundSpring V120, wetSpring V133, healthSpring V42

Relative-tolerance checks for floating-point validation. `check_abs_or_rel`
passes if EITHER absolute OR relative tolerance is met, avoiding false
negatives near zero.

**Location**: `ecoPrimal/src/validation/mod.rs`

**For hotSpring, wetSpring, groundSpring**: These are your patterns — primalSpring
now has them for cross-spring numeric validation of coordination metrics.

### 3. `NdjsonSink` — Streaming Validation Output

**Sources**: groundSpring V121, wetSpring V133, neuralSpring V122

Newline-delimited JSON sink that emits one JSON object per check. Each line
is independently parseable — ideal for log aggregation, CI pipelines, and
cross-process streaming.

**Location**: `ecoPrimal/src/validation/mod.rs::NdjsonSink<W>`

**For all teams**: Use `NdjsonSink::stdout()` for machine-readable CI output,
or `NdjsonSink::new(file)` for log files. Replace custom JSON emission patterns.

### 4. `IpcError::is_recoverable()` — Broader Recovery Classification

**Sources**: neuralSpring V122, wetSpring V133, groundSpring V121

Broader than `is_retriable()` — includes transient failures AND server-reported
errors that may resolve if the primal is restarted. Excludes `MethodNotFound`
(permanent) and `SerializationError` (client bug).

**Location**: `ecoPrimal/src/ipc/error.rs`

**For all teams**: Use `is_recoverable()` for circuit breaker decisions,
`is_retriable()` for immediate retry decisions.

### 5. `Transport` Enum (Unix + Tcp) — Cross-Platform IPC

**Sources**: airSpring V010, healthSpring V42, groundSpring V121

Unified `Transport::Unix | Transport::Tcp` enum with `connect_transport("unix:/path")`
and `connect_transport("tcp:127.0.0.1:9100")` address parsing. Callers don't
need to know the underlying socket type.

**Location**: `ecoPrimal/src/ipc/transport.rs`

**For airSpring, healthSpring**: These are your patterns. primalSpring now has
cross-platform transport for non-Unix environments.

### 6. `ipc::probes` — `OnceLock`-Cached Runtime Probes

**Sources**: hotSpring V0.6.32, neuralSpring V122, groundSpring V121

`OnceLock`-cached runtime probes for test parallelism. `neural_api_reachable()`,
`beardog_reachable()`, `tower_reachable()` etc. probe once per process and
cache the result — safe for parallel `#[test]` execution.

**Location**: `ecoPrimal/src/ipc/probes.rs`

**For all teams**: Replace repeated `neural_bridge().is_some()` checks in tests
with cached probes. Eliminates flaky race conditions and saves probe time.

### 7. `missing_docs` → `deny` Workspace-Wide

All public items in primalSpring are fully documented. The lint level was
evolved from `warn` to `deny` so new public items without docs fail compilation.

**For all teams**: Consider the same upgrade when your documentation coverage
is complete.

### 8. `validate_release.sh` — Release Quality Gate

**Sources**: groundSpring V121, neuralSpring V122, wetSpring V133

Scripts: `scripts/validate_release.sh` — runs fmt, clippy, deny, test (with
count floor of 320), and doc build. Returns 0 only if all pass.

**For all teams**: Adopt a similar release gate. The test count floor prevents
accidentally deleting tests from passing CI.

### 9. Server Dispatch Wired Through `normalize_method()`

The primalspring primal server now normalizes all incoming method names before
dispatch. Any ecosystem caller can send `primalspring.health.check` or just
`health.check` and get the same response.

---

## Test Count Evolution

| Phase | Tests | Delta |
|-------|-------|-------|
| Phase 12.0 (pre-absorption) | 280 | — |
| Phase 12.1 (first absorption) | 303 | +23 |
| Phase 12.2 (deep absorption) | 360 | +57 |

---

## What Each Team Should Absorb

### All Springs
- `normalize_method()` → wire into your server dispatch
- `NdjsonSink` → use for CI machine-readable output
- `OnceLock` probes → replace redundant test-time discovery checks
- `validate_release.sh` → adopt test count floor + quality gate

### hotSpring
- `check_relative()` / `check_abs_or_rel()` → you originated these; primalSpring now uses them for coordination metrics

### groundSpring
- `normalize_method()` → you pioneered this; primalSpring's version handles all 9 ecosystem prefixes
- `NdjsonSink` → converged with your pattern

### neuralSpring
- `is_recoverable()` → broader than `is_retriable()` for your Neural API client retry logic
- `OnceLock` probes → cached model availability checks

### wetSpring
- `Transport` enum → cross-platform IPC for your biology simulation sockets
- `is_recoverable()` → refined retry decisions for your 354+ bins

### airSpring
- `Transport` enum → you originated this; primalSpring's version adds `connect_transport()` address parsing
- `missing_docs = "deny"` → match your documentation discipline

### healthSpring
- `check_abs_or_rel()` → you use both absolute and relative tolerances; this combines them
- `Transport` enum → converged with your pattern
- `validate_release.sh` → formalize your pre-release checklist

### Phase 1/2 Primals (BearDog, Songbird, ToadStool, NestGate, Squirrel, biomeOS)
- `normalize_method()` → accept prefixed calls from any ecosystem caller
- `OnceLock` probes → cached health checks for parallel test suites

---

## Files Modified

| File | Change |
|------|--------|
| `ecoPrimal/src/ipc/mod.rs` | `normalize_method()`, `probes` + `transport` module wiring |
| `ecoPrimal/src/ipc/error.rs` | `is_recoverable()` + 8 tests |
| `ecoPrimal/src/ipc/transport.rs` | **NEW** — `Transport` enum, `connect_transport()`, 5 tests |
| `ecoPrimal/src/ipc/probes.rs` | **NEW** — `OnceLock` cached probes, 8 tests |
| `ecoPrimal/src/validation/mod.rs` | `check_relative()`, `check_abs_or_rel()`, `NdjsonSink`, 11 tests |
| `ecoPrimal/src/bin/primalspring_primal/main.rs` | Dispatch wired through `normalize_method()` |
| `Cargo.toml` | `missing_docs` → `deny` |
| `scripts/validate_release.sh` | **NEW** — release quality gate |
| `CHANGELOG.md` | Phase 12.2 entries |
| `wateringHole/README.md` | Updated metrics, IPC stack, absorption table |
| `whitePaper/baseCamp/README.md` | Updated to 360 tests, Phase 12.2 items |

---

**License**: AGPL-3.0-or-later
