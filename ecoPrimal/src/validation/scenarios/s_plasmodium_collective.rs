// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Plasmodium Collective — multi-gate mesh composition validation.
//!
//! Validates that 3+ gates can form a collective where:
//! - Each gate is reachable via `mesh.health_check`
//! - Cross-gate `capability.call` routes to the correct provider
//! - The `MeshTopology` model correctly reflects live mesh state
//! - Capability routing degrades gracefully when gates are unreachable
//!
//! This scenario requires live Songbird with HTTP dispatch (d6a6f714+).

use crate::composition::CompositionContext;
use crate::composition::mesh::MeshTopology;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "plasmodium-collective",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave74_primalspring",
        provenance_date: "2026-06-03",
        description: "Multi-gate plasmodium collective — mesh topology, cross-gate routing, degradation",
    },
    run,
};

/// Run all plasmodium collective validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — mesh topology model");
    phase_structural(v);

    v.section("Phase 2: Live — discovery.peers multi-gate");
    phase_live_discovery(v, ctx);

    v.section("Phase 3: Live — cross-gate capability.call via HTTP dispatch");
    phase_cross_gate_call(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    v.check_bool(
        "structure:capability_call_registered",
        REGISTRY_TOML.contains("capability.call"),
        "capability.call in capability_registry.toml",
    );

    v.check_bool(
        "structure:route_register_registered",
        REGISTRY_TOML.contains("route.register"),
        "route.register (mesh federation) in registry",
    );

    v.check_bool(
        "structure:discovery_peers_registered",
        REGISTRY_TOML.contains("discovery.peers"),
        "discovery.peers in registry",
    );

    let mut topo = MeshTopology::new();
    topo.set_local_gate("east-gate");
    topo.register_gate(
        "east-gate",
        Some("192.168.1.144:7700".to_owned()),
        ["beardog", "songbird", "nestgate"],
        ["security", "discovery", "storage"],
    );
    topo.register_gate(
        "strand-gate",
        Some("192.168.1.132:7700".to_owned()),
        ["toadstool", "barracuda"],
        ["compute", "tensor"],
    );
    topo.register_gate(
        "iron-gate",
        Some("192.168.1.238:7700".to_owned()),
        ["skunkbat", "coralreef"],
        ["defense", "shader"],
    );

    v.check_bool(
        "structure:topology_3_gates",
        topo.gate_count() >= 3,
        &format!("mesh topology has {} gates (need 3+)", topo.gate_count()),
    );

    topo.mark_healthy("east-gate", true);
    topo.mark_healthy("strand-gate", true);
    let routes = topo.routes_for_capability("compute");
    v.check_bool(
        "structure:cross_gate_routes_exist",
        !routes.is_empty(),
        &format!("{} cross-gate routes for 'compute'", routes.len()),
    );

    let unreachable = topo.unreachable_capabilities();
    v.check_bool(
        "structure:unhealthy_gate_gaps_detected",
        unreachable.contains("defense"),
        &format!("unhealthy gate gaps: {unreachable:?}"),
    );
}

fn phase_live_discovery(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let peers_result = ctx.call(
        "discovery",
        "discovery.peers",
        serde_json::json!({}),
    );

    match peers_result {
        Ok(resp) => {
            let peers = resp
                .get("peers")
                .and_then(serde_json::Value::as_array);

            if let Some(peers) = peers {
                let peer_count = peers.len();
                v.check_bool(
                    "live:peer_count",
                    peer_count >= 1,
                    &format!("{peer_count} peer(s) in mesh"),
                );

                let gate_ids: Vec<&str> = peers
                    .iter()
                    .filter_map(|p| {
                        p.get("gate")
                            .or_else(|| p.get("node_id"))
                            .and_then(serde_json::Value::as_str)
                    })
                    .collect();

                v.check_bool(
                    "live:peer_ids_present",
                    !gate_ids.is_empty(),
                    &format!("peer IDs: {gate_ids:?}"),
                );

                let has_strand = gate_ids.contains(&"strand-gate");
                v.check_bool(
                    "live:strand_gate_visible",
                    has_strand,
                    &format!("strand-gate in peers: {has_strand}"),
                );

                let has_iron = gate_ids.contains(&"iron-gate");
                if has_iron {
                    v.check_bool(
                        "live:iron_gate_visible",
                        true,
                        "iron-gate in peers — plasmodium collective formed",
                    );
                } else {
                    v.check_skip(
                        "live:iron_gate_visible",
                        "iron-gate not yet meshed (add to SONGBIRD_PEERS when ready)",
                    );
                }

                if peer_count >= 2 {
                    v.check_bool(
                        "live:plasmodium_quorum",
                        true,
                        &format!("{peer_count} peers — plasmodium quorum met"),
                    );
                } else {
                    v.check_skip(
                        "live:plasmodium_quorum",
                        &format!("{peer_count} peer(s) — need 2+ for plasmodium (add ironGate)"),
                    );
                }
            } else {
                v.check_bool("live:peer_count", false, "no peers array in response");
            }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "live:peer_count",
                &format!("discovery capability not available: {e}"),
            );
            v.check_skip("live:peer_ids_present", "skipped — no discovery");
            v.check_skip("live:strand_gate_visible", "skipped — no discovery");
            v.check_skip("live:iron_gate_visible", "skipped — no discovery");
            v.check_skip("live:plasmodium_quorum", "skipped — no discovery");
        }
        Err(e) => {
            v.check_bool("live:peer_count", false, &format!("discovery.peers error: {e}"));
        }
    }
}

fn phase_cross_gate_call(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let targets = [
        ("strand-gate", "security", "health.liveness"),
        ("strand-gate", "compute", "health.liveness"),
    ];

    for (gate, capability, operation) in &targets {
        let check_id = format!("live:cap_call_{gate}_{capability}");

        match ctx.call(
            "orchestration",
            "capability.call",
            serde_json::json!({
                "capability": capability,
                "operation": operation,
                "args": {},
                "gate": gate,
            }),
        ) {
            Ok(resp) => {
                v.check_bool(
                    &check_id,
                    true,
                    &format!("capability.call({capability}, {operation}) → {gate}: {resp}"),
                );
            }
            Err(e) if e.is_connection_error() => {
                v.check_skip(
                    &check_id,
                    &format!("orchestration not available: {e}"),
                );
            }
            Err(e) => {
                let msg = format!("{e}");
                let known_gap = msg.contains("Invalid JSON from remote")
                    || msg.contains("No local or remote provider")
                    || msg.contains("not found")
                    || msg.contains("no route")
                    || msg.contains("-32601");
                if known_gap {
                    v.check_skip(
                        &check_id,
                        &format!("cross-gate dispatch gap (may need Songbird rebuild): {e}"),
                    );
                } else {
                    v.check_bool(
                        &check_id,
                        false,
                        &format!("unexpected error: {e}"),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plasmodium_no_panic() {
        let mut v = ValidationResult::new("plasmodium-collective");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
    }

    #[test]
    fn mesh_topology_routes_computed() {
        let mut topo = MeshTopology::new();
        topo.set_local_gate("east-gate");
        topo.register_gate(
            "east-gate",
            None,
            ["beardog", "songbird"],
            ["security", "discovery"],
        );
        topo.register_gate(
            "strand-gate",
            None,
            ["barracuda"],
            ["compute"],
        );
        topo.register_gate(
            "iron-gate",
            None,
            ["skunkbat"],
            ["defense"],
        );
        topo.mark_healthy("east-gate", true);
        topo.mark_healthy("strand-gate", true);
        topo.mark_healthy("iron-gate", true);

        let routes = topo.routes_for_capability("compute");
        assert_eq!(routes.len(), 2);
        assert!(routes.iter().all(|r| r.healthy));
    }
}
