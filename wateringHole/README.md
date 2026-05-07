# wateringHole — primalSpring Ecosystem Guidance

**Version**: 0.9.25 (Phase 60)
**Last Updated**: May 7, 2026
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
| **UPSTREAM_CROSSTALK_AND_DOWNSTREAM_ABSORPTION.md** | Primal teams + spring teams | How primals interact without coupling (JSON-RPC, first-byte peek, capability wire, socket naming). How springs absorb NUCLEUS patterns. sourDough graduation standard. |
| **CRYPTO_CONSUMPTION_HIERARCHY.md** | Primal teams + spring teams | Crypto posture per primal role: key acquisition patterns, bonding hierarchy, Phase 3 convergence. |
| **PLASMINBIN_DEPOT_PATTERN.md** | All consumers | How to fetch primal binaries from plasmidBin GitHub Releases. |
| **PHASE58_COMPOSITION_HANDOFF_MAY03_2026.md** | All teams | Phase 58: skunkBat as 13th primal, guidestone hardening, plasmidBin CI hub. |
| **UPSTREAM_ABSORPTION_MAY06_2026.md** | All teams | 13/13 pull: toadStool, skunkBat, Provenance Trio, Phase 3 interop. |
| **FOUNDATION_ABSORPTION_MAY06_2026.md** | Spring + garden teams | Foundation layer: graph schema extensions, exp107 sediment pipeline. |
| **UPSTREAM_BLURBS_PHASE59_MAY06_2026.md** | Primal teams | Per-primal debt: 13/13 CLEAN, PG-55–PG-59 RESOLVED. |
| **DOWNSTREAM_HANDOFF_PHASE59_MAY06_2026.md** | projectNUCLEUS + springs | Phase 59 security convergence handoff. |
| **PHASE60_DEEP_DEBT_SOVEREIGNTY_HANDOFF_MAY07_2026.md** | All teams | Phase 60: deep debt evolution (9 tasks), 14/14 sovereignty gaps absorbed, registry 290→366, graph validator rewrite. |
| **DOWNSTREAM_HANDOFF_PHASE60_MAY07_2026.md** | Springs + gardens | Phase 60 downstream: registry patterns, content-addressed storage, RootPulse workflow, shader rewire, notebook pipeline, deep debt patterns. |
| This README | Everyone | Index and context. |

---

## Current Ecosystem State

- **13/13 primals** building standalone, distributed via plasmidBin genomeBin
  (Tier 1: x86_64, aarch64, armv7 — 40+ release assets)
- **Zero open P1 upstream gaps** — all 14 sovereignty gaps RESOLVED
- **366 registered capability methods** across 50+ domains
- **13/13 BTSP Phase 3 FULL AEAD**, 13/13 default `127.0.0.1`
- **RootPulse commit workflow** fully executable (6/6 phases)
- **NestGate content-addressed storage** live (8 `content.*` methods)
- **Graph method validator** — 0 primal drift, 91 spring-domain advisory
- **sourDough v0.2.0** scaffold generates ecosystem-compliant primals

## Key References (outside wateringHole)

| What | Where |
|------|-------|
| Gap registry | `docs/PRIMAL_GAPS.md` |
| Capability registry | `config/capability_registry.toml` (366 methods) |
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

All historical handoffs (66+ files, v0.1.0 through v0.9.24) are preserved in
`fossilRecord/wateringHole_phase56_apr2026/`. Git history retains full
provenance at their original paths.
