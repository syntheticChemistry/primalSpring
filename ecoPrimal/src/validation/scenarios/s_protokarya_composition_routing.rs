// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: protoKarya Composition Routing — validates that live compositions
//! (footPrint, tideGlass, JupyterHub) can route through the drawbridge → mesh
//! → capability pipeline.
//!
//! Wave 138a architecture:
//! - Each protoKarya project is a composition consuming primal capabilities
//! - Wildcard DNS (`*.primals.eco`) routes to Caddy on golgi
//! - songBird mesh/drawbridge dispatches to owning primals
//! - Data feeds register as drawbridge weak bonds → NestGate CAS
//! - Cross-feeding: projects share data via capability.call
//!
//! This scenario validates the structural prerequisites:
//! 1. Required capability domains exist (content, storage, compute, visualization)
//! 2. Routing: key methods resolve to correct owning primals
//! 3. Composition tiers: Tower, Node, Nest cover protoKarya needs
//! 4. Cross-feed topology: method ownership spans multiple primals

use crate::composition::neural_routing::canonical_routing_table;
use crate::composition::CompositionContext;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// protoKarya composition routing scenario metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "protokarya-composition-routing",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "wave138a_protokarya_routing",
        provenance_date: "2026-07-14",
        description:
            "protoKarya composition routing — live compositions consume capabilities via drawbridge mesh",
    },
    run,
};

/// Capabilities that protoKarya projects depend on.
const FOOTPRINT_DEPS: &[&str] = &["content.get", "content.put", "storage.fetch"];
const TIDEGLASS_DEPS: &[&str] = &["compute.submit", "math.matvec", "content.resolve"];
const JUPYTER_DEPS: &[&str] = &["compute.submit", "jupyter.kernel.spawn", "jupyter.health"];

/// Primals that must own capabilities consumed by protoKarya compositions.
const REQUIRED_OWNERS: &[&str] = &[
    primal_names::NESTGATE,
    primal_names::BARRACUDA,
    primal_names::TOADSTOOL,
];

/// Run all protoKarya composition routing validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Required domains for protoKarya");
    phase_required_domains(v);

    v.section("Phase 2: Capability routing for project dependencies");
    phase_capability_routing(v);

    v.section("Phase 3: Cross-feed topology");
    phase_cross_feed(v);

    v.section("Phase 4: Composition tier coverage");
    phase_tier_coverage(v);
}

fn phase_required_domains(v: &mut ValidationResult) {
    let required_domains = ["content", "storage", "compute", "visualization", "jupyter"];

    for domain in &required_domains {
        let section_marker = format!("[{domain}]");
        let present = REGISTRY_TOML.contains(&section_marker);
        v.check_bool(
            &format!("protokarya:domain_{domain}"),
            present,
            &format!("[{domain}] domain registered in capability_registry"),
        );
    }

    let http_proxy = REGISTRY_TOML.contains("http.proxy");
    v.check_bool(
        "protokarya:http_proxy_method",
        http_proxy,
        "http.proxy method registered (drawbridge routing entry point)",
    );
}

fn phase_capability_routing(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    for method in FOOTPRINT_DEPS {
        let routed = table.route(method).is_some();
        v.check_bool(
            &format!("protokarya:footprint_{}", method.replace('.', "_")),
            routed,
            &format!("footPrint dependency `{method}` routable"),
        );
    }

    for method in TIDEGLASS_DEPS {
        let routed = table.route(method).is_some();
        v.check_bool(
            &format!("protokarya:tideglass_{}", method.replace('.', "_")),
            routed,
            &format!("tideGlass dependency `{method}` routable"),
        );
    }

    for method in JUPYTER_DEPS {
        let routed = table.route(method).is_some();
        v.check_bool(
            &format!("protokarya:jupyter_{}", method.replace('.', "_")),
            routed,
            &format!("JupyterHub dependency `{method}` routable"),
        );
    }
}

fn phase_cross_feed(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let mut owners_seen = std::collections::HashSet::new();
    let all_deps: Vec<&str> = FOOTPRINT_DEPS
        .iter()
        .chain(TIDEGLASS_DEPS.iter())
        .chain(JUPYTER_DEPS.iter())
        .copied()
        .collect();

    for method in &all_deps {
        if let Some(route) = table.route(method) {
            owners_seen.insert(route.owner.as_ref().to_string());
        }
    }

    v.check_bool(
        "protokarya:cross_feed_multi_primal",
        owners_seen.len() >= 2,
        &format!(
            "protoKarya cross-feeds span {} primals (need ≥2 for data sharing)",
            owners_seen.len()
        ),
    );

    for owner in REQUIRED_OWNERS {
        let present = owners_seen.contains(*owner);
        v.check_bool(
            &format!("protokarya:owner_{owner}"),
            present,
            &format!("{owner} owns protoKarya-required capabilities"),
        );
    }
}

fn phase_tier_coverage(v: &mut ValidationResult) {
    let has_tower = REGISTRY_TOML.contains("[compositions.tower]");
    let has_node = REGISTRY_TOML.contains("[compositions.node]");
    let has_nest = REGISTRY_TOML.contains("[compositions.nest]");

    v.check_bool(
        "protokarya:tier_tower",
        has_tower,
        "compositions.tower tier defined (WAN gateway layer)",
    );
    v.check_bool(
        "protokarya:tier_node",
        has_node,
        "compositions.node tier defined (compute layer)",
    );
    v.check_bool(
        "protokarya:tier_nest",
        has_nest,
        "compositions.nest tier defined (storage/CAS layer)",
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
    fn required_domains_present() {
        for domain in &["content", "storage", "compute", "visualization", "jupyter"] {
            assert!(
                REGISTRY_TOML.contains(&format!("[{domain}]")),
                "missing [{domain}] in capability_registry"
            );
        }
    }

    #[test]
    fn cross_feed_spans_multiple_primals() {
        let table = canonical_routing_table();
        let mut owners = std::collections::HashSet::new();
        for method in FOOTPRINT_DEPS.iter().chain(TIDEGLASS_DEPS.iter()) {
            if let Some(route) = table.route(method) {
                owners.insert(route.owner.as_ref().to_string());
            }
        }
        assert!(
            owners.len() >= 2,
            "cross-feed should span ≥2 primals, got {}: {:?}",
            owners.len(),
            owners
        );
    }
}
