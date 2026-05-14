# Temporal Ecosystem Review — Debt, Horizons, and Stadial Transition

**Date**: May 13, 2026 (full pull: L3 V71/V64m/V167/V159 + node atomic, L4 H2-12 TLS shadow LIVE + DoT 10/10, L5 CATHEDRAL 6/7 Tier 2)
**Phase**: INTERSTADIAL — Niche Convergence → Atomic Deployment
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
| bearDog | 90.5% | LOW | ~~ionic lease~~ **DONE** (W102), ~~purpose-key~~ **DONE**. `crypto.seed_fingerprint` shipped. Federation (H3) |
| songbird | 73% | LOW | ~~VPS relay~~ **OPS-READY** (W202). ~~`capability.resolve`~~ **DONE** (W199-201). `mesh.*` on UDS (W204). Coverage 73→90% (H2), Tor/TLS (H2) |
| skunkBat | ~363 tests | MEDIUM | ~~live lineage~~ ~~enforcement~~ ~~NestGate protection~~ **H2 niche shipped**. Federation tests (H3) |

**Tower atomic niche**: ludoSpring wired (9 checks, 5 capabilities). bearDog
`crypto.seed_fingerprint` + songbird `mesh.*` on UDS unblock live validation.
GAP-16 addressable — remaining step is local deployment of all three.

### Node (proton) — Compute Trio Convergence

| Primal | Coverage | Debt | Critical Path |
|--------|----------|------|---------------|
| toadStool | ~85%+ | LOW | **Phase D WIRED** (S254, 74 methods, 22,900+ tests). AMD live dispatch. NV FECS-gated. |
| barraCuda | v0.4.0 | LOW | ~~DF64 NVK E2E~~ **GPU tests shipped** (Sprint 63). LAMMPS + SciPy + Kokkos benches live. NVK hardware remains. |
| coralReef | ~65% | LOW | ~~FECS/GPCCS~~ **STABILITY PROOF SHIPPED** (Sprint 7). ~~`naga::Module`~~ **`compile_module` DONE**. PTX SM120 (H2). |

**toadStool Phase D FACTORY WIRED** (S254). AMD sovereign dispatch **LIVE** via
`LocalDeviceFactory`. NV VFIO dispatch FECS-gated (`NvVfioComputeDevice` skeleton).
74 JSON-RPC methods, 22,900+ workspace tests. FECS firmware bridge is sole remaining
NV blocker (coralReef `GspBridge` contract). **Tier 2 Science API fully operational.**

**hotSpring dual-team deployment** (May 13): biomeGate continues sovereign dispatch
subproblem (FECS warm dispatch, kernel-level). strandGate (AMD + NVIDIA hardware)
stands up a secondary hotSpring team for **Node atomic deployment testing** — live
validation of the full compute trio composition against both GPU vendors.

### Nest (neutron) — Provenance Ready, Extracellular Pending

| Primal | Coverage | Debt | Horizon Gaps |
|--------|----------|------|--------------|
| nestGate | 84% | LOW | ~~transport parity~~ **DONE** (S60-61). Extracellular content distribution (H3) |
| rhizoCrypt | 93.9% | LOW | ~~GAP-36~~ **RESOLVED** (S68 — `normalize_method`, 1,637 tests). `dag.*` canonical, `provenance.*` alias |
| loamSpine | 90.9% | LOW | ~~GAP-36~~ **RESOLVED** (v0.9.16 — `session.*` aliased to `spine.*`, 1,522 tests). Postgres/RocksDB (v1) |
| sweetGrass | 91.7% | LOW | ~~GAP-36~~ **RESOLVED** (v0.7.35 — `braid.attribution.create` aliased, 1,549 tests). Auth verify (H3) |

### Meta + Scaffold

| Primal | Debt | Notes |
|--------|------|-------|
| biomeOS | LOW | v3.54 — `biomeos.spring_status` IMPLEMENTED (Tier 2 notebooks). `composition.deploy.shadow` shipped (v3.53). GAP-34 confirmed intentional |
| squirrel | LOW | Inference wiring + NestGate env unification shipped |
| petalTongue | LOW | v1.6.6 — ~~`backend=nestgate`~~ **WIRED** (`content.resolve`), live dashboard SSE |
| sourDough | LOW-MEDIUM | Scaffold JH-0/BTSP, needs musl cross-build |

---

## primalSpring Stadial Gate (L2)

- **Registry**: 418 methods, 302 exercised (73%) — `biomeos.spring_status` added
- **Waves 7/8/9**: All DONE. Wave 8 upstream items **ALL RESOLVED** (W8-07/08/09).
- **22 scenarios**, **77 deploy graphs**, **604 library tests** (all passing)
- **Phase 32 atomic evolution**: skunkBat in Tower, Nest reconciled with provenance trio
- **Tier 2 Science API**: LIVE_SCIENCE_API v1.3.0 — all methods IMPLEMENTED, 7/7 springs wired, `biomeos.spring_status` + content provenance metadata SHIPPED
- **Wire name reconciliation**: GAP-34/35/36 all resolved upstream, docs aligned

---

## River Delta Composition Readiness

| Spring | Proto-nucleate | gS | Open Gaps | Highest Leverage |
|--------|---------------|:--:|-----------|------------------|
| hotSpring | Tower+Node+Nest | L6 | FECS NV blocker | `s_node_atomic` scenario added. strandGate (AMD+NV), biomeGate (sovereign dispatch) |
| wetSpring | Tower+Node+Nest+Meta | **L5** | PG-02,04 (deploy-only) | Niche depth, B7 Tier 2 COMPLETE |
| groundSpring | Tower+Node+Nest | L4 | coralReef IPC, PRNG 2b | lithoSpore B3+B4 **INGESTED** |
| airSpring | Tower+Node+Nest | L4 | ~~AG-005~~ **RESOLVED** | LTEE E3, gS L5+ |
| healthSpring | Dual-Tower Ionic | L5 | ionic bridge (upstream) | NestComposition facade, cell.toml, Foundation T10 |
| neuralSpring | Tower+Node+Meta | L5 | Squirrel provider reg | NestGate weight persistence WIRED, Squirrel inference COMPLETE |
| ludoSpring | Tower+Node+Nest+Meta | L4 | coralReef IPC (GAP-01) | V71: MDA + matchmaking + chat. Foundation T9+T10 seeded |

### Cross-cutting Blockers

1. **Ionic runtime** — not implemented (hotSpring, groundSpring, healthSpring)
2. ~~**`capability.resolve`**~~ — **DONE** (songbird Wave 199-201)
3. ~~**Live provenance trio**~~ — **GAP-36 RESOLVED** (all 3 primals now normalize wire aliases)
4. **BTSP transport negotiation** — healthSpring `FAMILY_SEED` breaks mixed deploys
5. ~~**`toadstool.validate`/`list_workloads`**~~ — **IMPLEMENTED** (S250/S245+), Tier 2 UNBLOCKED
6. **FECS firmware bridge** — sole NV sovereign dispatch blocker. biomeGate team owns resolution. strandGate team validates Node atomic on AMD (working) + NV (FECS-gated)
7. ~~**Tower live validation (GAP-16)**~~ — **RESOLVED** (ludoSpring V70 — 6/6 LIVE against running primals)

---

## Interstadial Exit — Five Pillars

| Pillar | Status | Blocking Items |
|--------|--------|----------------|
| 1. Primal Sovereignty | **SENTINELS RESOLVED** | ~~Songbird VPS~~ OPS-READY (W202), ~~coralReef FECS~~ SHIPPED (Sprint 7). BearDog TLS + BTSP dual-auth remain (projectNUCLEUS) |
| 2. projectNUCLEUS Deploys | **ACTIVE** | **H2-12 BearDog TLS shadow LIVE** (10ms vs 120ms tunnel). DoT 10/10 FIXED. H2-2b/3a/3b/3c ready |
| 3. ABG Hosting | PARTIAL | WCM compositions through Nest+Node |
| 4. lithoSpore | **GATE EXCEEDED** | **6/7 modules Tier 2 LIVE** (was 4/7). ecoBin pure Rust BLAKE3. `litho-core` library extracted |
| 5. River Delta | **GATE MET** | 8/8 Tier 4, PG thresholds |

**Zero upstream code blockers remain.** Transition requires shadow run execution
(projectNUCLEUS owns). ~~plasmidBin checksum desync~~ **RESOLVED** (pipeline hardened May 12).

---

## Temporal Priority Stack

### RESOLVED (sentinel convergence — all complete)

1. ~~**toadStool Phase C+D**~~ — Phase C **COMPLETE** (S250), Phase D **FACTORY WIRED** (S254). AMD live. 74 methods, 22,900+ tests.
2. ~~**coralReef FECS/GPCCS**~~ — **STABILITY PROOF SHIPPED** (Sprint 7). `compile_module` (naga::Module direct ingest) DONE.
3. ~~**Songbird VPS relay**~~ — **OPS-READY** (Wave 202). `capability.resolve` DONE (W199-201). `mesh.*` UDS (W204).
4. ~~**lithoSpore Tier 1**~~ — **PASS** (Module 1+2 Tier 1+2 with BLAKE3).
5. ~~**Foundation threads**~~ — **10/10 active**, all expressions authored.
6. ~~**GAP-36 provenance trio**~~ — **RESOLVED** across all 3: rhizoCrypt S68, loamSpine v0.9.16, sweetGrass v0.7.35.
7. ~~**barraCuda parity benchmarks**~~ — LAMMPS + SciPy + Kokkos benches live (Sprint 63). DF64 GPU E2E tests shipped.

### Immediate (execution + infrastructure)

6. ~~**plasmidBin checksum sync**~~ — **RESOLVED** (pipeline hardened May 12, auto-harvest + post-harvest validation)
7. **projectNUCLEUS shadow runs** (H2-2b/3a/3b/3c) — all upstream ready, deploy immediately
8. **skunkBat E2E operational validation** — JH-5 Phase 3 shipped, exercise through deploy graphs
9. **Ionic runtime** — cross-spring RPC, CompositionContext L2 pass

### Stadial entry

10. **ABG WCM compositions** through full deploy graphs + provenance trio
11. **Upstream crate extraction** (wgsl-precision, proc-sysinfo) — community engagement
