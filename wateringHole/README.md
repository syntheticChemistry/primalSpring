# wateringHole — primalSpring Ecosystem Guidance

**Version**: 0.9.25 (Phase 60+ post-interstadial)
**Last Updated**: May 10, 2026
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

### Current Handoffs

| File | Audience | What It Covers |
|------|----------|----------------|
| **PRIMALSPRING_V0925_EVOLUTION_HANDOFF_MAY09_2026.md** | Upstream primals + spring teams + products | Comprehensive evolution handoff: primal consumption, upstream debt, per-spring targets, NUCLEUS composition, Neural API deployment, downstream guidance. |
| **PRIMALSPRING_V0925_UNIBIN_EUKARYOTIC_HANDOFF_MAY09_2026.md** | All teams | UniBin eukaryotic evolution: cell model, CLI surface, two-tier validation, certification organelle, scenario registry. |
| **PHASE60_COMPLETION_HANDOFF_MAY09_2026.md** | Downstream products + spring teams | Phase 60 completion: 13/13 primals clean, composition patterns for NUCLEUS via Neural API. |
| **INTERSTADIAL_FOSSILIZATION_HANDOFF.md** | Spring teams | Interstadial fossilization patterns: what to preserve, how to date, provenance READMEs. |

### Archived (resolved, in `archive/`)

Phase-specific handoffs from May 7-9 that are now absorbed into the current
handoffs above: phase 60 deep debt, security gate, ionic tokens, cross-spring
parity audit, sovereignty. Git history preserves full provenance.

---

## Current Ecosystem State

- **13/13 primals** building standalone, distributed via plasmidBin genomeBin
  (Tier 1: x86_64, aarch64, armv7 — 40+ release assets)
- **Zero open upstream gaps** — 13/13 primals at zero debt, JH-11 resolved (May 10), all sovereignty horizons advancing
- **413 registered capability methods** across 84 domains (including `auth.*`, `nautilus.*`, `game.*`, ionic token methods)
- **13/13 BTSP Phase 3 FULL AEAD**, 13/13 default `127.0.0.1`
- **RootPulse commit workflow** fully executable (6/6 phases)
- **NestGate content-addressed storage** live (8 `content.*` methods)
- **Graph method validator** — 0 primal drift, 91 spring-domain advisory
- **sourDough v0.2.0** scaffold generates ecosystem-compliant primals

## Key References (outside wateringHole)

| What | Where |
|------|-------|
| Gap registry | `docs/PRIMAL_GAPS.md` |
| Capability registry | `config/capability_registry.toml` (413 methods, zero drift) |
| Method gate CI | `tools/check_method_gate.sh` |
| Method string validator | `tools/check_method_strings.sh` |
| Graph method validator | `tools/check_graph_methods.sh` |
| Experiment tracks | `experiments/` (89 experiments, 20 tracks) |
| Deploy graphs | `graphs/` (77 TOMLs) |
| Checksum tool | `tools/regenerate_checksums.sh` |
| Binary fetch script | `tools/fetch_primals.sh` |
| NUCLEUS launcher | `tools/composition_nucleus.sh` |
| Composition library | `tools/nucleus_composition_lib.sh` |
| Fossil record | [fossilRecord repo](https://github.com/ecoPrimals/fossilRecord) (consolidated May 12, 2026) |

---

## Fossil Record

Historical handoffs are preserved in the [fossilRecord repository](https://github.com/ecoPrimals/fossilRecord) under `springs/primalSpring/`:
- `wateringHole_phase56_apr2026/` — v0.1.0 through Phase 56 (66+ files)
- `wateringHole_phase58_59_may2026/` — Phase 58–59 handoffs (6 files)

Git history in this repo retains full provenance at their original paths.
A local redirect stub exists at `fossilRecord/README.md`.
