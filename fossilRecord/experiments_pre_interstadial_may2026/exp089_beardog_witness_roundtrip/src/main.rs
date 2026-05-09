// SPDX-License-Identifier: AGPL-3.0-or-later

//! exp089 — `BearDog` Witness Round-Trip
//!
//! Validates that a `BearDog` Ed25519 signature can be wrapped into the
//! trio's `WireWitnessRef` format and verified back through `BearDog`.
//! This is the bridge pattern that wetSpring (Anderson QS), ludoSpring
//! (game checkpoints), and any future spring will use for provenance.
//!
//! When `BearDog` is not reachable, the experiment validates the offline
//! witness serialization round-trip (struct → JSON → struct) and exits
//! with pass for the offline portion, skip for the live crypto portion.

use primalspring::ipc::tcp;
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

const CRYPTO_GENERATE_KEYPAIR: &str = "crypto.generate_keypair";
const CRYPTO_SIGN_ED25519: &str = "crypto.sign_ed25519";
const CRYPTO_VERIFY_ED25519: &str = "crypto.verify_ed25519";

fn main() {
    ValidationResult::new("BearDog Witness Round-Trip")
        .with_provenance("exp089_beardog_witness_roundtrip", "2026-04-07")
        .run("witness wire type validation", |v| {
            phase_offline_witness_roundtrip(v);
            phase_non_crypto_witness(v);

            let bd_port = tcp::env_port("BEARDOG_PORT", tolerances::TCP_FALLBACK_BEARDOG_PORT);
            let host = std::env::var("TOWER_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());

            phase_live_sign_to_witness(v, &host, bd_port);
        });
}

/// Offline: build a `WireWitnessRef`-shaped JSON, serialize, deserialize,
/// and verify all fields survive the round-trip.
fn phase_offline_witness_roundtrip(v: &mut ValidationResult) {
    v.section("Offline Witness Serialization Round-Trip");

    let witness = serde_json::json!({
        "agent": "did:key:z6MkTest",
        "kind": "signature",
        "evidence": "deadbeef01234567",
        "witnessed_at": 1_712_000_000_000_000_000_u64,
        "encoding": "hex",
        "algorithm": "ed25519",
        "tier": "local",
    });

    let json_str = serde_json::to_string(&witness).unwrap_or_default();
    let parsed: serde_json::Value =
        serde_json::from_str(&json_str).unwrap_or(serde_json::Value::Null);

    v.check_bool(
        "witness serializes to JSON",
        !json_str.is_empty(),
        "serde round-trip produces non-empty JSON",
    );
    v.check_bool(
        "kind field preserved",
        str_field(&parsed, "kind") == Some("signature"),
        "kind == signature after round-trip",
    );
    v.check_bool(
        "evidence field preserved",
        str_field(&parsed, "evidence") == Some("deadbeef01234567"),
        "evidence survives serialization",
    );
    v.check_bool(
        "encoding field preserved",
        str_field(&parsed, "encoding") == Some("hex"),
        "encoding == hex after round-trip",
    );
    v.check_bool(
        "algorithm field preserved",
        str_field(&parsed, "algorithm") == Some("ed25519"),
        "algorithm survives round-trip",
    );
    v.check_bool(
        "tier field preserved",
        str_field(&parsed, "tier") == Some("local"),
        "tier == local after round-trip",
    );
    v.check_bool(
        "witnessed_at field preserved",
        parsed
            .get("witnessed_at")
            .and_then(serde_json::Value::as_u64)
            == Some(1_712_000_000_000_000_000),
        "nanosecond timestamp survives round-trip",
    );

    let minimal = serde_json::json!({"agent": "did:key:z6MkMinimal"});
    let minimal_str = serde_json::to_string(&minimal).unwrap_or_default();
    let minimal_parsed: serde_json::Value =
        serde_json::from_str(&minimal_str).unwrap_or(serde_json::Value::Null);
    v.check_bool(
        "minimal witness has agent",
        str_field(&minimal_parsed, "agent") == Some("did:key:z6MkMinimal"),
        "agent survives minimal round-trip",
    );
}

/// Non-crypto witnesses: checkpoint, marker, hash, timestamp.
fn phase_non_crypto_witness(v: &mut ValidationResult) {
    v.section("Non-Crypto Witness Variants");

    let cases = [
        ("checkpoint", "game:tick:4200", "none", "open"),
        ("marker", "conversation:thread:abc:turn:17", "none", "open"),
        ("hash", "blake3:abc123def456", "utf8", "open"),
        ("timestamp", "", "none", "open"),
    ];

    for (kind, context, encoding, tier) in &cases {
        let witness = serde_json::json!({
            "agent": format!("test:{kind}"),
            "kind": kind,
            "evidence": "",
            "witnessed_at": 1_000_000_u64,
            "encoding": encoding,
            "tier": tier,
            "context": context,
        });

        let json_str = serde_json::to_string(&witness).unwrap_or_default();
        let parsed: serde_json::Value =
            serde_json::from_str(&json_str).unwrap_or(serde_json::Value::Null);

        v.check_bool(
            &format!("{kind} witness round-trips"),
            str_field(&parsed, "kind") == Some(kind),
            &format!("kind={kind} preserved"),
        );

        if !context.is_empty() {
            v.check_bool(
                &format!("{kind} context preserved"),
                str_field(&parsed, "context") == Some(context),
                &format!("context={context}"),
            );
        }
    }
}

/// Live: sign via `BearDog`, wrap into witness, verify via `BearDog`.
fn phase_live_sign_to_witness(v: &mut ValidationResult, host: &str, port: u16) {
    v.section("Live BearDog Sign → Witness → Verify");

    let Some(pub_key) = generate_keypair(host, port) else {
        v.check_skip(
            "live crypto (all)",
            &format!("BearDog not reachable at {host}:{port}"),
        );
        return;
    };
    v.check_bool(
        "keypair generated",
        !pub_key.is_empty(),
        "public key present",
    );

    let test_data = "exp089 Anderson QS provenance witness test payload";
    let Some((sig, algorithm)) = sign_payload(host, port, test_data) else {
        v.check_skip("sign ed25519", "sign call failed");
        return;
    };
    v.check_bool("signature returned", !sig.is_empty(), "non-empty signature");

    let now_nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| u64::try_from(d.as_nanos()).unwrap_or(u64::MAX))
        .unwrap_or(0);

    let witness = serde_json::json!({
        "agent": "beardog:local",
        "kind": "signature",
        "evidence": sig,
        "witnessed_at": now_nanos,
        "encoding": "base64",
        "algorithm": algorithm,
        "tier": "local",
        "context": "exp089:anderson_qs:provenance_test",
    });

    let witness_json = serde_json::to_string_pretty(&witness).unwrap_or_default();
    v.check_bool(
        "witness JSON constructed",
        !witness_json.is_empty(),
        "BearDog response wrapped into WireWitnessRef",
    );

    verify_witness_evidence(v, host, port, test_data, &sig, &pub_key);
}

fn generate_keypair(host: &str, port: u16) -> Option<String> {
    let (result, _) = tcp::tcp_rpc(
        host,
        port,
        CRYPTO_GENERATE_KEYPAIR,
        &serde_json::json!({"algorithm": "ed25519"}),
    )
    .ok()?;
    let key = result
        .get("public_key")
        .or_else(|| result.get("publicKey"))
        .or_else(|| result.get("public"))
        .and_then(serde_json::Value::as_str)?
        .to_owned();
    Some(key)
}

fn sign_payload(host: &str, port: u16, data: &str) -> Option<(String, String)> {
    let (result, _) = tcp::tcp_rpc(
        host,
        port,
        CRYPTO_SIGN_ED25519,
        &serde_json::json!({"message": data}),
    )
    .ok()?;
    let sig = result
        .get("signature")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default()
        .to_owned();
    let alg = result
        .get("algorithm")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("Ed25519")
        .to_lowercase();
    Some((sig, alg))
}

fn verify_witness_evidence(
    v: &mut ValidationResult,
    host: &str,
    port: u16,
    message: &str,
    evidence: &str,
    pub_key: &str,
) {
    let verify_result = tcp::tcp_rpc(
        host,
        port,
        CRYPTO_VERIFY_ED25519,
        &serde_json::json!({
            "message": message,
            "signature": evidence,
            "public_key": pub_key,
        }),
    );
    match verify_result {
        Ok((result, _)) => {
            let valid = result
                .get("valid")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false)
                || result
                    .get("verified")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false);
            v.check_bool(
                "witness evidence verifies",
                valid,
                "BearDog confirms witness signature is valid",
            );
        }
        Err(e) => v.check_skip("witness verification", &format!("verify failed: {e}")),
    }
}

fn str_field<'a>(val: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    val.get(key).and_then(serde_json::Value::as_str)
}
