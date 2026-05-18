# wateringHole — primalSpring Ecosystem Guidance

**Version**: 0.9.25 (Wave 22 — upstream primal evolution, wetSpring sovereign pipeline live with trio composition)
**Last Updated**: May 17, 2026 (PM — 456-method registry, wetSpring upstream asks ingested, primal hardening wave)
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
| **handoffs/WAVE20_LITHOSPORE_AUDIT_SPRING_EVOLUTION_MAY17_2026.md** | Delta spring teams | lithoSpore audit absorption: stability tiers, degradation behavior, cross-tier parity, trio semantics, per-spring guidance (wetSpring highest priority). |
| **handoffs/WAVE21_GARDEN_EVOLUTION_MAY17_2026.md** | Garden teams | Wave 21 patterns: projectNUCLEUS sovereignty + cellMembrane, projectFOUNDATION thread lineage, lithoSpore cross-tier parity. |
| **handoffs/WAVE22_UPSTREAM_PRIMAL_EVOLUTION_MAY17_2026.md** | All primal teams | Wave 22 upstream hardening: deployment-valid checklist, per-primal action items, manifest drift, stadial pairing preview. |
| **handoffs/WAVE22_STADIAL_GATE_PRIMAL_BLURB_MAY17_2026.md** | All primal teams | Stadial gate final debt sweep: universal checklist, low-debt group (7 primals), focused sections (6 primals with higher debt), composition gaps, wetSpring upstream asks with wire formats. |

### Ecosystem Standards (infra/wateringHole)

| File | Audience | What It Covers |
|------|----------|----------------|
| **REPO_MEMBRANE_BOUNDARY.md** | All teams | Inner/outer membrane repo classification: Forgejo-only, dual-push, GitHub-only. Contamination risk matrix, .env audit, push policy. |
| **SOVEREIGNTY_STANDARDS.md** | All teams | Calibrate → shadow → cutover protocol, bonding model, membrane channels, credential management, Forgejo as primary. |
| **MEMBRANE_CHANNEL_ARCHITECTURE.md** | projectNUCLEUS | Three membrane channels (Signal/Relay/Surface), deployment models, crypto layers, fieldMouse classification. |

### Archived Handoffs (`handoffs/archive/`)

| File | Date | Summary |
|------|------|---------|
| `WAVE20_DEBT_RESOLUTION_MAY17_2026.md` | May 17 | Per-spring debt fixes — all resolved, zero debt confirmed |
| `WAVE20_DELTA_SPRING_EVOLUTION_MAY16_2026.md` | May 16 | Wave 20 absorption checklist — superseded by lithoSpore audit |
| `GARDEN_EVOLUTION_BLURB_MAY16_2026.md` | May 16 | Garden evolution guidance — superseded by Wave 21 blurb |
| `PRIMAL_BLOCKED_ASKS_MAY16_2026.md` | May 16 | Upstream blockers — partially resolved by Wave 21 |
| `CATHEDRAL_SPLIT_SPRING_GUIDANCE_MAY16_2026.md` | May 16 | lithoSpore/projectFOUNDATION split guidance — absorbed |
| `DOWNSTREAM_INTERIM_WAVE18_MAY16_2026.md` | May 16 | Wave 18 interim downstream prep — superseded by Wave 20/21 |
| `PRIMALSPRING_SOVEREIGNTY_LAYER4_EVOLUTION_MAY15_2026.md` | May 15 | Sovereignty track (3 scenarios), membrane deploy graph, routing config schema |
| `UPSTREAM_PATTERN_ESCALATION_MAY15_2026.md` | May 15 | Downstream-evolved patterns needing upstream adoption |
| `PRIMALSPRING_V0925_EVOLUTION_HANDOFF_MAY09_2026.md` | May 9 | Primal consumption, upstream debt, per-spring targets |
| `PRIMALSPRING_V0925_UNIBIN_EUKARYOTIC_HANDOFF_MAY09_2026.md` | May 9 | UniBin cell model, CLI surface, two-tier validation |
| `PHASE60_COMPLETION_HANDOFF_MAY09_2026.md` | May 9 | Phase 60 completion, 13/13 primals clean |

---

## Current Ecosystem State

- **13/13 primals** building standalone, distributed via plasmidBin genomeBin
  (Tier 1: x86_64, aarch64, armv7 — 40+ release assets)
- **Zero open upstream gaps** — 13/13 primals at zero debt, Waves 1-18 complete, zero panics in production
- **456 registered capability methods** across 84+ domains (including `auth.*`, `nautilus.*`, `game.*`, ionic token methods, `btsp.capabilities`, `toadstool.validate`, `barracuda.precision.route`, `shader.compile.gemm`, `fido2.*`, `primal.announce`, `primal.list`)
- **43 validation scenarios** (10 tracks, 3 tiers: Rust 9 / Both 10 / Live 24) with shared `validation::helpers`; sovereignty track validates membrane composition, routing parity, content sovereignty; schema-standard + nest-commit-live scenarios validate Wave 20 canonical shapes; signal dispatch parity + primal announce scenarios validate Neural API adoption
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
| Capability registry | `config/capability_registry.toml` (456 methods, zero drift) |
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
3. **Validate against 456 methods**: Ensure niche capability counts align
   with `config/capability_registry.toml` (Wave 20: `primal.list` added).
4. **Validate membrane compositions**: Downstream membrane deployments must conform
   to `config/routing_config_reference.toml` schema (backend types, trust tiers,
   telemetry). Use `graphs/membrane/tower_membrane.toml` as canonical VPS graph.
5. **Test with biomeOS v3.57**: Signal-tier interception in `capability.call`
   is now live — verify transparent composition collapse doesn't break
   existing call patterns.

### Downstream-Surfaced Primal Debt (May 15, 2026)

Patterns evolved by CATHEDRAL and projectNUCLEUS that require specific
primal team action. Full details: `handoffs/archive/UPSTREAM_PATTERN_ESCALATION_MAY15_2026.md`.

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
| Cross-tier parity | lithoSpore | `litho parity` proves Python and Rust agree on all 7 modules. Pattern for any dual-language validation. |
| Tier 3 provenance via JSON-RPC | lithoSpore | `provenance.rs` wires `dag.*` → `spine.*` → `braid.*` as JSON-RPC client. Reference implementation for trio consumers. |
| Ferment transcript / upstream braid | lithoSpore | guideStones carry summary stats + braid reference — raw data stays upstream. Contract in `LITHOSPORE_FERMENT_TRANSCRIPT_BRAID_HANDOFF_MAY17_2026.md`. |
| Method stability tiers | primalSpring | `capability_registry.toml` now annotates `stability = "stable/evolving/internal"`. Downstream consumers depend on stable names. |
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

## River Delta (Springs) — Evolution Summary (May 17, 2026)

All 8 springs pulled to HEAD. Combined: **9,539+ workspace tests** across the
delta. Every spring has absorbed lithoSpore audit patterns and is at zero debt.

| Spring | Tests | HEAD | Post-lithoSpore Absorption Status |
|--------|------:|------|-----------------------------------|
| hotSpring | 596 | latest | Caught up: `CROSS_TIER_PARITY.md`, renamed degradation doc, 7 handoffs archived, 204 experiments, VBIOS interpreter live HW, 22 scenarios |
| healthSpring | 1,018 | V64x | Stability tiers (15/41/2), B5 cross-tier parity 8/8 bit-identical, degradation doc, 57 scenarios |
| wetSpring | 1,962+ | V177 | **Exp381 breseq pipeline live** (Barrick 2009, 3/7 clones done), first ferment braid exported, `primals_reached` trio output, 43 niche caps |
| neuralSpring | 739 | V168 | Stability tiers on 37 caps, degradation doc, 10 scenarios, GPU parity all 6 stages |
| ludoSpring | 982 | V76 | Schell Lenses + CPU/GPU parity, `CROSS_TIER_PARITY.md`, `DEGRADATION_BEHAVIOR.md`, 982 tests |
| groundSpring | 1,123+ | V145 | `DEGRADATION_BEHAVIOR.md` (11 primals), niche metadata fix, 1,123 tests |
| airSpring | 1,373 | v0.10.0 | 3 new cross-tier parity validators (autocorrelation, gamma_cdf, soil_moisture_topp), 57 caps (53 stable, 4 evolving), trio `primals_reached` bug fix, 17 methods full 3-tier coverage |
| primalSpring | 744 | — | Wave 20 PM: lithoSpore audit absorption (R1-R4), stability tiers, degradation docs, trio semantics, cross-tier parity pattern, scorecard + LTEE tracker updated |

**Convergence state (May 17, 2026 PM)**: All 7 delta springs have absorbed
lithoSpore audit patterns. Stability tiers annotated ecosystem-wide. Degradation
behavior documented in all springs. Cross-tier parity validated. Trio transaction
semantics aligned. wetSpring executing the ecosystem's first real-data ferment
transcript braid. Registry 456 methods. Fragment-first graph composition adopted.
**Next vector**: downstream garden evolution (projectFOUNDATION + projectNUCLEUS absorb
Wave 20 patterns), then upstream primal evolution.

---

## Downstream Products (Gardens) — Evolution Summary (May 17, 2026 PM, post-Wave 21 absorption)

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

**Asks (status)**: ~~canonical `primal.list` schema~~ **SHIPPED** (Wave 20),
~~`capability.list` shape standardization~~ **SHIPPED** (Wave 20),
barraCuda scipy parity (ongoing), Songbird library crates for tunnelKeeper v0.3.
**Wave 21 absorption**: Path reorganization (`sporeGarden` → `gardens`), local
path hardcoding eliminated. cellMembrane owns fieldMouse Tower deployment.
**Deployment convergence**: Consumes primals from `plasmidBin`; gate TOMLs
route science to hardware; `deploy_membrane.sh` is the operational tooling.

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
- **Wave 21 absorption**: `DEGRADATION_BEHAVIOR.md` (formalizes "science first" for
  all 3 tiers), `PARITY_REPORT_SCHEMA.md` (ecosystem standard for cross-tier parity
  JSON), `provenance/braids/` directory for ferment transcript ingestion, stability
  tiers on `capability_registry.toml`, partial trio semantics in `provenance.rs`
- **Absorbable patterns**: ScopeManifest, graph↔registry cross-tests, DiscoveryPath
  telemetry, CLI integration harness, #[path = "tests.rs"] extraction, ParityReport schema
- **Deployment convergence**: Tier 3 requires `plasmidBin` primals via `stage_usb.sh`;
  geo-delocalized Tier 2 routes through cellMembrane Songbird TURN

### projectFOUNDATION — Knowledge Layer (own IDE focus team)

projectFOUNDATION is now its own IDE focus team building the knowledge layer.
Springs feed validation results to projectFOUNDATION as thread evidence.
Repo folder: `gardens/projectFOUNDATION` (sporeGarden org).

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
- Data integrity: ~~many `blake3 = ""`~~ BLAKE3 backfill **documented** (`BLAKE3_BACKFILL_STATUS.md`),
  165 manifest rows across 11 TOMLs still empty — medium priority, not blocking validation
- Needs from upstream: ~~RPC response schema standardization~~ **SHIPPED** (Wave 20),
  CompositionContext for elevation, neuralSpring ML sources (Thread 5)
- **Wave 21 absorption**: `DEGRADATION_BEHAVIOR.md` (phase-level degradation for
  `foundation_validate.sh`), `validation/wetSpring/braids/` directory for ferment
  transcript ingestion, stability tier annotations on workload TOMLs, Thread 5
  targets updated for braid evidence, composition gaps marked resolved
- **Deployment convergence**: Runs validation against primals from `plasmidBin`;
  `foundation_validate.sh` discovers primals via capability-first IPC

**Spring → projectFOUNDATION contract**: Check which threads reference your
spring in `projectFOUNDATION/lineage/THREAD_INDEX.toml` and `data/sources/*.toml`.
Ensure validation results are captured in `projectFOUNDATION/validation/` with
dated provenance folders. Thread 10 workload already targets primalspring_unibin.

### esotericWebb V9 — UI + Agentic Composition (Wave 20-21 absorbed)

At V9 — **357+ tests**, **24 capabilities**, 22 bridge methods. Signal-first
provenance via `nest_store`/`nest_commit` bridge methods (Neural API preferred,
direct fallback). Startup `primal.announce` to biomeOS. Full lifecycle
handlers: `health.version`, `health.drain`, `primal.announce`, `primal.info`.

- **Wave 20-21 absorption**: Canonical `capability.list` envelope (`capabilities`,
  `count`, `primal`), stability tiers on `capability_registry.toml`,
  `DEGRADATION_BEHAVIOR.md` (gameplay never gated on primals), `primals_reached`
  on `WorldState`, `unwrap_capabilities_envelope` client helper. GAP-026–030 resolved.
- **Signal adoption**: enrichment collapsed from multi-RPC to bridge `nest_store`/
  `nest_commit`; constants for `meta.observe`/`meta.intent` ready
- **Test extraction**: `#[path = "tests.rs"]` companion modules keep production
  files under 800 LOC cap without losing coverage (content: 23, session: 32 tests)
- **Capability↔registry cross-test**: `capabilities_match_registry_toml` enforces
  parity between `niche::CAPABILITIES` array and `capability_registry.toml`
- **Consumer reference**: strongest garden-level implementation of signal patterns
  — minimal bridge API as template for other gardens
- CRPG substrate with narrative DAG, YAML content model, 60Hz loop
- Strongest test case for the agentic composition pattern. Local to ironGate
  with direct UDS access to full NUCLEUS.
- **GAP-024 (open)**: signal paths not exercised live on biomeOS / ironGate
- Needs: biomeOS E2E signal validation, ludoSpring 6 game.* methods,
  Squirrel mechanical context, petalTongue DialogueTree
- **Deployment convergence**: Composes primals from `plasmidBin` genomeBins;
  BYOB composition via `tools/nucleus_composition_lib.sh` pattern

See `handoffs/WAVE21_GARDEN_EVOLUTION_MAY17_2026.md` for Wave 20 PM pattern
absorption guidance across all three garden teams (stability tiers, cross-tier
parity, ferment transcript, degradation behavior, cellMembrane integration).
Historical: `handoffs/GARDEN_EVOLUTION_BLURB_MAY16_2026.md` (Wave 18-era).

### blueFish

Remote repo not found (`404`). Either renamed, private, or not yet created.

### sporeGarden

Present in `gardens/`. Status and relationship TBD.

---

## Fossil Record

Historical handoffs are preserved in the [fossilRecord repository](https://github.com/ecoPrimals/fossilRecord) under `springs/primalSpring/`:
- `wateringHole_phase56_apr2026/` — v0.1.0 through Phase 56 (66+ files)
- `wateringHole_phase58_59_may2026/` — Phase 58–59 handoffs (6 files)

Git history in this repo retains full provenance at their original paths.
A local redirect stub exists at `fossilRecord/README.md`.
