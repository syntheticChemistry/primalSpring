# Downstream Interim — Wave 18 (May 16, 2026)

## Temporal Context

Delta springs are still absorbing the pre-CATHEDRAL-split evolution blurb
(Waves 16-17: playbook debt, Neural API signal elevation, signal adoption
standard). primalSpring has resolved its local deprecated-API debt and
prepared integration surfaces for downstream products.

This document captures what downstream products (projectNUCLEUS, lithoSpore,
projectFOUNDATION) can consume from primalSpring **now**, without waiting for
delta springs to complete their evolution pass.

---

## What Changed in Wave 18

| Item | Before | After |
|------|--------|-------|
| handlers.rs deprecated callers | 3 `#[allow(deprecated)]` blocks | 0 — all rewired to `validate_composition_ctx` / `CompositionContext` |
| exp107 `phase_health` | `probe_primal(name)` per primal | `ctx.health_check(cap)` per capability domain |
| exp004 `phase_composition_parity` | `probe_primal` aggregate latency | Timed `ctx.call(cap, "health.liveness")` loop |
| experiments/README.md | Stale counts (680 tests, 43 cells) | Reconciled (700 tests, 44 cells, deprecated refs removed) |
| deployment_matrix lithoSpore cell | 6/7 modules, 51/51 checks (May 14) | 7/7 modules, 75/75 checks (May 16) |
| README.md tools table | 11/24 documented | 24/24 documented |
| River Delta primalSpring tests | 767 (stale) | 700 (reconciled) |

---

## Surfaces Available for Downstream Consumption

### For lithoSpore

1. **Deployment matrix cell** (`config/deployment_matrix.toml` →
   `lithospore-x86-vm-uds`): 7/7 modules, 75/75 checks. This cell validates
   primalSpring's graph execution patterns on a fresh USB VM.

2. **Foundation validation graph** (`graphs/compositions/foundation_validation.toml`):
   12-node composition covering the full sediment pipeline. Run via:
   ```
   cargo run -p primalspring --bin primalspring_unibin -- certify --graph graphs/compositions/foundation_validation.toml
   ```

3. **Module interface note**: primalSpring does not directly expose
   `fn run_validation(data_dir, expected, max_tier) -> ModuleResult`. Our
   validation surfaces are graph-driven compositions (exp107, 43 scenarios)
   and UniBin subcommands (`validate`, `certify`). If lithoSpore needs a
   crate-level entry point, a thin shim over `validate_composition_ctx`
   is the path — tracked as future work.

### For projectFOUNDATION

1. **Thread 10 (Provenance/Economics)**: primalSpring co-owns this thread.
   Validation results from `exp107_foundation_validation` exercise the full
   sediment pipeline: DAG → Nest → ledger → braid. Results can be captured
   as dated provenance in `projectFOUNDATION/validation/primalSpring/`.

2. **Signal adoption evidence**: `s_signal_dispatch_parity` and
   `s_primal_announce` scenarios produce structured `ValidationResult` JSON
   that documents which of the 14 atomic signals are accepted by biomeOS.

3. **Modern API**: All primalSpring code now uses `CompositionContext` —
   zero `probe_primal` callers remain. This is the reference pattern for
   springs feeding validation results to projectFOUNDATION.

### For projectNUCLEUS

1. **Signal simplification**: Workload TOMLs can reference signals instead
   of individual method sequences. A `nest.store` signal replaces the
   4-call pattern (`content.put` → `dag.event.append` → `spine.seal` →
   `braid.create`).

2. **452 methods registry**: Canonical method surface is stable. Wave 20
   added `primal.list` (452nd method). Registry is the canonical source of truth.

3. **Composition coordination**: `validate_composition_ctx` is the single
   entry point for all composition health checks. No legacy fallbacks remain.

---

## Delta Spring Expectations

When delta springs complete their pre-split blurb absorption:

- They should adopt `ctx.dispatch()` for atomic signal consumption
  (see `wateringHole/SIGNAL_ADOPTION_STANDARD.md`)
- They should implement `ctx.announce()` for primal registration
- LTEE reproduction binaries that follow the `run_validation` interface
  can be wired into lithoSpore modules via `scope.toml`
- Validation results should flow to `projectFOUNDATION/validation/<spring>/`
  with dated provenance folders

The `CATHEDRAL_SPLIT_SPRING_GUIDANCE_MAY16_2026.md` handoff has the full
contract for both lithoSpore and projectFOUNDATION integration.

---

## Next Wave

Wave 19 will focus on delta springs completing their evolution and upstream
gap surfacing through the signal dispatch parity scenario. primalSpring's
local debt is resolved — the next pass is downstream product integration
hardening once springs have caught up.
