// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Depot Trust Verify — validates depot trust verification chain via
//! checksums, provenance, and Ed25519 signing pipeline.

use crate::composition::CompositionContext;
use crate::composition::neural_routing::canonical_routing_table;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const PROTO_NUCLEATE: &str = include_str!("../../../../config/proto_nucleate.toml");

/// Depot trust verify scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "depot-trust-verify",
        track: Track::Sovereignty,
        tier: Tier::Rust,
        provenance_crate: "wave138a_depot_trust_verify",
        provenance_date: "2026-07-14",
        description: "Depot trust verify — checksums, provenance, and Ed25519 signing pipeline",
    },
    run,
};

/// Run depot trust verify validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Depot fetch and verify contracts");

    let has_depot_fetch = REGISTRY_TOML.contains("depot.fetch")
        || REGISTRY_TOML.contains("storage.fetch")
        || REGISTRY_TOML.contains("storage.fetch_external");
    v.check_bool(
        "depot:fetch_method",
        has_depot_fetch,
        "depot.fetch or storage.fetch* registered for depot retrieval",
    );

    let has_depot_verify = REGISTRY_TOML.contains("depot.verify")
        || REGISTRY_TOML.contains("certificate.verify")
        || REGISTRY_TOML.contains("crypto.verify_ed25519");
    v.check_bool(
        "depot:verify_method",
        has_depot_verify,
        "depot.verify or certificate.verify or crypto.verify_ed25519 for trust chain",
    );

    v.section("Phase 2: signatures.toml / checksums validation references");

    v.check_bool(
        "depot:checksums_toml",
        PROTO_NUCLEATE.contains("checksums.toml"),
        "proto_nucleate.toml references checksums.toml validation",
    );
    v.check_bool(
        "depot:provenance_toml",
        PROTO_NUCLEATE.contains("provenance.toml"),
        "proto_nucleate.toml references provenance.toml validation",
    );
    v.check_bool(
        "depot:dual_verify",
        PROTO_NUCLEATE.contains("dual_verify = true"),
        "proto_nucleate enables dual-verify (signatures/checksums chain)",
    );

    v.section("Phase 3: Ed25519 signing pipeline");

    let signing_pipeline = [
        "crypto.sign_ed25519",
        "crypto.verify_ed25519",
        "crypto.hash",
        "anchor.verify",
    ];
    for method in signing_pipeline {
        v.check_bool(
            &format!("signing:{}", method.replace('.', "_")),
            REGISTRY_TOML.contains(method),
            &format!("{method} in Ed25519 signing pipeline"),
        );
    }

    let table = canonical_routing_table();
    if let Some(entry) = table.route("crypto.verify_ed25519") {
        v.check_bool(
            "signing:beardog_owner",
            &*entry.owner == primal_names::BEARDOG,
            &format!("crypto.verify_ed25519 → {} (expected bearDog)", entry.owner),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_passes_structural() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "scenario '{}' had {} failures",
            SCENARIO.meta.id, v.failed
        );
    }
}
