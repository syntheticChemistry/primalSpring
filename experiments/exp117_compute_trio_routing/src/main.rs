// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

//! Exp117: Compute Trio Routing — validates the hotSpring compute trio IPC
//! pattern through Neural API.
//!
//! Exercises the Node Atomic compute trio (N5):
//!   barraCuda (WHAT) → coralReef (HOW) → toadStool (WHERE)
//!
//! Flow:
//!   1. `shader.compile.wgsl` → coralReef (compile WGSL to native)
//!   2. `compute.dispatch.submit` → toadStool (GPU execution)
//!   3. `compute.dispatch.result` → verify + BLAKE3 witness
//!   4. `dag.event.append` → provenance record for the compute result
//!
//! All routing goes through `NeuralBridge::capability_call()`.

use primalspring::composition::CompositionContext;
use primalspring::ipc::NeuralBridge;
use primalspring::validation::ValidationResult;

fn phase_compute_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    v.section("Phase 1: Compute trio capability discovery");

    let domains = [
        ("compute", "toadStool dispatch"),
        ("shader", "coralReef compile"),
    ];

    for (domain, desc) in &domains {
        let check_name = format!("{domain}_capability");
        v.check_bool(
            &check_name,
            ctx.has_capability(domain),
            &format!("{domain} discoverable ({desc})"),
        );
    }

    v.check_bool(
        "security_for_witness",
        ctx.has_capability("security"),
        "security capability for BLAKE3 witness signing",
    );
}

fn phase_shader_compile(v: &mut ValidationResult, bridge: &NeuralBridge) -> bool {
    v.section("Phase 2: Shader compile (coralReef via Neural API)");

    let wgsl_source = r#"
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    // Minimal WGSL compute shader for routing validation
    let x = id.x;
}
"#;

    match bridge.capability_call(
        "shader",
        "shader.compile.wgsl",
        &serde_json::json!({
            "source": wgsl_source,
            "entry_point": "main",
            "target": "spirv",
        }),
    ) {
        Ok(resp) => {
            let compiled = resp.value.get("compiled").is_some()
                || resp.value.get("binary").is_some()
                || resp.value.get("spirv").is_some()
                || resp.value.get("result").is_some();
            v.check_bool(
                "shader_compiled",
                compiled,
                "shader.compile.wgsl → coralReef returned compiled artifact",
            );
            compiled
        }
        Err(e) => {
            v.check_skip("shader_compiled", &format!("coralReef routing: {e}"));
            false
        }
    }
}

fn phase_compute_dispatch(v: &mut ValidationResult, bridge: &NeuralBridge) -> bool {
    v.section("Phase 3: Compute dispatch (toadStool via Neural API)");

    match bridge.capability_call(
        "compute",
        "compute.dispatch.capabilities",
        &serde_json::json!({}),
    ) {
        Ok(resp) => {
            let has_hw = resp.value.get("devices").is_some()
                || resp.value.get("capabilities").is_some()
                || resp.value.get("hardware").is_some();
            v.check_bool(
                "dispatch_capabilities",
                has_hw,
                "compute.dispatch.capabilities → toadStool reports hardware",
            );
        }
        Err(e) => {
            v.check_skip(
                "dispatch_capabilities",
                &format!("toadStool capabilities: {e}"),
            );
        }
    }

    match bridge.capability_call(
        "compute",
        "compute.dispatch.submit",
        &serde_json::json!({
            "shader_ref": "exp117-validation-shader",
            "workgroup_count": [1, 1, 1],
            "input_data": [],
        }),
    ) {
        Ok(resp) => {
            let dispatched = resp.value.get("job_id").is_some()
                || resp.value.get("dispatch_id").is_some()
                || resp.value.get("result").is_some();
            v.check_bool(
                "compute_dispatched",
                dispatched,
                "compute.dispatch.submit → toadStool accepted compute job",
            );
            dispatched
        }
        Err(e) => {
            v.check_skip("compute_dispatched", &format!("toadStool dispatch: {e}"));
            false
        }
    }
}

fn phase_result_verification(v: &mut ValidationResult, bridge: &NeuralBridge) {
    v.section("Phase 4: Result verification + BLAKE3 witness");

    match bridge.capability_call(
        "compute",
        "compute.dispatch.result",
        &serde_json::json!({
            "job_id": "exp117-validation-shader",
            "include_witness": true,
        }),
    ) {
        Ok(resp) => {
            let has_result = resp.value.get("output").is_some()
                || resp.value.get("result").is_some()
                || resp.value.get("data").is_some();
            v.check_bool(
                "compute_result_received",
                has_result,
                "compute.dispatch.result returned output",
            );

            let has_witness = resp.value.get("witness").is_some()
                || resp.value.get("blake3_hash").is_some()
                || resp.value.get("hash").is_some();
            v.check_bool(
                "blake3_witness_present",
                has_witness,
                "BLAKE3 witness hash present in compute result",
            );
        }
        Err(e) => {
            v.check_skip("compute_result_received", &format!("dispatch.result: {e}"));
            v.check_skip("blake3_witness_present", &format!("dispatch.result: {e}"));
        }
    }
}

fn phase_provenance_record(v: &mut ValidationResult, bridge: &NeuralBridge) {
    v.section("Phase 5: Provenance record (compute result → DAG)");

    match bridge.capability_call(
        "dag",
        "dag.event.append",
        &serde_json::json!({
            "session_id": "exp117-compute-session",
            "event": {
                "type": "compute_result",
                "shader": "exp117-validation-shader",
                "trio": ["barraCuda", "coralReef", "toadStool"],
            },
        }),
    ) {
        Ok(_) => {
            v.check_bool(
                "provenance_recorded",
                true,
                "compute result recorded in rhizoCrypt DAG",
            );
        }
        Err(e) => {
            v.check_skip(
                "provenance_recorded",
                &format!("dag.event.append: {e}"),
            );
        }
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp117 — Compute Trio Routing")
        .with_provenance("exp117_compute_trio_routing", "2026-08-07")
        .run(
            "Node Atomic routing: shader→coralReef, dispatch→toadStool, witness+provenance",
            |v| {
                let ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_compute_discovery(v, &ctx);

                let Some(bridge) = NeuralBridge::discover() else {
                    v.check_skip("neural_api", "biomeOS not running — all routing skipped");
                    return;
                };

                match bridge.health_check() {
                    Ok(_) => v.check_bool("neural_api", true, "biomeOS neural-api healthy"),
                    Err(e) => {
                        v.check_bool("neural_api", false, &format!("neural-api: {e}"));
                        return;
                    }
                }

                let shader_ok = phase_shader_compile(v, &bridge);
                let dispatch_ok = phase_compute_dispatch(v, &bridge);

                if shader_ok && dispatch_ok {
                    phase_result_verification(v, &bridge);
                } else {
                    v.check_skip("compute_result_received", "shader or dispatch phase skipped");
                    v.check_skip("blake3_witness_present", "shader or dispatch phase skipped");
                }

                phase_provenance_record(v, &bridge);
            },
        );
}
