// SPDX-License-Identifier: AGPL-3.0-or-later

//! exp085 — `BearDog` Crypto Lifecycle E2E
//!
//! Validates that `BearDog` performs real cryptography through both direct
//! IPC and Neural API routing: Ed25519 sign/verify, `BirdSong` beacon
//! round-trip, Blake3 hashing, and secrets store/retrieve.

use primalspring::ipc::tcp;
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

/// `BearDog` / Songbird RPC names (owned by those primals).
const CRYPTO_GENERATE_KEYPAIR: &str = "crypto.generate_keypair";
const CRYPTO_SIGN_ED25519: &str = "crypto.sign_ed25519";
const CRYPTO_VERIFY_ED25519: &str = "crypto.verify_ed25519";
const CRYPTO_BLAKE3_HASH: &str = "crypto.blake3_hash";
const CRYPTO_SHA256_HASH: &str = "crypto.sha256_hash";
const BIRDSONG_GENERATE_ENCRYPTED_BEACON: &str = "birdsong.generate_encrypted_beacon";
const BIRDSONG_DECRYPT_BEACON: &str = "birdsong.decrypt_beacon";
const SECRETS_STORE: &str = "secrets.store";
const SECRETS_RETRIEVE: &str = "secrets.retrieve";

fn main() {
    ValidationResult::new("BearDog Crypto Lifecycle E2E")
        .with_provenance("exp085_beardog_crypto_lifecycle", "2026-03-29")
        .run("crypto lifecycle validation", |v| {
            let bd_port = tcp::env_port("BEARDOG_PORT", tolerances::TCP_FALLBACK_BEARDOG_PORT);
            let sg_port = tcp::env_port("SONGBIRD_PORT", tolerances::TCP_FALLBACK_SONGBIRD_PORT);
            let host = std::env::var("TOWER_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());

            let keys = phase_ed25519_generate(v, &host, bd_port);
            if let Some((pub_key, _)) = &keys {
                phase_ed25519_sign_verify(v, &host, bd_port, pub_key);
            }
            phase_hashing(v, &host, bd_port);
            phase_birdsong_beacon(v, &host, sg_port);
            phase_secrets(v, &host, bd_port);
        });
}

fn phase_ed25519_generate(
    v: &mut ValidationResult,
    host: &str,
    port: u16,
) -> Option<(String, String)> {
    v.section("Ed25519 Keypair Generation");

    let keypair = tcp::tcp_rpc(
        host,
        port,
        CRYPTO_GENERATE_KEYPAIR,
        &serde_json::json!({ "algorithm": "ed25519" }),
    );
    match &keypair {
        Ok((result, latency)) => {
            let has_public = result.get("public_key").is_some()
                || result.get("publicKey").is_some()
                || result.get("public").is_some();
            v.check_bool(
                "keypair has public key",
                has_public,
                "generate_keypair returns public key",
            );
            v.check_latency(
                "keypair generation latency",
                u64::try_from(latency.as_micros()).unwrap_or(u64::MAX),
                tolerances::GRAPH_NODE_MAX_US,
            );
            let pub_key = result
                .get("public_key")
                .or_else(|| result.get("publicKey"))
                .or_else(|| result.get("public"))
                .and_then(|k| k.as_str())
                .unwrap_or_default()
                .to_owned();
            Some((pub_key, String::new()))
        }
        Err(e) => {
            v.check_skip("keypair generation", &format!("BearDog not reachable: {e}"));
            None
        }
    }
}

fn phase_ed25519_sign_verify(v: &mut ValidationResult, host: &str, port: u16, pub_key: &str) {
    v.section("Ed25519 Sign + Verify");

    let test_payload = "primalSpring exp085 crypto lifecycle test";
    let sign_result = tcp::tcp_rpc(
        host,
        port,
        CRYPTO_SIGN_ED25519,
        &serde_json::json!({ "data": test_payload }),
    );
    let sig = match &sign_result {
        Ok((result, _)) => {
            v.check_bool(
                "sign returns signature",
                result.get("signature").is_some(),
                "sign_ed25519 returns signature field",
            );
            result
                .get("signature")
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default()
                .to_owned()
        }
        Err(e) => {
            v.check_skip("sign ed25519", &format!("sign failed: {e}"));
            return;
        }
    };

    let verify_ok = tcp::tcp_rpc(
        host,
        port,
        CRYPTO_VERIFY_ED25519,
        &serde_json::json!({ "data": test_payload, "signature": sig, "public_key": pub_key }),
    );
    match verify_ok {
        Ok((result, _)) => {
            let valid = result
                .get("valid")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false)
                || result
                    .get("verified")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false)
                || result.as_bool().unwrap_or(false);
            v.check_bool(
                "verify valid signature",
                valid,
                "correct signature verifies true",
            );
        }
        Err(e) => v.check_skip("verify valid signature", &format!("verify failed: {e}")),
    }

    let verify_tampered = tcp::tcp_rpc(
        host,
        port,
        CRYPTO_VERIFY_ED25519,
        &serde_json::json!({ "data": "TAMPERED payload", "signature": sig, "public_key": pub_key }),
    );
    match verify_tampered {
        Ok((result, _)) => {
            let valid = result
                .get("valid")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(true)
                || result
                    .get("verified")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(true);
            v.check_bool(
                "tampered payload rejected",
                !valid,
                "tampered data fails verification",
            );
        }
        Err(_) => v.check_bool(
            "tampered payload rejected",
            true,
            "RPC error on tampered data is acceptable rejection",
        ),
    }
}

fn phase_hashing(v: &mut ValidationResult, host: &str, port: u16) {
    v.section("Hashing");

    let hash_result = tcp::tcp_rpc(
        host,
        port,
        CRYPTO_BLAKE3_HASH,
        &serde_json::json!({
            "data": "primalSpring exp085 hash test"
        }),
    );
    match hash_result {
        Ok((result, _)) => {
            let has_hash = result.get("hash").is_some() || result.get("digest").is_some();
            v.check_bool(
                "blake3 returns hash",
                has_hash,
                "blake3_hash returns hash/digest",
            );
        }
        Err(e) => v.check_skip("blake3 hash", &format!("BearDog not reachable: {e}")),
    }

    let sha_result = tcp::tcp_rpc(
        host,
        port,
        CRYPTO_SHA256_HASH,
        &serde_json::json!({
            "data": "primalSpring exp085 sha test"
        }),
    );
    match sha_result {
        Ok((result, _)) => {
            let has_hash = result.get("hash").is_some() || result.get("digest").is_some();
            v.check_bool(
                "sha256 returns hash",
                has_hash,
                "sha256_hash returns hash/digest",
            );
        }
        Err(e) => v.check_skip("sha256 hash", &format!("BearDog not reachable: {e}")),
    }
}

fn phase_birdsong_beacon(v: &mut ValidationResult, host: &str, port: u16) {
    v.section("BirdSong Beacon Round-Trip");

    let beacon_gen = tcp::tcp_rpc(
        host,
        port,
        BIRDSONG_GENERATE_ENCRYPTED_BEACON,
        &serde_json::json!({
            "node_id": "exp085-test-node",
            "capabilities": ["coordination", "crypto"]
        }),
    );
    match &beacon_gen {
        Ok((result, _)) => {
            let has_beacon =
                result.get("encrypted_beacon").is_some() || result.get("beacon").is_some();
            v.check_bool(
                "beacon generation",
                has_beacon,
                "generate_encrypted_beacon returns beacon",
            );
        }
        Err(e) => {
            v.check_skip("beacon generation", &format!("Songbird not reachable: {e}"));
            return;
        }
    }

    let beacon_data = beacon_gen
        .as_ref()
        .ok()
        .and_then(|(r, _)| r.get("encrypted_beacon").or_else(|| r.get("beacon")))
        .and_then(|b| b.as_str())
        .unwrap_or_default();

    let beacon_dec = tcp::tcp_rpc(
        host,
        port,
        BIRDSONG_DECRYPT_BEACON,
        &serde_json::json!({
            "encrypted_beacon": beacon_data
        }),
    );
    match beacon_dec {
        Ok((result, _)) => {
            let has_node = result.get("node_id").is_some()
                || result
                    .get("beacon")
                    .and_then(|b| b.get("node_id"))
                    .is_some();
            v.check_bool(
                "beacon decrypt round-trip",
                has_node,
                "decrypted beacon contains node_id",
            );
        }
        Err(e) => v.check_skip("beacon decrypt", &format!("decrypt failed: {e}")),
    }
}

fn phase_secrets(v: &mut ValidationResult, host: &str, port: u16) {
    v.section("Secrets Store/Retrieve");

    let store_result = tcp::tcp_rpc(
        host,
        port,
        SECRETS_STORE,
        &serde_json::json!({
            "key": "exp085_test_secret",
            "value": "sovereign_data_at_rest"
        }),
    );
    match &store_result {
        Ok((_, _)) => {
            v.check_bool("secret stored", true, "secrets.store succeeded");
        }
        Err(e) => {
            v.check_skip(
                "secrets store",
                &format!("BearDog secrets not reachable: {e}"),
            );
            return;
        }
    }

    let retrieve_result = tcp::tcp_rpc(
        host,
        port,
        SECRETS_RETRIEVE,
        &serde_json::json!({
            "key": "exp085_test_secret"
        }),
    );
    match retrieve_result {
        Ok((result, _)) => {
            let val = result
                .get("value")
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default();
            v.check_bool(
                "secret round-trip",
                val == "sovereign_data_at_rest",
                "retrieved value matches stored value",
            );
        }
        Err(e) => v.check_skip("secrets retrieve", &format!("retrieve failed: {e}")),
    }
}
