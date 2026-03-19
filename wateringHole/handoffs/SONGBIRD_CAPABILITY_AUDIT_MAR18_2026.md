# Songbird Capability-Based Compliance Audit

**From**: primalSpring coordination (v0.3.5)  
**To**: Songbird team  
**Date**: 2026-03-18  
**Severity**: High — direct beardog coupling blocks capability-based composition

## Executive Summary

Songbird v3.33.0 has a split personality: `songbird-http-client` supports Neural API routing (`BEARDOG_MODE=neural`), but `songbird-tor-protocol`, `songbird-orchestrator`, `songbird-nfc`, and `songbird-sovereign-onion` all hardcode direct beardog JSON-RPC calls. This is the #1 blocker for swappable crypto providers.

## Critical Fixes

### 1. Register `health.liveness` and `capabilities.list`

**Current**: Songbird exposes `health` (not `health.liveness`) and `primal.capabilities` (not `capabilities.list`)  
**Impact**: All primalSpring live tests and biomeOS health probes return "Unknown method"  
**Fix**: Add `health.liveness`, `health.readiness`, `capabilities.list` as aliases in `service.rs`

### 2. Route ALL BearDog crypto through Neural API

**Non-compliant crates** (all use `BeardogCryptoClient` with direct JSON-RPC):

| Crate | Raw Methods Called |
|---|---|
| `songbird-tor-protocol` | `crypto.ntor.*`, `crypto.x25519.*`, `crypto.sign.ed25519`, `crypto.hash.sha3_256`, `crypto.hmac.sha256`, `crypto.aead.chacha20_poly1305_*` |
| `songbird-orchestrator` | `crypto.sign_ed25519`, `crypto.x25519_generate_ephemeral`, `crypto.x25519_derive_secret`, `crypto.chacha20_poly1305_*`, `crypto.hmac_sha256` |
| `songbird-nfc` | `crypto.generate_x25519_keypair`, `crypto.x25519_dh`, `crypto.ed25519_sign`, `crypto.chacha20poly1305_*` |
| `songbird-sovereign-onion` | `crypto.chacha20_poly1305_*` (also uses local chacha20poly1305 crate directly) |

**Compliant**: `songbird-http-client` (`BearDogProvider` with `RoutingMode::NeuralApi`)

**Evolution**: Extract `BearDogProvider`'s Neural API routing pattern into a shared `capability-crypto-client` crate. Replace `BeardogCryptoClient` in all crates with this capability-based client.

### 3. TLS 1.3 X25519 Path

**The exact failure path** in our live Tower test:

1. `TlsHandshake::handshake()` → `crypto.generate_x25519_keypair()`
2. `BearDogProvider::from_env()` — checks `BEARDOG_MODE`
3. If `direct`: connects to beardog socket, calls `x25519_generate_ephemeral` (bare) → **FAILS** (beardog only has `crypto.x25519_generate_ephemeral`)
4. If `neural`: connects to Neural API, calls `capability.call("crypto", "generate_keypair")` → Neural API translates → **WORKS**

**Fix**: Default to Neural API mode. Or update the `direct` path to use `crypto.` prefix.

## Inter-Primal Coupling Inventory

| Location | Coupling | Evolution |
|---|---|---|
| `songbird-tor-protocol/src/crypto/mod.rs:33-48` | `BEARDOG_SOCKET` env, hardcoded paths | `discover_by_capability("crypto")` |
| `songbird-orchestrator/src/crypto/discovery.rs` | `BEARDOG_CRYPTO_SOCKET`, `beardog.sock` | `discover_by_capability("crypto")` |
| `songbird-quic/src/config.rs:102-139` | `BEARDOG_SOCKET`, fallback paths | `discover_by_capability("crypto")` |
| `songbird-orchestrator/src/primal_discovery.rs:74` | `NESTGATE_SOCKET`, `["storage", "nestgate"]` | `discover_by_capability("storage")` |
| `songbird-capabilities/adapter/capability_query.rs:236` | `name.contains("beardog")` | Capability-based type inference |

## Socket Discovery

Songbird's orchestrator crypto discovery is 7 tiers with mixed capability and identity names:

1. `CRYPTO_PROVIDER_SOCKET` (capability-ish)
2. `CRYPTO_PROVIDER` (capability)
3. `BEARDOG_CRYPTO_SOCKET` (identity)
4. `BEARDOG_SOCKET` (identity)
5. `crypto.sock` (capability name)
6. `beardog.sock` (identity name)
7. Filesystem scan for `["crypto", "security", "beardog"]`

**Evolution**: Drop identity-based tiers (3, 4, 6). Keep capability-based tiers. Add Neural API as tier 0.

## What's Good

- `songbird-http-client` already has the right pattern (`BearDogProvider` with `RoutingMode::NeuralApi`)
- `forbid(unsafe_code)` across workspace
- Pure Rust TLS (no ring/openssl in main path)
- `primal.info` and `primal.capabilities` implemented
- Rich RPC method set (STUN, IGD, relay, discovery, mesh, punch, onion, tor, HTTP)
