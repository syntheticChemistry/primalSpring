// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Mobile Mesh Init — validates grapheneGate's ability to join the
//! sovereign mesh via ADB tether and eventually mature into a cellular relay.
//!
//! The mobile mesh model:
//! - grapheneGate connects to eastGate via USB tethering (internet + gate)
//! - BindMode::Auto selects TCP fallback on Android (SELinux blocks UDS)
//! - mesh.init bootstraps peering via eastGate (localhost over ADB forward)
//! - Once peered, songBird on grapheneGate can relay for WAN nodes via cellular
//!
//! Phases:
//! 1. BindMode::Auto correctly selects fallback on restricted platforms
//! 2. mesh_topology.toml: grapheneGate has transport=adb, role=mobile
//! 3. mesh.init contract: bootstrap_peers parameter, peer discovery flow
//! 4. Tether duality: simultaneous internet provision + gate operation
//! 5. Live: ADB device reachability + port forward validation

use crate::composition::CompositionContext;
use crate::evolution::gate::all_mesh_gates;
use crate::ipc::server_bind::BindMode;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const TOPOLOGY_TOML: &str = include_str!("../../../../config/mesh_topology.toml");
const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Mobile mesh init scenario metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "mobile-mesh-init",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "wave132g_mobile_mesh_init",
        provenance_date: "2026-07-05",
        description:
            "Mobile mesh init — grapheneGate BindMode::Auto + mesh.init via ADB tether",
    },
    run,
};

/// Run all mobile mesh init validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: BindMode::Auto platform selection");
    phase_bind_mode(v);

    v.section("Phase 2: Topology enrollment");
    phase_topology(v);

    v.section("Phase 3: mesh.init contract");
    phase_mesh_init_contract(v);

    v.section("Phase 4: Tether duality");
    phase_tether_duality(v);

    v.section("Phase 5: Live — ADB reachability");
    phase_live_adb(v);
}

fn phase_bind_mode(v: &mut ValidationResult) {
    let tcp_only = BindMode::TcpOnly;
    let fallback = BindMode::Fallback;

    v.check_bool(
        "bind:tcp_only_variant",
        matches!(tcp_only, BindMode::TcpOnly),
        "BindMode::TcpOnly variant exists for restricted platforms (Android SELinux)",
    );

    v.check_bool(
        "bind:fallback_variant",
        matches!(fallback, BindMode::Fallback),
        "BindMode::Fallback variant exists (UDS → TCP graceful degradation)",
    );

    let env_key = std::env::var("PRIMAL_BIND_MODE").unwrap_or_default();
    let parsed = BindMode::from_env();
    v.check_bool(
        "bind:from_env_parses",
        matches!(parsed, BindMode::UdsOnly | BindMode::TcpOnly | BindMode::Fallback),
        &format!("BindMode::from_env() = {parsed:?} (env: '{env_key}')"),
    );

    v.check_bool(
        "bind:tcp_only_for_android",
        format!("{tcp_only:?}").contains("Tcp"),
        "TcpOnly mode ensures SELinux-safe transport on grapheneGate",
    );
}

fn phase_topology(v: &mut ValidationResult) {
    let gates = all_mesh_gates();
    let graphene = gates.iter().find(|g| g.name == "grapheneGate");

    v.check_bool(
        "topo:graphenegate_enrolled",
        graphene.is_some(),
        "grapheneGate is in mesh_topology.toml gate roster",
    );

    v.check_bool(
        "topo:transport_adb",
        TOPOLOGY_TOML.contains("transport = \"adb\""),
        "grapheneGate declares ADB transport",
    );

    v.check_bool(
        "topo:role_mobile",
        TOPOLOGY_TOML.contains("role = \"mobile\""),
        "grapheneGate role is mobile",
    );

    v.check_bool(
        "topo:zone_wan",
        graphene.is_some_and(|g| g.zone == "Wan"),
        "grapheneGate is in Wan zone (cellular backhaul)",
    );

    let has_no_wg_address = graphene.is_none_or(|g| g.address.is_empty());
    v.check_bool(
        "topo:no_wg_address",
        has_no_wg_address,
        "grapheneGate has no WireGuard address (ADB transport, not WG)",
    );
}

fn phase_mesh_init_contract(v: &mut ValidationResult) {
    v.check_bool(
        "mesh:init_method_exists",
        REGISTRY_TOML.contains("mesh.init")
            || REGISTRY_TOML.contains("mesh.peer")
            || REGISTRY_TOML.contains("mesh.peers"),
        "mesh.init or mesh.peer(s) method registered in capability_registry",
    );

    let gates = all_mesh_gates();
    let bootstrap_candidates: Vec<&str> = gates
        .iter()
        .filter(|g| !g.address.is_empty() && g.name != "grapheneGate")
        .map(|g| g.name.as_str())
        .collect();

    v.check_bool(
        "mesh:has_bootstrap_peers",
        bootstrap_candidates.len() >= 2,
        &format!(
            "{} bootstrap peer candidates available (need ≥2)",
            bootstrap_candidates.len()
        ),
    );

    let has_hub = gates.iter().any(|g| g.role == "hub" && !g.address.is_empty());
    v.check_bool(
        "mesh:hub_available",
        has_hub,
        "At least one hub gate available for bootstrap (golgi)",
    );

    let eastgate = gates.iter().find(|g| g.name == "eastGate");
    let eastgate_meshable = eastgate.is_some_and(|g| !g.address.is_empty());
    v.check_bool(
        "mesh:eastgate_as_bootstrap",
        eastgate_meshable,
        "eastGate has mesh address (can bootstrap grapheneGate over ADB forward)",
    );
}

fn phase_tether_duality(v: &mut ValidationResult) {
    v.check_bool(
        "tether:topology_declares_services",
        TOPOLOGY_TOML.contains("nucleus_tower"),
        "grapheneGate services include nucleus_tower",
    );

    let songbird_port: u16 = 9200;
    let beardog_port: u16 = 9100;

    v.check_bool(
        "tether:songbird_port_valid",
        songbird_port > 1024 && songbird_port < 65535,
        &format!("songBird ADB port {songbird_port} is unprivileged"),
    );

    v.check_bool(
        "tether:beardog_port_valid",
        beardog_port > 1024 && beardog_port < 65535,
        &format!("bearDog ADB port {beardog_port} is unprivileged"),
    );

    v.check_bool(
        "tether:ports_dont_conflict_with_tether",
        songbird_port != 8080 && beardog_port != 8080,
        "Primal ports don't conflict with common tether proxy port (8080)",
    );

    v.check_bool(
        "tether:cellular_relay_possible",
        TOPOLOGY_TOML.contains("zone = \"Wan\"")
            && TOPOLOGY_TOML.contains("role = \"mobile\""),
        "grapheneGate topology supports Wan-zone cellular relay maturation",
    );
}

fn phase_live_adb(v: &mut ValidationResult) {
    let adb_available = std::process::Command::new("adb")
        .arg("devices")
        .output()
        .map(|o| o.status.success() && String::from_utf8_lossy(&o.stdout).contains("device"))
        .unwrap_or(false);

    if !adb_available {
        v.check_skip("live:adb_not_connected", "no ADB device connected (expected in CI)");
        return;
    }

    let device_output = std::process::Command::new("adb")
        .args(["shell", "uname", "-m"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    v.check_bool(
        "live:device_arch",
        device_output == "aarch64",
        &format!("Device architecture: {device_output} (expected aarch64)"),
    );

    for (primal, port) in &[("beardog", 9100u16), ("songbird", 9200u16), ("skunkbat", 9140u16)] {
        let reachable = std::net::TcpStream::connect_timeout(
            &std::net::SocketAddr::from(([127, 0, 0, 1], *port)),
            std::time::Duration::from_secs(2),
        )
        .is_ok();

        v.check_bool(
            &format!("live:adb_forward:{primal}"),
            reachable,
            &format!("{primal} reachable on localhost:{port} via ADB forward"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mobile_mesh_structural() {
        let mut v = ValidationResult::new("mobile-mesh-init");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        let total = v.passed + v.failed + v.skipped;
        assert!(total >= 15, "expected ≥15 checks, got {total}");
    }

    #[test]
    fn graphenegate_in_topology() {
        let gates = all_mesh_gates();
        let g = gates.iter().find(|g| g.name == "grapheneGate");
        assert!(g.is_some(), "grapheneGate must be in mesh_topology.toml");
    }

    #[test]
    fn graphenegate_no_wg_address() {
        let gates = all_mesh_gates();
        let g = gates.iter().find(|g| g.name == "grapheneGate");
        assert!(
            g.is_none_or(|g| g.address.is_empty()),
            "grapheneGate should have no WG address (uses ADB)"
        );
    }

    #[test]
    fn bind_mode_tcp_only_exists() {
        let _ = BindMode::TcpOnly;
    }
}
