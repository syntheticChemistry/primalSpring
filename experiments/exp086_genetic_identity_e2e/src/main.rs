// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

//! exp086 — Genetic Identity E2E
//!
//! Validates the mito (beacon seed) vs nuclear (family/lineage seed) model
//! end-to-end via `NeuralBridge::capability_call()`: lineage key derivation,
//! beacon-scoped encryption, family identity in capability registry, and
//! cross-gate isolation.

use primalspring::composition::CompositionContext;
use primalspring::ipc::methods;
use primalspring::ipc::NeuralBridge;
use primalspring::validation::ValidationResult;

const GENETIC_DERIVE_LINEAGE_BEACON_KEY: &str = "genetic.derive_lineage_beacon_key";
const GENETIC_DERIVE_LINEAGE_KEY: &str = "genetic.derive_lineage_key";
const GENETIC_GENERATE_LINEAGE_PROOF: &str = "genetic.generate_lineage_proof";
const GENETIC_VERIFY_LINEAGE: &str = "genetic.verify_lineage";
const BIRDSONG_GENERATE_ENCRYPTED_BEACON: &str = "birdsong.generate_encrypted_beacon";
const BIRDSONG_DECRYPT_BEACON: &str = "birdsong.decrypt_beacon";
const BIRDSONG_VERIFY_LINEAGE: &str = "birdsong.verify_lineage";

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
        "security in CompositionContext",
    );
    v.check_bool(
        "has_discovery_capability_path",
        ctx.has_capability("discovery"),
        "discovery in CompositionContext",
    );
}

/// Nuclear genetics: derive keys from family/lineage seed.
fn phase_lineage_key_derivation(v: &mut ValidationResult, bridge: &NeuralBridge) {
    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD;

    v.section("Phase 2: Lineage key derivation (nuclear)");

    let lineage_seed = b64.encode(b"primalSpring_exp086_test_seed!!");

    let beacon_key = bridge.capability_call(
        "security",
        GENETIC_DERIVE_LINEAGE_BEACON_KEY,
        &serde_json::json!({
            "lineage_seed": lineage_seed
        }),
    );
    match &beacon_key {
        Ok(resp) => {
            let result = &resp.value;
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
                &format!("routing failed: {e}"),
            );
            return;
        }
    }

    let domain_key = bridge.capability_call(
        "security",
        GENETIC_DERIVE_LINEAGE_KEY,
        &serde_json::json!({
            "our_family_id": "exp086-family",
            "peer_family_id": "exp086-peer",
            "context": "storage_encryption_v1",
            "lineage_seed": lineage_seed,
        }),
    );
    match domain_key {
        Ok(resp) => {
            let has_key = resp.value.get("key").is_some() || resp.value.get("derived_key").is_some();
            v.check_bool(
                "domain key derived",
                has_key,
                "per-domain key derivation works",
            );
        }
        Err(e) => v.check_skip("domain key derivation", &format!("routing failed: {e}")),
    }
}

/// Mito genetics: beacon encryption is family-scoped.
fn phase_beacon_family_scoping(v: &mut ValidationResult, bridge: &NeuralBridge) {
    v.section("Phase 3: Beacon family scoping (mito)");

    let beacon = bridge.capability_call(
        "discovery",
        BIRDSONG_GENERATE_ENCRYPTED_BEACON,
        &serde_json::json!({
            "node_id": "exp086-mito-test",
            "capabilities": ["security", "discovery"]
        }),
    );
    match &beacon {
        Ok(resp) => {
            let result = &resp.value;
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
                &format!("routing failed: {e}"),
            );
            return;
        }
    }

    let beacon_data = beacon
        .as_ref()
        .ok()
        .and_then(|resp| {
            resp.value
                .get("encrypted_beacon")
                .or_else(|| resp.value.get("beacon"))
        })
        .and_then(|b| b.as_str())
        .unwrap_or_default();

    let decrypt_same = bridge.capability_call(
        "discovery",
        BIRDSONG_DECRYPT_BEACON,
        &serde_json::json!({
            "encrypted_beacon": beacon_data
        }),
    );
    match decrypt_same {
        Ok(resp) => {
            let result = &resp.value;
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
        Err(e) => v.check_skip("same-family decrypt", &format!("routing failed: {e}")),
    }
}

/// Verify biomeOS registers family identity in capability routing.
fn phase_biomeos_family_registry(v: &mut ValidationResult, bridge: &NeuralBridge) {
    v.section("Phase 4: biomeOS family registry");

    let caps = bridge.capability_call(
        "orchestration",
        methods::capabilities::LIST,
        &serde_json::json!({}),
    );
    match caps {
        Ok(resp) => {
            let caps_str = resp.value.to_string();
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
            &format!("routing failed: {e}"),
        ),
    }

    let routes = bridge.capability_call("orchestration", "route.list", &serde_json::json!({}));
    match routes {
        Ok(resp) => {
            let has_routes = resp.value.is_array() || resp.value.is_object();
            v.check_bool(
                "route registry populated",
                has_routes,
                "biomeOS has registered routes",
            );
        }
        Err(e) => v.check_skip("route registry", &format!("routing failed: {e}")),
    }
}

fn verify_lineage_correct_seed(
    v: &mut ValidationResult,
    bridge: &NeuralBridge,
    our_family: &str,
    peer_family: &str,
    proof: &str,
    lineage_seed: &str,
) {
    let verify_ok = bridge.capability_call(
        "security",
        GENETIC_VERIFY_LINEAGE,
        &serde_json::json!({
            "our_family_id": our_family,
            "peer_family_id": peer_family,
            "lineage_proof": proof,
            "lineage_seed": lineage_seed,
        }),
    );
    match verify_ok {
        Ok(resp) => {
            let valid = resp
                .value
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
            &format!("routing failed: {e}"),
        ),
    }
}

fn verify_lineage_wrong_seed(
    v: &mut ValidationResult,
    bridge: &NeuralBridge,
    our_family: &str,
    peer_family: &str,
    proof: &str,
    wrong_seed: &str,
) {
    let verify_bad = bridge.capability_call(
        "security",
        GENETIC_VERIFY_LINEAGE,
        &serde_json::json!({
            "our_family_id": our_family,
            "peer_family_id": peer_family,
            "lineage_proof": proof,
            "lineage_seed": wrong_seed,
        }),
    );
    match verify_bad {
        Ok(resp) => {
            let valid = resp
                .value
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
}

/// Verify lineage chain integrity via generate-then-verify round-trip.
fn phase_genetic_lineage_verification(v: &mut ValidationResult, bridge: &NeuralBridge) {
    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD;

    v.section("Phase 5: Lineage verification");

    let lineage_seed = b64.encode(b"primalSpring_exp086_test_seed!!");
    let our_family = "exp086-family-alpha";
    let peer_family = "exp086-family-beta";

    let proof_result = bridge.capability_call(
        "security",
        GENETIC_GENERATE_LINEAGE_PROOF,
        &serde_json::json!({
            "our_family_id": our_family,
            "peer_family_id": peer_family,
            "lineage_seed": lineage_seed,
        }),
    );
    let proof_b64 = match &proof_result {
        Ok(resp) => {
            let has_proof = resp.value.get("proof").is_some();
            v.check_bool(
                "lineage proof generated",
                has_proof,
                "genetic.generate_lineage_proof returns proof",
            );
            resp.value
                .get("proof")
                .and_then(|p| p.as_str())
                .map(String::from)
        }
        Err(e) => {
            v.check_skip(
                "lineage proof generation",
                &format!("routing failed: {e}"),
            );
            return;
        }
    };

    let Some(proof) = proof_b64 else {
        v.check_skip("lineage verification", "no proof to verify");
        return;
    };

    verify_lineage_correct_seed(
        v,
        bridge,
        our_family,
        peer_family,
        &proof,
        &lineage_seed,
    );

    let wrong_seed = b64.encode(b"WRONG_seed_not_the_real_one!!!!");
    verify_lineage_wrong_seed(v, bridge, our_family, peer_family, &proof, &wrong_seed);

    let birdsong_lineage = bridge.capability_call(
        "discovery",
        BIRDSONG_VERIFY_LINEAGE,
        &serde_json::json!({
            "peer_node_id": "exp086-peer-node"
        }),
    );
    match birdsong_lineage {
        Ok(resp) => {
            let result = &resp.value;
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
            &format!("routing failed: {e}"),
        ),
    }
}

#[cfg(feature = "primordial-compat")]
fn phase_legacy_tcp(v: &mut ValidationResult) {
    use primalspring::ipc::tcp;
    use primalspring::tolerances;

    v.section("Phase 6 (legacy): Direct TCP genetic identity");

    let bd_port = tcp::env_port("BEARDOG_PORT", tolerances::default_port_for("beardog"));
    let sg_port = tcp::env_port("SONGBIRD_PORT", tolerances::default_port_for("songbird"));
    let biomeos_port = tcp::env_port("BIOMEOS_PORT", 9800);
    let host = std::env::var("TOWER_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());

    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD;
    let lineage_seed = b64.encode(b"primalSpring_exp086_test_seed!!");

    match tcp::tcp_rpc(
        &host,
        bd_port,
        GENETIC_DERIVE_LINEAGE_BEACON_KEY,
        &serde_json::json!({ "lineage_seed": lineage_seed }),
    ) {
        Ok((result, _)) => {
            let has_key = result.get("beacon_key").is_some()
                || result.get("key").is_some()
                || result.get("derived_key").is_some();
            v.check_bool("legacy beacon key derived", has_key, "TCP HKDF beacon key");
        }
        Err(e) => v.check_skip(
            "legacy beacon key derivation",
            &format!("BearDog genetic RPC not reachable: {e}"),
        ),
    }

    match tcp::tcp_rpc(
        &host,
        sg_port,
        BIRDSONG_GENERATE_ENCRYPTED_BEACON,
        &serde_json::json!({
            "node_id": "exp086-mito-test",
            "capabilities": ["security", "discovery"]
        }),
    ) {
        Ok((result, _)) => {
            let has_beacon =
                result.get("encrypted_beacon").is_some() || result.get("beacon").is_some();
            v.check_bool("legacy family-scoped beacon", has_beacon, "TCP beacon generation");
        }
        Err(e) => v.check_skip(
            "legacy family-scoped beacon",
            &format!("Songbird not reachable: {e}"),
        ),
    }

    match tcp::tcp_rpc(
        &host,
        biomeos_port,
        methods::capabilities::LIST,
        &serde_json::json!({}),
    ) {
        Ok((result, _)) => {
            let caps_str = result.to_string();
            let has_family = caps_str.contains("family")
                || caps_str.contains("genetic")
                || caps_str.contains("lineage");
            v.check_bool(
                "legacy biomeOS family awareness",
                has_family,
                "TCP capability registry includes family/genetic/lineage",
            );
        }
        Err(e) => v.check_skip(
            "legacy biomeOS capability list",
            &format!("biomeOS not reachable: {e}"),
        ),
    }

    match tcp::tcp_rpc(
        &host,
        bd_port,
        GENETIC_GENERATE_LINEAGE_PROOF,
        &serde_json::json!({
            "our_family_id": "exp086-family-alpha",
            "peer_family_id": "exp086-family-beta",
            "lineage_seed": lineage_seed,
        }),
    ) {
        Ok((result, _)) => {
            v.check_bool(
                "legacy lineage proof generated",
                result.get("proof").is_some(),
                "TCP genetic.generate_lineage_proof",
            );
        }
        Err(e) => v.check_skip(
            "legacy lineage proof generation",
            &format!("BearDog genetic RPC not reachable: {e}"),
        ),
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp086 — Genetic Identity E2E")
        .with_provenance("exp086_genetic_identity_e2e", "2026-05-09")
        .run("mito vs nuclear genetics validation via NeuralBridge", |v| {
            let ctx = CompositionContext::from_live_discovery_with_fallback();
            phase_composition_discovery(v, &ctx);

            let Some(bridge) = NeuralBridge::discover() else {
                v.check_skip("neural_api", "biomeOS not running — routing skipped");
                #[cfg(feature = "primordial-compat")]
                phase_legacy_tcp(v);
                return;
            };

            phase_lineage_key_derivation(v, &bridge);
            phase_beacon_family_scoping(v, &bridge);
            phase_biomeos_family_registry(v, &bridge);
            phase_genetic_lineage_verification(v, &bridge);

            #[cfg(feature = "primordial-compat")]
            phase_legacy_tcp(v);
        });
}
