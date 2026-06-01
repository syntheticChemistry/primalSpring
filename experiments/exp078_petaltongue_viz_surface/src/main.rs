// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp078: petalTongue Viz Surface

use primalspring::composition::CompositionContext;
use primalspring::ipc::NeuralBridge;
use primalspring::ipc::methods;
use primalspring::validation::ValidationResult;

fn phase_petaltongue_direct(v: &mut ValidationResult, ctx: &mut CompositionContext) -> bool {
    v.section("Phase 1: petalTongue direct");
    if !ctx.has_capability("visualization") {
        v.check_skip(
            "petaltongue_direct",
            "visualization capability not discovered — start petaltongue server",
        );
        return false;
    }

    let health = ctx.health_check("visualization").unwrap_or(false);
    v.check_bool("petaltongue_health", health, "petalTongue health check");

    match ctx.call(
        "visualization",
        methods::capabilities::LIST,
        serde_json::json!({}),
    ) {
        Ok(_) => v.check_bool(
            "petaltongue_capabilities",
            true,
            "petalTongue capabilities.list",
        ),
        Err(e) if e.is_connection_error() => {
            v.check_skip("petaltongue_capabilities", &format!("{e}"));
        }
        Err(e) => v.check_bool("petaltongue_capabilities", false, &format!("error: {e}")),
    }

    health
}

fn phase_viz_domains(v: &mut ValidationResult) {
    v.section("Phase 2: Visualization domains (Neural API)");
    let Some(bridge) = NeuralBridge::discover() else {
        v.check_skip(
            "viz_biomeos_routing",
            "biomeOS not running — visualization routing not tested",
        );
        return;
    };

    for domain in &["visualization", "ui", "interaction"] {
        let result = bridge.discover_capability(domain);
        let key = format!("{domain}_domain");
        match result {
            Ok(val) => {
                let registered = val.get("primary_socket").is_some();
                v.check_bool(
                    &key,
                    registered,
                    &format!("{domain} domain registered in biomeOS"),
                );
            }
            Err(_) => v.check_skip(&key, &format!("{domain} domain not yet registered")),
        }
    }
}

fn phase_tower_ai_viz_graph(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 3: Tower AI viz graph");
    let graph_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../graphs/tower_ai_viz.toml");

    v.check_bool(
        "tower_ai_viz_file",
        graph_path.exists(),
        "graphs/tower_ai_viz.toml exists",
    );

    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "tower_ai_viz_loaded",
            "orchestration capability missing — biomeOS graph list skipped",
        );
        return;
    }

    let graphs: Vec<serde_json::Value> =
        match ctx.call("orchestration", "graph.list", serde_json::json!({})) {
            Ok(v) => match v {
                serde_json::Value::Array(a) => a,
                v => serde_json::from_value(v).unwrap_or_default(),
            },
            Err(e) if e.is_connection_error() => {
                v.check_skip("tower_ai_viz_loaded", &format!("{e}"));
                return;
            }
            Err(e) => {
                v.check_bool("graph_list", false, &format!("error: {e}"));
                return;
            }
        };

    let ui_graph = graphs
        .iter()
        .find(|g| g.get("id").and_then(|i| i.as_str()) == Some("ui_atomic"));
    v.check_bool(
        "tower_ai_viz_loaded",
        ui_graph.is_some(),
        "ui_atomic graph loaded in biomeOS",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp078 — petalTongue Viz Surface")
        .with_provenance("exp078_petaltongue_viz_surface", "2026-05-09")
        .run(
            "primalSpring Exp078: Universal UI primal integration with biomeOS substrate",
            |v| {
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                let live = phase_petaltongue_direct(v, &mut ctx);
                if live {
                    phase_viz_domains(v);
                }
                phase_tower_ai_viz_graph(v, &mut ctx);
            },
        );
}
