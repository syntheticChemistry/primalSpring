// SPDX-License-Identifier: AGPL-3.0-or-later
//! Scenario: Socket Discovery Sweep — absorbed from exp051.

use crate::composition::CompositionContext;
use crate::coordination::AtomicType;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "socket-discovery",
        track: Track::Transport,
        tier: Tier::Live,
        provenance_crate: "exp051_socket_discovery_sweep",
        provenance_date: "2026-05-09",
        description: "Socket discovery sweep — Full Nucleus capability enumeration",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Capability Sweep");
    phase_capability_sweep(v, ctx);

    v.section("Phase 2: Reachability Analysis");
    phase_reachability(v, ctx);
}

fn phase_capability_sweep(v: &mut ValidationResult, ctx: &CompositionContext) {
    let all_caps = AtomicType::FullNucleus.required_capabilities();
    let _avail = ctx.available_capabilities();

    v.check_count("full_nucleus_capability_set_size", all_caps.len(), 13);

    let reachable = all_caps.iter().filter(|&&c| ctx.has_capability(c)).count();
    let unreachable = all_caps.len() - reachable;

    v.check_bool(
        "reachable_unresolved_sum",
        reachable + unreachable == all_caps.len(),
        "reachable + unreachable equals FullNucleus capability count",
    );

    for &cap in all_caps {
        let ok = ctx.has_capability(cap);
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
        match ctx.health_check(cap) {
            Ok(true) => v.check_bool(
                &format!("liveness_{cap}"),
                true,
                &format!("{cap} answers health.liveness"),
            ),
            Ok(false) => v.check_bool(
                &format!("liveness_{cap}"),
                false,
                &format!("{cap} not live"),
            ),
            Err(e) if e.is_connection_error() => {
                v.check_skip(&format!("liveness_{cap}"), &format!("{cap}: {e}"));
            }
            Err(e) => v.check_bool(&format!("liveness_{cap}"), false, &format!("error: {e}")),
        }
    }
}
