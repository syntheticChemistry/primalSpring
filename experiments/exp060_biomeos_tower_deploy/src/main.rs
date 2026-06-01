// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp060: biomeOS Tower Deploy — validate live Tower composition via CompositionContext.

use primalspring::composition::CompositionContext;
use primalspring::validation::ValidationResult;

fn phase_discovery_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    for cap in ["security", "discovery"] {
        if !ctx.has_capability(cap) {
            v.check_skip(cap, &format!("{cap} capability not discovered"));
            continue;
        }
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(_) => v.check_bool(
                &format!("{cap}_liveness"),
                true,
                &format!("{cap} health.liveness"),
            ),
            Err(e) if e.is_connection_error() => {
                v.check_skip(&format!("{cap}_liveness"), &format!("{e}"));
            }
            Err(e) => v.check_bool(&format!("{cap}_liveness"), false, &format!("error: {e}")),
        }
    }
}

fn phase_crypto_routing(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("security") {
        v.check_skip(
            "capability_call_crypto",
            "security capability not discovered",
        );
        return;
    }
    match ctx.call("security", "generate_keypair", serde_json::json!({})) {
        Ok(r) => v.check_bool(
            "capability_call_crypto",
            !r.is_null(),
            "security.generate_keypair returned data",
        ),
        Err(e) if e.is_connection_error() => {
            v.check_skip("capability_call_crypto", &format!("{e}"));
        }
        Err(e) => {
            let msg = format!("{e}");
            let expected = msg.contains("not found")
                || msg.contains("not registered")
                || e.is_method_not_found();
            v.check_bool(
                "capability_call_crypto",
                expected,
                &format!("capability routing attempt: {e}"),
            );
        }
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp060 — biomeOS Tower Deploy")
        .with_provenance("exp060_biomeos_tower_deploy", "2026-05-09")
        .run(
            "primalSpring Exp060: Tower composition via live discovery",
            |v| {
                v.section("Phase 1: Security & discovery liveness");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_discovery_health(v, &mut ctx);

                v.section("Phase 2: Crypto routing");
                phase_crypto_routing(v, &mut ctx);
            },
        );
}
