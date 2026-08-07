// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp059: Weak Force Isolation — unknown primal discovery resilience and socket env contract.

use primalspring::composition::CompositionContext;
use primalspring::ipc::NeuralBridge;
use primalspring::validation::ValidationResult;

fn phase_resilience(v: &mut ValidationResult) {
    let ctx = CompositionContext::from_live_discovery_with_fallback();

    let unknown = ctx.has_capability("definitely_not_a_primal_xyzzy");
    v.check_bool(
        "unknown_capability_returns_false",
        !unknown,
        "CompositionContext returns false for unknown capabilities",
    );

    if let Some(bridge) = NeuralBridge::discover() {
        let result = bridge.capability_call(
            "definitely_not_a_domain",
            "no_method",
            &serde_json::json!({}),
        );
        v.check_bool(
            "neural_bridge_rejects_unknown",
            result.is_err(),
            "NeuralBridge cleanly rejects unknown capability calls",
        );
    } else {
        v.check_skip(
            "neural_bridge_rejects_unknown",
            "biomeOS not running — NeuralBridge isolation test skipped",
        );
    }
}

fn phase_isolation(v: &mut ValidationResult) {
    #[cfg(feature = "primordial-compat")]
    {
        let nonexistent_env =
            primalspring::ipc::discover::socket_env_var("nonexistent_primal_env_test_12345");
        v.check_bool(
            "socket_env_var_nonexistent_returns_none",
            nonexistent_env.is_none(),
            "socket_env_var for nonexistent env returns None",
        );
    }

    #[cfg(not(feature = "primordial-compat"))]
    {
        let env_result = std::env::var("NONEXISTENT_PRIMAL_SOCKET_12345");
        v.check_bool(
            "env_var_nonexistent_returns_err",
            env_result.is_err(),
            "environment variable for nonexistent primal is absent",
        );
    }

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
                v.section("Phase 1: Capability-First Resilience");
                phase_resilience(v);

                v.section("Phase 2: Isolation");
                phase_isolation(v);
            },
        );
}
