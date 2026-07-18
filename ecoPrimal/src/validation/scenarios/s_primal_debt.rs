// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Primal Debt — detects missing modules from the composition by
//! comparing the `FullNucleus` capability roster against what is actually
//! discoverable on the local gate.
//!
//! "Debt" here means: a capability that the NUCLEUS composition *requires*
//! but the local gate does not currently provide (socket missing, primal not
//! running, or health probe fails).
//!
//! This scenario:
//! 1. Enumerates the full 13-capability roster from `AtomicType::FullNucleus`
//! 2. Probes each capability's discovery status
//! 3. For discovered capabilities, performs a health.liveness check
//! 4. Tracks debt ratio: (total - live) / total
//! 5. Asserts debt is within acceptable bounds for the gate class

use crate::composition::CompositionContext;
use crate::coordination::AtomicType;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Primal debt: missing capabilities from `FullNucleus` composition.
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

    let max_debt_pct = debt_cap_for_gate(&gate);

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

/// Resolve debt cap for a gate by discovering its roles and composition level
/// from the ecosystem manifest. No hardcoded gate → threshold mapping.
///
/// Priority:
/// - Gates with `build_hub` or `depot` role → tightest (NEST: provenance authority)
/// - Gates with `composition = "full"` and no special role → standard (REFERENCE)
/// - Gates with `tower` or `subset` composition → relaxed (VPS)
/// - Unknown gates → DEFAULT
fn debt_cap_for_gate(gate_name: &str) -> f64 {
    const MANIFEST: &str =
        include_str!("../../../../../../infra/wateringHole/ecosystem_manifest.toml");

    let parsed: toml::Value = match toml::from_str(MANIFEST) {
        Ok(v) => v,
        Err(_) => return crate::tolerances::DEBT_CAP_DEFAULT_PCT,
    };

    let gate_info = parsed
        .get("gates")
        .and_then(|g| g.as_table())
        .and_then(|gates| {
            gates.iter().find_map(|(name, info)| {
                if name.eq_ignore_ascii_case(gate_name) {
                    Some(info)
                } else {
                    None
                }
            })
        });

    let Some(info) = gate_info else {
        return crate::tolerances::DEBT_CAP_DEFAULT_PCT;
    };

    let roles: Vec<&str> = info
        .get("roles")
        .and_then(|r| r.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default();

    let is_depot_authority = roles.contains(&"depot");
    if is_depot_authority {
        return crate::tolerances::DEBT_CAP_NEST_PCT;
    }

    let composition = info
        .get("composition")
        .and_then(|c| c.as_str())
        .unwrap_or("unknown");

    match composition {
        "full" => crate::tolerances::DEBT_CAP_REFERENCE_PCT,
        "tower" | "subset" => crate::tolerances::DEBT_CAP_VPS_PCT,
        _ => crate::tolerances::DEBT_CAP_DEFAULT_PCT,
    }
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

    #[test]
    fn debt_cap_resolves_from_manifest() {
        let cap = debt_cap_for_gate("sporeGate");
        assert_eq!(
            cap,
            crate::tolerances::DEBT_CAP_NEST_PCT,
            "sporeGate has build_hub/depot roles → tightest ceiling"
        );

        let cap = debt_cap_for_gate("eastGate");
        assert_eq!(
            cap,
            crate::tolerances::DEBT_CAP_REFERENCE_PCT,
            "eastGate has composition=full, no provenance role → reference"
        );

        let cap = debt_cap_for_gate("unknown_gate_xyz");
        assert_eq!(
            cap,
            crate::tolerances::DEBT_CAP_DEFAULT_PCT,
            "unknown gate → default fallback"
        );
    }
}
