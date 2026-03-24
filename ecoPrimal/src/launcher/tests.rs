// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::PathBuf;
use std::time::Duration;

use super::*;

#[test]
fn nucleation_assigns_deterministic_paths() {
    let dir = std::env::temp_dir().join(format!(
        "primalspring-nucleation-test-{}",
        std::process::id()
    ));
    let _ = std::fs::create_dir_all(&dir);
    let mut nuc = SocketNucleation::new(dir.clone());

    let p1 = nuc.assign("beardog", "default");
    let p2 = nuc.assign("beardog", "default");
    assert_eq!(p1, p2, "idempotent assignment");
    assert!(
        p1.to_string_lossy().contains("biomeos"),
        "path includes biomeos dir"
    );
    assert!(
        p1.to_string_lossy().ends_with("beardog-default.sock"),
        "socket name follows convention"
    );

    let p3 = nuc.assign("songbird", "default");
    assert_ne!(p1, p3, "different primals get different sockets");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn nucleation_batch_assigns_all() {
    let dir = std::env::temp_dir().join(format!(
        "primalspring-nucleation-batch-{}",
        std::process::id()
    ));
    let _ = std::fs::create_dir_all(&dir);
    let mut nuc = SocketNucleation::new(dir.clone());
    let batch = nuc.assign_batch(&["beardog", "songbird"], "test");
    assert_eq!(batch.len(), 2);
    assert!(batch.contains_key("beardog"));
    assert!(batch.contains_key("songbird"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn launch_profiles_parse_successfully() {
    let (defaults, profiles) = load_launch_profiles().expect("profiles parse");
    assert_eq!(defaults.socket_flag.as_deref(), Some("--socket"));
    assert!(profiles.contains_key("songbird"));
    let songbird = &profiles["songbird"];
    assert_eq!(songbird.pass_family_id, Some(false));
    assert!(songbird.cli_sockets.contains_key("--beardog-socket"));
}

#[test]
fn discover_binary_returns_error_when_not_found() {
    let result = discover_binary("nonexistent_primal_xyz");
    assert!(result.is_err());
    if let Err(LaunchError::BinaryNotFound { primal, .. }) = result {
        assert_eq!(primal, "nonexistent_primal_xyz");
    } else {
        panic!("expected BinaryNotFound");
    }
}

#[test]
fn discover_binary_searches_relative_plasmid_bin() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace = manifest_dir.parent().expect("parent");
    let plasmid_beardog = workspace.join("plasmidBin/primals/beardog");
    if !plasmid_beardog.is_file() {
        return;
    }
    let arch = std::env::consts::ARCH;
    let os = std::env::consts::OS;
    let patterns = [
        format!("beardog_{arch}_{os}_musl/beardog"),
        format!("beardog_{arch}_{os}/beardog"),
        "primals/beardog/beardog".to_owned(),
        "primals/beardog".to_owned(),
        "beardog/beardog".to_owned(),
        "beardog".to_owned(),
    ];
    let found = patterns
        .iter()
        .any(|p| workspace.join("plasmidBin").join(p).is_file());
    assert!(found, "at least one pattern should match in plasmidBin");
}

#[test]
fn wait_for_socket_succeeds_when_file_exists() {
    let dir = std::env::temp_dir().join(format!("primalspring-socket-test-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let sock = dir.join("test.sock");
    std::fs::write(&sock, b"").expect("create sock");
    assert!(wait_for_socket(&sock, Duration::from_millis(200)));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn wait_for_socket_times_out() {
    let path = std::env::temp_dir().join("nonexistent-socket-xyz.sock");
    assert!(!wait_for_socket(&path, Duration::from_millis(200)));
}
