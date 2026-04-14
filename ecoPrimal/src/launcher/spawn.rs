// SPDX-License-Identifier: AGPL-3.0-or-later

//! Process spawning, socket readiness, and stderr relay for child primals.

use std::collections::HashMap;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use crate::tolerances;
use tracing::{debug, info};

use super::LaunchError;
use super::discovery::discover_binary;
use super::profiles::{LaunchProfile, load_launch_profiles};

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
    family_seed: Option<Vec<u8>>,
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
            family_seed: None,
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

    /// Set the BTSP family seed for this composition.
    ///
    /// When set, [`spawn_primal`] injects `FAMILY_SEED` into each child
    /// process so BearDog can start in Production BTSP mode.
    pub fn set_family_seed(&mut self, seed: Vec<u8>) {
        self.family_seed = Some(seed);
    }

    /// The BTSP family seed, if configured.
    #[must_use]
    pub fn family_seed(&self) -> Option<&[u8]> {
        self.family_seed.as_deref()
    }
}

// ---------------------------------------------------------------------------
// PrimalProcess
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

    /// Create a `PrimalProcess` with an active stderr relay thread.
    #[must_use]
    pub const fn with_stderr_relay(
        name: String,
        socket_path: PathBuf,
        child: Child,
        relay_handle: std::thread::JoinHandle<()>,
    ) -> Self {
        Self {
            name,
            socket_path,
            child,
            _relay_handle: Some(relay_handle),
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

// ---------------------------------------------------------------------------
// spawn_primal + socket wait
// ---------------------------------------------------------------------------

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
    if let Some(seed) = nucleation.family_seed() {
        cmd.env("FAMILY_SEED", String::from_utf8_lossy(seed).as_ref());
    }
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

    debug!(
        primal = %primal,
        binary = %binary.display(),
        "[launcher] spawning primal"
    );

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

    info!(
        primal = %primal,
        socket = %effective_socket.display(),
        pid = child.id(),
        "[launcher] primal ready"
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

/// Spawn a thread that reads a child's stderr and logs each line at `debug`.
pub fn relay_output(child: &mut Child, primal: &str) -> std::thread::JoinHandle<()> {
    let stderr = child.stderr.take();
    let name = primal.to_owned();
    std::thread::spawn(move || {
        if let Some(stream) = stderr {
            let reader = BufReader::new(stream);
            for line in reader.lines().map_while(Result::ok) {
                debug!(primal = %name, line = %line, "child stderr");
            }
        }
    })
}
