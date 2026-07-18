// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Phase 0: Pre-flight validation before primal spawning.
//!
//! Checks that all required binaries are discoverable and optionally
//! verifies BLAKE3 checksums, detects port conflicts, and cleans stale
//! UDS sockets. Failures here abort the launch before any process is
//! spawned, keeping the system in a known-clean state.

use std::collections::HashMap;
use std::path::PathBuf;

use primalspring::launcher::discover_binary;
use primalspring::tolerances;

use super::registry;

/// Pre-flight result with pass/fail details.
#[expect(
    dead_code,
    reason = "fields used for diagnostic output; callers check .passed"
)]
pub struct PreflightResult {
    pub passed: bool,
    pub binary_missing: Vec<String>,
    pub checksum_failures: Vec<String>,
    pub port_conflicts: Vec<(u16, Vec<String>)>,
    pub stale_sockets_cleaned: u32,
}

/// Run Phase 0 pre-flight checks for the given primal list.
#[expect(clippy::too_many_lines, reason = "preflight validation check sequence")]
pub fn run_preflight(primals: &[&str], uds_only: bool) -> PreflightResult {
    println!("=== Phase 0: Pre-flight validation ===");
    println!();

    let mut binary_missing = Vec::new();
    let mut checksum_failures = Vec::new();
    let mut binary_paths: Vec<(String, PathBuf)> = Vec::new();

    // 1. Binary discovery check
    print!("  Binary discovery:  ");
    for primal in primals {
        match discover_binary(primal) {
            Ok(path) => {
                binary_paths.push(((*primal).to_owned(), path));
            }
            Err(_) => {
                binary_missing.push((*primal).to_owned());
            }
        }
    }
    if binary_missing.is_empty() {
        println!(
            "\x1b[32mOK\x1b[0m ({} / {} found)",
            primals.len(),
            primals.len()
        );
    } else {
        println!(
            "\x1b[31mFAIL\x1b[0m ({} missing: {})",
            binary_missing.len(),
            binary_missing.join(", ")
        );
    }

    // 2. Checksum verification (if checksums.toml is reachable)
    let checksums_path = find_checksums_toml();
    if let Some(ref cpath) = checksums_path {
        print!("  Checksum verify:   ");
        if let Ok(content) = std::fs::read_to_string(cpath) {
            if let Ok(parsed) = content.parse::<toml::Table>() {
                let triple = tolerances::current_target_triple();
                let primals_table = parsed.get("primals").and_then(toml::Value::as_table);

                for (name, path) in &binary_paths {
                    if let Some(pt) = primals_table {
                        if let Some(hashes) = pt.get(name).and_then(toml::Value::as_table) {
                            if let Some(expected) =
                                hashes.get(&triple).and_then(toml::Value::as_str)
                            {
                                if let Ok(data) = std::fs::read(path) {
                                    let actual = blake3::hash(&data).to_hex().to_string();
                                    if actual != expected {
                                        checksum_failures.push(name.clone());
                                    }
                                }
                            }
                        }
                    }
                }

                if checksum_failures.is_empty() {
                    println!("\x1b[32mOK\x1b[0m ({} verified)", binary_paths.len());
                } else {
                    println!(
                        "\x1b[33mWARN\x1b[0m ({} mismatch: {})",
                        checksum_failures.len(),
                        checksum_failures.join(", ")
                    );
                }
            } else {
                println!("\x1b[33mSKIP\x1b[0m (checksums.toml parse error)");
            }
        } else {
            println!("\x1b[33mSKIP\x1b[0m (checksums.toml unreadable)");
        }
    } else {
        println!("  Checksum verify:   \x1b[90mSKIP\x1b[0m (no checksums.toml found)");
    }

    // 3. Port conflict detection
    let port_conflicts = if uds_only {
        println!("  Port conflicts:    \x1b[90mSKIP\x1b[0m (UDS-only mode)");
        Vec::new()
    } else {
        print!("  Port conflicts:    ");
        let conflicts = detect_port_conflicts(primals);
        if conflicts.is_empty() {
            println!("\x1b[32mOK\x1b[0m (no conflicts)");
        } else {
            for (port, names) in &conflicts {
                println!(
                    "\x1b[31mFAIL\x1b[0m port {port} claimed by: {}",
                    names.join(", ")
                );
            }
        }
        conflicts
    };

    // 4. Stale socket cleanup
    print!("  Stale sockets:     ");
    let stale_sockets_cleaned = clean_stale_sockets();
    if stale_sockets_cleaned > 0 {
        println!("\x1b[33mCLEANED {stale_sockets_cleaned}\x1b[0m");
    } else {
        println!("\x1b[32mOK\x1b[0m (none)");
    }

    println!();

    let passed = binary_missing.is_empty() && port_conflicts.is_empty();

    PreflightResult {
        passed,
        binary_missing,
        checksum_failures,
        port_conflicts,
        stale_sockets_cleaned,
    }
}

fn find_checksums_toml() -> Option<PathBuf> {
    let p = PathBuf::from(tolerances::plasmidbin_depot_root()).join("checksums.toml");
    if p.exists() { Some(p) } else { None }
}

fn detect_port_conflicts(primals: &[&str]) -> Vec<(u16, Vec<String>)> {
    let mut port_map: HashMap<u16, Vec<String>> = HashMap::new();
    for primal in primals {
        let port = registry::effective_port(primal);
        if port > 0 {
            port_map.entry(port).or_default().push((*primal).to_owned());
        }
    }
    port_map
        .into_iter()
        .filter(|(_, names)| names.len() > 1)
        .collect()
}

fn clean_stale_sockets() -> u32 {
    let socket_dir =
        PathBuf::from(tolerances::runtime_dir()).join(primalspring::env_keys::BIOMEOS_SUBDIR);
    let Ok(entries) = std::fs::read_dir(&socket_dir) else {
        return 0;
    };

    let mut cleaned = 0u32;
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(ext) = path.extension() else {
            continue;
        };
        if ext != "sock" {
            continue;
        }
        // If the socket file exists but no process holds it open, it's stale.
        // Try connecting to detect liveness; if connection fails, remove it.
        #[cfg(unix)]
        {
            use std::os::unix::net::UnixStream;
            if UnixStream::connect(&path).is_err() {
                let _ = std::fs::remove_file(&path);
                cleaned += 1;
            }
        }
    }
    cleaned
}
