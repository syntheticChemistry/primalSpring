# Wave 20 Debt Resolution — Delta Spring Self-Evolution Guide

**Date**: May 17, 2026
**Source**: primalSpring Wave 20 audit (pulled all 8 springs to HEAD)
**Audience**: All 7 delta spring teams
**Registry**: 452 methods (`primal.list` added Wave 20)

---

## Context

primalSpring audited all delta springs post-Wave 20. Strong adoption across the
board — every spring has the `capability.list` canonical envelope and registry
sync. This blurb documents **specific debt** surfaced per spring so each team
can self-evolve without merge conflicts.

Refer to `WAVE20_DELTA_SPRING_EVOLUTION_MAY16_2026.md` for the original
absorption checklist. This document covers **residual drift and debt only**.

---

## hotSpring

### Fossilized RPC handler — old `capability.list` shape
`barracuda/src/bin/_fossilized/hotspring_primal.rs` line ~234:
```rust
"capabilities.list" | "capability.list" => DispatchResult::Ok(json!({
    "capabilities": state.capabilities,
})),
```
**Fix**: Add `"count": state.capabilities.len()` and `"primal": "hotSpring"` to
match the canonical envelope used in your main dispatch path.

### `nest.commit` — candidate → adopted drift
`capability_registry.toml` correctly lists `nest.commit` as adopted. But these
docs still say "candidate":
- `docs/PRIMAL_GAPS.md` — "Signal candidates remaining: `nest.store`, `nest.commit`"
- `wateringHole/handoffs/HOTSPRING_WAVE17_SIGNAL_ADOPTION_HANDOFF_MAY16_2026.md` — `candidates = ["nest.store", "nest.commit"]`
- `wateringHole/handoffs/HOTSPRING_DOC_EVOLUTION_UPSTREAM_HANDOFF_MAY16_2026.md` — "Next candidates: `nest.store`, `nest.commit`"

**Fix**: Remove `nest.commit` from candidate lists. Only `nest.store` remains as
candidate (awaiting nestGate evolution).

### Test count drift
README and CHANGELOG cite `596/1,045` but workspace `cargo test` now yields
**1,607 passed**. Lib-only (`cargo test --lib`) is still 596.

**Fix**: Update status lines to `596/1,607` (lib/workspace).

### `commit_provenance()` unwired
`dag_provenance.rs` has `commit_provenance()` but it's never called from the
main pipeline. If this is intentional scaffolding, document it. If it should be
live, wire it into the Titan V pipeline's session finalization.

---

## healthSpring

### wateringHole/README.md status line stale
Header still reads:
> **Status**: V64o — Wave 17 Signal Adoption: ... 451-method registry sync

But the latest version is **V64r** (Wave 20 Schema Standard) and registry is
**452 methods**.

**Fix**: Update status line to V64r, 452-method registry, mention canonical
`capability.list` envelope adoption.

---

## wetSpring

### `ci_cross_sync.rs` threshold too low
`barracuda/tests/ci_cross_sync.rs` line ~196:
```rust
method_count >= 442,
"canonical registry has only {method_count} method entries — expected 452+"
```
The assertion text says "expected 452+" but the threshold is **442**. This will
pass even with 10 missing methods.

**Fix**: Tighten to `method_count >= 452`.

### `primal.list` not in CONSUMED_CAPABILITIES
`barracuda/src/niche.rs` `CONSUMED_CAPABILITIES` array ends at
`"signal.dispatch"` — no `"primal.list"` entry. Since wetSpring consumes
`primal.list` for composition health checks, it should be declared.

**Fix**: Add `"primal.list"` to `CONSUMED_CAPABILITIES`.

### PRIMAL_GAPS V158 attribution mix
`docs/PRIMAL_GAPS.md` under the V158 closure wave:
> CI cross-sync updated to 452 canonical methods (was 451, Wave 20)

This attributes a Wave 20 change to the V158 closure wave.

**Fix**: Clarify that the 452 threshold reflects Wave 20 (`primal.list` addition),
not V158 work.

### `primal.announce` signal status stale
Registry `[signals]` section has `pending = ["primal.announce"]` but
`primal.announce` is fully consumed — used in composition health probes.

**Fix**: Move `primal.announce` from `[signals].pending` to consumed/active.

---

## neuralSpring

**No code debt found.** neuralSpring aggressively adopted Wave 20 — both
`s_schema_standard` and `s_nest_commit` scenarios are implemented,
`execute_graph_live` and `store_science_result` are wired. Clean.

---

## ludoSpring

### wateringHole gap table — 451 → 452
`wateringHole/README.md` line ~100:
> GAP-12: Registry cross-sync | **RESOLVED** | V59: 28 `game.*` methods registered (451 total)

**Fix**: Update to `452 total` (Wave 20: `primal.list` added).

### `validate_tower_atomic.rs` compile error
`barracuda/src/bin/validate_tower_atomic.rs` uses
`ludospring_barracuda::ipc::IpcError` but the `ipc` module is gated behind
`#[cfg(feature = "ipc")]`. Running `cargo test` without the `ipc` feature
produces a compile error.

**Fix**: Either gate `validate_tower_atomic` behind `#[cfg(feature = "ipc")]`
in `Cargo.toml`, or add `ipc` to the binary's required features. The cleanest
approach is adding to the `[[bin]]` entry:
```toml
[[bin]]
name = "validate_tower_atomic"
required-features = ["ipc"]
```

---

## groundSpring

### Deploy graphs still reference `provenance.session_dehydrate`
Four graphs use `capability = "provenance.session_dehydrate"` as the operation:
- `graphs/groundspring_tower_bootstrap.toml`
- `graphs/groundspring_nucleus_local.toml`
- `graphs/groundspring_cross_substrate.toml`
- `graphs/groundspring_validation.toml`

The Rust code (`provenance.rs`) correctly falls back from `nest.commit` to
`provenance.session_dehydrate`, but the graphs themselves haven't evolved.

**Fix**: Consider updating graphs to use `nest.commit` as the primary capability
with `provenance.session_dehydrate` as fallback. Or document the graph's
capability references as intentional legacy compatibility.

### Notebook guideStone level stale
`notebooks/01-composition-validation.ipynb` cell references:
> guideStone Level | 3 (bare + IPC wired)

But groundSpring is now **Level 4**.

**Fix**: Update notebook cell to Level 4.

### Test count — 965 → 1,123+
PRIMAL_GAPS header says "956 tests" (ludoSpring, not groundSpring — this may
be a copy error). groundSpring's actual count is **1,123+** per the
`CONTROL_EXPERIMENT_STATUS.md`.

---

## airSpring

### Test mock missing canonical `count`
`barracuda/tests/primal_dispatch.rs` `capability.list` handler:
```rust
"capability.list" => DispatchOutcome::Ok(serde_json::json!({
    "niche": niche::NICHE_NAME,
    "domain": "ecology",
    "total": niche::CAPABILITIES.len(),
    "capabilities": niche::CAPABILITIES,
    ...
})),
```
Has `"total"` but not the canonical `"count"` field. Downstream consumers
(projectNUCLEUS, projectFOUNDATION) expect `"count"`.

**Fix**: Add `"count": niche::CAPABILITIES.len()` alongside `"total"`.

### PRIMAL_GAPS `--provenance-dir` status
`docs/PRIMAL_GAPS.md` says:
> Remaining: `--provenance-dir` for Thread 5+6 capture (when E3 LTEE starts)

But the main codebase has already implemented `--provenance-dir`. The gap doc
is stale.

**Fix**: Update to mark `--provenance-dir` as implemented, with E3 LTEE
activation as the remaining step.

---

## Cross-Cutting Summary

| Spring | Debt Items | Severity | Effort |
|--------|-----------|----------|--------|
| hotSpring | Fossilized RPC shape, nest.commit docs, test counts, unwired fn | Low | ~30 min |
| healthSpring | Status line stale | Trivial | ~5 min |
| wetSpring | ci_cross_sync threshold, CONSUMED list, V158 attribution, signal status | Low | ~20 min |
| neuralSpring | None | — | — |
| ludoSpring | Gap table 451, compile error (validate_tower_atomic) | Medium | ~15 min |
| groundSpring | Graph capabilities, notebook level, test count ref | Low | ~20 min |
| airSpring | Test mock count field, provenance-dir status | Low | ~10 min |

**Total estimated effort**: ~2 hours across all springs. No breaking changes.
All issues are documentation drift or minor code alignment.

---

## How To Use This

1. Find your spring's section above
2. Apply the fixes — all are small, self-contained changes
3. Run `cargo test` and `cargo clippy` to confirm
4. Push to `wateringHole/` with a debt resolution note
5. primalSpring will pull and update the scorecard on next wave
