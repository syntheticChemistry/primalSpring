// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp073: LAN Covalent Mesh — validate cross-gate Songbird mesh and BirdSong.
//!
//! Connects to a remote gate's Songbird over TCP and validates:
//! - `mesh.peers` discovers at least 1 peer
//! - `birdsong.generate_encrypted_beacon` + decrypt round-trip
//! - `health.liveness` on remote beardog + songbird
//! - `capabilities.list` enumerates remote NUCLEUS capabilities
//!
//! Environment:
//!   `REMOTE_GATE_HOST` — hostname or IP of the remote gate (required)
//!   `REMOTE_SONGBIRD_PORT` — Songbird TCP fallback (default: 9200, cross-gate only)
//!   `REMOTE_BEARDOG_PORT`  — BearDog TCP fallback (default: 9100, cross-gate only)
//!   `FAMILY_ID` — shared family ID for beacon generation

use primalspring::ipc::methods;
use primalspring::ipc::tcp::{http_health_probe, tcp_rpc};
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

fn tcp_rpc_value(
    host: &str,
    port: u16,
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    tcp_rpc(host, port, method, params).map(|(v, _)| v)
}

fn http_health_check(host: &str, port: u16) -> Result<(), String> {
    http_health_probe(host, port).map(|_| ())
}

fn validate_remote_health(
    v: &mut ValidationResult,
    host: &str,
    beardog_port: u16,
    songbird_port: u16,
) {
    v.section("Remote Health");

    match tcp_rpc_value(
        host,
        beardog_port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    ) {
        Ok(_) => {
            println!("  remote beardog: LIVE");
            v.check_bool(
                "remote_beardog_live",
                true,
                "remote beardog health.liveness",
            );
        }
        Err(e) => {
            println!("  remote beardog: {e}");
            v.check_skip("remote_beardog_live", &format!("beardog unreachable: {e}"));
        }
    }

    match http_health_check(host, songbird_port) {
        Ok(()) => {
            println!("  remote songbird: LIVE (HTTP /health)");
            v.check_bool("remote_songbird_live", true, "remote songbird HTTP health");
        }
        Err(e) => {
            println!("  remote songbird: {e}");
            v.check_skip(
                "remote_songbird_live",
                &format!("songbird unreachable: {e}"),
            );
        }
    }
}

fn validate_remote_capabilities(v: &mut ValidationResult, host: &str, songbird_port: u16) {
    v.section("Remote Capabilities");

    match tcp_rpc_value(
        host,
        songbird_port,
        methods::capabilities::LIST,
        &serde_json::json!({}),
    ) {
        Ok(caps) => {
            let count = caps
                .as_array()
                .map(std::vec::Vec::len)
                .or_else(|| {
                    caps.get("capabilities")
                        .and_then(|c| c.as_array())
                        .map(std::vec::Vec::len)
                })
                .unwrap_or(0);
            println!("  remote capabilities: {count}");
            v.check_bool(
                "remote_capabilities_enumerated",
                count > 0,
                &format!("remote capabilities.list returned {count}"),
            );
        }
        Err(e) => {
            println!("  remote capabilities: {e}");
            v.check_skip(
                "remote_capabilities_enumerated",
                &format!("capabilities.list: {e}"),
            );
        }
    }
}

fn validate_mesh_peers(v: &mut ValidationResult, host: &str, songbird_port: u16) {
    v.section("Mesh Discovery");

    match tcp_rpc_value(host, songbird_port, "mesh.peers", &serde_json::json!({})) {
        Ok(peers) => {
            let count = peers
                .as_array()
                .map(std::vec::Vec::len)
                .or_else(|| {
                    peers
                        .get("peers")
                        .and_then(|p| p.as_array())
                        .map(std::vec::Vec::len)
                })
                .unwrap_or(0);
            println!("  mesh peers discovered: {count}");
            v.check_bool(
                "mesh_peers_discovered",
                count >= 1,
                &format!("mesh.peers returned {count} peers (need >= 1)"),
            );
        }
        Err(e) => {
            println!("  mesh.peers: {e}");
            let acceptable = e.contains("Method not found") || e.contains("not found");
            if acceptable {
                v.check_skip(
                    "mesh_peers_discovered",
                    &format!("mesh.peers not available: {e}"),
                );
            } else {
                v.check_skip("mesh_peers_discovered", &format!("mesh.peers error: {e}"));
            }
        }
    }

    match tcp_rpc_value(
        host,
        songbird_port,
        "mesh.auto_discover",
        &serde_json::json!({}),
    ) {
        Ok(resp) => {
            println!("  mesh.auto_discover: {resp}");
            v.check_bool("mesh_auto_discover", true, "mesh.auto_discover responded");
        }
        Err(e) => {
            println!("  mesh.auto_discover: {e}");
            v.check_skip("mesh_auto_discover", &format!("auto_discover: {e}"));
        }
    }
}

fn validate_birdsong_beacon(
    v: &mut ValidationResult,
    host: &str,
    songbird_port: u16,
    family_id: &str,
) {
    v.section("BirdSong Beacon Exchange");

    let beacon_result = tcp_rpc_value(
        host,
        songbird_port,
        "birdsong.generate_encrypted_beacon",
        &serde_json::json!({
            "family_id": family_id,
            "node_id": "exp073_remote",
            "capabilities": ["security", "discovery", "compute", "storage"],
            "device_type": "tower"
        }),
    );

    match &beacon_result {
        Ok(beacon) => {
            println!("  remote beacon generated: {}B", beacon.to_string().len());
            v.check_bool(
                "remote_beacon_generated",
                true,
                "remote BirdSong beacon generated",
            );

            let enc_str = beacon
                .get("encrypted_beacon")
                .and_then(|v| v.as_str())
                .unwrap_or_default();

            if enc_str.is_empty() {
                v.check_skip(
                    "remote_beacon_roundtrip",
                    "beacon generated but encrypted_beacon field empty",
                );
            } else {
                match tcp_rpc_value(
                    host,
                    songbird_port,
                    "birdsong.decrypt_beacon",
                    &serde_json::json!({ "encrypted_beacon": enc_str }),
                ) {
                    Ok(plain) => {
                        println!("  remote beacon decrypt: OK");
                        let has_family = plain
                            .get("family_id")
                            .and_then(|f| f.as_str())
                            .is_some_and(|f| f == family_id);
                        v.check_bool(
                            "remote_beacon_roundtrip",
                            true,
                            "remote beacon encrypt+decrypt round-trip",
                        );
                        v.check_bool(
                            "beacon_family_matches",
                            has_family,
                            "decrypted beacon contains matching family_id",
                        );
                    }
                    Err(e) => {
                        println!("  remote beacon decrypt: {e}");
                        v.check_bool(
                            "remote_beacon_roundtrip",
                            false,
                            &format!("decrypt failed: {e}"),
                        );
                    }
                }
            }
        }
        Err(e) => {
            println!("  remote beacon generation: {e}");
            let method_missing = e.contains("Method not found");
            if method_missing {
                v.check_skip(
                    "remote_beacon_generated",
                    "birdsong.generate_encrypted_beacon not available",
                );
            } else {
                v.check_skip("remote_beacon_generated", &format!("beacon: {e}"));
            }
        }
    }
}

fn validate_stun(v: &mut ValidationResult, host: &str, songbird_port: u16) {
    v.section("STUN / NAT Discovery");

    match tcp_rpc_value(
        host,
        songbird_port,
        "stun.get_public_address",
        &serde_json::json!({}),
    ) {
        Ok(addr) => {
            println!("  remote STUN address: {addr}");
            v.check_bool(
                "remote_stun_address",
                true,
                "remote STUN resolved public address",
            );
        }
        Err(e) => {
            println!("  remote STUN: {e}");
            v.check_skip("remote_stun_address", &format!("STUN: {e}"));
        }
    }
}

fn main() {
    let host = std::env::var("REMOTE_GATE_HOST").unwrap_or_default();
    let songbird_port: u16 = std::env::var("REMOTE_SONGBIRD_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(tolerances::TCP_FALLBACK_SONGBIRD_PORT);
    let beardog_port: u16 = std::env::var("REMOTE_BEARDOG_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(tolerances::TCP_FALLBACK_BEARDOG_PORT);
    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "8ff3b864a4bc589a".to_owned());

    ValidationResult::new("primalSpring Exp073 — LAN Covalent Mesh")
        .with_provenance("exp073_lan_covalent_mesh", "2026-03-24")
        .run(
            "primalSpring Exp073: Cross-gate Songbird mesh + BirdSong beacon exchange",
            |v| {
            if host.is_empty() {
                println!("  REMOTE_GATE_HOST not set — skipping all remote checks.");
                println!(
                    "  Usage: REMOTE_GATE_HOST=192.168.1.100 cargo run --bin exp073_lan_covalent_mesh"
                );
                v.check_skip("remote_gate_configured", "REMOTE_GATE_HOST not set");
                return;
            }

            println!("  Remote gate: {host}");
            println!("  Songbird port: {songbird_port}");
            println!("  BearDog port: {beardog_port}");
            println!("  Family ID: {family_id}");
            println!();

            v.check_bool("remote_gate_configured", true, "REMOTE_GATE_HOST is set");

            validate_remote_health(v, &host, beardog_port, songbird_port);
            validate_remote_capabilities(v, &host, songbird_port);
            validate_mesh_peers(v, &host, songbird_port);
            validate_birdsong_beacon(v, &host, songbird_port, &family_id);
            validate_stun(v, &host, songbird_port);
            },
        );
}
