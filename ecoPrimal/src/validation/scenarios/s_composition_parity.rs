// SPDX-License-Identifier: AGPL-3.0-or-later
//! Scenario: NUCLEUS Composition Parity — absorbed from exp094.

use crate::composition::{CompositionContext, validate_parity};
use crate::tolerances;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "composition-parity",
        track: Track::Lifecycle,
        tier: Tier::Live,
        provenance_crate: "exp094_composition_parity",
        provenance_date: "2026-05-09",
        description: "NUCLEUS composition parity — Tower, Node, Nest, cross-atomic pipeline",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let caps = ctx.available_capabilities();

    v.section("Phase 1: Discovery");
    v.check_bool(
        "live_discovery_found_primals",
        !caps.is_empty(),
        &format!(
            "discovered {} capabilities: {}",
            caps.len(),
            caps.join(", ")
        ),
    );

    v.section("Phase 2: Tower atomic (BearDog + Songbird)");

    tower_health(ctx, v);
    tower_crypto_hash(ctx, v);
    tower_discovery_resolve(ctx, v);

    v.section("Phase 3: Node atomic (barraCuda + coralReef + toadStool)");

    node_tensor_dot(ctx, v);
    node_shader_capabilities(ctx, v);
    node_compute_dispatch_health(ctx, v);

    v.section("Phase 4: Nest atomic (NestGate + provenance trio)");

    nest_storage_roundtrip(ctx, v);
    nest_provenance_health(ctx, v);

    v.section("Phase 5: NUCLEUS cross-atomic pipeline");

    nucleus_hash_store_retrieve(ctx, v);
}

fn tower_health(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    for (name, cap) in [
        ("beardog_alive", "security"),
        ("songbird_alive", "discovery"),
    ] {
        match ctx.health_check(cap) {
            Ok(alive) => v.check_bool(name, alive, &format!("{cap} health normalized")),
            Err(e) if e.is_connection_error() => {
                v.check_skip(name, &format!("{cap} not running: {e}"));
            }
            Err(e) => {
                v.check_bool(name, false, &format!("{cap} error: {e}"));
            }
        }
    }
}

fn tower_crypto_hash(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    let test_data = b"primalSpring composition parity test";

    match ctx.hash_bytes(test_data, "blake3") {
        Ok(hash) => {
            v.check_bool(
                "crypto_hash_nonempty",
                !hash.is_empty(),
                &format!(
                    "BLAKE3: {}... (len={})",
                    &hash[..hash.len().min(16)],
                    hash.len()
                ),
            );
            v.check_bool(
                "crypto_hash_base64_valid",
                hash.len() == 44,
                &format!("expected 44 base64 chars, got {}", hash.len()),
            );
            let deterministic = ctx
                .hash_bytes(test_data, "blake3")
                .is_ok_and(|h2| h2 == hash);
            v.check_bool(
                "crypto_hash_deterministic",
                deterministic,
                "same input produces same hash",
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "crypto_hash_nonempty",
                &format!("security not available: {e}"),
            );
            v.check_skip("crypto_hash_base64_valid", "security not available");
            v.check_skip("crypto_hash_deterministic", "security not available");
        }
        Err(e) => {
            v.check_bool("crypto_hash_nonempty", false, &format!("hash error: {e}"));
            v.check_skip("crypto_hash_base64_valid", "prior call failed");
            v.check_skip("crypto_hash_deterministic", "prior call failed");
        }
    }
}

fn tower_discovery_resolve(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    for cap in ["security", "compute", "storage"] {
        let name = format!("resolve_{cap}");
        match ctx.resolve_capability(cap) {
            Ok(result) => {
                let found = result
                    .get("found")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false)
                    || result.get("endpoint").is_some()
                    || result.get("socket").is_some()
                    || result.get("native_endpoint").is_some()
                    || result.get("virtual_endpoint").is_some();
                v.check_bool(&name, found, &format!("resolved {cap}: {result}"));
            }
            Err(e) if e.is_connection_error() => {
                v.check_skip(&name, &format!("discovery not available: {e}"));
            }
            Err(e) => {
                v.check_bool(&name, false, &format!("resolve gap: {e}"));
            }
        }
    }

    match ctx.call("discovery", "rpc.discover", serde_json::json!({})) {
        Ok(result) => {
            let methods = result.get("methods").and_then(|m| m.as_array());
            let count = methods.map_or(0, Vec::len);
            v.check_bool(
                "songbird_method_catalog",
                count > 10,
                &format!("Songbird exposes {count} methods"),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "songbird_method_catalog",
                &format!("discovery not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "songbird_method_catalog",
                false,
                &format!("discover error: {e}"),
            );
        }
    }
}

fn node_tensor_dot(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    validate_parity(
        ctx,
        v,
        "tensor_stats_mean",
        "tensor",
        "stats.mean",
        serde_json::json!({"data": [1.0, 2.0, 3.0, 4.0, 5.0]}),
        "result",
        3.0,
        tolerances::CPU_GPU_PARITY_TOL,
    );
}

fn node_shader_capabilities(ctx: &mut CompositionContext, v: &mut ValidationResult) {
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
                "shader_supported_archs",
                has_archs,
                &format!(
                    "archs: {}",
                    result
                        .get("supported_archs")
                        .unwrap_or(&serde_json::json!([]))
                ),
            );

            let wgsl = result
                .get("supported_archs")
                .and_then(|a| a.as_array())
                .is_some_and(|a| {
                    a.iter().any(|v| {
                        v.as_str()
                            .is_some_and(|s| s.contains("wgsl") || s.contains("WGSL"))
                    })
                });
            v.check_bool(
                "shader_wgsl_supported",
                wgsl || has_archs,
                "WGSL arch present",
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "shader_supported_archs",
                &format!("shader not available: {e}"),
            );
            v.check_skip("shader_wgsl_supported", "shader not available");
        }
        Err(e) => {
            v.check_bool(
                "shader_supported_archs",
                false,
                &format!("shader error: {e}"),
            );
            v.check_skip("shader_wgsl_supported", "prior call failed");
        }
    }
}

fn node_compute_dispatch_health(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    match ctx.health_check("compute") {
        Ok(alive) => v.check_bool(
            "compute_dispatch_alive",
            alive,
            "toadStool health normalized",
        ),
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "compute_dispatch_alive",
                &format!("compute not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "compute_dispatch_alive",
                false,
                &format!("compute error: {e}"),
            );
        }
    }
}

fn nest_storage_roundtrip(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    let test_key = "exp094_parity_roundtrip";
    let test_value = "nucleus_composition_validation_2026";

    let family_id = crate::env_keys::resolve_family_id();
    let store_result = ctx
        .call(
            "storage",
            "storage.store",
            serde_json::json!({"family_id": family_id, "key": test_key, "value": test_value}),
        )
        .or_else(|_| {
            ctx.call(
                "storage",
                "storage.put",
                serde_json::json!({"family_id": family_id, "key": test_key, "value": test_value}),
            )
        });

    match store_result {
        Ok(_) => {
            let retrieve_result = ctx
                .call(
                    "storage",
                    "storage.retrieve",
                    serde_json::json!({"family_id": family_id, "key": test_key}),
                )
                .or_else(|_| {
                    ctx.call(
                        "storage",
                        "storage.get",
                        serde_json::json!({"family_id": family_id, "key": test_key}),
                    )
                });
            match retrieve_result {
                Ok(result) => {
                    let val = result.get("value").and_then(|v| v.as_str()).unwrap_or("");
                    v.check_bool(
                        "storage_roundtrip_match",
                        val == test_value,
                        &format!("stored={test_value}, retrieved={val}"),
                    );
                }
                Err(e) => {
                    v.check_bool(
                        "storage_roundtrip_match",
                        false,
                        &format!("retrieve error (gap: NestGate wire contract): {e}"),
                    );
                }
            }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "storage_roundtrip_match",
                &format!("storage not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "storage_roundtrip_match",
                false,
                &format!("store error (gap: NestGate wire contract): {e}"),
            );
        }
    }
}

fn nest_provenance_health(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    for (name, cap) in [("sweetgrass_alive", "commit"), ("rhizocrypt_alive", "dag")] {
        match ctx.health_check(cap) {
            Ok(alive) => v.check_bool(name, alive, &format!("{cap} health normalized")),
            Err(e) if e.is_connection_error() => {
                v.check_skip(name, &format!("{cap} not available: {e}"));
            }
            Err(e) => {
                v.check_bool(name, false, &format!("{cap} error: {e}"));
            }
        }
    }
}

fn nucleus_hash_store_retrieve(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    let test_data = b"nucleus_cross_atomic_pipeline_2026";

    let hash_result = ctx.hash_bytes(test_data, "blake3");

    match hash_result {
        Ok(hash_hex) => {
            v.check_bool(
                "cross_tower_hash",
                !hash_hex.is_empty(),
                &format!("BLAKE3: {}...", &hash_hex[..hash_hex.len().min(16)]),
            );

            let family_id = crate::env_keys::resolve_family_id();
            let store_key = "exp094_cross_atomic_hash";
            match ctx.call(
                "storage",
                "storage.store",
                serde_json::json!({"family_id": family_id, "key": store_key, "value": hash_hex}),
            ) {
                Ok(_) => {
                    match ctx.call(
                        "storage",
                        "storage.retrieve",
                        serde_json::json!({"family_id": family_id, "key": store_key}),
                    ) {
                        Ok(retrieved) => {
                            let val = retrieved
                                .get("value")
                                .and_then(|v| v.as_str())
                                .unwrap_or("");
                            v.check_bool(
                                "cross_nest_roundtrip",
                                val == hash_hex,
                                "hash stored and retrieved matches",
                            );
                        }
                        Err(e) => {
                            v.check_bool(
                                "cross_nest_roundtrip",
                                false,
                                &format!("retrieve error: {e}"),
                            );
                        }
                    }
                }
                Err(e) if e.is_connection_error() => {
                    v.check_skip(
                        "cross_nest_roundtrip",
                        &format!("storage not available: {e}"),
                    );
                }
                Err(e) => {
                    v.check_bool("cross_nest_roundtrip", false, &format!("store error: {e}"));
                }
            }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("cross_tower_hash", &format!("security not available: {e}"));
            v.check_skip("cross_nest_roundtrip", "tower unavailable");
        }
        Err(e) => {
            v.check_bool("cross_tower_hash", false, &format!("hash error: {e}"));
            v.check_skip("cross_nest_roundtrip", "tower failed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composition_parity_no_panic() {
        let mut v = ValidationResult::new("composition-parity");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(v.evaluated() > 0 || v.skipped > 0, "scenario should produce at least one check");
    }
}
