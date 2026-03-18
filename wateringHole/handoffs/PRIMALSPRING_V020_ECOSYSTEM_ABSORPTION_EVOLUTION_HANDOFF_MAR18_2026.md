# primalSpring v0.2.0 — Ecosystem Absorption Evolution

**Date:** March 18, 2026  
**Previous:** PRIMALSPRING_V010_DEEP_DEBT_AUDIT_EVOLUTION_HANDOFF_MAR17_2026

---

## Executive Summary

primalSpring absorbed converged IPC resilience patterns from 7 sibling springs (wetSpring V127, healthSpring V35, groundSpring V114, airSpring V089, neuralSpring V115, ludoSpring V24, hotSpring v0.6.31) and leveraged capabilities from 12 phase1/phase2 primals. Evolution from v0.1.0 (69 tests) to v0.2.0 (157 tests: 148 unit + 9 integration).

---

## Changes Made

### P0: IPC Resilience Stack

- **`ipc/error.rs`**: Typed `IpcError` (8 variants) with semantic query methods (`is_retriable`, `is_timeout_likely`, `is_method_not_found`, `is_connection_error`)
- **`ipc/resilience.rs`**: `CircuitBreaker` (closed/open/half-open), `RetryPolicy` (exponential backoff), `resilient_call()` wrapper
- **`ipc/dispatch.rs`**: `DispatchOutcome<T>` (Success/ProtocolError/ApplicationError) with `should_retry()`
- **`ipc/extract.rs`**: `extract_rpc_result<T>()` and `extract_rpc_dispatch<T>()` for centralized JSON-RPC result handling
- **`discover.rs`** evolved: 4-format capability parsing (Formats A-D), `health.liveness`/`health.readiness` probes
- **`client.rs`** evolved: uses typed `IpcError`, added `health_liveness()`, `health_readiness()` methods

### P1: Safety & Validation Patterns

- **`cast.rs`**: Safe numeric casts (`micros_u64`, `u128_to_u64`, `usize_to_u32`, `f64_to_usize`)
- **`validation/or_exit.rs`**: `OrExit<T>` trait for clean validation binary exits
- **`PRIMAL_NAME`**/ **`PRIMAL_DOMAIN`** constants replacing hardcoded strings
- **`ValidationSink`** trait with `StdoutSink` and `NullSink`

### P2: Experiment Evolution

- **exp050**: Full probe pattern for Sovereign Compute Triangle (toadStool + coralReef + barraCuda) with health.liveness/readiness
- **exp010-015**: Neural API health checks, biomeOS graph deployment readiness
- **exp020-025**: Provenance trio probe patterns, cross-spring ecology discovery
- **exp030-034**: FAMILY_ID-aware socket discovery, Tower primitive probes

### P3: Niche & Deploy (Phase 2b)

- **`niche.rs`**: Self-knowledge module — 21 capabilities, semantic mappings, operation dependencies, cost estimates, `register_with_target()` for biomeOS registration
- **`deploy.rs`**: Deploy graph parsing — `load_graph()`, `validate_structure()`, `validate_live()`, `validate_all_graphs()` for 6 biomeOS TOMLs
- **`validate_all`**: Meta-validator binary runs all 38 experiments in sequence
- **Server wiring**: `capabilities.list` returns structured niche knowledge; `graph.list` and `graph.validate` RPC methods added; niche registration on startup (background thread, non-blocking)

### P4: Testing Evolution

- **proptest**: IPC protocol fuzzing — 5 property tests (request round-trip, response parse robustness, success/error parsing, notify generation)
- **Integration tests**: 9 real JSON-RPC round-trip tests against live server (isolated socket per test)

---

## Metrics

| Metric | v0.1.0 | v0.2.0 |
|--------|--------|--------|
| Tests | 69 | 157 (128% increase) |
| Unit tests | — | 148 |
| Integration tests | — | 9 (real IPC round-trips) |
| New source files | — | 6 (error.rs, resilience.rs, dispatch.rs, extract.rs, niche.rs, deploy.rs) |
| New binaries | — | validate_all (meta-validator) |
| Clippy warnings | — | 0 (pedantic + nursery) |
| Unsafe code | — | 0 (workspace forbid) |
| Formatting issues | — | 0 |
| Experiments evolved | — | 38 with real discovery/probe patterns |

---

## What Blocks Phase 1

- Live primals for integration testing
- biomeOS graph executor for actual graph deployment
- Provenance Trio deployment for RootPulse end-to-end
- Songbird registration for cross-tower federation
- coralReef + toadStool for sovereign compute triangle validation

---

## Patterns Absorbed From Ecosystem

| Pattern | Source Springs |
|---------|----------------|
| Typed IpcError | wetSpring, healthSpring, groundSpring, airSpring, neuralSpring, ludoSpring |
| CircuitBreaker + RetryPolicy | wetSpring, healthSpring, airSpring |
| `DispatchOutcome<T>` | loamSpine, airSpring, healthSpring |
| `extract_rpc_result()` | wetSpring, ludoSpring |
| 4-format capability parsing | airSpring, rhizoCrypt, toadStool |
| health.liveness/readiness | rhizoCrypt, wetSpring, airSpring |
| safe_cast module | groundSpring, airSpring, neuralSpring |
| `OrExit<T>` | groundSpring, ludoSpring |
| PRIMAL_NAME constant | ecosystem standard |
| FAMILY_ID discovery | groundSpring |
| ValidationSink | airSpring, rhizoCrypt |
