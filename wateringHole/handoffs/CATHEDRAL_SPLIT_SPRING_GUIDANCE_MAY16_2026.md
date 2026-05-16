# CATHEDRAL Split — Spring Guidance Handoff

**Date**: May 16, 2026  
**From**: primalSpring (coordination spring)  
**To**: All delta springs (8), lithoSpore team, projectFOUNDATION team

## Context

The CATHEDRAL team has split into two dedicated IDE focus teams:

- **lithoSpore** — verification chassis (USB-deployable validation artifacts,
  module crates, geo-delocalized Tier 2)
- **projectFOUNDATION** — knowledge layer (thread lineage, data sources/targets,
  validation evidence capture)

Both teams are now mature enough for dedicated workstreams. Springs feed both.
This handoff defines the contract between springs and each workstream.

## What Changed for Springs

Previously, springs handed patterns down to "CATHEDRAL" as a single entity.
Now there are two distinct integration surfaces:

### 1. lithoSpore Integration (Verification)

lithoSpore consumes spring validation binaries as instance modules. The
integration contract:

**Module interface**: If your spring's validation binary follows:
```rust
pub fn run_validation(
    data_dir: &Path,
    expected: &ExpectedValues,
    max_tier: u8,
) -> ModuleResult
```
it can be wired into lithoSpore instances via `scope.toml`.

**Who this applies to**: Springs with LTEE reproductions or domain-specific
validation that produces expected-value JSON:
- **groundSpring** — LTEE B1-B4 (`expected_values.json` per paper)
- **hotSpring** — LTEE B2 Anderson
- **wetSpring** — LTEE B7 genomics
- **neuralSpring** — ML surrogates for B1
- **healthSpring** — B5 symbiont PK/PD

**Not applicable**: primalSpring (coordination — no data_dir/expected pattern),
ludoSpring (game science — no LTEE), airSpring (agricultural — no LTEE yet).

**Action**: Review your validation crates. If they expose the `run_validation()`
interface, they're wirable. If not, consider adding a thin adapter crate that
bridges your `ValidationResult` output to `ModuleResult`.

### 2. projectFOUNDATION Integration (Knowledge)

projectFOUNDATION captures validation evidence as thread-linked provenance.
The integration contract:

**Thread reference**: Check `projectFOUNDATION/lineage/THREAD_INDEX.toml`
and `data/sources/*.toml` for threads that reference your spring.

**Validation capture**: Export validation results to
`projectFOUNDATION/validation/<spring>/<YYYY-MM-DD>/` with:
- `results.json` (from `ValidationResult::to_json()` or equivalent)
- `provenance.toml` (run metadata: tier, primals available, duration)

**Thread ownership recap**:

| Thread | Name | Owner Springs | Status |
|--------|------|---------------|--------|
| 1 | Whole-Cell Modeling | hotSpring, wetSpring, healthSpring | ACTIVE (RPC blocked) |
| 2 | Plasma Physics / QCD | hotSpring | **12/12 PASS** |
| 3 | Immunology | healthSpring | Needs expression |
| 4 | Environmental Genomics | wetSpring | Needs expression + targets |
| 5 | LTEE / Evolution | groundSpring, neuralSpring, wetSpring, hotSpring | **ACTIVE** |
| 6 | Agricultural Science | airSpring, groundSpring, wetSpring | **36/36 PASS** |
| 7 | Anderson Mathematics | hotSpring, groundSpring, wetSpring, neuralSpring | **18/18 PASS** |
| 8 | Human Health | healthSpring | Needs expression |
| 9 | Gaming / Creative | ludoSpring | Needs expression |
| 10 | Provenance / Economics | ludoSpring, primalSpring, healthSpring | **SEEDED** |

**Action**: For threads you own, ensure dated validation folders exist in
projectFOUNDATION. For threads needing expressions (3, 4, 8, 9), write
the expression document mapping your domain models to thread targets.

## Deep Debt Checklist (from CATHEDRAL team)

When running your spring's next audit pass, check:

1. **Files over 1000 LOC** — split if possible
2. **`#[allow()]` in production code** — should only be `deprecated` bridges
3. **Stale mocks** — verify test fixtures match current wire formats
4. **barraCuda version currency** — ensure IPC tests exercise current API

primalSpring audit result (May 16, 2026): Zero files over 1000 LOC (max 903).
All 16 `#[allow(deprecated)]` are intentional backward-compatibility bridges.
No stale mocks. No direct barraCuda dependency (IPC-only coordination).

## Signal Elevation Context

This handoff coincides with Wave 17 (Neural API Signal Elevation). Springs
adopting `ctx.dispatch()` and `ctx.announce()` should note:

- lithoSpore module validation can add signal dispatch phases alongside
  individual method validation (automatic fallback for pre-v3.56 biomeOS)
- projectFOUNDATION validation evidence can capture signal-level results
  alongside method-level results for richer provenance

See: `wateringHole/SIGNAL_ADOPTION_STANDARD.md`

## primalSpring Specifics

primalSpring's relationship to the split:

- **lithoSpore**: primalSpring does NOT expose the `run_validation(data_dir, expected, max_tier)`
  interface — our scenarios use `fn run(&mut ValidationResult, &mut CompositionContext)`.
  This is correct: primalSpring validates coordination patterns, not domain data.
  We provide the validation *infrastructure* (registry, helpers, `CompositionContext`)
  that lithoSpore modules wire through.

- **projectFOUNDATION**: primalSpring co-owns Thread 10 (Provenance/Economics)
  with ludoSpring and healthSpring. Our `exp107_foundation_validation` experiment
  and `graphs/compositions/foundation_validation.toml` anchor this thread.
  Validation results export via `ValidationResult::to_json()` + `PRIMALSPRING_JSON` env.

## References

- `wateringHole/SIGNAL_ADOPTION_STANDARD.md` — signal API migration guide
- `wateringHole/PRIMAL_ANNOUNCE_PROTOCOL.md` — announce protocol wire format
- `docs/DOWNSTREAM_PATTERN_GUIDE.md` — full downstream pattern inventory
- `docs/CROSS_SPRING_PARITY_SCORECARD.md` — per-spring capability matrix
