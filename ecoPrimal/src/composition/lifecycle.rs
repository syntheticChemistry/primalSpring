// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Composition lifecycle executor — deploy→gossip→verify pipeline.
//!
//! Implements the three-phase lifecycle assessment for primals declared in a
//! `BiomeManifest`:
//!
//! 1. **Deploy**: Socket exists — primal process is running and listening.
//! 2. **Gossip**: Primal has registered its declared gossip events with
//!    swarmVine (verified via `gossip.status`).
//! 3. **Verify**: Primal is routable through the biomeOS Neural API mesh
//!    (verified via `primal.list` or `capability.call`).
//!
//! This module provides a structural assessment — it does not spawn processes.
//! Actual deployment is owned by cellMembrane/nucleus_launcher; primalSpring
//! validates lifecycle progression from the outside.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

use serde::{Deserialize, Serialize};

use super::manifest::{BiomeManifest, ManifestPrimalConfig};
use crate::ipc::client::PrimalClient;

/// Lifecycle phase for a primal in the deploy→gossip→verify pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum LifecyclePhase {
    /// Not deployed — no socket found.
    NotDeployed,
    /// Deployed — socket exists, process running.
    Deployed,
    /// Gossip registered — primal's events are in swarmVine.
    GossipRegistered,
    /// Fully verified — routable via biomeOS Neural API mesh.
    Verified,
}

impl LifecyclePhase {
    /// Human-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotDeployed => "not_deployed",
            Self::Deployed => "deployed",
            Self::GossipRegistered => "gossip_registered",
            Self::Verified => "verified",
        }
    }
}

/// Lifecycle state for a single primal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalLifecycleState {
    /// Primal slug (lowercase canonical name).
    pub slug: String,
    /// Current lifecycle phase.
    pub phase: LifecyclePhase,
    /// Declared gossip events from the manifest.
    pub declared_gossip_events: Vec<String>,
    /// Gossip events confirmed as registered in swarmVine.
    pub confirmed_gossip_events: Vec<String>,
    /// Whether the primal appears in biomeOS `primal.list`.
    pub mesh_routable: bool,
    /// Capabilities declared in the manifest.
    pub declared_capabilities: Vec<String>,
    /// Optional error message if a probe failed.
    pub error: Option<String>,
}

/// Full lifecycle report for a biome.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleReport {
    /// Gate name from the manifest.
    pub gate: String,
    /// Per-primal lifecycle states.
    pub primals: Vec<PrimalLifecycleState>,
    /// Summary counts by phase.
    pub summary: LifecycleSummary,
    /// Total assessment time in milliseconds.
    pub elapsed_ms: u64,
}

/// Summary counts for the lifecycle report.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LifecycleSummary {
    /// Primals assessed.
    pub total: usize,
    /// Primals not deployed (no socket).
    pub not_deployed: usize,
    /// Primals deployed but not gossip-registered.
    pub deployed_only: usize,
    /// Primals with gossip but not mesh-verified.
    pub gossip_only: usize,
    /// Primals fully verified (deploy→gossip→verify complete).
    pub verified: usize,
}

/// Assess the full deploy→gossip→verify lifecycle for all primals in a manifest.
///
/// This is a non-destructive read-only assessment. It probes:
/// 1. Socket existence in the biomeOS socket directory
/// 2. swarmVine `gossip.status` for event registration
/// 3. biomeOS `primal.list` for mesh routability
pub fn assess_lifecycle(manifest: &BiomeManifest) -> LifecycleReport {
    let start = Instant::now();
    let socket_dir = crate::tolerances::biomeos_socket_dir();

    let swarmvine_socket = find_primal_socket(&socket_dir, "swarmvine");
    let biomeos_socket = find_biomeos_socket(&socket_dir);

    let mesh_primals = probe_mesh_primals(&biomeos_socket);
    let gossip_registry = probe_gossip_registry(&swarmvine_socket);

    let mut states = Vec::new();

    for (slug, config) in &manifest.primals {
        if !config.enabled {
            continue;
        }

        let state = assess_primal(
            slug,
            config,
            &socket_dir,
            &gossip_registry,
            &mesh_primals,
        );
        states.push(state);
    }

    states.sort_by(|a, b| a.slug.cmp(&b.slug));

    let summary = compute_summary(&states);
    let elapsed_ms = start.elapsed().as_millis() as u64;

    LifecycleReport {
        gate: manifest.metadata.name.clone(),
        primals: states,
        summary,
        elapsed_ms,
    }
}

/// Assess a single primal's lifecycle phase.
fn assess_primal(
    slug: &str,
    config: &ManifestPrimalConfig,
    socket_dir: &std::path::Path,
    gossip_registry: &HashMap<String, Vec<String>>,
    mesh_primals: &[String],
) -> PrimalLifecycleState {
    let declared_gossip_events = config.gossip_events.clone();
    let declared_capabilities = config.capabilities.clone();

    let socket_alive = primal_socket_exists(socket_dir, slug);

    if !socket_alive {
        return PrimalLifecycleState {
            slug: slug.to_string(),
            phase: LifecyclePhase::NotDeployed,
            declared_gossip_events,
            confirmed_gossip_events: Vec::new(),
            mesh_routable: false,
            declared_capabilities,
            error: None,
        };
    }

    let confirmed_gossip_events = gossip_registry
        .get(slug)
        .cloned()
        .unwrap_or_default();

    let gossip_complete = if declared_gossip_events.is_empty() {
        true
    } else {
        declared_gossip_events
            .iter()
            .all(|e| confirmed_gossip_events.contains(e))
    };

    let mesh_routable = mesh_primals.iter().any(|p| p == slug);

    let phase = if mesh_routable && gossip_complete {
        LifecyclePhase::Verified
    } else if gossip_complete {
        LifecyclePhase::GossipRegistered
    } else {
        LifecyclePhase::Deployed
    };

    PrimalLifecycleState {
        slug: slug.to_string(),
        phase,
        declared_gossip_events,
        confirmed_gossip_events,
        mesh_routable,
        declared_capabilities,
        error: None,
    }
}

/// Check if a primal's socket exists in the biomeOS socket directory.
fn primal_socket_exists(socket_dir: &std::path::Path, slug: &str) -> bool {
    let sock = socket_dir.join(format!("{slug}.sock"));
    let tarpc_sock = socket_dir.join(format!("{slug}.tarpc.sock"));
    let neural_sock = socket_dir.join(format!("{slug}-neural.sock"));

    if sock.exists() || tarpc_sock.exists() || neural_sock.exists() {
        return true;
    }

    if slug == "biomeos" {
        let bio_neural = socket_dir.join("biomeos-neural.sock");
        let bio_api = std::fs::read_dir(socket_dir)
            .ok()
            .map(|entries| {
                entries.filter_map(Result::ok).any(|e| {
                    e.file_name()
                        .to_string_lossy()
                        .starts_with("biomeos-api-")
                })
            })
            .unwrap_or(false);
        return bio_neural.exists() || bio_api;
    }

    false
}

/// Find a primal's primary socket path.
fn find_primal_socket(socket_dir: &std::path::Path, slug: &str) -> Option<PathBuf> {
    let candidates = [
        socket_dir.join(format!("{slug}.sock")),
        socket_dir.join(format!("{slug}.tarpc.sock")),
        socket_dir.join(format!("{slug}-neural.sock")),
    ];
    candidates.into_iter().find(|p| p.exists())
}

/// Find the biomeOS Neural API socket.
fn find_biomeos_socket(socket_dir: &std::path::Path) -> Option<PathBuf> {
    let neural = socket_dir.join("biomeos-neural.sock");
    if neural.exists() {
        return Some(neural);
    }

    if let Ok(entries) = std::fs::read_dir(socket_dir) {
        for entry in entries.filter_map(Result::ok) {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("biomeos-api-") && name.ends_with(".sock") {
                return Some(entry.path());
            }
        }
    }

    let plain = socket_dir.join("biomeos.sock");
    if plain.exists() {
        return Some(plain);
    }

    None
}

/// Probe biomeOS `primal.list` to get the set of mesh-routable primals.
fn probe_mesh_primals(biomeos_socket: &Option<PathBuf>) -> Vec<String> {
    let Some(socket) = biomeos_socket else {
        return Vec::new();
    };

    let mut client = match PrimalClient::connect(socket, "biomeos") {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let response = client.call("primal.list", serde_json::json!({}));
    match response {
        Ok(resp) => {
            if let Some(result) = resp.result {
                extract_primal_slugs(&result)
            } else {
                Vec::new()
            }
        }
        Err(_) => Vec::new(),
    }
}

/// Extract primal slugs from a `primal.list` response.
///
/// Handles both array-of-strings and array-of-objects (`{slug: "..."}`) formats.
fn extract_primal_slugs(value: &serde_json::Value) -> Vec<String> {
    match value {
        serde_json::Value::Array(arr) => arr
            .iter()
            .filter_map(|v| match v {
                serde_json::Value::String(s) => Some(s.clone()),
                serde_json::Value::Object(obj) => obj
                    .get("slug")
                    .or_else(|| obj.get("name"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                _ => None,
            })
            .collect(),
        serde_json::Value::Object(obj) => {
            if let Some(primals) = obj.get("primals").and_then(|v| v.as_array()) {
                primals
                    .iter()
                    .filter_map(|v| match v {
                        serde_json::Value::String(s) => Some(s.clone()),
                        serde_json::Value::Object(o) => o
                            .get("slug")
                            .or_else(|| o.get("name"))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        _ => None,
                    })
                    .collect()
            } else {
                Vec::new()
            }
        }
        _ => Vec::new(),
    }
}

/// Probe swarmVine `gossip.status` for the gossip event registry.
///
/// Returns a map of primal_slug → registered event names.
fn probe_gossip_registry(swarmvine_socket: &Option<PathBuf>) -> HashMap<String, Vec<String>> {
    let Some(socket) = swarmvine_socket else {
        return HashMap::new();
    };

    let mut client = match PrimalClient::connect(socket, "swarmvine") {
        Ok(c) => c,
        Err(_) => return HashMap::new(),
    };

    let response = client.call("gossip.status", serde_json::json!({}));
    match response {
        Ok(resp) => {
            if let Some(result) = resp.result {
                extract_gossip_registry(&result)
            } else {
                HashMap::new()
            }
        }
        Err(_) => HashMap::new(),
    }
}

/// Extract per-primal gossip event registrations from `gossip.status` response.
///
/// Expected formats:
/// - `{"registered": {"primal_slug": ["event1", "event2"]}}`
/// - `{"primals": {"primal_slug": {"events": ["event1"]}}}`
/// - `{"events": [{"source": "primal_slug", "type": "event1"}]}`
fn extract_gossip_registry(value: &serde_json::Value) -> HashMap<String, Vec<String>> {
    let mut registry = HashMap::new();

    if let Some(obj) = value.as_object() {
        if let Some(registered) = obj.get("registered").and_then(|v| v.as_object()) {
            for (slug, events) in registered {
                if let Some(arr) = events.as_array() {
                    let event_names: Vec<String> = arr
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                    registry.insert(slug.clone(), event_names);
                }
            }
            return registry;
        }

        if let Some(primals) = obj.get("primals").and_then(|v| v.as_object()) {
            for (slug, info) in primals {
                if let Some(events) = info.get("events").and_then(|v| v.as_array()) {
                    let event_names: Vec<String> = events
                        .iter()
                        .filter_map(|v| {
                            v.as_str()
                                .map(|s| s.to_string())
                                .or_else(|| v.get("type").and_then(|t| t.as_str()).map(|s| s.to_string()))
                        })
                        .collect();
                    registry.insert(slug.clone(), event_names);
                }
            }
            return registry;
        }

        if let Some(events) = obj.get("events").and_then(|v| v.as_array()) {
            for event in events {
                if let (Some(source), Some(event_type)) = (
                    event.get("source").and_then(|v| v.as_str()),
                    event.get("type").and_then(|v| v.as_str()),
                ) {
                    registry
                        .entry(source.to_string())
                        .or_default()
                        .push(event_type.to_string());
                }
            }
            return registry;
        }
    }

    registry
}

/// Compute summary counts from lifecycle states.
fn compute_summary(states: &[PrimalLifecycleState]) -> LifecycleSummary {
    let mut summary = LifecycleSummary {
        total: states.len(),
        ..Default::default()
    };

    for state in states {
        match state.phase {
            LifecyclePhase::NotDeployed => summary.not_deployed += 1,
            LifecyclePhase::Deployed => summary.deployed_only += 1,
            LifecyclePhase::GossipRegistered => summary.gossip_only += 1,
            LifecyclePhase::Verified => summary.verified += 1,
        }
    }

    summary
}

/// Build a startup workflow that includes gossip verification steps.
///
/// Extends the standard nucleus startup workflow with gossip-verify gates
/// after each composition's readiness check.
pub fn lifecycle_startup_workflow(manifest: &BiomeManifest) -> super::manifest::CompositionWorkflow {
    use super::manifest::{WorkflowAction, WorkflowStep, WorkflowTarget, CompositionWorkflow};

    let mut steps = Vec::new();
    let mut prev_id: Option<String> = None;

    let mut comps: Vec<&super::manifest::CompositionGraph> = manifest
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
                .map(|r| r.timeout_secs)
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
                .map(|r| r.timeout_secs)
                .unwrap_or(60),
        });

        let gossip_id = format!("gossip_verify_{}", comp.name.replace('-', "_"));
        steps.push(WorkflowStep {
            id: gossip_id.clone(),
            target: WorkflowTarget::Composition(comp.name.clone()),
            action: WorkflowAction::HealthCheck,
            depends_on: vec![ready_id],
            timeout_secs: 30,
        });

        prev_id = Some(gossip_id);
    }

    steps.push(WorkflowStep {
        id: "mesh_verify".to_string(),
        target: WorkflowTarget::All,
        action: WorkflowAction::Reconcile,
        depends_on: prev_id.into_iter().collect(),
        timeout_secs: 30,
    });

    CompositionWorkflow {
        name: format!("{}_lifecycle", manifest.metadata.name),
        description: format!(
            "Deploy→gossip→verify lifecycle for {} ({} compositions)",
            manifest.metadata.name,
            comps.len()
        ),
        steps,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::manifest::load_biome_manifest;
    use std::path::Path;

    fn test_manifest() -> BiomeManifest {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("config/biome-eastgate.yaml");
        load_biome_manifest(&path).expect("biome-eastgate.yaml should parse")
    }

    #[test]
    fn lifecycle_phases_are_ordered() {
        assert!(LifecyclePhase::NotDeployed < LifecyclePhase::Deployed);
        assert!(LifecyclePhase::Deployed < LifecyclePhase::GossipRegistered);
        assert!(LifecyclePhase::GossipRegistered < LifecyclePhase::Verified);
    }

    #[test]
    fn assess_lifecycle_produces_report() {
        let manifest = test_manifest();
        let report = assess_lifecycle(&manifest);

        assert_eq!(report.gate, "eastgate");
        assert!(!report.primals.is_empty());
        assert_eq!(
            report.summary.total,
            report.summary.not_deployed
                + report.summary.deployed_only
                + report.summary.gossip_only
                + report.summary.verified
        );
    }

    #[test]
    fn assess_primal_not_deployed_when_no_socket() {
        let config = ManifestPrimalConfig {
            enabled: true,
            capabilities: vec!["test.cap".to_string()],
            gossip_events: vec!["test.event".to_string()],
            ..Default::default()
        };
        let socket_dir = std::path::Path::new("/nonexistent_dir_for_test");
        let gossip_registry = HashMap::new();
        let mesh_primals = Vec::new();

        let state = assess_primal("testprimal", &config, socket_dir, &gossip_registry, &mesh_primals);
        assert_eq!(state.phase, LifecyclePhase::NotDeployed);
        assert!(!state.mesh_routable);
    }

    #[test]
    fn extract_primal_slugs_from_array() {
        let value = serde_json::json!(["biomeos", "songbird", "beardog"]);
        let slugs = extract_primal_slugs(&value);
        assert_eq!(slugs, vec!["biomeos", "songbird", "beardog"]);
    }

    #[test]
    fn extract_primal_slugs_from_object_array() {
        let value = serde_json::json!({"primals": [{"slug": "biomeos"}, {"slug": "songbird"}]});
        let slugs = extract_primal_slugs(&value);
        assert_eq!(slugs, vec!["biomeos", "songbird"]);
    }

    #[test]
    fn extract_gossip_registry_from_registered() {
        let value = serde_json::json!({
            "registered": {
                "rhizocrypt": ["dag.session.created", "dag.session.complete"],
                "loamspine": ["cas.have", "braid.head"]
            }
        });
        let registry = extract_gossip_registry(&value);
        assert_eq!(registry["rhizocrypt"].len(), 2);
        assert_eq!(registry["loamspine"].len(), 2);
    }

    #[test]
    fn extract_gossip_registry_from_events() {
        let value = serde_json::json!({
            "events": [
                {"source": "rhizocrypt", "type": "dag.session.created"},
                {"source": "rhizocrypt", "type": "dag.branch.created"},
                {"source": "loamspine", "type": "cas.have"}
            ]
        });
        let registry = extract_gossip_registry(&value);
        assert_eq!(registry["rhizocrypt"].len(), 2);
        assert_eq!(registry["loamspine"].len(), 1);
    }

    #[test]
    fn lifecycle_startup_workflow_has_correct_structure() {
        let manifest = test_manifest();
        let workflow = lifecycle_startup_workflow(&manifest);

        assert!(workflow.name.contains("lifecycle"));
        assert!(!workflow.steps.is_empty());

        let last = workflow.steps.last().unwrap();
        assert_eq!(last.id, "mesh_verify");
    }

    #[test]
    fn summary_counts_add_up() {
        let states = vec![
            PrimalLifecycleState {
                slug: "a".to_string(),
                phase: LifecyclePhase::Verified,
                declared_gossip_events: vec![],
                confirmed_gossip_events: vec![],
                mesh_routable: true,
                declared_capabilities: vec![],
                error: None,
            },
            PrimalLifecycleState {
                slug: "b".to_string(),
                phase: LifecyclePhase::NotDeployed,
                declared_gossip_events: vec![],
                confirmed_gossip_events: vec![],
                mesh_routable: false,
                declared_capabilities: vec![],
                error: None,
            },
            PrimalLifecycleState {
                slug: "c".to_string(),
                phase: LifecyclePhase::Deployed,
                declared_gossip_events: vec!["x".to_string()],
                confirmed_gossip_events: vec![],
                mesh_routable: false,
                declared_capabilities: vec![],
                error: None,
            },
        ];

        let summary = compute_summary(&states);
        assert_eq!(summary.total, 3);
        assert_eq!(summary.verified, 1);
        assert_eq!(summary.not_deployed, 1);
        assert_eq!(summary.deployed_only, 1);
    }
}
