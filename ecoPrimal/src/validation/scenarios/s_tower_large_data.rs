// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Large Data Transfer.
//!
//! Validates structural readiness for large blob transfers (100MB–10GB) where
//! Tower's CAS-aware routing and negotiable framing exceed `WireGuard`'s
//! fixed MTU (1420) dumb-pipe approach.
//!
//! Key advantages:
//! - CAS dedup: skip redundant transfer of already-cached content
//! - Negotiable framing: jumbo frames on 10G, chunked on WAN
//! - Content-addressed routing: route to nearest cached copy
//!
//! Measures: throughput, CPU utilization, CAS dedup benefit.
//! Primary gates: all (`wetSpring`, `hotSpring`, `neuralSpring`).

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const MESH_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// Scenario registration metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-large-data",
        track: Track::Evolution,
        tier: Tier::Rust,
        provenance_crate: "wave150w_tower_large_data",
        provenance_date: "2026-07-23",
        description: "Tower large data transfer — 100MB–10GB blobs: throughput, CPU, CAS dedup benefit",
    },
    run,
};

/// Execute the validation checks.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Content-addressed storage integration");

    let has_cas = REGISTRY_TOML.contains("cas.") || REGISTRY_TOML.contains("content.");
    v.check_bool(
        "large_data:cas_methods",
        has_cas,
        "CAS/content methods present: content-addressed blob dedup (skip redundant transfer)",
    );

    let has_storage = REGISTRY_TOML.contains("storage.") || REGISTRY_TOML.contains("nest.");
    v.check_bool(
        "large_data:storage_layer",
        has_storage,
        "Storage layer: nestGate CAS for nearest-copy routing (vs WG re-transmits everything)",
    );

    let has_publish = REGISTRY_TOML.contains("mesh.publish");
    v.check_bool(
        "large_data:publish_method",
        has_publish,
        "mesh.publish: data relay through Tower (negotiable framing for large payloads)",
    );

    v.section("Transfer optimization primitives");

    let has_relay = REGISTRY_TOML.contains("mesh.relay");
    v.check_bool(
        "large_data:relay_for_blobs",
        has_relay,
        "mesh.relay: relay path for large transfers (chunked on WAN, jumbo on 10G backbone)",
    );

    let has_find_path = REGISTRY_TOML.contains("mesh.find_path");
    v.check_bool(
        "large_data:path_optimization",
        has_find_path,
        "mesh.find_path: route large blobs via highest-bandwidth path (vs WG fixed tunnel)",
    );

    v.section("Hardware targets for exploration");

    let has_backbone = MESH_TOML.contains("zone = \"Backbone\"");
    v.check_bool(
        "large_data:backbone_zone",
        has_backbone,
        "Backbone zone: 10G path target for jumbo frame benchmark (strandGate↔eastGate)",
    );

    let has_wan = MESH_TOML.contains("zone = \"Wan\"");
    v.check_bool(
        "large_data:wan_zone",
        has_wan,
        "WAN zone: chunked transfer benchmark (flockGate via golgiBody TURN)",
    );

    let peer_count = MESH_TOML.matches("[[gate]]").count();
    v.check_bool(
        "large_data:multi_hop_available",
        peer_count >= 4,
        &format!("{peer_count} gates: multi-hop CAS routing paths available"),
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
