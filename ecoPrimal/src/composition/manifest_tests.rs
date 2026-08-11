use super::*;
use crate::composition::workflow::*;

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
    assert_eq!(fed.peers.len(), 7);
    assert!(fed.peers.contains(&"sporeGate".to_string()));
}

#[test]
fn startup_workflow_structure() {
    let manifest: BiomeManifest = serde_yaml_ng::from_str(eastgate_yaml()).unwrap();
    let wf = nucleus_startup_workflow(&manifest);
    assert_eq!(wf.name, "eastgate_startup");
    assert_eq!(wf.steps.len(), 7);
    assert!(matches!(
        wf.steps.last().unwrap().action,
        WorkflowAction::Reconcile
    ));
}

#[test]
fn shutdown_workflow_reverse_priority() {
    let manifest: BiomeManifest = serde_yaml_ng::from_str(eastgate_yaml()).unwrap();
    let wf = nucleus_shutdown_workflow(&manifest);
    assert_eq!(wf.name, "eastgate_shutdown");
    assert_eq!(wf.steps.len(), 3);
    assert!(wf.steps[0].id.contains("node"));
    assert!(wf.steps[1].id.contains("nest"));
    assert!(wf.steps[2].id.contains("tower"));
}

#[test]
fn health_workflow_parallel() {
    let manifest: BiomeManifest = serde_yaml_ng::from_str(eastgate_yaml()).unwrap();
    let wf = nucleus_health_workflow(&manifest);
    assert_eq!(wf.steps.len(), 3);
    for step in &wf.steps {
        assert!(step.depends_on.is_empty());
    }
}

#[test]
fn workflow_wave_resolution() {
    let manifest: BiomeManifest = serde_yaml_ng::from_str(eastgate_yaml()).unwrap();
    let wf = nucleus_startup_workflow(&manifest);
    let waves = resolve_workflow_waves(&wf).unwrap();
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
