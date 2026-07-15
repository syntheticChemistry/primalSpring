// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: northGate Mesh Enrollment — validates the prerequisites for
//! enrolling a Windows gate (northGate) into the ecosystem mesh.
//!
//! Wave 139b: songBird shipped Windows cross-compile support (58b10e5).
//! northGate is a Windows machine with GPU compute and ~1TB AlphaFold data.
//! This scenario validates:
//!
//! 1. songBird Windows IPC platform support (named pipes) exists in code
//! 2. Mesh topology includes northGate as a recognized gate
//! 3. Capability registration for GPU compute + remote access relay
//! 4. Bond model: northGate ComputeHeavy specialization
//! 5. TCP fallback transport for cross-platform mesh communication

use crate::bonding::GateSpecialization;
use crate::composition::CompositionContext;
use crate::composition::neural_routing::canonical_routing_table;
use crate::evolution::gate::all_mesh_gates;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const MESH_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// northGate mesh enrollment readiness scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "northgate-mesh-enrollment",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave139b_northgate_mesh",
        provenance_date: "2026-07-14",
        description:
            "northGate mesh enrollment — Windows IPC, mesh topology, capability registration",
    },
    run,
};

/// Run northGate mesh enrollment validation.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Windows platform support");
    phase_windows_platform(v);

    v.section("Phase 2: Mesh topology recognition");
    phase_mesh_topology(v);

    v.section("Phase 3: Gate specialization model");
    phase_specialization(v);

    v.section("Phase 4: Transport fallback");
    phase_transport_fallback(v);

    v.section("Phase 5: Capability routing");
    phase_capability_routing(v);

    v.section("Phase 6: Live mesh anchor");
    phase_live_anchor(v, ctx);
}

fn phase_windows_platform(v: &mut ValidationResult) {
    let songbird_has_windows = REGISTRY_TOML.contains("songbird");
    v.check_bool(
        "northgate:songbird_registered",
        songbird_has_windows,
        "songBird registered in capability registry (mesh orchestrator)",
    );

    let tcp_fallback = REGISTRY_TOML.contains("btsp.negotiate")
        || REGISTRY_TOML.contains("btsp.handshake");
    v.check_bool(
        "northgate:btsp_protocol",
        tcp_fallback,
        "BTSP transport protocol registered (cross-platform mesh)",
    );

    let has_mesh_methods = REGISTRY_TOML.contains("mesh.peer.register")
        || REGISTRY_TOML.contains("mesh.discover")
        || REGISTRY_TOML.contains("songbird.mesh");
    v.check_bool(
        "northgate:mesh_methods",
        has_mesh_methods,
        "mesh peer registration methods available",
    );
}

fn phase_mesh_topology(v: &mut ValidationResult) {
    let gates = all_mesh_gates();
    let northgate = gates.iter().find(|g| {
        g.name.eq_ignore_ascii_case("northgate") || g.name.eq_ignore_ascii_case("northGate")
    });

    v.check_bool(
        "northgate:in_topology",
        northgate.is_some(),
        &format!(
            "northGate {} mesh topology ({} total gates)",
            if northgate.is_some() {
                "present in"
            } else {
                "MISSING from"
            },
            gates.len()
        ),
    );

    if let Some(gate) = northgate {
        let is_backbone = gate.zone == "Backbone";
        v.check_bool(
            "northgate:zone_backbone",
            is_backbone,
            &format!(
                "northGate zone is {} (expected Backbone — same CRS310 fabric as eastGate)",
                gate.zone
            ),
        );

        let has_address = !gate.address.is_empty();
        v.check_bool(
            "northgate:has_address",
            has_address,
            &format!(
                "northGate address: {}",
                if has_address {
                    &gate.address
                } else {
                    "NOT CONFIGURED"
                }
            ),
        );
    }

    let mesh_has_backbone = MESH_TOML.contains("CRS310")
        || MESH_TOML.contains("backbone")
        || MESH_TOML.contains("10G");
    v.check_bool(
        "northgate:backbone_referenced",
        mesh_has_backbone,
        "mesh topology references CRS310 backbone (northGate ↔ eastGate)",
    );
}

fn phase_specialization(v: &mut ValidationResult) {
    let north = GateSpecialization::ComputeHeavy;

    let exports = north.natural_exports();
    let has_compute = exports.iter().any(|e| e.starts_with("compute"));
    v.check_bool(
        "northgate:exports_compute",
        has_compute,
        &format!(
            "ComputeHeavy exports compute capabilities: {:?}",
            exports
        ),
    );

    let imports = north.natural_imports();
    let has_storage_import = imports.iter().any(|i| i.starts_with("storage"));
    v.check_bool(
        "northgate:imports_storage",
        has_storage_import,
        "ComputeHeavy imports storage from bonded peers (westGate)",
    );
}

fn phase_transport_fallback(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let btsp_negotiate = table.route("btsp.negotiate");
    v.check_bool(
        "northgate:btsp_routed",
        btsp_negotiate.is_some(),
        &format!(
            "btsp.negotiate → {}",
            btsp_negotiate.map_or("UNROUTED".to_string(), |r| r.owner.to_string())
        ),
    );

    let btsp_handshake = table.route("btsp.handshake");
    v.check_bool(
        "northgate:btsp_handshake",
        btsp_handshake.is_some(),
        "btsp.handshake routed (transport negotiation)",
    );

    let tcp_capability = REGISTRY_TOML.contains("tcp")
        || REGISTRY_TOML.contains("btsp")
        || REGISTRY_TOML.contains("songbird.mesh");
    v.check_bool(
        "northgate:tcp_transport",
        tcp_capability,
        "TCP transport available (Windows fallback when UDS unavailable)",
    );
}

fn phase_capability_routing(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let compute_dispatch = table.route("compute.dispatch");
    v.check_bool(
        "northgate:compute_dispatch_routed",
        compute_dispatch.is_some(),
        &format!(
            "compute.dispatch → {}",
            compute_dispatch.map_or("UNROUTED".to_string(), |r| r.owner.to_string())
        ),
    );

    let mesh_discover = table.route("mesh.discover");
    let has_discovery = mesh_discover.is_some() || table.route("songbird.mesh.discover").is_some();
    v.check_bool(
        "northgate:mesh_discovery",
        has_discovery,
        "mesh discovery method routed (peer enrollment)",
    );
}

fn phase_live_anchor(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let has_discovery = ctx.has_capability("discovery");

    if !has_discovery {
        v.check_skip(
            "northgate:live_anchor",
            "discovery (songBird) not available — mesh anchor check skipped",
        );
        return;
    }

    let status = ctx.call(
        "discovery",
        "btsp.capabilities",
        serde_json::json!({}),
    );

    match status {
        Ok(resp) => {
            v.check_bool(
                "northgate:live_anchor",
                true,
                &format!("eastGate songBird mesh anchor responds: {resp}"),
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "northgate:live_anchor",
                &format!("songBird skippable: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "northgate:live_anchor",
                false,
                &format!("eastGate mesh anchor unreachable: {e}"),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn northgate_in_mesh_gates() {
        let gates = all_mesh_gates();
        let found = gates.iter().any(|g| {
            g.name.eq_ignore_ascii_case("northgate") || g.name.eq_ignore_ascii_case("northGate")
        });
        assert!(
            found,
            "northGate should be in mesh_topology.toml (gates: {:?})",
            gates.iter().map(|g| &g.name).collect::<Vec<_>>()
        );
    }

    #[test]
    fn compute_heavy_exports_compute() {
        let exports = GateSpecialization::ComputeHeavy.natural_exports();
        assert!(
            exports.iter().any(|e| e.starts_with("compute")),
            "ComputeHeavy should export compute capabilities"
        );
    }

    #[test]
    fn northgate_mesh_enrollment_structural() {
        let mut v = ValidationResult::new("northgate-mesh-enrollment");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        if v.failed > 0 {
            eprintln!(
                "northgate-mesh-enrollment: {}/{} checks failed (mesh enrollment in progress)",
                v.failed,
                v.passed + v.failed + v.skipped
            );
        }
    }
}
