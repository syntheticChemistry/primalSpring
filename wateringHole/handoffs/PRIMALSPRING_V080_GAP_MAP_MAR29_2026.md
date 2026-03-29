# primalSpring v0.8.0 — Comprehensive Gap Map

**Date:** 2026-03-29
**From:** primalSpring (validation spring)
**Purpose:** Classify all identified debt as structural, deployment, documentation, or test — after deep investigation of each finding.

---

## Methodology

Each gap from the earlier audit was individually traced to source. BearDog's own
test suite, Songbird's discovery library, and primalSpring's experiment code were
all inspected. Findings that appeared "critical" in the initial scan often turned
out to be deployment wiring, intentional design, or already covered by the primal's
own test suites.

---

## Category 1: Deployment Wiring

These are configuration gaps where the feature is fully implemented but not
connected at deployment time. No structural code changes needed — just wiring.

### DW-1: SONGBIRD_DARK_FOREST not wired to BirdSongConfig

**Location:** `songbird/crates/songbird-orchestrator/src/app/discovery_startup.rs`

- `env_config::dark_forest_enabled()` parses the env var correctly
- `BirdSongConfig` has the `dark_forest_enabled` field
- Broadcaster/listener branch on `is_dark_forest_active()` correctly
- Dark Forest encrypt/decrypt logic is implemented and tested in `songbird-discovery`
- **The gap:** Three `BirdSongConfig { .. }` struct literals in `initialize_birdsong_processor`
  use `..Default::default()` which sets `dark_forest_enabled: false`
- **Fix:** Add `dark_forest_enabled: crate::env_config::dark_forest_enabled()` to each literal
- **Same gap affects:** `SONGBIRD_ACCEPT_LEGACY_BIRDSONG`, `SONGBIRD_DUAL_BROADCAST`

### DW-2: genetic.derive_lineage_beacon_key not registered

**Location:** `beardog/crates/beardog-tunnel/src/unix_socket_ipc/handlers/crypto_handler/method_list.rs`

- Implementation exists in `crypto_handlers_genetic/lineage.rs`
- Router match arm exists in `crypto_handler/genetic.rs`
- Tests exist and pass (`cargo test -p beardog-tunnel lineage`)
- **The gap:** The string `"genetic.derive_lineage_beacon_key"` is missing from
  `crypto_method_names()`. BearDog's `HandlerRegistry` routes by exact match against
  the handler's `methods()` list — NOT by prefix. So the registry returns
  "Method not found" before the router ever sees the request.
- **Fix:** Add one string to the array. Update method count assertions.
- **Impact on primalSpring:** exp086 calls this method but gets "Method not found",
  hits the `Err` branch, and reports a skip — so the test doesn't falsely pass,
  it just doesn't run.

---

## Category 2: Intentional Design (Not Bugs)

These were flagged as "disconnected" or "incomplete" but are actually deliberate
architectural choices after tracing the full code paths.

### ID-1: beacon.* vs birdsong.encrypt/decrypt are separate systems

The initial audit called this "two disconnected beacon crypto models." After deep
tracing, these serve genuinely different trust models:

| Aspect | `beacon.*` | `birdsong.encrypt/decrypt` |
|--------|-----------|---------------------------|
| **Purpose** | Dark Forest — meeting-exchanged secrets | Family discovery — anyone who knows family_id |
| **Key material** | 32-byte random seed + HKDF `beacon-encrypt-v1` | SHA3-256(`beardog:birdsong:discovery:v1:` ‖ family_id) |
| **Trust model** | Only peers with exchanged seed can decrypt | Any family member can derive key from family_id |
| **When to use** | Cross-internet, hostile network | LAN discovery, low-threat |

This is not a disconnect — it's two tools for two jobs. The naming overlap
(`BeaconSeed` lives under `beardog_genetics::birdsong`) is confusing but not wrong.

**What WOULD be structural debt:** If Songbird's Dark Forest broadcaster used
`birdsong.encrypt` instead of `beacon.*` for actual Dark Forest operations. But
Dark Forest mode (when wired, see DW-1) uses `encrypt_dark_forest_beacon` which
goes through a `BirdSongProcessor` encryption provider — a different code path
from both `birdsong.encrypt` and `beacon.encrypt`.

### ID-2: birdsong.verify_lineage is challenge step 1 only

The code explicitly comments: "For now, return the challenge for caller to handle
exchange." This is step 1 of a challenge-response protocol:

1. **Implemented:** `birdsong.verify_lineage` → calls `genetic.generate_challenge` → returns challenge
2. **Not implemented:** Send challenge over wire → receive response → call `genetic.verify_challenge_response`
3. **By design:** Step 2 requires a wire protocol between two Songbird nodes — that protocol doesn't exist yet

On BearDog's side, `genetic.verify_lineage` IS complete for its model: BLAKE3
proof over `lineage_seed ‖ our_family_id ‖ peer_family_id ‖ domain`. This is a
shared-secret proof (proves seed knowledge), not an asymmetric signature
(non-repudiation). That's appropriate for the family-trust model.

The Ed25519/certificate path exists separately as `genetic.sign_lineage_certificate`
and `genetic.verify_lineage_certificate` — for when non-repudiation matters.

### ID-3: BLAKE3 proof vs Ed25519 for lineage

Two models coexist by design:
- **BLAKE3 shared-secret:** Fast, symmetric, proves family membership. Used by `genetic.verify_lineage`.
- **Ed25519 certificate:** Asymmetric, non-repudiating, proves individual identity. Used by `genetic.sign_lineage_certificate`.

---

## Category 3: Defensive Coding Gaps

Real issues but not architectural — they're edge case handling.

### DC-1: Zero-byte default lineage seed

**Location:** `beardog/crates/beardog-tunnel/src/unix_socket_ipc/crypto_handlers_genetic/lineage.rs`

When `genetic.derive_lineage_beacon_key` is called without a `lineage_seed` param,
it falls back to 32 zero bytes. This produces a fully predictable key identical
across all deployments.

**Assessment:** Should return an error rather than defaulting. But the method isn't
even reachable via IPC yet (see DW-2), so this is second-order debt.

### DC-2: family_id as sole key material for discovery encryption

**Location:** `beardog/crates/beardog-genetics/src/birdsong/manager.rs`

The `birdsong.encrypt/decrypt` path derives its key from `family_id` only (with a
domain prefix). If `family_id` is known, the encryption provides no confidentiality.

**Assessment:** This is acceptable for LAN discovery (see ID-1 — it's designed for
this). It would be a problem if used for Dark Forest operations, but Dark Forest
uses a different path when wired (DW-1). The confusing part is that the method
name `birdsong.encrypt` doesn't signal its limited security posture.

---

## Category 4: Documentation / Naming Debt

These cause confusion when reading the code but don't affect runtime behavior.

### DN-1: BeaconSeed module path

`BeaconSeed` lives at `beardog_genetics::birdsong::beacon_seed` but has nothing to
do with the `birdsong.encrypt/decrypt` RPC methods. The `birdsong` module is
overloaded to mean "anything BirdSong-related" rather than "the birdsong.* IPC
namespace."

### DN-2: federation.verify_family_member misleading label

**Location:** `beardog/crates/beardog-tunnel/src/unix_socket_ipc/handlers/federation.rs`

Returns `algorithm: "genetic_lineage_hkdf"` but the verification is string equality
on family_id. Not wrong (it's a fast check), but the label implies HKDF-based proof.

### DN-3: encryption.encrypt/decrypt "HSM-backed" label

**Location:** `beardog/crates/beardog-tunnel/src/unix_socket_ipc/handlers/encryption.rs`

Labels the output as HSM-backed but uses software SHA-256 KDF. The HSM abstraction
exists (`SoftwareHsm`) but the label overstates.

### DN-4: Dead JsonRpcMethod variants in Songbird

**Location:** `songbird/crates/songbird-universal-ipc/src/handlers/service.rs`

Many `JsonRpcMethod` enum variants in `IpcServiceHandler::handle` return "Unknown
method" at runtime. These are declared as enum members with string mappings but no
dispatch implementation — either future stubs or removed features.

---

## Category 5: primalSpring Test Gaps

These are gaps in primalSpring's validation coverage. The critical distinction:
many of these are **already covered in the primal's own test suite** — the gap is
only from primalSpring's perspective as an external validator.

### Covered in BearDog's Own Tests (Not Real Gaps)

| Scenario | BearDog Test |
|----------|-------------|
| Wrong-family discovery decrypt | `test_discovery_wrong_family_fails` in `coverage_birdsong_manager_tests.rs` |
| Different beacon can't decrypt | `test_different_beacon_cannot_decrypt` in `beacon_seed.rs` |
| Wrong lineage fails | `test_wrong_lineage_fails` in `encryption.rs` |
| Wrong AES-GCM key fails | `crypto_operations_comprehensive_tests.rs` |
| Wrong ChaCha20 tag fails | `chacha_decrypt_wrong_tag_fails_authentication` in `crypto_fault_injection_tests.rs` |
| Truncated ciphertext fails | `chacha_decrypt_truncated_ciphertext_fails` |
| beacon.encrypt round-trip | `test_beacon_encrypt_decrypt_roundtrip` in `beacon_seed.rs` |
| birdsong.encrypt/decrypt IPC | `port_free_architecture_e2e_tests.rs` |
| Discovery encrypt/decrypt | `coverage_birdsong_manager_tests.rs` |

### Real primalSpring Gaps

| Gap | Why It Matters | Difficulty |
|-----|---------------|-----------|
| **exp086 sends empty `{}` to verify_lineage** | Test is vacuous — generates a skip or weak pass, never actually verifies lineage | Easy fix: send real `lineage_seed`, `our_family_id`, `peer_family_id`, `lineage_proof` |
| **No negative test FROM primalSpring** | primalSpring validates happy paths but never proves rejection | Requires multi-instance design or mock — see "By Design" note |
| **No graph encodes negative flows** | Graphs are all happy-path or narrative | Easy to add graph nodes for expected-failure scenarios |
| **exp086 beacon key test is a skip** | Due to DW-2, the method call fails and exp086 skips | Resolves automatically when DW-2 is fixed |

### By Design: Multi-Family Testing

Wrong-family decrypt requires two separate BearDog instances with different seeds.
primalSpring experiments currently connect to a single tower. Options:

1. **Two-tower experiment:** Spin up two BearDog instances on different ports (medium effort)
2. **Accept primal coverage:** BearDog's own `test_discovery_wrong_family_fails` and
   `test_different_beacon_cannot_decrypt` cover this at the unit/integration level
3. **Composition test via graph:** A future multi-node graph could encode this

---

## Summary Matrix

| Finding | Previous Classification | Actual Classification | Fix Complexity |
|---------|------------------------|----------------------|---------------|
| Two beacon crypto models | Critical (disconnected) | **Intentional design** (ID-1) | None needed |
| SONGBIRD_DARK_FOREST no effect | Critical | **Deployment wiring** (DW-1) | 3 struct fields |
| derive_lineage_beacon_key unregistered | Critical | **Deployment wiring** (DW-2) | 1 string + count |
| Zero-byte default seed | Critical | **Defensive coding** (DC-1) | Validation check |
| verify_lineage incomplete | Critical | **Intentional design** (ID-2) | Wire protocol (future) |
| No wrong-family test | High | **Covered in BearDog** | None for primalSpring |
| exp086 empty verify_lineage | Medium | **Real test gap** (TS) | Params fix |
| birdsong.encrypt untested | High (primalSpring) | **Covered in BearDog** | Optional for primalSpring |
| beacon.encrypt untested | High (primalSpring) | **Covered in BearDog** | Optional for primalSpring |
| Dead Songbird methods | Medium | **Documentation** (DN-4) | Cleanup |
| Federation misleading label | Medium | **Documentation** (DN-2) | Label fix |

---

## Actionable Items (Ordered by Impact/Effort)

### Quick Wins (each < 30 minutes)

1. **DW-2:** Add `"genetic.derive_lineage_beacon_key"` to `method_list.rs` in BearDog
2. **DW-1:** Wire 3 env vars into `discovery_startup.rs` struct literals in Songbird
3. **DC-1:** Add `lineage_seed` validation in `handle_derive_lineage_beacon_key`
4. **TS:** Fix exp086 to send real params to `genetic.verify_lineage`

### Medium Effort (hours)

5. **DN-2/DN-3:** Fix misleading labels in federation and encryption handlers
6. **DN-4:** Audit Songbird `JsonRpcMethod` variants, remove or implement dead ones
7. Add negative-flow graph nodes to `primalSpring/graphs/`

### Future Work (when wire protocol exists)

8. **ID-2:** Complete `birdsong.verify_lineage` with peer exchange protocol
9. Multi-tower primalSpring experiment for cross-family validation
10. Ancestor beacon implementation (see beacon architecture evolution handoff)
