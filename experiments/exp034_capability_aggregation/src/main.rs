// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp034: Capability Aggregation — Plasmodium routes to best gate when multiple primals expose capabilities.

use primalspring::composition::CompositionContext;
use primalspring::coordination::AtomicType;
use primalspring::ipc::discover::extract_capability_names;
use primalspring::validation::ValidationResult;

const MIN_AGGREGATION_PRIMALS: usize = 2;
const FULL_NUCLEUS_CAPS: usize = 13;

fn phase_structural(v: &mut ValidationResult) {
    let n = AtomicType::FullNucleus.required_capabilities().len();
    v.check_count(
        "full_nucleus_required_capability_count",
        n,
        FULL_NUCLEUS_CAPS,
    );
}

fn phase_capability_aggregation(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let live_caps: Vec<String> = ctx
        .available_capabilities()
        .into_iter()
        .map(str::to_owned)
        .collect();
    if live_caps.len() >= MIN_AGGREGATION_PRIMALS {
        let mut all_cap_names = Vec::new();
        for cap in &live_caps {
            match ctx.call(cap, "capabilities.list", serde_json::json!({})) {
                Ok(val) => all_cap_names.extend(extract_capability_names(Some(val))),
                Err(e) if e.is_connection_error() => {}
                Err(e) => {
                    v.check_bool(
                        &format!("capabilities_list_{cap}"),
                        false,
                        &format!("error: {e}"),
                    );
                }
            }
        }
        v.check_minimum(
            "aggregated_capabilities",
            all_cap_names.len(),
            MIN_AGGREGATION_PRIMALS,
        );
    } else {
        v.check_skip(
            "aggregated_capabilities",
            &format!(
                "need >= {MIN_AGGREGATION_PRIMALS} live capabilities for aggregation, found {}",
                live_caps.len(),
            ),
        );
    }

    v.check_skip(
        "capability_aggregation_routing",
        "needs live Plasmodium for best-gate routing",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp034 — Capability Aggregation")
        .with_provenance("exp034_capability_aggregation", "2026-05-09")
        .run(
            "primalSpring Exp034: Plasmodium Routes to Best Gate for Workload",
            |v| {
                v.section("Phase 1: Structural");
                phase_structural(v);

                v.section("Phase 2: Capability Aggregation");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_capability_aggregation(v, &mut ctx);
            },
        );
}
