# LTEE Paper Queue Tracker

**Last Updated**: May 17, 2026 PM (post-absorption audit — wetSpring Exp381 breseq pipeline executing)
**Thread**: Foundation Thread 5 (Evolutionary Biology / LTEE)
**lithoSpore**: latest — 7/7 modules Tier 2 PASS (75/75 checks), 117 tests, cross-tier parity, Tier 3 JSON-RPC wired
**wetSpring**: V177 — Exp381 breseq pipeline on Barrick 2009 live (3/7 clones done, first ferment transcript braid exported)

## Overview

The Long-Term Evolution Experiment (LTEE) paper queue is the heart of external
scientific validation in ecoPrimals. Four springs are actively reproducing LTEE
papers; results flow to lithoSpore as validation modules and to projectFOUNDATION
as Thread 5 evidence.

## Reproduction Status

| # | Paper | Spring | Python | Rust | `expected_values.json` | lithoSpore Module | Status |
|---|-------|--------|:------:|:----:|:----------------------:|:-----------------:|--------|
| B1 | Barrick 2009 (neutral mutation accumulation) | groundSpring | 8/8 | 8/8 | YES | Module 1: ltee-mutation | **COMPLETE** |
| B2 | Wiser 2013 (fitness dynamics) | groundSpring | 9/9 | 10/10 | YES | Module 2: ltee-fitness | **COMPLETE** |
| B3 | Good 2017 (clonal interference) | groundSpring | DONE | DONE | YES | Module 3: ltee-clonal | **COMPLETE** |
| B4 | Blount 2008/2012 (citrate utilization) | groundSpring | DONE | DONE | YES | Module 4: ltee-citrate | **COMPLETE** |
| B2 | Anderson/Wiser (fitness — independent) | hotSpring | N/A | Tier 1+2 | YES | Module 7: ltee-anderson | **COMPLETE** |
| B5 | Leonard 2024 (symbiont PK/PD) | healthSpring | 8/8 | 8/8 | YES | ltee-symbiont-pk | **COMPLETE** |
| B1-ML | Mutation accumulation (ML surrogates) | neuralSpring | 8/8 | Binary DONE | YES | ML surrogates | **COMPLETE** (Py+Rust) |
| B6 | BioBrick Burden (Nat Comms 2024) | groundSpring | DONE | 34/34 | YES | Module 5: ltee-biobrick | **COMPLETE** (V145) |
| B7 | Tenaillon 2016 (264 genomes) | wetSpring | Tier 2 | 27/27 PASS | YES | Module 6: ltee-breseq | **Tier 2 COMPLETE** |

## Per-Spring Progress Summary

### groundSpring — B1, B2, B3, B4, B6 (5 papers, ALL COMPLETE)

The anchor spring for LTEE validation. All five reproductions are complete with
both Python and Rust implementations passing. lithoSpore modules 1-5 are wired
and passing in VM-validated sporePrint CI. groundSpring also contributed the
`expected_values.json` convention adopted by all other springs.

**B6 (new — V145)**: BioBrick metabolic burden (Nat Comms 2024) — 301 plasmid
log-normal fit, AIC/BIC, jackknife, KS test. Anderson disorder analogy
(Thouless/localization length). 34/34 checks passing.

**Next**: B7-B9 queued. Results feed lithoSpore modules 1-5 and Thread 5.

### hotSpring — B2 Anderson (1 paper, COMPLETE)

Independent fitness dynamics reproduction using the Anderson formulation
(separate from groundSpring's Wiser formulation). Tier 1+2 validation passing.
lithoSpore module 7 wired.

**Next**: No LTEE work remaining. Focus shifts to GPU compute trio validation.

### healthSpring — B5 Leonard (1 paper, COMPLETE)

Symbiont PK/PD reproduction with full `--format json` output and `tolerances.toml`
configuration. This is the newest LTEE reproduction and the first to use the
lithoSpore module candidate format natively.

**Next**: lithoSpore module promotion. B5 is a strong candidate for the next
lithoSpore module addition (Module 8 candidate).

### neuralSpring — B1-ML (1 paper, COMPLETE)

ML surrogate reproductions (LSTM, ESN, HMM) of the mutation accumulation data.
Python implementations complete (8/8); Rust binary done. These are not traditional
LTEE reproductions but ML-based validations of the same underlying data.

**Next**: Potential Rust-native ML validation elevation (currently Python-primary).

### wetSpring — B7 Tenaillon (1 paper, Tier 2 COMPLETE) + Exp381 Barrick 2009 (IN PROGRESS)

264-genome LTEE analysis using breseq. Tier 2 (27/27 targets) is complete.
Tier 3 provenance validation is pending (requires provenance trio — see
`PRIMAL_BLOCKED_ASKS_MAY16_2026.md` Priority 4 for sweetGrass TCP gap).

**Exp381 (V177)**: First real-data Nest Atomic composition — breseq pipeline on
Barrick 2009 (SRP001569, 7 Ara-1 clones, ~1.3 GB). Running on 4TB NVMe at
southGate. 3/7 clones done (REL1164M: 579 mutations, REL2179M: 608,
REL8593M: 1108). Mutation accumulation trend confirmed (Barrick 2009 Fig. 1).
First ferment transcript braid exported to `provenance/braids/barrick_2009_mutations.json`
(trio fields empty in standalone mode — will populate under full NUCLEUS composition).

**Next**: Complete remaining 4 Barrick clones → Tenaillon 2016 (264 genomes, ~200 GB).
Thread 4 (Environmental Genomics) expression.

## Aggregate Metrics

- **9 papers** reproduced across **4 springs**
- **8 lithoSpore modules** wired and passing (Module 5 ltee-biobrick added V145)
- **1 module candidate** pending promotion (healthSpring B5)
- **All `expected_values.json` files** present and BLAKE3-anchored
- **Thread 5** fully seeded in projectFOUNDATION with 12 data sources and 18+ targets

## Data Flow

```
Springs (LTEE papers)
  │
  ├── control/ltee_*/expected_values.json ──► lithoSpore modules
  │     (BLAKE3-anchored via litho fetch)
  │
  ├── notebooks/papers/*-ltee-*.ipynb ──► projectFOUNDATION Thread 5
  │     (science documentation)
  │
  └── target/release/validate_ltee_* ──► projectNUCLEUS workloads
        (dispatchable Rust binaries)
```

## Remaining Work

| Item | Spring | Priority | Dependency |
|------|--------|----------|------------|
| B5 lithoSpore module promotion | healthSpring | Medium | lithoSpore team schedule |
| B7 Tier 3 provenance | wetSpring | Low | sweetGrass TCP without BTSP |
| Ferment transcript braids (Tenaillon 2016) | wetSpring | HIGH | breseq pipeline + trio provenance recording (after Barrick completion) |
| Ferment transcript braids (Barrick 2009) | wetSpring | HIGH | **IN PROGRESS** — 3/7 clones done, braid exported (standalone) |
| B1-ML Rust elevation | neuralSpring | Low | None (optional enrichment) |
| Thread 4 expression seeding | wetSpring/airSpring | Medium | Domain expertise |
| Framework parity benchmarks | all 4 springs | Low (stadial) | Kokkos/LAMMPS/SciPy baselines |

## Health Assessment

The LTEE paper queue is **healthy and substantially complete**. The remaining
items are enrichments (Tier 3 provenance, ML elevation, framework parity) rather
than gaps. The core reproduction goal — independent scientific validation of
ecoPrimals' computational correctness — has been achieved.

The pace is now methodical: any new LTEE paper additions would come from
springs identifying additional papers in their domains (e.g., airSpring E3 is
queued but not started). This is the correct stadial posture.
