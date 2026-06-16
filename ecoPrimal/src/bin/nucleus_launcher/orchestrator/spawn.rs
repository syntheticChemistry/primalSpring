// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Process lifecycle — spawn, stop, seed resolution for NUCLEUS primals.

use std::path::PathBuf;

use primalspring::env_keys;
use primalspring::launcher::discover_binary;
use primalspring::tolerances;

use super::LaunchConfig;

#[derive(Debug, thiserror::Error)]
pub(super) enum SpawnError {
    #[error("{0}")]
    Discovery(#[from] primalspring::launcher::LaunchError),
    #[error("profile load: {0}")]
    ProfileLoad(primalspring::launcher::LaunchError),
    #[error("{0}")]
    Io(#[source] std::io::Error),
    #[error("spawn failed: {0}")]
    Spawn(#[source] std::io::Error),
}

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

/// Stop a primal by its family-scoped PID file.
pub(super) fn stop_existing_family(primal: &str, family_id: &str) {
    let pid_dir = PathBuf::from(tolerances::runtime_dir())
        .join(primalspring::env_keys::BIOMEOS_SUBDIR)
        .join(".pids");

    let pid_file = if family_id.is_empty() {
        pid_dir.join(format!("{primal}.pid"))
    } else {
        pid_dir.join(format!("{primal}-{family_id}.pid"))
    };

    if let Ok(contents) = std::fs::read_to_string(&pid_file) {
        if let Ok(pid) = contents.trim().parse::<u32>() {
            let _ = signal_pid(pid);
            let _ = std::fs::remove_file(&pid_file);
            return;
        }
    }

    let legacy_pid_file = pid_dir.join(format!("{primal}.pid"));
    if legacy_pid_file != pid_file {
        if let Ok(contents) = std::fs::read_to_string(&legacy_pid_file) {
            if let Ok(pid) = contents.trim().parse::<u32>() {
                let _ = signal_pid(pid);
                let _ = std::fs::remove_file(&legacy_pid_file);
                return;
            }
        }
    }

    #[cfg(target_os = "linux")]
    stop_by_proc_scan(primal);
}

/// Send SIGTERM to a process by PID.
///
/// Pure-std implementation: writes to `/proc/{pid}/` to confirm existence,
/// then invokes `kill -TERM` via `Command`. No libc FFI, no unsafe, fully
/// portable across any Unix with a `/proc` or `kill(1)` binary.
pub(super) fn signal_pid(pid: u32) -> std::io::Result<()> {
    use std::process::Command;

    let status = Command::new("kill")
        .args(["-s", "TERM", &pid.to_string()])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "kill -TERM {pid}: exited {status}"
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
///
/// CLI args, env vars, and socket wiring are driven by the primal's
/// launch profile (`config/primal_launch_profiles.toml`) rather than
/// hardcoded per-primal if-blocks.
pub(super) fn spawn_primal(
    primal: &str,
    port: u16,
    socket: &std::path::Path,
    config: &LaunchConfig,
    family_seed: &str,
) -> Result<(), SpawnError> {
    let binary = discover_binary(primal).map_err(SpawnError::Discovery)?;
    let (defaults, profiles) =
        primalspring::launcher::load_launch_profiles().map_err(SpawnError::ProfileLoad)?;
    let empty = primalspring::launcher::LaunchProfile::default();
    let profile = profiles.get(primal).unwrap_or(&empty);

    let mut cmd = std::process::Command::new(&binary);

    let subcommand = profile
        .subcommand
        .as_deref()
        .or(defaults.subcommand.as_deref())
        .unwrap_or("server");
    if !subcommand.is_empty() {
        cmd.arg(subcommand);
    }

    let socket_flag = profile
        .socket_flag
        .as_deref()
        .or(defaults.socket_flag.as_deref())
        .unwrap_or("--socket");
    if socket_flag != "__skip__" {
        cmd.arg(socket_flag).arg(socket);
    }

    if port > 0 {
        cmd.arg("--port").arg(port.to_string());
    }

    let pass_fid = profile
        .pass_family_id
        .or(defaults.pass_family_id)
        .unwrap_or(true);
    if pass_fid {
        cmd.arg("--family-id").arg(&config.family_id);
    }

    for arg in &profile.extra_args {
        cmd.arg(arg);
    }

    cmd.env(env_keys::FAMILY_ID, &config.family_id);
    cmd.env(env_keys::FAMILY_SEED, family_seed);
    cmd.env(env_keys::BEARDOG_FAMILY_SEED, family_seed);

    for (key, val) in &defaults.extra_env {
        cmd.env(key, val);
    }
    for (key, val) in &profile.extra_env {
        cmd.env(key, val);
    }

    for (env_key, target_primal) in &profile.env_sockets {
        let resolved = resolve_profile_var(target_primal, socket, config);
        cmd.env(env_key, &resolved);
    }

    for (flag, target_primal) in &profile.cli_sockets {
        let resolved = resolve_profile_var(target_primal, socket, config);
        cmd.arg(flag).arg(&resolved);
    }

    for env_key in profile.passthrough_env.keys() {
        if let Ok(val) = std::env::var(env_key) {
            cmd.env(env_key, val);
        }
    }

    if config.dark_forest {
        cmd.arg("--dark-forest");
    }

    if let Some(fed_port) = config.federation_port {
        if profile.extra_env.contains_key("SONGBIRD_SECURITY_PROVIDER")
            || profile.extra_env.contains_key("SONGBIRD_DISCOVERY_MODE")
        {
            cmd.arg("--federation-port").arg(fed_port.to_string());
            cmd.arg("--bind").arg(tolerances::LAN_BIND_ADDRESS);
        }
    }

    let log_dir = PathBuf::from(tolerances::runtime_dir())
        .join(primalspring::env_keys::BIOMEOS_SUBDIR)
        .join("logs");
    let _ = std::fs::create_dir_all(&log_dir);
    let log_path = log_dir.join(format!("{primal}.log"));
    let log_file = std::fs::File::create(&log_path).map_err(SpawnError::Io)?;
    let log_err = log_file.try_clone().map_err(SpawnError::Io)?;

    cmd.stdout(log_file);
    cmd.stderr(log_err);

    let child = cmd.spawn().map_err(SpawnError::Spawn)?;

    let pid_dir = PathBuf::from(tolerances::runtime_dir())
        .join(primalspring::env_keys::BIOMEOS_SUBDIR)
        .join(".pids");
    let _ = std::fs::create_dir_all(&pid_dir);
    let pid_filename = if config.family_id.is_empty() {
        format!("{primal}.pid")
    } else {
        format!("{primal}-{}.pid", config.family_id)
    };
    let _ = std::fs::write(pid_dir.join(pid_filename), child.id().to_string());

    tracing::info!(primal, binary = %binary.display(), pid = child.id(), "spawned");
    Ok(())
}

/// Resolve profile variable placeholders.
///
/// `$family_id` → config.family_id, `$base_dir` / `$biomeos_dir` → socket parent,
/// primal names → their socket path via standard nucleation.
fn resolve_profile_var(val: &str, own_socket: &std::path::Path, config: &LaunchConfig) -> String {
    match val {
        "$family_id" => config.family_id.clone(),
        "$base_dir" | "$biomeos_dir" => own_socket
            .parent()
            .map(|p| p.display().to_string())
            .unwrap_or_default(),
        primal_ref => {
            let socket_dir = own_socket
                .parent()
                .unwrap_or_else(|| std::path::Path::new("/tmp"));
            let family = &config.family_id;
            socket_dir
                .join(primalspring::ipc::discover::socket_filename(
                    primal_ref, family,
                ))
                .display()
                .to_string()
        }
    }
}
