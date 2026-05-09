// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exp065: petalTongue Tower Dashboard — visualization via CompositionContext.

use primalspring::composition::CompositionContext;
use primalspring::ipc::discover::extract_capability_names;
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

fn phase_petaltongue_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("visualization") {
        v.check_skip(
            "petaltongue_healthy",
            "visualization capability not in context",
        );
        return;
    }
    match ctx.call("visualization", "health.liveness", serde_json::json!({})) {
        Ok(_) => v.check_bool("petaltongue_healthy", true, "petalTongue health OK"),
        Err(e) if e.is_connection_error() => {
            v.check_skip("petaltongue_healthy", &format!("{e}"));
        }
        Err(e) => v.check_bool("petaltongue_healthy", false, &format!("health: {e}")),
    }
}

fn phase_dashboard(v: &mut ValidationResult, ctx: &mut CompositionContext, family_id: &str) {
    if !ctx.has_capability("visualization") {
        v.check_skip(
            "dashboard_rendered",
            "visualization capability not in context",
        );
        v.check_skip(
            "grammar_rendered",
            "visualization capability not in context",
        );
        return;
    }

    let dashboard = ctx.call(
        "visualization",
        "visualization.render.dashboard",
        serde_json::json!({
            "session_id": family_id,
            "title": "Tower Atomic Health",
            "bindings": [{
                "channel_type": "bar",
                "id": "primal_status",
                "label": "Primal Status",
                "x_label": "Primal",
                "y_label": "Health",
                "categories": [primal_names::BEARDOG, primal_names::SONGBIRD],
                "values": [1.0, 1.0],
                "unit": "status"
            }],
            "modality": "description"
        }),
    );

    match &dashboard {
        Ok(resp) => {
            println!("  dashboard render: {} bytes", resp.to_string().len());
            v.check_bool(
                "dashboard_rendered",
                true,
                "Tower health dashboard rendered",
            );
        }
        Err(e) => {
            println!("  dashboard: {e}");
            v.check_bool("dashboard_rendered", false, &format!("dashboard: {e}"));
        }
    }

    let grammar = ctx.call(
        "visualization",
        "visualization.render.grammar",
        serde_json::json!({
            "session_id": family_id,
            "grammar": {
                "data_source": "tower_health",
                "variables": [
                    { "name": "x", "role": "X", "field": "primal" },
                    { "name": "y", "role": "Y", "field": "status" }
                ],
                "geometry": "Bar",
                "scales": [],
                "coordinate": "Cartesian",
                "facets": null,
                "aesthetics": [],
                "title": "Tower Health",
                "domain": "health"
            },
            "data": [
                { "primal": primal_names::BEARDOG, "status": 1 },
                { "primal": primal_names::SONGBIRD, "status": 1 }
            ],
            "modality": "description"
        }),
    );

    match &grammar {
        Ok(resp) => {
            let is_svg = resp.as_str().is_some_and(|s| s.contains("<svg"));
            let is_json = resp.is_object() || resp.is_array();
            println!(
                "  grammar render: {} bytes (svg={is_svg}, json={is_json})",
                resp.to_string().len()
            );
            v.check_bool(
                "grammar_rendered",
                true,
                "Grammar of Graphics expression rendered",
            );
        }
        Err(e) => {
            println!("  grammar: {e}");
            v.check_bool("grammar_rendered", false, &format!("grammar: {e}"));
        }
    }

    match ctx.call("visualization", "capabilities.list", serde_json::json!({})) {
        Ok(j) => {
            let n = extract_capability_names(Some(j)).len();
            v.check_minimum("petaltongue_caps", n, 1);
        }
        Err(_) => v.check_skip("petaltongue_caps", "capabilities.list failed"),
    }
}

fn main() {
    let family_id = format!("e065-{}", std::process::id());

    ValidationResult::new("primalSpring Exp065 — petalTongue Tower Dashboard")
        .with_provenance("exp065_petaltongue_tower_dashboard", "2026-05-09")
        .run(
            "primalSpring Exp065: Tower health visualization via petalTongue",
            |v| {
                v.section("Phase 1: petalTongue health");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_petaltongue_health(v, &mut ctx);

                v.section("Phase 2: Dashboard & grammar");
                phase_dashboard(v, &mut ctx, &family_id);
            },
        );
}
