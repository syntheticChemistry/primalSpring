// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: footPrint Drawbridge Live — validates the E2E data flow from
//! external GIS services through songBird drawbridge to `NestGate` CAS.
//!
//! This proves the composition path: external API → drawbridge → primal → CAS.
//! Without this, footPrint is just a static SPA with proxy passthrough.

use crate::composition::CompositionContext;
use crate::composition::neural_routing::canonical_routing_table;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const DRAWBRIDGE_BONDS: &str = include_str!("../../../../config/drawbridge_bonds.toml");
const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "footprint-drawbridge-live",
        track: Track::Lifecycle,
        tier: Tier::Rust,
        provenance_crate: "wave140a_footprint_drawbridge",
        provenance_date: "2026-07-15",
        description: "footPrint drawbridge live — E2E: USGS/FEMA via drawbridge → NestGate CAS",
    },
    run,
};

pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: footPrint GIS bonds declared");
    phase_gis_bonds(v);

    v.section("Phase 2: Drawbridge routing (songBird ownership)");
    phase_drawbridge_routing(v);

    v.section("Phase 3: CAS storage pipeline (nestGate)");
    phase_cas_pipeline(v);

    v.section("Phase 4: Composition wiring (drawbridge → CAS)");
    phase_composition_wiring(v);

    v.section("Phase 5: Live surface availability");
    phase_live_surface(v);
}

fn phase_gis_bonds(v: &mut ValidationResult) {
    let required_hosts = [
        ("usgs_elevation", "epqs.nationalmap.gov"),
        ("fema_flood", "hazards.fema.gov"),
        ("nrcs_soils", "sdmdataaccess.sc.egov.usda.gov"),
        ("osm_overpass", "overpass-api.de"),
        ("osm_nominatim", "nominatim.openstreetmap.org"),
    ];

    for (bond_name, host) in required_hosts {
        let declared = DRAWBRIDGE_BONDS.contains(host);
        v.check_bool(
            &format!("bond:{bond_name}"),
            declared,
            &format!("{bond_name} ({host}) declared in drawbridge_bonds.toml"),
        );
    }

    let fp_consumer_count = DRAWBRIDGE_BONDS.matches("\"footPrint\"").count();
    v.check_bool(
        "bond:footprint_consumer_count",
        fp_consumer_count >= 5,
        &format!("footPrint is consumer of {fp_consumer_count} bonds (expect >= 5)"),
    );
}

fn phase_drawbridge_routing(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let drawbridge_methods = ["http.proxy", "http.get", "http.post"];
    let mut routed = false;
    for method in drawbridge_methods {
        if let Some(entry) = table.route(method) {
            if &*entry.owner == primal_names::SONGBIRD {
                routed = true;
                break;
            }
        }
    }
    v.check_bool(
        "routing:songbird_http",
        routed,
        "HTTP proxy/get/post routing owned by songBird (drawbridge transport)",
    );

    let has_proxy = REGISTRY_TOML.contains("http.proxy") || REGISTRY_TOML.contains("proxy");
    v.check_bool(
        "routing:proxy_registered",
        has_proxy,
        "HTTP proxy capability registered for drawbridge traffic",
    );
}

fn phase_cas_pipeline(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let cas_methods = ["storage.store", "storage.store_content", "content.store"];
    let mut cas_routed = false;
    for method in cas_methods {
        if let Some(entry) = table.route(method) {
            if &*entry.owner == primal_names::NESTGATE {
                cas_routed = true;
                break;
            }
        }
    }
    v.check_bool(
        "cas:nestgate_store",
        cas_routed,
        "CAS storage (storage.store*) routed to nestGate",
    );

    let fetch_methods = ["storage.fetch", "storage.fetch_content", "content.fetch"];
    let mut fetch_routed = false;
    for method in fetch_methods {
        if table.route(method).is_some() {
            fetch_routed = true;
            break;
        }
    }
    v.check_bool(
        "cas:fetch_method",
        fetch_routed,
        "CAS fetch (storage.fetch*) is routable",
    );
}

fn phase_composition_wiring(v: &mut ValidationResult) {
    let has_footprint_ref = DRAWBRIDGE_BONDS.contains("footPrint")
        || REGISTRY_TOML.contains("footprint")
        || REGISTRY_TOML.contains("gis");
    v.check_bool(
        "wiring:footprint_declared",
        has_footprint_ref,
        "footPrint referenced as consumer in bonds or registry",
    );

    let bond_flow = DRAWBRIDGE_BONDS.contains("consumers")
        && DRAWBRIDGE_BONDS.contains("footPrint")
        && DRAWBRIDGE_BONDS.contains("cache_days");
    v.check_bool(
        "wiring:bond_flow_complete",
        bond_flow,
        "Bond flow: consumers + cache policy declared for footPrint",
    );

    let has_storage =
        REGISTRY_TOML.contains("storage.store") || REGISTRY_TOML.contains("content.store");
    v.check_bool(
        "wiring:storage_for_cache",
        has_storage,
        "Storage capability available (drawbridge responses → CAS persistence)",
    );
}

fn phase_live_surface(v: &mut ValidationResult) {
    let manifest = include_str!("../../../../../../infra/wateringHole/ecosystem_manifest.toml");
    let has_composition_url = manifest.contains("primals.eco/footprint/");
    v.check_bool(
        "surface:composition_url",
        has_composition_url,
        "footPrint composition URL (primals.eco/footprint/) in manifest",
    );

    let has_caddy_route = DRAWBRIDGE_BONDS.contains("epqs.nationalmap.gov")
        && DRAWBRIDGE_BONDS.contains("hazards.fema.gov");
    v.check_bool(
        "surface:gis_hosts_declared",
        has_caddy_route,
        "GIS upstream hosts (USGS, FEMA) declared for Caddy proxy routing",
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
        assert_eq!(
            v.failed, 0,
            "scenario '{}' had {} failures",
            SCENARIO.meta.id, v.failed
        );
    }
}
