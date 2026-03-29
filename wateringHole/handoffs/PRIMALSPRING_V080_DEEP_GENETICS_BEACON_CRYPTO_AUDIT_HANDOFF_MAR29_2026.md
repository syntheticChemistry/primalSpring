# primalSpring v0.8.0 — Deep Genetics, Beacon, and Crypto Audit

**Date:** 2026-03-29
**From:** primalSpring (validation spring)
**To:** BearDog team, Songbird team, biomeOS team
**Status:** Critical findings — security-relevant gaps in beacon crypto pipeline

---

## Executive Summary

A deep audit of the genetics/beacon/crypto surface across BearDog, Songbird, and
primalSpring reveals **five critical gaps** and **twelve significant debt items**.
The most important finding: **two separate encryption models exist for Dark Forest
beacon operations, and neither is fully wired end-to-end.**

---

## Critical Finding 1: Two Disconnected Beacon Crypto Models

BearDog exposes **two separate crypto paths** for beacon operations:

### Path A: `birdsong.encrypt` / `birdsong.decrypt` (SecurityHandler)

- Key derivation: **SHA3-256** of `beardog:birdsong:discovery:v1:` || `family_id`
- **No secret material mixed in** — confidentiality depends entirely on secrecy of `family_id`
- If `family_id` is known or guessable (it's 16 hex chars from genesis seed),
  the encryption provides no protection
- **This is the path Songbird actually uses** via `BearDogBirdSongProvider`

### Path B: `beacon.*` (BeaconHandler)

- Key derivation: **HKDF-SHA256** from 32-byte `BeaconSeed` with context `beacon-encrypt-v1`
- Uses proper **ChaCha20-Poly1305** with random nonces
- Supports `beacon.generate`, `beacon.encrypt`, `beacon.try_decrypt`, `beacon.try_decrypt_any`
- **Songbird does NOT use this path** — `BearDogBirdSongProvider` only calls `birdsong.*`

### Impact

The Dark Forest gating at `nestgate.io` uses the Songbird BirdSong pipeline, which
delegates to BearDog's `birdsong.encrypt/decrypt`. This means the beacon encryption
key is derived from `family_id` alone — a value that is partially public (it appears
in socket names, beacon address books, and log output).

### Recommendation

Songbird's `BearDogBirdSongProvider` should be evolved to use BearDog's `beacon.*`
methods for Dark Forest beacon operations, keeping `birdsong.encrypt/decrypt` only
for non-Dark-Forest legacy discovery. This connects the proper HKDF-seeded
ChaCha20-Poly1305 path to the actual beacon pipeline.

---

## Critical Finding 2: `SONGBIRD_DARK_FOREST` Has No Effect

The environment variable `SONGBIRD_DARK_FOREST` is parsed by `env_config::dark_forest_enabled()`
but the value is **never applied** to the `BirdSongConfig` used during discovery startup.
The config is built with `..Default::default()` which has `dark_forest_enabled: false`.

**Impact:** Setting `SONGBIRD_DARK_FOREST=true` in deployment graphs and service
definitions does nothing. Dark Forest behavior in the UDP broadcaster only activates
if `BirdSongConfig.enabled && BirdSongConfig.dark_forest_enabled` — and the latter
is always `false` at startup.

**Recommendation:** Wire `SONGBIRD_DARK_FOREST` into `BirdSongConfig` construction
in `discovery_startup::initialize_birdsong_processor`. This is likely a single-line
fix but has large behavioral implications.

---

## Critical Finding 3: `genetic.derive_lineage_beacon_key` Not Registered

The method `genetic.derive_lineage_beacon_key` is **implemented** in BearDog's
`crypto_handler/genetic.rs` and routed in `router.rs`, but it is **omitted from
`crypto_method_names()`** in `method_list.rs`. The `CryptoHandler` never advertises
or receives it.

**Impact:** Any IPC call to `genetic.derive_lineage_beacon_key` returns "Method not found".
primalSpring's exp086 calls this method — if it succeeds, it's because of fallback routing
or a different handler, not the genetic crypto implementation.

**Recommendation:** Add `"genetic.derive_lineage_beacon_key"` to `crypto_method_names()`
in `method_list.rs` and update the handler test counts.

---

## Critical Finding 4: Zero-Byte Default Lineage Seed

When `genetic.derive_lineage_beacon_key` is called with an empty `lineage_seed`
parameter, the implementation uses **32 zero bytes** as HKDF input key material.
This produces a **fully predictable** derived key.

**Impact:** If any caller omits the seed (as exp086 does — it sends only `{domain: "..."}` 
without an explicit seed), the resulting key is deterministic and identical across
all deployments.

**Recommendation:** Return an error when `lineage_seed` is empty rather than falling
back to zero bytes. Validate seed length >= 32 bytes.

---

## Critical Finding 5: `birdsong.verify_lineage` Is Incomplete

On Songbird, `birdsong.verify_lineage` only **generates a challenge** (opens BearDog
socket, calls `genetic.generate_challenge`). The full challenge-response verification
exchange is not implemented.

On BearDog, `genetic.verify_lineage` uses **BLAKE3 proof** (not Ed25519 signature)
over `lineage_seed || family_ids || domain`. This is a hash-based proof, not a
signature — it proves knowledge of the seed but doesn't provide non-repudiation.

**Impact:** No end-to-end lineage verification is possible via IPC. The verify path
is incomplete on both sides.

---

## Significant Debt Items

### BearDog

| # | Item | Location | Severity |
|---|------|----------|----------|
| 1 | `federation.verify_family_member` is string equality, labeled as "genetic_lineage_hkdf" | `handlers/federation.rs` | Medium — misleading |
| 2 | `federation.derive_subfed_key` returns metadata, not key material | `handlers/federation.rs` | Medium — stub |
| 3 | `primal.capabilities` registered on both CapabilitiesHandler and IntrospectionHandler | `handlers/mod.rs` | Low — shadowed |
| 4 | `birdsong.encrypt/decrypt` uses family_id-only key (no secret) | `security.rs` via BirdSongManager | **High** — see CF1 |
| 5 | `encryption.encrypt/decrypt` labels as "HSM-backed" but is software SHA-256 KDF | `handlers/encryption.rs` | Low — misleading |

### Songbird

| # | Item | Location | Severity |
|---|------|----------|----------|
| 6 | `SONGBIRD_DARK_FOREST` not wired to BirdSongConfig | `discovery_startup.rs` | **High** — see CF2 |
| 7 | `BearDogBirdSongProvider` uses `birdsong.*` not `beacon.*` | `beardog_birdsong_provider.rs` | **High** — see CF1 |
| 8 | `try_decrypt_with_beacon_id` ignores beacon-id loop | `processor.rs` | Medium — limits multi-beacon |
| 9 | Many `JsonRpcMethod` variants return "Unknown method" in `IpcServiceHandler` | `service.rs` | Medium — dead methods |
| 10 | Dark Forest broadcaster fallback beacon_id uses SHA256(node_id) or RNG | `anonymous/broadcaster.rs` | Medium — predictable |

### primalSpring

| # | Item | Location | Severity |
|---|------|----------|----------|
| 11 | No negative test (wrong-family decrypt fails) | All experiments | **High** |
| 12 | exp086 sends empty params to verify_lineage — vacuously passes | `exp086/src/main.rs` | Medium |

---

## primalSpring Experiment Coverage Matrix

### Methods Tested vs Available

| Method | Tested | Experiment | Notes |
|--------|--------|------------|-------|
| `crypto.generate_keypair` | Yes | exp085, exp075 | Direct TCP |
| `crypto.sign_ed25519` | Yes | exp085 | Direct TCP |
| `crypto.verify_ed25519` | Yes | exp085 | With tamper test |
| `crypto.blake3_hash` | Yes | exp085, exp087 | Direct + Neural API |
| `crypto.sha256_hash` | Yes | exp085 | Direct TCP |
| `birdsong.generate_encrypted_beacon` | Yes | exp085, exp086, exp063 | Happy path only |
| `birdsong.decrypt_beacon` | Yes | exp085, exp086 | Happy path only |
| `birdsong.verify_lineage` | Partial | exp086 | Empty params, vacuous |
| `birdsong.encrypt` | **No** | — | BearDog-side payload |
| `birdsong.decrypt` | **No** | — | BearDog-side payload |
| `genetic.derive_lineage_beacon_key` | Tested | exp086 | BearDog method NOT registered |
| `genetic.derive_lineage_key` | Yes | exp086 | |
| `genetic.verify_lineage` | Partial | exp086 | Empty params |
| `secrets.store` | Yes | exp085 | |
| `secrets.retrieve` | Yes | exp085 | |
| `beacon.generate` | Yes | exp075 | BearDog alias path |
| `beacon.encrypt` | **No** | — | The proper HKDF path |
| `beacon.try_decrypt` | **No** | — | The proper HKDF path |

### Negative Tests That Don't Exist

| Scenario | Why It Matters |
|----------|----------------|
| Wrong-family decrypt | Proves Dark Forest isolation |
| Wrong signing key verify | Proves Ed25519 not vacuous |
| Invalid beacon ciphertext | Proves clean error handling |
| Empty/zero lineage seed | Should fail, currently returns predictable key |
| Cross-gate different-family beacon | Proves multi-family isolation |
| Tampered beacon (modified ciphertext) | Proves AEAD integrity |

### Dark Forest Coverage

| Assertion | Status |
|-----------|--------|
| `SONGBIRD_DARK_FOREST` changes behavior | **Not tested** (and doesn't work in code) |
| Dark Forest beacon uses beacon.* path | **Not tested** (Songbird uses birdsong.* instead) |
| HTTP 403 on non-family access | **Not tested from primalSpring** (verified manually at nestgate.io) |
| `is_family` / `success` fields on decrypt_beacon | **Not asserted** in any experiment |
| Beacon rendezvous POST/check flow | **Not tested** |

---

## Proposed Experiments

### exp089_dark_forest_negative (Critical)

Tests wrong-family decryption fails. Two-seed scenario:
1. Generate beacon with family A's BearDog
2. Attempt decrypt with family B's BearDog
3. Assert `success: false` or `is_family: false`
4. Also test tampered ciphertext, empty beacon, garbage base64

### exp090_beacon_seed_vs_birdsong (Critical)

Tests both crypto paths:
1. `beacon.generate` + `beacon.encrypt` + `beacon.try_decrypt` (proper HKDF path)
2. `birdsong.generate_encrypted_beacon` + `birdsong.decrypt_beacon` (family_id path)
3. Cross-decrypt: beacon-encrypted data should NOT decrypt via birdsong path
4. Document the security difference between the two models

### exp091_lineage_verification_e2e

Tests the full lineage verification chain:
1. `genetic.derive_lineage_beacon_key` with real seed (not empty)
2. `genetic.generate_lineage_proof` with that key
3. `genetic.verify_lineage` with proof + correct seed -> pass
4. `genetic.verify_lineage` with proof + wrong seed -> fail
5. `birdsong.verify_lineage` challenge generation (document incompleteness)

### exp092_stun_dark_forest_gating

Tests STUN/NAT with Dark Forest:
1. `stun.discover` + `stun.detect_nat_type` (baseline)
2. Document `SONGBIRD_DARK_FOREST` non-functionality
3. Test `mesh.init` + `mesh.status` + `mesh.announce`
4. Verify `mesh.auto_discover` with and without Dark Forest context

---

## Evolution Priority

1. **Register `genetic.derive_lineage_beacon_key`** in BearDog `method_list.rs` (quick fix)
2. **Wire `SONGBIRD_DARK_FOREST`** into BirdSongConfig (quick fix, large impact)
3. **Reject empty lineage seeds** in BearDog genetic handlers (quick fix, security)
4. **Evolve `BearDogBirdSongProvider`** to use `beacon.*` for Dark Forest (medium effort)
5. **Complete `birdsong.verify_lineage`** on Songbird (medium effort)
6. **Create exp089-092** in primalSpring to validate fixes (this session)

---

## Metrics

- BearDog IPC methods audited: 92+ crypto, 7 beacon, 4 secrets, 9 security, 6 federation
- Songbird IPC methods defined: 100+ (many unimplemented in dispatch)
- Critical findings: 5
- Significant debt items: 12
- Proposed experiments: 4
- Untested method constants in primalSpring: `birdsong.encrypt`, `birdsong.decrypt`, `beacon.encrypt`, `beacon.try_decrypt`
