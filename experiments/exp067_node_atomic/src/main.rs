// SPDX-License-Identifier: AGPL-3.0-or-later
//! exp067 — Node Atomic validation: Tower + ToadStool compute
//!
//! Validates that the Node Atomic composition (beardog + songbird + toadstool)
//! starts, reports capabilities, and responds to compute health.

use primalspring::coordination::AtomicType;
use primalspring::harness::AtomicHarness;
use primalspring::ipc::client::PrimalClient;
use primalspring::validation::ValidationResult;

fn rpc(socket: &std::path::Path, method: &str, params: &serde_json::Value) -> Result<serde_json::Value, String> {
    let mut client = PrimalClient::connect(socket, "toadstool").map_err(|e| format!("{e}"))?;
    let resp = client.call(method, params.clone()).map_err(|e| format!("{e}"))?;
    if let Some(err) = resp.error {
        return Err(format!("RPC error {}: {}", err.code, err.message));
    }
    Ok(resp.result.unwrap_or(serde_json::Value::Null))
}

fn main() {
    let mut v = ValidationResult::new("exp067_node_atomic");

    let primals = AtomicType::Node.required_primals();
    v.check_bool("node_composition_valid", primals.len() == 3, "Node = beardog + songbird + toadstool");

    let family_id = format!("exp067-{}", std::process::id());
    let running = match AtomicHarness::new(AtomicType::Node).start(&family_id) {
        Ok(r) => { v.check_bool("node_startup", true, "Node Atomic started"); r }
        Err(e) => { v.check_bool("node_startup", false, &format!("{e}")); v.summary(); return; }
    };

    v.check_minimum("node_primal_count", running.primal_count(), 3);
    running.validate(&mut v);

    if let Some(ts) = running.socket_for("compute").or_else(|| running.socket_for_primal("toadstool")) {
        let health = rpc(ts, "toadstool.health", &serde_json::json!({}));
        v.check_bool("toadstool_health", health.as_ref().map_or(false, |v| v["healthy"] == true), "toadstool.health");

        let caps = rpc(ts, "toadstool.query_capabilities", &serde_json::json!({}));
        v.check_bool("toadstool_caps", caps.as_ref().map_or(false, |v| v["supported_workload_types"].is_array()), "toadstool capabilities");

        if let Ok(c) = &caps {
            let types = c["supported_workload_types"].as_array().map(|a| a.len()).unwrap_or(0);
            v.check_minimum("toadstool_workload_types", types, 1);
            let cores = c["available_resources"]["total_cpu_cores"].as_u64().unwrap_or(0) as usize;
            v.check_minimum("toadstool_cpu_cores", cores, 1);
            println!("  toadstool: {types} workload types, {cores} CPU cores");
        }

        v.check_bool("toadstool_version", rpc(ts, "toadstool.version", &serde_json::json!({})).is_ok(), "toadstool.version");
    } else {
        v.check_skip("toadstool_health", "toadstool socket not found");
    }

    v.summary();
}
