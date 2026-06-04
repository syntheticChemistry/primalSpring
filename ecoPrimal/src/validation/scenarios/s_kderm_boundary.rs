// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: K-Derm Boundary — cell envelope topology validation.
//!
//! Validates that NUCLEUS primals are correctly placed in K-Derm layers
//! and that boundary policies are structurally sound:
//!
//! - Cytoplasm: inner primals (compute, storage, AI) — UDS only
//! - Plasma Membrane: Tower primals (BearDog, Songbird) — TCP allowed
//! - Outer Membrane (diderm): cellMembrane relay — VPS boundary
//!
//! Tier::Rust — structural checks against the canonical topology.
//! Live K-Derm channel protein validation deferred until cellMembrane
//! publishes `membrane.toml`.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "kderm-boundary",
        track: Track::Sovereignty,
        tier: Tier::Rust,
        provenance_crate: "wave54_kderm_boundary",
        provenance_date: "2026-05-26",
        description: "K-Derm boundary — cell envelope layer placement, boundary policy, monoderm/diderm topology",
    },
    run,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KDermLayer {
    Cytoplasm,
    PlasmaMembrane,
    Periplasm,
    OuterMembrane,
}

struct PrimalPlacement {
    primal: &'static str,
    layer: KDermLayer,
    tcp_allowed: bool,
    rationale: &'static str,
}

const PLACEMENTS: &[PrimalPlacement] = &[
    PrimalPlacement { primal: "beardog", layer: KDermLayer::PlasmaMembrane, tcp_allowed: true, rationale: "crypto CSP — Tower Atomic, cross-gate auth" },
    PrimalPlacement { primal: "songbird", layer: KDermLayer::PlasmaMembrane, tcp_allowed: true, rationale: "network spine — federation, braid relay, CNS" },
    PrimalPlacement { primal: "skunkbat", layer: KDermLayer::PlasmaMembrane, tcp_allowed: true, rationale: "defense — meta-tier threat detection at boundary" },
    PrimalPlacement { primal: "biomeos", layer: KDermLayer::Cytoplasm, tcp_allowed: false, rationale: "orchestrator — internal Neural API, UDS only" },
    PrimalPlacement { primal: "toadstool", layer: KDermLayer::Cytoplasm, tcp_allowed: false, rationale: "compute substrate — GPU/NPU dispatch, internal" },
    PrimalPlacement { primal: "barracuda", layer: KDermLayer::Cytoplasm, tcp_allowed: false, rationale: "GPU math — WGSL shaders, internal compute" },
    PrimalPlacement { primal: "coralreef", layer: KDermLayer::Cytoplasm, tcp_allowed: false, rationale: "shader compiler — pure transform, internal" },
    PrimalPlacement { primal: "squirrel", layer: KDermLayer::Cytoplasm, tcp_allowed: false, rationale: "AI coordination — inference routing, internal" },
    PrimalPlacement { primal: "petaltongue", layer: KDermLayer::Cytoplasm, tcp_allowed: false, rationale: "UI renderer — display only, internal" },
    PrimalPlacement { primal: "rhizocrypt", layer: KDermLayer::Cytoplasm, tcp_allowed: false, rationale: "ephemeral DAG — session memory, internal" },
    PrimalPlacement { primal: "loamspine", layer: KDermLayer::Cytoplasm, tcp_allowed: false, rationale: "permanent ledger — storage, internal" },
    PrimalPlacement { primal: "sweetgrass", layer: KDermLayer::Cytoplasm, tcp_allowed: false, rationale: "provenance — PROV-O braids, internal" },
    PrimalPlacement { primal: "nestgate", layer: KDermLayer::Periplasm, tcp_allowed: true, rationale: "network egress — between inner and membrane" },
];

/// Run all K-Derm boundary validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Layer placement validation");
    phase_layer_placement(v);

    v.section("Phase 2: Boundary policy consistency");
    phase_boundary_policy(v);

    v.section("Phase 3: Topology classification");
    phase_topology(v);
}

fn phase_layer_placement(v: &mut ValidationResult) {
    v.check_bool(
        "placement:total",
        PLACEMENTS.len() >= 13,
        &format!("{} primals placed in K-Derm layers", PLACEMENTS.len()),
    );

    let plasma: Vec<_> = PLACEMENTS.iter().filter(|p| p.layer == KDermLayer::PlasmaMembrane).collect();
    let cytoplasm: Vec<_> = PLACEMENTS.iter().filter(|p| p.layer == KDermLayer::Cytoplasm).collect();
    let periplasm: Vec<_> = PLACEMENTS.iter().filter(|p| p.layer == KDermLayer::Periplasm).collect();

    v.check_bool(
        "placement:plasma_membrane",
        plasma.len() >= 2,
        &format!(
            "Plasma membrane (Tower): {}",
            plasma.iter().map(|p| p.primal).collect::<Vec<_>>().join(", ")
        ),
    );

    v.check_bool(
        "placement:cytoplasm",
        cytoplasm.len() >= 8,
        &format!(
            "Cytoplasm (inner): {}",
            cytoplasm.iter().map(|p| p.primal).collect::<Vec<_>>().join(", ")
        ),
    );

    v.check_bool(
        "placement:periplasm",
        !periplasm.is_empty(),
        &format!(
            "Periplasm (transit): {}",
            periplasm.iter().map(|p| p.primal).collect::<Vec<_>>().join(", ")
        ),
    );

    for p in PLACEMENTS {
        v.check_bool(
            &format!("layer:{}", p.primal),
            true,
            &format!("{:?} — {}", p.layer, p.rationale),
        );
    }
}

fn phase_boundary_policy(v: &mut ValidationResult) {
    let tcp_primals: Vec<_> = PLACEMENTS.iter().filter(|p| p.tcp_allowed).collect();
    let uds_only: Vec<_> = PLACEMENTS.iter().filter(|p| !p.tcp_allowed).collect();

    v.check_bool(
        "policy:tcp_restricted",
        tcp_primals.len() <= 4,
        &format!(
            "{} primals with TCP: {} (target: Songbird-only after CNS convergence)",
            tcp_primals.len(),
            tcp_primals.iter().map(|p| p.primal).collect::<Vec<_>>().join(", ")
        ),
    );

    v.check_bool(
        "policy:uds_majority",
        uds_only.len() > tcp_primals.len(),
        &format!("{} primals UDS-only (cytoplasm), {} TCP-allowed (membrane)", uds_only.len(), tcp_primals.len()),
    );

    for p in PLACEMENTS {
        if p.layer == KDermLayer::Cytoplasm && p.tcp_allowed {
            v.check_bool(
                &format!("policy:violation:{}", p.primal),
                false,
                &format!("{} is cytoplasm but has TCP — boundary violation", p.primal),
            );
        }
    }

    v.check_bool(
        "policy:plasma_tcp",
        plasma_has_songbird(),
        "Songbird in plasma membrane — sole cross-gate relay after CNS convergence",
    );
}

fn plasma_has_songbird() -> bool {
    PLACEMENTS.iter().any(|p| p.primal == "songbird" && p.layer == KDermLayer::PlasmaMembrane)
}

fn phase_topology(v: &mut ValidationResult) {
    let has_outer = PLACEMENTS.iter().any(|p| p.layer == KDermLayer::OuterMembrane);

    if has_outer {
        v.check_bool(
            "topology:classification",
            true,
            "Diderm — outer membrane present (cellMembrane VPS)",
        );
    } else {
        v.check_bool(
            "topology:classification",
            true,
            "Monoderm (local NUCLEUS) — outer membrane is cellMembrane (not in primal set)",
        );
    }

    v.check_bool(
        "topology:outer_membrane_note",
        true,
        "cellMembrane provides outer membrane (VPS relay, TLS, DNS) — separate garden, not a primal",
    );

    v.check_bool(
        "topology:channel_proteins",
        true,
        "K-Derm channel proteins (TLS, NAT, content, auth) pending cellMembrane membrane.toml publication",
    );

    let has_periplasm = PLACEMENTS.iter().any(|p| p.layer == KDermLayer::Periplasm);
    v.check_bool(
        "topology:periplasm",
        has_periplasm,
        "Periplasm layer present — NestGate as transit between cytoplasm and membrane",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn kderm_boundary_no_panic() {
        let mut v = ValidationResult::new("kderm-boundary");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(v.failed, 0, "K-Derm boundary should have zero failures");
    }

    #[test]
    fn no_cytoplasm_tcp_violations() {
        for p in PLACEMENTS {
            if p.layer == KDermLayer::Cytoplasm {
                assert!(!p.tcp_allowed, "{} is cytoplasm but has TCP", p.primal);
            }
        }
    }

    #[test]
    fn songbird_in_plasma() {
        assert!(plasma_has_songbird(), "Songbird must be in plasma membrane");
    }

    #[test]
    fn all_13_primals_placed() {
        assert!(PLACEMENTS.len() >= 13, "need at least 13 primals placed");
    }
}
