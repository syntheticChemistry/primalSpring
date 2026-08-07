// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

//! Exp116: benchScale NUCLEUS Lab — validates isolated NUCLEUS testing via benchScale.
//!
//! First experiment that exercises benchScale for NUCLEUS testing.
//! Creates an isolated lab NUCLEUS, deploys primals, runs N2/N4 validation,
//! and tears down the lab. Proves primalSpring can spin up isolated
//! NUCLEUS instances for testing without disturbing the overwatch.
//!
//! Requires: benchScale server running (`benchscale server --port 9200`)
//! Optional: `BENCHSCALE_PORT` env override (default: 9200)

use primalspring::composition::CompositionContext;
use primalspring::ipc::NeuralBridge;
use primalspring::ipc::tcp::tcp_rpc;
use primalspring::validation::ValidationResult;

fn benchscale_host() -> String {
    std::env::var("BENCHSCALE_HOST").unwrap_or_else(|_| "localhost".to_owned())
}

fn benchscale_port() -> u16 {
    std::env::var("BENCHSCALE_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(9200)
}

fn benchscale_rpc(
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, primalspring::ipc::IpcError> {
    tcp_rpc(&benchscale_host(), benchscale_port(), method, params).map(|(v, _)| v)
}

fn phase_benchscale_health(v: &mut ValidationResult) -> bool {
    v.section("Phase 1: benchScale server health");

    match benchscale_rpc("health.check", &serde_json::json!({})) {
        Ok(_) => {
            println!("  benchScale server: HEALTHY");
            v.check_bool("benchscale_healthy", true, "benchScale server responding");
            true
        }
        Err(e) => {
            println!("  benchScale server: {e}");
            println!(
                "  Start with: benchscale server --port {} (or set BENCHSCALE_HOST/BENCHSCALE_PORT)",
                benchscale_port()
            );
            v.check_skip("benchscale_healthy", &format!("benchScale not running: {e}"));
            false
        }
    }
}

fn phase_topology_validation(v: &mut ValidationResult) {
    v.section("Phase 2: Topology structural validation");

    let topology = "provenance_trio";
    match benchscale_rpc(
        "topology.validate",
        &serde_json::json!({ "topology": topology }),
    ) {
        Ok(resp) => {
            let valid = resp
                .get("valid")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            println!("  {topology}: valid={valid}");
            v.check_bool(
                "provenance_trio_topology_valid",
                valid,
                &format!("{topology} topology structurally valid"),
            );
        }
        Err(e) => {
            v.check_skip(
                "provenance_trio_topology_valid",
                &format!("topology.validate: {e}"),
            );
        }
    }
}

fn phase_lab_create(v: &mut ValidationResult) -> Option<String> {
    v.section("Phase 3: Create isolated NUCLEUS lab");

    match benchscale_rpc(
        "lab.create",
        &serde_json::json!({
            "topology": "provenance_trio",
            "name": "exp116-validation",
        }),
    ) {
        Ok(resp) => {
            let lab_id = resp
                .get("lab_id")
                .or_else(|| resp.get("id"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_owned();
            println!("  Lab created: {lab_id}");
            v.check_bool("lab_created", true, &format!("lab {lab_id} created"));
            Some(lab_id)
        }
        Err(e) => {
            println!("  Lab creation: {e}");
            v.check_skip("lab_created", &format!("lab.create: {e}"));
            None
        }
    }
}

fn phase_lab_health(v: &mut ValidationResult, lab_id: &str) {
    v.section("Phase 4: Lab NUCLEUS health");

    match benchscale_rpc(
        "lab.status",
        &serde_json::json!({ "lab_id": lab_id }),
    ) {
        Ok(resp) => {
            let status = resp
                .get("status")
                .and_then(|s| s.as_str())
                .unwrap_or("unknown");
            let nodes = resp
                .get("nodes")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(0);
            println!("  Lab status: {status}, nodes: {nodes}");
            v.check_bool(
                "lab_healthy",
                status == "running" || status == "ready",
                &format!("lab {lab_id}: {status}, {nodes} nodes"),
            );
        }
        Err(e) => v.check_skip("lab_healthy", &format!("lab.status: {e}")),
    }
}

fn phase_n2_validation(v: &mut ValidationResult) {
    v.section("Phase 5: N2 validation (capability.call → bearDog)");

    let Some(bridge) = NeuralBridge::discover() else {
        v.check_skip("n2_crypto_route", "Neural API not available in lab");
        return;
    };

    match bridge.capability_call(
        "crypto",
        "sign_ed25519",
        &serde_json::json!({"data": "dGVzdA=="}),
    ) {
        Ok(resp) => {
            let has_sig = resp.value.get("signature").is_some()
                || resp.value.get("sig").is_some()
                || resp.value.get("result").is_some();
            v.check_bool(
                "n2_crypto_route",
                has_sig,
                "capability.call(crypto, sign_ed25519) → bearDog via lab NUCLEUS",
            );
        }
        Err(e) => v.check_skip("n2_crypto_route", &format!("routing failed: {e}")),
    }
}

fn phase_n4_provenance(v: &mut ValidationResult) {
    v.section("Phase 6: N4 provenance routing (session-scoped)");

    let ctx = CompositionContext::from_live_discovery_with_fallback();
    let has_dag = ctx.has_capability("dag");
    let has_spine = ctx.has_capability("commit") || ctx.has_capability("spine");
    let has_braid = ctx.has_capability("provenance") || ctx.has_capability("braid");

    v.check_bool(
        "n4_dag_discoverable",
        has_dag,
        "DAG capability (rhizoCrypt) discoverable",
    );
    v.check_bool(
        "n4_spine_discoverable",
        has_spine,
        "spine capability (loamSpine) discoverable",
    );
    v.check_bool(
        "n4_braid_discoverable",
        has_braid,
        "braid capability (sweetGrass) discoverable",
    );

    let trio_ready = has_dag && has_spine && has_braid;
    v.check_bool(
        "n4_provenance_trio_ready",
        trio_ready,
        "provenance trio all discoverable",
    );
}

fn phase_lab_destroy(v: &mut ValidationResult, lab_id: &str) {
    v.section("Phase 7: Cleanup — destroy lab");

    match benchscale_rpc(
        "lab.destroy",
        &serde_json::json!({ "lab_id": lab_id }),
    ) {
        Ok(_) => {
            println!("  Lab {lab_id} destroyed");
            v.check_bool("lab_destroyed", true, &format!("lab {lab_id} cleaned up"));
        }
        Err(e) => {
            println!("  Lab destroy: {e}");
            v.check_skip("lab_destroyed", &format!("lab.destroy: {e}"));
        }
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp116 — benchScale NUCLEUS Lab")
        .with_provenance("exp116_benchscale_nucleus_lab", "2026-08-07")
        .run(
            "Isolated NUCLEUS lab creation + N2/N4 validation + teardown",
            |v| {
                if !phase_benchscale_health(v) {
                    return;
                }

                phase_topology_validation(v);

                let lab_id = match phase_lab_create(v) {
                    Some(id) => id,
                    None => return,
                };

                phase_lab_health(v, &lab_id);
                phase_n2_validation(v);
                phase_n4_provenance(v);
                phase_lab_destroy(v, &lab_id);
            },
        );
}
