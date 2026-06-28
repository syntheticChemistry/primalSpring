// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: PetalTongue Visualization — dashboard and render pipeline.
//!
//! Validates PetalTongue's visualization capabilities as the ecosystem's
//! proprioceptive surface. PetalTongue renders composition state, gate
//! topology, and primal health into dashboards.
//!
//! Phases:
//! 1. Capability coverage: viz.*, render.*, interaction.* methods registered
//! 2. Domain routing: all viz domains resolve to petaltongue
//! 3. Dashboard contract: expected query/render/stream surface
//! 4. Web UI alignment: play.html exists (static dashboard shell)
//! 5. Live: PetalTongue health + viz.query dispatch

use crate::composition::{CompositionContext, capability_to_primal};
use crate::primal_names;
use crate::tolerances::ports;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// PetalTongue visualization scenario metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "petaltongue-viz",
        track: Track::Lifecycle,
        tier: Tier::Both,
        provenance_crate: "wave124_petaltongue_viz",
        provenance_date: "2026-06-23",
        description: "PetalTongue visualization — dashboard, render pipeline, interaction surface",
    },
    run,
};

/// Expected visualization methods.
const VIZ_METHODS: &[(&str, &str)] = &[
    ("viz.dashboard", "ecosystem dashboard render"),
    ("viz.query", "composition state queries"),
    ("viz.render", "render pipeline dispatch"),
    ("viz.stream", "real-time event stream"),
    ("render.dashboard", "dashboard render entry"),
    ("interaction.poll", "interaction event polling"),
    ("interaction.subscribe", "event subscription"),
];

/// Run all PetalTongue visualization phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Capability coverage");
    phase_capabilities(v);

    v.section("Phase 2: Domain routing");
    phase_routing(v);

    v.section("Phase 3: Dashboard contract");
    phase_dashboard_contract(v);

    v.section("Phase 4: Web UI alignment");
    phase_web_ui(v);

    v.section("Phase 5: Live PetalTongue");
    phase_live(v, ctx);
}

fn phase_capabilities(v: &mut ValidationResult) {
    let mut registered = 0;
    for (method, desc) in VIZ_METHODS {
        let present = REGISTRY_TOML.contains(method);
        if present {
            registered += 1;
        }
        v.check_bool(
            &format!("cap:{}", method.replace('.', "_")),
            present,
            &format!("{method} — {desc}"),
        );
    }

    v.check_bool(
        "cap:viz_breadth",
        registered >= 6,
        &format!("{registered}/{} viz methods registered", VIZ_METHODS.len()),
    );

    let domains = ["viz", "render", "interaction"];
    for domain in domains {
        let has_section = REGISTRY_TOML.contains(&format!("[{domain}]"));
        v.check_bool(
            &format!("cap:domain_{domain}"),
            has_section,
            &format!("[{domain}] section in capability_registry.toml"),
        );
    }
}

fn phase_routing(v: &mut ValidationResult) {
    let domains = ["viz", "render", "interaction"];
    for domain in domains {
        let primal = capability_to_primal(domain);
        v.check_bool(
            &format!("route:{domain}_to_petaltongue"),
            primal == primal_names::PETALTONGUE,
            &format!("{domain} → \"{primal}\" (expected petaltongue)"),
        );
    }

    let port = ports::default_port_for(primal_names::PETALTONGUE);
    v.check_bool(
        "route:petaltongue_port",
        port > 0,
        &format!("petaltongue port = {port}"),
    );
}

fn phase_dashboard_contract(v: &mut ValidationResult) {
    let has_dashboard = REGISTRY_TOML.contains("viz.dashboard");
    let has_query = REGISTRY_TOML.contains("viz.query");
    let has_stream = REGISTRY_TOML.contains("viz.stream");
    let has_render = REGISTRY_TOML.contains("viz.render");

    v.check_bool(
        "contract:dashboard_entry",
        has_dashboard,
        "viz.dashboard method for ecosystem overview",
    );
    v.check_bool(
        "contract:query_surface",
        has_query,
        "viz.query for composition state inspection",
    );
    v.check_bool(
        "contract:stream_surface",
        has_stream,
        "viz.stream for real-time event subscription",
    );
    v.check_bool(
        "contract:render_pipeline",
        has_render,
        "viz.render for format-agnostic output",
    );

    let has_sensor_poll = REGISTRY_TOML.contains("interaction.sensor_stream.poll");
    v.check_bool(
        "contract:sensor_stream",
        has_sensor_poll,
        "interaction.sensor_stream.poll for proprioceptive input",
    );
}

fn phase_web_ui(v: &mut ValidationResult) {
    v.check_skip(
        "web:viz_served_by_primal",
        "petalTongue serves viz natively (web/ fossilized Wave 128)",
    );
}

fn phase_live(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    match ctx.client_for("viz") {
        Some(client) => {
            let resp = client.call("health.liveness", serde_json::json!({}));
            match resp {
                Ok(r) => {
                    v.check_bool(
                        "live:petaltongue_health",
                        r.is_success(),
                        "PetalTongue responding to health.liveness",
                    );
                }
                Err(e) if e.is_skippable() => {
                    v.check_skip("live:petaltongue_health", &format!("{e}"));
                }
                Err(e) => {
                    v.check_bool("live:petaltongue_health", false, &format!("{e}"));
                }
            }
        }
        None => {
            v.check_skip("live:petaltongue_health", "no viz client available");
        }
    }

    match ctx.client_for("viz") {
        Some(client) => {
            let resp = client.call("viz.query", serde_json::json!({"scope": "topology"}));
            match resp {
                Ok(r) => {
                    v.check_bool(
                        "live:viz_query_responds",
                        r.is_success(),
                        "viz.query returns topology data",
                    );
                }
                Err(e) if e.is_skippable() => {
                    v.check_skip("live:viz_query_responds", &format!("{e}"));
                }
                Err(e) => {
                    v.check_bool("live:viz_query_responds", false, &format!("{e}"));
                }
            }
        }
        None => {
            v.check_skip("live:viz_query_responds", "no viz client");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn petaltongue_viz_runs() {
        let mut v = ValidationResult::new("petaltongue-viz");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        let total = v.passed + v.failed + v.skipped;
        assert!(total >= 18, "expected ≥18 checks, got {total}");
    }

    #[test]
    fn viz_methods_registered() {
        let mut count = 0;
        for (method, _) in VIZ_METHODS {
            if REGISTRY_TOML.contains(method) {
                count += 1;
            }
        }
        assert!(count >= 6, "expected ≥6 viz methods, got {count}");
    }

    #[test]
    fn petaltongue_owns_viz() {
        assert_eq!(capability_to_primal("viz"), primal_names::PETALTONGUE);
    }

    #[test]
    fn petaltongue_owns_render() {
        assert_eq!(capability_to_primal("render"), primal_names::PETALTONGUE);
    }

    #[test]
    fn petaltongue_owns_interaction() {
        assert_eq!(
            capability_to_primal("interaction"),
            primal_names::PETALTONGUE
        );
    }

    #[test]
    fn petaltongue_has_port() {
        let port = ports::default_port_for(primal_names::PETALTONGUE);
        assert!(port > 0, "petaltongue should have a non-zero port");
    }

    #[test]
    fn dashboard_contract_complete() {
        assert!(REGISTRY_TOML.contains("viz.dashboard"));
        assert!(REGISTRY_TOML.contains("viz.query"));
        assert!(REGISTRY_TOML.contains("viz.stream"));
        assert!(REGISTRY_TOML.contains("viz.render"));
    }
}
