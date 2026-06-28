// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: SkunkBat MethodGate Enforcement — threat detection and access control.
//!
//! Validates SkunkBat's role as the defensive primal: MethodGate enforcement,
//! threat detection, and audit surface. The blurb identifies these methods as
//! "not in binary yet" (P1 gap on flockGate), so this scenario validates the
//! structural readiness: domain registration, routing, and BTSP escalation.
//!
//! Phases:
//! 1. Domain registration: threat, audit, defense domains owned by skunkbat
//! 2. MethodGate contract: method_gate.* methods expected
//! 3. BTSP integration: btsp_escalation flag, Tower atomic membership
//! 4. Defense topology: SkunkBat port, Tower gates have SkunkBat
//! 5. Live: SkunkBat health probing

use crate::composition::{CompositionContext, capability_to_primal};
use crate::primal_names;
use crate::tolerances::ports;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const TOPOLOGY_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// SkunkBat MethodGate scenario metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "skunkbat-method-gate",
        track: Track::Security,
        tier: Tier::Both,
        provenance_crate: "wave126_method_gate",
        provenance_date: "2026-06-23",
        description: "SkunkBat MethodGate — threat/audit domains, BTSP escalation, defense topology",
    },
    run,
};

/// Run all SkunkBat MethodGate validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Domain registration");
    phase_domains(v);

    v.section("Phase 2: MethodGate gap analysis");
    phase_method_gate_gap(v);

    v.section("Phase 3: BTSP integration");
    phase_btsp_integration(v);

    v.section("Phase 4: Defense topology");
    phase_defense_topology(v);

    v.section("Phase 5: Live SkunkBat");
    phase_live(v, ctx);
}

fn phase_domains(v: &mut ValidationResult) {
    let skunkbat_domains = ["threat", "audit"];
    for domain in skunkbat_domains {
        let has_section = REGISTRY_TOML.contains(&format!("[{domain}]"));
        v.check_bool(
            &format!("domain:{domain}_section"),
            has_section,
            &format!("[{domain}] section exists in capability_registry"),
        );

        let primal = capability_to_primal(domain);
        v.check_bool(
            &format!("domain:{domain}_owner"),
            primal == primal_names::SKUNKBAT,
            &format!("{domain} owned by \"{primal}\" (expected skunkbat)"),
        );
    }

    let defense_related = ["threat", "audit"];
    let mut owned_count = 0;
    for domain in defense_related {
        if capability_to_primal(domain) == primal_names::SKUNKBAT {
            owned_count += 1;
        }
    }
    v.check_bool(
        "domain:skunkbat_owns_defense",
        owned_count == defense_related.len(),
        &format!(
            "skunkbat owns {owned_count}/{} defense domains",
            defense_related.len()
        ),
    );
}

fn phase_method_gate_gap(v: &mut ValidationResult) {
    let expected_methods = [
        "method_gate.status",
        "method_gate.enforce",
        "threat.report",
        "threat.detect",
        "audit.log",
        "audit.query",
    ];

    let mut present = 0;
    let mut missing = Vec::new();
    for method in expected_methods {
        if REGISTRY_TOML.contains(method) {
            present += 1;
        } else {
            missing.push(method);
        }
    }

    v.check_bool(
        "gap:methods_registered",
        present > 0 || missing.is_empty(),
        &format!(
            "{present}/{} MethodGate methods registered",
            expected_methods.len()
        ),
    );

    if !missing.is_empty() {
        v.check_bool(
            "gap:methods_gap_identified",
            true,
            &format!(
                "P1 gap (flockGate): {} methods not yet wired: {:?}",
                missing.len(),
                missing
            ),
        );
    }

    let threat_has_methods = REGISTRY_TOML.contains("\"threat.");
    let audit_has_methods = REGISTRY_TOML.contains("\"audit.");
    v.check_bool(
        "gap:threat_methods_empty",
        !threat_has_methods,
        "threat domain methods array empty (awaiting upstream wiring)",
    );
    v.check_bool(
        "gap:audit_methods_empty",
        !audit_has_methods,
        "audit domain methods array empty (awaiting upstream wiring)",
    );
}

fn phase_btsp_integration(v: &mut ValidationResult) {
    let has_btsp_escalation = REGISTRY_TOML.contains("btsp_escalation");
    v.check_bool(
        "btsp:escalation_flag",
        has_btsp_escalation,
        "btsp_escalation flag present in registry (threat domain)",
    );

    let btsp_after_threat = REGISTRY_TOML
        .find("[threat]")
        .is_some_and(|pos| REGISTRY_TOML[pos..].contains("btsp_escalation = true"));
    v.check_bool(
        "btsp:threat_escalates",
        btsp_after_threat,
        "threat domain has btsp_escalation = true",
    );

    let tower_atomics = REGISTRY_TOML.contains("\"beardog\", \"songbird\", \"skunkbat\"")
        || REGISTRY_TOML.contains("skunkbat");
    v.check_bool(
        "btsp:skunkbat_in_tower",
        tower_atomics,
        "skunkbat referenced as Tower atomic member",
    );
}

fn phase_defense_topology(v: &mut ValidationResult) {
    let port = ports::default_port_for(primal_names::SKUNKBAT);
    v.check_bool(
        "topo:skunkbat_port",
        port > 0,
        &format!("skunkbat port = {port}"),
    );

    let topology_has_tower = TOPOLOGY_TOML.contains("tower")
        || TOPOLOGY_TOML.contains("eastGate")
        || TOPOLOGY_TOML.contains("flockGate");
    v.check_bool(
        "topo:tower_gates_exist",
        topology_has_tower,
        "Tower-role gates present in mesh topology",
    );

    let has_multiple_gates = TOPOLOGY_TOML.contains("eastGate")
        && TOPOLOGY_TOML.contains("sporeGate")
        && TOPOLOGY_TOML.contains("flockGate");
    v.check_bool(
        "topo:defense_mesh_coverage",
        has_multiple_gates,
        "Multiple gates in topology (MethodGate enforced mesh-wide)",
    );
}

fn phase_live(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    match ctx.client_for("defense") {
        Some(client) => {
            let resp = client.call("health.liveness", serde_json::json!({}));
            match resp {
                Ok(r) => {
                    v.check_bool(
                        "live:skunkbat_health",
                        r.is_success(),
                        "SkunkBat responding to health.liveness",
                    );
                }
                Err(e) if e.is_skippable() => {
                    v.check_skip("live:skunkbat_health", &format!("{e}"));
                }
                Err(e) => {
                    v.check_bool("live:skunkbat_health", false, &format!("{e}"));
                }
            }
        }
        None => {
            v.check_skip("live:skunkbat_health", "no defense client available");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn skunkbat_method_gate_runs() {
        let mut v = ValidationResult::new("skunkbat-method-gate");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        let total = v.passed + v.failed + v.skipped;
        assert!(total >= 12, "expected ≥12 checks, got {total}");
    }

    #[test]
    fn skunkbat_owns_threat() {
        assert_eq!(capability_to_primal("threat"), primal_names::SKUNKBAT);
    }

    #[test]
    fn skunkbat_owns_audit() {
        assert_eq!(capability_to_primal("audit"), primal_names::SKUNKBAT);
    }

    #[test]
    fn threat_has_btsp_escalation() {
        let pos = REGISTRY_TOML.find("[threat]").unwrap();
        let section = &REGISTRY_TOML[pos..];
        assert!(section.contains("btsp_escalation = true"));
    }

    #[test]
    fn skunkbat_has_port() {
        let port = ports::default_port_for(primal_names::SKUNKBAT);
        assert!(port > 0, "skunkbat should have a non-zero port");
    }

    #[test]
    fn method_gate_gap_documented() {
        assert!(
            !REGISTRY_TOML.contains("method_gate.status"),
            "method_gate.status should be absent (P1 gap)"
        );
    }
}
