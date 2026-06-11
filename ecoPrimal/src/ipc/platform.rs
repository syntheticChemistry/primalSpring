// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Platform capability detection for transport selection.
//!
//! Replaces error-based transport negotiation with a positive capability model.
//! Primals call [`PlatformCapabilities::detect()`] at startup to determine
//! which transports are available, then select their bind mode accordingly.
//!
//! # guideStone properties
//!
//! - **P4 (Environment-Agnostic)**: auto-senses transport without per-gate
//!   code or operator knowledge.
//! - **P1 (Deterministic)**: same environment conditions = same capability
//!   result = same transport selection.
//!
//! # Usage
//!
//! ```ignore
//! use primalspring::ipc::platform::PlatformCapabilities;
//!
//! let caps = PlatformCapabilities::detect();
//! let mode = caps.recommended_bind_mode();
//! let bound = primalspring::ipc::server_bind::bind_transport("myprimal", mode)?;
//! ```

use std::path::PathBuf;

use super::server_bind::BindMode;

/// Runtime transport capabilities detected on the current platform.
///
/// This struct captures what the platform supports — not what the user
/// *requested* via `PRIMAL_BIND_MODE`. The env override still takes
/// precedence at bind time; `PlatformCapabilities` informs the default.
#[derive(Debug, Clone)]
pub struct PlatformCapabilities {
    /// Filesystem-based Unix domain sockets can be created and connected.
    pub uds_available: bool,
    /// TCP loopback sockets can be bound.
    pub tcp_available: bool,
    /// Linux abstract sockets are functional (not blocked by SELinux).
    pub abstract_sockets: bool,
    /// Path to the detected socket directory, if UDS is available.
    pub socket_dir: Option<PathBuf>,
    /// Whether an explicit `PRIMAL_BIND_MODE` override is set.
    pub bind_mode_override: Option<BindMode>,
}

impl PlatformCapabilities {
    /// Probe the current platform to determine transport capabilities.
    ///
    /// Performs lightweight, non-destructive probes:
    /// - UDS: create + remove a temp socket in the resolved socket dir
    /// - TCP: bind + close on an ephemeral port
    /// - Abstract: attempt `UnixListener::bind` on an abstract name (Linux)
    /// - Env: check for `PRIMAL_BIND_MODE` override
    #[must_use]
    pub fn detect() -> Self {
        let bind_mode_override = read_bind_mode_override();
        let (uds_available, socket_dir) = probe_uds();
        let tcp_available = probe_tcp();
        let abstract_sockets = probe_abstract_sockets();

        Self {
            uds_available,
            tcp_available,
            abstract_sockets,
            socket_dir,
            bind_mode_override,
        }
    }

    /// Recommend a [`BindMode`] based on detected capabilities.
    ///
    /// If `PRIMAL_BIND_MODE` is explicitly set, that override wins.
    /// Otherwise: UDS if available, TCP if UDS unavailable, or Fallback
    /// if both are available but UDS might be flaky (abstract-only platform).
    #[must_use]
    pub const fn recommended_bind_mode(&self) -> BindMode {
        if let Some(override_mode) = self.bind_mode_override {
            return override_mode;
        }

        if self.uds_available {
            BindMode::UdsOnly
        } else {
            BindMode::TcpOnly
        }
    }

    /// Whether the platform can serve on at least one transport.
    #[must_use]
    pub const fn any_transport_available(&self) -> bool {
        self.uds_available || self.tcp_available
    }

    /// Log a summary of detected capabilities at startup.
    pub fn log_summary(&self) {
        tracing::info!(
            uds = self.uds_available,
            tcp = self.tcp_available,
            abstract_sockets = self.abstract_sockets,
            socket_dir = ?self.socket_dir,
            bind_mode_override = ?self.bind_mode_override,
            "platform capabilities detected"
        );
    }
}

/// Read `PRIMAL_BIND_MODE` if explicitly set. Returns `None` if unset.
fn read_bind_mode_override() -> Option<BindMode> {
    let val = std::env::var(crate::env_keys::PRIMAL_BIND_MODE).ok()?;
    if val.is_empty() {
        return None;
    }
    Some(match val.to_lowercase().as_str() {
        "tcp_only" | "tcp" => BindMode::TcpOnly,
        "fallback" | "auto" => BindMode::Fallback,
        _ => BindMode::UdsOnly,
    })
}

/// Probe UDS availability by creating and removing a test socket.
fn probe_uds() -> (bool, Option<PathBuf>) {
    let dir = resolve_probe_dir();

    if std::fs::create_dir_all(&dir).is_err() {
        return (false, None);
    }

    let probe_path = dir.join(".caps_probe.sock");
    let _ = std::fs::remove_file(&probe_path);

    let available = std::os::unix::net::UnixListener::bind(&probe_path).is_ok();
    let _ = std::fs::remove_file(&probe_path);
    (available, Some(dir))
}

/// Probe TCP loopback by binding an ephemeral port.
fn probe_tcp() -> bool {
    use std::net::{Ipv4Addr, SocketAddr, TcpListener};
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 0);
    TcpListener::bind(addr).is_ok()
}

/// Probe Linux abstract socket support.
///
/// Abstract sockets are Linux-specific and may be blocked by SELinux
/// (common on Android). Since `std::os::linux::net::SocketAddrExt` is
/// unstable and this crate forbids unsafe code, we check the kernel
/// feature via `/proc/net/unix` — if the file exists and is readable,
/// the kernel supports Unix sockets (abstract included unless SELinux
/// blocks them at bind time). A more precise probe would require
/// `unsafe` libc calls or nightly features.
///
/// Returns `false` on non-Linux.
fn probe_abstract_sockets() -> bool {
    #[cfg(target_os = "linux")]
    {
        // /proc/net/unix only exists on Linux with Unix socket support.
        // SELinux enforcement (Android) blocks bind at runtime, not here,
        // so this is an optimistic probe. For full accuracy, primals use
        // BindMode::Fallback which detects EACCES at bind time.
        std::path::Path::new("/proc/net/unix").exists()
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

/// Resolve the directory to probe for UDS support.
fn resolve_probe_dir() -> PathBuf {
    if let Ok(dir) = std::env::var(crate::env_keys::SOCKET_DIR) {
        return PathBuf::from(dir);
    }
    if let Ok(xdg) = std::env::var(crate::env_keys::XDG_RUNTIME_DIR) {
        return PathBuf::from(xdg).join(crate::env_keys::RUNTIME_SUBDIR);
    }
    std::env::temp_dir().join(crate::env_keys::RUNTIME_SUBDIR)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_returns_valid_capabilities() {
        let caps = PlatformCapabilities::detect();
        assert!(
            caps.any_transport_available(),
            "at least one transport should be available in CI/dev"
        );
    }

    #[test]
    fn uds_probe_succeeds_on_linux() {
        let (available, dir) = probe_uds();
        assert!(available, "UDS should be available on Linux dev");
        assert!(dir.is_some());
    }

    #[test]
    fn tcp_probe_succeeds() {
        assert!(probe_tcp(), "TCP loopback should always work");
    }

    #[test]
    fn recommended_mode_uds_when_available() {
        let caps = PlatformCapabilities {
            uds_available: true,
            tcp_available: true,
            abstract_sockets: false,
            socket_dir: Some(PathBuf::from("/tmp")),
            bind_mode_override: None,
        };
        assert_eq!(caps.recommended_bind_mode(), BindMode::UdsOnly);
    }

    #[test]
    fn recommended_mode_tcp_when_no_uds() {
        let caps = PlatformCapabilities {
            uds_available: false,
            tcp_available: true,
            abstract_sockets: false,
            socket_dir: None,
            bind_mode_override: None,
        };
        assert_eq!(caps.recommended_bind_mode(), BindMode::TcpOnly);
    }

    #[test]
    fn override_wins_over_detection() {
        let caps = PlatformCapabilities {
            uds_available: true,
            tcp_available: true,
            abstract_sockets: false,
            socket_dir: Some(PathBuf::from("/tmp")),
            bind_mode_override: Some(BindMode::TcpOnly),
        };
        assert_eq!(caps.recommended_bind_mode(), BindMode::TcpOnly);
    }

    #[test]
    fn no_transport_still_returns_tcp() {
        let caps = PlatformCapabilities {
            uds_available: false,
            tcp_available: false,
            abstract_sockets: false,
            socket_dir: None,
            bind_mode_override: None,
        };
        assert!(!caps.any_transport_available());
        assert_eq!(caps.recommended_bind_mode(), BindMode::TcpOnly);
    }

    #[test]
    fn recommended_mode_respects_override() {
        let caps = PlatformCapabilities {
            uds_available: true,
            tcp_available: true,
            abstract_sockets: true,
            socket_dir: Some(PathBuf::from("/tmp")),
            bind_mode_override: Some(BindMode::Fallback),
        };
        assert_eq!(caps.recommended_bind_mode(), BindMode::Fallback);
    }
}
