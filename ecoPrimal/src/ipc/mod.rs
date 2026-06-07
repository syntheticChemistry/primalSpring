// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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

pub mod btsp_handshake;
pub mod capability;
pub mod client;
pub mod discover;
pub mod dispatch;
pub mod error;
pub mod extract;
pub mod mcp;
pub mod method_gate;
pub mod methods;
pub mod neural_bridge;
pub mod verifiers;
#[cfg(test)]
mod proptest_ipc;
pub mod protocol;
pub mod provenance;
pub mod resilience;
pub mod tcp;
pub mod transport;

pub use error::IpcError;
pub use neural_bridge::NeuralBridge;

/// Dynamically-built set of known primal and spring slug prefixes
/// (e.g. `"primalspring."`, `"beardog."`) derived from `primal_names`.
///
/// Replaces the former hardcoded `LEGACY_PREFIXES` list. Adding a new
/// primal or spring variant to the `Primal`/`Spring` enums automatically
/// extends prefix normalization with zero manual maintenance.
static SLUG_PREFIXES: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| {
    use crate::primal_names::{Primal, Spring};
    let primals = [
        Primal::BearDog, Primal::Songbird, Primal::ToadStool,
        Primal::NestGate, Primal::Squirrel, Primal::BarraCuda,
        Primal::CoralReef, Primal::BiomeOS, Primal::PetalTongue,
        Primal::RhizoCrypt, Primal::LoamSpine, Primal::SweetGrass,
        Primal::SkunkBat,
    ];
    let springs = [
        Spring::PrimalSpring, Spring::HotSpring, Spring::GroundSpring,
        Spring::NeuralSpring, Spring::WetSpring, Spring::AirSpring,
        Spring::HealthSpring, Spring::LudoSpring,
    ];
    let mut prefixes: Vec<String> = primals
        .iter()
        .map(|p| format!("{}.", p.slug()))
        .chain(springs.iter().map(|s| format!("{}.", s.slug())))
        .collect();
    prefixes.sort_unstable_by_key(|b| std::cmp::Reverse(b.len()));
    prefixes
});

/// Normalize a JSON-RPC method name by stripping known primal/spring prefixes.
///
/// Ecosystem-wide standard: callers may send `primalspring.health.check`
/// or `health.check` — both resolve identically. Prefixes are derived from
/// the `Primal` and `Spring` enums so new additions are automatic.
///
/// Also normalizes the `capabilities.list` / `capability.list` alias.
#[must_use]
pub fn normalize_method(method: &str) -> &str {
    let stripped = SLUG_PREFIXES
        .iter()
        .find_map(|prefix| method.strip_prefix(prefix.as_str()))
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
