# Downstream Pattern Guide — Springs to Products

How the 8 river delta springs feed projectNUCLEUS, projectFOUNDATION, and lithoSpore.

**Last updated**: May 16, 2026 — Wave 18+: Garden evolution review. lithoSpore v1.0.0
(ScopeManifest, liveSpore.json, CLI integration tests), projectNUCLEUS V3 (55 tests,
discovery cascade, 7 gates, signal_executor.sh, tower_agent.toml), esotericWebb V8
(357 tests, signal-first provenance, lifecycle handlers), projectFOUNDATION (184 targets,
29 workloads, primal_ipc.sh, 6 CPU parity benchmarks). 456 methods, 43 scenarios
(10 tracks, 3 tiers), 14 atomic signal graphs. All UB-1–4 SHIPPED.

---

## Three Pattern Classes

Springs produce three kinds of output that downstream products consume:

1. **LTEE Reproductions** — validated scientific results (springs → lithoSpore)
2. **Foundation Thread Seeds** — data sources, targets, expressions (springs → foundation)
3. **Workload + Notebook Pipeline** — dispatchable binaries + science pages (springs → projectNUCLEUS)

```
                    springs (8)
                   ┌──────────┐
                   │ airSpring │──┐
                   │groundSpring│──┤  LTEE reproductions
                   │healthSpring│──┤  ──────────────────► lithoSpore (7 modules)
                   │ hotSpring  │──┤
                   │ ludoSpring │──┤  Foundation seeds
                   │neuralSpring│──┤  ──────────────────► projectFOUNDATION (10 threads)
                   │primalSpring│──┤
                   │ wetSpring  │──┤  Workloads + notebooks
                   └──────────┘  └──────────────────► projectNUCLEUS (deployment)
```

---

## 1. LTEE Reproduction Stack (springs → lithoSpore)

Each LTEE reproduction follows a standard pattern:

```
<spring>/control/ltee_*/expected_values.json     ← ground truth
<spring>/target/release/validate_ltee_*          ← Rust binary
<spring>/notebooks/papers/*-ltee-*.ipynb         ← science notebook
<spring>/experiments/results/ltee/*.json         ← raw outputs
```

### Per-Spring LTEE Status → lithoSpore Module Mapping

| Spring | Paper | Status | Output Location | lithoSpore Module |
|--------|-------|--------|-----------------|-------------------|
| **groundSpring** | B1 (Barrick 2009 — neutral mutation) | **COMPLETE** Py 8/8 + Rust 8/8 | `control/ltee_neutral/expected_values.json` | Module 1: ltee-mutation |
| **groundSpring** | B2 (Wiser 2013 — fitness dynamics) | **COMPLETE** Py 9/9 + Rust 10/10 | `control/ltee_fitness/expected_values.json` | Module 2: ltee-fitness |
| **groundSpring** | B3 (Good 2017 — clonal interference) | **COMPLETE** (V136) | `control/ltee_clonal/` | Module 3: ltee-clonal |
| **groundSpring** | B4 (Blount 2008/2012 — citrate) | **COMPLETE** (V140) | `control/ltee_citrate/` | Module 4: ltee-citrate |
| **hotSpring** | B2 (Anderson/Wiser — fitness) | **COMPLETE** Tier 1+2 | `experiments/results/ltee/ltee_b2_anderson_expected.json` | Module 7: ltee-anderson |
| **healthSpring** | B5 (Leonard 2024 — symbiont PK/PD) | **COMPLETE** Py 8/8 + Rust 8/8 (V64f) + `--format json` + `tolerances.toml` | `control/ltee_symbiont_pkpd/` (full lithoSpore module candidate) + `bin/validate_ltee_b5` | Module: ltee-symbiont-pk |
| **neuralSpring** | B1 (mutation accumulation ML) | **Py 8/8 + Rust binary DONE** (S201b) | `control/ltee_mutation_accumulation/` + `src/bin/validate_ltee_b1_*` | ML surrogate modules |
| **wetSpring** | B7 (Tenaillon 2016 — 264 genomes) | **Tier 2 COMPLETE** (27/27 PASS) | `experiments/results/ltee_b7_expected_values.json` | Module 6: ltee-breseq (Tier 2 PASS) |

**Convention**: lithoSpore modules consume `expected_values.json` from springs via
`litho fetch` (pure Rust, replaces 7 bash fetch scripts) that BLAKE3-anchor
the data into NestGate content storage.

### lithoSpore Status (May 15, 2026)

- **7/7 modules Tier 2 PASS** (75/75 checks) — module 5 (biobricks) promoted
  with metabolic burden validation (6/6)
- **Bash-to-Rust elevation COMPLETE** — all 8 shell scripts replaced with pure
  Rust CLI subcommands in the `litho` binary
- **Cross-platform deployment matrix**: musl-static Linux (5.1 MB), Windows
  cross-compiled via `x86_64-pc-windows-gnu` (7.9 MB, tested via Wine 11).
  Validated on Ubuntu airgap, VPS, Alpine chroot, read-only FS.
- **USB recreation**: `litho assemble` builds portable artifacts per
  `LITHOSPORE_USB_DEPLOYMENT.md`. argv[0] symlink detection for entry points.
- **Module lib.rs pattern**: each module exposes `run_validation()` for
  in-process dispatch — single binary replaces 7 separate module binaries.
- **Discovery chain**: env → UDS → TURN → standalone, with mode detection
  and `liveSpore.json` provenance recording.
- **Needs from upstream**: neuralSpring ML surrogates for B3/B4/B6
  (Songbird TURN client and genomeBin USB both SHIPPED — lithoSpore wiring pending)

### Geo-Delocalized lithoSpore Validation (via cellMembrane)

lithoSpore guideStone USBs can now validate at remote gates through the
cellMembrane relay. The USB carries its own data and runtime (hypogeal
cotyledon pattern — see `wateringHole/LITHOSPORE_USB_DEPLOYMENT.md`):

```
lithoSpore USB at remote gate
  → ./validate detects SONGBIRD_TURN_SERVER
  → primal IPC via Songbird TURN (:3478) through cellMembrane
  → Tier 2 validation against NUCLEUS compute primals
  → liveSpore.json appended (provenance entry)
  → spring absorbs results → sporePrint publishes
```

Three operating modes per USB:
1. **Standalone**: Tier 1 Python-only (no network required)
2. **LAN**: Tier 2 Rust + primal IPC via Unix sockets
3. **Geo-delocalized**: Tier 2 via Songbird TURN through cellMembrane

Cross-hardware validation across geographic locations (AMD at strandGate,
CPU-only at friend gates, NVIDIA at biomeGate) produces the stadial
evidence for interstadial exit — published as auditable journal entries
on sporePrint (primals.eco).

---

## 2. Foundation Thread Seeding (springs → projectFOUNDATION)

Foundation threads have three components: **expression** (the question), **data sources**
(where data lives), and **data targets** (expected numerical results).

### Thread Ownership and Seeding Status

| Thread | Name | Owner Springs | Expression | Sources | Targets | Status |
|--------|------|---------------|:----------:|:-------:|:-------:|--------|
| 1 | Whole-Cell Modeling | hotSpring, wetSpring, healthSpring | YES | YES (27) | YES (24) | **ACTIVE** |
| 2 | Plasma Physics / QCD | hotSpring | YES | YES | YES | **ACTIVE** |
| 3 | Immunology / Drug Discovery | wetSpring, airSpring, healthSpring | YES (V64) | YES | YES | **ACTIVE** |
| 4 | Environmental Genomics | wetSpring, airSpring | **NO** | YES | **NO** | **Needs expression + targets** |
| 5 | Evolutionary Biology / LTEE | groundSpring, neuralSpring, wetSpring, hotSpring | YES | YES (12) | YES (18) | **ACTIVE** (seeded May 11) |
| 6 | Agricultural Science | airSpring, groundSpring, wetSpring | YES | YES | YES | **ACTIVE** |
| 7 | Anderson Mathematics | hotSpring, groundSpring, wetSpring, neuralSpring | YES | YES | YES | **ACTIVE** |
| 8 | Human Health / Clinical | healthSpring | YES (V64) | YES | YES | **ACTIVE** |
| 9 | Gaming / Creative | ludoSpring | **YES** (V71) | YES (13) | YES | **ACTIVE** (ludoSpring T9 seeded) |
| 10 | Provenance / Economics | ludoSpring, primalSpring, healthSpring | **YES** (V71) | YES | YES | **ACTIVE** (ludoSpring T10 seeded, healthSpring gap documented) |

### Thread 5 (LTEE/Evolution) — SEEDED (May 11)

Thread 5 was the critical empty backbone for lithoSpore. Now seeded with:
- **Expression**: `expressions/LTEE_EVOLUTIONARY_DYNAMICS.md` — connects Barrick/Wiser/Good/Tenaillon papers
- **Data sources**: 12 NCBI/Dryad accessions (`data/sources/thread05_ltee.toml`)
- **Data targets**: 18 validation targets (`data/targets/thread05_ltee_targets.toml`) — 14 validated (groundSpring B1-B3, hotSpring B2, neuralSpring B1-ML), 4 pending (wetSpring B7)

**Remaining partial threads**: 4 (Environmental Genomics — partial, lithoSpore T4 integration active), 9 (Gaming — ludoSpring T9 seeded), 10 (Provenance — ludoSpring T10 seeded)

### Foundation Seed Pattern

Each thread seed follows the structure:

```toml
# data/sources/thread05_ltee.toml
[meta]
thread = 5
thread_name = "Evolutionary Biology / LTEE"
expression = "expressions/LTEE_EVOLUTIONARY_DYNAMICS.md"

[[sources]]
id = "dryad_wiser_2013"
database = "Dryad"
description = "Wiser et al. 2013 fitness trajectory data (50,000 generations)"
accessions = ["doi:10.5061/dryad.234"]
url = "https://datadryad.org/..."
format = "csv"
blake3 = ""
retrieved = ""
paper = "B2"
```

---

## 3. Workload + Notebook Pipeline (springs → projectNUCLEUS)

### Convergence Tiers (per ecoPrimals wateringHole SPRING_EVOLUTION_TARGETS.md)

| Tier | What | Spring Requirement |
|------|------|-------------------|
| 0 | CLI binary → `[OK]/[FAIL]` stdout | Validation binary in `target/release/` |
| 1 | + notebook + frozen data + sporePrint | `notebooks/01-05`, `sporeprint/`, `experiments/results/` |
| 2 | + JSON-RPC methods via toadStool | `--format json` flag, `toadstool.validate` wiring |
| 3 | + petalTongue live dashboards | Nothing new from springs |
| Standalone | NestGate-hosted, self-sovereign | Infrastructure evolution |

### Per-Spring Tier Status

| Spring | Tests | Tier | Workload TOMLs | Notebooks | sporePrint | LTEE | Foundation Threads |
|--------|------:|:----:|:--------------:|:---------:|:----------:|:----:|:------------------:|
| **airSpring** | 1,429 | 1 | YES (thread06_ag) | 5 + 20 papers | YES | E3 queued | 6 |
| **groundSpring** | 1,123 | 1 | YES | 5 + 29 baselines | YES | **B1-B4 DONE** | 5, 7 |
| **healthSpring** | 1,018 | 2 | YES (PK + Nest atomic + cell.toml) | 95 experiments | YES | V64m: NestComposition facade, cell.toml | 3, 5, 8, 10 |
| **hotSpring** | 592 | 1 | YES | 5 + 12 papers | YES | `s_node_atomic` added, base64 extracted | 2 |
| **ludoSpring** | 896 | 2 | YES (2 TOMLs + Tower atomic) | MDA + matchmaking + chat | YES | V71: **Tower LIVE** (6/6), Foundation T9+T10 | 9, 10 |
| **neuralSpring** | 910 | 1 | YES (7 IPC modules) | 397 baselines + 27 papers | YES | V159: NestGate weights WIRED, Squirrel inference COMPLETE | 5, 7 |
| **primalSpring** | 689+ | N/A | coordination | 5 (meta) | YES | coordination | coordination |
| **wetSpring** | 1,962 | 1 | YES | 384 experiments + papers | YES | V167: gS **L5**, B7 Tier 2 COMPLETE | 4 |

**All 8 springs at Tier 1+.** healthSpring and ludoSpring at Tier 2 (atomic niche wired). wetSpring promoted to gS L5.
`toadstool.validate` **IMPLEMENTED** (S250), Phase D **FACTORY WIRED** (S254 — AMD live).
`compute.dispatch.submit` **LIVE** for AMD, FECS-gated for NV. 7/7 springs Tier 2 wired.
Provenance trio **GAP-36 RESOLVED** — all wire aliases normalized upstream.

### What projectNUCLEUS Needs Next

1. **Workload TOML completeness** — ensure each spring has at least one validated workload in `projectNUCLEUS/workloads/<spring>/`
2. **`--format json` flag** — springs add structured JSON output to validation binaries (additive, doesn't break CLI)
3. **Notebook execution** — sporePrint CI (`notify-sporeprint.yml` with `content: "true"`) runs `nbconvert --execute` on push
4. **Foundation thread coverage** — Threads 5, 8, 10 need full seeding before projectFOUNDATION can validate them
5. **Geo-delocalized workload dispatch** — remote gates via cellMembrane relay can run Tier 2 workloads; `liveSpore.json` provenance feeds back to sporePrint for auditable publication

### CATHEDRAL Split (May 16, 2026)

The CATHEDRAL team has split into two focused workstreams:

- **lithoSpore** — verification chassis. USB-deployable validation artifacts,
  module crates, geo-delocalized Tier 2. Owns benchScale and agentReagents.
- **projectFOUNDATION** — knowledge layer. Thread lineage, data sources/targets,
  validation evidence capture. Owns `lineage/THREAD_INDEX.toml` and
  `data/sources/*.toml`.

Springs now feed both workstreams:
- **Validation results** flow to projectFOUNDATION as thread evidence
  (dated provenance folders in `projectFOUNDATION/validation/`)
- **Module crates** can be wired into lithoSpore instances via `scope.toml`
  if they expose `fn run_validation(data_dir, expected, max_tier) -> ModuleResult`

**lithoSpore status**: 7/7 modules Tier 2 PASS (75/75 checks). VM-validated via
benchScale + agentReagents. USB pipeline confirmed geo-delocalized.

**Upstream blockers**: All SHIPPED (UB-1 through UB-4). Remaining work is
lithoSpore-side wiring (TURN relay integration, FIDO2 witness path).

---

## 4. Dark Forest Gate Adoption (springs → security compliance)

The **Dark Forest Glacial Gate Standard** (`wateringHole/DARK_FOREST_GLACIAL_GATE_STANDARD.md`)
defines five security invariants every deployment must satisfy. primalSpring validates these
structurally via the `s_dark_forest_gate` scenario. Each spring should adopt the standard
through their `guidestone` feature gate.

### What Springs Must Validate

| Pillar | Spring CI Check |
|--------|----------------|
| 1. Zero Metadata | Verify `stripped = true` in plasmidBin manifest for all consumed primals |
| 2. Zero Port | Verify no hardcoded TCP ports in spring deploy graphs; `transport = "uds_only"` in graph metadata |
| 3. Songbird Network | Verify no non-songbird nodes advertise `http.*` or `tls.*` capabilities in deploy graphs |
| 4. BTSP Crypto | Verify all deploy graphs carry `secure_by_default = true` in `[graph.metadata]` |
| 5. Enclave | Verify `trust_model = "MethodGate"` in bonding policy; content capabilities route to NestGate |

### Adoption Pattern (via guidestone feature gate)

Springs already CI-validate against the 456-method registry. Dark Forest checks
are an additional axis in the same `#[cfg(feature = "guidestone")]` test module:

```rust
#[cfg(feature = "guidestone")]
mod dark_forest {
    #[test]
    fn deploy_graphs_secure_by_default() {
        // Parse all graphs in graphs/ directory
        // Assert [graph.metadata] secure_by_default = true
    }

    #[test]
    fn no_direct_network_in_graphs() {
        // For each graph node: if name != "songbird",
        // assert no http.* or tls.* in capabilities
    }

    #[test]
    fn tower_base_in_all_compositions() {
        // Assert beardog + songbird + skunkbat present
        // in every composition that includes tower_atomic
    }
}
```

### Adoption Status

| Spring | Registry Sync | Dark Forest Gate | Notes |
|--------|:------------:|:----------------:|-------|
| hotSpring | **G** | **PENDING** | Has `secure_by_default` in graphs already |
| neuralSpring | **G** | **PENDING** | Tower + skunkBat in V160 |
| wetSpring | **G** | **PENDING** | Composed — pure primal, minimal surface |
| healthSpring | **G** | **PENDING** | Dual-tower enclave — Pillar 5 critical path |
| ludoSpring | **G** | **PENDING** | Pure composition — minimal surface |
| groundSpring | **G** | **PENDING** | 6 deploy graphs need audit |
| airSpring | **G** | **PENDING** | 7 graphs, 10 scenarios — good foundation |
| primalSpring | **G** | **DONE** | `s_dark_forest_gate` scenario (29 checks) |

### sourDough Convergence (v0.4.0 roadmap)

`sourdough validate dark-forest <graph-dir>` will wrap the 5-pillar checks for any
spring's graph directory, providing a single CLI command for Dark Forest compliance.
This converges with `sourdough validate composition` (v0.3.0) and the plasmidBin
`validate_composition.sh` script.

---

## 5. Signal Consumption — Neural API Composition Collapse

The Neural API provides a **semantic collapse** layer: instead of springs
calling individual primal methods (456 in the registry), they dispatch
atomic signals (14 defined in `config/signal_tools.toml`) and let biomeOS
execute the underlying graph of method calls.

### The Pattern

```
BEFORE (flat method surface — spring manages sequencing):
  ctx.call("content", "content.put", data)
  ctx.call("dag", "dag.event.append", event)
  ctx.call("spine", "spine.seal", vertex)
  ctx.call("braid", "braid.create", braid)

AFTER (signal dispatch — biomeOS manages the graph):
  ctx.dispatch("nest.store", json!({ "content": data, "author": id }))
```

### APIs Available (primalSpring v0.9.25+, Wave 17+)

| Method | Purpose |
|--------|---------|
| `ctx.dispatch("tier.name", params)` | Unified signal dispatch (splits identifier, routes to biomeOS) |
| `ctx.announce(primal, methods, socket)` | Atomic registration replacing 3-call pattern |
| `ctx.signal(tier, name, params)` | Low-level signal dispatch (tier + name separate) |
| `ctx.signal_plan(intent)` | Squirrel-powered intent → signal sequence planning |
| `ctx.execute_plan(plan)` | Execute a squirrel signal plan |

### Signal Inventory (14 signals across 4 tiers)

| Tier | Signals | Composition Surface |
|------|---------|-------------------|
| **Tower** (electron) | `publish`, `authenticate`, `discover`, `health`, `bootstrap` | Identity, trust, mesh |
| **Node** (proton) | `compute` | Dispatch, compile, execute |
| **Nest** (neutron) | `store`, `commit`, `retrieve` | Provenance, storage, ledger |
| **Meta** | `observe`, `intent`, `render`, `health`, `deploy` | AI, orchestration, UI |

### Migration Path for Springs

1. **Registration**: Replace `method.register` + `capability.register` +
   `lifecycle.register` with a single `ctx.announce()` call. See
   `wateringHole/PRIMAL_ANNOUNCE_PROTOCOL.md`.

2. **Capability calls**: Identify multi-call sequences that correspond to
   atomic signals. Replace with `ctx.dispatch()`. Domain-specific math calls
   (`stats.mean`, `gpu.matmul`) stay as `ctx.call()`.

3. **Multi-signal workflows**: For complex intent, use `ctx.signal_plan()`
   to let squirrel decompose into a signal sequence, then `ctx.execute_plan()`.

### What This Means for Downstream Products

**projectNUCLEUS**: Workload TOMLs can reference signals instead of
individual method sequences. A workload that currently specifies
`nest.store → content.put + dag.event.append + spine.seal + braid.create`
can be simplified to `signal: "nest.store"`.

**lithoSpore**: Module validation that currently exercises individual primal
methods can add signal dispatch phases. The `dispatch()` API has automatic
fallback to `capability.call` for pre-v3.56 biomeOS, so existing validation
continues to work.

**projectFOUNDATION**: Thread expression validation can use signals for the
provenance storage leg (`nest.store` for results, `nest.commit` for session
finalization).

### Validation Coverage

primalSpring validates the signal API through:
- `s_signal_dispatch_parity` — dispatches all 14 signals, validates response shapes
- `s_primal_announce` — validates announce wire format and live registration
- `s_atomic_signals` — structural + live dispatch validation per signal graph
- `s_provenance_trio_pipeline` Phase 6 — `nest.store` signal dispatch

Full standard: `wateringHole/SIGNAL_ADOPTION_STANDARD.md`

---

## 6. Sovereignty Validation Patterns (primalSpring ↔ projectNUCLEUS)

The sovereignty track validates the 4-layer model from `PRIMAL_VS_SOVEREIGNTY_GOALS.md`:

| Layer | What | primalSpring Validation |
|-------|------|------------------------|
| 1. Primal Capabilities | 456 methods, 13 primals | Existing: `composition-parity`, `domain-contract-sweep` |
| 2. Security Validation | BTSP, MethodGate, Dark Forest | Existing: `dark-forest-gate`, `bearer-token-auth` |
| 3. Sovereignty Deployment | VPS membrane, content routing | **NEW**: `membrane-composition`, `sovereignty-parity` |
| 4. Sovereign Composition | All atomics self-hosted | **NEW**: `content-sovereignty` |

### Membrane Composition (`graphs/membrane/tower_membrane.toml`)

The VPS membrane graph defines the inner boundary: Tower primals (BearDog +
Songbird + SkunkBat) plus NestGate in cache-only mode. Three channels:

- **Channel 1 (Signal)**: UDS — primal-to-primal IPC on VPS
- **Channel 2 (Relay)**: BTSP tunnel — VPS to gate encrypted relay
- **Channel 3 (Surface)**: TLS — public HTTPS on `membrane.primals.eco`

`s_membrane_composition` (Tier::Rust) validates this graph structurally:
`secure_by_default`, `composition_model = "membrane"`, bonding policy,
telemetry contract.

### Content-Aware Routing (`config/routing_config_reference.toml`)

primalSpring owns the canonical routing schema; projectNUCLEUS owns instances.
The schema defines:

- **Backend types**: `btsp_tunnel`, `local_filesystem`, `songbird_p2p`, `http_proxy`
- **Match predicates**: `path_prefix`, `path_regex`, `host`, `content_type`, `header`, `min_size_mb`
- **Trust tiers**: `covalent` (all access), `ionic` (scoped), `metallic` (compute), `weak` (public only)
- **Telemetry**: shadow mode, cutover gate days, SkunkBat correlation

`s_sovereignty_parity` (Tier::Both) validates the schema structurally and
probes membrane health live.

### Calibrate-Shadow-Cutover Protocol

Sovereignty transitions follow `SOVEREIGNTY_STANDARDS.md`:

1. **Calibrate**: Baseline current metrics (latency, uptime, TLS, content hash parity)
2. **Shadow**: Run sovereign path in parallel, collect telemetry, compare parity
3. **Cutover**: After `cutover_gate_days` (>= 7) consecutive parity, switch primary

`s_content_sovereignty` (Tier::Live) validates the content pipeline through
the sovereign routing layer, verifying BLAKE3 round-trip fidelity and
SkunkBat audit correlation.

---

## Per-Spring Downstream Priority

| Spring | Highest Leverage Hand-Down |
|--------|--------------------------|
| **groundSpring** | LTEE B1-B3 → lithoSpore modules 1-3; Thread 5+7 → projectFOUNDATION; measurement science baselines |
| **hotSpring** | L6 guideStone template; LTEE B2 → lithoSpore module 7; Thread 2 → projectFOUNDATION; 3-tier ladder pattern |
| **wetSpring** | LTEE B7 genomics → lithoSpore module 6; forge workload taxonomy → projectNUCLEUS; Thread 4 → projectFOUNDATION |
| **neuralSpring** | ML surrogate scaffolding → lithoSpore forecasting; Threads 5+7 → projectFOUNDATION; helixVision primitives |
| **healthSpring** | PK validation → projectNUCLEUS workloads; Threads 3+8 → projectFOUNDATION; provenance trio IPC pattern |
| **airSpring** | 36 projectFOUNDATION targets → Thread 6; irrigation/soil ladder → projectNUCLEUS; methods.rs drift-proofing |
| **ludoSpring** | Composition-only niche → projectNUCLEUS; Threads 9+10 → projectFOUNDATION; HCI validation suite |
| **primalSpring** | foundation_validation.toml graph; BTSP/MethodGate standards; composition coordination truth |

---

## Upstream Pattern Escalation

Downstream products surface patterns that need upstream adoption. Full
inventory with primal blockers (UB-1..4), canonicalization targets, spring
adoption actions, and glacial horizon predictions:

**See**: `wateringHole/handoffs/archive/UPSTREAM_PATTERN_ESCALATION_MAY15_2026.md`

Highest-leverage for glacial push (UB-1 through UB-4 all SHIPPED):
- **lithoSpore TURN wiring** — wire `songbird-turn-client` into `litho_core` for geo-delocalized validation
- **Discovery chain env var standardization** — unifies spring/garden implementations
- **neuralSpring ML surrogates** — B3/B4/B6 lithoSpore module integration

### Signal Adoption Escalation (Wave 17+)

The Neural API signal layer is now fully surfaced. Upstream gaps that will
surface as springs adopt `ctx.dispatch()`:

- **Primals missing from signal graphs**: any primal not responding to
  capabilities expected by `graphs/signals/*.toml` will produce `-32601`
  errors in `s_signal_dispatch_parity`
- **biomeOS graph execution gaps**: signal graphs that biomeOS cannot fully
  execute will surface via response shape validation
- **Announce protocol adoption**: primals not implementing `primal.announce`
  will fall back to the 3-call pattern (functional but deprecated)

Signal adoption standard: `wateringHole/SIGNAL_ADOPTION_STANDARD.md`

### Ferment Transcript Pattern (lithoSpore, May 17 2026)

Springs processing large upstream datasets (10s–100s of GB) that feed
guideStone artifacts MUST produce **ferment transcripts**: summary statistics
plus a provenance braid that links to the full computation chain. The
guideStone carries the summary and the braid reference — not the raw data.

**Priority datasets awaiting upstream braids:**

| Dataset | Spring | Computation | Priority |
|---------|--------|-------------|----------|
| Tenaillon 2016 | wetSpring | breseq on 264 genomes (~200 GB) | HIGH |
| Barrick 2009 | wetSpring | breseq on 19 genomes (~15 GB) | HIGH |
| Good 2017 | wetSpring | metagenomic variant calling (~50 GB) | MEDIUM |
| Blount 2012 | wetSpring | replay experiment sequencing (~30 GB) | MEDIUM |

**Full contract**: `infra/wateringHole/handoffs/LITHOSPORE_FERMENT_TRANSCRIPT_BRAID_HANDOFF_MAY17_2026.md`
**Trio transaction semantics**: `infra/wateringHole/PROVENANCE_TRIO_INTEGRATION_GUIDE.md` § Transaction Semantics
