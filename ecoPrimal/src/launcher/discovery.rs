// SPDX-License-Identifier: AGPL-3.0-or-later

//! Binary discovery and resolution for primal executables.
//!
//! Binaries are resolved via the standard consumer pattern — the same
//! one used by downstream springs, compositions, and deployments:
//!
//! 1. `$ECOPRIMALS_PLASMID_BIN` (explicit override)
//! 2. `$BIOMEOS_PLASMID_BIN_DIR` (biomeOS override)
//! 3. `$XDG_DATA_HOME/ecoPrimals/plasmidBin` (XDG standard cache,
//!    populated by `tools/fetch_primals.sh`)
//!
//! No relative filesystem traversal into sibling repos or `../../infra/`.

use std::path::PathBuf;

use super::LaunchError;

/// Env var: override base directory for primal binaries.
pub const ENV_PLASMID_BIN: &str = "ECOPRIMALS_PLASMID_BIN";

/// Env var: biomeOS plasmid bin directory.
pub const ENV_BIOMEOS_BIN_DIR: &str = "BIOMEOS_PLASMID_BIN_DIR";

/// XDG-compliant default location for fetched primal binaries.
fn xdg_plasmid_bin() -> PathBuf {
    std::env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|_| std::env::var("HOME").map(|h| PathBuf::from(h).join(".local/share")))
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
        .join("ecoPrimals/plasmidBin")
}

/// Detect the Rust-style target triple for the current host.
fn host_target_triple() -> String {
    let arch = std::env::consts::ARCH;
    let os = std::env::consts::OS;
    match os {
        "linux" => format!("{arch}-unknown-linux-musl"),
        "macos" => format!("{arch}-apple-darwin"),
        _ => format!("{arch}-unknown-{os}"),
    }
}

/// Search for a primal binary using the 3-tier directory search.
///
/// Within each base directory, patterns are tried in order:
/// 1. `primals/{target-triple}/{primal}` (fetch.sh canonical layout)
/// 2. `primals/{primal}` (flat layout)
/// 3. `{primal}` (bare binary in base dir)
///
/// # Errors
///
/// Returns [`LaunchError::BinaryNotFound`] if no matching executable is
/// found after exhausting all directories and patterns.
pub fn discover_binary(primal: &str) -> Result<PathBuf, LaunchError> {
    let base_dirs: Vec<PathBuf> = [
        std::env::var(ENV_PLASMID_BIN).ok().map(PathBuf::from),
        std::env::var(ENV_BIOMEOS_BIN_DIR).ok().map(PathBuf::from),
        Some(xdg_plasmid_bin()),
    ]
    .into_iter()
    .flatten()
    .collect();

    let triple = host_target_triple();

    let patterns = [
        format!("primals/{triple}/{primal}"),
        format!("primals/{primal}"),
        primal.to_string(),
    ];

    let mut searched = Vec::new();

    for base in &base_dirs {
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

/// Discover the biomeOS binary in `plasmidBin/primals/` or `$PATH`.
///
/// biomeOS is the substrate primal — the ecosystem's composition,
/// coordination, and deployment orchestrator. The Neural API is one of
/// its `UniBin` modes (`biomeos api`).
///
/// # Errors
///
/// Returns [`LaunchError::BinaryNotFound`] if `biomeos` is not found.
pub fn discover_biomeos_binary() -> Result<PathBuf, LaunchError> {
    discover_binary(crate::primal_names::BIOMEOS)
}
