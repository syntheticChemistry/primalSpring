// SPDX-License-Identifier: AGPL-3.0-or-later

//! Phases 1-2: Cross-Architecture Tower Health + Three-Tier Genetics.

use crate::config::{pixel_beardog_port, pixel_host, pixel_nestgate_port, pixel_songbird_port, tcp_rpc_value};
use primalspring::ipc::methods;
use primalspring::validation::ValidationResult;

pub fn validate_pixel_tower_health(v: &mut ValidationResult) {
    v.section("Phase 1: Pixel Tower Health (aarch64)");

    let host = pixel_host();
    let bd_port = pixel_beardog_port();
    let sb_port = pixel_songbird_port();
    let ng_port = pixel_nestgate_port();

    println!("  target: {host} (BearDog:{bd_port} Songbird:{sb_port} NestGate:{ng_port})");

    let bd_health = tcp_rpc_value(
        &host,
        bd_port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    );
    let bd_ok = bd_health.is_ok();
    v.check_bool(
        "pixel_beardog_alive",
        bd_ok,
        &format!("BearDog at {host}:{bd_port}"),
    );

    if let Ok(ref result) = bd_health {
        if let Some(arch) = result.get("arch").and_then(serde_json::Value::as_str) {
            println!("  pixel BearDog arch: {arch}");
            v.check_bool(
                "pixel_beardog_aarch64",
                arch.contains("aarch64") || arch.contains("arm"),
                &format!("expected aarch64, got {arch}"),
            );
        } else {
            println!("  pixel BearDog: arch not reported in health response");
        }
    }

    let sb_ok = tcp_rpc_value(
        &host,
        sb_port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    )
    .is_ok();
    v.check_bool(
        "pixel_songbird_alive",
        sb_ok,
        &format!("Songbird at {host}:{sb_port}"),
    );

    let ng_result = tcp_rpc_value(
        &host,
        ng_port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    );
    match &ng_result {
        Ok(_) => v.check_bool(
            "pixel_nestgate_alive",
            true,
            &format!("NestGate at {host}:{ng_port}"),
        ),
        Err(e) => v.check_skip("pixel_nestgate_alive", &format!("NestGate: {e}")),
    }

    let bd_caps = tcp_rpc_value(&host, bd_port, "capabilities.list", &serde_json::json!({}));
    match &bd_caps {
        Ok(result) => {
            let cap_count = result
                .get("methods")
                .and_then(|m| m.as_array())
                .map(Vec::len)
                .or_else(|| result.as_array().map(Vec::len))
                .or_else(|| {
                    result
                        .get("capabilities")
                        .and_then(|c| c.as_array())
                        .map(Vec::len)
                })
                .unwrap_or(0);
            println!("  pixel BearDog capabilities: {cap_count} methods");
            v.check_bool(
                "pixel_beardog_capabilities",
                cap_count > 0,
                &format!("{cap_count} methods via capabilities.list"),
            );

            let has_transport_security = result.get("transport_security").is_some();
            v.check_bool(
                "pixel_btsp_detection",
                has_transport_security,
                "BearDog reports transport_security (BTSP programmatic detection)",
            );
        }
        Err(e) => v.check_skip("pixel_beardog_capabilities", &format!("capabilities: {e}")),
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "multi-phase validation is inherently sequential"
)]
pub fn validate_cross_arch_genetics(v: &mut ValidationResult) {
    v.section("Phase 2: Three-Tier Genetics (x86_64 → aarch64)");

    let host = pixel_host();
    let bd_port = pixel_beardog_port();
    let lineage_seed_b64 = "ZXhwMDk2X3BpeGVsX2Nyb3NzX2FyY2hfdGVzdA==";
    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "pixel-cross-arch-lab".into());

    let beacon_result = tcp_rpc_value(
        &host,
        bd_port,
        "genetic.derive_lineage_beacon_key",
        &serde_json::json!({ "lineage_seed": lineage_seed_b64 }),
    );
    match &beacon_result {
        Ok(result) => {
            let has_key = result.get("beacon_key").is_some();
            println!(
                "  Tier 1 mito-beacon: {}",
                if has_key { "derived" } else { "missing key" }
            );
            v.check_bool(
                "pixel_mito_beacon_derive",
                has_key,
                "genetic.derive_lineage_beacon_key on Pixel BearDog (aarch64)",
            );
        }
        Err(e) => {
            if e.is_method_not_found() {
                v.check_skip(
                    "pixel_mito_beacon_derive",
                    "genetic.* RPCs not available on Pixel BearDog",
                );
                println!("  Tier 1: skipped (genetic.* RPCs not available)");
                return;
            }
            v.check_bool(
                "pixel_mito_beacon_derive",
                false,
                &format!("mito-beacon: {e}"),
            );
        }
    }

    let genesis_result = tcp_rpc_value(
        &host,
        bd_port,
        "genetic.derive_lineage_key",
        &serde_json::json!({
            "lineage_seed": lineage_seed_b64,
            "our_family_id": family_id,
            "peer_family_id": family_id,
            "context": "exp096_nuclear_genesis"
        }),
    );

    let genesis_key = match &genesis_result {
        Ok(result) => {
            let has_key = result.get("key").is_some();
            let method = result
                .get("method")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown");
            println!(
                "  Tier 2 nuclear genesis: method={method}, key={}",
                if has_key { "derived" } else { "missing" }
            );
            v.check_bool(
                "pixel_nuclear_genesis",
                has_key,
                "genetic.derive_lineage_key genesis on Pixel BearDog (aarch64)",
            );
            result
                .get("key")
                .and_then(serde_json::Value::as_str)
                .map(String::from)
        }
        Err(e) => {
            v.check_skip("pixel_nuclear_genesis", &format!("nuclear genesis: {e}"));
            None
        }
    };

    if let Some(ref parent_key) = genesis_key {
        let child_result = tcp_rpc_value(
            &host,
            bd_port,
            "genetic.derive_lineage_key",
            &serde_json::json!({
                "lineage_seed": lineage_seed_b64,
                "our_family_id": family_id,
                "peer_family_id": family_id,
                "context": "exp096_nuclear_child_gen1"
            }),
        );
        match &child_result {
            Ok(result) => {
                let has_key = result.get("key").is_some();
                let child_key = result
                    .get("key")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("");
                let keys_differ = child_key != parent_key.as_str();
                println!("  Tier 2 nuclear child: distinct={keys_differ}");
                v.check_bool(
                    "pixel_nuclear_child",
                    has_key && keys_differ,
                    "nuclear child (different context) is distinct from genesis on Pixel (aarch64)",
                );
            }
            Err(e) => v.check_skip("pixel_nuclear_child", &format!("nuclear child: {e}")),
        }
    } else {
        v.check_skip("pixel_nuclear_child", "no genesis key for child derivation");
    }

    if genesis_key.is_some() {
        let proof_result = tcp_rpc_value(
            &host,
            bd_port,
            "genetic.generate_lineage_proof",
            &serde_json::json!({
                "lineage_seed": lineage_seed_b64,
                "our_family_id": family_id,
                "peer_family_id": family_id
            }),
        );
        match &proof_result {
            Ok(result) => {
                let has_proof = result.get("proof").is_some();
                println!(
                    "  lineage proof: {}",
                    if has_proof { "generated" } else { "missing" }
                );
                v.check_bool(
                    "pixel_lineage_proof_gen",
                    has_proof,
                    "genetic.generate_lineage_proof on Pixel BearDog (aarch64)",
                );

                if let Some(proof) = result.get("proof") {
                    let verify_result = tcp_rpc_value(
                        &host,
                        bd_port,
                        "genetic.verify_lineage",
                        &serde_json::json!({
                            "lineage_seed": lineage_seed_b64,
                            "our_family_id": family_id,
                            "peer_family_id": family_id,
                            "lineage_proof": proof
                        }),
                    );
                    match &verify_result {
                        Ok(result) => {
                            let valid = result
                                .get("valid")
                                .and_then(serde_json::Value::as_bool)
                                .unwrap_or(false);
                            println!("  lineage verify: {valid}");
                            v.check_bool(
                                "pixel_lineage_proof_verify",
                                valid,
                                "genetic.verify_lineage on Pixel BearDog (aarch64)",
                            );
                        }
                        Err(e) => {
                            v.check_skip("pixel_lineage_proof_verify", &format!("verify: {e}"));
                        }
                    }
                }
            }
            Err(e) => {
                v.check_skip("pixel_lineage_proof_gen", &format!("proof gen: {e}"));
                v.check_skip("pixel_lineage_proof_verify", "no proof to verify");
            }
        }
    }

    let mix_result = tcp_rpc_value(
        &host,
        bd_port,
        "genetic.mix_entropy",
        &serde_json::json!({
            "tier3_human": lineage_seed_b64,
            "tier1_machine": "bWFjaGluZS1lbnRyb3B5LWV4cDA5Ng=="
        }),
    );
    match &mix_result {
        Ok(result) => {
            let has_mixed =
                result.get("mixed_entropy").is_some() || result.get("entropy").is_some();
            println!(
                "  entropy mixing: {}",
                if has_mixed { "ok" } else { "missing" }
            );
            v.check_bool(
                "pixel_entropy_mix",
                has_mixed,
                "genetic.mix_entropy on Pixel BearDog (aarch64)",
            );
        }
        Err(e) => v.check_skip("pixel_entropy_mix", &format!("entropy mix: {e}")),
    }
}
