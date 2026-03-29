// SPDX-License-Identifier: AGPL-3.0-or-later

//! Binary discovery and resolution for primal executables.

use std::path::PathBuf;

use super::LaunchError;

/// Env var: override base directory for primal binaries.
pub const ENV_PLASMID_BIN: &str = "ECOPRIMALS_PLASMID_BIN";

/// Env var: biomeOS plasmid bin directory.
pub const ENV_BIOMEOS_BIN_DIR: &str = "BIOMEOS_PLASMID_BIN_DIR";

/// Relative fallback paths for plasmidBin (tiers 3-5 of binary discovery).
pub const RELATIVE_PLASMID_TIERS: &[&str] = &["./plasmidBin", "../plasmidBin", "../../plasmidBin"];

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
