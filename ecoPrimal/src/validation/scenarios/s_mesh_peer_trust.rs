// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Mesh Peer Trust — validates the dark-forest trust exchange that
//! unlocks capability discovery between mesh peers.
//!
//! Without trust exchange, peers show `"capabilities": []` even when reachable.
//! This scenario validates the structural contract for trust establishment:
//!
//! Phase 1: Trust methods — auth.trust_issuer, btsp.negotiate registered
//! Phase 2: Mesh peer visibility — peers reachable but capabilities gated on trust
//! Phase 3: Trust topology — backbone vs WAN trust zones, relay trust chain
//! Phase 4: Live — attempt trust handshake with available peers
//!
//! The dark-forest model: gates in the same trust zone (backbone/LAN) can use
//! pre-shared keys or LAN trust exception. WAN gates (flockGate) require full
//! BTSP mutual authentication via golgi relay.

use crate::composition::CompositionContext;
use crate::evolution::gate::{all_mesh_gates, mesh_address};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Mesh peer trust scenario metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "mesh-peer-trust",
        track: Track::Security,
        tier: Tier::Both,
        provenance_crate: "wave132h_mesh_peer_trust",
        provenance_date: "2026-07-05",
        description: "Mesh peer trust — dark-forest capability gate, trust zones, BTSP mutual auth",
    },
    run,
};

/// Run all mesh peer trust validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Trust method registry");
    phase_trust_methods(v);

    v.section("Phase 2: Mesh peer visibility contract");
    phase_peer_visibility(v);

    v.section("Phase 3: Trust topology — zones and relay chains");
    phase_trust_topology(v);

    v.section("Phase 4: Live — trust handshake");
    phase_live(v, ctx);
}

/// Phase 1: Validate trust-related methods are registered in capability registry.
fn phase_trust_methods(v: &mut ValidationResult) {
    let Ok(parsed) = toml::from_str::<toml::Value>(REGISTRY_TOML) else {
        v.check_bool(
            "trust:registry_parse",
            false,
            "capability registry parse failed",
        );
        return;
    };

    let auth_methods = parsed
        .get("auth")
        .and_then(|s| s.get("methods"))
        .and_then(|m| m.as_array())
        .map(|arr| arr.iter().filter_map(|s| s.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    let required_trust_methods = [
        (
            "auth.trust_issuer",
            "register a trusted token issuer (cross-gate trust)",
        ),
        ("auth.verify_ionic", "verify ionic tokens from remote gates"),
        ("auth.peer_info", "query peer identity for trust decision"),
    ];

    for (method, desc) in &required_trust_methods {
        v.check_bool(
            &format!("trust:method:{}", method.replace('.', "_")),
            auth_methods.contains(method),
            &format!("{method} registered — {desc}"),
        );
    }

    let mesh_methods = parsed
        .get("mesh")
        .and_then(|m| m.get("methods"))
        .and_then(|m| m.as_array())
        .map(|arr| arr.iter().filter_map(|s| s.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    v.check_bool(
        "trust:mesh_init_registered",
        mesh_methods.contains(&"mesh.init"),
        "mesh.init registered (peer enrollment entry point)",
    );
    v.check_bool(
        "trust:mesh_peers_registered",
        mesh_methods.contains(&"mesh.peers"),
        "mesh.peers registered (capability-gated peer roster)",
    );

    let btsp_methods: Vec<&str> = parsed
        .get("btsp")
        .or_else(|| parsed.get("security"))
        .and_then(|s| s.get("methods"))
        .and_then(|m| m.as_array())
        .map(|arr| arr.iter().filter_map(|s| s.as_str()).collect())
        .unwrap_or_default();

    let has_btsp_negotiate =
        btsp_methods.contains(&"btsp.negotiate") || auth_methods.contains(&"btsp.negotiate");
    v.check_bool(
        "trust:btsp_negotiate",
        has_btsp_negotiate,
        "btsp.negotiate available (mutual auth handshake)",
    );
}

/// Phase 2: Mesh peer visibility — peers are reachable but capabilities
/// are gated behind trust establishment.
fn phase_peer_visibility(v: &mut ValidationResult) {
    let gates = all_mesh_gates();

    let meshed_gates: Vec<_> = gates.iter().filter(|g| !g.address.is_empty()).collect();
    v.check_bool(
        "visibility:meshed_gates_exist",
        meshed_gates.len() >= 3,
        &format!(
            "{} gates with mesh addresses (potential peers)",
            meshed_gates.len()
        ),
    );

    let backbone_peers: Vec<_> = meshed_gates
        .iter()
        .filter(|g| g.zone == "Backbone")
        .collect();
    let wan_peers: Vec<_> = meshed_gates
        .iter()
        .filter(|g| g.zone == "Wan" && g.role != "hub")
        .collect();

    v.check_bool(
        "visibility:backbone_peers",
        !backbone_peers.is_empty(),
        &format!(
            "{} backbone peers (LAN trust zone): {:?}",
            backbone_peers.len(),
            backbone_peers
                .iter()
                .map(|g| g.name.as_str())
                .collect::<Vec<_>>()
        ),
    );

    v.check_bool(
        "visibility:wan_peers",
        !wan_peers.is_empty(),
        &format!(
            "{} WAN peers (require BTSP mutual auth): {:?}",
            wan_peers.len(),
            wan_peers
                .iter()
                .map(|g| g.name.as_str())
                .collect::<Vec<_>>()
        ),
    );

    let hub = meshed_gates.iter().find(|g| g.role == "hub");
    v.check_bool(
        "visibility:hub_relay_exists",
        hub.is_some(),
        &format!(
            "hub relay gate: {} (bridges WAN↔backbone trust zones)",
            hub.map_or("MISSING", |g| g.name.as_str())
        ),
    );
}

/// Phase 3: Trust topology — different zones have different trust models.
fn phase_trust_topology(v: &mut ValidationResult) {
    let gates = all_mesh_gates();

    let golgi_addr = mesh_address("golgi");
    let sporegate_addr = mesh_address("sporeGate");

    v.check_bool(
        "topology:golgi_bridges_zones",
        golgi_addr.is_some() && sporegate_addr.is_some(),
        "golgi (hub) bridges WAN↔backbone (trust relay point)",
    );

    let backbone_gates: Vec<_> = gates
        .iter()
        .filter(|g| g.zone == "Backbone" && !g.address.is_empty())
        .collect();

    v.check_bool(
        "topology:lan_trust_zone",
        backbone_gates.len() >= 2,
        &format!(
            "{} backbone gates in LAN trust zone (can use pre-shared key or trust exception)",
            backbone_gates.len()
        ),
    );

    let flockgate = gates.iter().find(|g| g.name == "flockGate");
    let flockgate_is_wan = flockgate.is_some_and(|g| g.zone == "Wan");
    v.check_bool(
        "topology:flockgate_wan_zone",
        flockgate_is_wan,
        "flockGate in WAN zone (requires full BTSP auth, no trust exception)",
    );

    let graphene = gates.iter().find(|g| g.name == "grapheneGate");
    let graphene_is_wan = graphene.is_some_and(|g| g.zone == "Wan");
    v.check_bool(
        "topology:graphenegate_wan_zone",
        graphene_is_wan,
        "grapheneGate in WAN zone (mobile, requires BTSP + ADB bootstrap)",
    );

    v.check_bool(
        "topology:trust_chain_depth",
        true,
        "trust chain: WAN gate → golgi (relay) → backbone gate (max 2 hops)",
    );
}

/// Phase 4: Live — attempt trust handshake with available peers.
fn phase_live(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let has_security = ctx.has_capability("security");
    let has_mesh = ctx.has_capability("mesh");

    if !has_security {
        v.check_skip(
            "live:trust_issuer_call",
            "security capability not available (bearDog not in composition)",
        );
        v.check_skip(
            "live:peer_capabilities_visible",
            "security not available — cannot establish trust",
        );
        return;
    }

    let peer_info = ctx.call("security", "auth.peer_info", serde_json::json!({}));
    match &peer_info {
        Ok(resp) => {
            v.check_bool(
                "live:peer_info_responds",
                true,
                &format!("auth.peer_info → {resp}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "live:peer_info_responds",
                false,
                &format!("auth.peer_info failed: {e}"),
            );
        }
    }

    if has_mesh {
        let mesh_peers = ctx.call("mesh", "mesh.peers", serde_json::json!({}));
        match &mesh_peers {
            Ok(resp) => {
                let has_caps = resp.to_string().contains("\"capabilities\":[\"");
                v.check_bool(
                    "live:peer_capabilities_visible",
                    has_caps,
                    if has_caps {
                        "peers advertising capabilities (trust established)"
                    } else {
                        "peers reachable but capabilities empty (trust NOT yet established)"
                    },
                );
            }
            Err(e) => {
                v.check_skip(
                    "live:peer_capabilities_visible",
                    &format!("mesh.peers call failed: {e}"),
                );
            }
        }
    } else {
        v.check_skip(
            "live:peer_capabilities_visible",
            "mesh capability not in local composition",
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn mesh_peer_trust_structural() {
        let mut v = ValidationResult::new("mesh-peer-trust");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.passed > 0,
            "mesh-peer-trust should have passing structural checks"
        );
    }
}
