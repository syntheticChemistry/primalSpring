// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]
//! Scenario: Cellular Deployment — absorbed from exp098.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};
use std::path::{Path, PathBuf};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "cellular-deployment",
        track: Track::Infrastructure,
        tier: Tier::Live,
        provenance_crate: "exp098_cellular_deployment",
        provenance_date: "2026-05-09",
        description: "Cellular deployment — cell graph manifests and biomeOS dry-run",
    },
    run,
};

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

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let base = find_project_root();

    v.section("Phase 1: Manifest");
    validate_manifest(v, &base);

    v.section("Phase 2: Structural");
    for cell_file in EXPECTED_CELLS {
        validate_cell_structure(v, &base, cell_file);
    }

    v.section("Phase 3: Capability coverage");
    validate_capability_coverage(v, &base);

    v.section("Phase 4: biomeOS deployment");
    validate_biomeos_deployment(v, &base, ctx);
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
            .map_or(0, std::vec::Vec::len);
        v.check_bool(
            "cells_manifest_count",
            cells == EXPECTED_CELLS.len(),
            &format!("{cells} cells indexed (expected {})", EXPECTED_CELLS.len()),
        );

        if let Some(arr) = val.get("cell").and_then(|c| c.as_array()) {
            let all_live = arr
                .iter()
                .all(|c| c.get("petaltongue_mode").and_then(|m| m.as_str()) == Some("live"));
            v.check_bool(
                "cells_manifest_all_live_mode",
                all_live,
                "all cells declare petaltongue_mode = live",
            );
        }
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "cell graph structural checks are sequential and verbose"
)]
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
    let cell_toml_ok = parsed.is_ok();
    v.check_bool(
        &format!("{stem}_cell_parses"),
        cell_toml_ok,
        &format!("{cell_file} is valid TOML"),
    );
    let Some(val) = parsed.ok() else { return };

    let has_graph = val.get("graph").is_some();
    v.check_bool(
        &format!("{stem}_has_graph"),
        has_graph,
        "[graph] section present",
    );

    let meta = val.get("graph").and_then(|g| g.get("metadata"));
    let pt_mode = meta
        .and_then(|m| m.get("petaltongue_mode"))
        .and_then(|v| v.as_str());
    v.check_bool(
        &format!("{stem}_metadata_live"),
        pt_mode == Some("live"),
        &format!("petaltongue_mode = {:?}", pt_mode.unwrap_or("MISSING")),
    );

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

    for required in NUCLEUS_REQUIRED {
        v.check_bool(
            &format!("{stem}_has_{required}"),
            node_names.iter().any(|n| n == required),
            &format!("{required} node present"),
        );
    }

    let petaltongue_node = nodes
        .iter()
        .find(|n| n.get("name").and_then(|v| v.as_str()) == Some("petaltongue"));
    let pt_live = petaltongue_node
        .and_then(|n| n.get("params"))
        .and_then(|p| p.get("mode"))
        .and_then(|m| m.as_str())
        == Some("live");
    v.check_bool(
        &format!("{stem}_petaltongue_live"),
        pt_live,
        "petaltongue node params.mode = live",
    );

    let has_validate = node_names.iter().any(|n| n.starts_with("validate"));
    v.check_bool(
        &format!("{stem}_has_validate_node"),
        has_validate,
        "validation health_check node present",
    );

    let base_primals = [
        "biomeos_neural_api",
        "beardog",
        "songbird",
        "toadstool",
        "barracuda",
        "coralreef",
        "nestgate",
        "rhizocrypt",
        "loamspine",
        "sweetgrass",
        "squirrel",
        "petaltongue",
    ];
    let domain_nodes: Vec<&String> = node_names
        .iter()
        .filter(|n| !base_primals.contains(&n.as_str()) && !n.starts_with("validate"))
        .collect();
    v.check_bool(
        &format!("{stem}_has_domain_overlay"),
        !domain_nodes.is_empty(),
        &format!(
            "domain node(s): {}",
            domain_nodes
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
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
        v.check_skip(
            "capability_coverage",
            "downstream_manifest.toml parse failed",
        );
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
        let Some(cell_val) = cell_parsed.ok() else {
            continue;
        };

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
            .map(|a| {
                a.iter()
                    .filter_map(|c| c.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let missing: Vec<&String> = required_caps
            .iter()
            .filter(|cap| !cell_caps.contains(cap))
            .collect();

        let detail = if missing.is_empty() {
            format!(
                "{}/{} validation capabilities covered",
                required_caps.len(),
                required_caps.len()
            )
        } else {
            format!(
                "{}/{} covered, missing: {}",
                required_caps.len() - missing.len(),
                required_caps.len(),
                missing
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        v.check_bool(
            &format!("{spring_name}_cap_coverage"),
            missing.is_empty(),
            &detail,
        );
    }
}

fn validate_biomeos_deployment(
    v: &mut ValidationResult,
    base: &Path,
    ctx: &mut CompositionContext,
) {
    let caps = ctx.available_capabilities();

    if caps.is_empty() {
        v.check_skip(
            "biomeos_live",
            "no live primals discovered — structural validation only",
        );
        return;
    }

    let has_orchestration = caps
        .iter()
        .any(|c| c.contains("graph") || c.contains("orchestration"));
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
            v.check_skip(&format!("{stem}_deploy"), &format!("{cell_file} not found"));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cellular_deployment_no_panic() {
        let mut v = ValidationResult::new("cellular-deployment");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(v.evaluated() > 0 || v.skipped > 0, "scenario should produce at least one check");
    }
}
