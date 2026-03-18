# primalSpring v0.2.0 — toadStool/barraCuda Compute Triangle Handoff

**Date:** March 18, 2026  
**From:** primalSpring (coordination spring)  
**To:** toadStool, barraCuda, coralReef teams  
**License:** AGPL-3.0-or-later  
**Covers:** Sovereign Compute Triangle validation, IPC resilience patterns, capability exposure requirements

---

## Executive Summary

- primalSpring v0.2.0 validates the Sovereign Compute Triangle (coralReef → toadStool → barraCuda) via **exp050**
- IPC resilience stack adopted ecosystem-wide: typed `IpcError`, `CircuitBreaker`, `RetryPolicy`, `DispatchOutcome<T>`
- 4-format capability parsing handles all known wire formats across the ecosystem
- `health.liveness` / `health.readiness` probes integrated as Kubernetes-style health checks
- **132 tests**, zero clippy warnings, zero unsafe, zero C deps

---

## 1. What primalSpring Validates (exp050: Compute Triangle)

exp050 probes all three Sovereign Compute Triangle primals and validates:

| Primal | Checks | Details |
|--------|--------|---------|
| **toadStool** | Socket discovery, `health.liveness`, `health.readiness`, latency < 500ms, `capabilities.list` | Probed via `probe_primal("toadstool")`, expects compute dispatch capabilities |
| **coralReef** | Socket discovery, `health.liveness`, `health.readiness`, latency < 500ms, `capabilities.list` | Probed via `probe_primal("coralreef")`, expects shader compilation capabilities |
| **barraCuda** | Socket discovery, `health.liveness`, `health.readiness`, latency < 500ms, `capabilities.list` | Probed via `probe_primal("barracuda")`, expects math primitive capabilities |

All checks use `check_or_skip()` — honest reporting when primals are not running.

### What exp050 Needs From Each Primal

```
toadStool:
  - Socket at $TOADSTOOL_SOCKET or $XDG_RUNTIME_DIR/biomeos/toadstool.sock
  - health.liveness → { "healthy": true }
  - health.readiness → { "ready": true, ... }
  - capabilities.list → any Format A/B/C/D response listing compute.* methods
  - Response latency < 500ms

coralReef:
  - Socket at $CORALREEF_SOCKET or $XDG_RUNTIME_DIR/biomeos/coralreef.sock
  - health.liveness → { "healthy": true }
  - health.readiness → { "ready": true, ... }
  - capabilities.list → shader compilation capabilities

barraCuda:
  - Socket at $BARRACUDA_SOCKET or $XDG_RUNTIME_DIR/biomeos/barracuda.sock
  - health.liveness → { "healthy": true }
  - health.readiness → { "ready": true, ... }
  - capabilities.list → math primitive capabilities
```

---

## 2. IPC Resilience Patterns (Ecosystem-Wide)

primalSpring absorbed and documented the converged IPC resilience stack from 7 sibling springs. These patterns are relevant for toadStool/barraCuda/coralReef IPC exposure:

### 2.1 Typed IpcError

8 semantic variants with query methods:

| Variant | `is_retriable()` | `is_timeout_likely()` |
|---------|------------------|-----------------------|
| `SocketNotFound` | false | false |
| `ConnectionRefused` | true | false |
| `ConnectionReset` | true | false |
| `Timeout` | true | true |
| `ProtocolError` | false | false |
| `MethodNotFound` | false | false |
| `ApplicationError` | false | false |
| `SerializationError` | false | false |

**toadStool/barraCuda action:** Return appropriate JSON-RPC error codes so clients can classify correctly. `-32601` for method-not-found, `-32602` for invalid params, `-32603` for internal errors.

### 2.2 DispatchOutcome\<T\>

Three-way classification enables smarter client behavior:

- `Success(T)` — call succeeded, result deserialized
- `ProtocolError(IpcError)` — transport failure, retryable
- `ApplicationError { code, message, data }` — server rejected the request, not retryable

**toadStool/barraCuda action:** Distinguish transport errors from application errors in responses. Use standard JSON-RPC error codes.

### 2.3 Health Probes

| Method | Purpose | Expected Response |
|--------|---------|-------------------|
| `health.liveness` | Is the process alive? | `{ "healthy": true }` |
| `health.readiness` | Ready to serve requests? | `{ "ready": true, "details": { ... } }` |

**toadStool/barraCuda action:** Implement both. `health.readiness` should report GPU availability, shader compilation status, and any blocking conditions.

### 2.4 4-Format Capability Parsing

primalSpring's `extract_capability_names()` handles all known ecosystem wire formats:

- **Format A** — `["compute.dispatch", "compute.status"]` (flat string array)
- **Format B** — `[{"method": "compute.dispatch"}]` (object array)
- **Format C** — `{"method_info": [{"name": "compute.dispatch"}]}` (nested)
- **Format D** — `{"semantic_mappings": {"compute": {"dispatch": {}}}}` (double-nested)

**toadStool/barraCuda action:** Any of these formats works. Format A is simplest.

---

## 3. What primalSpring Does NOT Validate

primalSpring validates **coordination** — that primals start, discover each other, respond to health checks, and compose correctly. It does NOT validate:

- barraCuda mathematical correctness (that's hotSpring/wetSpring/airSpring/etc.)
- toadStool GPU dispatch performance (that's the domain springs)
- coralReef shader compilation correctness (that's hotSpring)
- WGSL shader semantics (zero WGSL in primalSpring)

---

## 4. barraCuda Usage in primalSpring

primalSpring has **zero** barraCuda code dependencies. The relationship is IPC-only:

| Aspect | primalSpring | Domain Springs |
|--------|-------------|----------------|
| barraCuda import | None | `barracuda::*` in Cargo.toml |
| WGSL shaders | None | 767+ across ecosystem |
| Math primitives | None | stats, linalg, ops, spectral, nautilus |
| Compute dispatch | IPC via `probe_primal("barracuda")` | Direct `compute.dispatch.*` |

This is by design: primalSpring validates coordination, not math.

---

## 5. Patterns From Sibling Springs Worth Absorbing

Based on the ecosystem-wide survey conducted for primalSpring v0.2.0:

| Pattern | Source | Relevance to toadStool/barraCuda |
|---------|--------|----------------------------------|
| `PRIMAL_NAME` / `PRIMAL_DOMAIN` constants | Ecosystem standard | Every primal should have self-knowledge constants for logging and IPC |
| `safe_cast` module | groundSpring, airSpring | Prevent `as u64` truncation in metrics and timing code |
| `OrExit<T>` trait | groundSpring, ludoSpring | Clean exits for validation binaries instead of `unwrap()` |
| `FAMILY_ID`-aware discovery | groundSpring | Socket paths incorporate `$FAMILY_ID` for multi-tower isolation |
| proptest for IPC | primalSpring, neuralSpring | Property tests ensure protocol round-trips and parser robustness |

---

## 6. Evolution Requests for toadStool/barraCuda/coralReef

### 6.1 High Priority

1. **Implement `health.liveness` and `health.readiness`** on all three primals — this is the primary blocker for primalSpring Phase 3 (live primal integration)
2. **Ensure `capabilities.list` returns a parseable response** in any of the 4 formats
3. **Expose socket at standard paths** (`$XDG_RUNTIME_DIR/biomeos/<name>.sock` or `$<NAME>_SOCKET` env var)

### 6.2 Medium Priority

4. **Return standard JSON-RPC error codes** (-32601, -32602, -32603) so `IpcError` classification works correctly
5. **Add `PRIMAL_NAME` / `PRIMAL_DOMAIN` constants** if not already present
6. **Support `FAMILY_ID` in socket paths** for multi-tower isolation

### 6.3 Low Priority (Future)

7. **Expose `compute.dispatch.status`** for primalSpring to validate dispatch pipeline health
8. **Expose shader compilation status** via `health.readiness` details
9. **Protocol escalation readiness** — exp052 will validate HTTP → JSON-RPC → tarpc negotiation

---

## 7. Metrics

| Metric | Value |
|--------|-------|
| Tests | 132 |
| Experiments | 38 (7 tracks) |
| Compute triangle experiment | exp050 |
| IPC modules | 7 (client, discover, protocol, error, resilience, dispatch, extract) |
| Clippy warnings | 0 (pedantic + nursery) |
| Unsafe code | 0 (workspace forbid) |
| C dependencies | 0 |

---

## References

- `primalSpring/experiments/exp050_compute_triangle/src/main.rs` — compute triangle validation
- `primalSpring/wateringHole/PRIMALSPRING_COMPOSITION_GUIDANCE.md` — composition patterns
- `primalSpring/specs/BARRACUDA_REQUIREMENTS.md` — minimal barraCuda requirements
- `primalSpring/specs/SHOWCASE_MINING_REPORT.md` — patterns mined from phase1/phase2
- `ecoPrimals/wateringHole/BARRACUDA_LEVERAGE_GUIDE.md` — barraCuda capabilities
