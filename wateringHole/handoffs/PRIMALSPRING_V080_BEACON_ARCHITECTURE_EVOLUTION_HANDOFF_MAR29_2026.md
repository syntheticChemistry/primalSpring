# primalSpring v0.8.0 — Beacon Architecture Evolution Handoff

**Date:** 2026-03-29
**From:** primalSpring (validation spring)
**To:** biomeOS team, Songbird team, BearDog team
**Status:** Active recommendations for beacon stack evolution

---

## Context

The BirdSong beacon at `nestgate.io` was originally established as a single-purpose
Dark Forest gated rendezvous endpoint. The Cloudflare Tunnel configuration dates to
the initial Nov/Dec 2025 experiments, with biomeOS API code refactored in Feb 2026.

The current architecture serves one family's beacon from one machine. The ecosystem
now needs a more general model: an **ancestor beacon** that can host multiple beacons
and guide new nodes to the correct genetic material.

---

## Ancestor Beacon Concept

### Problem

Today's beacon at `nestgate.io` is a single-family, single-instance endpoint. A client
must already possess the correct mito beacon genetics to establish any connection. There
is no public discovery layer — you either have the seed or you get noise.

This is correct for the Dark Forest security model, but insufficient for:
- Onboarding new family members (how does a new device get its first seed?)
- Multi-family hosting (a university gate serving multiple research groups)
- Beacon federation (multiple sovereign towers cooperating)

### Solution: Ancestor Beacon

The ancestor beacon is a **generic** mito-beacon endpoint that:

1. **Serves noise by default** — identical to current behavior. Scanners, bots, and
   attackers see nothing useful.
2. **Recognizes enrolled genetics** — when a client presents valid mito beacon material,
   the ancestor beacon routes them to the correct family-specific beacon.
3. **Hosts multiple family beacons** — a single `nestgate.io` instance can serve as
   rendezvous for multiple families, each with isolated genetic material.
4. **Provides enrollment rendezvous** — new devices with a partial seed (enrollment
   token from `biomeos enroll`) can complete their genetic handshake through the
   ancestor beacon without exposing family material to the network.

### Architecture

```
Internet -> Cloudflare Tunnel -> biomeOS ancestor beacon
                                   |
                                   ├── generic mito gate (noise to outsiders)
                                   |
                                   ├── family beacon registry
                                   |     ├── family 8ff3b864 -> beacon + rendezvous
                                   |     ├── family a2c9f012 -> beacon + rendezvous
                                   |     └── ...
                                   |
                                   └── enrollment endpoint
                                         (partial-seed handshake)
```

### Key Properties

- **Zero information leakage**: the number of hosted families, their IDs, and their
  member counts are not discoverable without valid genetics
- **Backward compatible**: existing clients with `8ff3b864` genetics work unchanged
- **Envelope routing**: the ancestor beacon inspects the mito envelope (not the
  payload) to route to the correct family beacon
- **STUN/NAT integration**: each family's rendezvous and relay operates independently

---

## Debt Identified in Current Beacon Stack

### biomeOS API (`crates/biomeos-api/src/`)

| File | Issue | Recommendation |
|------|-------|----------------|
| `handlers/rendezvous.rs` | `RendezvousState::new(_deprecated_socket: &str)` — dead parameter | Remove parameter, use `Default` impl |
| `handlers/rendezvous.rs` | `lineage_hash` variable holds `family_id` value | Rename to `family_id` for clarity |
| `handlers/rendezvous.rs` | `anon-{epoch}` fallback on hash failure | Return error instead of weak identity |
| `dark_forest_gate.rs` + handlers | Double token: `X-Dark-Forest-Token` header AND JSON body | Standardize on one (header for middleware, remove body duplicate) |
| `.known_beacons.json` | No schema, hardcoded deployment data in repo | Move to `XDG_DATA_HOME`, define schema in wateringHole |

### Songbird (`songbird-igd`)

| Issue | Recommendation |
|-------|----------------|
| Port 3492 hardcoded in IGD examples and tests | Use `SONGBIRD_BEACON_PORT` constant from config |

### wateringHole Documentation

| Gap | Recommendation |
|-----|----------------|
| `README.md` doesn't link `DOMAIN_INFRASTRUCTURE.md` | Add to document index |
| `STANDARDS_AND_EXPECTATIONS.md` missing WAN beacon ref | Add section or cross-ref |
| `GATE_DEPLOYMENT_STANDARD.md` missing public beacon | Add nestgate.io section |
| `GLOSSARY.md` conflates NestGate primal with nestgate.io | Add disambiguation |
| No `.known_beacons.json` schema | Create `BEACON_ADDRESS_BOOK_SCHEMA.md` |

---

## primalSpring Validation Gaps

### What primalSpring Tests Today

- `birdsong.generate_encrypted_beacon` round-trip (exp085)
- `birdsong.decrypt_beacon` same-family success (exp085, exp086)
- `genetic.derive_lineage_beacon_key` HKDF derivation (exp086)
- `birdsong.verify_lineage` chain validation (exp086)
- Neural API routing to beacon capabilities (exp087)
- Cross-gate beacon exchange in graphs (three_node_covalent)

### What primalSpring Should Add

| Experiment | What It Proves | Methods Used |
|------------|----------------|--------------|
| exp089_dark_forest_negative | Wrong-family seed CANNOT decrypt beacon | `birdsong.generate_encrypted_beacon` (family A), `birdsong.decrypt_beacon` (family B) -> assert failure |
| exp090_beacon_rendezvous_e2e | Full HTTP rendezvous flow: POST beacon, check, match | `POST /api/v1/rendezvous/beacon`, `POST /api/v1/rendezvous/check` via biomeOS HTTP |
| exp091_ancestor_beacon | Multi-family routing: ancestor beacon dispatches to correct family | Generate beacons for family A and B, present to ancestor, verify routing |
| exp092_stun_dark_forest | STUN relay respects Dark Forest gating | `stun.discover` + `stun.detect_nat_type` with Dark Forest enabled |

### Beacon Tolerances to Add (`ecoPrimal/src/tolerances/`)

```rust
pub const BEACON_GENERATE_LATENCY_US: u64 = 50_000;  // 50ms max for beacon generation
pub const BEACON_DECRYPT_LATENCY_US: u64 = 25_000;    // 25ms max for beacon decryption
pub const BEACON_RENDEZVOUS_TTL_SECS: u64 = 300;      // 5min slot TTL
pub const BEACON_MAX_FAMILIES_PER_ANCESTOR: u32 = 256; // ancestor beacon capacity
```

---

## Evolution Path

### Phase 1: Clean Current Stack (biomeOS + Songbird teams)

- Remove deprecated `RendezvousState::new` parameter
- Fix `lineage_hash` -> `family_id` naming
- Standardize token passing (header only, remove body duplicate)
- Move `.known_beacons.json` to `XDG_DATA_HOME/biomeos/`
- Define `.known_beacons.json` schema in wateringHole

### Phase 2: Ancestor Beacon (biomeOS team)

- Evolve `dark_forest_gate.rs` to support multi-family routing
- Add family beacon registry (in-memory, seeded from `.family.seed` + enrolled families)
- Add enrollment endpoint for new devices
- primalSpring validates with exp091

### Phase 3: Beacon Federation (Songbird + biomeOS teams)

- Multiple sovereign towers can federate beacons
- Songbird mesh discovers other ancestor beacons
- Cross-family relay requires mutual enrollment
- primalSpring validates with composition graphs

### Phase 4: Sovereign TLS (Songbird team)

- BirdSong TLS replaces Cloudflare for TLS termination
- Ancestor beacon serves its own certificates
- `nestgate.io` becomes fully sovereign (no Cloudflare dependency)
- primalSpring pen-test validation suite

---

## Metrics

- biomeOS beacon API: 3 source files (~600 LOC), Feb 2026 refactor
- primalSpring beacon experiments: exp085, exp086, exp087 (partial coverage)
- Proposed new experiments: 4 (exp089-092)
- wateringHole gaps identified: 5 documentation cross-links
- Tunnel: 3 subdomains, 4 QUIC connections, Dark Forest active
