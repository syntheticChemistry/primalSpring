// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp067: Node Atomic — ToadStool compute via CompositionContext.

use primalspring::cast::u64_to_usize;
use primalspring::composition::CompositionContext;
use primalspring::coordination::AtomicType;
use primalspring::validation::ValidationResult;

fn phase_structural(v: &mut ValidationResult) {
    let primals = AtomicType::Node.required_primals();
    v.check_bool(
        "node_composition_valid",
        primals.len() == 5,
        "Node compute triangle (beardog + songbird + toadstool + barracuda + coralreef)",
    );
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    for cap in AtomicType::Node.required_capabilities() {
        v.check_bool(
            &format!("has_{cap}"),
            ctx.has_capability(cap),
            &format!("{cap} capability resolved"),
        );
    }
}

fn phase_toadstool(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("compute") {
        v.check_skip("toadstool_health", "compute capability not discovered");
        v.check_skip("toadstool_caps", "compute capability not discovered");
        v.check_skip(
            "toadstool_workload_types",
            "compute capability not discovered",
        );
        v.check_skip("toadstool_cpu_cores", "compute capability not discovered");
        v.check_skip("toadstool_version", "compute capability not discovered");
        return;
    }

    let health = ctx.call("compute", "toadstool.health", serde_json::json!({}));
    v.check_bool(
        "toadstool_health",
        health.as_ref().is_ok_and(|v| v["healthy"] == true),
        "toadstool.health",
    );

    let caps = ctx.call(
        "compute",
        "toadstool.query_capabilities",
        serde_json::json!({}),
    );
    v.check_bool(
        "toadstool_caps",
        caps.as_ref()
            .is_ok_and(|v| v["supported_workload_types"].is_array()),
        "toadstool capabilities",
    );

    if let Ok(c) = &caps {
        let types = c["supported_workload_types"].as_array().map_or(0, Vec::len);
        v.check_minimum("toadstool_workload_types", types, 1);
        let cores = u64_to_usize(
            c["available_resources"]["total_cpu_cores"]
                .as_u64()
                .unwrap_or(0),
        );
        v.check_minimum("toadstool_cpu_cores", cores, 1);
        println!("  toadstool: {types} workload types, {cores} CPU cores");
    }

    match ctx.call("compute", "toadstool.version", serde_json::json!({})) {
        Ok(_) => v.check_bool("toadstool_version", true, "toadstool.version"),
        Err(e) if e.is_connection_error() => {
            v.check_skip("toadstool_version", &format!("{e}"));
        }
        Err(e) => v.check_bool("toadstool_version", false, &format!("error: {e}")),
    }
}

fn main() {
    ValidationResult::new("exp067_node_atomic")
        .with_provenance("exp067_node_atomic", "2026-05-09")
        .run("Node Atomic — Tower + ToadStool compute", |v| {
            v.section("Phase 1: Structural");
            phase_structural(v);

            v.section("Phase 2: Discovery");
            let mut ctx = CompositionContext::from_live_discovery_with_fallback();
            phase_discovery(v, &ctx);

            v.section("Phase 3: ToadStool");
            phase_toadstool(v, &mut ctx);
        });
}
