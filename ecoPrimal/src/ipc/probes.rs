// SPDX-License-Identifier: AGPL-3.0-or-later

//! `OnceLock`-cached runtime resource probes for test parallelism.
//!
//! Absorbed from hotSpring V0.6.32, neuralSpring V122, and groundSpring V121.
//! When many tests need to know whether the Neural API or a specific primal
//! is reachable, performing one probe per test wastes time and creates
//! flaky race conditions. These probes run once per process, cache the
//! result in a `OnceLock`, and return the cached value for all subsequent
//! callers — safe for parallel `#[test]` execution.
//!
//! # Usage
//!
//! ```rust,no_run
//! use primalspring::ipc::probes;
//!
//! if !probes::neural_api_reachable() {
//!     // skip live tests
//! }
//! ```

use std::sync::OnceLock;

use super::discover;
use crate::primal_names;

/// Cached result of `neural_api_healthy()`.
static NEURAL_API_PROBE: OnceLock<bool> = OnceLock::new();

/// Cached result of `discover_primal("beardog")`.
static BEARDOG_PROBE: OnceLock<bool> = OnceLock::new();

/// Cached result of `discover_primal("songbird")`.
static SONGBIRD_PROBE: OnceLock<bool> = OnceLock::new();

/// Cached result of `discover_primal("toadstool")`.
static TOADSTOOL_PROBE: OnceLock<bool> = OnceLock::new();

/// Whether the Neural API is reachable (cached once per process).
#[must_use]
pub fn neural_api_reachable() -> bool {
    *NEURAL_API_PROBE.get_or_init(discover::neural_api_healthy)
}

/// Whether BearDog has a reachable socket (cached once per process).
#[must_use]
pub fn beardog_reachable() -> bool {
    *BEARDOG_PROBE.get_or_init(|| {
        discover::discover_primal(primal_names::BEARDOG)
            .socket
            .is_some()
    })
}

/// Whether Songbird has a reachable socket (cached once per process).
#[must_use]
pub fn songbird_reachable() -> bool {
    *SONGBIRD_PROBE.get_or_init(|| {
        discover::discover_primal(primal_names::SONGBIRD)
            .socket
            .is_some()
    })
}

/// Whether ToadStool has a reachable socket (cached once per process).
#[must_use]
pub fn toadstool_reachable() -> bool {
    *TOADSTOOL_PROBE.get_or_init(|| {
        discover::discover_primal(primal_names::TOADSTOOL)
            .socket
            .is_some()
    })
}

/// Whether a Tower atomic composition is plausibly reachable
/// (BearDog + Songbird both have sockets).
#[must_use]
pub fn tower_reachable() -> bool {
    beardog_reachable() && songbird_reachable()
}

/// Whether a Node atomic composition is plausibly reachable
/// (Tower + ToadStool).
#[must_use]
pub fn node_reachable() -> bool {
    tower_reachable() && toadstool_reachable()
}

/// Probe an arbitrary primal by name (NOT cached — use named probes above
/// for frequently checked primals).
#[must_use]
pub fn primal_reachable(name: &str) -> bool {
    discover::discover_primal(name).socket.is_some()
}

/// Probe an arbitrary capability domain (NOT cached).
#[must_use]
pub fn capability_reachable(capability: &str) -> bool {
    discover::discover_by_capability(capability)
        .socket
        .is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neural_api_probe_is_consistent() {
        let a = neural_api_reachable();
        let b = neural_api_reachable();
        assert_eq!(a, b, "OnceLock probe must return the same value");
    }

    #[test]
    fn beardog_probe_is_consistent() {
        let a = beardog_reachable();
        let b = beardog_reachable();
        assert_eq!(a, b);
    }

    #[test]
    fn songbird_probe_is_consistent() {
        let a = songbird_reachable();
        let b = songbird_reachable();
        assert_eq!(a, b);
    }

    #[test]
    fn toadstool_probe_is_consistent() {
        let a = toadstool_reachable();
        let b = toadstool_reachable();
        assert_eq!(a, b);
    }

    #[test]
    fn tower_reachable_requires_both() {
        let t = tower_reachable();
        if t {
            assert!(beardog_reachable());
            assert!(songbird_reachable());
        }
    }

    #[test]
    fn node_reachable_requires_tower_and_toadstool() {
        let n = node_reachable();
        if n {
            assert!(tower_reachable());
            assert!(toadstool_reachable());
        }
    }

    #[test]
    fn primal_reachable_returns_false_for_nonexistent() {
        assert!(!primal_reachable("definitely_not_a_real_primal_xyzzy"));
    }

    #[test]
    fn capability_reachable_returns_false_for_nonexistent() {
        assert!(!capability_reachable("definitely_not_a_real_capability"));
    }
}
