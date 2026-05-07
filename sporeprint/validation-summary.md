+++
title = "primalSpring Validation Summary"
description = "Meta-validation orchestrator — 613 tests, 85 experiments, 13/13 BTSP Phase 3, zero open gaps"
date = 2026-05-06

[taxonomies]
primals = ["biomeos", "barracuda", "toadstool", "nestgate", "beardog", "songbird", "squirrel", "rhizocrypt", "loamspine", "sweetgrass", "petaltongue", "skunkbat", "coralreef"]
springs = ["primalspring"]
+++

## Status

- **613 tests** passing, 0 failed (32.1s full suite)
- **85 experiments** across 15 categories (tower atomic → frontier)
- **13 deploy graphs** validated (74 total nodes, 5 bond types)
- **13/13 primals** BTSP Phase 3 AEAD, all defaulting to `127.0.0.1`
- **Zero open security gaps** (PG-55 through PG-59 all RESOLVED)
- **5-tier discovery hierarchy** validated across all primals

## Key Validation Binaries

- `primalspring_guidestone` — 4-phase validation (compile, structural, checksum, semantic)
- `primalspring_trio` — 3-primal IPC integration (BearDog + Songbird + ToadStool)
- `trio_operations` — live IPC operation sweep (16 operations, 30.9s)

## Notebooks (5)

| # | Notebook | Focus |
|---|----------|-------|
| 01 | Composition Validation | Deploy graphs, bond types, profiles, discovery tiers |
| 02 | Benchmark Comparison | Rust vs Python timing, energy, guidestone phases |
| 03 | Ecosystem Evidence | 85 experiments, gap resolution, security timeline |
| 04 | Cross-Spring Connections | Primal consumption matrix, ecosystem flows, sporePrint readiness |
| 05 | BTSP Security Deep Dive | Per-primal posture, convergence arc, discovery hierarchy |

## Workload TOMLs

Not yet created — contribute to `projectNUCLEUS/workloads/primalspring/`.

## See Also

- [Spring Catalog](https://primals.eco/architecture/spring-catalog-status-science-and-evolution/) on primals.eco
- [Lab Notebooks](https://primals.eco/lab/notebooks/) for rendered notebook views
- [baseCamp Papers 23, 26](https://primals.eco/science/)
