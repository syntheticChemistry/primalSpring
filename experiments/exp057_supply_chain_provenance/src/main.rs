// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp057: Supply Chain Provenance — dag, ledger, and commit (attribution) capabilities in context.

use primalspring::composition::CompositionContext;
use primalspring::validation::ValidationResult;

const PROVENANCE_CAPS: &[&str] = &["dag", "ledger", "attribution"];

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let avail = ctx.available_capabilities().join(", ");
    println!("  [INFO] capabilities: {avail}");

    for cap in PROVENANCE_CAPS {
        if ctx.has_capability(cap) {
            v.check_bool(
                &format!("has_{cap}"),
                true,
                &format!("{cap} capability present"),
            );
        } else {
            v.check_skip(&format!("has_{cap}"), &format!("{cap} not in context"));
        }
    }
}

fn phase_pipeline(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let mut any = false;
    for cap in PROVENANCE_CAPS {
        if !ctx.has_capability(cap) {
            continue;
        }
        any = true;
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(_) => v.check_bool(
                &format!("provenance_pipeline_{cap}_liveness"),
                true,
                &format!("{cap} health.liveness ok"),
            ),
            Err(e) if e.is_connection_error() => v.check_skip(
                &format!("provenance_pipeline_{cap}_liveness"),
                &format!("{e}"),
            ),
            Err(e) => v.check_bool(
                &format!("provenance_pipeline_{cap}_liveness"),
                false,
                &format!("error: {e}"),
            ),
        }
    }
    if !any {
        v.check_skip(
            "provenance_pipeline",
            "no dag/ledger/attribution clients in context",
        );
    }

    v.check_skip(
        "actual_dag_execution",
        "DAG execution with signing needs live rhizoCrypt / loamSpine / sweetGrass orchestration",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp057 — Supply Chain Provenance")
        .with_provenance("exp057_supply_chain_provenance", "2026-05-09")
        .run(
            "primalSpring Exp057: rhizoCrypt Farm-to-Table Provenance",
            |v| {
                v.section("Phase 1: Discovery");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_discovery(v, &ctx);

                v.section("Phase 2: Pipeline");
                phase_pipeline(v, &mut ctx);
            },
        );
}
