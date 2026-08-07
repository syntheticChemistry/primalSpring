// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp074: Cross-Gate Health — composition health via CompositionContext + Neural API.

use primalspring::composition::CompositionContext;
use primalspring::ipc::NeuralBridge;
use primalspring::validation::ValidationResult;

const HEALTH_DOMAINS: &[&str] = &["security", "discovery", "storage", "compute", "ai"];

fn phase_local_composition(v: &mut ValidationResult, ctx: &CompositionContext) {
    v.section("Phase 1: Local Composition Discovery");
    let caps = ctx.available_capabilities();
    if caps.is_empty() {
        v.check_skip(
            "local_composition_discovered",
            "no local capabilities in CompositionContext",
        );
    } else {
        v.check_bool(
            "local_composition_discovered",
            true,
            &format!("capabilities: {}", caps.join(", ")),
        );
    }
}

fn phase_health_via_composition(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 2: Health via CompositionContext");

    let mut live_count: u32 = 0;
    for domain in HEALTH_DOMAINS {
        let check_name = format!("{domain}_health");
        if !ctx.has_capability(domain) {
            v.check_skip(&check_name, &format!("{domain} not in composition"));
            continue;
        }
        match ctx.health_check(domain) {
            Ok(true) => {
                live_count += 1;
                println!("  {domain:<12} HEALTHY");
                v.check_bool(&check_name, true, &format!("{domain} healthy via context"));
            }
            Ok(false) => {
                println!("  {domain:<12} UNHEALTHY");
                v.check_bool(&check_name, false, &format!("{domain} unhealthy"));
            }
            Err(e) => {
                println!("  {domain:<12} UNREACHABLE");
                v.check_skip(&check_name, &format!("{domain} unreachable: {e}"));
            }
        }
    }

    let composition = match live_count {
        0 => "NO NUCLEUS",
        1..=2 => "TOWER ATOMIC (partial)",
        3 => "TOWER + one layer",
        4 => "NUCLEUS (near-complete)",
        5.. => "FULL NUCLEUS",
    };
    println!("  Composition: {composition} ({live_count}/{} domains healthy)", HEALTH_DOMAINS.len());
    v.check_bool(
        "nucleus_composition",
        live_count >= 2,
        &format!("{composition}: {live_count}/{} domains healthy", HEALTH_DOMAINS.len()),
    );
}

fn phase_neural_api_routing(v: &mut ValidationResult) {
    v.section("Phase 3: Health via Neural API routing");

    let Some(bridge) = NeuralBridge::discover() else {
        v.check_skip("neural_api_health", "biomeOS not running");
        return;
    };

    let health = bridge.health_check();
    v.check_bool(
        "neural_api_health",
        health.is_ok(),
        "biomeOS neural-api healthy",
    );

    for domain in HEALTH_DOMAINS {
        let check_name = format!("neural_{domain}_route");
        match bridge.capability_call(domain, "health.check", &serde_json::json!({})) {
            Ok(_) => v.check_bool(
                &check_name,
                true,
                &format!("{domain} routed via Neural API"),
            ),
            Err(e) => v.check_skip(&check_name, &format!("{domain}: {e}")),
        }
    }
}

#[cfg(feature = "primordial-compat")]
fn phase_legacy_tcp_probes(v: &mut ValidationResult) {
    use primalspring::ipc::methods;
    use primalspring::ipc::tcp::tcp_rpc;
    use primalspring::tolerances;

    v.section("Phase 4 (legacy): Direct TCP health probes");

    let host = std::env::var("REMOTE_GATE_HOST").unwrap_or_default();
    if host.is_empty() {
        v.check_skip("legacy_tcp_probes", "REMOTE_GATE_HOST not set");
        return;
    }

    for slug in &["beardog", "songbird", "nestgate", "toadstool", "squirrel"] {
        let Some(entry) = tolerances::port_entry_for(slug) else { continue };
        let port: u16 = std::env::var(entry.env_key)
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(entry.port);
        let check_name = format!("legacy_{slug}_health");

        match tcp_rpc(&host, port, methods::health::LIVENESS, &serde_json::json!({})) {
            Ok(_) => v.check_bool(&check_name, true, &format!("{slug} at {host}:{port}")),
            Err(e) => v.check_skip(&check_name, &format!("{slug}: {e}")),
        }
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp074 — Cross-Gate Health")
        .with_provenance("exp074_cross_gate_health", "2026-05-09")
        .run(
            "primalSpring Exp074: Composition health via CompositionContext + Neural API",
            |v| {
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_local_composition(v, &ctx);
                phase_health_via_composition(v, &mut ctx);
                phase_neural_api_routing(v);

                #[cfg(feature = "primordial-compat")]
                phase_legacy_tcp_probes(v);
            },
        );
}
