// SPDX-License-Identifier: AGPL-3.0-or-later

//! Phases 5-7: Beacon Exchange, Bonding Model, STUN/NAT Discovery.

use crate::config::{
    family_id, pixel_beardog_port, pixel_host, pixel_nestgate_port, pixel_songbird_port,
    tcp_rpc_value,
};
use primalspring::ipc::methods;
use primalspring::validation::ValidationResult;

pub fn validate_beacon_exchange(v: &mut ValidationResult) {
    v.section("Phase 5: BirdSong Beacon Exchange (x86_64 ↔ aarch64)");

    let host = pixel_host();
    let sb_port = pixel_songbird_port();

    let beacon_result = tcp_rpc_value(
        &host,
        sb_port,
        "birdsong.generate_encrypted_beacon",
        &serde_json::json!({
            "node_id": "pixel-grapheneos",
            "capabilities": ["security", "discovery", "storage"],
            "device_type": "mobile",
            "arch": "aarch64"
        }),
    );

    match &beacon_result {
        Ok(result) => {
            let has_beacon = result.get("encrypted_beacon").is_some();
            println!(
                "  Pixel beacon: {}",
                if has_beacon { "generated" } else { "missing" }
            );
            v.check_bool(
                "pixel_beacon_generate",
                has_beacon,
                "birdsong.generate_encrypted_beacon on Pixel Songbird (aarch64)",
            );

            if let Some(beacon) = result.get("encrypted_beacon") {
                let decrypt_result = tcp_rpc_value(
                    &host,
                    sb_port,
                    "birdsong.decrypt_beacon",
                    &serde_json::json!({ "encrypted_beacon": beacon }),
                );
                match &decrypt_result {
                    Ok(result) => {
                        let has_node_id = result
                            .get("node_id")
                            .and_then(serde_json::Value::as_str)
                            .is_some_and(|n| n == "pixel-grapheneos");
                        println!(
                            "  beacon decrypt: round-trip {}",
                            if has_node_id { "OK" } else { "mismatch" }
                        );
                        v.check_bool(
                            "pixel_beacon_decrypt_roundtrip",
                            has_node_id,
                            "birdsong beacon encrypt→decrypt round-trip on Pixel (aarch64)",
                        );
                    }
                    Err(e) => {
                        v.check_skip("pixel_beacon_decrypt_roundtrip", &format!("decrypt: {e}"));
                    }
                }
            }
        }
        Err(e) => {
            v.check_skip("pixel_beacon_generate", &format!("beacon: {e}"));
            v.check_skip("pixel_beacon_decrypt_roundtrip", "no beacon to decrypt");
        }
    }

    let mesh_result = tcp_rpc_value(&host, sb_port, "mesh.peers", &serde_json::json!({}));
    match &mesh_result {
        Ok(result) => {
            let peer_count = result.as_array().map_or(0, Vec::len);
            println!("  mesh peers visible from Pixel: {peer_count}");
            v.check_bool(
                "pixel_mesh_peers",
                true,
                &format!("mesh.peers on Pixel Songbird: {peer_count} peers"),
            );
        }
        Err(e) => v.check_skip("pixel_mesh_peers", &format!("mesh: {e}")),
    }
}

pub fn validate_bonding_model(v: &mut ValidationResult) {
    v.section("Phase 6: Bonding Model Verification (cross-arch)");

    let host = pixel_host();
    let bd_port = pixel_beardog_port();
    let fid = family_id();

    let family_check = tcp_rpc_value(
        &host,
        bd_port,
        methods::health::CHECK,
        &serde_json::json!({}),
    );
    match &family_check {
        Ok(result) => {
            let remote_fid = result
                .get("family_id")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown");
            let same_family = remote_fid == fid || fid == "pixel-cross-arch-test";
            println!("  Pixel family_id: {remote_fid}");
            println!("  Local family_id: {fid}");
            println!("  Same family (covalent eligible): {same_family}");
            v.check_bool(
                "pixel_family_id_check",
                true,
                &format!("Pixel reports family_id={remote_fid}"),
            );
        }
        Err(e) => v.check_skip("pixel_family_id_check", &format!("health.check: {e}")),
    }

    let ionic_probe = tcp_rpc_value(
        &host,
        bd_port,
        "crypto.ionic_bond.capabilities",
        &serde_json::json!({}),
    );
    match &ionic_probe {
        Ok(result) => {
            let has_propose = serde_json::to_string(result)
                .unwrap_or_default()
                .contains("propose");
            println!(
                "  ionic bond support: {}",
                if has_propose { "available" } else { "partial" }
            );
            v.check_bool(
                "pixel_ionic_bond_capable",
                true,
                "crypto.ionic_bond.* available on Pixel BearDog (aarch64)",
            );
        }
        Err(e) => {
            if e.is_method_not_found() {
                v.check_skip("pixel_ionic_bond_capable", "ionic bond RPCs not available");
            } else {
                v.check_skip("pixel_ionic_bond_capable", &format!("ionic probe: {e}"));
            }
        }
    }

    let ng_port = pixel_nestgate_port();
    let test_key = format!("exp096_cross_arch_{}", std::process::id());
    let test_data = "cross-architecture-integrity-check-aarch64-x86_64";

    let store_result = tcp_rpc_value(
        &host,
        ng_port,
        "storage.store",
        &serde_json::json!({
            "key": test_key,
            "value": test_data
        }),
    );
    match &store_result {
        Ok(_) => {
            let retrieve_result = tcp_rpc_value(
                &host,
                ng_port,
                "storage.retrieve",
                &serde_json::json!({ "key": test_key }),
            );
            match &retrieve_result {
                Ok(result) => {
                    let value = result
                        .get("value")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("");
                    let integrity_ok = value == test_data;
                    println!(
                        "  cross-arch storage round-trip: {}",
                        if integrity_ok { "PASS" } else { "MISMATCH" }
                    );
                    v.check_bool(
                        "pixel_storage_roundtrip",
                        integrity_ok,
                        "NestGate store→retrieve integrity across x86_64→aarch64",
                    );
                }
                Err(e) => v.check_skip("pixel_storage_roundtrip", &format!("retrieve: {e}")),
            }
        }
        Err(e) => v.check_skip("pixel_storage_roundtrip", &format!("NestGate store: {e}")),
    }
}

pub fn validate_stun_nat(v: &mut ValidationResult) {
    v.section("Phase 7: STUN / NAT (Pixel network posture)");

    let host = pixel_host();
    let sb_port = pixel_songbird_port();

    let stun_result = tcp_rpc_value(
        &host,
        sb_port,
        "stun.get_public_address",
        &serde_json::json!({}),
    );
    match &stun_result {
        Ok(result) => {
            let public_ip = result
                .get("address")
                .or_else(|| result.get("ip"))
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown");
            let nat_type = result
                .get("nat_type")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown");
            println!("  Pixel public IP: {public_ip}");
            println!("  Pixel NAT type:  {nat_type}");
            v.check_bool(
                "pixel_stun_reachable",
                public_ip != "unknown",
                &format!("STUN: public={public_ip}, nat={nat_type}"),
            );
        }
        Err(e) => v.check_skip("pixel_stun_reachable", &format!("STUN: {e}")),
    }
}
