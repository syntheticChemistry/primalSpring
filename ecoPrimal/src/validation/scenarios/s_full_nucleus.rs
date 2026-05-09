// SPDX-License-Identifier: AGPL-3.0-or-later
//! Scenario: Full NUCLEUS — absorbed from exp004.

use crate::cast;
use crate::composition::CompositionContext;
use crate::coordination::AtomicType;
use crate::ipc::NeuralBridge;
use crate::tolerances;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};
use std::time::Instant;

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "full-nucleus",
        track: Track::AtomicComposition,
        tier: Tier::Both,
        provenance_crate: "exp004_full_nucleus",
        provenance_date: "2026-05-09",
        description: "Full NUCLEUS — validate all 13 capability domains",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural");
    phase_structural(v);

    v.section("Phase 2: Discovery");
    phase_discovery(v, ctx);

    v.section("Phase 3: Health");
    phase_health(v, ctx);

    v.section("Phase 4: Composition parity");
    phase_composition_parity(v, ctx);
}

fn capability_count_from_list_value(val: &serde_json::Value) -> usize {
    if let Some(a) = val.as_array() {
        return a.len();
    }
    val.as_object().map_or(0, serde_json::Map::len)
}

fn phase_structural(v: &mut ValidationResult) {
    let required = AtomicType::FullNucleus.required_capabilities();
    v.check_count("full_nucleus_required_caps", required.len(), 13);
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let required = AtomicType::FullNucleus.required_capabilities();
    let caps = ctx.available_capabilities();
    v.check_bool(
        "discovery_found_capabilities",
        !caps.is_empty(),
        &format!("{} capabilities: {}", caps.len(), caps.join(", ")),
    );
    for cap in required {
        v.check_bool(
            &format!("has_{cap}"),
            ctx.has_capability(cap),
            &format!("{cap} capability resolved"),
        );
    }
}

fn phase_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let required = AtomicType::FullNucleus.required_capabilities();
    for cap in required {
        match ctx.health_check(cap) {
            Ok(true) => v.check_bool(
                &format!("health_liveness_{cap}"),
                true,
                &format!("{cap} health.liveness ok"),
            ),
            Ok(false) => v.check_bool(
                &format!("health_liveness_{cap}"),
                false,
                &format!("{cap} not live"),
            ),
            Err(e) if e.is_connection_error() => v.check_skip(
                &format!("health_liveness_{cap}"),
                &format!("{cap} not reachable: {e}"),
            ),
            Err(e) => v.check_bool(
                &format!("health_liveness_{cap}"),
                false,
                &format!("{cap} health check error: {e}"),
            ),
        }
    }
}

fn phase_composition_parity(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let required = AtomicType::FullNucleus.required_capabilities();
    let bridge = NeuralBridge::discover();

    let substrate_ok = if ctx.has_capability("orchestration") {
        ctx.health_check("orchestration").unwrap_or(false)
    } else if let Some(ref b) = bridge {
        b.health_check().unwrap_or(false)
    } else {
        let msg = "Neural API substrate not discoverable";
        v.check_skip("composition_discovery_ok", msg);
        v.check_skip("composition_substrate", msg);
        v.check_skip("composition_all_healthy", msg);
        v.check_skip("composition_total_caps", msg);
        v.check_skip("composition_aggregate_latency", msg);
        return;
    };

    let latency_us = if ctx.has_capability("orchestration") {
        let start = Instant::now();
        let _ = ctx.health_check("orchestration");
        cast::micros_u64(start.elapsed())
    } else if let Some(ref b) = bridge {
        let start = Instant::now();
        let _ = b.health_check();
        cast::micros_u64(start.elapsed())
    } else {
        0
    };

    let discovery_ok = required.iter().all(|&cap| ctx.has_capability(cap));
    v.check_bool(
        "composition_discovery_ok",
        discovery_ok,
        "each Full Nucleus capability has a client in context",
    );

    let mut primal_health_ok = true;
    for cap in required {
        if !ctx.has_capability(cap) {
            primal_health_ok = false;
            continue;
        }
        match ctx.health_check(cap) {
            Ok(true) => {}
            _ => primal_health_ok = false,
        }
    }

    v.check_bool(
        "composition_substrate",
        substrate_ok,
        &format!("substrate health, ~{latency_us}us"),
    );

    v.check_bool(
        "composition_all_healthy",
        primal_health_ok && substrate_ok,
        "primal liveness parity and substrate health",
    );

    let mut total_caps = 0usize;
    for cap in required {
        if let Some(client) = ctx.client_for(cap) {
            if let Ok(j) = client.capabilities() {
                total_caps += capability_count_from_list_value(&j);
            }
        }
    }
    v.check_minimum("composition_total_caps", total_caps, 8);

    let mut aggregate_us = 0u64;
    for cap in required {
        if ctx.has_capability(cap) {
            let start = Instant::now();
            let _ = ctx.health_check(cap);
            aggregate_us = aggregate_us.saturating_add(cast::micros_u64(start.elapsed()));
        }
    }
    v.check_latency(
        "composition_aggregate_latency",
        aggregate_us,
        tolerances::NUCLEUS_STARTUP_MAX_US,
    );
}
