# Dark Forest Glacial Gate Standard

**Version**: 1.0.0
**Date**: May 14, 2026
**Status**: Active — stadial entry gate
**Authority**: primalSpring (L2 coordination)
**Related**: `wateringHole/BTSP_PROTOCOL_STANDARD.md`, `wateringHole/birdsong/DARK_FOREST_BEACON_GENETICS_STANDARD.md`, `wateringHole/ECOBIN_ARCHITECTURE_STANDARD.md`

---

## Abstract

The Dark Forest Glacial Gate defines five security invariants that every NUCLEUS
deployment must satisfy before stadial transition. Named after the Three-Body
Problem principle: the safest strategy in a hostile universe is to remain
invisible. A NUCLEUS composition reveals nothing about its internal structure,
identity, or capabilities to external observers.

These invariants are validated by the `s_dark_forest_gate` scenario in
primalSpring and must pass at Tier::Rust (structural) without live primals.

---

## Gate Criteria

### Pillar 1: Zero Metadata Leakage

| Requirement | Validation |
|-------------|------------|
| Release binaries are stripped (no debug symbols) | ecoBin `stripped = true` in manifest |
| No hostnames, usernames, or filesystem paths embedded | Build-time path sanitization via `strip` + release profile |
| BirdSong beacons are encrypted to observers | Beacon payload is ChaCha20 encrypted with beacon seed; observers see noise |
| DNS queries never leak primal identity | All external DNS routed through Songbird; primals have no direct network |

**Pass condition**: All primal entries in `manifest.toml` declare `stripped = true`.
BirdSong encryption is structural (DARK_FOREST_BEACON_GENETICS v2.0 requires
encrypted beacons when `BEACON_SEED` is set). No primal binary contains
hardcoded external hostnames.

### Pillar 2: Zero Port Exposure

| Requirement | Validation |
|-------------|------------|
| UDS-only is the default transport | `PRIMALSPRING_TCP_TIER5` must be unset by default |
| TCP ports are opt-in, never default | Zero-Port Tower Atomic standard (Wave 10) |
| Port numbers are configurable via environment | `ports.env` uses `${VAR:-default}` pattern |
| No well-known fingerprint from port scanning | Configurable defaults, not fixed constants in binaries |

**Pass condition**: Tier 5 TCP discovery is off when `PRIMALSPRING_TCP_TIER5`
is unset. All 13 primal port assignments in `tolerances` match `ports.env`.
No port collision in the assignment table.

### Pillar 3: Songbird as Sole Network Surface

| Requirement | Validation |
|-------------|------------|
| All external traffic routes through Songbird | Deploy graphs use `by_capability = "network"` → songbird |
| No primal directly opens external TCP listeners | Only Songbird advertises `http`, `tls`, `mesh` capabilities |
| NAT traversal via Songbird STUN/TURN | cellMembrane relay is Songbird-operated |
| Cross-gate federation uses Songbird mesh | Multi-node graphs route through songbird nodes |

**Pass condition**: Every deploy graph that includes external network access
has a songbird node. No non-songbird graph node advertises `http` or `tls`
capabilities. The `tower_atomic` fragment includes songbird.

### Pillar 4: BTSP Crypto Integrity

| Requirement | Validation |
|-------------|------------|
| All IPC authenticated via BTSP handshake | 13/13 primals implement `btsp.negotiate` |
| ChaCha20-Poly1305 AEAD for data in transit | BTSP Phase 3 cipher negotiation returns `chacha20-poly1305` |
| HKDF-SHA256 key derivation from family seed | Handshake key info string is `btsp-v1` |
| No cleartext in production | `FAMILY_ID` set + `BIOMEOS_INSECURE=1` = refuse to start |
| Seed fingerprints verify binary authenticity | BLAKE3 checksums in `checksums.toml` for all binaries |

**Pass condition**: The BTSP protocol constants match the standard. Deploy
graphs declare `secure_by_default = true` in metadata. The `btsp.capabilities`
method (427th registry entry) is registered. All manifest primal entries that
declare `seed_fingerprint` use BLAKE3.

### Pillar 5: Enclave Computing

| Requirement | Validation |
|-------------|------------|
| Dual-tower ionic pattern for sensitive data | healthSpring proto-nucleate has `egress_fence` metadata |
| Compute dispatch respects enclave boundaries | toadStool dispatch uses `FAMILY_ID` for session isolation |
| Content-addressed storage is opaque | NestGate BLAKE3 hashes reveal no metadata about content |
| Provenance chains don't leak internal details | sweetGrass attribution uses opaque agent identifiers |

**Pass condition**: The healthspring enclave proto-nucleate graph declares
`trust_model` and `bonding_policy` with enclave semantics. Content-addressed
capabilities (`content.*`) are routed to NestGate which uses BLAKE3 (opaque).
The provenance trio graph fragment includes the three provenance primals.

---

## Validation

The `s_dark_forest_gate` scenario validates all five pillars structurally
(Tier::Rust). It reads manifest.toml, deploy graphs, and primalSpring
configuration — no live primals are needed.

Live validation of Dark Forest properties (e.g., verifying encrypted beacon
payloads on the wire) is deferred to the stadial phase where multi-gate
deployments provide real external observers.

---

## References

- BTSP Protocol Standard: `wateringHole/BTSP_PROTOCOL_STANDARD.md`
- Dark Forest Beacon Genetics: `wateringHole/birdsong/DARK_FOREST_BEACON_GENETICS_STANDARD.md`
- ecoBin Architecture: `wateringHole/ECOBIN_ARCHITECTURE_STANDARD.md`
- Zero-Port Standard: `s_zero_port_standard` scenario (Wave 10)
- Membrane Channel Architecture: `wateringHole/MEMBRANE_CHANNEL_ARCHITECTURE.md`
