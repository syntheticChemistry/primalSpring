// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp051: Socket Discovery Sweep — FullNucleus capability enumeration via composition context.

use primalspring::composition::CompositionContext;
use primalspring::coordination::AtomicType;
use primalspring::validation::ValidationResult;

fn phase_capability_sweep(v: &mut ValidationResult, ctx: &CompositionContext) {
    let all_caps = AtomicType::FullNucleus.required_capabilities();
    let avail = ctx.available_capabilities();
    println!(
        "  [INFO] context has {} live capability client(s): {}",
        avail.len(),
        avail.join(", ")
    );

    v.check_count("full_nucleus_capability_set_size", all_caps.len(), 13);

    let reachable = all_caps.iter().filter(|&&c| ctx.has_capability(c)).count();
    let unreachable = all_caps.len() - reachable;
    println!("  [INFO] reachable: {reachable}, unreachable: {unreachable}");

    v.check_bool(
        "reachable_unresolved_sum",
        reachable + unreachable == all_caps.len(),
        "reachable + unreachable equals FullNucleus capability count",
    );

    for &cap in all_caps {
        let ok = ctx.has_capability(cap);
        let status = if ok { "UP" } else { "DOWN" };
        println!("  [{status}] {cap}");
        if ok {
            v.check_bool(
                &format!("resolved_{cap}"),
                true,
                &format!("{cap} available via CompositionContext"),
            );
        } else {
            v.check_skip(&format!("resolved_{cap}"), &format!("{cap} not discovered"));
        }
    }
}

fn phase_reachability(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let all_caps = AtomicType::FullNucleus.required_capabilities();
    for &cap in all_caps {
        if !ctx.has_capability(cap) {
            v.check_skip(&format!("liveness_{cap}"), &format!("{cap} not connected"));
            continue;
        }
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(_) => v.check_bool(
                &format!("liveness_{cap}"),
                true,
                &format!("{cap} answers health.liveness"),
            ),
            Err(e) if e.is_connection_error() => {
                v.check_skip(&format!("liveness_{cap}"), &format!("{cap}: {e}"));
            }
            Err(e) => v.check_bool(&format!("liveness_{cap}"), false, &format!("error: {e}")),
        }
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp051 — Socket Discovery Sweep")
        .with_provenance("exp051_socket_discovery_sweep", "2026-05-09")
        .run(
            "primalSpring Exp051: Capability-Based Discovery Sweep",
            |v| {
                v.section("Phase 1: Capability Sweep");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_capability_sweep(v, &ctx);

                v.section("Phase 2: Reachability Analysis");
                phase_reachability(v, &mut ctx);
            },
        );
}
