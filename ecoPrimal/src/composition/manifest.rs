// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! NUCLEUS composition manifest — `biome.yaml` consumer.
//!
//! Parses toadStool's canonical v1 `BiomeManifest` schema and provides
//! composition lifecycle operations: topological sorting, readiness
//! validation, and live-state reconciliation.
//!
//! primalSpring consumes the manifest; toadStool owns the schema.
//! When the toadStool crate is available as a workspace dependency,
//! this module should re-export from `toadstool_core::manifest` instead.

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

use serde::{Deserialize, Serialize};

/// Canonical biome manifest — compatible with toadStool v1 schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeManifest {
    /// Schema version (e.g. `"v1"`)
    #[serde(default = "default_v1")]
    pub api_version: String,

    /// Manifest kind — always `"Biome"`
    #[serde(default = "default_biome")]
    pub kind: String,

    /// Biome identity and metadata
    pub metadata: BiomeMetadata,

    /// Primal configurations keyed by lowercase slug
    #[serde(default)]
    pub primals: HashMap<String, ManifestPrimalConfig>,

    /// Service definitions keyed by service name
    #[serde(default)]
    pub services: HashMap<String, serde_json::Value>,

    /// NUCLEUS composition sub-graphs
    #[serde(default)]
    pub compositions: Vec<CompositionGraph>,

    /// Resource limits for the entire biome
    #[serde(default)]
    pub resources: Option<serde_json::Value>,

    /// Security policies
    #[serde(default)]
    pub security: Option<ManifestSecurity>,

    /// Network configuration
    #[serde(default)]
    pub networking: Option<serde_json::Value>,

    /// Federation configuration
    #[serde(default)]
    pub federation: Option<ManifestFederation>,
}

fn default_v1() -> String {
    "v1".to_string()
}

fn default_biome() -> String {
    "Biome".to_string()
}

/// Biome metadata — identity, versioning, labels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeMetadata {
    /// Biome display name
    pub name: String,

    /// Semantic version string
    pub version: String,

    /// Human-readable description
    #[serde(default)]
    pub description: Option<String>,

    /// Team or organization
    #[serde(default)]
    pub team: Option<String>,

    /// Deployment environment
    #[serde(default)]
    pub environment: Option<String>,

    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,

    /// Key-value labels
    #[serde(default)]
    pub labels: HashMap<String, String>,

    /// Annotations (opaque metadata)
    #[serde(default)]
    pub annotations: HashMap<String, String>,
}

/// Primal configuration within a manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestPrimalConfig {
    /// Primal version
    #[serde(default)]
    pub version: Option<String>,

    /// Whether the primal is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Workload source specification
    #[serde(default)]
    pub source: Option<ManifestWorkloadSource>,

    /// Declared capabilities
    #[serde(default)]
    pub capabilities: Vec<String>,

    /// Primal names this depends on
    #[serde(default)]
    pub dependencies: Vec<String>,

    /// Gossip injection events
    #[serde(default)]
    pub gossip_events: Vec<String>,
}

fn default_true() -> bool {
    true
}

/// Source for loading a primal binary.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ManifestWorkloadSource {
    /// Native binary on the local filesystem
    Native {
        /// Path to binary (resolved from depot or absolute)
        path: String,
        /// Command-line arguments
        #[serde(default)]
        args: Vec<String>,
    },
    /// OCI container
    Container {
        /// Image name
        image: String,
        /// Tag
        #[serde(default = "default_latest")]
        tag: String,
    },
    /// WASM module
    Wasm {
        /// Path or URL
        source: String,
    },
}

fn default_latest() -> String {
    "latest".to_string()
}

/// NUCLEUS composition sub-graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionGraph {
    /// Composition name (e.g. `"tower-atomic"`)
    pub name: String,

    /// Composition kind
    #[serde(default)]
    pub kind: CompositionKind,

    /// Primal slugs included in this composition
    #[serde(default)]
    pub members: Vec<String>,

    /// Dependency edges: `{"songbird": ["biomeos"]}` means songbird
    /// depends on biomeos starting first
    #[serde(default)]
    pub dependencies: HashMap<String, Vec<String>>,

    /// Whether this composition auto-starts
    #[serde(default = "default_true")]
    pub auto_start: bool,

    /// Start order priority (lower = first)
    #[serde(default)]
    pub priority: u32,

    /// Readiness criteria
    #[serde(default)]
    pub readiness: Option<CompositionReadiness>,
}

/// Atomic composition kinds.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompositionKind {
    /// Tower Atomic — core infrastructure
    Tower,
    /// Nest Atomic — storage and data federation
    Nest,
    /// Node Atomic — compute dispatch
    Node,
    /// Custom composition
    Custom,
}

impl Default for CompositionKind {
    fn default() -> Self {
        Self::Custom
    }
}

/// Readiness criteria for a composition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionReadiness {
    /// Members that must pass health checks
    #[serde(default)]
    pub require_healthy: Vec<String>,

    /// Timeout before marking as failed (seconds)
    #[serde(default = "default_120")]
    pub timeout_secs: u64,
}

fn default_120() -> u64 {
    120
}

/// Security configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestSecurity {
    /// Isolation level
    #[serde(default)]
    pub isolation_level: Option<String>,

    /// Trust level
    #[serde(default)]
    pub trust_level: Option<String>,

    /// Whether a crypto provider is required
    #[serde(default)]
    pub crypto_required: bool,
}

/// Federation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestFederation {
    /// Whether federation is enabled
    #[serde(default)]
    pub enabled: bool,

    /// Peer gate names
    #[serde(default)]
    pub peers: Vec<String>,

    /// Replication strategy
    #[serde(default)]
    pub replication: Option<String>,
}

// ── Loading ─────────────────────────────────────────────────────────────────

/// Errors from loading or validating a biome manifest.
#[derive(Debug, thiserror::Error)]
pub enum ManifestError {
    /// File I/O error
    #[error("cannot read manifest: {0}")]
    Io(#[from] std::io::Error),

    /// YAML parse error
    #[error("cannot parse manifest: {0}")]
    Yaml(#[from] serde_yaml_ng::Error),

    /// Structural validation error
    #[error("manifest validation: {0}")]
    Validation(String),

    /// Dependency cycle detected
    #[error("dependency cycle in composition {composition}: {cycle}")]
    Cycle {
        /// Composition name
        composition: String,
        /// Cycle description
        cycle: String,
    },
}

/// Load and validate a `biome.yaml` manifest from disk.
pub fn load_biome_manifest(path: &Path) -> Result<BiomeManifest, ManifestError> {
    let contents = std::fs::read_to_string(path)?;
    let manifest: BiomeManifest = serde_yaml_ng::from_str(&contents)?;
    validate_manifest(&manifest)?;
    Ok(manifest)
}

/// Structural validation of a parsed manifest.
pub fn validate_manifest(manifest: &BiomeManifest) -> Result<(), ManifestError> {
    if manifest.metadata.name.is_empty() {
        return Err(ManifestError::Validation(
            "metadata.name is required".to_string(),
        ));
    }
    if manifest.metadata.version.is_empty() {
        return Err(ManifestError::Validation(
            "metadata.version is required".to_string(),
        ));
    }

    let primal_names: HashSet<&str> = manifest.primals.keys().map(String::as_str).collect();

    for comp in &manifest.compositions {
        for member in &comp.members {
            if !primal_names.contains(member.as_str()) {
                return Err(ManifestError::Validation(format!(
                    "composition '{}' references unknown primal '{member}'",
                    comp.name
                )));
            }
        }
        for (dep, requires) in &comp.dependencies {
            if !comp.members.contains(dep) {
                return Err(ManifestError::Validation(format!(
                    "composition '{}' dependency key '{dep}' is not a member",
                    comp.name
                )));
            }
            for req in requires {
                if !comp.members.contains(req) {
                    return Err(ManifestError::Validation(format!(
                        "composition '{}': '{dep}' depends on '{req}' which is not a member",
                        comp.name
                    )));
                }
            }
        }

        topological_sort(&comp.name, &comp.members, &comp.dependencies)?;
    }

    Ok(())
}

// ── Topological Sort ────────────────────────────────────────────────────────

/// Compute a topological start order for composition members based on
/// their dependency edges. Returns members in waves (each wave can
/// start in parallel; waves must be sequential).
pub fn topological_waves(
    composition: &CompositionGraph,
) -> Result<Vec<Vec<String>>, ManifestError> {
    topological_sort(&composition.name, &composition.members, &composition.dependencies)
}

/// Flatten composition members into a single dependency-ordered list.
pub fn topological_order(
    composition: &CompositionGraph,
) -> Result<Vec<String>, ManifestError> {
    let waves = topological_waves(composition)?;
    Ok(waves.into_iter().flatten().collect())
}

fn topological_sort(
    comp_name: &str,
    members: &[String],
    deps: &HashMap<String, Vec<String>>,
) -> Result<Vec<Vec<String>>, ManifestError> {
    let member_set: HashSet<&str> = members.iter().map(String::as_str).collect();
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    let mut adjacency: HashMap<&str, Vec<&str>> = HashMap::new();

    for m in &member_set {
        in_degree.entry(m).or_insert(0);
        adjacency.entry(m).or_default();
    }

    for (node, requires) in deps {
        if !member_set.contains(node.as_str()) {
            continue;
        }
        for req in requires {
            if member_set.contains(req.as_str()) {
                adjacency.entry(req.as_str()).or_default().push(node.as_str());
                *in_degree.entry(node.as_str()).or_insert(0) += 1;
            }
        }
    }

    let mut waves = Vec::new();
    let mut queue: VecDeque<&str> = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&node, _)| node)
        .collect();

    let mut sorted: Vec<&str> = queue.iter().copied().collect();
    sorted.sort_unstable();
    queue.clear();
    queue.extend(sorted.iter());

    let mut processed = 0usize;

    while !queue.is_empty() {
        let wave: Vec<String> = queue.drain(..).map(|s| s.to_string()).collect();
        processed += wave.len();

        let mut next_wave = Vec::new();
        for node in &wave {
            if let Some(dependents) = adjacency.get(node.as_str()) {
                for &dep in dependents {
                    let deg = in_degree.get_mut(dep).expect("member tracked");
                    *deg -= 1;
                    if *deg == 0 {
                        next_wave.push(dep);
                    }
                }
            }
        }
        next_wave.sort_unstable();
        queue.extend(next_wave);
        waves.push(wave);
    }

    if processed != member_set.len() {
        let stuck: Vec<&str> = in_degree
            .iter()
            .filter(|&(_, &deg)| deg > 0)
            .map(|(&n, _)| n)
            .collect();
        return Err(ManifestError::Cycle {
            composition: comp_name.to_string(),
            cycle: stuck.join(" -> "),
        });
    }

    Ok(waves)
}

// ── Composition Lifecycle ───────────────────────────────────────────────────

/// A resolved composition ready for execution.
#[derive(Debug, Clone)]
pub struct ResolvedComposition {
    /// Composition graph metadata
    pub graph: CompositionGraph,
    /// Primals in dependency-ordered start sequence (flattened waves)
    pub start_order: Vec<String>,
    /// Waves for parallel startup within each wave
    pub waves: Vec<Vec<String>>,
    /// Primal configs from the manifest
    pub primal_configs: HashMap<String, ManifestPrimalConfig>,
}

/// Resolve a manifest's compositions into executable plans, ordered by
/// priority. Each composition gets a topologically sorted start order.
pub fn resolve_compositions(
    manifest: &BiomeManifest,
) -> Result<Vec<ResolvedComposition>, ManifestError> {
    let mut auto_start: Vec<&CompositionGraph> = manifest
        .compositions
        .iter()
        .filter(|c| c.auto_start)
        .collect();
    auto_start.sort_by_key(|c| c.priority);

    let mut resolved = Vec::with_capacity(auto_start.len());
    for comp in auto_start {
        let waves = topological_waves(comp)?;
        let start_order: Vec<String> = waves.iter().flatten().cloned().collect();
        let primal_configs: HashMap<String, ManifestPrimalConfig> = comp
            .members
            .iter()
            .filter_map(|name| {
                manifest
                    .primals
                    .get(name)
                    .map(|cfg| (name.clone(), cfg.clone()))
            })
            .collect();

        resolved.push(ResolvedComposition {
            graph: comp.clone(),
            start_order,
            waves,
            primal_configs,
        });
    }

    Ok(resolved)
}

/// Collect all unique primals across all auto-start compositions,
/// in a globally consistent start order (respecting composition priority
/// and internal dependency ordering). Deduplicates across compositions.
pub fn global_start_order(
    manifest: &BiomeManifest,
) -> Result<Vec<String>, ManifestError> {
    let compositions = resolve_compositions(manifest)?;
    let mut seen = HashSet::new();
    let mut order = Vec::new();

    for comp in &compositions {
        for primal in &comp.start_order {
            if seen.insert(primal.clone()) {
                order.push(primal.clone());
            }
        }
    }

    Ok(order)
}

// ---------------------------------------------------------------------------
// Multi-Composition Orchestration
// ---------------------------------------------------------------------------

/// A workflow step targeting a composition or capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    /// Step identifier (for dependency references)
    pub id: String,
    /// Which composition(s) this step operates on
    pub target: WorkflowTarget,
    /// Action to perform
    pub action: WorkflowAction,
    /// Steps that must complete before this one begins
    #[serde(default)]
    pub depends_on: Vec<String>,
    /// Timeout for this step
    #[serde(default = "default_step_timeout_secs")]
    pub timeout_secs: u64,
}

fn default_step_timeout_secs() -> u64 {
    120
}

/// What a workflow step targets.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowTarget {
    /// Target a single composition by name
    Composition(String),
    /// Target a specific primal by slug
    Primal(String),
    /// Target all compositions (global)
    All,
}

/// Action a workflow step performs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowAction {
    /// Start the target (respecting dependency ordering)
    Start,
    /// Stop the target (reverse dependency ordering)
    Stop,
    /// Health check the target
    HealthCheck,
    /// Invoke a capability on the target
    CapabilityCall {
        /// Capability domain
        capability: String,
        /// Operation to invoke
        operation: String,
    },
    /// Wait for a readiness gate
    AwaitReady,
    /// Reconcile manifest against live state
    Reconcile,
}

/// A multi-step composition workflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionWorkflow {
    /// Workflow name
    pub name: String,
    /// Optional description
    #[serde(default)]
    pub description: String,
    /// Ordered steps (topologically resolved by `depends_on`)
    pub steps: Vec<WorkflowStep>,
}

/// Result of executing a single workflow step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    /// Step ID
    pub id: String,
    /// Whether the step succeeded
    pub success: bool,
    /// Human-readable outcome
    pub message: String,
    /// Elapsed milliseconds
    pub elapsed_ms: u64,
}

/// Result of executing an entire workflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {
    /// Workflow name
    pub name: String,
    /// Results per step
    pub steps: Vec<StepResult>,
    /// Whether all steps passed
    pub success: bool,
    /// Total elapsed milliseconds
    pub total_ms: u64,
}

/// Resolve a workflow's step ordering using topological sort on `depends_on`.
///
/// Returns steps grouped into parallel waves — steps in the same wave have
/// no mutual dependencies and can execute concurrently.
pub fn resolve_workflow_waves(
    workflow: &CompositionWorkflow,
) -> Result<Vec<Vec<&WorkflowStep>>, ManifestError> {
    let step_map: HashMap<&str, &WorkflowStep> = workflow
        .steps
        .iter()
        .map(|s| (s.id.as_str(), s))
        .collect();

    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    for step in &workflow.steps {
        in_degree.entry(step.id.as_str()).or_insert(0);
        for dep in &step.depends_on {
            if !step_map.contains_key(dep.as_str()) {
                return Err(ManifestError::Validation(format!(
                    "workflow '{}': step '{}' depends on unknown step '{dep}'",
                    workflow.name, step.id
                )));
            }
            *in_degree.entry(step.id.as_str()).or_insert(0) += 1;
        }
    }

    let mut waves = Vec::new();
    let mut remaining: HashSet<&str> = workflow.steps.iter().map(|s| s.id.as_str()).collect();

    while !remaining.is_empty() {
        let wave: Vec<&str> = remaining
            .iter()
            .copied()
            .filter(|id| {
                in_degree.get(id).copied().unwrap_or(0) == 0
            })
            .collect();

        if wave.is_empty() {
            return Err(ManifestError::Cycle {
                composition: workflow.name.clone(),
                cycle: remaining.iter().copied().collect::<Vec<_>>().join(" -> "),
            });
        }

        for &id in &wave {
            remaining.remove(id);
            for step in &workflow.steps {
                if step.depends_on.iter().any(|d| d == id) {
                    *in_degree.entry(step.id.as_str()).or_default() -= 1;
                }
            }
        }

        waves.push(wave.into_iter().filter_map(|id| step_map.get(id).copied()).collect());
    }

    Ok(waves)
}

/// Build the standard NUCLEUS startup workflow from a manifest.
///
/// Produces a workflow that starts compositions in priority order,
/// awaits readiness on each, then performs a final reconciliation.
pub fn nucleus_startup_workflow(manifest: &BiomeManifest) -> CompositionWorkflow {
    let mut steps = Vec::new();
    let mut prev_id: Option<String> = None;

    let mut comps: Vec<&CompositionGraph> = manifest
        .compositions
        .iter()
        .filter(|c| c.auto_start)
        .collect();
    comps.sort_by_key(|c| c.priority);

    for comp in &comps {
        let start_id = format!("start_{}", comp.name.replace('-', "_"));
        steps.push(WorkflowStep {
            id: start_id.clone(),
            target: WorkflowTarget::Composition(comp.name.clone()),
            action: WorkflowAction::Start,
            depends_on: prev_id.iter().cloned().collect(),
            timeout_secs: comp
                .readiness
                .as_ref()
                .map(|r| r.timeout_secs.into())
                .unwrap_or(120),
        });

        let ready_id = format!("ready_{}", comp.name.replace('-', "_"));
        steps.push(WorkflowStep {
            id: ready_id.clone(),
            target: WorkflowTarget::Composition(comp.name.clone()),
            action: WorkflowAction::AwaitReady,
            depends_on: vec![start_id],
            timeout_secs: comp
                .readiness
                .as_ref()
                .map(|r| r.timeout_secs.into())
                .unwrap_or(60),
        });

        prev_id = Some(ready_id);
    }

    steps.push(WorkflowStep {
        id: "final_reconcile".to_string(),
        target: WorkflowTarget::All,
        action: WorkflowAction::Reconcile,
        depends_on: prev_id.into_iter().collect(),
        timeout_secs: 30,
    });

    CompositionWorkflow {
        name: format!("{}_startup", manifest.metadata.name),
        description: format!(
            "Standard NUCLEUS startup for {} ({} compositions)",
            manifest.metadata.name,
            comps.len()
        ),
        steps,
    }
}

/// Build a graceful shutdown workflow (reverse of startup).
pub fn nucleus_shutdown_workflow(manifest: &BiomeManifest) -> CompositionWorkflow {
    let mut steps = Vec::new();
    let mut prev_id: Option<String> = None;

    let mut comps: Vec<&CompositionGraph> = manifest
        .compositions
        .iter()
        .filter(|c| c.auto_start)
        .collect();
    comps.sort_by_key(|c| std::cmp::Reverse(c.priority));

    for comp in &comps {
        let stop_id = format!("stop_{}", comp.name.replace('-', "_"));
        steps.push(WorkflowStep {
            id: stop_id.clone(),
            target: WorkflowTarget::Composition(comp.name.clone()),
            action: WorkflowAction::Stop,
            depends_on: prev_id.iter().cloned().collect(),
            timeout_secs: 30,
        });
        prev_id = Some(stop_id);
    }

    CompositionWorkflow {
        name: format!("{}_shutdown", manifest.metadata.name),
        description: format!(
            "Graceful NUCLEUS shutdown for {} (reverse priority)",
            manifest.metadata.name
        ),
        steps,
    }
}

/// Build a health-check workflow that probes all compositions in parallel.
pub fn nucleus_health_workflow(manifest: &BiomeManifest) -> CompositionWorkflow {
    let steps: Vec<WorkflowStep> = manifest
        .compositions
        .iter()
        .map(|comp| WorkflowStep {
            id: format!("health_{}", comp.name.replace('-', "_")),
            target: WorkflowTarget::Composition(comp.name.clone()),
            action: WorkflowAction::HealthCheck,
            depends_on: Vec::new(),
            timeout_secs: 15,
        })
        .collect();

    CompositionWorkflow {
        name: format!("{}_health", manifest.metadata.name),
        description: "Parallel health check across all compositions".to_string(),
        steps,
    }
}

/// Readiness result for a single composition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionReadinessResult {
    /// Composition name
    pub name: String,
    /// Composition kind
    pub kind: String,
    /// Whether all readiness criteria are met
    pub ready: bool,
    /// Members that are healthy
    pub healthy_members: Vec<String>,
    /// Members that are unhealthy or missing
    pub unhealthy_members: Vec<String>,
}

/// Summary of manifest reconciliation against live NUCLEUS state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestReconciliation {
    /// Gate name from manifest
    pub gate: String,
    /// Total primals declared in manifest
    pub declared: usize,
    /// Primals found alive on the gate
    pub alive: usize,
    /// Primals missing from the gate
    pub missing: Vec<String>,
    /// Extra primals on the gate not in manifest
    pub extra: Vec<String>,
    /// Composition readiness results
    pub compositions: Vec<CompositionReadinessResult>,
}

/// Reconcile a manifest against a live NUCLEUS state by checking socket
/// existence. Returns a summary of what matches and what diverges.
pub fn reconcile_with_live(
    manifest: &BiomeManifest,
) -> ManifestReconciliation {
    let socket_dir = crate::tolerances::biomeos_socket_dir();

    let declared: HashSet<String> = manifest
        .primals
        .keys()
        .filter(|name| manifest.primals[name.as_str()].enabled)
        .cloned()
        .collect();

    let mut alive_set = HashSet::new();
    let mut missing = Vec::new();

    for primal_slug in &declared {
        let sock = socket_dir.join(format!("{primal_slug}.sock"));
        let tarpc_sock = socket_dir.join(format!("{primal_slug}.tarpc.sock"));
        let neural_sock = socket_dir.join(format!("{primal_slug}-neural.sock"));
        if sock.exists() || tarpc_sock.exists() || neural_sock.exists() {
            alive_set.insert(primal_slug.clone());
        } else if primal_slug == "biomeos" {
            let bio_neural = socket_dir.join("biomeos-neural.sock");
            let bio_api = std::fs::read_dir(&socket_dir)
                .ok()
                .map(|entries| {
                    entries.filter_map(Result::ok).any(|e| {
                        e.file_name()
                            .to_string_lossy()
                            .starts_with("biomeos-api-")
                    })
                })
                .unwrap_or(false);
            if bio_neural.exists() || bio_api {
                alive_set.insert(primal_slug.clone());
            } else {
                missing.push(primal_slug.clone());
            }
        } else {
            missing.push(primal_slug.clone());
        }
    }
    missing.sort();

    let mut extra = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&socket_dir) {
        for entry in entries.filter_map(Result::ok) {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.ends_with(".sock") {
                continue;
            }
            let slug = name
                .trim_end_matches(".sock")
                .trim_end_matches(".tarpc")
                .trim_end_matches("-health")
                .trim_end_matches("-neural")
                .trim_end_matches("-default");
            if let Some(slug) = slug.split('-').next() {
                if !declared.contains(slug) && !slug.starts_with("biomeos") {
                    let s = slug.to_string();
                    if !extra.contains(&s) {
                        extra.push(s);
                    }
                }
            }
        }
    }
    extra.sort();

    let compositions = manifest
        .compositions
        .iter()
        .map(|comp| {
            let mut healthy = Vec::new();
            let mut unhealthy = Vec::new();
            for member in &comp.members {
                if alive_set.contains(member) {
                    healthy.push(member.clone());
                } else {
                    unhealthy.push(member.clone());
                }
            }
            let ready = comp
                .readiness
                .as_ref()
                .map_or(unhealthy.is_empty(), |r| {
                    r.require_healthy
                        .iter()
                        .all(|name| alive_set.contains(name))
                });
            CompositionReadinessResult {
                name: comp.name.clone(),
                kind: format!("{:?}", comp.kind),
                ready,
                healthy_members: healthy,
                unhealthy_members: unhealthy,
            }
        })
        .collect();

    ManifestReconciliation {
        gate: manifest.metadata.name.clone(),
        declared: declared.len(),
        alive: alive_set.len(),
        missing,
        extra,
        compositions,
    }
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn eastgate_yaml() -> &'static str {
        include_str!("../../../config/biome-eastgate.yaml")
    }

    #[test]
    fn parse_eastgate_manifest() {
        let manifest: BiomeManifest = serde_yaml_ng::from_str(eastgate_yaml()).unwrap();
        assert_eq!(manifest.metadata.name, "eastgate");
        assert_eq!(manifest.api_version, "v1");
        assert_eq!(manifest.primals.len(), 14);
        assert_eq!(manifest.compositions.len(), 3);
    }

    #[test]
    fn validate_eastgate_manifest() {
        let manifest: BiomeManifest = serde_yaml_ng::from_str(eastgate_yaml()).unwrap();
        validate_manifest(&manifest).unwrap();
    }

    #[test]
    fn tower_topological_order() {
        let manifest: BiomeManifest = serde_yaml_ng::from_str(eastgate_yaml()).unwrap();
        let tower = &manifest.compositions[0];
        assert_eq!(tower.kind, CompositionKind::Tower);
        let order = topological_order(tower).unwrap();
        let bio_pos = order.iter().position(|s| s == "biomeos").unwrap();
        let song_pos = order.iter().position(|s| s == "songbird").unwrap();
        let vine_pos = order.iter().position(|s| s == "swarmvine").unwrap();
        assert!(bio_pos < song_pos, "biomeos must start before songbird");
        assert!(song_pos < vine_pos, "songbird must start before swarmvine");
    }

    #[test]
    fn nest_topological_order() {
        let manifest: BiomeManifest = serde_yaml_ng::from_str(eastgate_yaml()).unwrap();
        let nest = manifest
            .compositions
            .iter()
            .find(|c| c.kind == CompositionKind::Nest)
            .unwrap();
        let order = topological_order(nest).unwrap();
        let rhizo_pos = order.iter().position(|s| s == "rhizocrypt").unwrap();
        let loam_pos = order.iter().position(|s| s == "loamspine").unwrap();
        let sweet_pos = order.iter().position(|s| s == "sweetgrass").unwrap();
        assert!(
            rhizo_pos < loam_pos,
            "rhizocrypt must start before loamspine"
        );
        assert!(
            loam_pos < sweet_pos,
            "loamspine must start before sweetgrass"
        );
    }

    #[test]
    fn node_topological_order() {
        let manifest: BiomeManifest = serde_yaml_ng::from_str(eastgate_yaml()).unwrap();
        let node = manifest
            .compositions
            .iter()
            .find(|c| c.kind == CompositionKind::Node)
            .unwrap();
        let order = topological_order(node).unwrap();
        let ts_pos = order.iter().position(|s| s == "toadstool").unwrap();
        let cr_pos = order.iter().position(|s| s == "coralreef").unwrap();
        let bc_pos = order.iter().position(|s| s == "barracuda").unwrap();
        assert!(ts_pos < cr_pos, "toadstool must start before coralreef");
        assert!(cr_pos < bc_pos, "coralreef must start before barracuda");
    }

    #[test]
    fn global_start_order_deduplicates() {
        let manifest: BiomeManifest = serde_yaml_ng::from_str(eastgate_yaml()).unwrap();
        let order = global_start_order(&manifest).unwrap();
        let mut seen = HashSet::new();
        for name in &order {
            assert!(seen.insert(name), "duplicate in global order: {name}");
        }
    }

    #[test]
    fn resolve_compositions_priority_order() {
        let manifest: BiomeManifest = serde_yaml_ng::from_str(eastgate_yaml()).unwrap();
        let resolved = resolve_compositions(&manifest).unwrap();
        assert_eq!(resolved.len(), 3);
        assert_eq!(resolved[0].graph.kind, CompositionKind::Tower);
        assert_eq!(resolved[1].graph.kind, CompositionKind::Nest);
        assert_eq!(resolved[2].graph.kind, CompositionKind::Node);
    }

    #[test]
    fn detect_cycle() {
        let yaml = r#"
api_version: v1
kind: Biome
metadata:
  name: cycle-test
  version: "1.0"
primals:
  a:
    capabilities: []
  b:
    capabilities: []
compositions:
  - name: cyclic
    kind: Custom
    members: [a, b]
    dependencies:
      a: [b]
      b: [a]
"#;
        let manifest: BiomeManifest = serde_yaml_ng::from_str(yaml).unwrap();
        let err = validate_manifest(&manifest).unwrap_err();
        assert!(
            matches!(err, ManifestError::Cycle { .. }),
            "expected cycle error, got: {err}"
        );
    }

    #[test]
    fn invalid_member_reference() {
        let yaml = r#"
api_version: v1
kind: Biome
metadata:
  name: bad-ref
  version: "1.0"
primals:
  a:
    capabilities: []
compositions:
  - name: broken
    kind: Custom
    members: [a, nonexistent]
"#;
        let manifest: BiomeManifest = serde_yaml_ng::from_str(yaml).unwrap();
        let err = validate_manifest(&manifest).unwrap_err();
        assert!(matches!(err, ManifestError::Validation(_)));
    }

    #[test]
    fn minimal_manifest() {
        let yaml = r#"
metadata:
  name: minimal
  version: "1.0"
"#;
        let manifest: BiomeManifest = serde_yaml_ng::from_str(yaml).unwrap();
        validate_manifest(&manifest).unwrap();
        assert_eq!(manifest.api_version, "v1");
        assert!(manifest.primals.is_empty());
        assert!(manifest.compositions.is_empty());
    }

    #[test]
    fn reconcile_returns_structure() {
        let manifest: BiomeManifest = serde_yaml_ng::from_str(eastgate_yaml()).unwrap();
        let recon = reconcile_with_live(&manifest);
        assert_eq!(recon.gate, "eastgate");
        assert_eq!(recon.declared, 14);
        assert_eq!(recon.compositions.len(), 3);
    }

    #[test]
    fn federation_peers() {
        let manifest: BiomeManifest = serde_yaml_ng::from_str(eastgate_yaml()).unwrap();
        let fed = manifest.federation.as_ref().unwrap();
        assert!(fed.enabled);
        assert_eq!(fed.peers.len(), 6);
        assert!(fed.peers.contains(&"sporeGate".to_string()));
    }

    #[test]
    fn startup_workflow_structure() {
        let manifest: BiomeManifest = serde_yaml_ng::from_str(eastgate_yaml()).unwrap();
        let wf = nucleus_startup_workflow(&manifest);
        assert_eq!(wf.name, "eastgate_startup");
        // 3 compositions × 2 (start + ready) + 1 reconcile = 7 steps
        assert_eq!(wf.steps.len(), 7);
        assert!(matches!(wf.steps.last().unwrap().action, WorkflowAction::Reconcile));
    }

    #[test]
    fn shutdown_workflow_reverse_priority() {
        let manifest: BiomeManifest = serde_yaml_ng::from_str(eastgate_yaml()).unwrap();
        let wf = nucleus_shutdown_workflow(&manifest);
        assert_eq!(wf.name, "eastgate_shutdown");
        assert_eq!(wf.steps.len(), 3);
        // Node (priority 20) should stop first, then Nest (10), then Tower (0)
        assert!(wf.steps[0].id.contains("node"));
        assert!(wf.steps[1].id.contains("nest"));
        assert!(wf.steps[2].id.contains("tower"));
    }

    #[test]
    fn health_workflow_parallel() {
        let manifest: BiomeManifest = serde_yaml_ng::from_str(eastgate_yaml()).unwrap();
        let wf = nucleus_health_workflow(&manifest);
        assert_eq!(wf.steps.len(), 3);
        // All steps have zero depends_on — fully parallel
        for step in &wf.steps {
            assert!(step.depends_on.is_empty());
        }
    }

    #[test]
    fn workflow_wave_resolution() {
        let manifest: BiomeManifest = serde_yaml_ng::from_str(eastgate_yaml()).unwrap();
        let wf = nucleus_startup_workflow(&manifest);
        let waves = resolve_workflow_waves(&wf).unwrap();
        // First wave: only the first step (start_tower_atomic, no deps)
        assert_eq!(waves[0].len(), 1);
        assert_eq!(waves[0][0].id, "start_tower_atomic");
    }

    #[test]
    fn workflow_cycle_detection() {
        let wf = CompositionWorkflow {
            name: "cyclic".to_string(),
            description: String::new(),
            steps: vec![
                WorkflowStep {
                    id: "a".to_string(),
                    target: WorkflowTarget::All,
                    action: WorkflowAction::HealthCheck,
                    depends_on: vec!["b".to_string()],
                    timeout_secs: 10,
                },
                WorkflowStep {
                    id: "b".to_string(),
                    target: WorkflowTarget::All,
                    action: WorkflowAction::HealthCheck,
                    depends_on: vec!["a".to_string()],
                    timeout_secs: 10,
                },
            ],
        };
        let err = resolve_workflow_waves(&wf).unwrap_err();
        assert!(matches!(err, ManifestError::Cycle { .. }));
    }
}
