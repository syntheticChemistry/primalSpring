// SPDX-License-Identifier: AGPL-3.0-or-later
//! Scenario: BearDog FIDO2 — IPC surface validation for UB-2.
//!
//! Validates the three FIDO2/CTAP2 methods shipped in BearDog Wave 103:
//! `beardog.fido2.discover`, `beardog.fido2.register`, `beardog.fido2.authenticate`.
//!
//! Rust tier: verifies the methods are present in the capability registry.
//! Live tier: probes discover (empty array without hardware is valid),
//! register/authenticate return proper error shapes without hardware
//! (expected: method exists but returns "no device" or empty result).
//!
//! The `ctap2` feature gate means builds without the feature will return
//! graceful empty-discover + clear error messages — both are valid.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const FIDO2_METHODS: &[&str] = &[
    "beardog.fido2.discover",
    "beardog.fido2.register",
    "beardog.fido2.authenticate",
];

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "beardog-fido2",
        track: Track::Security,
        tier: Tier::Both,
        provenance_crate: "beardog_wave103_ub2",
        provenance_date: "2026-05-16",
        description: "BearDog FIDO2/CTAP2: discover + register + authenticate IPC surface",
    },
    run,
};

/// Run the FIDO2 validation.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: FIDO2 registry presence (Rust tier)");
    phase_registry(v, ctx);

    v.section("Phase 2: beardog.fido2.discover (Live tier)");
    phase_discover(v, ctx);

    v.section("Phase 3: beardog.fido2.register error shape (Live tier)");
    phase_register_error(v, ctx);

    v.section("Phase 4: beardog.fido2.authenticate error shape (Live tier)");
    phase_authenticate_error(v, ctx);
}

fn phase_registry(v: &mut ValidationResult, _ctx: &CompositionContext) {
    let registry_content = include_str!("../../../../config/capability_registry.toml");

    for method in FIDO2_METHODS {
        let present = registry_content.contains(method);
        v.check_bool(
            &format!("fido2:registry:{method}"),
            present,
            &format!("{method} {} in capability_registry.toml", if present { "present" } else { "MISSING" }),
        );
    }

    let has_fido2_section = registry_content.contains("[fido2]");
    v.check_bool(
        "fido2:registry:section",
        has_fido2_section,
        &format!("[fido2] section {} in registry", if has_fido2_section { "present" } else { "MISSING" }),
    );
}

fn phase_discover(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let has_security = ctx.has_capability("security");
    if !has_security {
        v.check_skip("fido2:discover:responds", "BearDog security capability not discoverable");
        return;
    }

    match ctx.call(
        "security",
        "beardog.fido2.discover",
        serde_json::json!({}),
    ) {
        Ok(resp) => {
            let devices = resp.get("devices").and_then(|d| d.as_array());
            let count = devices.map_or(0, Vec::len);
            v.check_bool(
                "fido2:discover:responds",
                true,
                &format!("beardog.fido2.discover returned {count} device(s) (0 is valid without hardware)"),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("fido2:discover:responds", &format!("BearDog not available: {e}"));
        }
        Err(e) => {
            let msg = e.to_string();
            let is_capability_error = msg.contains("not available")
                || msg.contains("ctap2")
                || msg.contains("not enabled");
            v.check_bool(
                "fido2:discover:responds",
                is_capability_error,
                &format!("discover error (acceptable if ctap2 feature not enabled): {e}"),
            );
        }
    }
}

fn phase_register_error(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("security") {
        v.check_skip("fido2:register:error_shape", "BearDog not discoverable");
        return;
    }

    match ctx.call(
        "security",
        "beardog.fido2.register",
        serde_json::json!({
            "rp_id": "primalspring.test",
            "user_id": "scenario-test",
            "user_name": "test@primalspring.test",
        }),
    ) {
        Ok(_resp) => {
            v.check_bool(
                "fido2:register:error_shape",
                true,
                "register succeeded (hardware present or mock mode)",
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("fido2:register:error_shape", &format!("BearDog not available: {e}"));
        }
        Err(e) => {
            let msg = e.to_string();
            let is_expected = msg.contains("no device")
                || msg.contains("not available")
                || msg.contains("ctap2")
                || msg.contains("not enabled")
                || msg.contains("No FIDO2");
            v.check_bool(
                "fido2:register:error_shape",
                is_expected,
                &format!("register error (expected without hardware): {e}"),
            );
        }
    }
}

fn phase_authenticate_error(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("security") {
        v.check_skip("fido2:authenticate:error_shape", "BearDog not discoverable");
        return;
    }

    match ctx.call(
        "security",
        "beardog.fido2.authenticate",
        serde_json::json!({
            "rp_id": "primalspring.test",
            "credential_id": "nonexistent-credential",
        }),
    ) {
        Ok(_resp) => {
            v.check_bool(
                "fido2:authenticate:error_shape",
                true,
                "authenticate succeeded (unexpected without prior registration, but valid)",
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("fido2:authenticate:error_shape", &format!("BearDog not available: {e}"));
        }
        Err(e) => {
            let msg = e.to_string();
            let is_expected = msg.contains("no device")
                || msg.contains("not available")
                || msg.contains("ctap2")
                || msg.contains("not enabled")
                || msg.contains("No FIDO2")
                || msg.contains("credential");
            v.check_bool(
                "fido2:authenticate:error_shape",
                is_expected,
                &format!("authenticate error (expected without hardware): {e}"),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beardog_fido2_no_panic() {
        let mut v = ValidationResult::new("beardog-fido2");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(v.evaluated() > 0 || v.skipped > 0, "scenario should produce checks");
    }
}
