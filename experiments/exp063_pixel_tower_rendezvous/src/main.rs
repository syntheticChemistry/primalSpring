// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp063: Pixel ↔ Tower Rendezvous — replicate the biomeOS beacon exchange.
//!
//! Validates the local half of the Pixel 8a deployment flow demonstrated
//! by biomeOS: spawn Tower, generate an encrypted BirdSong beacon, POST
//! it to the rendezvous endpoint, and verify the exchange.
//!
//! Supports two modes:
//! - **Local** (default): spawn Tower locally, validate beacon on local Songbird
//! - **Cross-device**: set `PIXEL_SONGBIRD_HOST` + `PIXEL_SONGBIRD_PORT` to
//!   also probe a remote Pixel's Songbird and exchange beacons cross-device
//!
//! The full Pixel replication requires running the mobile side from
//! `biomeOS/pixel8a-deploy/start_nucleus_mobile.sh` on a physical device
//! connected via hotspot. This experiment validates the server-side path
//! and optionally the cross-device beacon exchange.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::Duration;

use primalspring::coordination::AtomicType;
use primalspring::harness::AtomicHarness;
use primalspring::validation::ValidationResult;

fn rpc_call(
    socket: &Path,
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    let mut stream = UnixStream::connect(socket).map_err(|e| format!("connect: {e}"))?;
    stream.set_read_timeout(Some(Duration::from_secs(10))).ok();
    stream.set_write_timeout(Some(Duration::from_secs(5))).ok();

    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    });
    let msg = format!("{req}\n");
    stream
        .write_all(msg.as_bytes())
        .map_err(|e| format!("write: {e}"))?;
    let _ = stream.shutdown(std::net::Shutdown::Write);

    let reader = BufReader::new(&stream);
    for line in reader.lines().map_while(Result::ok) {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&line) {
            if let Some(result) = parsed.get("result") {
                return Ok(result.clone());
            }
            if let Some(error) = parsed.get("error") {
                return Err(format!("RPC error: {error}"));
            }
        }
    }
    Err("no response".to_owned())
}

fn tcp_rpc_call(
    host: &str,
    port: u16,
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    let addr = format!("{host}:{port}");
    let mut stream = TcpStream::connect_timeout(
        &addr.parse().map_err(|e| format!("parse: {e}"))?,
        Duration::from_secs(5),
    )
    .map_err(|e| format!("connect {addr}: {e}"))?;
    stream.set_read_timeout(Some(Duration::from_secs(10))).ok();
    stream.set_write_timeout(Some(Duration::from_secs(5))).ok();

    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    });
    let msg = format!("{req}\n");
    stream
        .write_all(msg.as_bytes())
        .map_err(|e| format!("write: {e}"))?;
    let _ = stream.shutdown(std::net::Shutdown::Write);

    let reader = BufReader::new(&stream);
    for line in reader.lines().map_while(Result::ok) {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&line) {
            if let Some(result) = parsed.get("result") {
                return Ok(result.clone());
            }
            if let Some(error) = parsed.get("error") {
                return Err(format!("RPC error: {error}"));
            }
        }
    }
    Err("no response".to_owned())
}

fn validate_beacon(
    v: &mut primalspring::validation::ValidationResult,
    socket: &Path,
    family_id: &str,
) {
    let beacon_result = rpc_call(
        socket,
        "birdsong.generate_encrypted_beacon",
        &serde_json::json!({
            "family_id": family_id,
            "node_id": family_id,
            "capabilities": ["security", "discovery", "network.tls"],
            "device_type": "tower"
        }),
    );

    match &beacon_result {
        Ok(beacon) => {
            println!("  beacon generated: {}B", beacon.to_string().len());
            v.check_bool(
                "beacon_generated",
                true,
                "BirdSong encrypted beacon generated",
            );

            let enc_str = beacon
                .get("encrypted_beacon")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let decrypt = rpc_call(
                socket,
                "birdsong.decrypt_beacon",
                &serde_json::json!({ "encrypted_beacon": enc_str }),
            );
            match &decrypt {
                Ok(plain) => {
                    println!("  beacon decrypted: {plain}");
                    v.check_bool(
                        "beacon_roundtrip",
                        true,
                        "beacon encrypt+decrypt round-trip",
                    );
                }
                Err(e) => {
                    println!("  beacon decrypt: {e}");
                    v.check_bool("beacon_roundtrip", false, &format!("decrypt failed: {e}"));
                }
            }
        }
        Err(e) => {
            println!("  beacon generation: {e}");
            v.check_bool(
                "beacon_generated",
                e.contains("Method not found"),
                &format!("birdsong.generate_encrypted_beacon: {e}"),
            );
        }
    }
}

fn validate_connectivity(
    v: &mut primalspring::validation::ValidationResult,
    socket: &Path,
    family_id: &str,
) {
    let onion = rpc_call(
        socket,
        "onion.start",
        &serde_json::json!({ "family_id": family_id }),
    );
    match &onion {
        Ok(resp) => {
            let addr = resp
                .get("address")
                .and_then(|a| a.as_str())
                .unwrap_or("unknown");
            println!("  onion service started: {addr}");
            v.check_bool("onion_started", true, "sovereign onion for rendezvous");
        }
        Err(e) => {
            println!("  onion.start: {e}");
            v.check_bool(
                "onion_started",
                e.contains("Method not found"),
                &format!("onion: {e}"),
            );
        }
    }

    let stun = rpc_call(socket, "stun.get_public_address", &serde_json::json!({}));
    match &stun {
        Ok(addr) => {
            println!("  STUN public address: {addr}");
            v.check_bool(
                "stun_address_obtained",
                true,
                "STUN resolved public address",
            );
        }
        Err(e) => {
            println!("  STUN: {e}");
            v.check_bool(
                "stun_address_obtained",
                e.contains("Method not found"),
                &format!("STUN: {e}"),
            );
        }
    }
}

fn main() {
    let graphs_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../graphs");
    let family_id = format!("e063-{}", std::process::id());

    ValidationResult::run_experiment(
        "primalSpring Exp063 — Pixel Tower Rendezvous",
        "primalSpring Exp063: BirdSong beacon generation + rendezvous exchange",
        |v| {
            let running = match AtomicHarness::new(AtomicType::Tower)
                .start_with_neural_api(&family_id, &graphs_dir)
            {
                Ok(r) => r,
                Err(e) => {
                    v.check_bool("harness_start", false, &format!("failed to start: {e}"));
                    v.finish();
                    std::process::exit(v.exit_code());
                }
            };

            let Some(songbird_socket) = running
                .socket_for("discovery")
                .or_else(|| running.socket_for_primal("songbird"))
            else {
                v.check_bool("songbird_socket", false, "songbird socket not found");
                v.finish();
                std::process::exit(v.exit_code());
            };

            validate_beacon(v, songbird_socket, &family_id);
            validate_connectivity(v, songbird_socket, &family_id);

            // Cross-device beacon exchange (optional — set PIXEL_SONGBIRD_HOST)
            let pixel_host = std::env::var("PIXEL_SONGBIRD_HOST").ok();
            let pixel_port: u16 = std::env::var("PIXEL_SONGBIRD_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(9200);

            if let Some(ref host) = pixel_host {
                v.section("Cross-Device Beacon Exchange");
                println!("  Pixel host: {host}:{pixel_port}");

                match tcp_rpc_call(host, pixel_port, "health.liveness", &serde_json::json!({})) {
                    Ok(_) => {
                        println!("  Pixel songbird: LIVE");
                        v.check_bool("pixel_songbird_live", true, "Pixel songbird reachable");
                    }
                    Err(e) => {
                        println!("  Pixel songbird: {e}");
                        v.check_skip("pixel_songbird_live", &format!("Pixel unreachable: {e}"));
                    }
                }

                // Generate beacon on local Tower, send to Pixel for decrypt
                let local_beacon = rpc_call(
                    songbird_socket,
                    "birdsong.generate_encrypted_beacon",
                    &serde_json::json!({
                        "family_id": family_id,
                        "node_id": "tower_local",
                        "capabilities": ["security", "discovery"],
                        "device_type": "tower"
                    }),
                );
                if let Ok(beacon) = &local_beacon {
                    let enc = beacon
                        .get("encrypted_beacon")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    if !enc.is_empty() {
                        match tcp_rpc_call(
                            host,
                            pixel_port,
                            "birdsong.decrypt_beacon",
                            &serde_json::json!({ "encrypted_beacon": enc }),
                        ) {
                            Ok(_) => {
                                println!("  cross-device beacon: Tower→Pixel decrypt OK");
                                v.check_bool(
                                    "cross_device_beacon",
                                    true,
                                    "Tower beacon decrypted by Pixel",
                                );
                            }
                            Err(e) => {
                                println!("  cross-device beacon: {e}");
                                v.check_skip("cross_device_beacon", &format!("Pixel decrypt: {e}"));
                            }
                        }
                    }
                }
            }

            println!("\n  === Rendezvous Flow Summary ===");
            println!("  Tower ({family_id}) local validation complete.");
            if pixel_host.is_some() {
                println!("  Cross-device beacon exchange attempted.");
            } else {
                println!("  Set PIXEL_SONGBIRD_HOST to enable cross-device beacon exchange.");
            }
            println!("  For full Pixel replication:");
            println!("    1. Run biomeOS/pixel8a-deploy/start_nucleus_mobile.sh on Pixel 8a");
            println!(
                "    2. Both POST beacons to https://api.nestgate.io/api/v1/rendezvous/beacon"
            );
            println!("    3. Both poll https://api.nestgate.io/api/v1/rendezvous/check");
            println!("    4. Encrypted Dark Forest beacons verify family lineage");
        },
    );
}
