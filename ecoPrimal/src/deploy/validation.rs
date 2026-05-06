// SPDX-License-Identifier: AGPL-3.0-or-later

//! Deploy graph validation — structural checks, live probing, and deployment readiness.
//!
//! Separated from the core graph loading and merging logic in `deploy/mod.rs`
//! so that validation concerns (live probing, readiness checks, structural
//! audits) live in their own module.

use std::path::Path;

use serde::Serialize;

use crate::coordination::probe_primal;

use super::{DeployError, DeployGraph, GraphNode, load_graph};

/// Result of loading and validating a deploy graph.
#[derive(Debug, Clone, Serialize)]
pub struct GraphValidation {
    /// Path the graph was loaded from.
    pub path: String,
    /// Graph name.
    pub name: String,
    /// Whether the TOML parsed successfully.
    pub parsed: bool,
    /// Structural issues found (empty if clean).
    pub issues: Vec<String>,
    /// Number of nodes in the graph.
    pub node_count: usize,
    /// Number of required nodes.
    pub required_count: usize,
}

/// Result of live-validating a deploy graph against running primals.
#[derive(Debug, Clone, Serialize)]
pub struct LiveGraphValidation {
    /// Structural validation.
    pub structure: GraphValidation,
    /// Per-node health status.
    pub nodes: Vec<NodeHealth>,
    /// Number of nodes that are reachable and healthy.
    pub healthy_count: usize,
    /// Whether all required nodes are healthy.
    pub all_required_healthy: bool,
}

/// Health status of a single graph node.
#[derive(Debug, Clone, Serialize)]
pub struct NodeHealth {
    /// Node/primal name.
    pub name: String,
    /// Whether this node is required by the graph.
    pub required: bool,
    /// Whether the primal's socket was found.
    pub socket_found: bool,
    /// Whether the primal responded to health check.
    pub health_ok: bool,
    /// Capabilities reported by the primal.
    pub capabilities: Vec<String>,
}

/// A single readiness issue for a graph node or the graph as a whole.
#[derive(Debug, Clone, Serialize)]
pub struct ReadinessIssue {
    /// Which node this issue applies to (empty string for graph-level issues).
    pub node: String,
    /// Issue category.
    pub category: ReadinessCategory,
    /// Human-readable description.
    pub detail: String,
}

/// Categories of deployment readiness issues.
#[derive(Debug, Clone, Serialize)]
pub enum ReadinessCategory {
    /// Graph structure issue (from `structural_checks`).
    Structure,
    /// Binary not found via `launcher::discover_binary`.
    BinaryMissing,
    /// Required environment variable is not set.
    EnvMissing,
    /// Bonding policy inconsistency.
    BondingInconsistent,
}

/// Result of a deployment readiness check.
#[derive(Debug, Clone, Serialize)]
pub struct DeploymentReadiness {
    /// Whether the graph is ready to deploy (no blocking issues).
    pub ready: bool,
    /// Graph name.
    pub graph_name: String,
    /// Total spawnable nodes.
    pub spawnable_count: usize,
    /// Nodes with binaries found.
    pub binaries_found: usize,
    /// All issues found (empty = ready).
    pub issues: Vec<ReadinessIssue>,
}

/// Validate a deploy graph's live health using capability-based discovery
/// for nodes that declare `by_capability`, falling back to identity only
/// when no capability is declared.
///
/// This is the graph-as-source-of-truth path: the graph defines what
/// capabilities are needed, and this function discovers providers at runtime.
#[must_use]
pub fn validate_live_by_capability(path: &Path) -> LiveGraphValidation {
    let structure = validate_structure(path);
    if !structure.parsed {
        return LiveGraphValidation {
            structure,
            nodes: Vec::new(),
            healthy_count: 0,
            all_required_healthy: false,
        };
    }

    let graph = match load_graph(path) {
        Ok(g) => g,
        Err(e) => {
            let mut degraded = structure;
            degraded
                .issues
                .push(format!("graph changed between parse passes: {e}"));
            return LiveGraphValidation {
                structure: degraded,
                nodes: Vec::new(),
                healthy_count: 0,
                all_required_healthy: false,
            };
        }
    };

    let nodes: Vec<NodeHealth> = graph.graph.node.iter().map(probe_graph_node).collect();
    let healthy_count = nodes.iter().filter(|n| n.health_ok).count();
    let all_required_healthy = nodes.iter().filter(|n| n.required).all(|n| n.health_ok);

    LiveGraphValidation {
        structure,
        nodes,
        healthy_count,
        all_required_healthy,
    }
}

/// Structurally validate a deploy graph (no live probing).
#[must_use]
pub fn validate_structure(path: &Path) -> GraphValidation {
    match load_graph(path) {
        Ok(graph) => {
            let mut issues = Vec::new();
            structural_checks(&graph, &mut issues);
            GraphValidation {
                path: path.display().to_string(),
                name: graph.graph.name.clone(),
                parsed: true,
                node_count: graph.graph.node.len(),
                required_count: graph.graph.node.iter().filter(|n| n.required).count(),
                issues,
            }
        }
        Err(e) => GraphValidation {
            path: path.display().to_string(),
            name: String::new(),
            parsed: false,
            issues: vec![e.to_string()],
            node_count: 0,
            required_count: 0,
        },
    }
}

/// Validate a deploy graph against running primals.
///
/// Returns structural validation only (with an issue note) if the graph
/// file changes between the structural parse and the live-probe parse.
#[must_use]
pub fn validate_live(path: &Path) -> LiveGraphValidation {
    let structure = validate_structure(path);
    if !structure.parsed {
        return LiveGraphValidation {
            structure,
            nodes: Vec::new(),
            healthy_count: 0,
            all_required_healthy: false,
        };
    }

    let graph = match load_graph(path) {
        Ok(g) => g,
        Err(e) => {
            let mut degraded = structure;
            degraded
                .issues
                .push(format!("graph changed between parse passes: {e}"));
            return LiveGraphValidation {
                structure: degraded,
                nodes: Vec::new(),
                healthy_count: 0,
                all_required_healthy: false,
            };
        }
    };
    let nodes: Vec<NodeHealth> = graph.graph.node.iter().map(probe_graph_node).collect();

    let healthy_count = nodes.iter().filter(|n| n.health_ok).count();
    let all_required_healthy = nodes.iter().filter(|n| n.required).all(|n| n.health_ok);

    LiveGraphValidation {
        structure,
        nodes,
        healthy_count,
        all_required_healthy,
    }
}

/// Discover and validate all deploy graphs in a directory.
#[must_use]
pub fn validate_all_graphs(dir: &Path) -> Vec<GraphValidation> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut results = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "toml") {
            results.push(validate_structure(&path));
        }
    }
    results.sort_by(|a, b| a.path.cmp(&b.path));
    results
}

/// Check whether a deploy graph is ready to deploy on this system.
///
/// Combines:
/// - Structural validation (graph parses, nodes are well-formed)
/// - Binary discovery (each spawnable node's binary exists via `launcher::discover_binary`)
/// - Environment checks (`FAMILY_ID` and `XDG_RUNTIME_DIR` are set)
/// - Bonding consistency (security_model vs bonding_policy)
///
/// # Errors
///
/// Returns [`DeployError`] if the graph cannot be loaded at all.
pub fn validate_deployment_readiness(path: &Path) -> Result<DeploymentReadiness, DeployError> {
    let graph = load_graph(path)?;
    let mut issues = Vec::new();

    let mut structural_issues = Vec::new();
    structural_checks(&graph, &mut structural_issues);
    for issue in structural_issues {
        issues.push(ReadinessIssue {
            node: String::new(),
            category: ReadinessCategory::Structure,
            detail: issue,
        });
    }

    for key in [crate::env_keys::FAMILY_ID, crate::env_keys::XDG_RUNTIME_DIR] {
        if std::env::var(key).is_err() {
            issues.push(ReadinessIssue {
                node: String::new(),
                category: ReadinessCategory::EnvMissing,
                detail: format!("required env var {key} is not set"),
            });
        }
    }

    let spawnable: Vec<&GraphNode> = graph.graph.node.iter().filter(|n| n.spawn).collect();
    let spawnable_count = spawnable.len();
    let mut binaries_found = 0;

    for node in &spawnable {
        if node.binary.is_empty() {
            continue;
        }
        match crate::launcher::discover_binary(&node.name) {
            Ok(_) => binaries_found += 1,
            Err(_) => {
                issues.push(ReadinessIssue {
                    node: node.name.clone(),
                    category: ReadinessCategory::BinaryMissing,
                    detail: format!(
                        "binary '{}' for node '{}' not found in plasmidBin or XDG paths",
                        node.binary, node.name
                    ),
                });
            }
        }
    }

    let has_btsp_nodes = graph
        .graph
        .node
        .iter()
        .any(|n| n.security_model.as_deref() == Some("btsp"));
    if has_btsp_nodes && graph.graph.bonding_policy.is_none() {
        issues.push(ReadinessIssue {
            node: String::new(),
            category: ReadinessCategory::BondingInconsistent,
            detail: "nodes require BTSP but no bonding_policy is declared".to_owned(),
        });
    }

    let ready = issues.is_empty();

    Ok(DeploymentReadiness {
        ready,
        graph_name: graph.graph.name,
        spawnable_count,
        binaries_found,
        issues,
    })
}

/// Probe a graph node using capability-first discovery when `by_capability`
/// is set, falling back to identity-based discovery via `node.name`.
///
/// This is the loose coupling bridge: graph nodes that declare their
/// capability domain are discovered by what they provide, not who they are.
fn probe_graph_node(node: &GraphNode) -> NodeHealth {
    let health = node.by_capability.as_ref().map_or_else(
        || probe_primal(&node.name),
        |cap| {
            let disc = crate::ipc::discover::discover_by_capability(cap);
            if disc.socket.is_some() {
                let name = disc.resolved_primal.unwrap_or_else(|| node.name.clone());
                probe_primal(&name)
            } else {
                probe_primal(&node.name)
            }
        },
    );

    NodeHealth {
        name: node.name.clone(),
        required: node.required,
        socket_found: health.socket_found,
        health_ok: health.health_ok,
        capabilities: health.capabilities,
    }
}

pub(super) fn structural_checks(graph: &DeployGraph, issues: &mut Vec<String>) {
    if graph.graph.name.is_empty() {
        issues.push("graph.name is empty".to_owned());
    }
    if graph.graph.node.is_empty() {
        issues.push("graph has no nodes".to_owned());
    }

    let names: Vec<&str> = graph.graph.node.iter().map(|n| n.name.as_str()).collect();
    let is_multi_node = graph.graph.node.iter().any(|n| n.operation.is_some());
    for node in &graph.graph.node {
        if node.name.is_empty() {
            issues.push(format!("node at order {} has empty name", node.order));
        }
        if !is_multi_node && node.binary.is_empty() {
            issues.push(format!("node '{}' has empty binary", node.name));
        }
        if !is_multi_node && node.health_method.is_empty() {
            issues.push(format!("node '{}' has no health_method", node.name));
        }
        for dep in &node.depends_on {
            if !names.contains(&dep.as_str()) {
                issues.push(format!(
                    "node '{}' depends on '{}' which is not in the graph",
                    node.name, dep
                ));
            }
        }
    }

    let mut orders: Vec<u32> = graph.graph.node.iter().map(|n| n.order).collect();
    orders.sort_unstable();
    orders.dedup();
    if !is_multi_node && orders.len() != graph.graph.node.len() {
        issues.push("duplicate order values in graph nodes".to_owned());
    }

    let has_bonding_policy = graph.graph.bonding_policy.is_some();
    let has_btsp_nodes = graph
        .graph
        .node
        .iter()
        .any(|n| n.security_model.as_deref() == Some("btsp"));
    if has_btsp_nodes && !has_bonding_policy {
        issues.push(
            "nodes declare security_model=\"btsp\" but graph has no [graph.bonding_policy]"
                .to_owned(),
        );
    }

    let transport = graph
        .graph
        .metadata
        .as_ref()
        .and_then(|m| m.transport.as_deref());
    if transport == Some("uds_only") {
        for node in &graph.graph.node {
            let is_operation_only = node.operation.is_some() && node.primal.is_none();
            if is_operation_only {
                continue;
            }
            let has_by_capability = node.by_capability.is_some()
                || node
                    .primal
                    .as_ref()
                    .and_then(|p| p.get("by_capability"))
                    .is_some();
            if !has_by_capability && !node.name.is_empty() && node.spawn {
                issues.push(format!(
                    "transport=uds_only but node '{}' has no by_capability (needed for UDS discovery)",
                    node.name
                ));
            }
        }
    }
}
