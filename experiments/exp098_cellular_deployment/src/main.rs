// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]
//! exp098 — Cellular Deployment Validation
//!
//! Validates that every spring cell graph in `graphs/cells/` is structurally
//! sound and deployable via biomeOS:
//!
//! 1. STRUCTURAL: each cell TOML parses, has NUCLEUS base, domain overlay,
//!    petalTongue in `live` mode, and a validation health check node
//! 2. MANIFEST: cells_manifest.toml indexes all cells correctly
//! 3. CAPABILITY: each cell declares capabilities matching the spring's
//!    validation_capabilities from downstream_manifest.toml
//! 4. LIVE (if biomeOS running): graph.execute succeeds for each cell
//!
//! Exit 0 = all cells valid, 1 = failures, 2 = structural-only (no biomeOS)

use primalspring::validation::ValidationResult;
use std::path::{Path, PathBuf};

const CELLS_DIR: &str = "graphs/cells";
const CELLS_MANIFEST: &str = "graphs/cells/cells_manifest.toml";

const EXPECTED_CELLS: &[&str] = &[
    "hotspring_cell.toml",
    "wetspring_cell.toml",
    "neuralspring_cell.toml",
    "ludospring_cell.toml",
    "airspring_cell.toml",
    "groundspring_cell.toml",
    "healthspring_cell.toml",
    "esotericwebb_cell.toml",
];

const NUCLEUS_REQUIRED: &[&str] = &["beardog", "songbird"];

fn main() {
    ValidationResult::new("primalSpring Exp098 — Cellular Deployment")
        .with_provenance("exp098_cellular_deployment", "2026-04-21")
        .run("Exp098: Cell Graph Validation (8 springs)", |v| {
            let base = find_project_root();

            v.section("Manifest");
            validate_manifest(v, &base);

            v.section("Structural");
            for cell_file in EXPECTED_CELLS {
                validate_cell_structure(v, &base, cell_file);
            }

            v.section("Capability Coverage");
            validate_capability_coverage(v, &base);

            v.section("biomeOS Deployment");
            validate_biomeos_deployment(v, &base);
        });
}

fn find_project_root() -> PathBuf {
    let mut dir = std::env::current_dir().unwrap_or_default();
    for _ in 0..5 {
        if dir.join(CELLS_DIR).is_dir() {
            return dir;
        }
        if !dir.pop() {
            break;
        }
    }
    std::env::current_dir().unwrap_or_default()
}

fn validate_manifest(v: &mut ValidationResult, base: &Path) {
    let manifest_path = base.join(CELLS_MANIFEST);
    let exists = manifest_path.is_file();
    v.check_bool(
        "cells_manifest_exists",
        exists,
        &format!("{}", manifest_path.display()),
    );
    if !exists {
        return;
    }

    let content = std::fs::read_to_string(&manifest_path).unwrap_or_default();
    let parsed: Result<toml::Value, _> = content.parse();
    v.check_bool(
        "cells_manifest_parses",
        parsed.is_ok(),
        "cells_manifest.toml is valid TOML",
    );

    if let Ok(val) = parsed {
        let cells = val
            .get("cell")
            .and_then(|c| c.as_array())
            .map(|a| a.len())
            .unwrap_or(0);
        v.check_bool(
            "cells_manifest_count",
            cells == EXPECTED_CELLS.len(),
            &format!("{cells} cells indexed (expected {})", EXPECTED_CELLS.len()),
        );

        if let Some(arr) = val.get("cell").and_then(|c| c.as_array()) {
            let all_live = arr.iter().all(|c| {
                c.get("petaltongue_mode")
                    .and_then(|m| m.as_str())
                    == Some("live")
            });
            v.check_bool(
                "cells_manifest_all_live_mode",
                all_live,
                "all cells declare petaltongue_mode = live",
            );
        }
    }
}

fn validate_cell_structure(v: &mut ValidationResult, base: &Path, cell_file: &str) {
    let path = base.join(CELLS_DIR).join(cell_file);
    let stem = cell_file.trim_end_matches("_cell.toml");

    let exists = path.is_file();
    v.check_bool(
        &format!("{stem}_cell_exists"),
        exists,
        &format!("{}", path.display()),
    );
    if !exists {
        return;
    }

    let content = std::fs::read_to_string(&path).unwrap_or_default();
    let parsed: Result<toml::Value, _> = content.parse();
    let parses = parsed.is_ok();
    v.check_bool(
        &format!("{stem}_cell_parses"),
        parses,
        &format!("{cell_file} is valid TOML"),
    );
    let Some(val) = parsed.ok() else { return };

    // Check graph metadata
    let has_graph = val.get("graph").is_some();
    v.check_bool(
        &format!("{stem}_has_graph"),
        has_graph,
        "[graph] section present",
    );

    let meta = val
        .get("graph")
        .and_then(|g| g.get("metadata"));
    let pt_mode = meta
        .and_then(|m| m.get("petaltongue_mode"))
        .and_then(|v| v.as_str());
    v.check_bool(
        &format!("{stem}_metadata_live"),
        pt_mode == Some("live"),
        &format!("petaltongue_mode = {:?}", pt_mode.unwrap_or("MISSING")),
    );

    // Check nodes
    let nodes = val
        .get("graph")
        .and_then(|g| g.get("nodes"))
        .and_then(|n| n.as_array())
        .cloned()
        .unwrap_or_default();

    let node_names: Vec<String> = nodes
        .iter()
        .filter_map(|n| n.get("name").and_then(|v| v.as_str()).map(String::from))
        .collect();

    // NUCLEUS base present
    for required in NUCLEUS_REQUIRED {
        v.check_bool(
            &format!("{stem}_has_{required}"),
            node_names.iter().any(|n| n == required),
            &format!("{required} node present"),
        );
    }

    // petalTongue node exists with live mode params
    let pt_node = nodes.iter().find(|n| {
        n.get("name").and_then(|v| v.as_str()) == Some("petaltongue")
    });
    let pt_live = pt_node
        .and_then(|n| n.get("params"))
        .and_then(|p| p.get("mode"))
        .and_then(|m| m.as_str())
        == Some("live");
    v.check_bool(
        &format!("{stem}_petaltongue_live"),
        pt_live,
        "petaltongue node params.mode = live",
    );

    // Validation health check node exists
    let has_validate = node_names.iter().any(|n| n.starts_with("validate"));
    v.check_bool(
        &format!("{stem}_has_validate_node"),
        has_validate,
        "validation health_check node present",
    );

    // Domain overlay node exists (not a base primal)
    let base_primals = [
        "biomeos_neural_api", "beardog", "songbird", "toadstool", "barracuda",
        "coralreef", "nestgate", "rhizocrypt", "loamspine", "sweetgrass",
        "squirrel", "petaltongue",
    ];
    let domain_nodes: Vec<&String> = node_names
        .iter()
        .filter(|n| {
            !base_primals.contains(&n.as_str())
                && !n.starts_with("validate")
        })
        .collect();
    v.check_bool(
        &format!("{stem}_has_domain_overlay"),
        !domain_nodes.is_empty(),
        &format!("domain node(s): {}", domain_nodes.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")),
    );
}

fn validate_capability_coverage(v: &mut ValidationResult, base: &Path) {
    let downstream_path = base.join("graphs/downstream/downstream_manifest.toml");
    if !downstream_path.is_file() {
        v.check_skip("capability_coverage", "downstream_manifest.toml not found");
        return;
    }

    let content = std::fs::read_to_string(&downstream_path).unwrap_or_default();
    let parsed: Result<toml::Value, _> = content.parse();
    let Some(dm) = parsed.ok() else {
        v.check_skip("capability_coverage", "downstream_manifest.toml parse failed");
        return;
    };

    let downstreams = dm
        .get("downstream")
        .and_then(|d| d.as_array())
        .cloned()
        .unwrap_or_default();

    for ds in &downstreams {
        let Some(spring_name) = ds.get("spring_name").and_then(|v| v.as_str()) else {
            continue;
        };
        let cell_file = format!("{spring_name}_cell.toml");
        let cell_path = base.join(CELLS_DIR).join(&cell_file);
        if !cell_path.is_file() {
            continue;
        }

        let cell_content = std::fs::read_to_string(&cell_path).unwrap_or_default();
        let cell_parsed: Result<toml::Value, _> = cell_content.parse();
        let Some(cell_val) = cell_parsed.ok() else { continue };

        let cell_caps: Vec<String> = cell_val
            .get("graph")
            .and_then(|g| g.get("nodes"))
            .and_then(|n| n.as_array())
            .map(|nodes| {
                nodes
                    .iter()
                    .flat_map(|n| {
                        n.get("capabilities")
                            .and_then(|c| c.as_array())
                            .into_iter()
                            .flatten()
                            .filter_map(|c| c.as_str().map(String::from))
                    })
                    .collect()
            })
            .unwrap_or_default();

        let required_caps: Vec<String> = ds
            .get("validation_capabilities")
            .and_then(|c| c.as_array())
            .map(|a| a.iter().filter_map(|c| c.as_str().map(String::from)).collect())
            .unwrap_or_default();

        let missing: Vec<&String> = required_caps
            .iter()
            .filter(|cap| !cell_caps.contains(cap))
            .collect();

        let detail = if missing.is_empty() {
            format!("{}/{} validation capabilities covered", required_caps.len(), required_caps.len())
        } else {
            format!(
                "{}/{} covered, missing: {}",
                required_caps.len() - missing.len(),
                required_caps.len(),
                missing.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
            )
        };
        v.check_bool(
            &format!("{spring_name}_cap_coverage"),
            missing.is_empty(),
            &detail,
        );
    }
}

fn validate_biomeos_deployment(v: &mut ValidationResult, base: &Path) {
    let mut ctx = primalspring::composition::CompositionContext::from_live_discovery_with_fallback();
    let caps = ctx.available_capabilities();

    if caps.is_empty() {
        v.check_skip(
            "biomeos_live",
            "no live primals discovered — structural validation only",
        );
        return;
    }

    let has_orchestration = caps.iter().any(|c| c.contains("graph") || c.contains("orchestration"));
    if !has_orchestration {
        v.check_skip(
            "biomeos_graph_execute",
            "biomeOS neural-api not detected — cannot submit graphs",
        );
        return;
    }

    v.check_bool(
        "biomeos_live",
        true,
        &format!("biomeOS detected with {} capabilities", caps.len()),
    );

    for cell_file in EXPECTED_CELLS {
        let stem = cell_file.trim_end_matches("_cell.toml");
        let cell_path = base.join(CELLS_DIR).join(cell_file);
        if !cell_path.is_file() {
            v.check_skip(
                &format!("{stem}_deploy"),
                &format!("{cell_file} not found"),
            );
            continue;
        }

        let graph_result = ctx.call(
            "orchestration",
            "graph.execute",
            serde_json::json!({
                "graph_path": cell_path.to_string_lossy(),
                "validate_only": true,
                "dry_run": true,
            }),
        );
        match graph_result {
            Ok(_) => {
                v.check_bool(
                    &format!("{stem}_deploy_dry_run"),
                    true,
                    &format!("{cell_file} dry-run accepted by biomeOS"),
                );
            }
            Err(e) => {
                v.check_bool(
                    &format!("{stem}_deploy_dry_run"),
                    false,
                    &format!("{cell_file} rejected: {e}"),
                );
            }
        }
    }
}
