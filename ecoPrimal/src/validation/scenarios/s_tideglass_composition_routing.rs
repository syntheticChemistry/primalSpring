// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: tideGlass Composition Routing — validates the deploy graph and
//! compute pipeline for the sovereign GPS platform.
//!
//! tideGlass (Gene Perturbation Simulator) requires:
//! - barraCuda for linear algebra and statistical computation
//! - petalTongue for chart/visualization rendering
//! - songBird for drawbridge bonds (LINCS L1000, GEO, ChEMBL, NF Data Portal)
//! - nestGate for data persistence

use crate::composition::CompositionContext;
use crate::composition::neural_routing::canonical_routing_table;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const DRAWBRIDGE_BONDS: &str = include_str!("../../../../config/drawbridge_bonds.toml");
const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tideglass-composition-routing",
        track: Track::Lifecycle,
        tier: Tier::Rust,
        provenance_crate: "wave140a_tideglass_routing",
        provenance_date: "2026-07-15",
        description: "tideGlass composition routing — GPS deploy graph + compute pipeline",
    },
    run,
};

pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Compute pipeline (barraCuda math)");
    phase_compute_pipeline(v);

    v.section("Phase 2: Visualization pipeline (petalTongue)");
    phase_visualization(v);

    v.section("Phase 3: Science data bonds (external APIs)");
    phase_science_bonds(v);

    v.section("Phase 4: Deploy graph structural requirements");
    phase_deploy_graph(v);

    v.section("Phase 5: Manifest registration");
    phase_manifest(v);
}

fn phase_compute_pipeline(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let compute_methods = [
        ("tensor.matmul", "Matrix multiplication for perturbation models"),
        ("math.matvec", "Matrix-vector ops for gene expression"),
        ("math.stats", "Statistical computation for gene-gene interactions"),
    ];

    for (method, desc) in compute_methods {
        let routed = table.route(method).is_some();
        v.check_bool(
            &format!("compute:{}", method.replace('.', "_")),
            routed,
            &format!("{method} routable ({desc})"),
        );
    }

    if let Some(entry) = table.route("tensor.matmul") {
        v.check_bool(
            "compute:barracuda_owner",
            &*entry.owner == primal_names::BARRACUDA,
            &format!("tensor.matmul → {} (expected barraCuda)", entry.owner),
        );
    }
}

fn phase_visualization(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let viz_methods = [
        "visualization.render",
        "visualization.render.graph",
        "visualization.chart",
    ];

    let mut viz_routed = false;
    for method in viz_methods {
        if let Some(entry) = table.route(method) {
            if &*entry.owner == primal_names::PETALTONGUE {
                viz_routed = true;
                break;
            }
        }
    }
    v.check_bool(
        "viz:petaltongue_render",
        viz_routed,
        "Visualization rendering routed to petalTongue",
    );

    let has_chart_types = REGISTRY_TOML.contains("chart")
        || REGISTRY_TOML.contains("render")
        || REGISTRY_TOML.contains("visualization");
    v.check_bool(
        "viz:chart_types",
        has_chart_types,
        "Chart/render/visualization capabilities registered",
    );
}

fn phase_science_bonds(v: &mut ValidationResult) {
    let science_hosts = [
        "pubchem.ncbi.nlm.nih.gov",
        "eutils.ncbi.nlm.nih.gov",
        "rest.uniprot.org",
    ];

    for host in science_hosts {
        let declared = DRAWBRIDGE_BONDS.contains(host);
        v.check_bool(
            &format!("bond:{}", host.split('.').next().unwrap_or("unknown")),
            declared,
            &format!("{host} declared in drawbridge_bonds (science API)"),
        );
    }

    let proto_consumer = DRAWBRIDGE_BONDS.contains("\"protoKarya\"")
        || DRAWBRIDGE_BONDS.contains("\"tideGlass\"");
    v.check_bool(
        "bond:protokarya_consumer",
        proto_consumer,
        "protoKarya or tideGlass registered as bond consumer",
    );
}

fn phase_deploy_graph(v: &mut ValidationResult) {
    let required_primals = [
        primal_names::BARRACUDA,
        primal_names::PETALTONGUE,
        primal_names::SONGBIRD,
        primal_names::NESTGATE,
        primal_names::BEARDOG,
    ];

    for primal in required_primals {
        let in_registry = REGISTRY_TOML.contains(primal);
        v.check_bool(
            &format!("graph:{primal}"),
            in_registry,
            &format!("{primal} available in capability registry (tideGlass dep)"),
        );
    }

    let table = canonical_routing_table();
    let has_storage = table.route("storage.store").is_some();
    let has_compute = table.route("math.matmul").is_some()
        || table.route("compute.dispatch").is_some();
    v.check_bool(
        "graph:storage_compute_pair",
        has_storage && has_compute,
        "Both storage and compute capabilities routable (tideGlass core deps)",
    );
}

fn phase_manifest(v: &mut ValidationResult) {
    let manifest = include_str!("../../../../../../infra/wateringHole/ecosystem_manifest.toml");

    v.check_bool(
        "manifest:tideglass_registered",
        manifest.contains("[repos.tideGlass]"),
        "tideGlass registered in ecosystem_manifest.toml",
    );

    v.check_bool(
        "manifest:tideglass_protist",
        manifest.contains("tideGlass") && manifest.contains("protist"),
        "tideGlass categorized as protist in manifest",
    );

    v.check_bool(
        "manifest:composition_url",
        manifest.contains("tideglass.primals.eco"),
        "tideGlass composition URL declared in manifest",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenario_passes_structural() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
        assert_eq!(v.failed, 0, "scenario '{}' had {} failures", SCENARIO.meta.id, v.failed);
    }
}
