// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Platform resolution helpers: runtime directories, depot paths, target triples.
//!
//! These functions resolve filesystem paths using environment variables with
//! fallback chains, avoiding hardcoded assumptions about the host layout.

/// Default host for TCP fallback discovery (Tier 5).
///
/// Used when `PRIMALSPRING_HOST` is not set. Loopback-only by default;
/// containers and remote compositions override via env var.
pub const DEFAULT_HOST: &str = "127.0.0.1";

/// LAN bind address for multi-gate services (Songbird federation, etc.).
pub const LAN_BIND_ADDRESS: &str = "0.0.0.0";

/// Fallback runtime directory when `XDG_RUNTIME_DIR` is not set.
pub const RUNTIME_DIR_FALLBACK: &str = "/tmp";

#[must_use]
/// Resolve `XDG_RUNTIME_DIR` with correct fallback chain.
///
/// Prefers the env var; falls back to `/run/user/{uid}` (Linux convention),
/// then to [`RUNTIME_DIR_FALLBACK`] (`/tmp`). Avoids hardcoded UID `1000`.
pub fn runtime_dir() -> String {
    std::env::var(crate::env_keys::XDG_RUNTIME_DIR).unwrap_or_else(|_| {
        #[cfg(target_os = "linux")]
        {
            if let Some(uid) = real_uid() {
                let candidate = format!("/run/user/{uid}");
                if std::path::Path::new(&candidate).is_dir() {
                    return candidate;
                }
            }
        }
        RUNTIME_DIR_FALLBACK.to_owned()
    })
}

#[must_use]
/// Resolve the biomeOS socket directory.
pub fn biomeos_socket_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(runtime_dir()).join(crate::env_keys::BIOMEOS_SUBDIR)
}

/// Resolve the XDG data home directory.
///
/// Checks `XDG_DATA_HOME` then falls back to `$HOME/.local/share`.
#[must_use]
pub fn xdg_data_home() -> String {
    std::env::var(crate::env_keys::XDG_DATA_HOME).unwrap_or_else(|_| {
        format!(
            "{}/.local/share",
            std::env::var(crate::env_keys::HOME).unwrap_or_default()
        )
    })
}

/// Resolve the plasmidBin depot root directory.
///
/// Checks `ECOPRIMALS_PLASMID_BIN`, then `$ECOPRIMALS_ROOT/infra/plasmidBin`,
/// then `$XDG_DATA_HOME/ecoPrimals/plasmidBin`.
#[must_use]
pub fn plasmidbin_depot_root() -> String {
    std::env::var(crate::env_keys::ECOPRIMALS_PLASMID_BIN)
        .or_else(|_| {
            std::env::var(crate::env_keys::ECOPRIMALS_ROOT).map(|r| format!("{r}/infra/plasmidBin"))
        })
        .unwrap_or_else(|_| {
            format!(
                "{}/{}/plasmidBin",
                xdg_data_home(),
                crate::env_keys::ECOPRIMALS_DIR_NAME
            )
        })
}

/// Detect the Rust-style target triple for the current host.
#[must_use]
pub fn current_target_triple() -> String {
    let arch = std::env::consts::ARCH;
    let os = std::env::consts::OS;
    match os {
        "linux" => format!("{arch}-unknown-linux-musl"),
        "macos" => format!("{arch}-apple-darwin"),
        _ => format!("{arch}-unknown-{os}"),
    }
}

/// Read the real UID from `/proc/self/status` (no libc, no unsafe).
#[cfg(target_os = "linux")]
fn real_uid() -> Option<u32> {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("Uid:"))
                .and_then(|l| l.split_whitespace().nth(1))
                .and_then(|u| u.parse::<u32>().ok())
        })
}
