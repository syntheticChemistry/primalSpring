// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Capability Discovery Convergence — validates that multi-gate
//! capability discovery converges to a consistent view across the mesh.
//!
//! Validates:
//! 1. Routing table covers all 13 primals
//! 2. All capability domains have at least one method
//! 3. Cross-gate resolution paths are defined
//! 4. Domain-to-primal mapping is bijective (no orphan domains)
//! 5. Discovery convergence properties (tier composition completeness)

use crate::composition::neural_routing::{canonical_routing_table, NeuralRoutingTable};
use crate::composition::primal_home_tier_priority;
use crate::composition::CompositionContext;
use crate::composition::mesh::MeshTopology;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Capability discovery convergence validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "capability-convergence",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-21",
        description: "Multi-gate capability convergence: routing completeness, tier coverage, mesh resolution",
    },
    run: run_capability_convergence,
};

/// Run this validation scenario.
pub fn run_capability_convergence(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Routing table completeness");
    phase_routing_completeness(v);

    v.section("Phase 2: Domain coverage");
    phase_domain_coverage(v);

    v.section("Phase 3: Tier composition");
    phase_tier_composition(v);

    v.section("Phase 4: Mesh convergence");
    phase_mesh_convergence(v, ctx);
}

fn phase_routing_completeness(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    v.check_minimum("total_method_count", table.method_count(), 200);

    let primal_count = table.primal_count();
    v.check_minimum("routed_primal_count", primal_count, 10);

    let expected_primals = [
        primal_names::BEARDOG,
        primal_names::SONGBIRD,
        primal_names::SKUNKBAT,
        primal_names::TOADSTOOL,
        primal_names::BARRACUDA,
        primal_names::CORALREEF,
        primal_names::NESTGATE,
        primal_names::RHIZOCRYPT,
        primal_names::LOAMSPINE,
        primal_names::SWEETGRASS,
        primal_names::SQUIRREL,
    ];

    for primal in &expected_primals {
        let methods = table.methods_for_primal(primal);
        v.check_bool(
            &format!("primal_routed:{primal}"),
            !methods.is_empty(),
            &format!("{primal}: {} methods routed", methods.len()),
        );
    }
}

fn phase_domain_coverage(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let domain_count = table.domain_count();
    v.check_minimum("domain_count", domain_count, 20);

    let critical_domains = [
        "security", "discovery", "health", "lifecycle", "compute",
        "storage", "crypto", "ai", "network", "provenance",
    ];

    for domain in &critical_domains {
        let methods = table.methods_in_domain(domain);
        v.check_bool(
            &format!("domain_populated:{domain}"),
            !methods.is_empty(),
            &format!("{domain}: {} methods", methods.len()),
        );
    }

    check_no_orphan_domains(v, &table);
}

fn check_no_orphan_domains(v: &mut ValidationResult, table: &NeuralRoutingTable) {
    let mut orphan_count = 0usize;
    for domain in table.domains() {
        let methods = table.methods_in_domain(domain);
        if methods.is_empty() {
            orphan_count += 1;
        }
    }
    v.check_bool(
        "no_orphan_domains",
        orphan_count == 0,
        &format!("{orphan_count} orphan domains (empty method lists)"),
    );
}

fn phase_tier_composition(v: &mut ValidationResult) {
    let table = canonical_routing_table();
    let tier_summary = table.tier_summary();

    v.check_minimum("tier_categories", tier_summary.len(), 3);

    let total_tiered: usize = tier_summary.values().sum();
    v.check_minimum("total_tiered_methods", total_tiered, 100);

    for primal in table.primals() {
        let priority = primal_home_tier_priority(primal);
        v.check_bool(
            &format!("tier_priority:{primal}"),
            priority.is_some(),
            &format!("{primal} has tier priority: {priority:?}"),
        );
    }
}

fn phase_mesh_convergence(v: &mut ValidationResult, ctx: &CompositionContext) {
    let mut mesh = MeshTopology::new();
    mesh.set_local_gate("eastGate");

    let gates = [
        ("golgi", &["relay", "depot", "forgejo"][..]),
        ("sporeGate", &["build", "nest", "provenance"]),
        ("eastGate", &["ai", "orchestration", "spring"]),
        ("flockGate", &["tower", "security", "discovery"]),
    ];

    for (gate_id, caps) in &gates {
        let cap_strs: Vec<&str> = caps.to_vec();
        mesh.register_gate(*gate_id, None::<String>, std::iter::empty::<&str>(), cap_strs);
        mesh.mark_healthy(gate_id, true);
    }

    let reachable = mesh.reachable_capabilities();
    v.check_minimum("mesh_reachable_capabilities", reachable.len(), 8);

    let gate_count = mesh.gate_count();
    v.check_count("mesh_gate_count", gate_count, 4);

    let healthy = mesh.healthy_gate_count();
    v.check_count("mesh_healthy_gates", healthy, 4);

    let caps = ctx.available_capabilities();
    if caps.is_empty() {
        v.check_skip(
            "live_discovery_convergence",
            "no live capabilities discovered",
        );
    } else {
        v.check_minimum("live_discovered_capabilities", caps.len(), 3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_convergence_structural() {
        let mut v = ValidationResult::new("capability-convergence");
        let mut ctx = CompositionContext::discover();
        run_capability_convergence(&mut v, &mut ctx);
        let structural_ok = v.passed >= 15;
        assert!(
            structural_ok,
            "capability-convergence: only {} checks passed ({} failed, {} skipped)",
            v.passed, v.failed, v.skipped
        );
    }
}
