// SPDX-License-Identifier: AGPL-3.0-or-later
//! Scenario: NestGate Content Pipeline — contract test for `content.*` methods.
//!
//! Validates that the `content` capability domain is discoverable and that
//! `content.put` → `content.get` produces a correct round-trip (BLAKE3 hash
//! match, byte fidelity). This scenario catches the gap class where methods
//! are registered in the 413-method registry but not reachable on all
//! transports (Wave 7 — semantic gate evolution).

use base64::Engine;
use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "nestgate-content-pipeline",
        track: Track::AtomicComposition,
        tier: Tier::Live,
        provenance_crate: "wave7_contract_testing",
        provenance_date: "2026-05-11",
        description: "NestGate content.* contract — put/get round-trip, exists, list, resolve",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Content capability discovery");
    phase_discovery(v, ctx);

    v.section("Phase 2: content.put (BLAKE3 CAS store)");
    let hash = phase_content_put(v, ctx);

    v.section("Phase 3: content.get (round-trip verification)");
    phase_content_get(v, ctx, hash.as_deref());

    v.section("Phase 4: content.exists + content.list");
    phase_content_exists_list(v, ctx, hash.as_deref());

    v.section("Phase 5: content.resolve (manifest path resolution)");
    phase_content_resolve(v, ctx);
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let has_content = ctx.has_capability("content");
    v.check_bool(
        "content_capability_discovered",
        has_content,
        if has_content {
            "content domain resolved to NestGate"
        } else {
            "content domain not discoverable — NestGate may not be running or content not in discovery"
        },
    );

    let has_storage = ctx.has_capability("storage");
    v.check_bool(
        "storage_capability_present",
        has_storage,
        "storage domain available (NestGate baseline)",
    );
}

fn phase_content_put(v: &mut ValidationResult, ctx: &mut CompositionContext) -> Option<String> {
    let test_bytes = b"primalSpring Wave 7 contract test - content pipeline validation 2026-05-11";
    let data_b64 = base64::engine::general_purpose::STANDARD.encode(test_bytes);
    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "nucleus01".to_owned());

    let result = ctx.call(
        "content",
        "content.put",
        serde_json::json!({
            "data": data_b64,
            "content_type": "text/plain",
            "family_id": family_id,
        }),
    );

    match result {
        Ok(resp) => {
            let hash = resp.get("hash").and_then(|h| h.as_str()).unwrap_or("");
            v.check_bool(
                "content_put_returns_hash",
                !hash.is_empty(),
                &format!("BLAKE3 hash: {}...", &hash[..hash.len().min(16)]),
            );
            v.check_bool(
                "content_put_hash_length",
                hash.len() == 64,
                &format!("expected 64 hex chars, got {}", hash.len()),
            );

            let stored = resp.get("stored").and_then(serde_json::Value::as_bool).unwrap_or(false);
            let dedup = resp.get("deduplicated").and_then(serde_json::Value::as_bool).unwrap_or(false);
            v.check_bool(
                "content_put_stored_or_dedup",
                stored || dedup,
                &format!("stored={stored}, deduplicated={dedup}"),
            );

            if hash.is_empty() {
                None
            } else {
                Some(hash.to_owned())
            }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("content_put_returns_hash", &format!("content not available: {e}"));
            v.check_skip("content_put_hash_length", "content not available");
            v.check_skip("content_put_stored_or_dedup", "content not available");
            None
        }
        Err(e) => {
            let msg = format!("content.put error: {e}");
            v.check_bool("content_put_returns_hash", false, &msg);
            v.check_skip("content_put_hash_length", "prior call failed");
            v.check_skip("content_put_stored_or_dedup", "prior call failed");
            None
        }
    }
}

fn phase_content_get(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    hash: Option<&str>,
) {
    let Some(hash) = hash else {
        v.check_skip("content_get_returns_data", "no hash from content.put");
        v.check_skip("content_get_roundtrip_match", "no hash from content.put");
        return;
    };

    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "nucleus01".to_owned());
    let result = ctx.call(
        "content",
        "content.get",
        serde_json::json!({ "hash": hash, "family_id": family_id }),
    );

    match result {
        Ok(resp) => {
            let data_b64 = resp.get("data").and_then(|d| d.as_str()).unwrap_or("");
            v.check_bool(
                "content_get_returns_data",
                !data_b64.is_empty(),
                &format!("retrieved {} base64 chars", data_b64.len()),
            );

            let expected = b"primalSpring Wave 7 contract test - content pipeline validation 2026-05-11";
            let expected_b64 = base64::engine::general_purpose::STANDARD.encode(expected);
            v.check_bool(
                "content_get_roundtrip_match",
                data_b64 == expected_b64,
                if data_b64 == expected_b64 {
                    "round-trip byte fidelity confirmed"
                } else {
                    "DATA MISMATCH — content.get returned different bytes than content.put stored"
                },
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("content_get_returns_data", &format!("content not available: {e}"));
            v.check_skip("content_get_roundtrip_match", "content not available");
        }
        Err(e) => {
            v.check_bool("content_get_returns_data", false, &format!("content.get error: {e}"));
            v.check_skip("content_get_roundtrip_match", "prior call failed");
        }
    }
}

fn phase_content_exists_list(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    hash: Option<&str>,
) {
    let Some(hash) = hash else {
        v.check_skip("content_exists_confirms_hash", "no hash from content.put");
        v.check_skip("content_list_nonempty", "no hash from content.put");
        return;
    };

    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "nucleus01".to_owned());

    match ctx.call(
        "content",
        "content.exists",
        serde_json::json!({ "hash": hash, "family_id": family_id }),
    ) {
        Ok(resp) => {
            let exists = resp.get("exists").and_then(serde_json::Value::as_bool).unwrap_or(false);
            v.check_bool(
                "content_exists_confirms_hash",
                exists,
                &format!("hash {hash} exists={exists}"),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("content_exists_confirms_hash", &format!("content not available: {e}"));
        }
        Err(e) => {
            v.check_bool("content_exists_confirms_hash", false, &format!("content.exists error: {e}"));
        }
    }

    match ctx.call(
        "content",
        "content.list",
        serde_json::json!({ "family_id": family_id }),
    ) {
        Ok(resp) => {
            let items = resp.get("items").and_then(|i| i.as_array());
            let count = items.map_or(0, Vec::len);
            v.check_bool(
                "content_list_nonempty",
                count > 0,
                &format!("{count} content items listed"),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("content_list_nonempty", &format!("content not available: {e}"));
        }
        Err(e) => {
            v.check_bool("content_list_nonempty", false, &format!("content.list error: {e}"));
        }
    }
}

fn phase_content_resolve(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "nucleus01".to_owned());

    match ctx.call(
        "content",
        "content.resolve",
        serde_json::json!({ "path": "index.html", "family_id": family_id }),
    ) {
        Ok(resp) => {
            let has_content = resp.get("content").is_some() || resp.get("data").is_some();
            v.check_bool(
                "content_resolve_responds",
                true,
                if has_content {
                    "content.resolve returned content for path"
                } else {
                    "content.resolve returned response (no content for path — manifest may not exist)"
                },
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("content_resolve_responds", &format!("content not available: {e}"));
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("not found") || msg.contains("No manifest") {
                v.check_skip(
                    "content_resolve_responds",
                    "content.resolve: no manifest published (expected for fresh NestGate)",
                );
            } else {
                v.check_bool("content_resolve_responds", false, &format!("content.resolve error: {e}"));
            }
        }
    }
}
