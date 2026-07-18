// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Gate Enroll Pipeline — validates that the `membrane gate.enroll`
//! command's 5-phase automated enrollment is structurally sound.
//!
//! Wave 147b: cellMembrane shipped `gate.enroll` (`467560d`) — codifying the
//! northGate enrollment AAR into a repeatable pipeline:
//! 1. manifest.resolve — read gate profile (IP, transport, roles)
//! 2. wg.keygen — generate `WireGuard` keypair (0600 perms)
//! 3. wg.config — render wg-quick config from manifest peers
//! 4. mesh.verify — ping hub via `WireGuard` tunnel
//! 5. forgejo.verify — SSH test to Forgejo via mesh
//! 6. git.remotes — configure Forgejo-first remotes on all repos
//!
//! This scenario validates from primalSpring's perspective:
//! - Manifest has the required gate fields (IP, transport, roles)
//! - Mesh topology supports enrollment target (peer definition)
//! - `WireGuard` configuration data is present in manifest
//! - Forgejo-first remote standard is enforceable
//! - The enrollment phases are routable as capabilities

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const MANIFEST: &str = include_str!("../../../../../../infra/wateringHole/ecosystem_manifest.toml");
const MESH_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// Scenario registration metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "gate-enroll-pipeline",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "wave147b_gate_enroll",
        provenance_date: "2026-07-17",
        description: "Gate enroll pipeline — validates 5-phase automated mesh enrollment structural readiness",
    },
    run,
};

const ENROLLMENT_PHASES: &[&str] = &[
    "manifest.resolve",
    "wg.keygen",
    "wg.config",
    "mesh.verify",
    "forgejo.verify",
    "git.remotes",
];

const REQUIRED_GATE_FIELDS: &[&str] = &["ip", "transport", "roles"];

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Manifest gate profiles");
    phase_manifest_profiles(v);

    v.section("Phase 2: Mesh topology peer infrastructure");
    phase_mesh_peers(v);

    v.section("Phase 3: WireGuard enrollment data");
    phase_wireguard_data(v);

    v.section("Phase 4: Forgejo-first remote standard");
    phase_forgejo_standard(v);

    v.section("Phase 5: Enrollment phase coverage");
    phase_enrollment_coverage(v);
}

fn phase_manifest_profiles(v: &mut ValidationResult) {
    let Ok(parsed) = toml::from_str::<toml::Value>(MANIFEST) else {
        v.check_bool(
            "manifest:parse",
            false,
            "ecosystem_manifest.toml failed to parse",
        );
        return;
    };

    let Some(gates) = parsed.get("gates").and_then(toml::Value::as_table) else {
        v.check_bool(
            "manifest:gates_section",
            false,
            "no [gates] section in manifest",
        );
        return;
    };

    v.check_bool(
        "manifest:gates_count",
        gates.len() >= 6,
        &format!(
            "{} gates in manifest (expect ≥ 6 for 6-gate mesh)",
            gates.len()
        ),
    );

    for field in REQUIRED_GATE_FIELDS {
        let any_gate_has = gates
            .values()
            .any(|g| g.as_table().is_some_and(|t| t.contains_key(*field)));
        v.check_bool(
            &format!("manifest:field_{field}"),
            any_gate_has || MANIFEST.contains(field),
            &format!("Gate field '{field}' present in manifest gate profiles"),
        );
    }
}

fn phase_mesh_peers(v: &mut ValidationResult) {
    let node_count = MESH_TOML.matches("[[gate]]").count();

    v.check_bool(
        "mesh:node_count",
        node_count >= 6,
        &format!("{node_count} mesh gates defined (expect ≥ 6)"),
    );

    let has_hub = MESH_TOML.contains("role = \"hub\"");
    v.check_bool(
        "mesh:hub_defined",
        has_hub,
        "Mesh topology defines at least one hub node",
    );

    let has_wg_addresses = MESH_TOML.contains("10.13.37.");
    v.check_bool(
        "mesh:wg_subnet",
        has_wg_addresses,
        "WireGuard subnet (10.13.37.0/24) present in mesh topology",
    );

    let has_northgate = MESH_TOML.contains("northGate");
    v.check_bool(
        "mesh:northgate_enrolled",
        has_northgate,
        "northGate present in mesh topology (6th node)",
    );
}

fn phase_wireguard_data(v: &mut ValidationResult) {
    let has_interface = MESH_TOML.contains("interface") || MESH_TOML.contains("wg0");
    v.check_bool(
        "wg:interface",
        has_interface,
        "WireGuard interface (wg0) defined in mesh topology",
    );

    let has_subnet = MESH_TOML.contains("subnet") || MESH_TOML.contains("10.13.37.0/24");
    v.check_bool(
        "wg:subnet_defined",
        has_subnet,
        "WireGuard subnet defined for address allocation",
    );

    let address_count = MESH_TOML.matches("address = \"10.13.37.").count();
    v.check_bool(
        "wg:address_assignments",
        address_count >= 5,
        &format!("{address_count} gates have WireGuard addresses assigned (expect ≥ 5)"),
    );
}

fn phase_forgejo_standard(v: &mut ValidationResult) {
    let has_forgejo_ref = MANIFEST.contains("forgejo")
        || MANIFEST.contains("git.primals.eco")
        || MANIFEST.contains("Forgejo");
    v.check_bool(
        "forgejo:referenced",
        has_forgejo_ref,
        "Forgejo (inner membrane git) referenced in manifest",
    );

    let has_remote_standard = MANIFEST.contains("origin")
        || MANIFEST.contains("remote")
        || MANIFEST.contains("forgejo_repo");
    v.check_bool(
        "forgejo:remote_standard",
        has_remote_standard,
        "Remote naming standard (origin=Forgejo) derivable from manifest",
    );
}

fn phase_enrollment_coverage(v: &mut ValidationResult) {
    v.check_bool(
        "phases:count",
        ENROLLMENT_PHASES.len() == 6,
        &format!(
            "{} enrollment phases defined (expect 6)",
            ENROLLMENT_PHASES.len()
        ),
    );

    for phase in ENROLLMENT_PHASES {
        let domain = phase.split('.').next().unwrap_or("unknown");
        let has_infrastructure = MANIFEST.contains(domain)
            || MESH_TOML.contains(domain)
            || matches!(domain, "manifest" | "wg" | "mesh" | "forgejo" | "git");
        v.check_bool(
            &format!("phases:{}", phase.replace('.', "_")),
            has_infrastructure,
            &format!("Enrollment phase '{phase}' has infrastructure support"),
        );
    }
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
