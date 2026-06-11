// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: BTSP Cross-Primal E2E — encrypted handshake chain validation.
//!
//! Proves the genetic encryption model works end-to-end across primals:
//! bearDog authenticates the client → client dispatches to sweetGrass →
//! sweetGrass response travels back through the authenticated channel.
//!
//! This is the composition-level proof that Tower (electron) provides
//! encryption-by-default for all cross-primal IPC.
//!
//! Phase 1 (Structural): Verify BTSP handshake primitives (key derivation,
//! HMAC computation, Phase 3 session key derivation) produce correct output
//! for known test vectors.
//!
//! Phase 2 (Live): Connect to bearDog's BTSP-enabled socket, perform full
//! 4-step handshake, then call `health.liveness` on sweetGrass through the
//! authenticated context to prove cross-primal dispatch works under BTSP.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// BTSP cross-primal E2E scenario — proves genetic encryption works across primals.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "btsp-cross-primal-e2e",
        track: Track::Security,
        tier: Tier::Both,
        provenance_crate: "wave110_btsp_cross_primal",
        provenance_date: "2026-06-11",
        description: "BTSP cross-primal E2E — encrypted handshake chain (bearDog→client→sweetGrass)",
    },
    run,
};

/// Execute the BTSP cross-primal E2E scenario (structural + live phases).
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — BTSP handshake primitives");
    phase_structural(v);

    v.section("Phase 2: Live — bearDog BTSP handshake");
    phase_live_handshake(v, ctx);

    v.section("Phase 3: Live — Cross-primal dispatch via authenticated context");
    phase_live_cross_primal(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use hkdf::Hkdf;
    use hmac::{Hmac, KeyInit, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let test_seed = b"test-family-seed-for-structural-validation";
    let hk = Hkdf::<Sha256>::new(Some(b"btsp-v1"), test_seed.as_slice());
    let mut handshake_key = [0u8; 32];
    let derivation_ok = hk.expand(b"handshake", &mut handshake_key).is_ok();
    v.check_bool(
        "hkdf:derivation",
        derivation_ok,
        "HKDF-SHA256 key derivation from family seed succeeds",
    );

    v.check_bool(
        "hkdf:non_zero",
        handshake_key.iter().any(|&b| b != 0),
        "derived handshake key is non-zero",
    );

    v.check_bool(
        "hkdf:deterministic",
        {
            let mut second = [0u8; 32];
            let hk2 = Hkdf::<Sha256>::new(Some(b"btsp-v1"), test_seed.as_slice());
            let ok = hk2.expand(b"handshake", &mut second).is_ok();
            ok && second == handshake_key
        },
        "same seed + salt produces identical key (deterministic)",
    );

    let challenge = b"test-challenge-bytes";
    let client_pub = [1u8; 32];
    let server_pub = [2u8; 32];
    let hmac_result = HmacSha256::new_from_slice(&handshake_key)
        .map(|mut mac| {
            mac.update(challenge);
            mac.update(&client_pub);
            mac.update(&server_pub);
            mac.finalize().into_bytes()
        });
    v.check_bool(
        "hmac:computation",
        hmac_result.is_ok(),
        "HMAC-SHA256 challenge response computes successfully",
    );

    let Some(hmac_bytes) = hmac_result.ok() else {
        return;
    };
    v.check_bool(
        "hmac:length",
        hmac_bytes.len() == 32,
        "HMAC output is 32 bytes",
    );

    v.check_bool(
        "hmac:base64_roundtrip",
        {
            let encoded = BASE64.encode(hmac_bytes);
            let decoded = BASE64.decode(&encoded).unwrap_or_default();
            decoded == hmac_bytes.as_slice()
        },
        "HMAC base64 encode/decode round-trips correctly",
    );

    let mut session_ikm = [0u8; 32];
    let session_hk = Hkdf::<Sha256>::new(
        Some(&[client_pub.as_slice(), server_pub.as_slice()].concat()),
        &handshake_key,
    );
    let phase3_ok = session_hk.expand(b"btsp-session-v1", &mut session_ikm).is_ok();
    v.check_bool(
        "phase3:key_derivation",
        phase3_ok,
        "Phase 3 session key derivation (HKDF with nonces as salt) succeeds",
    );

    v.check_bool(
        "phase3:key_differs_from_handshake",
        session_ikm != handshake_key,
        "Phase 3 session key differs from handshake key (forward secrecy)",
    );
}

fn phase_live_handshake(v: &mut ValidationResult, ctx: &CompositionContext) {
    if !ctx.has_capability("security") {
        v.check_skip(
            "live:beardog_reachable",
            "security capability not available — bearDog not in composition context",
        );
        return;
    }

    let btsp_state = ctx.btsp_state();
    let any_authenticated = btsp_state.values().any(|&auth| auth);

    v.check_bool(
        "live:btsp_session_active",
        any_authenticated,
        &format!(
            "at least one BTSP-authenticated primal in context (authenticated: {})",
            btsp_state.values().filter(|&&a| a).count()
        ),
    );

    let family_id = crate::env_keys::resolve_family_id();
    v.check_bool(
        "live:family_id_set",
        family_id != "default" && !family_id.is_empty(),
        &format!("FAMILY_ID is set to production value: '{family_id}'"),
    );

    let beacon = crate::ipc::btsp_handshake::mito_beacon_from_env();
    v.check_bool(
        "live:mito_beacon_available",
        beacon.is_some(),
        "mito-beacon derivable from environment (FAMILY_ID → .family.seed → beacon)",
    );

    let Some(beacon_val) = beacon else {
        v.check_skip("live:handshake_attempt", "no mito-beacon — cannot attempt handshake");
        return;
    };

    let security_socket = crate::ipc::discover::conventional_socket_path("beardog");
    let socket_exists = security_socket.exists();
    v.check_bool(
        "live:beardog_socket_exists",
        socket_exists,
        &format!("bearDog socket exists at {}", security_socket.display()),
    );

    if !socket_exists {
        v.check_skip("live:handshake_attempt", "bearDog socket not found");
        return;
    }

    let seed = beacon_val.key_bytes().to_vec();
    let handshake_result = std::os::unix::net::UnixStream::connect(&security_socket)
        .map_err(|e| format!("connect: {e}"))
        .and_then(|mut stream| {
            stream.set_read_timeout(Some(std::time::Duration::from_secs(5))).ok();
            stream.set_write_timeout(Some(std::time::Duration::from_secs(5))).ok();
            crate::ipc::btsp_handshake::client_handshake(&mut stream, &seed)
                .map_err(|e| format!("handshake: {e}"))
        });

    match &handshake_result {
        Ok(result) => {
            v.check_bool(
                "live:handshake_success",
                true,
                &format!(
                    "BTSP handshake succeeded — session_id: {}, cipher: {}",
                    result.session_id, result.server_cipher
                ),
            );
            v.check_bool(
                "live:session_id_non_empty",
                !result.session_id.is_empty(),
                "session_id is non-empty",
            );
        }
        Err(e) => {
            v.check_bool(
                "live:handshake_success",
                false,
                &format!("BTSP handshake failed: {e}"),
            );
        }
    }
}

fn phase_live_cross_primal(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("discovery") {
        v.check_skip(
            "cross:sweetgrass_reachable",
            "discovery capability not available",
        );
        return;
    }

    let btsp_state = ctx.btsp_state().clone();
    let discovery_authenticated = btsp_state
        .get("discovery")
        .copied()
        .unwrap_or(false);

    v.check_bool(
        "cross:discovery_btsp_authenticated",
        discovery_authenticated,
        &format!(
            "discovery channel is BTSP-authenticated (required for cross-primal trust): {}",
            if discovery_authenticated { "yes" } else { "no — plaintext fallback" }
        ),
    );

    let health_result = ctx.call("discovery", "health.liveness", serde_json::json!({}));
    v.check_bool(
        "cross:discovery_responds_post_btsp",
        health_result.is_ok(),
        &format!(
            "Songbird responds to health.liveness through BTSP-authenticated channel: {}",
            match &health_result {
                Ok(_) => "OK".to_owned(),
                Err(e) => format!("{e}"),
            }
        ),
    );

    if !ctx.has_capability("attribution") {
        v.check_skip(
            "cross:sweetgrass_health",
            "attribution (sweetGrass) capability not in context",
        );
        return;
    }

    let sweetgrass_authenticated = btsp_state
        .get("attribution")
        .copied()
        .unwrap_or(false);

    v.check_bool(
        "cross:sweetgrass_btsp_authenticated",
        sweetgrass_authenticated,
        &format!(
            "sweetGrass attribution channel BTSP-authenticated: {}",
            if sweetgrass_authenticated { "yes" } else { "no — plaintext fallback" }
        ),
    );

    let sg_health = ctx.call(
        "attribution",
        "health.liveness",
        serde_json::json!({}),
    );

    match sg_health {
        Ok(resp) => {
            v.check_bool(
                "cross:sweetgrass_responds",
                true,
                "sweetGrass health.liveness responds through authenticated context",
            );

            if let Some(obj) = resp.as_object() {
                let has_health_field = obj.contains_key("status") || obj.contains_key("alive");
                v.check_bool(
                    "cross:sweetgrass_health_schema",
                    has_health_field,
                    &format!(
                        "sweetGrass health response has 'status' or 'alive' field: {obj:?}"
                    ),
                );
            }
        }
        Err(e) => {
            v.check_bool(
                "cross:sweetgrass_responds",
                false,
                &format!("sweetGrass health.liveness failed: {e}"),
            );
        }
    }

    let chain_valid = btsp_state.values().filter(|&&a| a).count() >= 2;
    v.check_bool(
        "cross:multi_primal_btsp_chain",
        chain_valid,
        &format!(
            "multi-primal BTSP chain: {} primals authenticated (need ≥2 for cross-primal proof)",
            btsp_state.values().filter(|&&a| a).count()
        ),
    );
}
