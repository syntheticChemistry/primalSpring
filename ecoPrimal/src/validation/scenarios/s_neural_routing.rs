// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scenario: Neural Routing Surface — validates the 452-method routing table.
//!
//! Verifies that the `NeuralRoutingTable` correctly maps every registered
//! method to its owner, domain, and composition tier, and that the tier
//! composition plans are structurally valid. This is the foundation for
//! adaptive routing (Layer 4) and learned routing (Layer 5).

use crate::composition::neural_dispatch::NeuralDispatcher;
use crate::composition::neural_routing::{
    canonical_routing_table, CompositionTier,
};
use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Neural routing surface scenario — Tier::Rust structural check.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "neural-routing-surface",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "primalspring_neural_routing",
        provenance_date: "2026-05-22",
        description: "Neural routing table: all 452 methods map to owners, tiers, and composition patterns",
    },
    run,
};

/// Run the neural routing surface validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let table = canonical_routing_table();

    // Phase 1: Method coverage — all registry methods are in the routing table.
    let method_count = table.method_count();
    v.check_bool(
        "routing-table-population",
        method_count >= 450,
        &format!("{method_count} methods loaded (expect 450+)"),
    );

    // Phase 2: Tier coverage — every tier has methods.
    let summary = table.tier_summary();
    for tier_name in &["tower", "node", "nest", "meta", "orchestration"] {
        let count = summary.get(tier_name).copied().unwrap_or(0);
        v.check_bool(
            &format!("tier-{tier_name}-populated"),
            count > 0,
            &format!("{tier_name} tier: {count} methods"),
        );
    }

    // Phase 3: Owner integrity — security methods → beardog, compute → toadstool.
    let security_methods = table.methods_in_domain("security");
    let has_crypto_hash = security_methods.contains(&"crypto.hash".to_owned());
    v.check_bool(
        "security-domain-has-crypto-hash",
        has_crypto_hash,
        "crypto.hash present in security domain",
    );

    if let Some(entry) = table.route("crypto.hash") {
        v.check_bool(
            "crypto-hash-owner",
            entry.owner == "beardog",
            &format!("crypto.hash owner: {}", entry.owner),
        );
    }

    if let Some(entry) = table.route("compute.dispatch") {
        v.check_bool(
            "compute-dispatch-owner",
            entry.owner == "toadstool",
            &format!("compute.dispatch owner: {}", entry.owner),
        );
    }

    if let Some(entry) = table.route("storage.store") {
        v.check_bool(
            "storage-store-owner",
            entry.owner == "nestgate",
            &format!("storage.store owner: {}", entry.owner),
        );
    }

    if let Some(entry) = table.route("science.eigensolve") {
        v.check_bool(
            "science-eigensolve-owner",
            entry.owner == "neuralspring",
            &format!("science.eigensolve owner: {}", entry.owner),
        );
    }

    // Phase 4: Tier composition plans.
    let tower = table.tier_composition(CompositionTier::Tower);
    v.check_bool(
        "tower-has-beardog",
        tower.primals.contains(&"beardog".to_owned()),
        "Tower composition includes beardog",
    );
    v.check_bool(
        "tower-has-songbird",
        tower.primals.contains(&"songbird".to_owned()),
        "Tower composition includes songbird",
    );
    v.check_bool(
        "tower-has-skunkbat",
        tower.primals.contains(&"skunkbat".to_owned()),
        "Tower composition includes skunkbat",
    );

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

    // Phase 5: Composition patterns.
    let patterns = table.patterns();
    v.check_bool(
        "patterns-registered",
        patterns.len() >= 3,
        &format!("{} composition patterns", patterns.len()),
    );

    let rootpulse = patterns.iter().find(|p| p.name == "rootpulse_commit");
    v.check_bool(
        "rootpulse-pattern-exists",
        rootpulse.is_some(),
        "rootpulse_commit pattern registered",
    );
    if let Some(rp) = rootpulse {
        v.check_bool(
            "rootpulse-4-primals",
            rp.primals.len() == 4,
            &format!("rootpulse involves {} primals", rp.primals.len()),
        );
    }

    // Phase 6: NeuralDispatcher status report.
    let dispatcher = NeuralDispatcher::with_table(table);
    let report = dispatcher.status_report();
    v.check_bool(
        "dispatcher-report-methods",
        report["total_methods"].as_u64().unwrap_or(0) >= 450,
        &format!("dispatcher reports {} methods", report["total_methods"]),
    );

    // Phase 7: Tier sum integrity — all methods accounted for.
    let total_in_tiers: usize = summary.values().sum();
    v.check_bool(
        "tier-sum-equals-total",
        total_in_tiers == method_count,
        &format!("tier sum {total_in_tiers} == total {method_count}"),
    );
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
            v.failed, 0,
            "neural routing surface: {}/{} checks passed ({} failed)",
            v.passed, v.passed + v.failed, v.failed
        );
    }
}
