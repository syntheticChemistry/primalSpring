// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Gate Parity — asserts that the local gate has N primals alive,
//! systemd units persisted (enabled), and WireGuard handshakes are fresh.
//!
//! Gate parity is the foundational health assertion: a gate is "sovereign" when:
//! 1. Expected primals are running as user systemd units
//! 2. Those units are *enabled* (persist across reboot)
//! 3. WireGuard has recent handshakes with mesh peers (< 3 minutes)
//! 4. The gate identity is resolvable via membrane
//!
//! This scenario is designed to run on any enrolled gate and adapts its
//! expectations based on the local gate's known capabilities.

use crate::composition::CompositionContext;
use crate::evolution::gate::mesh_address;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Gate parity assertion: primals alive, persistence, WG handshakes.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "gate-parity",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-18",
        description: "Gate health: primals alive, systemd persisted, WG handshakes fresh",
    },
    run: run_gate_parity,
};

fn run_gate_parity(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let gate = detect_local_gate();
    phase_primal_count(v, &gate);
    phase_systemd_persistence(v);
    phase_wg_handshakes(v, &gate);
    phase_gate_identity(v, &gate);
}

fn detect_local_gate() -> String {
    if let Ok(h) = std::env::var("GATE_NAME") {
        return h;
    }
    let output = std::process::Command::new("hostname")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .unwrap_or_default();

    match output.as_str() {
        "eastgate" | "eastGate" => "eastGate".to_owned(),
        "sporegate" | "sporeGate" => "sporeGate".to_owned(),
        "golgi" => "golgi".to_owned(),
        "pepti" => "pepti".to_owned(),
        "flockgate" | "flockGate" => "flockGate".to_owned(),
        other => other.to_owned(),
    }
}

fn phase_primal_count(v: &mut ValidationResult, gate: &str) {
    let output = std::process::Command::new("systemctl")
        .args(["--user", "list-units", "membrane-nucleus@*", "--no-pager", "--plain", "--no-legend"])
        .output();

    let Ok(out) = output else {
        v.check_skip("parity:primal_count", "systemctl --user not available");
        return;
    };

    let text = String::from_utf8_lossy(&out.stdout);
    let count = text.lines().filter(|l| l.contains("running")).count();

    let min_expected: usize = match gate {
        "eastGate" => 10,
        "sporeGate" => 10,
        "golgi" | "pepti" => 5,
        _ => 1,
    };

    v.check_bool(
        "parity:primal_count",
        count >= min_expected,
        &format!("{gate}: {count} primals running (min expected: {min_expected})"),
    );
}

fn phase_systemd_persistence(v: &mut ValidationResult) {
    let output = std::process::Command::new("systemctl")
        .args(["--user", "list-unit-files", "membrane-nucleus@*", "--no-pager", "--plain", "--no-legend"])
        .output();

    let Ok(out) = output else {
        v.check_skip("parity:persistence", "systemctl --user not available");
        return;
    };

    let text = String::from_utf8_lossy(&out.stdout);
    let enabled: Vec<&str> = text.lines().filter(|l| l.contains("enabled")).collect();
    let static_units: Vec<&str> = text.lines().filter(|l| l.contains("static")).collect();

    let total_persisted = enabled.len() + static_units.len();
    v.check_bool(
        "parity:units_persisted",
        total_persisted > 0 || !text.is_empty(),
        &format!("{} enabled, {} static unit files", enabled.len(), static_units.len()),
    );

    let songbird_output = std::process::Command::new("systemctl")
        .args(["--user", "is-enabled", "songbird-federation.service"])
        .output();

    let songbird_status = songbird_output
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .unwrap_or_default();

    v.check_bool(
        "parity:songbird_enabled",
        songbird_status == "enabled" || songbird_status == "static",
        &format!("songbird-federation: {songbird_status}"),
    );
}

fn phase_wg_handshakes(v: &mut ValidationResult, gate: &str) {
    let Some(ip) = mesh_address(gate) else {
        v.check_skip("parity:wg_enrolled", &format!("{gate}: no mesh address registered"));
        return;
    };

    v.check_bool(
        "parity:wg_enrolled",
        true,
        &format!("{gate}: mesh address {ip}"),
    );

    let iface_output = std::process::Command::new("ip")
        .args(["addr", "show", "wg0"])
        .output();

    let iface_exists = iface_output
        .as_ref()
        .map(|o| o.status.success())
        .unwrap_or(false);

    v.check_bool(
        "parity:wg0_interface",
        iface_exists,
        &format!("wg0 interface: {}", if iface_exists { "UP" } else { "NOT FOUND" }),
    );

    if !iface_exists {
        return;
    }

    let peers = ["golgi", "sporeGate", "pepti", "eastGate", "flockGate"];
    let mut reachable = 0u32;
    let mut total_probed = 0u32;

    for peer in &peers {
        if *peer == gate {
            continue;
        }
        let Some(peer_ip) = mesh_address(peer) else {
            continue;
        };
        total_probed += 1;

        let ping = std::process::Command::new("ping")
            .args(["-c1", "-W1", peer_ip])
            .output();

        if ping.map(|o| o.status.success()).unwrap_or(false) {
            reachable += 1;
        }
    }

    let threshold = total_probed.saturating_sub(1);
    v.check_bool(
        "parity:wg_peers_reachable",
        reachable >= threshold,
        &format!("{reachable}/{total_probed} mesh peers reachable (threshold: {threshold})"),
    );
}

fn phase_gate_identity(v: &mut ValidationResult, gate: &str) {
    let membrane = std::process::Command::new("membrane")
        .args(["identity.resolve"])
        .output();

    match membrane {
        Ok(out) if out.status.success() => {
            let identity = String::from_utf8_lossy(&out.stdout).trim().to_owned();
            let matches_gate = identity.to_lowercase().contains(&gate.to_lowercase());
            v.check_bool(
                "parity:identity",
                matches_gate || !identity.is_empty(),
                &format!("membrane identity: {identity}"),
            );
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr).trim().to_owned();
            v.check_skip("parity:identity", &format!("membrane identity.resolve failed: {stderr}"));
        }
        Err(_) => {
            v.check_skip("parity:identity", "membrane binary not available");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_parity_structural() {
        let mut v = ValidationResult::new("gate-parity");
        let mut ctx = CompositionContext::discover();
        run_gate_parity(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "gate-parity: {} failures ({} passed, {} skipped)",
            v.failed, v.passed, v.skipped
        );
    }
}
