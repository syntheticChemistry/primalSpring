// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Multi-composition workflow orchestration.
//!
//! Extends the `biome.yaml` manifest model with multi-step workflows that
//! compose operations across compositions. Workflows are DAGs of steps —
//! each step targets a composition (or primal) and performs an action
//! (start, stop, health check, capability call, await readiness, reconcile).
//!
//! The workflow engine resolves step dependencies into parallel waves,
//! enabling concurrent execution within each wave.

use std::collections::{HashMap, HashSet};

use super::manifest::{
    BiomeManifest, CompositionGraph, CompositionWorkflow, ManifestError, WorkflowAction,
    WorkflowStep, WorkflowTarget,
};

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
            .filter(|id| in_degree.get(id).copied().unwrap_or(0) == 0)
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

        waves.push(
            wave.into_iter()
                .filter_map(|id| step_map.get(id).copied())
                .collect(),
        );
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
