// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exp080: Cross-Spring Ecology Live

use std::path::PathBuf;

use primalspring::composition::CompositionContext;
use primalspring::ipc::NeuralBridge;
use primalspring::validation::ValidationResult;

fn discover_cross_spring_graph() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("BIOMEOS_GRAPHS_DIR") {
        let p = PathBuf::from(&dir).join("cross_spring_ecology.toml");
        if p.is_file() {
            return Some(p);
        }
    }

    let candidates = [
        "../../primals/biomeOS/graphs/cross_spring_ecology.toml",
        "../../../primals/biomeOS/graphs/cross_spring_ecology.toml",
    ];
    for c in &candidates {
        let p = PathBuf::from(c);
        if p.is_file() {
            return std::fs::canonicalize(&p).ok();
        }
    }
    None
}

fn phase_graph_structure(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Cross-spring graph structure");
    let graph_path = discover_cross_spring_graph();
    v.check_bool(
        "cross_spring_file_exists",
        graph_path.is_some(),
        "cross_spring_ecology.toml found on disk",
    );

    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "cross_spring_loaded",
            "orchestration capability not available — cannot validate graph loading",
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
                v.check_skip("cross_spring_loaded", &format!("{e}"));
                return;
            }
            Err(e) => {
                v.check_bool("graph_list", false, &format!("error: {e}"));
                return;
            }
        };

    let ecology = graphs
        .iter()
        .find(|g| g.get("id").and_then(|i| i.as_str()) == Some("cross_spring_ecology"));

    match ecology {
        Some(g) => {
            let node_count = g
                .get("node_count")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(0);
            v.check_minimum(
                "cross_spring_nodes",
                usize::try_from(node_count).unwrap_or(0),
                3,
            );

            let desc = g.get("description").and_then(|d| d.as_str()).unwrap_or("");
            let has_ecology_desc = desc.contains("ecology") || desc.contains("ET");
            v.check_bool(
                "cross_spring_description",
                has_ecology_desc,
                &format!("graph description: {desc}"),
            );
        }
        None => {
            v.check_bool(
                "cross_spring_loaded",
                false,
                "cross_spring_ecology not in graph.list",
            );
        }
    }
}

fn phase_live_routing(v: &mut ValidationResult) {
    v.section("Phase 2: Live capability routing");
    let Some(bridge) = NeuralBridge::discover() else {
        v.check_skip(
            "cross_spring_live",
            "biomeOS not running — live routing skipped",
        );
        return;
    };

    for domain in &["ecology", "science", "neural_science"] {
        let result = bridge.discover_capability(domain);
        let key = format!("live_{domain}");
        match result {
            Ok(val) => {
                let registered = val.get("primary_socket").is_some();
                if registered {
                    v.check_bool(&key, true, &format!("{domain} domain has live provider"));
                } else {
                    v.check_skip(
                        &key,
                        &format!("{domain} registered but no live provider socket"),
                    );
                }
            }
            Err(_) => v.check_skip(
                &key,
                &format!("{domain} not registered — spring primal not running"),
            ),
        }
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp080 — Cross-Spring Ecology Live")
        .with_provenance("exp080_cross_spring_ecology_live", "2026-05-09")
        .run(
            "primalSpring Exp080: Cross-spring ecology validation (structural + live routing)",
            |v| {
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_graph_structure(v, &mut ctx);
                phase_live_routing(v);
            },
        );
}
