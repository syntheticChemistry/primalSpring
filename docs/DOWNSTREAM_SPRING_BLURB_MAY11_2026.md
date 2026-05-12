# River Delta → Downstream: Pattern Handoff Blurb (May 11, 2026)

**To**: foundation, projectNUCLEUS, lithoSpore teams
**From**: primalSpring coordination
**Context**: All 8 springs are Tier 4+, 13,100+ total tests, zero upstream debt, zero local debt. The springs have evolved mature patterns ready for downstream absorption.

---

## What This Blurb Covers

Three things downstream teams need from each spring:

1. **LTEE outputs** → lithoSpore module data
2. **Foundation thread seeds** → foundation content and validation targets
3. **Workload + notebook artifacts** → projectNUCLEUS deployment pipeline

---

## Per-Spring Guidance

### groundSpring — LTEE Backbone + Measurement Science

**What's ready**:
- B1 (Barrick 2009), B2 (Wiser 2013), B3 (Good 2017) reproductions are **COMPLETE** (Python + Rust)
- `control/ltee_neutral/expected_values.json`, `control/ltee_fitness/expected_values.json`, `control/ltee_clonal/` — all validated
- 29 measurement baselines, 395/395 checks, 1,125 tests

**lithoSpore**: Consume `expected_values.json` from B1-B3 into modules 1-3 (`ltee-mutation`, `ltee-fitness`, `ltee-clonal`). BLAKE3-hash all artifacts through NestGate content pipeline.

**foundation**: Thread 5 (LTEE/Evolution) and Thread 7 (Anderson Math) are the primary threads. Thread 5 is now seeded with expression + 12 data sources + 18 targets referencing groundSpring paths. Validate targets against `expected_values.json` and set `validated = true` + BLAKE3 hash.

**projectNUCLEUS**: `validate_ltee_*` binaries are available in `target/release/`. Wire into `workloads/groundSpring/` TOMLs. Notebooks 01-05 + 29 baselines ready for `nbconvert --execute` in sporePrint CI.

---

### hotSpring — LTEE Anderson RMT + Plasma Physics

**What's ready**:
- B2 (Anderson/Wiser fitness) LTEE reproduction **COMPLETE** (Tier 1+2)
- `experiments/results/ltee/ltee_b2_anderson_expected.json` — Anderson RMT eigenvalue spacing validated
- L6 guideStone template pattern, 3-tier ladder (CPU → GPU → sovereign), 1,025 tests

**lithoSpore**: Consume `ltee_b2_anderson_expected.json` into module 7 (`ltee-anderson`). The Anderson RMT connection (eigenvalue spacing follows GOE distribution) bridges LTEE to physics threads.

**foundation**: Thread 2 (Plasma/QCD) is fully active. Thread 7 (Anderson Math) has cross-spring contributions from hotSpring's RMT analysis.

**projectNUCLEUS**: GPU workload TOMLs ready. The 3-tier ladder pattern (CPU fallback → CUDA GPU → sovereign compute) is the reference model for all GPU-accelerated workloads.

---

### wetSpring — LTEE Genomics + Environmental Science

**What's ready**:
- B7 (Tenaillon 2016) 264-genome pipeline is **STARTED** — sovereign variant calling
- Forge workload taxonomy pattern, 1,613 tests (highest in river delta)
- Thread 4 (Environmental Genomics) sources seeded

**lithoSpore**: Module 6 (`ltee-genomics`) is scaffolded and waiting for B7 pipeline completion. The 264-genome dataset is the largest single LTEE data source — when complete, it feeds hypermutator phenotype detection + parallelism scoring.

**foundation**: Thread 4 needs **data targets + expression**. The wetSpring paper queue has NCBI accessions for Thread 4 sources. Thread 5 targets reference `wetSpring/experiments/ltee/` for B7 outputs (4 targets, `validated = false` pending pipeline).

**projectNUCLEUS**: Forge taxonomy pattern is reference for multi-tool workloads. Wire existing notebooks into sporePrint CI.

---

### neuralSpring — ML Surrogates + Helical Primitives

**What's ready**:
- B1 ML surrogate (LSTM/ESN/HMM mutation forecasting) Python baseline **DONE** (8/8)
- 1,453 tests (second highest), helixVision primitives
- Thread 7 (Anderson Math) — neural surrogate contributions

**lithoSpore**: ML surrogate modules consume B1-ML baseline outputs from `control/ltee_mutation_accumulation/`. The LSTM MSE (2.3) and ESN MSE (3.1) are Thread 5 targets. **Next**: Rust validation binary to bring ML surrogates to Tier 2.

**foundation**: Thread 5 (LTEE) ML targets are now seeded. Thread 7 contributions pending.

**projectNUCLEUS**: helixVision primitives feed future scientific visualization workloads.

---

### healthSpring — PK Validation + Clinical Threads

**What's ready**:
- PK/PD validation models, 999 tests, 53 scripts
- Thread 1 (WCM) contributions via ABG whole-cell modeling
- Provenance trio IPC pattern (rhizoCrypt → loamSpine → sweetGrass for clinical audit)

**foundation**: Thread 3 (Immunology) has sources + targets but **needs expression**. Thread 8 (Human Health) has sources + targets but **needs expression**. healthSpring owns both expressions — this is the highest-leverage contribution.

**projectNUCLEUS**: PK model workload TOMLs are ready. Wire into `workloads/healthSpring/`.

---

### airSpring — Agricultural Science + Environmental Monitoring

**What's ready**:
- 1,389 tests, 36 foundation targets documented, E3 LTEE paper queued
- Thread 6 (Agricultural Science) fully active with sources + targets

**foundation**: Thread 6 is the most complete agricultural thread (36 targets). Thread 3 (Immunology) has airSpring as contributor — can help seed expression. Thread 4 has airSpring contributions.

**projectNUCLEUS**: Irrigation and soil sensor workloads ready. Wire notebooks into sporePrint CI. `methods.rs` drift-proofing pattern is reference for all springs.

---

### ludoSpring — Composition-Only Niche + Gaming Science

**What's ready**:
- 854 tests, 2 workload TOMLs, Composition-only validation niche (no local primal builds)
- Thread 9 (Gaming) seeded via RPGPT/HCI papers
- Adaptive tick model (`TICK_MODE=adaptive`) reference implementation

**foundation**: Thread 9 has sources + targets but **needs expression**. Thread 10 (Provenance/Economics) is **empty** — needs full seeding. ludoSpring + primalSpring co-own Thread 10.

**projectNUCLEUS**: The `nucleus_composition_lib.sh` adaptive tick model (fixed/adaptive/event) was designed for ludoSpring's 60Hz requirement. Workload TOMLs exist. Notebooks are in `whitePaper/` (non-standard path — projectNUCLEUS should symlink or adapt).

---

### primalSpring — Coordination Truth + Validation Gate

**What's ready**:
- 689+ tests, 77 deploy graphs, 22 scenarios, 72% method coverage
- Zero local debt (Wave 9 complete)
- All coordination docs updated: `DOWNSTREAM_PATTERN_GUIDE.md`, `CROSS_SPRING_PARITY_SCORECARD.md`

**foundation**: Thread 10 (Provenance/Economics) co-owner with ludoSpring. NFT (Novel Ferment Transcript) concept sketched in `whitePaper/gen4/`. Thread 10 needs full seeding.

**projectNUCLEUS**: `foundation_validation.toml` deploy graph validates foundation thread consistency. BTSP/MethodGate standards are the reference for primal IPC. The `check_method_coverage.sh` inverse drift tool catches registered-but-untested methods.

---

## Summary: What Downstream Needs to Do

### lithoSpore
1. Ingest `expected_values.json` from groundSpring B1-B3 and hotSpring B2
2. BLAKE3-anchor all data through NestGate content pipeline
3. Wait for wetSpring B7 and neuralSpring Rust binary before activating modules 6 and ML surrogates

### foundation
1. **Critical**: Write expressions for Threads 3, 4, 8, 9 (healthSpring, wetSpring, healthSpring, ludoSpring own these)
2. Thread 5 is now seeded — validate targets as springs complete reproductions
3. Thread 10 needs full seeding (ludoSpring + primalSpring)
4. Set `validated = true` + BLAKE3 hashes on targets as they pass

### projectNUCLEUS
1. Wire `validate_ltee_*` binaries into workload TOMLs
2. Run `nbconvert --execute` on spring notebooks via sporePrint CI
3. Plan Tier 2 evolution: `--format json` flag on spring binaries → `toadstool.validate` JSON-RPC wiring
