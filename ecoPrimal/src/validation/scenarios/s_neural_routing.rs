// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Neural Routing Surface — validates the capability method routing table.
//!
//! Verifies that the `NeuralRoutingTable` correctly maps every registered
//! method to its owner, domain, and composition tier, and that the tier
//! composition plans are structurally valid. This is the foundation for
//! adaptive routing (Layer 4) and learned routing (Layer 5).

use crate::composition::CompositionContext;
use crate::composition::neural_dispatch::NeuralDispatcher;
use crate::composition::neural_routing::{CompositionTier, canonical_routing_table};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Neural routing surface scenario — `Tier::Rust` structural check.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "neural-routing-surface",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "primalspring_neural_routing",
        provenance_date: "2026-05-22",
        description: "Neural routing table: all methods map to owners, tiers, and composition patterns",
    },
    run,
};

/// Run the neural routing surface validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let table = canonical_routing_table();

    let method_count = table.method_count();
    v.check_bool(
        "routing-table-population",
        method_count >= crate::tolerances::MIN_REGISTERED_METHODS,
        &format!(
            "{method_count} methods loaded (expect {}+)",
            crate::tolerances::MIN_REGISTERED_METHODS
        ),
    );

    let summary = table.tier_summary();
    for tier_name in &["tower", "node", "nest", "meta", "orchestration"] {
        let count = summary.get(tier_name).copied().unwrap_or(0);
        v.check_bool(
            &format!("tier-{tier_name}-populated"),
            count > 0,
            &format!("{tier_name} tier: {count} methods"),
        );
    }

    phase_owner_integrity(v, &table);
    phase_tier_composition(v, &table);
    phase_composition_patterns(v, &table);

    let dispatcher = NeuralDispatcher::with_table(table);
    let report = dispatcher.status_report();
    v.check_bool(
        "dispatcher-report-methods",
        report["total_methods"].as_u64().unwrap_or(0)
            >= crate::tolerances::MIN_REGISTERED_METHODS as u64,
        &format!("dispatcher reports {} methods", report["total_methods"]),
    );

    let total_in_tiers: usize = summary.values().sum();
    v.check_bool(
        "tier-sum-equals-total",
        total_in_tiers == method_count,
        &format!("tier sum {total_in_tiers} == total {method_count}"),
    );
}

fn phase_owner_integrity(
    v: &mut ValidationResult,
    table: &crate::composition::neural_routing::NeuralRoutingTable,
) {
    let security_methods = table.methods_in_domain("security");
    let has_crypto_hash = security_methods.iter().any(|m| &**m == "crypto.hash");
    v.check_bool(
        "security-domain-has-crypto-hash",
        has_crypto_hash,
        "crypto.hash present in security domain",
    );

    let owner_checks: &[(&str, &str, &str)] = &[
        ("crypto.hash", "beardog", "crypto-hash-owner"),
        ("compute.dispatch", "toadstool", "compute-dispatch-owner"),
        ("storage.store", "nestgate", "storage-store-owner"),
        (
            "science.eigensolve",
            "neuralspring",
            "science-eigensolve-owner",
        ),
    ];
    for &(method, expected_owner, check_id) in owner_checks {
        if let Some(entry) = table.route(method) {
            v.check_bool(
                check_id,
                &*entry.owner == expected_owner,
                &format!("{method} owner: {}", entry.owner),
            );
        }
    }
}

fn phase_tier_composition(
    v: &mut ValidationResult,
    table: &crate::composition::neural_routing::NeuralRoutingTable,
) {
    let tower = table.tier_composition(CompositionTier::Tower);
    for (primal, check_id) in &[
        ("beardog", "tower-has-beardog"),
        ("songbird", "tower-has-songbird"),
        ("skunkbat", "tower-has-skunkbat"),
    ] {
        v.check_bool(
            check_id,
            tower.primals.contains(&(*primal).to_owned()),
            &format!("Tower composition includes {primal}"),
        );
    }

    let nest = table.tier_composition(CompositionTier::Nest);
    v.check_bool(
        "nest-has-nestgate",
        nest.primals.contains(&"nestgate".to_owned()),
        "Nest composition includes nestgate",
    );
    v.check_bool(
        "nest-has-provenance",
        nest.primals.contains(&"sweetgrass".to_owned())
            || nest.primals.contains(&"loamspine".to_owned()),
        "Nest composition includes provenance primals",
    );
}

fn phase_composition_patterns(
    v: &mut ValidationResult,
    table: &crate::composition::neural_routing::NeuralRoutingTable,
) {
    let patterns = table.patterns();
    v.check_bool(
        "patterns-registered",
        patterns.len() >= 3,
        &format!("{} composition patterns", patterns.len()),
    );

    let rootpulse = patterns.iter().find(|p| &*p.name == "rootpulse_commit");
    v.check_bool(
        "rootpulse-pattern-exists",
        rootpulse.is_some(),
        "rootpulse_commit pattern registered",
    );
    if let Some(rp) = rootpulse {
        v.check_bool(
            "rootpulse-6-primals",
            rp.primals.len() == 6,
            &format!(
                "rootpulse involves {} primals (TOML-driven)",
                rp.primals.len()
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neural_routing_surface_scenario_passes() {
        let mut v = ValidationResult::new("neural-routing-surface");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(
            v.failed,
            0,
            "neural routing surface: {}/{} checks passed ({} failed)",
            v.passed,
            v.passed + v.failed,
            v.failed
        );
    }
}
