// SPDX-License-Identifier: AGPL-3.0-or-later

//! Shared validation helpers for graph TOML, Dark Forest invariants,
//! and capability registry cross-referencing.
//!
//! Extracted from duplicated patterns across `s_atomic_signals`,
//! `s_agentic_tower`, and `s_meta_tier_signals` during the eukaryotic
//! evolution of the validation infrastructure. Any scenario that
//! validates graph TOML, checks Dark Forest metadata, or cross-references
//! capabilities against the registry should use these helpers instead
//! of reimplementing them locally.

use super::ValidationResult;

/// Parse TOML content and record a pass/fail check on `v`.
///
/// Returns `Some(parsed)` on success, `None` on parse error (with
/// a failure check recorded).
pub fn graph_parses(
    v: &mut ValidationResult,
    label: &str,
    content: &str,
) -> Option<toml::Value> {
    match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => {
            v.check_bool(
                &format!("{label}:parse"),
                true,
                &format!("{label} parses as valid TOML"),
            );
            Some(parsed)
        }
        Err(e) => {
            v.check_bool(
                &format!("{label}:parse"),
                false,
                &format!("{label} TOML parse error: {e}"),
            );
            None
        }
    }
}

/// Extract all `binary` field values from `[[graph.nodes]]` in a TOML
/// graph string. Returns an empty vec on parse failure.
#[must_use] 
pub fn graph_binaries(content: &str) -> Vec<String> {
    let parsed: toml::Value = match toml::from_str(content) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    graph_binaries_from_parsed(&parsed)
}

/// Extract all `binary` field values from an already-parsed graph.
#[must_use] 
pub fn graph_binaries_from_parsed(parsed: &toml::Value) -> Vec<String> {
    parsed
        .get("graph")
        .and_then(|g| g.get("nodes"))
        .and_then(|n| n.as_array())
        .map(|nodes| {
            nodes
                .iter()
                .filter_map(|n| n.get("binary").and_then(|b| b.as_str()).map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

/// Safe accessor for `graph.nodes` from a parsed TOML value.
#[must_use] 
pub fn graph_nodes(parsed: &toml::Value) -> Option<&Vec<toml::Value>> {
    parsed
        .get("graph")
        .and_then(|g| g.get("nodes"))
        .and_then(|n| n.as_array())
}

/// Safe accessor for `graph.metadata` from a parsed TOML value.
#[must_use] 
pub fn graph_metadata(parsed: &toml::Value) -> Option<&toml::Value> {
    parsed.get("graph").and_then(|g| g.get("metadata"))
}

/// Validate Dark Forest security invariants on a graph's metadata:
/// `secure_by_default = true`, `security_model = "btsp_enforced"`,
/// `transport = "uds_only"`.
pub fn validate_dark_forest(
    v: &mut ValidationResult,
    label: &str,
    parsed: &toml::Value,
) {
    let metadata = graph_metadata(parsed);

    let secure_by_default = metadata
        .and_then(|m| m.get("secure_by_default"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);
    v.check_bool(
        &format!("{label}:secure_by_default"),
        secure_by_default,
        &format!("{label} graph has secure_by_default = true"),
    );

    let btsp_enforced = metadata
        .and_then(|m| m.get("security_model"))
        .and_then(|s| s.as_str()) == Some("btsp_enforced");
    v.check_bool(
        &format!("{label}:btsp_enforced"),
        btsp_enforced,
        &format!("{label} graph has security_model = btsp_enforced"),
    );

    let transport = metadata
        .and_then(|m| m.get("transport"))
        .and_then(|t| t.as_str())
        .unwrap_or("unknown");
    let valid_transport = transport == "uds_only" || transport == "uds_and_mesh";
    v.check_bool(
        &format!("{label}:uds_only"),
        valid_transport,
        &format!("{label} graph transport = {transport} (valid: uds_only | uds_and_mesh)"),
    );
}

/// Validate that every capability referenced by graph nodes is registered
/// in the capability registry, and that each node has a `by_capability`
/// routing hint.
pub fn validate_node_capabilities(
    v: &mut ValidationResult,
    label: &str,
    parsed: &toml::Value,
    registry_caps: &[String],
) {
    let Some(nodes) = graph_nodes(parsed) else {
        return;
    };

    for node in nodes {
        let node_name = node
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("unknown");
        let caps = node
            .get("capabilities")
            .and_then(|c| c.as_array())
            .unwrap_or(&Vec::new())
            .clone();

        for cap in &caps {
            if let Some(cap_str) = cap.as_str() {
                let registered = registry_caps.iter().any(|r| r == cap_str);
                v.check_bool(
                    &format!("{label}:{node_name}:{cap_str}:registered"),
                    registered,
                    &format!("{cap_str} in {node_name} is registered in capability_registry"),
                );
            }
        }

        let has_by_capability = node
            .get("by_capability")
            .and_then(|b| b.as_str())
            .is_some();
        v.check_bool(
            &format!("{label}:{node_name}:by_capability"),
            has_by_capability,
            &format!("{node_name} has by_capability for capability routing"),
        );
    }
}

/// Parse the capability registry TOML and return a flat list of all
/// registered method strings (excluding test fixtures, false positives,
/// and signal definitions).
#[must_use] 
pub fn load_registry_capabilities() -> Vec<String> {
    let registry_toml = include_str!("../../../config/capability_registry.toml");
    parse_registry_capabilities(registry_toml)
}

/// Parse capability methods from a registry TOML string.
#[must_use] 
pub fn parse_registry_capabilities(registry_toml: &str) -> Vec<String> {
    let parsed: toml::Value = match toml::from_str(registry_toml) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let skip = ["test_fixtures", "false_positives", "signals"];
    let mut caps = Vec::new();
    if let Some(table) = parsed.as_table() {
        for (section, value) in table {
            if skip.contains(&section.as_str()) {
                continue;
            }
            if let Some(methods) = value.get("methods").and_then(|m| m.as_array()) {
                for m in methods {
                    if let Some(s) = m.as_str() {
                        caps.push(s.to_owned());
                    }
                }
            }
        }
    }
    caps
}

/// Validate graph structure fields: `id`, `coordination`, `signal_tier`,
/// `signal_name`, and at least one node.
pub fn validate_graph_structure(
    v: &mut ValidationResult,
    label: &str,
    parsed: &toml::Value,
) {
    let graph = parsed.get("graph");

    let has_id = graph
        .and_then(|g| g.get("id"))
        .and_then(|id| id.as_str())
        .is_some();
    v.check_bool(
        &format!("{label}:has_id"),
        has_id,
        &format!("{label} graph has id field"),
    );

    let has_coordination = graph
        .and_then(|g| g.get("coordination"))
        .and_then(|c| c.as_str())
        .is_some();
    v.check_bool(
        &format!("{label}:has_coordination"),
        has_coordination,
        &format!("{label} graph has coordination field"),
    );

    let has_signal_tier = graph
        .and_then(|g| g.get("signal_tier"))
        .and_then(|t| t.as_str())
        .is_some();
    v.check_bool(
        &format!("{label}:has_signal_tier"),
        has_signal_tier,
        &format!("{label} graph has signal_tier field"),
    );

    let has_signal_name = graph
        .and_then(|g| g.get("signal_name"))
        .and_then(|n| n.as_str())
        .is_some();
    v.check_bool(
        &format!("{label}:has_signal_name"),
        has_signal_name,
        &format!("{label} graph has signal_name field"),
    );

    let node_count = graph_nodes(parsed).map_or(0, Vec::len);
    v.check_bool(
        &format!("{label}:has_nodes"),
        node_count > 0,
        &format!("{label} graph has {node_count} nodes"),
    );
}
