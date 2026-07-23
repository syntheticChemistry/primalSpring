// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Multi-Stack Routing.
//!
//! Validates structural readiness for N `songBird` instances on `golgiBody`,
//! each tuned for a different traffic class. `WireGuard` has one tunnel per
//! peer — Tower can run multiple relay stacks (RPC, blob, metrics) with
//! per-purpose tuning.
//!
//! Measures: N `songBird` instances on `golgiBody`, per-purpose tuning.
//! Primary gate: flockGate (Tower primal team).

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const MESH_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// Scenario registration metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-multi-stack",
        track: Track::Evolution,
        tier: Tier::Rust,
        provenance_crate: "wave150w_tower_multi_stack",
        provenance_date: "2026-07-23",
        description: "Tower multi-stack routing — N songBird instances on golgiBody, per-purpose tuning",
    },
    run,
};

/// Execute the validation checks.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Multi-stack relay infrastructure");

    let has_relay = REGISTRY_TOML.contains("mesh.relay");
    v.check_bool(
        "multi_stack:relay_base",
        has_relay,
        "mesh.relay: base relay capability (each instance = one traffic class)",
    );

    let has_mirror = REGISTRY_TOML.contains("mesh.mirror");
    v.check_bool(
        "multi_stack:traffic_mirroring",
        has_mirror,
        "mesh.mirror: traffic mirroring for shadow (dual-stack WG + Tower simultaneous)",
    );

    let golgi_present = MESH_TOML.contains("10.13.37.1");
    v.check_bool(
        "multi_stack:hub_present",
        golgi_present,
        "golgiBody (.1) in topology: multi-stack target host (RPC + blob + relay profiles)",
    );

    v.section("Stack differentiation");

    let has_rpc = REGISTRY_TOML.contains("mesh.connect");
    v.check_bool(
        "multi_stack:rpc_stack",
        has_rpc,
        "mesh.connect: RPC traffic stack (low-latency, small payloads)",
    );

    let has_publish = REGISTRY_TOML.contains("mesh.publish");
    v.check_bool(
        "multi_stack:blob_stack",
        has_publish,
        "mesh.publish: blob/data traffic stack (high-throughput, large payloads)",
    );

    let has_mesh_announce = REGISTRY_TOML.contains("mesh.announce");
    v.check_bool(
        "multi_stack:control_stack",
        has_mesh_announce,
        "mesh.announce: control plane stack (peer advertisement, topology updates)",
    );

    v.section("Per-purpose tuning prerequisites");

    let has_find_path = REGISTRY_TOML.contains("mesh.find_path");
    v.check_bool(
        "multi_stack:route_selection",
        has_find_path,
        "mesh.find_path: route selection per traffic class (RPC→fast, blob→throughput)",
    );

    let has_btsp = REGISTRY_TOML.contains("btsp.negotiate");
    v.check_bool(
        "multi_stack:per_stack_crypto",
        has_btsp,
        "btsp.negotiate: per-stack crypto policy (lightweight for metrics, full for secrets)",
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
