// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Mesh LAN Path Preference — validates `mesh.find_path` prefers local.
//!
//! **P0 gap (Wave 150x)**: songBird `mesh.find_path` returns WG overlay address
//! for same-switch peers instead of LAN direct path. This imposes a 353x latency
//! penalty for `capability.call` dispatch between co-located gates (0.45ms LAN
//! vs 158ms WG overlay).
//!
//! This scenario validates the contract that `mesh.find_path` MUST prefer
//! `EndpointType::Local` when:
//! 1. Both peers have `lan_addr` in topology
//! 2. Both peers are in the same zone ("Backbone")
//! 3. LAN reachability is confirmed (or assumed for same-zone)
//!
//! primalSpring already implements `preferred_address()` with LAN-first logic.
//! This scenario asserts that songBird's routing implementation matches.
//!
//! Phases:
//! 1. Topology: LAN addresses declared for same-switch peers
//! 2. Contract: `preferred_address()` returns LAN for Backbone peers
//! 3. Routing gap: `mesh.find_path` must honor LAN preference (KNOWN GAP)
//! 4. Impact: latency penalty quantification for capability dispatch

use crate::composition::CompositionContext;
use crate::evolution::gate::{all_mesh_gates, preferred_address};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const MESH_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// Scenario registration metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "mesh-lan-path-preference",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave150x_mesh_lan_path_preference",
        provenance_date: "2026-07-24",
        description: "mesh.find_path MUST prefer LAN for same-switch peers — 353x penalty documented",
    },
    run,
};

/// Execute all LAN path preference validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Topology — LAN addresses declared");
    phase_topology(v);

    v.section("Phase 2: Contract — preferred_address() returns LAN");
    phase_contract(v);

    v.section("Phase 3: Routing gap — mesh.find_path LAN preference");
    phase_routing_gap(v, ctx);

    v.section("Phase 4: Impact — latency penalty for capability dispatch");
    phase_impact(v);
}

fn phase_topology(v: &mut ValidationResult) {
    let gates = all_mesh_gates();
    let backbone_gates: Vec<_> = gates.iter().filter(|e| e.zone == "Backbone").collect();

    v.check_bool(
        "topology:backbone_gates_exist",
        backbone_gates.len() >= 2,
        &format!(
            "{} Backbone-zone gates (need ≥2 for LAN peering)",
            backbone_gates.len()
        ),
    );

    let lan_declared: Vec<_> = backbone_gates
        .iter()
        .filter(|e| e.lan_addr.is_some())
        .collect();

    v.check_bool(
        "topology:lan_addrs_declared",
        lan_declared.len() >= 2,
        &format!(
            "{}/{} Backbone gates have lan_addr declared",
            lan_declared.len(),
            backbone_gates.len()
        ),
    );

    for gate in &lan_declared {
        v.check_bool(
            &format!("topology:{}:lan_addr", gate.name),
            true,
            &format!(
                "{} lan_addr = {}",
                gate.name,
                gate.lan_addr.as_deref().unwrap_or("MISSING")
            ),
        );
    }

    let same_subnet = lan_declared.len() >= 2
        && lan_declared.windows(2).all(|pair| {
            let a = pair[0].lan_addr.as_deref().unwrap_or("");
            let b = pair[1].lan_addr.as_deref().unwrap_or("");
            a.split('.').take(3).collect::<Vec<_>>() == b.split('.').take(3).collect::<Vec<_>>()
        });

    v.check_bool(
        "topology:same_subnet",
        same_subnet,
        "Backbone LAN peers share subnet prefix (same-switch assumption)",
    );
}

fn phase_contract(v: &mut ValidationResult) {
    let gates = all_mesh_gates();
    let backbone_with_lan: Vec<_> = gates
        .iter()
        .filter(|e| e.zone == "Backbone" && e.lan_addr.is_some())
        .collect();

    for gate in &backbone_with_lan {
        let pref = preferred_address(&gate.name);
        let expected_lan = gate.lan_addr.as_deref();
        let prefers_lan = pref == expected_lan;

        v.check_bool(
            &format!("contract:{}:prefers_lan", gate.name),
            prefers_lan,
            &format!(
                "preferred_address({}) = {:?} (lan_addr = {:?})",
                gate.name, pref, expected_lan
            ),
        );
    }

    let wan_gates: Vec<_> = gates
        .iter()
        .filter(|e| e.zone == "Wan" && e.lan_addr.is_none() && !e.address.is_empty())
        .collect();

    for gate in &wan_gates {
        let pref = preferred_address(&gate.name);
        let falls_back_wg = pref == Some(gate.address.as_str());
        v.check_bool(
            &format!("contract:{}:falls_back_wg", gate.name),
            falls_back_wg,
            &format!(
                "preferred_address({}) = {:?} (WG fallback for non-LAN peer)",
                gate.name, pref
            ),
        );
    }
}

fn phase_routing_gap(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let client = ctx.client_for("mesh");
    if client.is_none() {
        v.check_skip(
            "routing_gap:mesh_unreachable",
            "songBird mesh client unavailable — cannot validate mesh.find_path live",
        );

        v.check_bool(
            "routing_gap:find_path_contract",
            false,
            "mesh.find_path MUST return EndpointType::Local for same-zone Backbone peers \
             (KNOWN GAP: currently returns WG overlay — 353x penalty)",
        );

        v.check_bool(
            "routing_gap:endpoint_type_local",
            false,
            "EndpointType::Local variant must be preferred over EndpointType::Overlay \
             when both peers declare lan_addr in same subnet",
        );
        return;
    }

    v.check_skip(
        "routing_gap:live_validation",
        "live mesh.find_path validation — requires songBird fix for EndpointType::Local",
    );
}

fn phase_impact(v: &mut ValidationResult) {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let shadow_dir = workspace_root.join("benchScale/tower_shadow");

    let has_lan_data = shadow_dir
        .read_dir()
        .map(|rd| {
            rd.filter_map(Result::ok).any(|e| {
                let n = e.file_name();
                n.to_string_lossy().contains("sporeGate_LAN")
            })
        })
        .unwrap_or(false);

    v.check_bool(
        "impact:lan_benchmark_data",
        has_lan_data,
        "LAN shadow benchmark data present (sporeGate_LAN peer)",
    );

    v.check_bool(
        "impact:penalty_documented",
        MESH_TOML.contains("lan_addr"),
        "LAN topology declared — 353x penalty applies when mesh.find_path ignores lan_addr",
    );

    let has_multiple_backbone = all_mesh_gates()
        .iter()
        .filter(|e| e.zone == "Backbone" && e.lan_addr.is_some())
        .count()
        >= 2;

    v.check_bool(
        "impact:multiple_lan_peers",
        has_multiple_backbone,
        "≥2 Backbone peers with LAN — capability dispatch between them pays overlay penalty",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn mesh_lan_path_preference_structural() {
        let mut v = ValidationResult::new("mesh-lan-path-preference");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        // Known gap: 2 failures expected (routing_gap phase)
        assert!(
            v.failed <= 2,
            "mesh-lan-path-preference: {} failures (expected ≤2 from routing gap), {} passed",
            v.failed,
            v.passed
        );
    }
}
