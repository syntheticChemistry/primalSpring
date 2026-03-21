# Tower Stability Sprint — Handoff Report

**Date**: 2026-03-21  
**Author**: primalSpring team  
**Result**: 24/24 Tower Atomic gates passing — **Tower Stable**

---

## Summary

primalSpring executed all 9 remaining Tower Stability gates in a single
sprint, making changes across beardog, biomeOS, and songbird codebases.
All three primal binaries were rebuilt and harvested to `plasmidBin/`.

## Gates Resolved

| Gate | Before | After | Change |
|------|--------|-------|--------|
| 1.5 Zombie check | untested | PASS | `tower_zombie_check` test added |
| 3.1 Songbird crypto routing | FAIL | PASS | `songbird-crypto-provider` crate, Neural API mode |
| 3.5 Discovery peer list | untested | PASS | `tower_discovery_peer_list` test added |
| 4.1 TLS handshake | FAIL | PASS | `tower_tls_handshake` test added |
| 4.2 TLS internet reach | FAIL | PASS | `tower_tls_internet_reach` test added |
| 4.3 TLS routing audit | FAIL | PASS | `tower_tls_routing_audit` test added |
| 5.1 BearDog 5-tier sockets | FAIL | PASS | `discovery.rs` + `neural_registration.rs` refactored |
| 5.2 Songbird capability discovery | FAIL | PASS | All crates use `songbird-crypto-provider` |
| 5.3 biomeOS `discover_by_capability` | FAIL | PASS | `identity.rs`, `discovery/mod.rs`, `songbird.rs` refactored |
| 6.1 biomeOS Neural API caller | FAIL | PASS | `enroll.rs`, `beacon.rs` refactored |
| 6.2 biomeOS graph executor | FAIL | PASS | `node_handlers.rs`, `subfederation/beardog.rs` refactored |

## Files Changed

### beardog (`/home/eastgate/Development/ecoPrimals/phase1/beardog`)

- `crates/beardog-tower-atomic/src/discovery.rs` — 5-tier `biomeos/` namespace
- `crates/beardog-ipc/src/neural_registration.rs` — 5-tier Neural API discovery
- `crates/beardog-ipc/src/songbird_client.rs` — removed hardcoded `/tmp` fallback
- `crates/beardog-core/src/lib.rs` — doc drift fix (3-tier → 5-tier)
- `crates/beardog-core/src/socket_config.rs` — doc drift fix

### biomeOS (`/home/eastgate/Development/ecoPrimals/phase2/biomeOS`)

- `crates/biomeos/src/modes/enroll.rs` — `NeuralApiCapabilityCaller` with `DirectBeardogCaller` bootstrap fallback
- `crates/biomeos-spore/src/dark_forest/beacon.rs` — `from_neural_api()` constructor
- `crates/biomeos-graph/src/executor/node_handlers.rs` — `call_neural_api()` helper, `capability.call` routing
- `crates/biomeos-federation/src/subfederation/beardog.rs` — Neural API routing via `AtomicClient`
- `crates/biomeos-nucleus/src/identity.rs` — capability-based discovery
- `crates/biomeos-federation/src/discovery/mod.rs` — capability-based discovery
- `crates/biomeos-ui/src/device_management_server/songbird.rs` — capability-based discovery

### songbird (`/home/eastgate/Development/ecoPrimals/phase1/songbird`)

- **NEW** `crates/songbird-crypto-provider/` — shared crate with `CryptoProvider`, `RoutingMode`, socket discovery
- `crates/songbird-tor-protocol/src/crypto/mod.rs` — removed `BeardogCryptoClient`, uses `CryptoProvider`
- `crates/songbird-orchestrator/src/crypto/discovery.rs` — Neural API socket discovery
- `crates/songbird-nfc/src/genesis.rs` — `CryptoProvider` replaces `BearDogNfcCrypto`
- `crates/songbird-sovereign-onion/src/beardog_crypto.rs` — wraps `CryptoProvider`
- `crates/songbird-quic/src/config.rs` — `neural_api_socket` replaces `beardog_socket`

### primalSpring

- `ecoPrimal/src/harness/mod.rs` — `RunningAtomic::pids()` method
- `ecoPrimal/tests/server_integration.rs` — 5 new integration tests
- `specs/TOWER_STABILITY.md` — updated to 24/24
- `CHANGELOG.md` — v0.4.0 entry

## Architecture Pattern

The sprint established a consistent pattern across all three primals:

1. **Socket discovery**: 5-tier `biomeos/` namespace (env → orchestrator → XDG → `/run/user` → `/tmp`)
2. **Capability routing**: All inter-primal calls use `capability.call` through the Neural API
3. **Bootstrap fallback**: Direct connections retained only for pre-Neural API scenarios (e.g., initial enrollment)
4. **Shared providers**: `songbird-crypto-provider` centralizes crypto routing for all songbird crates

## Next Steps

Tower Atomic is stable. The progression path is:

1. **Nest Atomic** — add nestgate (storage), define storage gates
2. **Node Atomic** — add toadstool (compute), define compute gates
3. **Full NUCLEUS** — add squirrel (AI coordination), define AI gates

Primal teams can now return to evolution debt and polish. primalSpring
will define Nest Atomic gates and begin the next stability cycle.

## Binary Versions Harvested

| Binary | Version | Size |
|--------|---------|------|
| beardog | v0.9.0 | 6.7 MB |
| songbird | v0.2.1 | 21 MB |
| neural-api-server (biomeOS) | v0.1.0 | 16 MB |

## Test Summary

- 239 unit tests — PASS
- 10 non-ignored integration tests — PASS
- 11 ignored integration tests (6 existing + 5 new) — **ALL 11 PASS with plasmidBin**
- 2 doc-tests — PASS
- **Total: 262 passed (251 auto + 11 live), 0 failed**

### Capability Registration Fix (Post-Sprint)

The initial 11 live tests showed 7/11 passing. The Neural API was not
registering beardog/songbird capabilities because:

1. `capability_registry.toml` loaded translations but didn't bridge
   `[domains]` to the NeuralRouter — fixed in `server_lifecycle.rs`
2. Neural API socket was placed outside the `biomeos/` namespace —
   fixed in `launcher/mod.rs`
3. primalSpring's graphs dir lacks biomeOS's `[[nodes]]` format —
   `discover_biomeos_graphs()` now finds the biomeOS source tree

After these fixes: **11/11 live tests pass.**
