# Cross-Spring Composition Parity Scorecard

> papers → Python/R → Rust → primals (IPC) → NUCLEUS composition

**Last updated**: May 12, 2026 — Phase 32 atomic model (skunkBat in Tower, Nest reconciled with provenance trio, Wave 8 upstream progress: barraCuda v0.4.0 stadial gate, toadStool Phase A+B complete, coralReef soft-deprecated absorbed crates)
**Audited by**: primalSpring composition audit
**Method**: Pulled all 8 springs to HEAD, assessed each across 9 axes

## Legend

- **G** = Green (fully implemented / present)
- **Y** = Yellow (partial / in-progress)
- **R** = Red (absent / not started)

## Scorecard

| Spring | Tests | barraCuda Coupling | primalSpring Dep | Guidestone Level | Capability Registry | Deploy Graphs | Composition Experiments | Paper Notebooks | deny.toml |
|--------|------:|-------------------|-----------------|-----------------|-------------------|--------------|----------------------|----------------|-----------|
| **primalSpring** | 689+ | None (validates, doesn't consume) | N/A (is primalSpring) | L8 (absorbed) | **G** 413 methods, sync-tested, 72% exercised | **G** 77 graphs | **G** 89 exp crates + 22 scenarios + Wave 7+8+9 contracts | **Y** 5 (frozen JSON, not live paper) | **G** bans ring/openssl |
| **hotSpring** | 1,025 | **G** optional=true, IPC-first default (`default=[]`) | **G** unconditional | **G** L6 (certified) | **G** local TOML + sync test | **G** 5 graphs | **G** 189+ exp (Tier 4 + LTEE B2 + 3-GPU sovereign) | **G** 17 + LTEE notebook | **G** bans ring/openssl/aws-lc-sys |
| **healthSpring** | 1,011 | **G** optional=true, IPC-first default (`default=[]`) | **Y** feature-gated | **G** L5 (Tier 1-3) | **G** 118 methods in TOML + CI cross-sync vs canonical 413 | **G** 7 graphs, skunkBat node | **G** 95 exp crates (exp123 NUCLEUS parity) | **G** 53 .ipynb (all controls converted) | **G** bans ring/openssl/aws-lc-sys |
| **wetSpring** | 1,613 | **G** optional=true, IPC-first default (`default=[]`) | **Y** feature-gated | **G** L4 (38/38 NUCLEUS) | **G** TOML + cross-sync 413 | **G** 7 graphs | **G** 1 exp crate (exp400 NUCLEUS composition parity) | **G** 19 + Kachkovskiy | **G** bans ring + openssl |
| **neuralSpring** | 1,453 | **G** optional=true, IPC-first default (`default=[]`) | **Y** feature-gated | **G** L5 (19 certification tests) | **G** 34 capabilities, TOML + sync test | **G** 4 graphs (3 new Phase 60) | **G** exp094 parity crate, IPC in playGround | **G** 10 (paper-linked, DOI) | **G** bans ring/openssl/rustls |
| **ludoSpring** | 854 | **G** optional=true, IPC-first default (`default=["ipc"]`) | **Y** feature-gated | **G** L4 (Tier 1-3, 3-tier certification) | **G** 28 game.* + cross-sync 413 | **G** 12 graphs, skunkBat node | **G** 100 exp fossilized, 8 scenarios | **R** 0 .ipynb (Python baselines in baselines/) | **G** bans ring/openssl |
| **groundSpring** | 1,125 | **G** optional=true, IPC-first default (`default=[]`) | **Y** feature-gated | **G** L4 (modularized 5-layer guidestone) | **G** 16 MCP tools + 6 registry sync tests | **G** 6 graphs | **G** LTEE B2+B1 reproductions (control/ + validate bins) | **G** 34 (paper-linked) | **G** bans ring/openssl |
| **airSpring** | 1,389 | **G** optional=true, IPC-first default (`default=[]`) | **Y** feature-gated (guidestone) | **G** L4 (7 deploy graphs, 10 scenarios) | **G** 46 capabilities in TOML + cross-sync 413 | **G** 7 graphs | **G** 3 exp crates (exp001-003) | **G** 25 (paper-linked) | **G** bans ring/openssl/aws-lc-sys |

## Summary by Axis

### 1. barraCuda Coupling

**COMPLETE**: All 8 springs now have `barracuda` as `optional = true` with IPC-first defaults (`default = []` or `default = ["ipc"]`). This was the primary Tier 4 evolution target and is now **achieved across the entire river delta** (May 11, 2026). ludoSpring was the exemplar; wetSpring (V162b), neuralSpring (S201), hotSpring, healthSpring, airSpring, and groundSpring all followed the pattern.

### 2. primalSpring Integration

- **hotSpring**: Only spring with **unconditional** primalSpring dependency (reference implementation pattern)
- **6 springs**: Feature-gated via `guidestone` feature (healthSpring, wetSpring, neuralSpring, ludoSpring, groundSpring, airSpring)

### 3. Guidestone Level

| Level | Springs |
|-------|---------|
| L8 | primalSpring (absorbed) |
| L6 | hotSpring (certified) |
| L5 | neuralSpring, healthSpring |
| L4 | wetSpring, groundSpring, ludoSpring, airSpring |

### 4. Capability Registry

- **Sync-tested** (highest maturity): primalSpring (canonical 413), hotSpring, neuralSpring (34 caps), groundSpring (16 MCP + 6 sync tests), healthSpring (118 methods + CI cross-sync), ludoSpring (28 game.* + cross-sync 413), wetSpring (cross-sync 413), airSpring (46 caps + cross-sync 413)
- All 8 springs CI-validated against canonical 413 (May 11)

### 5. Deploy Graphs

Total across ecosystem: **76** (primalSpring) + **5** (hotSpring) + **7** (healthSpring) + **7** (wetSpring) + **4** (neuralSpring) + **12** (ludoSpring) + **6** (groundSpring) + **4** (airSpring) = **121 deploy graphs**

### 6. Composition Experiments

| Tier | Springs |
|------|---------|
| Deep (50+ exp crates) | primalSpring (89), ludoSpring (100), healthSpring (94) |
| Moderate (exp bins/crates) | hotSpring (src/bin exp files), groundSpring (2: exp094/095 w/ CompositionContext), airSpring (3: exp001-003), wetSpring (1: exp400) |
| Minimal (guidestone/playground only) | neuralSpring (IPC in playGround) |

### 7. Paper Baselines

| Tier | Springs |
|------|---------|
| Rich (15+ notebooks) | healthSpring (53), groundSpring (34), airSpring (25), wetSpring (20), hotSpring (17) |
| Moderate (5-15) | neuralSpring (10), primalSpring (5) |
| Scripts only | ludoSpring (baselines/python/) |

### 8. Security Posture (deny.toml)

All 8 springs have workspace-root `deny.toml`. All ban `openssl`/`openssl-sys` and `ring`. hotSpring, healthSpring, and airSpring also ban `aws-lc-sys`/`aws-lc-rs`. airSpring's previous "missing workspace deny.toml" was a **false gap** — it exists and bans aws-lc-sys.

### 9. skunkBat IPC (NEW — May 11)

**8/8 springs wired** — all springs now have Rust-native skunkBat IPC modules
or skunkBat nodes in deploy graphs. The primary gap (ludoSpring, groundSpring,
airSpring had "graphs only") is now **RESOLVED**:
- **Rust IPC module**: wetSpring, neuralSpring, ludoSpring, groundSpring, airSpring, hotSpring
- **Deploy graph node**: healthSpring, ludoSpring (both)
- All modules follow the exemplar pattern: `emit_audit_event()`, socket discovery cascade,
  fire-and-forget semantics, graceful degradation when skunkBat is absent

### 10. Contract Testing (NEW — Wave 7, May 11)

**primalSpring only** — Wave 7 evolved the gate from structural to semantic contract testing:

| Item | Status | What |
|------|--------|------|
| W7-01 | **DONE** | `content` in `ALL_CAPS` + `capability_to_primal` routing |
| W7-02 | **DONE** | `s_nestgate_content_pipeline` scenario (put/get/exists/list/resolve) |
| W7-03 | **DONE** | Content Gate 1-3 in `server_ecosystem_compose.rs` |
| W7-04 | **DONE** | `content_pipeline_smoke.toml` deploy graph |
| W7-06 | **DONE** | `check_method_coverage.sh` inverse drift detection (125/413 uncovered) |
| W7-05 | **DONE** | `content.resolve` petalTongue validation — NestGate Session 60 shipped transport parity |
| W7-07 | **DONE** | NestGate transport parity verified — Session 60 wired all 4 surfaces |

**gen4 sketch**: `whitePaper/gen4/architecture/CONTRACT_TESTING_THESIS_SKETCH.md` connects
Wave 7 to the gen4 "wire format IS the contract" thesis — including NFT pipeline implications,
PrimalBridge degradation tiers, and the transport parity matrix pattern.

**Springs impact**: Once NestGate ships transport parity (W7-07), springs should adopt
`by_capability = "content"` in their deploy graphs alongside `"storage"`.

### 11. Compute Trio Composition (NEW — Wave 8, May 11)

**primalSpring only** — Wave 8 sketched the compute trio (coralReef + toadStool +
barraCuda) as the Node atomic's sovereign compute pipeline:

| Item | Status | What |
|------|--------|------|
| W8-01 | **DONE** | `docs/COMPUTE_TRIO_EVOLUTION.md` — HOW/WHERE/WHAT domain split, IPC contracts, absorption path |
| W8-02 | **DONE** | `s_compute_triangle` evolved to 5-phase sovereign dispatch contract test |
| W8-03 | **DONE** | Inverse drift audit — 5 compute aliases uncovered, critical path exercised |
| W8-04 | **DONE** | Compute trio gate tests 1-4 in `server_ecosystem_compose.rs` |
| W8-05 | **DONE** | `compute_trio_smoke.toml` deploy graph (6-phase trio health+capabilities+math) |
| W8-06 | **DONE** | gen4 `SOVEREIGN_COMPUTE_TRIO_SKETCH.md` — composition pattern + warm-catch sovereignty |
| W8-07 | **MAJOR PROGRESS** | Phase C batches 1-4 LANDED (S245-S249, cylinder 415 tests). Remaining: VFIO channels, NvDevice, Phase D |
| W8-08 | **IN PROGRESS** | coralReef coral-ember/glowplug soft-deprecated; Phase C/D markers |
| W8-09 | **DONE** | barraCuda v0.4.0 stadial gate — 15-tier PrecisionTier, 71/71 IPC |

**gen4 sketch**: `whitePaper/gen4/architecture/SOVEREIGN_COMPUTE_TRIO_SKETCH.md` connects
the compute trio to the HOW/WHERE/WHAT composition principle, warm-catch sovereignty pattern,
and era-agnostic compute (SM35 through SM120 + GFX10 + Akida NPU).

**Springs impact**: Springs that compose Node atomic (neuralSpring for ML, hotSpring for
QCD, ludoSpring for games, wetSpring for bio) will gain sovereign compute dispatch when
toadStool completes the ember/glowplug absorption (W8-07).

## 12. Wave 9: Domain Contract Sweep + Low-Priority Debt Closure (May 11, 2026)

**primalSpring** — closes all remaining LOW/deferred gaps:

| Item | Status | What |
|------|--------|------|
| PG-63 | **DONE** | Matplotlib Agg guidance — already reconciled (4 refs aligned) |
| PG-54 | **DONE** | Adaptive tick model — 3 modes (fixed/adaptive/event) in `nucleus_composition_lib.sh` |
| NestGate `storage.list` | **DONE** | Gate tests confirm opaque BLAKE3 hashes; auth scoping validated as low-risk by design |
| W9-01 | **DONE** | `s_domain_contract_sweep` scenario — 7-phase contract test (secrets, bonding, defense, discovery, provenance, spine, network) |
| W9-02 | **DONE** | `domain_contract_sweep.toml` deploy graph — 7-phase domain coverage |
| W9-03 | **DONE** | Auth boundary gate tests (`nestgate_storage_list_returns_opaque_hashes`, `nestgate_storage_list_content_addressed`) |
| W7-05 | **DONE** | `content.resolve` for petalTongue — NestGate Session 60 shipped transport parity |
| W7-07 | **DONE** | NestGate transport parity verified — all 4 surfaces |

**Metrics**: 22 scenarios, 77 deploy graphs, 301/413 methods exercised (72% coverage,
up from 69%). Remaining 112 uncovered are test fixtures, domain-specific (game/nautilus/ml),
or require external infrastructure — not primalSpring gate debt.

## 13. Downstream Readiness (NEW — Wave 10 Pattern Handoff, May 11, 2026)

Tracks per-spring readiness to hand patterns down to foundation, projectNUCLEUS, and lithoSpore.

### LTEE Reproduction Status → lithoSpore

| Spring | Paper | Python | Rust | `expected_values.json` | lithoSpore Module |
|--------|-------|:------:|:----:|:----------------------:|-------------------|
| **groundSpring** | B1 (Barrick 2009) | 8/8 | 8/8 | YES | 1: ltee-mutation |
| **groundSpring** | B2 (Wiser 2013) | 9/9 | 10/10 | YES | 2: ltee-fitness |
| **groundSpring** | B3 (Good 2017) | DONE | DONE | YES | 3: ltee-clonal |
| **hotSpring** | B2 (Anderson/Wiser) | N/A | Tier 1+2 | YES | 7: ltee-anderson |
| **neuralSpring** | B1-ML (LSTM/ESN/HMM) | 8/8 | **NO** | YES | ML surrogates |
| **wetSpring** | B7 (Tenaillon 2016) | STARTED | **NO** | **NO** | 6: ltee-genomics |

**lithoSpore integration**: 5/6 reproductions ready for module ingestion (neuralSpring Rust binary DONE — S201b `validate_ltee_b1_mutation_accumulation`). Remaining: wetSpring B7 pipeline.

### Foundation Thread Seeding Status

| Thread | Name | Expression | Sources | Targets | Status |
|--------|------|:----------:|:-------:|:-------:|--------|
| 1 | Whole-Cell Modeling | YES | 27 | 24 | **ACTIVE** |
| 2 | Plasma Physics / QCD | YES | YES | YES | **ACTIVE** |
| 3 | Immunology | **NO** | YES | YES | Needs expression (healthSpring) |
| 4 | Environmental Genomics | **NO** | YES | **NO** | Needs expression + targets (wetSpring) |
| 5 | LTEE / Evolution | **YES** | **12** | **18** | **ACTIVE** (seeded May 11) |
| 6 | Agricultural Science | YES | YES | YES | **ACTIVE** |
| 7 | Anderson Mathematics | YES | YES | YES | **ACTIVE** |
| 8 | Human Health | **NO** | YES | YES | Needs expression (healthSpring) |
| 9 | Gaming / Creative | **NO** | YES | YES | Needs expression (ludoSpring) |
| 10 | Provenance / Economics | **NO** | **NO** | **NO** | **EMPTY** (ludoSpring + primalSpring) |

**Foundation seeding**: 5/10 threads fully active. Thread 5 elevated from EMPTY → ACTIVE (Wave 10). Threads 3, 4, 8, 9 need expressions. Thread 10 needs full seeding.

### Spring → Thread Ownership for Remaining Seeding

| Thread | Name | Owner Spring | Action Needed |
|--------|------|-------------|---------------|
| 3 | Immunology | **healthSpring** | Add expression mapping (immune pathway models → thread targets) |
| 4 | Environmental Genomics | **wetSpring** | Add expression + define targets (metagenomic diversity metrics) |
| 8 | Human Health | **healthSpring** | Add expression mapping (PK/PD models → clinical targets) |
| 9 | Gaming / Creative | **ludoSpring** | Add expression (game science patterns → creative economy targets) |
| 10 | Provenance / Economics | **ludoSpring** + **primalSpring** | Full seeding: sources, targets, expression (NFT/attestation models) |

### projectNUCLEUS Tier Status

| Spring | Tier 0 (CLI) | Tier 1 (notebook+sporePrint) | Tier 2 (JSON-RPC) | Workload TOMLs | Notebooks |
|--------|:------------:|:----------------------------:|:------------------:|:--------------:|:---------:|
| **groundSpring** | YES | YES | **NO** | YES | 34 |
| **hotSpring** | YES | YES | **NO** | YES | 17 |
| **healthSpring** | YES | YES | **NO** | YES | 53 |
| **wetSpring** | YES | YES | **NO** | YES | 20 |
| **neuralSpring** | YES | YES | **NO** | YES | 10 |
| **airSpring** | YES | YES | **NO** | YES | 25 |
| **ludoSpring** | YES | PARTIAL | **NO** | YES (2) | 0 (scripts) |
| **primalSpring** | N/A | N/A | N/A | coordination | 5 |

**Tier 2 blocker**: `toadstool.validate` + `toadstool.list_workloads` JSON-RPC methods not yet implemented upstream. All springs ready once wired.

### Per-Spring Downstream Readiness Summary

| Spring | lithoSpore | foundation | projectNUCLEUS | Highest Leverage |
|--------|:----------:|:----------:|:--------------:|-----------------|
| **groundSpring** | **G** B1-B3 done | **Y** T5+T7 seeded | **G** Tier 1 | LTEE data → lithoSpore modules 1-3 |
| **hotSpring** | **G** B2 done | **G** T2 active | **G** Tier 1 | GPU ladder → NUCLEUS workloads |
| **healthSpring** | N/A | **Y** T3+T8 need expr | **G** Tier 1 | Thread 3+8 expressions |
| **wetSpring** | **Y** B7 started | **Y** T4 needs expr+tgt | **G** Tier 1 | B7 pipeline + T4 seeding |
| **neuralSpring** | **G** Py+Rust done (S201b) | **Y** T5 seeded | **G** Tier 1 | lithoSpore ML module integration |
| **airSpring** | N/A | **G** T6 active | **G** Tier 1 | T6 36 targets → foundation |
| **ludoSpring** | N/A | **Y** T9+T10 need seed | **Y** Tier 1 partial | T9 expression + T10 seeding |
| **primalSpring** | N/A | **Y** T10 co-owner | **G** coordination | T10 seeding + validation graphs |

## Phase 32: Composition Gap Sweep (May 12, 2026)

### neuralSpring Nest Decision

neuralSpring's proto-nucleate intentionally **excludes `nest_atomic`** — weight storage
via NestGate IPC is absent from the upstream proof. The spring-deploy graph includes
Nest + provenance trio for richer local validation.

**Decision**: This is **intentional dual representation**. neuralSpring's proto-nucleate
validates the ML inference pipeline (Tower + Node + Meta) without storage dependencies.
The spring-deploy graph adds Nest for lab completeness. Both are correct for their scope.

**Action**: neuralSpring should document this split in its `PRIMAL_GAPS.md` as a design
decision, not a gap. If weight storage via NestGate IPC becomes a production requirement
(e.g., lithoSpore ML modules need model persistence), the proto-nucleate should evolve.

### healthSpring BTSP Transport Negotiation

healthSpring's `FAMILY_SEED` configuration causes unconditional BTSP negotiation attempts
to all peers, breaking mixed deployments where some primals don't support BTSP server mode.

**Root cause**: healthSpring's dual-tower ionic pattern uses a `FAMILY_SEED` env var that
triggers BTSP client attempts to every discovered primal. Primals without BTSP server
endpoints reject these connections.

**Coordination needed** (primalSpring L2):
1. primalSpring should define a `btsp.capabilities` method for primals to advertise BTSP
   server support (already implied by `btsp.negotiate` but not explicitly queryable)
2. healthSpring should probe `btsp.capabilities` before attempting negotiation
3. This aligns with the `capability.resolve` pattern Songbird will ship

### Per-Spring Composition Gap Summary (Phase 32)

| Spring | Open Gaps | Phase 32 Action |
|--------|-----------|-----------------|
| hotSpring | GAP-HS-087 (trio rewire), GAP-HS-005 (ionic) | Trio rewire active; ionic awaits BearDog `crypto.sign_contract` |
| wetSpring | PG-02 (trio live), PG-03 (capability.resolve), PG-04 (NestGate live), PG-05 (sovereignty) | Awaits Songbird `capability.resolve` + lab infra |
| groundSpring | GAP-GS-008 (ionic), GAP-GS-009 (BTSP/barraCuda) | Awaits upstream ionic + BTSP wire |
| airSpring | AG-005 through AG-012 | Per-gap: Squirrel science path, coralReef compile, opaque dispatch, NestGate weather, petalTongue, TensorSession, Anderson shader, toadStool live API |
| healthSpring | Ionic bridge, NestGate egress fence, BTSP interop | See BTSP coordination above |
| neuralSpring | Nest weight IPC, BTSP session | See Nest decision above |
| ludoSpring | GAP-01 (coralReef SM), GAP-02 (barraCuda domain), GAP-04 (TensorSession) | Awaits coralReef SM rebuild + upstream barraCuda domain methods |

---

## Primordial Extinction Wave — COMPLETED (May 9, 2026)

All 8 springs have completed the interstadial eukaryotic evolution:

- **UniBin consolidation**: 8/8 springs have single unified binaries
- **Guidestone absorption**: 8/8 springs have certification organelles
- **Scenario registry**: 8/8 springs have `validation/scenarios/` with `ScenarioMeta`
- **Fossil record**: 8/8 springs consolidated to `github.com/ecoPrimals/fossilRecord` (canonical archive)
- **Zero debt markers**: 8/8 springs at zero TODO/FIXME/HACK, zero clippy warnings
- **primalSpring v0.9.25 pin**: 7/8 (healthSpring upgraded; ludoSpring pinned)

### Remaining Coordination Targets — Interstadial / Stadial Tagging

Interstadial exit criteria: `infra/wateringHole/INTERSTADIAL_EXIT_CRITERIA.md`

**INTERSTADIAL** (pre-wire — exit gate items):

1. ~~**barraCuda IPC migration (Tier 4)**~~ — `INTERSTADIAL P5`: **DONE** — **8/8 springs**
   now have `barracuda` as `optional = true` with IPC-first defaults.
   Owner: **spring teams (L3)** — COMPLETED May 11, 2026.
2. **`CompositionContext` full migration** — `INTERSTADIAL P5`: `PrimalClient`
   encapsulated inside `CompositionContext` (by design). L2 coordination pass.
   Owner: **primalSpring (L2)**.
3. ~~**Guidestone level convergence**~~ — `INTERSTADIAL P5`: **DONE** — airSpring **L4**,
   neuralSpring **L5** (exceeded L4 target). Owner: **spring teams (L3)** — COMPLETED May 11, 2026.
4. ~~**wetSpring PG gaps**~~ — `INTERSTADIAL P5`: **DONE** — 4 open (PG-02, PG-03,
   PG-04, PG-05 — all external/upstream). Closed PG-06, PG-10, PG-17, PG-18.
   Owner: **wetSpring (L3)** — COMPLETED May 11, 2026.
5. **Foundation seeding** — `INTERSTADIAL P5`: 5/10 threads active. Target: 7+/10 with
   sources/targets (seed Threads 3, 5, 8, 10). Owner: **spring teams (L3) + foundation (L5)**.
6. **LTEE paper queue progress** — `INTERSTADIAL P4`: **ACTIVE** — 4 springs reproducing.
   groundSpring B2+B1 **COMPLETE** (Python+Rust PASS), hotSpring B2 STARTED,
   wetSpring B7 STARTED, neuralSpring B1 STARTED. Critical path cleared.
   Owner: **spring teams (L3)**.

**STADIAL** (external validation — deferred to stadial cycle):

7. **barraCuda version alignment**: Mild skew (air v0.3.7 → health v0.3.13). Low
   priority. Owner: **spring teams (L3)**.
8. **Notebook gap**: ludoSpring Python baselines as scripts, not notebooks. Owner: **ludoSpring (L3)**.
9. **Framework parity benchmarks**: Per-spring vs Kokkos/LAMMPS/SciPy. Owner: **spring teams (L3)**.
10. **Upstream crate extraction**: wgsl-precision, proc-sysinfo → crates.io. Owner: **primal teams (L1)**.

**RESOLVED**:
- ~~airSpring aws-lc-sys ban~~: FALSE GAP — workspace deny.toml exists
- ~~Registry cross-sync CI~~: 8/8 springs CI-validated against 413
- ~~GAP-12~~: 28 `game.*` methods (413 total, zero drift)

## Aggregate Metrics

| Metric | Value |
|--------|-------|
| Total tests across 8 springs | **13,100+** (primalSpring 689+ + hotSpring 1,025 + healthSpring 1,011 + wetSpring 1,613 + neuralSpring 1,453 + ludoSpring 854 + groundSpring 1,125 + airSpring 1,389 + metalForge/integration/Python suites) |
| Total deploy graphs | **122** (primalSpring 77 + hotSpring 5 + healthSpring 7 + wetSpring 7 + neuralSpring 4 + ludoSpring 12 + groundSpring 6 + airSpring 4) |
| Total experiment crates | **293** (primalSpring 89 + ludoSpring 100 + healthSpring 95 + groundSpring 2 + airSpring 3 + wetSpring 1 + hotSpring exp bins + neuralSpring playGround) |
| Total paper notebooks | **198+** (healthSpring 53 + groundSpring 34 + airSpring 25 + wetSpring 20 + hotSpring 17 + neuralSpring 10 + primalSpring 5 + ludoSpring baselines) |
| Registered capability methods | **413** (primalSpring canonical, zero drift) |
| Primals in plasmidBin release | **13/13** (all architectures) |
