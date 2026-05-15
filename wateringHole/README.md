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
`primal.announce` protocol), several primals carry merge conflict debt from
upstream pushes. These are merge conflicts (UU markers) from stashed local
work colliding with upstream evolution — they need resolution before the
next ecosystem pull.

### Clean (pushed, ready for cross-repo pull)

| Primal | HEAD | Notes |
|--------|------|-------|
| biomeOS | v3.57 (`75209fc`) | Neural API evolution complete: signal dispatch, announce protocol, metrics tagging |
| squirrel | `db3db3a` | `signal_plan` mode for `ai.query` — composition collapse |
| barraCuda | Sprint 69 (`10473ba`) | `health.version` standalone RPC |
| bearDog | Wave 102 (`103982c`) | Ionic lease on `crypto.sign_contract` (but see conflict debt below) |
| skunkBat | H2 niche (`85ee1e0`) | Live lineage, enforcement, NestGate protection |
| sourDough | v0.3.0 (`1b744b2`) | Scaffold docs updated |

### Merge Conflict Debt (UU markers — needs team resolution)

| Primal | UU Files | Conflict Domain | Severity |
|--------|---------|-----------------|----------|
| **beardog** | **38** | HSM management, self-discovery, ecosystem spawner, BTSP handlers, crypto handler, types/capabilities, constraints, production config | **HIGH** — deep structural evolution collided with upstream; blocks next bearDog wave |
| **rhizoCrypt** | 5 | Cargo.lock, niche.rs, niche_tests.rs, rhizocrypt_tests.rs | MEDIUM — niche/test alignment |
| **petalTongue** | 3 | IPC dispatch, motor handlers, paint primitives | MEDIUM — UI + IPC evolution |
| **coralReef** | 1 | `newline_jsonrpc.rs` IPC framing | LOW — single file |
| **loamSpine** | 1 | `btsp/handshake.rs` | LOW — BTSP alignment |
| **songbird** | 1 (DU) | `env_config.rs` deleted upstream, modified locally | LOW — delete/modify conflict |
| **sweetGrass** | 1 | `bin/service.rs` | LOW — service entry point |

### Modified (no conflicts, uncommitted)

| Primal | Changes | Notes |
|--------|---------|-------|
| nestGate | 2 modified (`run.rs`, `subcommands.rs`) | CLI subcommand evolution — review and commit |
| toadStool | 1 modified (`mappings_extended.rs`) | Semantic method mappings — review and commit |
| bingoCube | 1 modified (`Cargo.toml`) | Dependency update — review and commit |

### Evolution Targets (post-conflict resolution)

Once merge conflicts are resolved, each primal should:

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

### Infra Repos (separate push scope)

| Repo | State | Action |
|------|-------|--------|
| infra/wateringHole | Clean | Consider syncing `PRIMAL_ANNOUNCE_PROTOCOL.md` |
| infra/whitePaper | 3 modified neuralAPI chapters (00, 01, 03) | Push by whitePaper maintainer |
| infra/benchScale | 1 modified spec + 1 untracked topology | Push when reviewed |
| neuralSpring | 1 untracked `inference.rs` | Minor — add or gitignore |

---

## Fossil Record

Historical handoffs are preserved in the [fossilRecord repository](https://github.com/ecoPrimals/fossilRecord) under `springs/primalSpring/`:
- `wateringHole_phase56_apr2026/` — v0.1.0 through Phase 56 (66+ files)
- `wateringHole_phase58_59_may2026/` — Phase 58–59 handoffs (6 files)

Git history in this repo retains full provenance at their original paths.
A local redirect stub exists at `fossilRecord/README.md`.
