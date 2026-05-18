# Signal Adoption Standard — Neural API Composition Collapse for Springs

**Version**: 1.0  
**Date**: May 16, 2026  
**Author**: primalSpring (reference implementation)  
**Status**: Active — primalSpring v0.9.25+ (Wave 17)

## Overview

This document defines the migration path from the flat 456-method RPC surface
to the Neural API's atomic signal dispatch model. Springs that adopt this
standard replace explicit multi-call orchestration with single `dispatch()`
calls that let biomeOS execute provenance graphs on behalf of the caller.

**Composition collapse** = reducing N sequential method calls to 1 signal
dispatch, where biomeOS manages sequencing, error recovery, and provenance.

## The Problem

Springs currently compose science workloads via low-level method calls:

```rust
// 4 calls, spring manages sequencing and error propagation
let hash = ctx.call("content", "content.put", data)?;
let event = ctx.call("dag", "dag.event.append", event_params)?;
let commit = ctx.call("spine", "spine.seal", vertex_params)?;
let braid = ctx.call("braid", "braid.create", braid_params)?;
```

This forces every spring to:
- Know which primals handle which capabilities
- Manage call ordering and rollback
- Understand the full method surface (456+ methods)
- Handle partial failures across primal boundaries

## The Solution: Signal Dispatch

```rust
// 1 call, biomeOS manages the graph
let result = ctx.dispatch("nest.store", serde_json::json!({
    "content": data,
    "author": "wetSpring:ltee-b7",
}))?;
```

biomeOS decomposes `nest.store` into the full provenance graph:
`NestGate.content.put → rhizoCrypt.dag.event.append → loamSpine.spine.seal → sweetGrass.braid.create`

The spring gets back a composed result with all provenance artifacts.

## Migration Guide

### Step 1: Registration — `method.register` → `primal.announce`

**Before** (3 separate RPC calls):
```rust
rpc::send(biomeos, "method.register", json!({ "primal": "airspring", ... }));
rpc::send(biomeos, "capability.register", json!({ ... }));
rpc::send(biomeos, "lifecycle.register", json!({ ... }));
```

**After** (single announce):
```rust
ctx.announce(
    "airspring",
    &["ag.measure", "ag.calibrate", "ag.predict"],
    Path::new("/run/ecoprimals/airspring-family.sock"),
)?;
```

Or raw JSON-RPC per the [Primal Announce Protocol](./PRIMAL_ANNOUNCE_PROTOCOL.md):
```json
{
  "method": "primal.announce",
  "params": {
    "primal": "airspring",
    "socket": "/run/ecoprimals/airspring-family.sock",
    "capabilities": ["agriculture"],
    "methods": ["ag.measure", "ag.calibrate", "ag.predict"],
    "signal_tiers": ["node"],
    "version": "1.2.0"
  }
}
```

### Step 2: Capability Calls — `ctx.call()` → `ctx.dispatch()`

Identify call sequences in your spring that correspond to atomic signals.
The 14 signals are defined in `config/signal_tools.toml`:

| Signal | Tier | Replaces |
|--------|------|----------|
| `tower.publish` | tower | bearDog.sign + songbird.announce + skunkBat.audit |
| `tower.authenticate` | tower | bearDog.negotiate + skunkBat.verify_lineage |
| `tower.discover` | tower | songbird.discover + bearDog.verify + skunkBat.audit |
| `tower.health` | tower | bearDog.health + songbird.health + skunkBat.health |
| `tower.bootstrap` | tower | Phase 1 cold start + Phase 2 registry seed |
| `node.compute` | node | toadStool.dispatch + coralReef.compile + barraCuda.execute |
| `nest.store` | nest | NestGate.put + rhizoCrypt.append + loamSpine.seal + sweetGrass.braid |
| `nest.commit` | nest | rhizoCrypt.dehydrate + bearDog.sign + NestGate.store + loamSpine.seal |
| `nest.retrieve` | nest | NestGate.get + loamSpine.state + sweetGrass.provenance |
| `meta.observe` | meta | petalTongue.session + squirrel.context + biomeOS.graphs |
| `meta.intent` | meta | petalTongue.capture + squirrel.plan + biomeOS.dispatch |
| `meta.render` | meta | biomeOS.collect + squirrel.summarize + petalTongue.render |
| `meta.health` | meta | biomeOS.health + squirrel.health + petalTongue.health |
| `meta.deploy` | meta | squirrel.plan + biomeOS.deploy + skunkBat.audit |

### Step 3: Plan Mode — Multi-Signal Workflows

For complex workloads requiring multiple signals, use the squirrel-powered
plan API:

```rust
let plan = ctx.signal_plan("Reproduce LTEE generation B7 and publish results")?;
// squirrel decomposes this into:
// 1. node.compute (LTEE simulation)
// 2. nest.store (results + provenance)
// 3. tower.publish (signed announcement)

let results = ctx.execute_plan(plan)?;
```

## Spring Archetype Examples

### Compute-Heavy (e.g., hotSpring, wetSpring)

**Before:**
```rust
let result = ctx.call("tensor", "stats.mean", json!({"data": measurements}))?;
ctx.call("content", "content.put", json!({"data": result}))?;
ctx.call("dag", "dag.event.append", json!({"event": "computation", "hash": hash}))?;
```

**After:**
```rust
let result = ctx.call("tensor", "stats.mean", json!({"data": measurements}))?;
ctx.dispatch("nest.store", json!({
    "content": result,
    "author": "hotSpring:experiment-42",
}))?;
```

Domain-specific math calls (`stats.mean`, `gpu.matmul`) stay as `ctx.call()`
because they are direct capability operations, not composed workflows. The
signal replaces only the provenance/storage orchestration.

### Provenance-Heavy (e.g., groundSpring, deepSpring)

**Before:**
```rust
let session = ctx.call("dag", "dag.session.create", json!({}))?;
for event in events {
    ctx.call("dag", "dag.event.append", json!({"session": session, "event": event}))?;
}
ctx.call("dag", "dag.dehydration.trigger", json!({"session": session}))?;
ctx.call("spine", "spine.seal", json!({"session": session}))?;
ctx.call("certificate", "certificate.mint", json!({"session": session}))?;
```

**After:**
```rust
// Each event is still a method call (fine-grained DAG operations)
let session = ctx.call("dag", "dag.session.create", json!({}))?;
for event in events {
    ctx.call("dag", "dag.event.append", json!({"session": session, "event": event}))?;
}
// The commit/seal/mint orchestration collapses into one signal
ctx.dispatch("nest.commit", json!({ "session_id": session }))?;
```

### Content-Heavy (e.g., mossSpring, leafSpring)

**Before:**
```rust
let cid = ctx.call("content", "content.put", json!({"data": blob}))?;
let state = ctx.call("spine", "session.state", json!({"cid": cid}))?;
let provenance = ctx.call("braid", "braid.get", json!({"cid": cid}))?;
```

**After:**
```rust
ctx.dispatch("nest.store", json!({ "content": blob, "author": author }))?;
// Later retrieval is also collapsed:
let result = ctx.dispatch("nest.retrieve", json!({
    "content_cid": cid,
    "include_provenance": true,
}))?;
```

## Niche Signal Definition (Future)

Springs that identify repeated multi-capability workflows within their domain
can propose new signal graphs. The pattern:

1. Define a TOML graph in `graphs/signals/{tier}_{name}.toml`
2. Add the signal to `config/signal_tools.toml`
3. Register the signal tier in `config/capability_registry.toml`
4. Submit a wateringHole proposal

This is future work — the 14 foundation signals cover all current ecosystem
composition patterns.

## Fallback Behavior

Both `dispatch()` and `announce()` include automatic fallback:

- `dispatch()`: falls back to `capability.call` if `signal.dispatch` is
  unavailable (pre-v3.56 biomeOS)
- `announce()`: falls back to `method.register` if `primal.announce` is
  unavailable (pre-v3.57 biomeOS)

Springs can adopt the new API immediately without requiring a biomeOS upgrade.

**Endorsed pattern** (`announce_or_register`): Several springs (groundSpring,
airSpring, healthSpring, wetSpring) have implemented an `announce_or_register()`
wrapper that tries `ctx.announce()` first and falls back to the legacy 3-call
pattern. This is the **canonical backward-compatible approach** and is endorsed
by primalSpring. Springs that alias `primal.announce` to their existing
`lifecycle.register` handler (as ludoSpring does) are also correctly aligned.

## Validation

primalSpring includes validation scenarios that exercise the signal API:

- `s_signal_dispatch_parity` — dispatches all 14 signals and validates response shapes
- `s_primal_announce` — validates announce wire format and live registration
- `s_atomic_signals` — structural validation + live dispatch for each signal graph
- `s_provenance_trio_pipeline` — Phase 6 validates `nest.store` signal dispatch

Springs should add similar dispatch-based validation phases to their own
experiment scenarios.

## References

- [Primal Announce Protocol](./PRIMAL_ANNOUNCE_PROTOCOL.md)
- `config/signal_tools.toml` — canonical signal schema
- `config/capability_registry.toml` — method/signal registry
- `ecoPrimal/src/composition/context.rs` — `dispatch()`, `announce()`, `signal_plan()`, `execute_plan()`
