// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: Node Atomic — absorbed from exp002.

use crate::composition::CompositionContext;
use crate::coordination::AtomicType;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "node-atomic",
        track: Track::AtomicComposition,
        tier: Tier::Live,
        provenance_crate: "exp002_node_atomic",
        provenance_date: "2026-05-09",
        description: "Node Atomic — security, discovery, and compute capabilities",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural");
    phase_structural(v);

    v.section("Phase 2: Discovery");
    phase_discovery(v, ctx);

    v.section("Phase 3: Health");
    phase_health(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    let node_caps = AtomicType::Node.required_capabilities();
    v.check_count("node_required_caps", node_caps.len(), 5);
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let node_caps = AtomicType::Node.required_capabilities();
    let caps = ctx.available_capabilities();
    v.check_bool(
        "discovery_context_nonempty",
        !caps.is_empty(),
        &format!("{} context capabilities: {}", caps.len(), caps.join(", ")),
    );
    for cap in node_caps {
        v.check_bool(
            &format!("has_{cap}"),
            ctx.has_capability(cap),
            &format!("{cap} capability resolved"),
        );
    }
}

fn phase_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let node_caps = AtomicType::Node.required_capabilities();
    for cap in node_caps {
        match ctx.health_check(cap) {
            Ok(true) => v.check_bool(
                &format!("health_liveness_{cap}"),
                true,
                &format!("{cap} health.liveness ok"),
            ),
            Ok(false) => v.check_bool(
                &format!("health_liveness_{cap}"),
                false,
                &format!("{cap} not live"),
            ),
            Err(e) if e.is_skippable() => v.check_skip(
                &format!("health_liveness_{cap}"),
                &format!("{cap} not reachable: {e}"),
            ),
            Err(e) => v.check_bool(
                &format!("health_liveness_{cap}"),
                false,
                &format!("{cap} health check error: {e}"),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_atomic_no_panic() {
        let mut v = ValidationResult::new("node-atomic");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.evaluated() > 0 || v.skipped > 0,
            "scenario should produce at least one check"
        );
    }
}
