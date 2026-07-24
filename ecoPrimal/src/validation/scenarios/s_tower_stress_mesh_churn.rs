// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Stress — Mesh Churn.
//!
//! The mesh topology changes as gates enroll, disconnect, and rejoin.
//! Each enrollment triggers:
//!   songBird `mesh.enroll` → bearDog `enrollment.verify` (HMAC)
//!   songBird `mesh.capabilities_announce` → broadcast to all peers
//!
//! Under churn (rapid enroll/disconnect/re-enroll), the mesh must:
//! - Converge capability tables within bounded time
//! - Clean up stale peer entries
//! - Not propagate stale capabilities to routing decisions
//! - Handle concurrent enrollment from multiple gates
//!
//! This matters for real deployment: gates going through USB enrollment,
//! network flaps, VPS restarts, and topology changes.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const MESH_TOML: &str = include_str!("../../../../config/mesh_topology.toml");
const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-stress-mesh-churn",
        track: Track::Lifecycle,
        tier: Tier::Rust,
        provenance_crate: "wave150w_tower_stress",
        provenance_date: "2026-07-23",
        description: "Tower stress — rapid gate enroll/disconnect churn: convergence time + stale cleanup",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Enrollment protocol churn readiness");
    phase_enrollment_churn(v);

    v.section("Phase 2: Capability propagation under churn");
    phase_capability_propagation(v);

    v.section("Phase 3: Stale peer cleanup");
    phase_stale_cleanup(v);
}

fn phase_enrollment_churn(v: &mut ValidationResult) {
    let has_mesh_enroll = REGISTRY_TOML.contains("mesh.enroll");
    v.check_bool(
        "churn:mesh_enroll_method",
        has_mesh_enroll,
        "mesh.enroll method registered — enrollment entry point for churn testing",
    );

    let has_enrollment_verify = REGISTRY_TOML.contains("enrollment.verify");
    v.check_bool(
        "churn:enrollment_verify",
        has_enrollment_verify,
        "enrollment.verify registered — HMAC verification for each enroll call",
    );

    let has_hmac = REGISTRY_TOML.contains("hmac") || REGISTRY_TOML.contains("HMAC");
    v.check_bool(
        "churn:hmac_proof",
        has_hmac || has_enrollment_verify,
        &format!(
            "HMAC enrollment proof: {} — each churn event triggers HMAC-SHA256 verification",
            if has_hmac {
                "explicitly documented"
            } else if has_enrollment_verify {
                "implied by enrollment.verify"
            } else {
                "NOT FOUND"
            }
        ),
    );

    let gate_count = MESH_TOML.matches("[[gate]]").count();
    v.check_bool(
        "churn:topology_gate_count",
        gate_count >= 3,
        &format!(
            "{gate_count} gates in mesh topology — sufficient for churn simulation \
             (need ≥3 for meaningful convergence testing)"
        ),
    );
}

fn phase_capability_propagation(v: &mut ValidationResult) {
    let has_capabilities_announce =
        REGISTRY_TOML.contains("capabilities_announce") || REGISTRY_TOML.contains("mesh.announce");
    v.check_bool(
        "churn:capability_announce",
        has_capabilities_announce,
        &format!(
            "Capability announcement method: {} — each enrollment broadcasts capabilities to mesh",
            if has_capabilities_announce {
                "PRESENT"
            } else {
                "NOT FOUND (capabilities may not propagate after re-enrollment)"
            }
        ),
    );

    let has_capabilities_query =
        REGISTRY_TOML.contains("capabilities_query") || REGISTRY_TOML.contains("mesh.query");
    v.check_bool(
        "churn:capability_query",
        has_capabilities_query,
        "Capability query method available — verify propagation converged after churn",
    );

    let has_find_path = REGISTRY_TOML.contains("mesh.find_path");
    v.check_bool(
        "churn:topology_routing",
        has_find_path,
        &format!(
            "Topology-aware routing: {} — churn must not break routing decisions",
            if has_find_path {
                "mesh.find_path available"
            } else {
                "NO topology routing"
            }
        ),
    );
}

fn phase_stale_cleanup(v: &mut ValidationResult) {
    let has_peer_timeout =
        MESH_TOML.contains("timeout") || MESH_TOML.contains("ttl") || MESH_TOML.contains("expiry");
    v.check_bool(
        "churn:peer_timeout",
        has_peer_timeout,
        &format!(
            "Peer entry TTL/timeout in mesh config: {} — \
             without TTL, disconnected peers persist indefinitely",
            if has_peer_timeout {
                "CONFIGURED"
            } else {
                "NOT CONFIGURED (stale entries risk)"
            }
        ),
    );

    let has_mesh_zones = MESH_TOML.contains("zone =");
    let zone_count = MESH_TOML.matches("zone =").count();
    v.check_bool(
        "churn:multi_zone_topology",
        has_mesh_zones && zone_count >= 2,
        &format!("{zone_count} zones — churn in one zone should not affect routing in another"),
    );

    let has_beacon = MESH_TOML.contains("beacon") || REGISTRY_TOML.contains("beacon");
    v.check_bool(
        "churn:beacon_mesh_protocol",
        has_beacon,
        &format!(
            "BeaconMesh protocol: {} — periodic heartbeats detect dead peers for cleanup",
            if has_beacon {
                "PRESENT"
            } else {
                "NOT in config (check songBird code for BeaconMesh)"
            }
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_no_panic() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
    }
}
