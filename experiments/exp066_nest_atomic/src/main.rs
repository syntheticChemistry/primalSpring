// SPDX-License-Identifier: AGPL-3.0-or-later
//! exp066 — Nest Atomic validation: Tower + NestGate storage
//!
//! Validates that the Nest Atomic composition (beardog + songbird + nestgate)
//! starts, stores/retrieves data, and passes health checks.

use primalspring::coordination::AtomicType;
use primalspring::harness::AtomicHarness;
use primalspring::ipc::client::PrimalClient;
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

fn rpc(
    socket: &std::path::Path,
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    let mut client =
        PrimalClient::connect(socket, primal_names::NESTGATE).map_err(|e| format!("{e}"))?;
    let resp = client
        .call(method, params.clone())
        .map_err(|e| format!("{e}"))?;
    if let Some(err) = resp.error {
        return Err(format!("RPC error {}: {}", err.code, err.message));
    }
    Ok(resp.result.unwrap_or(serde_json::Value::Null))
}

fn main() {
    ValidationResult::new("exp066_nest_atomic")
        .with_provenance("exp066_nest_atomic", "2026-03-23")
        .run("Nest Atomic — Tower + NestGate storage", |v| {
            let primals = AtomicType::Nest.required_primals();
            v.check_bool(
                "nest_composition_valid",
                primals.len() == 3,
                "Nest = beardog + songbird + nestgate",
            );

            let family_id = format!("exp066-{}", std::process::id());
            let running = match AtomicHarness::new(AtomicType::Nest).start(&family_id) {
                Ok(r) => {
                    v.check_bool("nest_startup", true, "Nest Atomic started");
                    r
                }
                Err(e) => {
                    v.check_bool("nest_startup", false, &format!("startup failed: {e}"));
                    return;
                }
            };

            v.check_minimum("nest_primal_count", running.primal_count(), 3);
            running.validate(v);

            if let Some(ng) = running
                .socket_for("storage")
                .or_else(|| running.socket_for_primal(primal_names::NESTGATE))
            {
                let store = rpc(
                    ng,
                    "storage.store",
                    &serde_json::json!({
                        "family_id": family_id,
                        "key": "exp066_test",
                        "data": {"experiment": "nest_atomic", "timestamp": "2026-03-22"}
                    }),
                );
                v.check_bool("nestgate_store", store.is_ok(), "storage.store");

                let retrieve = rpc(
                    ng,
                    "storage.retrieve",
                    &serde_json::json!({
                        "family_id": family_id, "key": "exp066_test"
                    }),
                );
                v.check_bool("nestgate_retrieve", retrieve.is_ok(), "storage.retrieve");

                if let Ok(val) = &retrieve {
                    v.check_bool(
                        "nestgate_data_integrity",
                        val.get("data")
                            .and_then(|d| d.get("experiment"))
                            .and_then(|e| e.as_str())
                            == Some("nest_atomic"),
                        "data round-trip integrity",
                    );
                }

                v.check_bool(
                    "nestgate_health",
                    rpc(ng, "health", &serde_json::json!({})).is_ok(),
                    "nestgate health",
                );
                v.check_bool(
                    "nestgate_caps",
                    rpc(ng, "discover_capabilities", &serde_json::json!({})).is_ok(),
                    "nestgate capabilities",
                );
            } else {
                v.check_skip("nestgate_store", "nestgate socket not found");
            }
        });
}
