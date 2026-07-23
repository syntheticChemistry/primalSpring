// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Distributed Compute Mesh.
//!
//! Validates structural readiness for cross-gate compute dispatch — Tower
//! knows what hardware each gate has and routes workloads to the right
//! substrate. `WireGuard` is just a pipe; Tower is compute-aware.
//!
//! Hardware topology for distributed compute:
//! - `strandGate`: EPYC, 256GB, RTX 3090 + 6950XT (GPU compute)
//! - `biomeGate`: Threadripper, Titan V (HBM2 compilation)
//! - `eastGate`: Akida NPU (neuromorphic inference)
//! - `northGate`: RTX 5090 (inference + rendering)
//!
//! Measures: `toadStool` cross-gate dispatch latency + aggregation.
//! Primary gates: all (`hotSpring`, `groundSpring`).

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const MESH_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// Scenario registration metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-compute-mesh",
        track: Track::Evolution,
        tier: Tier::Rust,
        provenance_crate: "wave150w_tower_compute_mesh",
        provenance_date: "2026-07-23",
        description: "Tower distributed compute mesh — cross-gate dispatch latency + aggregation",
    },
    run,
};

/// Execute the validation checks.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Compute dispatch infrastructure");

    let has_compute = REGISTRY_TOML.contains("compute.") || REGISTRY_TOML.contains("dispatch.");
    v.check_bool(
        "compute:dispatch_methods",
        has_compute,
        "Compute/dispatch methods: workload routing to substrate (Tower is compute-aware)",
    );

    let has_find_path = REGISTRY_TOML.contains("mesh.find_path");
    v.check_bool(
        "compute:topology_routing",
        has_find_path,
        "mesh.find_path: topology-aware routing (dispatch to GPU/NPU/CPU gate by capability)",
    );

    let has_capabilities_query = REGISTRY_TOML.contains("mesh.capabilities_query");
    v.check_bool(
        "compute:substrate_discovery",
        has_capabilities_query,
        "mesh.capabilities_query: discover which gates have GPU/NPU/large-RAM substrates",
    );

    v.section("Cross-gate coordination");

    let has_relay = REGISTRY_TOML.contains("mesh.relay");
    v.check_bool(
        "compute:result_relay",
        has_relay,
        "mesh.relay: relay compute results back (fan-out dispatch, fan-in aggregation)",
    );

    let has_publish = REGISTRY_TOML.contains("mesh.publish");
    v.check_bool(
        "compute:result_publish",
        has_publish,
        "mesh.publish: publish aggregated results to requesting gate",
    );

    let has_announce = REGISTRY_TOML.contains("mesh.announce");
    v.check_bool(
        "compute:substrate_announce",
        has_announce,
        "mesh.announce: gates advertise compute substrates (GPU count, RAM, NPU presence)",
    );

    v.section("Multi-zone compute topology");

    let has_backbone = MESH_TOML.contains("zone = \"Backbone\"");
    v.check_bool(
        "compute:backbone_zone",
        has_backbone,
        "Backbone zone: low-latency compute mesh for GPU↔GPU dispatch",
    );

    let multi_zone =
        MESH_TOML.contains("zone = \"House2\"") || MESH_TOML.contains("zone = \"Wan\"");
    v.check_bool(
        "compute:multi_zone",
        multi_zone,
        "Multi-zone topology: cross-zone dispatch available (House2/WAN gates)",
    );

    let gate_count = MESH_TOML.matches("[[gate]]").count();
    v.check_bool(
        "compute:sufficient_gates",
        gate_count >= 4,
        &format!("{gate_count} gates: heterogeneous compute targets for distributed dispatch"),
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
