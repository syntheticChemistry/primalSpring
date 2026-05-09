// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp080: Cross-Spring Ecology Live — validate cross-spring capability
//! routing through biomeOS substrate.
//!
//! When biomeOS is running with spring primals registered, this experiment
//! validates that the `cross_spring_ecology.toml` graph's capabilities
//! can be discovered and routed: airSpring (ecology), wetSpring (science),
//! and neuralSpring (`neural_science`) domains.
//!
//! Falls back to structural graph validation when spring primals are not
//! running.

use std::path::PathBuf;

use primalspring::ipc::NeuralBridge;
use primalspring::ipc::client::PrimalClient;
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

fn validate_graph_structure(v: &mut ValidationResult) {
    let graph_path = discover_cross_spring_graph();
    v.check_bool(
        "cross_spring_file_exists",
        graph_path.is_some(),
        "cross_spring_ecology.toml found on disk",
    );

    let Some(bridge) = NeuralBridge::discover() else {
        v.check_skip(
            "cross_spring_loaded",
            "biomeOS not running — cannot validate graph loading",
        );
        return;
    };

    let mut client = match PrimalClient::connect(bridge.socket_path(), "biomeos") {
        Ok(c) => c,
        Err(e) => {
            v.check_bool("biomeos_connect", false, &format!("{e}"));
            return;
        }
    };

    let resp = client.call("graph.list", serde_json::json!({}));
    let graphs: Vec<serde_json::Value> = match resp {
        Ok(r) => r
            .result
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default(),
        Err(e) => {
            v.check_bool("graph_list", false, &format!("{e}"));
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

fn validate_live_routing(v: &mut ValidationResult) {
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
        .with_provenance("exp080_cross_spring_ecology_live", "2026-03-27")
        .run(
            "primalSpring Exp080: Cross-spring ecology validation (structural + live routing)",
            |v| {
                validate_graph_structure(v);
                validate_live_routing(v);
            },
        );
}
