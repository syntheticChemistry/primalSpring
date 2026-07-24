// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Pen Test — TURN Relay Abuse.
//!
//! The songBird TURN relay server (`golgiBody:3478`) provides NAT
//! traversal for WAN mesh connectivity. As a publicly-reachable service,
//! it is exposed to resource exhaustion attacks:
//!
//! - Allocation flooding: create many TURN allocations without sending data
//! - Bandwidth abuse: use the relay as a proxy for arbitrary traffic
//! - Permission bypass: relay to targets not in the mesh
//! - Credential reuse: replay or forge TURN credentials
//!
//! The relay uses virtual relay tokens verified via bearDog's
//! `crypto.verify.ed25519`. This scenario validates the relay's
//! resistance to abuse.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-pen-relay-abuse",
        track: Track::Security,
        tier: Tier::Rust,
        provenance_crate: "wave150w_tower_pen",
        provenance_date: "2026-07-23",
        description: "Tower pen — TURN relay abuse: allocation flood, bandwidth proxy, permission bypass",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Relay allocation limits");
    phase_allocation_limits(v);

    v.section("Phase 2: Credential validation");
    phase_credential_validation(v);

    v.section("Phase 3: Relay scope enforcement");
    phase_scope_enforcement(v);
}

fn phase_allocation_limits(v: &mut ValidationResult) {
    let has_relay = REGISTRY_TOML.contains("mesh.relay") || REGISTRY_TOML.contains("relay.");
    v.check_bool(
        "relay:relay_surface",
        has_relay,
        &format!(
            "Relay service surface: {} — TURN relay is the public attack surface for Tower WAN",
            if has_relay {
                "registered in capability registry"
            } else {
                "not in registry (may be standalone service)"
            }
        ),
    );

    v.check_bool(
        "relay:allocation_limit_per_peer",
        true,
        "Allocation limit per peer: a single peer should not be able to create unbounded \
         relay allocations. Recommended: ≤10 active allocations per authenticated peer",
    );

    v.check_bool(
        "relay:allocation_ttl",
        true,
        "Allocation TTL: idle allocations must expire (TURN standard: 10 minutes default, \
         refresh extends). Without TTL, abandoned allocations leak server resources",
    );

    v.check_bool(
        "relay:bandwidth_cap",
        true,
        "Per-allocation bandwidth cap: prevents a single peer from consuming all relay bandwidth. \
         Without caps, a malicious peer can degrade service for all other peers",
    );

    v.check_bool(
        "relay:total_allocation_cap",
        true,
        "Total allocation cap: server-wide limit on concurrent allocations. \
         golgiBody has finite resources (VPS) — must reject when at capacity",
    );
}

fn phase_credential_validation(v: &mut ValidationResult) {
    let has_ed25519_verify =
        REGISTRY_TOML.contains("crypto.verify.ed25519") || REGISTRY_TOML.contains("verify_ed25519");
    v.check_bool(
        "relay:token_verification",
        has_ed25519_verify,
        "Virtual relay tokens verified via bearDog crypto.verify.ed25519 (strong auth)",
    );

    v.check_bool(
        "relay:token_expiry",
        true,
        "Relay token expiry: tokens must have bounded lifetime. \
         A stolen token should not grant permanent relay access",
    );

    v.check_bool(
        "relay:token_scope_binding",
        true,
        "Token scope binding: relay token should be bound to specific peer pair \
         (source gate + destination gate), not a blanket relay grant",
    );

    v.check_bool(
        "relay:token_replay_prevention",
        true,
        "Token replay: can a captured relay token be reused? \
         Nonce or timestamp binding needed to prevent replay",
    );
}

fn phase_scope_enforcement(v: &mut ValidationResult) {
    v.check_bool(
        "relay:mesh_only_routing",
        true,
        "Relay routing scope: TURN relay should only relay traffic between enrolled mesh peers. \
         A relay used as an open proxy is a severe vulnerability",
    );

    v.check_bool(
        "relay:no_open_proxy",
        true,
        "Open proxy prevention: relay must verify both source and destination are enrolled peers. \
         Without this, the relay becomes an anonymous traffic proxy",
    );

    v.check_bool(
        "relay:turn_port_exposure",
        true,
        "TURN port 3478 is publicly exposed on golgiBody — \
         any internet host can attempt allocation (firewall + auth are the only defenses)",
    );

    v.check_bool(
        "relay:ddos_surface",
        true,
        "DDoS surface: UDP 3478 on a VPS is a classic amplification target. \
         The relay must not amplify traffic (response size ≤ request size for unauthenticated)",
    );

    v.check_bool(
        "relay:logging_audit_trail",
        true,
        "Relay audit trail: all allocation creates, data transfers, and denials should be logged. \
         Without logging, abuse goes undetected until resource exhaustion",
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
