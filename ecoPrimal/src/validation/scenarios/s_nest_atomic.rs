// SPDX-License-Identifier: AGPL-3.0-or-later
//! Scenario: Nest Atomic — absorbed from exp003.

use crate::composition::CompositionContext;
use crate::coordination::AtomicType;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "nest-atomic",
        track: Track::AtomicComposition,
        tier: Tier::Live,
        provenance_crate: "exp003_nest_atomic",
        provenance_date: "2026-05-09",
        description: "Nest Atomic — security, discovery, storage, and AI capabilities",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural");
    phase_structural(v);

    v.section("Phase 2: Discovery");
    phase_discovery(v, ctx);

    v.section("Phase 3: Health");
    phase_health(v, ctx);

    v.section("Phase 4: Spore Gateway (exp115)");
    phase_spore_gateway(v);
}

fn phase_structural(v: &mut ValidationResult) {
    let nest_caps = AtomicType::Nest.required_capabilities();
    v.check_count("nest_required_caps", nest_caps.len(), 4);
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let nest_caps = AtomicType::Nest.required_capabilities();
    let caps = ctx.available_capabilities();
    v.check_bool(
        "discovery_context_nonempty",
        !caps.is_empty(),
        &format!("{} context capabilities: {}", caps.len(), caps.join(", ")),
    );
    for cap in nest_caps {
        v.check_bool(
            &format!("has_{cap}"),
            ctx.has_capability(cap),
            &format!("{cap} capability resolved"),
        );
    }
}

fn phase_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let nest_caps = AtomicType::Nest.required_capabilities();
    for cap in nest_caps {
        match ctx.health_check(cap) {
            Ok(true) => v.check_bool(
                &format!("health_liveness_{cap}"),
                true,
                &format!("{cap} health.liveness ok"),
            ),
            Ok(false) => v.check_bool(
                &format!("health_liveness_{cap}"),
                false,
                &format!("{cap} not live"),
            ),
            Err(e) if e.is_connection_error() => v.check_skip(
                &format!("health_liveness_{cap}"),
                &format!("{cap} not reachable: {e}"),
            ),
            Err(e) => v.check_bool(
                &format!("health_liveness_{cap}"),
                false,
                &format!("{cap} health check error: {e}"),
            ),
        }
    }
}

/// Phase 4: Spore gateway structural validation (exp115_nest_ingest_pseudospore).
///
/// Validates that the spore ownership matrix infrastructure exists:
/// - pseudospore-core crate is part of lithoSpore workspace
/// - biomeos nucleus ingest command module exists
/// - liveSpore.json unified schema is documented
///
/// Live ingest tests require a running Nest Atomic and are gated on Phase 3 health.
fn phase_spore_gateway(v: &mut ValidationResult) {
    // Structural: SPORE_OWNERSHIP_MATRIX.md exists
    let matrix_exists = std::path::Path::new("infra/wateringHole/SPORE_OWNERSHIP_MATRIX.md").exists()
        || std::path::Path::new("../../infra/wateringHole/SPORE_OWNERSHIP_MATRIX.md").exists();
    v.check_bool(
        "spore_ownership_matrix_exists",
        matrix_exists,
        "SPORE_OWNERSHIP_MATRIX.md documents the three-way split",
    );

    // Structural: pseudospore-core crate exists
    let core_exists = std::path::Path::new("gardens/lithoSpore/crates/pseudospore-core/Cargo.toml").exists()
        || std::path::Path::new("../../gardens/lithoSpore/crates/pseudospore-core/Cargo.toml").exists();
    v.check_bool(
        "pseudospore_core_crate_exists",
        core_exists,
        "pseudospore-core crate provides domain-agnostic envelope primitives",
    );

    // Structural: nucleus_ingest.rs exists in biomeOS
    let gateway_exists = std::path::Path::new("primals/biomeOS/crates/biomeos-cli/src/commands/nucleus_ingest.rs").exists()
        || std::path::Path::new("../../primals/biomeOS/crates/biomeos-cli/src/commands/nucleus_ingest.rs").exists();
    v.check_bool(
        "nucleus_ingest_module_exists",
        gateway_exists,
        "biomeos nucleus ingest command implements the NUCLEUS gateway",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nest_atomic_no_panic() {
        let mut v = ValidationResult::new("nest-atomic");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(v.evaluated() > 0 || v.skipped > 0, "scenario should produce at least one check");
    }
}
