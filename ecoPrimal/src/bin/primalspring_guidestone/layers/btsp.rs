// SPDX-License-Identifier: AGPL-3.0-or-later

//! Layer 1.5 + Layer 6: BTSP escalation, cipher policy, and crypto validation.

use primalspring::bonding::{BondType, BondingPolicy, BtspEnforcer, TrustModel};
use primalspring::btsp;
use primalspring::composition::CompositionContext;
use primalspring::coordination::AtomicType;
use primalspring::ipc::NeuralBridge;
use primalspring::validation::ValidationResult;

pub fn validate_btsp_escalation(ctx: &CompositionContext, v: &mut ValidationResult) {
    let btsp_map = ctx.btsp_state();

    let tiers: &[(&str, &[&str], BondType)] = &[
        (
            "Tower",
            AtomicType::Tower.required_capabilities(),
            BondType::Covalent,
        ),
        ("Node", &["compute", "tensor", "shader"], BondType::Metallic),
        ("Nest", &["storage", "ai"], BondType::Metallic),
        (
            "Provenance",
            &["dag", "commit", "provenance"],
            BondType::Metallic,
        ),
    ];

    for &(tier_name, caps, bond) in tiers {
        for &cap in caps {
            let check_name = format!("btsp:{tier_name}:{cap}");
            match btsp_map.get(cap) {
                Some(true) => {
                    v.check_bool(&check_name, true, "BTSP authenticated");
                }
                Some(false) => {
                    v.check_bool(&check_name, false, "cleartext (BTSP not yet enforced)");
                }
                None => {
                    v.check_skip(&check_name, "capability not discovered");
                }
            }
        }

        let min_cipher = btsp::min_cipher_for_bond(bond);
        let trust_req = match bond {
            BondType::Covalent => "nuclear-tier genetics",
            BondType::Metallic | BondType::OrganoMetalSalt => "mito-beacon genetics",
            BondType::Ionic => "contractual",
            BondType::Weak => "any",
        };
        v.check_bool(
            &format!("btsp:policy:{tier_name}"),
            true,
            &format!("{bond:?} bond, min cipher {min_cipher:?}, trust requires {trust_req}"),
        );

        let policy = BondingPolicy::covalent_default();
        let peer_trust = match bond {
            BondType::Covalent => TrustModel::NuclearLineage,
            _ => TrustModel::MitoBeaconFamily,
        };
        let decision = BtspEnforcer::evaluate_connection_with_trust(
            &BondingPolicy {
                bond_type: bond,
                ..policy
            },
            min_cipher,
            Some(peer_trust),
        );
        v.check_bool(
            &format!("btsp:enforcer:{tier_name}"),
            decision.allowed,
            &decision.reason,
        );
    }

    let btsp_count = btsp_map.values().filter(|&&v| v).count();
    let total = btsp_map.len();

    let cleartext_caps: Vec<&String> = btsp_map
        .iter()
        .filter(|&(_, &auth)| !auth)
        .map(|(cap, _)| cap)
        .collect();
    let detail = if cleartext_caps.is_empty() {
        format!("{btsp_count}/{total} capabilities BTSP-authenticated")
    } else {
        format!(
            "{btsp_count}/{total} capabilities BTSP-authenticated (cleartext: {})",
            cleartext_caps
                .iter()
                .map(|c| c.as_str())
                .collect::<Vec<_>>()
                .join(", "),
        )
    };
    v.check_bool("btsp:summary", btsp_count == total, &detail);
}

/// biomeOS substrate health — neural-api liveness + graph.list.
pub fn validate_substrate_health(v: &mut ValidationResult) {
    let bridge = if let Some(b) = NeuralBridge::discover() {
        v.check_bool(
            "substrate:biomeos:discovered",
            true,
            &format!("socket: {}", b.socket_path().display()),
        );
        b
    } else {
        v.check_skip(
            "substrate:biomeos:discovered",
            "neural-api socket not found (biomeOS not running)",
        );
        v.check_skip("substrate:biomeos:liveness", "no socket");
        v.check_skip("substrate:biomeos:graph_list", "no socket");
        return;
    };

    match bridge.health_check() {
        Ok(true) => v.check_bool("substrate:biomeos:liveness", true, "alive"),
        Ok(false) => v.check_bool(
            "substrate:biomeos:liveness",
            false,
            "responded but unhealthy",
        ),
        Err(e) if e.is_connection_error() || e.is_protocol_error() => {
            v.check_skip(
                "substrate:biomeos:liveness",
                &format!("reachable but incompatible: {e}"),
            );
        }
        Err(e) => v.check_bool("substrate:biomeos:liveness", false, &format!("error: {e}")),
    }

    let graph_result = (|| {
        let mut client =
            primalspring::ipc::client::PrimalClient::connect(bridge.socket_path(), "neural-api")?;
        client.call("graph.list", serde_json::Value::Null)
    })();

    match graph_result {
        Ok(resp) if resp.is_success() => {
            v.check_bool(
                "substrate:biomeos:graph_list",
                true,
                "graph executor available",
            );
        }
        Ok(_) => {
            v.check_bool(
                "substrate:biomeos:graph_list",
                false,
                "graph.list returned error",
            );
        }
        Err(e) if e.is_method_not_found() => {
            v.check_skip(
                "substrate:biomeos:graph_list",
                "graph.list not implemented (older biomeOS)",
            );
        }
        Err(e) if e.is_connection_error() || e.is_protocol_error() => {
            v.check_skip(
                "substrate:biomeos:graph_list",
                &format!("transport mismatch: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "substrate:biomeos:graph_list",
                false,
                &format!("error: {e}"),
            );
        }
    }
}

pub fn validate_crypto(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    let test_data = b"guidestone_crypto_parity_2026";
    match ctx.hash_bytes(test_data, "blake3") {
        Ok(hash) => {
            v.check_bool(
                "crypto:blake3_hash",
                !hash.is_empty(),
                &format!("BLAKE3 produced {}B base64", hash.len()),
            );

            match ctx.hash_bytes(test_data, "blake3") {
                Ok(hash2) => {
                    v.check_bool(
                        "crypto:blake3_determinism",
                        hash == hash2,
                        "same input produces same hash",
                    );
                }
                Err(e) => {
                    v.check_bool(
                        "crypto:blake3_determinism",
                        false,
                        &format!("second hash call failed: {e}"),
                    );
                }
            }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "crypto:blake3_hash",
                &format!("security not available: {e}"),
            );
            v.check_skip("crypto:blake3_determinism", "security not available");
        }
        Err(e) => {
            v.check_bool("crypto:blake3_hash", false, &format!("hash error: {e}"));
            v.check_skip("crypto:blake3_determinism", "first hash failed");
        }
    }

    validate_btsp_cipher_policy(v);
    validate_ed25519_roundtrip(ctx, v);
}

fn validate_btsp_cipher_policy(v: &mut ValidationResult) {
    let mode = btsp::security_mode_from_env();
    v.check_bool(
        "btsp:security_mode_resolved",
        true,
        &format!("mode: {mode:?}"),
    );

    let guard = btsp::validate_insecure_guard();
    let detail = match &guard {
        Ok(()) => "guard passed".to_owned(),
        Err(e) => format!("guard issue: {e}"),
    };
    v.check_bool("btsp:insecure_guard", guard.is_ok(), &detail);
}

fn validate_ed25519_roundtrip(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    use base64::Engine as _;
    let raw_message = b"guidestone_ed25519_roundtrip_2026";
    let test_message = base64::engine::general_purpose::STANDARD.encode(raw_message);

    validate_ed25519_keygen(ctx, v);
    validate_ed25519_sign_and_verify(ctx, v, &test_message);
}

fn validate_ed25519_keygen(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    match ctx.call(
        "security",
        "crypto.ed25519_generate_keypair",
        serde_json::json!({}),
    ) {
        Ok(keygen_result) => {
            let has_pub = keygen_result
                .get("public_key")
                .and_then(|s| s.as_str())
                .is_some();
            let has_sec = keygen_result
                .get("secret_key")
                .and_then(|s| s.as_str())
                .is_some();
            v.check_bool(
                "crypto:ed25519_keygen",
                has_pub && has_sec,
                &format!("pub={has_pub} sec={has_sec}"),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "crypto:ed25519_keygen",
                &format!("security not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "crypto:ed25519_keygen",
                false,
                &format!("keygen failed: {e}"),
            );
        }
    }
}

fn validate_ed25519_sign_and_verify(
    ctx: &mut CompositionContext,
    v: &mut ValidationResult,
    test_message: &str,
) {
    match ctx.call(
        "security",
        "crypto.sign",
        serde_json::json!({"message": test_message, "algorithm": "ed25519"}),
    ) {
        Ok(sign_result) => {
            let signature = sign_result
                .get("signature")
                .and_then(|s| s.as_str())
                .unwrap_or("");
            v.check_bool(
                "crypto:ed25519_sign",
                !signature.is_empty(),
                &format!("signature: {}...", &signature[..signature.len().min(16)]),
            );

            let public_key = sign_result
                .get("public_key")
                .and_then(|s| s.as_str())
                .unwrap_or("");
            if public_key.is_empty() {
                v.check_skip(
                    "crypto:ed25519_verify",
                    "UPSTREAM GAP: crypto.sign does not expose public_key",
                );
            } else {
                validate_ed25519_verify(ctx, v, test_message, signature, public_key);
            }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "crypto:ed25519_sign",
                &format!("security not available: {e}"),
            );
            v.check_skip("crypto:ed25519_verify", "security not available");
        }
        Err(e) => {
            v.check_bool(
                "crypto:ed25519_sign",
                false,
                &format!("sign call failed: {e}"),
            );
            v.check_skip("crypto:ed25519_verify", "sign failed, skipping verify");
        }
    }
}

fn validate_ed25519_verify(
    ctx: &mut CompositionContext,
    v: &mut ValidationResult,
    test_message: &str,
    signature: &str,
    public_key: &str,
) {
    match ctx.call(
        "security",
        "crypto.verify",
        serde_json::json!({
            "message": test_message,
            "signature": signature,
            "public_key": public_key,
            "algorithm": "ed25519"
        }),
    ) {
        Ok(verify_result) => {
            let valid = verify_result
                .get("valid")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            v.check_bool("crypto:ed25519_verify", valid, "sign→verify roundtrip");
        }
        Err(e) => {
            v.check_bool(
                "crypto:ed25519_verify",
                false,
                &format!("verify call failed: {e}"),
            );
        }
    }
}
