# Provenance Trio — Integration Readiness Handoff

**Date:** March 22, 2026
**From:** primalSpring v0.7.0
**To:** sweetGrass team, loamSpine team, rhizoCrypt team
**License:** AGPL-3.0-or-later

---

## Executive Summary

primalSpring is **structurally ready** to integrate the provenance trio
(sweetGrass, loamSpine, rhizoCrypt) but **cannot build any of the three
today** due to a missing shared dependency. This handoff documents:

1. What primalSpring has prepared
2. The blocker preventing builds
3. What each team needs to deliver
4. How the trio will compose once binaries are available

---

## 1. What primalSpring Has Prepared

### Launch Profiles (`config/primal_launch_profiles.toml`)

```toml
[profiles.sweetgrass]   # → SWEETGRASS_SOCKET, BIOMEOS_SOCKET_DIR, BEARDOG_SOCKET
[profiles.loamspine]    # → LOAMSPINE_SOCKET, BIOMEOS_SOCKET_DIR, BEARDOG_SOCKET, RHIZOCRYPT_SOCKET
[profiles.rhizocrypt]   # → RHIZOCRYPT_SOCKET, BIOMEOS_SOCKET_DIR, BEARDOG_SOCKET
```

### Deploy Graph (`graphs/provenance_overlay.toml`)

Tower base (beardog + songbird) + provenance trio:
```
beardog (security) → songbird (discovery)
                   → rhizocrypt (dag)
                   → loamspine (lineage, depends: rhizocrypt)
                   → sweetgrass (provenance, depends: loamspine)
```

### Squirrel Wiring

Squirrel's `CONSUMED_CAPABILITIES` already declares `dag.*`, `model.*`,
`storage.*` needs. Once provenance trio sockets exist, Squirrel discovers
them via `$XDG_RUNTIME_DIR/biomeos/` socket scan or explicit env vars.

### Graphs That Reference Provenance Trio

| Graph | Primals Referenced |
|-------|-------------------|
| `continuous_tick.toml` | rhizocrypt, loamspine, sweetgrass |
| `streaming_pipeline.toml` | sweetgrass |
| `provenance_overlay.toml` | rhizocrypt, loamspine, sweetgrass |
| `nucleus_complete.toml` | rhizocrypt, loamspine, sweetgrass |

---

## 2. The Blocker: `provenance-trio-types`

All three crates depend on a shared types crate:

```
phase2/sweetGrass/crates/sweet-grass-core → path = "../../provenance-trio-types"
phase2/loamSpine/crates/loam-spine-core   → path = "../../provenance-trio-types"
phase2/rhizoCrypt/crates/rhizo-crypt-core → path = "../../provenance-trio-types"
```

**The path `phase2/provenance-trio-types/` does not exist on disk.**

This is the **single blocker** preventing any of the trio from building.

### Fix Options

1. **Create the crate**: If `provenance-trio-types` was previously extracted
   but not committed/cloned, create it at `phase2/provenance-trio-types/`
2. **Inline the types**: If the crate was planned but never created, each
   team can inline the shared types temporarily
3. **Git submodule**: If it lives in a separate repo, add it as a submodule

---

## 3. What Each Team Needs to Deliver

### All Three Teams

- [ ] Resolve the `provenance-trio-types` dependency
- [ ] Build release binary and copy to `plasmidBin/primals/`
- [ ] Verify Unix socket JSON-RPC works (line-delimited NDJSON on
  `$BIOMEOS_SOCKET_DIR/{primal}-{family_id}.sock`)
- [ ] Respond to `health.liveness` and `capability.list` methods

### rhizoCrypt

Binary name: `rhizocrypt` → copy as `plasmidBin/primals/rhizocrypt`

Required JSON-RPC methods for primalSpring integration:
- `health.liveness`
- `capability.list`
- `dag.session.create`, `dag.session.get`, `dag.session.list`
- `dag.event.append`
- `dag.vertex.get`, `dag.frontier.get`
- `dag.merkle.root`, `dag.merkle.proof`, `dag.merkle.verify`

### loamSpine

Binary name: `loamspine` → copy as `plasmidBin/primals/loamspine`

Required JSON-RPC methods:
- `health.liveness`
- `capability.list`
- `spine.create`, `spine.get`, `spine.seal`
- `entry.append`, `entry.get`
- `certificate.mint`, `certificate.transfer`
- `session.commit`

### sweetGrass

Binary name: `sweetgrass` → copy as `plasmidBin/primals/sweetgrass`

Required JSON-RPC methods:
- `health.liveness`
- `capability.list`
- `braid.create`, `braid.get`, `braid.commit`
- `anchoring.anchor`, `anchoring.verify`
- `provenance.graph`, `provenance.export_provo`
- `attribution.chain`

---

## 4. How Integration Will Work

Once binaries are in `plasmidBin/primals/`, primalSpring will:

1. Add the trio to `AtomicType` or compose via `provenance_overlay.toml`
2. Create `exp071_provenance_trio` experiment validating:
   - DAG session lifecycle (rhizoCrypt)
   - Spine + entry round-trip (loamSpine)
   - Braid creation + provenance graph (sweetGrass)
   - Cross-trio: create DAG session → append entries → commit spine → create braid
3. Add integration tests (`provenance_dag_session`, `provenance_spine_entry`, etc.)
4. Wire into Squirrel: `ai.query` results stored in content-addressed DAG,
   lineage tracked, cryptographically signed

### The Prize

With the provenance trio integrated, Squirrel can:
- Store AI query results in a content-addressed DAG (rhizoCrypt)
- Track the lineage of every AI output (loamSpine)
- Create attribution chains for data provenance (sweetGrass)
- Cryptographically sign the provenance chain (beardog)

This completes the **sovereign AI stack**: every AI output is verifiable,
traceable, and attributable.

---

## 5. primalSpring Reference Material

| Document | What It Shows |
|----------|--------------|
| `specs/TOWER_STABILITY.md` | 77/77 gates, full progression path |
| `specs/CROSS_SPRING_EVOLUTION.md` | Phase 8 complete, Phase 9 awaiting trio |
| `wateringHole/PRIMALSPRING_COMPOSITION_GUIDANCE.md` | Sections 1-10: all composition patterns |
| `config/primal_launch_profiles.toml` | Launch profiles for all primals including trio |
| `graphs/provenance_overlay.toml` | Deploy graph ready for trio |
| `ecoPrimal/src/harness/mod.rs` | Graph-driven composition harness |
| `ecoPrimal/src/deploy.rs` | Deploy graph parsing + validation |

---

**License**: AGPL-3.0-or-later
