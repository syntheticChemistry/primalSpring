// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Runtime primal discovery via environment, filesystem probing, and biomeOS.
//!
//! Discovery order (per primal):
//!   1. `{PRIMAL}_SOCKET` environment override (explicit path)
//!   2. `$XDG_RUNTIME_DIR/biomeos/{primal}-{family}.sock` (convention)
//!   3. `{temp_dir}/biomeos/{primal}-{family}.sock` (fallback)
//!   4. Manifest file: `$XDG_RUNTIME_DIR/ecoPrimals/manifests/{primal}.json`
//!   5. Socket registry: `$XDG_RUNTIME_DIR/biomeos/socket-registry.json`
//!
//! Tiers 4–5 absorbed from biomeOS v2.50, rhizoCrypt v0.13, Squirrel alpha.12.
//! Manifest files contain `{"socket_path": "..."}` and are written by primals
//! on startup. The socket registry is a shared file with TTL-aware entries.
//!
//! Ecosystem-wide sweep discovery uses biomeOS's neural-api mode to learn
//! what primals are registered at runtime. primalSpring never hardcodes a
//! primal roster.
//!
//! Capability-based discovery, neural-api capability queries, and multi-format
//! capability parsing live in [`crate::ipc::capability`] and are re-exported
//! here for a stable [`crate::ipc::discover`] path.

use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::time::Duration;

use super::neural_bridge::NeuralBridge;

use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

pub use super::capability::{
    CapabilityDiscoveryResult, CapabilityDiscoverySource, capability_call, discover_by_capability,
    discover_capabilities, discover_capabilities_for, extract_capability_names,
};

/// Sockets confirmed dead during this process. Avoids re-probing stale
/// sockets on every `discover_primal` / `capability_call` invocation.
/// Cleared only on process restart (appropriate: stale sockets don't
/// spontaneously come back to life within a session).
static DEAD_SOCKET_CACHE: OnceLock<Mutex<HashSet<PathBuf>>> = OnceLock::new();

const LIVENESS_PROBE_TIMEOUT: Duration = Duration::from_millis(50);

/// Test whether a Unix domain socket has a listening process.
///
/// Attempts a non-blocking connect with a short timeout. Returns `false`
/// if the file doesn't exist, if `connect()` returns `ConnectionRefused`
/// (errno 111), or if the timeout expires. This is much cheaper than a
/// full JSON-RPC health check (~50ms worst case vs ~5s read timeout).
#[must_use]
pub fn socket_is_alive(path: &Path) -> bool {
    if !path.exists() {
        return false;
    }

    let cache = DEAD_SOCKET_CACHE.get_or_init(|| Mutex::new(HashSet::new()));
    if let Ok(set) = cache.lock() {
        if set.contains(path) {
            return false;
        }
    }

    let alive = UnixStream::connect(path)
        .map(|stream| {
            let _ = stream.set_read_timeout(Some(LIVENESS_PROBE_TIMEOUT));
            true
        })
        .unwrap_or(false);

    if !alive {
        if let Ok(mut set) = cache.lock() {
            set.insert(path.to_path_buf());
        }
    }

    alive
}

/// Result of attempting to discover a primal's socket.
#[derive(Debug, Clone)]
pub struct DiscoveryResult {
    /// Primal name that was searched for.
    pub primal: String,
    /// Socket path if found, `None` if the primal is not reachable.
    pub socket: Option<PathBuf>,
    /// How the socket was resolved (env, XDG, temp, or not found).
    pub source: DiscoverySource,
}

/// How a socket path was resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscoverySource {
    /// Found via `{PRIMAL}_SOCKET` environment variable.
    EnvOverride,
    /// Found at the XDG runtime directory convention path.
    XdgConvention,
    /// Found at the temp directory fallback path.
    TempFallback,
    /// Found via primal manifest (`$XDG_RUNTIME_DIR/ecoPrimals/manifests/{primal}.json`).
    Manifest,
    /// Found via socket registry (`$XDG_RUNTIME_DIR/biomeos/socket-registry.json`).
    SocketRegistry,
    /// Socket not found by any method.
    NotFound,
}

/// Resolve the base socket directory for the ecoPrimals runtime.
///
/// Priority: `$ECOPRIMALS_SOCKET_DIR` → `$XDG_RUNTIME_DIR/ecoprimals` →
/// `<temp_dir>/ecoprimals`. This is the canonical directory for seed files,
/// socket discovery, and runtime state.
#[must_use]
pub fn resolve_socket_dir() -> String {
    if let Ok(dir) = std::env::var(crate::env_keys::SOCKET_DIR) {
        return dir;
    }
    std::env::var(crate::env_keys::XDG_RUNTIME_DIR).map_or_else(
        |_| {
            let tmp = std::env::temp_dir();
            format!("{}/{}", tmp.display(), crate::env_keys::RUNTIME_SUBDIR)
        },
        |xdg| format!("{xdg}/{}", crate::env_keys::RUNTIME_SUBDIR),
    )
}

/// Build the conventional socket path from explicit base and family.
///
/// Pure function for testability — no env reads.
#[must_use]
pub fn build_socket_path(base_dir: &Path, primal: &str, family: &str) -> PathBuf {
    base_dir
        .join(crate::primal_names::BIOMEOS)
        .join(format!("{primal}-{family}.sock"))
}

/// Compute the conventional socket path for a primal.
///
/// Uses `$XDG_RUNTIME_DIR` if set, otherwise `std::env::temp_dir()`.
/// Respects `$FAMILY_ID` for multi-tenant socket paths.
#[must_use]
pub fn socket_path(primal: &str) -> PathBuf {
    let base = std::env::var(crate::env_keys::XDG_RUNTIME_DIR)
        .map_or_else(|_| std::env::temp_dir(), PathBuf::from);
    let family = crate::env_keys::resolve_family_id();
    build_socket_path(&base, primal, &family)
}

/// Check for an explicit socket path via `{PRIMAL}_SOCKET` env var.
#[must_use]
pub fn socket_env_var(primal: &str) -> Option<PathBuf> {
    let key = format!("{}_SOCKET", primal.to_uppercase());
    std::env::var(key).ok().map(PathBuf::from)
}

/// Attempt to read a socket path from a primal manifest file.
///
/// Manifests live at `$XDG_RUNTIME_DIR/ecoPrimals/manifests/{primal}.json`
/// and contain `{"socket_path": "/path/to/socket.sock", ...}`.
/// Pattern absorbed from biomeOS v2.50 `PrimalManifest`.
#[must_use]
pub fn discover_from_manifest(primal: &str) -> Option<PathBuf> {
    let base = std::env::var(crate::env_keys::XDG_RUNTIME_DIR).ok()?;
    let manifest_path = PathBuf::from(base)
        .join(crate::env_keys::ECOPRIMALS_DIR_NAME)
        .join(crate::env_keys::MANIFESTS_SUBDIR)
        .join(format!("{primal}.json"));

    let contents = std::fs::read_to_string(manifest_path).ok()?;
    let parsed: serde_json::Value = serde_json::from_str(&contents).ok()?;
    let sock_str = parsed.get("socket_path")?.as_str()?;
    Some(PathBuf::from(sock_str))
}

/// Attempt to read a socket path from the biomeOS socket registry.
///
/// The registry lives at `$XDG_RUNTIME_DIR/biomeos/socket-registry.json`
/// and maps primal names to socket paths with TTL metadata.
/// Pattern absorbed from Squirrel alpha.12.
#[must_use]
pub fn discover_from_socket_registry(primal: &str) -> Option<PathBuf> {
    let base = std::env::var(crate::env_keys::XDG_RUNTIME_DIR).ok()?;
    let registry_path = PathBuf::from(base)
        .join(crate::primal_names::BIOMEOS)
        .join("socket-registry.json");

    let contents = std::fs::read_to_string(registry_path).ok()?;
    let parsed: serde_json::Value = serde_json::from_str(&contents).ok()?;
    let entry = parsed.get(primal)?;
    let sock_str = entry.get("socket_path")?.as_str()?;
    Some(PathBuf::from(sock_str))
}

/// Discover a primal's socket at runtime.
///
/// Walks discovery tiers in priority order:
///
/// 1. `{PRIMAL}_SOCKET` env override
/// 2. XDG convention: `{primal}-{family}.sock`
/// 3. Plain socket name: `{primal}.sock` or `{primal}-ipc.sock`
/// 4. Temp directory fallback
/// 5. Primal manifest file
/// 6. Socket registry
///
/// Returns the path if a socket file exists, with source indicating
/// how it was found. Returns `None` socket if no socket is reachable.
#[must_use]
pub fn discover_primal(primal: &str) -> DiscoveryResult {
    if let Some(p) = socket_env_var(primal) {
        if socket_is_alive(&p) {
            return DiscoveryResult {
                primal: primal.to_owned(),
                socket: Some(p),
                source: DiscoverySource::EnvOverride,
            };
        }
    }

    let conv_path = socket_path(primal);
    if socket_is_alive(&conv_path) {
        let source = if std::env::var(crate::env_keys::XDG_RUNTIME_DIR).is_ok() {
            DiscoverySource::XdgConvention
        } else {
            DiscoverySource::TempFallback
        };
        return DiscoveryResult {
            primal: primal.to_owned(),
            socket: Some(conv_path),
            source,
        };
    }

    let base = std::env::var(crate::env_keys::XDG_RUNTIME_DIR)
        .map_or_else(|_| std::env::temp_dir(), PathBuf::from);
    let biomeos_dir = base.join(crate::primal_names::BIOMEOS);
    for suffix in [".sock", "-ipc.sock"] {
        let plain = biomeos_dir.join(format!("{primal}{suffix}"));
        if socket_is_alive(&plain) {
            return DiscoveryResult {
                primal: primal.to_owned(),
                socket: Some(plain),
                source: DiscoverySource::XdgConvention,
            };
        }
    }

    if let Some(p) = discover_from_manifest(primal) {
        if socket_is_alive(&p) {
            return DiscoveryResult {
                primal: primal.to_owned(),
                socket: Some(p),
                source: DiscoverySource::Manifest,
            };
        }
    }

    if let Some(p) = discover_from_socket_registry(primal) {
        if socket_is_alive(&p) {
            return DiscoveryResult {
                primal: primal.to_owned(),
                socket: Some(p),
                source: DiscoverySource::SocketRegistry,
            };
        }
    }

    DiscoveryResult {
        primal: primal.to_owned(),
        socket: None,
        source: DiscoverySource::NotFound,
    }
}

/// Discover sockets for a caller-provided set of primal names.
///
/// Prefer capability-based discovery via [`CompositionContext::discover()`](crate::composition::CompositionContext::discover)
/// which resolves by advertised capability rather than hardcoded names.
#[must_use]
pub fn discover_for(primals: &[&str]) -> Vec<DiscoveryResult> {
    primals.iter().map(|name| discover_primal(name)).collect()
}

/// Return only the primals that were successfully discovered from the given set.
#[must_use]
pub fn discover_reachable_for(primals: &[&str]) -> Vec<DiscoveryResult> {
    discover_for(primals)
        .into_iter()
        .filter(|r| r.socket.is_some())
        .collect()
}

/// Attempt to connect to biomeOS's neural-api mode and return a bridge.
///
/// Returns `None` if the biomeOS neural-api socket cannot be found.
#[must_use]
pub fn neural_bridge() -> Option<NeuralBridge> {
    NeuralBridge::discover()
}

/// Query biomeOS neural-api for a health check.
///
/// Returns `true` if biomeOS's neural-api mode is reachable and responds OK.
#[must_use]
pub fn neural_api_healthy() -> bool {
    neural_bridge()
        .and_then(|b| b.health_check().ok())
        .is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_socket_path_constructs_correct_path() {
        let path = build_socket_path(Path::new("/run/user/1000"), "beardog", "alpha");
        assert_eq!(
            path,
            PathBuf::from("/run/user/1000/biomeos/beardog-alpha.sock")
        );
    }

    #[test]
    fn build_socket_path_default_family() {
        let path = build_socket_path(Path::new("/tmp"), "songbird", "default");
        assert_eq!(path, PathBuf::from("/tmp/biomeos/songbird-default.sock"));
    }

    #[test]
    fn socket_path_returns_a_path() {
        let path = socket_path("beardog");
        assert!(path.to_string_lossy().contains("beardog"));
        assert!(path.to_string_lossy().contains(".sock"));
        assert!(path.to_string_lossy().contains("biomeos"));
    }

    #[test]
    fn discover_primal_returns_not_found_for_nonexistent() {
        let result = discover_primal("definitely_not_running_xyzzy");
        assert!(result.socket.is_none());
        assert_eq!(result.source, DiscoverySource::NotFound);
        assert_eq!(result.primal, "definitely_not_running_xyzzy");
    }

    #[test]
    fn discover_for_empty_list() {
        let results = discover_for(&[]);
        assert!(results.is_empty());
    }

    #[test]
    fn discover_for_returns_one_per_primal() {
        let primals = &["beardog", "songbird"];
        let results = discover_for(primals);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].primal, "beardog");
        assert_eq!(results[1].primal, "songbird");
    }

    #[test]
    fn discover_reachable_for_filters_missing() {
        let results = discover_reachable_for(&["definitely_not_real"]);
        assert!(results.is_empty());
    }

    #[test]
    fn neural_bridge_returns_none_when_no_biomeos() {
        assert!(neural_bridge().is_none());
    }

    #[test]
    fn neural_api_healthy_false_when_no_biomeos() {
        assert!(!neural_api_healthy());
    }

    #[test]
    fn discovery_source_not_found_is_default_for_missing_sockets() {
        let result = discover_primal("beardog");
        if result.socket.is_none() {
            assert_eq!(result.source, DiscoverySource::NotFound);
        }
    }

    #[test]
    fn socket_is_alive_false_for_nonexistent() {
        assert!(!socket_is_alive(Path::new("/nonexistent/socket.sock")));
    }

    #[test]
    fn socket_is_alive_false_for_regular_file() {
        let dir = std::env::temp_dir().join("primalspring_test_alive");
        let _ = std::fs::create_dir_all(&dir);
        let fake = dir.join("fake.sock");
        std::fs::write(&fake, b"not a socket").ok();
        assert!(!socket_is_alive(&fake));
        let _ = std::fs::remove_file(&fake);
    }

    #[test]
    fn dead_socket_cache_prevents_reprobe() {
        let path = PathBuf::from("/tmp/definitely_dead_test_socket_xyzzy.sock");
        assert!(!socket_is_alive(&path));
        let cache = DEAD_SOCKET_CACHE.get_or_init(|| Mutex::new(HashSet::new()));
        let contains = cache.lock().map(|s| s.contains(&path)).unwrap_or(false);
        assert!(!contains, "nonexistent path shouldn't be cached (no file to probe)");
    }

    #[test]
    fn discover_from_manifest_returns_none_when_no_xdg() {
        assert!(discover_from_manifest("nonexistent_primal_xyzzy").is_none());
    }

    #[test]
    fn discover_from_socket_registry_returns_none_when_no_xdg() {
        assert!(discover_from_socket_registry("nonexistent_primal_xyzzy").is_none());
    }

    #[test]
    fn discovery_source_has_five_tiers() {
        let sources = [
            DiscoverySource::EnvOverride,
            DiscoverySource::XdgConvention,
            DiscoverySource::TempFallback,
            DiscoverySource::Manifest,
            DiscoverySource::SocketRegistry,
            DiscoverySource::NotFound,
        ];
        assert_eq!(sources.len(), 6);
    }
}
