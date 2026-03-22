# Tower Co-Evolution Guide

**From**: primalSpring coordination  
**To**: BearDog, Songbird, biomeOS teams  
**Date**: 2026-03-18  
**Goal**: Stable Tower Atomic with full capability routing

## What This Is

primalSpring is the spring that validates coordination. We co-evolve with your teams
by running live integration tests against your binaries and reporting what passes and
what doesn't. This document defines the shared contract.

## The Sprint Loop

```
   Your team                    primalSpring
   ─────────                    ────────────
   Build + publish binary  ──→  Harvest to plasmidBin/
                                Run Tower integration suite
                           ←──  Gate report + delta handoff
   Fix flagged issues      ──→  Re-harvest, re-test
                           ←──  Updated gate report
   ... repeat until all gates green ...
```

Each cycle:
1. You build and place your binary in `ecoPrimals/plasmidBin/primals/`
2. primalSpring runs `cargo test --test server_integration -- tower`
3. Results go to `wateringHole/handoffs/` as delta reports
4. You fix, we re-test

## The Contract: 3 Standard Methods

Every primal in the Tower MUST respond to these three methods over JSON-RPC 2.0:

### `health.liveness`

```json
{"jsonrpc":"2.0","method":"health.liveness","params":{},"id":1}
```

Response: any valid JSON-RPC response (success or error) = alive.
A connection error or timeout = not alive.

### `capabilities.list`

```json
{"jsonrpc":"2.0","method":"capabilities.list","params":{},"id":2}
```

Response: JSON array of capability descriptors. Format flexible — primalSpring
handles 4 formats (string array, object array, method_info nested, semantic_mappings).

### `capability.call` (via Neural API only)

```json
{"jsonrpc":"2.0","method":"capability.call","params":{"capability":"crypto","operation":"generate_keypair","args":{}},"id":3}
```

This is how primals communicate with each other. NOT direct socket connections.

## Per-Team Quick Wins

### BearDog — 3 Changes

**Goal**: Unlock Gates 2.1, 2.3, and backward-compatible bare crypto aliases.

**Change 1** — `crates/beardog-tunnel/src/unix_socket_ipc/handlers/health.rs`

Add to `HealthHandler::methods()`:
```rust
vec!["ping", "health", "status", "check", "health.liveness", "health.readiness", "health.check"]
```

Add to `HealthHandler::handle()`:
```rust
"health.liveness" | "health.readiness" | "health.check" => { /* same as "health" */ }
```

**Change 2** — `crates/beardog-tunnel/src/unix_socket_ipc/handlers/capabilities.rs`

Add `"capabilities.list"` to `methods()` and route to existing `handle_capabilities()`.

**Change 3** — `crates/beardog-tunnel/src/unix_socket_ipc/handlers/mod.rs`

Add a pre-routing mapper in `HandlerRegistry::route()`:
```rust
let method = match method {
    "x25519_generate_ephemeral" => "crypto.x25519_generate_ephemeral",
    "sign_ed25519" => "crypto.sign",
    "hmac_sha256" => "crypto.hmac",
    // ... other bare names
    other => other,
};
```

This unblocks songbird TLS while songbird migrates to Neural API routing.

### Songbird — 2 Changes

**Goal**: Unlock Gates 2.2, 2.4, and begin Neural API routing for crypto.

**Change 1** — `crates/songbird-universal-ipc/src/service.rs`

Add `"health.liveness"` and `"capabilities.list"` as method aliases pointing to
existing `health` and `primal.capabilities` handlers.

**Change 2** — Start routing crypto through `RoutingMode::NeuralApi`

You already have the pattern in `songbird-http-client/src/crypto/beardog_provider.rs`.
Extract `BearDogProvider` with `RoutingMode::NeuralApi` into a shared crate.
Use it in:
- `songbird-tor-protocol/src/crypto/mod.rs` (replacing `BeardogCryptoClient`)
- `songbird-orchestrator/src/crypto/beardog_crypto_client.rs`
- `songbird-nfc/src/genesis.rs`
- `songbird-sovereign-onion/src/crypto.rs`

### biomeOS — 3 Changes

**Goal**: Unlock Gates 3.2, 3.3, 6.1, 6.2, 6.3. Eat your own dogfood.

**Change 1** — `config/capability_registry.toml`

Add missing domains:
```toml
[domains.genetic]
derive_lineage_key = { provider = "beardog", method = "genetic.derive_lineage_key" }
mix_entropy = { provider = "beardog", method = "genetic.mix_entropy" }
verify_lineage = { provider = "beardog", method = "genetic.verify_lineage" }
generate_lineage_proof = { provider = "beardog", method = "genetic.generate_lineage_proof" }

[domains.lineage]
verify_siblings = { provider = "beardog", method = "lineage.verify_siblings" }
verify_members = { provider = "beardog", method = "lineage.verify_members" }
```

**Change 2** — `crates/biomeos/src/modes/enroll.rs`

Replace:
```rust
let caller = DirectBeardogCaller::new(&beardog_socket);
```

With:
```rust
let caller = NeuralApiCapabilityCaller::new(&neural_api_socket);
```

Keep `DirectBeardogCaller` only for bootstrap (before Neural API is running).

**Change 3** — `crates/biomeos-graph/src/executor/node_handlers.rs`

Replace:
```rust
let socket = discover_beardog_socket(&context.env);
```

With:
```rust
let socket = discover_by_capability("security", &context.env);
```

## How primalSpring Validates

primalSpring runs these tests against your live binaries:

| Test | What It Checks |
|---|---|
| `tower_atomic_live_health_check` | Both primals respond to `health.liveness` within 5s |
| `tower_atomic_live_capabilities` | Both primals respond to `capabilities.list` with non-empty results |
| `tower_atomic_live_validation_result` | Full harness validation (health + caps + socket cleanup) |
| `tower_neural_api_health` | Neural API starts in COORDINATED MODE |
| `tower_neural_api_capability_discovery` | Capability queries route through Neural API |
| `tower_neural_api_full_validation` | End-to-end validation including capability translation |

Future tests (once gates unblock):
| Test | What It Checks |
|---|---|
| `tower_tls_handshake` | TLS 1.3 X25519 via capability routing |
| `tower_tls_internet_reach` | Full HTTPS to external endpoint |
| `tower_method_availability_audit` | Probe for both raw and semantic method names |
| `tower_capability_translation_coverage` | All raw methods have semantic mappings in registry |

## Squirrel as Reference

Squirrel is the gold standard. It already implements:
- `health.liveness` and `health.readiness`
- `capability.list`
- 5-tier socket discovery
- `capability_registry.toml`
- `deny(unwrap_used)`, `deny(expect_used)`

Look at Squirrel's `jsonrpc_server.rs` and `capability_registry.toml` for patterns.

## Timeline

| Sprint | Focus | Target Gates |
|---|---|---|
| Current | Standard methods (`health.liveness`, `capabilities.list`) | Gates 2.1–2.5 |
| Next | Neural API crypto routing (songbird TLS path) | Gates 3.1–3.5 |
| Next+1 | TLS 1.3 end-to-end | Gates 4.1–4.3 |
| Next+2 | Socket discovery alignment + dogfooding | Gates 5.x, 6.x |
| Next+3 | **Stable Tower** — all 24 gates green | Nest Atomic begins |
