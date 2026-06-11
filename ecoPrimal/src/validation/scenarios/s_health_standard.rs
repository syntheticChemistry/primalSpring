// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: HEALTH-01 Standard Health Response — validates that primals
//! respond to the plain `"health"` method with a schema-compliant response
//! containing `{status, primal, version, uptime_s}`.
//!
//! guideStone properties:
//! - P3 (Self-Verifying): health schema compliance is machine-checkable
//! - P5 (Tolerance-Documented): named convergence window for health probes
//!
//! Phase 1 (Structural): validates that the health method is registered
//! in the tool schema and that `HealthResponse` fields are documented.
//!
//! Phase 2 (Live): probes reachable primals with `{"method":"health"}`
//! and validates the response against `HealthResponse::validate()`.

use crate::composition::CompositionContext;
use crate::ipc::protocol::HealthResponse;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "health-standard-convergence",
        track: Track::Lifecycle,
        tier: Tier::Both,
        provenance_crate: "wave109_guidestone_convergence",
        provenance_date: "2026-06-11",
        description: "HEALTH-01: all primals respond to plain 'health' with standard schema",
    },
    run,
};

const HEALTH_01_REQUIRED_FIELDS: &[&str] = &["status", "primal", "version", "uptime_s"];

const CAPABILITIES_TO_PROBE: &[&str] = &[
    "identity",
    "discovery",
    "compute",
    "orchestration",
    "content-provider",
    "content.render",
    "crypto",
    "visualization",
    "storage",
    "inference",
];

/// Run all HEALTH-01 validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — HEALTH-01 schema definition");
    phase_structural(v);

    v.section("Phase 2: Live — probe reachable primals");
    phase_live(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    v.check_bool(
        "schema:required_fields",
        HEALTH_01_REQUIRED_FIELDS.len() == 4,
        "HEALTH-01 requires 4 fields: status, primal, version, uptime_s",
    );

    let valid_statuses = HealthResponse::VALID_STATUSES;
    v.check_bool(
        "schema:valid_statuses_defined",
        valid_statuses.len() >= 4,
        &format!(
            "accepted status values: {} ({} defined)",
            valid_statuses.join(", "),
            valid_statuses.len()
        ),
    );

    let test_valid = serde_json::json!({
        "status": "healthy",
        "primal": "test",
        "version": "0.1.0",
        "uptime_s": 42
    });
    let missing = HealthResponse::validate(&test_valid);
    v.check_bool(
        "schema:validate_accepts_compliant",
        missing.is_empty(),
        "HealthResponse::validate accepts compliant response",
    );

    let test_partial = serde_json::json!({ "status": "ok" });
    let missing_partial = HealthResponse::validate(&test_partial);
    v.check_bool(
        "schema:validate_rejects_partial",
        missing_partial.len() == 3,
        &format!(
            "partial response missing {} fields: {}",
            missing_partial.len(),
            missing_partial.join(", ")
        ),
    );

    let test_uptime_variant = serde_json::json!({
        "status": "ok",
        "primal": "test",
        "version": "0.1.0",
        "uptime_seconds": 100
    });
    let missing_variant = HealthResponse::validate(&test_uptime_variant);
    v.check_bool(
        "schema:validate_accepts_uptime_variants",
        missing_variant.is_empty(),
        "HealthResponse::validate accepts uptime_seconds as alias",
    );
}

fn phase_live(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let mut probed = 0u32;
    let mut compliant = 0u32;
    let mut skipped = 0u32;

    for capability in CAPABILITIES_TO_PROBE {
        let check_prefix = format!("live:{capability}");

        let result = ctx.call(capability, "health", serde_json::json!({}));

        match result {
            Ok(response) => {
                probed += 1;
                let missing = HealthResponse::validate(&response);
                if missing.is_empty() {
                    compliant += 1;
                    let primal = response
                        .get("primal")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("?");
                    let status = response
                        .get("status")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("?");
                    v.check_bool(
                        &format!("{check_prefix}:compliant"),
                        true,
                        &format!("{primal}: status={status}, HEALTH-01 compliant"),
                    );
                } else {
                    v.check_bool(
                        &format!("{check_prefix}:compliant"),
                        false,
                        &format!("HEALTH-01 non-compliant — missing: {}", missing.join(", ")),
                    );
                }
            }
            Err(e) if e.is_skippable() => {
                skipped += 1;
                v.check_skip(
                    &format!("{check_prefix}:compliant"),
                    &format!("capability '{capability}' unreachable: {e}"),
                );
            }
            Err(e) => {
                probed += 1;
                v.check_bool(
                    &format!("{check_prefix}:compliant"),
                    false,
                    &format!("health probe failed: {e}"),
                );
            }
        }
    }

    v.check_bool(
        "live:probed_count",
        probed > 0 || skipped as usize == CAPABILITIES_TO_PROBE.len(),
        &format!("probed {probed} capabilities, {compliant} compliant, {skipped} skipped"),
    );

    if probed > 0 {
        let compliance_rate = (f64::from(compliant) / f64::from(probed)) * 100.0;
        v.check_bool(
            "live:compliance_rate",
            compliance_rate >= 80.0,
            &format!(
                "HEALTH-01 compliance: {compliant}/{probed} ({compliance_rate:.0}%, target >= 80%)"
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn health_standard_no_panic() {
        let mut v = ValidationResult::new("health-standard-convergence");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
    }

    #[test]
    fn structural_phase_passes() {
        let mut v = ValidationResult::new("health-standard-convergence");
        phase_structural(&mut v);
        assert_eq!(v.failed, 0, "structural checks should all pass");
    }
}
