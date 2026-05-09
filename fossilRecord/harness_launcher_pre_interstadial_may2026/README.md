# Fossilized Harness + Launcher — Pre-Interstadial (May 9, 2026)

Snapshot of the primal-spawning infrastructure taken at the interstadial
transition. These modules spawn NUCLEUS primals as child processes — a pattern
superseded by plasmidBin ecoBin deployment via biomeOS composition.

## What was archived

| Directory | Content |
|---|---|
| `harness/` | `AtomicHarness`, `RunningAtomic` — spawns primals in topological order, manages sockets, tears down on drop |
| `launcher/` | `spawn_primal`, `spawn_biomeos`, binary discovery, socket nucleation, profile loading |
| `tests/` | Integration tests (`server_ecosystem*.rs`) that used `AtomicHarness::start()` to validate live compositions |

## Why fossilize

The spawn-from-harness pattern served primalSpring during the rapid primal-wiring
period (Phase 0–58). With all 13 NUCLEUS primals at zero upstream debt and
biomeOS providing composition deployment via Neural API, the correct validation
path is:

1. Deploy composition from plasmidBin ecoBins (via biomeOS or `plasmidBin deploy`)
2. Validate via `CompositionContext::from_live_discovery_with_fallback()`
3. Call primals by capability through `ctx.call(cap, method, params)`

The harness and launcher remain in the library with `#[deprecated]` attributes
for backward compatibility, but new validation must use live NUCLEUS composition.

## Architecture reference

See `docs/VALIDATION_TIERS.md` for the two-tier validation architecture
(Rust/lib tier vs Live NUCLEUS tier).

**License**: AGPL-3.0-or-later
