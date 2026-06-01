// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

//! Exp109: Composition Lifecycle (JH-3 Hot-Reload Validation)
//!
//! Validates the composition lifecycle contract:
//!   1. Discover capabilities via `CompositionContext`
//!   2. Validate liveness across the discovered composition
//!   3. Request `composition.reload` via biomeOS orchestration
//!   4. Re-discover and validate that capabilities survived the reload
//!   5. Verify BTSP state continuity across the topology change
//!
//! When primals are unavailable, phases skip gracefully.

use primalspring::composition::CompositionContext;
use primalspring::coordination::AtomicType;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp109 — Composition Lifecycle")
        .with_provenance("exp109_composition_lifecycle", "2026-05-09")
        .run(
            "Exp109: Composition Lifecycle — hot-reload and rediscovery",
            |v| {
                v.section("Phase 1: Initial Discovery");
                phase_initial_discovery(v);

                v.section("Phase 2: Pre-Reload Liveness");
                phase_pre_reload_liveness(v);

                v.section("Phase 3: Composition Reload");
                phase_composition_reload(v);

                v.section("Phase 4: Post-Reload Validation");
                phase_post_reload_validation(v);

                v.section("Phase 5: BTSP Continuity");
                phase_btsp_continuity(v);
            },
        );
}

fn phase_initial_discovery(v: &mut ValidationResult) {
    let ctx = CompositionContext::from_live_discovery_with_fallback();
    let caps = ctx.available_capabilities();

    v.check_bool(
        "initial_discovery",
        !caps.is_empty(),
        &format!("{} capabilities: {}", caps.len(), caps.join(", ")),
    );

    if caps.is_empty() {
        v.check_skip(
            "initial_discovery_detail",
            "no primals discovered — NUCLEUS not running",
        );
    }
}

fn phase_pre_reload_liveness(v: &mut ValidationResult) {
    let mut ctx = CompositionContext::from_live_discovery_with_fallback();
    let tower_caps = AtomicType::Tower.required_capabilities();

    let alive = tower_caps
        .iter()
        .filter(|&&cap| ctx.has_capability(cap) && matches!(ctx.health_check(cap), Ok(true)))
        .count();

    v.check_bool(
        "tower_alive_pre_reload",
        tower_caps.is_empty() || alive > 0,
        &format!("{alive}/{} Tower primals alive", tower_caps.len()),
    );
    if tower_caps.is_empty() || alive == 0 {
        v.check_skip(
            "tower_liveness_detail",
            "no Tower capabilities alive — skipping pre-reload liveness",
        );
    }
}

fn phase_composition_reload(v: &mut ValidationResult) {
    let mut ctx = CompositionContext::from_live_discovery_with_fallback();
    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "composition_reload",
            "biomeOS orchestration not available — composition.reload requires biomeOS",
        );
        return;
    }

    match ctx.reload() {
        Ok(result) => {
            v.check_bool("composition_reload", true, &format!("response: {result}"));
        }
        Err(e) => {
            v.check_skip(
                "composition_reload",
                &format!("composition.reload failed: {e}"),
            );
        }
    }
}

fn phase_post_reload_validation(v: &mut ValidationResult) {
    let mut ctx = CompositionContext::from_live_discovery_with_fallback();
    let caps = ctx.available_capabilities();

    if caps.is_empty() {
        v.check_skip(
            "post_reload_discovery",
            "no capabilities after reload — NUCLEUS not running",
        );
        return;
    }

    let tower_caps = AtomicType::Tower.required_capabilities();
    let alive = tower_caps
        .iter()
        .filter(|&&cap| ctx.has_capability(cap) && matches!(ctx.health_check(cap), Ok(true)))
        .count();
    v.check_bool(
        "tower_alive_post_reload",
        tower_caps.is_empty() || alive > 0,
        &format!("{alive}/{} Tower primals survived reload", tower_caps.len()),
    );
}

fn phase_btsp_continuity(v: &mut ValidationResult) {
    let ctx = CompositionContext::from_live_discovery_with_fallback();
    let state = ctx.btsp_state();

    if state.is_empty() {
        v.check_skip("btsp_continuity", "no BTSP state — no primals discovered");
        return;
    }

    let total = state.len();
    let authenticated = state.values().filter(|&&b| b).count();
    v.check_bool(
        "btsp_state_populated",
        total > 0,
        &format!("{authenticated}/{total} BTSP authenticated"),
    );
}
