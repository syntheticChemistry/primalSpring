# wateringHole — primalSpring Ecosystem Guidance

**Version**: 0.9.25 (Wave 17 — Neural API Signal Elevation, eukaryotic validation, atomic signals)
**Last Updated**: May 16, 2026
**License**: AGPL-3.0-or-later  

---

## What This Is

The wateringHole is primalSpring's outward-facing guidance surface for upstream
primal teams and downstream spring/garden consumers. It defines the patterns
that make the ecosystem composable.

Historical handoffs live in [fossilRecord](https://github.com/ecoPrimals/fossilRecord) (consolidated May 12, 2026).

---

## Documents

### Living Standards

| File | Audience | What It Covers |
|------|----------|----------------|
| **CRYPTO_CONSUMPTION_HIERARCHY.md** | Primal teams + spring teams | Crypto posture per primal role: key acquisition patterns, bonding hierarchy, Phase 3 convergence. |
| **PLASMINBIN_DEPOT_PATTERN.md** | All consumers | How to fetch primal binaries from plasmidBin GitHub Releases. |
| **METHOD_GATE_STANDARD.md** | All primal teams | JH-0 ecosystem standard: pre-dispatch capability authorization, exempt whitelist, error codes, enforcement modes. |
| **PRIMAL_ANNOUNCE_PROTOCOL.md** | All primal teams | `primal.announce` atomic self-registration: wire format, field reference, registration order, signal-tier membership, backward compatibility. Replaces separate `lifecycle.register` + `capability.register` + `method.register` calls (biomeOS v3.57+). |
| **SIGNAL_ADOPTION_STANDARD.md** | All spring teams | Neural API composition collapse migration guide: `ctx.dispatch()` and `ctx.announce()` APIs, signal inventory (14 signals), spring archetype examples (compute/provenance/content-heavy), fallback behavior, validation coverage. |

### Living Handoffs

| File | Audience | What It Covers |
|------|----------|----------------|
| **INTERSTADIAL_FOSSILIZATION_HANDOFF.md** | Spring teams | Interstadial fossilization patterns: what to preserve, how to date, provenance READMEs. |
| **handoffs/PRIMALSPRING_SOVEREIGNTY_LAYER4_EVOLUTION_MAY15_2026.md** | All teams | Sovereignty track (3 scenarios), membrane deploy graph, routing config schema, 4-layer model. |
| **handoffs/UPSTREAM_PATTERN_ESCALATION_MAY15_2026.md** | Primal teams + springs | Downstream-evolved patterns needing upstream adoption: primal blockers (UB-1..4), canonicalization targets, spring actions, glacial horizon. |

### Archived Handoffs (`handoffs/archive/`)

| File | Date | Summary |
|------|------|---------|
| `PRIMALSPRING_V0925_EVOLUTION_HANDOFF_MAY09_2026.md` | May 9 | Primal consumption, upstream debt, per-spring targets |
| `PRIMALSPRING_V0925_UNIBIN_EUKARYOTIC_HANDOFF_MAY09_2026.md` | May 9 | UniBin cell model, CLI surface, two-tier validation |
| `PHASE60_COMPLETION_HANDOFF_MAY09_2026.md` | May 9 | Phase 60 completion, 13/13 primals clean |

---

## Current Ecosystem State

- **13/13 primals** building standalone, distributed via plasmidBin genomeBin
  (Tier 1: x86_64, aarch64, armv7 — 40+ release assets)
- **Zero open upstream gaps** — 13/13 primals at zero debt, Waves 1-18 complete, zero panics in production
- **451 registered capability methods** across 84+ domains (including `auth.*`, `nautilus.*`, `game.*`, ionic token methods, `btsp.capabilities`, `toadstool.validate`, `barracuda.precision.route`, `shader.compile.gemm`, `fido2.*`, `primal.announce`)
- **41 validation scenarios** (10 tracks, 3 tiers: Rust/Live/Both) with shared `validation::helpers`; sovereignty track validates membrane composition, routing parity, content sovereignty; signal dispatch parity + primal announce scenarios validate Neural API adoption
- **14 atomic signal graphs** (`graphs/signals/`) defining Neural API composition collapse layer
- **13/13 BTSP Phase 3 FULL AEAD**, 13/13 default `127.0.0.1`
- **RootPulse commit workflow** fully executable (6/6 phases)
- **NestGate content-addressed storage** live (8 `content.*` methods)
- **Graph method validator** — 0 primal drift, 91 spring-domain advisory
- **sourDough v0.2.0** scaffold generates ecosystem-compliant primals

## Key References (outside wateringHole)

| What | Where |
|------|-------|
| Gap registry | `docs/PRIMAL_GAPS.md` |
| Capability registry | `config/capability_registry.toml` (451 methods, zero drift) |
| Routing config schema | `config/routing_config_reference.toml` (canonical membrane routing) |
| Membrane deploy graph | `graphs/membrane/tower_membrane.toml` (VPS sovereignty boundary) |
| Method gate CI | `tools/check_method_gate.sh` |
| Method string validator | `tools/check_method_strings.sh` |
| Graph method validator | `tools/check_graph_methods.sh` |
| Experiment tracks | `experiments/` (89 experiments, 20 tracks) |
| Deploy graphs | `graphs/` (80 deploy TOMLs + 14 atomic signal graphs) |
| Signal tools | `config/signal_tools.toml` (14 atomic signals for Squirrel AI) |
| Checksum tool | `tools/regenerate_checksums.sh` |
| Binary fetch script | `tools/fetch_primals.sh` |
| NUCLEUS launcher | `tools/composition_nucleus.sh` |
| Composition library | `tools/nucleus_composition_lib.sh` |
| Fossil record | [fossilRecord repo](https://github.com/ecoPrimals/fossilRecord) (consolidated May 12, 2026) |

---

## Upstream Primal Debt and Evolution Status (May 15, 2026)

Post-Neural API evolution (biomeOS v3.55–v3.57, squirrel `signal_plan`,
`primal.announce` protocol). All primals are at `origin/main` HEAD — remote
is canonical and all pushed work is preserved. Stale merge artifacts on
eastGate have been cleaned (7 primals reset to `origin/main`).

### All 13 Primals (current HEAD)

| Primal | HEAD | Latest |
|--------|------|--------|
| biomeOS | `75209fc` | v3.57: Neural API evolution — announce protocol, metrics tagging, signal wiring |
| squirrel | `db3db3a` | Signal plan mode for `ai.query` — Neural API composition collapse |
| bearDog | `103982c` | Wave 102: ionic lease on `crypto.sign_contract` + `crypto.seed_fingerprint` |
| songbird | `237f7e2` | Wave 204: GAP-16 Tower Atomic — `mesh.*` on canonical UDS |
| toadStool | `cf7e212` | S263: CPUCTL_ALIAS breakthrough — FECS alive through warm handoff, Titan V dispatch |
| barraCuda | `10473ba` | Sprint 69: add `health.version` standalone RPC for trio consistency |
| coralReef | `d9d681c` | Sprint 12: synchronize all root docs, 3,181 tests |
| nestGate | `737660d` | Session 62: content provenance metadata (`artifact_query`) |
| skunkBat | `85ee1e0` | H2 niche evolution — live lineage, enforcement, NestGate protection |
| rhizoCrypt | `d52c527` | S68: enrich `dag.session.get` with agents/genesis/frontier |
| loamSpine | `606acbf` | GAP-36 provenance trio wire reconciliation — session aliases |
| sweetGrass | `925ed25` | v0.7.35: GAP-36 wire-name reconciliation + `lifecycle.status` |
| sourDough | `1b744b2` | v0.3.0: scaffold docs updated |

### Uncommitted Local Work (eastGate — review and push upstream)

These are real uncommitted changes on eastGate that need upstream team review:

| Primal | Files | What | Action |
|--------|-------|------|--------|
| nestGate | `run.rs`, `subcommands.rs` (+15 lines) | Adds `--socket` CLI flag for explicit socket path override, matching BearDog/ToadStool convention | Commit and push — useful feature |
| toadStool | `mappings_extended.rs` (-12/+9 lines) | Removes 8 false `inference.*`/`ollama.*` capability advertisements (S169). Inference is Squirrel's domain, not compute substrate | Commit and push — correct cleanup |
| bingoCube | `Cargo.toml` (2 lines) | Downgrades egui/eframe 0.29 -> 0.28 (compat fix) | Review — may be intentional pin |

### Evolution Targets (all primals)

With biomeOS v3.57 live, each primal should:

1. **Adopt `primal.announce`**: Replace separate `lifecycle.register` +
   `capability.register` + `method.register` startup calls with a single
   `primal.announce` RPC (see `PRIMAL_ANNOUNCE_PROTOCOL.md`).
2. **Declare signal-tier membership**: Include `signal_tiers` in the announce
   payload so biomeOS can route atomic signals through the correct graphs.
3. **Validate against 451 methods**: Ensure niche capability counts align
   with `config/capability_registry.toml`.
4. **Validate membrane compositions**: Downstream membrane deployments must conform
   to `config/routing_config_reference.toml` schema (backend types, trust tiers,
   telemetry). Use `graphs/membrane/tower_membrane.toml` as canonical VPS graph.
5. **Test with biomeOS v3.57**: Signal-tier interception in `capability.call`
   is now live — verify transparent composition collapse doesn't break
   existing call patterns.

### Downstream-Surfaced Primal Debt (May 15, 2026)

Patterns evolved by CATHEDRAL and projectNUCLEUS that require specific
primal team action. Full details: `handoffs/UPSTREAM_PATTERN_ESCALATION_MAY15_2026.md`.

| Primal | What | Priority | Downstream Requester |
|--------|------|----------|---------------------|
| **Songbird** | TURN client library — expose reusable crate for TURN-relayed JSON-RPC | HIGH | lithoSpore (geo-delocalized Tier 2) |
| ~~**BearDog**~~ | ~~FIDO2/CTAP2 protocol~~ — **SHIPPED**: Wave 103 `fido2.rs` (487 lines), IPC surface live | ~~MEDIUM~~ | lithoSpore (hardware-attested provenance) |
| **biomeOS** | Handle `composition_model = "membrane"` in `composition.deploy(graph)` | MEDIUM | projectNUCLEUS (VPS membrane deployment) |
| ~~**sporePrint**~~ | ~~sporePrint pipeline wiring~~ — **RESOLVED**: wired by primalSpring in `auto-refresh.yml` (not NestGate's ownership) | ~~MEDIUM~~ | lithoSpore / sporePrint |
| ~~**plasmidBin**~~ | ~~`genomeBin stage --target usb`~~ — **RESOLVED**: `stage_usb.sh` ships USB staging | ~~MEDIUM~~ | lithoSpore (Barrick Lab delivery) |

### Patterns to Absorb (from downstream evolution)

These are not blockers but patterns that downstream teams evolved and
validated that upstream primals and springs should study and adopt:

| Pattern | Evolved By | What It Means Upstream |
|---------|-----------|----------------------|
| Bash-to-Rust elevation | lithoSpore | 8/8 scripts replaced with pure Rust CLI. Primals/springs should evaluate which scripts benefit from elevation. |
| Discovery chain (env->UDS->TURN->standalone) | lithoSpore | Matches `CompositionContext::discover()` in consumer form. Env var names should be canonicalized. |
| Cross-platform deployment matrix | lithoSpore | musl-static + Windows cross-compile + read-only FS. Template for all ecoBin artifacts. |
| Module lib.rs in-process dispatch | lithoSpore | Mirrors UniBin absorbed-experiment pattern. Canonical for all Targeted GuideStones. |
| Content-aware routing schema compliance | projectNUCLEUS | `routing_config_reference.toml` schema owned by primalSpring. All membrane deployments validate against it. |
| Calibrate-shadow-cutover protocol | projectNUCLEUS | Sovereignty transitions with measurable gates. primalSpring validates structurally. |

### Infra Repos

| Repo | State | Action |
|------|-------|--------|
| infra/wateringHole | Clean | Consider syncing `PRIMAL_ANNOUNCE_PROTOCOL.md` |
| infra/whitePaper | 3 modified neuralAPI chapters (00, 01, 03) | Review and push |
| infra/benchScale | 1 modified spec + 1 untracked topology | Review and push |
| neuralSpring | 1 untracked `inference.rs` | Add or gitignore |

---

## River Delta (Springs) — Evolution Summary (May 15, 2026)

All 8 springs pulled to HEAD. Combined: **10,218 `#[test]` markers** across the
delta. Every spring has completed deep debt sweeps and is at zero debt.

| Spring | Tests | HEAD | Recent Evolution |
|--------|------:|------|------------------|
| wetSpring | 2,064 | V168 | Live NUCLEUS guideStone 30/31, barraCuda v0.4.0 absorbed, coralReef niche |
| neuralSpring | 1,556 | S206 | Compute trio wave, skunkBat triple-first Tower, inference pipeline |
| airSpring | 1,468 | — | Tower triple-first, atomic deployment handoff, AG-005 inference resolved |
| groundSpring | 1,286 | V142 | Compute trio wave, shader.compile.gemm, 3 upstream gaps |
| hotSpring | 1,148 | — | Sovereign dispatch validated on Titan V, CPUCTL_ALIAS breakthrough, Blackwell gaps |
| healthSpring | 1,019 | V64n | Tower atomic, deploy graph canonicalization, barraCuda v0.4.0 |
| ludoSpring | 910 | V72 | health.version + health.drain, 418-method registry alignment |
| primalSpring | 718+ | — | Wave 20: schema-standard + nest-commit-live scenarios, primal.list canonical schema, Thread 10 provenance wiring, LTEE tracker |

**Convergence state**: All springs CI-validated against canonical 452 methods. 43 scenarios across 10 tracks. Wave 20: schema standardization (primal.list + capability.list), nest.commit E2E validation, primal-blocked asks documented, LTEE paper queue tracked.
All implement BYOB niche model, deploy graphs, and Tier 1/2 validation.
Fragment-first graph composition adopted ecosystem-wide.

---

## Downstream Products (Gardens) — Evolution Summary (May 16, 2026)

### projectNUCLEUS V3 — Sovereignty + Validation Evolution

projectNUCLEUS has driven massive sovereignty infrastructure and validation evolution:

- **Forgejo PRIMARY**: 32 repos, 3 orgs, dual-push mirror to GitHub
- **VPS Tower LIVE**: DigitalOcean 2GB, Songbird TURN :3478, RustDesk,
  BearDog, SkunkBat, Caddy — hardened membrane posture
- **55 Rust tests** (darkforest 34, tunnelKeeper 21) — zero before V3
- **Discovery cascade**: biomeOS `primal.list` → per-primal env vars → compiled
  defaults → `health.liveness` + `capability.list` per primal
- **7 gate TOMLs** with `[science]` dispatch metadata: irongate, biomegate,
  strandgate, northgate, westgate, flockgate, nuc-intake
- **signal_executor.sh**: Squirrel `ai.query` signal_plan → biomeOS `signal.dispatch`;
  --plan-only, --dry-run, --shadow (tower_agent graph)
- **tower_agent.toml**: 5-node agentic graph (BearDog → Songbird + skunkBat →
  biomeOS neural-api → Squirrel) with tcp_fallback_port
- **4 new specs**: VALIDATION_PLAYBOOK (7 artifacts, cross-artifact ordering),
  FUZZ_EVOLUTION (multi-gate, corpus sharing), SCIENCE_DISPATCH_MAP (toadStool
  routing), TIER2_CEREMONY_DESIGN (BearDog RPC sequences)
- **FAMILY_HPC_MODEL**: owner-priority dispatch, multi-household WAN pattern
- **Horizon 4**: transactions, ceremony, federation, membrane fuzz — 15 items
- 267 bash security baseline PASS, 33 Dark Forest PASS, 17 membrane PASS
- Graphs synchronized to primalSpring v3.0.0

**Still in progress**: BTSP JupyterHub cutover (dual-auth shadow active),
petalTongue extracellular wiring, sovereign DNS (knot-dns, H2-17–H2-20),
Forgejo Actions CI porting.

**Asks**: canonical `primal.list` schema, `capability.list` shape standardization,
barraCuda scipy parity, Songbird library crates for tunnelKeeper v0.3.

### lithoSpore v1.0.0 — Verification Chassis (own IDE focus team)

The CATHEDRAL team has split into dedicated workstreams (May 16, 2026).
lithoSpore is now its own IDE focus team building the verification chassis.
**v1.0.0 tagged** — first stable guidestone artifact.

- **7/7 modules PASS** at Tier 2 (75/75 checks), chaos tested, deployment-validated
- **Bash-to-Rust elevation COMPLETE**: all 8 shell scripts replaced with pure
  Rust CLI subcommands (`litho fetch/assemble/validate/verify/chaos-test/deploy-test`).
  Only `scripts/build-container.sh` remains as shell.
- **ScopeManifest** (`scope.toml`): declarative guidestone identity — springs, modules,
  foundation threads. Scope-driven validation replaces hardcoded module lists.
- **liveSpore.json** provenance journal: append-only Tier-stamped entries with
  DiscoveryPath, hostname BLAKE3, runtime metrics. Corruption-resilient (backup + continue).
- **Capability-first discovery**: env → UDS `ipc.resolve` → TURN → standalone; `DiscoveryPath`
  enum feeds provenance. Signal annotations (`signal = "nest.store"`) in registry.
- **CLI integration test harness**: tempfile roots, CARGO_BIN_EXE_*, corruption/drift
  fixtures, scope-driven + fallback validation tests.
- **sporePrint dispatch CI**: `notify-sporeprint.yml` fires `repository-dispatch` to
  `ecoPrimals/sporePrint` on push to main.
- **THREAD_INDEX.toml**: 6 entries (threads 1, 2, 4, 5, 6, 7) linking modules to
  foundation references.
- **Cross-platform validation**: musl-static Linux (5.1 MB), Windows cross-compiled
  via `x86_64-pc-windows-gnu` (7.9 MB litho.exe tested via Wine 11).
- **USB recreation**: `litho assemble` builds portable USB artifacts.
- **Module interface**: `fn run_validation(data_dir, expected, max_tier) -> ModuleResult`
  for in-process dispatch. Single `litho` binary replaces 7 separate binaries.
- Ingested primalSpring patterns: capability registry, Dark Forest graphs, `graph_checks` module
- Owns: benchScale, agentReagents
- Needs from upstream: TURN relay wiring (UB-1 SHIPPED, integration pending), neuralSpring ML surrogates
- **Absorbable patterns**: ScopeManifest, graph↔registry cross-tests, DiscoveryPath
  telemetry, CLI integration harness, #[path = "tests.rs"] extraction

### projectFOUNDATION — Knowledge Layer (own IDE focus team)

projectFOUNDATION is now its own IDE focus team building the knowledge layer.
Springs feed validation results to projectFOUNDATION as thread evidence.

- **184 targets** across 10 threads, **146 validated (79.3%)**, 38 remaining
- **29 workloads** across all 10 threads with Standard isolation + provenance trio
- **Per-spring validation folders** with PROVENANCE_FOLDER_CONVENTION.md:
  `validation/<spring>/<YYYY-MM-DD>/` with results.json + provenance.toml + braid.json
- **primal_ipc.sh**: capability-first discovery for bash (env → UDS → defaults)
- **target_compare.sh**: dual-schema tolerance comparison (expected_value vs expected)
- **6 barraCuda CPU parity benchmarks**: matmul, linalg_solve, stats_mean, stats_variance,
  md_velocity_verlet, spectral_eigenvalues — all in CI
- **CI gates**: shellcheck, TOML schema, target count reconciliation, graph structural
  validation, workload integrity, gate naming, benchmark runs
- **ag-guidestone proposal**: Thread 6 agricultural guidestone scope (airSpring + groundSpring)
- **FOUNDATION_VALIDATE elevation review**: targeting CompositionContext + Rust crates
  for health, RPC, fetch, BLAKE3, workloads, provenance — bash stays interim
- **Thread 10 workload** targets `primalspring_unibin validate` directly
- Thread 2 Plasma **12/12 PASS**, Thread 6 Agricultural **36/36 PASS**,
  Thread 7 Anderson **18/18 PASS**
- Thread 1 WCM: fetch infra ok, **RPC upstream-blocked** (0/24 pending review)
- Data integrity: many `blake3 = ""` in source TOMLs — needs fetch + backfill
- Needs from upstream: RPC response schema standardization, CompositionContext
  for elevation, neuralSpring ML sources (Thread 5)

**Spring → projectFOUNDATION contract**: Check which threads reference your
spring in `projectFOUNDATION/lineage/THREAD_INDEX.toml` and `data/sources/*.toml`.
Ensure validation results are captured in `projectFOUNDATION/validation/` with
dated provenance folders. Thread 10 workload already targets primalspring_unibin.

### esotericWebb V8 — UI + Agentic Composition

At V8 — **357 tests**, **24 capabilities**, 22 bridge methods. Signal-first
provenance via `nest_store`/`nest_commit` bridge methods (Neural API preferred,
direct fallback). Startup `primal.announce` to biomeOS. Full lifecycle
handlers: `health.version`, `health.drain`, `primal.announce`, `primal.info`.

- **Signal adoption**: enrichment collapsed from multi-RPC to bridge `nest_store`/
  `nest_commit`; constants for `meta.observe`/`meta.intent` ready
- **Test extraction**: `#[path = "tests.rs"]` companion modules keep production
  files under 800 LOC cap without losing coverage (content: 23, session: 32 tests)
- **Capability↔registry cross-test**: `capabilities_match_registry_toml` enforces
  parity between `niche::CAPABILITIES` array and `capability_registry.toml`
- **Consumer reference**: strongest garden-level implementation of Wave 17 signal
  patterns — minimal bridge API as template for other gardens
- CRPG substrate with narrative DAG, YAML content model, 60Hz loop
- Strongest test case for the agentic composition pattern. Local to ironGate
  with direct UDS access to full NUCLEUS.
- **GAP-024 (open)**: signal paths not exercised live on biomeOS / ironGate
- Needs: biomeOS E2E signal validation, ludoSpring 6 game.* methods,
  Squirrel mechanical context, petalTongue DialogueTree

See `handoffs/GARDEN_EVOLUTION_BLURB_MAY16_2026.md` for full evolution guidance.

### blueFish

Remote repo not found (`404`). Either renamed, private, or not yet created.

---

## Fossil Record

Historical handoffs are preserved in the [fossilRecord repository](https://github.com/ecoPrimals/fossilRecord) under `springs/primalSpring/`:
- `wateringHole_phase56_apr2026/` — v0.1.0 through Phase 56 (66+ files)
- `wateringHole_phase58_59_may2026/` — Phase 58–59 handoffs (6 files)

Git history in this repo retains full provenance at their original paths.
A local redirect stub exists at `fossilRecord/README.md`.
