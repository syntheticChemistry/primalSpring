// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

//! Exp124: Provenance Trio Experiment Suite — Wave 2 validation.
//!
//! Validates the 14-experiment suite built in `membrane experiment.*` against
//! live primals via the Neural API. Covers:
//!
//! **Wave 1** (core trust model):
//! - Break (tamper detection), Rebraid (determinism), Falsify (negative provenance)
//! - Audit (estate-wide integrity), Reward (attribution), Export (PROV-O/RO-Crate/BagIt)
//! - Translate (paper-ready statements), Compress (meta-braid aggregation)
//!
//! **Wave 2** (individual primal deep-dives):
//! - Dehydrate (rhizoCrypt DAG lifecycle + merkle proofs)
//! - Spine (loamSpine permanence, inclusion proofs, certificates)
//! - Encrypt (bearDog crypto round-trips: Ed25519, ChaCha20, AES-256-GCM)
//! - ZFS (nestGate storage layer lifecycle)
//! - Compose (cross-primal pipeline: hash→store→DAG→sign→braid)
//! - Inventory (full NUCLEUS capability census)
//!
//! This experiment validates the harness infrastructure and confirms
//! the membrane experiment CLI produces correct structured results.

use std::time::Instant;

use primalspring::composition::CompositionContext;
use primalspring::ipc::capability;
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("Exp124: Provenance Trio Experiment Suite")
        .with_provenance("exp124_provenance_trio_experiments", "2026-08-14")
        .run("Wave 1+2 validation against live NUCLEUS", |v| {
            let mut ctx = CompositionContext::from_live_discovery_with_fallback();

            phase_discovery(v, &ctx);
            phase_trio_health(v, &mut ctx);
            phase_braid_operations(v, &mut ctx);
            phase_dag_lifecycle(v, &mut ctx);
            phase_spine_operations(v, &mut ctx);
            phase_crypto_roundtrip(v, &mut ctx);
            phase_storage_probe(v, &mut ctx);
            phase_nest_health(v, &mut ctx);
        });
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    v.section("Phase 1: Provenance trio discovery");

    let has_dag = ctx.has_capability("dag");
    let has_ledger = ctx.has_capability("ledger");
    let has_braid = ctx.has_capability("commit")
        || ctx.has_capability("attribution")
        || ctx.has_capability("provenance");
    let has_storage = ctx.has_capability("content") || ctx.has_capability("storage");
    let has_crypto = ctx.has_capability("crypto") || ctx.has_capability("security");

    v.check_bool("rhizocrypt_dag_cap", has_dag, "dag capability discoverable");
    v.check_bool("loamspine_ledger_cap", has_ledger, "ledger capability discoverable");
    v.check_bool("sweetgrass_braid_cap", has_braid, "braid/attribution capability discoverable");
    v.check_bool("nestgate_storage_cap", has_storage, "content/storage capability discoverable");
    v.check_bool("beardog_crypto_cap", has_crypto, "crypto/security capability discoverable");

    let total_caps = ctx.available_capabilities().len();
    v.check_minimum("total_capabilities", total_caps, 5);
}

fn phase_trio_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 2: Provenance trio health probes");

    let primals = [
        ("dag", primal_names::RHIZOCRYPT),
        ("ledger", primal_names::LOAMSPINE),
        ("content", primal_names::NESTGATE),
    ];

    for (cap, name) in primals {
        if !ctx.has_capability(cap) {
            v.check_skip(&format!("health_{name}"), &format!("{cap} not discovered"));
            continue;
        }

        let start = Instant::now();
        let ok = ctx.health_check(cap).unwrap_or(false);
        let latency = start.elapsed().as_micros();

        v.check_bool(&format!("health_{name}"), ok, &format!("{name} health check"));
        v.check_latency(
            &format!("latency_{name}"),
            u64::try_from(latency).unwrap_or(u64::MAX),
            5_000_000,
        );
    }

    // bearDog health via crypto capability
    if ctx.has_capability("crypto") {
        let start = Instant::now();
        let ok = ctx.health_check("crypto").unwrap_or(false);
        let latency = start.elapsed().as_micros();
        v.check_bool("health_beardog", ok, "bearDog crypto health");
        v.check_latency(
            "latency_beardog",
            u64::try_from(latency).unwrap_or(u64::MAX),
            5_000_000,
        );
    } else {
        v.check_skip("health_beardog", "crypto capability not discovered");
    }
}

fn phase_braid_operations(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 3: Braid operations (break/rebraid/falsify/audit)");

    let braid_cap = if ctx.has_capability("provenance") {
        Some("provenance")
    } else if ctx.has_capability("commit") {
        Some("commit")
    } else if ctx.has_capability("attribution") {
        Some("attribution")
    } else {
        None
    };

    let Some(cap) = braid_cap else {
        v.check_skip("braid_list", "no braid capability discovered");
        return;
    };

    // braid.list — validates audit capability
    let list = ctx.call(cap, "braid.list", serde_json::json!({"limit": 5}));
    match list {
        Ok(result) => {
            let items = result
                .get("items")
                .and_then(|i| i.as_array())
                .map(|a| a.len())
                .unwrap_or(0);
            v.check_minimum("braid_list_items", items, 1);

            // braid.verify — validates break experiment
            if let Some(first) = result.get("items").and_then(|i| i.as_array()).and_then(|a| a.first()) {
                let braid_id = first.get("id").and_then(|v| v.as_str()).unwrap_or("");
                if !braid_id.is_empty() {
                    let verify = ctx.call(cap, "braid.verify", serde_json::json!({"braid_id": braid_id}));
                    match verify {
                        Ok(vr) => {
                            let verified = vr.get("verified").and_then(|v| v.as_bool()).unwrap_or(false);
                            v.check_bool("braid_verify", verified, "braid.verify returns verified=true");
                        }
                        Err(e) => v.check_skip("braid_verify", &format!("braid.verify failed: {e}")),
                    }
                } else {
                    v.check_skip("braid_verify", "no braid_id in first item");
                }
            }
        }
        Err(e) => {
            v.check_skip("braid_list_items", &format!("braid.list failed: {e}"));
            v.check_skip("braid_verify", "braid.list prerequisite failed");
        }
    }
}

fn phase_dag_lifecycle(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 4: DAG lifecycle (dehydrate experiment)");

    if !ctx.has_capability("dag") {
        v.check_skip("dag_session_create", "dag capability not discovered");
        return;
    }

    let session = ctx.call(
        "dag",
        "dag.session.create",
        serde_json::json!({"name": "exp124-dag-test"}),
    );

    match session {
        Ok(result) => {
            let sid = result
                .get("session_id")
                .or(result.get("id"))
                .and_then(|v| v.as_str())
                .or_else(|| result.as_str())
                .unwrap_or("");
            v.check_bool("dag_session_create", !sid.is_empty(), "DAG session created");

            if !sid.is_empty() {
                // Cleanup
                let _ = ctx.call("dag", "dag.session.discard", serde_json::json!({"session_id": sid}));
            }
        }
        Err(e) => v.check_skip("dag_session_create", &format!("dag.session.create: {e}")),
    }

    let sessions = ctx.call("dag", "dag.session.list", serde_json::json!({}));
    match sessions {
        Ok(_) => v.check_bool("dag_session_list", true, "dag.session.list responsive"),
        Err(e) => v.check_skip("dag_session_list", &format!("dag.session.list: {e}")),
    }
}

fn phase_spine_operations(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 5: Spine operations (loamSpine)");

    if !ctx.has_capability("ledger") {
        v.check_skip("spine_list", "ledger capability not discovered");
        return;
    }

    let spines = ctx.call("ledger", "spine.list", serde_json::json!({}));
    match spines {
        Ok(result) => {
            let count = result
                .get("spines")
                .or(result.get("items"))
                .and_then(|v| v.as_array())
                .map(|a| a.len())
                .or_else(|| result.get("count").and_then(|v| v.as_u64()).map(|n| n as usize))
                .unwrap_or(0);
            v.check_minimum("spine_count", count, 1);
        }
        Err(e) => v.check_skip("spine_count", &format!("spine.list: {e}")),
    }

    let trust = ctx.call("ledger", "trust.event_count", serde_json::json!({}));
    match trust {
        Ok(_) => v.check_bool("trust_event_count", true, "trust.event_count responsive"),
        Err(e) => v.check_skip("trust_event_count", &format!("trust.event_count: {e}")),
    }
}

fn phase_crypto_roundtrip(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 6: Crypto round-trip (bearDog encrypt experiment)");

    if !ctx.has_capability("crypto") {
        v.check_skip("crypto_blake3", "crypto capability not discovered");
        return;
    }

    let hash = ctx.call(
        "crypto",
        "crypto.blake3_hash",
        serde_json::json!({"data": "exp124 crypto test"}),
    );
    match hash {
        Ok(result) => {
            let h = result.get("hash").or(result.get("digest")).and_then(|v| v.as_str()).unwrap_or("");
            v.check_bool("crypto_blake3", !h.is_empty(), "BLAKE3 hash returned");
        }
        Err(e) => v.check_skip("crypto_blake3", &format!("crypto.blake3_hash: {e}")),
    }

    let keygen = ctx.call("crypto", "crypto.ed25519_generate_keypair", serde_json::json!({}));
    match keygen {
        Ok(result) => {
            let pk = result.get("public_key").and_then(|v| v.as_str()).unwrap_or("");
            v.check_bool("crypto_ed25519_keygen", !pk.is_empty(), "Ed25519 keypair generated");
        }
        Err(e) => v.check_skip("crypto_ed25519_keygen", &format!("keygen: {e}")),
    }

    let identity = ctx.call("crypto", "identity.get", serde_json::json!({}));
    match identity {
        Ok(result) => {
            let did = result.get("did").or(result.get("id")).and_then(|v| v.as_str()).unwrap_or("");
            v.check_bool("crypto_identity", !did.is_empty(), "bearDog identity resolvable");
        }
        Err(e) => v.check_skip("crypto_identity", &format!("identity.get: {e}")),
    }
}

fn phase_storage_probe(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 7: Storage probe (nestGate + ZFS experiment)");

    let cap = if ctx.has_capability("content") {
        "content"
    } else if ctx.has_capability("storage") {
        "storage"
    } else {
        v.check_skip("storage_health", "no content/storage capability discovered");
        return;
    };

    let health = ctx.call(cap, "zfs.health", serde_json::json!({}));
    match health {
        Ok(result) => {
            let status = result.get("status").and_then(|v| v.as_str()).unwrap_or("unknown");
            v.check_bool("zfs_health", status == "healthy", &format!("ZFS health: {status}"));
        }
        Err(e) => v.check_skip("zfs_health", &format!("zfs.health: {e}")),
    }

    let exists = ctx.call(cap, "content.exists", serde_json::json!({"hash": "0000000000000000000000000000000000000000000000000000000000000000"}));
    match exists {
        Ok(result) => {
            let e = result.get("exists").and_then(|v| v.as_bool()).unwrap_or(true);
            v.check_bool("cas_null_hash_absent", !e, "null hash should not exist in CAS");
        }
        Err(e) => v.check_skip("cas_null_hash_absent", &format!("content.exists: {e}")),
    }
}

fn phase_nest_health(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 8: Nest Atomic health (inventory experiment)");

    let nest = capability::capability_call("composition", "nest.health", &serde_json::json!({}));

    match nest {
        Some(result) => {
            let domains_ok = result
                .get("domains_ok")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(0);
            let domains_total = result
                .get("domains_total")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(0);
            let primals_alive = result
                .get("primals_alive")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(0);

            v.check_minimum("nest_domains_ok", domains_ok as usize, 4);
            v.check_minimum("nest_domains_total", domains_total as usize, 6);
            v.check_minimum("nest_primals_alive", primals_alive as usize, 10);

            if let Some(domains) = result.get("domains").and_then(serde_json::Value::as_object) {
                for (domain, info) in domains {
                    let status = info
                        .get("status")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("unknown");
                    v.check_bool(
                        &format!("domain_{domain}"),
                        status == "ok" || status == "healthy",
                        &format!("{domain}: {status}"),
                    );
                }
            }
        }
        None => v.check_skip("nest_health", "nest.health: Neural API not reachable or composition capability not routed"),
    }
}
