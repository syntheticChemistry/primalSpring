// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp095: Proto-Nucleate Parity Template

use primalspring::composition::{CompositionContext, validate_parity};
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

fn nucleus_base(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    v.section("Tower (Security + Discovery)");
    match ctx.health_check("security") {
        Ok(alive) => v.check_bool("tower_security_alive", alive, "BearDog health"),
        Err(e) if e.is_connection_error() => {
            v.check_skip("tower_security_alive", &format!("{e}"));
        }
        Err(e) => v.check_bool("tower_security_alive", false, &format!("{e}")),
    }

    v.section("Node (Compute Triangle)");
    validate_parity(
        ctx,
        v,
        "mean_3elem",
        "tensor",
        "stats.mean",
        serde_json::json!({"data": [2.0, 4.0, 6.0]}),
        "result",
        4.0,
        tolerances::EXACT_PARITY_TOL,
    );

    v.section("Nest (Storage Round-Trip)");
    let store_ok = ctx
        .call(
            "storage",
            "storage.put",
            serde_json::json!({"key": "exp095_test", "value": "proto_nucleate"}),
        )
        .is_ok();
    if store_ok {
        match ctx.call(
            "storage",
            "storage.get",
            serde_json::json!({"key": "exp095_test"}),
        ) {
            Ok(r) => {
                let val = r.get("value").and_then(|v| v.as_str()).unwrap_or_default();
                v.check_bool(
                    "storage_roundtrip",
                    val == "proto_nucleate",
                    &format!("stored='proto_nucleate', retrieved='{val}'"),
                );
            }
            Err(e) => v.check_skip("storage_roundtrip", &format!("{e}")),
        }
    } else {
        v.check_skip("storage_roundtrip", "storage.put not available");
    }
}

fn niche_parity(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    v.section("Niche — Domain Parity (replace with your science)");

    validate_parity(
        ctx,
        v,
        "example_mean_5elem",
        "tensor",
        "stats.mean",
        serde_json::json!({"data": [1.0, 2.0, 3.0, 4.0, 5.0]}),
        "result",
        3.0,
        tolerances::EXACT_PARITY_TOL,
    );

    validate_parity(
        ctx,
        v,
        "example_std_dev_8elem",
        "tensor",
        "stats.std_dev",
        serde_json::json!({"data": [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]}),
        "result",
        2.138_089_935_299_395,
        1e-6,
    );

    match ctx.call(
        "security",
        "crypto.hash",
        serde_json::json!({"data": "niche domain payload", "algorithm": "blake3"}),
    ) {
        Ok(r) => {
            let hash = r.get("hash").and_then(|h| h.as_str()).unwrap_or_default();
            v.check_bool(
                "cross_atomic_hash",
                !hash.is_empty(),
                &format!("hash length: {}", hash.len()),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("cross_atomic_hash", &format!("{e}"));
        }
        Err(e) => v.check_bool("cross_atomic_hash", false, &format!("{e}")),
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp095 — Proto-Nucleate Parity Template")
        .with_provenance("exp095_proto_nucleate_template", "2026-05-09")
        .run(
            "NUCLEUS base + niche domain parity (template — replace niche section)",
            |v| {
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                let caps = ctx.available_capabilities();

                v.section("Phase 1: Discovery");
                v.check_bool(
                    "capabilities_found",
                    !caps.is_empty(),
                    &format!(
                        "discovered {} capabilities: {}",
                        caps.len(),
                        caps.join(", ")
                    ),
                );

                v.section("Phase 2: NUCLEUS base");
                nucleus_base(&mut ctx, v);
                v.section("Phase 3: Niche domain parity");
                niche_parity(&mut ctx, v);
            },
        );
}
