# primalSpring Phase 15: Cross-Ecosystem Absorption Handoff

**Version**: v0.7.0 Phase 15
**Date**: March 24, 2026
**Tests**: 361 (unit + integration + doc-tests + proptest)
**Clippy**: 0 warnings (pedantic + nursery + cast + unwrap/expect discipline, `--all-targets`)

---

## What Changed

### 1. Slug Constants — Zero Hardcoded Primal Names

`primal_names` module now exports `pub const` slug constants:

```rust
pub const BEARDOG: &str = "beardog";
pub const SONGBIRD: &str = "songbird";
pub const TOADSTOOL: &str = "toadstool";
pub const NESTGATE: &str = "nestgate";
pub const SQUIRREL: &str = "squirrel";
pub const RHIZOCRYPT: &str = "rhizocrypt";
pub const LOAMSPINE: &str = "loamspine";
pub const SWEETGRASS: &str = "sweetgrass";
```

All production code (`coordination/mod.rs`, `ipc/probes.rs`, `bin/main.rs`) now uses
`primal_names::BEARDOG` instead of `"beardog"` string literals. This eliminates
scattered duplicates and gives a single source of truth for primal identity.

**Action for all teams**: If you reference primal names in code, import from a
centralized constants module. Don't scatter `"beardog"` across files.

### 2. Unwrap/Expect Discipline (healthSpring V42 / wetSpring V135)

Workspace lints now include:

```toml
[workspace.lints.clippy]
unwrap_used = "warn"
expect_used = "warn"
```

Production code has zero `.unwrap()`/`.expect()` calls. Test code is explicitly
allowed via `#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]`
on `lib.rs` and `#![allow(...)]` on integration test crate roots.

**Action for Spring Teams**: Add `unwrap_used = "warn"` and `expect_used = "warn"`
to your workspace clippy lints. Use `.or_exit()` in binaries and proper `?`/`Result`
in library code. Allow in test code via `cfg_attr(test, allow(...))`.

**Action for Primal Teams**: BearDog already has this. Other primals should adopt.

### 3. Smart Module Extraction — launcher/tests.rs

`launcher/mod.rs` was 802 LOC (approaching 1000 limit). Extracted 109 lines of
test code to `launcher/tests.rs`, bringing production code to 699 LOC.

Additionally, hardcoded env var names and relative discovery paths were extracted
as named constants:

```rust
const ENV_PLASMID_BIN: &str = "ECOPRIMALS_PLASMID_BIN";
const ENV_BIOMEOS_BIN_DIR: &str = "BIOMEOS_PLASMID_BIN_DIR";
const RELATIVE_PLASMID_TIERS: &[&str] = &["./plasmidBin", "../plasmidBin", "../../plasmidBin"];
```

**Pattern**: When a module approaches 800 LOC, extract `#[cfg(test)]` blocks to
sibling `tests.rs` files first. This preserves production code cohesion while
reducing file size.

### 4. Provenance Docs Updated for rhizoCrypt Backend Change

`ipc/provenance.rs` capability routing table now documents:
- rhizoCrypt: `redb + memory` backend (sled removed in v0.14)
- loamSpine: capability-based env vars only
- sweetGrass: zero-copy braids

**Action for Provenance Trio Teams**: Legacy `BEARDOG_ADDRESS` / `NESTGATE_ADDRESS`
env vars are eliminated in rhizoCrypt S20. Use capability-based resolution:
`SIGNING_ENDPOINT`, `PERMANENT_STORAGE_ENDPOINT`, etc.

### 5. CONTRIBUTING.md + SECURITY.md

Created ecosystem-standard contributor and security policy docs (neuralSpring V124 pattern).

**Action for all teams**: Create `CONTRIBUTING.md` and `SECURITY.md` in your repo root
if you don't have them already.

---

## Cross-Ecosystem Patterns Reviewed

primalSpring reviewed all 7 sibling springs and 10+ primals. Key patterns identified:

| Source | Pattern | Status in primalSpring |
|--------|---------|----------------------|
| wetSpring V135 | `finish_with_code()` for `fn main() -> ExitCode` | Documented for future adoption |
| wetSpring V135 | Validation stack decomposition (sink/harness/or_exit/data_dir/timing/domain) | Template for next `validation/mod.rs` growth |
| neuralSpring V124 | Upstream tolerance contract pins | Ready when numeric work is added |
| airSpring V010 | `f64::total_cmp` for NaN-safe ordering | Ready when sorting numeric results |
| Squirrel alpha.23 | `resolve_capability_unix_socket()` tiered env resolution | Aligns with our 5-tier discovery |
| BearDog Wave 14 | Ephemeral ports (`:0`) | Ready when adding TCP tests |
| biomeOS v2.67 | 5-tier discovery explicitly aligned with primalSpring | Parity maintained |

---

## What primalSpring's Builder Pattern Offers Other Teams

None of the 7 sibling springs have adopted primalSpring's builder `.run()` pattern yet:

```rust
ValidationResult::new("Title")
    .with_provenance("crate", "date")
    .run("subtitle", |v| {
        v.check_bool("name", actual, expected);
    });
```

This eliminates 5-8 lines of boilerplate per experiment (banner, finish, exit).
Springs with many experiments (wetSpring 354, healthSpring 83) would benefit most.

---

## Current Quality Posture

| Metric | Value |
|--------|-------|
| Tests | 361 |
| Experiments | 53 (10 tracks) |
| Deploy graphs | 22 TOMLs |
| Gates | 87/87 |
| Clippy warnings | 0 (pedantic + nursery + cast + unwrap/expect, all-targets) |
| Doc warnings | 0 |
| Unsafe code | `forbid` workspace-level |
| C dependencies | 0 (deny.toml enforced) |
| Files over 1000 LOC | 0 |
| `.unwrap()` in production | 0 |
| `#[allow()]` in production | 0 |
| Provenance coverage | 100% (53/53 experiments) |
| Hardcoded primal names | 0 (all via `primal_names::` constants) |

---

**License**: AGPL-3.0-or-later
