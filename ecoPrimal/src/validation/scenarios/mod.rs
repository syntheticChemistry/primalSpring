// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Validation scenarios — eukaryotic patterns evolved from absorbed experiments.
//!
//! Each scenario is a self-contained validation function that exercises
//! a specific NUCLEUS composition pattern. Scenarios evolved from the
//! prokaryotic experiment binary era (exp001–exp111) and were absorbed
//! into the library at the interstadial transition.
//!
//! # Modern Pattern
//!
//! Every scenario follows this structure:
//!
//! 1. A `pub const SCENARIO: Scenario` with metadata (`id`, `track`, `tier`,
//!    `provenance_crate`, `provenance_date`, `description`)
//! 2. A `pub fn run(v, ctx)` that performs validation checks
//! 3. A `#[cfg(test)] mod tests` block exercised by `cargo test --lib`
//! 4. Shared helpers from [`super::helpers`] for graph parsing, Dark Forest
//!    invariant checking, and capability registry cross-referencing
//!
//! # Tier Strategy
//!
//! - **`Tier::Rust`** scenarios test structural invariants; their `#[cfg(test)]`
//!   blocks assert `v.failed == 0`.
//! - **`Tier::Both`** scenarios have structural + live phases; tests exercise
//!   the structural phase directly, or verify the full run completes without panic.
//! - **`Tier::Live`** scenarios require deployed primals; tests verify the scenario
//!   runs to completion without panic (failures are expected without primals).
//!
//! # Usage
//!
//! ```rust,no_run
//! use primalspring::validation::scenarios::{ScenarioRegistry, Tier};
//!
//! let registry = ScenarioRegistry::new();
//! for scenario in registry.filter_by_tier(Tier::Rust) {
//!     println!("{}: {}", scenario.meta.id, scenario.meta.track);
//! }
//! ```

mod registry;

pub use registry::{Scenario, ScenarioMeta, ScenarioRegistry, Tier, Track};

// ───────────────────────────────────────────────────────────────────
// Absorbed scenario modules (one per track representative)
// ───────────────────────────────────────────────────────────────────
pub mod s_agentic_tower;
pub mod s_atomic_compositions;
pub mod s_barracuda_precision;
pub mod s_beardog_fido2;
pub mod s_bearer_token_auth;
pub mod s_biomeos_neural_api;
pub mod s_composition_lifecycle;
pub mod s_composition_parity;
pub mod s_compute_triangle;
pub mod s_coordination_api;
pub mod s_crypto_identity_surface;
pub mod s_coralreef_shader_targets;
pub mod s_dark_forest_gate;
pub mod s_deployment_pipeline;
pub mod s_covalent_bond;
mod covalent_mesh_trust;
pub mod s_covalent_mesh;
pub mod s_cross_gate_capability_call;
pub mod s_plasmodium_collective;
pub mod s_cross_spring_data_flow;
pub mod s_domain_contract_sweep;
pub mod s_feedback_loop;
pub mod s_ferment_transcript;
pub mod s_full_nucleus;
pub mod s_routing_consistency;
pub mod s_schema_standard;
pub mod s_gate_failure;
pub mod s_health_lifecycle_surface;
pub mod s_ionic_bond;
pub mod s_loam_certificate_lifecycle;
pub mod s_meta_tier_compositions;
pub mod s_nest_atomic;
pub mod s_nest_commit_live;
pub mod s_nestgate_content_pipeline;
pub mod s_neural_dispatch_live;
pub mod s_neural_routing;
pub mod s_node_atomic;
pub mod s_observatory_parity;
pub mod s_primal_announce;
pub mod s_provenance_trio_pipeline;
pub mod s_sequential_graph;
pub mod s_composition_dispatch_parity;
pub mod s_socket_discovery;
pub mod s_sporeprint_pure_primal;
pub mod s_sporeprint_surface;
pub mod s_startup_ordering;
pub mod s_tier2_science_api;
pub mod s_token_federation;
pub mod s_tower_atomic;
pub mod s_cephalization;
pub mod s_ecosystem_freshness;
pub mod s_tower_cns;
pub mod s_tcp_fallback;
pub mod s_zero_port_standard;

/// Build the canonical scenario registry with all absorbed scenarios.
#[must_use]
pub fn build_registry() -> ScenarioRegistry {
    let mut r = ScenarioRegistry::new();
    r.register(s_tower_atomic::SCENARIO);
    r.register(s_node_atomic::SCENARIO);
    r.register(s_nest_atomic::SCENARIO);
    r.register(s_full_nucleus::SCENARIO);
    r.register(s_startup_ordering::SCENARIO);
    r.register(s_sequential_graph::SCENARIO);
    r.register(s_covalent_bond::SCENARIO);
    r.register(s_covalent_mesh::SCENARIO);
    r.register(s_plasmodium_collective::SCENARIO);
    r.register(s_ionic_bond::SCENARIO);
    r.register(s_gate_failure::SCENARIO);
    r.register(s_cross_spring_data_flow::SCENARIO);
    r.register(s_compute_triangle::SCENARIO);
    r.register(s_socket_discovery::SCENARIO);
    r.register(s_bearer_token_auth::SCENARIO);
    r.register(s_biomeos_neural_api::SCENARIO);
    r.register(s_composition_parity::SCENARIO);
    r.register(s_nestgate_content_pipeline::SCENARIO);
    r.register(s_token_federation::SCENARIO);
    r.register(s_composition_lifecycle::SCENARIO);
    r.register(s_domain_contract_sweep::SCENARIO);
    r.register(s_routing_consistency::SCENARIO);
    r.register(s_zero_port_standard::SCENARIO);
    r.register(s_tier2_science_api::SCENARIO);
    r.register(s_barracuda_precision::SCENARIO);
    r.register(s_coralreef_shader_targets::SCENARIO);
    r.register(s_dark_forest_gate::SCENARIO);
    r.register(s_atomic_compositions::SCENARIO);
    r.register(s_meta_tier_compositions::SCENARIO);
    r.register(s_agentic_tower::SCENARIO);
    r.register(s_provenance_trio_pipeline::SCENARIO);
    r.register(s_ferment_transcript::SCENARIO);
    r.register(s_loam_certificate_lifecycle::SCENARIO);
    r.register(s_beardog_fido2::SCENARIO);
    r.register(s_composition_dispatch_parity::SCENARIO);
    r.register(s_primal_announce::SCENARIO);
    r.register(s_schema_standard::SCENARIO);
    r.register(s_nest_commit_live::SCENARIO);
    r.register(s_sporeprint_pure_primal::SCENARIO);
    r.register(s_sporeprint_surface::SCENARIO);
    r.register(s_cross_gate_capability_call::SCENARIO);
    r.register(s_neural_routing::SCENARIO);
    r.register(s_neural_dispatch_live::SCENARIO);
    r.register(s_observatory_parity::SCENARIO);
    r.register(s_feedback_loop::SCENARIO);
    r.register(s_coordination_api::SCENARIO);
    r.register(s_health_lifecycle_surface::SCENARIO);
    r.register(s_crypto_identity_surface::SCENARIO);
    r.register(s_cephalization::SCENARIO);
    r.register(s_tower_cns::SCENARIO);
    r.register(s_ecosystem_freshness::SCENARIO);
    r.register(s_deployment_pipeline::SCENARIO);
    r.register(s_tcp_fallback::SCENARIO);
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;
    use std::collections::HashSet;

    const EXPECTED_SCENARIO_COUNT: usize = 53;

    #[test]
    fn registry_scenario_count() {
        let r = build_registry();
        assert_eq!(
            r.len(),
            EXPECTED_SCENARIO_COUNT,
            "build_registry() should return {EXPECTED_SCENARIO_COUNT} scenarios, got {}",
            r.len()
        );
    }

    #[test]
    fn registry_no_duplicate_ids() {
        let r = build_registry();
        let mut seen = HashSet::new();
        for s in r.all() {
            assert!(
                seen.insert(s.meta.id),
                "duplicate scenario id: {}",
                s.meta.id
            );
        }
    }

    #[test]
    fn registry_all_tracks_covered() {
        let r = build_registry();
        let tracks: HashSet<String> = r.all().iter().map(|s| s.meta.track.to_string()).collect();
        let expected = [
            "atomic-composition",
            "graph-execution",
            "cross-spring",
            "bonding",
            "transport",
            "security",
            "biomeos-deploy",
            "infrastructure",
            "lifecycle",
        ];
        for track in &expected {
            assert!(
                tracks.contains(*track),
                "Track '{track}' has no registered scenario"
            );
        }
    }

    #[test]
    fn registry_all_rust_tier_pass() {
        // Pre-existing known failures in Rust-tier scenarios that predate this
        // meta-test. Track them explicitly so we notice when they get fixed
        // (update the list) or when new failures appear (fail loudly).
        // Wave 107: skunkBat TCP 9750 resolved — zero known debt.
        const KNOWN_DEBT: &[(&str, u32)] = &[];

        let r = build_registry();
        let mut ctx = CompositionContext::discover();
        for s in r.filter_by_tier(Tier::Rust) {
            if s.meta.tier != Tier::Rust {
                continue;
            }
            let mut v = ValidationResult::new(s.meta.id);
            (s.run)(&mut v, &mut ctx);

            let expected_failures = KNOWN_DEBT
                .iter()
                .find(|(id, _)| *id == s.meta.id)
                .map_or(0, |(_, n)| *n);
            assert_eq!(
                v.failed, expected_failures,
                "Rust-tier scenario '{}': expected {} failures (known debt), got {}",
                s.meta.id, expected_failures, v.failed
            );
        }
    }

    #[test]
    fn registry_valid_provenance_dates() {
        let r = build_registry();
        for s in r.all() {
            let date = s.meta.provenance_date;
            assert!(
                date.len() == 10
                    && date.as_bytes()[4] == b'-'
                    && date.as_bytes()[7] == b'-'
                    && date[..4].parse::<u16>().is_ok()
                    && date[5..7].parse::<u8>().is_ok()
                    && date[8..10].parse::<u8>().is_ok(),
                "scenario '{}' has invalid provenance_date: '{date}' (expected YYYY-MM-DD)",
                s.meta.id
            );
        }
    }
}
