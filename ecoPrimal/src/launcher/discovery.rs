// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Binary discovery and resolution for primal executables.
//!
//! Binaries are resolved via a 4-tier directory search:
//!
//! 1. `$ECOPRIMALS_PLASMID_BIN` (explicit override — post-primordial production)
//! 2. `$BIOMEOS_PLASMID_BIN_DIR` (biomeOS override)
//! 3. `$ECOPRIMALS_ROOT/infra/plasmidBin` (**pre-primordial debt** — local workspace
//!    copy, violates depot-from-VPS principle. Retained for backward compat only.)
//! 4. `$XDG_DATA_HOME/ecoPrimals/plasmidBin` (XDG standard cache,
//!    populated by `plasmidbin sync` or `membrane plasmid.fetch`)
//!
//! # Post-Primordial (Wave 114+)
//!
//! Production gates fetch binaries from cellMembrane's VPS depot via
//! `plasmidbin sync` into the XDG cache (tier 4) or an explicit
//! `ECOPRIMALS_PLASMID_BIN` path (tier 1). Tier 3 (workspace-local) is
//! pre-primordial scaffolding — it works but the bootstrap-readiness
//! scenario will flag it as a deployment hygiene failure.

use std::path::PathBuf;

use super::LaunchError;

/// XDG-compliant default location for fetched primal binaries.
fn xdg_plasmid_bin() -> PathBuf {
    PathBuf::from(crate::tolerances::xdg_data_home())
        .join(crate::env_keys::ECOPRIMALS_DIR_NAME)
        .join("plasmidBin")
}

/// Search for a primal binary using the 4-tier directory search.
///
/// Within each base directory, patterns are tried in order:
/// 1. `primals/{target-triple}/{primal}` (plasmidbin fetch canonical layout)
/// 2. `primals/{primal}` (flat layout)
/// 3. `{primal}` (bare binary in base dir)
///
/// # Errors
///
/// Returns [`LaunchError::BinaryNotFound`] if no matching executable is
/// found after exhausting all directories and patterns.
pub fn discover_binary(primal: &str) -> Result<PathBuf, LaunchError> {
    let base_dirs: Vec<PathBuf> = [
        std::env::var(crate::env_keys::ECOPRIMALS_PLASMID_BIN)
            .ok()
            .map(PathBuf::from),
        std::env::var(crate::env_keys::BIOMEOS_PLASMID_BIN_DIR)
            .ok()
            .map(PathBuf::from),
        std::env::var(crate::env_keys::ECOPRIMALS_ROOT)
            .ok()
            .map(|r| PathBuf::from(r).join("infra/plasmidBin")),
        Some(xdg_plasmid_bin()),
    ]
    .into_iter()
    .flatten()
    .collect();

    let triple = crate::tolerances::current_target_triple();

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
