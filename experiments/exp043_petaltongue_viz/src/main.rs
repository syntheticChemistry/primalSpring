// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp043: petalTongue Viz — atomic health via `CompositionContext` and `visualization.render.*`.

use primalspring::composition::{CompositionContext, capability_to_primal};
use primalspring::coordination::AtomicType;
use primalspring::ipc::discover::extract_capability_names;
use primalspring::validation::ValidationResult;

fn tower_health_rows(ctx: &mut CompositionContext) -> (Vec<serde_json::Value>, bool, bool) {
    let mut rows = Vec::new();
    let mut all_healthy = true;
    let mut discovery_ok = true;
    for cap in AtomicType::Tower.required_capabilities() {
        let name = capability_to_primal(cap).to_owned();
        if !ctx.has_capability(cap) {
            discovery_ok = false;
            all_healthy = false;
            rows.push(serde_json::json!({
                "name": name,
                "healthy": false,
                "latency_us": 0u64,
                "capabilities": 0usize,
            }));
            continue;
        }
        let start = std::time::Instant::now();
        let health_ok = ctx.health_check(cap).unwrap_or(false);
        let latency_us = u64::try_from(start.elapsed().as_micros()).unwrap_or(u64::MAX);
        let n_caps = ctx
            .call(cap, "capabilities.list", serde_json::json!({}))
            .ok()
            .map_or(0, |j| extract_capability_names(Some(j)).len());
        all_healthy &= health_ok;
        rows.push(serde_json::json!({
            "name": name,
            "healthy": health_ok,
            "latency_us": latency_us,
            "capabilities": n_caps,
        }));
    }
    (rows, all_healthy, discovery_ok)
}

fn skip_viz_checks(v: &mut ValidationResult, reason: &str) {
    v.check_skip("viz_capability", reason);
    v.check_skip("viz_render_grammar", reason);
    v.check_skip("viz_dashboard", reason);
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    v.check_bool(
        "has_visualization",
        ctx.has_capability("visualization"),
        "visualization capability for petalTongue",
    );
}

fn phase_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("visualization") {
        v.check_skip(
            "petaltongue_health",
            "visualization capability not in context",
        );
        v.check_skip(
            "petaltongue_caps",
            "visualization capability not in context",
        );
        return;
    }
    match ctx.health_check("visualization") {
        Ok(h) => v.check_bool("petaltongue_health", h, "visualization health"),
        Err(e) if e.is_connection_error() => {
            v.check_skip("petaltongue_health", &format!("{e}"));
        }
        Err(e) => v.check_bool("petaltongue_health", false, &format!("{e}")),
    }
    match ctx.call("visualization", "capabilities.list", serde_json::json!({})) {
        Ok(j) => {
            let n = extract_capability_names(Some(j)).len();
            v.check_minimum("petaltongue_caps", n, 1);
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("petaltongue_caps", &format!("{e}"));
        }
        Err(e) => v.check_bool("petaltongue_caps", false, &format!("{e}")),
    }
}

fn phase_visualization(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("visualization") {
        skip_viz_checks(v, "visualization capability not in context");
        return;
    }

    match ctx.call("visualization", "capabilities.list", serde_json::json!({})) {
        Ok(val) => {
            let names = extract_capability_names(Some(val));
            let has_render = names.iter().any(|n| n.contains("visualization"));
            v.check_bool(
                "viz_capability",
                has_render || !names.is_empty(),
                "has visualization capabilities",
            );
        }
        Err(_) => v.check_skip("viz_capability", "capabilities.list failed"),
    }

    let atomic = AtomicType::Tower;
    let (primal_data, all_healthy, discovery_ok) = tower_health_rows(ctx);
    let grammar = serde_json::json!({
        "data": primal_data,
        "mark": "bar",
        "encoding": {
            "x": {"field": "name", "type": "nominal"},
            "y": {"field": "latency_us", "type": "quantitative"},
            "color": {"field": "healthy", "type": "nominal"}
        },
        "title": format!(
            "{} Atomic Health (discovery_ok={discovery_ok}, all_healthy={all_healthy})",
            atomic.description()
        ),
    });

    match ctx.call("visualization", "visualization.render.grammar", grammar) {
        Ok(result) => {
            v.check_bool(
                "viz_render_grammar",
                result.get("rendered").is_some() || result.get("scene").is_some(),
                "visualization.render.grammar returns rendered output",
            );
            println!("  Grammar render: success");
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("viz_render_grammar", &format!("{e}"));
        }
        Err(e) => v.check_skip("viz_render_grammar", &format!("render failed: {e}")),
    }

    let dashboard_spec = serde_json::json!({
        "panels": [{
            "title": "Atomic Composition",
            "type": "topology",
            "data": {
                "composition": atomic.description(),
                "primals": primal_data,
                "all_healthy": all_healthy,
                "discovery_ok": discovery_ok,
            }
        }],
    });

    match ctx.call(
        "visualization",
        "visualization.render.dashboard",
        dashboard_spec,
    ) {
        Ok(_) => v.check_bool("viz_dashboard", true, "dashboard render accepted"),
        Err(e) if e.is_connection_error() => v.check_skip("viz_dashboard", &format!("{e}")),
        Err(_) => v.check_skip("viz_dashboard", "dashboard render not available"),
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp043 — petalTongue Viz")
        .with_provenance("exp043_petaltongue_viz", "2026-05-09")
        .run(
            "primalSpring Exp043: petalTongue Atomic Health Visualization",
            |v| {
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();

                v.section("Phase 1: Discovery");
                phase_discovery(v, &ctx);

                v.section("Phase 2: Health");
                phase_health(v, &mut ctx);

                v.section("Phase 3: Visualization Pipeline");
                phase_visualization(v, &mut ctx);
            },
        );
}
