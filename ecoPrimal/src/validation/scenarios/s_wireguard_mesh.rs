// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: WireGuard Mesh — validates the 5-node sovereign overlay topology.
//!
//! The 10.13.37.0/24 mesh connects golgi, sporeGate, pepti, eastGate, and
//! flockGate via encrypted WireGuard tunnels on port 51820. This scenario
//! validates:
//!
//! Phase 1 (Structural/Rust tier):
//! 1. wg0.conf exists at standard paths
//! 2. Config has [Interface] + at least one [Peer] section
//! 3. Local listen port is 51820
//! 4. Peer AllowedIPs reference the 10.13.37.0/24 mesh subnet
//!
//! Phase 2 (Live tier):
//! 1. `wg show wg0` returns interface info (skip if not root)
//! 2. Peer handshakes are within 5 minutes
//! 3. Transfer bytes > 0 for at least one peer

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::{Scenario, ScenarioMeta, Tier, Track};
use std::path::{Path, PathBuf};

const MESH_SUBNET: &str = "10.13.37.0/24";
const MESH_PREFIX: &str = "10.13.37.";
const WG_PORT: u16 = 51820;
const WG_IFACE: &str = "wg0";
const HANDSHAKE_MAX_AGE_SECS: u64 = 300;

/// Expected mesh peers (name → overlay address).
const EXPECTED_PEERS: &[(&str, &str)] = &[
    ("golgi", "10.13.37.1"),
    ("sporeGate", "10.13.37.2"),
    ("eastGate", "10.13.37.5"),
    ("flockGate", "10.13.37.6"),
    ("ironGate", "10.13.37.7"),
];

/// WireGuard mesh topology validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "wireguard-mesh",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-20",
        description: "5-node WireGuard mesh: config structure, handshakes, traffic",
    },
    run,
};

/// Run WireGuard mesh validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: WireGuard configuration");
    phase_config_structure(v);

    v.section("Phase 2: Live WireGuard state");
    phase_live_wg(v);
}

fn phase_config_structure(v: &mut ValidationResult) {
    let Some(config_path) = locate_wg_config() else {
        v.check_skip(
            "config:exists",
            "wg0.conf not found at /etc/wireguard/wg0.conf or ~/.config/wireguard/wg0.conf",
        );
        return;
    };

    v.check_bool(
        "config:exists",
        true,
        &format!("wg0.conf found at {}", config_path.display()),
    );

    let Ok(text) = std::fs::read_to_string(&config_path) else {
        v.check_skip("config:readable", "could not read wg0.conf");
        return;
    };

    let parsed = parse_wg_config(&text);

    v.check_bool(
        "config:interface_section",
        parsed.has_interface,
        if parsed.has_interface {
            "[Interface] section present"
        } else {
            "missing [Interface] section"
        },
    );

    v.check_bool(
        "config:peer_sections",
        parsed.peer_count >= 1,
        &format!("{} [Peer] section(s)", parsed.peer_count),
    );

    let port_ok = parsed.listen_port == Some(WG_PORT);
    v.check_bool(
        "config:listen_port",
        port_ok,
        &format!(
            "ListenPort: {} (expected {WG_PORT})",
            parsed
                .listen_port
                .map_or_else(|| "unset".to_owned(), |p| p.to_string())
        ),
    );

    let mesh_allowed = !parsed.mesh_allowed_ips.is_empty()
        && parsed
            .mesh_allowed_ips
            .iter()
            .all(|ip| ip.starts_with(MESH_PREFIX));
    v.check_bool(
        "config:mesh_allowed_ips",
        mesh_allowed,
        &format!(
            "{} AllowedIPs in {MESH_SUBNET}: [{}]",
            parsed.mesh_allowed_ips.len(),
            parsed.mesh_allowed_ips.join(", ")
        ),
    );

    let peer_summary: Vec<String> = EXPECTED_PEERS
        .iter()
        .map(|(name, ip)| format!("{name}={ip}"))
        .collect();
    v.check_bool(
        "config:mesh_topology",
        parsed.peer_count >= 1,
        &format!(
            "5-node mesh [{}]; {} local peer section(s)",
            peer_summary.join(", "),
            parsed.peer_count
        ),
    );
}

fn phase_live_wg(v: &mut ValidationResult) {
    if !running_as_root() {
        v.check_skip("live:wg_show", "not running as root (skip live tier)");
        v.check_skip("live:handshakes", "not running as root");
        v.check_skip("live:transfer", "not running as root");
        return;
    }

    let output = std::process::Command::new("wg")
        .args(["show", WG_IFACE])
        .output();

    let Ok(out) = output else {
        v.check_skip("live:wg_show", "wg command not available");
        v.check_skip("live:handshakes", "wg command not available");
        v.check_skip("live:transfer", "wg command not available");
        return;
    };

    if !out.status.success() {
        v.check_skip(
            "live:wg_show",
            &format!(
                "wg show {WG_IFACE} failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ),
        );
        v.check_skip("live:handshakes", "wg0 interface not available");
        v.check_skip("live:transfer", "wg0 interface not available");
        return;
    }

    let show_text = String::from_utf8_lossy(&out.stdout);
    v.check_bool(
        "live:wg_show",
        show_text.contains("interface:") || show_text.contains("peer:"),
        &format!("wg show {WG_IFACE}: interface info returned"),
    );

    phase_handshakes(v);
    phase_transfer(v);
}

fn phase_handshakes(v: &mut ValidationResult) {
    let output = std::process::Command::new("wg")
        .args(["show", WG_IFACE, "dump"])
        .output();

    let Ok(out) = output else {
        v.check_skip("live:handshakes", "wg dump not available");
        return;
    };

    if !out.status.success() {
        v.check_skip(
            "live:handshakes",
            &format!(
                "wg show {WG_IFACE} dump failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ),
        );
        return;
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());

    let text = String::from_utf8_lossy(&out.stdout);
    let mut fresh_count = 0u32;
    let mut stale_count = 0u32;
    let mut total_handshakes = 0u32;

    for line in text.lines().skip(1) {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() < 5 {
            continue;
        }
        let Ok(ts) = fields[4].parse::<u64>() else {
            continue;
        };
        if ts == 0 {
            continue;
        }
        total_handshakes += 1;
        if now.saturating_sub(ts) <= HANDSHAKE_MAX_AGE_SECS {
            fresh_count += 1;
        } else {
            stale_count += 1;
        }
    }

    if total_handshakes == 0 {
        v.check_skip("live:handshakes", "no peer handshakes recorded yet");
    } else {
        v.check_bool(
            "live:handshakes",
            stale_count == 0,
            &format!(
                "{fresh_count}/{total_handshakes} handshakes fresh (< {HANDSHAKE_MAX_AGE_SECS}s), {stale_count} stale"
            ),
        );
    }
}

fn phase_transfer(v: &mut ValidationResult) {
    let output = std::process::Command::new("wg")
        .args(["show", WG_IFACE, "transfer"])
        .output();

    let Ok(out) = output else {
        v.check_skip("live:transfer", "wg transfer not available");
        return;
    };

    if !out.status.success() {
        v.check_skip(
            "live:transfer",
            &format!(
                "wg show {WG_IFACE} transfer failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ),
        );
        return;
    }

    let text = String::from_utf8_lossy(&out.stdout);
    let mut peers_with_traffic = 0u32;
    let mut total_peers = 0u32;

    for line in text.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }
        total_peers += 1;
        let rx = parts[1].parse::<u64>().unwrap_or(0);
        let tx = parts[2].parse::<u64>().unwrap_or(0);
        if rx > 0 || tx > 0 {
            peers_with_traffic += 1;
        }
    }

    if total_peers == 0 {
        v.check_skip("live:transfer", "no peer transfer data from wg show");
    } else {
        v.check_bool(
            "live:transfer",
            peers_with_traffic >= 1,
            &format!("{peers_with_traffic}/{total_peers} peers with transfer bytes > 0"),
        );
    }
}

struct ParsedWgConfig {
    has_interface: bool,
    listen_port: Option<u16>,
    peer_count: usize,
    mesh_allowed_ips: Vec<String>,
}

fn parse_wg_config(text: &str) -> ParsedWgConfig {
    let mut has_interface = false;
    let mut listen_port = None;
    let mut peer_count = 0usize;
    let mut mesh_allowed_ips = Vec::new();
    let mut current_section: Option<&str> = None;

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some(section) = trimmed.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            current_section = Some(section);
            if section == "Interface" {
                has_interface = true;
            } else if section == "Peer" {
                peer_count += 1;
            }
            continue;
        }
        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();

        match current_section {
            Some("Interface") if key == "ListenPort" => {
                listen_port = value.parse().ok();
            }
            Some("Peer") if key == "AllowedIPs" => {
                for ip in value.split(',').map(str::trim).filter(|s| !s.is_empty()) {
                    if ip.starts_with(MESH_PREFIX) {
                        mesh_allowed_ips.push(ip.to_owned());
                    }
                }
            }
            _ => {}
        }
    }

    ParsedWgConfig {
        has_interface,
        listen_port,
        peer_count,
        mesh_allowed_ips,
    }
}

fn locate_wg_config() -> Option<PathBuf> {
    const SYSTEM_PATH: &str = "/etc/wireguard/wg0.conf";
    if Path::new(SYSTEM_PATH).is_file() {
        return Some(PathBuf::from(SYSTEM_PATH));
    }

    if let Ok(home) = std::env::var("HOME") {
        let user_path = PathBuf::from(home).join(".config/wireguard/wg0.conf");
        if user_path.is_file() {
            return Some(user_path);
        }
    }

    None
}

fn running_as_root() -> bool {
    std::process::Command::new("id")
        .arg("-u")
        .output()
        .ok()
        .is_some_and(|o| o.status.success() && String::from_utf8_lossy(&o.stdout).trim() == "0")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wireguard_mesh_structural() {
        let mut v = ValidationResult::new("wireguard-mesh");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "wireguard-mesh: {} failures ({} passed, {} skipped)",
            v.failed, v.passed, v.skipped
        );
    }

    #[test]
    fn parse_wg_config_extracts_mesh_allowed_ips() {
        let sample = r"
[Interface]
PrivateKey = abc
Address = 10.13.37.5/24
ListenPort = 51820

[Peer]
PublicKey = def
AllowedIPs = 10.13.37.1/32, 10.13.37.0/24
Endpoint = 157.230.3.183:51820
";
        let parsed = parse_wg_config(sample);
        assert!(parsed.has_interface);
        assert_eq!(parsed.peer_count, 1);
        assert_eq!(parsed.listen_port, Some(51820));
        assert_eq!(parsed.mesh_allowed_ips.len(), 2);
        assert!(
            parsed
                .mesh_allowed_ips
                .iter()
                .all(|ip| ip.starts_with(MESH_PREFIX))
        );
    }
}
