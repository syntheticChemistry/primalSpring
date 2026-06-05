// SPDX-License-Identifier: AGPL-3.0-or-later

//! Phases 3-4: BTSP Phase 3 Readiness + HSM Probing.

use crate::config::{pixel_beardog_port, pixel_host, tcp_rpc_value};
use primalspring::validation::ValidationResult;

pub fn validate_btsp_phase3_readiness(v: &mut ValidationResult) {
    v.section("Phase 3: BTSP Phase 3 Cipher Readiness (aarch64)");

    let host = pixel_host();
    let bd_port = pixel_beardog_port();

    let crypto_caps = tcp_rpc_value(&host, bd_port, "capabilities.list", &serde_json::json!({}));

    let (has_chacha, has_hmac, has_encrypt) =
        crypto_caps
            .as_ref()
            .map_or((false, false, false), |result| {
                let caps_str = serde_json::to_string(result).unwrap_or_default();
                let chacha = caps_str.contains("chacha20") || caps_str.contains("ChaCha20");
                let hmac = caps_str.contains("hmac") || caps_str.contains("HMAC");
                let encrypt = caps_str.contains("encrypt") || caps_str.contains("cipher");
                (chacha, hmac, encrypt)
            });

    v.check_bool(
        "pixel_chacha20_poly1305_cap",
        has_chacha,
        "Pixel BearDog advertises ChaCha20-Poly1305 capability",
    );
    v.check_bool(
        "pixel_hmac_cap",
        has_hmac,
        "Pixel BearDog advertises HMAC capability",
    );

    let encrypt_result = tcp_rpc_value(
        &host,
        bd_port,
        "crypto.encrypt_chacha20_poly1305",
        &serde_json::json!({
            "plaintext": "cross-arch-phase3-test",
            "key_id": "session_test"
        }),
    );
    match &encrypt_result {
        Ok(result) => {
            let has_ciphertext = result.get("ciphertext").is_some();
            println!(
                "  ChaCha20-Poly1305 encrypt: {}",
                if has_ciphertext {
                    "ok"
                } else {
                    "no ciphertext"
                }
            );
            v.check_bool(
                "pixel_chacha20_encrypt",
                has_ciphertext,
                "crypto.encrypt_chacha20_poly1305 on Pixel BearDog (aarch64)",
            );
        }
        Err(e) => v.check_skip("pixel_chacha20_encrypt", &format!("encrypt: {e}")),
    }

    let hash_result = tcp_rpc_value(
        &host,
        bd_port,
        "crypto.hash",
        &serde_json::json!({
            "data": "Y3Jvc3MtYXJjaC1oYXNoLXRlc3Q=",
            "algorithm": "blake3"
        }),
    );
    match &hash_result {
        Ok(result) => {
            let has_hash = result.get("hash").is_some();
            println!("  BLAKE3 hash: {}", if has_hash { "ok" } else { "missing" });
            v.check_bool(
                "pixel_blake3_hash",
                has_hash,
                "crypto.hash (BLAKE3) on Pixel BearDog (aarch64) — content integrity primitive",
            );
        }
        Err(e) => v.check_skip("pixel_blake3_hash", &format!("hash: {e}")),
    }

    if !has_encrypt {
        println!("  NOTE: BearDog does not advertise encrypt/cipher capabilities");
        println!("  BTSP Phase 3 requires upstream BearDog + Songbird cipher negotiation");
    }
}

pub fn validate_hsm_capabilities(v: &mut ValidationResult) {
    v.section("Phase 4: HSM / Hardware Security Probing (Titan M2)");

    let host = pixel_host();
    let bd_port = pixel_beardog_port();

    let keypair = tcp_rpc_value(
        &host,
        bd_port,
        "crypto.generate_keypair",
        &serde_json::json!({}),
    );
    match &keypair {
        Ok(result) => {
            let has_pubkey = result.get("public_key").is_some();
            let backend = result
                .get("backend")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("software");
            println!("  keypair gen: backend={backend}");
            v.check_bool(
                "pixel_keypair_gen",
                has_pubkey,
                &format!("crypto.generate_keypair on Pixel (backend: {backend})"),
            );

            let is_hw = backend.contains("hardware")
                || backend.contains("hsm")
                || backend.contains("titan")
                || backend.contains("strongbox")
                || backend.contains("keymaster");
            if is_hw {
                println!("  HSM-BACKED key generation detected!");
            } else {
                println!("  software-only key generation (HSM integration not yet wired)");
            }
            v.check_bool(
                "pixel_hsm_backend",
                is_hw,
                "hardware-backed key generation (Titan M2 / StrongBox)",
            );
        }
        Err(e) => v.check_skip("pixel_keypair_gen", &format!("keypair: {e}")),
    }

    let sign_result = tcp_rpc_value(
        &host,
        bd_port,
        "crypto.sign",
        &serde_json::json!({
            "data": "Y3Jvc3MtYXJjaC1zaWduLXRlc3Q=",
            "algorithm": "ed25519"
        }),
    );
    match &sign_result {
        Ok(result) => {
            let has_sig = result.get("signature").is_some();
            println!(
                "  Ed25519 sign: {}",
                if has_sig { "ok" } else { "no signature" }
            );
            v.check_bool(
                "pixel_ed25519_sign",
                has_sig,
                "crypto.sign (Ed25519) on Pixel BearDog (aarch64)",
            );

            if let (Some(sig), Some(pubkey)) = (
                result.get("signature"),
                result
                    .get("public_key")
                    .or_else(|| keypair.as_ref().ok().and_then(|k| k.get("public_key"))),
            ) {
                let verify_result = tcp_rpc_value(
                    &host,
                    bd_port,
                    "crypto.verify",
                    &serde_json::json!({
                        "data": "Y3Jvc3MtYXJjaC1zaWduLXRlc3Q=",
                        "signature": sig,
                        "public_key": pubkey,
                        "algorithm": "ed25519"
                    }),
                );
                match &verify_result {
                    Ok(result) => {
                        let valid = result
                            .get("valid")
                            .and_then(serde_json::Value::as_bool)
                            .unwrap_or(false);
                        println!("  Ed25519 verify: {valid}");
                        v.check_bool(
                            "pixel_ed25519_verify",
                            valid,
                            "crypto.verify round-trip on Pixel BearDog (aarch64)",
                        );
                    }
                    Err(e) => v.check_skip("pixel_ed25519_verify", &format!("verify: {e}")),
                }
            }
        }
        Err(e) => v.check_skip("pixel_ed25519_sign", &format!("sign: {e}")),
    }
}
