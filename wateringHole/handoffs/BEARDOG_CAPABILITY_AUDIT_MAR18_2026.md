# BearDog Capability-Based Compliance Audit

**From**: primalSpring coordination (v0.3.5)  
**To**: BearDog team  
**Date**: 2026-03-18  
**Severity**: Medium — quick fixes unlock live Tower TLS

## Executive Summary

BearDog v0.9.0 is architecturally sound (pure Rust, `forbid(unsafe_code)`, 90+ crypto methods). Three quick method-registration fixes unblock the live Tower TLS 1.3 stack and ecosystem-wide `health.liveness` probing.

## Critical Fixes (Priority Order)

### 1. Register `health.liveness` and `health.readiness`

**File**: `crates/beardog-tunnel/src/unix_socket_ipc/handlers/health.rs`  
**Current**: Registers `ping`, `health`, `status`, `check`  
**Missing**: `health.liveness`, `health.readiness`, `health.check`  
**Impact**: Every primalSpring live test and biomeOS health probe fails with -32601

### 2. Register `capabilities.list`

**File**: `crates/beardog-tunnel/src/unix_socket_ipc/handlers/capabilities.rs`  
**Current**: Registers `capabilities`, `get_capabilities`, `discover_capabilities`  
**Missing**: `capabilities.list`  
**Impact**: Ecosystem capability discovery cannot enumerate beardog's methods

### 3. Register bare crypto method aliases (TLS 1.3 blocker)

**File**: `crates/beardog-tunnel/src/unix_socket_ipc/handlers/crypto_handler.rs`  
**Current**: `crypto.x25519_generate_ephemeral` works; bare `x25519_generate_ephemeral` fails  
**Fix**: Add bare names as aliases in `CryptoHandler::methods()` and route to same handler  
**Impact**: Songbird's TLS 1.3 client calls bare names → TLS handshake fails → no HTTPS

Bare aliases needed:
- `x25519_generate_ephemeral` → existing `crypto.x25519_generate_ephemeral`
- `x25519_derive_secret` → existing `crypto.x25519_derive_secret`
- `chacha20_poly1305_encrypt` → existing `crypto.chacha20_poly1305_encrypt`
- `chacha20_poly1305_decrypt` → existing `crypto.chacha20_poly1305_decrypt`
- `sign_ed25519` → existing `crypto.sign`
- `hmac_sha256` → existing `crypto.hmac`
- `blake3_hash` → existing `crypto.blake3_hash`

**Alternative**: Add a pre-routing step that maps bare names to `crypto.*` before `HandlerRegistry::route()`.

## Inter-Primal Coupling

| Location | Coupling | Evolution |
|---|---|---|
| `beardog-types/src/btsp/rpc.rs:284-285` | `peer_id: "songbird-nat0"` default | Config/env, not literal |
| `beardog-tunnel/src/modes/server.rs:275` | `register_with_legacy_songbird()` | `register_with_capability("discovery")` |
| `beardog-ipc/src/songbird_client.rs` | Songbird socket paths | `discover_by_capability("discovery")` |
| `beardog-ipc/src/neural_registration.rs:235` | `/tmp/neural-api.sock` hardcoded | Standard 5-tier Neural API discovery |

## Socket Discovery Inconsistency

| Component | Dir Pattern | Tiers |
|---|---|---|
| Server (`beardog-tunnel`) | `biomeos/` | 3 |
| Client (`beardog-tower-atomic`) | `ecoPrimals/` | 4 |
| Ecosystem standard | `biomeos/` | 5 |

Align both to the 5-tier standard: `PRIMAL_SOCKET` → `BIOMEOS_SOCKET_PATH` → `XDG/biomeos/` → `/run/user/{uid}/biomeos/` → `/tmp/biomeos/`

## What's Good

- `forbid(unsafe_code)` across all crates
- Pure Rust crypto (ed25519-dalek, x25519-dalek, blake3, chacha20poly1305)
- Rich crypto method set (90+ methods)
- `crypto.*` semantic prefix already in place
- `primal.info` and `primal.capabilities` implemented
- No `todo!()` or `unimplemented!()` in production
