# Niche Climate Evolution — Pre-Stadial Requirements

**Date**: 2026-05-28
**Phase**: Interstadial → Stadial
**Scope**: primalSpring-tracked requirements that must be met before stadial gates

---

## Overview

The ecosystem is in an **interstadial** phase: primals are clean (13/13), basic
NUCLEUS runs on 2 gates (eastGate, ironGate), and the primalSpring coordination
layer validates 56 scenarios across 10 tracks. But the **niche climate** — the
deployment topology, spore flow, and cross-gate mesh — is not yet warm enough
for stadial entry.

This document defines the niche climate evolution that must occur. Each section
maps to a team handoff and a validation artifact in primalSpring.

---

## NC-1: postPrimordial Spore Gateway

**Owner**: biomeOS team + lithoSpore team
**primalSpring validation**: exp115, s_nest_atomic Phase 4, NUCLEUS_VALIDATION_MATRIX U/V/W
**composition graph**: `graphs/compositions/nest_ingest_spore.toml`

### Requirements

| ID | Requirement | Owner | Blocked By |
|----|------------|-------|------------|
| NC-1.1 | `biomeos nucleus ingest` CLI subcommand lands in biomeOS | biomeOS | **CRITICAL** — hotSpring deploy step 7 already calls it |
| NC-1.2 | `biomeos nucleus emit` CLI subcommand lands in biomeOS | biomeOS | NC-1.1 |
| NC-1.3 | `pseudospore-core` wired as dependency of `ltee-cli` | lithoSpore | Parallel `litho-core::pseudospore` must be deprecated |
| NC-1.4 | `pseudospore-core` wired as dependency of biomeOS gateway | biomeOS | NC-1.1 |
| NC-1.5 | hotSpring pseudoSpore v1.6.1 ingests via NUCLEUS path (Era 3) | hotSpring + biomeOS | NC-1.1 |
| NC-1.6 | groundSpring pseudoSpore ingests via NUCLEUS path (2nd spring) | groundSpring + biomeOS | NC-1.1 |
| NC-1.7 | Provenance trio signs ingestion (rhizoCrypt + loamSpine + sweetGrass) | trio primals | Live trio on target gate |

### Climate Metric

Column U passes for 2+ springs. Column V passes for 1+ spring. Gate criterion met:
**"Any spring can emit a pseudoSpore; any NUCLEUS can ingest it."**

---

## NC-2: Multi-Gate NUCLEUS Mesh

**Owner**: primalSpring (orchestration) + cellMembrane team (ironGate VPS)
**primalSpring validation**: s_covalent_mesh, s_cross_gate_capability_call, deployment_matrix P0 cell
**Deploy graph**: `graphs/multi_node/basement_hpc_covalent.toml`

### Requirements

| ID | Requirement | Owner | Blocked By |
|----|------------|-------|------------|
| NC-2.1 | southGate stabilized to 13/13 health-responding | wetSpring / neuralSpring ops | Runtime — Songbird crashes, BearDog timeout, biomeOS socket |
| NC-2.2 | Live `s_covalent_mesh` passes eastGate ↔ ironGate ↔ southGate | primalSpring | NC-2.1 |
| NC-2.3 | Live `s_cross_gate_capability_call` passes with cellMembrane relay | primalSpring + cellMembrane | VPS Nest deployment |
| NC-2.4 | P0 deployment_matrix cell `nucleus-x86-mixed-uds` passes | primalSpring | NC-2.2 |
| NC-2.5 | Bidirectional mesh seeding coordinated across 3+ gates | primalSpring + ops | NC-2.1 |
| NC-2.6 | biomeGate elevated from Node to full NUCLEUS (9→13 primals) | hotSpring + ops | Hardware (HBM2 capacity) |

### Climate Metric

Plasmodium: 3+ gates meshed, `discovery.peers` returns cross-gate entries,
`capability.call` routes to remote primals via Songbird federation.

---

## NC-3: cellMembrane Sovereignty Boundary

**Owner**: cellMembrane team (ironGate)
**primalSpring validation**: s_membrane_composition, s_sovereignty_parity, s_kderm_boundary
**Deploy graph**: `graphs/membrane/tower_membrane.toml`

### Requirements

| ID | Requirement | Owner | Blocked By |
|----|------------|-------|------------|
| NC-3.1 | NestGate + provenance trio on VPS for remote spore ingest | cellMembrane + ops | VPS provisioning |
| NC-3.2 | K-Derm boundary published as `membrane.toml` | cellMembrane | Architecture sign-off |
| NC-3.3 | knot-dns shadow → primary cutover | cellMembrane + ops | DNS migration plan |
| NC-3.4 | Forgejo releases alongside GitHub Releases for sovereignty | cellMembrane + plasmidBin | Forgejo CI |
| NC-3.5 | sporePrint living content via NestGate `content.put` | cellMembrane + petalTongue | NC-3.1 |

### Climate Metric

VPS sovereignty stack operational: TLS (bearDog ACME), DNS (knot-dns), content
(NestGate), mesh (Songbird federation), remote access (RustDesk + TURN).

---

## NC-4: Spring NUCLEUS Depth

**Owner**: Individual spring teams
**primalSpring validation**: NUCLEUS_VALIDATION_MATRIX rows per spring

### Per-Gate Climate

| Gate | Springs | Required Depth | Current |
|------|---------|---------------|---------|
| **eastGate** | airSpring, groundSpring | Full NUCLEUS + live validation | Operational |
| **ironGate** | healthSpring, ludoSpring | Full NUCLEUS + live composition | Operational |
| **southGate** | wetSpring, neuralSpring | Node Atomic + multi-GPU compute | **7/13 health** |
| **biomeGate** | hotSpring | Node Atomic + HBM2 bench | 9/13 primals |

### Team Handoffs

| Team | Gate | Action Items |
|------|------|-------------|
| **wetSpring / neuralSpring** | southGate | Stabilize 13/13 health; Songbird crash investigation; biomeOS socket debugging; then run live `s_covalent_mesh` |
| **cellMembrane** | ironGate | NestGate + trio VPS deployment; `s_cross_gate_capability_call` live; K-Derm boundary publication |
| **hotSpring** | biomeGate | Full NUCLEUS elevation (9→13); pseudoSpore 2.0 first ingest via NUCLEUS |
| **healthSpring / ludoSpring** | ironGate | NestComposition facade; coralReef IPC gap (GAP-01); live scenarios |

---

## NC-5: lithoSpore postPrimordial Emission Pattern

**Owner**: lithoSpore team + primalSpring (coordination)
**primalSpring validation**: exp115 Phase 3-5

### The Pattern

For lithoSpore emissions to be **postPrimordial**, every pseudoSpore must:

1. **Be emitted via `litho emit-pseudospore`** (domain-agnostic, `--spring` flag)
2. **Use `pseudospore-core`** for envelope construction (not ad-hoc)
3. **Be ingested via `biomeos nucleus ingest`** (not transitional `litho ingest-pseudospore`)
4. **Carry provenance trio signatures** (Era 3)
5. **Use plasmidBin provenance-elevated checksums** (Layer 2 composite fingerprint)
6. **Produce a sweetGrass braid** linking spring origin + storage CID + trio session

### What Changes for lithoSpore

| Before (Era 2) | After (Era 3 / postPrimordial) |
|----------------|-------------------------------|
| `litho emit-pseudospore` → filesystem artifact | Same emission, but also `biomeos nucleus ingest` |
| `litho ingest-pseudospore` (local registry) | `biomeos nucleus ingest` (NestGate sovereign storage) |
| `litho audit` (standalone) | `litho audit` + NUCLEUS gateway receipt verification |
| No provenance trio | rhizoCrypt session + loamSpine entry + sweetGrass braid |
| No plasmidBin checksums | Layer 2 composite fingerprint on emitted binaries |
| Ad-hoc braid (v1.6.1 pending slots) | Filled trio braid (v2.0) |

---

## Stadial Gate Readiness Checklist

| NC | Requirement | Status | Blocks Stadial? |
|----|------------|--------|-----------------|
| NC-1 | postPrimordial Spore Gateway | **VPS DEPLOYED** — NUCLEUS 13/13 on VPS (Wave 59). Spring overlay graph validated. **Gated on biomeOS `graph.execute` (v0.2) for live column U.** | **YES** (column U) |
| NC-2 | Multi-Gate NUCLEUS Mesh | **IN PROGRESS** — VPS NUCLEUS live. southGate 7/13, biomeGate 9/13. Live `s_covalent_mesh` pending. wet/neuralSpring on southGate as concentrated pattern node. | **YES** |
| NC-3 | cellMembrane Sovereignty | **ADVANCING** — NUCLEUS deployed, 175 tests. NC-3.5 RESOLVED (bearDog `auth.issue_session` scope). **Cutovers open**: DNS NS registrar, Forgejo releases, CI inversion. | Partial |
| NC-4 | Spring NUCLEUS Depth | **ADVANCING** — VPS NUCLEUS operational. Focus: southGate 7→13/13 (wet/neuralSpring pattern node). east/iron operational. biomeGate 9→13 pending. | **YES** |
| NC-5 | lithoSpore postPrimordial | **UNBLOCKED** — Derivation anchoring wired (GUIDESTONE-GRADE 11-14, 192 tests). **Gated on `graph.execute` + column U.** | **YES** |

**Stadial entry requires**: NC-1 (2+ springs), NC-2 (3+ gates), NC-4 (all 4 named gates healthy).
NC-3 and NC-5 are progressive — partial satisfaction enables stadial entry, full
satisfaction is the stadial target.

---

## primalSpring Artifacts (This Repo)

| Artifact | Purpose | Status |
|----------|---------|--------|
| `graphs/compositions/nest_ingest_spore.toml` | 6-step spore ingest signal | **LANDED** |
| `experiments/exp115_nest_ingest_pseudospore/` | 5-phase gateway validation | **LANDED** (structural) |
| `ecoPrimal/src/validation/scenarios/s_nest_atomic.rs` Phase 4 | Spore gateway checks | **EVOLVED** (5 checks) |
| `specs/NUCLEUS_VALIDATION_MATRIX.md` U/V/W columns | Per-spring spore readiness | **LANDED** |
| `specs/NICHE_CLIMATE_EVOLUTION.md` (this file) | Pre-stadial requirement map | **LANDED** |
| `s_covalent_mesh`, `s_cross_gate_capability_call` | Multi-gate live scenarios | **EXIST** (need live run) |
| `s_membrane_composition`, `s_sovereignty_parity` | cellMembrane boundary | **EXIST** (structural pass) |
