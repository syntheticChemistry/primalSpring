# BatchGuard Migration Guide — From Per-Op Calls to Fused Pipelines

**Date**: April 13, 2026
**Owner**: primalSpring (on behalf of barraCuda Sprint 42)
**For**: All downstream springs using tensor/compute composition
**License**: AGPL-3.0-or-later

---

## What Changed

barraCuda Sprint 42 introduced `tensor.batch.submit` — the 32nd JSON-RPC method.
This replaces the pattern of sequential per-operation IPC calls with a single
fused pipeline call that executes multiple tensor operations in one round-trip.

The old `TensorSession` API is deprecated. `BatchGuard` is the stable replacement.

---

## Before: Per-Op Sequential Calls

```rust
// 3 IPC round-trips, 3 connection setups, 3 response parses
let mean = ctx.call("tensor", "stats.mean", json!({"data": [1.0, 2.0, 3.0]}))?;
let matmul = ctx.call("tensor", "tensor.matmul", json!({
    "a": {"data": [1,0,0,1], "shape": [2,2]},
    "b": {"data": [2,0,0,2], "shape": [2,2]},
}))?;
let hash = ctx.call("security", "crypto.hash", json!({"data": "result"}))?;
```

---

## After: Fused Batch Pipeline

```rust
// 1 IPC round-trip for all tensor ops
let batch = ctx.call("tensor", "tensor.batch.submit", json!({
    "operations": [
        {"op": "stats.mean", "params": {"data": [1.0, 2.0, 3.0]}},
        {"op": "tensor.matmul", "params": {
            "a": {"data": [1,0,0,1], "shape": [2,2]},
            "b": {"data": [2,0,0,2], "shape": [2,2]},
        }},
    ]
}))?;

// Results come back as an array matching operation order
let results = batch.get("results").and_then(|r| r.as_array());
```

---

## Wire Contract

`tensor.batch.submit` follows the tensor wire contract (`TENSOR_WIRE_CONTRACT.md`):

**Request**:
```json
{
    "operations": [
        {"op": "stats.mean", "params": {"data": [1.0, 2.0, 3.0]}},
        {"op": "tensor.matmul", "params": {"a": {...}, "b": {...}}}
    ]
}
```

**Response** (Category 3 — batch):
```json
{
    "batch_id": "...",
    "status": "completed",
    "results": [
        {"status": "ok", "value": 2.0, "op": "stats.mean"},
        {"status": "ok", "result_id": "...", "shape": [2,2], "op": "tensor.matmul"}
    ]
}
```

**Error handling**: If any operation in the batch has invalid params, the entire
batch returns `INVALID_PARAMS` (-32602) without executing — even without a GPU.
This is the Sprint 42 Phase 8 pre-validation behavior.

---

## Response Categories

Per `TENSOR_WIRE_CONTRACT.md` v1.0.0:

| Category | Operations | Response Shape |
|----------|-----------|----------------|
| 1 — tensor-producing | `tensor.matmul`, `tensor.fft`, `tensor.sort` | `{result_id, shape, elements}` |
| 2 — scalar | `stats.mean`, `stats.variance`, `tensor.dot` | `{value, op, status}` |
| 3 — batch | `tensor.batch.submit` | `{batch_id, results: [...]}` |

---

## Migration Checklist

1. **Identify sequential tensor calls** in your composition code
2. **Group related ops** into a single `tensor.batch.submit` call
3. **Parse the `results` array** — index matches operation order
4. **Handle errors per-op**: individual operations can fail within a batch
5. **Keep cross-domain calls separate**: `tensor.batch.submit` only batches
   tensor ops — security, storage, etc. remain separate `ctx.call()` invocations

---

## Composition Context Usage

```rust
use primalspring::composition::CompositionContext;

let mut ctx = CompositionContext::from_live_discovery_with_fallback();

// Batch submit works through both Direct and Gateway routes
let result = ctx.call("tensor", "tensor.batch.submit", serde_json::json!({
    "operations": [
        {"op": "stats.mean", "params": {"data": my_data}},
        {"op": "tensor.softmax", "params": {"data": logits}},
    ]
}))?;
```

---

## When NOT to Use Batch

- **Single operations** — overhead of batch wrapping isn't worth it for one call
- **Cross-domain pipelines** — batch only handles tensor domain; use separate calls
  for security, storage, etc.
- **Latency-critical single-op** — batch pre-validation adds marginal overhead

---

## Deprecated API

`TensorSession` (Sprint 40 alias) still works but will be removed. Migrate to
`tensor.batch.submit` which is the stable wire method name.

---

**License**: AGPL-3.0-or-later
