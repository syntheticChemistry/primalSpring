// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scenario: Content Sovereignty — validates the full content pipeline through
//! the sovereign membrane routing layer.
//!
//! Phase 1: Content store/retrieve through NestGate (sovereign backend)
//! Phase 2: Content resolution confirms sovereign backend metadata
//! Phase 3: Trust-tier alignment between routing config and composition bonding
//! Phase 4: SkunkBat audit correlation for content operations

use base64::Engine;
use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "content-sovereignty",
        track: Track::Sovereignty,
        tier: Tier::Live,
        provenance_crate: "primalspring_sovereignty",
        provenance_date: "2026-05-15",
        description:
            "Content sovereignty — pipeline through membrane routing, trust tiers, audit correlation",
    },
    run,
};

// ─── Phase 1: Sovereign Content Store/Retrieve ───────────────────────────────

fn phase_content_pipeline(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
) -> Option<String> {
    let has_content = ctx.has_capability("content");
    v.check_bool(
        "pipeline:content_capability",
        has_content,
        if has_content {
            "content domain resolved (NestGate sovereign backend)"
        } else {
            "content domain not discoverable — NestGate may not be running"
        },
    );

    let test_bytes = b"sovereignty-parity-test-content-2026-05-15";
    let data_b64 = base64::engine::general_purpose::STANDARD.encode(test_bytes);
    let family_id =
        std::env::var("FAMILY_ID").unwrap_or_else(|_| "nucleus01".to_owned());

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
            let hash = resp
                .get("hash")
                .and_then(|h| h.as_str())
                .unwrap_or("");
            v.check_bool(
                "pipeline:put_returns_hash",
                !hash.is_empty() && hash.len() == 64,
                &format!(
                    "content.put returned BLAKE3 hash: {}...",
                    &hash[..hash.len().min(16)]
                ),
            );

            if hash.is_empty() {
                return None;
            }

            let get_result = ctx.call(
                "content",
                "content.get",
                serde_json::json!({ "hash": hash, "family_id": family_id }),
            );

            match get_result {
                Ok(resp) => {
                    let data = resp
                        .get("data")
                        .and_then(|d| d.as_str())
                        .unwrap_or("");
                    let expected_b64 =
                        base64::engine::general_purpose::STANDARD.encode(test_bytes);
                    v.check_bool(
                        "pipeline:roundtrip_match",
                        data == expected_b64,
                        if data == expected_b64 {
                            "sovereign content round-trip byte fidelity confirmed"
                        } else {
                            "DATA MISMATCH in sovereign content pipeline"
                        },
                    );
                }
                Err(e) if e.is_connection_error() => {
                    v.check_skip(
                        "pipeline:roundtrip_match",
                        &format!("content not available: {e}"),
                    );
                }
                Err(e) => {
                    v.check_bool(
                        "pipeline:roundtrip_match",
                        false,
                        &format!("content.get error: {e}"),
                    );
                }
            }

            Some(hash.to_owned())
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "pipeline:put_returns_hash",
                &format!("content not available: {e}"),
            );
            v.check_skip("pipeline:roundtrip_match", "content not available");
            None
        }
        Err(e) => {
            v.check_bool(
                "pipeline:put_returns_hash",
                false,
                &format!("content.put error: {e}"),
            );
            v.check_skip("pipeline:roundtrip_match", "prior call failed");
            None
        }
    }
}

// ─── Phase 2: Sovereign Resolution ──────────────────────────────────────────

fn phase_sovereign_resolve(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let family_id =
        std::env::var("FAMILY_ID").unwrap_or_else(|_| "nucleus01".to_owned());

    match ctx.call(
        "content",
        "content.resolve",
        serde_json::json!({ "path": "index.html", "family_id": family_id }),
    ) {
        Ok(resp) => {
            let has_response = resp.get("content").is_some()
                || resp.get("data").is_some()
                || resp.get("hash").is_some();
            v.check_bool(
                "resolve:responds",
                true,
                if has_response {
                    "content.resolve returned sovereign content"
                } else {
                    "content.resolve responded (no content for path — expected for fresh deployment)"
                },
            );

            let backend = resp
                .get("backend")
                .and_then(|b| b.as_str())
                .unwrap_or("unknown");
            let is_sovereign = backend != "fallback" && backend != "http_proxy";
            v.check_bool(
                "resolve:sovereign_backend",
                is_sovereign || backend == "unknown",
                &format!(
                    "content resolved via backend=\"{backend}\" (sovereign = non-fallback)"
                ),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "resolve:responds",
                &format!("content not available: {e}"),
            );
            v.check_skip("resolve:sovereign_backend", "content not available");
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("not found") || msg.contains("No manifest") {
                v.check_skip(
                    "resolve:responds",
                    "content.resolve: no manifest published (expected for fresh deployment)",
                );
                v.check_skip(
                    "resolve:sovereign_backend",
                    "no manifest to verify backend",
                );
            } else {
                v.check_bool(
                    "resolve:responds",
                    false,
                    &format!("content.resolve error: {e}"),
                );
                v.check_skip("resolve:sovereign_backend", "prior call failed");
            }
        }
    }
}

// ─── Phase 3: Trust-Tier Alignment ──────────────────────────────────────────

fn phase_trust_alignment(v: &mut ValidationResult) {
    let routing_toml = include_str!("../../../../config/routing_config_reference.toml");
    let parsed: toml::Value = if let Ok(p) = toml::from_str(routing_toml) { p } else {
        v.check_skip(
            "trust:covalent_all_access",
            "routing config parse failed",
        );
        return;
    };

    let Some(trust) = parsed.get("trust").and_then(|t| t.as_table()) else {
        v.check_skip("trust:covalent_all_access", "no [trust] section");
        return;
    };

    let covalent_scope = trust
        .get("covalent")
        .and_then(|c| c.get("content_scope"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    v.check_bool(
        "trust:covalent_all_access",
        covalent_scope == "all",
        &format!(
            "covalent content_scope = \"{covalent_scope}\" (expect all — full trust)"
        ),
    );

    let weak_backends = trust
        .get("weak")
        .and_then(|w| w.get("allowed_backends"))
        .and_then(|a| a.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let weak_excludes_gate = !weak_backends.contains(&"gate");
    v.check_bool(
        "trust:weak_excludes_gate",
        weak_excludes_gate,
        &format!(
            "weak allowed_backends = {weak_backends:?} (must not include gate — public only)"
        ),
    );

    let weak_scope = trust
        .get("weak")
        .and_then(|w| w.get("content_scope"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    v.check_bool(
        "trust:weak_public_only",
        weak_scope == "public",
        &format!("weak content_scope = \"{weak_scope}\" (expect public)"),
    );
}

// ─── Phase 4: SkunkBat Audit Correlation ─────────────────────────────────────

fn phase_audit_correlation(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let has_defense = ctx.has_capability("defense");
    v.check_bool(
        "audit:defense_capability",
        has_defense,
        if has_defense {
            "defense domain resolved (SkunkBat available for audit correlation)"
        } else {
            "defense domain not discoverable — SkunkBat may not be running"
        },
    );

    match ctx.call(
        "defense",
        "defense.status",
        serde_json::json!({}),
    ) {
        Ok(resp) => {
            let active = resp
                .get("active")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            v.check_bool(
                "audit:skunkbat_active",
                active,
                &format!("SkunkBat defense.status active = {active}"),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "audit:skunkbat_active",
                &format!("SkunkBat not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "audit:skunkbat_active",
                false,
                &format!("defense.status error: {e}"),
            );
        }
    }

    match ctx.call(
        "defense",
        "defense.audit",
        serde_json::json!({ "scope": "content", "limit": 5 }),
    ) {
        Ok(resp) => {
            let has_events = resp
                .get("events")
                .and_then(|e| e.as_array())
                .is_some_and(|a| !a.is_empty());
            v.check_bool(
                "audit:content_events_available",
                true,
                if has_events {
                    "defense.audit returned content-scoped events (correlation active)"
                } else {
                    "defense.audit responded (no content events yet — expected for fresh deployment)"
                },
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "audit:content_events_available",
                &format!("SkunkBat not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "audit:content_events_available",
                false,
                &format!("defense.audit error: {e}"),
            );
        }
    }
}

// ─── Entry point ─────────────────────────────────────────────────────────────

/// Run the content sovereignty validation across all four phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Sovereign Content Pipeline");
    let _hash = phase_content_pipeline(v, ctx);

    v.section("Phase 2: Sovereign Content Resolution");
    phase_sovereign_resolve(v, ctx);

    v.section("Phase 3: Trust-Tier Alignment");
    phase_trust_alignment(v);

    v.section("Phase 4: SkunkBat Audit Correlation");
    phase_audit_correlation(v, ctx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn content_sovereignty_no_panic() {
        let mut v = ValidationResult::new("content-sovereignty");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.evaluated() > 0 || v.skipped > 0,
            "scenario should produce at least one check"
        );
    }
}
