# Eukaryotic Validation Migration Guide

How to consolidate prokaryotic validation binaries (`[[bin]]` per experiment)
into a single eukaryotic UniBin with a scenario registry.

**Reference implementation**: wetSpring V182 (349 → 1 binary, 345 scenarios).
**Coordination library**: `primalspring::validation` (scenarios, numeric bridge, helpers).

---

## The Eras

| Era | Binary model | Validation model | CI pattern |
|-----|-------------|-----------------|------------|
| **Prokaryotic** | One `[[bin]]` per experiment | Each binary owns its own `main()` | `cargo run --bin validate_*` |
| **Eukaryotic** | Single UniBin with clap subcommands | `ScenarioRegistry` + `ScenarioMeta` | `spring validate --scenario id` |

The migration preserves all experiment logic. Only the dispatch layer changes.

---

## Step 1: Define Your Scenario Registry

Copy the pattern from `primalspring::validation::scenarios::registry`:

```rust
use primalspring::validation::scenarios::registry::{ScenarioMeta, Tier};
use primalspring::validation::ValidationResult;
use primalspring::composition::CompositionContext;

pub struct Scenario {
    pub meta: ScenarioMeta,
    pub run: fn(&mut ValidationResult, &mut CompositionContext),
}

pub struct ScenarioRegistry {
    scenarios: Vec<Scenario>,
}
```

**Note**: Your spring defines its own `Track` enum (science domains differ
from primalSpring's 10 coordination tracks). `Tier` and `ScenarioMeta` are
shared from the library.

---

## Step 2: Wrap Legacy Experiments

Each prokaryotic experiment gets a thin wrapper. Two patterns:

### Pattern A: Direct rewrite (preferred for small experiments)

```rust
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "diversity-richness",
        track: Track::Ecology,
        tier: Tier::Rust,
        provenance_crate: "exp042_diversity_richness",
        provenance_date: "2026-05-20",
        description: "Shannon diversity index validation against Python baseline",
    },
    run: |v, _ctx| {
        v.check_bool("richness_correct", compute_richness() == 42, "richness = 42");
    },
};
```

### Pattern B: Numeric bridge (for experiments with f64 baselines)

Use `primalspring::validation::numeric::NumericValidator`:

```rust
use primalspring::validation::numeric::NumericValidator;

fn legacy_validation(v: &mut NumericValidator) {
    v.check_f64("pi", compute_pi(), std::f64::consts::PI, 1e-10);
    v.check_f64_rel("growth_rate", measure_rate(), 0.693, 0.01);
    v.check_count("peaks", count_peaks(), 17);
}

pub fn run_as_scenario(result: &mut ValidationResult) {
    let mut v = NumericValidator::new("validate_pi_growth");
    legacy_validation(&mut v);
    v.bridge_into(result);  // preserves per-check granularity
}

pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta { /* ... */ },
    run: |v, _ctx| run_as_scenario(v),
};
```

`bridge_into` emits each accumulated check as a separate `check_bool` entry.
For coarse-grained bridging (one pass/fail per experiment), use `bridge_into_summary`.

---

## Step 3: Bulk Registration

For large experiment sets, use a registration function:

```rust
// experiments/mod.rs
pub fn register_all(r: &mut ScenarioRegistry) {
    r.register(exp_diversity::SCENARIO);
    r.register(exp_alignment::SCENARIO);
    // ... 300+ more
}

// scenarios/mod.rs
pub fn build_registry() -> ScenarioRegistry {
    let mut r = ScenarioRegistry::new();
    r.register(s_composition::SCENARIO);  // hand-written composition scenarios
    experiments::register_all(&mut r);     // bulk-registered experiments
    r
}
```

---

## Step 4: Feature Gating

Gate hardware-dependent experiments:

```rust
#[cfg(feature = "gpu")]
pub mod exp_gpu_shader;

pub fn register_all(r: &mut ScenarioRegistry) {
    #[cfg(feature = "gpu")]
    r.register(exp_gpu_shader::SCENARIO);
}
```

---

## Step 5: UniBin CLI

Template for your spring's `main.rs`:

```rust
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Clone, Copy, Default, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
}

#[derive(Subcommand)]
pub enum Commands {
    Validate {
        #[arg(long)] scenario: Option<String>,
        #[arg(long)] tier: Option<String>,
        #[arg(long)] list: bool,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    Certify { /* ... */ },
    Serve,
    Status,
    Version,
}
```

---

## Step 6: Cargo.toml Cleanup

Before (prokaryotic):
```toml
[[bin]]
name = "validate_diversity"
path = "src/bin/validate_diversity.rs"
# ... 349 more entries
```

After (eukaryotic):
```toml
autobins = false
[[bin]]
name = "myspring"
path = "src/bin/myspring/main.rs"
```

---

## Step 7: CI Exit Codes

| Code | Meaning |
|------|---------|
| 0 | All checks passed |
| 1 | One or more checks failed |
| 2 | All checks skipped (no primals available) |

`ValidationResult` provides `exit_code_skip_aware()` for this semantics.

---

## Step 8: Migration Script

wetSpring used a Python script (`scripts/migrate_to_unibin.py`) for bulk
file movement. For springs with fewer than ~50 experiments, manual migration
using Pattern A or B above is faster and produces cleaner code.

---

## Checklist

- [ ] Define spring-local `Track` enum for domain taxonomy
- [ ] Create `scenarios/registry.rs` (or import from primalspring)
- [ ] Wrap each experiment as `SCENARIO` const + `run` function
- [ ] Use `NumericValidator::bridge_into` for f64 baselines
- [ ] Feature-gate GPU/vault/hardware experiments
- [ ] Set `autobins = false`, single `[[bin]]` entry
- [ ] Clap subcommands: validate, certify, serve, status, version
- [ ] Typed `OutputFormat` enum (text/json)
- [ ] Meta-tests: scenario count, no duplicate IDs, all tracks covered
- [ ] Archive old `src/bin/validate_*.rs` to `fossilRecord/`
