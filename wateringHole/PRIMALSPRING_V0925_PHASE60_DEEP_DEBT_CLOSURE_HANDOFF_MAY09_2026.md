# primalSpring v0.9.25 — Phase 60+ Deep Debt Closure Handoff

**From**: primalSpring (syntheticChemistry)
**Date**: May 9, 2026
**To**: Delta spring teams, downstream products (projectNUCLEUS, esotericWebb), primal consumers
**Supersedes**: Supplements `PHASE60_COMPLETION_HANDOFF_MAY09_2026.md` and `PRIMALSPRING_V0925_EVOLUTION_HANDOFF_MAY09_2026.md`

---

## Headline

Phase 60+ deep debt closure: graph hygiene across 7 deploy graphs, integration test
decomposition (2 monoliths → 5 focused modules, all under 500 LOC), `validate_all`
broadened to sweep trio crates, capability naming harmonized, zero `#[allow]` without
reason. **No test count change (666 stable). No API change. No breaking graph change.**

---

## 1. What Changed (May 9, 2026)

### Graph Hygiene — `by_capability` Alignment

Seven deploy graphs used `by_capability = "shader_compile"` for CoralReef nodes.
The canonical fragments (`fragments/node_atomic.toml`, `fragments/nucleus.toml`) and
`routing.rs` use `"shader"`. All seven are now aligned:

| Graph | Node | Old | New |
|-------|------|-----|-----|
| `cells/hotspring_cell.toml` | CoralReef | `shader_compile` | `shader` |
| `cells/wetspring_cell.toml` | CoralReef | `shader_compile` | `shader` |
| `cells/neuralspring_cell.toml` | CoralReef | `shader_compile` | `shader` |
| `cells/ludospring_cell.toml` | CoralReef | `shader_compile` | `shader` |
| `neuralspring_inference_pipeline.toml` | CoralReef | `shader_compile` | `shader` |
| `hotspring_qcd_pipeline.toml` | CoralReef | `shader_compile` | `shader` |
| `spring_deploy/spring_deploy_template.toml` | CoralReef | `shader_compile` | `shader` |

**Action for springs**: If you copied cell or pipeline graphs from primalSpring before
this date, grep for `shader_compile` and replace with `shader`. The guidestone
structural check will catch drift at validation time.

### Proto-Nucleate Template — Guidestone `by_capability`

`graphs/downstream/proto_nucleate_template.toml` now includes `by_capability = "coordination"`
on the guidestone node entry, matching the pattern used by all other primal nodes in
the template. Springs generating proto-nucleate graphs from this template will
automatically inherit the field.

### `validate_all` — Trio Crate Discovery

The `validate_all` meta-validator now discovers crates matching both `primalspring-exp*`
and `primalspring-trio*`. This ensures trio integration crates (like `primalspring-trio-ops`)
are included in umbrella validation sweeps.

**Action for springs**: If you have standalone integration crates outside the `exp*`
naming convention, ensure your own `validate_all` (or equivalent) discovers them.

### Capability Registration — Dual Form

`niche.rs` now registers both `capabilities.list` (plural, Wire Standard L2) and
`capability.list` (singular, biomeOS convention) in `LOCAL_CAPABILITIES`. This ensures
primalSpring is discoverable regardless of which naming convention the consumer uses.

**Action for springs**: Register both forms in your niche if you expose capability
advertisement. The Wire Standard harmonization is tracked but not yet mandated — both
forms are valid.

### Discovery Scope — `orchestration` Domain

`ALL_CAPS` in `routing.rs` now includes `"orchestration"`, ensuring
`CompositionContext::discover()` materializes biomeOS as an orchestration client
during capability-based discovery. This was previously implicit through Neural API
routing but is now explicit in the routing table.

### Lint Discipline — Zero `#[allow]` Without Reason

The last `#[allow]` without reason — the test-only `#![cfg_attr(test, allow(...))]`
in `lib.rs` — has been migrated to `#[expect(..., reason = "...")]`. The codebase
now has **zero** bare `#[allow]` attributes.

**Pattern for springs**:
```rust
#![cfg_attr(
    test,
    expect(
        clippy::unwrap_used,
        clippy::expect_used,
        reason = "test code permits panicking unwraps for assertion-style failures"
    )
)]
```

### `SECURITY.md` — Banned Crate Count

The documented banned crate count was corrected from 14 to 18, matching the actual
`deny.toml` configuration. The 4 additional bans are: `native-tls`, `schannel`,
`security-framework`, `security-framework-sys`.

### Coverage Alias

`.cargo/config.toml` now defines `cargo coverage` as an alias for
`cargo llvm-cov --workspace --ignore-filename-regex tests/`. This was previously
documented in the README but required manual invocation.

---

## 2. Integration Test Decomposition

Two large integration test files were decomposed into focused, thematic modules:

| Original File | Lines | Split Into | Lines | Domain |
|---------------|-------|------------|-------|--------|
| `server_ecosystem.rs` | 1,106 | `server_ecosystem.rs` | 498 | Tower atomic + Squirrel AI |
| | | `server_ecosystem_songbird.rs` | 309 | Songbird IPC surface |
| | | `server_ecosystem_genetics.rs` | 327 | Three-tier genetics |
| `server_ecosystem_compose.rs` | 807 | `server_ecosystem_compose.rs` | 411 | Nest + Node composition |
| | | `server_ecosystem_overlay.rs` | 407 | Graph-driven overlays |

**Zero test loss.** All 666 tests remain (618 passed + 48 ignored). The split is purely
organizational — shared test infrastructure remains in `tests/integration/`.

**Pattern for springs**: When a test file exceeds 500-800 LOC, decompose by domain
theme rather than by arbitrary line count. Each file should map to a single composition
surface or primal interaction pattern.

---

## 3. Composition Patterns — Learnings for Downstream

### Fragment-First Is the Law

74 deploy graphs from 6 fragments. The correct way to add a new deployment:

1. Check if a fragment exists for your atomic tier (`tower_atomic`, `node_atomic`, `nest_atomic`)
2. Create a thin profile that composes fragments via `resolve = true`
3. Create a cell graph that overlays your spring logic on a NUCLEUS profile
4. Add `[graph.bonding_policy]` with `bond_type` and `trust_model`
5. Use `by_capability` on every operation node — never target primals by binary name

### `by_capability` is Mandatory on Operation Nodes

Every `[[graph.nodes]]` entry targeting a primal MUST include `by_capability`. The
guidestone structural validator checks this. The canonical capability domains map to
primals via `routing.rs::capability_to_primal()`:

| Domain | Primal | Example |
|--------|--------|---------|
| `security`, `crypto`, `auth` | BearDog | `by_capability = "security"` |
| `discovery`, `mesh`, `http` | Songbird | `by_capability = "discovery"` |
| `compute`, `ember` | ToadStool | `by_capability = "compute"` |
| `tensor`, `math`, `stats`, `ml` | barraCuda | `by_capability = "tensor"` |
| `shader` | CoralReef | `by_capability = "shader"` |
| `storage`, `content` | NestGate | `by_capability = "storage"` |
| `dag`, `provenance` | rhizoCrypt | `by_capability = "dag"` |
| `ledger`, `commit` | LoamSpine | `by_capability = "ledger"` |
| `attribution` | sweetGrass | `by_capability = "attribution"` |
| `ai` | Squirrel | `by_capability = "ai"` |
| `visualization` | petalTongue | `by_capability = "visualization"` |
| `orchestration` | biomeOS | `by_capability = "orchestration"` |
| `defense` | skunkBat | `by_capability = "defense"` |
| `coordination` | primalSpring | `by_capability = "coordination"` |

### Neural API Deployment Pipeline

```
1. tools/fetch_primals.sh              → download plasmidBin
2. tools/desktop_nucleus.sh            → launch NUCLEUS (biomeOS-first)
3. biomeos graph execute <cell>.toml   → deploy spring cell
4. CompositionContext::discover()      → 5-tier escalation
5. ctx.call("domain.method", params)   → Neural API routing
```

Springs are compositions. The deploy graph is the product. biomeOS is the runtime.

---

## 4. Quality Gate Verification

All gates pass as of this handoff:

```
cargo check --workspace                    → ok
cargo clippy --workspace -- -D warnings    → 0 warnings
cargo test -p primalspring                 → 666 tests (618 passed + 48 ignored)
cargo deny check                           → advisories ok, bans ok, licenses ok, sources ok
```

| Metric | Value |
|--------|-------|
| Tests | 666 (stable) |
| Experiments | 85 (19 tracks) |
| Deploy graphs | 74 |
| Capability methods | 389 |
| Clippy warnings | 0 |
| `unsafe` blocks | 0 |
| `#[allow()]` without reason | 0 |
| Max file size (integration) | 498 LOC |
| Max file size (library) | < 700 LOC |

---

## 5. Files Modified in This Closure

```
.cargo/config.toml                               +3
SECURITY.md                                      +1 -1
ecoPrimal/src/bin/validate_all/main.rs           +3 -2
ecoPrimal/src/composition/routing.rs             +1
ecoPrimal/src/lib.rs                             +7 -2
ecoPrimal/src/niche.rs                           +1
ecoPrimal/tests/server_ecosystem.rs              -608 (decomposed)
ecoPrimal/tests/server_ecosystem_compose.rs      -396 (decomposed)
ecoPrimal/tests/server_ecosystem_songbird.rs     +309 (new)
ecoPrimal/tests/server_ecosystem_genetics.rs     +327 (new)
ecoPrimal/tests/server_ecosystem_overlay.rs      +407 (new)
graphs/cells/hotspring_cell.toml                 +1 -1
graphs/cells/wetspring_cell.toml                 +1 -1
graphs/cells/neuralspring_cell.toml              +1 -1
graphs/cells/ludospring_cell.toml                +1 -1
graphs/downstream/proto_nucleate_template.toml   +1
graphs/hotspring_qcd_pipeline.toml               +1 -1
graphs/neuralspring_inference_pipeline.toml      +1 -1
graphs/spring_deploy/spring_deploy_template.toml +1 -1
```

---

**Generated**: May 9, 2026 — primalSpring v0.9.25, Phase 60+ deep debt closure
**License**: CC-BY-SA 4.0
