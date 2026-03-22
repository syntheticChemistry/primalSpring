# Tower Stability Specification

**Status**: **NUCLEUS COMPOSITION VALIDATED** — primalSpring v0.6.0  
**Date**: 2026-03-22  
**Strategy**: Tower first, then Nest, then Node, then NUCLEUS

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

### Gate 12: Nest Atomic Startup

| # | Criterion | primalSpring Test |
|---|---|---|
| 12.1 | nestgate starts in socket-only mode (no ZFS required) | `nest_atomic_live_health_check` |
| 12.2 | Nest composition = beardog + songbird + nestgate (3 primals) | `nest_atomic_live_health_check` |
| 12.3 | All 3 primals pass health liveness | `nest_atomic_live_validation` |

### Gate 13: NestGate Storage

| # | Criterion | primalSpring Test |
|---|---|---|
| 13.1 | storage.store round-trip succeeds | `nest_storage_round_trip` |
| 13.2 | storage.retrieve returns correct data | `nest_storage_round_trip` |
| 13.3 | storage.list + storage.exists respond | `nest_storage_list_exists` |
| 13.4 | model.register + model.locate respond | `nest_model_cache` |
| 13.5 | nestgate health + discover_capabilities respond | `nest_direct_health`, `nest_discover_capabilities` |

### Gate 14: Node Atomic Startup

| # | Criterion | primalSpring Test |
|---|---|---|
| 14.1 | toadstool starts in server mode with JSON-RPC | `node_atomic_live_health_check` |
| 14.2 | Node composition = beardog + songbird + toadstool (3 primals) | `node_atomic_live_health_check` |
| 14.3 | All 3 primals pass health liveness | `node_atomic_live_validation` |

### Gate 15: ToadStool Compute

| # | Criterion | primalSpring Test |
|---|---|---|
| 15.1 | toadstool.health returns healthy | `node_toadstool_health` |
| 15.2 | toadstool.query_capabilities reports workload types | `node_toadstool_capabilities` |

### Gate 16: NUCLEUS Composition

| # | Criterion | primalSpring Test |
|---|---|---|
| 16.1 | Tower + Nest + Node all start in single experiment | `exp068_full_nucleus` |
| 16.2 | All primals across all atomic layers pass health | `exp068_full_nucleus` |
| 16.3 | nestgate storage works within NUCLEUS context | `exp068_full_nucleus` |
| 16.4 | toadstool compute caps available within NUCLEUS context | `exp068_full_nucleus` |

## Current Status vs Gates (2026-03-22)

| Gate | Status | Notes |
|---|---|---|
| 1. Process Lifecycle | **PASS** (5/5) | All lifecycle gates pass |
| 2. Standard Methods | **PASS** (5/5) | `health.liveness`, `capabilities.list` confirmed live |
| 3. Capability Routing | **PASS** (5/5) | All crypto routes through Neural API |
| 4. TLS 1.3 E2E | **PASS** (3/3) | TLS handshake, internet reach, routing audit |
| 5. Socket Discovery | **PASS** (3/3) | beardog 5-tier, songbird crypto-provider, biomeOS capability-based |
| 6. Neural API Dogfooding | **PASS** (3/3) | All Neural API paths validated |
| 7. Subsystem Health | **PASS** (6/6) | discovery, STUN, BirdSong, onion, Tor, federation |
| 8. Beacon Round-Trip | **PASS** (2/2) | BirdSong encrypt→decrypt verified |
| 9. Rendezvous | **PASS** (3/3) | Local Tower beacon + STUN + onion |
| 10. Internet Reach | **PASS** (3/3) | STUN, Onion, Tor paths available |
| 11. Visualization | **PASS** (3/3) | petalTongue dashboard + grammar rendering |
| 12. Nest Startup | **PASS** (3/3) | nestgate socket-only, 3 primals, all healthy |
| 13. NestGate Storage | **PASS** (5/5) | store, retrieve, list, model cache, capabilities |
| 14. Node Startup | **PASS** (3/3) | toadstool JSON-RPC, 3 primals, all healthy |
| 15. ToadStool Compute | **PASS** (2/2) | toadstool.health, toadstool.query_capabilities |
| 16. NUCLEUS Composition | **PASS** (4/4) | Tower+Nest+Node all compose and validate |

**Gates 1-6: 24/24 PASS — Tower Core Stable**  
**Gates 7-11: 17/17 PASS — Tower Full Utilization Validated**  
**Gates 12-13: 8/8 PASS — Nest Atomic Validated**  
**Gates 14-15: 5/5 PASS — Node Atomic Validated**  
**Gate 16: 4/4 PASS — NUCLEUS Composition Validated**  
**Overall: 58/58 gates passing — Full NUCLEUS Composition**

**Live test results (31/31 green, validated 2026-03-22, parallel execution ~5s):**

Tower tests (19):
- `tower_atomic_live_health_check` — PASS
- `tower_atomic_live_capabilities` — PASS
- `tower_atomic_live_validation_result` — PASS
- `tower_neural_api_health` — PASS
- `tower_neural_api_capability_discovery` — PASS
- `tower_neural_api_full_validation` — PASS
- `tower_zombie_check` — PASS
- `tower_discovery_peer_list` — PASS
- `tower_tls_handshake` — PASS
- `tower_tls_internet_reach` — PASS
- `tower_tls_routing_audit` — PASS
- `tower_discovery_announce_find` — PASS
- `tower_stun_public_address` — PASS
- `tower_birdsong_beacon` — PASS
- `tower_onion_service` — PASS
- `tower_tor_status` — PASS
- `tower_federation_status` — PASS
- `tower_squirrel_ai_query` — PASS
- `tower_squirrel_composition_health` — PASS

Nest tests (8):
- `nest_atomic_live_health_check` — PASS
- `nest_atomic_live_capabilities` — PASS
- `nest_atomic_live_validation` — PASS
- `nest_storage_round_trip` — PASS
- `nest_storage_list_exists` — PASS
- `nest_model_cache` — PASS
- `nest_direct_health` — PASS
- `nest_discover_capabilities` — PASS

Node tests (4):
- `node_atomic_live_health_check` — PASS
- `node_atomic_live_validation` — PASS
- `node_toadstool_health` — PASS
- `node_toadstool_capabilities` — PASS

**Experiment results (all pass, validated 2026-03-22):**
- `exp062_tower_subsystem_sweep` — ALL PASS (11/12 UP)
- `exp063_pixel_tower_rendezvous` — ALL PASS (beacon + onion + STUN)
- `exp064_nestgate_internet_reach` — ALL PASS (3/5 internet paths)
- `exp065_petaltongue_tower_dashboard` — ALL PASS (dashboard + grammar)
- `exp066_nest_atomic` — ALL PASS (13/13: storage round-trip, data integrity)
- `exp067_node_atomic` — ALL PASS (13/13: 4 workload types, 24 CPU cores)
- `exp068_full_nucleus` — ALL PASS (16/16: Tower+Nest+Node composing)

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
Tower Atomic (beardog + songbird + biomeOS)     ✅ FULLY UTILIZED (41/41)
    │   Core: 24/24 gates PASS
    │   Full Utilization: 17/17 gates PASS
    │   + Squirrel AI composition ✅
    │   + petalTongue visualization ✅
    │   + songbird subsystems (11/12 UP) ✅
    │
    ▼  Tower gates inherited
Nest Atomic (Tower + nestgate)                  ✅ VALIDATED (8/8)
    │   Storage store/retrieve round-trip ✅
    │   Model cache register/locate ✅
    │   nestgate socket-only mode (no ZFS) ✅
    │
    ▼  Nest gates inherited
Node Atomic (Tower + toadstool)                 ✅ VALIDATED (5/5)
    │   toadstool.health + query_capabilities ✅
    │   4 workload types, 24 CPU cores ✅
    │   JSON-RPC dual-protocol socket ✅
    │
    ▼  All gates inherited
NUCLEUS Composition (Tower+Nest+Node)           ✅ VALIDATED (4/4)
    │   All 3 atomic layers compose together ✅
    │   58/58 total gates passing ✅
    │   31 integration tests, 7 experiments ✅
    │
    ▼  Future
Full NUCLEUS (+ Squirrel + provenance trio)
    AI coordination + provenance gates
```

Each tier inherits all gates from the previous tier.  
A primal joins the composition only when the previous tier is stable.
