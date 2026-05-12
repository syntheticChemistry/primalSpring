# Downstream Pattern Guide — Springs to Products

How the 8 river delta springs feed projectNUCLEUS, foundation, and lithoSpore.

**Last updated**: May 11, 2026 (Wave 9 — zero local debt)

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
| **groundSpring** | B3 (Good 2017 — clonal interference) | **COMPLETE** | `control/ltee_clonal/` | Module 3: ltee-clonal |
| **hotSpring** | B2 (Anderson/Wiser — fitness) | **COMPLETE** Tier 1+2 | `experiments/results/ltee/ltee_b2_anderson_expected.json` | Module 7: ltee-anderson |
| **neuralSpring** | B1 (mutation accumulation ML) | **Python DONE** 8/8 | `control/ltee_mutation_accumulation/` | ML surrogate modules |
| **wetSpring** | B7 (Tenaillon 2016 — 264 genomes) | **STARTED** | `experiments/ltee/` (in progress) | Module 6: ltee-genomics |

**Convention**: lithoSpore modules consume `expected_values.json` from springs via
`fetch_and_hash.sh` scripts that BLAKE3-anchor the data into NestGate content storage.

### What lithoSpore Needs Next

- **groundSpring B1-B3** outputs are ready for integration into `ltee-mutation`, `ltee-fitness`, `ltee-clonal` modules
- **hotSpring B2** ready for `ltee-anderson` module integration
- **neuralSpring** needs Rust validation binary (Python baseline done, Rust not yet)
- **wetSpring B7** is in progress — feeds `ltee-genomics` when 264-genome pipeline completes
- **All 7 modules** are scaffold/SKIP until upstream data flows in

---

## 2. Foundation Thread Seeding (springs → foundation)

Foundation threads have three components: **expression** (the question), **data sources**
(where data lives), and **data targets** (expected numerical results).

### Thread Ownership and Seeding Status

| Thread | Name | Owner Springs | Expression | Sources | Targets | Status |
|--------|------|---------------|:----------:|:-------:|:-------:|--------|
| 1 | Whole-Cell Modeling | hotSpring, wetSpring, healthSpring | YES | YES (27) | YES (24) | **ACTIVE** |
| 2 | Plasma Physics / QCD | hotSpring | YES | YES | YES | **ACTIVE** |
| 3 | Immunology / Drug Discovery | wetSpring, airSpring, healthSpring | **NO** | YES | YES | **Needs expression** |
| 4 | Environmental Genomics | wetSpring, airSpring | **NO** | YES | **NO** | **Needs expression + targets** |
| 5 | Evolutionary Biology / LTEE | groundSpring, neuralSpring, wetSpring | **NO** | **NO** | **NO** | **CRITICAL — empty** |
| 6 | Agricultural Science | airSpring, groundSpring, wetSpring | YES | YES | YES | **ACTIVE** |
| 7 | Anderson Mathematics | hotSpring, groundSpring, wetSpring, neuralSpring | YES | YES | YES | **ACTIVE** |
| 8 | Human Health / Clinical | healthSpring | **NO** | YES | YES | **Needs expression** |
| 9 | Gaming / Creative | ludoSpring | **NO** | YES | YES | **Needs expression** |
| 10 | Provenance / Economics | ludoSpring, primalSpring | **NO** | **NO** | **NO** | **Empty** |

### Critical Gap: Thread 5 (LTEE/Evolution)

Thread 5 is the backbone thread for lithoSpore — every LTEE reproduction feeds through it.
Yet it has zero foundation content. Springs must seed:

- **Expression**: document connecting LTEE papers (Barrick 2009, Wiser 2013, Good 2017, Tenaillon 2016) to the evolutionary biology questions they answer
- **Data sources**: Dryad/NCBI accessions for LTEE datasets (groundSpring + wetSpring paper queues have these)
- **Data targets**: `expected_values.json` outputs from completed B1-B3 reproductions

**Ownership**: groundSpring (primary, 3 complete reproductions) + neuralSpring (ML surrogate) + wetSpring (genomics pipeline)

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
accessions = ["doi:10.5061/dryad.XXX"]
url = "https://datadryad.org/..."
format = "csv"
blake3 = ""
retrieved = ""
paper = "B2"
```

---

## 3. Workload + Notebook Pipeline (springs → projectNUCLEUS)

### Convergence Tiers (per SPRING_EVOLUTION_TARGETS.md)

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
| **airSpring** | 1,389 | 1 | YES (thread06_ag) | 5 + 20 papers | YES | E3 queued | 6 |
| **groundSpring** | 1,125 | 1 | YES | 5 + 29 baselines | YES | **B1-B3 DONE** | 5, 7 |
| **healthSpring** | 999 | 1 | YES (PK models) | 53 scripts | YES | B5/E2/E4 queued | 3, 8 |
| **hotSpring** | 1,025 | 1 | YES | 5 + 12 papers | YES | **B2 DONE** | 2 |
| **ludoSpring** | 854 | 1 | YES (2 TOMLs) | whitePaper/ | YES | N/A | 9, 10 |
| **neuralSpring** | 1,453 | 1 | YES | 5 + 8 papers | YES | B1 Python DONE | 5, 7 |
| **primalSpring** | 689+ | N/A | coordination | 5 (meta) | YES | coordination | coordination |
| **wetSpring** | 1,613 | 1 | YES | 5 + papers | YES | B7 STARTED | 4 |

**All 8 springs are at Tier 1.** Tier 2 requires `toadstool.validate` + `toadstool.list_workloads`
JSON-RPC methods, which are specified in `LIVE_SCIENCE_API.md` but not yet implemented upstream.

### What projectNUCLEUS Needs Next

1. **Workload TOML completeness** — ensure each spring has at least one validated workload in `projectNUCLEUS/workloads/<spring>/`
2. **`--format json` flag** — springs add structured JSON output to validation binaries (additive, doesn't break CLI)
3. **Notebook execution** — sporePrint CI (`notify-sporeprint.yml` with `content: "true"`) runs `nbconvert --execute` on push
4. **Foundation thread coverage** — Threads 5, 8, 10 need full seeding before foundation can validate them

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
