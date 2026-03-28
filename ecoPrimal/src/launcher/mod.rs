// SPDX-License-Identifier: AGPL-3.0-or-later

//! Primal process launching and lifecycle management.
//!
//! Synchronous port of biomeOS `primal_spawner` / `nucleation` modules,
//! adapted for primalSpring's coordination validation domain.
//!
//! # Binary Discovery
//!
//! [`discover_binary`] resolves a primal binary using the same 5-tier
//! search order as biomeOS:
//!
//! 1. `$ECOPRIMALS_PLASMID_BIN`
//! 2. `$BIOMEOS_PLASMID_BIN_DIR`
//! 3. `./plasmidBin`
//! 4. `../plasmidBin`
//! 5. `../../plasmidBin`
//!
//! Within each base directory, 6 binary-name patterns are tried.
//!
//! # Socket Nucleation
//!
//! [`SocketNucleation`] assigns deterministic socket paths so that
//! primals and their dependents agree on socket locations before any
//! process starts.
//!
//! # Launch Profiles
//!
//! [`LaunchProfile`] is loaded from `config/primal_launch_profiles.toml`
//! (compile-time `include_str!`). Profiles describe per-primal CLI flags,
//! environment variables, and cross-primal socket wiring.

use std::collections::HashMap;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use crate::tolerances;

/// Env var: override base directory for primal binaries.
const ENV_PLASMID_BIN: &str = "ECOPRIMALS_PLASMID_BIN";

/// Env var: biomeOS plasmid bin directory.
const ENV_BIOMEOS_BIN_DIR: &str = "BIOMEOS_PLASMID_BIN_DIR";

/// Relative fallback paths for plasmidBin (tiers 3-5 of binary discovery).
const RELATIVE_PLASMID_TIERS: &[&str] = &["./plasmidBin", "../plasmidBin", "../../plasmidBin"];

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Typed errors for primal launch operations.
#[derive(Debug, thiserror::Error)]
pub enum LaunchError {
    /// Binary not found after searching all tiers and patterns.
    #[error("binary not found for '{primal}'; searched: {searched:?}")]
    BinaryNotFound {
        /// The primal name that was searched for.
        primal: String,
        /// Candidate paths that were checked.
        searched: Vec<PathBuf>,
    },
    /// `std::process::Command::spawn` failed.
    #[error("spawn failed for '{primal}': {source}")]
    SpawnFailed {
        /// The primal whose binary failed to spawn.
        primal: String,
        /// The underlying I/O error.
        source: std::io::Error,
    },
    /// Socket did not appear within the timeout.
    #[error("socket timeout for '{primal}' at {socket} after {waited:.1?}")]
    SocketTimeout {
        /// The primal whose socket was expected.
        primal: String,
        /// The socket path that was waited on.
        socket: PathBuf,
        /// How long we waited before giving up.
        waited: Duration,
    },
    /// A spawned primal failed its post-launch health check.
    #[error("health check failed for '{primal}': {detail}")]
    HealthCheckFailed {
        /// The primal that failed the check.
        primal: String,
        /// Detail from the failed health call.
        detail: String,
    },
    /// Launch profiles TOML failed to parse.
    #[error("launch profile parse error: {0}")]
    ProfileParseError(
        /// Parse error detail.
        String,
    ),
}

// ---------------------------------------------------------------------------
// Binary discovery
// ---------------------------------------------------------------------------

/// Search for a primal binary using the 5-tier directory search and
/// 6 binary-name patterns (same algorithm as biomeOS `discover_primal_binary`).
///
/// # Errors
///
/// Returns [`LaunchError::BinaryNotFound`] if no matching executable is
/// found after exhausting all directories and patterns.
pub fn discover_binary(primal: &str) -> Result<PathBuf, LaunchError> {
    let env_overrides: Vec<Option<PathBuf>> = vec![
        std::env::var(ENV_PLASMID_BIN).ok().map(PathBuf::from),
        std::env::var(ENV_BIOMEOS_BIN_DIR).ok().map(PathBuf::from),
    ];
    let base_dirs: Vec<Option<PathBuf>> = env_overrides
        .into_iter()
        .chain(
            RELATIVE_PLASMID_TIERS
                .iter()
                .map(|p| Some(PathBuf::from(p))),
        )
        .collect();

    let arch = std::env::consts::ARCH;
    let os = std::env::consts::OS;

    let patterns = [
        format!("{primal}_{arch}_{os}_musl/{primal}"),
        format!("{primal}_{arch}_{os}/{primal}"),
        format!("primals/{primal}/{primal}"),
        format!("primals/{primal}"),
        format!("{primal}/{primal}"),
        primal.to_string(),
    ];

    let mut searched = Vec::new();

    for base in base_dirs.iter().filter_map(Option::as_ref) {
        if !base.exists() {
            continue;
        }
        for pattern in &patterns {
            let candidate = base.join(pattern);
            if candidate.is_file() {
                return Ok(candidate);
            }
            searched.push(candidate);
        }
    }

    Err(LaunchError::BinaryNotFound {
        primal: primal.to_owned(),
        searched,
    })
}

// ---------------------------------------------------------------------------
// Socket nucleation
// ---------------------------------------------------------------------------

/// Deterministic socket path assignment for coordinated primal startup.
///
/// Assigns `$XDG_RUNTIME_DIR/biomeos/{primal}-{family}.sock` (or `/tmp/`
/// fallback) before any process is spawned, so that both the primal and
/// its dependents agree on the socket location.
pub struct SocketNucleation {
    base_dir: PathBuf,
    assignments: HashMap<String, PathBuf>,
}

impl SocketNucleation {
    /// Create a nucleation coordinator rooted at `base_dir`.
    ///
    /// `base_dir` should be `$XDG_RUNTIME_DIR` or a test-specific temp dir.
    /// A `biomeos/` subdirectory is created automatically.
    #[must_use]
    pub fn new(base_dir: PathBuf) -> Self {
        let biomeos_dir = base_dir.join("biomeos");
        let _ = std::fs::create_dir_all(&biomeos_dir);
        Self {
            base_dir,
            assignments: HashMap::new(),
        }
    }

    /// Create a nucleation coordinator using the default XDG runtime dir.
    #[must_use]
    pub fn from_env() -> Self {
        let base =
            std::env::var("XDG_RUNTIME_DIR").map_or_else(|_| std::env::temp_dir(), PathBuf::from);
        Self::new(base)
    }

    /// Assign a socket path for `primal` with the given `family_id`.
    ///
    /// Idempotent — returns the same path on repeated calls.
    pub fn assign(&mut self, primal: &str, family_id: &str) -> PathBuf {
        let key = format!("{primal}-{family_id}");
        if let Some(existing) = self.assignments.get(&key) {
            return existing.clone();
        }
        let socket = self.base_dir.join("biomeos").join(format!("{key}.sock"));
        self.assignments.insert(key, socket.clone());
        socket
    }

    /// Assign sockets for all primals in `names`.
    pub fn assign_batch(&mut self, names: &[&str], family_id: &str) -> HashMap<String, PathBuf> {
        names
            .iter()
            .map(|name| ((*name).to_owned(), self.assign(name, family_id)))
            .collect()
    }

    /// Look up a previously assigned socket (returns `None` if unassigned).
    #[must_use]
    pub fn get(&self, primal: &str, family_id: &str) -> Option<&PathBuf> {
        let key = format!("{primal}-{family_id}");
        self.assignments.get(&key)
    }

    /// Remap a primal's socket path (e.g. to point to a JSON-RPC suffix).
    pub fn remap(&mut self, primal: &str, family_id: &str, new_path: PathBuf) {
        let key = format!("{primal}-{family_id}");
        self.assignments.insert(key, new_path);
    }

    /// The base directory (typically `$XDG_RUNTIME_DIR`).
    #[must_use]
    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }
}

// ---------------------------------------------------------------------------
// Launch profiles
// ---------------------------------------------------------------------------

static LAUNCH_PROFILES_TOML: &str = include_str!("../../../config/primal_launch_profiles.toml");

/// Per-primal socket configuration loaded from TOML.
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct LaunchProfile {
    /// Override the default `"server"` subcommand (e.g. `"daemon"`).
    /// Set to `""` to skip the subcommand entirely.
    pub subcommand: Option<String>,
    /// CLI flag name for passing socket path (e.g. `"--socket"`).
    /// Set to `"__skip__"` to omit the socket CLI flag entirely.
    pub socket_flag: Option<String>,
    /// Suffix appended to the assigned socket path to get the JSON-RPC
    /// socket (e.g. `".jsonrpc.sock"` for toadstool's dual-protocol mode).
    #[serde(default)]
    pub jsonrpc_socket_suffix: Option<String>,
    /// Whether to pass `--family-id` on the command line.
    pub pass_family_id: Option<bool>,
    /// Env var name for socket path fallback (e.g. `"PRIMAL_SOCKET"`).
    pub env_socket: Option<String>,
    /// Static environment variables to set on the child process.
    #[serde(default)]
    pub extra_env: HashMap<String, String>,
    /// Env vars whose values are resolved socket paths of other primals.
    #[serde(default)]
    pub env_sockets: HashMap<String, String>,
    /// Extra CLI flags whose values are resolved socket paths.
    #[serde(default)]
    pub cli_sockets: HashMap<String, String>,
    /// Extra CLI arguments to pass verbatim (e.g. `["--port", "0"]`).
    #[serde(default)]
    pub extra_args: Vec<String>,
    /// Env vars to forward from the parent process when set.
    #[serde(default)]
    pub passthrough_env: HashMap<String, bool>,
}

#[derive(Debug, serde::Deserialize)]
struct ProfilesConfig {
    default: LaunchProfile,
    #[serde(default)]
    profiles: HashMap<String, LaunchProfile>,
}

/// Load launch profiles from the embedded TOML.
///
/// # Errors
///
/// Returns [`LaunchError::ProfileParseError`] if the TOML is malformed.
pub fn load_launch_profiles() -> Result<(LaunchProfile, HashMap<String, LaunchProfile>), LaunchError>
{
    let config: ProfilesConfig = toml::from_str(LAUNCH_PROFILES_TOML)
        .map_err(|e| LaunchError::ProfileParseError(e.to_string()))?;
    Ok((config.default, config.profiles))
}

// ---------------------------------------------------------------------------
// Spawn + wait
// ---------------------------------------------------------------------------

/// A running primal process with RAII cleanup.
///
/// When dropped, sends `SIGTERM` (via `Child::kill`) and waits for exit.
/// The socket file is removed if it still exists.
pub struct PrimalProcess {
    /// Primal name (e.g. `"beardog"`).
    pub name: String,
    /// Path to the Unix socket file.
    pub socket_path: PathBuf,
    child: Child,
    _relay_handle: Option<std::thread::JoinHandle<()>>,
}

impl PrimalProcess {
    /// The child PID.
    #[must_use]
    pub fn pid(&self) -> u32 {
        self.child.id()
    }

    /// Construct from pre-spawned parts (for custom spawn logic).
    #[must_use]
    pub const fn from_parts(name: String, socket_path: PathBuf, child: Child) -> Self {
        Self {
            name,
            socket_path,
            child,
            _relay_handle: None,
        }
    }
}

impl Drop for PrimalProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = std::fs::remove_file(&self.socket_path);
    }
}

impl fmt::Debug for PrimalProcess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PrimalProcess")
            .field("name", &self.name)
            .field("socket_path", &self.socket_path)
            .field("pid", &self.child.id())
            .finish_non_exhaustive()
    }
}

/// Spawn a primal process and wait for its socket to appear.
///
/// # Arguments
///
/// * `primal` — primal name (e.g. `"beardog"`)
/// * `family_id` — family identifier for socket naming
/// * `nucleation` — socket nucleation coordinator
///
/// # Errors
///
/// Returns [`LaunchError`] on binary-not-found, spawn failure, or
/// socket timeout.
pub fn spawn_primal(
    primal: &str,
    family_id: &str,
    nucleation: &mut SocketNucleation,
) -> Result<PrimalProcess, LaunchError> {
    let binary = discover_binary(primal)?;
    let socket_path = nucleation.assign(primal, family_id);

    let (defaults, profiles) = load_launch_profiles()?;
    let profile = profiles.get(primal);

    let socket_flag = profile
        .and_then(|p| p.socket_flag.as_deref())
        .or(defaults.socket_flag.as_deref())
        .unwrap_or("--socket");

    let pass_family_id = profile
        .and_then(|p| p.pass_family_id)
        .or(defaults.pass_family_id)
        .unwrap_or(true);

    let subcommand = profile
        .and_then(|p| p.subcommand.as_deref())
        .unwrap_or("server");

    let mut cmd = Command::new(&binary);
    if !subcommand.is_empty() {
        cmd.arg(subcommand);
    }
    if socket_flag != "__skip__" {
        cmd.arg(socket_flag).arg(&socket_path);
    }

    if pass_family_id {
        cmd.arg("--family-id").arg(family_id);
    }

    cmd.env("FAMILY_ID", family_id);
    cmd.env(
        "XDG_RUNTIME_DIR",
        nucleation.base_dir().to_string_lossy().as_ref(),
    );

    if let Some(p) = profile {
        let base_dir_str = nucleation.base_dir().to_string_lossy().to_string();
        for (key, value) in &p.extra_env {
            let resolved = value.replace("$base_dir", &base_dir_str);
            cmd.env(key, &resolved);
        }
        for (env_name, socket_ref) in &p.env_sockets {
            if socket_ref == "$family_id" {
                cmd.env(env_name, family_id);
            } else if let Some(resolved) = nucleation.get(socket_ref, family_id) {
                cmd.env(env_name, resolved);
            }
        }
        for (flag, socket_ref) in &p.cli_sockets {
            if let Some(resolved) = nucleation.get(socket_ref, family_id) {
                cmd.arg(flag).arg(resolved);
            }
        }
        for arg in &p.extra_args {
            cmd.arg(arg);
        }
        for (env_name, &enabled) in &p.passthrough_env {
            if enabled {
                if let Ok(val) = std::env::var(env_name) {
                    cmd.env(env_name, val);
                }
            }
        }
    }

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    println!("[launcher] spawning {primal} from {}", binary.display());

    let mut child = cmd.spawn().map_err(|e| LaunchError::SpawnFailed {
        primal: primal.to_owned(),
        source: e,
    })?;

    let relay_handle = relay_output(&mut child, primal);

    let effective_socket = await_socket_ready(
        primal,
        family_id,
        profile,
        socket_path,
        nucleation,
        &mut child,
    )?;

    println!(
        "[launcher] {primal} ready at {} (pid {})",
        effective_socket.display(),
        child.id()
    );

    Ok(PrimalProcess {
        name: primal.to_owned(),
        socket_path: effective_socket,
        child,
        _relay_handle: Some(relay_handle),
    })
}

/// Wait for the primal's JSON-RPC socket to appear and resolve the effective
/// socket path. Some primals (e.g. toadstool) expose a JSON-RPC socket at a
/// suffix-derived path separate from the primary tarpc socket.
fn await_socket_ready(
    primal: &str,
    family_id: &str,
    profile: Option<&LaunchProfile>,
    socket_path: PathBuf,
    nucleation: &mut SocketNucleation,
    child: &mut std::process::Child,
) -> Result<PathBuf, LaunchError> {
    let wait_path = profile
        .and_then(|p| p.jsonrpc_socket_suffix.as_deref())
        .map_or_else(
            || socket_path.clone(),
            |suffix| {
                let base = socket_path.to_string_lossy();
                PathBuf::from(base.replace(".sock", suffix))
            },
        );

    let timeout = Duration::from_secs(tolerances::LAUNCHER_SOCKET_TIMEOUT_SECS);
    if !wait_for_socket(&wait_path, timeout) {
        let _ = child.kill();
        let _ = child.wait();
        return Err(LaunchError::SocketTimeout {
            primal: primal.to_owned(),
            socket: wait_path,
            waited: timeout,
        });
    }

    if wait_path == socket_path {
        Ok(socket_path)
    } else {
        nucleation.remap(primal, family_id, wait_path.clone());
        Ok(wait_path)
    }
}

/// Discover the biomeOS binary in `plasmidBin/primals/` or `$PATH`.
///
/// biomeOS is the substrate primal — the ecosystem's composition,
/// coordination, and deployment orchestrator. The Neural API is one of
/// its UniBin modes (`biomeos api`).
///
/// # Errors
///
/// Returns [`LaunchError::BinaryNotFound`] if `biomeos` is not found.
pub fn discover_biomeos_binary() -> Result<PathBuf, LaunchError> {
    discover_binary("biomeos")
}

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
    cmd.env(
        "XDG_RUNTIME_DIR",
        nucleation.base_dir().to_string_lossy().as_ref(),
    );
    cmd.env("BIOMEOS_MODE", "coordinated");
    if let Ok(plasmid) = std::env::var("ECOPRIMALS_PLASMID_BIN") {
        cmd.env("BIOMEOS_PLASMID_BIN_DIR", &plasmid);
    }

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    println!(
        "[launcher] spawning biomeOS (neural-api mode) from {}",
        binary.display()
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

    println!(
        "[launcher] biomeOS ready at {} (pid {})",
        socket_path.display(),
        child.id()
    );

    Ok(PrimalProcess {
        name: "biomeos".to_owned(),
        socket_path,
        child,
        _relay_handle: Some(relay_handle),
    })
}

/// Legacy alias for [`spawn_biomeos`].
///
/// # Errors
///
/// Returns [`LaunchError`] on binary-not-found, spawn failure, or socket timeout.
#[deprecated(note = "use spawn_biomeos() — Neural API is a biomeOS mode, not a separate entity")]
pub fn spawn_neural_api(
    family_id: &str,
    nucleation: &SocketNucleation,
    graphs_dir: &Path,
) -> Result<PrimalProcess, LaunchError> {
    spawn_biomeos(family_id, nucleation, graphs_dir)
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

/// Poll for a socket file to appear on disk.
///
/// Returns `true` if the socket appeared, `false` on timeout.
#[must_use]
pub fn wait_for_socket(path: &Path, timeout: Duration) -> bool {
    let start = Instant::now();
    let poll_interval = Duration::from_millis(tolerances::LAUNCHER_POLL_INTERVAL_MS);
    while start.elapsed() < timeout {
        if path.exists() {
            std::thread::sleep(Duration::from_millis(tolerances::LAUNCHER_SOCKET_SETTLE_MS));
            return true;
        }
        std::thread::sleep(poll_interval);
    }
    false
}

/// Spawn a thread that reads a child's stderr and prints prefixed lines.
fn relay_output(child: &mut Child, primal: &str) -> std::thread::JoinHandle<()> {
    let stderr = child.stderr.take();
    let name = primal.to_owned();
    std::thread::spawn(move || {
        if let Some(stream) = stderr {
            let reader = BufReader::new(stream);
            for line in reader.lines().map_while(Result::ok) {
                eprintln!("[{name}] {line}");
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests;
