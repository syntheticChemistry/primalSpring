# Interstadial Fossilization Handoff

**Date**: May 9, 2026 — Phase 60+ INTERSTADIAL
**From**: primalSpring
**To**: All delta springs, projectNUCLEUS, primal teams

## Summary

primalSpring has completed its interstadial transition: all 89 experiments and
the core ecoPrimal library now route through `CompositionContext` for primal
interactions. Direct-spawn infrastructure (`AtomicHarness`, `spawn_primal`) and
low-level probing (`probe_primal`, `validate_composition`) are `#[deprecated]`
with forwarding notes pointing to the modern patterns. Pre-interstadial code is
preserved in the [fossilRecord repository](https://github.com/ecoPrimals/fossilRecord) under `springs/primalSpring/`.

This handoff defines the **two-tier validation architecture** and provides
per-spring inventories and checklists for replicating the transition.

---

## Two-Tier Validation Architecture

### Tier 1: Rust Validation (Library)

Structural checks, type systems, graph parsing, protocol validation, tolerance
constants. These are pure Rust analytical validation — no IPC needed. Library
interactions with primal crates are acceptable here.

**Examples**: graph TOML parsing, bonding policy validation, capability registry
checks, BTSP protocol serialization round-trips, tolerance assertions.

**Pattern**: `use primalspring::{ deploy, bonding, validation, ... };`

### Tier 2: Live NUCLEUS (Primal IPC)

Any validation that touches primal **behavior** MUST go through live IPC to
deployed ecoBins from plasmidBin. No spawning primals from harness, no
`PrimalClient` to spawned processes, no direct primal crate imports for
behavioral testing.

**Pattern**:
```rust
let mut ctx = CompositionContext::from_live_discovery_with_fallback();
let result = ctx.call("capability", "method.name", params)?;
```

**Requirements**:
- Primals deployed as ecoBins in plasmidBin
- biomeOS orchestrates composition deployment via Neural API
- Springs call primals by **capability**, not by identity
- Discovery uses the 5-tier escalation hierarchy (Songbird → Neural API → UDS → Registry → TCP)

### What belongs where

| Validation Type | Tier | Transport |
|---|---|---|
| Graph structure parsing | 1 (Rust) | Library |
| TOML schema validation | 1 (Rust) | Library |
| Bonding policy checks | 1 (Rust) | Library |
| Capability registry sync | 1 (Rust) | Library |
| Tolerance assertions | 1 (Rust) | Library |
| Primal health checks | 2 (Live) | IPC via CompositionContext |
| Capability probing | 2 (Live) | IPC via CompositionContext |
| Cross-atomic composition | 2 (Live) | IPC via CompositionContext |
| Bearer token auth flows | 2 (Live) | IPC via CompositionContext |
| Neural API orchestration | 2 (Live) | IPC via CompositionContext |

---

## What primalSpring Fossilized

### AtomicHarness + Launcher (deprecated spawn infrastructure)

| Item | Status |
|---|---|
| `ecoPrimal/src/harness/mod.rs` — `AtomicHarness`, `RunningAtomic` | `#[deprecated]` |
| `ecoPrimal/src/launcher/spawn.rs` — `spawn_primal` | `#[deprecated]` |
| `ecoPrimal/src/launcher/biomeos.rs` — `spawn_biomeos` | `#[deprecated]` |
| `ecoPrimal/src/composition/context.rs` — `from_running()` | `#[deprecated]` |
| Fossil record | [fossilRecord](https://github.com/ecoPrimals/fossilRecord) → `springs/primalSpring/harness_launcher_pre_interstadial_may2026/` |

### Coordination Probes (PrimalClient-based)

| Item | Status |
|---|---|
| `coordination::probe_primal` | `#[deprecated]` |
| `coordination::check_capability_health` | `#[deprecated]` |
| `coordination::validate_composition` | `#[deprecated]` |
| `coordination::validate_composition_by_capability` | `#[deprecated]` |

### Experiments

| Item | Status |
|---|---|
| 85 pre-interstadial experiment sources | [fossilRecord](https://github.com/ecoPrimals/fossilRecord) → `springs/primalSpring/experiments_pre_interstadial_may2026/` |
| All 89 experiments rewired to `CompositionContext` | Complete |
| `exp105 world.rs` — last `PrimalClient` holdout | Rewired |

---

## Per-Spring Fossilization Inventory

These primal-based subdirectories in sibling springs represent pre-absorption
shader evolution and primal-specific code. Each spring should fossilize these
to its own fossilRecord (local or [shared repo](https://github.com/ecoPrimals/fossilRecord)) and evolve to IPC via `CompositionContext`.

| Spring | Primal Subdir | Content | WGSL Count | Action |
|---|---|---|---|---|
| **hotSpring** | `barracuda/` | Largest shader tree (lattice, MD, physics, coral sovereign) | 128 | Fossilize after confirming barraCuda upstream coverage |
| **neuralSpring** | `metalForge/shaders/` | ML/attention/fitness shaders + existing fossils | 43 | Partially fossilized, rest pending |
| **ludoSpring** | `barracuda/shaders/game/` | Game shaders (fog, pathfind, lighting, perlin) | 16 | Fossilize |
| **healthSpring** | `ecoPrimal/shaders/health/` | Domain health shaders (PK, dose-response, diversity) | 6 | Fossilize |
| **healthSpring** | `toadstool/` | toadStool-specific dispatch pipeline | — | Elevate to IPC |
| **groundSpring** | `metalForge/shaders/` | Anderson-Lyapunov shaders | 2 | Fossilize |
| **wetSpring** | `barracuda/src/ncbi/nestgate/` | NestGate discovery/fetch/RPC/storage | — | Elevate to IPC |
| **wetSpring** | `metalForge/forge/src/inventory/songbird.rs` | Songbird socket discovery | — | Review — may be valid IPC helper |

### Cargo.toml Path Dependencies to Primal Crates

These direct `path = "../../primals/..."` dependencies must evolve to capability-based
IPC via `CompositionContext`. Library-level type sharing (Tier 1) is acceptable,
but behavioral calls must go through IPC (Tier 2).

| Spring | Path Deps |
|---|---|
| **hotSpring** | barraCuda, toadStool (akida), coralReef (coral-gpu) |
| **wetSpring** | barraCuda, toadStool, bingoCube |
| **airSpring** | barraCuda, toadStool (akida) |
| **healthSpring** | barraCuda (barracuda-core) |
| **neuralSpring** | barraCuda, coralReef |
| **groundSpring** | barraCuda, toadStool (akida) |
| **ludoSpring** | barraCuda; experiments with rhizoCrypt/loamSpine/sweetGrass path deps |

---

## Fossilization Pattern

Each spring follows this pattern for pre-absorption code:

1. **Snapshot** — Copy `src/` of the target subdir into
   `fossilRecord/{name}_pre_interstadial_may2026/` (local or shared repo) with a README explaining
   provenance and what supersedes it.

2. **Deprecate** — Add `#[deprecated(since = "...", note = "...")]` to public
   APIs that are superseded. Add `#[allow(deprecated)]` to internal callers
   that must remain for backward compatibility.

3. **Evolve** — Replace direct primal calls with `CompositionContext::call()`
   routing through capability-based IPC. Test against deployed ecoBins from
   plasmidBin.

4. **Verify** — `cargo build + clippy + fmt + test` — zero errors, zero warnings.

---

## barraCuda Shader Absorption Checklist

For local WGSL shaders in any spring:

- [ ] Identify all local `.wgsl` files and their domain
- [ ] Check barraCuda upstream for equivalent ops (stats, linalg, spectral, nn, etc.)
- [ ] For each shader with upstream equivalent:
  - [ ] Validate parity (output tolerance ≤ documented threshold)
  - [ ] Switch to `barraCuda::{domain}::{op}` call
  - [ ] Snapshot local shader to fossilRecord (local or shared repo)
  - [ ] Remove local shader source
- [ ] For shaders without upstream equivalent:
  - [ ] File handoff to barraCuda team: `BARRACUDA_SHADER_ABSORPTION_{DOMAIN}_HANDOFF.md`
  - [ ] Keep local shader until absorbed, mark as `pre-absorption`

---

## Cargo.toml Path Dep Elimination Checklist

For each `path = "../../primals/..."` dependency:

- [ ] Identify what the spring uses from the primal crate
- [ ] Categorize each usage:
  - **Type sharing** (structs, enums for serialization) → Keep as Tier 1 lib dep
  - **Behavioral calls** (function invocations that need a running primal) → Migrate to Tier 2 IPC
- [ ] For Tier 2 migrations:
  - [ ] Replace with `ctx.call("capability", "method", params)`
  - [ ] Remove path dep from `Cargo.toml` if no Tier 1 usage remains
  - [ ] Add integration tests using `CompositionContext::from_live_discovery_with_fallback()`

---

## Key References

| Document | Location |
|---|---|
| Validation Tiers architecture | `docs/VALIDATION_TIERS.md` |
| Fossil record | `fossilRecord/README.md` (local redirect → [shared repo](https://github.com/ecoPrimals/fossilRecord)) |
| Experiment catalog | `experiments/results/experiment_catalog.json` |
| Capability registry | `config/capability_registry.toml` (441 methods) |
| CompositionContext docs | `ecoPrimal/src/composition/context.rs` |
| plasmidBin depot pattern | `wateringHole/PLASMINBIN_DEPOT_PATTERN.md` |
| Method Gate standard | `wateringHole/METHOD_GATE_STANDARD.md` |
| Crypto consumption hierarchy | `wateringHole/CRYPTO_CONSUMPTION_HIERARCHY.md` |

---

## Post-Interstadial Evolution Delta (May 15, 2026)

Since the original handoff (May 9), primalSpring has undergone significant evolution:

- **32 validation scenarios** (was 20 at handoff) — added `s_atomic_signals`,
  `s_meta_tier_signals`, `s_agentic_tower`, `s_dark_forest_gate`,
  `s_deployment_pipeline`, and several others
- **441 capability methods** (was 389) — signal names, meta-tier methods, and
  agentic tower capabilities added to registry
- **Eukaryotic validation infrastructure** — shared `validation::helpers` module
  extracts graph parsing, Dark Forest invariants, and capability cross-referencing
  from duplicated scenario code. Registry meta-tests validate scenario consistency.
- **14 atomic signal graphs** (`graphs/signals/`) define the Neural API
  composition collapse layer: tower/node/nest/meta tiers
- **`tower.bootstrap` signal** resolves the bootstrap paradox with a two-phase
  cold-start sequence (static Phase 1, graph-driven Phase 2)
- **`validate_all` binary deprecated** in favor of `primalspring validate`

**Neural API evolution** (biomeOS v3.55–v3.57):
- `signal.dispatch` is now the preferred dispatch path for atomic signals;
  `CompositionContext::signal()` uses it with `capability.call` fallback
- `capability.call` transparently intercepts signal-tier requests and executes
  the backing graph (composition collapse)
- `primal.announce` atomic self-registration replaces separate `lifecycle.register`
  + `capability.register` + `method.register` calls — see `wateringHole/PRIMAL_ANNOUNCE_PROTOCOL.md`
- Squirrel `signal_plan` mode decomposes natural-language intent into structured
  atomic signal step sequences via `config/signal_tools.toml`
- Tier 2 validation dynamically checks `signal.list` counts and `signal.schema`
  tool definitions (no hardcoded signal counts)

Springs consuming this handoff should reference `docs/VALIDATION_TIERS.md` for
the current eukaryotic validation pattern and `config/signal_tools.toml` for
the atomic signal tool surface.

---

**License**: AGPL-3.0-or-later
