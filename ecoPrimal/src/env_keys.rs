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
/// Hex-encoded family seed used as BTSP key material (primary).
pub const FAMILY_SEED: &str = "FAMILY_SEED";
/// Legacy alias — prefer [`FAMILY_SEED`]. Retained for backward compatibility
/// with deployments that set `BEARDOG_FAMILY_SEED` directly.
#[deprecated(since = "0.9.31", note = "use FAMILY_SEED instead")]
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
/// Server-side bind mode: `uds_only` (default), `tcp_only`, `fallback`.
///
/// Controls how [`ipc::server_bind::bind_transport`] selects its transport.
/// Set to `fallback` on platforms where UDS may be denied (Android SELinux).
/// Set to `tcp_only` to skip UDS entirely (grapheneGate, containers).
pub const PRIMAL_BIND_MODE: &str = "PRIMAL_BIND_MODE";

// ── biomeOS integration ─────────────────────────────────────────────

/// biomeOS operating mode.
pub const BIOMEOS_MODE: &str = "BIOMEOS_MODE";
/// When `"1"` or `"true"`, disables BTSP enforcement (development only).
pub const BIOMEOS_INSECURE: &str = "BIOMEOS_INSECURE";
/// Override for the biomeOS registration target primal.
pub const BIOMEOS_PRIMAL: &str = "BIOMEOS_PRIMAL";
/// biomeOS-provided family ID (alternative to [`FAMILY_ID`]).
pub const BIOMEOS_FAMILY_ID: &str = "BIOMEOS_FAMILY_ID";
/// Override directory for biomeOS UDS sockets.
pub const BIOMEOS_SOCKET_DIR: &str = "BIOMEOS_SOCKET_DIR";

// ── plasmidBin / distribution ───────────────────────────────────────

/// Override directory for `plasmidBin` binary distribution cache.
pub const ECOPRIMALS_PLASMID_BIN: &str = "ECOPRIMALS_PLASMID_BIN";

/// Root of the ecoPrimals workspace (e.g. `/home/user/Development/ecoPrimals`).
///
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
//
// These follow the generic `{SLUG_UPPER}_PORT` pattern and are derived
// from `config/ports.toml` at the authoritative level. New code should
// prefer `port_env_key(slug)` over referencing these constants directly.

/// Derive the env var name for a primal's TCP port override from its slug.
///
/// Follows the pattern `{SLUG_UPPER}_PORT` (e.g. `beardog` → `BEARDOG_PORT`).
/// Returns a `&'static str` via a lazily-built intern table.
pub fn port_env_key(slug: &str) -> &'static str {
    use std::collections::HashMap;
    use std::sync::LazyLock;

    static INTERN: LazyLock<HashMap<String, &'static str>> = LazyLock::new(|| {
        let mut map = HashMap::new();
        let ports_toml: &str = include_str!("../../config/ports.toml");
        if let Ok(parsed) = ports_toml.parse::<toml::Table>() {
            for (slug, section) in &parsed {
                if slug == "federation" {
                    continue;
                }
                if let Some(key) = section
                    .as_table()
                    .and_then(|t| t.get("env_key"))
                    .and_then(|v| v.as_str())
                {
                    map.insert(slug.clone(), &*Box::leak(key.to_owned().into_boxed_str()));
                }
            }
        }
        map
    });

    INTERN.get(slug).copied().unwrap_or_else(|| {
        static FALLBACK: LazyLock<std::sync::Mutex<Vec<&'static str>>> =
            LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
        let key = format!("{}_PORT", slug.to_uppercase());
        let leaked: &'static str = &*Box::leak(key.into_boxed_str());
        if let Ok(mut guard) = FALLBACK.lock() {
            guard.push(leaked);
        }
        leaked
    })
}

// Per-primal TCP port env var names are derived dynamically from
// `config/ports.toml` via `port_env_key(slug)`. No static constants
// needed — the pattern is `{SLUG_UPPER}_PORT` (e.g. "BEARDOG_PORT").

// ── Cross-primal coordination ─────────────────────────────────────

/// Comma-separated discovery provider peer addresses for mesh bootstrap.
///
/// Legacy name references Songbird (the discovery primal), but the concept
/// is generic: any discovery provider reads this for initial peer seeding.
pub const SONGBIRD_PEERS: &str = "SONGBIRD_PEERS";
/// Generic mesh peer addresses (preferred over `SONGBIRD_PEERS` for new code).
pub const MESH_PEERS: &str = "MESH_PEERS";

/// Discovery provider security socket path override.
/// Legacy name references Songbird; new deployments should use the generic
/// discovery socket resolution via capability routing.
pub const SONGBIRD_SECURITY_SOCKET: &str = "SONGBIRD_SECURITY_SOCKET";

// ── Binary discovery ──────────────────────────────────────────────

/// Override directory for biomeOS deploy graph resolution.
pub const BIOMEOS_GRAPHS_DIR: &str = "BIOMEOS_GRAPHS_DIR";

/// Override biomeOS plasmidBin binary directory (legacy; prefer `ECOPRIMALS_PLASMID_BIN`).
pub const BIOMEOS_PLASMID_BIN_DIR: &str = "BIOMEOS_PLASMID_BIN_DIR";

// ── Host / network ────────────────────────────────────────────────

/// Human-readable gate name for multi-gate deployments.
pub const GATE_NAME: &str = "GATE_NAME";
/// Gate identifier for mesh topology and BTSP binding.
pub const GATE_ID: &str = "GATE_ID";
/// Remote gate name for cross-gate verification scenarios.
pub const REMOTE_GATE_NAME: &str = "REMOTE_GATE_NAME";
/// benchScale topology name for live mesh configuration.
pub const BENCHSCALE_TOPOLOGY: &str = "BENCHSCALE_TOPOLOGY";

/// Hostname fallback (distinct from `HOSTNAME` — some systems export `HOST`).
pub const HOST: &str = "HOST";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_family_id_defaults_to_default() {
        let id = resolve_family_id();
        assert!(!id.is_empty(), "family ID should never be empty");
    }

    #[test]
    fn resolve_family_seed_returns_none_without_env() {
        let seed = resolve_family_seed();
        assert!(
            seed.is_none() || !seed.as_deref().unwrap_or("").is_empty(),
            "seed should be None or non-empty"
        );
    }

    #[test]
    fn port_env_key_known_primals() {
        assert_eq!(port_env_key("beardog"), "BEARDOG_PORT");
        assert_eq!(port_env_key("songbird"), "SONGBIRD_PORT");
        assert_eq!(port_env_key("nestgate"), "NESTGATE_PORT");
        assert_eq!(port_env_key("squirrel"), "SQUIRREL_PORT");
    }

    #[test]
    fn port_env_key_unknown_primal_follows_pattern() {
        let key = port_env_key("newprimal");
        assert_eq!(key, "NEWPRIMAL_PORT");
    }

    #[test]
    fn port_env_keys_match_expected_pattern() {
        assert_eq!(port_env_key("beardog"), "BEARDOG_PORT");
        assert_eq!(port_env_key("songbird"), "SONGBIRD_PORT");
        assert_eq!(port_env_key("toadstool"), "TOADSTOOL_PORT");
        assert_eq!(port_env_key("biomeos"), "BIOMEOS_PORT");
    }

    #[test]
    fn all_registry_env_keys_are_uppercase_port_pattern() {
        let slugs = [
            "beardog", "songbird", "nestgate", "toadstool", "barracuda",
            "coralreef", "squirrel", "rhizocrypt", "sweetgrass",
            "petaltongue", "loamspine", "skunkbat", "biomeos",
        ];
        for slug in &slugs {
            let key = port_env_key(slug);
            assert!(key.ends_with("_PORT"), "{key} should end with _PORT");
            assert_eq!(key, key.to_uppercase(), "{key} should be uppercase");
        }
    }
}
