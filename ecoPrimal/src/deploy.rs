// SPDX-License-Identifier: AGPL-3.0-or-later

//! Deploy graph parsing and structural validation.
//!
//! Loads biomeOS BYOB deploy graphs from TOML, validates their structure,
//! and optionally probes whether each node's primal is actually running.
//! This bridges the gap between static graph specs and live deployments.
//!
//! # Graph TOML Format
//!
//! ```toml
//! [graph]
//! name = "primalspring_coordination_niche"
//! version = "0.2.0"
//!
//! [[graph.node]]
//! name = "beardog"
//! binary = "beardog_primal"
//! order = 1
//! required = true
//! health_method = "health"
//! capabilities = ["crypto.sign"]
//! ```
//!
//! # Live Validation
//!
//! [`validate_live`] probes each node's primal socket and returns
//! per-node health status alongside structural validation results.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::coordination::probe_primal;

/// A parsed biomeOS deploy graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployGraph {
    /// Top-level graph metadata.
    pub graph: GraphMeta,
}

/// Graph metadata and node list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphMeta {
    /// Graph identifier (e.g. `"primalspring_coordination_niche"`).
    pub name: String,
    /// Human-readable description.
    #[serde(default)]
    pub description: String,
    /// Semantic version of this graph.
    #[serde(default)]
    pub version: String,
    /// Coordination pattern (Sequential, Parallel, etc.).
    #[serde(default)]
    pub coordination: Option<String>,
    /// Ordered list of primal nodes.
    #[serde(default)]
    pub node: Vec<GraphNode>,
}

/// A single node in a deploy graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    /// Primal name (e.g. `"beardog"`).
    pub name: String,
    /// Binary to invoke (e.g. `"beardog_primal"`).
    pub binary: String,
    /// Startup order (1-indexed).
    pub order: u32,
    /// Whether the deployment fails if this node can't start.
    #[serde(default)]
    pub required: bool,
    /// Nodes that must be healthy before this one starts.
    #[serde(default)]
    pub depends_on: Vec<String>,
    /// JSON-RPC method name for health probing.
    #[serde(default)]
    pub health_method: String,
    /// Capability routing key (optional).
    #[serde(default)]
    pub by_capability: Option<String>,
    /// Capabilities this node provides.
    #[serde(default)]
    pub capabilities: Vec<String>,
    /// Condition predicate for conditional DAG execution.
    #[serde(default)]
    pub condition: Option<String>,
    /// Skip predicate for conditional DAG execution.
    #[serde(default)]
    pub skip_if: Option<String>,
}

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

/// Load a deploy graph from a TOML file path.
///
/// # Errors
///
/// Returns a string description if reading or parsing fails.
pub fn load_graph(path: &Path) -> Result<DeployGraph, String> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read {}: {e}", path.display()))?;
    toml::from_str(&contents).map_err(|e| format!("failed to parse {}: {e}", path.display()))
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
            issues: vec![e],
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
    let nodes: Vec<NodeHealth> = graph
        .graph
        .node
        .iter()
        .map(|node| {
            let health = probe_primal(&node.name);
            NodeHealth {
                name: node.name.clone(),
                required: node.required,
                socket_found: health.socket_found,
                health_ok: health.health_ok,
                capabilities: health.capabilities,
            }
        })
        .collect();

    let healthy_count = nodes.iter().filter(|n| n.health_ok).count();
    let all_required_healthy = nodes.iter().filter(|n| n.required).all(|n| n.health_ok);

    LiveGraphValidation {
        structure,
        nodes,
        healthy_count,
        all_required_healthy,
    }
}

fn structural_checks(graph: &DeployGraph, issues: &mut Vec<String>) {
    if graph.graph.name.is_empty() {
        issues.push("graph.name is empty".to_owned());
    }
    if graph.graph.node.is_empty() {
        issues.push("graph has no nodes".to_owned());
    }

    let names: Vec<&str> = graph.graph.node.iter().map(|n| n.name.as_str()).collect();
    for node in &graph.graph.node {
        if node.name.is_empty() {
            issues.push(format!("node at order {} has empty name", node.order));
        }
        if node.binary.is_empty() {
            issues.push(format!("node '{}' has empty binary", node.name));
        }
        if node.health_method.is_empty() {
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
    if orders.len() != graph.graph.node.len() {
        issues.push("duplicate order values in graph nodes".to_owned());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_primalspring_deploy_graph() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs/primalspring_deploy.toml");
        let graph = load_graph(&path).unwrap();
        assert_eq!(graph.graph.name, "primalspring_coordination_niche");
        assert!(!graph.graph.node.is_empty());
        assert_eq!(graph.graph.node[0].name, "beardog");
    }

    #[test]
    fn load_coralforge_pipeline() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs/coralforge_pipeline.toml");
        let graph = load_graph(&path).unwrap();
        assert_eq!(graph.graph.name, "coralforge_pipeline");
        assert_eq!(graph.graph.coordination.as_deref(), Some("Pipeline"));
    }

    #[test]
    fn load_conditional_fallback() {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs/conditional_fallback.toml");
        let graph = load_graph(&path).unwrap();
        assert_eq!(graph.graph.name, "conditional_fallback");
        let toadstool = graph
            .graph
            .node
            .iter()
            .find(|n| n.name == "toadstool")
            .unwrap();
        assert!(toadstool.condition.is_some());
    }

    #[test]
    fn validate_structure_primalspring_deploy() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs/primalspring_deploy.toml");
        let result = validate_structure(&path);
        assert!(result.parsed);
        assert!(result.issues.is_empty(), "issues: {:?}", result.issues);
        assert!(result.required_count >= 2);
    }

    #[test]
    fn validate_structure_all_graphs_clean() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs");
        let results = validate_all_graphs(&dir);
        assert!(!results.is_empty());
        for r in &results {
            assert!(r.parsed, "graph {} failed to parse", r.path);
            assert!(
                r.issues.is_empty(),
                "graph {} has issues: {:?}",
                r.path,
                r.issues
            );
        }
    }

    #[test]
    fn validate_structure_nonexistent_path() {
        let result = validate_structure(Path::new("/nonexistent/graph.toml"));
        assert!(!result.parsed);
        assert!(!result.issues.is_empty());
    }

    #[test]
    fn structural_checks_detect_empty_name() {
        let graph = DeployGraph {
            graph: GraphMeta {
                name: String::new(),
                description: String::new(),
                version: String::new(),
                coordination: None,
                node: vec![],
            },
        };
        let mut issues = Vec::new();
        structural_checks(&graph, &mut issues);
        assert!(issues.iter().any(|i| i.contains("name is empty")));
        assert!(issues.iter().any(|i| i.contains("no nodes")));
    }

    #[test]
    fn structural_checks_detect_missing_dependency() {
        let graph = DeployGraph {
            graph: GraphMeta {
                name: "test".to_owned(),
                description: String::new(),
                version: String::new(),
                coordination: None,
                node: vec![GraphNode {
                    name: "alpha".to_owned(),
                    binary: "alpha_primal".to_owned(),
                    order: 1,
                    required: true,
                    depends_on: vec!["nonexistent".to_owned()],
                    health_method: "health".to_owned(),
                    by_capability: None,
                    capabilities: vec![],
                    condition: None,
                    skip_if: None,
                }],
            },
        };
        let mut issues = Vec::new();
        structural_checks(&graph, &mut issues);
        assert!(issues.iter().any(|i| i.contains("nonexistent")));
    }

    #[test]
    fn validate_all_graphs_empty_on_nonexistent_dir() {
        let results = validate_all_graphs(Path::new("/nonexistent/dir/graphs"));
        assert!(results.is_empty());
    }

    #[test]
    fn validate_live_nonexistent_path_degrades() {
        let result = validate_live(Path::new("/nonexistent/graph.toml"));
        assert!(!result.structure.parsed);
        assert!(!result.all_required_healthy);
        assert!(result.nodes.is_empty());
    }

    #[test]
    fn validate_live_primalspring_deploy_degrades_gracefully() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs/primalspring_deploy.toml");
        let result = validate_live(&path);
        assert!(result.structure.parsed);
        assert!(result.structure.issues.is_empty());
        assert!(!result.nodes.is_empty());
    }

    #[test]
    fn structural_checks_detect_duplicate_orders() {
        let graph = DeployGraph {
            graph: GraphMeta {
                name: "dup_orders".to_owned(),
                description: String::new(),
                version: String::new(),
                coordination: None,
                node: vec![
                    GraphNode {
                        name: "alpha".to_owned(),
                        binary: "alpha_primal".to_owned(),
                        order: 1,
                        required: true,
                        depends_on: vec![],
                        health_method: "health".to_owned(),
                        by_capability: None,
                        capabilities: vec![],
                        condition: None,
                        skip_if: None,
                    },
                    GraphNode {
                        name: "beta".to_owned(),
                        binary: "beta_primal".to_owned(),
                        order: 1,
                        required: false,
                        depends_on: vec![],
                        health_method: "health".to_owned(),
                        by_capability: None,
                        capabilities: vec![],
                        condition: None,
                        skip_if: None,
                    },
                ],
            },
        };
        let mut issues = Vec::new();
        structural_checks(&graph, &mut issues);
        assert!(issues.iter().any(|i| i.contains("duplicate order")));
    }

    #[test]
    fn structural_checks_detect_empty_binary() {
        let graph = DeployGraph {
            graph: GraphMeta {
                name: "test".to_owned(),
                description: String::new(),
                version: String::new(),
                coordination: None,
                node: vec![GraphNode {
                    name: "alpha".to_owned(),
                    binary: String::new(),
                    order: 1,
                    required: true,
                    depends_on: vec![],
                    health_method: "health".to_owned(),
                    by_capability: None,
                    capabilities: vec![],
                    condition: None,
                    skip_if: None,
                }],
            },
        };
        let mut issues = Vec::new();
        structural_checks(&graph, &mut issues);
        assert!(issues.iter().any(|i| i.contains("empty binary")));
    }
}
