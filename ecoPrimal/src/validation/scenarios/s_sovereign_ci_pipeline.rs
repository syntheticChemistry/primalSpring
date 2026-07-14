// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Sovereign CI Pipeline — validates CI and pipeline concepts are
//! registered and structurally wired in the capability registry.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const PROTO_NUCLEATE: &str = include_str!("../../../../config/proto_nucleate.toml");

/// Sovereign CI pipeline scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "sovereign-ci-pipeline",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "wave138a_sovereign_ci_pipeline",
        provenance_date: "2026-07-14",
        description: "Sovereign CI pipeline — registry CI checks and graph pipeline methods",
    },
    run,
};

/// Run sovereign CI pipeline validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: CI registry contract");

    v.check_bool(
        "ci:registry_check_documented",
        REGISTRY_TOML.contains("primalspring registry --check"),
        "capability_registry.toml documents primalspring registry --check CI subcommands",
    );
    v.check_bool(
        "ci:drift_detection",
        REGISTRY_TOML.contains("drift detection") || REGISTRY_TOML.contains("drift check"),
        "registry documents drift detection for sovereign CI",
    );

    v.section("Phase 2: Pipeline methods");

    let pipeline_methods = ["graph.pipeline", "graph.execute", "graph.validate"];
    for method in pipeline_methods {
        v.check_bool(
            &format!("pipeline:{}", method.replace('.', "_")),
            REGISTRY_TOML.contains(method),
            &format!("{method} registered"),
        );
    }

    v.section("Phase 3: Proto-nucleate validation gates");

    v.check_bool(
        "ci:proto_nucleate_validation_section",
        PROTO_NUCLEATE.contains("[validation]"),
        "proto_nucleate.toml declares [validation] CI gate section",
    );
    v.check_bool(
        "ci:required_properties",
        PROTO_NUCLEATE.contains("required_properties"),
        "proto-nucleate requires guideStone properties before promotion",
    );
    v.check_bool(
        "ci:checksum_dual_verify",
        PROTO_NUCLEATE.contains("dual_verify"),
        "proto-nucleate enables BLAKE3 dual-verify in CI pipeline",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_passes_structural() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "scenario '{}' had {} failures",
            SCENARIO.meta.id, v.failed
        );
    }
}
