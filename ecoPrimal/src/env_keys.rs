// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Centralized environment variable name constants and seed configuration.
//!
//! Every `std::env::var("...")` call in the codebase should reference a
//! constant from this module instead of a bare string literal. This makes
//! typos impossible and env-var usage greppable from one location.
//!
//! [`SeedConfig`] provides thread-safe in-process storage for identity and
//! seed values, eliminating the need for `unsafe { env::set_var }` in
//! single-binary contexts (guidestone, harness). Library code should use
//! [`resolve_family_id`] and [`resolve_family_seed`] instead of reading
//! env vars directly.

use std::sync::OnceLock;

// ── Identity & genetics ──────────────────────────────────────────────

/// Family group identifier for multi-tenant socket paths and BTSP genetics.
pub const FAMILY_ID: &str = "FAMILY_ID";
/// Hex-encoded family seed used as BTSP key material.
pub const FAMILY_SEED: &str = "FAMILY_SEED";
/// BearDog-specific alias for the family seed.
pub const BEARDOG_FAMILY_SEED: &str = "BEARDOG_FAMILY_SEED";
/// Guidestone-level seed override (highest priority in mito-tier resolution).
pub const GUIDESTONE_SEED: &str = "GUIDESTONE_SEED";

// ── SeedConfig: thread-safe in-process identity storage ─────────────

static SEED_CONFIG: OnceLock<SeedConfig> = OnceLock::new();

/// In-process identity and seed configuration.
///
/// Replaces `unsafe { env::set_var }` for binaries that need to propagate
/// identity to library code within the same process. External processes
/// still receive env vars via `Command::env()` in the launcher.
#[derive(Debug, Clone)]
pub struct SeedConfig {
    /// Family group identifier.
    pub family_id: String,
    /// Hex-encoded family seed for BTSP key material.
    pub hex_seed: String,
}

/// Initialize the global seed config. Call once in `main()` before any
/// library code that resolves family identity.
///
/// # Errors
///
/// Returns the `SeedConfig` back if already initialized.
pub fn init_seed_config(config: SeedConfig) -> Result<(), SeedConfig> {
    SEED_CONFIG.set(config)
}

/// Resolve the family ID: checks [`SeedConfig`] first, then env vars
/// (`FAMILY_ID`, `BIOMEOS_FAMILY_ID`), then falls back to `"default"`.
#[must_use]
pub fn resolve_family_id() -> String {
    if let Some(cfg) = SEED_CONFIG.get() {
        if !cfg.family_id.is_empty() && cfg.family_id != "default" {
            return cfg.family_id.clone();
        }
    }
    std::env::var(FAMILY_ID)
        .or_else(|_| std::env::var(BIOMEOS_FAMILY_ID))
        .unwrap_or_else(|_| "default".to_owned())
}

/// Resolve the family seed hex string: checks [`SeedConfig`] first,
/// then `FAMILY_SEED` env var.
#[must_use]
pub fn resolve_family_seed() -> Option<String> {
    if let Some(cfg) = SEED_CONFIG.get() {
        if !cfg.hex_seed.is_empty() {
            return Some(cfg.hex_seed.clone());
        }
    }
    std::env::var(FAMILY_SEED).ok().filter(|s| !s.is_empty())
}

// ── XDG / OS paths ──────────────────────────────────────────────────

/// XDG runtime directory for sockets and ephemeral state.
pub const XDG_RUNTIME_DIR: &str = "XDG_RUNTIME_DIR";
/// XDG data home for persistent user data.
pub const XDG_DATA_HOME: &str = "XDG_DATA_HOME";
/// User home directory.
pub const HOME: &str = "HOME";
/// System hostname.
pub const HOSTNAME: &str = "HOSTNAME";

// ── Filesystem layout conventions ───────────────────────────────────

/// Runtime subdirectory for ecoPrimals sockets and state.
pub const RUNTIME_SUBDIR: &str = "ecoprimals";
/// Runtime subdirectory for biomeOS sockets.
pub const BIOMEOS_SUBDIR: &str = "biomeos";
/// Subdirectory for primal manifest files.
pub const MANIFESTS_SUBDIR: &str = "manifests";
/// Parent directory name for manifests under XDG.
pub const ECOPRIMALS_DIR_NAME: &str = "ecoPrimals";

// ── Socket / discovery ──────────────────────────────────────────────

/// Explicit override for the socket directory.
pub const SOCKET_DIR: &str = "SOCKET_DIR";
/// Override for the ecoPrimals runtime socket directory.
pub const ECOPRIMALS_SOCKET_DIR: &str = "ECOPRIMALS_SOCKET_DIR";
/// Explicit path to the biomeOS neural-api socket.
pub const NEURAL_API_SOCKET: &str = "NEURAL_API_SOCKET";

// ── Remote / deployment ─────────────────────────────────────────────

/// Hostname of a remote gate for deployment matrix validation.
pub const REMOTE_GATE_HOST: &str = "REMOTE_GATE_HOST";
/// Target cell for deployment matrix selection.
pub const MATRIX_CELL: &str = "MATRIX_CELL";
/// Transport mode for primal connections (`uds`, `tcp`, `http`).
pub const PRIMAL_TRANSPORT: &str = "PRIMAL_TRANSPORT";
/// Target architecture for cross-arch deployment.
pub const DEPLOY_ARCH: &str = "DEPLOY_ARCH";

// ── primalSpring configuration ──────────────────────────────────────

/// Host address for TCP fallback connections.
pub const PRIMALSPRING_HOST: &str = "PRIMALSPRING_HOST";
/// Override directory for deploy graph resolution.
pub const PRIMALSPRING_GRAPHS_DIR: &str = "PRIMALSPRING_GRAPHS_DIR";
/// When set, guidestone emits JSON output instead of human-readable text.
pub const PRIMALSPRING_JSON: &str = "PRIMALSPRING_JSON";
/// Authentication enforcement mode for the RPC server (`open`, `verify`, `strict`).
pub const PRIMALSPRING_AUTH_MODE: &str = "PRIMALSPRING_AUTH_MODE";
/// Socket mode for the primal server (`abstract`, `path`).
pub const PRIMALSPRING_SOCKET_MODE: &str = "PRIMALSPRING_SOCKET_MODE";
/// Generic socket mode (fallback for `PRIMALSPRING_SOCKET_MODE`).
pub const PRIMAL_SOCKET_MODE: &str = "PRIMAL_SOCKET_MODE";

// ── biomeOS integration ─────────────────────────────────────────────

/// biomeOS operating mode.
pub const BIOMEOS_MODE: &str = "BIOMEOS_MODE";
/// When `"1"` or `"true"`, disables BTSP enforcement (development only).
pub const BIOMEOS_INSECURE: &str = "BIOMEOS_INSECURE";
/// Override for the biomeOS registration target primal.
pub const BIOMEOS_PRIMAL: &str = "BIOMEOS_PRIMAL";
/// biomeOS-provided family ID (alternative to [`FAMILY_ID`]).
pub const BIOMEOS_FAMILY_ID: &str = "BIOMEOS_FAMILY_ID";

// ── plasmidBin / distribution ───────────────────────────────────────

/// Override directory for `plasmidBin` binary distribution cache.
pub const ECOPRIMALS_PLASMID_BIN: &str = "ECOPRIMALS_PLASMID_BIN";

/// Root of the ecoPrimals workspace (e.g. `/home/user/Development/ecoPrimals`).
/// When set, `discover_binary` also searches `$ECOPRIMALS_ROOT/infra/plasmidBin`
/// for harvested binaries, bridging the workspace checkout to the runtime consumer.
pub const ECOPRIMALS_ROOT: &str = "ECOPRIMALS_ROOT";

// ── Tier 5 TCP discovery control ────────────────────────────────────

/// When `"1"` or `"true"`, enables Tier 5 TCP port probing in `discover()`.
///
/// Tier 5 exposes well-known TCP ports per primal, which is a metadata leak:
/// an observer can infer which primals are running by probing ports. The
/// zero-port Tower Atomic standard (UDS-only) avoids this. Tier 5 remains
/// valid for containers, cross-arch, and legacy deployments that opt in.
pub const PRIMALSPRING_TCP_TIER5: &str = "PRIMALSPRING_TCP_TIER5";

// ── Per-primal TCP port overrides ───────────────────────────────────

/// TCP port override for BearDog.
pub const BEARDOG_PORT: &str = "BEARDOG_PORT";
/// TCP port override for Songbird.
pub const SONGBIRD_PORT: &str = "SONGBIRD_PORT";
/// TCP port override for NestGate.
pub const NESTGATE_PORT: &str = "NESTGATE_PORT";
/// TCP port override for toadStool.
pub const TOADSTOOL_PORT: &str = "TOADSTOOL_PORT";
/// TCP port override for barraCuda.
pub const BARRACUDA_PORT: &str = "BARRACUDA_PORT";
/// TCP port override for coralReef.
pub const CORALREEF_PORT: &str = "CORALREEF_PORT";
/// TCP port override for Squirrel.
pub const SQUIRREL_PORT: &str = "SQUIRREL_PORT";
/// TCP port override for rhizoCrypt.
pub const RHIZOCRYPT_PORT: &str = "RHIZOCRYPT_PORT";
/// TCP port override for sweetGrass.
pub const SWEETGRASS_PORT: &str = "SWEETGRASS_PORT";
/// TCP port override for petalTongue.
pub const PETALTONGUE_PORT: &str = "PETALTONGUE_PORT";
/// TCP port override for loamSpine.
pub const LOAMSPINE_PORT: &str = "LOAMSPINE_PORT";
/// TCP port override for skunkBat.
pub const SKUNKBAT_PORT: &str = "SKUNKBAT_PORT";
/// TCP port override for biomeOS.
pub const BIOMEOS_PORT: &str = "BIOMEOS_PORT";

// ── Cross-primal coordination ─────────────────────────────────────

/// Comma-separated Songbird peer addresses for mesh bootstrap.
pub const SONGBIRD_PEERS: &str = "SONGBIRD_PEERS";

/// Songbird security socket path override.
pub const SONGBIRD_SECURITY_SOCKET: &str = "SONGBIRD_SECURITY_SOCKET";

// ── Binary discovery ──────────────────────────────────────────────

/// Override directory for biomeOS deploy graph resolution.
pub const BIOMEOS_GRAPHS_DIR: &str = "BIOMEOS_GRAPHS_DIR";

/// Override biomeOS plasmidBin binary directory (legacy; prefer `ECOPRIMALS_PLASMID_BIN`).
pub const BIOMEOS_PLASMID_BIN_DIR: &str = "BIOMEOS_PLASMID_BIN_DIR";

// ── Host / network ────────────────────────────────────────────────

/// Hostname fallback (distinct from `HOSTNAME` — some systems export `HOST`).
pub const HOST: &str = "HOST";
