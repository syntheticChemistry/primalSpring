# Capability Routing Trace — Hardcoded → Semantic Evolution

**Status**: Active trace — primalSpring v0.3.5+  
**Date**: 2026-03-18  
**Context**: Tower Atomic live validation revealed method-naming mismatch between songbird and beardog, traced to hardcoded inter-primal calls that bypass capability-based routing.

## Architecture Principle

Primals have **self-knowledge only**. Inter-primal communication MUST go through
the Neural API's semantic capability translation layer:

```
Primal A  →  capability.call("crypto", "generate_keypair", {})
          →  Neural API translates → beardog's "crypto.x25519_generate_ephemeral"
          →  BearDog responds
          →  Neural API returns result to Primal A
```

**Why**: Primals evolve independently. If songbird hardcodes `x25519_generate_ephemeral`,
then beardog can never rename, refactor, or swap its X25519 implementation without
breaking songbird. The Neural API translation registry is the contract — primals
speak their own internal language, and the Neural API translates between them.

**Analogy**: A mechanic and a structural engineer don't need the same vocabulary.
They need a translator who understands both. The Neural API is that translator.

## Triggering Evidence

Live Tower test (primalSpring v0.3.5) revealed:

```
songbird TLS client → beardog.x25519_generate_ephemeral  → -32601 Method not found
songbird TLS client → beardog.crypto.x25519_generate_ephemeral → ✅ SUCCESS
```

Songbird calls the bare method name directly on beardog's socket.
BearDog v0.9.0 only exposes `crypto.`-prefixed methods.
The 93% TLS 1.3 success rate (87 sites) from biomeOS Phase 1 was from a
build where this naming happened to align.

## Traced Hardcoding Sites

### Category 1: Songbird → BearDog Direct Crypto Calls

Songbird's TLS client, Tor protocol, and orchestrator all contain
`BeardogCryptoClient` that connects directly to beardog's socket and
calls raw method names:

| Songbird Crate | Raw Method | Should Be |
|---|---|---|
| `songbird-tls` | `x25519_generate_ephemeral` | `capability.call("crypto", "generate_keypair")` |
| `songbird-tls` | `hmac_sha256` | `capability.call("crypto", "hmac")` |
| `songbird-tor-protocol` | `x25519_generate_ephemeral` | `capability.call("crypto", "generate_keypair")` |
| `songbird-tor-protocol` | `chacha20_poly1305_encrypt` | `capability.call("crypto", "encrypt")` |
| `songbird-tor-protocol` | `chacha20_poly1305_decrypt` | `capability.call("crypto", "decrypt")` |
| `songbird-tor-protocol` | `sign_ed25519` | `capability.call("crypto", "sign")` |
| `songbird-sovereign-onion` | `x25519_generate_ephemeral` | `capability.call("crypto", "generate_keypair")` |
| `songbird-sovereign-onion` | `chacha20_poly1305_encrypt` | `capability.call("crypto", "encrypt")` |
| `songbird-orchestrator` | `sign_ed25519` | `capability.call("crypto", "sign")` |
| `songbird-orchestrator` | `x25519_generate_ephemeral` | `capability.call("crypto", "generate_keypair")` |

### Category 2: biomeOS → BearDog Direct Calls

biomeOS orchestration code bypasses the Neural API it provides:

| biomeOS Crate | Pattern | Should Be |
|---|---|---|
| `biomeos-spore` | `DirectBeardogCaller` struct | `NeuralApiCapabilityCaller` |
| `biomeos-spore/dark_forest` | `crypto.chacha20_poly1305_encrypt` | `capability.call("crypto", "encrypt")` |
| `biomeos-spore/dark_forest` | `crypto.blake3_hash` | `capability.call("crypto", "hash")` |
| `biomeos-spore/dark_forest` | `genetic.derive_lineage_key` | `capability.call("genetic", "derive_key")` |
| `biomeos-spore/dark_forest` | `genetic.verify_lineage` | `capability.call("genetic", "verify")` |
| `biomeos-api` | `crypto.blake3_hash` direct call | `capability.call("crypto", "hash")` |
| `biomeos-federation` | `security.verify_primal_identity` | `capability.call("security", "verify_identity")` |
| `biomeos-nucleus` | `identity.get_proof` | `capability.call("identity", "get_proof")` |
| `biomeos-graph` | `crypto.derive_child_seed` | `capability.call("crypto", "derive_seed")` |

### Category 3: Hardcoded Socket Discovery Functions

Functions that resolve primal sockets by name instead of capability:

| Location | Function | Should Be |
|---|---|---|
| `biomeos-federation/subfederation/beardog.rs` | `discover_beardog_socket()` | `discover_by_capability("security")` |
| `biomeos-federation/discovery/mod.rs` | `discover_songbird_socket()` | `discover_by_capability("discovery")` |
| `biomeos-nucleus/identity.rs` | `discover_beardog_socket()` | `discover_by_capability("security")` |
| `biomeos/modes/enroll.rs` | `discover_beardog_socket_in()` | `discover_by_capability("crypto")` |
| `biomeos-graph/executor/node_handlers.rs` | `discover_beardog_socket(&env)` | `discover_capability_socket("security")` |
| `songbird-orchestrator/crypto/discovery.rs` | `get_beardog_crypto_socket()` | `discover_by_capability("crypto")` |

### Category 4: Hardcoded Socket Paths

| Location | Path | Should Be |
|---|---|---|
| `biomeos-spore/beacon_genetics/capability.rs` | `/tmp/beardog.sock`, `{XDG}/biomeos/beardog.sock` | Nucleation or capability discovery |
| `biomeos-api/beacon_verification.rs` | `beardog-{family_id}.sock` | `discover_by_capability("crypto")` |
| `biomeos-nucleus/identity.rs` | `/run/user/{uid}/biomeos/beardog.sock` | Capability discovery |
| `biomeos-api/beacon_verification.rs` | `["beardog", "songbird"]` hardcoded roster | Capability-based provider list |

### Category 5: Hardcoded Primal-to-Capability Mapping

| Location | Pattern | Should Be |
|---|---|---|
| `biomeos/modes/nucleus.rs` | `"security" => BEARDOG, "discovery" => SONGBIRD` | `CapabilityTaxonomy::resolve(capability)` |
| `biomeos-atomic-deploy/http_client.rs` | `"songbird".to_string()` as default | Capability discovery |
| `biomeos-graph/executor/node_handlers.rs` | `SECURITY_PROVIDER = BEARDOG` | Capability discovery |

### Category 6: Missing Standard Methods

Primals that don't register ecosystem-standard method names:

| Primal | `health.liveness` | `capabilities.list` | Notes |
|---|---|---|---|
| BearDog | Missing (has `ping`, `health`, `status`, `check`) | Missing (has `capabilities`, `get_capabilities`) | Quick fix: add aliases |
| Songbird | Missing (has `health`) | Missing (has `primal.capabilities`) | Quick fix: add aliases |
| NestGate | Missing (has `health.check`) | Missing | Quick fix: add aliases |
| ToadStool | Missing | Missing | Needs implementation |
| Squirrel | **Implemented** | **Implemented** | Gold standard |

### Category 7: Socket Discovery Inconsistencies

| Primal | Server Tiers | Client Tiers | Dir Pattern | Standard (5-tier) |
|---|---|---|---|---|
| BearDog | 3 | 4 | `biomeos/` vs `ecoPrimals/` | No |
| Songbird | 7 (mixed) | N/A | `biomeos/` + identity names | No |
| NestGate | 4 | N/A | `biomeos/` | No |
| ToadStool | 5 | N/A | `biomeos/` | **Yes** |
| Squirrel | 5 | N/A | `biomeos/` | **Yes** |

## Counts

| Category | Sites |
|---|---|
| Direct raw method calls (songbird→beardog) | ~15 |
| Direct raw method calls (biomeOS→beardog) | ~12 |
| Hardcoded socket discovery functions | ~8 |
| Hardcoded socket paths | ~6 |
| Identity-based capability mapping | ~4 |
| Missing standard methods | ~8 (across 4 primals) |
| Socket discovery non-standard | 3 primals |
| **Total** | **~53+ sites** |

## Impact

When these sites use hardcoded patterns:
- **Primal renaming breaks the ecosystem** (can't rename beardog → rhizocrypt)
- **Method evolution breaks callers** (beardog adding `crypto.` prefix broke songbird TLS)
- **Primal swapping impossible** (can't replace beardog with another crypto provider)
- **Testing requires real primals** (can't mock by capability)

## Evolution Path

### Phase 1: Neural API Routing (primalSpring validates)

1. Replace all `DirectBeardogCaller` with `NeuralApiCapabilityCaller`
2. Replace `discover_{primal}_socket()` with `discover_by_capability()`
3. Replace raw method calls with `capability.call(domain, operation, args)`
4. primalSpring adds validation: detect direct inter-primal socket connections

### Phase 2: Backward Compatibility Bridge

During migration, beardog should register BOTH forms:
- `x25519_generate_ephemeral` (raw, for backward compat)
- `crypto.x25519_generate_ephemeral` (semantic, preferred)

This allows old songbird binaries to keep working while new ones migrate.

### Phase 3: Strict Capability-Only

Once all primals route through Neural API:
- Remove raw method aliases
- primalSpring enforces: no direct inter-primal socket connections
- Composition validation confirms all calls go through capability routing

## primalSpring Validation Role

primalSpring can validate this evolution by:

1. **Live Tower + Neural API test** — verify calls route through capability translation
2. **Method availability audit** — probe each primal for both raw and semantic method names
3. **Socket connection trace** — detect when a primal opens a connection to another primal's socket directly (bypass detection)
4. **Capability translation coverage** — verify all registered raw methods have semantic mappings
