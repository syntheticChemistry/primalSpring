// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Drawbridge Weak Bond Ingestion — validates the data ingestion
//! pipeline from external sources through drawbridge weak bonds into NestGate CAS.
//!
//! Wave 138a architecture:
//! - External data (USGS, ArcGIS, NF Portal, Pluto.bio, LINCS) enters via
//!   drawbridge weak bonds — minimal trust, maximum provenance
//! - Data lands in NestGate CAS with BLAKE3 content-addressing + Loam Certificates
//! - Downstream primals consume data via capability.call (zero trust, full lineage)
//! - Weak bonds are the universal data ingestion point for all protoKarya projects
//!
//! Validation phases:
//! 1. Bond lifecycle methods: propose → accept → meter
//! 2. Content pipeline: storage.put/get → content.resolve → BLAKE3 hash chain
//! 3. Provenance chain: certificate → anchor → proof → attribution
//! 4. Cross-project availability: data ingested once, consumed many

use crate::composition::neural_routing::canonical_routing_table;
use crate::composition::CompositionContext;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Drawbridge weak bond ingestion scenario metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "drawbridge-weak-bond-ingestion",
        track: Track::Security,
        tier: Tier::Rust,
        provenance_crate: "wave138a_weak_bond_ingestion",
        provenance_date: "2026-07-14",
        description:
            "Drawbridge weak bond ingestion — external data → drawbridge → NestGate CAS → mesh capabilities",
    },
    run,
};

/// Bond lifecycle methods (bonding domain).
const BOND_LIFECYCLE: &[&str] = &[
    "bonding.propose",
    "bonding.accept",
    "bonding.status",
    "bonding.terminate",
];

/// Content storage pipeline methods consumed after ingestion.
const CONTENT_PIPELINE: &[&str] = &[
    "storage.store",
    "storage.fetch",
    "content.put",
    "content.get",
    "content.resolve",
    "content.hash",
];

/// Provenance methods ensuring data lineage.
const PROVENANCE_CHAIN: &[&str] = &[
    "certificate.mint",
    "certificate.verify",
    "provenance.commit",
    "provenance.verify",
    "anchor.publish",
];

/// Run all drawbridge weak bond ingestion validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Bond lifecycle methods");
    phase_bond_lifecycle(v);

    v.section("Phase 2: Content ingestion pipeline");
    phase_content_pipeline(v);

    v.section("Phase 3: Provenance chain for ingested data");
    phase_provenance_chain(v);

    v.section("Phase 4: Single-ingest, multi-consume topology");
    phase_multi_consume(v);
}

fn phase_bond_lifecycle(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    for method in BOND_LIFECYCLE {
        let routed = table.route(method).is_some();
        v.check_bool(
            &format!("weakbond:lifecycle_{}", method.replace('.', "_")),
            routed,
            &format!("`{method}` routable (bond lifecycle)"),
        );
    }

    let bonding_section = REGISTRY_TOML.contains("[bonding]");
    v.check_bool(
        "weakbond:bonding_domain",
        bonding_section,
        "[bonding] domain present in capability_registry",
    );
}

fn phase_content_pipeline(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    for method in CONTENT_PIPELINE {
        let route = table.route(method);
        let routed = route.is_some();
        let owner = route.map_or("UNROUTED", |r| r.owner.as_ref());
        v.check_bool(
            &format!("weakbond:pipeline_{}", method.replace('.', "_")),
            routed,
            &format!("`{method}` → {owner}"),
        );
    }

    let nestgate_owns_content = table
        .route("content.put")
        .is_some_and(|r| r.owner.as_ref() == primal_names::NESTGATE);
    v.check_bool(
        "weakbond:nestgate_owns_content",
        nestgate_owns_content,
        "NestGate owns content.put (CAS authority)",
    );

    let nestgate_owns_storage = table
        .route("storage.store")
        .is_some_and(|r| r.owner.as_ref() == primal_names::NESTGATE);
    v.check_bool(
        "weakbond:nestgate_owns_storage",
        nestgate_owns_storage,
        "NestGate owns storage.store (blob authority)",
    );
}

fn phase_provenance_chain(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    for method in PROVENANCE_CHAIN {
        let route = table.route(method);
        let routed = route.is_some();
        v.check_bool(
            &format!("weakbond:provenance_{}", method.replace('.', "_")),
            routed,
            &format!("`{method}` routable (data provenance)"),
        );
    }

    let has_certificate = REGISTRY_TOML.contains("[certificate]");
    let has_provenance = REGISTRY_TOML.contains("[provenance]");
    v.check_bool(
        "weakbond:certificate_domain",
        has_certificate,
        "[certificate] domain present (Loam Certificates)",
    );
    v.check_bool(
        "weakbond:provenance_domain",
        has_provenance,
        "[provenance] domain present (data lineage)",
    );
}

fn phase_multi_consume(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let content_get = table.route("content.get");
    let storage_fetch = table.route("storage.fetch");

    let both_nestgate = content_get
        .is_some_and(|r| r.owner.as_ref() == primal_names::NESTGATE)
        && storage_fetch.is_some_and(|r| r.owner.as_ref() == primal_names::NESTGATE);

    v.check_bool(
        "weakbond:single_authority",
        both_nestgate,
        "content.get + storage.fetch both NestGate — single CAS, multi-consumer",
    );

    let compute_method = table.route("compute.submit");
    let viz_method = table.route("visualization.render");

    let has_downstream = compute_method.is_some() || viz_method.is_some();
    v.check_bool(
        "weakbond:downstream_consumers",
        has_downstream,
        "downstream consumers (compute/viz) can access ingested data",
    );
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
            "scenario '{}' had {} failures (passed={})",
            SCENARIO.meta.id, v.failed, v.passed
        );
    }

    #[test]
    fn bond_lifecycle_registered() {
        let table = canonical_routing_table();
        for method in BOND_LIFECYCLE {
            assert!(
                table.route(method).is_some(),
                "bond lifecycle method `{method}` not routable"
            );
        }
    }

    #[test]
    fn nestgate_is_cas_authority() {
        let table = canonical_routing_table();
        let content = table.route("content.put");
        assert!(content.is_some(), "content.put not routed");
        assert_eq!(
            content.map(|r| r.owner.as_ref().to_string()),
            Some(primal_names::NESTGATE.to_string()),
            "NestGate must own content.put"
        );
    }
}
