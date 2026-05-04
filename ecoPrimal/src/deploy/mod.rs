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
//! id = "primalspring-coordination-niche"
//! name = "primalspring_coordination_niche"
//! version = "0.2.0"
//!
//! [[graph.nodes]]
//! name = "beardog"
//! binary = "beardog_primal"
//! order = 1
//! required = true
//! health_method = "health.liveness"
//! by_capability = "security"
//! capabilities = ["crypto.sign_ed25519", "crypto.verify_ed25519"]
//! ```
//!
//! The parser also accepts the legacy `[[graph.node]]` (singular) format
//! and top-level `[[nodes]]` (merged into `graph.node` on load).
//!
//! # Live Validation
//!
//! [`validate_live`] probes each node's primal socket and returns
//! per-node health status alongside structural validation results.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::coordination::probe_primal;

/// Typed errors for deploy graph operations.
#[derive(Debug, thiserror::Error)]
pub enum DeployError {
    /// Failed to read a file from disk.
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),
    /// TOML parsing failed.
    #[error("graph parse: {0}")]
    Parse(String),
    /// Fragment resolution failed.
    #[error("fragment resolution: {0}")]
    FragmentResolution(String),
    /// Topological ordering is impossible (cycle or missing node).
    #[error("topological sort: {0}")]
    TopologicalSort(String),
}

/// A parsed biomeOS deploy graph.
///
/// Accepts three TOML node formats:
/// - `[[graph.node]]`  — primalSpring legacy (singular)
/// - `[[graph.nodes]]` — biomeOS native (plural, canonical)
/// - `[[nodes]]`       — top-level shorthand (e.g. `basement_hpc_covalent`)
///
/// Top-level `[[nodes]]` are merged into `graph.node` during [`load_graph`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployGraph {
    /// Top-level graph metadata.
    pub graph: GraphMeta,

    /// Top-level `[[nodes]]` (alternative to `[[graph.node]]` / `[[graph.nodes]]`).
    /// Merged into `graph.node` by [`load_graph`].
    #[serde(default)]
    pub nodes: Vec<GraphNode>,
}

/// Graph metadata and node list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphMeta {
    /// Graph identifier — biomeOS uses hyphenated `id` (e.g. `"nest-deploy"`).
    #[serde(default)]
    pub id: Option<String>,
    /// Graph name (e.g. `"primalspring_coordination_niche"`).
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
    /// Graph metadata sub-table (fragments, `security_model`, etc.).
    #[serde(default)]
    pub metadata: Option<GraphMetadata>,
    /// Ordered list of primal nodes.
    /// Accepts both `[[graph.node]]` (legacy) and `[[graph.nodes]]` (biomeOS native).
    #[serde(default, alias = "nodes")]
    pub node: Vec<GraphNode>,
}

/// Metadata sub-table of a deploy graph (`[graph.metadata]`).
///
/// Only `fragments` + `resolve` are actively used for composition; all
/// other metadata fields (`security_model`, transport, etc.) are preserved
/// as opaque TOML for downstream consumers.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GraphMetadata {
    /// Fragment names to resolve at load time (e.g. `["tower_atomic", "node_atomic"]`).
    /// Each name maps to `fragments/{name}.toml` relative to the graphs root.
    #[serde(default)]
    pub fragments: Vec<String>,
    /// When `true`, fragment nodes are loaded as the base layer and the
    /// graph's own `[[graph.nodes]]` act as delta overrides. When `false`
    /// (default), fragments are metadata-only and the graph's nodes are
    /// used as-is.
    #[serde(default)]
    pub resolve: bool,
}

/// A parsed fragment file (`[fragment]` + `[[fragment.nodes]]`).
#[derive(Debug, Clone, Deserialize)]
struct FragmentFile {
    fragment: FragmentMeta,
}

/// Fragment metadata and node list.
#[derive(Debug, Clone, Deserialize)]
struct FragmentMeta {
    #[expect(dead_code, reason = "structural completeness; used for diagnostics")]
    name: String,
    #[serde(default, alias = "node")]
    nodes: Vec<GraphNode>,
}

/// A single node in a deploy graph.
///
/// Accepts both the standard single-node format (with `name`, `binary`, `order`,
/// `health_method`) and the multi-node bonding format (with `id` instead of
/// `name`, and nested `primal`/`operation`/`constraints` sub-tables). The
/// multi-node sub-tables are captured as opaque TOML values; the standard
/// deploy pipeline uses only the flat fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    /// Primal name (e.g. `"beardog"`).
    /// Multi-node graphs use `id` instead — accepted via alias.
    #[serde(alias = "id", default)]
    pub name: String,
    /// Binary to invoke (e.g. `"beardog_primal"`).
    /// Not present in multi-node bonding graphs.
    #[serde(default)]
    pub binary: String,
    /// Startup order (1-indexed).
    /// Not present in multi-node bonding graphs (order is implicit via `depends_on`).
    #[serde(default)]
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
    /// Multi-node: nested primal sub-table (opaque).
    #[serde(default)]
    pub primal: Option<toml::Value>,
    /// Multi-node: nested operation sub-table (opaque).
    #[serde(default)]
    pub operation: Option<toml::Value>,
    /// Multi-node: nested constraints sub-table (opaque).
    #[serde(default)]
    pub constraints: Option<toml::Value>,
    /// Multi-node: output artifact label.
    #[serde(default)]
    pub output: Option<String>,
}

const fn default_spawn() -> bool {
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
/// Handles three node formats: `[[graph.node]]`, `[[graph.nodes]]` (serde
/// alias), and top-level `[[nodes]]` (merged into `graph.node`).
///
/// If the graph declares `[graph.metadata] fragments = [...]`, each fragment
/// is loaded from `fragments/{name}.toml` (relative to the graphs root) and
/// its nodes are merged as the base layer. The graph's own nodes override
/// any same-name fragment nodes, acting as delta overlays.
///
/// # Errors
///
/// Returns [`DeployError`] if reading or parsing fails.
pub fn load_graph(path: &Path) -> Result<DeployGraph, DeployError> {
    let contents = std::fs::read_to_string(path)?;
    let mut graph: DeployGraph = toml::from_str(&contents)
        .map_err(|e| DeployError::Parse(format!("{}: {e}", path.display())))?;

    if !graph.nodes.is_empty() {
        graph.graph.node.append(&mut graph.nodes);
    }

    resolve_fragments(&mut graph, path)?;

    graph.graph.node.sort_by_key(|n| n.order);
    Ok(graph)
}

/// Resolve fragment references declared in `[graph.metadata].fragments`.
///
/// Only runs when `[graph.metadata] resolve = true`. Locates the
/// `fragments/` directory by searching from the graph file's parent
/// directory upward. Fragment nodes are merged as the base layer;
/// the graph's explicit nodes override any same-name fragment node.
fn resolve_fragments(graph: &mut DeployGraph, graph_path: &Path) -> Result<(), DeployError> {
    let fragment_names = match &graph.graph.metadata {
        Some(meta) if meta.resolve && !meta.fragments.is_empty() => meta.fragments.clone(),
        _ => return Ok(()),
    };

    let fragments_dir = find_fragments_dir(graph_path).ok_or_else(|| {
        DeployError::FragmentResolution(format!(
            "graph {} declares fragments but no fragments/ directory found",
            graph_path.display()
        ))
    })?;

    let mut base_nodes: Vec<GraphNode> = Vec::new();
    for frag_name in &fragment_names {
        let frag_path = fragments_dir.join(format!("{frag_name}.toml"));
        if !frag_path.is_file() {
            continue;
        }
        let frag_contents = std::fs::read_to_string(&frag_path)?;
        let frag_file: FragmentFile = toml::from_str(&frag_contents)
            .map_err(|e| DeployError::Parse(format!("fragment {}: {e}", frag_path.display())))?;
        for frag_node in frag_file.fragment.nodes {
            if !base_nodes.iter().any(|n| n.name == frag_node.name) {
                base_nodes.push(frag_node);
            }
        }
    }

    if base_nodes.is_empty() {
        return Ok(());
    }

    let delta_nodes = std::mem::take(&mut graph.graph.node);
    for delta in delta_nodes {
        if let Some(existing) = base_nodes.iter_mut().find(|n| n.name == delta.name) {
            *existing = delta;
        } else {
            base_nodes.push(delta);
        }
    }
    graph.graph.node = base_nodes;

    Ok(())
}

/// Walk up from the graph file to find the `fragments/` directory.
fn find_fragments_dir(graph_path: &Path) -> Option<std::path::PathBuf> {
    let mut dir = graph_path.parent()?;
    for _ in 0..3 {
        let candidate = dir.join("fragments");
        if candidate.is_dir() {
            return Some(candidate);
        }
        dir = dir.parent()?;
    }
    None
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
/// Given Tower (`beardog` order=1) → Songbird (order=2, `depends_on`=`beardog`):
/// - Wave 0: `["beardog"]`
/// - Wave 1: `["songbird"]`
pub fn topological_waves(graph: &DeployGraph) -> Result<Vec<Vec<String>>, DeployError> {
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
                return Err(DeployError::TopologicalSort(format!(
                    "node '{}' depends on '{}' which is not in the graph",
                    node.name, dep
                )));
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
        return Err(DeployError::TopologicalSort(
            "graph contains a dependency cycle".to_owned(),
        ));
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
        if let Some(existing) = merged_nodes
            .iter_mut()
            .find(|n| n.name == overlay_node.name)
        {
            *existing = overlay_node.clone();
        } else {
            merged_nodes.push(overlay_node.clone());
        }
    }
    merged_nodes.sort_by_key(|n| n.order);

    DeployGraph {
        graph: GraphMeta {
            id: overlay.graph.id.clone().or_else(|| base.graph.id.clone()),
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
            metadata: overlay
                .graph
                .metadata
                .clone()
                .or_else(|| base.graph.metadata.clone()),
            node: merged_nodes,
        },
        nodes: Vec::new(),
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
mod tests;
