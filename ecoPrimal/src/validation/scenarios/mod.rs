// SPDX-License-Identifier: AGPL-3.0-or-later

//! Validation scenarios — absorbed experiment patterns.
//!
//! Each scenario is a self-contained validation function that exercises
//! a specific NUCLEUS composition pattern. Scenarios evolved from the
//! prokaryotic experiment binary era (exp001–exp111) and were absorbed
//! into the library at the interstadial transition.
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
pub mod s_barracuda_precision;
pub mod s_bearer_token_auth;
pub mod s_biomeos_neural_api;
pub mod s_biomeos_tower_deploy;
pub mod s_cellular_deployment;
pub mod s_composition_lifecycle;
pub mod s_composition_parity;
pub mod s_compute_triangle;
pub mod s_coralreef_shader_targets;
pub mod s_dark_forest_gate;
pub mod s_deployment_pipeline;
pub mod s_covalent_bond;
pub mod s_cross_spring_data_flow;
pub mod s_deployment_matrix;
pub mod s_domain_contract_sweep;
pub mod s_full_nucleus;
pub mod s_routing_consistency;
pub mod s_gate_failure;
pub mod s_ionic_bond;
pub mod s_nest_atomic;
pub mod s_nestgate_content_pipeline;
pub mod s_node_atomic;
pub mod s_sequential_graph;
pub mod s_socket_discovery;
pub mod s_startup_ordering;
pub mod s_tier2_science_api;
pub mod s_token_federation;
pub mod s_tower_atomic;
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
    r.register(s_ionic_bond::SCENARIO);
    r.register(s_gate_failure::SCENARIO);
    r.register(s_cross_spring_data_flow::SCENARIO);
    r.register(s_compute_triangle::SCENARIO);
    r.register(s_socket_discovery::SCENARIO);
    r.register(s_bearer_token_auth::SCENARIO);
    r.register(s_biomeos_tower_deploy::SCENARIO);
    r.register(s_biomeos_neural_api::SCENARIO);
    r.register(s_deployment_matrix::SCENARIO);
    r.register(s_composition_parity::SCENARIO);
    r.register(s_nestgate_content_pipeline::SCENARIO);
    r.register(s_cellular_deployment::SCENARIO);
    r.register(s_token_federation::SCENARIO);
    r.register(s_composition_lifecycle::SCENARIO);
    r.register(s_domain_contract_sweep::SCENARIO);
    r.register(s_routing_consistency::SCENARIO);
    r.register(s_zero_port_standard::SCENARIO);
    r.register(s_tier2_science_api::SCENARIO);
    r.register(s_barracuda_precision::SCENARIO);
    r.register(s_coralreef_shader_targets::SCENARIO);
    r.register(s_dark_forest_gate::SCENARIO);
    r.register(s_deployment_pipeline::SCENARIO);
    r
}
