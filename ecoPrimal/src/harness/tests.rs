// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;

#[test]
fn running_atomic_drops_cleanly_even_if_empty() {
    let dir =
        std::env::temp_dir().join(format!("primalspring-harness-empty-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let nuc = SocketNucleation::new(dir.clone());
    let running = RunningAtomic {
        processes: vec![],
        biomeos_process: None,
        nucleation: nuc,
        family_id: "test".to_owned(),
        mito_seed: None,
        nuclear_generation: None,
        runtime_dir: dir.clone(),
        atomic: AtomicType::Tower,
        overlay_capabilities: HashMap::new(),
    };
    assert_eq!(running.primal_count(), 0);
    drop(running);
    assert!(!dir.exists(), "runtime dir should be removed on drop");
}

#[test]
fn capability_to_primal_mapping() {
    let dir = std::env::temp_dir().join(format!("primalspring-harness-cap-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let nuc = SocketNucleation::new(dir.clone());
    let running = RunningAtomic {
        processes: vec![],
        biomeos_process: None,
        nucleation: nuc,
        family_id: "test".to_owned(),
        mito_seed: None,
        nuclear_generation: None,
        runtime_dir: dir,
        atomic: AtomicType::Tower,
        overlay_capabilities: HashMap::new(),
    };
    assert_eq!(
        running.capability_to_primal("security"),
        Some("beardog".to_owned())
    );
    assert_eq!(
        running.capability_to_primal("discovery"),
        Some("songbird".to_owned())
    );
    assert_eq!(running.capability_to_primal("nonexistent"), None);
    drop(running);
}

#[test]
fn capability_to_primal_overlay_fallback() {
    let dir = std::env::temp_dir().join(format!(
        "primalspring-harness-overlay-{}",
        std::process::id()
    ));
    let _ = std::fs::create_dir_all(&dir);
    let nuc = SocketNucleation::new(dir.clone());
    let mut overlay = HashMap::new();
    overlay.insert("ai".to_owned(), "squirrel".to_owned());
    let running = RunningAtomic {
        processes: vec![],
        biomeos_process: None,
        nucleation: nuc,
        family_id: "test".to_owned(),
        mito_seed: None,
        nuclear_generation: None,
        runtime_dir: dir,
        atomic: AtomicType::Tower,
        overlay_capabilities: overlay,
    };
    assert_eq!(
        running.capability_to_primal("security"),
        Some("beardog".to_owned()),
        "base tier capabilities still resolve"
    );
    assert_eq!(
        running.capability_to_primal("ai"),
        Some("squirrel".to_owned()),
        "overlay capabilities resolve"
    );
    assert_eq!(running.capability_to_primal("nonexistent"), None);
    let all_caps = running.all_capabilities();
    assert!(all_caps.contains(&"security".to_owned()));
    assert!(all_caps.contains(&"ai".to_owned()));
    drop(running);
}

#[test]
fn compute_spawn_order_without_graph() {
    let harness = AtomicHarness::new(AtomicType::Tower);
    let (order, overlay) = harness.compute_spawn_order().unwrap();
    assert_eq!(order, vec!["beardog", "songbird"]);
    assert!(overlay.is_empty(), "no overlay without graph");
}

#[test]
fn compute_spawn_order_with_graph() {
    let graph_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs/profiles/tower.toml");
    let harness = AtomicHarness::with_graph(AtomicType::Tower, &graph_path);
    let (order, _overlay) = harness.compute_spawn_order().unwrap();
    assert!(
        order.contains(&"beardog".to_owned()),
        "should include beardog"
    );
    assert!(
        order.contains(&"songbird".to_owned()),
        "should include songbird"
    );
    let beardog_pos = order.iter().position(|n| n == "beardog").unwrap();
    let songbird_pos = order.iter().position(|n| n == "songbird").unwrap();
    assert!(
        beardog_pos < songbird_pos,
        "beardog should start before songbird (topological order)"
    );
}

#[test]
fn compute_spawn_order_node_with_graph() {
    let graph_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs/profiles/node.toml");
    let harness = AtomicHarness::with_graph(AtomicType::Node, &graph_path);
    let (order, _overlay) = harness.compute_spawn_order().unwrap();
    assert!(
        order.len() >= 3,
        "Node includes at least beardog + songbird + toadstool"
    );
    assert!(order.contains(&"beardog".to_owned()));
    assert!(order.contains(&"songbird".to_owned()));
    assert!(order.contains(&"toadstool".to_owned()));
}

#[test]
fn compute_spawn_order_overlay_includes_extra_primals() {
    let graph_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs/profiles/tower_ai.toml");
    assert!(
        graph_path.exists(),
        "required test fixture missing: {}",
        graph_path.display()
    );
    let harness = AtomicHarness::with_graph(AtomicType::Tower, &graph_path);
    let (order, overlay) = harness.compute_spawn_order().unwrap();
    assert!(
        order.contains(&"beardog".to_owned()),
        "base tier beardog present"
    );
    assert!(
        order.contains(&"songbird".to_owned()),
        "base tier songbird present"
    );
    assert!(
        order.contains(&"squirrel".to_owned()),
        "overlay squirrel present from graph"
    );
    assert!(
        overlay.contains_key("ai"),
        "overlay should map ai capability"
    );
    assert_eq!(overlay.get("ai").unwrap(), "squirrel");
}

#[test]
fn harness_new_creates_without_graph() {
    let harness = AtomicHarness::new(AtomicType::Tower);
    assert!(harness.graph_path.is_none());
}

#[test]
fn harness_with_graph_stores_path() {
    let harness = AtomicHarness::with_graph(AtomicType::Tower, "/tmp/test.toml");
    assert_eq!(
        harness.graph_path.as_deref(),
        Some(Path::new("/tmp/test.toml"))
    );
}

#[test]
fn generate_harness_mito_seed_deterministic() {
    let s1 = generate_harness_mito_seed("test-family-1");
    let s2 = generate_harness_mito_seed("test-family-1");
    assert_eq!(s1, s2, "same family_id → same seed");
}

#[test]
fn generate_harness_mito_seed_unique_per_family() {
    let s1 = generate_harness_mito_seed("family-alpha");
    let s2 = generate_harness_mito_seed("family-bravo");
    assert_ne!(s1, s2, "different family_ids → different seeds");
}

#[test]
fn generate_harness_mito_seed_is_hex_ascii() {
    let seed = generate_harness_mito_seed("exp061-12345");
    assert_eq!(seed.len(), 64, "32 bytes hex-encoded = 64 chars");
    assert!(
        seed.iter().all(|&b| b.is_ascii_hexdigit()),
        "all bytes should be ASCII hex digits"
    );
}

#[test]
fn generate_harness_nuclear_is_genesis() {
    let nuc = generate_harness_nuclear("test-family-nuc");
    assert!(nuc.is_genesis());
    assert_eq!(nuc.generation(), 0);
    assert!(!nuc.key_bytes().is_empty());
}

#[test]
fn generate_harness_nuclear_deterministic() {
    let n1 = generate_harness_nuclear("family-det");
    let n2 = generate_harness_nuclear("family-det");
    assert_eq!(n1.key_bytes(), n2.key_bytes());
}

#[test]
fn generate_harness_nuclear_unique_per_family() {
    let n1 = generate_harness_nuclear("family-x");
    let n2 = generate_harness_nuclear("family-y");
    assert_ne!(n1.key_bytes(), n2.key_bytes());
}

#[test]
fn harness_nuclear_child_spawn() {
    let parent = generate_harness_nuclear("test-spawn");
    let child = parent.spawn_child(
        vec![0xCC; 32],
        b"child-proof".to_vec(),
        "test-child".to_owned(),
    );
    assert_eq!(child.generation(), 1);
    assert_ne!(child.key_bytes(), parent.key_bytes());
}

#[test]
fn nucleation_family_seed_round_trip() {
    let dir = std::env::temp_dir().join(format!(
        "primalspring-harness-seed-rt-{}",
        std::process::id()
    ));
    let _ = std::fs::create_dir_all(&dir);
    let mut nuc = SocketNucleation::new(dir.clone());
    assert!(nuc.family_seed().is_none());
    nuc.set_family_seed(b"test-seed".to_vec());
    assert_eq!(nuc.family_seed(), Some(b"test-seed".as_slice()));
    let _ = std::fs::remove_dir_all(&dir);
}
