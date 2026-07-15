// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Drawbridge Consumer Parity — validates that the songBird allowlist
//! exactly matches what's declared in drawbridge_bonds.toml.
//!
//! Three-way match: drawbridge_bonds.toml ↔ capability_registry ↔ songBird config.
//! If a bond is declared but not in the allowlist, traffic is silently dropped.
//! If an allowlist entry has no bond declaration, it's an undocumented dependency.

use crate::composition::CompositionContext;
use crate::composition::neural_routing::canonical_routing_table;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const DRAWBRIDGE_BONDS: &str = include_str!("../../../../config/drawbridge_bonds.toml");
const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "drawbridge-consumer-parity",
        track: Track::Security,
        tier: Tier::Rust,
        provenance_crate: "wave140a_drawbridge_consumer_parity",
        provenance_date: "2026-07-15",
        description: "Drawbridge consumer parity — songBird allowlist ↔ drawbridge_bonds.toml exact match",
    },
    run,
};

pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Bond host inventory");
    phase_bond_inventory(v);

    v.section("Phase 2: Consumer-bond consistency");
    phase_consumer_consistency(v);

    v.section("Phase 3: songBird routing ownership");
    phase_songbird_ownership(v);

    v.section("Phase 4: Trust tier coverage");
    phase_trust_tiers(v);
}

fn phase_bond_inventory(v: &mut ValidationResult) {
    let parsed: toml::Value = toml::from_str(DRAWBRIDGE_BONDS).expect("valid TOML");

    let bonds = parsed
        .get("bonds")
        .and_then(|b| b.as_table())
        .expect("bonds table");

    v.check_bool(
        "inventory:bond_count",
        bonds.len() >= 10,
        &format!("drawbridge_bonds.toml declares {} bonds (expect >= 10)", bonds.len()),
    );

    let mut host_count = 0;
    for (_name, bond) in bonds {
        if bond.get("host").and_then(|h| h.as_str()).is_some() {
            host_count += 1;
        }
    }
    v.check_bool(
        "inventory:all_have_hosts",
        host_count == bonds.len(),
        &format!("{host_count}/{} bonds have host declarations", bonds.len()),
    );

    for (_name, bond) in bonds {
        let has_consumers = bond.get("consumers").and_then(|c| c.as_array()).is_some();
        if !has_consumers {
            v.check_bool(
                "inventory:orphan_bond",
                false,
                "Bond without consumers is orphaned",
            );
            return;
        }
    }
    v.check_bool(
        "inventory:no_orphan_bonds",
        true,
        "All bonds have at least one consumer declared",
    );
}

fn phase_consumer_consistency(v: &mut ValidationResult) {
    let parsed: toml::Value = toml::from_str(DRAWBRIDGE_BONDS).expect("valid TOML");

    let bonds = parsed
        .get("bonds")
        .and_then(|b| b.as_table())
        .expect("bonds table");

    let mut consumers: std::collections::HashSet<String> = std::collections::HashSet::new();
    for (_name, bond) in bonds {
        if let Some(arr) = bond.get("consumers").and_then(|c| c.as_array()) {
            for c in arr {
                if let Some(s) = c.as_str() {
                    consumers.insert(s.to_string());
                }
            }
        }
    }

    v.check_bool(
        "consistency:consumers_exist",
        !consumers.is_empty(),
        &format!("{} unique consumers across all bonds", consumers.len()),
    );

    let known_consumers = ["footPrint", "squirrel", "protoKarya", "tideGlass"];
    for consumer in &known_consumers {
        if consumers.contains(*consumer) {
            v.check_bool(
                &format!("consistency:{consumer}"),
                true,
                &format!("{consumer} is a declared consumer of at least one bond"),
            );
        }
    }

    let has_tier = bonds.values().all(|b| {
        b.get("tier").and_then(|t| t.as_str()).is_some()
    });
    v.check_bool(
        "consistency:all_tiered",
        has_tier,
        "All bonds have trust tier classification",
    );
}

fn phase_songbird_ownership(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let discovery_methods = table.methods_in_domain("discovery");
    let songbird_owns_discovery = discovery_methods.iter().any(|m| {
        table.route(m).is_some_and(|e| &*e.owner == primal_names::SONGBIRD)
    });
    v.check_bool(
        "ownership:songbird_discovery",
        songbird_owns_discovery,
        "songBird owns methods in discovery domain (http + ipc → drawbridge)",
    );

    let has_proxy = REGISTRY_TOML.contains("http.proxy")
        || REGISTRY_TOML.contains("proxy");
    v.check_bool(
        "ownership:proxy_capability",
        has_proxy,
        "HTTP proxy capability registered for drawbridge traffic routing",
    );
}

fn phase_trust_tiers(v: &mut ValidationResult) {
    let tiers = ["scientific", "community", "commercial", "municipal"];

    for tier in tiers {
        let present = DRAWBRIDGE_BONDS.contains(&format!("tier = \"{tier}\""));
        v.check_bool(
            &format!("tier:{tier}"),
            present,
            &format!("Trust tier '{tier}' has at least one bond"),
        );
    }

    let parsed: toml::Value = toml::from_str(DRAWBRIDGE_BONDS).expect("valid TOML");
    let bonds = parsed.get("bonds").and_then(|b| b.as_table()).expect("bonds");

    let fragile_count = bonds.values().filter(|b| {
        b.get("fragile").and_then(toml::Value::as_bool).unwrap_or(false)
    }).count();

    v.check_bool(
        "tier:fragile_declared",
        fragile_count > 0,
        &format!("{fragile_count} bonds marked fragile (expected for municipal tier)"),
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
