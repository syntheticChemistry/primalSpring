# wateringHole — primalSpring Ecosystem Guidance

**Version**: 0.9.24 (Phase 58)
**Last Updated**: May 6, 2026
**License**: AGPL-3.0-or-later  

---

## What This Is

The wateringHole is primalSpring's outward-facing guidance surface for upstream
primal teams and downstream spring/garden consumers. It defines the patterns
that make the ecosystem composable.

Six documents. Nothing else belongs here — historical handoffs live in
`fossilRecord/`.

---

## Documents

| File | Audience | What It Covers |
|------|----------|----------------|
| **UPSTREAM_CROSSTALK_AND_DOWNSTREAM_ABSORPTION.md** | Primal teams + spring teams | How primals interact without coupling (JSON-RPC, first-byte peek, capability wire, socket naming). How springs absorb NUCLEUS patterns. sourDough graduation standard. |
| **CRYPTO_CONSUMPTION_HIERARCHY.md** | Primal teams + spring teams | Crypto posture per primal role: key acquisition patterns (self-derive vs Tower-provided), bonding escalation hierarchy, composition contexts, recommended AEAD posture. Phase 3 convergence standard. |
| **PLASMINBIN_DEPOT_PATTERN.md** | All consumers | How to fetch primal binaries from plasmidBin GitHub Releases. `fetch_primals.sh`, `ECOPRIMALS_PLASMID_BIN` env, XDG cache, checksum verification. The standard consumer pattern. |
| **PHASE58_COMPOSITION_HANDOFF_MAY03_2026.md** | All teams | Phase 58 evolution: skunkBat as 13th NUCLEUS primal, guidestone hardening (BTSP alias routing, flex keys, cell health), plasmidBin CI hub architecture, composition patterns for downstream, remaining debt. |
| **UPSTREAM_ABSORPTION_MAY06_2026.md** | All teams | 13/13 primals pulled: toadStool S222-S223, skunkBat port fix (9750→9140), fieldMouse reclassification, Provenance Trio Gap 9 resolved, Phase 3 interop confirmed across ecosystem. deploy/validation.rs split. |
| This README | Everyone | Index and context. |

---

## Current Ecosystem State

- **13/13 primals** building standalone, distributed via plasmidBin genomeBin
  (Tier 1: x86_64, aarch64, armv7 — 40+ release assets)
- **skunkBat** wired as 13th NUCLEUS primal (meta-tier, defense/recon) — Phase 58
- **Zero HIGH blockers** — BTSP Phase 3 converged (13/13 FULL AEAD, May 2, 2026)
- **plasmidBin CI hub** — sole paid Actions repo, per-primal concurrency,
  signing roadmap documented
- **All primals** have `ci.yml` (lean single-job) + `notify-plasmidbin.yml`
- **sourDough v0.2.0** scaffold generates ecosystem-compliant primals
  (server crate, CI, deny.toml, capability wire, first-byte peek, socket naming)
- **Guidestone hardened** — BTSP alias routing, flex key resolution, desktop
  cell health, Squirrel reconnect-on-failed-probe

## Key References (outside wateringHole)

| What | Where |
|------|-------|
| Gap registry | `docs/PRIMAL_GAPS.md` |
| Experiment tracks | `experiments/` (84 experiments, 18 tracks) |
| Deploy graphs | `graphs/` (71 TOMLs) |
| Binary fetch script | `tools/fetch_primals.sh` |
| NUCLEUS launcher | `tools/composition_nucleus.sh` |
| Composition library | `tools/nucleus_composition_lib.sh` |
| Fossil record | `fossilRecord/` (archived handoffs, stale code, historical docs) |

---

## Fossil Record

All historical handoffs (66 files, v0.1.0 through v0.9.24) are preserved in
`fossilRecord/wateringHole_phase56_apr2026/`. Git history retains full
provenance at their original paths.
