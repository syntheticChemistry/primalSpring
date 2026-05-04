// SPDX-License-Identifier: AGPL-3.0-or-later

//! Centralized environment variable name constants.
//!
//! Every `std::env::var("...")` call in the codebase should reference a
//! constant from this module instead of a bare string literal. This makes
//! typos impossible and env-var usage greppable from one location.

// ── Identity & genetics ──────────────────────────────────────────────

/// Family group identifier for multi-tenant socket paths and BTSP genetics.
pub const FAMILY_ID: &str = "FAMILY_ID";
/// Hex-encoded family seed used as BTSP key material.
pub const FAMILY_SEED: &str = "FAMILY_SEED";
/// BearDog-specific alias for the family seed.
pub const BEARDOG_FAMILY_SEED: &str = "BEARDOG_FAMILY_SEED";

// ── XDG / OS paths ──────────────────────────────────────────────────

/// XDG runtime directory for sockets and ephemeral state.
pub const XDG_RUNTIME_DIR: &str = "XDG_RUNTIME_DIR";
/// XDG data home for persistent user data.
pub const XDG_DATA_HOME: &str = "XDG_DATA_HOME";
/// User home directory.
pub const HOME: &str = "HOME";
/// System hostname.
pub const HOSTNAME: &str = "HOSTNAME";

// ── Socket / discovery ──────────────────────────────────────────────

/// Explicit override for the socket directory.
pub const SOCKET_DIR: &str = "SOCKET_DIR";
/// Explicit path to the biomeOS neural-api socket.
pub const NEURAL_API_SOCKET: &str = "NEURAL_API_SOCKET";

// ── primalSpring configuration ──────────────────────────────────────

/// Host address for TCP fallback connections.
pub const PRIMALSPRING_HOST: &str = "PRIMALSPRING_HOST";
/// Override directory for deploy graph resolution.
pub const PRIMALSPRING_GRAPHS_DIR: &str = "PRIMALSPRING_GRAPHS_DIR";
/// When set, guidestone emits JSON output instead of human-readable text.
pub const PRIMALSPRING_JSON: &str = "PRIMALSPRING_JSON";

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
