# Tower Stability Specification

**Status**: Active — primalSpring v0.3.6+  
**Date**: 2026-03-18  
**Strategy**: Tower first, then Nest, then NUCLEUS

## Co-Evolution Strategy

primalSpring co-evolves with **three teams** until the Tower Atomic is stable:

```
┌─────────────────────────────────────────────────────┐
│                  Tower Atomic                        │
│                                                      │
│   beardog (security)  ←──Neural API──→  songbird    │
│          ↑                                   ↑       │
│          └───── biomeOS (orchestration) ─────┘       │
│                                                      │
│   primalSpring validates the composition             │
└─────────────────────────────────────────────────────┘
```

Once Tower is stable → add **nestgate** for Nest Atomic.  
Once Nest is stable → add **toadstool + squirrel** for Full NUCLEUS.

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

## Current Status vs Gates

| Gate | Status | Notes |
|---|---|---|
| 1. Process Lifecycle | **PASS** (4/5) | 1.5 zombie check not yet automated |
| 2. Standard Methods | **UNBLOCKED** (5/5) | beardog + songbird now register `health.liveness`, `capabilities.list` (commit c8f882d, d882ee8) |
| 3. Capability Routing | **PARTIAL** (2/5) | 3.2 + 3.3 unblocked: biomeOS registry now has `genetic.*` + `lineage.*` (commit daa60aa). 3.1 still needs songbird Neural API routing |
| 4. TLS 1.3 E2E | **PARTIAL** (1/3) | 4.1 unblocked: beardog bare crypto bridge maps `x25519_generate_ephemeral` → `crypto.x25519_generate_ephemeral`. Full E2E needs songbird Neural API path |
| 5. Socket Discovery | **FAIL** (0/3) | beardog 3-tier, songbird 7-tier mixed — deeper structural change |
| 6. Neural API Dogfooding | **PARTIAL** (1/3) | 6.3 done: registry complete. 6.1 + 6.2 need biomeOS code changes |

**Overall: ~13/24 gates unblocked** (up from 4/24). Needs live binary rebuild + test to confirm.

## Per-Team Quick Wins

### BearDog Team (3 changes → unlocks Gates 2.1, 2.3, 4.x)

1. **`health.rs`**: Add `"health.liveness"`, `"health.readiness"` to `HealthHandler::methods()`
2. **`capabilities.rs`**: Add `"capabilities.list"` to `CapabilitiesHandler::methods()`
3. **`crypto_handler.rs`**: Add bare aliases OR pre-routing mapper for `x25519_generate_ephemeral` etc.

### Songbird Team (2 changes → unlocks Gates 2.2, 2.4, 3.1, 4.x)

1. **`service.rs`**: Add `"health.liveness"`, `"capabilities.list"` as aliases
2. **TLS + Tor + Orchestrator**: Route crypto calls through `BearDogProvider` with `RoutingMode::NeuralApi` (pattern already exists in `songbird-http-client`)

### biomeOS Team (3 changes → unlocks Gates 3.2, 3.3, 6.x)

1. **`capability_registry.toml`**: Add `genetic.*` and `lineage.*` domain translations
2. **`enroll.rs`**: Replace `DirectBeardogCaller` with `NeuralApiCapabilityCaller`
3. **`node_handlers.rs`**: Replace `discover_beardog_socket` → `discover_by_capability("security")`

## Sprint Cadence

Each sprint, primalSpring:

1. **Harvests** — pull latest binaries from each team into `plasmidBin/`
2. **Tests** — run full Tower integration suite (`cargo test --test server_integration -- tower`)
3. **Reports** — gate status table updated, regressions flagged
4. **Handoffs** — delta report to `wateringHole/handoffs/` for each team

## Progression Path

```
Tower Atomic (beardog + songbird + biomeOS)     ← WE ARE HERE
    24 gates, currently 4/24
    │
    ▼  all 24 gates green
Nest Atomic (Tower + nestgate)
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
