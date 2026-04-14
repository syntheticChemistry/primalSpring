// SPDX-License-Identifier: AGPL-3.0-or-later

//! exp095 — Proto-Nucleate Parity Template
//!
//! Starter experiment for downstream springs to validate their domain science
//! as primal compositions. Copy this crate, rename, and replace the niche
//! section with your domain-specific parity checks.
//!
//! This template demonstrates:
//!   - NUCLEUS base validation (Tower alive, Node compute, Nest storage)
//!   - Scalar parity via `validate_parity` (local baseline vs primal result)
//!   - Vector parity via `validate_parity_vec`
//!   - Graceful skip when primals aren't running
//!   - biomeOS gateway routing for Docker/benchScale mode
//!
//! Environment:
//!   `REMOTE_GATE_HOST` — enables TCP/gateway mode (e.g. Docker lab)
//!   `BIOMEOS_PORT`     — biomeOS TCP port (default 9800)
//!   `FAMILY_ID`        — primal family for socket scoping

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
                let val = r
                    .get("value")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
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

/// Replace this with your spring's domain-specific parity checks.
///
/// Pattern: for each domain operation, compute the expected result locally
/// (your Rust baseline, matching your Python baseline), then compare against
/// the primal composition result via `validate_parity` or `validate_parity_vec`.
fn niche_parity(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    v.section("Niche — Domain Parity (replace with your science)");

    // Example: scalar parity — Python: np.mean([1, 2, 3, 4, 5]) = 3.0
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

    // barraCuda stats.std_dev uses ddof=1 (sample std dev)
    // Python: np.std([2, 4, 4, 4, 5, 5, 7, 9], ddof=1) ≈ 2.1381
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

    // Example: cross-atomic — hash data with Tower, store with Nest
    match ctx.call(
        "security",
        "crypto.hash",
        serde_json::json!({"data": "niche domain payload", "algorithm": "blake3"}),
    ) {
        Ok(r) => {
            let hash = r
                .get("hash")
                .and_then(|h| h.as_str())
                .unwrap_or_default();
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
    ValidationResult::new("Proto-Nucleate Parity Template")
        .with_provenance("exp095_proto_nucleate_template", "2026-04-09")
        .run(
            "NUCLEUS base + niche domain parity (template — replace niche section)",
            |v| {
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                let caps = ctx.available_capabilities();

                v.section("Discovery");
                v.check_bool(
                    "capabilities_found",
                    !caps.is_empty(),
                    &format!("discovered {} capabilities: {}", caps.len(), caps.join(", ")),
                );

                nucleus_base(&mut ctx, v);
                niche_parity(&mut ctx, v);
            },
        );
}
