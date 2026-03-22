# Tower Stability Specification

**Status**: **STABLE** — primalSpring v0.4.0  
**Date**: 2026-03-21  
**Strategy**: Tower first, then Nest, then NUCLEUS

## Co-Evolution Strategy

primalSpring co-evolves with **three teams** until the Tower Atomic is stable:

```
┌─────────────────────────────────────────────────────┐
│                  Tower Atomic  ✅ STABLE              │
│                                                      │
│   beardog (security)  ←──Neural API──→  songbird    │
│          ↑                                   ↑       │
│          └───── biomeOS (orchestration) ─────┘       │
│                                                      │
│   primalSpring validates the composition             │
└─────────────────────────────────────────────────────┘
```

Tower is stable (24/24 gates) → next: add **nestgate** for Nest Atomic.  
Once Nest is stable → add **toadstool + squirrel** for Full NUCLEUS.

**Tower + Squirrel composition validated** (2026-03-21): Squirrel AI primal added
alongside Tower (beardog + songbird) with live Anthropic Claude API queries.
Two new integration tests (`tower_squirrel_ai_query`, `tower_squirrel_composition_health`)
confirm Tower remains stable with Squirrel added.

## What "Stable Tower" Means

All acceptance criteria must pass in primalSpring's live integration tests
with binaries from `ecoPrimals/plasmidBin/`.

### Gate 1: Process Lifecycle

| # | Criterion | primalSpring Test |
|---|---|---|
| 1.1 | beardog starts, binds socket, responds within 5s | `tower_atomic_live_health_check` |
| 1.2 | songbird starts, binds socket, responds within 5s | `tower_atomic_live_health_check` |
| 1.3 | Neural API starts, detects Tower, enters COORDINATED MODE | `tower_neural_api_health` |
| 1.4 | Clean RAII shutdown: Neural API → songbird → beardog | `RunningAtomic::drop()` |
| 1.5 | No zombie processes after test completion | harness `waitpid` check |

### Gate 2: Standard Methods

| # | Criterion | Method | primalSpring Test |
|---|---|---|---|
| 2.1 | beardog responds to `health.liveness` | `health.liveness` | `tower_atomic_live_health_check` |
| 2.2 | songbird responds to `health.liveness` | `health.liveness` | `tower_atomic_live_health_check` |
| 2.3 | beardog responds to `capabilities.list` | `capabilities.list` | `tower_atomic_live_capabilities` |
| 2.4 | songbird responds to `capabilities.list` | `capabilities.list` | `tower_atomic_live_capabilities` |
| 2.5 | Neural API responds to health probe | `NeuralBridge::health()` | `tower_neural_api_health` |

### Gate 3: Capability Routing

| # | Criterion | primalSpring Test |
|---|---|---|
| 3.1 | songbird crypto calls route through Neural API (not direct beardog socket) | `tower_neural_api_capability_discovery` |
| 3.2 | Neural API registers beardog's crypto capabilities | capability translation coverage |
| 3.3 | Neural API registers songbird's discovery capabilities | capability translation coverage |
| 3.4 | `capability.call("crypto", "generate_keypair")` returns valid X25519 pair | `tower_neural_api_full_validation` |
| 3.5 | `capability.call("discovery", "peers")` returns peer list | `tower_neural_api_full_validation` |

### Gate 4: TLS 1.3 End-to-End

| # | Criterion | primalSpring Test |
|---|---|---|
| 4.1 | songbird TLS client completes X25519 key exchange via Neural API | new: `tower_tls_handshake` |
| 4.2 | songbird can reach an external HTTPS endpoint | new: `tower_tls_internet_reach` |
| 4.3 | TLS 1.3 handshake uses `capability.call` path (not direct beardog) | new: `tower_tls_routing_audit` |

### Gate 5: Socket Discovery

| # | Criterion | primalSpring Test |
|---|---|---|
| 5.1 | beardog uses 5-tier socket resolution (server + client aligned) | static audit |
| 5.2 | songbird uses capability-based crypto socket discovery (no identity tiers) | static audit |
| 5.3 | biomeOS uses `discover_by_capability()` for beardog (not `discover_beardog_socket()`) | static audit |

### Gate 6: Neural API Dogfooding

| # | Criterion | primalSpring Test |
|---|---|---|
| 6.1 | biomeOS enrollment uses `NeuralApiCapabilityCaller` (not `DirectBeardogCaller`) | static audit |
| 6.2 | biomeOS graph executor uses `capability.call` (not direct `UnixStream`) | static audit |
| 6.3 | `genetic.*` and `lineage.*` methods registered in capability translation registry | registry coverage test |

### Gate 7: Songbird Subsystem Health

| # | Criterion | primalSpring Test |
|---|---|---|
| 7.1 | discovery.announce + discovery.find_primals respond | `tower_discovery_announce_find` |
| 7.2 | stun.get_public_address resolves (network-dependent) | `tower_stun_public_address` |
| 7.3 | birdsong beacon encrypt + decrypt round-trip | `tower_birdsong_beacon` |
| 7.4 | onion.start + onion.status sovereign onion lifecycle | `tower_onion_service` |
| 7.5 | tor.status Tor subsystem responds | `tower_tor_status` |
| 7.6 | federation.peers cross-tower federation query | `tower_federation_status` |

### Gate 8: Beacon Round-Trip

| # | Criterion | primalSpring Test |
|---|---|---|
| 8.1 | BirdSong beacon generated with family_id + capabilities | `tower_birdsong_beacon`, `exp063` |
| 8.2 | BirdSong beacon decrypts to valid payload | `tower_birdsong_beacon`, `exp063` |

### Gate 9: Rendezvous (Pixel ↔ Tower)

| # | Criterion | primalSpring Test |
|---|---|---|
| 9.1 | Local Tower generates beacon for rendezvous exchange | `exp063_pixel_tower_rendezvous` |
| 9.2 | STUN public address obtained for hotspot meeting | `exp063_pixel_tower_rendezvous` |
| 9.3 | Onion service available as Tor rendezvous fallback | `exp063_pixel_tower_rendezvous` |

### Gate 10: Internet Reach

| # | Criterion | primalSpring Test |
|---|---|---|
| 10.1 | HTTPS probe to api.nestgate.io succeeds | `exp064_nestgate_internet_reach` |
| 10.2 | STUN resolves public IP | `exp064_nestgate_internet_reach` |
| 10.3 | At least one internet path available | `exp064_nestgate_internet_reach` |

### Gate 11: Visualization (petalTongue)

| # | Criterion | primalSpring Test |
|---|---|---|
| 11.1 | petalTongue spawns in headless server mode | `exp065_petaltongue_tower_dashboard` |
| 11.2 | visualization.render.dashboard returns Tower health | `exp065_petaltongue_tower_dashboard` |
| 11.3 | visualization.render.grammar produces SVG/JSON output | `exp065_petaltongue_tower_dashboard` |

## Current Status vs Gates (2026-03-21)

| Gate | Status | Notes |
|---|---|---|
| 1. Process Lifecycle | **PASS** (5/5) | All lifecycle gates pass. 1.5 zombie check automated via `tower_zombie_check` |
| 2. Standard Methods | **PASS** (5/5) | `health.liveness`, `capabilities.list` confirmed live via integration tests |
| 3. Capability Routing | **PASS** (5/5) | All crypto routes through Neural API. `tower_discovery_peer_list` validates 3.5 |
| 4. TLS 1.3 E2E | **PASS** (3/3) | `tower_tls_handshake`, `tower_tls_internet_reach`, `tower_tls_routing_audit` |
| 5. Socket Discovery | **PASS** (3/3) | beardog 5-tier aligned, songbird uses `songbird-crypto-provider` (Neural API), biomeOS uses `discover_by_capability()` |
| 6. Neural API Dogfooding | **PASS** (3/3) | biomeOS enrollment uses `NeuralApiCapabilityCaller`, graph executor uses `capability.call`, registry complete |
| 7. Subsystem Health | **PENDING** (0/6) | Tests added, require live plasmidBin validation sprint |
| 8. Beacon Round-Trip | **PENDING** (0/2) | Depends on Gate 7 BirdSong validation |
| 9. Rendezvous | **PENDING** (0/3) | Depends on Gates 7+8, requires Pixel 8a for full validation |
| 10. Internet Reach | **PENDING** (0/3) | Depends on network access + nestgate.io availability |
| 11. Visualization | **PENDING** (0/3) | petalTongue wired, requires live validation sprint |

**Gates 1-6: 24/24 PASS — Tower Core Stable**  
**Gates 7-11: 0/17 PENDING — Tower Full Utilization (tests written, awaiting live sprint)**  
**Overall: 24/41 gates passing — Tower Full Utilization in progress**

**Live test results (11/11 green, validated 2026-03-21):**
- `tower_atomic_live_health_check` — PASS
- `tower_atomic_live_capabilities` — PASS
- `tower_atomic_live_validation_result` — PASS
- `tower_neural_api_health` — PASS
- `tower_neural_api_capability_discovery` — PASS
- `tower_neural_api_full_validation` — PASS
- `tower_zombie_check` — PASS (Gate 1.5)
- `tower_discovery_peer_list` — PASS (Gate 3.5)
- `tower_tls_handshake` — PASS (Gate 4.1)
- `tower_tls_internet_reach` — PASS (Gate 4.2)
- `tower_tls_routing_audit` — PASS (Gate 4.3)

**New tests (awaiting live sprint):**
- `tower_discovery_announce_find` — Gate 7.1
- `tower_stun_public_address` — Gate 7.2
- `tower_birdsong_beacon` — Gate 7.3
- `tower_onion_service` — Gate 7.4
- `tower_tor_status` — Gate 7.5
- `tower_federation_status` — Gate 7.6

## Tower Stability Sprint (2026-03-21) — Completed

All 9 remaining gates resolved in a single sprint by primalSpring team,
executing changes across beardog, songbird, and biomeOS codebases.

### BearDog (Gate 5.1)

1. ~~**`health.rs`**: Add `"health.liveness"`, `"health.readiness"` to `HealthHandler::methods()`~~ DONE
2. ~~**`capabilities.rs`**: Add `"capabilities.list"` to `CapabilitiesHandler::methods()`~~ DONE
3. ~~**`mod.rs`**: Pre-routing mapper for bare crypto aliases~~ DONE
4. ~~**`discovery.rs`**: 5-tier `biomeos/` namespace alignment~~ DONE (sprint)
5. ~~**`neural_registration.rs`**: 5-tier Neural API socket discovery~~ DONE (sprint)
6. ~~**`songbird_client.rs`**: Remove hardcoded `/tmp/beardog-default.sock`~~ DONE (sprint)

### Songbird (Gates 3.1, 5.2)

1. ~~**`service.rs`**: Add `"health.liveness"`, `"capabilities.list"` as aliases~~ DONE
2. ~~**TLS + Tor + Orchestrator + NFC + Sovereign-Onion + QUIC**: Route all crypto through `songbird-crypto-provider` with Neural API mode~~ DONE (sprint)
3. ~~**`songbird-crypto-provider`**: New shared crate extracted from `songbird-http-client`~~ DONE (sprint)

### biomeOS (Gates 5.3, 6.1, 6.2)

1. ~~**`capability_registry.toml`**: Add `genetic.*` and `lineage.*` domain translations~~ DONE
2. ~~**`enroll.rs`**: Replace `DirectBeardogCaller` with `NeuralApiCapabilityCaller`~~ DONE (sprint)
3. ~~**`node_handlers.rs`**: Graph executor uses `capability.call` through Neural API~~ DONE (sprint)
4. ~~**`subfederation/beardog.rs`**: Uses `capability.call` through Neural API~~ DONE (sprint)
5. ~~**`identity.rs`**, **`discovery/mod.rs`**, **`songbird.rs`**: `discover_by_capability()` replacing identity-based discovery~~ DONE (sprint)

## Sprint Cadence

Each sprint, primalSpring:

1. **Harvests** — pull latest binaries from each team into `plasmidBin/`
2. **Tests** — run full Tower integration suite (`cargo test --test server_integration -- tower`)
3. **Reports** — gate status table updated, regressions flagged
4. **Handoffs** — delta report to `wateringHole/handoffs/` for each team

## Progression Path

```
Tower Atomic (beardog + songbird + biomeOS)     ✅ CORE STABLE (24/24)
    │
    ├── Tower Full Utilization                  ← IN PROGRESS (24/41)
    │   Gates 7-11: subsystems, beacon, rendezvous, internet, visualization
    │   + Squirrel AI composition
    │   + petalTongue visualization
    │
    ▼  Tower gates inherited
Nest Atomic (Tower + nestgate)                  ← NEXT
    Tower gates + storage gates
    │
    ▼  all storage gates green
Node Atomic (Nest + toadstool)
    Nest gates + compute gates
    │
    ▼  all compute gates green
Full NUCLEUS (Node + squirrel)
    All gates + AI coordination gates
```

Each tier inherits all gates from the previous tier.  
A primal joins the composition only when the previous tier is stable.
