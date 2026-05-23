+++
title = "primalSpring Validation Summary"
description = "Meta-validation orchestrator — 784 lib tests, 89 experiments, 458 capability methods, 13/13 BTSP, zero DEBT, Wave 46"
date = 2026-05-23

[taxonomies]
primals = ["biomeos", "barracuda", "toadstool", "nestgate", "beardog", "songbird", "squirrel", "rhizocrypt", "loamspine", "sweetgrass", "petaltongue", "skunkbat", "coralreef"]
springs = ["primalspring"]
+++

## Status

- **784 lib tests** (784 passed, 2 ignored), 0 failed
- **89 experiments** across 20 tracks (tower atomic → frontier)
- **94 deploy graphs** (80 deploy + 14 signal), 49 validation scenarios (10 tracks)
- **13/13 primals** BTSP Phase 3 AEAD, all defaulting to `127.0.0.1`
- **458 registered capability methods** (including 6 `neural_api.*` methods)
- **Zero DEBT markers**, zero unsafe blocks (`SeedConfig` + `OnceLock`)
- **6-tier discovery hierarchy** validated across all primals
- **Waves 1–46 complete** — Wave 46: typed errors (DispatchError, IonicProtocolError, PhasedIpcError → thiserror), env centralization, deprecated API removal, clippy sweep. Wave 45: all upstream Neural API blockers resolved, 12/12 primal.announce compliant. biomeOS v3.70 persistent routing weights (redb), weight health, utilization tracking, observatory posture, composition intelligence

## Key Validation Binaries

- `primalspring validate` — UniBin validation (49 scenarios across 10 tracks, 3 tiers)
- `primalspring certify` — Certification engine (L0-L8, BTSP, seed provenance, cellular)
- `primalspring serve` — RPC server (JSON-RPC 2.0 over UDS)

## Notebooks (5)

| # | Notebook | Focus |
|---|----------|-------|
| 01 | Composition Validation | Deploy graphs, bond types, profiles, discovery tiers |
| 02 | Benchmark Comparison | Rust vs Python timing, energy, guidestone phases |
| 03 | Ecosystem Evidence | 89 experiments, gap resolution, security timeline |
| 04 | Cross-Spring Connections | Primal consumption matrix, ecosystem flows, sporePrint readiness |
| 05 | BTSP Security Deep Dive | Per-primal posture, convergence arc, discovery hierarchy |

## Workload TOMLs

Not yet created — contribute to `projectNUCLEUS/workloads/primalspring/`.

## See Also

- [Spring Catalog](@/architecture/SPRING_CATALOG.md) on primals.eco
- [Lab Notebooks](https://primals.eco/lab/notebooks/) for rendered notebook views
- [baseCamp Papers 23, 26](https://primals.eco/science/)
