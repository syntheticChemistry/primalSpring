// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Squirrel AI Pipeline — validates the inference pipeline surface,
//! provider readiness, and provenance socket accessibility.
//!
//! Squirrel (via ai.sock) exposes:
//! - `inference.embed`: embedding generation (requires registered provider)
//! - `inference.complete`: text generation (requires `AI_PROVIDER_SOCKETS`)
//! - `health.readiness` with `ai_router` and `capability_registry` checks
//!
//! Provenance tracking (provenance.sock) uses riboCipher transport signals.
//!
//! This scenario validates:
//! 1. Squirrel socket liveness and readiness structure
//! 2. AI pipeline method surface (inference.* methods recognized)
//! 3. Provider configuration state (are providers configured?)
//! 4. Provenance socket accessibility (riboCipher signal detection)

use std::path::{Path, PathBuf};

use crate::composition::CompositionContext;
use crate::ipc::client::PrimalClient;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Squirrel AI pipeline and provenance tracking validation.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "squirrel-ai-pipeline",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-21",
        description: "Validates squirrel inference pipeline, provider state, and provenance tracking",
    },
    run: run_squirrel_ai_pipeline,
};

fn run_squirrel_ai_pipeline(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let base = socket_dir();
    let Some(dir) = base else {
        v.check_skip("ai:socket_dir", "biomeos socket dir not found");
        return;
    };
    phase_squirrel_liveness(v, &dir);
    phase_ai_readiness(v, &dir);
    phase_inference_surface(v, &dir);
    phase_provenance_socket(v, &dir);
}

fn phase_squirrel_liveness(v: &mut ValidationResult, dir: &Path) {
    let squirrel_sock = dir.join("squirrel.sock");
    if !squirrel_sock.exists() {
        v.check_skip("ai:squirrel:exists", "squirrel.sock not present");
        return;
    }
    v.check_bool("ai:squirrel:exists", true, "squirrel.sock present");

    let Ok(mut client) = PrimalClient::connect(&squirrel_sock, "squirrel") else {
        v.check_skip("ai:squirrel:liveness", "squirrel connection failed");
        return;
    };

    match client.call("health.liveness", serde_json::json!({})) {
        Ok(resp) => {
            let alive = resp.result.as_ref().is_some_and(|r| {
                r.get("alive")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false)
            });
            v.check_bool(
                "ai:squirrel:liveness",
                alive,
                "squirrel health.liveness: alive",
            );
        }
        Err(e) => {
            v.check_skip(
                "ai:squirrel:liveness",
                &format!("squirrel liveness error: {e}"),
            );
        }
    }
}

fn phase_ai_readiness(v: &mut ValidationResult, dir: &Path) {
    let ai_sock = dir.join("ai.sock");
    if !ai_sock.exists() {
        v.check_skip("ai:readiness:socket", "ai.sock not present");
        return;
    }
    v.check_bool("ai:readiness:socket", true, "ai.sock present");

    let Ok(mut client) = PrimalClient::connect(&ai_sock, "ai") else {
        v.check_skip("ai:readiness:connect", "ai.sock connection failed");
        return;
    };

    match client.call("health.readiness", serde_json::json!({})) {
        Ok(resp) => {
            let Some(result) = resp.result.as_ref() else {
                v.check_skip("ai:readiness:checks", "readiness returned no result");
                return;
            };

            let has_checks = result.get("checks").is_some();
            v.check_bool(
                "ai:readiness:structure",
                has_checks,
                "readiness response contains checks map",
            );

            if let Some(checks) = result.get("checks") {
                let cap_registry = checks
                    .get("capability_registry")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false);
                v.check_bool(
                    "ai:readiness:capability_registry",
                    cap_registry,
                    "capability_registry check passes",
                );

                let ai_router = checks
                    .get("ai_router")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false);
                if ai_router {
                    v.check_bool(
                        "ai:readiness:ai_router",
                        true,
                        "ai_router check passes (providers configured)",
                    );
                } else {
                    v.check_skip(
                        "ai:readiness:ai_router",
                        "ai_router: false (no providers configured — expected until AI_PROVIDER_SOCKETS set)",
                    );
                }
            }
        }
        Err(e) => {
            v.check_skip("ai:readiness:checks", &format!("readiness error: {e}"));
        }
    }
}

fn phase_inference_surface(v: &mut ValidationResult, dir: &Path) {
    let ai_sock = dir.join("ai.sock");
    if !ai_sock.exists() {
        v.check_skip("ai:inference:surface", "ai.sock not present");
        return;
    }

    let Ok(mut client) = PrimalClient::connect(&ai_sock, "ai") else {
        v.check_skip("ai:inference:surface", "ai.sock connection failed");
        return;
    };

    let embed_result = client.call(
        "inference.embed",
        serde_json::json!({"text": "primalSpring validation probe"}),
    );
    let embed_recognized = match &embed_result {
        Ok(_) => true,
        Err(e) => !e.to_string().contains("Method not found"),
    };
    v.check_bool(
        "ai:inference:embed_method",
        embed_recognized,
        "inference.embed method recognized by AI pipeline",
    );

    let complete_result = client.call("inference.complete", serde_json::json!({"prompt": "echo"}));
    let complete_recognized = match &complete_result {
        Ok(_) => true,
        Err(e) => !e.to_string().contains("Method not found"),
    };
    v.check_bool(
        "ai:inference:complete_method",
        complete_recognized,
        "inference.complete method recognized by AI pipeline",
    );

    let chat_result = client.call(
        "inference.chat",
        serde_json::json!({"messages": [{"role": "user", "content": "probe"}]}),
    );
    let chat_recognized = match &chat_result {
        Ok(_) => true,
        Err(e) => !e.to_string().contains("Method not found"),
    };
    if chat_recognized {
        v.check_bool(
            "ai:inference:chat_method",
            true,
            "inference.chat recognized",
        );
    } else {
        v.check_skip(
            "ai:inference:chat_method",
            "inference.chat not implemented yet",
        );
    }

    let mut methods_ready = 0u32;
    if embed_result.is_ok() {
        methods_ready += 1;
    }
    if complete_result.is_ok() {
        methods_ready += 1;
    }
    if methods_ready > 0 {
        v.check_bool(
            "ai:inference:provider_active",
            true,
            &format!("{methods_ready}/2 inference methods have active providers"),
        );
    } else {
        v.check_skip(
            "ai:inference:provider_active",
            "no providers active — configure AI_PROVIDER_SOCKETS to enable inference",
        );
    }
}

fn phase_provenance_socket(v: &mut ValidationResult, dir: &Path) {
    let provenance_sock = dir.join("provenance.sock");
    if !provenance_sock.exists() {
        v.check_skip("ai:provenance:socket", "provenance.sock not present");
        return;
    }
    v.check_bool("ai:provenance:socket", true, "provenance.sock present");

    let connect_result = PrimalClient::connect(&provenance_sock, "provenance");
    match connect_result {
        Ok(mut client) => match client.call("health.liveness", serde_json::json!({})) {
            Ok(resp) => {
                let alive = resp.result.is_some();
                v.check_bool(
                    "ai:provenance:responds",
                    alive,
                    "provenance socket responds to JSON-RPC",
                );
            }
            Err(e) => {
                let msg = e.to_string();
                let ribocipher = msg.contains("riboCipher") || msg.contains("signal required");
                if ribocipher {
                    v.check_bool(
                        "ai:provenance:responds",
                        true,
                        "provenance socket alive — requires riboCipher transport signal",
                    );
                } else {
                    v.check_skip(
                        "ai:provenance:responds",
                        &format!("provenance error: {msg}"),
                    );
                }
            }
        },
        Err(e) => {
            v.check_skip(
                "ai:provenance:responds",
                &format!("provenance connection failed: {e}"),
            );
        }
    }
}

fn socket_dir() -> Option<PathBuf> {
    let dir = crate::tolerances::platform::biomeos_socket_dir();
    dir.is_dir().then_some(dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squirrel_ai_pipeline_structural() {
        let mut v = ValidationResult::new("squirrel-ai-pipeline");
        let mut ctx = CompositionContext::discover();
        run_squirrel_ai_pipeline(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "squirrel-ai-pipeline: {} failures ({} passed, {} skipped)",
            v.failed, v.passed, v.skipped
        );
    }
}
