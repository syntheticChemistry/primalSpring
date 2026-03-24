# primalSpring baseCamp — Coordination and Composition Validation

**Date**: March 24, 2026
**Status**: Phase 14 — DEEP DEBT + BUILDER PATTERN + FULL PROVENANCE (87/87 gates), 53 experiments, 361 tests, 22 deploy graphs

---

## What This Is

Where baseCamp papers for other springs explore scientific questions using the
ecoPrimals infrastructure, primalSpring's baseCamp explores **the infrastructure
itself**. The "papers" are the atomics. The "experiments" are composition patterns.
The validation target is biomeOS and the Neural API.

## The Paper

See `ecoPrimals/whitePaper/gen3/baseCamp/README.md` (Paper 23 section) for
the full baseCamp paper documenting primalSpring's validation of ecosystem coordination.

## Experiments by Track

| Track | Domain | Experiments | Key Question |
|-------|--------|-------------|--------------|
| 1 | Atomic Composition | exp001–006 | Do atomics deploy correctly? |
| 2 | Graph Execution | exp010–015 | Do all 5 coordination patterns work? (3/5 live) |
| 3 | Emergent Systems | exp020–025 | Do Layer 3 systems emerge correctly? |
| 4 | Bonding & Plasmodium | exp030–034 | Does multi-gate coordination work? |
| 5 | coralForge | (exp025) | Does the neural object pipeline work? |
| 6 | Cross-Spring | exp040–044 | Do cross-spring data flows work? |
| 7 | Showcase-Mined | exp050–059 | Do mined phase1/phase2 coordination patterns hold? |
| 8 | Live Composition | exp060–070 | Tower + Squirrel AI + Nest + Node + NUCLEUS + Graph Overlays + Cross-Primal Discovery |
| 9 | Multi-Node Bonding | exp071–072 | Do bonding policies and data federation structures validate? |
| 10 | Cross-Gate Deployment | exp073–074 | Does cross-gate health probing and LAN covalent mesh work? |

## Current State (v0.7.0)

| Metric | Value |
|--------|-------|
| Experiments | 53 (10 tracks) |
| Total tests | **361** (unit + integration + doc-tests + proptest, 42 ignored live) |
| Proptest fuzz tests | 22 (IPC protocol, extract, capability parsing, cross-cutting pipeline) |
| clippy (pedantic+nursery) | 0 warnings |
| cargo doc | 0 warnings |
| `#[allow()]` in production | 0 |
| unsafe_code | Workspace-level `forbid` |
| C dependencies | 0 (pure Rust, ecoBin compliant, `deny.toml` enforced) |
| Deploy graphs | 22 TOMLs (18 single-node + 4 multi-node), all nodes `by_capability`, topologically validated |
| Discovery | Capability-first: 5-tier + Neural API + `discover_by_capability()` |
| RPC endpoints | 17 methods (including `graph.waves`, `graph.capabilities`) |
| Niche self-knowledge | `niche.rs` — 37 capabilities, semantic mappings, cost estimates |
| MCP tools | 8 typed tools via `mcp.tools.list` for Squirrel AI |
| Validation harness | Builder `.run()`, `check_bool`, `check_skip`, `check_or_skip`, `check_relative`, `check_abs_or_rel`, `with_provenance()`, `NdjsonSink` |
| Provenance coverage | **100%** — all 53 experiments carry `with_provenance()` metadata |
| Dishonest scaffolding | 0 (all experiments use honest skip or real validation) |
| Tower Atomic | **FULLY UTILIZED** — 41/41 gates (24 core + 17 full utilization) |
| Nest Atomic | **VALIDATED** — 8/8 gates (nestgate storage, model cache) |
| Node Atomic | **VALIDATED** — 5/5 gates (toadstool compute, dual-protocol) |
| NUCLEUS | **VALIDATED** — 58/58 base gates (Tower + Nest + Node) |
| Graph Overlays | **VALIDATED** — 14/14 gates (tier-independent primals via deploy graphs) |
| Squirrel Discovery | **VALIDATED** — 5/5 gates (cross-primal env_sockets, capability.discover) |
| Graph Execution | **LIVE** — 6/6 gates (3/5 coordination patterns live) |
| Provenance Readiness | **STRUCTURAL** — 4/4 gates (launch profiles + deploy graph) |
| Total Gates | **87/87** |
| Squirrel AI | Composition validated (Tower + Squirrel + Anthropic Claude) |
| petalTongue | v1.6.6 integrated, visualization.render.dashboard + grammar |

## What Changed — Phase 14 (Deep Debt + Builder Pattern + Full Provenance)

### Builder-Pattern Validation (March 24, 2026)
All 53 experiments standardized on the builder-pattern `ValidationResult`:
1. **`ValidationResult::run()`** — consumes self, prints banner, executes checks, prints summary, exits
2. **All 53 experiments carry structured provenance** — `with_provenance(source, date)` on every experiment
3. **`validation/tests.rs` extracted** — 493-line test module separated from production code (540 lines, was 1016)
4. **Zero `.unwrap()` in experiment binaries** — all replaced with `.or_exit("context")`
5. **Zero `#[allow()]` in production** — 3 integration test files evolved to `#[expect(reason)]`
6. **Doc and config fixes** — broken intra-doc link, stale socket path doc, capability_registry version sync
7. **361 tests** (up from 360), 0 clippy warnings, 0 doc warnings, 0 fmt diff

## What Changed — Phase 11–12 (Multi-Node Bonding + Federation)

### Provenance Trio Neural API (Phase 11)
1. **`ipc::provenance` module** — full RootPulse pipeline via `capability.call` (zero compile coupling)
2. **4 experiments evolved** — exp020 (6-phase commit), exp021 (branch/merge), exp022 (diff/federate), exp041 (E2E chain)
3. **Live probing** — sweetGrass LIVE, rhizoCrypt LIVE (TCP), loamSpine BROKEN (runtime panic)
4. **4 gaps documented** — wire format mismatches, param schema drift

### Multi-Node Bonding + Federation (Phase 12)
5. **BondType expanded** — Covalent, Metallic, Ionic, Weak, OrganoMetalSalt (5 variants)
6. **TrustModel** — GeneticLineage, Contractual, Organizational, ZeroTrust
7. **BondingConstraint + BondingPolicy** — capability masks, bandwidth limits, time windows, concurrency
8. **4 multi-node deploy graphs** — basement HPC, friend remote, idle compute, data federation
9. **`graph_metadata.rs`** — parse + validate `[graph.metadata]` and `[graph.bonding_policy]` from TOML
10. **`stun_tiers.rs`** — 4-tier STUN config parser, sovereignty-first escalation validation
11. **exp071 + exp072** — idle compute policy and data federation validation
12. **303 tests**, 51 experiments, 22 deploy graphs (at time of Phase 12)

### Ecosystem Absorption Wave (Phase 12.1, March 23, 2026)
Absorbed patterns from all 7 sibling springs into primalSpring core:
13. **`deny.toml` ban list** — merged groundSpring V120 + wetSpring V132 C-dep bans (aws-lc-sys, cmake, cc, pkg-config, vcpkg)
14. **Cast discipline lints** — neuralSpring S170 / airSpring V010 clippy cast_* lints workspace-wide
15. **`ValidationSink` enrichment** — `section()` + `write_summary()` from groundSpring V120
16. **`exit_code_skip_aware()`** — 3-way exit from wetSpring V132 (0=pass, 1=fail, 2=all-skipped)
17. **`proptest_ipc` module** — 7 cross-cutting property tests fuzzing the IPC pipeline (healthSpring V41)
18. **`primal_names` module** — canonical display↔slug mapping for 23 primals/springs (neuralSpring pattern)
19. **Provenance trio circuit breaker** — epoch-based breaker + exponential backoff in `ipc::provenance` (healthSpring V41)
20. **303 tests** (up from 280) — zero clippy warnings, zero TODO/FIXME in production

### Ecosystem Absorption Wave (Phase 12.2, March 23, 2026)
Absorbed deeper patterns from all 7 sibling springs into primalSpring core:
21. **`normalize_method()`** — ecosystem-wide JSON-RPC dispatch standard (groundSpring V121, neuralSpring V122, wetSpring V133, healthSpring V42)
22. **`check_relative()` + `check_abs_or_rel()`** — robust numeric validation for both relative and absolute tolerance (groundSpring V120, healthSpring V42)
23. **`NdjsonSink`** — streaming newline-delimited JSON validation output (groundSpring V121, wetSpring V133, neuralSpring V122)
24. **`IpcError::is_recoverable()`** — broader recovery classification beyond `is_retriable()` (neuralSpring V122, wetSpring V133)
25. **`Transport` enum (Unix + Tcp)** — cross-platform IPC layer (airSpring V010, healthSpring V42, groundSpring V121)
26. **`ipc::probes`** — `OnceLock`-cached runtime resource probes for test parallelism (hotSpring V0.6.32, neuralSpring V122)
27. **`validate_release.sh`** — release quality gate: fmt + clippy + deny + test floor (320) + docs
28. **`missing_docs` → `deny`** — all public items documented, lint level upgraded from warn
29. **Server `normalize_method()` dispatch** — prefix-agnostic routing for all ecosystem callers
30. **360 tests** (up from 303) — zero clippy warnings, zero missing docs

### Cross-Gate Deployment Tooling (Phase 13, March 23, 2026)
Built deployment pipeline for live multi-gate LAN covalent deployment:
31. **`build_ecosystem_musl.sh`** — build all 6 core primals as x86_64 + aarch64 musl static binaries
32. **`prepare_spore_payload.sh`** — assemble USB spore deployment payload (binaries + graphs + genetics)
33. **`validate_remote_gate.sh`** — probe remote gate NUCLEUS health via TCP JSON-RPC
34. **exp073_lan_covalent_mesh** — cross-gate Songbird mesh + BirdSong beacon exchange via TCP
35. **exp074_cross_gate_health** — remote per-primal TCP health + capabilities + composition assessment
36. **exp063 evolved** — cross-device Pixel beacon exchange via `PIXEL_SONGBIRD_HOST` + TCP
37. **`basement_hpc_covalent.toml`** — annotated with full gate inventory from HARDWARE.md
38. **LAN_COVALENT_DEPLOYMENT_GUIDE** — step-by-step handoff for all gate operators
39. **53 experiments** (up from 51), **10 tracks** (up from 9)

## What Changed (v0.6.0 -> v0.7.0)

### Graph-Driven Overlay Composition
1. **Tier-independent primals** — Squirrel, petalTongue, biomeOS compose at any atomic tier via deploy graphs
2. **`spawn` field on GraphNode** — distinguishes primal nodes from validation nodes
3. **5 new overlay deploy graphs** — tower_ai, tower_ai_viz, nest_viz, node_ai, full_overlay
4. **`merge_graphs()`** — merge base + overlay deploy graphs at runtime
5. **exp069** — end-to-end overlay validation (25/25 checks)

### Squirrel Cross-Primal Discovery
6. **9 env_sockets** — Squirrel discovers NestGate, ToadStool, Songbird, BearDog via explicit env vars
7. **full_overlay.toml** — Tower + Nest + Node + Squirrel (all capability domains)
8. **exp070** — cross-primal discovery validation
9. **4 new integration tests** — squirrel_discovers_sibling_primals, tool_list, context_create, ai_query

### Graph Execution Patterns (3/5 Live)
10. **exp010 Sequential** — live Tower composition with ordering verification
11. **exp011 Parallel** — live 4-primal burst (beardog+songbird+nestgate+toadstool)
12. **exp012 ConditionalDag** — live toadstool/CPU fallback branching
13. exp013/014 — awaiting provenance trio (sweetGrass, rhizoCrypt, loamSpine)

### Provenance Readiness
14. **Launch profiles** — sweetGrass, loamSpine, rhizoCrypt socket wiring
15. **provenance_overlay.toml** — Tower + RootPulse deploy graph
16. **Handoffs delivered** — provenance trio team + all teams

## What Changed (v0.5.0 -> v0.6.0)

### NUCLEUS Composition (v0.6.0)
1. **Nest Atomic** — nestgate storage primal integrated
2. **Node Atomic** — toadstool compute primal integrated
3. **NUCLEUS Composition** — all 3 atomic layers compose together (58/58 gates)
4. **3 new experiments** — exp066, exp067, exp068
5. **12 new integration tests** — 8 Nest + 4 Node

## Co-Evolution Strategy

| Phase | Focus | Partners | Gate Target | Status |
|---|---|---|---|---|
| Tower Stability | All 24 Tower gates | beardog, songbird, biomeOS | Gates 1–6 (24/24) | **DONE** |
| Tower + Squirrel | AI composition | + squirrel | AI gates | **DONE** |
| Tower Full Utilization | Subsystems + viz | + petalTongue | Gates 7–11 (41/41) | **DONE** |
| Nest Atomic | Storage gates | + nestgate | Gates 12–13 (8/8) | **DONE** |
| Node Atomic | Compute gates | + toadstool | Gates 14–15 (5/5) | **DONE** |
| NUCLEUS Composition | All layers compose | Tower + Nest + Node | Gate 16 (4/4) | **DONE** |
| Graph Overlays | Tier-independent primals | + squirrel, petalTongue | Gates 17–20 (14/14) | **DONE** |
| Squirrel Discovery | Cross-primal wiring | + all primals | Gate 21 (5/5) | **DONE** |
| Graph Execution | 3/5 patterns live | Tower + Nest + Node | Gate 22 (6/6) | **DONE** |
| Provenance Readiness | Structural prep | sweetGrass/loamSpine/rhizoCrypt | Gate 23 (4/4) | **DONE** |
| Provenance Trio Neural API | ipc::provenance wired | + sweetGrass/loamSpine/rhizoCrypt | Neural API gates | **DONE** |
| Multi-Node Bonding | BondType, BondingPolicy, STUN, federation graphs | + Songbird, NestGate | Bonding gates | **DONE** |
| Emergent E2E | RootPulse + coralForge live | + provenance trio running | Emergent gates | **NEXT** |

See `specs/TOWER_STABILITY.md` for the full 87-gate acceptance criteria.

## Hardware Validation (March 22, 2026)

primalSpring validated against physical hardware: Pixel 8a (aarch64), 3 USB
spores (biomeOS1, LiveSpore, ColdSpore), and SoloKey 2.

| Target | Result |
|--------|--------|
| aarch64-unknown-linux-musl cross-compile | primalspring 0.7.0 runs on Pixel |
| Pixel primal execution | beardog 0.9.0, songbird 3.33.0, squirrel 0.1.0, toadstool 0.1.0, nestgate 2.1.0 all execute |
| USB primal execution | 5/6 primals run (nestgate has corrupt static build on USB) |
| Host validate_all | 47/49 pass (exp060/061 blocked by external primal socket timeouts) |
| genomeBin packages | Zero .genome self-extractors exist; raw binaries only |
| plasmidBin aarch64 | Missing; only x86_64 in ecosystem plasmidBin |

**Blockers for full Pixel atomic deployment:**
- BearDog v0.9.0 abstract socket regression (Android SELinux blocks filesystem sockets)
- No aarch64 binaries in ecosystem plasmidBin (all x86_64)
- nestgate USB build corrupted (segfault, needs rebuild)
- No biomeOS orchestrator binary on Pixel

## What Remains

- **Emergent systems E2E** — RootPulse commit/branch/merge/diff/federate with live trio (ipc::provenance wired, awaiting biomeOS + trio running)
- **Live multi-node validation** — deploy NUCLEUS on 2+ machines, validate covalent mesh, BondingPolicy enforcement, NAT traversal, data federation
- **Pipeline + Continuous graph execution** (exp013/014) — awaiting sweetGrass/rhizoCrypt live
- **Bonding live coordination** — multi-gate covalent mesh, ionic contracts, metallic fleets
- **Cross-spring integration** — wetSpring genetic lineage via trio, BYOB primal DAG execution
- **Anchoring + Economics** — sweetGrass anchoring to BTC/ETH, Novel Ferment Transcripts, sunCloud
- Protocol escalation (JSON-RPC -> tarpc sidecar)
- biomeOS self-composition (biomeOS composes its own graphs at runtime)
- **ecoBin compliance**: rebuild all primals as static musl for both x86_64 and aarch64
- **genomeBin packaging**: run sourDough to produce actual .genome self-extractors
- **beardog Android socket**: fix abstract socket regression for Pixel deployment

---

**License**: AGPL-3.0-or-later
