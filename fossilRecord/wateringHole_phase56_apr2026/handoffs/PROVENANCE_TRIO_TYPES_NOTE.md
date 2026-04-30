# Provenance Trio: `provenance-trio-types` Resolution Needed

**Date**: March 22, 2026
**For**: sweetGrass, loamSpine, rhizoCrypt teams
**From**: primalSpring coordination

---

## The Situation

All three trio workspaces declare a dependency on `provenance-trio-types`:

```toml
# sweetGrass/Cargo.toml
provenance-trio-types = { path = "../provenance-trio-types" }

# loamSpine/Cargo.toml
provenance-trio-types = { path = "../provenance-trio-types" }

# rhizoCrypt/Cargo.toml
provenance-trio-types = { path = "../provenance-trio-types" }
```

The directory `phase2/provenance-trio-types/` does not exist on disk,
so `cargo check` fails for all three workspaces.

## Types In Use

From grepping the trio source, these are the types imported from the crate:

| Type | Used By | Purpose |
|------|---------|---------|
| `DehydrationSummary` | rhizoCrypt (produces), loamSpine (consumes), sweetGrass (re-exports) | Wire format for DAG dehydration results |
| `AgentRef` | rhizoCrypt | Agent reference in dehydration attestations |
| `AttestationRef` | rhizoCrypt | Attestation reference in dehydration |
| `PipelineRequest` | sweetGrass (`pipeline.attribute` handler) | Input to the attribution pipeline |
| `PipelineResult` | sweetGrass (`pipeline.attribute` handler) | Output of the attribution pipeline |
| `ProvenancePipeline` | Referenced in docs/roadmap | Trait for springs producing provenance sessions |

## What primalSpring Needs

**Nothing from you** — primalSpring has zero compile-time coupling to the
trio or to `provenance-trio-types`. All integration is via Neural API
`capability.call` routing (`dag.*` → rhizoCrypt, `commit.*` → loamSpine,
`provenance.*` → sweetGrass). The `ipc::provenance` module is complete
and 4 experiments are wired up, awaiting live trio binaries in
`plasmidBin/primals/`.

## What You Need to Decide

How to resolve the shared types. Options:

1. **Create/clone the crate** at `phase2/provenance-trio-types/` —
   if it exists in a separate repo or was extracted but not pushed
2. **Inline the types** — each workspace defines its own copy of the
   shared structs, with `From` impls for wire compatibility
3. **Consolidate into one workspace** — one of the three trio crates
   owns the canonical types, the others depend on that crate directly

Once resolved, build release binaries and place them in
`ecoPrimals/plasmidBin/primals/{rhizocrypt,loamspine,sweetgrass}/`.
primalSpring's experiments will automatically exercise real provenance
flows the next time they detect live trio sockets.

## primalSpring's Readiness

| What | Status |
|------|--------|
| Launch profiles for all 3 trio primals | Done (`config/primal_launch_profiles.toml`) |
| Deploy graph: `provenance_overlay.toml` | Done (Tower + trio sequential startup) |
| `ipc::provenance` module | Done (9 functions, 18 tests, graceful degradation) |
| exp020 RootPulse 6-phase commit | Wired via `capability.call` |
| exp021 Branch/merge | Wired via `capability.call` |
| exp022 Diff/federate | Wired via `capability.call` |
| exp041 Trio science E2E chain | Wired via `capability.call` |
| biomeOS capability routing | Already in `capability_domains.rs` |
