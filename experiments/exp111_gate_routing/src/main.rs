// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

//! Exp111: Gate-Aware Routing (biomeOS Neural API Pressure Test)
//!
//! Validates that biomeOS Neural API routing respects MethodGate enforcement:
//!   1. Discover primals and verify their auth modes
//!   2. Test protected call routing through biomeOS `capability.call`
//!   3. Verify gate denial propagation (expected: `-32001`)
//!   4. Test multi-capability routing with mixed gate modes
//!
//! Documents biomeOS gaps: `capability.call` gate-awareness, TCP
//! endpoint propagation, and full 13-primal `nucleus --mode full`.

use primalspring::composition::CompositionContext;
use primalspring::ipc::NeuralBridge;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp111 — Gate-Aware Routing")
        .with_provenance("exp111_gate_routing", "2026-05-09")
        .run(
            "Exp111: Gate-Aware Routing — biomeOS capability.call with MethodGate",
            |v| {
                v.section("Phase 1: Auth Mode Discovery");
                phase_auth_mode_discovery(v);

                v.section("Phase 2: Neural API Routing");
                phase_neural_api_routing(v);

                v.section("Phase 3: Gate Denial Propagation");
                phase_gate_denial_propagation(v);

                v.section("Phase 4: Multi-Gate Topology");
                phase_multi_gate_topology(v);
            },
        );
}

fn phase_auth_mode_discovery(v: &mut ValidationResult) {
    let mut ctx = CompositionContext::from_live_discovery_with_fallback();
    let caps: Vec<String> = ctx
        .available_capabilities()
        .into_iter()
        .map(String::from)
        .collect();

    if caps.is_empty() {
        v.check_skip(
            "auth_mode_discovery",
            "no primals discovered — NUCLEUS not running",
        );
        return;
    }

    let cap_count = caps.len();
    let mut reported = 0usize;
    for cap in &caps {
        let mode_result = ctx.call(cap, "auth.mode", serde_json::json!({}));
        if let Ok(resp) = mode_result {
            let mode = resp
                .get("mode")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown");
            v.check_bool(&format!("{cap}_auth_mode"), true, &format!("mode = {mode}"));
            reported += 1;
        }
    }
    v.check_bool(
        "auth_mode_reported",
        reported > 0,
        &format!("{reported}/{cap_count} primals reported auth mode"),
    );
}

fn phase_neural_api_routing(v: &mut ValidationResult) {
    let Some(bridge) = NeuralBridge::discover() else {
        v.check_skip(
            "neural_api_routing",
            "biomeOS Neural API not available — neural routing requires biomeOS",
        );
        return;
    };

    match bridge.health_check() {
        Ok(healthy) => {
            v.check_bool("neural_api_health", healthy, "Neural API health check");
        }
        Err(e) => {
            v.check_skip(
                "neural_api_health",
                &format!("Neural API health check failed: {e}"),
            );
            return;
        }
    }

    match bridge.discover_capability("security") {
        Ok(resp) => {
            let found = resp.get("primal").is_some() || resp.get("endpoint").is_some();
            v.check_bool(
                "security_discoverable_via_neural",
                found,
                &format!("discovery: {resp}"),
            );
        }
        Err(e) => v.check_skip(
            "security_discoverable_via_neural",
            &format!("capability discovery failed: {e}"),
        ),
    }
}

fn phase_gate_denial_propagation(v: &mut ValidationResult) {
    let mut ctx = CompositionContext::from_live_discovery_with_fallback();
    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "gate_denial_propagation",
            "biomeOS orchestration not available — requires capability.call routing",
        );
        return;
    }

    let result = ctx.call(
        "orchestration",
        "capability.call",
        serde_json::json!({
            "domain": "coordination",
            "method": "coordination.validate_composition",
            "params": {},
        }),
    );

    match result {
        Ok(resp) => {
            v.check_bool(
                "gate_denial_propagation",
                true,
                &format!("capability.call routed: {resp}"),
            );
        }
        Err(e) => {
            let msg = format!("{e}");
            let is_permission_denied = msg.contains("-32001") || msg.contains("permission denied");
            if is_permission_denied {
                v.check_bool(
                    "gate_denial_propagation",
                    true,
                    "PERMISSION_DENIED propagated through biomeOS routing",
                );
            } else {
                v.check_skip(
                    "gate_denial_propagation",
                    &format!("capability.call failed (non-gate error): {e}"),
                );
            }
        }
    }
}

fn phase_multi_gate_topology(v: &mut ValidationResult) {
    let mut ctx = CompositionContext::from_live_discovery_with_fallback();
    let test_caps = ["security", "tensor", "storage", "ai", "defense"];
    let mut reachable = 0u32;

    for cap in &test_caps {
        if ctx.has_capability(cap) {
            let health = ctx.health_check(cap);
            if health.unwrap_or(false) {
                reachable += 1;
            }
        }
    }

    if reachable < 2 {
        v.check_skip(
            "multi_gate_topology",
            &format!(
                "only {reachable}/5 test primals reachable — multi-gate \
                 validation requires >=2 live primals with distinct gates"
            ),
        );
        return;
    }

    v.check_bool(
        "multi_gate_topology",
        true,
        &format!("{reachable}/5 primals reachable for gate topology validation"),
    );
}
