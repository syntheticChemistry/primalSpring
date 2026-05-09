# Fossilized Experiments — Pre-Interstadial (May 9, 2026)

Snapshot of all 85 experiment source directories taken immediately before the
Interstadial rewire that evolved every experiment to the modern primalStructured
pattern (`CompositionContext` + phased `v.section()` + v0.9.25).

## What changed in the rewire

| Old pattern | Modern replacement |
|---|---|
| `discover_primal()` / `discover_by_capability()` | `CompositionContext::from_live_discovery_with_fallback()` |
| `PrimalClient::connect()` + manual RPC | `ctx.call(capability, method, params)` |
| `AtomicHarness` live spawning | Removed — composition is live via biomeOS |
| Flat check lists | `v.section("Phase N: ...")` + extracted `fn phase_*()` |
| `check_or_skip` closure pattern | `match ctx.call() { Err(e) if e.is_connection_error() => check_skip }` |
| Package version 0.8.0 | 0.9.25 |

## Why fossilize

These patterns represent the evolutionary history of primalSpring's experiment
infrastructure from Phase 0 (initial scaffolding) through Phase 58 (skunkBat
NUCLEUS). The old discovery, client, and harness APIs served their purpose
during the rapid primal-wiring period. With all 13 NUCLEUS primals at zero
upstream debt and the CompositionContext abstracting all IPC, the old patterns
are superseded.

The fossil record preserves them for reference, pattern archaeology, and
documentation of the ecosystem's evolution from direct socket wiring to
capability-based composition.

## Structure

Each subdirectory mirrors the experiment's `src/` tree at the moment of
fossilization. Only source code is preserved — `Cargo.toml` and build
artifacts are not included since the code patterns are what matter.

**License**: AGPL-3.0-or-later
