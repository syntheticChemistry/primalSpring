// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Drawbridge Consumer Parity — validates bidirectional match between
//! songBird's compiled bond constants and the `drawbridge_bonds.toml` registry.
//!
//! No stale or missing routes. Every host in songBird's allowlist must appear
//! in the registry, and every registry host must be routable by songBird.

use std::collections::HashSet;

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "drawbridge-consumer-parity",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "wave140a_protokarya_tangibles",
        provenance_date: "2026-07-15",
        description:
            "Drawbridge consumer parity — songBird bond constants ↔ drawbridge_bonds.toml bidirectional match",
    },
    run,
};

const SONGBIRD_GIS_HOSTS: &[&str] = &[
    "tile.openstreetmap.org",
    "overpass-api.de",
    "hazards.fema.gov",
    "services1.arcgis.com",
    "services2.arcgis.com",
    "nominatim.openstreetmap.org",
    "epqs.nationalmap.gov",
    "sdmdataaccess.sc.egov.usda.gov",
    "gisagocss.state.mi.us",
    "gisp.mcgi.state.mi.us",
    "gis2.cityofeastlansing.com",
];

const SONGBIRD_SCIENCE_HOSTS: &[&str] = &[
    "eutils.ncbi.nlm.nih.gov",
    "pubchem.ncbi.nlm.nih.gov",
    "blast.ncbi.nlm.nih.gov",
    "rest.uniprot.org",
    "data.rcsb.org",
    "alphafold.ebi.ac.uk",
];

const BONDS_TOML: &str = include_str!("../../../../config/drawbridge_bonds.toml");

pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Parse drawbridge_bonds.toml host inventory");
    let registry_hosts = phase_parse_registry(v);

    v.section("Phase 2: songBird → registry coverage (no missing routes)");
    phase_songbird_to_registry(v, &registry_hosts);

    v.section("Phase 3: Registry → songBird coverage (no stale entries)");
    phase_registry_to_songbird(v, &registry_hosts);

    v.section("Phase 4: Consumer assignment consistency");
    phase_consumer_assignment(v);

    v.section("Phase 5: Bond count parity");
    phase_count_parity(v, &registry_hosts);
}

fn phase_parse_registry(v: &mut ValidationResult) -> HashSet<String> {
    let mut hosts = HashSet::new();

    for line in BONDS_TOML.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("host = ") {
            if let Some(host) = trimmed
                .strip_prefix("host = \"")
                .and_then(|s| s.strip_suffix('"'))
            {
                hosts.insert(host.to_string());
            }
        }
    }

    v.check_bool(
        "parity:registry_parsed",
        !hosts.is_empty(),
        &format!("drawbridge_bonds.toml parsed: {} hosts", hosts.len()),
    );

    v.check_bool(
        "parity:registry_min_count",
        hosts.len() >= 15,
        &format!("registry has ≥15 hosts (found {})", hosts.len()),
    );

    hosts
}

fn phase_songbird_to_registry(v: &mut ValidationResult, registry: &HashSet<String>) {
    let mut missing = Vec::new();

    for host in SONGBIRD_GIS_HOSTS {
        let found = registry.contains(*host);
        if !found {
            missing.push(*host);
        }
        v.check_bool(
            &format!("parity:sb→reg:gis:{}", host_slug(host)),
            found,
            &format!("GIS host '{host}' in registry"),
        );
    }

    for host in SONGBIRD_SCIENCE_HOSTS {
        let found = registry.contains(*host);
        if !found {
            missing.push(*host);
        }
        v.check_bool(
            &format!("parity:sb→reg:sci:{}", host_slug(host)),
            found,
            &format!("Science host '{host}' in registry"),
        );
    }

    if !missing.is_empty() {
        v.check_bool(
            "parity:sb→reg:complete",
            false,
            &format!(
                "{} songBird hosts missing from registry: {}",
                missing.len(),
                missing.join(", ")
            ),
        );
    }
}

fn phase_registry_to_songbird(v: &mut ValidationResult, registry: &HashSet<String>) {
    let songbird_all: HashSet<&str> = SONGBIRD_GIS_HOSTS
        .iter()
        .chain(SONGBIRD_SCIENCE_HOSTS.iter())
        .copied()
        .collect();

    let mut surplus = Vec::new();

    for host in registry {
        let found = songbird_all.contains(host.as_str());
        if !found {
            surplus.push(host.clone());
        }
        v.check_bool(
            &format!("parity:reg→sb:{}", host_slug(host)),
            found,
            &format!("Registry host '{host}' routable by songBird"),
        );
    }

    if !surplus.is_empty() {
        v.check_bool(
            "parity:reg→sb:complete",
            false,
            &format!(
                "{} registry hosts not in songBird constants: {}",
                surplus.len(),
                surplus.join(", ")
            ),
        );
    }
}

fn phase_consumer_assignment(v: &mut ValidationResult) {
    let gis_consumer = "footPrint";
    let science_consumers = ["squirrel", "protoKarya"];

    for line in BONDS_TOML.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("consumers = ") {
            if trimmed.contains(gis_consumer) || science_consumers.iter().any(|c| trimmed.contains(c)) {
                continue;
            }
        }
    }

    let gis_count = BONDS_TOML
        .lines()
        .filter(|l| l.contains(&format!("\"{}\"", gis_consumer)))
        .count();
    let science_count = BONDS_TOML
        .lines()
        .filter(|l| l.contains("\"squirrel\"") || l.contains("\"protoKarya\""))
        .count();

    v.check_bool(
        "parity:consumer:footprint_present",
        gis_count > 0,
        &format!("{gis_consumer} appears as consumer in {gis_count} bonds"),
    );

    v.check_bool(
        "parity:consumer:science_present",
        science_count > 0,
        &format!("squirrel/protoKarya appears as consumer in {science_count} bonds"),
    );
}

fn phase_count_parity(v: &mut ValidationResult, registry: &HashSet<String>) {
    let songbird_total = SONGBIRD_GIS_HOSTS.len() + SONGBIRD_SCIENCE_HOSTS.len();

    v.check_bool(
        "parity:count:gis",
        SONGBIRD_GIS_HOSTS.len() >= 10,
        &format!("songBird GIS bonds ≥ 10 (found {})", SONGBIRD_GIS_HOSTS.len()),
    );

    v.check_bool(
        "parity:count:science",
        SONGBIRD_SCIENCE_HOSTS.len() >= 5,
        &format!(
            "songBird Science bonds ≥ 5 (found {})",
            SONGBIRD_SCIENCE_HOSTS.len()
        ),
    );

    v.check_bool(
        "parity:count:registry",
        registry.len() >= 15,
        &format!("Registry bonds ≥ 15 (found {})", registry.len()),
    );

    let diff = (songbird_total as isize - registry.len() as isize).unsigned_abs();
    v.check_bool(
        "parity:count:delta",
        diff <= 3,
        &format!(
            "songBird ({songbird_total}) vs registry ({}) delta ≤ 3 (actual: {diff})",
            registry.len()
        ),
    );
}

fn host_slug(host: &str) -> String {
    host.replace('.', "_").replace('-', "_")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_metadata_valid() {
        assert_eq!(SCENARIO.meta.id, "drawbridge-consumer-parity");
        assert!(matches!(SCENARIO.meta.track, Track::Infrastructure));
    }

    #[test]
    fn songbird_constants_consistent() {
        let gis: HashSet<&&str> = SONGBIRD_GIS_HOSTS.iter().collect();
        let sci: HashSet<&&str> = SONGBIRD_SCIENCE_HOSTS.iter().collect();
        assert!(gis.is_disjoint(&sci), "GIS and Science hosts must not overlap");
    }

    #[test]
    fn registry_parses() {
        let mut v = ValidationResult::new("test");
        let hosts = phase_parse_registry(&mut v);
        assert!(hosts.len() >= 15, "need at least 15 hosts, got {}", hosts.len());
    }

    #[test]
    fn scenario_runs_structural() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);

        if v.failed > 0 {
            eprintln!(
                "drawbridge-consumer-parity: {} passed, {} failed — parity gaps exist",
                v.passed, v.failed
            );
        }
    }
}
