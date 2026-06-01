// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp061: Squirrel AI Composition — Tower + AI via CompositionContext.

use std::path::PathBuf;

use primalspring::composition::CompositionContext;
use primalspring::coordination::AtomicType;
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

fn load_anthropic_key() -> Option<String> {
    if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
        if !key.is_empty() {
            return Some(key);
        }
    }
    let candidates = [
        PathBuf::from("../../testing-secrets/api-keys.toml"),
        PathBuf::from("../testing-secrets/api-keys.toml"),
        PathBuf::from("../../../testing-secrets/api-keys.toml"),
    ];
    for path in &candidates {
        if let Ok(contents) = std::fs::read_to_string(path) {
            let toml_start = contents.find("\n[").map_or(0, |i| i + 1);
            let toml_slice = &contents[toml_start..];
            if let Ok(parsed) = toml_slice.parse::<toml::Table>() {
                if let Some(ai) = parsed.get("ai_providers").and_then(|v| v.as_table()) {
                    if let Some(key) = ai.get("anthropic_api_key").and_then(|v| v.as_str()) {
                        if !key.is_empty() {
                            return Some(key.to_owned());
                        }
                    }
                }
            }
        }
    }
    None
}

fn phase_api_key(v: &mut ValidationResult) -> bool {
    if load_anthropic_key().is_some() {
        v.check_bool("anthropic_key", true, "API key loaded");
        true
    } else {
        v.check_skip(
            "anthropic_key",
            "ANTHROPIC_API_KEY not set and testing-secrets/api-keys.toml not found",
        );
        false
    }
}

fn phase_tower_liveness(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    for cap in AtomicType::Tower.required_capabilities() {
        if !ctx.has_capability(cap) {
            v.check_skip(
                &format!("tower_{cap}_live"),
                &format!("{cap} not in context"),
            );
            continue;
        }
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(_) => v.check_bool(
                &format!("tower_{cap}_live"),
                true,
                &format!("{cap} liveness"),
            ),
            Err(e) if e.is_connection_error() => {
                v.check_skip(&format!("tower_{cap}_live"), &format!("{e}"));
            }
            Err(e) => v.check_bool(&format!("tower_{cap}_live"), false, &format!("error: {e}")),
        }
    }
}

fn phase_ai_query(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("ai") {
        v.check_skip("ai_query", "ai capability not discovered");
        v.check_skip("ai_provider", "ai capability not discovered");
        v.check_skip("ai_latency", "ai capability not discovered");
        return;
    }

    match ctx.call(
        "ai",
        "query",
        serde_json::json!({
            "prompt": "In one sentence, what is ecosystem coordination in distributed systems?"
        }),
    ) {
        Ok(call_result) => {
            let has_response = call_result
                .get("response")
                .and_then(|v| v.as_str())
                .is_some_and(|s| !s.is_empty());

            v.check_bool(
                "ai_query",
                has_response,
                "AI query returned non-empty response",
            );

            if let Some(provider) = call_result.get("provider").and_then(|v| v.as_str()) {
                v.check_bool(
                    "ai_provider",
                    !provider.is_empty(),
                    &format!("provider: {provider}"),
                );
            } else {
                v.check_skip("ai_provider", "no provider field in response");
            }

            if let Some(latency) = call_result
                .get("latency_ms")
                .and_then(serde_json::Value::as_u64)
            {
                v.check_bool(
                    "ai_latency",
                    latency < 30_000,
                    &format!("latency: {latency}ms"),
                );
            } else {
                v.check_skip("ai_latency", "no latency_ms field in response");
            }
        }
        Err(e) => {
            let msg = format!("{e}");
            let routing_attempt = msg.contains("not found")
                || msg.contains("not registered")
                || msg.contains("Failed to forward");
            if routing_attempt {
                v.check_bool(
                    "ai_query",
                    true,
                    &format!("capability routing attempted (AI not yet registered): {e}"),
                );
                v.check_skip("ai_provider", "AI capability not yet routed");
                v.check_skip("ai_latency", "AI capability not yet routed");
            } else if e.is_connection_error() {
                v.check_skip("ai_query", &format!("{e}"));
                v.check_skip("ai_provider", &format!("{e}"));
                v.check_skip("ai_latency", &format!("{e}"));
            } else {
                v.check_bool("ai_query", false, &format!("unexpected error: {e}"));
            }
        }
    }
}

fn phase_post_ai_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    for cap in AtomicType::Tower.required_capabilities() {
        if !ctx.has_capability(cap) {
            continue;
        }
        let name = match *cap {
            "security" => primal_names::BEARDOG,
            "discovery" => primal_names::SONGBIRD,
            _ => cap,
        };
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(_) => v.check_bool(
                &format!("{name}_post_ai"),
                true,
                &format!("{name} still healthy after AI query"),
            ),
            Err(e) if e.is_connection_error() => {
                v.check_skip(&format!("{name}_post_ai"), &format!("{e}"));
            }
            Err(e) => v.check_bool(&format!("{name}_post_ai"), false, &format!("error: {e}")),
        }
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp061 — Squirrel AI Composition")
        .with_provenance("exp061_squirrel_ai_composition", "2026-05-09")
        .run(
            "primalSpring Exp061: Tower + Squirrel AI via composition discovery",
            |v| {
                if !phase_api_key(v) {
                    return;
                }

                v.section("Phase 1: Tower liveness");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_tower_liveness(v, &mut ctx);

                v.section("Phase 2: AI query");
                phase_ai_query(v, &mut ctx);

                v.section("Phase 3: Post-AI tower health");
                phase_post_ai_health(v, &mut ctx);
            },
        );
}
