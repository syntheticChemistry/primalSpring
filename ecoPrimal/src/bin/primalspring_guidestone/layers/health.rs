// SPDX-License-Identifier: AGPL-3.0-or-later

//! Layer 2 + 3 + 4: Atomic health, capability parity, and cross-atomic pipeline.

use primalspring::composition::{self, CompositionContext};
use primalspring::coordination::AtomicType;
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

pub fn validate_atomic_health(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    let tiers: &[(&str, &[&str])] = &[
        ("Tower", AtomicType::Tower.required_capabilities()),
        ("Node", AtomicType::Node.required_capabilities()),
        ("Nest", AtomicType::Nest.required_capabilities()),
    ];

    for &(tier_name, caps) in tiers {
        for &cap in caps {
            let check_name = format!("health:{tier_name}:{cap}");
            match ctx.health_check(cap) {
                Ok(true) => v.check_bool(&check_name, true, "alive"),
                Ok(false) => v.check_bool(&check_name, false, "responded but not alive"),
                Err(e) if e.is_connection_error() => {
                    v.check_skip(&check_name, &format!("not reachable: {e}"));
                }
                Err(e) if e.is_protocol_error() => {
                    v.check_skip(
                        &check_name,
                        &format!("reachable but protocol mismatch: {e}"),
                    );
                }
                Err(e) => v.check_bool(&check_name, false, &format!("error: {e}")),
            }
        }
    }
}

pub fn validate_math_parity(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    composition::validate_parity_flex(
        ctx,
        v,
        "parity:stats.mean",
        "tensor",
        "stats.mean",
        serde_json::json!({"data": [1.0, 2.0, 3.0, 4.0, 5.0]}),
        &["result", "mean", "output", "data", "value"],
        3.0,
        tolerances::IPC_ROUND_TRIP_TOL,
    );

    composition::validate_parity_vec_flex(
        ctx,
        v,
        "parity:tensor.matmul_identity",
        "tensor",
        "tensor.matmul_inline",
        serde_json::json!({
            "lhs": [[1.0, 0.0], [0.0, 1.0]],
            "rhs": [[3.0, 7.0], [2.0, 5.0]]
        }),
        &["result", "data", "output", "matrix"],
        &[3.0, 7.0, 2.0, 5.0],
        tolerances::IPC_ROUND_TRIP_TOL,
    );
}

pub fn validate_storage_roundtrip(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    let test_key = "guidestone_parity_roundtrip";
    let test_value = "primalspring_guidestone_2026";

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
                Ok(retrieved) => {
                    let val = retrieved
                        .get("value")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    v.check_bool(
                        "parity:storage_roundtrip",
                        val == test_value,
                        &format!("stored={test_value}, retrieved={val}"),
                    );
                }
                Err(e) => {
                    v.check_bool(
                        "parity:storage_roundtrip",
                        false,
                        &format!("retrieve failed: {e}"),
                    );
                }
            }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "parity:storage_roundtrip",
                &format!("storage not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "parity:storage_roundtrip",
                false,
                &format!("store failed: {e}"),
            );
        }
    }
}

pub fn validate_shader_capabilities(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    match ctx.call(
        "shader",
        "shader.compile.capabilities",
        serde_json::json!({}),
    ) {
        Ok(result) => {
            let has_archs = result
                .get("supported_archs")
                .and_then(|c| c.as_array())
                .is_some_and(|c| !c.is_empty());
            let has_legacy = result
                .get("capabilities")
                .and_then(|c| c.as_array())
                .is_some_and(|c| !c.is_empty());
            let arch_count = result
                .get("supported_archs")
                .and_then(|c| c.as_array())
                .map_or(0, Vec::len);
            v.check_bool(
                "parity:shader_capabilities",
                has_archs || has_legacy,
                &format!("{arch_count} supported architectures"),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "parity:shader_capabilities",
                &format!("shader not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "parity:shader_capabilities",
                false,
                &format!("call failed: {e}"),
            );
        }
    }
}

pub fn validate_cross_atomic_pipeline(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    let test_data = b"guidestone_cross_atomic_2026";

    let hash_result = ctx.hash_bytes(test_data, "blake3");
    match hash_result {
        Ok(hash_b64) => {
            v.check_bool(
                "pipeline:tower_hash",
                !hash_b64.is_empty(),
                &format!("BLAKE3: {}...", &hash_b64[..hash_b64.len().min(16)]),
            );

            let store_key = "guidestone_pipeline_hash";
            match ctx.call(
                "storage",
                "storage.store",
                serde_json::json!({"key": store_key, "value": &hash_b64}),
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
                                "pipeline:nest_roundtrip",
                                val == hash_b64,
                                "hash stored and retrieved matches",
                            );
                        }
                        Err(e) => {
                            v.check_bool(
                                "pipeline:nest_roundtrip",
                                false,
                                &format!("retrieve failed: {e}"),
                            );
                        }
                    }
                }
                Err(e) if e.is_connection_error() => {
                    v.check_skip(
                        "pipeline:nest_roundtrip",
                        &format!("storage not available: {e}"),
                    );
                }
                Err(e) => {
                    v.check_bool(
                        "pipeline:nest_roundtrip",
                        false,
                        &format!("store failed: {e}"),
                    );
                }
            }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "pipeline:tower_hash",
                &format!("security not available: {e}"),
            );
            v.check_skip(
                "pipeline:nest_roundtrip",
                "tower unavailable, skipping nest",
            );
        }
        Err(e) => {
            v.check_bool("pipeline:tower_hash", false, &format!("hash error: {e}"));
            v.check_skip("pipeline:nest_roundtrip", "tower failed, skipping nest");
        }
    }
}
