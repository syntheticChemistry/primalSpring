// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp079: Spring Deploy Sweep — validate biomeOS deploy graphs for all
//! sibling springs via two paths:
//!
//! 1. **Filesystem**: verify graph TOML files exist in biomeOS `graphs/`
//! 2. **Neural API**: query `graph.list` on running biomeOS to confirm
//!    each spring's graph is loaded and has valid metadata
//!
//! biomeOS graphs use their own schema (`id`, `[[nodes]]`) which differs
//! from primalSpring's internal `DeployGraph` schema (`name`, `[[graph.node]]`).
//! This experiment validates via the biomeOS runtime, not primalSpring parsing.

use std::path::{Path, PathBuf};

use primalspring::ipc::NeuralBridge;
use primalspring::ipc::client::PrimalClient;
use primalspring::validation::ValidationResult;

fn discover_biomeos_graphs_dir() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("BIOMEOS_GRAPHS_DIR") {
        let p = PathBuf::from(&dir);
        if p.is_dir() {
            return Some(p);
        }
    }

    let candidates = [
        PathBuf::from("../primals/biomeOS/graphs"),
        PathBuf::from("../../primals/biomeOS/graphs"),
        PathBuf::from("../../../primals/biomeOS/graphs"),
    ];
    for c in &candidates {
        if c.join("tower_atomic_bootstrap.toml").is_file() {
            return std::fs::canonicalize(c).ok();
        }
    }
    None
}

const SPRING_GRAPH_IDS: &[(&str, &str)] = &[
    ("airspring_deploy", "airspring_deploy.toml"),
    ("groundspring_deploy", "groundspring_deploy.toml"),
    ("healthspring_deploy", "healthspring_deploy.toml"),
    ("hotspring_deploy", "hotspring_deploy.toml"),
    ("ludospring_deploy", "ludospring_deploy.toml"),
    ("neuralspring_deploy", "neuralspring_deploy.toml"),
    ("wetspring_deploy", "wetspring_deploy.toml"),
];

const PIPELINE_GRAPH_IDS: &[(&str, &str)] = &[
    (
        "airspring_ecology_pipeline",
        "airspring_ecology_pipeline.toml",
    ),
    (
        "neuralspring_spectral_pipeline",
        "neuralspring_spectral_pipeline.toml",
    ),
    ("cross_spring_ecology", "cross_spring_ecology.toml"),
    (
        "cross_spring_soil_microbiome",
        "cross_spring_soil_microbiome.toml",
    ),
];

fn validate_files_exist(graphs_dir: &Path, v: &mut ValidationResult) {
    println!("\n  Filesystem check (biomeOS graphs/):");
    for &(id, filename) in SPRING_GRAPH_IDS.iter().chain(PIPELINE_GRAPH_IDS.iter()) {
        let path = graphs_dir.join(filename);
        v.check_bool(
            &format!("{id}_file"),
            path.exists(),
            &format!("{filename} exists"),
        );
    }
}

fn validate_via_neural_api(v: &mut ValidationResult) {
    let Some(bridge) = NeuralBridge::discover() else {
        v.check_skip(
            "neural_api_graphs",
            "biomeOS not running — graph.list skipped",
        );
        return;
    };

    let mut client = match PrimalClient::connect(bridge.socket_path(), "biomeos") {
        Ok(c) => c,
        Err(e) => {
            v.check_bool("neural_api_connect", false, &format!("{e}"));
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

    v.check_minimum("total_graphs_loaded", graphs.len(), 20);

    let loaded_ids: Vec<String> = graphs
        .iter()
        .filter_map(|g| g.get("id")?.as_str().map(String::from))
        .collect();

    println!("\n  Neural API graph.list validation:");
    for &(id, _) in SPRING_GRAPH_IDS.iter().chain(PIPELINE_GRAPH_IDS.iter()) {
        let found = loaded_ids.iter().any(|lid| lid == id);
        v.check_bool(
            &format!("{id}_loaded"),
            found,
            &format!("{id} in biomeOS graph.list"),
        );
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp079 — Spring Deploy Sweep")
        .with_provenance("exp079_spring_deploy_sweep", "2026-03-27")
        .run(
            "primalSpring Exp079: Validate spring deploy graphs (filesystem + Neural API)",
            |v| {
                let graphs_dir = discover_biomeos_graphs_dir();
                if let Some(ref dir) = graphs_dir {
                    v.check_bool(
                        "biomeos_graphs_dir",
                        true,
                        &format!("biomeOS graphs at {}", dir.display()),
                    );
                    validate_files_exist(dir, v);
                } else {
                    v.check_skip(
                        "biomeos_graphs_dir",
                        "biomeOS graphs/ not found — set BIOMEOS_GRAPHS_DIR",
                    );
                }

                validate_via_neural_api(v);
            },
        );
}
