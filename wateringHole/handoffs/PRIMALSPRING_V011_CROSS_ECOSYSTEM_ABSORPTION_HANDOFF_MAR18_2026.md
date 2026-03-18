# primalSpring V0.1.1 — Cross-Ecosystem Absorption Handoff

**Date**: 2026-03-18  
**Previous**: `PRIMALSPRING_V010_NEURAL_API_EVOLUTION_HANDOFF_MAR17_2026.md`  
**Tests**: 127 (from 69)  
**Clippy**: 0 warnings  
**Status**: Phase 2 — ecosystem pattern convergence

## What Changed

### 1. IPC Resilience: CircuitBreaker + RetryPolicy (ipc/resilience.rs)

Absorbed the ecosystem-wide circuit breaker and retry pattern from
healthSpring/wetSpring/groundSpring/neuralSpring. The `resilient_call()`
free function wraps IPC calls with:

- **CircuitBreaker**: Closed/Open/HalfOpen state machine — fails fast
  against demonstrably-down primals after configurable failure threshold
- **RetryPolicy**: Exponential backoff with configurable max delay,
  only retries when `IpcError::is_retriable()` returns true

Wired into `coordination::health_check()` for resilient per-primal probing.

### 2. Semantic IpcError (ipc/error.rs)

Replaced the flat `IpcError` variants with semantically classified errors:
`SocketNotFound`, `ConnectionRefused`, `ConnectionReset`, `Timeout`,
`ProtocolError`, `MethodNotFound`, `ApplicationError`, `SerializationError`.

Added query helpers: `is_retriable()`, `is_timeout_likely()`,
`is_method_not_found()`, `is_connection_error()`. These enable smart
retry decisions in `CircuitBreaker` and `RetryPolicy`.

Added `classify_io_error()` for mapping raw `std::io::Error` to semantic variants.

### 3. DispatchOutcome (ipc/dispatch.rs)

Three-way classification of JSON-RPC results:
- `Success(T)` — call succeeded
- `ProtocolError(IpcError)` — transport failure (retriable)
- `ApplicationError { code, message, data }` — server-side rejection

`should_retry()` delegates to `IpcError::is_retriable()` for protocol errors.

### 4. Typed Result Extraction (ipc/extract.rs)

`extract_rpc_result<T>()` and `extract_rpc_dispatch<T>()` replace ad-hoc
`response.result.unwrap()` with safe typed extraction that handles errors,
missing results, and deserialization in one call.

### 5. Health Probes: Liveness + Readiness

Server now exposes:
- `health.liveness` — am I alive? (same as `health.check`)
- `health.readiness` — am I ready? (reports Neural API status + discovered primal count)

Client adds `health_liveness()` and `health_readiness()` with fallback to
`health.check` for primals that don't implement the K8s-style probes.

### 6. Self-Knowledge Constants (lib.rs)

`PRIMAL_NAME = "primalspring"` and `PRIMAL_DOMAIN = "coordination"` —
single source of truth for self-knowledge. Server binary now uses these
instead of hardcoded strings in all JSON-RPC responses and log messages.

### 7. OrExit Trait (validation/or_exit.rs)

Zero-panic exit for validation binaries. `.or_exit(msg)` replaces verbose
`let Ok(v) = expr else { eprintln!(...); process::exit(1); }` boilerplate.
Implemented for both `Result<T, E>` and `Option<T>`.

### 8. ValidationResult::with_provenance()

Added `provenance: Option<String>` field and `.with_provenance()` builder.
Skipped from JSON when `None`. Survives round-trip serialization.

### 9. ValidationSink Trait

Pluggable output for validation checks — `StdoutSink` (default) and
`NullSink` (tests). Enables test harnesses to capture output without stdout.

### 10. Safe Cast Module (cast.rs)

`u128_to_u64`, `micros_u64`, `usize_to_u32`, `usize_to_u64`, `f64_to_usize`.
Replaces all `as` casts in the codebase with saturating/clamping alternatives.

### 11. wateringHole Handoff Hygiene

Archived 22 superseded handoffs in `ecoPrimals/wateringHole/handoffs/archive/`.
Kept only the latest per-primal. Fixed primalSpring's own handoff locations.

## Patterns NOT Absorbed (and why)

- **Graph DAG execution** — primalSpring validates, not executes
- **Tolerance tiers** (eps::, tol::) — coordination latency, not numerical accuracy
- **GPU dispatch** — zero GPU workload
- **barracuda math** — zero math dependency
- **NeuralBridge reimplementation** — already uses canonical `neural-api-client-sync`

## Files Changed

| File | Change |
|------|--------|
| `ipc/resilience.rs` | CircuitBreaker, RetryPolicy, resilient_call |
| `ipc/error.rs` | Semantic IpcError with query helpers |
| `ipc/dispatch.rs` | DispatchOutcome<T> three-way classification |
| `ipc/extract.rs` | Typed result extraction |
| `ipc/client.rs` | Uses error.rs, adds liveness/readiness |
| `ipc/mod.rs` | Registers all modules, re-exports IpcError |
| `coordination/mod.rs` | Uses resilient_call, safe casts |
| `validation/mod.rs` | with_provenance, ValidationSink |
| `validation/or_exit.rs` | OrExit trait |
| `cast.rs` | Safe numeric casts |
| `lib.rs` | PRIMAL_NAME, PRIMAL_DOMAIN, cast module |
| `bin/primalspring_primal/main.rs` | health.liveness, health.readiness, constants |

## Test Summary

127 tests, 0 failures, 0 Clippy warnings.

## Next Phase

- Wire resilient_call into probe_primal() for retry on transient connect failures
- Add circuit breaker state per-primal (currently per-call)
- Integrate provenance with the Provenance Trio when available
- Expand experiments to use OrExit and DispatchOutcome
