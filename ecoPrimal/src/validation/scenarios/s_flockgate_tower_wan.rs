// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: flockGate Tower WAN — validates flockGate's Tower atomic
//! (BearDog, Songbird, SkunkBat) over the WireGuard WAN mesh.
//!
//! Phase 1 (Structural): flockGate zone, mesh address, and Tower composition model.
//! Phase 2 (Live): ICMP + TCP reachability of all three Tower primals at 10.13.37.6.

use std::net::{Ipv4Addr, SocketAddr, TcpStream};
use std::time::Duration;

use crate::composition::CompositionContext;
use crate::coordination::AtomicType;
use crate::evolution::gate::{CytoplasmZone, GateMatrix, mesh_address};
use crate::primal_names;
use crate::tolerances::{SCENARIO_TCP_PROBE_TIMEOUT_MS, WAN_HEALTH_MAX_MS, ports::port_entry_for};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const FLOCKGATE: &str = "flockGate";
const TOWER_PRIMALS: &[&str] = &[
    primal_names::BEARDOG,
    primal_names::SONGBIRD,
    primal_names::SKUNKBAT,
];

/// flockGate Tower WAN validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "flockgate-tower-wan",
        track: Track::AtomicComposition,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-20",
        description: "flockGate Tower atomic (BearDog, Songbird, SkunkBat) over WAN WireGuard mesh",
    },
    run,
};

/// Execute flockGate Tower WAN validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — flockGate Tower composition");
    phase_structural(v);

    v.section("Phase 2: Live — WAN mesh probe from eastGate");
    phase_live(v);
}

fn phase_structural(v: &mut ValidationResult) {
    let zone = CytoplasmZone::for_gate(FLOCKGATE);
    v.check_bool(
        "struct:flockgate:wan_zone",
        zone == CytoplasmZone::Wan,
        &format!("expected wan, got {}", zone.label()),
    );

    let mesh = mesh_address(FLOCKGATE);
    v.check_bool(
        "struct:flockgate:mesh_address",
        mesh.is_some(),
        &format!("flockGate mesh address resolved from SSOT: {mesh:?}"),
    );

    let matrix = GateMatrix::ecosystem_snapshot();
    let in_matrix = matrix.gates.iter().any(|g| g.name == FLOCKGATE);
    v.check_bool(
        "struct:flockgate:in_matrix",
        in_matrix,
        "flockGate tracked in ecosystem gate matrix",
    );

    if let Some(gate) = matrix.gates.iter().find(|g| g.name == FLOCKGATE) {
        v.check_bool(
            "struct:flockgate:matrix_zone",
            gate.zone == CytoplasmZone::Wan,
            &format!("matrix zone: {}", gate.zone.label()),
        );
    }

    let tower_slugs = AtomicType::Tower.required_primal_slugs();
    v.check_bool(
        "struct:tower:three_primals",
        tower_slugs.len() == 3,
        &format!(
            "Tower composition: {} primals ({})",
            tower_slugs.len(),
            tower_slugs.join(", ")
        ),
    );

    for slug in TOWER_PRIMALS {
        v.check_bool(
            &format!("struct:tower:slug:{slug}"),
            tower_slugs.contains(slug),
            &format!("{slug} in Tower required_primal_slugs"),
        );
    }

    v.check_bool(
        "struct:tower:graph_name",
        AtomicType::Tower.graph_name() == "tower_atomic_bootstrap",
        "Tower deploy graph registered",
    );

    for slug in TOWER_PRIMALS {
        let port = port_entry_for(slug).map_or(0, |e| e.port);
        v.check_bool(
            &format!("struct:port:{slug}"),
            port > 0,
            &format!("{slug} TCP port = {port}"),
        );
    }

    let east_mesh = mesh_address("eastGate");
    v.check_bool(
        "struct:eastgate:mesh_peer",
        east_mesh.is_some(),
        &format!("eastGate mesh address resolved for WAN probe origin: {east_mesh:?}"),
    );
}

fn phase_live(v: &mut ValidationResult) {
    if !std::path::Path::new("/sys/class/net/wg0").exists() {
        v.check_skip("live:wg0", "wg0 not present — skipping WAN mesh probes");
        return;
    }

    let Some(flockgate_ip) = mesh_address(FLOCKGATE) else {
        v.check_skip("live:flockgate:ping", "flockGate mesh address not in SSOT");
        return;
    };

    let ping_reachable = icmp_reachable(flockgate_ip);
    v.check_bool(
        "live:flockgate:ping",
        ping_reachable,
        &format!(
            "ping {flockgate_ip} ({FLOCKGATE}): {}",
            if ping_reachable {
                "REACHABLE"
            } else {
                "UNREACHABLE"
            }
        ),
    );

    if !ping_reachable {
        for slug in TOWER_PRIMALS {
            v.check_skip(
                &format!("live:tcp:{slug}"),
                &format!("{FLOCKGATE} unreachable — skipping {slug} TCP probe"),
            );
        }
        v.check_skip(
            "live:tower:all_reachable",
            "flockGate unreachable — cannot verify Tower primals over WAN",
        );
        return;
    }

    if let Some(rtt_ms) = measure_ping_rtt_ms(flockgate_ip) {
        v.check_bool(
            "live:flockgate:latency",
            rtt_ms <= WAN_HEALTH_MAX_MS,
            &format!("ping RTT {rtt_ms}ms (max {WAN_HEALTH_MAX_MS}ms)"),
        );
    }

    let timeout = Duration::from_millis(SCENARIO_TCP_PROBE_TIMEOUT_MS);
    let mut reachable_count = 0u32;

    for slug in TOWER_PRIMALS {
        let port = port_entry_for(slug).map_or(0, |e| e.port);
        if port == 0 {
            v.check_skip(
                &format!("live:tcp:{slug}"),
                &format!("no port registered for {slug}"),
            );
            continue;
        }

        let reachable = probe_tcp(flockgate_ip, port, timeout);
        if reachable {
            reachable_count += 1;
        }
        v.check_bool(
            &format!("live:tcp:{slug}"),
            reachable,
            &format!(
                "{slug} @ {flockgate_ip}:{port}: {}",
                if reachable {
                    "REACHABLE"
                } else {
                    "UNREACHABLE"
                }
            ),
        );
    }

    #[expect(clippy::cast_possible_truncation, reason = "Tower has 3 primals")]
    let expected = TOWER_PRIMALS.len() as u32;
    v.check_bool(
        "live:tower:all_reachable",
        reachable_count == expected,
        &format!("{reachable_count}/{expected} Tower primals TCP-reachable over WAN"),
    );
}

fn icmp_reachable(ip: &str) -> bool {
    std::process::Command::new("ping")
        .args(["-c", "1", "-W", "2", ip])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

fn measure_ping_rtt_ms(ip: &str) -> Option<u64> {
    let output = std::process::Command::new("ping")
        .args(["-c", "1", "-W", "2", ip])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8_lossy(&output.stdout);
    for line in text.lines() {
        if let Some(rest) = line.split("time=").nth(1) {
            let ms_str: String = rest
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.')
                .collect();
            if let Ok(ms) = ms_str.parse::<f64>() {
                #[expect(clippy::cast_possible_truncation, reason = "ping RTT ms always small")]
                #[expect(clippy::cast_sign_loss, reason = "ping RTT always positive")]
                return Some(ms as u64);
            }
        }
    }
    Some(0)
}

fn probe_tcp(host: &str, port: u16, timeout: Duration) -> bool {
    let Ok(ip) = host.parse::<Ipv4Addr>() else {
        return false;
    };
    let addr = SocketAddr::new(ip.into(), port);
    TcpStream::connect_timeout(&addr, timeout).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flockgate_tower_wan_structural() {
        let mut v = ValidationResult::new("flockgate-tower-wan");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.passed + v.failed + v.skipped > 0,
            "flockgate-tower-wan should evaluate at least one check"
        );
    }

    #[test]
    fn tower_ports_registered() {
        for slug in TOWER_PRIMALS {
            assert!(
                port_entry_for(slug).is_some_and(|e| e.port > 0),
                "{slug} should have a registered TCP port"
            );
        }
    }

    #[test]
    fn tcp_probe_timeout_within_wan_budget() {
        assert!(SCENARIO_TCP_PROBE_TIMEOUT_MS > 0);
        assert!(
            SCENARIO_TCP_PROBE_TIMEOUT_MS <= crate::tolerances::TCP_CONNECT_TIMEOUT_SECS * 1000
        );
    }
}
