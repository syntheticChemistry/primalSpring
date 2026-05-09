// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exp068: Full NUCLEUS — live capabilities via CompositionContext.

use primalspring::composition::CompositionContext;
use primalspring::coordination::AtomicType;
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

fn phase_structural(v: &mut ValidationResult) {
    let primals = AtomicType::FullNucleus.required_primals();
    v.check_minimum("nucleus_required_primals", primals.len(), 5);
}

fn phase_tower_slice(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    println!("\n=== Tower Atomic slice ===");
    for cap in AtomicType::Tower.required_capabilities() {
        if !ctx.has_capability(cap) {
            v.check_skip(
                &format!("tower_health_{cap}"),
                &format!("{cap} not discovered"),
            );
            continue;
        }
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(_) => v.check_bool(&format!("tower_live_{cap}"), true, &format!("{cap} live")),
            Err(e) if e.is_connection_error() => {
                v.check_skip(&format!("tower_live_{cap}"), &format!("{e}"));
            }
            Err(e) => v.check_bool(&format!("tower_live_{cap}"), false, &format!("error: {e}")),
        }
    }
}

fn phase_nest_slice(v: &mut ValidationResult, ctx: &mut CompositionContext, nest_fam: &str) {
    println!("\n=== Nest slice ===");
    if !ctx.has_capability("storage") {
        v.check_skip("nest_store", "storage capability not discovered");
        return;
    }
    let store = ctx.call(
        "storage",
        "storage.store",
        serde_json::json!({
            "family_id": nest_fam,
            "key": "nucleus_test",
            "data": {"source": "exp068"}
        }),
    );
    v.check_bool("nest_store", store.is_ok(), "nestgate store");
}

fn phase_node_slice(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    println!("\n=== Node slice ===");
    if !ctx.has_capability("compute") {
        v.check_skip("node_compute_caps", "compute capability not discovered");
        return;
    }
    let caps = ctx.call(
        "compute",
        "toadstool.query_capabilities",
        serde_json::json!({}),
    );
    v.check_bool("node_compute_caps", caps.is_ok(), "toadstool caps");
}

fn phase_nest_live(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    for cap in AtomicType::Nest.required_capabilities() {
        if !ctx.has_capability(cap) {
            v.check_skip(
                &format!("nest_{cap}_live"),
                &format!("{cap} not discovered"),
            );
            continue;
        }
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(_) => v.check_bool(&format!("nest_{cap}_live"), true, &format!("{cap} live")),
            Err(e) if e.is_connection_error() => {
                v.check_skip(&format!("nest_{cap}_live"), &format!("{e}"));
            }
            Err(e) => v.check_bool(&format!("nest_{cap}_live"), false, &format!("error: {e}")),
        }
    }
}

fn phase_node_live(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    for cap in AtomicType::Node.required_capabilities() {
        if !ctx.has_capability(cap) {
            v.check_skip(
                &format!("node_{cap}_live"),
                &format!("{cap} not discovered"),
            );
            continue;
        }
        let label = match *cap {
            "security" => primal_names::BEARDOG,
            "discovery" => primal_names::SONGBIRD,
            "compute" => primal_names::TOADSTOOL,
            "tensor" => primal_names::BARRACUDA,
            "shader" => primal_names::CORALREEF,
            _ => *cap,
        };
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(_) => v.check_bool(
                &format!("node_{label}_live"),
                true,
                &format!("{label} live"),
            ),
            Err(e) if e.is_connection_error() => {
                v.check_skip(&format!("node_{label}_live"), &format!("{e}"));
            }
            Err(e) => v.check_bool(&format!("node_{label}_live"), false, &format!("error: {e}")),
        }
    }
}

fn main() {
    ValidationResult::new("exp068_full_nucleus")
        .with_provenance("exp068_full_nucleus", "2026-05-09")
        .run("Full NUCLEUS — all primals composed", |v| {
            v.section("Phase 1: Structural");
            phase_structural(v);

            v.section("Phase 2: Live composition");
            let mut ctx = CompositionContext::from_live_discovery_with_fallback();

            phase_tower_slice(v, &mut ctx);

            let nest_fam = format!("exp068n-{}", std::process::id());
            phase_nest_slice(v, &mut ctx, &nest_fam);
            phase_node_slice(v, &mut ctx);

            phase_nest_live(v, &mut ctx);
            phase_node_live(v, &mut ctx);

            let btsp = ctx.btsp_state();
            let btsp_count = btsp.values().filter(|&&ok| ok).count();
            v.check_bool(
                "btsp_any_authenticated",
                btsp_count > 0 || btsp.is_empty(),
                &format!("{btsp_count}/{} BTSP authenticated", btsp.len()),
            );
        });
}
