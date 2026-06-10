// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Server-side transport binding with graceful UDS → TCP fallback.
//!
//! Provides [`bind_transport`] — the canonical pattern for primals that
//! need to run on platforms where Unix domain sockets are unavailable
//! or restricted (e.g. Android SELinux denies UDS bind with `EACCES`).
//!
//! # Fallback chain
//!
//! ```text
//! 1. Try UDS bind at resolved socket path
//! 2. If EACCES/EPERM → fall back to TCP on localhost:PORT
//! 3. If TCP also fails → return error
//! ```
//!
//! The fallback is transparent to JSON-RPC callers — they connect via
//! whichever transport succeeded. Discovery-side support already exists:
//! `CompositionContext` tiers 2-5 probe both UDS and TCP.
//!
//! # Environment control
//!
//! - `PRIMAL_BIND_MODE=tcp_only` — skip UDS entirely, bind TCP only
//! - `PRIMAL_BIND_MODE=uds_only` — UDS only, fail on EACCES (default)
//! - `PRIMAL_BIND_MODE=fallback` — try UDS, fall back to TCP on permission error
//!
//! # Adoption
//!
//! Replace:
//! ```ignore
//! let listener = UnixListener::bind(&path)?;
//! // fatal on EACCES
//! ```
//!
//! With:
//! ```ignore
//! use primalspring::ipc::server_bind::{bind_transport, BindMode};
//! let bound = bind_transport("myprimal", BindMode::from_env())?;
//! match bound {
//!     BoundTransport::Unix(listener, path) => { /* UDS loop */ }
//!     BoundTransport::Tcp(listener, addr) => { /* TCP loop */ }
//! }
//! ```

use std::net::{Ipv4Addr, SocketAddr, TcpListener};
use std::os::unix::net::UnixListener;
use std::path::PathBuf;

/// Transport binding mode — controls UDS vs TCP selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindMode {
    /// UDS only — fatal on permission error (default, Dark Forest compliant).
    UdsOnly,
    /// TCP only — skip UDS entirely (Android, containers, `--no-uds`).
    TcpOnly,
    /// Try UDS first, fall back to TCP on `EACCES`/`EPERM`.
    Fallback,
}

impl BindMode {
    /// Resolve bind mode from `PRIMAL_BIND_MODE` environment variable.
    ///
    /// Values: `uds_only` (default), `tcp_only`, `fallback`.
    #[must_use]
    pub fn from_env() -> Self {
        match std::env::var("PRIMAL_BIND_MODE")
            .unwrap_or_default()
            .to_lowercase()
            .as_str()
        {
            "tcp_only" | "tcp" => Self::TcpOnly,
            "fallback" | "auto" => Self::Fallback,
            _ => Self::UdsOnly,
        }
    }
}

/// Result of a successful transport bind.
#[derive(Debug)]
pub enum BoundTransport {
    /// Bound to a Unix domain socket at the given path.
    Unix(UnixListener, PathBuf),
    /// Bound to a TCP socket at the given address.
    Tcp(TcpListener, SocketAddr),
}

impl BoundTransport {
    /// Human-readable description of the bound endpoint.
    #[must_use]
    pub fn endpoint_display(&self) -> String {
        match self {
            Self::Unix(_, path) => format!("unix:{}", path.display()),
            Self::Tcp(_, addr) => format!("tcp:{addr}"),
        }
    }

    /// Whether this is a TCP binding (useful for logging transport mode).
    #[must_use]
    pub const fn is_tcp(&self) -> bool {
        matches!(self, Self::Tcp(..))
    }
}

/// Error during transport binding.
#[derive(Debug, thiserror::Error)]
pub enum BindError {
    /// UDS bind failed with a permission error (EACCES/EPERM).
    #[error("UDS bind permission denied at {path}: {source}")]
    UdsPermissionDenied {
        /// Socket path that was attempted.
        path: PathBuf,
        /// Underlying OS error.
        source: std::io::Error,
    },
    /// UDS bind failed for a non-permission reason.
    #[error("UDS bind failed at {path}: {source}")]
    UdsFailed {
        /// Socket path that was attempted.
        path: PathBuf,
        /// Underlying OS error.
        source: std::io::Error,
    },
    /// TCP bind failed.
    #[error("TCP bind failed on {addr}: {source}")]
    TcpFailed {
        /// Address that was attempted.
        addr: SocketAddr,
        /// Underlying OS error.
        source: std::io::Error,
    },
    /// No port configured for TCP fallback.
    #[error("no TCP fallback port configured for primal '{primal}'")]
    NoPortConfigured {
        /// Primal slug.
        primal: String,
    },
}

/// Returns `true` if the I/O error is a permission denial (EACCES or EPERM).
fn is_permission_error(e: &std::io::Error) -> bool {
    matches!(e.kind(), std::io::ErrorKind::PermissionDenied)
}

/// Resolve the TCP fallback port for a primal.
///
/// Priority: `{PRIMAL}_PORT` env var → `ports.toml` registry → `None`.
fn resolve_tcp_port(primal_slug: &str) -> Option<u16> {
    let env_key = crate::env_keys::port_env_key(primal_slug);
    if let Ok(val) = std::env::var(env_key) {
        if let Ok(port) = val.parse::<u16>() {
            return Some(port);
        }
    }
    let port = crate::tolerances::default_port_for(primal_slug);
    if port > 0 { Some(port) } else { None }
}

/// Bind a server transport with graceful UDS → TCP fallback.
///
/// This is the canonical server-side bind pattern for the ecoPrimals
/// ecosystem. Primals adopting this function get automatic SELinux/Android
/// adaptation without per-primal code changes.
///
/// # Arguments
///
/// * `primal_slug` — lowercase primal name (e.g. `"coralreef"`)
/// * `mode` — bind mode ([`BindMode::from_env`] for env-driven selection)
///
/// # Errors
///
/// Returns [`BindError`] if all attempted transports fail.
pub fn bind_transport(primal_slug: &str, mode: BindMode) -> Result<BoundTransport, BindError> {
    match mode {
        BindMode::UdsOnly => bind_uds(primal_slug),
        BindMode::TcpOnly => bind_tcp(primal_slug),
        BindMode::Fallback => bind_with_fallback(primal_slug),
    }
}

fn bind_uds(primal_slug: &str) -> Result<BoundTransport, BindError> {
    let sock_path = crate::ipc::discover::socket_path(primal_slug);

    if let Some(parent) = sock_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let _ = std::fs::remove_file(&sock_path);

    match UnixListener::bind(&sock_path) {
        Ok(listener) => Ok(BoundTransport::Unix(listener, sock_path)),
        Err(e) if is_permission_error(&e) => Err(BindError::UdsPermissionDenied {
            path: sock_path,
            source: e,
        }),
        Err(e) => Err(BindError::UdsFailed {
            path: sock_path,
            source: e,
        }),
    }
}

fn bind_tcp(primal_slug: &str) -> Result<BoundTransport, BindError> {
    let port = resolve_tcp_port(primal_slug).ok_or_else(|| BindError::NoPortConfigured {
        primal: primal_slug.to_owned(),
    })?;

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), port);
    match TcpListener::bind(addr) {
        Ok(listener) => Ok(BoundTransport::Tcp(listener, addr)),
        Err(e) => Err(BindError::TcpFailed { addr, source: e }),
    }
}

fn bind_with_fallback(primal_slug: &str) -> Result<BoundTransport, BindError> {
    match bind_uds(primal_slug) {
        Ok(bound) => Ok(bound),
        Err(BindError::UdsPermissionDenied { path, source }) => {
            tracing::warn!(
                primal = primal_slug,
                path = %path.display(),
                error = %source,
                "UDS bind denied (SELinux/permissions) — falling back to TCP"
            );
            bind_tcp(primal_slug)
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bind_mode_parse_values() {
        assert_eq!(BindMode::UdsOnly, BindMode::UdsOnly);
        assert_eq!(BindMode::TcpOnly, BindMode::TcpOnly);
        assert_eq!(BindMode::Fallback, BindMode::Fallback);
    }

    #[test]
    fn resolve_tcp_port_from_registry() {
        let port = resolve_tcp_port("beardog");
        assert!(port.is_some(), "beardog should have a port in ports.toml");
        assert_eq!(port.unwrap(), 9100);
    }

    #[test]
    fn resolve_tcp_port_unknown_primal() {
        let port = resolve_tcp_port("nonexistent_primal_zzz");
        assert_eq!(port, None);
    }

    #[test]
    fn resolve_tcp_port_known_primals() {
        assert_eq!(resolve_tcp_port("coralreef"), Some(9730));
        assert_eq!(resolve_tcp_port("nestgate"), Some(9500));
        assert_eq!(resolve_tcp_port("biomeos"), Some(9800));
        assert_eq!(resolve_tcp_port("petaltongue"), Some(9900));
    }

    #[test]
    fn bind_transport_uds_succeeds() {
        let result = bind_transport("primalspring_test_uds", BindMode::UdsOnly);
        if let Ok(BoundTransport::Unix(_, path)) = &result {
            let _ = std::fs::remove_file(path);
            assert!(path.to_string_lossy().contains("primalspring_test_uds"));
        }
    }

    #[test]
    fn bind_transport_tcp_no_port_errors() {
        let result = bind_transport("nonexistent_primal_zzz", BindMode::TcpOnly);
        assert!(
            matches!(result, Err(BindError::NoPortConfigured { .. })),
            "should error with no port configured: {result:?}"
        );
    }

    #[test]
    fn bound_transport_display() {
        let path = PathBuf::from("/tmp/primalspring_bind_test.sock");
        let _ = std::fs::remove_file(&path);
        let display = BoundTransport::Unix(
            UnixListener::bind(&path).unwrap(),
            path.clone(),
        )
        .endpoint_display();
        let _ = std::fs::remove_file(&path);
        assert!(display.starts_with("unix:"));
    }

    #[test]
    fn bound_transport_is_tcp() {
        let path = PathBuf::from("/tmp/primalspring_tcp_test.sock");
        let _ = std::fs::remove_file(&path);
        let uds = BoundTransport::Unix(
            UnixListener::bind(&path).unwrap(),
            path.clone(),
        );
        assert!(!uds.is_tcp());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn permission_error_detection() {
        let e = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "test EACCES");
        assert!(is_permission_error(&e));

        let e = std::io::Error::new(std::io::ErrorKind::NotFound, "test ENOENT");
        assert!(!is_permission_error(&e));

        let e = std::io::Error::new(std::io::ErrorKind::AddrInUse, "test EADDRINUSE");
        assert!(!is_permission_error(&e));
    }
}
