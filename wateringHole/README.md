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

### Living Handoff

| File | Audience | What It Covers |
|------|----------|----------------|
| **INTERSTADIAL_FOSSILIZATION_HANDOFF.md** | Spring teams | Interstadial fossilization patterns: what to preserve, how to date, provenance READMEs. |

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
- **32 validation scenarios** (9 tracks, 3 tiers: Rust/Live/Both) with shared `validation::helpers`
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
| Method gate CI | `tools/check_method_gate.sh` |
| Method string validator | `tools/check_method_strings.sh` |
| Graph method validator | `tools/check_graph_methods.sh` |
| Experiment tracks | `experiments/` (89 experiments, 20 tracks) |
| Deploy graphs | `graphs/` (79 deploy TOMLs + 14 atomic signal graphs) |
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
| toadStool | `ccd9243` | S262: expose `device.gr.init` IPC + coralReef shader metadata aliases |
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

## Fossil Record

Historical handoffs are preserved in the [fossilRecord repository](https://github.com/ecoPrimals/fossilRecord) under `springs/primalSpring/`:
- `wateringHole_phase56_apr2026/` — v0.1.0 through Phase 56 (66+ files)
- `wateringHole_phase58_59_may2026/` — Phase 58–59 handoffs (6 files)

Git history in this repo retains full provenance at their original paths.
A local redirect stub exists at `fossilRecord/README.md`.
