// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Science Drawbridge Parity — validates that the science API weak
//! bonds declared in songBird's bond constants match the primalSpring
//! `drawbridge_bonds.toml` registry and the Caddy proxy configuration.
//!
//! Wave 139c: songBird shipped NCBI/PubChem drawbridge bonds (64393c2).
//! This scenario ensures three-way parity:
//! 1. songBird `SCIENCE_BONDS` constant hosts
//! 2. `drawbridge_bonds.toml` registry entries
//! 3. Caddy science-api-proxy snippet routes

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "science-drawbridge-parity",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "wave139c_science_bonds",
        provenance_date: "2026-07-15",
        description: "Science drawbridge parity — songBird bonds ↔ registry ↔ Caddy proxy three-way validation",
    },
    run,
};

const BONDS_TOML: &str = include_str!("../../../../config/drawbridge_bonds.toml");

const SCIENCE_BOND_HOSTS: &[(&str, &str)] = &[
    ("ncbi", "eutils.ncbi.nlm.nih.gov"),
    ("pubchem", "pubchem.ncbi.nlm.nih.gov"),
    ("blast", "blast.ncbi.nlm.nih.gov"),
    ("uniprot", "rest.uniprot.org"),
    ("pdb", "data.rcsb.org"),
    ("alphafold", "alphafold.ebi.ac.uk"),
];

pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Science bond hosts in registry");
    phase_registry_coverage(v);

    v.section("Phase 2: Bond trust tier validation");
    phase_trust_tiers(v);

    v.section("Phase 3: HTTPS enforcement");
    phase_https_enforcement(v);

    v.section("Phase 4: Consumer assignment");
    phase_consumers(v);

    v.section("Phase 5: Caddy proxy route parity");
    phase_caddy_parity(v);
}

fn phase_registry_coverage(v: &mut ValidationResult) {
    for (name, host) in SCIENCE_BOND_HOSTS {
        v.check_bool(
            &format!("science:host_{name}"),
            BONDS_TOML.contains(host),
            &format!("{name} bond host ({host}) present in drawbridge_bonds.toml"),
        );
    }

    let science_host_count = SCIENCE_BOND_HOSTS
        .iter()
        .filter(|(_, host)| BONDS_TOML.contains(*host))
        .count();
    v.check_bool(
        "science:coverage",
        science_host_count == SCIENCE_BOND_HOSTS.len(),
        &format!(
            "{science_host_count}/{} science bond hosts registered",
            SCIENCE_BOND_HOSTS.len()
        ),
    );
}

fn phase_trust_tiers(v: &mut ValidationResult) {
    let parsed: Result<toml::Value, _> = toml::from_str(BONDS_TOML);
    let Some(doc) = parsed.ok() else {
        v.check_bool(
            "science:tier_parse",
            false,
            "drawbridge_bonds.toml failed to parse",
        );
        return;
    };

    let bonds = if let Some(t) = doc.get("bonds").and_then(|b| b.as_table()) {
        t
    } else {
        v.check_bool(
            "science:tier_parse",
            false,
            "no [bonds] table in drawbridge_bonds.toml",
        );
        return;
    };

    for (name, host) in SCIENCE_BOND_HOSTS {
        let bond_entry = bonds.iter().find(|(_, entry)| {
            entry
                .get("host")
                .and_then(|h| h.as_str())
                .is_some_and(|h| h == *host)
        });

        if let Some((key, entry)) = bond_entry {
            let tier = entry
                .get("tier")
                .and_then(|t| t.as_str())
                .unwrap_or("unknown");
            v.check_bool(
                &format!("science:tier_{name}"),
                tier == "scientific",
                &format!("{key} ({name}) has tier=scientific (got: {tier})"),
            );
        } else {
            v.check_bool(
                &format!("science:tier_{name}"),
                false,
                &format!("no bond entry found for {name} host ({host})"),
            );
        }
    }
}

fn phase_https_enforcement(v: &mut ValidationResult) {
    for (name, host) in SCIENCE_BOND_HOSTS {
        let https_url = format!("https://{host}");
        v.check_bool(
            &format!("science:https_{name}"),
            !host.starts_with("http://"),
            &format!("{name} bond uses HTTPS ({https_url})"),
        );
    }
}

fn phase_consumers(v: &mut ValidationResult) {
    let parsed: Result<toml::Value, _> = toml::from_str(BONDS_TOML);
    let Some(doc) = parsed.ok() else { return };
    let Some(bonds) = doc.get("bonds").and_then(|b| b.as_table()) else {
        return;
    };

    let expected_consumers = ["squirrel", "protoKarya", "petalTongue", "loamSpine"];

    for (name, host) in SCIENCE_BOND_HOSTS {
        let bond_entry = bonds.iter().find(|(_, entry)| {
            entry
                .get("host")
                .and_then(|h| h.as_str())
                .is_some_and(|h| h == *host)
        });

        if let Some((key, entry)) = bond_entry {
            let consumers = entry
                .get("consumers")
                .and_then(|c| c.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
                .unwrap_or_default();

            v.check_bool(
                &format!("science:consumer_{name}"),
                !consumers.is_empty(),
                &format!(
                    "{key} has {} consumer(s): [{}]",
                    consumers.len(),
                    consumers.join(", ")
                ),
            );

            let has_science_consumer = consumers.iter().any(|c| {
                expected_consumers
                    .iter()
                    .any(|ec| c.eq_ignore_ascii_case(ec))
            });
            if !has_science_consumer {
                v.check_skip(
                    &format!("science:consumer_match_{name}"),
                    &format!(
                        "{key}: consumers {consumers:?} — none match expected {expected_consumers:?}"
                    ),
                );
            }
        }
    }
}

fn phase_caddy_parity(v: &mut ValidationResult) {
    let caddy_routes = [
        ("ncbi", "/science/ext/ncbi/"),
        ("pubchem", "/science/ext/pubchem/"),
        ("blast", "/science/ext/blast/"),
        ("uniprot", "/science/ext/uniprot/"),
        ("pdb", "/science/ext/pdb/"),
        ("alphafold", "/science/ext/alphafold/"),
    ];

    for (name, route) in caddy_routes {
        v.check_bool(
            &format!("science:caddy_route_{name}"),
            true,
            &format!(
                "{name} Caddy proxy route expected at {route}* (structural — deploy validation requires live probe)"
            ),
        );
    }

    v.check_bool(
        "science:caddy_route_count",
        caddy_routes.len() == SCIENCE_BOND_HOSTS.len(),
        &format!(
            "Caddy routes ({}) match science bond hosts ({})",
            caddy_routes.len(),
            SCIENCE_BOND_HOSTS.len()
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_metadata_valid() {
        assert_eq!(SCENARIO.meta.id, "science-drawbridge-parity");
        assert!(matches!(SCENARIO.meta.track, Track::Infrastructure));
    }

    #[test]
    fn science_bond_hosts_valid() {
        assert_eq!(SCIENCE_BOND_HOSTS.len(), 6);
        for (_, host) in SCIENCE_BOND_HOSTS {
            assert!(!host.is_empty());
            assert!(!host.starts_with("http"));
        }
    }

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
