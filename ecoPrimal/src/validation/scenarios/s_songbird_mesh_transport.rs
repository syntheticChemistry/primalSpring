// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Songbird Mesh Transport — mesh.init, relay, and federation.
//!
//! Validates Songbird's cross-gate transport surface: mesh initialization,
//! relay forwarding, peer federation, and capability announcement.
//!
//! Phases:
//! 1. Capability coverage: mesh.*, songbird.*, network.*, stun.* breadth
//! 2. Mesh init contract: mesh.init registered, WG auto-init capability
//! 3. Relay surface: mesh.relay registered, federation peers available
//! 4. Cross-gate routing: Songbird addressable on all meshed gates
//! 5. Live: Songbird health + mesh.peers dispatch

use crate::composition::{CompositionContext, capability_to_primal};
use crate::evolution::{all_mesh_gates, mesh_address};
use crate::primal_names;
use crate::tolerances::ports;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Songbird mesh transport scenario metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "songbird-mesh-transport",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave126_mesh_transport",
        provenance_date: "2026-06-23",
        description: "Songbird mesh transport — init, relay, federation, cross-gate routing",
    },
    run,
};

const MESH_METHODS: &[(&str, &str)] = &[
    ("mesh.init", "WG auto-init (zero-config mesh join)"),
    ("mesh.relay", "relay frame forwarding"),
    ("mesh.publish", "sub-second impulse propagation"),
    ("mesh.peers", "peer roster query"),
    ("mesh.connect", "establish mesh connection"),
    ("mesh.discover_remotes", "remote gate discovery"),
    ("mesh.capabilities_announce", "announce local capabilities"),
    ("mesh.capabilities_query", "query remote capabilities"),
    ("mesh.find_path", "route path calculation"),
    ("mesh.announce", "presence announcement"),
];

const FEDERATION_METHODS: &[(&str, &str)] = &[
    ("songbird.federation.peers", "federation peer list"),
    ("songbird.federation.status", "federation health"),
    ("stun.discover", "NAT traversal discovery"),
    ("stun.detect_nat_type", "NAT type classification"),
];

/// Run all Songbird mesh transport validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Mesh method coverage");
    phase_mesh_coverage(v);

    v.section("Phase 2: Mesh init contract");
    phase_mesh_init(v);

    v.section("Phase 3: Relay + federation surface");
    phase_relay_federation(v);

    v.section("Phase 4: Cross-gate routing");
    phase_cross_gate_routing(v);

    v.section("Phase 5: Live Songbird");
    phase_live(v, ctx);
}

fn phase_mesh_coverage(v: &mut ValidationResult) {
    let mut registered = 0;
    for (method, desc) in MESH_METHODS {
        let present = REGISTRY_TOML.contains(method);
        if present {
            registered += 1;
        }
        v.check_bool(
            &format!("mesh:{}", method.replace('.', "_")),
            present,
            &format!("{method} — {desc}"),
        );
    }

    v.check_bool(
        "mesh:coverage_breadth",
        registered >= 8,
        &format!(
            "{registered}/{} mesh methods registered",
            MESH_METHODS.len()
        ),
    );
}

fn phase_mesh_init(v: &mut ValidationResult) {
    let has_init = REGISTRY_TOML.contains("mesh.init");
    v.check_bool(
        "init:mesh_init_registered",
        has_init,
        "mesh.init registered (WG auto-init capability)",
    );

    let has_auto_discover = REGISTRY_TOML.contains("mesh.auto_discover");
    v.check_bool(
        "init:auto_discover",
        has_auto_discover,
        "mesh.auto_discover for zero-config peer finding",
    );

    let has_mirror = REGISTRY_TOML.contains("mesh.mirror");
    v.check_bool(
        "init:mesh_mirror",
        has_mirror,
        "mesh.mirror for topology replication",
    );

    let mesh_domain = capability_to_primal("mesh");
    v.check_bool(
        "init:mesh_owns_songbird",
        mesh_domain == primal_names::SONGBIRD,
        &format!("mesh domain owned by \"{mesh_domain}\" (expected songbird)"),
    );
}

fn phase_relay_federation(v: &mut ValidationResult) {
    let mut fed_count = 0;
    for (method, desc) in FEDERATION_METHODS {
        let present = REGISTRY_TOML.contains(method);
        if present {
            fed_count += 1;
        }
        v.check_bool(
            &format!("fed:{}", method.replace('.', "_")),
            present,
            &format!("{method} — {desc}"),
        );
    }

    v.check_bool(
        "fed:federation_depth",
        fed_count >= 3,
        &format!(
            "{fed_count}/{} federation methods present",
            FEDERATION_METHODS.len()
        ),
    );

    let has_relay = REGISTRY_TOML.contains("mesh.relay");
    v.check_bool(
        "relay:mesh_relay_registered",
        has_relay,
        "mesh.relay for cross-gate frame forwarding",
    );

    let has_birdsong_beacon = REGISTRY_TOML.contains("network.birdsong.beacon");
    v.check_bool(
        "relay:birdsong_beacon",
        has_birdsong_beacon,
        "network.birdsong.beacon for Dark Forest discovery",
    );
}

fn phase_cross_gate_routing(v: &mut ValidationResult) {
    let gates = all_mesh_gates();
    let meshed_gates: Vec<&str> = gates
        .iter()
        .filter(|g| !g.address.is_empty())
        .map(|g| g.name.as_str())
        .collect();

    v.check_bool(
        "route:meshed_gates",
        meshed_gates.len() >= 4,
        &format!("{} gates with WG addresses", meshed_gates.len()),
    );

    let songbird_port = ports::default_port_for(primal_names::SONGBIRD);
    v.check_bool(
        "route:songbird_port",
        songbird_port > 0,
        &format!("songbird port = {songbird_port}"),
    );

    for gate in &["eastGate", "sporeGate", "flockGate", "ironGate"] {
        let addr = mesh_address(gate);
        v.check_bool(
            &format!("route:{gate}_addressable"),
            addr.is_some(),
            &format!("{gate} WG address: {addr:?}"),
        );
    }

    let golgi_addr = mesh_address("golgi");
    v.check_bool(
        "route:golgi_hub",
        golgi_addr.is_some(),
        &format!("golgi (relay hub) has mesh address: {golgi_addr:?}"),
    );
}

fn phase_live(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let Some(client) = ctx.client_for("discovery") else {
        v.check_skip("live:songbird_health", "no discovery client available");
        v.check_skip("live:mesh_peers_responds", "no discovery client");
        return;
    };

    let resp = client.call("health.liveness", serde_json::json!({}));
    match resp {
        Ok(r) => {
            v.check_bool(
                "live:songbird_health",
                r.is_success(),
                "Songbird responding to health.liveness",
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
                "live:mesh_peers_responds",
                r.is_success(),
                "mesh.peers returns peer roster",
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("live:mesh_peers_responds", &format!("{e}"));
        }
        Err(e) => {
            v.check_bool("live:mesh_peers_responds", false, &format!("{e}"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn songbird_mesh_transport_runs() {
        let mut v = ValidationResult::new("songbird-mesh-transport");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        let total = v.passed + v.failed + v.skipped;
        assert!(total >= 25, "expected ≥25 checks, got {total}");
    }

    #[test]
    fn mesh_init_registered() {
        assert!(REGISTRY_TOML.contains("mesh.init"));
    }

    #[test]
    fn mesh_relay_registered() {
        assert!(REGISTRY_TOML.contains("mesh.relay"));
    }

    #[test]
    fn songbird_owns_mesh() {
        assert_eq!(capability_to_primal("mesh"), primal_names::SONGBIRD);
    }

    #[test]
    fn songbird_owns_stun() {
        assert_eq!(capability_to_primal("stun"), primal_names::SONGBIRD);
    }

    #[test]
    fn federation_methods_present() {
        assert!(REGISTRY_TOML.contains("songbird.federation.peers"));
        assert!(REGISTRY_TOML.contains("songbird.federation.status"));
    }

    #[test]
    fn mesh_methods_breadth() {
        let count = MESH_METHODS
            .iter()
            .filter(|(m, _)| REGISTRY_TOML.contains(m))
            .count();
        assert!(count >= 8, "expected ≥8 mesh methods, got {count}");
    }

    #[test]
    fn all_gates_addressable() {
        for gate in &["eastGate", "sporeGate", "flockGate", "ironGate", "golgi"] {
            assert!(
                mesh_address(gate).is_some(),
                "{gate} should have WG address"
            );
        }
    }

    #[test]
    fn golgi_is_relay_hub() {
        assert!(
            mesh_address("golgi").is_some(),
            "golgi should have a mesh address in mesh_topology.toml"
        );
    }
}
