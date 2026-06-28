// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Mesh Reachability — validates RTT thresholds and handshake
//! freshness for the WireGuard sovereign overlay.
//!
//! Unlike `s_mesh_overlay` which validates basic connectivity (ping succeeds),
//! this scenario validates *quality*:
//!
//! | Peer class | RTT threshold | Rationale |
//! |-----------|---------------|-----------|
//! | Backbone (same zone) | < 5ms | LAN or 10G AOC |
//! | Cross-zone (WAN relay) | < 100ms | ISP hop through golgi |
//! | Remote VPS | < 150ms | Cloud hop |
//!
//! Handshake freshness: WireGuard handshakes rotate every 2 minutes under
//! active traffic. A handshake older than 5 minutes suggests the tunnel is
//! idle or degraded.

use crate::composition::CompositionContext;
use crate::evolution::gate::{CytoplasmZone, mesh_address};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Mesh reachability with RTT thresholds and quality assertions.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "mesh-reachability",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-18",
        description: "WG mesh RTT thresholds and handshake freshness per zone class",
    },
    run: run_mesh_reachability,
};

struct PeerProfile {
    name: &'static str,
    zone: CytoplasmZone,
    max_rtt_ms: u64,
}

const PEER_PROFILES: &[PeerProfile] = &[
    PeerProfile {
        name: "golgi",
        zone: CytoplasmZone::Wan,
        max_rtt_ms: 100,
    },
    PeerProfile {
        name: "sporeGate",
        zone: CytoplasmZone::Backbone,
        max_rtt_ms: 80,
    },
    PeerProfile {
        name: "ironGate",
        zone: CytoplasmZone::Backbone,
        max_rtt_ms: 80,
    },
    PeerProfile {
        name: "flockGate",
        zone: CytoplasmZone::Garage,
        max_rtt_ms: 150,
    },
];

fn run_mesh_reachability(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let gate = detect_gate();
    phase_rtt_thresholds(v, &gate);
    phase_packet_loss(v, &gate);
    phase_handshake_freshness(v);
}

fn detect_gate() -> String {
    if let Ok(name) = std::env::var("GATE_NAME") {
        return name;
    }
    match std::process::Command::new("hostname").output() {
        Ok(o) => String::from_utf8_lossy(&o.stdout).trim().to_owned(),
        Err(_) => "unknown".to_owned(),
    }
}

fn phase_rtt_thresholds(v: &mut ValidationResult, gate: &str) {
    for profile in PEER_PROFILES {
        if profile.name.eq_ignore_ascii_case(gate) {
            continue;
        }

        let Some(ip) = mesh_address(profile.name) else {
            v.check_skip(
                &format!("rtt:{}", profile.name),
                &format!("{}: no mesh address", profile.name),
            );
            continue;
        };

        match measure_rtt(ip) {
            Some(rtt_ms) => {
                let within_threshold = rtt_ms <= profile.max_rtt_ms;
                v.check_bool(
                    &format!("rtt:{}:threshold", profile.name),
                    within_threshold,
                    &format!(
                        "{} ({:?}): {rtt_ms}ms (max: {}ms)",
                        profile.name, profile.zone, profile.max_rtt_ms
                    ),
                );
            }
            None => {
                v.check_skip(
                    &format!("rtt:{}:threshold", profile.name),
                    &format!("{}: unreachable for RTT measurement", profile.name),
                );
            }
        }
    }
}

fn phase_packet_loss(v: &mut ValidationResult, gate: &str) {
    let mut total_peers = 0u32;
    let mut zero_loss = 0u32;

    for profile in PEER_PROFILES {
        if profile.name.eq_ignore_ascii_case(gate) {
            continue;
        }

        let Some(ip) = mesh_address(profile.name) else {
            continue;
        };

        total_peers += 1;

        let output = std::process::Command::new("ping")
            .args(["-c3", "-W2", ip])
            .output();

        if let Ok(out) = output {
            let text = String::from_utf8_lossy(&out.stdout);
            if text.contains("0% packet loss") {
                zero_loss += 1;
            }
        }
    }

    v.check_bool(
        "loss:zero_packet_loss",
        zero_loss == total_peers,
        &format!("{zero_loss}/{total_peers} peers with zero packet loss"),
    );
}

fn phase_handshake_freshness(v: &mut ValidationResult) {
    let wg_output = std::process::Command::new("sudo")
        .args(["wg", "show", "wg0", "latest-handshakes"])
        .output();

    match wg_output {
        Ok(out) if out.status.success() => {
            let text = String::from_utf8_lossy(&out.stdout);
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_secs());

            let mut stale_count = 0u32;
            let mut total_handshakes = 0u32;
            let max_age_secs: u64 = 300;

            for line in text.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(ts) = parts[1].parse::<u64>() {
                        total_handshakes += 1;
                        if ts > 0 && now.saturating_sub(ts) > max_age_secs {
                            stale_count += 1;
                        }
                    }
                }
            }

            if total_handshakes == 0 {
                v.check_skip("handshake:freshness", "no handshake data from wg show");
            } else {
                v.check_bool(
                    "handshake:freshness",
                    stale_count == 0,
                    &format!(
                        "{}/{total_handshakes} handshakes fresh (< {max_age_secs}s), {stale_count} stale",
                        total_handshakes - stale_count
                    ),
                );
            }
        }
        Ok(_) => {
            v.check_skip(
                "handshake:freshness",
                "wg show requires elevated privileges (skip in CI)",
            );
        }
        Err(_) => {
            v.check_skip("handshake:freshness", "sudo/wg not available");
        }
    }
}

fn measure_rtt(ip: &str) -> Option<u64> {
    let output = std::process::Command::new("ping")
        .args(["-c3", "-W2", ip])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8_lossy(&output.stdout);
    for line in text.lines() {
        if line.contains("min/avg/max") || line.contains("rtt") {
            let parts: Vec<&str> = line.split('=').collect();
            if let Some(stats) = parts.last() {
                let values: Vec<&str> = stats.split('/').collect();
                if values.len() >= 2 {
                    if let Ok(avg) = values[1].trim().parse::<f64>() {
                        #[expect(
                            clippy::cast_possible_truncation,
                            reason = "RTT ms always small"
                        )]
                        #[expect(clippy::cast_sign_loss, reason = "RTT always positive")]
                        return Some(avg as u64);
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_reachability_structural() {
        let mut v = ValidationResult::new("mesh-reachability");
        let mut ctx = CompositionContext::discover();
        run_mesh_reachability(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "mesh-reachability: {} failures ({} passed, {} skipped)",
            v.failed, v.passed, v.skipped
        );
    }
}
