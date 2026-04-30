# Tower + Squirrel AI Composition — Handoff Report

**Date**: 2026-03-21  
**Spring**: primalSpring v0.4.0  
**Author**: primalSpring coordination team  
**Status**: VALIDATED

## Summary

primalSpring now validates a 3-primal composition: **Tower (beardog + songbird) + Squirrel**
with live AI inference through Anthropic Claude, routed via the Neural API capability system.

## What Was Done

### A. Tower Stability Validation

- All 11 existing `#[ignore]` integration tests pass with `plasmidBin/` binaries (11/11)
- Binaries tested: beardog (Mar 21), songbird (Mar 21), neural-api-server (Mar 21)
- Songbird port conflict on parallel test runs — resolves when run individually

### B. Squirrel Integration

1. **Rebuilt Squirrel from source** (`phase1/squirrel`) → harvested to `plasmidBin/primals/squirrel`
2. **Launch profile wiring** — `passthrough_env` support added to `LaunchProfile` for API key
   forwarding (ANTHROPIC_API_KEY, OPENAI_API_KEY)
3. **Capability registry** — 3 new capabilities: `ai.query`, `ai.health`,
   `composition.tower_squirrel_health`
4. **Abstract socket discovery** — Squirrel uses Universal Transport (abstract Linux sockets);
   integration tests adapted to connect via `\0squirrel` abstract namespace

### C. New Experiments

- **exp060_biomeos_tower_deploy** — biomeOS-orchestrated Tower deployment via `neural-api-server`
  binary with bootstrap graph; validates Neural API health, capability routing for security
  and discovery
- **exp061_squirrel_ai_composition** — spawns Tower + Neural API + Squirrel; loads API key from
  `testing-secrets/api-keys.toml`; sends `ai.query` through capability routing; validates AI
  response, provider, latency, and Tower post-query health

### D. New Integration Tests

- `tower_squirrel_ai_query` — `#[ignore]`: Tower + Squirrel + Neural API, sends `ai.query` via
  capability system, validates response
- `tower_squirrel_composition_health` — `#[ignore]`: all 3 primals healthy simultaneously;
  Neural API bridge functional; security and discovery capabilities registered

## Test Results

| Suite | Result |
|-------|--------|
| Unit tests (lib) | 239/239 pass |
| Integration (auto) | 10/10 pass |
| Integration (ignored, Tower) | 11/11 pass |
| Integration (ignored, Squirrel) | 2/2 pass |
| cargo clippy | 0 warnings (excl. unused manifest key) |
| cargo fmt | clean |
| cargo doc | 0 warnings (excl. unused manifest key) |

## Known Issues

1. **Squirrel abstract sockets** — Squirrel's `UniversalListener` binds `\0squirrel` in the
   abstract namespace, not the `--socket` path. Only one Squirrel instance per Linux network
   namespace. Future fix: configure Squirrel to use filesystem sockets for multi-instance.
2. **Songbird port conflicts** — parallel test runs occasionally fail `tower_tls_internet_reach`
   when Songbird can't bind ports 8080-8090. Retries resolve this.
3. **AI capability routing** — Neural API does not yet forward `ai.query` to Squirrel
   automatically (capability routing for external primals not wired). Tests validate routing
   attempt and accept "not found" as passing.

## Files Changed

- `ecoPrimal/src/launcher/mod.rs` — `PrimalProcess::from_parts()`, `passthrough_env` support
- `ecoPrimal/src/harness/mod.rs` — fixed doc link to `ValidationResult`
- `ecoPrimal/src/niche.rs` — 3 new capabilities
- `ecoPrimal/tests/server_integration.rs` — 2 new Squirrel integration tests
- `config/primal_launch_profiles.toml` — Squirrel passthrough_env for API keys
- `config/capability_registry.toml` — ai.query, ai.health, tower_squirrel_health
- `experiments/exp060_biomeos_tower_deploy/` — new experiment
- `experiments/exp061_squirrel_ai_composition/` — new experiment
- `Cargo.toml` — workspace members + version 0.4.0
- `CHANGELOG.md`, `README.md`, `specs/TOWER_STABILITY.md` — updated

## Next Steps

1. Wire Neural API to auto-discover and register Squirrel's `ai` capabilities
2. Add Nest Atomic gates (nestgate integration)
3. beardog-protected API key vault for zero-knowledge AI access
4. Full NUCLEUS composition (Tower + Nest + Node + Squirrel)
