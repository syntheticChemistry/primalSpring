# primalSpring — Paper Review Queue

**Date**: March 22, 2026  
**Status**: Phase 10 — 49 experiments, 8 tracks, GRAPH EXECUTION + PROVENANCE READINESS (87/87 gates), 253+ tests

---

## Overview

primalSpring's "papers" are not published scientific papers — they are the
coordination patterns and emergent systems that the ecoPrimals ecosystem
produces. The review queue tracks which patterns are ready for validation.

## Queue

| Priority | Pattern | Track | Status | Dependencies |
|----------|---------|-------|--------|-------------|
| P0 | Tower Atomic (security + discovery) | 1 | **STABLE** (24/24 gates) | Live: beardog + songbird + biomeOS |
| P0 | Tower + Squirrel AI | 8 | **Validated** | Live: Tower + squirrel + Anthropic Claude |
| P0 | Nest Atomic (Tower + storage) | 1 | **VALIDATED** (8/8 gates) | nestgate storage, ZFS fallback, model cache |
| P0 | Node Atomic (Tower + compute) | 1 | **VALIDATED** (5/5 gates) | toadstool compute, dual-protocol, 4 workloads |
| P0 | NUCLEUS Composition (Tower+Nest+Node) | 1 | **VALIDATED** (58/58 total) | All 3 layers compose together |
| P0 | Graph-Driven Overlay Composition | 8 | **VALIDATED** (72/72 total) | Tower+AI, Nest+Viz, Node+AI overlays, graph merge |
| P1 | Sequential graph execution | 2 | Discovery wired | Full NUCLEUS |
| P1 | Parallel graph execution | 2 | Discovery wired | Full NUCLEUS |
| P1 | ConditionalDag execution | 2 | Discovery wired | Full NUCLEUS |
| P1 | Pipeline streaming | 2 | Discovery wired | Full NUCLEUS |
| P1 | Continuous 60Hz tick | 2 | Discovery wired | Full NUCLEUS |
| P1 | PathwayLearner optimization | 2 | Discovery wired | Full NUCLEUS |
| P2 | RootPulse commit | 3 | Discovery wired | Provenance Trio + Track 2 |
| P2 | RootPulse branch/merge | 3 | Discovery wired | RootPulse commit |
| P2 | RPGPT session | 3 | Discovery wired | Continuous graph + RootPulse |
| P2 | coralForge pipeline | 3 | Discovery wired | Pipeline graph + multi-spring |
| P2 | Covalent bonding | 4 | Discovery wired | Full NUCLEUS x2 |
| P2 | Plasmodium formation | 4 | Discovery wired | Covalent bonds validated |
| P3 | Cross-spring data flow | 6 | Discovery wired | Multiple springs deployed |
| P3 | fieldMouse ingestion | 6 | Discovery wired | NestGate + sweetGrass |
| P3 | petalTongue visualization | 6 | Discovery wired | biomeOS SSE + petalTongue |
| P3 | Squirrel AI coordination | 6 | Discovery wired | Squirrel + biomeOS |
| P3 | Compute triangle | 7 | Discovery wired | coralReef + toadStool + barraCuda |
| P3 | Socket discovery sweep | 7 | Discovery wired | XDG_RUNTIME_DIR convention |
| P3 | Protocol escalation | 7 | Discovery wired | HTTP + JSON-RPC + tarpc |
| P3 | Multi-primal lifecycle | 7 | Discovery wired | 6 primals |
| P3 | Bearer token auth | 7 | Discovery wired | BearDog |
| P3 | Wait-for-health | 7 | Discovery wired | Health probe pattern |
| P3 | Cross-tower federation | 7 | Discovery wired | BYOB manifest |
| P3 | Supply chain provenance | 7 | Discovery wired | 7-stage DAG |
| P3 | Semantic attribution | 7 | Discovery wired | sweetGrass |
| P3 | Weak force isolation | 7 | Discovery wired | Unknown primals |

## Next Steps (Post Phase 10)

1. **Provenance trio live** — sweetGrass, loamSpine, rhizoCrypt integration (awaiting provenance-trio-types crate)
2. **Pipeline + Continuous graph execution** — exp013/014 (Track 2, remaining 2/5 patterns)
3. **Emergent systems end-to-end** — Track 3 (RootPulse, coralForge) with real provenance trio
4. **Cross-arch deployment** — aarch64 musl binaries + genomeBin packaging (see hardware validation handoff)
5. **Bonding + cross-spring + showcase** — Track 4/6/7 patterns with full NUCLEUS compositions
