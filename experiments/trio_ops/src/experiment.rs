// SPDX-License-Identifier: AGPL-3.0-or-later

//! Shared experiment utilities — common patterns extracted from 90+ experiments.
//!
//! These helpers eliminate boilerplate for the most common experiment phases:
//! composition discovery, NeuralBridge guard, remote gate configuration,
//! and health probing.

use primalspring::composition::CompositionContext;
use primalspring::ipc::NeuralBridge;
use primalspring::validation::ValidationResult;

/// Run the standard composition discovery phase.
///
/// Reports discovered capabilities and checks that `required_capabilities`
/// are all present. Defaults to `["security"]` if none specified.
pub fn phase_composition_discovery(
    v: &mut ValidationResult,
    ctx: &CompositionContext,
    required_capabilities: &[&str],
) {
    v.section("Composition discovery");
    let caps = ctx.available_capabilities();
    v.check_bool(
        "composition_capabilities_non_empty",
        !caps.is_empty(),
        &format!("{} capabilities: {}", caps.len(), caps.join(", ")),
    );
    let required = if required_capabilities.is_empty() {
        &["security"][..]
    } else {
        required_capabilities
    };
    for &cap in required {
        let check_name = format!("has_{cap}_capability_path");
        v.check_bool(&check_name, ctx.has_capability(cap), &format!("{cap} capability path"));
    }
}

/// Discover NeuralBridge or skip the experiment.
///
/// Returns `Some(bridge)` on success. On failure, records a skip check
/// and returns `None` — callers should `return` from the experiment closure.
pub fn require_neural_bridge(v: &mut ValidationResult) -> Option<NeuralBridge> {
    let bridge = NeuralBridge::discover()?;
    match bridge.health_check() {
        Ok(_) => {
            v.check_bool("neural_api_health", true, "biomeOS neural-api healthy");
            Some(bridge)
        }
        Err(e) => {
            v.check_bool("neural_api_health", false, &format!("neural-api: {e}"));
            None
        }
    }
}

/// Remote gate connection configuration from environment variables.
///
/// Used by cross-gate experiments (exp073, exp081-084) that need a
/// `REMOTE_GATE_HOST` or `GATE_HOSTS` to target.
pub struct RemoteGateConfig {
    /// Primary remote host (from `REMOTE_GATE_HOST`).
    pub host: Option<String>,
    /// Additional gate hosts (from `GATE_HOSTS`, comma-separated).
    pub extra_hosts: Vec<String>,
    /// Family ID (from `FAMILY_ID` env var).
    pub family_id: String,
}

impl RemoteGateConfig {
    /// Read remote gate configuration from environment.
    #[must_use]
    pub fn from_env() -> Self {
        let host = std::env::var("REMOTE_GATE_HOST").ok().filter(|h| !h.is_empty());
        let extra_hosts = std::env::var("GATE_HOSTS")
            .unwrap_or_default()
            .split(',')
            .map(str::trim)
            .filter(|h| !h.is_empty())
            .map(String::from)
            .collect();
        let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());
        Self {
            host,
            extra_hosts,
            family_id,
        }
    }

    /// All hosts to probe (primary + extras), deduplicated.
    #[must_use]
    pub fn all_hosts(&self) -> Vec<&str> {
        let mut hosts: Vec<&str> = Vec::new();
        if let Some(h) = &self.host {
            hosts.push(h);
        }
        for h in &self.extra_hosts {
            if !hosts.contains(&h.as_str()) {
                hosts.push(h);
            }
        }
        hosts
    }

    /// Whether any remote gate is configured.
    #[must_use]
    pub fn has_remote(&self) -> bool {
        self.host.is_some() || !self.extra_hosts.is_empty()
    }

    /// Skip the experiment if no remote gate is configured, running only
    /// structural checks via the provided closure.
    pub fn require_remote_or_structural(
        &self,
        v: &mut ValidationResult,
        structural: impl FnOnce(&mut ValidationResult),
    ) -> bool {
        if self.has_remote() {
            return true;
        }
        v.section("Structural checks (no remote gate configured)");
        structural(v);
        false
    }
}

/// Read an environment variable with a default value.
#[must_use]
pub fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_owned())
}
