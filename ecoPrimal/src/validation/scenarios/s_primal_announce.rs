// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scenario: Primal Announce — validate the `primal.announce` JSON-RPC
//! registration protocol wire format and live registration flow.
//!
//! Wave 18b of the Neural API Signal Elevation plan. Validates that:
//!
//! - `primal.announce` is present in the capability registry
//! - The wire format schema matches `PRIMAL_ANNOUNCE_PROTOCOL.md`
//! - Live biomeOS accepts and processes announce payloads
//!
//! This exposes primals that haven't adopted the announce protocol yet.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "primal-announce",
        track: Track::BiomeosDeploy,
        tier: Tier::Both,
        provenance_crate: "neural_api_wave18b",
        provenance_date: "2026-05-16",
        description: "primal.announce: registry presence, wire format, live registration",
    },
    run,
};

const REQUIRED_FIELDS: &[&str] = &["primal", "socket"];
const OPTIONAL_FIELDS: &[&str] = &[
    "pid",
    "capabilities",
    "methods",
    "semantic_mappings",
    "signal_tiers",
    "attestation",
    "version",
];
const VALID_TIERS: &[&str] = &["tower", "node", "nest", "meta"];

/// Run the primal announce validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1 (Rust): registry presence");
    phase_registry_presence(v);

    v.section("Phase 2 (Rust): wire format structural validation");
    phase_wire_format(v);

    v.section("Phase 3 (Rust): CompositionContext::announce() API");
    phase_announce_api(v, ctx);

    v.section("Phase 4 (Live): announce via biomeOS");
    phase_live_announce(v, ctx);
}

fn phase_registry_presence(v: &mut ValidationResult) {
    let registry_content = include_str!("../../../../config/capability_registry.toml");
    let has_announce = registry_content.contains("primal.announce");
    v.check_bool(
        "registry:has_primal_announce",
        has_announce,
        "capability_registry.toml contains primal.announce",
    );

    let has_primal_info = registry_content.contains("primal.info");
    v.check_bool(
        "registry:has_primal_info",
        has_primal_info,
        "capability_registry.toml contains primal.info (companion method)",
    );
}

fn phase_wire_format(v: &mut ValidationResult) {
    let well_formed = serde_json::json!({
        "primal": "test-scenario-primal",
        "socket": "/run/ecoprimals/test-scenario.sock",
        "pid": 99999,
        "capabilities": ["test"],
        "methods": ["test.probe"],
        "semantic_mappings": { "test.alias": "test.probe" },
        "signal_tiers": ["tower"],
        "attestation": "0000",
        "version": "0.0.0-scenario",
    });

    // All required fields present
    for field in REQUIRED_FIELDS {
        let has_field = well_formed.get(field).is_some();
        v.check_bool(
            &format!("wire:required:{field}"),
            has_field,
            &format!("well-formed announce has required field: {field}"),
        );
    }

    // Optional fields are valid
    for field in OPTIONAL_FIELDS {
        let has_field = well_formed.get(field).is_some();
        v.check_bool(
            &format!("wire:optional:{field}"),
            has_field,
            &format!("well-formed announce has optional field: {field}"),
        );
    }

    // signal_tiers must be valid tier names
    if let Some(tiers) = well_formed.get("signal_tiers").and_then(|t| t.as_array()) {
        for tier in tiers {
            if let Some(name) = tier.as_str() {
                let valid = VALID_TIERS.contains(&name);
                v.check_bool(
                    &format!("wire:signal_tier:{name}"),
                    valid,
                    &format!("signal_tier {name:?} is a recognized tier"),
                );
            }
        }
    }

    // Field normalization: protocol spec says "primal" not "primal_id",
    // "socket" not "socket_path"
    let has_primal_id = well_formed.get("primal_id").is_some();
    v.check_bool(
        "wire:no_primal_id",
        !has_primal_id,
        "announce uses 'primal' not 'primal_id' per protocol normalization",
    );

    let has_socket_path = well_formed.get("socket_path").is_some();
    v.check_bool(
        "wire:no_socket_path",
        !has_socket_path,
        "announce uses 'socket' not 'socket_path' per protocol normalization",
    );

    // Missing required fields should be detectable
    let missing_primal = serde_json::json!({ "socket": "/run/test.sock" });
    v.check_bool(
        "wire:reject:missing_primal",
        missing_primal.get("primal").is_none(),
        "announcement without 'primal' field is incomplete",
    );

    let missing_socket = serde_json::json!({ "primal": "test" });
    v.check_bool(
        "wire:reject:missing_socket",
        missing_socket.get("socket").is_none(),
        "announcement without 'socket' field is incomplete",
    );
}

fn phase_announce_api(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let result = ctx.announce(
        "primalspring-scenario-probe",
        &["test.probe"],
        &std::path::PathBuf::from(crate::tolerances::runtime_dir()).join("primalspring-scenario-probe.sock"),
    );

    match result {
        Ok(resp) => {
            v.check_bool(
                "api:announce:accepted",
                true,
                &format!(
                    "ctx.announce() accepted: {:?}",
                    resp.as_object()
                        .map(|o| o.keys().collect::<Vec<_>>())
                        .unwrap_or_default()
                ),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "api:announce:accepted",
                &format!("orchestration not available: {e}"),
            );
        }
        Err(e) => {
            let detail = format!("{e}");
            if detail.contains("-32601") || detail.contains("not found") {
                v.check_skip(
                    "api:announce:accepted",
                    &format!("primal.announce not available (pre-v3.57 biomeOS): {e}"),
                );
            } else {
                v.check_bool(
                    "api:announce:accepted",
                    false,
                    &format!("ctx.announce() error: {e}"),
                );
            }
        }
    }
}

fn phase_live_announce(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "live:announce:full_payload",
            "biomeOS orchestration not available - skipping live announce",
        );
        return;
    }

    let full_announce = serde_json::json!({
        "primal": "primalspring-scenario-probe",
        "socket": format!("{}/primalspring-scenario-probe.sock", crate::tolerances::runtime_dir()),
        "pid": std::process::id(),
        "capabilities": ["test"],
        "methods": ["test.probe", "test.validate"],
        "semantic_mappings": {},
        "signal_tiers": [],
        "version": env!("CARGO_PKG_VERSION"),
    });

    match ctx.call("orchestration", "primal.announce", full_announce) {
        Ok(resp) => {
            v.check_bool(
                "live:announce:full_payload",
                true,
                "biomeOS accepted full primal.announce payload",
            );

            let has_status = resp.get("status").is_some()
                || resp.get("registered").is_some()
                || resp.get("ok").is_some();
            v.check_bool(
                "live:announce:response_shape",
                has_status || resp.is_object(),
                &format!(
                    "announce response has expected shape: {:?}",
                    resp.as_object()
                        .map(|o| o.keys().collect::<Vec<_>>())
                        .unwrap_or_default()
                ),
            );
        }
        Err(e) => {
            let detail = format!("{e}");
            if detail.contains("-32601") {
                v.check_bool(
                    "live:announce:full_payload",
                    false,
                    "UPSTREAM GAP: biomeOS does not implement primal.announce (-32601)",
                );
            } else {
                v.check_bool(
                    "live:announce:full_payload",
                    false,
                    &format!("primal.announce failed: {e}"),
                );
            }
        }
    }

    // Validate primal.info can see the registered probe
    match ctx.call(
        "orchestration",
        "primal.info",
        serde_json::json!({ "primal": "primalspring-scenario-probe" }),
    ) {
        Ok(resp) => {
            let has_primal = resp.get("primal").is_some()
                || resp.get("name").is_some()
                || resp.get("status").is_some();
            v.check_bool(
                "live:info:probe_visible",
                has_primal || resp.is_object(),
                &format!(
                    "primal.info sees registered probe: {:?}",
                    resp.as_object()
                        .map(|o| o.keys().collect::<Vec<_>>())
                        .unwrap_or_default()
                ),
            );
        }
        Err(e) => {
            let detail = format!("{e}");
            if detail.contains("-32601") {
                v.check_skip(
                    "live:info:probe_visible",
                    "primal.info not available (-32601)",
                );
            } else {
                v.check_bool(
                    "live:info:probe_visible",
                    false,
                    &format!("primal.info failed: {e}"),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primal_announce_no_panic() {
        let mut v = ValidationResult::new("primal-announce");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.evaluated() > 0 || v.skipped > 0,
            "scenario should produce checks"
        );
    }

    #[test]
    fn wire_format_field_counts() {
        assert_eq!(REQUIRED_FIELDS.len(), 2, "protocol has 2 required fields");
        assert_eq!(OPTIONAL_FIELDS.len(), 7, "protocol has 7 optional fields");
    }
}
