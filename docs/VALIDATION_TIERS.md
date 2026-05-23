# Validation Tiers

primalSpring and all delta springs use a two-tier validation architecture that
separates structural/analytical validation (pure Rust) from primal behavioral
validation (live IPC). For behavioral validation, **`CompositionContext`**
is the supported pattern; `probe_primal` and `PrimalClient::connect` remain
only as deprecated compatibility shims (see **Deprecated Patterns** below).

## Tier 1: Rust Validation (Library)

Structural checks that exercise type hierarchies, graph parsing, protocol
serialization, tolerance constants, and bonding policy rules. These are pure
Rust library interactions — no running primals required.

### What belongs in Tier 1

- Deploy graph TOML parsing and schema validation
- Bonding policy rule checks (`BondType`, `TrustModel`)
- Capability registry string matching against canonical 458 methods
- BTSP protocol frame serialization round-trips
- Tolerance constant assertions (documented thresholds)
- `ValidationResult` harness structural tests
- Genetics / permission tier serialization
- Graph topological ordering and wave computation

### Pattern

```rust
use primalspring::deploy::load_graph;
use primalspring::bonding::BondType;
use primalspring::validation::ValidationResult;

let graph = load_graph("graphs/fragments/tower_atomic.toml")?;
assert_eq!(graph.nodes.len(), 3); // bearDog + songbird + skunkBat (Phase 32)
assert!(graph.bonding_policy.bond_type == BondType::Covalent);
```

### Dependencies

Tier 1 code may depend on `primalspring` as a library crate and on primal
crates for type definitions (structs, enums for serialization). No binary
execution, no socket communication, no process management.

---

## Tier 2: Live NUCLEUS Validation (Primal IPC)

Any validation that touches primal **behavior** — health checks, capability
probing, cross-atomic composition, bearer token auth flows, Neural API
orchestration — MUST go through live IPC to deployed ecoBins from plasmidBin.

### What belongs in Tier 2

- Primal health check validation
- Capability discovery and probing
- Cross-atomic composition testing (Tower, Node, Nest, NUCLEUS)
- Bearer token authentication flows
- Neural API orchestration
- Save/load provenance DAG operations (via NestGate, rhizoCrypt, loamSpine, sweetGrass)
- Shader dispatch validation (via barraCuda, coralReef)
- Inference calls (via Squirrel)
- Visualization rendering (via petalTongue)

### Pattern

```rust
use primalspring::composition::CompositionContext;
use primalspring::validation::ValidationResult;

let mut ctx = CompositionContext::from_live_discovery_with_fallback();

let health = ctx.health_check("security")?;
let result = ctx.call("math", "stats.mean", serde_json::json!({"data": [1,2,3]}))?;
```

### Requirements

1. **Primals must be deployed** as ecoBins in plasmidBin (not spawned from test harness)
2. **biomeOS orchestrates** composition deployment via Neural API
3. **Capability-based routing** — call by `"security"`, `"math"`, `"storage"`, not by `"beardog"`, `"barracuda"`, `"nestgate"`
4. **5-tier discovery escalation**:
   - Tier 1: Songbird routing (`ipc.resolve`)
   - Tier 2: biomeOS Neural API (`capability.discover`)
   - Tier 3: UDS filesystem convention (`primal-family.sock`)
   - Tier 4: Socket registry / primal manifests
   - Tier 5: TCP probing (opt-in, covalent mesh only)

### Graceful Degradation

When primals are not deployed (CI, development without plasmidBin), experiments
MUST degrade gracefully:

```rust
match ctx.call("math", "stats.mean", params) {
    Ok(result) => v.check_bool("mean_result", true, "stats.mean via barraCuda"),
    Err(_) => v.check_skip("mean_result", "barraCuda not available"),
}
```

The `check_skip` pattern ensures validation runs are always informative — SKIP
is not FAIL. The experiment catalog records which primals were available for
each run.

---

## Tier 3: Provenance-Backed Validation (Primal Trio)

Tier 3 extends Tier 2 by recording validation results into the provenance trio
(rhizoCrypt DAG + loamSpine ledger + sweetGrass braid). This creates an
immutable, attributable record of every validation run — the guideStone
standard for deployed artifacts.

### What belongs in Tier 3

- Everything in Tier 2 PLUS provenance recording
- `primal.announce` self-registration with biomeOS
- DAG session lifecycle: `dag.session.create` → `event.append` × N → `session.complete`
- Spine anchoring: `spine.create` → `entry.append`
- Braid attribution: `braid.create` linking validation to operator and artifact
- Output of `Tier3Session` metadata (dag_session_id, merkle_root, spine_id, braid_id)

### Reference Implementation

lithoSpore's `try_record_tier3()` is the ecosystem's first Tier 3 consumer
(see `litho-core/src/provenance.rs`). primalSpring's `--provenance-dir` flag
writes `results.json` + `provenance.toml` to a dated folder, aligning with
projectFOUNDATION Thread 10.

### Partial Completion

See `PROVENANCE_TRIO_INTEGRATION_GUIDE.md` § Transaction Semantics for the
definitive specification of partial completion behavior.

---

## Cross-Tier Parity Pattern (from lithoSpore, May 17 2026)

When validation runs in multiple implementation languages (e.g., Python Tier 1
AND Rust Tier 2), **parity checking** proves the math is stable across
implementations. lithoSpore's `litho parity` subcommand runs both tiers
side-by-side and reports MATCH/DIVERGENCE per module.

### Adopting Parity in Springs

Springs with dual-language validation (notebooks + Rust scenarios) should
implement parity checking:

1. Run the Python baseline → capture expected values as JSON
2. Run the Rust implementation → capture computed values as JSON
3. Compare within documented tolerances (`tolerances.toml` or per-scenario)
4. Report structured `ParityResult` with status per check

### Ecosystem Value

A three-layer proof becomes possible with full Tier adoption:
- **Tier 1** confirms the science (Python baseline matches published claims)
- **Tier 2** confirms the implementation (Rust matches Python)
- **Tier 3** confirms the provenance chain (immutable record of who ran what)

For tolerance handling, see lithoSpore's `artifact/tolerances.toml` — named
tolerances with scientific justification per validation dimension.

---

## Scenario Infrastructure (Eukaryotic)

The validation infrastructure has evolved from standalone experiment binaries
(prokaryotic era: exp001–exp111) into 43 absorbed scenarios in
`ecoPrimal/src/validation/scenarios/`. Every scenario has:

- A `pub const SCENARIO: Scenario` with metadata (id, track, tier, provenance)
- A `pub fn run(v, ctx)` performing validation checks via `ValidationResult`
- A `#[cfg(test)] mod tests` block exercised by `cargo test --lib`

**Shared helpers** (`validation::helpers`) provide reusable graph TOML parsing,
Dark Forest invariant checking, and capability registry cross-referencing.
New scenarios should use these helpers instead of reimplementing locally.

**Registry meta-test** in `scenarios/mod.rs` validates:
- `build_registry()` returns exactly 45 scenarios
- No duplicate scenario IDs
- Every `Track` variant has at least one scenario
- All `Tier::Rust` scenarios pass structurally
- All provenance dates are valid ISO 8601

| Tier | Count | `cargo test` strategy |
|------|------:|----------------------|
| Rust | 9 | Assert `v.failed == 0` (full structural pass) |
| Both | 10 | Test structural phase or verify no panics |
| Live | 24 | Verify scenario runs to completion (failures expected without primals) |

`cargo test --lib` exercises all 45 scenarios — the single authoritative CI gate.

---

## Deprecated Patterns (Fossilized)

These patterns belong to the pre-interstadial direct-spawn era and are
`#[deprecated]` in `primalspring v0.9.25`:

| Pattern | Replacement |
|---|---|
| `AtomicHarness::new(type).start(id)` | plasmidBin ecoBin deploy via biomeOS |
| `spawn_primal(name, fid, nucleation)` | plasmidBin ecoBin deploy via biomeOS |
| `CompositionContext::from_running(harness)` | `CompositionContext::from_live_discovery_with_fallback()` |
| `probe_primal(name)` | `ctx.health_check(capability)` |
| `check_capability_health(v, cap)` | `ctx.health_check(cap)` + manual check recording |
| `validate_composition(atomic)` | `CompositionContext::discover()` + per-capability health checks |
| `PrimalClient::connect(socket, name)` | `ctx.call(capability, method, params)` |
| `discover_primal(name)` / `discover_by_capability(cap)` | Implicit in `CompositionContext` construction |

Fossilized code is preserved in [fossilRecord](https://github.com/ecoPrimals/fossilRecord) → `springs/primalSpring/`:
- `harness_launcher_pre_interstadial_may2026/`
- `experiments_pre_interstadial_may2026/`

---

## For Sibling Springs

Springs that currently import primal crates via `path = "../../primals/..."` in
their `Cargo.toml` should evaluate each dependency:

- **Type sharing** (structs, enums for serialization, constants): Keep as Tier 1.
  This is pure Rust analytical validation.

- **Behavioral calls** (functions that need a running primal process): Migrate
  to Tier 2. Replace with `CompositionContext::call(capability, method, params)`.

See `wateringHole/INTERSTADIAL_FOSSILIZATION_HANDOFF.md` for per-spring
inventories and checklists.

---

**License**: AGPL-3.0-or-later
