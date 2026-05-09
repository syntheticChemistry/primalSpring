// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exp055: Wait For Health — discovery latency and health probe pattern (NestGate-style).

use std::time::Instant;

use primalspring::composition::CompositionContext;
use primalspring::tolerances::DISCOVERY_MAX_US;
use primalspring::validation::ValidationResult;

fn phase_discovery_timing(v: &mut ValidationResult) {
    let start = Instant::now();
    let _ctx = CompositionContext::from_live_discovery_with_fallback();
    let elapsed_us = u64::try_from(start.elapsed().as_micros()).unwrap_or(u64::MAX);
    v.check_bool(
        "composition_discovery_within_tolerance",
        elapsed_us <= DISCOVERY_MAX_US,
        &format!("CompositionContext discovery took {elapsed_us}µs (max: {DISCOVERY_MAX_US}µs)"),
    );
}

fn phase_health_probe(v: &mut ValidationResult) {
    let mut ctx = CompositionContext::from_live_discovery_with_fallback();
    let first_owned = {
        let caps = ctx.available_capabilities();
        caps.first().copied().map(str::to_string)
    };
    let Some(first_owned) = first_owned else {
        v.check_skip(
            "health_probe_loop",
            "no capabilities discovered — nothing to probe",
        );
        return;
    };
    let label = first_owned.clone();
    match ctx.call(
        first_owned.as_str(),
        "health.liveness",
        serde_json::json!({}),
    ) {
        Ok(_) => v.check_bool(
            "sample_health_liveness",
            true,
            &format!("{label} health.liveness ok"),
        ),
        Err(e) if e.is_connection_error() => {
            v.check_skip("health_probe_loop", &format!("{label}: {e}"));
        }
        Err(e) => v.check_bool("sample_health_liveness", false, &format!("error: {e}")),
    }

    v.check_skip(
        "exhaustive_health_probe_loop",
        "full NestGate-style wait loop needs orchestrated primal startup",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp055 — Wait For Health")
        .with_provenance("exp055_wait_for_health", "2026-05-09")
        .run(
            "primalSpring Exp055: Health Probe Pattern (NestGate start_ecosystem)",
            |v| {
                v.section("Phase 1: Discovery Timing");
                phase_discovery_timing(v);

                v.section("Phase 2: Health Probe");
                phase_health_probe(v);
            },
        );
}
