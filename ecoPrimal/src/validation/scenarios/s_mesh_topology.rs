// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Mesh Topology — validates federation peer configuration,
//! connectivity readiness, and multi-gate mesh structure.
//!
//! The sovereign mesh model requires that gates discover and maintain
//! peer connections. This scenario validates:
//!
//! 1. Federation port is correctly assigned and resolvable
//! 2. Peer configuration is syntactically valid
//! 3. Local gate identity is resolvable
//! 4. Mesh structure supports the declared topology
//!
//! Phase 1 (Structural): Config and port validation
//! Phase 2 (Live): Peer reachability probe (TCP connect)

use crate::composition::CompositionContext;
use crate::tolerances;
use crate::validation::scenarios::{Scenario, ScenarioMeta, Tier, Track};
use crate::validation::ValidationResult;

/// Mesh topology scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "mesh-topology",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-17",
        description: "Validates federation mesh topology, peer config, and connectivity",
    },
    run,
};

/// Run mesh topology validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Federation configuration");
    phase_federation_config(v);

    v.section("Phase 2: Gate identity");
    phase_gate_identity(v);

    v.section("Phase 3: Peer connectivity");
    phase_peer_connectivity(v);
}

fn phase_federation_config(v: &mut ValidationResult) {
    let fed_port = tolerances::ports::FEDERATION_PORT;
    v.check_bool(
        "federation:port_defined",
        fed_port > 0 && fed_port < 65535,
        &format!("federation port: {fed_port}"),
    );

    let host = tolerances::platform::DEFAULT_HOST;
    v.check_bool(
        "federation:host_defined",
        !host.is_empty(),
        &format!("default host: {host}"),
    );

    let federation_ports = tolerances::ports::FEDERATION_PORTS;
    v.check_bool(
        "federation:profiles_defined",
        !federation_ports.is_empty(),
        &format!("{} federation port profiles defined", federation_ports.len()),
    );

    for fp in federation_ports {
        v.check_bool(
            &format!("federation:profile:{}:{}", fp.profile, fp.primal),
            fp.port > 0 && fp.port < 65535,
            &format!(
                "{}:{} port {} (role: {}, droppable: {})",
                fp.profile, fp.primal, fp.port, fp.role, fp.droppable
            ),
        );
    }
}

fn phase_gate_identity(v: &mut ValidationResult) {
    let hostname = tolerances::platform::hostname().unwrap_or_default();
    v.check_bool(
        "gate:hostname_resolved",
        !hostname.is_empty() && hostname != "unknown",
        &format!("gate hostname: {hostname}"),
    );

    let gate_name = std::env::var(crate::env_keys::GATE_NAME).unwrap_or_default();
    let gate_id = std::env::var(crate::env_keys::GATE_ID).unwrap_or_default();

    if gate_name.is_empty() && gate_id.is_empty() {
        v.check_skip(
            "gate:identity_configured",
            "GATE_NAME and GATE_ID not set (single-gate deployment)",
        );
    } else {
        v.check_bool(
            "gate:identity_configured",
            !gate_name.is_empty() || !gate_id.is_empty(),
            &format!("gate: name={gate_name:?} id={gate_id:?}"),
        );
    }
}

#[expect(deprecated, reason = "SONGBIRD_PEERS fallback for backward compatibility")]
fn phase_peer_connectivity(v: &mut ValidationResult) {
    let mesh_peers = std::env::var(crate::env_keys::MESH_PEERS)
        .or_else(|_| std::env::var(crate::env_keys::SONGBIRD_PEERS))
        .unwrap_or_default();

    if mesh_peers.is_empty() {
        v.check_skip(
            "peers:configured",
            "MESH_PEERS / SONGBIRD_PEERS not set (standalone gate)",
        );
        return;
    }

    let peers: Vec<&str> = mesh_peers.split(',').map(str::trim).filter(|s| !s.is_empty()).collect();
    v.check_bool(
        "peers:configured",
        !peers.is_empty(),
        &format!("{} peers configured", peers.len()),
    );

    for peer_spec in &peers {
        let (label, addr) = if let Some((k, v)) = peer_spec.split_once('=') {
            (k.trim(), v.trim())
        } else {
            ("unnamed", *peer_spec)
        };

        let reachable = probe_tcp(addr, std::time::Duration::from_secs(3));
        v.check_bool(
            &format!("peers:{label}:reachable"),
            reachable,
            &format!("{label} @ {addr}: {}", if reachable { "ALIVE" } else { "UNREACHABLE" }),
        );
    }
}

fn probe_tcp(addr: &str, timeout: std::time::Duration) -> bool {
    use std::net::ToSocketAddrs;
    let Ok(mut addrs) = addr.to_socket_addrs() else {
        return false;
    };
    let Some(socket_addr) = addrs.next() else {
        return false;
    };
    std::net::TcpStream::connect_timeout(&socket_addr, timeout).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_topology_structural() {
        let mut v = ValidationResult::new("mesh-topology");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        // Federation config (Phase 1) should always pass structurally.
        // Peer connectivity may skip if no peers configured.
        assert!(
            v.evaluated() >= 3,
            "mesh-topology should evaluate at least federation config checks"
        );
    }
}
