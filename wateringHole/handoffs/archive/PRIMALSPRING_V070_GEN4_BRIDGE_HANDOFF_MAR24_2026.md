# primalSpring v0.7.0 → Phase 17 — gen4 Composition Bridge

**Date**: March 24, 2026
**From**: primalSpring coordination team
**To**: All primal teams + all spring teams + sporeGarden (esotericWebb, helixVision)
**Supersedes**: Phase 16 handoff (Phase 16 complete; this defines Phase 17 direction)
**Related**: `ecoPrimals/wateringHole/GEN4_BRIDGE.md`, `specs/GEN4_COMPOSITION_AUDIT.md`

---

## Summary

gen4 is live. **esotericWebb** (sporeGarden) is the first product that composes
deployed primals into a consumable application — a CRPG engine where primals
are invisible infrastructure. **helixVision** is planned as the second gen4
product for sovereign genomics. Both consume primals via `plasmidBin` binaries,
deploy graphs, and JSON-RPC IPC.

primalSpring sits uniquely at the gen3→gen4 boundary. We validate that primals
compose correctly; gen4 extends this to "primals compose into products." Phase 17
wires this bridge.

This handoff describes what each team needs to know and absorb.

---

## The Generational Model

| Gen | Concern | Entities |
|-----|---------|----------|
| gen1 | Build capability | Phase 1 primals (beardog, songbird, …) |
| gen2 | Protocol + philosophy | biomeOS manifesto, IPC, sovereignty |
| gen3 | Scientific validation | Springs (hotSpring, wetSpring, primalSpring, …) + primals |
| gen4 | Product composition | sporeGarden products (esotericWebb, helixVision) |
| gen5 | biomeOS as sovereign OS | The operating system itself |

**Key insight**: gen3 entities (springs, primals) don't become gen4. gen4
products *compose* gen3 entities. The springs validate that composition works.

---

## What esotericWebb Taught Us

esotericWebb V4 is ~8.5k LOC of Rust across 32 files with 166 tests. It
composes 8 primal domains into a CRPG substrate via `PrimalBridge` (JSON-RPC
IPC, TCP-first with UDS fallback). Key patterns we found:

### 1. Domain-Centric Discovery

Webb organizes primals by domain, not by primal name:

| Domain | Primal | Webb's Name |
|--------|--------|-------------|
| security | beardog | `beardog` |
| mesh | songbird | `songbird` |
| storage | nestgate | `nestgate` |
| compute | toadstool | `toadstool` |
| ai | squirrel | `squirrel` |
| dialogue | petaltongue | `petaltongue` |
| provenance | rhizocrypt | `rhizocrypt` |
| attribution | sweetgrass | `sweetgrass` |

### 2. Deploy Graphs as Composition Contract

Webb ships ~10 deploy graph TOMLs (`webb_tower.toml` through `webb_full.toml`).
Each graph includes a `validate_webb_*` node that references `primalspring_primal`
with `spawn = false` — declaring 6 composition health capabilities:

| Capability | What It Validates |
|------------|-------------------|
| `composition.webb_tower_health` | BearDog + Songbird IPC baseline |
| `composition.webb_node_health` | Tower + ToadStool compute |
| `composition.webb_nest_health` | Tower + NestGate storage |
| `composition.webb_ai_viz_health` | Tower + Squirrel + PetalTongue overlay |
| `composition.webb_provenance_health` | Nest + rhizoCrypt + loamSpine + sweetGrass |
| `composition.webb_full_health` | All domains composed |

**None of these are wired in primalSpring yet.** This is Phase 17's primary work.

### 3. Transport Priority Inversion

primalSpring discovers UDS-first (5-tier: env → XDG → temp → manifest → registry).
Webb discovers TCP-first with UDS fallback. This means primalSpring validates one
transport path and Webb uses another. Phase 17 must test both.

### 4. Capability String Drift

Capabilities appear in 4 places: primal `capabilities.list` responses, biomeOS
`neural.graph.topology` rollup, Webb's `PRIMAL_DOMAINS` table, and primalSpring's
`capability_registry.toml`. No automated check that they stay consistent.

### 5. Session Pipeline Ordering

Webb's `GameSession::act()` runs a 6-phase pipeline: narrate → dialogue → flow →
render → DAG append → complete. This depends on primal response ordering. No
primalSpring experiment validates multi-primal pipeline ordering today.

### 6. Resilience Semantics

Webb's `PrimalBridge` uses `CircuitBreaker`, `RetryPolicy`, and `call_or_default`
for graceful degradation. primalSpring has the same patterns (absorbed from
healthSpring V41) but doesn't test whether degradation *produces correct results*
from a product perspective.

---

## 7 Shortcomings (Full Audit in `specs/GEN4_COMPOSITION_AUDIT.md`)

| # | Shortcoming | Severity | Phase 17 Action |
|---|-------------|----------|-----------------|
| 1 | Composition health namespace mismatch | High | Implement `composition.webb_*_health` endpoints |
| 2 | Transport priority inversion | Medium | Add TCP-first transport tests |
| 3 | No capability drift detection | Medium | Cross-reference 4 surfaces |
| 4 | No session pipeline ordering | Medium | exp075: multi-primal pipeline ordering |
| 5 | Untested resilience semantics | Medium | Test circuit breaker + degradation correctness |
| 6 | No degradation correctness testing | Medium | Verify `call_or_default` returns sensible results |
| 7 | ludoSpring missing from plasmidBin | High | ludoSpring team must make releases |

---

## What Each Team Must Do

### BearDog Team
- **Verify** `capabilities.list` matches what Webb expects (`security.*`)
- **Publish** releases to `plasmidBin` with checksum validation
- **Test** TCP transport (Webb uses TCP-first, most primalSpring tests use UDS)

### Songbird Team
- **Verify** all 11 subsystems return correct capabilities
- **BirdSong beacon** exchange protocol needs TCP-first path testing

### NestGate Team
- **Verify** storage round-trip under Webb's `nest.*` capability expectations
- **Filesystem fallback** behavior must be tested via TCP transport

### ToadStool / barraCuda Team
- **Verify** compute dispatch works via TCP (Webb's `compute.*` path)
- **Workload type** strings must match between deploy graphs and `capabilities.list`

### Squirrel Team
- **`ai.query` reliability** — Webb uses this as the primary AI inference path
- **MCP tool discovery** via `mcp.tools.list` must match cross-primal tool schema

### PetalTongue Team
- **Dialogue trees** are GAP-001 in `esotericWebb/EVOLUTION_GAPS.md`
  (character dialogue flow protocol, text streaming, emotion metadata)
- **Grammar of Graphics** rendering via `viz.render` must be stable under TCP

### Provenance Trio (rhizoCrypt + loamSpine + sweetGrass)
- **Session lifecycle** (`dag.session.create` → `dag.event.append` → `dag.session.complete`)
  is used by Webb for every game playthrough — must be bulletproof
- **loamSpine** had a panic on `cert.verify` (GAP-003 in EVOLUTION_GAPS.md)

### ludoSpring Team
- **CRITICAL**: ludoSpring binary is **missing from plasmidBin**
  (GAP-007 in EVOLUTION_GAPS.md; confirmed by `plasmidBin/` audit)
- Webb needs `game.*` RPCs for HCI flow evaluation, engagement, DDA
- Must publish `ludospring_primal` release to `plasmidBin` with harvest.sh

### primalSpring Team (ourselves)
- **Phase 17**: Implement 6 `composition.webb_*_health` endpoints
- **exp075–077**: gen4 composition experiments (pipeline ordering, drift detection, degradation)
- **TCP-first transport path**: Add tests that mirror Webb's discovery priority
- **helixVision**: Prepare for second gen4 product (wetSpring + coralForge composition)

### All Spring Teams
- **Capability strings**: Audit your `capabilities.list` output against what appears in
  Webb's `PRIMAL_DOMAINS` and primalSpring's `capability_registry.toml`. Report drift.
- **TCP transport**: If you validate primals, add TCP transport tests alongside UDS.
- **plasmidBin releases**: Ensure your primals are released with checksums.

---

## helixVision (Second gen4 Product)

helixVision composes wetSpring genomics (16S pipeline, microbiome analytics, PFAS
screening), neuralSpring/coralForge (AlphaFold structure prediction), and the
provenance trio into a sovereign genomics discovery platform.

Same architecture pattern as esotericWebb: PrimalBridge, deploy graphs, TCP-first IPC,
graceful degradation. Different domain: field science instead of gaming.

**Implications for primalSpring**: We'll need `composition.helix_*_health` endpoints
analogous to the Webb composition health endpoints. The pattern established in Phase 17
for esotericWebb becomes the template for all gen4 products.

---

## plasmidBin Deployment Model

gen4 products consume primals via `plasmidBin` — versioned, checksummed binaries.
The deployment flow:

```
gen3 primal team → cargo build --release → harvest.sh → plasmidBin/
plasmidBin/ → fetch.sh → gen4 product binary directory
gen4 product → deploy graph → topological_waves() → spawn primals → IPC
```

**Every primal team** must have a release in `plasmidBin`. Current inventory:
beardog, songbird, nestgate, toadstool, squirrel, petaltongue, rhizocrypt,
loamspine, sweetgrass, biomeOS, ecoscribe. **Missing: ludoSpring.**

---

## Files to Review

| File | Where | What |
|------|-------|------|
| `GEN4_BRIDGE.md` | `ecoPrimals/wateringHole/` | Ecosystem-wide gen4 bridge doc |
| `HELIX_VISION.md` | `ecoPrimals/whitePaper/gen4/products/` | helixVision product paper |
| `BIOMEOS_OS_TRAJECTORY.md` | `ecoPrimals/whitePaper/gen4/architecture/` | biomeOS → sovereign OS vision |
| `GEN4_COMPOSITION_AUDIT.md` | `primalSpring/specs/` | 7 shortcomings, Phase 17 work items |
| `CROSS_SPRING_EVOLUTION.md` | `primalSpring/specs/` | Phase 17 gen4 bridge role defined |
| `CAPABILITY_ROUTING_TRACE.md` | `primalSpring/specs/` | gen4 routing categories 8–11 |
| `EVOLUTION_GAPS.md` | `esotericWebb/` | 8 gaps tracked by Webb |

---

## Relation to Prior Handoffs

| Handoff | Status |
|---------|--------|
| Phase 16 (Deep Debt Audit) | Active — all patterns still valid |
| Phase 15 (Cross-Ecosystem Absorption) | Archived — patterns absorbed into Phase 16 |
| Phase 14 (Deep Debt + Builder) | Archived — patterns absorbed into Phase 16 |
| Full Evolution (Mar 23) | Archived — this handoff supersedes for gen4 context |
| LAN Covalent Deployment | Active — deployment guide still relevant |

---

**License**: AGPL-3.0-or-later
