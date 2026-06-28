// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Relay Forward — E2E cross-gate method relay via Songbird mesh.
//!
//! Validates that a `capability.call` originating on one gate can be relayed
//! to a remote primal on another gate through Songbird's `mesh.relay` transport
//! and cellMembrane's forwarding logic.
//!
//! Phases:
//! 1. Structural: relay methods registered, mesh topology populated
//! 2. Wire contract: capability.call + mesh.relay schemas, relay-eligible domains
//! 3. Routing: domain → primal → gate address chain resolves end-to-end
//! 4. Live: Songbird health + capability.call dispatch to a remote domain

use crate::composition::{CompositionContext, capability_to_primal, method_to_capability_domain};
use crate::evolution::{all_mesh_gates, mesh_address};
use crate::primal_names;
use crate::tolerances::ports;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Relay forward scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "relay-forward",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave128_relay_forward",
        provenance_date: "2026-06-28",
        description: "Relay forward — E2E cross-gate method relay via Songbird mesh + cellMembrane",
    },
    run,
};

const RELAY_METHODS: &[(&str, &str)] = &[
    ("capability.call", "cross-primal capability dispatch"),
    ("mesh.relay", "mesh frame forwarding"),
    ("mesh.init", "mesh peer initialization"),
    ("mesh.peers", "peer roster query"),
];

const RELAY_DOMAINS: &[&str] = &["compute", "defense", "discovery", "mesh", "attestation"];

/// Run all relay forward validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — relay infrastructure topology");
    phase_structural(v);

    v.section("Phase 2: Wire contract — relay method surface");
    phase_wire_contract(v);

    v.section("Phase 3: Routing — E2E domain → gate address chain");
    phase_routing(v);

    v.section("Phase 4: Live — Songbird relay dispatch");
    phase_live(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    let gates = all_mesh_gates();

    let meshed_count = gates.len();
    v.check_bool(
        "struct:mesh_has_gates",
        meshed_count >= 3,
        &format!("{meshed_count} gates in mesh_topology.toml (need ≥3 for relay)"),
    );

    let has_hub = gates.iter().any(|g| g.role == "hub" || g.role == "relay");
    v.check_bool(
        "struct:hub_exists",
        has_hub,
        "mesh has a hub/relay node (golgi) for frame forwarding",
    );

    let hub_addr = mesh_address("golgi");
    v.check_bool(
        "struct:golgi_addressable",
        hub_addr.is_some(),
        &format!("golgi mesh address: {hub_addr:?}"),
    );

    let songbird_port = ports::default_port_for(primal_names::SONGBIRD);
    v.check_bool(
        "struct:songbird_port",
        songbird_port > 0,
        &format!("Songbird port: {songbird_port}"),
    );

    let membrane_port = ports::default_port_for("cellmembrane");
    v.check_bool(
        "struct:membrane_port",
        membrane_port > 0,
        &format!("cellMembrane port: {membrane_port}"),
    );

    for gate_name in ["eastGate", "sporeGate", "ironGate"] {
        let addr = mesh_address(gate_name);
        v.check_bool(
            &format!("struct:{}_addressable", gate_name.to_lowercase()),
            addr.is_some(),
            &format!("{gate_name} mesh address: {addr:?}"),
        );
    }
}

fn phase_wire_contract(v: &mut ValidationResult) {
    let mut registered = 0;
    for (method, desc) in RELAY_METHODS {
        let present = REGISTRY_TOML.contains(method);
        if present {
            registered += 1;
        }
        v.check_bool(
            &format!("wire:{}", method.replace('.', "_")),
            present,
            &format!("{method} ({desc}) in capability_registry.toml"),
        );
    }

    v.check_bool(
        "wire:relay_coverage",
        registered >= 3,
        &format!(
            "{registered}/{} relay methods registered",
            RELAY_METHODS.len()
        ),
    );

    let has_capability_call = REGISTRY_TOML.contains("capability.call");
    let has_mesh_relay = REGISTRY_TOML.contains("mesh.relay");
    v.check_bool(
        "wire:relay_chain_complete",
        has_capability_call && has_mesh_relay,
        "capability.call + mesh.relay both present (relay chain)",
    );
}

fn phase_routing(v: &mut ValidationResult) {
    for domain in RELAY_DOMAINS {
        let primal = capability_to_primal(domain);
        let resolves = !primal.is_empty();
        v.check_bool(
            &format!("route:{domain}_resolves"),
            resolves,
            &format!("capability_to_primal(\"{domain}\") → \"{primal}\""),
        );
    }

    let songbird = capability_to_primal("discovery");
    v.check_bool(
        "route:discovery_is_songbird",
        songbird == "songbird",
        &format!("discovery domain → '{songbird}' (expected 'songbird')"),
    );

    let membrane = capability_to_primal("capability");
    let membrane_resolves =
        membrane == "cellmembrane" || membrane == "biomeOS" || !membrane.is_empty();
    v.check_bool(
        "route:capability_resolves",
        membrane_resolves,
        &format!("capability domain → '{membrane}'"),
    );

    let mesh_domain = method_to_capability_domain("mesh.relay");
    let relay_primal = capability_to_primal(mesh_domain);
    v.check_bool(
        "route:mesh_relay_chain",
        !relay_primal.is_empty(),
        &format!("mesh.relay → domain '{mesh_domain}' → primal '{relay_primal}'"),
    );

    let cross_gate_pairs = [
        ("eastGate", "ironGate"),
        ("eastGate", "sporeGate"),
        ("sporeGate", "ironGate"),
    ];
    for (src, dst) in cross_gate_pairs {
        let src_addr = mesh_address(src);
        let dst_addr = mesh_address(dst);
        let both = src_addr.is_some() && dst_addr.is_some();
        v.check_bool(
            &format!(
                "route:pair_{}_{}_addressable",
                src.to_lowercase(),
                dst.to_lowercase()
            ),
            both,
            &format!("{src} ({src_addr:?}) ↔ {dst} ({dst_addr:?})"),
        );
    }
}

fn phase_live(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let Some(client) = ctx.client_for("discovery") else {
        v.check_skip("live:songbird_health", "no discovery client available");
        v.check_skip("live:mesh_relay_dispatch", "no discovery client");
        v.check_skip("live:capability_call_dispatch", "no discovery client");
        return;
    };

    let resp = client.call("health.liveness", serde_json::json!({}));
    match resp {
        Ok(r) => {
            v.check_bool(
                "live:songbird_health",
                r.is_success(),
                "Songbird responding to health.liveness (relay transport ready)",
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("live:songbird_health", &format!("{e}"));
        }
        Err(e) => {
            v.check_bool("live:songbird_health", false, &format!("{e}"));
        }
    }

    let resp = client.call("mesh.peers", serde_json::json!({}));
    match resp {
        Ok(r) => {
            v.check_bool(
                "live:mesh_peers_available",
                r.is_success(),
                "mesh.peers returns roster (relay targets exist)",
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("live:mesh_peers_available", &format!("{e}"));
        }
        Err(e) => {
            v.check_bool("live:mesh_peers_available", false, &format!("{e}"));
        }
    }

    let relay_payload = serde_json::json!({
        "target_gate": "ironGate",
        "method": "health.liveness",
        "params": {}
    });
    let resp = client.call("mesh.relay", relay_payload);
    match resp {
        Ok(r) => {
            v.check_bool(
                "live:mesh_relay_dispatch",
                r.is_success(),
                "mesh.relay to ironGate health.liveness succeeded",
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("live:mesh_relay_dispatch", &format!("mesh.relay: {e}"));
        }
        Err(e) => {
            v.check_bool("live:mesh_relay_dispatch", false, &format!("{e}"));
        }
    }

    match ctx.client_for("capability") {
        Some(cap_client) => {
            let call_payload = serde_json::json!({
                "capability": "compute",
                "method": "health.liveness",
                "params": {},
                "routing": "mesh"
            });
            let resp = cap_client.call("capability.call", call_payload);
            match resp {
                Ok(r) => {
                    v.check_bool(
                        "live:capability_call_dispatch",
                        r.is_success(),
                        "capability.call(compute, health.liveness, routing=mesh) succeeded",
                    );
                }
                Err(e) if e.is_skippable() => {
                    v.check_skip(
                        "live:capability_call_dispatch",
                        &format!("capability.call: {e}"),
                    );
                }
                Err(e) => {
                    v.check_bool("live:capability_call_dispatch", false, &format!("{e}"));
                }
            }
        }
        None => {
            v.check_skip(
                "live:capability_call_dispatch",
                "no capability client available",
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn relay_forward_no_panic() {
        let mut v = ValidationResult::new("relay-forward");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        let total = v.passed + v.failed + v.skipped;
        assert!(total >= 20, "expected ≥20 checks, got {total}");
    }

    #[test]
    fn relay_methods_in_registry() {
        assert!(REGISTRY_TOML.contains("capability.call"));
        assert!(REGISTRY_TOML.contains("mesh.relay"));
    }

    #[test]
    fn hub_is_addressable() {
        assert!(
            mesh_address("golgi").is_some(),
            "golgi (relay hub) should have a mesh address"
        );
    }

    #[test]
    fn discovery_is_songbird() {
        assert_eq!(
            capability_to_primal("discovery"),
            "songbird",
            "discovery domain should route to songbird"
        );
    }

    #[test]
    fn cross_gate_pairs_addressable() {
        for gate in ["eastGate", "sporeGate", "ironGate"] {
            assert!(
                mesh_address(gate).is_some(),
                "{gate} should have a mesh address"
            );
        }
    }

    #[test]
    fn mesh_relay_domain_resolves() {
        let domain = method_to_capability_domain("mesh.relay");
        let primal = capability_to_primal(domain);
        assert!(
            !primal.is_empty(),
            "mesh.relay → {domain} → primal should resolve"
        );
    }
}
