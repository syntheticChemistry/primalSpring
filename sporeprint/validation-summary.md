+++
title = "primalSpring Validation Summary"
description = "Composition validation (meta-spring) — 303 tests, 51 experiments, deploy graph wiring, cross-gate bonding"
date = 2026-05-06

[taxonomies]
primals = ["biomeos", "barracuda", "toadstool", "nestgate", "beardog", "songbird"]
springs = ["primalspring"]
+++

## Status

- **303 tests** across 51 experiments
- **22 deploy graphs** validated (18 single-node + 4 multi-node)
- All 5 biomeOS coordination patterns validated (Sequential, Parallel, ConditionalDag, Pipeline, Continuous)
- All nodes addressed by capability, topologically sorted
- Phase 59: 85 experiments, 661 tests, 74 deploy graphs

## Key Validation Binaries

<!-- TODO: Update with actual binary names from target/release/ -->
- `primalspring_guidestone` — structural deploy graph validation
- `validate_composition` — cross-gate bonding verification
- `validate_discovery` — 5-tier discovery hierarchy
- `validate_btsp` — AEAD encryption pipeline

## Workload TOMLs

Not yet created — contribute to `projectNUCLEUS/workloads/primalspring/`.

## See Also

- [Spring Catalog](https://primals.eco/architecture/spring-catalog-status-science-and-evolution/) on primals.eco
- [baseCamp Papers 23, 26](https://primals.eco/science/)
