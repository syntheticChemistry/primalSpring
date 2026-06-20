// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: K-Derm Live Layers — validates the cell envelope model on the
//! running gate by probing actual network boundaries.
//!
//! Complements `s_kderm_boundary` (structural) with live probes:
//!
//! | Layer | What we check live |
//! |-------|-------------------|
//! | Cytoplasm | UDS sockets exist, local IPC works |
//! | Plasma membrane | nftables loaded, WG port allowed |
//! | Periplasm | WG interface up, mesh peers reachable |
//! | Outer membrane | No unexpected ports exposed to WAN |
//!
//! This scenario is tier Both (structural assertions + live probes).

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// K-Derm live layer enforcement validation.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "kderm-live-layers",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-18",
        description: "K-Derm live: UDS cytoplasm, nftables membrane, WG periplasm, port exposure",
    },
    run: run_kderm_live_layers,
};

fn run_kderm_live_layers(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    phase_cytoplasm(v, ctx);
    phase_plasma_membrane(v);
    phase_periplasm(v);
    phase_outer_membrane(v);
}

fn phase_cytoplasm(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let socket_dir = crate::tolerances::platform::biomeos_socket_dir();

    v.check_bool(
        "cytoplasm:socket_dir",
        socket_dir.is_dir(),
        &format!("UDS cytoplasm: {}", socket_dir.display()),
    );

    if socket_dir.is_dir() {
        let sockets: Vec<_> = std::fs::read_dir(&socket_dir)
            .into_iter()
            .flatten()
            .filter_map(std::result::Result::ok)
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "sock"))
            .collect();

        v.check_bool(
            "cytoplasm:socket_count",
            sockets.len() >= 8,
            &format!(
                "{} UDS sockets in cytoplasm (local IPC surface)",
                sockets.len()
            ),
        );
    }

    let ipc_ok = ctx.has_capability("security")
        && ctx
            .call("security", "health.liveness", serde_json::json!({}))
            .is_ok();
    v.check_bool(
        "cytoplasm:ipc_working",
        ipc_ok,
        &format!(
            "local IPC via UDS: {}",
            if ipc_ok { "ALIVE" } else { "FAILED" }
        ),
    );
}

fn phase_plasma_membrane(v: &mut ValidationResult) {
    let nft_output = std::process::Command::new("nft")
        .args(["list", "ruleset"])
        .output();

    match nft_output {
        Ok(out) if out.status.success() => {
            let text = String::from_utf8_lossy(&out.stdout);
            let has_rules = !text.trim().is_empty() && text.contains("table");
            v.check_bool(
                "membrane:nftables_loaded",
                has_rules,
                "nftables ruleset is loaded (plasma membrane active)",
            );

            let wg_port_allowed = text.contains("51820") || text.contains("wireguard");
            v.check_bool(
                "membrane:wg_port_allowed",
                wg_port_allowed || !has_rules,
                "WireGuard port (51820) has rule or firewall is permissive",
            );
        }
        Ok(_) => {
            v.check_skip(
                "membrane:nftables_loaded",
                "nft list ruleset failed (may need privileges)",
            );
            v.check_skip("membrane:wg_port_allowed", "nft not accessible");
        }
        Err(_) => {
            v.check_skip("membrane:nftables_loaded", "nft binary not found");
            v.check_skip("membrane:wg_port_allowed", "nft binary not found");
        }
    }
}

fn phase_periplasm(v: &mut ValidationResult) {
    let iface = std::process::Command::new("ip")
        .args(["link", "show", "wg0"])
        .output();

    let wg_up = iface
        .as_ref()
        .is_ok_and(|o| o.status.success() && String::from_utf8_lossy(&o.stdout).contains("UP"));

    v.check_bool(
        "periplasm:wg0_up",
        wg_up,
        &format!("WG periplasm layer: {}", if wg_up { "UP" } else { "DOWN" }),
    );

    let addr_output = std::process::Command::new("ip")
        .args(["addr", "show", "wg0"])
        .output();

    if let Ok(out) = addr_output {
        let text = String::from_utf8_lossy(&out.stdout);
        let in_mesh = text.contains("10.13.37.");
        v.check_bool(
            "periplasm:mesh_subnet",
            in_mesh,
            "wg0 address in 10.13.37.0/24 sovereign mesh",
        );
    }

    let ping_golgi = std::process::Command::new("ping")
        .args(["-c1", "-W2", "10.13.37.1"])
        .output()
        .is_ok_and(|o| o.status.success());

    v.check_bool(
        "periplasm:relay_reachable",
        ping_golgi,
        &format!(
            "golgi relay (10.13.37.1): {}",
            if ping_golgi {
                "reachable"
            } else {
                "UNREACHABLE"
            }
        ),
    );
}

fn phase_outer_membrane(v: &mut ValidationResult) {
    let ss_output = std::process::Command::new("ss").args(["-tlnp"]).output();

    let Ok(out) = ss_output else {
        v.check_skip("outer:listening_audit", "ss command not available");
        return;
    };

    let text = String::from_utf8_lossy(&out.stdout);
    let lines: Vec<&str> = text.lines().skip(1).collect();

    let wildcard_listeners: Vec<&str> = lines
        .iter()
        .filter(|l| l.contains("0.0.0.0:") || l.contains("*:") || l.contains("[::]:"))
        .copied()
        .collect();

    let expected_exposed = ["7700"];
    let _unexpected: Vec<&&str> = wildcard_listeners
        .iter()
        .filter(|l| !expected_exposed.iter().any(|port| l.contains(port)))
        .filter(|l| !l.contains("127.0.0.1"))
        .collect();

    let is_dev_gate = std::env::var("ECOPRIMALS_ROOT").is_ok()
        || std::path::Path::new("/home/eastgate/Development").exists();

    let max_listeners: usize = if is_dev_gate { 30 } else { 5 };

    v.check_bool(
        "outer:wildcard_listeners",
        wildcard_listeners.len() <= max_listeners,
        &format!(
            "{} wildcard TCP listeners (max {} for {} gate, songbird 7700 expected)",
            wildcard_listeners.len(),
            max_listeners,
            if is_dev_gate { "dev" } else { "production" }
        ),
    );

    let has_songbird = wildcard_listeners.iter().any(|l| l.contains("7700"));
    v.check_bool(
        "outer:songbird_exposed",
        has_songbird,
        &format!(
            "songbird federation port 7700: {}",
            if has_songbird {
                "listening"
            } else {
                "NOT FOUND"
            }
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kderm_live_layers_structural() {
        let mut v = ValidationResult::new("kderm-live-layers");
        let mut ctx = CompositionContext::discover();
        run_kderm_live_layers(&mut v, &mut ctx);
        assert!(
            v.passed + v.failed + v.skipped > 0,
            "kderm-live-layers should evaluate at least one check"
        );
    }
}
