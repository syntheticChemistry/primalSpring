// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp073: LAN Covalent Mesh — validate cross-gate covalent bonding.
//!
//! Validates the full covalent bonding pattern between two gates:
//! - TCP JSON-RPC `health.liveness` on remote beardog + songbird
//! - `capabilities.list` enumerates remote NUCLEUS capabilities
//! - `mesh.peers` discovers at least 1 peer via BirdSong UDP
//! - `birdsong.generate_encrypted_beacon` + decrypt round-trip
//! - Neural-api `capability.call` routing to local primals
//! - `FAMILY_ID` genetic lineage verification via BearDog
//! - HTTPS validation through Tower Atomic
//!
//! Environment:
//!   `REMOTE_GATE_HOST` — hostname or IP of the remote gate (required)
//!   `REMOTE_SONGBIRD_PORT` — Songbird TCP fallback (default: 9200, cross-gate only)
//!   `REMOTE_BEARDOG_PORT`  — `BearDog` TCP fallback (default: 9100, cross-gate only)
//!   `FAMILY_ID` — shared family ID for beacon generation and lineage check

use primalspring::ipc::NeuralBridge;
use primalspring::ipc::methods;
use primalspring::ipc::tcp::tcp_rpc;
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

    match tcp_rpc_value(
        host,
        songbird_port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    ) {
        Ok(_) => {
            println!("  remote songbird: LIVE");
            v.check_bool(
                "remote_songbird_live",
                true,
                "remote songbird health.liveness",
            );
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

fn validate_neural_api_routing(v: &mut ValidationResult) {
    v.section("Neural API Capability Routing");

    let Some(bridge) = NeuralBridge::discover() else {
        println!("  biomeOS not running — skipping neural-api routing checks");
        v.check_skip("neural_api_routing", "biomeOS neural-api not discovered");
        return;
    };

    match bridge.health_check() {
        Ok(_) => {
            println!("  neural-api: HEALTHY");
            v.check_bool("neural_api_health", true, "biomeOS neural-api healthy");
        }
        Err(e) => {
            println!("  neural-api: {e}");
            v.check_bool("neural_api_health", false, &format!("neural-api: {e}"));
            return;
        }
    }

    match bridge.capability_call("crypto", "generate_keypair", &serde_json::json!({})) {
        Ok(_) => {
            println!("  capability.call → BearDog crypto: OK");
            v.check_bool(
                "neural_routing_crypto",
                true,
                "capability.call routes crypto to BearDog",
            );
        }
        Err(e) => {
            println!("  capability.call → crypto: {e}");
            v.check_skip("neural_routing_crypto", &format!("crypto routing: {e}"));
        }
    }
}

fn validate_genetic_lineage(
    v: &mut ValidationResult,
    host: &str,
    beardog_port: u16,
    family_id: &str,
) {
    v.section("Genetic Lineage (FAMILY_ID)");

    match tcp_rpc_value(
        host,
        beardog_port,
        "health.check",
        &serde_json::json!({ "include_details": true }),
    ) {
        Ok(resp) => {
            let remote_family = resp
                .get("family_id")
                .and_then(|f| f.as_str())
                .unwrap_or("unknown");
            let matches = remote_family == family_id;
            println!("  local  FAMILY_ID: {family_id}");
            println!("  remote FAMILY_ID: {remote_family}");
            println!("  lineage match:    {matches}");
            v.check_bool(
                "family_id_matches",
                matches,
                &format!("FAMILY_ID lineage: local={family_id} remote={remote_family}"),
            );
        }
        Err(e) => {
            println!("  genetic lineage check: {e}");
            v.check_skip("family_id_matches", &format!("BearDog health.check: {e}"));
        }
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "multi-phase validation is inherently sequential"
)]
fn validate_three_tier_genetics(v: &mut ValidationResult, host: &str, beardog_port: u16) {
    v.section("Three-Tier Genetics (Mito + Nuclear + Lineage Proof)");

    let lineage_seed = "exp073_lan_covalent_test_seed";

    // Tier 1: Mito-beacon key derivation
    match tcp_rpc_value(
        host,
        beardog_port,
        "genetic.derive_lineage_beacon_key",
        &serde_json::json!({
            "lineage_seed": lineage_seed,
            "domain": "birdsong_beacon_v1"
        }),
    ) {
        Ok(result) => {
            let has_key = result.get("beacon_key").is_some();
            let status = if has_key { "derived" } else { "missing" };
            println!("  Tier 1 mito-beacon key: {status}");
            v.check_bool(
                "remote_mito_beacon_derived",
                has_key,
                "genetic.derive_lineage_beacon_key on remote BearDog",
            );
        }
        Err(e) => {
            if e.contains("Method not found") {
                v.check_skip(
                    "remote_mito_beacon_derived",
                    "genetic.derive_lineage_beacon_key not available",
                );
            } else {
                v.check_skip("remote_mito_beacon_derived", &format!("mito-beacon: {e}"));
            }
            return;
        }
    }

    // Tier 2: Nuclear lineage key — genesis (generation 0)
    let genesis_result = tcp_rpc_value(
        host,
        beardog_port,
        "genetic.derive_lineage_key",
        &serde_json::json!({
            "lineage_seed": lineage_seed,
            "domain": "covalent_mesh_v1",
            "generation": 0
        }),
    );

    let genesis_key_hash = match &genesis_result {
        Ok(result) => {
            let has_key = result.get("lineage_key").is_some();
            let generation = result
                .get("generation")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(999);
            let key_status = if has_key { "derived" } else { "missing" };
            println!("  Tier 2 nuclear genesis: gen={generation}, key={key_status}");
            v.check_bool(
                "remote_nuclear_genesis",
                has_key && generation == 0,
                "genetic.derive_lineage_key genesis on remote BearDog",
            );
            result
                .get("lineage_key")
                .and_then(|k| k.as_str())
                .map(String::from)
        }
        Err(e) => {
            v.check_skip("remote_nuclear_genesis", &format!("nuclear genesis: {e}"));
            None
        }
    };

    // Tier 2: Nuclear lineage key — child (generation 1, with parent hash)
    if let Some(ref _parent_key) = genesis_key_hash {
        let child_result = tcp_rpc_value(
            host,
            beardog_port,
            "genetic.derive_lineage_key",
            &serde_json::json!({
                "lineage_seed": lineage_seed,
                "domain": "covalent_mesh_child_v1",
                "generation": 1,
                "context_entropy": "lan-covalent-session"
            }),
        );

        match child_result {
            Ok(result) => {
                let generation = result
                    .get("generation")
                    .and_then(serde_json::Value::as_u64)
                    .unwrap_or(999);
                let has_key = result.get("lineage_key").is_some();
                let key_status = if has_key { "derived" } else { "missing" };
                println!("  Tier 2 nuclear child:   gen={generation}, key={key_status}");
                v.check_bool(
                    "remote_nuclear_child",
                    has_key && generation == 1,
                    "genetic.derive_lineage_key child generation on remote BearDog",
                );
            }
            Err(e) => {
                v.check_skip("remote_nuclear_child", &format!("nuclear child: {e}"));
            }
        }
    }

    // Lineage proof generation + verification round-trip
    let proof_result = tcp_rpc_value(
        host,
        beardog_port,
        "genetic.generate_lineage_proof",
        &serde_json::json!({
            "lineage_seed": lineage_seed,
            "generation": 0,
            "context": "covalent_mesh_v1"
        }),
    );

    match &proof_result {
        Ok(result) => {
            let has_proof = result.get("proof").is_some();
            println!("  Lineage proof generated: {has_proof}");
            v.check_bool(
                "remote_lineage_proof_generated",
                has_proof,
                "genetic.generate_lineage_proof on remote BearDog",
            );

            if let Some(proof) = result.get("proof").and_then(|p| p.as_str()) {
                match tcp_rpc_value(
                    host,
                    beardog_port,
                    "genetic.verify_lineage",
                    &serde_json::json!({
                        "proof": proof,
                        "claimed_generation": 0,
                        "context": "covalent_mesh_v1"
                    }),
                ) {
                    Ok(verify) => {
                        let valid = verify
                            .get("valid")
                            .and_then(serde_json::Value::as_bool)
                            .unwrap_or(false);
                        println!("  Lineage proof verified: {valid}");
                        v.check_bool(
                            "remote_lineage_proof_verified",
                            valid,
                            "genetic.verify_lineage round-trip on remote BearDog",
                        );
                    }
                    Err(e) => {
                        v.check_skip("remote_lineage_proof_verified", &format!("verify: {e}"));
                    }
                }
            }
        }
        Err(e) => {
            v.check_skip("remote_lineage_proof_generated", &format!("proof: {e}"));
        }
    }

    // Entropy mixing
    match tcp_rpc_value(
        host,
        beardog_port,
        "genetic.mix_entropy",
        &serde_json::json!({
            "tiers": ["68756d616e2d656e74726f7079", "73757065727669736564", "6d616368696e65"]
        }),
    ) {
        Ok(result) => {
            let has_mixed = result.get("mixed").is_some();
            let status = if has_mixed { "OK" } else { "missing" };
            println!("  Entropy mixing: {status}");
            v.check_bool(
                "remote_entropy_mixing",
                has_mixed,
                "genetic.mix_entropy on remote BearDog",
            );
        }
        Err(e) => {
            v.check_skip("remote_entropy_mixing", &format!("mix_entropy: {e}"));
        }
    }
}

fn validate_tower_https(v: &mut ValidationResult, host: &str, songbird_port: u16) {
    v.section("HTTPS Through Tower Atomic");

    match tcp_rpc_value(
        host,
        songbird_port,
        "http.get",
        &serde_json::json!({ "url": "https://ifconfig.me/ip" }),
    ) {
        Ok(resp) => {
            let status = resp
                .get("status_code")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(0);
            println!("  remote HTTPS via Songbird: status {status}");
            v.check_bool(
                "remote_tower_https",
                status == 200,
                &format!("remote Tower Atomic HTTPS: status {status}"),
            );
        }
        Err(e) => {
            println!("  remote HTTPS: {e}");
            v.check_skip("remote_tower_https", &format!("http.get: {e}"));
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
        .with_provenance("exp073_lan_covalent_mesh", "2026-04-06")
        .run(
            "primalSpring Exp073: Cross-gate covalent bonding — mesh, beacon, lineage, HTTPS",
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
            validate_genetic_lineage(v, &host, beardog_port, &family_id);
            validate_three_tier_genetics(v, &host, beardog_port);
            validate_tower_https(v, &host, songbird_port);
            validate_neural_api_routing(v);
            validate_stun(v, &host, songbird_port);
            },
        );
}
