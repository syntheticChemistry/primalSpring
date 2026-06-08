// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Layer 0: Bare properties — structural validation that requires no running primals.

use std::path::Path;

use crate::bonding::{BondType, BondingPolicy};
use crate::deploy;
use crate::validation::ValidationResult;

/// Layer 0: validate deploy graphs, fragments, downstream manifest, bonding types,
/// checksums, and deployment readiness of cell graphs.
pub fn validate_bare_properties(v: &mut ValidationResult) {
    validate_graph_parsing(v);
    validate_fragment_resolution(v);
    validate_manifest_consistency(v);
    validate_bonding_type_wellformed(v);
    validate_checksums(v);
    validate_deployment_readiness_cells(v);
}

fn validate_checksums(v: &mut ValidationResult) {
    crate::checksums::verify_manifest(v, "validation/CHECKSUMS");
}

fn validate_graph_parsing(v: &mut ValidationResult) {
    let graph_dirs: &[&str] = &["graphs/profiles", "graphs/multi_node", "graphs/compositions"];

    let skip_suffixes: &[&str] = &["_manifest.toml", "_template.toml"];

    let mut total = 0usize;
    let mut clean = 0usize;

    for dir_name in graph_dirs {
        let dir = Path::new(dir_name);
        if !dir.exists() {
            continue;
        }
        let results = deploy::validate_all_graphs(dir);
        for result in &results {
            if skip_suffixes.iter().any(|s| result.path.ends_with(s)) {
                continue;
            }
            total += 1;
            if result.parsed && result.issues.is_empty() {
                clean += 1;
            } else if !result.parsed {
                v.check_bool(
                    &format!("graph_parse:{}", result.path),
                    false,
                    "failed to parse",
                );
            } else {
                v.check_bool(
                    &format!("graph_structural:{}", result.path),
                    false,
                    &result.issues.join("; "),
                );
            }
        }
    }

    let downstream_dir = Path::new("graphs/downstream");
    if downstream_dir.exists() {
        for result in &deploy::validate_all_graphs(downstream_dir) {
            if skip_suffixes.iter().any(|s| result.path.ends_with(s)) {
                continue;
            }
            total += 1;
            if result.parsed && result.issues.is_empty() {
                clean += 1;
            } else if !result.parsed {
                v.check_bool(
                    &format!("graph_parse:{}", result.path),
                    false,
                    "failed to parse as deploy graph",
                );
            } else {
                v.check_bool(
                    &format!("graph_structural:{}", result.path),
                    false,
                    &result.issues.join("; "),
                );
            }
        }
    }

    for extra_dir in ["graphs/spring_validation", "graphs/spring_deploy"] {
        let dir = Path::new(extra_dir);
        if !dir.exists() {
            continue;
        }
        for result in &deploy::validate_all_graphs(dir) {
            if skip_suffixes.iter().any(|s| result.path.ends_with(s)) {
                continue;
            }
            total += 1;
            if result.parsed && result.issues.is_empty() {
                clean += 1;
            } else if !result.parsed {
                v.check_bool(
                    &format!("graph_parse:{}", result.path),
                    false,
                    "failed to parse",
                );
            } else {
                v.check_bool(
                    &format!("graph_structural:{}", result.path),
                    false,
                    &result.issues.join("; "),
                );
            }
        }
    }

    v.check_bool(
        "bare:all_graphs_parse",
        total > 0 && clean == total,
        &format!("{clean}/{total} deploy graphs clean"),
    );
}

fn validate_fragment_resolution(v: &mut ValidationResult) {
    let fragment_dir = Path::new("graphs/fragments");
    if !fragment_dir.exists() {
        v.check_skip("bare:fragments_exist", "graphs/fragments/ not found");
        return;
    }

    let expected_fragments = &[
        "tower_atomic",
        "node_atomic",
        "nest_atomic",
        "nucleus",
        "meta_tier",
        "provenance_trio",
    ];

    for &expected in expected_fragments {
        let path = fragment_dir.join(format!("{expected}.toml"));
        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    let parsed: Result<toml::Value, _> = toml::from_str(&content);
                    match parsed {
                        Ok(table) => {
                            let has_fragment = table.get("fragment").is_some();
                            let has_nodes = table
                                .get("fragment")
                                .and_then(|f| f.get("nodes").or_else(|| f.get("node")))
                                .and_then(|n| n.as_array())
                                .is_some_and(|a| !a.is_empty());
                            v.check_bool(
                                &format!("bare:fragment:{expected}"),
                                has_fragment && has_nodes,
                                &format!("[fragment] section: {has_fragment}, nodes: {has_nodes}"),
                            );
                        }
                        Err(e) => {
                            v.check_bool(
                                &format!("bare:fragment:{expected}"),
                                false,
                                &format!("TOML parse error: {e}"),
                            );
                        }
                    }
                }
                Err(e) => {
                    v.check_bool(
                        &format!("bare:fragment:{expected}"),
                        false,
                        &format!("cannot read: {e}"),
                    );
                }
            }
        } else {
            v.check_bool(
                &format!("bare:fragment:{expected}"),
                false,
                "missing from graphs/fragments/",
            );
        }
    }
}

fn validate_manifest_consistency(v: &mut ValidationResult) {
    let manifest_path = Path::new("graphs/downstream/downstream_manifest.toml");
    if !manifest_path.exists() {
        v.check_skip("bare:manifest_exists", "downstream_manifest.toml not found");
        return;
    }

    let Ok(content) = std::fs::read_to_string(manifest_path) else {
        v.check_bool("bare:manifest_readable", false, "failed to read");
        return;
    };

    let parsed: Result<toml::Value, _> = toml::from_str(&content);
    match parsed {
        Ok(table) => {
            let entries = table
                .get("downstream")
                .and_then(|d| d.as_array())
                .map_or(0, Vec::len);
            v.check_bool(
                "bare:manifest_valid",
                entries > 0,
                &format!("{entries} downstream entries"),
            );

            if let Some(arr) = table.get("downstream").and_then(|d| d.as_array()) {
                for entry in arr {
                    let name = entry
                        .get("spring_name")
                        .and_then(|n| n.as_str())
                        .unwrap_or("unknown");
                    let has_caps = entry
                        .get("validation_capabilities")
                        .and_then(|c| c.as_array())
                        .is_some_and(|c| !c.is_empty());
                    v.check_bool(
                        &format!("bare:manifest:{name}:has_capabilities"),
                        has_caps,
                        if has_caps {
                            "validation_capabilities present"
                        } else {
                            "missing or empty validation_capabilities"
                        },
                    );
                }
            }
        }
        Err(e) => {
            v.check_bool(
                "bare:manifest_valid",
                false,
                &format!("TOML parse error: {e}"),
            );
        }
    }
}

fn validate_bonding_type_wellformed(v: &mut ValidationResult) {
    for &bond in BondType::all() {
        let desc = bond.description();
        v.check_bool(
            &format!("bare:bondtype:{bond:?}"),
            !desc.is_empty(),
            &format!("description: {desc}"),
        );
    }

    let policy = BondingPolicy::covalent_default();
    let errors = policy.validate();
    let detail = if errors.is_empty() {
        "clean".to_owned()
    } else {
        errors.join("; ")
    };
    v.check_bool(
        "bare:bondpolicy:covalent_default_valid",
        errors.is_empty(),
        &detail,
    );

    let ionic = BondingPolicy::ionic_contract(vec!["compute".to_owned()]);
    let errors = ionic.validate();
    let detail = if errors.is_empty() {
        "clean".to_owned()
    } else {
        errors.join("; ")
    };
    v.check_bool(
        "bare:bondpolicy:ionic_contract_valid",
        errors.is_empty(),
        &detail,
    );
}

fn validate_deployment_readiness_cells(v: &mut ValidationResult) {
    let cells_dir = Path::new("graphs/cells");
    if !cells_dir.exists() {
        v.check_skip(
            "bare:deployment_readiness",
            "graphs/cells/ not found",
        );
        return;
    }

    let skip_suffixes: &[&str] = &["_manifest.toml", "_template.toml"];
    let mut checked = 0u32;
    let mut ready = 0u32;

    let Ok(entries) = std::fs::read_dir(cells_dir) else {
        v.check_bool("bare:deployment_readiness:read_dir", false, "cannot read graphs/cells/");
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "toml") {
            continue;
        }
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        if skip_suffixes.iter().any(|s| name.ends_with(s)) {
            continue;
        }

        checked += 1;
        match deploy::validate_deployment_readiness(&path) {
            Ok(result) => {
                // Bare certification only flags structural and bonding issues.
                // Binary/env checks require deployment context.
                let blocking: Vec<&crate::deploy::ReadinessIssue> = result
                    .issues
                    .iter()
                    .filter(|i| matches!(
                        i.category,
                        crate::deploy::ReadinessCategory::Structure
                            | crate::deploy::ReadinessCategory::BondingInconsistent
                    ))
                    .collect();

                if blocking.is_empty() {
                    ready += 1;
                } else {
                    let issue_summary: Vec<String> = blocking
                        .iter()
                        .take(3)
                        .map(|i| i.detail.clone())
                        .collect();
                    v.check_bool(
                        &format!("bare:cell_readiness:{}", result.graph_name),
                        false,
                        &issue_summary.join("; "),
                    );
                }
            }
            Err(e) => {
                v.check_bool(
                    &format!("bare:cell_parse:{name}"),
                    false,
                    &format!("failed to load: {e}"),
                );
            }
        }
    }

    v.check_bool(
        "bare:deployment_readiness:cells_checked",
        checked > 0,
        &format!("{ready}/{checked} cell graphs deployment-ready (structural)"),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bonding_type_descriptions_nonempty() {
        for &bond in BondType::all() {
            let desc = bond.description();
            assert!(!desc.is_empty(), "{bond:?} has empty description");
        }
    }

    #[test]
    fn bonding_type_wellformed_checks_pass() {
        let mut v = crate::validation::ValidationResult::new("test");
        validate_bonding_type_wellformed(&mut v);
        assert!(v.passed > 0);
        assert_eq!(v.failed, 0, "bond type checks should all pass");
    }

    #[test]
    fn validate_bare_properties_produces_checks() {
        let mut v = crate::validation::ValidationResult::new("test");
        validate_bare_properties(&mut v);
        let total = v.passed + v.failed + v.skipped;
        assert!(total > 0, "bare properties should produce at least one check");
    }
}
