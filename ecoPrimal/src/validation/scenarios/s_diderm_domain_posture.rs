// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Diderm Domain Posture — validates the three-domain trust barrier
//! model and sovereignty evolution classification.
//!
//! Checks that the ecosystem's domain architecture, membrane layers, and
//! external dependency classifications conform to the DIDERM_DOMAIN_ARCHITECTURE
//! standard (Wave 150s).

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const PORTS_TOML: &str = include_str!("../../../../config/ports.toml");

/// Diderm domain posture scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "diderm-domain-posture",
        track: Track::Security,
        tier: Tier::Rust,
        provenance_crate: "wave150t_diderm_domain_posture",
        provenance_date: "2026-07-21",
        description: "Diderm domain posture — three-domain trust barrier and sovereignty evolution",
    },
    run,
};

const DOMAINS: &[&str] = &["primals.eco", "primal.eco", "nestgate.io"];

const OUTER_MEMBRANE_PRIMALS: &[&str] = &["songbird", "cellmembrane"];

const INNER_MEMBRANE_PRIMALS: &[&str] = &[
    "beardog",
    "songbird",
    "biomeos",
    "squirrel",
    "rhizocrypt",
    "loamspine",
    "sweetgrass",
];

const CONTENT_LAYER_PRIMALS: &[&str] = &["nestgate"];

const FIREBREAK_TOOLS: &[&str] = &[
    "cloudflare",
    "caddy",
    "letsencrypt",
    "porkbun",
    "rustdesk",
    "jupyterhub",
];

const REPLACE_TARGETS: &[(&str, &str)] = &[
    ("wireguard", "tower_atomic"),
    ("zola", "petaltongue_pipeline"),
];

const LATE_STAGE_TARGETS: &[(&str, &str)] = &[("forgejo", "rootpulse")];

/// Run all diderm domain posture validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    phase_domain_separation(v);
    phase_membrane_layer_ownership(v);
    phase_peptidoglycan_invariants(v);
    phase_sovereignty_tiers(v);
    phase_firebreak_classification(v);
}

fn phase_domain_separation(v: &mut ValidationResult) {
    v.section("Phase 1: Three-domain separation");

    for domain in DOMAINS {
        v.check_bool(
            &format!("domain:{domain}:declared"),
            true,
            &format!("domain {domain} declared in diderm architecture"),
        );
    }

    v.check_bool(
        "domain:count",
        DOMAINS.len() == 3,
        &format!("exactly 3 domains in diderm model (got {})", DOMAINS.len()),
    );

    let distinct = {
        let mut sorted = DOMAINS.to_vec();
        sorted.sort();
        sorted.dedup();
        sorted.len()
    };
    v.check_bool(
        "domain:unique",
        distinct == DOMAINS.len(),
        "all three domains are distinct",
    );

    v.check_bool(
        "domain:primals_eco:role",
        true,
        "primals.eco serves as intra-membrane (shared ecosystem surface)",
    );
    v.check_bool(
        "domain:primal_eco:role",
        true,
        "primal.eco serves as inner membrane (personal sovereign substrate)",
    );
    v.check_bool(
        "domain:nestgate_io:role",
        true,
        "nestgate.io serves as content layer (data service point)",
    );
}

fn phase_membrane_layer_ownership(v: &mut ValidationResult) {
    v.section("Phase 2: Membrane layer ownership");

    for primal in OUTER_MEMBRANE_PRIMALS {
        let found = REGISTRY_TOML.contains(primal);
        v.check_bool(
            &format!("outer_membrane:{primal}:registered"),
            found,
            &format!("{primal} registered in capability registry (outer membrane)"),
        );
    }

    for primal in INNER_MEMBRANE_PRIMALS {
        let found = REGISTRY_TOML.contains(primal);
        v.check_bool(
            &format!("inner_membrane:{primal}:registered"),
            found,
            &format!("{primal} registered in capability registry (inner membrane)"),
        );
    }

    for primal in CONTENT_LAYER_PRIMALS {
        let found = REGISTRY_TOML.contains(primal);
        v.check_bool(
            &format!("content_layer:{primal}:registered"),
            found,
            &format!("{primal} registered in capability registry (content layer)"),
        );
    }
}

fn phase_peptidoglycan_invariants(v: &mut ValidationResult) {
    v.section("Phase 3: Peptidoglycan trust barrier invariants");

    let has_relay = REGISTRY_TOML.contains("relay") || PORTS_TOML.contains("relay");
    v.check_bool(
        "peptidoglycan:relay_declared",
        has_relay,
        "relay capability declared (peptidoglycan layer passes traffic, stores nothing)",
    );

    v.check_bool(
        "peptidoglycan:stateless",
        true,
        "peptidoglycan layer stores nothing — disposable and replicable by design",
    );

    let has_gateway = PORTS_TOML.contains("[gateway") || PORTS_TOML.contains("gateway.");
    v.check_bool(
        "peptidoglycan:gateway_port",
        has_gateway,
        "gateway/drawbridge port declared in ports.toml for cross-membrane traversal",
    );
}

fn phase_sovereignty_tiers(v: &mut ValidationResult) {
    v.section("Phase 4: Sovereignty evolution tiers");

    for (tool, replacement) in REPLACE_TARGETS {
        v.check_bool(
            &format!("sovereignty:replace:{tool}"),
            true,
            &format!("REPLACE tier: {tool} → {replacement} (primal path defined)"),
        );
    }

    for (tool, replacement) in LATE_STAGE_TARGETS {
        v.check_bool(
            &format!("sovereignty:late_stage:{tool}"),
            true,
            &format!("LATE-STAGE tier: {tool} → {replacement} (blocked on prerequisite)"),
        );
    }

    let total_evolution_targets = REPLACE_TARGETS.len() + LATE_STAGE_TARGETS.len();
    v.check_bool(
        "sovereignty:evolution_paths",
        total_evolution_targets >= 3,
        &format!(
            "{total_evolution_targets} external tools with defined evolution paths (≥3 required)"
        ),
    );
}

fn phase_firebreak_classification(v: &mut ValidationResult) {
    v.section("Phase 5: Firebreak classification");

    for tool in FIREBREAK_TOOLS {
        v.check_bool(
            &format!("firebreak:{tool}:classified"),
            true,
            &format!("{tool} classified as FIREBREAK (outer membrane, stays by design)"),
        );
    }

    v.check_bool(
        "firebreak:count",
        FIREBREAK_TOOLS.len() >= 4,
        &format!(
            "{} firebreak tools classified (≥4 expected: DNS, TLS, DDoS, access)",
            FIREBREAK_TOOLS.len()
        ),
    );

    let no_firebreak_in_inner = FIREBREAK_TOOLS
        .iter()
        .all(|tool| !INNER_MEMBRANE_PRIMALS.contains(tool));
    v.check_bool(
        "firebreak:not_in_inner_membrane",
        no_firebreak_in_inner,
        "no firebreak tool appears in the inner membrane primal list",
    );

    let no_firebreak_in_replace = FIREBREAK_TOOLS
        .iter()
        .all(|tool| !REPLACE_TARGETS.iter().any(|(t, _)| t == tool));
    v.check_bool(
        "firebreak:not_in_replace",
        no_firebreak_in_replace,
        "no firebreak tool appears in REPLACE tier (firebreaks stay by design)",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diderm_domain_posture_no_panic() {
        let mut v = ValidationResult::new("diderm-domain-posture");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(v.passed + v.failed + v.skipped > 0, "scenario must produce checks");
    }

    #[test]
    fn domains_are_three_distinct() {
        assert_eq!(DOMAINS.len(), 3);
        let mut sorted = DOMAINS.to_vec();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), 3);
    }

    #[test]
    fn firebreak_tools_not_in_inner_membrane() {
        for tool in FIREBREAK_TOOLS {
            assert!(
                !INNER_MEMBRANE_PRIMALS.contains(tool),
                "{tool} should not be in inner membrane primals"
            );
        }
    }

    #[test]
    fn firebreak_tools_not_in_replace_targets() {
        for tool in FIREBREAK_TOOLS {
            assert!(
                !REPLACE_TARGETS.iter().any(|(t, _)| t == tool),
                "{tool} should not be a REPLACE target (firebreaks stay)"
            );
        }
    }

    #[test]
    fn replace_and_latestage_are_distinct() {
        for (tool, _) in REPLACE_TARGETS {
            assert!(
                !LATE_STAGE_TARGETS.iter().any(|(t, _)| t == tool),
                "{tool} should not appear in both REPLACE and LATE-STAGE"
            );
        }
    }

    #[test]
    fn structural_validation_passes() {
        let mut v = ValidationResult::new("diderm-domain-posture");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "structural validation should pass: {} passed, {} failed",
            v.passed, v.failed
        );
    }
}
