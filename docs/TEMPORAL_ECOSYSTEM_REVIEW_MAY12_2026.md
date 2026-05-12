# Temporal Ecosystem Review — Debt, Horizons, and Stadial Transition

**Date**: May 12, 2026
**Phase**: INTERSTADIAL — Sovereignty Pre-Wire
**Scope**: Full ecosystem stack audit — upstream sentinels through downstream products

---

## Atomic Model (Phase 31 → Phase 32 Evolution)

This review reflects the **Phase 32** atomic model, which elevates skunkBat from
a topology overlay to an integral Tower member.

| Atomic | Particle | Primals | Count | Key Capabilities |
|--------|----------|---------|------:|------------------|
| **Tower** | electron | bearDog, songbird, **skunkBat** | **3** | security, discovery, **defense** |
| **Node** | proton | Tower + toadStool, barraCuda, coralReef | **6** | + compute, tensor, shader |
| **Nest** | neutron | Tower + nestGate, rhizoCrypt, loamSpine, sweetGrass | **7** | + storage, dag, ledger, attribution |
| **NUCLEUS** | atom | Tower + Node + Nest (deduplicated) | **10** | all domain capabilities |
| **Meta tier** | — | biomeOS, squirrel, petalTongue | **3** | orchestration, ai, visualization |
| **Full NUCLEUS** | — | NUCLEUS + meta | **13** | 13 capability domains |

### What Changed (Phase 32)

- **Tower**: 2 → 3 primals (skunkBat added). Every higher atomic inherits defense.
- **Nest**: Squirrel + ai → provenance trio (rhizoCrypt + loamSpine + sweetGrass).
  Aligns Rust `AtomicType::Nest` with `nest_atomic.toml` graph fragment.
- **Fragment versions**: 2.0.0 → 3.0.0 across tower/node/nest/nucleus.
- **NUCLEUS core**: 9 → 10 domain primals.

---

## Upstream Sentinel Debt Summary

All 13 primals pass the structural gate: **13/13 MethodGate, 13/13 BTSP AEAD,
13/13 Edition 2024, 13/13 deny.toml**. Remaining debt is integration-class.

### Tower (electron) — Defense is the Weakest Link

| Primal | Coverage | Debt | Horizon Gaps |
|--------|----------|------|--------------|
| bearDog | 90.5% | LOW | `crypto.sign_contract` ionic lease (H2), purpose-key derivation (H2), federation (H3) |
| songbird | 73% | MEDIUM | `capability.resolve` (H2), coverage 73→90% (H2), Tor/TLS delegation (H2), VPS relay (H3) |
| skunkBat | ~338 tests | HIGH (integration) | BearDog live lineage (H2), network enforcement (H2), NestGate data protection (H2), federation tests (H3) |

**Cascade risk**: skunkBat's HIGH integration debt propagates to every atomic
tier. The JH-5 audit pipeline (skunkBat → rhizoCrypt → sweetGrass) is operational
but its defense-layer gaps mean audit-grade deployments are **structurally wired
but not operationally validated**.

### Node (proton) — Compute Trio Convergence

| Primal | Coverage | Debt | Critical Path |
|--------|----------|------|---------------|
| toadStool | 83.6% | MEDIUM-HIGH | **Phase C** (coral-driver absorption): VFIO, AMD/NVIDIA hw, DRM, device abstraction |
| barraCuda | v0.4.0 | LOW | VFIO Titan V revalidation, DF64 NVK E2E, 90% coverage |
| coralReef | ~65% | MEDIUM-HIGH | `bind_stat` timeout, FECS/GPCCS cold silicon, `naga::Module` direct ingest |

**toadStool Phase C** is the single highest-leverage upstream work item. It
unblocks: compute sovereignty E2E, hotSpring hardware validation, and downstream
Tier 2 APIs (`toadstool.validate`, `toadstool.list_workloads`).

### Nest (neutron) — Provenance Ready, Extracellular Pending

| Primal | Coverage | Debt | Horizon Gaps |
|--------|----------|------|--------------|
| nestGate | 84% | MEDIUM | Extracellular content distribution (H3, depends Songbird VPS relay) |
| rhizoCrypt | 93.9% | LOW | Trio integration rate-limited by skunkBat maturity |
| loamSpine | 90.9% | LOW | PostgreSQL/RocksDB backends (v1 target) |
| sweetGrass | ~LOW-MED | LOW-MEDIUM | Depends on skunkBat audit completeness for H3 |

### Meta + Scaffold

| Primal | Debt | Notes |
|--------|------|-------|
| biomeOS | LOW | Production validated, zero bypasses |
| squirrel | LOW | 90% coverage, 7.2k tests |
| petalTongue | MEDIUM | Spec-heavy, needs NestGate backend |
| sourDough | LOW-MEDIUM | Scaffold JH-0/BTSP, needs musl cross-build |

---

## primalSpring Stadial Gate (L2)

- **Registry**: 413 methods, 301 exercised (72%)
- **Waves 7/8/9**: All local items DONE. 3 upstream Wave 8 items open (W8-07/08/09)
- **22 scenarios**, **77 deploy graphs**, **602 library tests** (all passing)
- **Phase 32 atomic evolution**: skunkBat in Tower, Nest reconciled with provenance trio
- **Downstream Tier 2 gap**: `LIVE_SCIENCE_API.md` needed for `toadstool.validate`/`list_workloads`

---

## River Delta Composition Readiness

| Spring | Proto-nucleate | gS | Open Gaps | Highest Leverage |
|--------|---------------|:--:|-----------|------------------|
| hotSpring | Tower+Node+Nest | L6 | Compute trio rewire, ionic lease | GPU sovereign E2E |
| wetSpring | Tower+Node+Nest+Meta | L4 | `capability.resolve`, live trio | Close PG-02–05, L5 |
| groundSpring | Tower+Node+Nest | L4 | Ionic upstream, Squirrel | lithoSpore integration |
| airSpring | Tower+Node+Nest | L4 | AG-005–012, L6 unchecked | LTEE E3, L5+ |
| healthSpring | Dual-Tower Ionic | L5 | NestGate egress, BTSP interop | Thread 3+8 expressions |
| neuralSpring | Tower+Node+Meta | L5 | Nest weight IPC, BTSP session | Rust binary, Nest decision |
| ludoSpring | Tower+Node+Nest+Meta | L4 | coralReef SM, barraCuda domain | T9+T10 seeding |

### Cross-cutting Blockers

1. **Ionic runtime** — not implemented (hotSpring, groundSpring, healthSpring)
2. **`capability.resolve`** — pending Songbird (wetSpring, others)
3. **Live provenance trio** — UDS handlers exist, trio not running in lab
4. **BTSP transport negotiation** — healthSpring `FAMILY_SEED` breaks mixed deploys
5. **`toadstool.validate`/`list_workloads`** — blocking Tier 2 downstream

---

## Interstadial Exit — Five Pillars

| Pillar | Status | Blocking Items |
|--------|--------|----------------|
| 1. Primal Sovereignty | PARTIAL | BearDog TLS shadow, Songbird NAT+VPS, BTSP dual-auth |
| 2. projectNUCLEUS Deploys | NOT YET | H2-2b/3a/3b/3c shadow runs |
| 3. ABG Hosting | PARTIAL | WCM compositions through Nest+Node |
| 4. lithoSpore | NOT YET | 2+ modules Tier 1 with real data |
| 5. River Delta | **GATE MET** | 8/8 Tier 4, PG thresholds |

**Transition requires shadow runs more than new code** — the wiring exists,
the validation infrastructure needs to execute.

---

## Temporal Priority Stack

### Immediate (sentinel convergence)

1. **toadStool Phase C** — coral-driver absorption (highest leverage)
2. **coralReef VFIO/FECS** — silicon init alignment
3. **songbird `capability.resolve` + coverage** — unblocks springs
4. **skunkBat integration hardening** — rate-limits JH-5 pipeline

### Near-term (gate + delta)

5. **LIVE_SCIENCE_API.md** — formalize Tier 2 methods
6. **Spring composition gap sweep** — drive PG-* resolution
7. **Foundation threads 3, 4, 8, 9, 10** — expression + seeding
8. **Nucleus cell testing** — move matrix from `untested` to `pass`

### Stadial entry (shadow runs)

9. **projectNUCLEUS shadow runs** (H2-2b/3a/3b/3c)
10. **lithoSpore Tier 1** with real Dryad/NCBI data
11. **ABG WCM compositions** through full deploy graphs
