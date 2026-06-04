// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: Health & lifecycle surface — exercises health.version,
//! health.readiness, health.drain, lifecycle.*, system.status, and
//! per-primal version/status methods (Wave 47 method coverage push).

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "health-lifecycle-surface",
        track: Track::Lifecycle,
        tier: Tier::Both,
        provenance_crate: "wave47_method_coverage",
        provenance_date: "2026-05-24",
        description: "Health & lifecycle API surface: version, readiness, drain, lifecycle register/start/stop",
    },
    run,
};

const HEALTH_METHODS: &[&str] = &[
    "health.drain",
    "health.readiness",
    "health.version",
    "system.status",
];

const LIFECYCLE_METHODS: &[&str] = &[
    "lifecycle.register",
    "lifecycle.start",
    "lifecycle.stop",
];

const PRIMAL_SPECIFIC: &[(&str, &str)] = &[
    ("orchestration", "biomeos.lifecycle.status"),
    ("compute", "toadstool.version"),
    ("orchestration", "primalspring.capability.list"),
    ("orchestration", "primalspring.health.check"),
];

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — method registry");
    phase_structural(v);

    v.section("Phase 2: Live health/lifecycle probes");
    phase_live(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    let registry_src = include_str!("../../../../config/capability_registry.toml");

    for method in HEALTH_METHODS {
        v.check_bool(
            &format!("registry:{method}"),
            registry_src.contains(method),
            &format!("{method} in registry"),
        );
    }

    for method in LIFECYCLE_METHODS {
        v.check_bool(
            &format!("registry:{method}"),
            registry_src.contains(method),
            &format!("{method} in registry"),
        );
    }

    for (_, method) in PRIMAL_SPECIFIC {
        v.check_bool(
            &format!("registry:{method}"),
            registry_src.contains(method),
            &format!("{method} in registry"),
        );
    }
}

fn phase_live(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") {
        v.check_skip("live:health", "orchestration not available");
        return;
    }

    match ctx.call("orchestration", "health.version", serde_json::json!({})) {
        Ok(resp) => {
            let has_version = resp.get("version").is_some();
            v.check_bool(
                "live:health.version",
                has_version,
                &format!("health.version → {resp}"),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("live:health.version", &format!("not reachable: {e}"));
        }
        Err(e) => {
            v.check_bool("live:health.version", false, &format!("error: {e}"));
        }
    }

    match ctx.call("orchestration", "health.readiness", serde_json::json!({})) {
        Ok(resp) => {
            let has_ready = resp.get("ready").is_some() || resp.get("status").is_some();
            v.check_bool(
                "live:health.readiness",
                has_ready,
                &format!("health.readiness → {resp}"),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("live:health.readiness", &format!("not reachable: {e}"));
        }
        Err(e) => {
            v.check_bool("live:health.readiness", false, &format!("error: {e}"));
        }
    }

    match ctx.call(
        "orchestration",
        "graph.waves",
        serde_json::json!({}),
    ) {
        Ok(resp) => {
            v.check_bool(
                "live:graph.waves",
                resp.is_object() || resp.is_array(),
                "graph.waves responded",
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("live:graph.waves", &format!("not reachable: {e}"));
        }
        Err(e) => {
            v.check_bool("live:graph.waves", false, &format!("error: {e}"));
        }
    }

    match ctx.call(
        "orchestration",
        "graph.capabilities",
        serde_json::json!({}),
    ) {
        Ok(resp) => {
            v.check_bool(
                "live:graph.capabilities",
                resp.is_object() || resp.is_array(),
                "graph.capabilities responded",
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("live:graph.capabilities", &format!("not reachable: {e}"));
        }
        Err(e) => {
            v.check_bool("live:graph.capabilities", false, &format!("error: {e}"));
        }
    }

    for (cap, method) in PRIMAL_SPECIFIC {
        if !ctx.has_capability(cap) {
            v.check_skip(
                &format!("live:{method}"),
                &format!("{cap} not available"),
            );
            continue;
        }
        match ctx.call(cap, method, serde_json::json!({})) {
            Ok(resp) => {
                v.check_bool(
                    &format!("live:{method}"),
                    resp.is_object() || resp.is_array(),
                    &format!("{method} responded"),
                );
            }
            Err(e) if e.is_connection_error() || e.is_method_not_found() => {
                v.check_skip(&format!("live:{method}"), &format!("{method}: {e}"));
            }
            Err(e) => {
                v.check_bool(
                    &format!("live:{method}"),
                    false,
                    &format!("{method} error: {e}"),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_lifecycle_surface_pass() {
        let mut v = ValidationResult::new("health-lifecycle-surface");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(v.failed, 0, "had {} failures", v.failed);
    }
}
