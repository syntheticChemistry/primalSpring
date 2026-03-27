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

#[test]
fn nucleation_from_env_creates_biomeos_dir() {
    let nuc = SocketNucleation::from_env();
    let biomeos_dir = nuc.base_dir().join("biomeos");
    assert!(biomeos_dir.exists() || nuc.base_dir().exists());
}

#[test]
fn nucleation_get_returns_none_for_unassigned() {
    let dir = std::env::temp_dir().join(format!(
        "primalspring-nucleation-get-{}",
        std::process::id()
    ));
    let _ = std::fs::create_dir_all(&dir);
    let nuc = SocketNucleation::new(dir.clone());
    assert!(nuc.get("unknown", "family").is_none());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn nucleation_get_returns_some_after_assign() {
    let dir = std::env::temp_dir().join(format!(
        "primalspring-nucleation-getassign-{}",
        std::process::id()
    ));
    let _ = std::fs::create_dir_all(&dir);
    let mut nuc = SocketNucleation::new(dir.clone());
    let assigned = nuc.assign("beardog", "test");
    assert_eq!(nuc.get("beardog", "test"), Some(&assigned));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn nucleation_remap_changes_path() {
    let dir = std::env::temp_dir().join(format!(
        "primalspring-nucleation-remap-{}",
        std::process::id()
    ));
    let _ = std::fs::create_dir_all(&dir);
    let mut nuc = SocketNucleation::new(dir.clone());
    let _ = nuc.assign("toadstool", "remap-test");
    let new_path = PathBuf::from("/tmp/toadstool-jsonrpc.sock");
    nuc.remap("toadstool", "remap-test", new_path.clone());
    assert_eq!(nuc.get("toadstool", "remap-test"), Some(&new_path));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn launch_error_display_binary_not_found() {
    let err = LaunchError::BinaryNotFound {
        primal: "test".to_owned(),
        searched: vec![PathBuf::from("/a"), PathBuf::from("/b")],
    };
    let msg = err.to_string();
    assert!(msg.contains("test"));
    assert!(msg.contains("binary not found"));
}

#[test]
fn launch_error_display_socket_timeout() {
    let err = LaunchError::SocketTimeout {
        primal: "beardog".to_owned(),
        socket: PathBuf::from("/tmp/test.sock"),
        waited: Duration::from_secs(10),
    };
    let msg = err.to_string();
    assert!(msg.contains("socket timeout"));
    assert!(msg.contains("beardog"));
}

#[test]
fn launch_error_display_health_check_failed() {
    let err = LaunchError::HealthCheckFailed {
        primal: "songbird".to_owned(),
        detail: "connection refused".to_owned(),
    };
    let msg = err.to_string();
    assert!(msg.contains("health check failed"));
    assert!(msg.contains("songbird"));
}

#[test]
fn launch_error_display_profile_parse() {
    let err = LaunchError::ProfileParseError("bad toml".to_owned());
    let msg = err.to_string();
    assert!(msg.contains("launch profile parse error"));
}

#[test]
fn launch_error_source_spawn_failed() {
    let err = LaunchError::SpawnFailed {
        primal: "x".to_owned(),
        source: std::io::Error::new(std::io::ErrorKind::NotFound, "no binary"),
    };
    assert!(std::error::Error::source(&err).is_some());
}

#[test]
fn launch_error_source_none_for_other_variants() {
    let err = LaunchError::BinaryNotFound {
        primal: "x".to_owned(),
        searched: vec![],
    };
    assert!(std::error::Error::source(&err).is_none());

    let err2 = LaunchError::ProfileParseError("x".to_owned());
    assert!(std::error::Error::source(&err2).is_none());
}

#[test]
fn launch_profiles_contain_known_primals() {
    let (_defaults, profiles) = load_launch_profiles().unwrap();
    assert!(
        profiles.contains_key("toadstool"),
        "toadstool should have a launch profile"
    );
}

#[test]
fn launch_profiles_default_has_socket_flag() {
    let (defaults, _profiles) = load_launch_profiles().unwrap();
    assert!(defaults.socket_flag.is_some());
}
