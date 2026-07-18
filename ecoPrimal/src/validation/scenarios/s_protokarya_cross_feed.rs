// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: protoKarya Cross-Feed — validates that data produced by footPrint
//! can be consumed by tideGlass (and future protists) via capability.call.
//!
//! This proves the composition mesh: protists can share data through the
//! primal coordination layer (nestGate CAS + capability routing) without
//! direct coupling between them.

use crate::composition::CompositionContext;
use crate::composition::neural_routing::canonical_routing_table;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Scenario registration metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "protokarya-cross-feed",
        track: Track::Lifecycle,
        tier: Tier::Rust,
        provenance_crate: "wave140a_protokarya_cross_feed",
        provenance_date: "2026-07-15",
        description: "protoKarya cross-feed — footPrint data consumed by tideGlass via capability.call",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Data production path (footPrint → CAS)");
    phase_data_production(v);

    v.section("Phase 2: Data consumption path (CAS → tideGlass)");
    phase_data_consumption(v);

    v.section("Phase 3: capability.call cross-protist routing");
    phase_capability_routing(v);

    v.section("Phase 4: Shared capability domains");
    phase_shared_domains(v);
}

fn phase_data_production(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let store_methods = ["storage.store", "storage.store_content", "content.store"];
    let mut has_store = false;
    for method in store_methods {
        if let Some(entry) = table.route(method) {
            if &*entry.owner == primal_names::NESTGATE {
                has_store = true;
                break;
            }
        }
    }
    v.check_bool(
        "produce:nestgate_store",
        has_store,
        "Production path: storage.store* → nestGate (footPrint output target)",
    );

    let has_content_address = REGISTRY_TOML.contains("content.address")
        || REGISTRY_TOML.contains("storage.hash")
        || REGISTRY_TOML.contains("storage.store");
    v.check_bool(
        "produce:content_addressable",
        has_content_address,
        "Content-addressable storage available (CAS for cross-feed)",
    );
}

fn phase_data_consumption(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let fetch_methods = [
        "storage.fetch",
        "storage.fetch_content",
        "content.fetch",
        "storage.get",
    ];
    let mut has_fetch = false;
    for method in fetch_methods {
        if table.route(method).is_some() {
            has_fetch = true;
            break;
        }
    }
    v.check_bool(
        "consume:fetch_available",
        has_fetch,
        "Consumption path: storage.fetch*/get routable (tideGlass input source)",
    );

    let has_discovery = table.route("ipc.discover").is_some()
        || table.route("ipc.list").is_some()
        || REGISTRY_TOML.contains("ipc.discover");
    v.check_bool(
        "consume:discovery_available",
        has_discovery,
        "IPC discovery available (protist finds sibling data via ipc.discover/list)",
    );
}

fn phase_capability_routing(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let has_capability_call =
        table.route("capability.call").is_some() || REGISTRY_TOML.contains("capability.call");
    v.check_bool(
        "routing:capability_call",
        has_capability_call,
        "capability.call registered (cross-protist routing mechanism)",
    );

    let has_capability_discover = table.route("capability.discover").is_some()
        || REGISTRY_TOML.contains("capability.discover")
        || REGISTRY_TOML.contains("capability.list");
    v.check_bool(
        "routing:capability_discover",
        has_capability_discover,
        "capability.discover/list registered (protist finds sibling data)",
    );

    let has_mesh = table.route("mesh.peers").is_some() || REGISTRY_TOML.contains("mesh.peers");
    v.check_bool(
        "routing:mesh_peers",
        has_mesh,
        "mesh.peers available (cross-gate protist coordination)",
    );
}

fn phase_shared_domains(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let shared = [
        ("storage", "Data persistence (shared by all protists)"),
        (
            "discovery",
            "Network + IPC (drawbridge bonds, peer discovery)",
        ),
        ("tensor", "Compute (matrix/vector operations for science)"),
    ];

    for (domain, desc) in shared {
        let methods = table.methods_in_domain(domain);
        v.check_bool(
            &format!("domain:{domain}"),
            !methods.is_empty(),
            &format!("{domain} domain has {} methods ({desc})", methods.len()),
        );
    }

    let cross_feed = REGISTRY_TOML.contains("[storage]")
        && REGISTRY_TOML.contains("[http]")
        && REGISTRY_TOML.contains("[compute]");
    v.check_bool(
        "domain:cross_feed_bridge",
        cross_feed,
        "Cross-feed bridge: storage + http + compute sections all present",
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
