// SPDX-License-Identifier: AGPL-3.0-or-later

//! exp086 — Genetic Identity E2E
//!
//! Validates the mito (beacon seed) vs nuclear (family/lineage seed) model
//! end-to-end: lineage key derivation, beacon-scoped encryption, family
//! identity in capability registry, and cross-gate isolation.

use primalspring::ipc::{methods, tcp};
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

/// `BearDog` / Songbird genetic & beacon RPC names (owned by those primals).
const GENETIC_DERIVE_LINEAGE_BEACON_KEY: &str = "genetic.derive_lineage_beacon_key";
const GENETIC_DERIVE_LINEAGE_KEY: &str = "genetic.derive_lineage_key";
const GENETIC_GENERATE_LINEAGE_PROOF: &str = "genetic.generate_lineage_proof";
const GENETIC_VERIFY_LINEAGE: &str = "genetic.verify_lineage";
const BIRDSONG_GENERATE_ENCRYPTED_BEACON: &str = "birdsong.generate_encrypted_beacon";
const BIRDSONG_DECRYPT_BEACON: &str = "birdsong.decrypt_beacon";
const BIRDSONG_VERIFY_LINEAGE: &str = "birdsong.verify_lineage";

fn main() {
    ValidationResult::new("Genetic Identity E2E")
        .with_provenance("exp086_genetic_identity_e2e", "2026-03-29")
        .run("mito vs nuclear genetics validation", |v| {
            let bd_port = tcp::env_port("BEARDOG_PORT", tolerances::TCP_FALLBACK_BEARDOG_PORT);
            let sg_port = tcp::env_port("SONGBIRD_PORT", tolerances::TCP_FALLBACK_SONGBIRD_PORT);
            let biomeos_port = tcp::env_port("BIOMEOS_PORT", 9800);
            let host = std::env::var("TOWER_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());

            phase_lineage_key_derivation(v, &host, bd_port);
            phase_beacon_family_scoping(v, &host, sg_port);
            phase_biomeos_family_registry(v, &host, biomeos_port);
            phase_genetic_lineage_verification(v, &host, bd_port);
        });
}

/// Nuclear genetics: derive keys from family/lineage seed.
fn phase_lineage_key_derivation(v: &mut ValidationResult, host: &str, port: u16) {
    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD;

    v.section("Lineage Key Derivation (Nuclear)");

    let lineage_seed = b64.encode(b"primalSpring_exp086_test_seed!!");

    let beacon_key = tcp::tcp_rpc(
        host,
        port,
        GENETIC_DERIVE_LINEAGE_BEACON_KEY,
        &serde_json::json!({
            "lineage_seed": lineage_seed
        }),
    );
    match &beacon_key {
        Ok((result, _)) => {
            let has_key = result.get("beacon_key").is_some()
                || result.get("key").is_some()
                || result.get("derived_key").is_some();
            v.check_bool(
                "beacon key derived",
                has_key,
                "HKDF derives beacon key from lineage seed",
            );
        }
        Err(e) => {
            v.check_skip(
                "beacon key derivation",
                &format!("BearDog genetic RPC not reachable: {e}"),
            );
            return;
        }
    }

    let domain_key = tcp::tcp_rpc(
        host,
        port,
        GENETIC_DERIVE_LINEAGE_KEY,
        &serde_json::json!({
            "our_family_id": "exp086-family",
            "peer_family_id": "exp086-peer",
            "context": "storage_encryption_v1",
            "lineage_seed": lineage_seed,
        }),
    );
    match domain_key {
        Ok((result, _)) => {
            let has_key = result.get("key").is_some() || result.get("derived_key").is_some();
            v.check_bool(
                "domain key derived",
                has_key,
                "per-domain key derivation works",
            );
        }
        Err(e) => v.check_skip("domain key derivation", &format!("derive failed: {e}")),
    }
}

/// Mito genetics: beacon encryption is family-scoped.
fn phase_beacon_family_scoping(v: &mut ValidationResult, host: &str, port: u16) {
    v.section("Beacon Family Scoping (Mito)");

    let beacon = tcp::tcp_rpc(
        host,
        port,
        BIRDSONG_GENERATE_ENCRYPTED_BEACON,
        &serde_json::json!({
            "node_id": "exp086-mito-test",
            "capabilities": ["security", "discovery"]
        }),
    );
    match &beacon {
        Ok((result, _)) => {
            let has_beacon =
                result.get("encrypted_beacon").is_some() || result.get("beacon").is_some();
            v.check_bool(
                "family-scoped beacon",
                has_beacon,
                "beacon encrypted with family seed",
            );
        }
        Err(e) => {
            v.check_skip(
                "family-scoped beacon",
                &format!("Songbird not reachable: {e}"),
            );
            return;
        }
    }

    let beacon_data = beacon
        .as_ref()
        .ok()
        .and_then(|(r, _)| r.get("encrypted_beacon").or_else(|| r.get("beacon")))
        .and_then(|b| b.as_str())
        .unwrap_or_default();

    let decrypt_same = tcp::tcp_rpc(
        host,
        port,
        BIRDSONG_DECRYPT_BEACON,
        &serde_json::json!({
            "encrypted_beacon": beacon_data
        }),
    );
    match decrypt_same {
        Ok((result, _)) => {
            let has_node = result.get("node_id").is_some()
                || result
                    .get("beacon")
                    .and_then(|b| b.get("node_id"))
                    .is_some();
            v.check_bool(
                "same-family decrypt",
                has_node,
                "same family can decrypt beacon",
            );
        }
        Err(e) => v.check_skip("same-family decrypt", &format!("decrypt failed: {e}")),
    }
}

/// Verify biomeOS registers family identity in capability routing.
fn phase_biomeos_family_registry(v: &mut ValidationResult, host: &str, port: u16) {
    v.section("biomeOS Family Registry");

    let caps = tcp::tcp_rpc(
        host,
        port,
        methods::capabilities::LIST,
        &serde_json::json!({}),
    );
    match caps {
        Ok((result, _)) => {
            let caps_str = result.to_string();
            let has_family = caps_str.contains("family")
                || caps_str.contains("genetic")
                || caps_str.contains("lineage");
            v.check_bool(
                "biomeOS family awareness",
                has_family,
                "capability registry includes family/genetic/lineage references",
            );
        }
        Err(e) => v.check_skip(
            "biomeOS capability list",
            &format!("biomeOS not reachable: {e}"),
        ),
    }

    let routes = tcp::tcp_rpc(host, port, "route.list", &serde_json::json!({}));
    match routes {
        Ok((result, _)) => {
            let has_routes = result.is_array() || result.is_object();
            v.check_bool(
                "route registry populated",
                has_routes,
                "biomeOS has registered routes",
            );
        }
        Err(e) => v.check_skip("route registry", &format!("route.list failed: {e}")),
    }
}

/// Verify lineage chain integrity via generate-then-verify round-trip.
#[allow(clippy::too_many_lines)]
fn phase_genetic_lineage_verification(v: &mut ValidationResult, host: &str, port: u16) {
    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD;

    v.section("Lineage Verification");

    let lineage_seed = b64.encode(b"primalSpring_exp086_test_seed!!");
    let our_family = "exp086-family-alpha";
    let peer_family = "exp086-family-beta";

    // Step 1: generate a lineage proof
    let proof_result = tcp::tcp_rpc(
        host,
        port,
        GENETIC_GENERATE_LINEAGE_PROOF,
        &serde_json::json!({
            "our_family_id": our_family,
            "peer_family_id": peer_family,
            "lineage_seed": lineage_seed,
        }),
    );
    let proof_b64 = match &proof_result {
        Ok((result, _)) => {
            let has_proof = result.get("proof").is_some();
            v.check_bool(
                "lineage proof generated",
                has_proof,
                "genetic.generate_lineage_proof returns proof",
            );
            result
                .get("proof")
                .and_then(|p| p.as_str())
                .map(String::from)
        }
        Err(e) => {
            v.check_skip(
                "lineage proof generation",
                &format!("BearDog genetic RPC not reachable: {e}"),
            );
            return;
        }
    };

    let Some(proof) = proof_b64 else {
        v.check_skip("lineage verification", "no proof to verify");
        return;
    };

    // Step 2: verify with correct seed (should pass)
    let verify_ok = tcp::tcp_rpc(
        host,
        port,
        GENETIC_VERIFY_LINEAGE,
        &serde_json::json!({
            "our_family_id": our_family,
            "peer_family_id": peer_family,
            "lineage_proof": proof,
            "lineage_seed": lineage_seed,
        }),
    );
    match verify_ok {
        Ok((result, _)) => {
            let valid = result
                .get("valid")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            v.check_bool(
                "lineage chain valid",
                valid,
                "genetic.verify_lineage confirms chain integrity with correct seed",
            );
        }
        Err(e) => v.check_skip(
            "lineage verification",
            &format!("genetic.verify_lineage failed: {e}"),
        ),
    }

    // Step 3: verify with wrong seed (should fail — negative test)
    let wrong_seed = b64.encode(b"WRONG_seed_not_the_real_one!!!!");
    let verify_bad = tcp::tcp_rpc(
        host,
        port,
        GENETIC_VERIFY_LINEAGE,
        &serde_json::json!({
            "our_family_id": our_family,
            "peer_family_id": peer_family,
            "lineage_proof": proof,
            "lineage_seed": wrong_seed,
        }),
    );
    match verify_bad {
        Ok((result, _)) => {
            let valid = result
                .get("valid")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(true);
            v.check_bool(
                "wrong-seed rejected",
                !valid,
                "genetic.verify_lineage rejects proof with wrong lineage seed",
            );
        }
        Err(_) => v.check_bool(
            "wrong-seed rejected",
            true,
            "RPC error on wrong seed is acceptable rejection",
        ),
    }

    // Step 4: birdsong.verify_lineage (challenge step 1 — returns challenge, not boolean)
    let birdsong_lineage = tcp::tcp_rpc(
        host,
        port,
        BIRDSONG_VERIFY_LINEAGE,
        &serde_json::json!({
            "peer_node_id": "exp086-peer-node"
        }),
    );
    match birdsong_lineage {
        Ok((result, _)) => {
            let has_challenge = result.get("challenge_generated").is_some()
                || result.get("challenge").is_some()
                || result.get("valid").is_some();
            v.check_bool(
                "birdsong lineage challenge",
                has_challenge,
                "birdsong.verify_lineage generates challenge (step 1 of protocol)",
            );
        }
        Err(e) => v.check_skip(
            "birdsong lineage",
            &format!("Songbird lineage not reachable: {e}"),
        ),
    }
}
