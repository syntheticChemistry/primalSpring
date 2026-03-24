// SPDX-License-Identifier: AGPL-3.0-or-later

//! Validates zero information leakage with unknown primals (weak bonding).
//! Source: `phase2/biomeOS/graphs/BONDING_TESTS_README.md`

use primalspring::ipc::discover::{DiscoverySource, discover_primal, socket_env_var};
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp059 — Weak Force Isolation")
        .with_provenance("exp059_weak_force_isolation", "2026-03-24")
        .run(
            "primalSpring Exp059: Zero Trust with Unknown Primals (Weak Bonding)",
            |v| {
                let unknown = discover_primal("definitely_not_a_primal_xyzzy");
                v.check_bool(
                    "unknown_primal_returns_not_found",
                    unknown.socket.is_none() && unknown.source == DiscoverySource::NotFound,
                    "discover_primal for unknown names returns NotFound",
                );

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
            },
        );
}
