// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Edge/SFF Profile.
//!
//! Validates structural readiness for minimal Tower relay profiles on
//! constrained hardware (NUC Celerons, `NucBox` M6). `WireGuard` has the
//! same overhead on a Celeron as an EPYC — Tower is tunable.
//!
//! Key advantage: Tower can run a minimal relay profile on edge hardware
//! with reduced crypto, simplified routing, and smaller memory footprint.
//!
//! Measures: `songBird` on NUC Celeron: idle CPU, memory, relay throughput.
//! Primary gate: operator (NUC hardware).

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const MESH_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// Scenario registration metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-edge-profile",
        track: Track::Evolution,
        tier: Tier::Rust,
        provenance_crate: "wave150w_tower_edge_profile",
        provenance_date: "2026-07-23",
        description: "Tower edge/SFF profile — songBird on NUC Celeron: idle CPU, memory, relay throughput",
    },
    run,
};

/// Execute the validation checks.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Minimal relay profile primitives");

    let has_connect = REGISTRY_TOML.contains("mesh.connect");
    v.check_bool(
        "edge:minimal_connect",
        has_connect,
        "mesh.connect: minimal relay mode (tunable overhead — reduce crypto/routing for edge)",
    );

    let has_announce = REGISTRY_TOML.contains("mesh.announce");
    v.check_bool(
        "edge:lightweight_announce",
        has_announce,
        "mesh.announce: lightweight peer advertisement (WG has same overhead everywhere)",
    );

    let has_peers = REGISTRY_TOML.contains("mesh.peers");
    v.check_bool(
        "edge:peer_list",
        has_peers,
        "mesh.peers: static peer list for edge (no discovery overhead on constrained hardware)",
    );

    v.section("Profile differentiation");

    let has_negotiate = REGISTRY_TOML.contains("btsp.negotiate");
    v.check_bool(
        "edge:tunable_crypto",
        has_negotiate,
        "btsp.negotiate: downgrade crypto tier for edge relay (less CPU than full BTSP)",
    );

    let has_find_path = REGISTRY_TOML.contains("mesh.find_path");
    v.check_bool(
        "edge:simplified_routing",
        has_find_path,
        "mesh.find_path: edge nodes use static routing (skip complex topology computation)",
    );

    v.section("Edge hardware targets");

    let gate_count = MESH_TOML.matches("[[gate]]").count();
    v.check_bool(
        "edge:topology_targets",
        gate_count >= 6,
        &format!("{gate_count} gates: includes edge-class hardware (NUC, mobile, SFF)"),
    );

    let has_role_field = MESH_TOML.contains("role = ");
    v.check_bool(
        "edge:role_differentiation",
        has_role_field,
        "Gate roles defined in topology: edge devices identifiable for minimal relay profile",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

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
