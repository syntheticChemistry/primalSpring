// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scenario: Sovereignty Parity — routing config schema validation and
//! live membrane boundary health.
//!
//! Structural phase (Tier::Rust):
//!   Parses `config/routing_config_reference.toml` and validates the schema:
//!   routing rules, backend types, trust tiers, telemetry, cache/cost sections.
//!
//! Live phase (Tier::Live):
//!   Calls `capability.route` via biomeOS to verify membrane routing is
//!   reachable, then health-checks Tower primals (security, discovery, defense)
//!   to confirm the membrane boundary is up.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "sovereignty-parity",
        track: Track::Sovereignty,
        tier: Tier::Both,
        provenance_crate: "primalspring_sovereignty",
        provenance_date: "2026-05-15",
        description:
            "Sovereignty parity — routing config schema + live membrane boundary health",
    },
    run,
};

const VALID_BACKEND_TYPES: &[&str] = &[
    "btsp_tunnel",
    "local_filesystem",
    "songbird_p2p",
    "http_proxy",
];

const REQUIRED_TRUST_TIERS: &[&str] = &["covalent", "ionic", "metallic", "weak"];

// ─── Structural: Routing Config Schema ───────────────────────────────────────

fn phase_routing_schema(v: &mut ValidationResult) {
    let routing_toml = include_str!("../../../../config/routing_config_reference.toml");
    let parsed: toml::Value = match toml::from_str(routing_toml) {
        Ok(p) => p,
        Err(e) => {
            v.check_bool(
                "schema:parse",
                false,
                &format!("routing_config_reference.toml parse error: {e}"),
            );
            return;
        }
    };

    v.check_bool(
        "schema:parse",
        true,
        "routing_config_reference.toml parses as valid TOML",
    );

    let Some(routing) = parsed.get("routing") else {
        v.check_bool("schema:routing_section", false, "missing [routing] section");
        return;
    };

    v.check_bool("schema:routing_section", true, "[routing] section present");

    let version = routing
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    v.check_bool(
        "schema:version_present",
        !version.is_empty(),
        &format!("routing.version = \"{version}\""),
    );

    let default_backend = routing
        .get("default_backend")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    v.check_bool(
        "schema:default_backend",
        !default_backend.is_empty(),
        &format!("default_backend = \"{default_backend}\""),
    );

    // Validate backends
    let backends = routing.get("backends").and_then(|b| b.as_table());
    v.check_bool(
        "schema:backends_present",
        backends.is_some(),
        "[routing.backends] section present",
    );

    if let Some(backends) = backends {
        v.check_bool(
            "schema:backend_count",
            backends.len() >= 2,
            &format!("{} backends defined (expect >= 2)", backends.len()),
        );

        for (name, backend) in backends {
            let backend_type = backend
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            v.check_bool(
                &format!("schema:backend:{name}:valid_type"),
                VALID_BACKEND_TYPES.contains(&backend_type),
                &format!(
                    "{name}.type = \"{backend_type}\" (valid: {VALID_BACKEND_TYPES:?})"
                ),
            );

            let has_description = backend
                .get("description")
                .and_then(|v| v.as_str())
                .is_some_and(|s| !s.is_empty());
            v.check_bool(
                &format!("schema:backend:{name}:has_description"),
                has_description,
                &format!("{name} has description"),
            );

            v.check_bool(
                &format!("schema:backend:{name}:auth_required"),
                backend.get("auth_required").is_some(),
                &format!("{name} declares auth_required"),
            );
        }

        v.check_bool(
            "schema:default_backend_exists",
            backends.contains_key(default_backend),
            &format!("default_backend \"{default_backend}\" exists in backends"),
        );
    }

    // Validate routing rules
    let rules = routing
        .get("rules")
        .and_then(|r| r.as_array())
        .cloned()
        .unwrap_or_default();

    v.check_bool(
        "schema:rules_present",
        !rules.is_empty(),
        &format!("{} routing rules defined", rules.len()),
    );

    for rule in &rules {
        let name = rule
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("<unnamed>");

        let has_match = rule.get("match").is_some();
        v.check_bool(
            &format!("schema:rule:{name}:has_match"),
            has_match,
            &format!("rule \"{name}\" has match predicate"),
        );

        let has_backend = rule
            .get("backend")
            .and_then(|v| v.as_str())
            .is_some_and(|s| !s.is_empty());
        v.check_bool(
            &format!("schema:rule:{name}:has_backend"),
            has_backend,
            &format!("rule \"{name}\" has backend"),
        );

        let has_reason = rule
            .get("reason")
            .and_then(|v| v.as_str())
            .is_some_and(|s| !s.is_empty());
        v.check_bool(
            &format!("schema:rule:{name}:has_reason"),
            has_reason,
            &format!("rule \"{name}\" has reason"),
        );
    }

    // Validate trust tiers
    let trust = parsed.get("trust").and_then(|t| t.as_table());
    v.check_bool(
        "schema:trust_section",
        trust.is_some(),
        "[trust] section present",
    );

    if let Some(trust) = trust {
        for tier in REQUIRED_TRUST_TIERS {
            let tier_section = trust.get(*tier);
            v.check_bool(
                &format!("schema:trust:{tier}:present"),
                tier_section.is_some(),
                &format!("trust.{tier} tier defined"),
            );

            if let Some(ts) = tier_section {
                let has_backends = ts
                    .get("allowed_backends")
                    .and_then(|v| v.as_array())
                    .is_some_and(|a| !a.is_empty());
                v.check_bool(
                    &format!("schema:trust:{tier}:allowed_backends"),
                    has_backends,
                    &format!("trust.{tier} has allowed_backends"),
                );

                let has_scope = ts
                    .get("content_scope")
                    .and_then(|v| v.as_str())
                    .is_some_and(|s| !s.is_empty());
                v.check_bool(
                    &format!("schema:trust:{tier}:content_scope"),
                    has_scope,
                    &format!("trust.{tier} has content_scope"),
                );
            }
        }
    }

    // Validate telemetry section
    let telemetry = parsed.get("telemetry").and_then(|t| t.as_table());
    v.check_bool(
        "schema:telemetry_section",
        telemetry.is_some(),
        "[telemetry] section present",
    );

    if let Some(telemetry) = telemetry {
        let shadow_mode = telemetry
            .get("shadow_mode")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        v.check_bool(
            "schema:telemetry:shadow_mode",
            !shadow_mode.is_empty(),
            &format!("shadow_mode = \"{shadow_mode}\""),
        );

        let cutover_days = telemetry
            .get("cutover_gate_days")
            .and_then(toml::Value::as_integer)
            .unwrap_or(0);
        v.check_bool(
            "schema:telemetry:cutover_gate_days",
            cutover_days >= 7,
            &format!("cutover_gate_days = {cutover_days} (expect >= 7)"),
        );
    }

    // Validate cache and cost sections
    let cache = parsed.get("cache");
    v.check_bool(
        "schema:cache_section",
        cache.is_some(),
        "[cache] section present",
    );

    let cost = parsed.get("cost");
    v.check_bool(
        "schema:cost_section",
        cost.is_some(),
        "[cost] section present",
    );
}

// ─── Live: Membrane Boundary Health ──────────────────────────────────────────

fn phase_membrane_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let tower_caps = ["security", "discovery", "defense"];
    for cap in &tower_caps {
        match ctx.health_check(cap) {
            Ok(true) => v.check_bool(
                &format!("live:health:{cap}"),
                true,
                &format!("{cap} health.liveness ok (membrane boundary up)"),
            ),
            Ok(false) => v.check_bool(
                &format!("live:health:{cap}"),
                false,
                &format!("{cap} unhealthy"),
            ),
            Err(e) if e.is_connection_error() => {
                v.check_skip(
                    &format!("live:health:{cap}"),
                    &format!("{cap} not available: {e}"),
                );
            }
            Err(e) => v.check_bool(
                &format!("live:health:{cap}"),
                false,
                &format!("{cap} health error: {e}"),
            ),
        }
    }

    match ctx.health_check("content") {
        Ok(true) => v.check_bool(
            "live:health:nestgate_cache",
            true,
            "NestGate cache health.liveness ok",
        ),
        Ok(false) => v.check_bool(
            "live:health:nestgate_cache",
            false,
            "NestGate cache unhealthy",
        ),
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "live:health:nestgate_cache",
                &format!("NestGate cache not available: {e}"),
            );
        }
        Err(e) => v.check_bool(
            "live:health:nestgate_cache",
            false,
            &format!("NestGate cache health error: {e}"),
        ),
    }

    match ctx.call(
        "orchestration",
        "capability.discover",
        serde_json::json!({}),
    ) {
        Ok(resp) => {
            let capabilities = resp
                .get("capabilities")
                .or_else(|| resp.get("domains"))
                .and_then(|c| c.as_array())
                .map_or(0, Vec::len);
            v.check_bool(
                "live:orchestration:discover",
                capabilities > 0,
                &format!(
                    "capability.discover returned {capabilities} capabilities via biomeOS"
                ),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "live:orchestration:discover",
                &format!("biomeOS not available: {e}"),
            );
        }
        Err(e) => v.check_bool(
            "live:orchestration:discover",
            false,
            &format!("capability.discover error: {e}"),
        ),
    }
}

// ─── Entry point ─────────────────────────────────────────────────────────────

/// Run the sovereignty parity validation: structural schema + live membrane health.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Routing Config Schema Validation (structural)");
    phase_routing_schema(v);

    v.section("Phase 2: Membrane Boundary Health (live)");
    phase_membrane_health(v, ctx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn sovereignty_parity_structural() {
        let mut v = ValidationResult::new("sovereignty-parity");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        // Structural checks must all pass; live checks may skip without primals
        assert!(
            v.failed == 0,
            "Sovereignty parity has {} failures",
            v.failed,
        );
    }

    #[test]
    fn routing_config_has_all_trust_tiers() {
        let toml_str = include_str!("../../../../config/routing_config_reference.toml");
        for tier in REQUIRED_TRUST_TIERS {
            assert!(
                toml_str.contains(&format!("[trust.{tier}]")),
                "missing trust tier: {tier}"
            );
        }
    }

    #[test]
    fn routing_config_backends_use_valid_types() {
        let toml_str = include_str!("../../../../config/routing_config_reference.toml");
        let parsed: toml::Value = toml::from_str(toml_str).expect("valid TOML");
        let backends = parsed
            .get("routing")
            .and_then(|r| r.get("backends"))
            .and_then(|b| b.as_table())
            .expect("backends table");
        for (name, backend) in backends {
            let t = backend
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            assert!(
                VALID_BACKEND_TYPES.contains(&t),
                "backend {name} has invalid type: {t}"
            );
        }
    }
}
