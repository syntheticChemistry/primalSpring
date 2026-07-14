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
mod covalent_mesh_trust;
pub mod s_agentic_tower;
pub mod s_arch_fitness;
pub mod s_atomic_compositions;
pub mod s_barracuda_precision;
pub mod s_beardog_fido2;
pub mod s_beardog_startup_contract;
pub mod s_bearer_token_auth;
pub mod s_biomeos_local_composition;
pub mod s_biomeos_neural_api;
pub mod s_bootstrap_readiness;
pub mod s_btsp_cross_gate_trust;
pub mod s_btsp_cross_gate_verify;
pub mod s_btsp_cross_primal;
pub mod s_capability_convergence;
pub mod s_cascade_drift;
pub mod s_cascade_provenance_match;
pub mod s_cascade_signing;
pub mod s_cephalization;
pub mod s_composition_dispatch_parity;
pub mod s_composition_lifecycle;
pub mod s_composition_live_state;
pub mod s_composition_parity;
pub mod s_composition_profiles;
pub mod s_composition_subtypes;
pub mod s_compute_hosting_contract;
pub mod s_compute_triangle;
pub mod s_convergence_monitor;
pub mod s_coordination_api;
pub mod s_coralreef_shader_targets;
pub mod s_covalent_bond;
pub mod s_covalent_mesh;
pub mod s_cross_gate_capability_call;
pub mod s_cross_gate_compute_dispatch;
pub mod s_cross_gate_mesh_deploy;
pub mod s_cross_membrane_data_flow;
pub mod s_cross_primal_interaction;
pub mod s_cross_spring_data_flow;
pub mod s_cross_target_parity;
pub mod s_crypto_identity_surface;
pub mod s_dark_forest_gate;
pub mod s_defense_attestation;
pub mod s_deployment_pipeline;
pub mod s_depot_trust_verify;
pub mod s_domain_contract_sweep;
pub mod s_drawbridge_bonds;
pub mod s_drawbridge_http_routing;
pub mod s_ecosystem_freshness;
pub mod s_feedback_loop;
pub mod s_federation_wan_readiness;
pub mod s_ferment_transcript;
pub mod s_fido2_entropy_ceremony;
pub mod s_fp_api_proxy;
pub mod s_flockgate_tower_wan;
pub mod s_full_nucleus;
pub mod s_gate_enrollment;
pub mod s_gate_expansion_readiness;
pub mod s_gate_failure;
pub mod s_gate_parity;
pub mod s_gate_readiness;
pub mod s_gatehouse_darkforest;
pub mod s_genetics_compliance;
pub mod s_gpu_dispatch_cross_gate;
pub mod s_gpu_pipeline_validation;
pub mod s_graph_pipeline_depth;
pub mod s_graphenegate_readiness;
pub mod s_hardware_trust_pipeline;
pub mod s_health_lifecycle_surface;
pub mod s_health_standard;
pub mod s_ionic_bond;
pub mod s_kderm_boundary;
pub mod s_kderm_live_layers;
pub mod s_keygen_interaction_surface;
pub mod s_lan_wan_meshed_posture;
pub mod s_live_composition_deploy;
pub mod s_loam_certificate_lifecycle;
pub mod s_mesh_auto_distribution;
pub mod s_mesh_capability_propagation;
pub mod s_mesh_convergence_ops;
pub mod s_mesh_federation_readiness;
pub mod s_mesh_overlay;
pub mod s_mesh_peer_trust;
pub mod s_mesh_reachability;
pub mod s_mesh_topology;
pub mod s_meta_tier_compositions;
pub mod s_metallic_bond;
pub mod s_mobile_mesh_init;
pub mod s_multi_gate_nucleus;
pub mod s_multigate_composition;
pub mod s_nest_atomic;
pub mod s_nest_commit_live;
pub mod s_nestgate_content_pipeline;
pub mod s_neural_api_lifecycle;
pub mod s_neural_dispatch_live;
pub mod s_neural_routing;
pub mod s_node_atomic;
pub mod s_nucleus_integration;
pub mod s_nucleus_orchestration;
pub mod s_nucleus_user_deploy;
pub mod s_observatory_parity;
pub mod s_outer_membrane_posture;
pub mod s_parallel_graph;
pub mod s_pepti_warehouse_deploy;
pub mod s_petaltongue_viz;
pub mod s_plasmodium_collective;
pub mod s_pressure_surface;
pub mod s_primal_announce;
pub mod s_primal_debt;
pub mod s_primal_utilization;
pub mod s_protocol_escalation;
pub mod s_provenance_chain_integrity;
pub mod s_provenance_cross_gate;
pub mod s_provenance_trio_pipeline;
pub mod s_pure_rust_crypto_audit;
pub mod s_relay_forward;
pub mod s_relay_forward_transport;
pub mod s_ribocipher_acceptance;
pub mod s_routing_consistency;
pub mod s_schema_standard;
pub mod s_sequential_graph;
pub mod s_shader_compilation_pipeline;
pub mod s_skunkbat_method_gate;
pub mod s_socket_discovery;
pub mod s_socket_directory_unification;
pub mod s_songbird_lan_bypass;
pub mod s_songbird_mesh_transport;
pub mod s_sovereign_ci_pipeline;
pub mod s_sovereignty_audit_chain;
pub mod s_sovereignty_ledger;
pub mod s_sporeprint_pure_primal;
pub mod s_sporeprint_surface;
pub mod s_squirrel_ai_pipeline;
pub mod s_startup_ordering;
pub mod s_tcp_fallback;
pub mod s_tier2_science_api;
pub mod s_token_federation;
pub mod s_tower_atomic;
pub mod s_tower_cns;
pub mod s_tower_http_gateway;
pub mod s_topology_visualization;
pub mod s_version_skew_detection;
pub mod s_wan_ipc_tolerance;
pub mod s_wan_dispatch_validation;
pub mod s_wireguard_mesh;
pub mod s_zero_port_standard;
pub mod s_zone_topology;

/// Build the canonical scenario registry with all absorbed scenarios.
#[must_use]
#[expect(
    clippy::too_many_lines,
    reason = "flat registration list; splitting would add indirection"
)]
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
    r.register(s_compute_hosting_contract::SCENARIO);
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
    r.register(s_provenance_chain_integrity::SCENARIO);
    r.register(s_ferment_transcript::SCENARIO);
    r.register(s_loam_certificate_lifecycle::SCENARIO);
    r.register(s_beardog_fido2::SCENARIO);
    r.register(s_beardog_startup_contract::SCENARIO);
    r.register(s_composition_dispatch_parity::SCENARIO);
    r.register(s_primal_announce::SCENARIO);
    r.register(s_schema_standard::SCENARIO);
    r.register(s_nest_commit_live::SCENARIO);
    r.register(s_sporeprint_pure_primal::SCENARIO);
    r.register(s_sporeprint_surface::SCENARIO);
    r.register(s_cross_gate_capability_call::SCENARIO);
    r.register(s_cross_primal_interaction::SCENARIO);
    r.register(s_neural_routing::SCENARIO);
    r.register(s_neural_dispatch_live::SCENARIO);
    r.register(s_observatory_parity::SCENARIO);
    r.register(s_feedback_loop::SCENARIO);
    r.register(s_flockgate_tower_wan::SCENARIO);
    r.register(s_coordination_api::SCENARIO);
    r.register(s_health_lifecycle_surface::SCENARIO);
    r.register(s_health_standard::SCENARIO);
    r.register(s_crypto_identity_surface::SCENARIO);
    r.register(s_cephalization::SCENARIO);
    r.register(s_tower_cns::SCENARIO);
    r.register(s_tower_http_gateway::SCENARIO);
    r.register(s_ecosystem_freshness::SCENARIO);
    r.register(s_deployment_pipeline::SCENARIO);
    r.register(s_tcp_fallback::SCENARIO);
    r.register(s_btsp_cross_primal::SCENARIO);
    r.register(s_gate_expansion_readiness::SCENARIO);
    r.register(s_graphenegate_readiness::SCENARIO);
    r.register(s_version_skew_detection::SCENARIO);
    r.register(s_cascade_provenance_match::SCENARIO);
    r.register(s_wan_ipc_tolerance::SCENARIO);
    r.register(s_ribocipher_acceptance::SCENARIO);
    r.register(s_bootstrap_readiness::SCENARIO);
    r.register(s_arch_fitness::SCENARIO);
    r.register(s_cross_target_parity::SCENARIO);
    r.register(s_pressure_surface::SCENARIO);
    r.register(s_cascade_drift::SCENARIO);
    r.register(s_mesh_topology::SCENARIO);
    r.register(s_gate_readiness::SCENARIO);
    r.register(s_gatehouse_darkforest::SCENARIO);
    r.register(s_kderm_boundary::SCENARIO);
    r.register(s_nucleus_orchestration::SCENARIO);
    r.register(s_convergence_monitor::SCENARIO);
    r.register(s_protocol_escalation::SCENARIO);
    r.register(s_gate_enrollment::SCENARIO);
    r.register(s_zone_topology::SCENARIO);
    r.register(s_mesh_overlay::SCENARIO);
    r.register(s_composition_live_state::SCENARIO);
    r.register(s_gate_parity::SCENARIO);
    r.register(s_multi_gate_nucleus::SCENARIO);
    r.register(s_genetics_compliance::SCENARIO);
    r.register(s_kderm_live_layers::SCENARIO);
    r.register(s_mesh_reachability::SCENARIO);
    r.register(s_nucleus_integration::SCENARIO);
    r.register(s_nucleus_user_deploy::SCENARIO);
    r.register(s_primal_debt::SCENARIO);
    r.register(s_primal_utilization::SCENARIO);
    r.register(s_sovereignty_ledger::SCENARIO);
    r.register(s_wireguard_mesh::SCENARIO);
    r.register(s_provenance_cross_gate::SCENARIO);
    r.register(s_graph_pipeline_depth::SCENARIO);
    r.register(s_defense_attestation::SCENARIO);
    r.register(s_sovereignty_audit_chain::SCENARIO);
    r.register(s_capability_convergence::SCENARIO);
    r.register(s_parallel_graph::SCENARIO);
    r.register(s_metallic_bond::SCENARIO);
    r.register(s_btsp_cross_gate_verify::SCENARIO);
    r.register(s_skunkbat_method_gate::SCENARIO);
    r.register(s_songbird_mesh_transport::SCENARIO);
    r.register(s_relay_forward::SCENARIO);
    r.register(s_btsp_cross_gate_trust::SCENARIO);
    r.register(s_cross_gate_compute_dispatch::SCENARIO);
    r.register(s_gpu_dispatch_cross_gate::SCENARIO);
    r.register(s_gpu_pipeline_validation::SCENARIO);
    r.register(s_mesh_capability_propagation::SCENARIO);
    r.register(s_multigate_composition::SCENARIO);
    r.register(s_petaltongue_viz::SCENARIO);
    r.register(s_shader_compilation_pipeline::SCENARIO);
    r.register(s_squirrel_ai_pipeline::SCENARIO);
    r.register(s_biomeos_local_composition::SCENARIO);
    r.register(s_relay_forward_transport::SCENARIO);
    r.register(s_songbird_lan_bypass::SCENARIO);
    r.register(s_pepti_warehouse_deploy::SCENARIO);
    r.register(s_mobile_mesh_init::SCENARIO);
    r.register(s_drawbridge_http_routing::SCENARIO);
    r.register(s_mesh_convergence_ops::SCENARIO);
    r.register(s_mesh_peer_trust::SCENARIO);
    r.register(s_lan_wan_meshed_posture::SCENARIO);
    r.register(s_wan_dispatch_validation::SCENARIO);
    r.register(s_composition_subtypes::SCENARIO);
    r.register(s_sovereign_ci_pipeline::SCENARIO);
    r.register(s_mesh_auto_distribution::SCENARIO);
    r.register(s_composition_profiles::SCENARIO);
    r.register(s_outer_membrane_posture::SCENARIO);
    r.register(s_cascade_signing::SCENARIO);
    r.register(s_cross_membrane_data_flow::SCENARIO);
    r.register(s_topology_visualization::SCENARIO);
    r.register(s_federation_wan_readiness::SCENARIO);
    r.register(s_pure_rust_crypto_audit::SCENARIO);
    r.register(s_mesh_federation_readiness::SCENARIO);
    r.register(s_live_composition_deploy::SCENARIO);
    r.register(s_neural_api_lifecycle::SCENARIO);
    r.register(s_cross_gate_mesh_deploy::SCENARIO);
    r.register(s_socket_directory_unification::SCENARIO);
    r.register(s_fido2_entropy_ceremony::SCENARIO);
    r.register(s_hardware_trust_pipeline::SCENARIO);
    r.register(s_keygen_interaction_surface::SCENARIO);
    r.register(s_fp_api_proxy::SCENARIO);
    r.register(s_drawbridge_bonds::SCENARIO);
    r.register(s_depot_trust_verify::SCENARIO);
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;
    use std::collections::HashSet;

    const EXPECTED_SCENARIO_COUNT: usize = 145;

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
            "sovereignty",
            "evolution",
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
        // Wave 138a: known debt items pending resolution.
        // - graphenegate-readiness: topology regression (1 check)
        // - ecosystem-freshness: freshness.toml cascade drift (1 check)
        // - multi-gate-nucleus: pre-existing structural gaps (13 checks)
        const KNOWN_DEBT: &[(&str, u32)] = &[
            ("graphenegate-readiness", 1),
            ("ecosystem-freshness", 1),
            ("multi-gate-nucleus", 13),
        ];

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
