# Downstream Pattern Guide — Springs to Products

How the 8 river delta springs feed projectNUCLEUS, foundation, and lithoSpore.

**Last updated**: May 14, 2026 — Wave 13 ecosystem reconciliation (427 methods), Tier 2 Science API exemplar, 7 new methods from spring evolution (toadstool.validate, barracuda.precision.route, shader.compile.gemm, etc.)

---

## Three Pattern Classes

Springs produce three kinds of output that downstream products consume:

1. **LTEE Reproductions** — validated scientific results (springs → lithoSpore)
2. **Foundation Thread Seeds** — data sources, targets, expressions (springs → foundation)
3. **Workload + Notebook Pipeline** — dispatchable binaries + science pages (springs → projectNUCLEUS)

```
                    springs (8)
                   ┌──────────┐
                   │ airSpring │──┐
                   │groundSpring│──┤  LTEE reproductions
                   │healthSpring│──┤  ──────────────────► lithoSpore (7 modules)
                   │ hotSpring  │──┤
                   │ ludoSpring │──┤  Foundation seeds
                   │neuralSpring│──┤  ──────────────────► foundation (10 threads)
                   │primalSpring│──┤
                   │ wetSpring  │──┤  Workloads + notebooks
                   └──────────┘  └──────────────────► projectNUCLEUS (deployment)
```

---

## 1. LTEE Reproduction Stack (springs → lithoSpore)

Each LTEE reproduction follows a standard pattern:

```
<spring>/control/ltee_*/expected_values.json     ← ground truth
<spring>/target/release/validate_ltee_*          ← Rust binary
<spring>/notebooks/papers/*-ltee-*.ipynb         ← science notebook
<spring>/experiments/results/ltee/*.json         ← raw outputs
```

### Per-Spring LTEE Status → lithoSpore Module Mapping

| Spring | Paper | Status | Output Location | lithoSpore Module |
|--------|-------|--------|-----------------|-------------------|
| **groundSpring** | B1 (Barrick 2009 — neutral mutation) | **COMPLETE** Py 8/8 + Rust 8/8 | `control/ltee_neutral/expected_values.json` | Module 1: ltee-mutation |
| **groundSpring** | B2 (Wiser 2013 — fitness dynamics) | **COMPLETE** Py 9/9 + Rust 10/10 | `control/ltee_fitness/expected_values.json` | Module 2: ltee-fitness |
| **groundSpring** | B3 (Good 2017 — clonal interference) | **COMPLETE** (V136) | `control/ltee_clonal/` | Module 3: ltee-clonal |
| **groundSpring** | B4 (Blount 2008/2012 — citrate) | **COMPLETE** (V140) | `control/ltee_citrate/` | Module 4: ltee-citrate |
| **hotSpring** | B2 (Anderson/Wiser — fitness) | **COMPLETE** Tier 1+2 | `experiments/results/ltee/ltee_b2_anderson_expected.json` | Module 7: ltee-anderson |
| **healthSpring** | B5 (Leonard 2024 — symbiont PK/PD) | **COMPLETE** Py 8/8 + Rust 8/8 (V64f) + `--format json` + `tolerances.toml` | `control/ltee_symbiont_pkpd/` (full lithoSpore module candidate) + `bin/validate_ltee_b5` | Module: ltee-symbiont-pk |
| **neuralSpring** | B1 (mutation accumulation ML) | **Py 8/8 + Rust binary DONE** (S201b) | `control/ltee_mutation_accumulation/` + `src/bin/validate_ltee_b1_*` | ML surrogate modules |
| **wetSpring** | B7 (Tenaillon 2016 — 264 genomes) | **Tier 2 COMPLETE** (27/27 PASS) | `experiments/results/ltee_b7_expected_values.json` | Module 6: ltee-breseq (Tier 2 PASS) |

**Convention**: lithoSpore modules consume `expected_values.json` from springs via
`fetch_and_hash.sh` scripts that BLAKE3-anchor the data into NestGate content storage.

### What lithoSpore Needs Next

- **groundSpring B1-B4** INGESTED — lithoSpore modules 3+4 promoted to Tier 2 (B3 Good 2017, B4 Blount 2008/2012)
- **hotSpring B2** ready for `ltee-anderson` module integration (module 7 already Tier 2 PASS)
- **neuralSpring** ML surrogates additive to modules 3+4 (not blocking)
- **healthSpring** B5 (symbiont PK/PD) **COMPLETE** — full lithoSpore module candidate
- **wetSpring B7** Tier 2 COMPLETE — feeds `ltee-breseq` (module 6 Tier 2 PASS)
- **6/7 modules Tier 2 LIVE** — only module 5 (biobricks) remains scaffold (DOI pending)

### Geo-Delocalized lithoSpore Validation (via cellMembrane)

lithoSpore guideStone USBs can now validate at remote gates through the
cellMembrane relay. The USB carries its own data and runtime (hypogeal
cotyledon pattern — see `wateringHole/LITHOSPORE_USB_DEPLOYMENT.md`):

```
lithoSpore USB at remote gate
  → ./validate detects SONGBIRD_TURN_SERVER
  → primal IPC via Songbird TURN (:3478) through cellMembrane
  → Tier 2 validation against NUCLEUS compute primals
  → liveSpore.json appended (provenance entry)
  → spring absorbs results → sporePrint publishes
```

Three operating modes per USB:
1. **Standalone**: Tier 1 Python-only (no network required)
2. **LAN**: Tier 2 Rust + primal IPC via Unix sockets
3. **Geo-delocalized**: Tier 2 via Songbird TURN through cellMembrane

Cross-hardware validation across geographic locations (AMD at strandGate,
CPU-only at friend gates, NVIDIA at biomeGate) produces the stadial
evidence for interstadial exit — published as auditable journal entries
on sporePrint (primals.eco).

---

## 2. Foundation Thread Seeding (springs → foundation)

Foundation threads have three components: **expression** (the question), **data sources**
(where data lives), and **data targets** (expected numerical results).

### Thread Ownership and Seeding Status

| Thread | Name | Owner Springs | Expression | Sources | Targets | Status |
|--------|------|---------------|:----------:|:-------:|:-------:|--------|
| 1 | Whole-Cell Modeling | hotSpring, wetSpring, healthSpring | YES | YES (27) | YES (24) | **ACTIVE** |
| 2 | Plasma Physics / QCD | hotSpring | YES | YES | YES | **ACTIVE** |
| 3 | Immunology / Drug Discovery | wetSpring, airSpring, healthSpring | YES (V64) | YES | YES | **ACTIVE** |
| 4 | Environmental Genomics | wetSpring, airSpring | **NO** | YES | **NO** | **Needs expression + targets** |
| 5 | Evolutionary Biology / LTEE | groundSpring, neuralSpring, wetSpring, hotSpring | YES | YES (12) | YES (18) | **ACTIVE** (seeded May 11) |
| 6 | Agricultural Science | airSpring, groundSpring, wetSpring | YES | YES | YES | **ACTIVE** |
| 7 | Anderson Mathematics | hotSpring, groundSpring, wetSpring, neuralSpring | YES | YES | YES | **ACTIVE** |
| 8 | Human Health / Clinical | healthSpring | YES (V64) | YES | YES | **ACTIVE** |
| 9 | Gaming / Creative | ludoSpring | **YES** (V71) | YES (13) | YES | **ACTIVE** (ludoSpring T9 seeded) |
| 10 | Provenance / Economics | ludoSpring, primalSpring, healthSpring | **YES** (V71) | YES | YES | **ACTIVE** (ludoSpring T10 seeded, healthSpring gap documented) |

### Thread 5 (LTEE/Evolution) — SEEDED (May 11)

Thread 5 was the critical empty backbone for lithoSpore. Now seeded with:
- **Expression**: `expressions/LTEE_EVOLUTIONARY_DYNAMICS.md` — connects Barrick/Wiser/Good/Tenaillon papers
- **Data sources**: 12 NCBI/Dryad accessions (`data/sources/thread05_ltee.toml`)
- **Data targets**: 18 validation targets (`data/targets/thread05_ltee_targets.toml`) — 14 validated (groundSpring B1-B3, hotSpring B2, neuralSpring B1-ML), 4 pending (wetSpring B7)

**Remaining partial threads**: 4 (Environmental Genomics — partial, lithoSpore T4 integration active), 9 (Gaming — ludoSpring T9 seeded), 10 (Provenance — ludoSpring T10 seeded)

### Foundation Seed Pattern

Each thread seed follows the structure:

```toml
# data/sources/thread05_ltee.toml
[meta]
thread = 5
thread_name = "Evolutionary Biology / LTEE"
expression = "expressions/LTEE_EVOLUTIONARY_DYNAMICS.md"

[[sources]]
id = "dryad_wiser_2013"
database = "Dryad"
description = "Wiser et al. 2013 fitness trajectory data (50,000 generations)"
accessions = ["doi:10.5061/dryad.234"]
url = "https://datadryad.org/..."
format = "csv"
blake3 = ""
retrieved = ""
paper = "B2"
```

---

## 3. Workload + Notebook Pipeline (springs → projectNUCLEUS)

### Convergence Tiers (per ecoPrimals wateringHole SPRING_EVOLUTION_TARGETS.md)

| Tier | What | Spring Requirement |
|------|------|-------------------|
| 0 | CLI binary → `[OK]/[FAIL]` stdout | Validation binary in `target/release/` |
| 1 | + notebook + frozen data + sporePrint | `notebooks/01-05`, `sporeprint/`, `experiments/results/` |
| 2 | + JSON-RPC methods via toadStool | `--format json` flag, `toadstool.validate` wiring |
| 3 | + petalTongue live dashboards | Nothing new from springs |
| Standalone | NestGate-hosted, self-sovereign | Infrastructure evolution |

### Per-Spring Tier Status

| Spring | Tests | Tier | Workload TOMLs | Notebooks | sporePrint | LTEE | Foundation Threads |
|--------|------:|:----:|:--------------:|:---------:|:----------:|:----:|:------------------:|
| **airSpring** | 1,429 | 1 | YES (thread06_ag) | 5 + 20 papers | YES | E3 queued | 6 |
| **groundSpring** | 1,123 | 1 | YES | 5 + 29 baselines | YES | **B1-B4 DONE** | 5, 7 |
| **healthSpring** | 1,018 | 2 | YES (PK + Nest atomic + cell.toml) | 95 experiments | YES | V64m: NestComposition facade, cell.toml | 3, 5, 8, 10 |
| **hotSpring** | 592 | 1 | YES | 5 + 12 papers | YES | `s_node_atomic` added, base64 extracted | 2 |
| **ludoSpring** | 896 | 2 | YES (2 TOMLs + Tower atomic) | MDA + matchmaking + chat | YES | V71: **Tower LIVE** (6/6), Foundation T9+T10 | 9, 10 |
| **neuralSpring** | 910 | 1 | YES (7 IPC modules) | 397 baselines + 27 papers | YES | V159: NestGate weights WIRED, Squirrel inference COMPLETE | 5, 7 |
| **primalSpring** | 689+ | N/A | coordination | 5 (meta) | YES | coordination | coordination |
| **wetSpring** | 1,962 | 1 | YES | 384 experiments + papers | YES | V167: gS **L5**, B7 Tier 2 COMPLETE | 4 |

**All 8 springs at Tier 1+.** healthSpring and ludoSpring at Tier 2 (atomic niche wired). wetSpring promoted to gS L5.
`toadstool.validate` **IMPLEMENTED** (S250), Phase D **FACTORY WIRED** (S254 — AMD live).
`compute.dispatch.submit` **LIVE** for AMD, FECS-gated for NV. 7/7 springs Tier 2 wired.
Provenance trio **GAP-36 RESOLVED** — all wire aliases normalized upstream.

### What projectNUCLEUS Needs Next

1. **Workload TOML completeness** — ensure each spring has at least one validated workload in `projectNUCLEUS/workloads/<spring>/`
2. **`--format json` flag** — springs add structured JSON output to validation binaries (additive, doesn't break CLI)
3. **Notebook execution** — sporePrint CI (`notify-sporeprint.yml` with `content: "true"`) runs `nbconvert --execute` on push
4. **Foundation thread coverage** — Threads 5, 8, 10 need full seeding before foundation can validate them
5. **Geo-delocalized workload dispatch** — remote gates via cellMembrane relay can run Tier 2 workloads; `liveSpore.json` provenance feeds back to sporePrint for auditable publication

### CATHEDRAL Ownership (May 14, 2026)

CATHEDRAL (lithoSpore + Foundation) now owns:
- **benchScale** — VM provisioning CLI (`--backend libvirt` wired, `russh` 0.60)
- **agentReagents** — VM image templates (`lithoSpore-validation.yaml` for musl-static validation)
- Rust evolution of bash scripts is on their roadmap

**USB pipeline VM-validated**: 6/7 lithoSpore modules Tier 2 PASS (51/51 checks) on fresh
VM with different `hostname_hash`, confirming geo-delocalized validation works.

**Upstream blockers for CATHEDRAL**:
- Songbird TURN client library (geo-delocalized Tier 2 without LAN)
- BearDog FIDO2/CTAP2 (SoloKey witness in `liveSpore.json`)
- genomeBin Tier 3 packaging for lithoSpore USB
- sporePrint pipeline wiring (`liveSpore.json` → `primals.eco`)

---

## Per-Spring Downstream Priority

| Spring | Highest Leverage Hand-Down |
|--------|--------------------------|
| **groundSpring** | LTEE B1-B3 → lithoSpore modules 1-3; Thread 5+7 → foundation; measurement science baselines |
| **hotSpring** | L6 guideStone template; LTEE B2 → lithoSpore module 7; Thread 2 → foundation; 3-tier ladder pattern |
| **wetSpring** | LTEE B7 genomics → lithoSpore module 6; forge workload taxonomy → projectNUCLEUS; Thread 4 → foundation |
| **neuralSpring** | ML surrogate scaffolding → lithoSpore forecasting; Threads 5+7 → foundation; helixVision primitives |
| **healthSpring** | PK validation → projectNUCLEUS workloads; Threads 3+8 → foundation; provenance trio IPC pattern |
| **airSpring** | 36 foundation targets → Thread 6; irrigation/soil ladder → projectNUCLEUS; methods.rs drift-proofing |
| **ludoSpring** | Composition-only niche → projectNUCLEUS; Threads 9+10 → foundation; HCI validation suite |
| **primalSpring** | foundation_validation.toml graph; BTSP/MethodGate standards; composition coordination truth |
