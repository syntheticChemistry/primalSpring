// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp059: Weak Force Isolation — unknown primal discovery resilience and socket env contract.

use primalspring::ipc::discover::{DiscoverySource, discover_primal, socket_env_var};
use primalspring::validation::ValidationResult;

fn phase_resilience(v: &mut ValidationResult) {
    let unknown = discover_primal("definitely_not_a_primal_xyzzy");
    v.check_bool(
        "unknown_primal_returns_not_found",
        unknown.socket.is_none() && unknown.source == DiscoverySource::NotFound,
        "discover_primal for unknown names returns NotFound",
    );
}

fn phase_isolation(v: &mut ValidationResult) {
    let nonexistent_env = socket_env_var("nonexistent_primal_env_test_12345");
    v.check_bool(
        "socket_env_var_nonexistent_returns_none",
        nonexistent_env.is_none(),
        "socket_env_var for nonexistent env returns None",
    );

    v.check_skip(
        "actual_isolation_testing",
        "actual isolation testing needs live primals",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp059 — Weak Force Isolation")
        .with_provenance("exp059_weak_force_isolation", "2026-05-09")
        .run(
            "primalSpring Exp059: Zero Trust with Unknown Primals (Weak Bonding)",
            |v| {
                v.section("Phase 1: Resilience");
                phase_resilience(v);

                v.section("Phase 2: Isolation");
                phase_isolation(v);
            },
        );
}
