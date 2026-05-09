// SPDX-License-Identifier: AGPL-3.0-or-later

//! Contract tests using stub primals.
//!
//! These tests validate primalSpring's composition and IPC infrastructure
//! against stub JSON-RPC responders that speak the Tower contract (BearDog +
//! Songbird method names, response shapes, error codes) without requiring
//! live plasmidBin binaries.
//!
//! Activated with `cargo test --features stub-primals`.

#![cfg(feature = "stub-primals")]

mod integration;

use integration::stub_harness::{stub_beardog, stub_songbird, stub_tower};

use primalspring::ipc::client::PrimalClient;
use primalspring::ipc::method_gate::{
    BearDogVerifier, CallerContext, EnforcementMode, MethodGate, TokenVerifier,
};

// ── BearDog contract tests ──────────────────────────────────────────────

#[test]
fn stub_beardog_health_liveness() {
    let bd = stub_beardog();
    let mut client = PrimalClient::connect(&bd.socket_path, "beardog").unwrap();
    let alive = client.health_liveness().unwrap();
    assert!(alive);
}

#[test]
fn stub_beardog_identity() {
    let bd = stub_beardog();
    let mut client = PrimalClient::connect(&bd.socket_path, "beardog").unwrap();
    let resp = client.call("identity.get", serde_json::json!({})).unwrap();
    let result = resp.result.unwrap();
    assert_eq!(result["primal_id"], "beardog");
    assert_eq!(result["domain"], "security");
}

#[test]
fn stub_beardog_capabilities_list() {
    let bd = stub_beardog();
    let mut client = PrimalClient::connect(&bd.socket_path, "beardog").unwrap();
    let resp = client
        .call("capabilities.list", serde_json::json!({}))
        .unwrap();
    let result = resp.result.unwrap();
    let caps = result["capabilities"].as_array().unwrap();
    assert!(caps.iter().any(|c| c == "crypto.hash"));
    assert!(caps.iter().any(|c| c == "auth.check"));
}

#[test]
fn stub_beardog_auth_mode() {
    let bd = stub_beardog();
    let mut client = PrimalClient::connect(&bd.socket_path, "beardog").unwrap();
    let resp = client.call("auth.mode", serde_json::json!({})).unwrap();
    let result = resp.result.unwrap();
    assert_eq!(result["mode"], "permissive");
}

#[test]
fn stub_beardog_token_roundtrip() {
    let bd = stub_beardog();
    let mut client = PrimalClient::connect(&bd.socket_path, "beardog").unwrap();

    let issue_resp = client
        .call(
            "auth.issue_ionic",
            serde_json::json!({ "scope": "stats.*", "purpose": "test" }),
        )
        .unwrap();
    let token = issue_resp.result.unwrap()["token"]
        .as_str()
        .unwrap()
        .to_owned();
    assert!(token.starts_with("stub-ionic-"));

    let verify_resp = client
        .call("auth.verify_ionic", serde_json::json!({ "token": token }))
        .unwrap();
    let verify = verify_resp.result.unwrap();
    assert!(verify["valid"].as_bool().unwrap());
}

#[test]
fn stub_beardog_verifier_trait() {
    let bd = stub_beardog();
    let verifier = BearDogVerifier::new(bd.socket_path.clone());
    let verified = verifier.verify("stub-ionic-stats.*");
    assert!(verified.is_some());
    let v = verified.unwrap();
    assert!(!v.scopes.is_empty());
}

#[test]
fn stub_beardog_crypto_hash() {
    let bd = stub_beardog();
    let mut client = PrimalClient::connect(&bd.socket_path, "beardog").unwrap();
    let resp = client
        .call(
            "crypto.hash",
            serde_json::json!({ "data": "aGVsbG8=", "algorithm": "blake3" }),
        )
        .unwrap();
    let result = resp.result.unwrap();
    assert!(result["hash"].as_str().unwrap().starts_with("stub-hash-"));
}

// ── Songbird contract tests ─────────────────────────────────────────────

#[test]
fn stub_songbird_health_liveness() {
    let bd = stub_beardog();
    let sb = stub_songbird(&[("beardog", &bd.socket_path)]);
    let mut client = PrimalClient::connect(&sb.socket_path, "songbird").unwrap();
    let alive = client.health_liveness().unwrap();
    assert!(alive);
}

#[test]
fn stub_songbird_resolve() {
    let bd = stub_beardog();
    let sb = stub_songbird(&[("beardog", &bd.socket_path)]);
    let mut client = PrimalClient::connect(&sb.socket_path, "songbird").unwrap();
    let resp = client
        .call("ipc.resolve", serde_json::json!({ "primal_id": "beardog" }))
        .unwrap();
    let result = resp.result.unwrap();
    assert_eq!(result["primal_id"], "beardog");
    assert!(result["socket"].as_str().is_some());
}

// ── Tower contract tests (BearDog + Songbird) ──────────────────────────

#[test]
fn stub_tower_pair_starts() {
    let (bd, sb) = stub_tower();
    let mut bd_client = PrimalClient::connect(&bd.socket_path, "beardog").unwrap();
    let mut sb_client = PrimalClient::connect(&sb.socket_path, "songbird").unwrap();
    assert!(bd_client.health_liveness().unwrap());
    assert!(sb_client.health_liveness().unwrap());
}

// ── Gate integration with stubs ─────────────────────────────────────────

#[test]
fn gate_with_stub_beardog_verifier() {
    let bd = stub_beardog();
    let verifier = Box::new(BearDogVerifier::new(bd.socket_path.clone()));
    let gate = MethodGate::with_verifier(EnforcementMode::Enforced, verifier);

    let ctx = CallerContext::loopback();
    let params = serde_json::json!({
        "_bearer_token": "stub-ionic-stats.*",
        "values": [1, 2, 3],
    });
    let enriched = ctx.with_params_token(&params, gate.verifier());
    assert!(enriched.verified.is_some());
    assert!(gate.check("stats.mean", &enriched).is_ok());
}

#[test]
fn gate_rejects_wrong_scope_with_stub_verifier() {
    let bd = stub_beardog();
    let verifier = Box::new(BearDogVerifier::new(bd.socket_path.clone()));
    let gate = MethodGate::with_verifier(EnforcementMode::Enforced, verifier);

    let ctx = CallerContext::loopback();
    let params = serde_json::json!({
        "_bearer_token": "stub-ionic-storage.*",
    });
    let enriched = ctx.with_params_token(&params, gate.verifier());
    assert!(enriched.verified.is_some());
    let result = gate.check("stats.mean", &enriched);
    assert!(result.is_err());
}

// ── Method-not-found contract test ──────────────────────────────────────

#[test]
fn stub_returns_method_not_found_for_unknown() {
    let bd = stub_beardog();
    let mut client = PrimalClient::connect(&bd.socket_path, "beardog").unwrap();
    let resp = client
        .call("nonexistent.method", serde_json::json!({}))
        .unwrap();
    assert!(resp.error.is_some());
    assert_eq!(resp.error.unwrap().code, -32601);
}
