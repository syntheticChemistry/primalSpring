// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use super::*;

fn test_node(name: &str, order: u32) -> GraphNode {
    GraphNode {
        name: name.to_owned(),
        binary: format!("{name}_primal"),
        order,
        required: true,
        spawn: true,
        depends_on: vec![],
        health_method: "health".to_owned(),
        by_capability: None,
        capabilities: vec![],
        condition: None,
        skip_if: None,
    }
}

fn test_graph(name: &str, nodes: Vec<GraphNode>) -> DeployGraph {
    DeployGraph {
        graph: GraphMeta {
            name: name.to_owned(),
            description: String::new(),
            version: String::new(),
            coordination: None,
            node: nodes,
        },
    }
}

fn graphs_path(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../graphs/{name}"))
}

#[test]
fn load_primalspring_deploy_graph() {
    let graph = load_graph(&graphs_path("primalspring_deploy.toml")).unwrap();
    assert_eq!(graph.graph.name, "primalspring_coordination_niche");
    assert!(!graph.graph.node.is_empty());
    assert_eq!(graph.graph.node[0].name, "beardog");
}

#[test]
fn load_coralforge_pipeline() {
    let graph = load_graph(&graphs_path("coralforge_pipeline.toml")).unwrap();
    assert_eq!(graph.graph.name, "coralforge_pipeline");
    assert_eq!(graph.graph.coordination.as_deref(), Some("Pipeline"));
}

#[test]
fn load_conditional_fallback() {
    let graph = load_graph(&graphs_path("conditional_fallback.toml")).unwrap();
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
    let result = validate_structure(&graphs_path("primalspring_deploy.toml"));
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
        assert!(r.issues.is_empty(), "graph {} has issues: {:?}", r.path, r.issues);
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
    let graph = test_graph("", vec![]);
    let mut issues = Vec::new();
    structural_checks(&graph, &mut issues);
    assert!(issues.iter().any(|i| i.contains("name is empty")));
    assert!(issues.iter().any(|i| i.contains("no nodes")));
}

#[test]
fn structural_checks_detect_missing_dependency() {
    let mut node = test_node("alpha", 1);
    node.depends_on = vec!["nonexistent".to_owned()];
    let graph = test_graph("test", vec![node]);

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
    let result = validate_live(&graphs_path("primalspring_deploy.toml"));
    assert!(result.structure.parsed);
    assert!(result.structure.issues.is_empty());
    assert!(!result.nodes.is_empty());
}

#[test]
fn structural_checks_detect_duplicate_orders() {
    let graph = test_graph("dup_orders", vec![test_node("alpha", 1), test_node("beta", 1)]);
    let mut issues = Vec::new();
    structural_checks(&graph, &mut issues);
    assert!(issues.iter().any(|i| i.contains("duplicate order")));
}

#[test]
fn topological_waves_tower_graph() {
    let graph = load_graph(&graphs_path("tower_atomic_bootstrap.toml")).unwrap();
    let waves = topological_waves(&graph).unwrap();
    assert!(waves.len() >= 2, "tower should have at least 2 waves");
    assert!(waves[0].contains(&"beardog".to_owned()), "beardog should be in wave 0 (no deps)");
}

#[test]
fn topological_waves_nucleus_complete() {
    let graph = load_graph(&graphs_path("nucleus_complete.toml")).unwrap();
    let waves = topological_waves(&graph).unwrap();
    assert!(waves.len() >= 4, "nucleus should have at least 4 waves");
    assert!(waves[0].contains(&"beardog".to_owned()), "beardog should be in wave 0");
}

#[test]
fn topological_waves_empty_graph() {
    let graph = test_graph("empty", vec![]);
    let waves = topological_waves(&graph).unwrap();
    assert!(waves.is_empty());
}

#[test]
fn topological_waves_single_node() {
    let mut node = test_node("alpha", 1);
    node.by_capability = Some("security".to_owned());
    let graph = test_graph("single", vec![node]);
    let waves = topological_waves(&graph).unwrap();
    assert_eq!(waves.len(), 1);
    assert_eq!(waves[0], vec!["alpha"]);
}

#[test]
fn topological_waves_detects_missing_dependency() {
    let mut node = test_node("alpha", 1);
    node.depends_on = vec!["ghost".to_owned()];
    let graph = test_graph("bad_dep", vec![node]);
    let result = topological_waves(&graph);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("ghost"));
}

#[test]
fn topological_waves_detects_cycle() {
    let mut alpha = test_node("alpha", 1);
    alpha.binary = "a".to_owned();
    alpha.depends_on = vec!["beta".to_owned()];
    let mut beta = test_node("beta", 2);
    beta.binary = "b".to_owned();
    beta.depends_on = vec!["alpha".to_owned()];

    let graph = test_graph("cycle", vec![alpha, beta]);
    let result = topological_waves(&graph);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("cycle"));
}

#[test]
fn topological_waves_parallel_nodes() {
    let root = test_node("root", 1);
    let mut leaf_a = test_node("leaf_a", 2);
    leaf_a.depends_on = vec!["root".to_owned()];
    let mut leaf_b = test_node("leaf_b", 3);
    leaf_b.depends_on = vec!["root".to_owned()];

    let graph = test_graph("parallel", vec![root, leaf_a, leaf_b]);
    let waves = topological_waves(&graph).unwrap();
    assert_eq!(waves.len(), 2);
    assert_eq!(waves[0], vec!["root"]);
    assert_eq!(waves[1].len(), 2);
    assert!(waves[1].contains(&"leaf_a".to_owned()));
    assert!(waves[1].contains(&"leaf_b".to_owned()));
}

#[test]
fn graph_required_capabilities_from_nucleus() {
    let graph = load_graph(&graphs_path("nucleus_complete.toml")).unwrap();
    let caps = graph_required_capabilities(&graph);
    assert!(caps.contains(&"security".to_owned()));
    assert!(caps.contains(&"discovery".to_owned()));
    assert!(caps.contains(&"compute".to_owned()));
    assert!(caps.contains(&"storage".to_owned()));
    assert!(caps.contains(&"coordination".to_owned()));
}

#[test]
fn graph_required_capabilities_empty_for_no_by_capability() {
    let graph = test_graph("bare", vec![test_node("alpha", 1)]);
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
        assert!(waves.is_ok(), "graph {} has a cycle: {:?}", r.path, waves.err());
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
                name, node.name
            );
        }
    }
}

#[test]
fn graph_spawnable_primals_filters_spawn_false() {
    let mut coord = test_node("primalspring", 99);
    coord.spawn = false;
    coord.required = false;
    coord.by_capability = Some("coordination".to_owned());

    let mut beardog = test_node("beardog", 1);
    beardog.by_capability = Some("security".to_owned());

    let graph = test_graph("overlay_test", vec![beardog, coord]);
    let spawnable = graph_spawnable_primals(&graph);
    assert_eq!(spawnable, vec!["beardog"]);
    assert!(!spawnable.contains(&"primalspring".to_owned()));
}

#[test]
fn graph_capability_map_builds_mapping() {
    let mut beardog = test_node("beardog", 1);
    beardog.by_capability = Some("security".to_owned());
    let mut squirrel = test_node("squirrel", 2);
    squirrel.required = false;
    squirrel.by_capability = Some("ai".to_owned());

    let graph = test_graph("cap_test", vec![beardog, squirrel]);
    let map = graph_capability_map(&graph);
    assert_eq!(map.get("security").unwrap(), "beardog");
    assert_eq!(map.get("ai").unwrap(), "squirrel");
}

#[test]
fn merge_graphs_combines_nodes() {
    let mut base_node = test_node("beardog", 1);
    base_node.by_capability = Some("security".to_owned());
    let base = DeployGraph {
        graph: GraphMeta {
            name: "base".to_owned(),
            description: "base graph".to_owned(),
            version: "1.0.0".to_owned(),
            coordination: Some("sequential".to_owned()),
            node: vec![base_node],
        },
    };

    let mut overlay_node = test_node("squirrel", 3);
    overlay_node.required = false;
    overlay_node.depends_on = vec!["beardog".to_owned()];
    overlay_node.by_capability = Some("ai".to_owned());
    let overlay = DeployGraph {
        graph: GraphMeta {
            name: "ai_overlay".to_owned(),
            description: "AI overlay".to_owned(),
            version: "1.0.0".to_owned(),
            coordination: None,
            node: vec![overlay_node],
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
    let mut base_node = test_node("beardog", 1);
    base_node.by_capability = Some("security".to_owned());
    let base = test_graph("base", vec![base_node]);

    let mut override_node = test_node("beardog", 1);
    override_node.binary = "beardog_v2".to_owned();
    override_node.required = false;
    override_node.health_method = "health.v2".to_owned();
    override_node.by_capability = Some("security".to_owned());
    let overlay = DeployGraph {
        graph: GraphMeta {
            name: "override".to_owned(),
            description: String::new(),
            version: "2.0.0".to_owned(),
            coordination: None,
            node: vec![override_node],
        },
    };

    let merged = merge_graphs(&base, &overlay);
    assert_eq!(merged.graph.node.len(), 1);
    assert_eq!(merged.graph.node[0].binary, "beardog_v2");
}

#[test]
fn structural_checks_detect_empty_binary() {
    let mut node = test_node("alpha", 1);
    node.binary = String::new();
    let graph = test_graph("test", vec![node]);
    let mut issues = Vec::new();
    structural_checks(&graph, &mut issues);
    assert!(issues.iter().any(|i| i.contains("empty binary")));
}
