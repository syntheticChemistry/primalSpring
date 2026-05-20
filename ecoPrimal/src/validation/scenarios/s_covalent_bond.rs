// SPDX-License-Identifier: AGPL-3.0-or-later
//! Scenario: Covalent Bond — absorbed from exp030.

use crate::bonding::graph_metadata::validate_graph_bonding;
use crate::bonding::{BondType, BondingPolicy, TrustModel};
use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};
use std::path::Path;

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "covalent-bond",
        track: Track::Bonding,
        tier: Tier::Both,
        provenance_crate: "exp030_covalent_bond",
        provenance_date: "2026-05-09",
        description: "Covalent bond — bonding policy, graph metadata, Tower health",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Bond Type Properties");
    phase_bond_type_properties(v);

    v.section("Phase 2: Bonding Policy");
    phase_bonding_policy(v);

    v.section("Phase 3: Graph Metadata");
    phase_graph_metadata(v);

    v.section("Phase 4: Live Discovery + Health");
    phase_live_discovery(v, ctx);

    phase_multi_node_skips(v);
}

fn phase_bond_type_properties(v: &mut ValidationResult) {
    let bond = BondType::Covalent;
    v.check_bool(
        "covalent_description_non_empty",
        !bond.description().is_empty(),
        &format!("BondType::Covalent — {}", bond.description()),
    );
    v.check_bool(
        "covalent_shares_electrons",
        bond.shares_electrons(),
        "Covalent bonds share electrons (Tower state)",
    );
    v.check_bool(
        "covalent_not_metered",
        !bond.is_metered(),
        "Covalent bonds are not metered (cooperative)",
    );
}

fn phase_bonding_policy(v: &mut ValidationResult) {
    let policy = BondingPolicy::covalent_default();
    let policy_errors = policy.validate();
    v.check_bool(
        "covalent_default_policy_valid",
        policy_errors.is_empty(),
        &format!(
            "BondingPolicy::covalent_default() validates cleanly (errors: {})",
            policy_errors.len()
        ),
    );
    v.check_bool(
        "covalent_policy_offers_relay",
        policy.offer_relay,
        "Covalent default policy offers relay to family",
    );
    v.check_bool(
        "covalent_policy_genetic_trust",
        policy.trust_model == TrustModel::GeneticLineage,
        "Covalent policy uses GeneticLineage trust",
    );
}

fn phase_graph_metadata(v: &mut ValidationResult) {
    let graph_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../graphs/multi_node/basement_hpc_covalent.toml");
    v.check_or_skip(
        "covalent_graph_metadata",
        graph_path.exists().then_some(&()),
        "basement_hpc_covalent.toml not found",
        |&(), v| {
            let meta = validate_graph_bonding(&graph_path);
            v.check_bool(
                "graph_bond_type_is_covalent",
                meta.internal_bond_type == Some(BondType::Covalent),
                &format!("graph internal_bond_type = {:?}", meta.internal_bond_type),
            );
            v.check_bool(
                "graph_trust_is_genetic",
                meta.trust_model == Some(TrustModel::GeneticLineage),
                &format!("graph trust_model = {:?}", meta.trust_model),
            );
            v.check_bool(
                "graph_no_bonding_issues",
                meta.issues.is_empty(),
                &format!("graph bonding issues: {:?}", meta.issues),
            );
        },
    );
}

fn phase_live_discovery(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    for cap in ["security", "discovery"] {
        if !ctx.has_capability(cap) {
            v.check_skip(
                &format!("health_liveness_{cap}"),
                &format!("capability {cap} not in composition context"),
            );
            continue;
        }
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
            Err(e) if e.is_connection_error() => v.check_skip(
                &format!("health_liveness_{cap}"),
                &format!("{cap} not reachable: {e}"),
            ),
            Err(e) => v.check_bool(
                &format!("health_liveness_{cap}"),
                false,
                &format!("error: {e}"),
            ),
        }
    }
}

fn phase_multi_node_skips(v: &mut ValidationResult) {
    v.check_skip("family_seed_sharing", "needs 2 live NUCLEUS instances");
    v.check_skip(
        "mesh_auto_discover_second_gate",
        "needs live Songbird mesh with 2+ gates",
    );
    // cross_gate_capability_call: covered by s_cross_gate_capability_call (CM-3, Wave 29)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn covalent_bond_pass() {
        let mut v = ValidationResult::new("covalent-bond");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "covalent bond scenario had {} failures (use --nocapture for details)",
            v.failed
        );
    }
}
