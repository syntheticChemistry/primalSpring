// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Pen Test — Capability Escalation.
//!
//! `capability.call` routes requests based on a `routing` parameter
//! (e.g., `"local"`, `"any"`, `"gate:sporeGate"`). An attacker on a
//! remote gate could attempt:
//!
//! - Call `capability.call` with `routing: "any"` for capabilities that
//!   should be restricted to local access (e.g., `PostPrimordial` operations)
//! - Invoke admin-only methods via capability routing (bypassing skunkBat
//!   method gate by routing through songBird dispatch)
//! - Cross-gate capability injection (claim a capability on gate A,
//!   have it routed from gate B)
//!
//! This scenario validates the authorization boundaries in the
//! capability routing system.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const DISPATCH_SRC: &str = include_str!(
    "../../../../../../primals/songBird/crates/songbird-universal-ipc/src/service/capability_dispatch.rs"
);

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-pen-capability-escalation",
        track: Track::Security,
        tier: Tier::Rust,
        provenance_crate: "wave150w_tower_pen",
        provenance_date: "2026-07-23",
        description: "Tower pen — capability escalation: routing abuse, local-only bypass, admin method access",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Routing authorization model");
    phase_routing_auth(v);

    v.section("Phase 2: Local-only capability protection");
    phase_local_only(v);

    v.section("Phase 3: Cross-gate injection");
    phase_cross_gate_injection(v);
}

fn phase_routing_auth(v: &mut ValidationResult) {
    let has_routing_param = DISPATCH_SRC.contains("routing") || DISPATCH_SRC.contains("Routing");
    v.check_bool(
        "escalation:routing_parameter",
        has_routing_param,
        &format!(
            "Routing parameter in capability.call: {} — determines local vs remote dispatch",
            if has_routing_param {
                "PRESENT"
            } else {
                "NOT FOUND"
            }
        ),
    );

    let has_routing_validation = DISPATCH_SRC.contains("validate_routing")
        || DISPATCH_SRC.contains("check_routing")
        || DISPATCH_SRC.contains("allowed_routing");
    v.check_bool(
        "escalation:routing_validation",
        has_routing_validation,
        &format!(
            "Routing validation: {} — determines if caller is authorized for requested route",
            if has_routing_validation {
                "PRESENT"
            } else {
                "NOT FOUND (any caller can request any routing mode)"
            }
        ),
    );

    let has_caller_identity = DISPATCH_SRC.contains("caller_id")
        || DISPATCH_SRC.contains("peer_info")
        || DISPATCH_SRC.contains("source_gate")
        || DISPATCH_SRC.contains("origin");
    v.check_bool(
        "escalation:caller_identification",
        has_caller_identity,
        &format!(
            "Caller identification in dispatch: {} — without caller identity, \
             cannot distinguish local from remote requests",
            if has_caller_identity {
                "PRESENT"
            } else {
                "ABSENT (all callers treated equally)"
            }
        ),
    );
}

fn phase_local_only(v: &mut ValidationResult) {
    let has_local_only_check = DISPATCH_SRC.contains("local_only")
        || DISPATCH_SRC.contains("LocalOnly")
        || DISPATCH_SRC.contains("restrict_remote");
    v.check_bool(
        "escalation:local_only_enforcement",
        has_local_only_check,
        &format!(
            "Local-only capability enforcement: {} — some capabilities (e.g., crypto key ops) \
             should never be remotely callable",
            if has_local_only_check {
                "PRESENT"
            } else {
                "ABSENT (remote callers can invoke any local capability)"
            }
        ),
    );

    let sensitive_caps = ["crypto.generate", "auth.exchange_trust", "secrets.store"];
    let mut found_in_registry = 0;
    for cap in &sensitive_caps {
        if REGISTRY_TOML.contains(cap) {
            found_in_registry += 1;
        }
    }
    v.check_bool(
        "escalation:sensitive_caps_registered",
        found_in_registry > 0,
        &format!(
            "{found_in_registry}/{} sensitive capabilities in registry — \
             these MUST NOT be remotely dispatchable via routing:any",
            sensitive_caps.len()
        ),
    );

    let has_postprimordial =
        DISPATCH_SRC.contains("post_primordial") || DISPATCH_SRC.contains("`PostPrimordial`");
    v.check_bool(
        "escalation:postprimordial_protection",
        has_postprimordial,
        &format!(
            "`PostPrimordial` capability protection: {} — critical inter-communication primals \
             need hard enforcement against remote escalation",
            if has_postprimordial {
                "PRESENT"
            } else {
                "NOT IN DISPATCH (enforcement may be elsewhere)"
            }
        ),
    );
}

fn phase_cross_gate_injection(v: &mut ValidationResult) {
    let has_capability_announce =
        REGISTRY_TOML.contains("capabilities_announce") || REGISTRY_TOML.contains("mesh.announce");
    v.check_bool(
        "escalation:capability_announcement",
        has_capability_announce,
        "Capability announcement exists — a rogue gate could announce false capabilities",
    );

    let has_announcement_validation = DISPATCH_SRC.contains("validate_announce")
        || DISPATCH_SRC.contains("trust_announce")
        || DISPATCH_SRC.contains("verify_capability");
    v.check_bool(
        "escalation:announcement_validation",
        has_announcement_validation,
        &format!(
            "Capability announcement validation: {} — without validation, \
             a compromised gate can claim any capability and intercept dispatch",
            if has_announcement_validation {
                "PRESENT (announcements verified)"
            } else {
                "ABSENT (trust-on-first-use for capability claims)"
            }
        ),
    );

    let has_remote_dispatch =
        DISPATCH_SRC.contains("remote_dispatch") || DISPATCH_SRC.contains("remote_call");
    v.check_bool(
        "escalation:remote_dispatch_path",
        has_remote_dispatch,
        &format!(
            "Remote dispatch path: {} — pen test target for cross-gate capability spoofing",
            if has_remote_dispatch {
                "PRESENT"
            } else {
                "NOT IN DISPATCH (may use separate module)"
            }
        ),
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
