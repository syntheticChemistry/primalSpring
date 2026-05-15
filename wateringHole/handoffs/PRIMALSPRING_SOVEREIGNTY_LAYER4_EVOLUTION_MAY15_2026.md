# primalSpring Sovereignty Layer 4 Evolution — May 15, 2026

## Summary

primalSpring internalized the sovereignty patterns pioneered by projectNUCLEUS
(membrane composition, content-aware routing, calibrate-shadow-cutover protocol)
into its validation framework. This enables upstream structural validation of
the VPS inner membrane before downstream deployment.

## What Shipped

### New Validation Track: Sovereignty (10th track, 35 scenarios total)

| Scenario | Tier | What it validates |
|----------|------|-------------------|
| `membrane-composition` | Rust | Structural: graph metadata, tower node completeness, bonding policy, telemetry contract |
| `sovereignty-parity` | Both | Structural: routing config schema (backends, rules, trust tiers, telemetry). Live: membrane boundary health |
| `content-sovereignty` | Live | Content pipeline through sovereign routing, BLAKE3 round-trip, trust-tier alignment, SkunkBat audit correlation |

### New Deploy Graph: `graphs/membrane/tower_membrane.toml`

VPS membrane composition: 5 nodes (biomeOS + BearDog + Songbird + SkunkBat +
NestGate cache), 3-channel architecture:

- Channel 1 (Signal): UDS primal-to-primal IPC on VPS
- Channel 2 (Relay): BTSP tunnel VPS-to-gate encrypted relay
- Channel 3 (Surface): TLS public HTTPS on membrane.primals.eco

`composition_model = "membrane"` (distinct from `nucleus_complete.toml`'s `"nucleated"`).

### Canonical Routing Schema: `config/routing_config_reference.toml`

primalSpring now owns the content-aware routing schema. Downstream membrane
deployments (projectNUCLEUS `deploy/routing_config.toml`) must conform.

Schema defines:
- 4 backend types: `btsp_tunnel`, `local_filesystem`, `songbird_p2p`, `http_proxy`
- Match predicates: `path_prefix`, `path_regex`, `host`, `content_type`, `header`, `min_size_mb`
- 4 trust tiers: covalent (all), ionic (scoped), metallic (compute), weak (public)
- Telemetry: shadow mode, cutover gate days (>=7), SkunkBat correlation

## What This Enables

1. **Upstream validation before deployment**: `primalspring validate --track sovereignty`
   catches membrane graph regressions before they reach VPS.
2. **Schema-driven routing**: downstream teams validate their `routing_config.toml`
   against primalSpring's canonical schema rather than ad-hoc checks.
3. **Sovereignty parity protocol**: the calibrate-shadow-cutover pipeline has
   measurable gates (7-day cutover threshold, shadow telemetry, SkunkBat audit)
   structurally validated by primalSpring.

## 4-Layer Model (from `PRIMAL_VS_SOVEREIGNTY_GOALS.md`)

| Layer | Owner | primalSpring Validation |
|-------|-------|------------------------|
| 1. Primal Capabilities | Primal teams | `composition-parity`, `domain-contract-sweep` (441 methods) |
| 2. Security Validation | BearDog/SkunkBat | `dark-forest-gate`, `bearer-token-auth` (5-pillar) |
| 3. Sovereignty Deployment | primalSpring + projectNUCLEUS | `membrane-composition`, `sovereignty-parity` |
| 4. Sovereign Composition | Product teams | `content-sovereignty` + full pipeline validation |

## Test Results

- 673 lib tests pass (0 failures)
- All 7 new scenario-specific tests pass
- All 5 meta-tests pass (registry count, no duplicates, all tracks covered,
  all Rust-tier pass, valid provenance dates)
- `s_membrane_composition` structural: 0 failures (all 4 pillars green)
- `s_sovereignty_parity` structural: 0 failures (schema + trust tiers green)

## Files Changed

**New (5):**
- `ecoPrimal/src/validation/scenarios/s_membrane_composition.rs`
- `ecoPrimal/src/validation/scenarios/s_sovereignty_parity.rs`
- `ecoPrimal/src/validation/scenarios/s_content_sovereignty.rs`
- `graphs/membrane/tower_membrane.toml`
- `config/routing_config_reference.toml`

**Modified (12):**
- `registry.rs` — Sovereignty track variant
- `mod.rs` — 3 new scenario registrations, count 32->35
- `ARCHITECTURE.md` — membrane diagram, sovereignty track, evolution path
- `CHANGELOG.md` — Wave 15 entry
- `CONTEXT.md` — updated counts
- `README.md` — updated counts
- `DOWNSTREAM_PATTERN_GUIDE.md` — sovereignty validation patterns section
- `PRIMAL_GAPS.md` — Layer 4 reframed as Sovereignty Composition
- `VALIDATION_TIERS.md` — updated scenario count
- `CROSS_SPRING_PARITY_SCORECARD.md` — updated primalSpring row
- `wateringHole/README.md` — updated counts, routing config and membrane refs

## Downstream Action Items

- **projectNUCLEUS**: validate `deploy/routing_config.toml` conforms to
  `config/routing_config_reference.toml` schema (backend types, trust tiers)
- **biomeOS**: `composition.deploy(graph)` should handle `composition_model = "membrane"`
  as a recognized model (VPS subset vs full nucleated)
- **CATHEDRAL**: lithoSpore validation across Windows/Linux/USB recreation should
  exercise membrane-aware content resolution when sovereign hosting is active
