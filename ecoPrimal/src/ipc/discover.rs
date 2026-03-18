// SPDX-License-Identifier: AGPL-3.0-or-later

//! Runtime primal discovery via environment, filesystem probing, and Neural API.
//!
//! Discovery order (per primal):
//!   1. `{PRIMAL}_SOCKET` environment override (explicit path)
//!   2. `$XDG_RUNTIME_DIR/biomeos/{primal}-{family}.sock` (convention)
//!   3. `{temp_dir}/biomeos/{primal}-{family}.sock` (fallback)
//!
//! Ecosystem-wide sweep discovery uses the Neural API to learn what primals
//! are registered at runtime. primalSpring never hardcodes a primal roster.

use std::path::{Path, PathBuf};

use neural_api_client_sync::NeuralBridge;

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
    /// Socket not found by any method.
    NotFound,
}

/// Build the conventional socket path from explicit base and family.
///
/// Pure function for testability — no env reads.
#[must_use]
pub fn build_socket_path(base_dir: &Path, primal: &str, family: &str) -> PathBuf {
    base_dir
        .join("biomeos")
        .join(format!("{primal}-{family}.sock"))
}

/// Compute the conventional socket path for a primal.
///
/// Uses `$XDG_RUNTIME_DIR` if set, otherwise `std::env::temp_dir()`.
#[must_use]
pub fn socket_path(primal: &str) -> PathBuf {
    let base =
        std::env::var("XDG_RUNTIME_DIR").map_or_else(|_| std::env::temp_dir(), PathBuf::from);
    let family = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());
    build_socket_path(&base, primal, &family)
}

/// Check for an explicit socket path via `{PRIMAL}_SOCKET` env var.
#[must_use]
pub fn socket_env_var(primal: &str) -> Option<PathBuf> {
    let key = format!("{}_SOCKET", primal.to_uppercase());
    std::env::var(key).ok().map(PathBuf::from)
}

/// Discover a primal's socket at runtime.
///
/// Returns the path if a socket file exists, with source indicating
/// how it was found. Returns `None` socket if no socket is reachable.
#[must_use]
pub fn discover_primal(primal: &str) -> DiscoveryResult {
    if let Some(p) = socket_env_var(primal) {
        if p.exists() {
            return DiscoveryResult {
                primal: primal.to_owned(),
                socket: Some(p),
                source: DiscoverySource::EnvOverride,
            };
        }
    }

    let conv_path = socket_path(primal);
    if conv_path.exists() {
        let source = if std::env::var("XDG_RUNTIME_DIR").is_ok() {
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

    DiscoveryResult {
        primal: primal.to_owned(),
        socket: None,
        source: DiscoverySource::NotFound,
    }
}

/// Discover sockets for a caller-provided set of primal names.
///
/// Use this when you already know which primals a composition requires
/// (e.g. from [`crate::coordination::AtomicType::required_primals()`]).
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

/// Attempt to connect to the Neural API and return a bridge.
///
/// Returns `None` if the Neural API socket cannot be found (biomeOS not running).
#[must_use]
pub fn neural_bridge() -> Option<NeuralBridge> {
    NeuralBridge::discover()
}

/// Query the Neural API for a health check.
///
/// Returns `true` if the Neural API is reachable and responds OK.
#[must_use]
pub fn neural_api_healthy() -> bool {
    neural_bridge()
        .and_then(|b| b.health_check().ok())
        .is_some()
}

/// Query the Neural API for available capabilities.
///
/// Returns a JSON value describing what capabilities are registered, or
/// `None` if the Neural API is unreachable.
#[must_use]
pub fn discover_capabilities(capability: &str) -> Option<serde_json::Value> {
    neural_bridge().and_then(|b| b.discover_capability(capability).ok())
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
        // Neural API is not running in test environment
        assert!(neural_bridge().is_none());
    }

    #[test]
    fn neural_api_healthy_false_when_no_biomeos() {
        assert!(!neural_api_healthy());
    }

    #[test]
    fn discover_capabilities_none_when_no_biomeos() {
        assert!(discover_capabilities("crypto").is_none());
    }

    #[test]
    fn discovery_source_not_found_is_default_for_missing_sockets() {
        let result = discover_primal("beardog");
        if result.socket.is_none() {
            assert_eq!(result.source, DiscoverySource::NotFound);
        }
    }
}
