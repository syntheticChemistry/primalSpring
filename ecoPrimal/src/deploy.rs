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
    /// Whether the harness should spawn this node as a primal process.
    ///
    /// Defaults to `true`. Set to `false` for validation/coordination
    /// nodes (e.g. `primalspring`) that the harness should not spawn.
    #[serde(default = "default_spawn")]
    pub spawn: bool,
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

fn default_spawn() -> bool {
    true
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

/// Compute topological startup waves from a deploy graph.
///
/// Uses Kahn's algorithm to group nodes into "waves" — each wave contains
/// nodes whose dependencies are all satisfied by earlier waves. This is the
/// biomeOS germination ordering: wave 0 starts first, wave 1 after wave 0 is
/// healthy, and so on.
///
/// # Errors
///
/// Returns `Err` if the graph contains a dependency cycle (impossible to
/// satisfy all `depends_on` constraints) or references a non-existent node.
///
/// # Example
///
/// Given Tower (`beardog` order=1) → Songbird (order=2, depends_on=`beardog`):
/// - Wave 0: `["beardog"]`
/// - Wave 1: `["songbird"]`
pub fn topological_waves(graph: &DeployGraph) -> Result<Vec<Vec<String>>, String> {
    use std::collections::{HashMap, VecDeque};

    let nodes = &graph.graph.node;
    if nodes.is_empty() {
        return Ok(Vec::new());
    }

    let name_set: HashMap<&str, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.name.as_str(), i))
        .collect();

    let mut in_degree: Vec<usize> = vec![0; nodes.len()];
    let mut dependents: Vec<Vec<usize>> = vec![Vec::new(); nodes.len()];

    for (i, node) in nodes.iter().enumerate() {
        for dep in &node.depends_on {
            let Some(&dep_idx) = name_set.get(dep.as_str()) else {
                return Err(format!(
                    "node '{}' depends on '{}' which is not in the graph",
                    node.name, dep
                ));
            };
            in_degree[i] += 1;
            dependents[dep_idx].push(i);
        }
    }

    let mut waves: Vec<Vec<String>> = Vec::new();
    let mut queue: VecDeque<usize> = in_degree
        .iter()
        .enumerate()
        .filter(|&(_, d)| *d == 0)
        .map(|(i, _)| i)
        .collect();
    let mut processed = 0;

    while !queue.is_empty() {
        let wave_size = queue.len();
        let mut wave = Vec::with_capacity(wave_size);
        for _ in 0..wave_size {
            let Some(idx) = queue.pop_front() else {
                break;
            };
            wave.push(nodes[idx].name.clone());
            processed += 1;
            for &dep_idx in &dependents[idx] {
                in_degree[dep_idx] -= 1;
                if in_degree[dep_idx] == 0 {
                    queue.push_back(dep_idx);
                }
            }
        }
        wave.sort();
        waves.push(wave);
    }

    if processed != nodes.len() {
        return Err("graph contains a dependency cycle".to_owned());
    }

    Ok(waves)
}

/// Extract the required capabilities from a graph by reading each node's
/// `by_capability` field.
///
/// This makes the deploy graph the **source of truth** for what capabilities
/// a composition needs — no hardcoded rosters in Rust code.
#[must_use]
pub fn graph_required_capabilities(graph: &DeployGraph) -> Vec<String> {
    graph
        .graph
        .node
        .iter()
        .filter_map(|n| n.by_capability.clone())
        .collect()
}

/// Extract the names of spawnable primal nodes from a graph.
///
/// Returns only nodes where `spawn` is `true` (the default). Validation
/// and coordination nodes set `spawn = false` and are excluded.
#[must_use]
pub fn graph_spawnable_primals(graph: &DeployGraph) -> Vec<String> {
    graph
        .graph
        .node
        .iter()
        .filter(|n| n.spawn)
        .map(|n| n.name.clone())
        .collect()
}

/// Build a capability-to-primal mapping from the graph's spawnable nodes.
///
/// For each node with `spawn = true` and a `by_capability` field, maps
/// `capability -> primal_name`. This lets the harness resolve capabilities
/// for overlay primals that aren't in the static `AtomicType` mapping.
#[must_use]
pub fn graph_capability_map(graph: &DeployGraph) -> std::collections::HashMap<String, String> {
    graph
        .graph
        .node
        .iter()
        .filter(|n| n.spawn)
        .filter_map(|n| {
            n.by_capability
                .as_ref()
                .map(|cap| (cap.clone(), n.name.clone()))
        })
        .collect()
}

/// Merge two deploy graphs: a base graph with an overlay.
///
/// All nodes from the overlay are appended to the base graph. If a node
/// with the same name exists in both, the overlay version wins. The
/// resulting graph's name is `"{base_name}+{overlay_name}"`.
#[must_use]
pub fn merge_graphs(base: &DeployGraph, overlay: &DeployGraph) -> DeployGraph {
    let mut merged_nodes = base.graph.node.clone();
    for overlay_node in &overlay.graph.node {
        if let Some(existing) = merged_nodes.iter_mut().find(|n| n.name == overlay_node.name) {
            *existing = overlay_node.clone();
        } else {
            merged_nodes.push(overlay_node.clone());
        }
    }
    merged_nodes.sort_by_key(|n| n.order);

    DeployGraph {
        graph: GraphMeta {
            name: format!("{}+{}", base.graph.name, overlay.graph.name),
            description: format!(
                "{} merged with {}",
                base.graph.description, overlay.graph.description
            ),
            version: overlay.graph.version.clone(),
            coordination: overlay
                .graph
                .coordination
                .clone()
                .or_else(|| base.graph.coordination.clone()),
            node: merged_nodes,
        },
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
                    spawn: true,
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
                        spawn: true,
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
                        spawn: true,
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
    fn topological_waves_tower_graph() {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs/tower_atomic_bootstrap.toml");
        let graph = load_graph(&path).unwrap();
        let waves = topological_waves(&graph).unwrap();
        assert!(waves.len() >= 2, "tower should have at least 2 waves");
        assert!(
            waves[0].contains(&"beardog".to_owned()),
            "beardog should be in wave 0 (no deps)"
        );
    }

    #[test]
    fn topological_waves_nucleus_complete() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs/nucleus_complete.toml");
        let graph = load_graph(&path).unwrap();
        let waves = topological_waves(&graph).unwrap();
        assert!(waves.len() >= 4, "nucleus should have at least 4 waves");
        assert!(
            waves[0].contains(&"beardog".to_owned()),
            "beardog should be in wave 0"
        );
    }

    #[test]
    fn topological_waves_empty_graph() {
        let graph = DeployGraph {
            graph: GraphMeta {
                name: "empty".to_owned(),
                description: String::new(),
                version: String::new(),
                coordination: None,
                node: vec![],
            },
        };
        let waves = topological_waves(&graph).unwrap();
        assert!(waves.is_empty());
    }

    #[test]
    fn topological_waves_single_node() {
        let graph = DeployGraph {
            graph: GraphMeta {
                name: "single".to_owned(),
                description: String::new(),
                version: String::new(),
                coordination: None,
                node: vec![GraphNode {
                    name: "alpha".to_owned(),
                    binary: "alpha_primal".to_owned(),
                    order: 1,
                    required: true,
                    spawn: true,
                    depends_on: vec![],
                    health_method: "health".to_owned(),
                    by_capability: Some("security".to_owned()),
                    capabilities: vec![],
                    condition: None,
                    skip_if: None,
                }],
            },
        };
        let waves = topological_waves(&graph).unwrap();
        assert_eq!(waves.len(), 1);
        assert_eq!(waves[0], vec!["alpha"]);
    }

    #[test]
    fn topological_waves_detects_missing_dependency() {
        let graph = DeployGraph {
            graph: GraphMeta {
                name: "bad_dep".to_owned(),
                description: String::new(),
                version: String::new(),
                coordination: None,
                node: vec![GraphNode {
                    name: "alpha".to_owned(),
                    binary: "alpha_primal".to_owned(),
                    order: 1,
                    required: true,
                    spawn: true,
                    depends_on: vec!["ghost".to_owned()],
                    health_method: "health".to_owned(),
                    by_capability: None,
                    capabilities: vec![],
                    condition: None,
                    skip_if: None,
                }],
            },
        };
        let result = topological_waves(&graph);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("ghost"));
    }

    #[test]
    fn topological_waves_detects_cycle() {
        let graph = DeployGraph {
            graph: GraphMeta {
                name: "cycle".to_owned(),
                description: String::new(),
                version: String::new(),
                coordination: None,
                node: vec![
                    GraphNode {
                        name: "alpha".to_owned(),
                        binary: "a".to_owned(),
                        order: 1,
                        required: true,
                        spawn: true,
                        depends_on: vec!["beta".to_owned()],
                        health_method: "health".to_owned(),
                        by_capability: None,
                        capabilities: vec![],
                        condition: None,
                        skip_if: None,
                    },
                    GraphNode {
                        name: "beta".to_owned(),
                        binary: "b".to_owned(),
                        order: 2,
                        required: true,
                        spawn: true,
                        depends_on: vec!["alpha".to_owned()],
                        health_method: "health".to_owned(),
                        by_capability: None,
                        capabilities: vec![],
                        condition: None,
                        skip_if: None,
                    },
                ],
            },
        };
        let result = topological_waves(&graph);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cycle"));
    }

    #[test]
    fn topological_waves_parallel_nodes() {
        let graph = DeployGraph {
            graph: GraphMeta {
                name: "parallel".to_owned(),
                description: String::new(),
                version: String::new(),
                coordination: None,
                node: vec![
                    GraphNode {
                        name: "root".to_owned(),
                        binary: "r".to_owned(),
                        order: 1,
                        required: true,
                        spawn: true,
                        depends_on: vec![],
                        health_method: "health".to_owned(),
                        by_capability: None,
                        capabilities: vec![],
                        condition: None,
                        skip_if: None,
                    },
                    GraphNode {
                        name: "leaf_a".to_owned(),
                        binary: "a".to_owned(),
                        order: 2,
                        required: true,
                        spawn: true,
                        depends_on: vec!["root".to_owned()],
                        health_method: "health".to_owned(),
                        by_capability: None,
                        capabilities: vec![],
                        condition: None,
                        skip_if: None,
                    },
                    GraphNode {
                        name: "leaf_b".to_owned(),
                        binary: "b".to_owned(),
                        order: 3,
                        required: true,
                        spawn: true,
                        depends_on: vec!["root".to_owned()],
                        health_method: "health".to_owned(),
                        by_capability: None,
                        capabilities: vec![],
                        condition: None,
                        skip_if: None,
                    },
                ],
            },
        };
        let waves = topological_waves(&graph).unwrap();
        assert_eq!(waves.len(), 2);
        assert_eq!(waves[0], vec!["root"]);
        assert_eq!(waves[1].len(), 2);
        assert!(waves[1].contains(&"leaf_a".to_owned()));
        assert!(waves[1].contains(&"leaf_b".to_owned()));
    }

    #[test]
    fn graph_required_capabilities_from_nucleus() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs/nucleus_complete.toml");
        let graph = load_graph(&path).unwrap();
        let caps = graph_required_capabilities(&graph);
        assert!(caps.contains(&"security".to_owned()));
        assert!(caps.contains(&"discovery".to_owned()));
        assert!(caps.contains(&"compute".to_owned()));
        assert!(caps.contains(&"storage".to_owned()));
        assert!(caps.contains(&"coordination".to_owned()));
    }

    #[test]
    fn graph_required_capabilities_empty_for_no_by_capability() {
        let graph = DeployGraph {
            graph: GraphMeta {
                name: "bare".to_owned(),
                description: String::new(),
                version: String::new(),
                coordination: None,
                node: vec![GraphNode {
                    name: "alpha".to_owned(),
                    binary: "a".to_owned(),
                    order: 1,
                    required: true,
                    spawn: true,
                    depends_on: vec![],
                    health_method: "health".to_owned(),
                    by_capability: None,
                    capabilities: vec![],
                    condition: None,
                    skip_if: None,
                }],
            },
        };
        assert!(graph_required_capabilities(&graph).is_empty());
    }

    #[test]
    fn topological_waves_all_graphs_acyclic() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs");
        let results = validate_all_graphs(&dir);
        for r in &results {
            if !r.parsed {
                continue;
            }
            let graph = load_graph(Path::new(&r.path)).unwrap();
            let waves = topological_waves(&graph);
            assert!(
                waves.is_ok(),
                "graph {} has a cycle: {:?}",
                r.path,
                waves.err()
            );
        }
    }

    #[test]
    fn all_graphs_have_by_capability_on_every_node() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs");
        let skip = ["spring_byob_template.toml"];
        for entry in std::fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "toml") {
                continue;
            }
            let name = path.file_name().unwrap().to_string_lossy();
            if skip.iter().any(|s| name.as_ref() == *s) {
                continue;
            }
            let graph = load_graph(&path).unwrap();
            for node in &graph.graph.node {
                assert!(
                    node.by_capability.is_some(),
                    "graph '{}' node '{}' missing by_capability",
                    name,
                    node.name
                );
            }
        }
    }

    #[test]
    fn graph_spawnable_primals_filters_spawn_false() {
        let graph = DeployGraph {
            graph: GraphMeta {
                name: "overlay_test".to_owned(),
                description: String::new(),
                version: String::new(),
                coordination: None,
                node: vec![
                    GraphNode {
                        name: "beardog".to_owned(),
                        binary: "beardog_primal".to_owned(),
                        order: 1,
                        required: true,
                        spawn: true,
                        depends_on: vec![],
                        health_method: "health".to_owned(),
                        by_capability: Some("security".to_owned()),
                        capabilities: vec![],
                        condition: None,
                        skip_if: None,
                    },
                    GraphNode {
                        name: "primalspring".to_owned(),
                        binary: "primalspring_primal".to_owned(),
                        order: 99,
                        required: false,
                        spawn: false,
                        depends_on: vec![],
                        health_method: "health".to_owned(),
                        by_capability: Some("coordination".to_owned()),
                        capabilities: vec![],
                        condition: None,
                        skip_if: None,
                    },
                ],
            },
        };
        let spawnable = graph_spawnable_primals(&graph);
        assert_eq!(spawnable, vec!["beardog"]);
        assert!(!spawnable.contains(&"primalspring".to_owned()));
    }

    #[test]
    fn graph_capability_map_builds_mapping() {
        let graph = DeployGraph {
            graph: GraphMeta {
                name: "cap_test".to_owned(),
                description: String::new(),
                version: String::new(),
                coordination: None,
                node: vec![
                    GraphNode {
                        name: "beardog".to_owned(),
                        binary: "beardog_primal".to_owned(),
                        order: 1,
                        required: true,
                        spawn: true,
                        depends_on: vec![],
                        health_method: "health".to_owned(),
                        by_capability: Some("security".to_owned()),
                        capabilities: vec![],
                        condition: None,
                        skip_if: None,
                    },
                    GraphNode {
                        name: "squirrel".to_owned(),
                        binary: "squirrel_primal".to_owned(),
                        order: 2,
                        required: false,
                        spawn: true,
                        depends_on: vec![],
                        health_method: "health".to_owned(),
                        by_capability: Some("ai".to_owned()),
                        capabilities: vec![],
                        condition: None,
                        skip_if: None,
                    },
                ],
            },
        };
        let map = graph_capability_map(&graph);
        assert_eq!(map.get("security").unwrap(), "beardog");
        assert_eq!(map.get("ai").unwrap(), "squirrel");
    }

    #[test]
    fn merge_graphs_combines_nodes() {
        let base = DeployGraph {
            graph: GraphMeta {
                name: "base".to_owned(),
                description: "base graph".to_owned(),
                version: "1.0.0".to_owned(),
                coordination: Some("sequential".to_owned()),
                node: vec![GraphNode {
                    name: "beardog".to_owned(),
                    binary: "beardog_primal".to_owned(),
                    order: 1,
                    required: true,
                    spawn: true,
                    depends_on: vec![],
                    health_method: "health".to_owned(),
                    by_capability: Some("security".to_owned()),
                    capabilities: vec![],
                    condition: None,
                    skip_if: None,
                }],
            },
        };
        let overlay = DeployGraph {
            graph: GraphMeta {
                name: "ai_overlay".to_owned(),
                description: "AI overlay".to_owned(),
                version: "1.0.0".to_owned(),
                coordination: None,
                node: vec![GraphNode {
                    name: "squirrel".to_owned(),
                    binary: "squirrel_primal".to_owned(),
                    order: 3,
                    required: false,
                    spawn: true,
                    depends_on: vec!["beardog".to_owned()],
                    health_method: "health".to_owned(),
                    by_capability: Some("ai".to_owned()),
                    capabilities: vec![],
                    condition: None,
                    skip_if: None,
                }],
            },
        };
        let merged = merge_graphs(&base, &overlay);
        assert_eq!(merged.graph.name, "base+ai_overlay");
        assert_eq!(merged.graph.node.len(), 2);
        assert_eq!(merged.graph.node[0].name, "beardog");
        assert_eq!(merged.graph.node[1].name, "squirrel");
    }

    #[test]
    fn merge_graphs_overlay_overrides_existing() {
        let base = DeployGraph {
            graph: GraphMeta {
                name: "base".to_owned(),
                description: String::new(),
                version: "1.0.0".to_owned(),
                coordination: None,
                node: vec![GraphNode {
                    name: "beardog".to_owned(),
                    binary: "beardog_primal".to_owned(),
                    order: 1,
                    required: true,
                    spawn: true,
                    depends_on: vec![],
                    health_method: "health".to_owned(),
                    by_capability: Some("security".to_owned()),
                    capabilities: vec![],
                    condition: None,
                    skip_if: None,
                }],
            },
        };
        let overlay = DeployGraph {
            graph: GraphMeta {
                name: "override".to_owned(),
                description: String::new(),
                version: "2.0.0".to_owned(),
                coordination: None,
                node: vec![GraphNode {
                    name: "beardog".to_owned(),
                    binary: "beardog_v2".to_owned(),
                    order: 1,
                    required: false,
                    spawn: true,
                    depends_on: vec![],
                    health_method: "health.v2".to_owned(),
                    by_capability: Some("security".to_owned()),
                    capabilities: vec![],
                    condition: None,
                    skip_if: None,
                }],
            },
        };
        let merged = merge_graphs(&base, &overlay);
        assert_eq!(merged.graph.node.len(), 1);
        assert_eq!(merged.graph.node[0].binary, "beardog_v2");
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
                    spawn: true,
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
