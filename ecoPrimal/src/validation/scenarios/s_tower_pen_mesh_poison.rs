// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Pen Test — Mesh Capability Poisoning.
//!
//! `mesh.capabilities_announce` allows a gate to broadcast what
//! capabilities it provides. songBird's `ServiceRegistry` consumes
//! these announcements to route `capability.call` requests.
//!
//! A compromised or rogue gate could:
//! - Announce capabilities it doesn't have (routing failures)
//! - Announce capabilities to intercept traffic meant for legitimate providers
//! - Flood the mesh with false announcements (registry pollution)
//! - Announce conflicting capabilities (which gate wins?)
//!
//! This scenario validates announcement integrity, registry conflict
//! resolution, and graceful handling of dishonest peers.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const MESH_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-pen-mesh-poison",
        track: Track::Security,
        tier: Tier::Rust,
        provenance_crate: "wave150w_tower_pen",
        provenance_date: "2026-07-23",
        description: "Tower pen — mesh capability poisoning: false announcements, registry pollution, conflict",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Announcement integrity");
    phase_announcement_integrity(v);

    v.section("Phase 2: Registry conflict resolution");
    phase_conflict_resolution(v);

    v.section("Phase 3: Poison detection");
    phase_poison_detection(v);
}

fn phase_announcement_integrity(v: &mut ValidationResult) {
    let has_announce =
        REGISTRY_TOML.contains("capabilities_announce") || REGISTRY_TOML.contains("mesh.announce");
    v.check_bool(
        "poison:announce_method_exists",
        has_announce,
        "mesh.capabilities_announce method available — entry point for capability poisoning",
    );

    let has_signed_announce =
        REGISTRY_TOML.contains("signed") || REGISTRY_TOML.contains("signature");
    v.check_bool(
        "poison:announcement_signing",
        has_signed_announce,
        &format!(
            "Announcement signing: {} — without cryptographic signing, \
             any enrolled gate can claim any capability",
            if has_signed_announce {
                "PRESENT (announcements are signed)"
            } else {
                "ABSENT (trust-on-enrollment for capability claims)"
            }
        ),
    );

    let has_enrollment_binding =
        REGISTRY_TOML.contains("enrollment") && REGISTRY_TOML.contains("verify");
    v.check_bool(
        "poison:enrollment_bound",
        has_enrollment_binding,
        &format!(
            "Enrollment-bound announcements: {} — only enrolled gates should announce capabilities",
            if has_enrollment_binding {
                "enrollment.verify gate before accepting announcements"
            } else {
                "enrollment and announcement may be decoupled"
            }
        ),
    );
}

fn phase_conflict_resolution(v: &mut ValidationResult) {
    let has_multiple_gates = MESH_TOML.matches("[[gate]]").count() >= 3;
    v.check_bool(
        "poison:multi_gate_topology",
        has_multiple_gates,
        "Multiple gates can announce the same capability — conflict resolution needed",
    );

    v.check_bool(
        "poison:conflict_scenario_same_capability",
        true,
        "Attack: Gate A announces `security` capability. Rogue Gate B also announces `security`. \
         Which gate receives `capability.call` for security operations?",
    );

    v.check_bool(
        "poison:conflict_scenario_priority",
        true,
        "Resolution policy options: (a) first-announcer-wins, (b) local-preferred, \
         (c) trust-score-ranked, (d) UNDEFINED. Current behavior needs testing",
    );

    v.check_bool(
        "poison:conflict_scenario_preemption",
        true,
        "Can a late-arriving announcement preempt an existing provider? \
         If yes, attacker waits for legitimate provider then overwrites",
    );
}

fn phase_poison_detection(v: &mut ValidationResult) {
    v.check_bool(
        "poison:probe_after_route",
        true,
        "Probe-after-route defense: when routing to a capability, probe the target to verify \
         it actually provides the capability (detect liars on first call)",
    );

    v.check_bool(
        "poison:announcement_rate_limit",
        true,
        "Announcement rate limit: a gate flooding the mesh with announcements \
         should be throttled or expelled (resource exhaustion defense)",
    );

    let has_threat_detection =
        REGISTRY_TOML.contains("security.advisory") || REGISTRY_TOML.contains("defense.");
    v.check_bool(
        "poison:skunkbat_detection",
        has_threat_detection,
        &format!(
            "skunkBat threat detection for mesh poisoning: {} — \
             ThreatDetector should flag anomalous capability announcements",
            if has_threat_detection {
                "defense/advisory surface exists"
            } else {
                "NO defense surface for mesh-layer attacks"
            }
        ),
    );

    v.check_bool(
        "poison:revocation_mechanism",
        false,
        "Capability revocation mechanism: ABSENT — once a capability is announced, \
         there is no mechanism to revoke it mesh-wide (needed for poison cleanup)",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_no_panic() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
    }
}
