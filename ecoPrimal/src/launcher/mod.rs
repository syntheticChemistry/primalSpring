// SPDX-License-Identifier: AGPL-3.0-or-later

//! Primal process launching and lifecycle management.
//!
//! Synchronous port of biomeOS `primal_spawner` / `nucleation` modules,
//! adapted for primalSpring's coordination validation domain.
//!
//! # Binary Discovery
//!
//! [`discover_binary`] resolves a primal binary using the same 5-tier
//! search order as biomeOS:
//!
//! 1. `$ECOPRIMALS_PLASMID_BIN`
//! 2. `$BIOMEOS_PLASMID_BIN_DIR`
//! 3. `./plasmidBin`
//! 4. `../plasmidBin`
//! 5. `../../plasmidBin`
//!
//! Within each base directory, 6 binary-name patterns are tried.
//!
//! # Socket Nucleation
//!
//! [`SocketNucleation`] assigns deterministic socket paths so that
//! primals and their dependents agree on socket locations before any
//! process starts.
//!
//! # Launch Profiles
//!
//! [`LaunchProfile`] is loaded from `config/primal_launch_profiles.toml`
//! (compile-time `include_str!`). Profiles describe per-primal CLI flags,
//! environment variables, and cross-primal socket wiring.

use std::path::PathBuf;
use std::time::Duration;

mod biomeos;
mod discovery;
mod profiles;
mod spawn;

pub use biomeos::spawn_biomeos;
pub use discovery::{discover_binary, discover_biomeos_binary};
pub use profiles::{LaunchProfile, load_launch_profiles};
pub use spawn::{PrimalProcess, SocketNucleation, spawn_primal, wait_for_socket};

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Typed errors for primal launch operations.
#[derive(Debug, thiserror::Error)]
pub enum LaunchError {
    /// Binary not found after searching all tiers and patterns.
    #[error("binary not found for '{primal}'; searched: {searched:?}")]
    BinaryNotFound {
        /// The primal name that was searched for.
        primal: String,
        /// Candidate paths that were checked.
        searched: Vec<PathBuf>,
    },
    /// `std::process::Command::spawn` failed.
    #[error("spawn failed for '{primal}': {source}")]
    SpawnFailed {
        /// The primal whose binary failed to spawn.
        primal: String,
        /// The underlying I/O error.
        source: std::io::Error,
    },
    /// Socket did not appear within the timeout.
    #[error("socket timeout for '{primal}' at {socket} after {waited:.1?}")]
    SocketTimeout {
        /// The primal whose socket was expected.
        primal: String,
        /// The socket path that was waited on.
        socket: PathBuf,
        /// How long we waited before giving up.
        waited: Duration,
    },
    /// A spawned primal failed its post-launch health check.
    #[error("health check failed for '{primal}': {detail}")]
    HealthCheckFailed {
        /// The primal that failed the check.
        primal: String,
        /// Detail from the failed health call.
        detail: String,
    },
    /// Launch profiles TOML failed to parse.
    #[error("launch profile parse error: {0}")]
    ProfileParseError(
        /// Parse error detail.
        String,
    ),
}

#[cfg(test)]
mod tests;
