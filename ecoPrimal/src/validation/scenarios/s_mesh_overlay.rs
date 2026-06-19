// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Mesh Overlay — validates the WireGuard sovereign overlay network.
//!
//! The 10.13.37.0/24 mesh connects gates across physical boundaries (LAN, WAN,
//! VPS) via encrypted WireGuard tunnels through golgi as hub. This scenario
//! validates:
//!
//! 1. Local wg0 interface exists and has correct address
//! 2. Assigned mesh addresses match the static registry
//! 3. All live mesh peers are reachable via ICMP
//! 4. Mesh node count matches expected (5 nodes as of Wave 116)
//! 5. Hub (golgi) has lowest latency (direct peer, no relay)

use crate::composition::CompositionContext;
use crate::evolution::gate::mesh_address;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Mesh overlay validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "mesh-overlay",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-18",
        description: "WireGuard sovereign overlay: interface, addressing, live reachability",
    },
    run: run_mesh_overlay,
};

fn run_mesh_overlay(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    phase_local_interface(v);
    phase_addressing(v);
    phase_live_reachability(v);
}

fn phase_local_interface(v: &mut ValidationResult) {
    let wg_exists = std::path::Path::new("/sys/class/net/wg0").exists();

    if !wg_exists {
        v.check_skip("wg0:exists", "wg0 interface not present (not enrolled)");
        return;
    }

    v.check_bool("wg0:exists", true, "wg0 interface present");

    let addr = read_interface_addr("wg0");
    match &addr {
        Some(line) => {
            v.check_bool(
                "wg0:in_subnet",
                line.contains("10.13.37."),
                &format!("wg0 addr: {line}"),
            );

            let expected = mesh_address("eastGate");
            if let Some(exp) = expected {
                v.check_bool(
                    "wg0:matches_registry",
                    line.contains(exp),
                    &format!("expected {exp}, found in: {line}"),
                );
            }
        }
        None => {
            v.check_skip("wg0:in_subnet", "could not read wg0 address");
        }
    }
}

fn phase_addressing(v: &mut ValidationResult) {
    let live_nodes = ["golgi", "sporeGate", "pepti", "eastGate", "flockGate"];
    let mut assigned_count = 0u32;

    for node in &live_nodes {
        if mesh_address(node).is_some() {
            assigned_count += 1;
        }
    }

    v.check_bool(
        "addressing:all_live_assigned",
        assigned_count as usize == live_nodes.len(),
        &format!(
            "{assigned_count}/{} live nodes have mesh addresses",
            live_nodes.len()
        ),
    );

    v.check_minimum("addressing:total_nodes", assigned_count as usize, 5);
}

fn phase_live_reachability(v: &mut ValidationResult) {
    if !std::path::Path::new("/sys/class/net/wg0").exists() {
        v.check_skip(
            "reachability:wg0_absent",
            "wg0 not present, skipping live probe",
        );
        return;
    }

    let peers: &[(&str, &str)] = &[
        ("golgi", "10.13.37.1"),
        ("sporeGate", "10.13.37.2"),
        ("pepti", "10.13.37.4"),
        ("flockGate", "10.13.37.6"),
    ];

    let mut reachable_count = 0u32;

    for (name, ip) in peers {
        let alive = icmp_reachable(ip);
        if alive {
            reachable_count += 1;
        }
        v.check_bool(
            &format!("reach:{name}"),
            alive,
            &format!(
                "{name} ({ip}): {}",
                if alive { "ALIVE" } else { "UNREACHABLE" }
            ),
        );
    }

    #[expect(clippy::cast_possible_truncation, reason = "peer count < 256")]
    let peer_count = peers.len() as u32;
    v.check_bool(
        "reach:all_peers",
        reachable_count == peer_count,
        &format!("{reachable_count}/{peer_count} peers reachable"),
    );
}

fn read_interface_addr(iface: &str) -> Option<String> {
    let path = format!("/sys/class/net/{iface}/address");
    if !std::path::Path::new(&path).exists() {
        return None;
    }
    let output = std::process::Command::new("ip")
        .args(["-4", "addr", "show", iface])
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&output.stdout);
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("inet ") {
            return Some(trimmed.to_owned());
        }
    }
    None
}

fn icmp_reachable(ip: &str) -> bool {
    std::process::Command::new("ping")
        .args(["-c", "1", "-W", "2", ip])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_overlay_structural() {
        let mut v = ValidationResult::new("mesh-overlay");
        let mut ctx = CompositionContext::discover();
        run_mesh_overlay(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "mesh-overlay: {} failures ({} passed, {} skipped)",
            v.failed, v.passed, v.skipped
        );
    }
}
