// SPDX-License-Identifier: AGPL-3.0-or-later

//! Process lifecycle — spawn, stop, seed resolution for NUCLEUS primals.

use std::path::PathBuf;

use primalspring::env_keys;
use primalspring::launcher::discover_binary;
use primalspring::primal_names;
use primalspring::tolerances;

use super::LaunchConfig;

/// Resolve or generate a family seed.
pub(super) fn resolve_family_seed(socket_dir: &std::path::Path) -> Vec<u8> {
    if let Ok(val) = std::env::var(env_keys::BEARDOG_FAMILY_SEED) {
        return val.into_bytes();
    }
    if let Ok(val) = std::env::var(env_keys::FAMILY_SEED) {
        return val.into_bytes();
    }
    let seed_file = socket_dir.join(".family.seed");
    if let Ok(contents) = std::fs::read_to_string(&seed_file) {
        let trimmed = contents.trim();
        if !trimmed.is_empty() {
            return trimmed.as_bytes().to_vec();
        }
    }
    let mut buf = [0u8; 32];
    if getrandom::fill(&mut buf).is_err() {
        eprintln!("WARNING: getrandom failed — deriving seed from PID + clock");
        let pid = std::process::id().to_le_bytes();
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
            .to_le_bytes();
        buf[..4].copy_from_slice(&pid);
        buf[4..20].copy_from_slice(&ts);
    }
    let mut hex_seed = String::with_capacity(64);
    for b in buf {
        use std::fmt::Write;
        let _ = write!(hex_seed, "{b:02x}");
    }
    hex_seed.into_bytes()
}

/// Attempt to stop any running instance of a primal.
///
/// Reads the PID file written at spawn time. Falls back to scanning
/// `/proc` on Linux when no PID file exists.
pub(super) fn stop_existing(primal: &str) {
    let pid_dir = PathBuf::from(tolerances::runtime_dir())
        .join("biomeos")
        .join(".pids");
    let pid_file = pid_dir.join(format!("{primal}.pid"));

    if let Ok(contents) = std::fs::read_to_string(&pid_file) {
        if let Ok(pid) = contents.trim().parse::<u32>() {
            let _ = signal_pid(pid);
            let _ = std::fs::remove_file(&pid_file);
            return;
        }
    }

    #[cfg(target_os = "linux")]
    stop_by_proc_scan(primal);
}

/// Send SIGTERM to a process by PID.
///
/// Uses the `kill` binary (POSIX standard) rather than libc to maintain
/// `forbid(unsafe_code)`.
pub(super) fn signal_pid(pid: u32) -> std::io::Result<()> {
    let status = std::process::Command::new("kill")
        .args(["-15", &pid.to_string()])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "kill -15 {pid} exited with {status}"
        )))
    }
}

/// Scan `/proc` for processes matching the primal binary pattern.
#[cfg(target_os = "linux")]
fn stop_by_proc_scan(primal: &str) {
    let pattern = format!("primals/{primal}");
    let Ok(entries) = std::fs::read_dir("/proc") else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(pid_str) = name.to_str() else {
            continue;
        };
        let Ok(pid) = pid_str.parse::<u32>() else {
            continue;
        };
        let cmdline_path = entry.path().join("cmdline");
        if let Ok(cmdline) = std::fs::read_to_string(&cmdline_path) {
            if cmdline.contains(&pattern) {
                let _ = signal_pid(pid);
            }
        }
    }
}

/// Spawn a primal process using its discovered binary.
pub(super) fn spawn_primal(
    primal: &str,
    port: u16,
    socket: &std::path::Path,
    config: &LaunchConfig,
    family_seed: &str,
) -> Result<(), String> {
    let binary = discover_binary(primal).map_err(|e| e.to_string())?;

    let mut cmd = std::process::Command::new(&binary);
    cmd.arg("server");
    cmd.arg("--socket").arg(socket);
    if port > 0 {
        cmd.arg("--port").arg(port.to_string());
    }
    cmd.arg("--family-id").arg(&config.family_id);

    cmd.env(env_keys::FAMILY_ID, &config.family_id);
    cmd.env(env_keys::FAMILY_SEED, family_seed);
    cmd.env(env_keys::BEARDOG_FAMILY_SEED, family_seed);

    if config.dark_forest {
        cmd.arg("--dark-forest");
    }

    if primal == primal_names::SONGBIRD {
        if let Some(fed_port) = config.federation_port {
            cmd.arg("--federation-port").arg(fed_port.to_string());
            cmd.arg("--bind").arg(tolerances::LAN_BIND_ADDRESS);
        }
        cmd.env(env_keys::SONGBIRD_SECURITY_SOCKET, socket);
    }

    let log_dir = PathBuf::from(tolerances::runtime_dir())
        .join("biomeos")
        .join("logs");
    let _ = std::fs::create_dir_all(&log_dir);
    let log_path = log_dir.join(format!("{primal}.log"));
    let log_file = std::fs::File::create(&log_path)
        .map_err(|e| format!("cannot create log file {}: {e}", log_path.display()))?;
    let log_err = log_file
        .try_clone()
        .map_err(|e| format!("cannot clone log file: {e}"))?;

    cmd.stdout(log_file);
    cmd.stderr(log_err);

    let child = cmd.spawn().map_err(|e| format!("spawn failed: {e}"))?;

    let pid_dir = PathBuf::from(tolerances::runtime_dir())
        .join("biomeos")
        .join(".pids");
    let _ = std::fs::create_dir_all(&pid_dir);
    let _ = std::fs::write(
        pid_dir.join(format!("{primal}.pid")),
        child.id().to_string(),
    );

    tracing::info!(primal, binary = %binary.display(), pid = child.id(), "spawned");
    Ok(())
}
