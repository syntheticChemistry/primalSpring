// SPDX-License-Identifier: AGPL-3.0-or-later

//! Layer 0: Bare properties — structural validation that requires no running primals.

use std::path::Path;

use primalspring::bonding::{BondType, BondingPolicy};
use primalspring::deploy;
use primalspring::validation::ValidationResult;

pub fn validate_bare_properties(v: &mut ValidationResult) {
    validate_graph_parsing(v);
    validate_fragment_resolution(v);
    validate_manifest_consistency(v);
    validate_bonding_type_wellformed(v);
    validate_checksums(v);
}

fn validate_checksums(v: &mut ValidationResult) {
    primalspring::checksums::verify_manifest(v, "validation/CHECKSUMS");
}

fn validate_graph_parsing(v: &mut ValidationResult) {
    let graph_dirs: &[&str] = &["graphs/profiles", "graphs/multi_node"];

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

    let validation_dir = Path::new("graphs/spring_validation");
    if validation_dir.exists() {
        for result in &deploy::validate_all_graphs(validation_dir) {
            if skip_suffixes.iter().any(|s| result.path.ends_with(s)) {
                continue;
            }
            total += 1;
            if result.parsed && result.issues.is_empty() {
                clean += 1;
            }
        }
    }

    let deploy_dir = Path::new("graphs/spring_deploy");
    if deploy_dir.exists() {
        for result in &deploy::validate_all_graphs(deploy_dir) {
            if skip_suffixes.iter().any(|s| result.path.ends_with(s)) {
                continue;
            }
            total += 1;
            if result.parsed && result.issues.is_empty() {
                clean += 1;
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
