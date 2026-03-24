// SPDX-License-Identifier: AGPL-3.0-or-later
//! exp068 — Full NUCLEUS composition: all available primals
//!
//! Validates that every available primal in plasmidBin can start together,
//! respond to health, and report capabilities. This is the full ecosystem
//! composition test.

use primalspring::coordination::AtomicType;
use primalspring::harness::AtomicHarness;
use primalspring::ipc::client::PrimalClient;
use primalspring::validation::ValidationResult;

fn rpc(
    socket: &std::path::Path,
    primal: &str,
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    let mut client = PrimalClient::connect(socket, primal).map_err(|e| format!("{e}"))?;
    let resp = client
        .call(method, params.clone())
        .map_err(|e| format!("{e}"))?;
    if let Some(err) = resp.error {
        return Err(format!("RPC error {}: {}", err.code, err.message));
    }
    Ok(resp.result.unwrap_or(serde_json::Value::Null))
}

fn validate_tower(v: &mut ValidationResult) {
    println!("\n=== Tower Atomic ===");
    let tower_fam = format!("exp068t-{}", std::process::id());
    match AtomicHarness::new(AtomicType::Tower).start(&tower_fam) {
        Ok(r) => {
            v.check_bool("tower_start", true, "Tower started");
            r.validate(v);
        }
        Err(e) => v.check_bool("tower_start", false, &format!("{e}")),
    }
}

fn validate_nest(v: &mut ValidationResult) {
    println!("\n=== Nest Atomic ===");
    let nest_fam = format!("exp068n-{}", std::process::id());
    match AtomicHarness::new(AtomicType::Nest).start(&nest_fam) {
        Ok(r) => {
            v.check_bool("nest_start", true, "Nest started");
            for (name, live) in &r.health_check_all() {
                v.check_bool(&format!("nest_{name}_live"), *live, &format!("{name} live"));
            }
            if let Some(ng) = r
                .socket_for("storage")
                .or_else(|| r.socket_for_primal("nestgate"))
            {
                let store = rpc(
                    ng,
                    "nestgate",
                    "storage.store",
                    &serde_json::json!({
                        "family_id": nest_fam, "key": "nucleus_test", "data": {"source": "exp068"}
                    }),
                );
                v.check_bool("nest_store", store.is_ok(), "nestgate store");
            }
        }
        Err(e) => v.check_bool("nest_start", false, &format!("{e}")),
    }
}

fn validate_node(v: &mut ValidationResult) {
    println!("\n=== Node Atomic ===");
    let node_fam = format!("exp068d-{}", std::process::id());
    match AtomicHarness::new(AtomicType::Node).start(&node_fam) {
        Ok(r) => {
            v.check_bool("node_start", true, "Node started");
            for (name, live) in &r.health_check_all() {
                v.check_bool(&format!("node_{name}_live"), *live, &format!("{name} live"));
            }
            if let Some(ts) = r
                .socket_for("compute")
                .or_else(|| r.socket_for_primal("toadstool"))
            {
                let caps = rpc(
                    ts,
                    "toadstool",
                    "toadstool.query_capabilities",
                    &serde_json::json!({}),
                );
                v.check_bool("node_compute_caps", caps.is_ok(), "toadstool caps");
            }
        }
        Err(e) => v.check_bool("node_start", false, &format!("{e}")),
    }
}

fn main() {
    ValidationResult::new("exp068_full_nucleus")
        .with_provenance("exp068_full_nucleus", "2026-03-23")
        .run("Full NUCLEUS — all primals composed", |v| {
            let primals = AtomicType::FullNucleus.required_primals();
            v.check_minimum("nucleus_required_primals", primals.len(), 5);

            validate_tower(v);
            validate_nest(v);
            validate_node(v);
        });
}
