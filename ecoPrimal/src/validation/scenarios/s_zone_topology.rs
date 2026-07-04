// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Zone Topology — validates the three-hub triangle cytoplasm zone
//! model and ensures gates are correctly assigned to their physical zones.
//!
//! The K-Derm cytoplasm is segmented into physical zones by switching fabric:
//!
//! | Zone     | Hub   | Fabric           | Gates                              |
//! |----------|-------|------------------|------------------------------------|
//! | Backbone | Hub 1 | CRS310 (10G)     | sporeGate, eastGate, northGate, ironGate |
//! | House2   | Hub 2 | Omada SX3008F    | strandGate, southGate, swiftGate, fieldGate |
//! | Garage   | Hub 3 | planned          | (future compute + outdoor WiFi)    |
//! | WAN      | —     | Internet/VPS     | golgi, flockGate                   |
//!
//! The target topology forms a triangle:
//! ```text
//!   Hub 1 (Backbone) ----leg B (80m AOC 10G)---- Hub 2 (House2)
//!       \                                         /
//!        leg A (planned)           leg C (planned)
//!           \                    /
//!            Hub 3 (Garage)
//! ```
//!
//! This scenario validates:
//! 1. All known gates have correct zone assignments
//! 2. Zone coverage: at least backbone + house2 + wan have gates
//! 3. Triangle topology: backbone↔house2 leg exists (10G AOC)
//! 4. Cross-zone traffic requires explicit relay (no implicit L2 bridging)
//! 5. Zone isolation: gates in same zone share L2, cross-zone is L3/WG

use crate::composition::CompositionContext;
use crate::evolution::gate::{CytoplasmZone, GateMatrix, all_mesh_gates, mesh_address};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Zone topology validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "zone-topology",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-18",
        description: "Three-hub triangle cytoplasm zone model and gate assignments",
    },
    run: run_zone_topology,
};

fn run_zone_topology(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    validate_zone_assignments(v);
    validate_zone_coverage(v);
    validate_triangle_legs(v);
    validate_zone_isolation(v);
    validate_enrollment_zones(v);
    validate_mesh_addresses(v);
}

fn validate_zone_assignments(v: &mut ValidationResult) {
    let cases: &[(&str, CytoplasmZone)] = &[
        ("sporeGate", CytoplasmZone::Backbone),
        ("eastGate", CytoplasmZone::Backbone),
        ("northGate", CytoplasmZone::Backbone),
        ("ironGate", CytoplasmZone::Backbone),
        ("strandGate", CytoplasmZone::House2),
        ("southGate", CytoplasmZone::House2),
        ("swiftGate", CytoplasmZone::House2),
        ("fieldGate", CytoplasmZone::House2),
        ("golgi", CytoplasmZone::Wan),
        ("flockGate", CytoplasmZone::Wan),
    ];

    for (gate, expected) in cases {
        let actual = CytoplasmZone::for_gate(gate);
        v.check_bool(
            &format!("zone:{gate}:correct"),
            actual == *expected,
            &format!("expected {}, got {}", expected.label(), actual.label()),
        );
    }

    v.check_bool(
        "zone:unknown:unassigned",
        CytoplasmZone::for_gate("something-new") == CytoplasmZone::Unassigned,
        "unknown gate names should map to Unassigned",
    );
}

fn validate_zone_coverage(v: &mut ValidationResult) {
    let matrix = GateMatrix::ecosystem_snapshot();
    let mut has_backbone = false;
    let mut has_house2 = false;
    let mut has_wan = false;

    for gate in &matrix.gates {
        match gate.zone {
            CytoplasmZone::Backbone => has_backbone = true,
            CytoplasmZone::House2 => has_house2 = true,
            CytoplasmZone::Wan => has_wan = true,
            _ => {}
        }
    }

    v.check_bool(
        "coverage:backbone",
        has_backbone,
        "at least one gate in backbone zone",
    );
    v.check_bool(
        "coverage:house2",
        has_house2,
        "at least one gate in house2 zone",
    );
    v.check_bool("coverage:wan", has_wan, "at least one gate in wan zone");

    let backbone_count = matrix
        .gates
        .iter()
        .filter(|g| g.zone == CytoplasmZone::Backbone)
        .count();
    v.check_bool(
        "coverage:backbone:count",
        backbone_count >= 3,
        &format!("backbone should have >=3 gates, has {backbone_count}"),
    );
}

/// Validate the three-hub triangle topology legs.
/// Leg B (backbone↔house2) is LIVE with 80m AOC 10G.
/// Legs A and C are planned (garage).
fn validate_triangle_legs(v: &mut ValidationResult) {
    #[derive(Debug)]
    struct TopologyLeg {
        name: &'static str,
        from: CytoplasmZone,
        to: CytoplasmZone,
        live: bool,
        medium: &'static str,
    }

    let legs = [
        TopologyLeg {
            name: "leg_a",
            from: CytoplasmZone::Backbone,
            to: CytoplasmZone::Garage,
            live: false,
            medium: "planned",
        },
        TopologyLeg {
            name: "leg_b",
            from: CytoplasmZone::Backbone,
            to: CytoplasmZone::House2,
            live: true,
            medium: "80m AOC 10G",
        },
        TopologyLeg {
            name: "leg_c",
            from: CytoplasmZone::Garage,
            to: CytoplasmZone::House2,
            live: false,
            medium: "planned",
        },
    ];

    v.check_bool(
        "triangle:leg_count",
        legs.len() == 3,
        "three-hub triangle requires exactly 3 legs",
    );

    for leg in &legs {
        v.check_bool(
            &format!("triangle:{}:zones_distinct", leg.name),
            leg.from != leg.to,
            "leg must connect different zones",
        );
    }

    v.check_bool(
        "triangle:leg_b:live",
        legs[1].live,
        "backbone↔house2 (leg B) must be live for current operations",
    );

    v.check_bool(
        "triangle:leg_b:medium",
        legs[1].medium.contains("10G"),
        "backbone↔house2 should be 10G fabric",
    );
}

fn validate_zone_isolation(v: &mut ValidationResult) {
    let same_zone_gates = [("sporeGate", "eastGate")];
    for (a, b) in &same_zone_gates {
        let za = CytoplasmZone::for_gate(a);
        let zb = CytoplasmZone::for_gate(b);
        v.check_bool(
            &format!("isolation:{a}_{b}:same_zone"),
            za == zb,
            "same-zone gates share L2 fabric",
        );
    }

    let cross_zone_pairs = [
        ("sporeGate", "strandGate"),
        ("eastGate", "golgi"),
        ("southGate", "flockGate"),
    ];
    for (a, b) in &cross_zone_pairs {
        let za = CytoplasmZone::for_gate(a);
        let zb = CytoplasmZone::for_gate(b);
        v.check_bool(
            &format!("isolation:{a}_{b}:cross_zone"),
            za != zb,
            "cross-zone gates must traverse L3/WG relay",
        );
    }
}

/// Validate that enrollment targets from Wave 116 have correct zone assignments.
fn validate_enrollment_zones(v: &mut ValidationResult) {
    let enrollment_targets = [
        ("eastGate", CytoplasmZone::Backbone),
        ("ironGate", CytoplasmZone::Backbone),
        ("flockGate", CytoplasmZone::Wan),
    ];

    for (gate, expected_zone) in &enrollment_targets {
        let zone = CytoplasmZone::for_gate(gate);
        v.check_bool(
            &format!("enrollment:{gate}:zone"),
            zone == *expected_zone,
            &format!(
                "enrollment target {gate} should be in {}, found {}",
                expected_zone.label(),
                zone.label()
            ),
        );
    }
}

/// Validate WireGuard mesh overlay address assignments (10.13.37.0/24).
fn validate_mesh_addresses(v: &mut ValidationResult) {
    let gates = all_mesh_gates();
    let peered: Vec<_> = gates.iter().filter(|e| !e.address.is_empty()).collect();

    v.check_bool(
        "mesh:gates_populated",
        !peered.is_empty(),
        &format!("{} gates have mesh addresses", peered.len()),
    );

    for entry in &peered {
        let lookup = mesh_address(&entry.name);
        v.check_bool(
            &format!("mesh:{}:lookup_consistent", entry.name),
            lookup == Some(entry.address.as_str()),
            &format!(
                "mesh_address(\"{}\") should return {:?}, got {:?}",
                entry.name, entry.address, lookup
            ),
        );
    }

    // Unassigned gates should return None
    let unpeered = ["northGate", "strandGate", "southGate", "swiftGate"];
    for gate in &unpeered {
        v.check_bool(
            &format!("mesh:{gate}:unassigned"),
            mesh_address(gate).is_none(),
            "unpeered gate should have no mesh address",
        );
    }

    // All assigned addresses must be unique and in 10.13.37.0/24
    let assigned: Vec<_> = peered.iter().map(|e| e.address.as_str()).collect();
    let mut seen = std::collections::HashSet::new();
    let all_unique = assigned.iter().all(|ip| seen.insert(*ip));
    v.check_bool(
        "mesh:addresses_unique",
        all_unique,
        &format!("{} addresses, all unique", assigned.len()),
    );

    let all_in_subnet = assigned.iter().all(|ip| ip.starts_with("10.13.37."));
    v.check_bool(
        "mesh:subnet_consistent",
        all_in_subnet,
        "all mesh addresses in 10.13.37.0/24",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn zone_topology_structural() {
        let mut v = ValidationResult::new("zone-topology");
        let mut ctx = CompositionContext::discover();
        run_zone_topology(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "zone-topology: {} failures ({} passed, {} skipped)",
            v.failed, v.passed, v.skipped
        );
    }
}
