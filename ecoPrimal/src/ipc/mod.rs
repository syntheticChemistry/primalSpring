// SPDX-License-Identifier: AGPL-3.0-or-later

//! IPC client for primalSpring coordination validation.
//!
//! Provides JSON-RPC 2.0 over IPC ([`transport::Transport`]: Unix sockets for
//! local primals, TCP when connecting by address) for communicating with primals
//! during composition and coordination validation. Discovery is
//! capability-based: primalSpring has only self-knowledge and discovers
//! peers at runtime via environment overrides, XDG socket convention,
//! or the Neural API.
//!
//! # Resilience
//!
//! The [`resilience`] module provides `CircuitBreaker` and `RetryPolicy`
//! for robust IPC under intermittent primal availability. The [`dispatch`]
//! module classifies responses into `Success`, `ProtocolError`, or
//! `ApplicationError` for informed retry decisions.

pub mod capability;
pub mod client;
pub mod discover;
pub mod dispatch;
pub mod error;
pub mod extract;
pub mod mcp;
pub mod methods;
pub mod neural_bridge;
pub mod probes;
#[cfg(test)]
mod proptest_ipc;
pub mod protocol;
pub mod provenance;
pub mod resilience;
pub mod tcp;
pub mod transport;

pub use error::IpcError;
pub use neural_bridge::NeuralBridge;

/// Known legacy method prefixes that callers may prepend.
///
/// Ecosystem convention (groundSpring V121, neuralSpring V122, wetSpring V133,
/// healthSpring V42, `BearDog` Waves 9-12, Songbird v0.2.1): strip any of these
/// dotted prefixes before dispatch so `primalspring.health.check` resolves
/// identically to `health.check`.
const LEGACY_PREFIXES: &[&str] = &[
    "primalspring.",
    "barracuda.",
    "biomeos.",
    "groundspring.",
    "neuralspring.",
    "wetspring.",
    "airspring.",
    "healthspring.",
    "ludospring.",
];

/// Normalize a JSON-RPC method name by stripping known legacy prefixes.
///
/// Ecosystem-wide standard absorbed from groundSpring V121, neuralSpring V122,
/// wetSpring V133, healthSpring V42, `BearDog`, and Songbird. Callers may send
/// `primalspring.health.check` or `health.check` — both resolve identically.
///
/// Also handles the `capabilities.list` → `capabilities.list` plural alias
/// that some ecosystem callers send.
#[must_use]
pub fn normalize_method(method: &str) -> &str {
    let stripped = LEGACY_PREFIXES
        .iter()
        .find_map(|prefix| method.strip_prefix(prefix))
        .unwrap_or(method);

    match stripped {
        "capability.list" => "capabilities.list",
        other => other,
    }
}

#[cfg(test)]
mod normalize_tests {
    use super::*;

    #[test]
    fn strips_primalspring_prefix() {
        assert_eq!(
            normalize_method("primalspring.health.check"),
            "health.check"
        );
    }

    #[test]
    fn strips_barracuda_prefix() {
        assert_eq!(
            normalize_method("barracuda.compute.submit"),
            "compute.submit"
        );
    }

    #[test]
    fn strips_biomeos_prefix() {
        assert_eq!(
            normalize_method("biomeos.lifecycle.status"),
            "lifecycle.status"
        );
    }

    #[test]
    fn no_prefix_unchanged() {
        assert_eq!(normalize_method("health.check"), "health.check");
    }

    #[test]
    fn capability_singular_to_plural() {
        assert_eq!(normalize_method("capability.list"), "capabilities.list");
    }

    #[test]
    fn capabilities_plural_unchanged() {
        assert_eq!(normalize_method("capabilities.list"), "capabilities.list");
    }

    #[test]
    fn prefix_plus_alias() {
        assert_eq!(
            normalize_method("primalspring.capability.list"),
            "capabilities.list"
        );
    }

    #[test]
    fn empty_method_unchanged() {
        assert_eq!(normalize_method(""), "");
    }
}
