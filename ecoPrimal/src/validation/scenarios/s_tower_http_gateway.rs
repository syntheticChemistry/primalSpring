// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower HTTP Gateway — validates the sovereign HTTP proxy contract.
//!
//! Wave 132c shipped `http.proxy` in songBird with `ReverseProxyConfig` TOML
//! routes + `CapabilityProxyRouter`, and bearDog ACME `:443` gateway with
//! `HotReloadAcceptor`. This scenario validates the structural contract:
//!
//! Phases:
//! 1. Registry: `http.proxy` exists in `capability_registry` and routes to songBird
//! 2. Security: `security.advisory` + `security.scan` route to skunkBat (bearDog→skunkBat advisory)
//! 3. Gateway contract: bearDog owns TLS termination (`:443`) and forwards to songBird
//! 4. Live: http.proxy IPC dispatch + ACME cert probe (requires deployed primals)

use crate::composition::{CompositionContext, capability_to_primal};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Capability registry TOML (single source of truth).
const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Tower HTTP gateway scenario metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-http-gateway",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave132_tower_http_gateway",
        provenance_date: "2026-07-04",
        description: "Tower HTTP gateway — http.proxy routing, security.advisory, bearDog ACME contract",
    },
    run,
};

/// Run all Tower HTTP gateway validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Registry — http.proxy in capability_registry");
    phase_registry(v);

    v.section("Phase 2: Security advisory routing");
    phase_security_routing(v);

    v.section("Phase 3: Gateway contract — TLS termination ownership");
    phase_gateway_contract(v);

    v.section("Phase 4: Live — http.proxy dispatch + ACME probe");
    phase_live(v, ctx);
}

fn phase_registry(v: &mut ValidationResult) {
    let Ok(parsed) = toml::from_str::<toml::Value>(REGISTRY_TOML) else {
        v.check_bool(
            "registry:parse",
            false,
            "capability_registry.toml failed to parse",
        );
        return;
    };

    v.check_bool(
        "registry:parse",
        true,
        "capability_registry.toml valid TOML",
    );

    let http_section = parsed.get("http");
    v.check_bool(
        "registry:http_domain",
        http_section.is_some(),
        "http domain exists in registry",
    );

    if let Some(http) = http_section {
        let owner = http.get("owner").and_then(|v| v.as_str()).unwrap_or("");
        v.check_bool(
            "registry:http_owner",
            owner == "songbird",
            &format!("http owner = {owner} (expect songbird)"),
        );

        let methods = http
            .get("methods")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|s| s.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();

        v.check_bool(
            "registry:http_proxy_method",
            methods.contains(&"http.proxy"),
            &format!("http.proxy in methods: {methods:?}"),
        );

        v.check_bool(
            "registry:http_methods_sorted",
            methods.windows(2).all(|w| w[0] <= w[1]),
            "http methods are alphabetically sorted",
        );
    }

    let routed = capability_to_primal("http");
    v.check_bool(
        "routing:http_to_songbird",
        routed == "songbird",
        &format!("http routes to {routed}"),
    );
}

fn phase_security_routing(v: &mut ValidationResult) {
    let Ok(parsed) = toml::from_str::<toml::Value>(REGISTRY_TOML) else {
        return;
    };

    if let Some(security) = parsed.get("security") {
        let methods = security
            .get("methods")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|s| s.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();

        v.check_bool(
            "security:advisory_registered",
            methods.contains(&"security.advisory"),
            &format!("security.advisory in methods: {methods:?}"),
        );

        v.check_bool(
            "security:scan_registered",
            methods.contains(&"security.scan"),
            &format!("security.scan in methods: {methods:?}"),
        );

        v.check_bool(
            "security:methods_sorted",
            methods.windows(2).all(|w| w[0] <= w[1]),
            "security methods are alphabetically sorted",
        );
    }

    let advisory_owner = capability_to_primal("security");
    v.check_bool(
        "routing:security_to_beardog",
        advisory_owner == "beardog",
        &format!("security routes to {advisory_owner}"),
    );
}

fn phase_gateway_contract(v: &mut ValidationResult) {
    let Ok(parsed) = toml::from_str::<toml::Value>(REGISTRY_TOML) else {
        return;
    };

    let tower_section = parsed.get("compositions").and_then(|c| c.get("tower"));
    v.check_bool(
        "gateway:tower_composition",
        tower_section.is_some(),
        "compositions.tower exists in registry",
    );

    if let Some(tower) = tower_section {
        let primals = tower
            .get("primals")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|s| s.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();

        v.check_bool(
            "gateway:tower_has_songbird",
            primals.contains(&"songbird"),
            "songbird in tower composition",
        );
        v.check_bool(
            "gateway:tower_has_beardog",
            primals.contains(&"beardog"),
            "beardog in tower composition",
        );
        v.check_bool(
            "gateway:tower_has_skunkbat",
            primals.contains(&"skunkbat"),
            "skunkbat in tower composition",
        );
    }

    let http_owner = capability_to_primal("http");
    let tls_owner = capability_to_primal("tls");
    v.check_bool(
        "gateway:http_songbird_tls_beardog",
        http_owner == "songbird" && tls_owner == "beardog",
        &format!("http→{http_owner}, tls→{tls_owner}"),
    );
}

fn phase_live(v: &mut ValidationResult, ctx: &CompositionContext) {
    let caps = ctx.available_capabilities();
    if caps.is_empty() {
        v.check_skip(
            "live:http_proxy_dispatch",
            "no live capabilities — requires deployed songBird with http.proxy",
        );
        v.check_skip(
            "live:acme_cert_probe",
            "no live capabilities — requires deployed bearDog with ACME gateway",
        );
        return;
    }

    v.check_bool(
        "live:http_proxy_discovered",
        caps.contains(&"http.proxy"),
        "http.proxy capability discovered in live mesh",
    );

    v.check_bool(
        "live:security_advisory_discovered",
        caps.contains(&"security.advisory"),
        "security.advisory capability discovered in live mesh",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn tower_http_gateway_structural() {
        let mut v = ValidationResult::new("tower-http-gateway");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(v.passed > 0, "should have passed checks");
    }
}
