# wateringHole — primalSpring Ecosystem Guidance

**Version**: 0.9.25 (Phase 60+ — Interstadial, eukaryotic validation, atomic signals)
**Last Updated**: May 15, 2026
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

### Living Handoffs

| File | Audience | What It Covers |
|------|----------|----------------|
| **INTERSTADIAL_FOSSILIZATION_HANDOFF.md** | Spring teams | Interstadial fossilization patterns: what to preserve, how to date, provenance READMEs. |
| **handoffs/PRIMALSPRING_SOVEREIGNTY_LAYER4_EVOLUTION_MAY15_2026.md** | All teams | Sovereignty track (3 scenarios), membrane deploy graph, routing config schema, 4-layer model. |

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
- **Zero open upstream gaps** — 13/13 primals at zero debt, Waves 1-12 complete, zero panics in production
- **441 registered capability methods** across 84+ domains (including `auth.*`, `nautilus.*`, `game.*`, ionic token methods, `btsp.capabilities`, `toadstool.validate`, `barracuda.precision.route`, `shader.compile.gemm`)
- **35 validation scenarios** (10 tracks, 3 tiers: Rust/Live/Both) with shared `validation::helpers`; sovereignty track validates membrane composition, routing parity, content sovereignty
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
| Capability registry | `config/capability_registry.toml` (441 methods, zero drift) |
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
3. **Validate against 441 methods**: Ensure niche capability counts align
   with `config/capability_registry.toml`.
4. **Validate membrane compositions**: Downstream membrane deployments must conform
   to `config/routing_config_reference.toml` schema (backend types, trust tiers,
   telemetry). Use `graphs/membrane/tower_membrane.toml` as canonical VPS graph.
4. **Test with biomeOS v3.57**: Signal-tier interception in `capability.call`
   is now live — verify transparent composition collapse doesn't break
   existing call patterns.

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
| primalSpring | 767 | — | Neural API evolution, signal dispatch, primal.announce, doc reconciliation |

**Convergence state**: All springs CI-validated against canonical 441 methods. 35 scenarios across 10 tracks.
All implement BYOB niche model, deploy graphs, and Tier 1/2 validation.
Fragment-first graph composition adopted ecosystem-wide.

---

## Downstream Products (Gardens) — Evolution Summary (May 15, 2026)

### projectNUCLEUS — Sovereignty Evolution

projectNUCLEUS has driven massive sovereignty infrastructure evolution:

- **Forgejo PRIMARY**: 32 repos, 3 orgs, dual-push mirror to GitHub
- **VPS Tower LIVE**: DigitalOcean 2GB, Songbird TURN :3478, RustDesk,
  BearDog, SkunkBat, Caddy — hardened membrane posture
- **Channel 3 TLS**: `membrane.primals.eco` with DNS grey-cloud, ACME
  (Let's Encrypt E8), sporePrint served from VPS cache, HTTP parity PASS
- **Membrane telemetry**: Continuous sovereignty shadow, rolling
  `membrane_7day.toml` baselines, `darkforest` v0.2.1 membrane audit
- **Interstadial exit gate CLEARED**: Dark Forest PASS, 13/13 primals LIVE
- **BearDog TLS shadow**: :8443, 11ms RPC vs 188ms Cloudflare

**Still in progress**: BTSP JupyterHub cutover (dual-auth shadow active),
petalTongue extracellular wiring, sovereign DNS (knot-dns, H2-17–H2-20),
Forgejo Actions CI porting.

### CATHEDRAL (lithoSpore + foundation)

**lithoSpore** — First Targeted GuideStone (hypogeal cotyledon):
- **7/7 modules PASS** at Tier 2 (75/75 checks), chaos tested
- Ingested primalSpring patterns: capability registry, Dark Forest graphs
- Deep evolution: UDS RPC implemented, monoliths refactored, USB pipeline hardened
- Interactive SceneGraph handback complete (6/6 phases via petalTongue)
- Needs from upstream: Songbird TURN client library, genomeBin Tier 3 USB,
  neuralSpring ML surrogates for B3/B4/B6

**foundation** — Validated scientific lineage (10 domain threads):
- Thread 2 Plasma **12/12 PASS**, Thread 6 Agricultural **36/36 PASS**,
  Thread 7 Anderson **18/18 PASS**
- Thread 1 WCM: fetch infra ok, **RPC upstream-blocked** (0/24 pending review)
- Data integrity: many `blake3 = ""` in source TOMLs — needs fetch + backfill
- Upstream audit prep delivered (May 15)
- Needs from upstream: RPC stack for Thread 1, neuralSpring ML sources (Thread 5)

### esotericWebb

At V7 — self-composed via primal composition (ludoSpring decomposed).
Neural API routing wired in PrimalBridge. Clean.

### blueFish

Remote repo not found (`404`). Either renamed, private, or not yet created.

---

## Fossil Record

Historical handoffs are preserved in the [fossilRecord repository](https://github.com/ecoPrimals/fossilRecord) under `springs/primalSpring/`:
- `wateringHole_phase56_apr2026/` — v0.1.0 through Phase 56 (66+ files)
- `wateringHole_phase58_59_may2026/` — Phase 58–59 handoffs (6 files)

Git history in this repo retains full provenance at their original paths.
A local redirect stub exists at `fossilRecord/README.md`.
