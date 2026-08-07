// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

//! exp085 — `BearDog` Crypto Lifecycle E2E
//!
//! Validates that `BearDog` performs real cryptography through Neural API
//! routing via `NeuralBridge::capability_call()`: Ed25519 sign/verify,
//! `BirdSong` beacon round-trip, Blake3 hashing, and secrets store/retrieve.

use primalspring::composition::CompositionContext;
use primalspring::ipc::NeuralBridge;
use primalspring::validation::ValidationResult;

const CRYPTO_GENERATE_KEYPAIR: &str = "crypto.generate_keypair";
const CRYPTO_SIGN_ED25519: &str = "crypto.sign_ed25519";
const CRYPTO_VERIFY_ED25519: &str = "crypto.verify_ed25519";
const CRYPTO_BLAKE3_HASH: &str = "crypto.blake3_hash";
const CRYPTO_SHA256_HASH: &str = "crypto.sha256_hash";
const BIRDSONG_GENERATE_ENCRYPTED_BEACON: &str = "birdsong.generate_encrypted_beacon";
const BIRDSONG_DECRYPT_BEACON: &str = "birdsong.decrypt_beacon";
const SECRETS_STORE: &str = "secrets.store";
const SECRETS_RETRIEVE: &str = "secrets.retrieve";

fn phase_composition_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    v.section("Phase 1: Composition discovery");
    let caps = ctx.available_capabilities();
    v.check_bool(
        "composition_capabilities_non_empty",
        !caps.is_empty(),
        &format!("{} capabilities: {}", caps.len(), caps.join(", ")),
    );
    v.check_bool(
        "has_security_capability_path",
        ctx.has_capability("security"),
        "security capability for BearDog via NeuralBridge",
    );
}

fn phase_ed25519_generate(
    v: &mut ValidationResult,
    bridge: &NeuralBridge,
) -> Option<(String, String)> {
    v.section("Phase 2: Ed25519 keypair generation");

    let start = std::time::Instant::now();
    let keypair = bridge.capability_call(
        "security",
        CRYPTO_GENERATE_KEYPAIR,
        &serde_json::json!({ "algorithm": "ed25519" }),
    );
    let latency_us = u64::try_from(start.elapsed().as_micros()).unwrap_or(u64::MAX);
    match &keypair {
        Ok(resp) => {
            let result = &resp.value;
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
                latency_us,
                primalspring::tolerances::GRAPH_NODE_MAX_US,
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
            v.check_skip("keypair generation", &format!("routing failed: {e}"));
            None
        }
    }
}

fn phase_ed25519_sign_verify(v: &mut ValidationResult, bridge: &NeuralBridge, pub_key: &str) {
    v.section("Phase 3: Ed25519 sign + verify");

    let test_payload = "primalSpring exp085 crypto lifecycle test";
    let sign_result = bridge.capability_call(
        "security",
        CRYPTO_SIGN_ED25519,
        &serde_json::json!({ "data": test_payload }),
    );
    let sig = match &sign_result {
        Ok(resp) => {
            let result = &resp.value;
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
            v.check_skip("sign ed25519", &format!("routing failed: {e}"));
            return;
        }
    };

    let verify_ok = bridge.capability_call(
        "security",
        CRYPTO_VERIFY_ED25519,
        &serde_json::json!({ "data": test_payload, "signature": sig, "public_key": pub_key }),
    );
    match verify_ok {
        Ok(resp) => {
            let result = &resp.value;
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
        Err(e) => v.check_skip("verify valid signature", &format!("routing failed: {e}")),
    }

    let verify_tampered = bridge.capability_call(
        "security",
        CRYPTO_VERIFY_ED25519,
        &serde_json::json!({ "data": "TAMPERED payload", "signature": sig, "public_key": pub_key }),
    );
    match verify_tampered {
        Ok(resp) => {
            let result = &resp.value;
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

fn phase_hashing(v: &mut ValidationResult, bridge: &NeuralBridge) {
    v.section("Phase 4: Hashing");

    let hash_result = bridge.capability_call(
        "security",
        CRYPTO_BLAKE3_HASH,
        &serde_json::json!({
            "data": "primalSpring exp085 hash test"
        }),
    );
    match hash_result {
        Ok(resp) => {
            let has_hash = resp.value.get("hash").is_some() || resp.value.get("digest").is_some();
            v.check_bool(
                "blake3 returns hash",
                has_hash,
                "blake3_hash returns hash/digest",
            );
        }
        Err(e) => v.check_skip("blake3 hash", &format!("routing failed: {e}")),
    }

    let sha_result = bridge.capability_call(
        "security",
        CRYPTO_SHA256_HASH,
        &serde_json::json!({
            "data": "primalSpring exp085 sha test"
        }),
    );
    match sha_result {
        Ok(resp) => {
            let has_hash = resp.value.get("hash").is_some() || resp.value.get("digest").is_some();
            v.check_bool(
                "sha256 returns hash",
                has_hash,
                "sha256_hash returns hash/digest",
            );
        }
        Err(e) => v.check_skip("sha256 hash", &format!("routing failed: {e}")),
    }
}

fn phase_birdsong_beacon(v: &mut ValidationResult, bridge: &NeuralBridge) {
    v.section("Phase 5: BirdSong beacon round-trip");

    let beacon_gen = bridge.capability_call(
        "discovery",
        BIRDSONG_GENERATE_ENCRYPTED_BEACON,
        &serde_json::json!({
            "node_id": "exp085-test-node",
            "capabilities": ["coordination", "crypto"]
        }),
    );
    match &beacon_gen {
        Ok(resp) => {
            let result = &resp.value;
            let has_beacon =
                result.get("encrypted_beacon").is_some() || result.get("beacon").is_some();
            v.check_bool(
                "beacon generation",
                has_beacon,
                "generate_encrypted_beacon returns beacon",
            );
        }
        Err(e) => {
            v.check_skip("beacon generation", &format!("routing failed: {e}"));
            return;
        }
    }

    let beacon_data = beacon_gen
        .as_ref()
        .ok()
        .and_then(|resp| {
            resp.value
                .get("encrypted_beacon")
                .or_else(|| resp.value.get("beacon"))
        })
        .and_then(|b| b.as_str())
        .unwrap_or_default();

    let beacon_dec = bridge.capability_call(
        "discovery",
        BIRDSONG_DECRYPT_BEACON,
        &serde_json::json!({
            "encrypted_beacon": beacon_data
        }),
    );
    match beacon_dec {
        Ok(resp) => {
            let result = &resp.value;
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
        Err(e) => v.check_skip("beacon decrypt", &format!("routing failed: {e}")),
    }
}

fn phase_secrets(v: &mut ValidationResult, bridge: &NeuralBridge) {
    v.section("Phase 6: Secrets store/retrieve");

    let store_result = bridge.capability_call(
        "security",
        SECRETS_STORE,
        &serde_json::json!({
            "key": "exp085_test_secret",
            "value": "sovereign_data_at_rest"
        }),
    );
    match &store_result {
        Ok(_) => {
            v.check_bool("secret stored", true, "secrets.store succeeded");
        }
        Err(e) => {
            v.check_skip(
                "secrets store",
                &format!("routing failed: {e}"),
            );
            return;
        }
    }

    let retrieve_result = bridge.capability_call(
        "security",
        SECRETS_RETRIEVE,
        &serde_json::json!({
            "key": "exp085_test_secret"
        }),
    );
    match retrieve_result {
        Ok(resp) => {
            let val = resp
                .value
                .get("value")
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default();
            v.check_bool(
                "secret round-trip",
                val == "sovereign_data_at_rest",
                "retrieved value matches stored value",
            );
        }
        Err(e) => v.check_skip("secrets retrieve", &format!("routing failed: {e}")),
    }
}

#[cfg(feature = "primordial-compat")]
fn phase_legacy_tcp(v: &mut ValidationResult) {
    use primalspring::ipc::tcp;
    use primalspring::tolerances;

    v.section("Phase 7 (legacy): Direct TCP crypto lifecycle");

    let bd_port = tcp::env_port("BEARDOG_PORT", tolerances::default_port_for("beardog"));
    let sg_port = tcp::env_port("SONGBIRD_PORT", tolerances::default_port_for("songbird"));
    let host = std::env::var("TOWER_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());

    let keypair = tcp::tcp_rpc(
        &host,
        bd_port,
        CRYPTO_GENERATE_KEYPAIR,
        &serde_json::json!({ "algorithm": "ed25519" }),
    );
    let pub_key = match &keypair {
        Ok((result, _)) => {
            let has_public = result.get("public_key").is_some()
                || result.get("publicKey").is_some()
                || result.get("public").is_some();
            v.check_bool(
                "legacy keypair has public key",
                has_public,
                "TCP generate_keypair returns public key",
            );
            result
                .get("public_key")
                .or_else(|| result.get("publicKey"))
                .or_else(|| result.get("public"))
                .and_then(|k| k.as_str())
                .unwrap_or_default()
                .to_owned()
        }
        Err(e) => {
            v.check_skip("legacy keypair generation", &format!("BearDog not reachable: {e}"));
            String::new()
        }
    };

    if !pub_key.is_empty() {
        let test_payload = "primalSpring exp085 crypto lifecycle test";
        if let Ok((result, _)) = tcp::tcp_rpc(
            &host,
            bd_port,
            CRYPTO_SIGN_ED25519,
            &serde_json::json!({ "data": test_payload }),
        ) {
            if let Some(sig) = result.get("signature").and_then(serde_json::Value::as_str) {
                if let Ok((verify, _)) = tcp::tcp_rpc(
                    &host,
                    bd_port,
                    CRYPTO_VERIFY_ED25519,
                    &serde_json::json!({ "data": test_payload, "signature": sig, "public_key": pub_key }),
                ) {
                    let valid = verify
                        .get("valid")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(false);
                    v.check_bool("legacy verify valid signature", valid, "TCP sign+verify");
                }
            }
        }
    }

    match tcp::tcp_rpc(
        &host,
        bd_port,
        CRYPTO_BLAKE3_HASH,
        &serde_json::json!({ "data": "primalSpring exp085 hash test" }),
    ) {
        Ok((result, _)) => {
            let has_hash = result.get("hash").is_some() || result.get("digest").is_some();
            v.check_bool("legacy blake3 returns hash", has_hash, "TCP blake3_hash");
        }
        Err(e) => v.check_skip("legacy blake3 hash", &format!("BearDog not reachable: {e}")),
    }

    match tcp::tcp_rpc(
        &host,
        sg_port,
        BIRDSONG_GENERATE_ENCRYPTED_BEACON,
        &serde_json::json!({
            "node_id": "exp085-test-node",
            "capabilities": ["coordination", "crypto"]
        }),
    ) {
        Ok((result, _)) => {
            let has_beacon =
                result.get("encrypted_beacon").is_some() || result.get("beacon").is_some();
            v.check_bool("legacy beacon generation", has_beacon, "TCP generate_encrypted_beacon");
        }
        Err(e) => v.check_skip("legacy beacon generation", &format!("Songbird not reachable: {e}")),
    }

    match tcp::tcp_rpc(
        &host,
        bd_port,
        SECRETS_STORE,
        &serde_json::json!({
            "key": "exp085_test_secret",
            "value": "sovereign_data_at_rest"
        }),
    ) {
        Ok(_) => v.check_bool("legacy secret stored", true, "TCP secrets.store succeeded"),
        Err(e) => v.check_skip("legacy secrets store", &format!("BearDog secrets not reachable: {e}")),
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp085 — BearDog Crypto Lifecycle E2E")
        .with_provenance("exp085_beardog_crypto_lifecycle", "2026-05-09")
        .run("crypto lifecycle validation via NeuralBridge", |v| {
            let ctx = CompositionContext::from_live_discovery_with_fallback();
            phase_composition_discovery(v, &ctx);

            let Some(bridge) = NeuralBridge::discover() else {
                v.check_skip("neural_api", "biomeOS not running — routing skipped");
                #[cfg(feature = "primordial-compat")]
                phase_legacy_tcp(v);
                return;
            };

            let keys = phase_ed25519_generate(v, &bridge);
            if let Some((pub_key, _)) = &keys {
                phase_ed25519_sign_verify(v, &bridge, pub_key);
            }
            phase_hashing(v, &bridge);
            phase_birdsong_beacon(v, &bridge);
            phase_secrets(v, &bridge);

            #[cfg(feature = "primordial-compat")]
            phase_legacy_tcp(v);
        });
}
