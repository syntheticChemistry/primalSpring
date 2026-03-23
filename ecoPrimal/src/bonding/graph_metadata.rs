// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bonding metadata validation for deploy graphs.
//!
//! Parses `[graph.metadata]` and `[graph.bonding_policy]` sections from
//! biomeOS deploy graphs and validates their consistency with the
//! NUCLEUS bonding model.

use std::path::Path;

use serde::Deserialize;

use super::{BondType, BondingConstraint, BondingPolicy, TrustModel};

/// Raw TOML representation of a graph's bonding metadata.
#[derive(Debug, Deserialize)]
struct RawGraphToml {
    graph: RawGraph,
}

#[derive(Debug, Deserialize)]
struct RawGraph {
    #[serde(default)]
    id: String,
    #[serde(default)]
    metadata: Option<RawMetadata>,
    #[serde(default)]
    bonding_policy: Option<RawBondingPolicy>,
}

#[derive(Debug, Deserialize)]
struct RawMetadata {
    #[serde(default)]
    internal_bond_type: Option<String>,
    #[serde(default)]
    default_interaction_bond: Option<String>,
    #[serde(default)]
    trust_model: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawBondingPolicy {
    #[serde(default)]
    bond_type: Option<String>,
    #[serde(default)]
    trust_model: Option<String>,
    #[serde(default)]
    label: Option<String>,
    #[serde(default)]
    offer_relay: Option<bool>,
    #[serde(default)]
    active_windows: Option<Vec<String>>,
    #[serde(default)]
    constraints: Option<RawConstraints>,
}

#[derive(Debug, Deserialize)]
struct RawConstraints {
    #[serde(default)]
    capability_allow: Option<Vec<String>>,
    #[serde(default)]
    capability_deny: Option<Vec<String>>,
    #[serde(default)]
    bandwidth_limit_mbps: Option<u64>,
    #[serde(default)]
    max_concurrent_requests: Option<u32>,
}

/// Parsed and validated bonding metadata from a deploy graph.
#[derive(Debug, Clone)]
pub struct GraphBondingMetadata {
    /// Graph ID from the TOML.
    pub graph_id: String,
    /// Internal bond type for nodes within this graph.
    pub internal_bond_type: Option<BondType>,
    /// Default bond type for external interactions.
    pub default_interaction_bond: Option<BondType>,
    /// Trust model governing this graph's bonds.
    pub trust_model: Option<TrustModel>,
    /// Bonding policy (if declared).
    pub policy: Option<BondingPolicy>,
    /// Validation issues found.
    pub issues: Vec<String>,
}

/// Parse a bond type string from graph TOML.
fn parse_bond_type(s: &str) -> Option<BondType> {
    match s.to_lowercase().as_str() {
        "covalent" => Some(BondType::Covalent),
        "metallic" => Some(BondType::Metallic),
        "ionic" => Some(BondType::Ionic),
        "weak" => Some(BondType::Weak),
        "organo_metal_salt" | "organometalsalt" => Some(BondType::OrganoMetalSalt),
        _ => None,
    }
}

/// Parse a trust model string from graph TOML.
fn parse_trust_model(s: &str) -> Option<TrustModel> {
    match s.to_lowercase().replace('-', "_").as_str() {
        "genetic_lineage" | "geneticlineage" => Some(TrustModel::GeneticLineage),
        "contractual" | "contract" => Some(TrustModel::Contractual),
        "organizational" | "org" => Some(TrustModel::Organizational),
        "zero_trust" | "zerotrust" => Some(TrustModel::ZeroTrust),
        _ => None,
    }
}

/// Load and validate bonding metadata from a deploy graph TOML file.
///
/// Returns `GraphBondingMetadata` with any validation issues collected.
/// Graphs without `[graph.metadata]` return an empty metadata with no issues.
pub fn validate_graph_bonding(path: &Path) -> GraphBondingMetadata {
    let contents = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            return GraphBondingMetadata {
                graph_id: String::new(),
                internal_bond_type: None,
                default_interaction_bond: None,
                trust_model: None,
                policy: None,
                issues: vec![format!("failed to read {}: {e}", path.display())],
            };
        }
    };

    let raw: RawGraphToml = match toml::from_str(&contents) {
        Ok(r) => r,
        Err(e) => {
            return GraphBondingMetadata {
                graph_id: String::new(),
                internal_bond_type: None,
                default_interaction_bond: None,
                trust_model: None,
                policy: None,
                issues: vec![format!("failed to parse {}: {e}", path.display())],
            };
        }
    };

    let mut issues = Vec::new();
    let graph_id = raw.graph.id.clone();

    let internal_bond_type = raw
        .graph
        .metadata
        .as_ref()
        .and_then(|m| m.internal_bond_type.as_deref())
        .and_then(|s| {
            let bt = parse_bond_type(s);
            if bt.is_none() {
                issues.push(format!("unknown internal_bond_type: {s:?}"));
            }
            bt
        });

    let default_interaction_bond = raw
        .graph
        .metadata
        .as_ref()
        .and_then(|m| m.default_interaction_bond.as_deref())
        .and_then(|s| {
            let bt = parse_bond_type(s);
            if bt.is_none() {
                issues.push(format!("unknown default_interaction_bond: {s:?}"));
            }
            bt
        });

    let trust_model = raw
        .graph
        .metadata
        .as_ref()
        .and_then(|m| m.trust_model.as_deref())
        .and_then(|s| {
            let tm = parse_trust_model(s);
            if tm.is_none() {
                issues.push(format!("unknown trust_model: {s:?}"));
            }
            tm
        });

    // Cross-validate: covalent requires genetic lineage trust
    if let (Some(BondType::Covalent), Some(tm)) = (internal_bond_type, trust_model) {
        if tm != TrustModel::GeneticLineage {
            issues.push(format!(
                "covalent internal_bond_type requires genetic_lineage trust, got {tm:?}"
            ));
        }
    }

    // Parse bonding policy
    let policy = raw.graph.bonding_policy.map(|raw_policy| {
        let bond = raw_policy
            .bond_type
            .as_deref()
            .and_then(parse_bond_type)
            .unwrap_or(internal_bond_type.unwrap_or(BondType::Covalent));

        let tm = raw_policy
            .trust_model
            .as_deref()
            .and_then(parse_trust_model)
            .unwrap_or(trust_model.unwrap_or(TrustModel::GeneticLineage));

        let constraints = raw_policy
            .constraints
            .map(|rc| BondingConstraint {
                capability_allow: rc.capability_allow.unwrap_or_default(),
                capability_deny: rc.capability_deny.unwrap_or_default(),
                bandwidth_limit_mbps: rc.bandwidth_limit_mbps.unwrap_or(0),
                max_concurrent_requests: rc.max_concurrent_requests.unwrap_or(0),
            })
            .unwrap_or_default();

        let policy = BondingPolicy {
            bond_type: bond,
            trust_model: tm,
            constraints,
            active_windows: raw_policy.active_windows.unwrap_or_default(),
            offer_relay: raw_policy.offer_relay.unwrap_or(false),
            label: raw_policy
                .label
                .unwrap_or_else(|| graph_id.clone()),
        };

        let policy_issues = policy.validate();
        for issue in policy_issues {
            issues.push(format!("bonding_policy: {issue}"));
        }

        policy
    });

    GraphBondingMetadata {
        graph_id,
        internal_bond_type,
        default_interaction_bond,
        trust_model,
        policy,
        issues,
    }
}

/// Validate all graph files in a directory for bonding metadata.
pub fn validate_all_graph_bonding(dir: &Path) -> Vec<GraphBondingMetadata> {
    let mut results = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return results;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "toml") {
            results.push(validate_graph_bonding(&path));
        }
    }
    // Recurse into subdirectories
    let Ok(dirs) = std::fs::read_dir(dir) else {
        return results;
    };
    for entry in dirs.flatten() {
        let path = entry.path();
        if path.is_dir() {
            results.extend(validate_all_graph_bonding(&path));
        }
    }
    results.sort_by(|a, b| a.graph_id.cmp(&b.graph_id));
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_temp_toml(content: &str) -> tempfile::NamedTempFile {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f
    }

    #[test]
    fn parse_covalent_graph() {
        let f = write_temp_toml(
            r#"
[graph]
id = "test_covalent"
[graph.metadata]
internal_bond_type = "covalent"
default_interaction_bond = "covalent"
trust_model = "genetic_lineage"
"#,
        );
        let meta = validate_graph_bonding(f.path());
        assert_eq!(meta.graph_id, "test_covalent");
        assert_eq!(meta.internal_bond_type, Some(BondType::Covalent));
        assert_eq!(meta.trust_model, Some(TrustModel::GeneticLineage));
        assert!(meta.issues.is_empty(), "issues: {:?}", meta.issues);
    }

    #[test]
    fn parse_ionic_graph() {
        let f = write_temp_toml(
            r#"
[graph]
id = "test_ionic"
[graph.metadata]
internal_bond_type = "ionic"
trust_model = "contractual"
"#,
        );
        let meta = validate_graph_bonding(f.path());
        assert_eq!(meta.internal_bond_type, Some(BondType::Ionic));
        assert_eq!(meta.trust_model, Some(TrustModel::Contractual));
        assert!(meta.issues.is_empty());
    }

    #[test]
    fn catches_trust_mismatch() {
        let f = write_temp_toml(
            r#"
[graph]
id = "bad_trust"
[graph.metadata]
internal_bond_type = "covalent"
trust_model = "contractual"
"#,
        );
        let meta = validate_graph_bonding(f.path());
        assert!(!meta.issues.is_empty());
        assert!(meta.issues[0].contains("genetic_lineage"));
    }

    #[test]
    fn parse_bonding_policy() {
        let f = write_temp_toml(
            r#"
[graph]
id = "idle_compute"
[graph.metadata]
internal_bond_type = "covalent"
trust_model = "genetic_lineage"
[graph.bonding_policy]
bond_type = "Covalent"
trust_model = "GeneticLineage"
label = "idle-compute"
offer_relay = false
active_windows = ["22:00-06:00"]
[graph.bonding_policy.constraints]
capability_allow = ["compute.*"]
capability_deny = ["storage.*"]
bandwidth_limit_mbps = 100
max_concurrent_requests = 4
"#,
        );
        let meta = validate_graph_bonding(f.path());
        assert!(meta.issues.is_empty(), "issues: {:?}", meta.issues);
        let policy = meta.policy.unwrap();
        assert_eq!(policy.bond_type, BondType::Covalent);
        assert!(policy.constraints.permits("compute.submit"));
        assert!(!policy.constraints.permits("storage.store"));
        assert_eq!(policy.constraints.bandwidth_limit_mbps, 100);
        assert_eq!(policy.active_windows, vec!["22:00-06:00"]);
    }

    #[test]
    fn no_metadata_is_valid() {
        let f = write_temp_toml(
            r#"
[graph]
id = "minimal"
"#,
        );
        let meta = validate_graph_bonding(f.path());
        assert!(meta.issues.is_empty());
        assert!(meta.internal_bond_type.is_none());
        assert!(meta.policy.is_none());
    }

    #[test]
    fn unknown_bond_type_reports_issue() {
        let f = write_temp_toml(
            r#"
[graph]
id = "bad"
[graph.metadata]
internal_bond_type = "hydrogen"
"#,
        );
        let meta = validate_graph_bonding(f.path());
        assert!(!meta.issues.is_empty());
        assert!(meta.issues[0].contains("hydrogen"));
    }
}
