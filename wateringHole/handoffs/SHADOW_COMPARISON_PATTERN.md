# Pattern: Shadow Comparison (A/B Validation)

**Origin**: songbird Wave 213 — TURN vs cloudflared parallel execution  
**Abstracted in**: `primalspring::validation::shadow`  
**Dissemination target**: All primals with alternative code paths

---

## Problem

When migrating between implementation strategies (transport protocols, providers,
algorithms), you need to run both paths in parallel and compare results to build
confidence before switching. This is especially common during:

- Protocol migrations (TURN → cloudflared, HTTP/2 → HTTP/3)
- Provider changes (local GPU → remote inference)
- Algorithm upgrades (HMAC-SHA256 → BLAKE3)

## Pattern

```rust
use primalspring::validation::shadow::{ShadowComparison, ShadowResult};

let result = ShadowComparison::run("transport", || {
    // Primary path — current production implementation
    let resp = current_transport.send(payload)?;
    Ok(blake3::hash(&resp).to_hex().to_string())
}, || {
    // Shadow path — candidate replacement
    let resp = new_transport.send(payload)?;
    Ok(blake3::hash(&resp).to_hex().to_string())
});

// Record on ValidationResult for structured reporting
result.record_on(&mut validation);

// Access metrics for dashboards
println!("latency ratio: {:.2}x", result.latency_ratio());
println!("outcomes match: {}", result.outcomes_match);
```

## Key Types

### `ShadowResult`

| Field | Type | Description |
|---|---|---|
| `label` | `String` | Comparison identifier |
| `primary_latency_us` | `u64` | Primary path wall-clock microseconds |
| `shadow_latency_us` | `u64` | Shadow path wall-clock microseconds |
| `outcomes_match` | `bool` | Whether outputs are identical |
| `primary_ok` / `shadow_ok` | `bool` | Whether each path succeeded |
| `primary_value` / `shadow_value` | `Option<String>` | Outputs (for debugging divergence) |

### `ShadowComparison::run(label, primary, shadow) -> ShadowResult`

Runs both closures sequentially, captures timing, compares string outputs.
Both closures return `Result<String, E>` where the string is a comparable
output (hash, status code, response body hash, etc).

## Integration with ValidationResult

Call `result.record_on(&mut v)` to emit three boolean checks:

1. `shadow_{label}_primary_ok` — primary path succeeded
2. `shadow_{label}_shadow_ok` — shadow path succeeded
3. `shadow_{label}_match` — outputs are identical

These integrate with primalSpring's JSON output mode (`PRIMALSPRING_JSON=1`)
and CI pipelines.

## Adoption Guidance

| Primal | Use Case |
|---|---|
| songbird | TURN vs cloudflared (already native) |
| toadStool | GPU driver hot-swap verification |
| coralReef | WGSL → SPIR-V compiler path comparison |
| bearDog | Session token issuance: purpose-based vs legacy |
| barraCuda | stat implementations: reference vs optimized |

## When NOT to Use

- Single-path validation (use regular `check_bool` / `check_float`)
- Paths with intentionally different outputs (use custom comparison)
- Latency-critical hot paths (shadow adds ~2x wall time)
