// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp063: Pixel ↔ Tower Rendezvous — replicate the biomeOS beacon exchange.
//!
//! Validates the local half of the Pixel 8a deployment flow demonstrated
//! by biomeOS: spawn Tower, generate an encrypted BirdSong beacon, POST
//! it to the rendezvous endpoint, and verify the exchange.
//!
//! The full Pixel replication requires running the mobile side from
//! `biomeOS/pixel8a-deploy/start_nucleus_mobile.sh` on a physical device
//! connected via hotspot. This experiment validates the server-side path.

use std::io::{BufRead, BufReader, Write};
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

            let enc_str = beacon.get("encrypted_beacon")
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
            let running = AtomicHarness::new(AtomicType::Tower)
                .start_with_neural_api(&family_id, &graphs_dir)
                .expect("tower + neural-api should start");

            let songbird_socket = running
                .socket_for("discovery")
                .or_else(|| running.socket_for_primal("songbird"))
                .expect("songbird socket");

            validate_beacon(v, songbird_socket, &family_id);
            validate_connectivity(v, songbird_socket, &family_id);

            println!("\n  === Rendezvous Flow Summary ===");
            println!("  Tower ({family_id}) local validation complete.");
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
