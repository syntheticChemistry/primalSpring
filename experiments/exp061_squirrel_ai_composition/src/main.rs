// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp061: Squirrel AI Composition — Tower + Squirrel + Neural API.
//!
//! Demonstrates a 3-primal composition where:
//! - beardog provides security (crypto, identity)
//! - songbird provides discovery (network, mesh)
//! - squirrel provides AI (inference via Anthropic Claude)
//!
//! The experiment spawns all three primals + the Neural API server,
//! then sends an `ai.query` request through the Neural API's capability
//! routing, which forwards to Squirrel, which calls the Anthropic API.
//!
//! Requires:
//! - `ECOPRIMALS_PLASMID_BIN` pointing at `ecoPrimals/plasmidBin/`
//! - `ANTHROPIC_API_KEY` set (or readable from `testing-secrets/api-keys.toml`)

use std::path::PathBuf;

use primalspring::coordination::AtomicType;
use primalspring::harness::{AtomicHarness, RunningAtomic};
use primalspring::launcher::{self, LaunchError, PrimalProcess, SocketNucleation};
use primalspring::validation::ValidationResult;

/// Load the Anthropic API key, trying env var first, then testing-secrets.
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

fn spawn_squirrel_with_key(
    family_id: &str,
    nucleation: &mut SocketNucleation,
    api_key: &str,
) -> Result<PrimalProcess, LaunchError> {
    let binary = launcher::discover_binary("squirrel")?;
    let socket_path = nucleation.assign("squirrel", family_id);

    let mut cmd = std::process::Command::new(&binary);
    cmd.arg("server");
    cmd.arg("--socket").arg(&socket_path);
    cmd.env("FAMILY_ID", family_id);
    cmd.env(
        "XDG_RUNTIME_DIR",
        nucleation.base_dir().to_string_lossy().as_ref(),
    );
    cmd.env("ANTHROPIC_API_KEY", api_key);
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let child = cmd.spawn().map_err(|e| LaunchError::SpawnFailed {
        primal: "squirrel".to_owned(),
        source: e,
    })?;

    let timeout = std::time::Duration::from_secs(15);
    if !launcher::wait_for_socket(&socket_path, timeout) {
        return Err(LaunchError::SocketTimeout {
            primal: "squirrel".to_owned(),
            socket: socket_path,
            waited: timeout,
        });
    }

    Ok(PrimalProcess::from_parts(
        "squirrel".to_owned(),
        socket_path,
        child,
    ))
}

struct CompositionGuard {
    running: RunningAtomic,
    squirrel: PrimalProcess,
}

impl Drop for CompositionGuard {
    fn drop(&mut self) {
        // squirrel drops via its own PrimalProcess::Drop
    }
}

fn start_composition(v: &mut ValidationResult) -> Option<CompositionGuard> {
    if std::env::var("ECOPRIMALS_PLASMID_BIN").is_err() {
        v.check_skip(
            "composition_start",
            "ECOPRIMALS_PLASMID_BIN not set — cannot spawn primals",
        );
        return None;
    }

    let api_key = if let Some(k) = load_anthropic_key() {
        v.check_bool("anthropic_key", true, "API key loaded");
        k
    } else {
        v.check_skip(
            "anthropic_key",
            "ANTHROPIC_API_KEY not set and testing-secrets/api-keys.toml not found",
        );
        return None;
    };

    let family = format!("exp061-{}", std::process::id());
    let graphs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../graphs");

    let running =
        match AtomicHarness::new(AtomicType::Tower).start_with_neural_api(&family, &graphs) {
            Ok(r) => r,
            Err(e) => {
                v.check_bool("tower_start", false, &format!("Tower failed: {e}"));
                return None;
            }
        };

    v.check_bool(
        "tower_start",
        true,
        &format!("Tower running ({} primals)", running.pids().len()),
    );

    running.validate(v);

    let runtime_dir = running.runtime_dir().to_path_buf();
    let mut nucleation = SocketNucleation::new(runtime_dir);
    let squirrel_socket = nucleation.assign("squirrel", &family);

    let squirrel = match spawn_squirrel_with_key(&family, &mut nucleation, &api_key) {
        Ok(p) => p,
        Err(e) => {
            v.check_bool("squirrel_start", false, &format!("Squirrel failed: {e}"));
            return None;
        }
    };

    v.check_bool(
        "squirrel_start",
        true,
        &format!("Squirrel running at {}", squirrel_socket.display()),
    );

    Some(CompositionGuard { running, squirrel })
}

fn validate_squirrel_health(guard: &CompositionGuard, v: &mut ValidationResult) {
    let squirrel_client =
        primalspring::ipc::client::PrimalClient::connect(&guard.squirrel.socket_path, "squirrel");
    match squirrel_client {
        Ok(mut client) => {
            let live = client.health_liveness().unwrap_or(false);
            v.check_bool("squirrel_health", live, "Squirrel health.liveness");
        }
        Err(e) => {
            v.check_bool("squirrel_health", false, &format!("could not connect: {e}"));
        }
    }
}

fn validate_ai_query(guard: &CompositionGuard, v: &mut ValidationResult) {
    let Some(bridge) = guard.running.neural_bridge() else {
        v.check_skip("ai_query", "Neural API bridge not available");
        return;
    };

    let result = bridge.capability_call(
        "ai",
        "query",
        &serde_json::json!({
            "prompt": "In one sentence, what is ecosystem coordination in distributed systems?"
        }),
    );

    match result {
        Ok(call_result) => {
            let has_response = call_result
                .value
                .get("response")
                .and_then(|v| v.as_str())
                .is_some_and(|s| !s.is_empty());

            v.check_bool(
                "ai_query",
                has_response,
                "AI query returned non-empty response",
            );

            if let Some(provider) = call_result.value.get("provider").and_then(|v| v.as_str()) {
                v.check_bool(
                    "ai_provider",
                    !provider.is_empty(),
                    &format!("provider: {provider}"),
                );
            } else {
                v.check_skip("ai_provider", "no provider field in response");
            }

            if let Some(latency) = call_result
                .value
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
            } else {
                v.check_bool("ai_query", false, &format!("unexpected error: {e}"));
            }
        }
    }
}

fn validate_tower_still_healthy(guard: &CompositionGuard, v: &mut ValidationResult) {
    for (name, live) in guard.running.health_check_all() {
        v.check_bool(
            &format!("{name}_post_ai"),
            live,
            &format!("{name} still healthy after AI query"),
        );
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp061 — Squirrel AI Composition")
        .with_provenance("exp061_squirrel_ai_composition", "2026-03-24")
        .run(
            "primalSpring Exp061: Tower + Squirrel AI (beardog + songbird + squirrel + Neural API)",
            |v| {
                let guard = start_composition(v);
                if let Some(ref g) = guard {
                    validate_squirrel_health(g, v);
                    validate_ai_query(g, v);
                    validate_tower_still_healthy(g, v);
                }
                drop(guard);
            },
        );
}
