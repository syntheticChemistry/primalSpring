# primalSpring v0.8.0 — Ecosystem Debt Resolution Handoff

**Date:** 2026-03-29
**From:** primalSpring v0.8.0 (Phase 23)
**To:** All primal teams, spring teams, gen4 product teams
**Scope:** Debt resolution findings, upstream fixes delivered, new standards, absorption opportunities

---

## What Happened

primalSpring conducted a comprehensive ecosystem audit that traced code paths
across BearDog, Songbird, and biomeOS. The audit reclassified 11 initial
findings into proper categories and executed all actionable debt.

### Reclassification Summary

| Finding | Initial Classification | Actual Classification |
|---------|----------------------|----------------------|
| Two beacon crypto models | Critical (disconnected) | Intentional design — separate trust models |
| SONGBIRD_DARK_FOREST no effect | Critical | Deployment wiring — 3 struct fields |
| derive_lineage_beacon_key unregistered | Critical | Deployment wiring — 1 method_list entry |
| Zero-byte default seed | Critical | Defensive coding — validation check |
| verify_lineage incomplete | Critical | Intentional design — challenge-response protocol |
| No wrong-family test | High | Already covered in BearDog unit tests |
| exp086 empty verify_lineage | Medium | Real test gap — fixed |
| birdsong.encrypt untested | High (primalSpring) | Covered in BearDog unit tests |
| beacon.encrypt untested | High (primalSpring) | Covered in BearDog unit tests |
| Dead Songbird methods | Medium | Not dead — used by other entrypoints |
| Federation misleading label | Medium | Documentation fix — delivered |

Full details: `PRIMALSPRING_V080_GAP_MAP_MAR29_2026.md`

---

## Upstream Fixes Delivered

### BearDog (all committed and pushed)

1. **`genetic.derive_lineage_beacon_key` registered** in `method_list.rs` (method count 92→93)
2. **Empty/zero/short lineage seed rejection** — `handle_derive_lineage_beacon_key` now validates seed length (≥16 bytes), rejects all-zeros, rejects empty. 3 new negative tests.
3. **`federation.verify_family_member` label fixed** — `genetic_lineage_hkdf` → `family_id_equality` (accurate for string comparison)
4. **`encryption.encrypt/decrypt` HSM labeling removed** — docs, logs, and method descriptions no longer claim "HSM-backed" (software SHA-256 KDF)

### Songbird (all committed and pushed)

5. **Dark Forest env vars wired** — `dark_forest_enabled`, `accept_legacy_format`, `dual_broadcast` now read from `env_config` in all 3 `BirdSongConfig` struct literals in `discovery_startup.rs` (was using `..Default::default()`)

### biomeOS (committed, not yet pushed)

6. **`eprintln!` → `tracing::warn!`** in `capability_domains.rs` test

---

## New Artifacts

### Deploy Graph
- `graphs/spring_validation/crypto_negative_validate.toml` — 9-node graph validating security rejection paths

### primalSpring Changes
- `exp086` now performs full generate-then-verify lineage round-trip with positive + negative tests
- `ipc::methods::genetic::GENERATE_LINEAGE_PROOF` added
- `exp086` `Cargo.toml` now depends on `base64 = "0.22"`

### Standards (in `infra/wateringHole/`)
- `COMPOSITION_PATTERNS.md` — deploy graph formats, niche YAML, launch profiles, socket discovery
- `SPOREGARDEN_DEPLOYMENT_STANDARD.md` — BYOB model, esotericWebb reference, env contract
- `PRIMALSPRING_COMPOSITION_GUIDANCE.md` — synced from primalSpring
- Per-primal debt handoffs: BearDog, Songbird, biomeOS

### wateringHole Housekeeping
- `GLOSSARY.md` — 6 new composition terms
- `README.md` — new "Composition & Deployment" section, handoff count 70→88

---

## What Primal Teams Should Know

### BearDog Team
- Your method count is now 93 (was 92). Update any hardcoded assertions.
- Empty seed rejection is a behavioral change — callers that relied on zero-byte default will now get errors. This is the correct behavior.
- See `BEARDOG_TEAM_DEBT_HANDOFF_MAR29_2026.md` for full categorized debt.

### Songbird Team
- `SONGBIRD_DARK_FOREST=true` now actually enables Dark Forest mode. Test your deployment configs.
- See `SONGBIRD_TEAM_DEBT_HANDOFF_MAR29_2026.md` for full categorized debt.

### biomeOS Team
- The `crypto_negative_validate.toml` graph is designed to run against a live Tower. Add it to your validation suite.
- See `BIOMEOS_TEAM_DEBT_HANDOFF_MAR29_2026.md` for full categorized debt.

---

## What Spring Teams Should Know

### Absorption Opportunities

1. **Negative validation graph pattern** — `crypto_negative_validate.toml` demonstrates how to encode negative tests (expect = "invalid", expect = "error") in deploy graphs. Springs with security-sensitive operations should create equivalent graphs.

2. **`ipc::methods` expansion** — 16 domain modules now (crypto, birdsong, genetic, secrets, storage, game, health, capabilities, provenance, coordination, graph, lifecycle, mcp, discovery, network, compute). Springs should use these constants instead of hardcoding method strings.

3. **Generate-then-verify pattern** — exp086 demonstrates the correct pattern for lineage verification: generate proof with one RPC, verify with another, then verify with wrong seed. Springs doing genetic operations should follow this pattern.

4. **Composition standards** — `COMPOSITION_PATTERNS.md` documents both graph formats. Springs creating deploy graphs should use `[[graph.node]]` format (canonical for new compositions).

### Cross-Spring Evolution Pressure

| Source | Insight | Absorbing Springs |
|--------|---------|-------------------|
| Two beacon models are intentional | `beacon.*` (HKDF) and `birdsong.*` (family_id) serve different trust models | All springs using Dark Forest discovery |
| `verify_lineage` is challenge step 1 | Songbird's verify_lineage generates a challenge; BearDog's does full BLAKE3 verification | Springs doing cross-gate trust |
| Socket discovery 8-step order | env → capability → XDG → abstract → /tmp → registry → Neural API → TCP | All springs doing primal discovery |

---

## What gen4 Product Teams Should Know

- `SPOREGARDEN_DEPLOYMENT_STANDARD.md` is now the canonical reference for product composition
- `COMPOSITION_PATTERNS.md` documents both graph schema variants your deploy graphs may use
- esotericWebb's `[[nodes]]` format is valid for multi-arch deployments; new single-gate graphs should use `[[graph.node]]`
- Launch profiles contract is documented with all fields

---

## Metrics

| Metric | Before (v0.7.0) | After (v0.8.0) |
|--------|-----------------|-----------------|
| Version | 0.7.0 | 0.8.0 |
| Experiments | 67 | 67 |
| Tests | 413 | 413 |
| Deploy graphs | 59 | 60 |
| Spring validation graphs | 7 | 8 |
| wateringHole handoffs (infra) | 85 | 89 |
| Upstream fixes delivered | 0 | 6 (3 BearDog, 1 Songbird, 1 biomeOS, 1 primalSpring) |
| Standards documents created | 0 | 2 |
