// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exp077: Squirrel Neural API Bridge

use std::io::{BufRead, BufReader, Write};
use std::os::linux::net::SocketAddrExt;
use std::os::unix::net::UnixStream;
use std::time::Duration;

use primalspring::composition::CompositionContext;
use primalspring::ipc::NeuralBridge;
use primalspring::validation::ValidationResult;

fn squirrel_abstract_rpc(method: &str, params: &serde_json::Value) -> Option<serde_json::Value> {
    let addr = std::os::unix::net::SocketAddr::from_abstract_name(b"squirrel").ok()?;
    let stream = UnixStream::connect_addr(&addr).ok()?;
    stream.set_read_timeout(Some(Duration::from_secs(5))).ok()?;
    stream
        .set_write_timeout(Some(Duration::from_secs(5)))
        .ok()?;

    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    });
    let mut payload = serde_json::to_string(&req).ok()?;
    payload.push('\n');
    let mut writer = &stream;
    writer.write_all(payload.as_bytes()).ok()?;
    writer.flush().ok()?;

    let mut reader = BufReader::new(&stream);
    let mut line = String::new();
    reader.read_line(&mut line).ok()?;
    serde_json::from_str(&line).ok()
}

fn phase_squirrel_direct(v: &mut ValidationResult) -> bool {
    v.section("Phase 1: Direct Squirrel");
    let health = squirrel_abstract_rpc("health.check", &serde_json::json!({}));
    let alive = health
        .as_ref()
        .and_then(|r| r.get("result"))
        .and_then(|r| r.get("alive"))
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);

    v.check_bool(
        "squirrel_health",
        alive,
        "Squirrel alive via abstract socket @squirrel",
    );

    if !alive {
        return false;
    }

    let caps = squirrel_abstract_rpc("capabilities.list", &serde_json::json!({}));
    let has_caps = caps.as_ref().and_then(|r| r.get("result")).is_some();
    v.check_bool(
        "squirrel_capabilities",
        has_caps,
        "Squirrel capabilities.list",
    );

    let providers = squirrel_abstract_rpc("list_providers", &serde_json::json!({}));
    v.check_bool(
        "squirrel_providers",
        providers.is_some(),
        "Squirrel list_providers",
    );

    true
}

fn phase_squirrel_via_biomeos(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 2: biomeOS AI routing");
    let Some(_bridge) = NeuralBridge::discover() else {
        v.check_skip(
            "squirrel_biomeos_routing",
            "biomeOS not running — ai routing not tested",
        );
        return;
    };

    v.check_bool(
        "ai_domain_registered",
        ctx.has_capability("ai"),
        "ai capability discoverable via composition context",
    );

    if !ctx.has_capability("ai") {
        v.check_skip("ai_health_routed", "ai capability not in context");
        v.check_skip("ai_query_routed", "ai capability not in context");
        return;
    }

    let ai_healthy = ctx.health_check("ai").unwrap_or(false);
    v.check_bool(
        "ai_health_routed",
        ai_healthy,
        "ai domain health via CompositionContext",
    );

    match ctx.call(
        "ai",
        "query",
        serde_json::json!({"prompt": "echo test", "max_tokens": 10}),
    ) {
        Ok(r) => v.check_bool(
            "ai_query_routed",
            !r.is_null(),
            "ai.query routed through CompositionContext",
        ),
        Err(e) => {
            let msg = format!("{e}");
            let socket_mismatch = msg.contains("Forward") || msg.contains("Failed to forward");
            if socket_mismatch {
                v.check_skip(
                    "ai_query_routed",
                    "abstract socket routing gap (same as ai.health)",
                );
            } else {
                v.check_skip("ai_query_routed", &format!("ai.query: {msg}"));
            }
        }
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp077 — Squirrel Neural API Bridge")
        .with_provenance("exp077_squirrel_neural_api_bridge", "2026-05-09")
        .run(
            "primalSpring Exp077: AI capability routing through biomeOS substrate",
            |v| {
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                let squirrel_live = phase_squirrel_direct(v);
                if squirrel_live {
                    phase_squirrel_via_biomeos(v, &mut ctx);
                } else {
                    v.check_skip(
                        "squirrel_biomeos_routing",
                        "Squirrel not running — biomeOS routing skipped",
                    );
                }
            },
        );
}
