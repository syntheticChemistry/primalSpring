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

/// Resolve the system hostname without libc.
///
/// Reads `/etc/hostname` (Linux) or falls back to `HOSTNAME`/`HOST` env vars.
/// Returns `None` only if no hostname source is available.
#[must_use]
pub fn hostname() -> Option<String> {
    std::fs::read_to_string("/etc/hostname")
        .ok()
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty())
        .or_else(|| std::env::var("HOSTNAME").ok())
        .or_else(|| std::env::var("HOST").ok())
}

/// Current Unix epoch seconds (safe fallback to 0 on pre-epoch systems).
#[must_use]
pub fn epoch_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Current Unix epoch milliseconds.
#[must_use]
pub fn epoch_millis() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

/// Convert Unix epoch seconds to ISO 8601 UTC string.
///
/// Pure arithmetic — no allocator dependencies, no libc. Uses the
/// Euclidean affine civil date algorithm.
#[must_use]
pub fn unix_secs_to_iso(total_secs: u64) -> String {
    const SECS_PER_DAY: u64 = 86_400;
    let days = total_secs / SECS_PER_DAY;
    let day_secs = total_secs % SECS_PER_DAY;
    let hours = day_secs / 3600;
    let mins = (day_secs % 3600) / 60;
    let secs = day_secs % 60;

    let z = days + 719_468;
    let era = z / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };

    format!("{y:04}-{m:02}-{d:02}T{hours:02}:{mins:02}:{secs:02}Z")
}

/// Current time as ISO 8601 UTC string.
#[must_use]
pub fn iso_now() -> String {
    unix_secs_to_iso(epoch_secs())
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
