// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Lifecycle operations: stop and status for running NUCLEUS primals.

use std::path::PathBuf;
use std::time::Duration;

use primalspring::coordination::AtomicType;
use primalspring::env_keys;
use primalspring::tolerances;

use super::registry;
use super::spawn;

/// Stop all primals in the given list (reverse dependency order).
///
/// Uses the environment `FAMILY_ID` to locate family-scoped PID files.
/// Falls back to unscoped PID files for backward compatibility.
pub fn stop_all(primals: &[&str]) {
    let family_id = std::env::var(env_keys::FAMILY_ID).unwrap_or_default();
    stop_all_family(primals, &family_id);
}

/// Stop all primals scoped to a specific family.
pub fn stop_all_family(primals: &[&str], family_id: &str) {
    let pid_dir = PathBuf::from(tolerances::runtime_dir())
        .join(primalspring::env_keys::BIOMEOS_SUBDIR)
        .join(".pids");

    let label = if family_id.is_empty() {
        "default"
    } else {
        family_id
    };
    println!("=== Stopping primals (family: {label}) ===");
    for primal in primals.iter().rev() {
        let pid_file = if family_id.is_empty() {
            pid_dir.join(format!("{primal}.pid"))
        } else {
            pid_dir.join(format!("{primal}-{family_id}.pid"))
        };

        if try_stop_pid_file(primal, &pid_file, "") {
            continue;
        }

        let legacy = pid_dir.join(format!("{primal}.pid"));
        if legacy != pid_file && try_stop_pid_file(primal, &legacy, " (legacy)") {
            continue;
        }

        println!("  {primal:<14} \x1b[90mnot running\x1b[0m");
    }

    std::thread::sleep(Duration::from_secs(1));
    println!("  Done.");
}

/// Read a PID file, signal the process, and remove the file. Returns `true` if stopped.
fn try_stop_pid_file(primal: &str, pid_file: &std::path::Path, suffix: &str) -> bool {
    let Ok(contents) = std::fs::read_to_string(pid_file) else {
        return false;
    };
    let Ok(pid) = contents.trim().parse::<u32>() else {
        return false;
    };
    print!("  {primal:<14} pid={pid:<8}{suffix} ");
    if spawn::signal_pid(pid).is_ok() {
        println!("\x1b[33mSIGTERM\x1b[0m");
    } else {
        println!("\x1b[31mFAILED\x1b[0m");
    }
    let _ = std::fs::remove_file(pid_file);
    true
}

/// Show status of all primals via PID files and UDS/TCP health probes.
///
/// Prefers UDS socket liveness (Tower Atomic default). Falls back to TCP
/// only when a socket is unavailable and a port is configured.
///
/// Reports results relative to the active composition profile (e.g.
/// Tower = 3 primals, not 13). A full green sweep means PASS for that profile.
pub fn show_status(primals: &[&str]) {
    let pid_dir = PathBuf::from(tolerances::runtime_dir())
        .join(primalspring::env_keys::BIOMEOS_SUBDIR)
        .join(".pids");
    let health_timeout = Duration::from_secs(3);

    let profile_label = AtomicType::from_primal_count(primals.len());

    println!(
        "=== NUCLEUS Status ({profile_label}: {}/{} primals) ===",
        primals.len(),
        primals.len()
    );
    println!();

    let mut alive = 0usize;
    let mut total = 0usize;

    for primal in primals {
        total += 1;
        let pid_file = pid_dir.join(format!("{primal}.pid"));
        let socket = registry::socket_path_for(primal);

        let pid_status = std::fs::read_to_string(&pid_file)
            .ok()
            .and_then(|c| c.trim().parse::<u32>().ok())
            .filter(|pid| std::path::Path::new(&format!("/proc/{pid}")).exists());

        let (health_ok, transport) = if registry::capability_probe(primal) {
            (true, "cap")
        } else if socket.exists() && registry::health_check_uds(&socket).is_alive() {
            (true, "uds")
        } else {
            let port = registry::effective_port(primal);
            if port > 0 && registry::health_check_tcp(port, health_timeout).is_alive() {
                (true, "tcp")
            } else {
                (false, "---")
            }
        };

        let status = match (pid_status, health_ok) {
            (Some(_), true) => {
                alive += 1;
                "\x1b[32mALIVE\x1b[0m"
            }
            (Some(_), false) => "\x1b[33mSTARTED\x1b[0m",
            (None, true) => {
                alive += 1;
                "\x1b[32mALIVE\x1b[0m (no PID file)"
            }
            (None, false) => "\x1b[31mDOWN\x1b[0m",
        };

        let pid_str = pid_status.map_or_else(|| "-".to_owned(), |p| p.to_string());
        println!("  {primal:<14} [{status}] pid={pid_str:<8} via={transport}");
    }

    println!();
    if alive == total {
        println!("  \x1b[32mPASS\x1b[0m {alive}/{total} primals healthy ({profile_label})");
    } else {
        println!("  \x1b[33m{alive}/{total}\x1b[0m primals responding ({profile_label})");
    }
}
