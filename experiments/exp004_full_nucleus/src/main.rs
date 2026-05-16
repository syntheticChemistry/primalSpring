// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp004: Full NUCLEUS — validate all 13 capability domains.
//!
//! Phases:
//!   1. Structural — Full Nucleus capability count
//!   2. Discovery — `CompositionContext` resolves each domain
//!   3. Health — `health.liveness` per required capability
//!   4. Composition parity — discovery, substrate, capability breadth, aggregate probe latency

use primalspring::composition::CompositionContext;
use primalspring::coordination::{AtomicType, probe_substrate};
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

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
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(_) => v.check_bool(
                &format!("health_liveness_{cap}"),
                true,
                &format!("{cap} health.liveness ok"),
            ),
            Err(e) if e.is_connection_error() => v.check_skip(
                &format!("health_liveness_{cap}"),
                &format!("{cap} not reachable: {e}"),
            ),
            Err(e) => v.check_bool(
                &format!("health_liveness_{cap}"),
                false,
                &format!("{cap} health.liveness error: {e}"),
            ),
        }
    }
}

fn phase_composition_parity(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let required = AtomicType::FullNucleus.required_capabilities();
    let substrate = probe_substrate();

    let Some(s) = substrate.as_ref() else {
        let msg = "Neural API substrate not discoverable";
        v.check_skip("composition_discovery_ok", msg);
        v.check_skip("composition_substrate", msg);
        v.check_skip("composition_all_healthy", msg);
        v.check_skip("composition_total_caps", msg);
        v.check_skip("composition_aggregate_latency", msg);
        return;
    };

    let discovery_ok = required.iter().all(|&cap| ctx.has_capability(cap));
    v.check_bool(
        "composition_discovery_ok",
        discovery_ok,
        "each Full Nucleus capability has a client in context",
    );

    let mut primal_health_ok = true;
    let mut aggregate_us: u64 = 0;
    for cap in required {
        if !ctx.has_capability(cap) {
            primal_health_ok = false;
            continue;
        }
        let start = std::time::Instant::now();
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(_) => {
                aggregate_us += start.elapsed().as_micros() as u64;
            }
            Err(_) => {
                aggregate_us += start.elapsed().as_micros() as u64;
                primal_health_ok = false;
            }
        }
    }

    let substrate_ok = s.health_ok;
    v.check_bool(
        "composition_substrate",
        substrate_ok,
        &format!("Neural API substrate, {}us", s.latency_us),
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

    v.check_latency(
        "composition_aggregate_latency",
        aggregate_us,
        tolerances::NUCLEUS_STARTUP_MAX_US,
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp004 — Full NUCLEUS")
        .with_provenance("exp004_full_nucleus", "2026-05-09")
        .run(
            "primalSpring Exp004: Full NUCLEUS (all capability domains)",
            |v| {
                v.section("Phase 1: Structural");
                phase_structural(v);

                v.section("Phase 2: Discovery");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_discovery(v, &ctx);

                v.section("Phase 3: Health");
                phase_health(v, &mut ctx);

                v.section("Phase 4: Composition parity");
                phase_composition_parity(v, &mut ctx);
            },
        );
}
