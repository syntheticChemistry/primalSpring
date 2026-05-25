# wateringHole — primalSpring Ecosystem Guidance

**Version**: 0.9.28 (Wave 48 — 787 tests, 458 methods, 52 scenarios, 13/13 behavioral convergence, 8/8 springs sounded off, 4 gates operational)
**Last Updated**: May 25, 2026 (Wave 48 — Covalent mesh: 8 delta springs on 4 gates with Songbird TCP :7700 federation. `nucleus_launcher` Rust `--federation-port`. Deployment issues documented.)
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
| **PLASMIDBIN_DEPOT_PATTERN.md** | All consumers | How to fetch primal binaries from plasmidBin GitHub Releases. |
| **METHOD_GATE_STANDARD.md** | All primal teams | JH-0 ecosystem standard: pre-dispatch capability authorization, exempt whitelist, error codes, enforcement modes. |
| **PRIMAL_ANNOUNCE_PROTOCOL.md** | All primal teams | `primal.announce` atomic self-registration: wire format, field reference, registration order, signal-tier membership, backward compatibility. Replaces separate `lifecycle.register` + `capability.register` + `method.register` calls (biomeOS v3.57+). |
| **SIGNAL_ADOPTION_STANDARD.md** | All spring teams | Neural API composition collapse migration guide: `ctx.dispatch()` and `ctx.announce()` APIs, signal inventory (14 signals), spring archetype examples (compute/provenance/content-heavy), fallback behavior, validation coverage. |
| **DEPLOYMENT_BEHAVIOR_STANDARD.md** | All primal teams | Post-primordial deployment surface contract: `--socket` CLI, `health.liveness` shape, `lifecycle.status`, socket cleanup, signal handling. Compliance matrix for all 13 primals. |

### Team Ownership

| File | Audience | What It Covers |
|------|----------|----------------|
| **TEAM_OWNERSHIP_MATRIX.md** | All teams | Team boundaries: cellMembrane (infra/ops), projectNUCLEUS (deploy/compute), primalSpring (standards/observatory). |

### Living Handoffs

| File | Audience | What It Covers |
|------|----------|----------------|
| **handoffs/DEPENDENCY_VALIDATION_PATTERN.md** | All primal teams | Pre-dispatch data dependency staging pattern (from toadStool S266). |
| **handoffs/SHADOW_COMPARISON_PATTERN.md** | All primal teams | A/B shadow comparison pattern for protocol migrations (from songbird Wave 213). |
| **handoffs/archive/** | Historical | 32 fossilized handoffs from Waves 20–45 (deployment guides, announce blurbs, stadial gate, upstream resolution). |

### Archived Handoffs (`handoffs/archive/`)

All pre-Wave 39 handoffs have been absorbed and archived:

| File | Date | Summary |
|------|------|---------|
| `INTERSTADIAL_FOSSILIZATION_HANDOFF.md` | May 9 | Interstadial fossilization patterns: preservation, dating, provenance. |
| `WAVE38_TEAM_BLURBS_MAY22_2026.md` | May 22 | Per-team evolution blurbs with action items (absorbed by Wave 39-41). |
| `WAVE38_UPSTREAM_EVOLUTION_BLURB_MAY22_2026.md` | May 22 | Upstream evolution state (absorbed by Wave 39 Neural API evolution). |
| `WAVE37_STADIAL_APPROACH_CATALOGUE.md` | May 21 | Stadial approach catalogue (absorbed by Wave 38-39). |
| `WAVE31_UPSTREAM_EVOLUTION_BLURB_MAY20_2026.md` | May 20 | Wave 31 upstream blurb. |
| `WAVE24_SHADOW_RUN_EXECUTION_MAY19_2026.md` | May 19 | Shadow run execution. |
| `WAVE23_WETSPRING_E2E_COMPLETION_MAY18_2026.md` | May 18 | wetSpring E2E completion. |
| `WAVE22_COMPOSITION_PATTERNS_HANDOFF_MAY18_2026.md` | May 18 | Composition patterns handoff. |
| `WAVE22_UPSTREAM_PRIMAL_EVOLUTION_MAY17_2026.md` | May 17 | Wave 22 upstream hardening. |
| `WAVE22_STADIAL_GATE_PRIMAL_BLURB_MAY17_2026.md` | May 17 | Stadial gate final sweep. |
| + 11 earlier handoffs | May 9–17 | Various pre-stadial handoffs. |

### Ecosystem Standards (infra/wateringHole)

| File | Audience | What It Covers |
|------|----------|----------------|
| **REPO_MEMBRANE_BOUNDARY.md** | All teams | Inner/outer membrane repo classification: Forgejo-only, dual-push, GitHub-only. Contamination risk matrix, .env audit, push policy. |
| **SOVEREIGNTY_STANDARDS.md** | All teams | Calibrate → shadow → cutover protocol, bonding model, membrane channels, credential management, Forgejo as primary. |
| **MEMBRANE_CHANNEL_ARCHITECTURE.md** | projectNUCLEUS | Three membrane channels (Signal/Relay/Surface), deployment models, crypto layers, fieldMouse classification. |

### Archived Handoffs — Earlier Waves (`handoffs/archive/`)

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

## Current Ecosystem State (Wave 47)

- **13/13 primals** building standalone, distributed via plasmidBin genomeBin
  (Tier 1: x86_64, aarch64, armv7 — 40+ release assets)
- **Zero open upstream gaps** — 13/13 primals at zero debt, zero panics in production
- **458 registered capability methods** across 84+ domains — includes `neural_api.*` (6 methods), `science.*` (6), ionic bond methods, FIDO2, `primal.announce`, `primal.list`
- **49 validation scenarios** (10 tracks, 3 tiers: Rust / Both / Live) with shared `validation::helpers`; includes S47 neural-dispatch-live, S48 observatory-parity, S49 feedback-loop (live Neural API validation)
- **784 lib tests** (784 pass, 2 ignored)
- **biomeOS v3.70** — Neural API with adaptive routing weights (redb-persistent), weight health introspection, composition intelligence, and capability utilization tracking (hot/cold methods)
- **NeuralBridge observatory + feedback loop** — primalSpring consumes biomeOS routing intelligence and feeds dispatch outcomes back through `capability_call_instrumented` + `record_bridge_outcome`
- **14 atomic signal graphs** (`graphs/signals/`) defining Neural API composition collapse layer
- **13/13 BTSP Phase 3 FULL AEAD**, 13/13 default `127.0.0.1`
- **RootPulse commit workflow** fully executable (6/6 phases)
- **NestGate content-addressed storage** live (8 `content.*` methods)
- **sourDough v0.2.0** scaffold generates ecosystem-compliant primals
- **Team restructuring**: cellMembrane team owns infrastructure/ops, projectNUCLEUS refocused on deploy pipelines + big compute (see `TEAM_OWNERSHIP_MATRIX.md`)

## Key References (outside wateringHole)

| What | Where |
|------|-------|
| Gap registry | `docs/PRIMAL_GAPS.md` |
| Capability registry | `config/capability_registry.toml` (458 methods, zero drift) |
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

## Upstream Primal Evolution Status (May 24, 2026)

**Post-behavioral convergence.** All 13 primals at zero debt with uniform
deployment surface (DEPLOYMENT_BEHAVIOR_STANDARD 13/13). biomeOS v3.73
with capability-domain composition. Deep debt sweeps completed across
toadStool (S273), nestgate (S72), barraCuda, rhizoCrypt, coralReef.

### Completed (Wave 47)

- **DEPLOYMENT_BEHAVIOR_STANDARD** — 13/13 compliant. `--socket`, `health.liveness` → `"alive"`, `lifecycle.status`, SIGTERM, socket cleanup
- **`primal.announce`** — 12/12 (biomeOS exempt)
- **Deep debt** — toadStool zero unwrap, nestgate refactored unix_adapter, biomeOS capability-domain composition, barraCuda pollster eliminated, rhizoCrypt typed errors, coralReef zero across 11 categories

### Remaining Upstream Asks

| Primal | What | Priority | Status |
|--------|------|----------|--------|
| ~~**biomeOS**~~ | ~~Membrane model enforcement in `composition.deploy`~~ | ~~MEDIUM~~ | **RESOLVED** v3.74: validation gate on both live and shadow deploy paths |
| ~~**bearDog**~~ | ~~E2E ionic bond signing~~ | ~~MEDIUM~~ | **RESOLVED** — Wave 109: `crypto.ionic_bond.verify_proposal` shipped. primalSpring `s_ionic_bond` evolved to live RPC lifecycle (`bonding.propose`→`accept`→`status`→`terminate` + `crypto.ionic_bond.verify_proposal` probe). |
| ~~**bearDog**~~ | ~~ACME renewal daemon~~ | ~~LOW~~ | **RESOLVED** — Wave 112: `AcmeClient::run_renewal_loop()` wired into `beardog server` as background tokio task. Config via `BEARDOG_TLS_MODE=acme`. |

**All upstream primal asks are RESOLVED.** No remaining blockers from L1 primals.

### Glacial Shift Horizons

| Horizon | What | Owner |
|---------|------|-------|
| **Covalent bonding** | Multi-gate NUCLEUS mesh across 5 LAN gates | primalSpring + delta springs |
| **Sovereignty cutover** | Forgejo-primary, sovereign DNS, membrane channels | cellMembrane team |
| **Neural API as neural network** | Evolve adaptive routing weights toward learned routing | biomeOS + primalSpring |
| **Method coverage 80%** | Push from 322/458 (70%) to ~366/458 (80%) exercised | primalSpring scenarios |
| **plasmidBin Rust elevation** | Replace bash launcher scripts with Rust binary | primalSpring / plasmidBin |
| **sporePrint living content** | NestGate `content.put` pipeline for dynamic site updates | primalSpring (S6) |

---

## River Delta (Springs) — Evolution Summary (May 24, 2026)

All 8 springs pulled to HEAD. Combined: **9,700+ workspace tests** across the
delta. Every spring at zero debt. 5 springs actively deploying NUCLEUS on LAN gates.

| Spring | Tests | Gate | Latest |
|--------|------:|------|--------|
| hotSpring | 596 | biomeGate / strandGate | 204 experiments, VBIOS interpreter, 22 scenarios |
| healthSpring | 1,018 | ironGate | V64z: ionic absorption, dual-tower 7/7 Tower A, 57 scenarios |
| wetSpring | 1,962+ | southGate | V177+: breseq pipeline, ferment braid, 43 niche caps |
| neuralSpring | 739 | southGate | V170: 6 `science.*` methods, 10 scenarios |
| ludoSpring | 982 | ironGate | V77: 11/11 NUCLEUS PASS, proto-nucleate validated |
| groundSpring | 1,123+ | ironGate | V145: degradation behavior, NestGate IPC |
| airSpring | 1,373 | eastGate | v0.10.0: 57 caps, 17 methods full 3-tier coverage |
| primalSpring | 784 | eastGate / ironGate | Wave 47: 13/13 behavioral convergence, 458 methods, 49 scenarios |

**Convergence state**: All springs at zero debt. All primals at 13/13 behavioral
convergence. Neural API observatory operational with full feedback loop. biomeOS v3.73
owns composition intelligence with persistent routing weights and capability-domain
discovery. **Next vector**: covalent bonding across gates, method coverage push,
sovereignty cutover, plasmidBin Rust elevation.

---

## Downstream Products (Gardens) — Summary

### projectNUCLEUS V3

Forgejo PRIMARY (32 repos, 3 orgs). VPS Tower LIVE (DigitalOcean, Songbird
TURN, RustDesk, BearDog, SkunkBat, Caddy). 267+ bash security PASS, 33 Dark
Forest PASS. `deploy_membrane.sh` is operational tooling. cellMembrane Nest
LIVE on VPS with 10/10 membrane provenance PASS.

### lithoSpore v1.0.0

Verification chassis. 7/7 modules PASS at Tier 2 (75/75 checks). Bash-to-Rust
elevation complete. ScopeManifest, liveSpore.json provenance journal.

### projectFOUNDATION

Knowledge layer. 184 targets across 10 threads, 146 validated (79.3%).
Thread 10 targets `primalspring validate` directly.

### esotericWebb V9

357+ tests, 24 capabilities, 22 bridge methods. Signal-first provenance.
Strongest garden-level signal pattern implementation.

---

## Fossil Record

Historical handoffs are preserved in the [fossilRecord repository](https://github.com/ecoPrimals/fossilRecord) under `springs/primalSpring/`:
- `wateringHole_phase56_apr2026/` — v0.1.0 through Phase 56 (66+ files)
- `wateringHole_phase58_59_may2026/` — Phase 58–59 handoffs (6 files)

Git history in this repo retains full provenance at their original paths.
A local redirect stub exists at `fossilRecord/README.md`.
