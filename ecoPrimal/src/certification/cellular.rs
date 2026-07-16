// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Layer 7: Cellular deployment — per-spring deploy graph validation.

use std::path::Path;

use crate::primal_names;
use crate::validation::ValidationResult;

/// Layer 7: parse `graphs/cells/` deploy graphs and assert live-mode + node conventions.
pub fn validate_cellular_graphs(v: &mut ValidationResult) {
    let cells_dir = Path::new("graphs/cells");

    if !cells_dir.is_dir() {
        v.check_skip("cellular:dir_exists", "graphs/cells/ not found");
        return;
    }
    v.check_bool("cellular:dir_exists", true, "graphs/cells/ present");

    let manifest_path = cells_dir.join("cells_manifest.toml");
    let manifest_ok = manifest_path.is_file()
        && std::fs::read_to_string(&manifest_path)
            .ok()
            .and_then(|s| s.parse::<toml::Value>().ok())
            .is_some();
    v.check_bool(
        "cellular:manifest_parses",
        manifest_ok,
        "cells_manifest.toml present and valid",
    );

    let cell_files: Vec<_> = std::fs::read_dir(cells_dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| {
            e.path()
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.ends_with("_cell.toml"))
        })
        .collect();

    v.check_bool(
        "cellular:cell_count",
        !cell_files.is_empty(),
        &format!("{} cell graphs found", cell_files.len()),
    );

    for entry in &cell_files {
        let path = entry.path();
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        let Ok(content) = std::fs::read_to_string(&path) else {
            v.check_bool(
                &format!("cellular:{stem}:readable"),
                false,
                "file not readable",
            );
            continue;
        };

        let val: toml::Value = match content.parse() {
            Ok(p) => p,
            Err(e) => {
                v.check_bool(
                    &format!("cellular:{stem}:parses"),
                    false,
                    &format!("parse error: {e}"),
                );
                continue;
            }
        };
        validate_cell_graph_toml(v, stem, &val);
    }
}

fn validate_cell_graph_toml(v: &mut ValidationResult, stem: &str, val: &toml::Value) {
    v.check_bool(&format!("cellular:{stem}:parses"), true, "valid TOML");

    let has_graph = val.get("graph").is_some();
    v.check_bool(
        &format!("cellular:{stem}:graph_section"),
        has_graph,
        "[graph] section present",
    );

    let pt_mode = val
        .get("graph")
        .and_then(|g| g.get("metadata"))
        .and_then(|m| m.get("petaltongue_mode"))
        .and_then(toml::Value::as_str);
    v.check_bool(
        &format!("cellular:{stem}:live_mode"),
        pt_mode == Some("live"),
        &format!("petaltongue_mode = {:?}", pt_mode.unwrap_or("MISSING")),
    );

    let nodes = val
        .get("graph")
        .and_then(|g| g.get("nodes"))
        .and_then(toml::Value::as_array)
        .or_else(|| val.get("nodes").and_then(toml::Value::as_array));

    let node_names: Vec<&str> = nodes
        .iter()
        .flat_map(|arr| arr.iter())
        .filter_map(|n| {
            n.get("name")
                .and_then(toml::Value::as_str)
                .or_else(|| n.get("id").and_then(toml::Value::as_str))
        })
        .collect();

    let tower_slugs = primal_names::Primal::TOWER_SLUGS;
    let has_tower = tower_slugs.iter().any(|slug| node_names.contains(slug));
    v.check_bool(
        &format!("cellular:{stem}:tower"),
        has_tower,
        "At least one Tower primal present in cellular graph",
    );

    let has_petaltongue = node_names.contains(&primal_names::PETALTONGUE);
    v.check_bool(
        &format!("cellular:{stem}:petaltongue"),
        has_petaltongue,
        "petalTongue node present",
    );

    let has_validate = node_names
        .iter()
        .any(|n| n.starts_with("validate") || n.starts_with("validate-"));
    v.check_bool(
        &format!("cellular:{stem}:health_check"),
        has_validate,
        "validation health_check node present",
    );

    let security_models: Vec<&str> = nodes
        .iter()
        .flat_map(|arr| arr.iter())
        .filter_map(|n| n.get("security_model").and_then(toml::Value::as_str))
        .collect();
    let all_btsp = !security_models.is_empty()
        && security_models
            .iter()
            .all(|&m| m == "btsp" || m == "btsp_enforced");
    let btsp_count = security_models
        .iter()
        .filter(|&&m| m == "btsp" || m == "btsp_enforced")
        .count();
    v.check_bool(
        &format!("cellular:{stem}:btsp_default"),
        all_btsp,
        &format!(
            "{}/{} nodes declare btsp security_model",
            btsp_count,
            security_models.len()
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cell_toml(live: bool, nodes: &[(&str, Option<&str>)]) -> toml::Value {
        let node_arr: Vec<toml::Value> = nodes
            .iter()
            .map(|(name, sec)| {
                let mut map = toml::map::Map::new();
                map.insert("name".into(), toml::Value::String(name.to_string()));
                if let Some(s) = sec {
                    map.insert("security_model".into(), toml::Value::String(s.to_string()));
                }
                toml::Value::Table(map)
            })
            .collect();

        let mut metadata = toml::map::Map::new();
        if live {
            metadata.insert(
                "petaltongue_mode".into(),
                toml::Value::String("live".into()),
            );
        }

        let mut graph = toml::map::Map::new();
        graph.insert("nodes".into(), toml::Value::Array(node_arr));
        graph.insert("metadata".into(), toml::Value::Table(metadata));

        let mut root = toml::map::Map::new();
        root.insert("graph".into(), toml::Value::Table(graph));
        toml::Value::Table(root)
    }

    #[test]
    fn valid_cell_graph_passes() {
        let val = make_cell_toml(
            true,
            &[
                (primal_names::BEARDOG, Some("btsp")),
                (primal_names::SONGBIRD, Some("btsp")),
                (primal_names::PETALTONGUE, Some("btsp")),
                ("validate-health", Some("btsp")),
            ],
        );
        let mut v = crate::validation::ValidationResult::new("test");
        validate_cell_graph_toml(&mut v, "test_cell", &val);
        assert!(v.passed > 0);
        assert_eq!(v.failed, 0, "valid cell should pass all checks");
    }

    #[test]
    fn missing_tower_primals_detected() {
        let val = make_cell_toml(
            true,
            &[
                (primal_names::PETALTONGUE, Some("btsp")),
                ("validate-health", Some("btsp")),
            ],
        );
        let mut v = crate::validation::ValidationResult::new("test");
        validate_cell_graph_toml(&mut v, "incomplete_cell", &val);
        assert!(v.failed > 0, "missing Tower primals should fail");
    }

    #[test]
    fn non_live_mode_detected() {
        let val = make_cell_toml(
            false,
            &[
                (primal_names::BEARDOG, Some("btsp")),
                (primal_names::SONGBIRD, Some("btsp")),
                (primal_names::PETALTONGUE, Some("btsp")),
                ("validate-health", Some("btsp")),
            ],
        );
        let mut v = crate::validation::ValidationResult::new("test");
        validate_cell_graph_toml(&mut v, "nonlive_cell", &val);
        assert!(v.failed > 0, "non-live mode should fail");
    }

    #[test]
    fn missing_btsp_security_model_detected() {
        let val = make_cell_toml(
            true,
            &[
                (primal_names::BEARDOG, None),
                (primal_names::SONGBIRD, None),
                (primal_names::PETALTONGUE, None),
                ("validate-health", None),
            ],
        );
        let mut v = crate::validation::ValidationResult::new("test");
        validate_cell_graph_toml(&mut v, "no_btsp_cell", &val);
        assert!(
            v.failed > 0,
            "missing security_model should fail btsp_default"
        );
    }
}
