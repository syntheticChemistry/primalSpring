// SPDX-License-Identifier: AGPL-3.0-or-later

//! Launch profile loading from embedded TOML.

use std::collections::HashMap;

use super::LaunchError;

static LAUNCH_PROFILES_TOML: &str = include_str!("../../../config/primal_launch_profiles.toml");

/// Per-primal socket configuration loaded from TOML.
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct LaunchProfile {
    /// Override the default `"server"` subcommand (e.g. `"daemon"`).
    /// Set to `""` to skip the subcommand entirely.
    pub subcommand: Option<String>,
    /// CLI flag name for passing socket path (e.g. `"--socket"`).
    /// Set to `"__skip__"` to omit the socket CLI flag entirely.
    pub socket_flag: Option<String>,
    /// Suffix appended to the assigned socket path to get the JSON-RPC
    /// socket (e.g. `".jsonrpc.sock"` for toadstool's dual-protocol mode).
    #[serde(default)]
    pub jsonrpc_socket_suffix: Option<String>,
    /// Whether to pass `--family-id` on the command line.
    pub pass_family_id: Option<bool>,
    /// Env var name for socket path fallback (e.g. `"PRIMAL_SOCKET"`).
    pub env_socket: Option<String>,
    /// Security model: `"btsp"` (Tower perimeter) or `"tower_delegated"`.
    ///
    /// When `"btsp"`, BTSP handshake is required for client connections
    /// (e.g. BearDog). The harness uses this to select the right transport.
    #[serde(default)]
    pub security_model: Option<String>,
    /// Static environment variables to set on the child process.
    #[serde(default)]
    pub extra_env: HashMap<String, String>,
    /// Env vars whose values are resolved socket paths of other primals.
    #[serde(default)]
    pub env_sockets: HashMap<String, String>,
    /// Extra CLI flags whose values are resolved socket paths.
    #[serde(default)]
    pub cli_sockets: HashMap<String, String>,
    /// Extra CLI arguments to pass verbatim (e.g. `["--port", "0"]`).
    #[serde(default)]
    pub extra_args: Vec<String>,
    /// Env vars to forward from the parent process when set.
    #[serde(default)]
    pub passthrough_env: HashMap<String, bool>,
}

#[derive(Debug, serde::Deserialize)]
struct ProfilesConfig {
    default: LaunchProfile,
    #[serde(default)]
    profiles: HashMap<String, LaunchProfile>,
}

/// Load launch profiles from the embedded TOML.
///
/// # Errors
///
/// Returns [`LaunchError::ProfileParseError`] if the TOML is malformed.
pub fn load_launch_profiles() -> Result<(LaunchProfile, HashMap<String, LaunchProfile>), LaunchError>
{
    let config: ProfilesConfig = toml::from_str(LAUNCH_PROFILES_TOML)
        .map_err(|e| LaunchError::ProfileParseError(e.to_string()))?;
    Ok((config.default, config.profiles))
}

/// Whether a primal uses BTSP (`security_model = "btsp"` in its profile).
///
/// Returns `true` for BearDog and similar perimeter primals; `false` for
/// tower-delegated primals that accept cleartext on their UDS socket.
#[must_use]
pub fn primal_requires_btsp(primal: &str) -> bool {
    let Ok((_defaults, profiles)) = load_launch_profiles() else {
        return false;
    };
    profiles
        .get(primal)
        .and_then(|p| p.security_model.as_deref())
        .is_some_and(|m| m == "btsp")
}
