# Provenance Trio — Integration Readiness Handoff

**Date:** March 22, 2026
**From:** primalSpring v0.7.0
**To:** sweetGrass team, loamSpine team, rhizoCrypt team
**License:** AGPL-3.0-or-later

---

## Executive Summary

primalSpring is **fully wired** to integrate the provenance trio
(sweetGrass, loamSpine, rhizoCrypt) via Neural API `capability.call`.
All four RootPulse experiments exercise real IPC when the trio is running
and degrade gracefully when not. This handoff documents:

1. What primalSpring has prepared (including `ipc::provenance` module)
2. A build note for the trio teams (`provenance-trio-types`)
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

## 2. Build Note: `provenance-trio-types` (RESOLVED)

All three trio teams have **inlined their types** and removed the
`provenance-trio-types` path dependency (confirmed March 22, 2026):

- sweetGrass: `deny.toml` bans `provenance-trio-types`, types in local modules
- loamSpine: types in `loam-spine-core/src/trio_types.rs`
- rhizoCrypt: types in `rhizo-crypt-core/src/dehydration_wire.rs`

The temporary shim crate at `phase2/provenance-trio-types/` has been deleted.
primalSpring has zero compile-time dependency on any trio crate — all
integration is via Neural API `capability.call`.

---

## 3. What Each Team Needs to Deliver

### All Three Teams

- [x] Resolve `provenance-trio-types` path dependency (done: all trio teams inlined types)
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
| `specs/TOWER_STABILITY.md` | 87/87 gates, full progression path |
| `specs/CROSS_SPRING_EVOLUTION.md` | Phase 12 complete, future Phases 13–19 |
| `wateringHole/PRIMALSPRING_COMPOSITION_GUIDANCE.md` | Sections 1-10: all composition patterns |
| `config/primal_launch_profiles.toml` | Launch profiles for all primals including trio |
| `graphs/provenance_overlay.toml` | Deploy graph ready for trio |
| `ecoPrimal/src/harness/mod.rs` | Graph-driven composition harness |
| `ecoPrimal/src/deploy/` | Deploy graph parsing + validation |

---

**License**: AGPL-3.0-or-later
