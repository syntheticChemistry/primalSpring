// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Footprint API Proxy — validates footprint/web API proxy routing
//! through the drawbridge HTTP proxy surface.

use crate::composition::{CompositionContext, capability_to_primal};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const PORTS_TOML: &str = include_str!("../../../../config/ports.toml");

/// Footprint API proxy scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "fp-api-proxy",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "wave138a_fp_api_proxy",
        provenance_date: "2026-07-14",
        description: "Footprint API proxy — HTTP proxy routing for external web API access",
    },
    run,
};

/// Run footprint API proxy validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Proxy method surface");

    let has_fp = REGISTRY_TOML.contains("footprint.")
        || REGISTRY_TOML.contains("fp.")
        || REGISTRY_TOML.contains("http.proxy");
    v.check_bool(
        "proxy:method_surface",
        has_fp,
        "footprint.*/fp.* methods or http.proxy registered for API proxy",
    );

    let proxy_methods = [
        "http.proxy",
        "http.get",
        "http.request",
        "network.http_request",
    ];
    for method in proxy_methods {
        v.check_bool(
            &format!("proxy:{}", method.replace('.', "_")),
            REGISTRY_TOML.contains(method),
            &format!("{method} registered for structural web proxy"),
        );
    }

    v.section("Phase 2: Drawbridge proxy ownership");

    let http_owner = capability_to_primal("http");
    v.check_bool(
        "proxy:songbird_owner",
        http_owner == "songbird",
        &format!("http proxy owner: {http_owner} (songBird drawbridge)"),
    );

    v.check_bool(
        "proxy:drawbridge_port",
        PORTS_TOML.contains("[gateway.drawbridge]") && PORTS_TOML.contains("port = 7780"),
        "drawbridge port 7780 registered for external API proxy crossing",
    );

    v.section("Phase 3: Capability call routing chain");

    v.check_bool(
        "proxy:capability_call",
        REGISTRY_TOML.contains("capability.call"),
        "capability.call registered (proxy routes through capability dispatch)",
    );
    v.check_bool(
        "proxy:mesh_relay",
        REGISTRY_TOML.contains("mesh.relay"),
        "mesh.relay registered for cross-gate proxy backend routing",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_passes_structural() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "scenario '{}' had {} failures",
            SCENARIO.meta.id, v.failed
        );
    }
}
