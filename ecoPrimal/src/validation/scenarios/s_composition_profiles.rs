// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Composition Profiles — validates deployment profiles in config/profiles/.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const PROFILE_PATHS: &[(&str, &str)] = &[
    (
        "tower_atomic",
        include_str!("../../../../config/profiles/tower_atomic.toml"),
    ),
    (
        "nest_atomic",
        include_str!("../../../../config/profiles/nest_atomic.toml"),
    ),
    (
        "full_nucleus",
        include_str!("../../../../config/profiles/full_nucleus.toml"),
    ),
    (
        "compute_heavy",
        include_str!("../../../../config/profiles/compute_heavy.toml"),
    ),
    (
        "edge_light",
        include_str!("../../../../config/profiles/edge_light.toml"),
    ),
];

/// Composition profiles scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "composition-profiles",
        track: Track::AtomicComposition,
        tier: Tier::Rust,
        provenance_crate: "wave138a_composition_profiles",
        provenance_date: "2026-07-14",
        description: "Composition profiles — config/profiles/ manifests parse as valid TOML",
    },
    run,
};

/// Run composition profiles validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Profile count and parse validity");

    v.check_bool(
        "profiles:minimum_count",
        PROFILE_PATHS.len() >= 3,
        &format!("{} profiles available (need ≥3)", PROFILE_PATHS.len()),
    );

    let mut parsed_count = 0;
    for (name, content) in PROFILE_PATHS {
        let parsed = toml::from_str::<toml::Value>(content);
        let ok = parsed.is_ok();
        if ok {
            parsed_count += 1;
        }
        v.check_bool(
            &format!("profile:{name}:parses"),
            ok,
            &format!("config/profiles/{name}.toml parses as valid TOML"),
        );
    }

    v.section("Phase 2: Profile structural contracts");

    for (name, content) in PROFILE_PATHS {
        v.check_bool(
            &format!("profile:{name}:manifest"),
            content.contains("[manifest]"),
            &format!("{name} profile declares [manifest] section"),
        );
        v.check_bool(
            &format!("profile:{name}:composition"),
            content.contains("[composition]"),
            &format!("{name} profile declares [composition] section"),
        );
    }

    v.check_bool(
        "profiles:all_parse",
        parsed_count == PROFILE_PATHS.len(),
        &format!(
            "{parsed_count}/{} profiles parse successfully",
            PROFILE_PATHS.len()
        ),
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
