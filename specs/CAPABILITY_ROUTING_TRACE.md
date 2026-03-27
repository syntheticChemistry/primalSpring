# Capability Routing Trace — Hardcoded → Semantic Evolution

> **Historical note (2026-03-23)**: This trace was created during the Mar 18 Tower Atomic investigation when gates were 4/24. As of v0.7.0, NUCLEUS is **87/87 STABLE** with full Tower + Nest + Node composition, graph-driven overlays, provenance trio integration, and multi-node bonding. The hardcoding sites documented below remain relevant as the ongoing evolution roadmap for achieving full Neural API semantic routing across the ecosystem. See `TOWER_STABILITY.md` for current gate status.

**Status**: Historical trace with ongoing evolution relevance — primalSpring v0.7.0  
**Date**: 2026-03-18 (updated 2026-03-23)  
**Context**: Tower Atomic live validation revealed method-naming mismatch between songbird and beardog, traced to hardcoded inter-primal calls that bypass capability-based routing. Now extends to multi-node bonding scenarios where capability routing is critical for cross-machine federation.

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

## Evolution Strategy: Tower First

primalSpring co-evolves with **beardog, songbird, biomeOS** until the Tower Atomic
is stable. Only then does nestgate join (Nest Atomic), then toadstool/squirrel (Full NUCLEUS).

```
Tower Atomic (beardog + songbird + biomeOS)     ← STABLE (24/24)
    ▼  all gates green
Nest Atomic (Tower + nestgate)                  ← VALIDATED (8/8)
    ▼  all storage gates green
Node Atomic (Nest + toadstool)                  ← VALIDATED (5/5)
    ▼  all compute gates green
Full NUCLEUS (Node + squirrel)                  ← VALIDATED (58/58 → 87/87 with overlays)
    ▼  overlays + provenance + Squirrel AI
Multi-Node (NUCLEUS + bonding + federation)     ← STRUCTURAL (Phase 12)
    ▼  4 deploy graphs, BondingPolicy, STUN tiers
Live Multi-Node (bonded NUCLEUS mesh)           ← FUTURE (Phase 18)
```

See `specs/TOWER_STABILITY.md` for the full gate acceptance criteria.

### gen4 Composition Validation (Phase 17)

```
Full NUCLEUS (Node + squirrel)                  ← VALIDATED (87/87 with overlays)
    ▼  overlays + provenance + Squirrel AI
Multi-Node (NUCLEUS + bonding + federation)     ← STRUCTURAL (Phase 12)
    ▼  4 deploy graphs, BondingPolicy, STUN tiers
gen4 Bridge (NUCLEUS + Webb composition)        ← NEXT (Phase 17)
    ▼  6 composition.webb_* capabilities, drift detection, session pipeline
Live Multi-Node (bonded NUCLEUS mesh)           ← FUTURE (Phase 18)
```

## Evolution Path

### Phase 1: Standard Methods (done)

Beardog and songbird register `health.liveness` and `capabilities.list`.
Beardog adds bare crypto aliases as backward-compat bridge.

- BearDog: 3 changes (`health.rs`, `capabilities.rs`, pre-routing mapper)
- Songbird: 1 change (`service.rs` aliases)
- primalSpring: live tests validate

### Phase 2: Neural API Routing

Songbird routes ALL crypto through `capability.call` via Neural API.
biomeOS replaces `DirectBeardogCaller` with `NeuralApiCapabilityCaller`.

- Songbird: extract `BearDogProvider` Neural API pattern into shared crate
- biomeOS: complete `capability_registry.toml` (`genetic.*`, `lineage.*`)
- biomeOS: replace `discover_beardog_socket()` with `discover_by_capability()`
- primalSpring: new tests validate routing path

### Phase 3: TLS 1.3 End-to-End

Songbird TLS 1.3 handshake completes X25519 via capability routing.
External HTTPS connectivity verified.

### Phase 4: Socket Discovery Alignment

All Tower primals align on 5-tier socket discovery.
biomeOS eats its own dogfood for all inter-primal calls.

### Phase 5: Stable Tower → Nest Atomic

All 24 Tower gates green. nestgate joins.
Repeat co-evolution with nestgate team for storage gates.

## primalSpring Validation Role

primalSpring validates each phase by:

1. **Live Tower integration suite** — `cargo test --test server_integration -- tower`
2. **Method availability audit** — probe each primal for both raw and semantic method names
3. **Socket connection trace** — detect when a primal opens a connection to another primal's socket directly (bypass detection)
4. **Capability translation coverage** — verify all registered raw methods have semantic mappings
5. **Gate reports** — per-sprint gate status to `wateringHole/handoffs/`

See `wateringHole/handoffs/TOWER_COEVOLUTION_GUIDE.md` for the shared contract with all three teams.

## gen4 Product Composition Routing (Phase 17 — Track 9)

gen4 products introduce a new routing topology: products consume primals via
IPC with **zero knowledge of the Neural API**. Webb's `PrimalBridge` discovers
primals by domain, not by Neural API capability routing. This creates a parallel
routing surface that primalSpring must validate.

### Category 8: esotericWebb Domain-Centric Discovery

Webb discovers primals by **8 capability domains** defined in `webb/src/ipc/mod.rs`:

| Domain | Default Primal | Webb Method Constants |
|--------|----------------|----------------------|
| `ai` | squirrel | `ai.chat`, `ai.summarize`, `ai.query` |
| `visualization` | petaltongue | `visualization.render.scene`, `interaction.poll` |
| `compute` | toadstool | `compute.dispatch.submit` |
| `storage` | nestgate | `storage.put`, `storage.get` |
| `game` | ludospring | `game.narrate_action`, `game.npc_dialogue`, `game.evaluate_flow` |
| `dag` | rhizocrypt | `dag.session.create`, `dag.event.append`, `dag.session.complete` |
| `lineage` | loamspine | `lineage.certify`, `lineage.verify` |
| `provenance` | sweetgrass | `provenance.session_create`, `provenance.braid.create` |

These domain→primal mappings are hardcoded in `PRIMAL_DOMAINS` — similar to
Category 5 (biomeOS hardcoded mapping) but in a different codebase. The difference:
Webb is a gen4 consumer, not an orchestrator. It deliberately uses domain-based
lookup because it cannot access the Neural API translation layer.

**Validation implication**: primalSpring should validate that domain→primal
mappings in Webb's bridge stay consistent with the capability registry and
deploy graphs, since Webb can't self-validate against the Neural API.

### Category 9: Capability String Drift Across Surfaces

Webb uses capability strings in four independent locations:

| Surface | File | Example |
|---------|------|---------|
| Bridge method constants | `webb/src/ipc/mod.rs` | `dag.session.create` |
| Deploy graph capabilities | `graphs/webb_*.toml` | `composition.webb_full_health` |
| Capability registry | `webb/capability_registry.toml` | `webb.session.start`, `tools.list` |
| Niche YAML | `niches/*.yaml` | `game.*`, `ai.*` (dotted glob patterns) |

No single source of truth enforces consistency across these four surfaces.
primalSpring is the natural validation point — it already validates deploy
graph capabilities and capability registry consistency for gen3 primals.
Extending this to gen4 product surfaces is a Track 9 goal.

### Category 10: Transport Priority Mismatch

Webb's `PrimalBridge::discover` uses **TCP-first, UDS fallback**:
1. If `tcp_addr` exists in capability registry → try TCP → `health_liveness()` → accept
2. Else if `socket_path` exists → try UDS → `health_liveness()` → accept
3. Else → domain absent → degrade gracefully

This is the **opposite** of primalSpring's gen3 validation, which uses UDS-first
(local socket) because springs run on the same machine as primals. gen4 products
may run on different machines (containers, remote gates), making TCP the primary
transport.

primalSpring should validate both transport orderings and ensure primals respond
identically regardless of transport.

### Category 11: Resilience Contract

Webb's `resilient_call` enforces a specific resilience contract:

```
1. CircuitBreaker.is_allowed(domain)?  → if no, Err(circuit open) immediately
2. client.call(method, params)
3. On Ok → circuit.record_success()
4. On Err:
   a. is_recoverable(err)? → retry (up to max, with exponential backoff)
   b. not recoverable? → circuit.record_failure() → Err immediately
5. All retries exhausted → Err
```

`is_recoverable` matches: Io, Timeout, ConnectionFailed.
Not recoverable: MethodNotFound, InvalidParams, ParseError.

primalSpring already has `is_recoverable()` (absorbed from wetSpring V133) and
`CircuitBreaker` (absorbed from healthSpring V42). The validation gap is that
primalSpring has never tested these against a gen4 consumer's actual call patterns.

## Multi-Node Capability Routing (Phase 12+)

Multi-node deployments amplify every hardcoding problem — a hardcoded socket path
that works on one machine is meaningless across a LAN or over NAT traversal. The
bonding model enforces capability routing as the *only* inter-node communication path.

### New Routing Patterns

| Pattern | Where It Matters | Routing Path |
|---------|-----------------|--------------|
| Cross-machine capability | HPC mesh, friend remote | `capability.call` → Songbird mesh → remote Neural API → remote primal |
| Federated storage | Data federation | `capability.call("storage", "replicate")` → NestGate cross-site |
| Bonded compute dispatch | Idle compute | `capability.call("compute", "submit")` → BondingPolicy filter → remote ToadStool |
| Provenance tracking | All federation | `capability.call("provenance.*")` → trio via Neural API (zero compile coupling) |
| NAT traversal | Remote nodes | Songbird STUN escalation → sovereignty-first → hole-punch → relay |

### BondingPolicy as Capability Filter

`BondingConstraint` acts as a runtime capability firewall for bonded nodes:

```
Remote capability.call("storage.store", ...) 
  → BondingPolicy.constraints.capability_deny contains "storage.*"
  → REJECTED (idle compute nodes share compute only)
```

This means even with full Neural API routing, a bonded node only exposes the
capabilities permitted by its BondingPolicy — enforced at the bond layer, not the
application layer.
