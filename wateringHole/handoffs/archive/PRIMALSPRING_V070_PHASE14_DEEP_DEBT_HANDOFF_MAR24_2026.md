# SPDX-License-Identifier: AGPL-3.0-or-later
#
# primalSpring v0.7.0 Phase 14 — Deep Debt + Builder Pattern + Full Provenance
#
# Handoff for: All primal teams, all spring teams, biomeOS team
# Date: March 24, 2026
# Author: primalSpring coordination team

---

# primalSpring Phase 14 Handoff — Deep Debt Resolution

## Summary

Phase 14 evolved primalSpring from "working and tested" to "idiomatic,
provenance-complete, and zero-warning at every lint level." This handoff
documents patterns every primal and spring team should absorb.

**What changed:**
- Builder-pattern `ValidationResult::run()` on all 53 experiments
- 100% structured provenance via `with_provenance(source, date)`
- Smart module extraction (validation/tests.rs: 1016 → 540 + 493 LOC)
- Zero `.unwrap()` in experiment binaries (all `.or_exit("context")`)
- Zero `#[allow()]` in production (all evolved to `#[expect(reason)]`)
- 361 tests, 0 clippy warnings, 0 doc warnings, 0 fmt diff

---

## 1. Builder-Pattern Validation (Absorb This)

### Before (manual boilerplate)

```rust
fn main() {
    println!("=== Title ===");
    let mut v = ValidationResult::new("Title");
    // ... checks ...
    v.finish();
    std::process::exit(v.exit_code());
}
```

### After (builder — zero boilerplate)

```rust
fn main() -> ! {
    ValidationResult::new("Title")
        .with_provenance("crate_name", "2026-03-24")
        .run("Subtitle", |v| {
            v.check_bool("name", actual, expected);
        });
}
```

**Why this matters for every spring:**
- `run()` consumes `self`, prints banner, executes checks, prints summary, exits
- Cannot forget `finish()` or `exit()` — the compiler enforces it
- Provenance is chainable and structural (source + date on every experiment)
- The `-> !` return type documents that this function never returns

**Action for sibling springs**: hotSpring, wetSpring, airSpring, groundSpring,
neuralSpring, healthSpring, ludoSpring — adopt `ValidationResult::run()` for
all validation binaries. The pattern eliminates 3-5 lines of boilerplate per
binary and makes provenance opt-in at the builder level.

---

## 2. Provenance Coverage (Absorb This)

Every experiment now carries:
```rust
.with_provenance("exp001_tower_atomic", "2026-03-24")
```

This creates a structured `Provenance { source, date }` on the
`ValidationResult`, emitted in both human-readable summary and NDJSON output.

**Target for all springs**: 100% provenance on all validation binaries.
The `with_provenance()` API is already in the shared `ValidationResult` —
every spring has access.

---

## 3. Smart Module Extraction (Pattern)

`validation/mod.rs` was 1016 lines — over the 1000 LOC limit. Rather than
arbitrary splitting, we extracted the 493-line `#[cfg(test)]` block to
`validation/tests.rs`:

```
validation/
├── mod.rs    # 540 lines — production code only
└── tests.rs  # 493 lines — all unit tests (#[cfg(test)])
```

**Why smart extraction, not arbitrary split:**
- Production code stays cohesive in one file
- Test code gets its own file with full access to the module internals
- Both files are well under 1000 LOC
- The `mod.rs` file only adds `#[cfg(test)] mod tests;`

**Pattern for primals**: BearDog, Squirrel, and other primals with large
modules should extract test code to sibling files rather than splitting
production logic across arbitrary boundaries.

---

## 4. `.unwrap()` Elimination (Absorb This)

All `.unwrap()` calls in experiment binaries were replaced with `.or_exit()`:

```rust
// Before (panics with no context)
let graph = load_graph("path").unwrap();

// After (clean exit with context message)
let graph = load_graph("path").or_exit("failed to load deploy graph");
```

The `OrExit<T>` trait is in the shared validation module. It prints a
human-readable error and exits with code 1 — no stack trace, no panic.

**Target for all springs**: Zero `.unwrap()` in validation binaries.
Use `.or_exit("context")` for infallible operations or `?` with proper
error types for fallible ones.

---

## 5. `#[allow()]` → `#[expect(reason)]` (Absorb This)

Modern Rust (Edition 2024+) provides `#[expect(lint, reason = "...")]` which:
- Documents *why* the lint is suppressed
- Warns when the suppression is no longer needed (the condition is fixed)
- Is self-documenting and auditable

```rust
// Before (opaque, no explanation)
#[allow(dead_code)]

// After (documented, self-expiring)
#[expect(dead_code, reason = "shared helpers — each test file uses a different subset")]
```

**Target for all primals and springs**: Zero `#[allow()]` in production code.
Use `#[expect(reason)]` for justified exceptions in test code only.

---

## 6. What Each Team Should Do

### Spring Teams (hotSpring, wetSpring, airSpring, groundSpring, neuralSpring, healthSpring, ludoSpring)

1. **Adopt builder `.run()`** for all validation binaries
2. **Add `with_provenance()`** to all experiments (target 100% coverage)
3. **Extract `#[cfg(test)]` blocks** from files over 800 LOC
4. **Replace `.unwrap()`** with `.or_exit()` in all binaries
5. **Evolve `#[allow()]` → `#[expect(reason)]`** with documented reasons

### Primal Teams (BearDog, Songbird, ToadStool, NestGate, Squirrel)

1. **Smart file refactoring** — extract tests, not production logic
2. **`#[expect(reason)]` over `#[allow()]`** — BearDog and Squirrel already
   follow this; ensure all new code does
3. **Zero `.unwrap()` in binary entry points** — `.or_exit()` for clean exits
4. **Provenance on JSON-RPC responses** — consider adding `provenance` fields
   to structured responses where traceability matters

### biomeOS Team

1. **Deploy graph validation** — primalSpring validates all 22 graphs at
   test time. New graphs should follow `by_capability` on all nodes
2. **Builder pattern for orchestration tests** — the same `ValidationResult`
   pattern applies to biomeOS integration testing

---

## 7. Code Quality Baseline Achieved

| Metric | Value |
|--------|-------|
| Tests | 361 (343 unit + 10 integration + 4 doc-tests + 4 proptest) |
| Clippy (pedantic+nursery) | 0 warnings |
| `cargo doc` | 0 warnings |
| `cargo fmt` | 0 diff |
| `cargo deny` | Clean (license + advisory + ban) |
| `#[allow()]` in production | 0 |
| `.unwrap()` in experiments | 0 |
| `unsafe` | Workspace-level `forbid` |
| C dependencies | 0 |
| Files over 1000 LOC | 0 |
| Provenance coverage | 100% (53/53 experiments) |

---

## 8. Files Changed

| File | Change |
|------|--------|
| `ecoPrimal/src/validation/mod.rs` | Refactored: 1016 → 540 LOC, added `run()` method, extracted tests |
| `ecoPrimal/src/validation/tests.rs` | Created: 493 LOC, all unit tests |
| `experiments/exp*/src/main.rs` (×53) | Standardized on builder `.run()` + `with_provenance()` |
| `experiments/exp010-012/src/main.rs` | `.unwrap()` → `.or_exit()` |
| `experiments/exp044/src/main.rs` | Extracted `probe_squirrel_rpc` helper (too_many_lines fix) |
| `experiments/exp063/src/main.rs` | Extracted `cross_device_exchange` helper (too_many_lines fix) |
| `ecoPrimal/tests/server_*.rs` (×3) | `#[allow(dead_code)]` → `#[expect(dead_code, reason)]` |
| `ecoPrimal/src/ipc/provenance.rs` | Fixed broken intra-doc link |
| `ecoPrimal/src/launcher/mod.rs` | Fixed stale Neural API socket path doc |
| `config/capability_registry.toml` | Version synced: 0.5.0 → 0.7.0 |

---

## 9. Cross-Ecosystem Learnings

### What we learned from the deep debt audit:

1. **Builder pattern eliminates boilerplate classes of bugs** — forgetting
   `finish()` or `exit()` is now impossible. Other springs should adopt this
   for any "setup → body → teardown → exit" pattern.

2. **100% provenance is achievable in a single pass** — it took one focused
   session to add provenance to all 53 experiments. The cost is one line per
   experiment. The value is full traceability.

3. **Smart refactoring beats splitting** — the validation module was 1016
   lines. Splitting it into two production files would have fragmented the
   API. Extracting tests preserved cohesion and met the LOC target.

4. **`#[expect(reason)]` is self-healing** — unlike `#[allow()]`, it warns
   when the suppression is no longer needed. This means lint suppressions
   automatically surface for cleanup as code evolves.

5. **Zero `.unwrap()` is achievable** — `.or_exit()` + `?` cover 100% of
   use cases in validation binaries. The result is better error messages
   and no stack traces in user-facing output.

---

**License**: AGPL-3.0-or-later
