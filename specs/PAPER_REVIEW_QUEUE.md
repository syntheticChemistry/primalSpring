# primalSpring — Paper Review Queue

**Date**: March 23, 2026  
**Status**: Phase 12 — 51 experiments, 9 tracks, MULTI-NODE BONDING + FEDERATION (87/87 gates), 280+ tests, 22 deploy graphs

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
| P2 | RootPulse commit | 3 | **Neural API wired** | Provenance Trio + Track 2 |
| P2 | RootPulse branch/merge | 3 | **Neural API wired** | RootPulse commit |
| P2 | RPGPT session | 3 | Discovery wired | Continuous graph + RootPulse |
| P2 | coralForge pipeline | 3 | Discovery wired | Pipeline graph + multi-spring |
| P1 | BondType full taxonomy | 4 | **Validated** (exp030/032/033) | 5 bond types: Covalent, Metallic, Ionic, Weak, OrganoMetalSalt |
| P1 | BondingPolicy + constraints | 4 | **Validated** (exp071) | Capability masks, bandwidth, time windows, concurrency limits |
| P1 | Multi-node deploy graphs | 8 | **Structural** (4 graphs) | basement_hpc, friend_remote, idle_compute, data_federation |
| P1 | Graph bonding metadata | 8 | **Validated** (graph_metadata.rs) | [graph.metadata] + [graph.bonding_policy] parsing + consistency |
| P1 | STUN tier sovereignty-first | 8 | **Validated** (stun_tiers.rs) | 4-tier escalation: Lineage → Self-hosted → Public → Rendezvous |
| P2 | Covalent bonding (live) | 4 | Structural validated | Full NUCLEUS x2 on LAN, BirdSong mesh |
| P2 | Plasmodium formation (live) | 4 | Structural validated | Covalent bonds → capability aggregation |
| P2 | Idle compute federation | 8 | **Structural** (exp071) | BondingPolicy enforcement with time windows |
| P2 | Data federation cross-site | 8 | **Structural** (exp072) | NestGate replication + trio provenance, 7-phase pipeline |
| P2 | Friend remote NAT traversal | 8 | Structural only | STUN tiers, hole-punch, relay fallback |
| P2 | Basement HPC covalent mesh | 8 | Structural only | Multi-machine NUCLEUS with genetic lineage trust |
| P2 | Metallic fleet specialization | 4 | Structural only | Electron-sea model for homogeneous racks |
| P2 | Ionic contract-based bonds | 4 | Discovery wired | Cloud burst GPU, external API metering |
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
| P3 | BTC/ETH provenance anchoring | 8 | Future | sweetGrass anchoring.anchor → chain hash attestation |
| P3 | Novel Ferment Transcript (NFT) | 8 | Future | LoamSpine cert + DAG + braid + BearDog sig + anchor |
| P3 | sunCloud economic attribution | 8 | Future | sweetGrass braid radiating attribution |
| P3 | BYOB primal DAG execution | 8 | Future | Primals as DAG nodes for custom niche compositions |

## Next Steps (Post Phase 12)

1. **Emergent systems end-to-end** — Track 3: RootPulse commit/branch/merge/diff/federate with live trio; coralForge pipeline (exp013), continuous tick (exp014) — `ipc::provenance` wired, promoting to Live Validated when biomeOS + trio run
2. **Live multi-node validation** — Track 8: deploy NUCLEUS on 2+ machines (basement HPC, friend remote), validate BondingPolicy enforcement, NAT traversal, plasmodium formation, data federation with trio provenance
3. **Bonding live coordination** — Track 4: multi-gate covalent mesh, ionic contract bonds, metallic fleet, weak force isolation, OrganoMetalSalt mixed bonds
4. **Pipeline + Continuous graph execution** — exp013/014 (Track 2, remaining 2/5 patterns, need sweetGrass/trio running)
5. **Cross-spring integration** — Track 6: full ecosystem data flow, wetSpring genetic data lineage via trio
6. **Anchoring + Economics** — sweetGrass anchoring to BTC/ETH, Novel Ferment Transcripts, sunCloud attribution
7. **BYOB primal DAG execution** — primals as complexity-focused DAG nodes for custom niche compositions
8. **Cross-arch deployment** — aarch64 musl binaries + genomeBin packaging
