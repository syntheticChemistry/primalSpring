// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exp066: Nest Atomic — NestGate storage via CompositionContext.

use primalspring::composition::CompositionContext;
use primalspring::coordination::AtomicType;
use primalspring::validation::ValidationResult;

fn phase_structural(v: &mut ValidationResult) {
    let primals = AtomicType::Nest.required_primals();
    v.check_bool(
        "nest_composition_valid",
        primals.len() == 4,
        "Nest = beardog + songbird + nestgate + squirrel",
    );
}

fn phase_nest_capabilities(v: &mut ValidationResult, ctx: &CompositionContext) {
    for cap in AtomicType::Nest.required_capabilities() {
        v.check_bool(
            &format!("has_{cap}"),
            ctx.has_capability(cap),
            &format!("{cap} capability resolved"),
        );
    }
}

fn phase_storage(v: &mut ValidationResult, ctx: &mut CompositionContext, family_id: &str) {
    if !ctx.has_capability("storage") {
        v.check_skip("nestgate_store", "storage capability not discovered");
        v.check_skip("nestgate_retrieve", "storage capability not discovered");
        v.check_skip(
            "nestgate_data_integrity",
            "storage capability not discovered",
        );
        v.check_skip("nestgate_health", "storage capability not discovered");
        v.check_skip("nestgate_caps", "storage capability not discovered");
        return;
    }

    let store = ctx.call(
        "storage",
        "storage.store",
        serde_json::json!({
            "family_id": family_id,
            "key": "exp066_test",
            "data": {"experiment": "nest_atomic", "timestamp": "2026-03-22"}
        }),
    );
    v.check_bool("nestgate_store", store.is_ok(), "storage.store");

    let retrieve = ctx.call(
        "storage",
        "storage.retrieve",
        serde_json::json!({
            "family_id": family_id, "key": "exp066_test"
        }),
    );
    v.check_bool("nestgate_retrieve", retrieve.is_ok(), "storage.retrieve");

    if let Ok(val) = &retrieve {
        v.check_bool(
            "nestgate_data_integrity",
            val.get("data")
                .and_then(|d| d.get("experiment"))
                .and_then(|e| e.as_str())
                == Some("nest_atomic"),
            "data round-trip integrity",
        );
    }

    match ctx.call("storage", "health.liveness", serde_json::json!({})) {
        Ok(_) => v.check_bool("nestgate_health", true, "nestgate health.liveness"),
        Err(e) if e.is_connection_error() => {
            v.check_skip("nestgate_health", &format!("{e}"));
        }
        Err(e) => v.check_bool("nestgate_health", false, &format!("error: {e}")),
    }

    match ctx.call("storage", "capabilities.list", serde_json::json!({})) {
        Ok(_) => v.check_bool("nestgate_caps", true, "nestgate capabilities.list"),
        Err(e) if e.is_connection_error() => {
            v.check_skip("nestgate_caps", &format!("{e}"));
        }
        Err(e) => v.check_bool("nestgate_caps", false, &format!("error: {e}")),
    }
}

fn main() {
    ValidationResult::new("exp066_nest_atomic")
        .with_provenance("exp066_nest_atomic", "2026-05-09")
        .run("Nest Atomic — Tower + NestGate + Squirrel", |v| {
            v.section("Phase 1: Structural");
            phase_structural(v);

            v.section("Phase 2: Discovery");
            let mut ctx = CompositionContext::from_live_discovery_with_fallback();
            phase_nest_capabilities(v, &ctx);

            v.section("Phase 3: NestGate storage");
            let family_id = format!("exp066-{}", std::process::id());
            phase_storage(v, &mut ctx, &family_id);
        });
}
