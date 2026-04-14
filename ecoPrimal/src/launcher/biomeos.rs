// SPDX-License-Identifier: AGPL-3.0-or-later

//! biomeOS Neural API process launch (substrate primal).

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use crate::tolerances;
use tracing::{debug, info};

use super::LaunchError;
use super::discovery::discover_biomeos_binary;
use super::spawn::{PrimalProcess, SocketNucleation, relay_output, wait_for_socket};

/// Spawn biomeOS in neural-api mode (graph orchestration + capability routing).
///
/// biomeOS is the substrate primal. Its `neural-api` mode provides
/// graph execution, capability routing, and primal coordination. Unlike
/// regular primals that use `server` subcommand, biomeOS uses the
/// `neural-api` subcommand.
///
/// The socket is created at `{nucleation_base}/biomeos/neural-api-{family}.sock`.
/// biomeOS detects already-running primals via its own nucleation and
/// enters companion mode.
///
/// # Arguments
///
/// * `family_id` — shared family identifier (must match the primals)
/// * `nucleation` — socket nucleation coordinator (for `XDG_RUNTIME_DIR`)
/// * `graphs_dir` — directory containing deploy graph TOMLs
///
/// # Errors
///
/// Returns [`LaunchError`] on binary-not-found, spawn failure, or socket timeout.
pub fn spawn_biomeos(
    family_id: &str,
    nucleation: &SocketNucleation,
    graphs_dir: &Path,
) -> Result<PrimalProcess, LaunchError> {
    let relative_binary = discover_biomeos_binary()?;
    let binary = std::fs::canonicalize(&relative_binary).unwrap_or(relative_binary);

    let biomeos_dir = nucleation.base_dir().join("biomeos");
    let _ = std::fs::create_dir_all(&biomeos_dir);
    let socket_path = biomeos_dir.join(format!("neural-api-{family_id}.sock"));
    let _ = std::fs::remove_file(&socket_path);

    let effective_graphs_dir = discover_biomeos_graphs(graphs_dir);
    let working_dir = effective_graphs_dir
        .parent()
        .unwrap_or(&effective_graphs_dir);

    let mut cmd = Command::new(&binary);
    cmd.arg("neural-api");
    cmd.arg("--socket").arg(&socket_path);
    cmd.arg("--graphs-dir").arg(&effective_graphs_dir);
    cmd.arg("--family-id").arg(family_id);
    cmd.current_dir(working_dir);
    cmd.env("FAMILY_ID", family_id);
    if let Some(seed) = nucleation.family_seed() {
        cmd.env("FAMILY_SEED", String::from_utf8_lossy(seed).as_ref());
    }
    cmd.env(
        "XDG_RUNTIME_DIR",
        nucleation.base_dir().to_string_lossy().as_ref(),
    );
    cmd.env("BIOMEOS_MODE", "coordinated");
    if let Ok(plasmid) = std::env::var(super::discovery::ENV_PLASMID_BIN) {
        cmd.env("BIOMEOS_PLASMID_BIN_DIR", &plasmid);
    }

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    debug!(
        binary = %binary.display(),
        "[launcher] spawning biomeOS (neural-api mode)"
    );

    let mut child = cmd.spawn().map_err(|e| LaunchError::SpawnFailed {
        primal: "biomeos".to_owned(),
        source: e,
    })?;

    let relay_handle = relay_output(&mut child, "biomeos");

    let timeout = Duration::from_secs(tolerances::LAUNCHER_SOCKET_TIMEOUT_SECS);
    if !wait_for_socket(&socket_path, timeout) {
        let _ = child.kill();
        let _ = child.wait();
        return Err(LaunchError::SocketTimeout {
            primal: "biomeos".to_owned(),
            socket: socket_path,
            waited: timeout,
        });
    }

    info!(
        socket = %socket_path.display(),
        pid = child.id(),
        "[launcher] biomeOS ready"
    );

    Ok(PrimalProcess::with_stderr_relay(
        "biomeos".to_owned(),
        socket_path,
        child,
        relay_handle,
    ))
}

/// Discover the biomeOS graphs directory, preferring the biomeOS source tree
/// (which has the full `[[nodes]]` graph and `../config/capability_registry.toml`).
/// Falls back to the caller-provided directory.
fn discover_biomeos_graphs(fallback: &Path) -> PathBuf {
    const ENV_BIOMEOS_GRAPHS: &str = "BIOMEOS_GRAPHS_DIR";
    if let Ok(val) = std::env::var(ENV_BIOMEOS_GRAPHS) {
        let p = PathBuf::from(&val);
        if p.is_dir() {
            return p;
        }
    }

    let candidates = [
        PathBuf::from("../primals/biomeOS/graphs"),
        PathBuf::from("../../primals/biomeOS/graphs"),
        PathBuf::from("../../../primals/biomeOS/graphs"),
    ];
    for candidate in &candidates {
        if candidate.join("tower_atomic_bootstrap.toml").is_file()
            && candidate
                .join("../config/capability_registry.toml")
                .is_file()
        {
            if let Ok(p) = std::fs::canonicalize(candidate) {
                return p;
            }
        }
    }

    fallback.to_path_buf()
}
