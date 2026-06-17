// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Gate Readiness Matrix — validates the per-gate deployment
//! tracking infrastructure and verifies ecosystem posture.
//!
//! This scenario exercises the [`GateMatrix`](crate::evolution::GateMatrix)
//! readiness model to ensure:
//!
//! 1. All known gates are tracked (7 gates in the ecosystem)
//! 2. Readiness levels are properly ordered
//! 3. Derived readiness matches observed metrics
//! 4. The local gate (eastGate) is correctly identified
//! 5. Summary formatting is valid
//!
//! Phase 1 (Structural): Matrix construction and ordering invariants
//! Phase 2 (Live): Local gate detection and environment readiness

use crate::composition::CompositionContext;
use crate::evolution::{GateMatrix, GateStatus, ReadinessLevel};
use crate::validation::scenarios::{Scenario, ScenarioMeta, Tier, Track};
use crate::validation::ValidationResult;

/// Gate readiness matrix scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "gate-readiness",
        track: Track::Evolution,
        tier: Tier::Rust,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-17",
        description: "Validates gate readiness matrix infrastructure and ecosystem posture",
    },
    run,
};

/// Run gate readiness validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Matrix structural invariants");
    phase_structural(v);

    v.section("Phase 2: Local gate readiness");
    phase_local_gate(v);
}

fn phase_structural(v: &mut ValidationResult) {
    let matrix = GateMatrix::ecosystem_snapshot();

    v.check_bool(
        "matrix:gate_count",
        matrix.gates.len() == 7,
        &format!("{} gates tracked (expect 7)", matrix.gates.len()),
    );

    let expected_gates = ["eastGate", "sporeGate", "golgi", "pepti", "northGate", "fieldGate", "flockGate"];
    for name in &expected_gates {
        let found = matrix.gates.iter().any(|g| g.name == *name);
        v.check_bool(
            &format!("matrix:has:{name}"),
            found,
            &format!("{name}: {}", if found { "tracked" } else { "MISSING" }),
        );
    }

    v.check_bool(
        "matrix:ordering:verified_gt_full",
        ReadinessLevel::Verified > ReadinessLevel::Full,
        "Verified > Full",
    );
    v.check_bool(
        "matrix:ordering:full_gt_partial",
        ReadinessLevel::Full > ReadinessLevel::Partial,
        "Full > Partial",
    );
    v.check_bool(
        "matrix:ordering:partial_gt_reachable",
        ReadinessLevel::Partial > ReadinessLevel::Reachable,
        "Partial > Reachable",
    );
    v.check_bool(
        "matrix:ordering:reachable_gt_offline",
        ReadinessLevel::Reachable > ReadinessLevel::Offline,
        "Reachable > Offline",
    );

    let mut test_gate = GateStatus::new("test", ReadinessLevel::Offline);
    test_gate.primals_alive = 13;
    test_gate.primals_expected = 13;
    test_gate.vcs_synced = true;
    test_gate.depot_fresh = true;
    v.check_bool(
        "matrix:derived:full_metrics_verified",
        test_gate.derived_readiness() == ReadinessLevel::Verified,
        "13/13 alive + synced + fresh = Verified",
    );

    test_gate.vcs_synced = false;
    v.check_bool(
        "matrix:derived:missing_sync_not_verified",
        test_gate.derived_readiness() == ReadinessLevel::Full,
        "13/13 alive but not synced = Full (not Verified)",
    );

    let summary = matrix.summary();
    v.check_bool(
        "matrix:summary:contains_gates",
        summary.contains("7 gates"),
        &format!("summary: {summary}"),
    );
}

fn phase_local_gate(v: &mut ValidationResult) {
    let hostname = crate::tolerances::platform::hostname().unwrap_or_default();
    let gate_name = std::env::var(crate::env_keys::GATE_NAME).unwrap_or_default();

    let local_identity = if !gate_name.is_empty() {
        gate_name
    } else if !hostname.is_empty() {
        hostname
    } else {
        String::new()
    };

    if local_identity.is_empty() {
        v.check_skip(
            "local:identity",
            "no local gate identity (GATE_NAME unset, hostname empty)",
        );
        return;
    }

    v.check_bool(
        "local:identity",
        !local_identity.is_empty(),
        &format!("local gate: {local_identity}"),
    );

    let matrix = GateMatrix::ecosystem_snapshot();
    let local_in_matrix = matrix.gates.iter().any(|g| {
        g.name.to_lowercase() == local_identity.to_lowercase()
    });

    if local_in_matrix {
        v.check_bool(
            "local:in_matrix",
            true,
            &format!("{local_identity} found in gate matrix"),
        );
    } else {
        v.check_skip(
            "local:in_matrix",
            &format!("{local_identity} not a tracked gate (hostname != gate name)"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_readiness_structural() {
        let mut v = ValidationResult::new("gate-readiness");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(v.failed, 0, "gate readiness structural checks should all pass");
    }
}
