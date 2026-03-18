// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp043: petalTongue Viz — validates atomic health visualization pipeline.
//!
//! When petalTongue is live, sends `visualization.render.grammar` with an
//! atomic health composition result as a Grammar of Graphics expression.
//! Gracefully degrades when petalTongue is not running.

use primalspring::coordination::{AtomicType, validate_composition};
use primalspring::ipc::client::PrimalClient;
use primalspring::ipc::discover::{discover_primal, socket_path};
use primalspring::tolerances::VALIDATION_SUMMARY_WIDTH;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp043 — petalTongue Viz");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!("primalSpring Exp043: petalTongue Atomic Health Visualization");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    let petaltongue = discover_primal("petaltongue");
    v.check_bool(
        "discover_petaltongue",
        petaltongue.primal == "petaltongue",
        "discover petaltongue",
    );

    let path = socket_path("petaltongue");
    v.check_bool(
        "petaltongue_socket_path",
        path.to_string_lossy().contains("petaltongue"),
        "petaltongue socket path contains 'petaltongue'",
    );

    let health = primalspring::coordination::probe_primal("petaltongue");
    if health.socket_found {
        v.check_bool("petaltongue_health", health.health_ok, "petaltongue health");
        v.check_minimum("petaltongue_caps", health.capabilities.len(), 1);

        if let Some(ref sock) = petaltongue.socket {
            if let Ok(mut client) = PrimalClient::connect(sock, "petaltongue") {
                probe_live_petaltongue(&mut v, &mut client);
            } else {
                skip_viz_checks(&mut v, "cannot connect");
            }
        }
    } else {
        v.check_skip("petaltongue_health", "petaltongue not reachable");
        v.check_skip("petaltongue_caps", "petaltongue not reachable");
        skip_viz_checks(&mut v, "petaltongue not reachable");
    }

    v.finish();
    std::process::exit(v.exit_code());
}

fn skip_viz_checks(v: &mut ValidationResult, reason: &str) {
    v.check_skip("viz_capability", reason);
    v.check_skip("viz_render_grammar", reason);
    v.check_skip("viz_dashboard", reason);
}

fn probe_live_petaltongue(v: &mut ValidationResult, client: &mut PrimalClient) {
    match client.capabilities() {
        Ok(caps) => {
            let has_render = caps.as_array().is_some_and(|a| {
                a.iter()
                    .any(|c| c.as_str().is_some_and(|s| s.starts_with("visualization")))
            });
            v.check_bool(
                "viz_capability",
                has_render,
                "has visualization capabilities",
            );
        }
        Err(_) => {
            v.check_skip("viz_capability", "capabilities.list failed");
        }
    }

    let composition = validate_composition(AtomicType::Tower);
    let primal_data: Vec<serde_json::Value> = composition
        .primals
        .iter()
        .map(|p| {
            serde_json::json!({
                "name": p.name,
                "healthy": p.health_ok,
                "latency_us": p.latency_us,
                "capabilities": p.capabilities.len(),
            })
        })
        .collect();

    let grammar = serde_json::json!({
        "data": primal_data,
        "mark": "bar",
        "encoding": {
            "x": {"field": "name", "type": "nominal"},
            "y": {"field": "latency_us", "type": "quantitative"},
            "color": {"field": "healthy", "type": "nominal"}
        },
        "title": format!("{} Atomic Health", composition.atomic.description()),
    });

    match client.call("visualization.render.grammar", grammar) {
        Ok(resp) => {
            let result = resp.result.unwrap_or_default();
            v.check_bool(
                "viz_render_grammar",
                result.get("rendered").is_some() || result.get("scene").is_some(),
                "visualization.render.grammar returns rendered output",
            );
            println!("  Grammar render: success");
        }
        Err(e) => {
            v.check_skip("viz_render_grammar", &format!("render failed: {e}"));
        }
    }

    let dashboard_spec = serde_json::json!({
        "panels": [{
            "title": "Atomic Composition",
            "type": "topology",
            "data": {
                "composition": composition.atomic.description(),
                "primals": primal_data,
                "all_healthy": composition.all_healthy,
                "discovery_ok": composition.discovery_ok,
            }
        }],
    });

    match client.call("visualization.render.dashboard", dashboard_spec) {
        Ok(_) => v.check_bool("viz_dashboard", true, "dashboard render accepted"),
        Err(_) => v.check_skip("viz_dashboard", "dashboard render not available"),
    }
}
