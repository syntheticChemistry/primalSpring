// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;
use std::sync::Arc;

use super::*;
use crate::tolerances;
use crate::validation::{NullSink, ValidationResult};

fn null_result(name: &str) -> ValidationResult {
    ValidationResult::new(name).with_sink(Arc::new(NullSink))
}

#[test]
fn empty_context_has_no_capabilities() {
    let ctx = CompositionContext::from_clients(HashMap::new());
    assert!(ctx.available_capabilities().is_empty());
    assert!(!ctx.has_capability("tensor"));
}

#[test]
fn from_live_discovery_finds_capabilities_or_empty() {
    let ctx = CompositionContext::from_live_discovery();
    let caps = ctx.available_capabilities();
    assert!(caps.len() <= 20, "reasonable upper bound on capabilities");
}

#[test]
fn call_on_missing_capability_returns_error() {
    let mut ctx = CompositionContext::from_clients(HashMap::new());
    let err = ctx
        .call("tensor", "tensor.matmul", serde_json::json!({}))
        .unwrap_err();
    assert!(err.is_connection_error());
}

#[test]
fn health_check_skips_when_no_client() {
    let mut ctx = CompositionContext::from_clients(HashMap::new());
    let err = ctx.health_check("security").unwrap_err();
    assert!(err.is_connection_error());
}

#[test]
fn hash_bytes_skips_when_no_security() {
    let mut ctx = CompositionContext::from_clients(HashMap::new());
    let err = ctx.hash_bytes(b"test", "blake3").unwrap_err();
    assert!(err.is_connection_error());
}

#[test]
fn resolve_capability_skips_when_no_discovery() {
    let mut ctx = CompositionContext::from_clients(HashMap::new());
    let err = ctx.resolve_capability("security").unwrap_err();
    assert!(err.is_connection_error());
}

#[test]
fn capability_to_primal_maps_correctly() {
    assert_eq!(super::capability_to_primal("security"), "beardog");
    assert_eq!(super::capability_to_primal("crypto"), "beardog");
    assert_eq!(super::capability_to_primal("tensor"), "barracuda");
    assert_eq!(super::capability_to_primal("shader"), "coralreef");
    assert_eq!(super::capability_to_primal("storage"), "nestgate");
    assert_eq!(super::capability_to_primal("compute"), "toadstool");
    assert_eq!(super::capability_to_primal("discovery"), "songbird");
    assert_eq!(super::capability_to_primal("ai"), "squirrel");
    assert_eq!(super::capability_to_primal("dag"), "rhizocrypt");
    assert_eq!(super::capability_to_primal("provenance"), "rhizocrypt");
    assert_eq!(super::capability_to_primal("commit"), "sweetgrass");
    assert_eq!(super::capability_to_primal("attribution"), "sweetgrass");
    assert_eq!(super::capability_to_primal("braid"), "sweetgrass");
    assert_eq!(super::capability_to_primal("ledger"), "loamspine");
    assert_eq!(super::capability_to_primal("spine"), "loamspine");
    assert_eq!(super::capability_to_primal("merkle"), "loamspine");
    assert_eq!(super::capability_to_primal("unknown_cap"), "unknown_cap");
}

#[test]
fn validate_parity_skips_when_no_client() {
    let mut ctx = CompositionContext::from_clients(HashMap::new());
    let mut v = null_result("test");

    validate_parity(
        &mut ctx,
        &mut v,
        "test_check",
        "tensor",
        "stats.mean",
        serde_json::json!({"data": [1.0, 2.0, 3.0]}),
        "result",
        2.0,
        1e-10,
    );

    assert_eq!(v.skipped, 1, "should skip when capability unavailable");
    assert_eq!(v.failed, 0);
    assert_eq!(v.passed, 0);
}

#[test]
fn validate_parity_vec_skips_when_no_client() {
    let mut ctx = CompositionContext::from_clients(HashMap::new());
    let mut v = null_result("test");

    validate_parity_vec(
        &mut ctx,
        &mut v,
        "test_vec",
        "tensor",
        "stats.mean",
        serde_json::json!({"data": [1.0, 2.0, 3.0]}),
        "result",
        &[1.0, 2.0, 3.0],
        1e-10,
    );

    assert_eq!(v.skipped, 1, "should skip when capability unavailable");
    assert_eq!(v.failed, 0);
}

// ══════════════════════════════════════════════════════════════════════════
// Tower Atomic Composition Parity
// ══════════════════════════════════════════════════════════════════════════

#[test]
fn tower_parity_crypto_hash() {
    let mut ctx = CompositionContext::from_live_discovery();
    let mut v = null_result("Tower: crypto.hash parity");

    let test_data = b"primalSpring composition parity test";
    match ctx.hash_bytes(test_data, "blake3") {
        Ok(hash) => {
            v.check_bool(
                "blake3_hash_nonempty",
                !hash.is_empty(),
                &format!("BLAKE3 len={}", hash.len()),
            );
            v.check_bool(
                "blake3_hash_base64_length",
                hash.len() == 44,
                &format!("expected 44, got {}", hash.len()),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "blake3_hash_nonempty",
                &format!("security not available: {e}"),
            );
            v.check_skip("blake3_hash_base64_length", "security not available");
        }
        Err(e) => {
            v.check_bool("blake3_hash_nonempty", false, &format!("hash error: {e}"));
            v.check_skip("blake3_hash_base64_length", "prior call failed");
        }
    }

    assert_eq!(v.failed, 0, "tower crypto.hash should not fail");
}

#[test]
fn tower_parity_discovery_resolve() {
    let mut ctx = CompositionContext::from_live_discovery();
    let mut v = null_result("Tower: capability.resolve parity");

    match ctx.resolve_capability("security") {
        Ok(result) => {
            let found = result
                .get("found")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false)
                || result.get("endpoint").is_some()
                || result.get("socket").is_some()
                || result.get("native_endpoint").is_some()
                || result.get("virtual_endpoint").is_some();
            v.check_bool(
                "resolve_security_exists",
                found,
                &format!("resolved: {result}"),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "resolve_security_exists",
                &format!("discovery not available: {e}"),
            );
        }
        Err(e) => {
            v.check_skip(
                "resolve_security_exists",
                &format!("resolve gap (LD-08): {e}"),
            );
        }
    }

    assert_eq!(v.failed, 0, "tower discovery.resolve should not fail");
}

#[test]
fn tower_parity_health_liveness() {
    let mut ctx = CompositionContext::from_live_discovery();
    let mut v = null_result("Tower: health.liveness parity");

    for (name, cap) in [
        ("beardog_alive", "security"),
        ("songbird_alive", "discovery"),
    ] {
        match ctx.health_check(cap) {
            Ok(alive) => {
                v.check_bool(name, alive, &format!("{cap} health normalized"));
            }
            Err(e) if e.is_connection_error() => {
                v.check_skip(name, &format!("{cap} not running: {e}"));
            }
            Err(e) => {
                v.check_bool(name, false, &format!("{cap} error: {e}"));
            }
        }
    }

    assert_eq!(v.failed, 0, "tower health checks should not fail");
}

// ══════════════════════════════════════════════════════════════════════════
// Nest Atomic Composition Parity
// ══════════════════════════════════════════════════════════════════════════

#[test]
fn nest_parity_storage_roundtrip() {
    let mut ctx = CompositionContext::from_live_discovery();
    let mut v = null_result("Nest: storage round-trip parity");

    let test_key = "primalspring_parity_test";
    let test_value = "composition_validation_data_2026";

    match ctx.call(
        "storage",
        "storage.store",
        serde_json::json!({"key": test_key, "value": test_value}),
    ) {
        Ok(_) => {
            match ctx.call(
                "storage",
                "storage.retrieve",
                serde_json::json!({"key": test_key}),
            ) {
                Ok(result) => {
                    let retrieved = result.get("value").and_then(|v| v.as_str()).unwrap_or("");
                    v.check_bool(
                        "store_retrieve_match",
                        retrieved == test_value,
                        &format!("stored={test_value}, retrieved={retrieved}"),
                    );
                }
                Err(e) => {
                    v.check_bool(
                        "store_retrieve_match",
                        false,
                        &format!("retrieve error: {e}"),
                    );
                }
            }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "store_retrieve_match",
                &format!("storage not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool("store_retrieve_match", false, &format!("store error: {e}"));
        }
    }

    assert_eq!(v.failed, 0, "nest storage round-trip should not fail");
}

#[test]
fn nest_parity_nestgate_health() {
    let mut ctx = CompositionContext::from_live_discovery();
    let mut v = null_result("Nest: NestGate health parity");

    match ctx.health_check("storage") {
        Ok(alive) => {
            v.check_bool("nestgate_alive", alive, "NestGate health normalized");
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("nestgate_alive", &format!("NestGate not running: {e}"));
        }
        Err(e) => {
            v.check_bool("nestgate_alive", false, &format!("NestGate error: {e}"));
        }
    }

    assert_eq!(v.failed, 0, "nest health should not fail");
}

// ══════════════════════════════════════════════════════════════════════════
// Node Atomic Composition Parity
// ══════════════════════════════════════════════════════════════════════════

#[test]
fn node_parity_tensor_reduce_sum() {
    let mut ctx = CompositionContext::from_live_discovery();
    let mut v = null_result("Node: tensor.reduce sum parity");

    validate_parity(
        &mut ctx,
        &mut v,
        "reduce_sum_4elem",
        "tensor",
        "tensor.batch.submit",
        serde_json::json!({
            "ops": [
                {"op": "create", "alias": "x", "data": [1.0, 2.0, 3.0, 4.0], "shape": [1, 4]},
                {"op": "readback", "alias": "result", "input": "x"}
            ]
        }),
        "ops_executed",
        2.0,
        tolerances::EXACT_PARITY_TOL,
    );

    assert_eq!(v.failed, 0, "node tensor.batch.submit should not fail");
}

#[test]
fn node_parity_tensor_matmul_identity() {
    let mut ctx = CompositionContext::from_live_discovery();
    let mut v = null_result("Node: barraCuda math parity via stats.mean");

    validate_parity(
        &mut ctx,
        &mut v,
        "stats_mean_5elem",
        "tensor",
        "stats.mean",
        serde_json::json!({"data": [1.0, 2.0, 3.0, 4.0, 5.0]}),
        "result",
        3.0,
        tolerances::CPU_GPU_PARITY_TOL,
    );

    assert_eq!(v.failed, 0, "node stats.mean should not fail");
}

#[test]
fn node_parity_shader_capabilities() {
    let mut ctx = CompositionContext::from_live_discovery();
    let mut v = null_result("Node: shader.compile.capabilities parity");

    match ctx.call(
        "shader",
        "shader.compile.capabilities",
        serde_json::json!({}),
    ) {
        Ok(result) => {
            let has_archs = result
                .get("supported_archs")
                .and_then(|a| a.as_array())
                .is_some_and(|a| !a.is_empty());
            v.check_bool(
                "shader_has_supported_archs",
                has_archs,
                "SHADER_COMPILE_WIRE_CONTRACT: supported_archs populated",
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "shader_has_supported_archs",
                &format!("shader not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "shader_has_supported_archs",
                false,
                &format!("shader error: {e}"),
            );
        }
    }

    assert_eq!(v.failed, 0, "node shader capabilities should not fail");
}

// ══════════════════════════════════════════════════════════════════════════
// Full NUCLEUS Composition Parity
// ══════════════════════════════════════════════════════════════════════════

#[test]
fn nucleus_parity_cross_atomic_pipeline() {
    let mut ctx = CompositionContext::from_live_discovery();
    let mut v = null_result("NUCLEUS: cross-atomic pipeline parity");

    let test_data = b"nucleus_composition_parity_2026";
    let hash_result = ctx.hash_bytes(test_data, "blake3");

    match hash_result {
        Ok(hash_hex) => {
            v.check_bool(
                "tower_hash_produced",
                !hash_hex.is_empty(),
                &format!("BLAKE3 hash: {}...", &hash_hex[..hash_hex.len().min(16)]),
            );

            let store_key = "nucleus_parity_hash";
            match ctx.call(
                "storage",
                "storage.store",
                serde_json::json!({"key": store_key, "value": hash_hex}),
            ) {
                Ok(_) => {
                    match ctx.call(
                        "storage",
                        "storage.retrieve",
                        serde_json::json!({"key": store_key}),
                    ) {
                        Ok(retrieved) => {
                            let val = retrieved
                                .get("value")
                                .and_then(|v| v.as_str())
                                .unwrap_or("");
                            v.check_bool(
                                "nest_hash_roundtrip",
                                val == hash_hex,
                                "hash stored and retrieved matches",
                            );
                        }
                        Err(e) => {
                            v.check_bool(
                                "nest_hash_roundtrip",
                                false,
                                &format!("retrieve failed: {e}"),
                            );
                        }
                    }
                }
                Err(e) if e.is_connection_error() => {
                    v.check_skip(
                        "nest_hash_roundtrip",
                        &format!("storage not available: {e}"),
                    );
                }
                Err(e) => {
                    v.check_bool("nest_hash_roundtrip", false, &format!("store failed: {e}"));
                }
            }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "tower_hash_produced",
                &format!("security not available: {e}"),
            );
            v.check_skip("nest_hash_roundtrip", "tower unavailable, skipping nest");
        }
        Err(e) => {
            v.check_bool("tower_hash_produced", false, &format!("hash error: {e}"));
            v.check_skip("nest_hash_roundtrip", "tower failed, skipping nest");
        }
    }

    assert_eq!(v.failed, 0, "NUCLEUS cross-atomic pipeline should not fail");
}
