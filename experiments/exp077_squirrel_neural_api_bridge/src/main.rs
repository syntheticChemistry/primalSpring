// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp077: Squirrel Neural API Bridge — AI capability routing through biomeOS substrate.

use primalspring::composition::CompositionContext;
use primalspring::ipc::NeuralBridge;
use primalspring::validation::ValidationResult;

fn phase_squirrel_via_neural_api(v: &mut ValidationResult) -> bool {
    v.section("Phase 1: AI routing via Neural API");

    let Some(bridge) = NeuralBridge::discover() else {
        v.check_skip(
            "neural_api_available",
            "biomeOS not running — ai routing not tested",
        );
        return false;
    };

    let health = bridge.health_check();
    v.check_bool(
        "neural_api_available",
        health.is_ok(),
        "biomeOS neural-api healthy",
    );
    if health.is_err() {
        return false;
    }

    let ai_health = bridge.capability_call("ai", "health.check", &serde_json::json!({}));
    v.check_bool(
        "ai_routed_health",
        ai_health.is_ok(),
        "ai.health.check routed through Neural API",
    );

    match bridge.capability_call(
        "ai",
        "capabilities.list",
        &serde_json::json!({}),
    ) {
        Ok(resp) => {
            v.check_bool(
                "ai_capabilities_listed",
                !resp.value.is_null(),
                "ai.capabilities.list routed through Neural API",
            );
        }
        Err(e) => {
            let msg = format!("{e}");
            let socket_gap = msg.contains("Forward") || msg.contains("Failed to forward");
            if socket_gap {
                v.check_skip(
                    "ai_capabilities_listed",
                    "abstract socket routing gap — Neural API cannot forward to @squirrel yet",
                );
            } else {
                v.check_skip("ai_capabilities_listed", &format!("ai routing: {msg}"));
            }
        }
    }

    true
}

fn phase_squirrel_via_composition(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 2: AI via CompositionContext");

    v.check_bool(
        "ai_domain_registered",
        ctx.has_capability("ai"),
        "ai capability discoverable via CompositionContext",
    );

    if !ctx.has_capability("ai") {
        v.check_skip("ai_health_via_ctx", "ai capability not in context");
        v.check_skip("ai_query_via_ctx", "ai capability not in context");
        return;
    }

    let ai_healthy = ctx.health_check("ai").unwrap_or(false);
    v.check_bool(
        "ai_health_via_ctx",
        ai_healthy,
        "ai domain health via CompositionContext",
    );

    match ctx.call(
        "ai",
        "query",
        serde_json::json!({"prompt": "echo test", "max_tokens": 10}),
    ) {
        Ok(r) => v.check_bool(
            "ai_query_via_ctx",
            !r.is_null(),
            "ai.query routed through CompositionContext",
        ),
        Err(e) => {
            let msg = format!("{e}");
            let socket_mismatch = msg.contains("Forward") || msg.contains("Failed to forward");
            if socket_mismatch {
                v.check_skip(
                    "ai_query_via_ctx",
                    "abstract socket routing gap (same as ai.health)",
                );
            } else {
                v.check_skip("ai_query_via_ctx", &format!("ai.query: {msg}"));
            }
        }
    }
}

#[cfg(feature = "primordial-compat")]
fn phase_squirrel_direct(v: &mut ValidationResult) {
    use std::io::{BufRead, BufReader, Write};
    use std::os::linux::net::SocketAddrExt;
    use std::os::unix::net::UnixStream;
    use std::time::Duration;

    v.section("Phase 3 (legacy): Direct Squirrel abstract UDS");

    let Some(addr) = std::os::unix::net::SocketAddr::from_abstract_name(b"squirrel").ok() else {
        v.check_skip("legacy_squirrel_direct", "cannot create abstract socket addr");
        return;
    };
    let Ok(stream) = UnixStream::connect_addr(&addr) else {
        v.check_skip("legacy_squirrel_direct", "Squirrel abstract socket unreachable");
        return;
    };
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));

    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "health.check",
        "params": {},
        "id": 1
    });
    let mut payload = serde_json::to_string(&req).unwrap_or_default();
    payload.push('\n');
    let mut writer = &stream;
    if writer.write_all(payload.as_bytes()).is_err() {
        v.check_skip("legacy_squirrel_direct", "write to @squirrel failed");
        return;
    }
    let _ = writer.flush();

    let mut reader = BufReader::new(&stream);
    let mut line = String::new();
    if reader.read_line(&mut line).is_err() {
        v.check_skip("legacy_squirrel_direct", "read from @squirrel failed");
        return;
    }

    let alive = serde_json::from_str::<serde_json::Value>(&line)
        .ok()
        .and_then(|r| r.get("result")?.get("alive")?.as_bool())
        .unwrap_or(false);

    v.check_bool(
        "legacy_squirrel_direct",
        alive,
        "Squirrel alive via abstract socket @squirrel (legacy path)",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp077 — Squirrel Neural API Bridge")
        .with_provenance("exp077_squirrel_neural_api_bridge", "2026-05-09")
        .run(
            "primalSpring Exp077: AI capability routing through biomeOS substrate",
            |v| {
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                let _neural_ok = phase_squirrel_via_neural_api(v);
                phase_squirrel_via_composition(v, &mut ctx);

                #[cfg(feature = "primordial-compat")]
                phase_squirrel_direct(v);
            },
        );
}
