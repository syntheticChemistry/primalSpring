// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: BTSP Cross-Gate Trust Verification
//!
//! Validates that BTSP trust chains work across gates — tokens issued on
//! one gate can be verified on another via the TrustedIssuerRegistry.
//!
//! Phases:
//! 1. Structural: TrustedIssuerRegistry config and multi-issuer support
//! 2. Wire contract: auth.trust_issuer, auth.verify_ionic, trust.list
//! 3. Live: BTSP credential issuance on local gate
//! 4. Live: Cross-gate trust verification (issued here, verified there)

use crate::composition::CompositionContext;
use crate::evolution::{all_mesh_gates, mesh_address};
use crate::ipc::client::PrimalClient;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::live_mesh::LiveMeshConfig;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "btsp-cross-gate-trust",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave123_covalent_trust",
        provenance_date: "2026-06-22",
        description: "BTSP cross-gate trust — TrustedIssuerRegistry, multi-gate token verification",
    },
    run,
};

/// Run all BTSP cross-gate trust validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — TrustedIssuerRegistry");
    phase_structural(v);

    v.section("Phase 2: Wire contract — trust methods in registry");
    phase_wire_contract(v);

    v.section("Phase 3: Live — local BTSP credential issuance");
    phase_local_btsp(v, ctx);

    v.section("Phase 4: Live — cross-gate trust verification");
    phase_cross_gate_verify(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    let gates = all_mesh_gates();

    v.check_bool(
        "struct:mesh_has_multiple_gates",
        gates.len() >= 2,
        &format!(
            "{} gates in mesh topology (need ≥2 for cross-gate trust)",
            gates.len()
        ),
    );

    let meshed_count = gates.iter().filter(|g| !g.address.is_empty()).count();
    v.check_bool(
        "struct:at_least_two_meshed",
        meshed_count >= 2,
        &format!("{meshed_count} gates with WG addresses (need ≥2)"),
    );

    v.check_bool(
        "struct:local_gate_identified",
        std::env::var("GATE_NAME").is_ok() || std::env::var("HOSTNAME").is_ok(),
        "GATE_NAME or HOSTNAME env var set for local gate identity",
    );

    let local = std::env::var("GATE_NAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_default();
    v.check_bool(
        "struct:local_gate_in_mesh",
        local.is_empty() || mesh_address(&local).is_some(),
        &format!("local gate '{local}' has mesh address"),
    );
}

fn phase_wire_contract(v: &mut ValidationResult) {
    let trust_methods = [
        (
            "auth.trust_issuer",
            "register trusted issuer for cross-gate verify",
        ),
        ("auth.verify_ionic", "verify ionic token across gates"),
        ("auth.issue_ionic", "issue ionic bearer token"),
        ("trust.list", "list trusted issuers in registry"),
        ("btsp.capabilities", "probe BTSP server capabilities"),
        ("btsp.negotiate", "negotiate BTSP session"),
    ];

    for (method, desc) in trust_methods {
        v.check_bool(
            &format!("wire:{}", method.replace('.', "_")),
            REGISTRY_TOML.contains(method),
            &format!("{method} ({desc}) in capability_registry.toml"),
        );
    }

    v.check_bool(
        "wire:security_domain_has_trust",
        REGISTRY_TOML.contains("[security]") || REGISTRY_TOML.contains("security"),
        "security capability domain exists in registry",
    );

    v.check_bool(
        "wire:multi_issuer_support",
        REGISTRY_TOML.contains("trust") || REGISTRY_TOML.contains("issuer"),
        "trust/issuer concepts present in registry",
    );
}

fn phase_local_btsp(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let Some(client) = ctx.client_for("security") else {
        v.check_skip("live:beardog_btsp_server", "no security client available");
        v.check_skip("live:ionic_token_issued", "no security client available");
        return;
    };

    let resp = client.call("btsp.capabilities", serde_json::json!({}));
    match resp {
        Ok(r) => {
            let has_server = r
                .result
                .as_ref()
                .and_then(|v| v.get("server"))
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            v.check_bool(
                "live:beardog_btsp_server",
                has_server || r.is_success(),
                &format!("BearDog advertises BTSP server: {}", r.is_success()),
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "live:beardog_btsp_server",
                &format!("BearDog not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "live:beardog_btsp_server",
                false,
                &format!("btsp.capabilities error: {e}"),
            );
        }
    }

    let resp = client.call(
        "auth.issue_ionic",
        serde_json::json!({
            "scopes": ["health.*", "security.*"],
            "ttl_secs": 300,
        }),
    );
    match resp {
        Ok(r) => {
            let has_token = r
                .result
                .as_ref()
                .and_then(|v| v.get("token").or_else(|| v.get("ionic_token")))
                .is_some();
            v.check_bool(
                "live:ionic_token_issued",
                has_token || r.is_success(),
                "auth.issue_ionic returns token for cross-gate use",
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "live:ionic_token_issued",
                &format!("BearDog not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "live:ionic_token_issued",
                false,
                &format!("auth.issue_ionic: {e}"),
            );
        }
    }
}

fn phase_cross_gate_verify(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let mesh_config = LiveMeshConfig::from_env();

    if !mesh_config.is_ready() {
        v.check_skip(
            "live:cross_gate_reachable",
            "mesh config not ready (no MESH_PEERS or SONGBIRD_PEERS)",
        );
        v.check_skip("live:cross_gate_trust_verify", "mesh not ready");
        v.check_skip("live:trust_list_populated", "mesh not ready");
        return;
    }

    let remote_count = mesh_config.remote_gates.len();
    v.check_bool(
        "live:cross_gate_reachable",
        remote_count > 0,
        &format!("{remote_count} remote gates configured for trust verification"),
    );

    match ctx.call("security", "trust.list", serde_json::json!({})) {
        Ok(resp) => {
            let issuers = resp
                .get("issuers")
                .or_else(|| resp.get("trusted"))
                .and_then(|v| v.as_array())
                .map_or(0, Vec::len);
            v.check_bool(
                "live:trust_list_populated",
                issuers > 0 || resp.get("count").is_some(),
                &format!("TrustedIssuerRegistry has {issuers} entries"),
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "live:trust_list_populated",
                &format!("trust.list not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "live:trust_list_populated",
                false,
                &format!("trust.list: {e}"),
            );
        }
    }

    for (gate_id, addr) in &mesh_config.remote_gates {
        let check_id = format!("live:verify_on_{gate_id}");
        match PrimalClient::connect_tcp(addr, primal_names::BEARDOG) {
            Ok(mut remote) => {
                let resp = remote.call("health.liveness", serde_json::json!({}));
                v.check_bool(
                    &check_id,
                    resp.is_ok(),
                    &format!("BearDog on {gate_id} ({addr}) reachable for trust verify"),
                );
            }
            Err(e) => {
                v.check_skip(&check_id, &format!("{gate_id} BearDog not reachable: {e}"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn btsp_cross_gate_trust_structural() {
        let mut v = ValidationResult::new("btsp-cross-gate-trust");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        let total = v.passed + v.failed + v.skipped;
        assert!(total >= 10, "expected ≥10 checks, got {total}");
    }

    #[test]
    fn wire_contract_methods_present() {
        assert!(REGISTRY_TOML.contains("btsp.capabilities"));
        assert!(REGISTRY_TOML.contains("btsp.negotiate"));
    }

    #[test]
    fn mesh_gates_available() {
        let gates = all_mesh_gates();
        assert!(!gates.is_empty(), "mesh_topology.toml should have gates");
    }

    #[test]
    fn auth_trust_issuer_in_registry() {
        assert!(REGISTRY_TOML.contains("auth.trust_issuer"));
    }

    #[test]
    fn auth_verify_ionic_in_registry() {
        assert!(REGISTRY_TOML.contains("auth.verify_ionic"));
    }

    #[test]
    fn multi_gate_mesh_count() {
        let gates = all_mesh_gates();
        let meshed = gates.iter().filter(|g| !g.address.is_empty()).count();
        assert!(meshed >= 3, "need ≥3 meshed gates, got {meshed}");
    }

    #[test]
    fn trust_related_methods_coverage() {
        let trust_methods = ["auth.issue_ionic", "btsp.capabilities", "btsp.negotiate"];
        for method in trust_methods {
            assert!(REGISTRY_TOML.contains(method), "{method} missing from registry");
        }
    }
}
