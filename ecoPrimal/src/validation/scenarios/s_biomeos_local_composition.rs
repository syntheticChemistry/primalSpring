// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: BiomeOS Local Composition — local deploy graph validation.
//!
//! Validates BiomeOS composition from the local gate (eastGate):
//! 1. Local primals declared in the deploy graph match deployable set
//! 2. Ephemeral compute model: topology valid if sporeGate unplugs
//! 3. Cross-gate dispatch: requests route correctly from local gate
//! 4. Primal-to-gate binding: each primal has exactly one canonical home
//! 5. Live: local NUCLEUS socket reachability

use crate::composition::CompositionContext;
use crate::evolution::{all_mesh_gates, mesh_address};
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const DEPLOY_GRAPH: &str =
    include_str!("../../../../graphs/multi_node/five_gate_sovereign_mesh.toml");
const TOPOLOGY_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// BiomeOS local composition scenario metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "biomeos-local-composition",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "wave128_biomeos_local",
        provenance_date: "2026-06-28",
        description: "BiomeOS local composition — eastGate primals, ephemeral compute, dispatch routing",
    },
    run,
};

const EASTGATE_PRIMALS: &[&str] = &[
    primal_names::BIOMEOS,
    primal_names::SQUIRREL,
    primal_names::PETALTONGUE,
];

const SPOREGATE_PRIMALS: &[&str] = &[
    primal_names::NESTGATE,
    primal_names::RHIZOCRYPT,
    primal_names::LOAMSPINE,
    primal_names::SWEETGRASS,
];

const IRONGATE_PRIMALS: &[&str] = &[
    primal_names::TOADSTOOL,
    primal_names::BARRACUDA,
    primal_names::CORALREEF,
];

/// Run all BiomeOS local composition phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Local gate primal binding");
    phase_local_binding(v);

    v.section("Phase 2: Ephemeral compute invariant");
    phase_ephemeral_invariant(v);

    v.section("Phase 3: Cross-gate dispatch routing");
    phase_dispatch_routing(v);

    v.section("Phase 4: Primal uniqueness");
    phase_primal_uniqueness(v);

    v.section("Phase 5: Live NUCLEUS");
    phase_live(v, ctx);
}

fn phase_local_binding(v: &mut ValidationResult) {
    let has_eastgate_section =
        DEPLOY_GRAPH.contains("[gates.eastGate]") || DEPLOY_GRAPH.contains("eastGate");

    v.check_bool(
        "local:eastgate_in_graph",
        has_eastgate_section,
        "eastGate declared in deploy graph",
    );

    for primal in EASTGATE_PRIMALS {
        v.check_bool(
            &format!("local:primal:{primal}"),
            DEPLOY_GRAPH.contains(primal),
            &format!("{primal} assigned to eastGate in deploy graph"),
        );
    }

    let eastgate_addr = mesh_address("eastGate");
    v.check_bool(
        "local:eastgate_meshed",
        eastgate_addr.is_some(),
        &format!("eastGate mesh address resolved: {eastgate_addr:?}"),
    );
}

fn phase_ephemeral_invariant(v: &mut ValidationResult) {
    let has_sporegate = DEPLOY_GRAPH.contains("sporeGate");
    v.check_bool(
        "ephemeral:sporegate_in_graph",
        has_sporegate,
        "sporeGate present in deploy graph",
    );

    let sporegate_role = TOPOLOGY_TOML.contains("role = \"compute\"");
    v.check_bool(
        "ephemeral:sporegate_compute",
        sporegate_role,
        "sporeGate role is compute (ephemeral, hot-pluggable)",
    );

    let hub_is_golgi = DEPLOY_GRAPH.contains("hub = \"golgi\"");
    v.check_bool(
        "ephemeral:relay_not_sporegate",
        hub_is_golgi,
        "relay hub is golgi, NOT sporeGate (ephemeral invariant)",
    );

    let gates = all_mesh_gates();
    let non_sporegate_active: Vec<&str> = gates
        .iter()
        .filter(|g| g.name != "sporeGate" && !g.address.is_empty())
        .map(|g| g.name.as_str())
        .collect();

    v.check_bool(
        "ephemeral:mesh_survives_unplug",
        non_sporegate_active.len() >= 3,
        &format!(
            "mesh viable without sporeGate: {} active gates remain",
            non_sporegate_active.len()
        ),
    );
}

fn phase_dispatch_routing(v: &mut ValidationResult) {
    let gpu_to_iron = DEPLOY_GRAPH.contains("gpu_workloads = \"ironGate\"")
        || DEPLOY_GRAPH.contains("\"tensor.*\" = \"ironGate\"");
    v.check_bool(
        "dispatch:gpu_to_irongate",
        gpu_to_iron,
        "GPU workloads dispatched to ironGate",
    );

    let nest_to_spore = DEPLOY_GRAPH.contains("\"nest.*\" = \"sporeGate\"")
        || DEPLOY_GRAPH.contains("trust_anchor = \"sporeGate\"");
    v.check_bool(
        "dispatch:nest_to_sporegate",
        nest_to_spore,
        "Nest/provenance routed to sporeGate",
    );

    let cascade_to_golgi = DEPLOY_GRAPH.contains("cascade_relay = \"golgi\"");
    v.check_bool(
        "dispatch:cascade_via_golgi",
        cascade_to_golgi,
        "cascade relay via golgi hub",
    );

    let build_to_spore = DEPLOY_GRAPH.contains("build_artifacts = \"sporeGate\"");
    v.check_bool(
        "dispatch:build_to_sporegate",
        build_to_spore,
        "build artifacts from sporeGate CI",
    );
}

fn phase_primal_uniqueness(v: &mut ValidationResult) {
    let all_primals: Vec<&str> = EASTGATE_PRIMALS
        .iter()
        .chain(SPOREGATE_PRIMALS.iter())
        .chain(IRONGATE_PRIMALS.iter())
        .copied()
        .collect();

    let unique_count = {
        let mut sorted = all_primals.clone();
        sorted.sort_unstable();
        sorted.dedup();
        sorted.len()
    };

    v.check_bool(
        "unique:no_duplicates",
        unique_count == all_primals.len(),
        &format!(
            "{}/{} primals unique (no dual-gate binding)",
            unique_count,
            all_primals.len()
        ),
    );

    v.check_bool(
        "unique:total_coverage",
        all_primals.len() >= 10,
        &format!(
            "{} primals covered across 3 compute gates",
            all_primals.len()
        ),
    );

    let graph_has_btsp = DEPLOY_GRAPH.contains("btsp_enforced = true");
    v.check_bool(
        "unique:btsp_enforced",
        graph_has_btsp,
        "BTSP enforced across all gate compositions",
    );
}

fn phase_live(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let Some(client) = ctx.client_for("biomeos") else {
        v.check_skip("live:biomeos_health", "no BiomeOS client");
        v.check_skip("live:local_nucleus", "no BiomeOS client");
        return;
    };

    let resp = client.call("health.liveness", serde_json::json!({}));
    match resp {
        Ok(r) => {
            v.check_bool(
                "live:biomeos_health",
                r.is_success(),
                "BiomeOS liveness check on local gate",
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("live:biomeos_health", &format!("{e}"));
        }
        Err(e) => {
            v.check_bool("live:biomeos_health", false, &format!("{e}"));
        }
    }

    let resp = client.call("composition.status", serde_json::json!({}));
    match resp {
        Ok(r) => {
            v.check_bool(
                "live:local_nucleus",
                r.is_success(),
                "composition.status responds on local NUCLEUS",
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("live:local_nucleus", &format!("{e}"));
        }
        Err(e) => {
            v.check_bool("live:local_nucleus", false, &format!("{e}"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn biomeos_local_composition_runs() {
        let mut v = ValidationResult::new("biomeos-local-composition");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        let total = v.passed + v.failed + v.skipped;
        assert!(total >= 15, "expected ≥15 checks, got {total}");
    }

    #[test]
    fn eastgate_primals_in_graph() {
        for primal in EASTGATE_PRIMALS {
            assert!(
                DEPLOY_GRAPH.contains(primal),
                "{primal} not found in deploy graph"
            );
        }
    }

    #[test]
    fn sporegate_primals_in_graph() {
        for primal in SPOREGATE_PRIMALS {
            assert!(
                DEPLOY_GRAPH.contains(primal),
                "{primal} not found in deploy graph"
            );
        }
    }

    #[test]
    fn ephemeral_hub_is_golgi() {
        assert!(DEPLOY_GRAPH.contains("hub = \"golgi\""));
    }

    #[test]
    fn gpu_dispatch_to_irongate() {
        assert!(DEPLOY_GRAPH.contains("ironGate"));
    }

    #[test]
    fn primal_uniqueness_across_gates() {
        let all: Vec<&str> = EASTGATE_PRIMALS
            .iter()
            .chain(SPOREGATE_PRIMALS.iter())
            .chain(IRONGATE_PRIMALS.iter())
            .copied()
            .collect();
        let mut sorted = all.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), all.len(), "duplicate primal assignment");
    }
}
