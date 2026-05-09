# wateringHole — primalSpring Ecosystem Guidance

**Version**: 0.9.25 (Phase 60)
**Last Updated**: May 9, 2026
**License**: AGPL-3.0-or-later  

---

## What This Is

The wateringHole is primalSpring's outward-facing guidance surface for upstream
primal teams and downstream spring/garden consumers. It defines the patterns
that make the ecosystem composable.

Historical handoffs live in `fossilRecord/`.

---

## Documents

| File | Audience | What It Covers |
|------|----------|----------------|
| **CRYPTO_CONSUMPTION_HIERARCHY.md** | Primal teams + spring teams | Crypto posture per primal role: key acquisition patterns, bonding hierarchy, Phase 3 convergence. |
| **PLASMINBIN_DEPOT_PATTERN.md** | All consumers | How to fetch primal binaries from plasmidBin GitHub Releases. |
| **METHOD_GATE_STANDARD.md** | All primal teams | JH-0 ecosystem standard: pre-dispatch capability authorization, exempt whitelist, error codes (-32000/-32001/-32002), enforcement modes, peer credential extraction, adoption path. |
| **PHASE60_DEEP_DEBT_SOVEREIGNTY_HANDOFF_MAY07_2026.md** | All teams | Phase 60: deep debt evolution (9 tasks), 14/14 sovereignty gaps absorbed, registry 290→384, graph validator rewrite. |
| **DOWNSTREAM_HANDOFF_PHASE60_MAY07_2026.md** | Springs + gardens | Phase 60 downstream: registry patterns, content-addressed storage, RootPulse workflow, shader rewire, notebook pipeline, deep debt patterns. |
| **DOWNSTREAM_HANDOFF_PHASE60_SECURITY_GATE_MAY07_2026.md** | Springs + gardens + projectNUCLEUS | Phase 60 security gate: method gate pattern, `auth.*` methods, `PermissionDenied` handling, `SeedConfig`, composition patterns for NUCLEUS deployment via Neural API. |
| **DOWNSTREAM_HANDOFF_PHASE60_IONIC_TOKENS_MAY08_2026.md** | Springs + gardens + projectNUCLEUS | JH-1 ionic tokens (BearDog Ed25519), GAP-11 18/18 closure (nautilus sessions, ml.mlp_train), token flow in compositions, registry 389 methods. |
| **CROSS_SPRING_PARITY_HANDOFF_MAY08_2026.md** | All spring teams + downstream | Cross-spring composition parity audit: per-spring scorecard (8 axes), evolution targets, universal gaps, 8,737+ total tests across 8 springs, 112 deploy graphs. |
| **PRIMALSPRING_V0925_EVOLUTION_HANDOFF_MAY09_2026.md** | Upstream primals + spring teams + products | Comprehensive evolution handoff: primal consumption review, upstream debt, per-spring absorption targets, NUCLEUS composition patterns, Neural API deployment, downstream product guidance, lessons learned. |
| **PHASE60_COMPLETION_HANDOFF_MAY09_2026.md** | Downstream products + spring teams | Phase 60 completion: 13/13 primals clean, resolved gaps table, composition patterns for NUCLEUS via Neural API, per-spring absorption targets, quality gate. |
| **PRIMALSPRING_V0925_PHASE60_DEEP_DEBT_CLOSURE_HANDOFF_MAY09_2026.md** | Delta spring teams + products | Phase 60+ deep debt closure: graph `by_capability` alignment (7 graphs), integration test decomposition (2→5 files), `validate_all` broadened, capability naming harmonized, zero `#[allow]` without reason. |
| This README | Everyone | Index and context. |

---

## Current Ecosystem State

- **13/13 primals** building standalone, distributed via plasmidBin genomeBin
  (Tier 1: x86_64, aarch64, armv7 — 40+ release assets)
- **Zero open upstream gaps** — 13/13 primals at zero debt (Phase 60 complete)
- **389 registered capability methods** across 82 domains (including `auth.*`, `nautilus.*`, ionic token methods)
- **13/13 BTSP Phase 3 FULL AEAD**, 13/13 default `127.0.0.1`
- **RootPulse commit workflow** fully executable (6/6 phases)
- **NestGate content-addressed storage** live (8 `content.*` methods)
- **Graph method validator** — 0 primal drift, 91 spring-domain advisory
- **sourDough v0.2.0** scaffold generates ecosystem-compliant primals

## Key References (outside wateringHole)

| What | Where |
|------|-------|
| Gap registry | `docs/PRIMAL_GAPS.md` |
| Capability registry | `config/capability_registry.toml` (389 methods) |
| Method gate CI | `tools/check_method_gate.sh` |
| Method string validator | `tools/check_method_strings.sh` |
| Graph method validator | `tools/check_graph_methods.sh` |
| Experiment tracks | `experiments/` (85 experiments, 19 tracks) |
| Deploy graphs | `graphs/` (74 TOMLs) |
| Checksum tool | `tools/regenerate_checksums.sh` |
| Binary fetch script | `tools/fetch_primals.sh` |
| NUCLEUS launcher | `tools/composition_nucleus.sh` |
| Composition library | `tools/nucleus_composition_lib.sh` |
| Fossil record | `fossilRecord/` (archived handoffs, stale code, historical docs) |

---

## Fossil Record

Historical handoffs are preserved in `fossilRecord/`:
- `wateringHole_phase56_apr2026/` — v0.1.0 through Phase 56 (66+ files)
- `wateringHole_phase58_59_may2026/` — Phase 58–59 handoffs (6 files)

Git history retains full provenance at their original paths.
