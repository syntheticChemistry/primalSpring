// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Process lifecycle — spawn, stop, seed resolution for NUCLEUS primals.

use std::path::PathBuf;

use primalspring::env_keys;
use primalspring::launcher::discover_binary;
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
/// Uses the `nix` crate's safe wrapper around `kill(2)` — no unsafe code,
/// no external process spawn, typed `Errno` for ESRCH (process already gone).
pub(super) fn signal_pid(pid: u32) -> std::io::Result<()> {
    use nix::sys::signal::{Signal, kill};
    use nix::unistd::Pid;

    let nix_pid = Pid::from_raw(i32::try_from(pid).map_err(|e| {
        std::io::Error::other(format!("PID {pid} out of i32 range: {e}"))
    })?);
    kill(nix_pid, Signal::SIGTERM).map_err(|e| {
        std::io::Error::other(format!("kill({pid}, SIGTERM): {e}"))
    })
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
) -> Result<(), String> {
    let binary = discover_binary(primal).map_err(|e| e.to_string())?;
    let (defaults, profiles) = primalspring::launcher::load_launch_profiles()
        .map_err(|e| format!("profile load: {e}"))?;
    let empty = primalspring::launcher::LaunchProfile::default();
    let profile = profiles.get(primal).unwrap_or(&empty);

    let mut cmd = std::process::Command::new(&binary);

    let subcommand = profile.subcommand.as_deref()
        .or(defaults.subcommand.as_deref())
        .unwrap_or("server");
    if !subcommand.is_empty() {
        cmd.arg(subcommand);
    }

    let socket_flag = profile.socket_flag.as_deref()
        .or(defaults.socket_flag.as_deref())
        .unwrap_or("--socket");
    if socket_flag != "__skip__" {
        cmd.arg(socket_flag).arg(socket);
    }

    if port > 0 {
        cmd.arg("--port").arg(port.to_string());
    }

    let pass_fid = profile.pass_family_id
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
                .join(format!("{primal_ref}-{family}.sock"))
                .display()
                .to_string()
        }
    }
}
