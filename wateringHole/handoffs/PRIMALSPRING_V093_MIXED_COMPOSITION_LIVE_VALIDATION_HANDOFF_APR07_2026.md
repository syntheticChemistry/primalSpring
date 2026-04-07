# primalSpring v0.9.3 — Mixed Composition + Live Validation Matrix Handoff

**Date**: April 7, 2026  
**From**: primalSpring (coordination spring)  
**To**: All primal teams, biomeOS team, downstream springs + sporeGarden  
**Phase**: 26 — Mixed Composition + Live Validation Matrix

---

## Summary

primalSpring v0.9.3 introduces the particle model for compositional reasoning
(Tower=electron, Node=proton, Nest=neutron, NUCLEUS=atom), a layered validation
framework (L0-L3), 17 new sketch graphs, 3 new experiments, and the first
systematic live validation matrix run against Eastgate. Six critical-to-low
gaps documented as GAP-MATRIX items.

## Particle Model (Paper 23)

Grounded in `gen3/baseCamp/23_mass_energy_information_equivalence.md`:

| Particle | Atomic | Role | Key Property |
|----------|--------|------|-------------|
| **Electron** | Tower (BearDog + Songbird) | Mediates bonds, trust boundary | `shares_electrons()` for Covalent/Metallic bonds |
| **Proton** | Node (Tower + ToadStool) | Compute = energy, defines gate identity | Fungible across gates |
| **Neutron** | Nest (Tower + NestGate + Squirrel) | Data = energy at rest, stabilizes nucleus | Non-fungible, unique content |
| **Atom** | NUCLEUS | Complete system | All particles composed |
| **Information** | biomeOS + sweetGrass | Structural organization | Makes mass-energy conversion functional |

Tower acts as the trust boundary — all inter-atomic bonding flows through
electron (Tower) transfer or sharing, matching the `BondType::shares_electrons()`
code pattern already in primalSpring.

## Layered Validation Framework

| Layer | Scope | Sketch Graph | Experiment |
|-------|-------|-------------|------------|
| **L0** | biomeOS + any single primal routing | `primal_routing_matrix.toml` | exp091 |
| **L1** | Each atomic independently (Tower, Node, Nest) | existing graphs | exp001-003 |
| **L2** | Mixed atomics (Node + dedicated Tower, dual-Tower ionic, nest enclave) | `dual_tower_ionic.toml`, `node_with_dedicated_tower.toml`, `nest_enclave.toml` | exp092 |
| **L3** | Bonding patterns on top of atomics | `covalent_mesh_backup.toml`, `ionic_capability_lease.toml`, `organo_metal_salt.toml` | exp093 |

Springs specialize these generic patterns: healthSpring might use a dedicated
Tower for ZK client data with a second Tower/Node for health modeling.
esotericWebb might use dual-Tower for game save + AI inference separation.

## Live Validation Results (Eastgate, April 7, 2026)

### Tower Atomic (BearDog + Songbird)

| Probe | Result | Details |
|-------|--------|---------|
| BearDog `health.liveness` | **LIVE PASS** | v0.9.0, 9 capability groups |
| BearDog `crypto.sign_ed25519` | **LIVE PASS** | Ed25519 signature generated |
| BearDog `crypto.blake3_hash` | **LIVE PASS** | BLAKE3 hash confirmed |
| Songbird `health.liveness` | **LIVE PASS** | Healthy |
| Songbird HTTPS `ifconfig.me` | **LIVE PASS** | HTTP 200, 283ms, Tower crypto confirmed |
| Neural API `capability.call` | **LIVE FAIL** | GAP-MATRIX-01: 0 capabilities registered |

### Primals Not Live-Tested

NestGate, ToadStool, Squirrel, rhizoCrypt, loamSpine, sweetGrass — require
individual launch and configuration. See GAP-MATRIX-05.

## GAP-MATRIX Items

### GAP-MATRIX-01: Neural API Capability Registration (Critical)

biomeOS Neural API v2 detects BearDog and Songbird sockets and confirms them
"healthy," but reports 0 capabilities for each. The primals ARE advertising
capabilities (BearDog reports 9 capability groups via `capabilities.list`),
but biomeOS's probe mechanism doesn't match the response format.

**Impact**: All L0 Neural API routing tests FAIL. Direct IPC works.  
**Blocks**: `capability.call` path that all springs rely on.  
**Owner**: biomeOS team  
**Work**: Align `probe_primal_capabilities_standalone` with Format A/B/C/D
capability wire formats that primals actually emit.

### GAP-MATRIX-02: biomeOS Graph Parser (Medium)

biomeOS internal parser rejects `tower_atomic_bootstrap.toml` despite it being
valid TOML. Likely requires `id` field or biomeOS-specific node structure.

**Impact**: Cannot load semantic graph translations from primalSpring TOMLs.  
**Owner**: biomeOS team  
**Work**: Accept `[[graph.nodes]]` format with optional `id`, or document
the exact biomeOS TOML schema requirement.

### GAP-MATRIX-03: Songbird TLS Cipher Suites (Low)

Some HTTPS targets (httpbin.org) fail TLS handshake while others (ifconfig.me)
succeed. Songbird's custom TLS 1.3 stack may not support all required cipher
suites.

**Impact**: Some HTTPS endpoints unreachable through Tower Atomic.  
**Owner**: Songbird team  
**Work**: Expand cipher suite support (AES-256-GCM, ChaCha20-Poly1305 minimum).

### GAP-MATRIX-04: NestGate IPC Model (Medium)

NestGate uses HTTP REST, not JSON-RPC over UDS. This diverges from the uniform
IPC model that primalSpring, biomeOS, and all other primals use.

**Impact**: NestGate integration requires HTTP bridge or NestGate evolution.  
**Owner**: NestGate team  
**Work**: Either add JSON-RPC over UDS mode to NestGate, or build a thin
HTTP-to-UDS bridge primal.

### GAP-MATRIX-05: Untested Primals (Medium)

rhizoCrypt, loamSpine, sweetGrass, Squirrel, ToadStool were not running
during live validation. Their L0 routing and L1 composition patterns remain
structural-only.

**Impact**: L0 domains 3-8 are structural PASS, not live PASS.  
**Owner**: Each primal team (launch + verify `capabilities.list`)  
**Work**: Start each primal, probe health + capabilities, test Neural API routing.

### GAP-MATRIX-06: plasmidBin Binary Freshness (Low)

Binary build dates vary: primalspring_primal (Apr 7), BearDog (Mar 27),
Songbird (Mar 27 plasmidBin), NestGate (Mar 28), provenance trio (Apr 7).

**Impact**: Some binaries may not reflect latest primal evolution.  
**Owner**: Ecosystem (plasmidBin maintainer)  
**Work**: Rebuild all from source, update `manifest.toml` + `checksums.toml`.

## New Artifacts

| Type | Path | Purpose |
|------|------|---------|
| Spec | `specs/MIXED_COMPOSITION_PATTERNS.md` | Particle model, layered validation, gap inventory |
| Spec | `specs/NUCLEUS_VALIDATION_MATRIX.md` | Updated with live results + sketch cross-refs |
| Sketches | `graphs/sketches/validation/` | L0 primal routing matrix |
| Sketches | `graphs/sketches/mixed_atomics/` | L2 dual-tower, dedicated tower, nest enclave |
| Sketches | `graphs/sketches/bonding_patterns/` | L3 covalent mesh, ionic lease, OMS |
| Exp | `experiments/exp091_primal_routing_matrix/` | L0 routing validation |
| Exp | `experiments/exp092_dual_tower_ionic/` | L2 dual-tower structural |
| Exp | `experiments/exp093_covalent_mesh_backup/` | L3 mesh backup structural |

## Metrics

- **404 tests**, 72 experiments (15 tracks), 99 deploy graphs
- **43/44 (98%)** subsystem live validation (unchanged from Phase 25)
- **6 GAP-MATRIX items** from live ecosystem validation (1 critical, 2 medium, 2 low, 1 medium)
- **8 open primal gaps** (zero critical) in `docs/PRIMAL_GAPS.md`
- **Particle model** grounded in Paper 23, codified in specs + sketch graphs

## What Springs Should Do

1. **Read `specs/MIXED_COMPOSITION_PATTERNS.md`** — understand the particle model
2. **Review sketch graphs in `graphs/sketches/`** — specialize for your domain
3. **Track GAP-MATRIX-01** — if you depend on `capability.call`, you are blocked until biomeOS fixes capability registration
4. **Test your primal individually** — run `capabilities.list` and verify the format matches what biomeOS expects

---

**License**: AGPL-3.0-or-later
