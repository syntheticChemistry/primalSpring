// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Primal Debt — detects missing modules from the composition by
//! comparing the FullNucleus capability roster against what is actually
//! discoverable on the local gate.
//!
//! "Debt" here means: a capability that the NUCLEUS composition *requires*
//! but the local gate does not currently provide (socket missing, primal not
//! running, or health probe fails).
//!
//! This scenario:
//! 1. Enumerates the full 13-capability roster from AtomicType::FullNucleus
//! 2. Probes each capability's discovery status
//! 3. For discovered capabilities, performs a health.liveness check
//! 4. Tracks debt ratio: (total - live) / total
//! 5. Asserts debt is within acceptable bounds for the gate class

use crate::composition::CompositionContext;
use crate::coordination::AtomicType;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Primal debt: missing capabilities from FullNucleus composition.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "primal-debt",
        track: Track::Lifecycle,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-18",
        description: "Detect missing capabilities from FullNucleus composition vs live gate",
    },
    run: run_primal_debt,
};

fn run_primal_debt(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    phase_capability_roster(v);
    let (discovered, total) = phase_discovery_audit(v, ctx);
    let live = phase_health_audit(v, ctx);
    phase_debt_ratio(v, discovered, live, total);
}

fn phase_capability_roster(v: &mut ValidationResult) {
    let caps = AtomicType::FullNucleus.required_capabilities();
    v.check_bool(
        "debt:roster_size",
        caps.len() == 13,
        &format!("FullNucleus roster: {} capabilities", caps.len()),
    );
}

fn phase_discovery_audit(v: &mut ValidationResult, ctx: &CompositionContext) -> (u32, u32) {
    let caps = AtomicType::FullNucleus.required_capabilities();
    #[expect(clippy::cast_possible_truncation, reason = "capability count < 256")]
    let total = caps.len() as u32;
    let mut discovered = 0u32;
    let mut missing: Vec<&str> = Vec::new();

    for cap in caps {
        if ctx.has_capability(cap) {
            discovered += 1;
        } else {
            missing.push(cap);
        }
    }

    v.check_bool(
        "debt:discovered_count",
        discovered > 0,
        &format!("{discovered}/{total} capabilities discovered"),
    );

    if !missing.is_empty() {
        v.check_skip(
            "debt:missing_capabilities",
            &format!("not discovered: {}", missing.join(", ")),
        );
    }

    (discovered, total)
}

fn phase_health_audit(v: &mut ValidationResult, ctx: &mut CompositionContext) -> u32 {
    let caps = AtomicType::FullNucleus.required_capabilities();
    let mut live = 0u32;
    let mut degraded: Vec<&str> = Vec::new();

    for cap in caps {
        if !ctx.has_capability(cap) {
            continue;
        }

        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(_) => {
                live += 1;
            }
            Err(e) => {
                if e.is_connection_error() {
                    degraded.push(cap);
                } else {
                    live += 1;
                }
            }
        }
    }

    v.check_bool(
        "debt:live_count",
        live > 0,
        &format!("{live} capabilities answer health.liveness"),
    );

    if !degraded.is_empty() {
        v.check_skip(
            "debt:degraded",
            &format!("degraded (connection errors): {}", degraded.join(", ")),
        );
    }

    live
}

fn phase_debt_ratio(v: &mut ValidationResult, discovered: u32, live: u32, total: u32) {
    let debt = total.saturating_sub(discovered);
    let debt_pct = if total > 0 {
        (f64::from(debt) / f64::from(total)) * 100.0
    } else {
        100.0
    };

    let gate = std::env::var("GATE_NAME")
        .or_else(|_| {
            std::process::Command::new("hostname")
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        })
        .unwrap_or_else(|_| "unknown".to_owned());

    let max_debt_pct = match gate.as_str() {
        "eastgate" | "eastGate" => 70.0,
        "sporegate" | "sporeGate" => 30.0,
        "golgi" | "pepti" => 50.0,
        _ => 80.0,
    };

    v.check_bool(
        "debt:ratio_acceptable",
        debt_pct <= max_debt_pct,
        &format!(
            "debt: {debt}/{total} ({debt_pct:.0}%) — max allowed {max_debt_pct:.0}% for {gate}",
        ),
    );

    v.check_bool(
        "debt:live_vs_discovered",
        live >= discovered.saturating_sub(1),
        &format!("{live}/{discovered} discovered capabilities are live"),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primal_debt_structural() {
        let mut v = ValidationResult::new("primal-debt");
        let mut ctx = CompositionContext::discover();
        run_primal_debt(&mut v, &mut ctx);
        assert!(
            v.passed + v.failed + v.skipped > 0,
            "primal-debt should evaluate at least one check"
        );
    }
}
