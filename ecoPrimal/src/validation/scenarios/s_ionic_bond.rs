// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: Ionic Bond — absorbed from exp031, enriched Wave 37 (WS-1).

use crate::bonding::ionic::{
    AttributionTerms, DataReturnPolicy, IonicProposal, TerminationReason, TerminationRequest,
};
use crate::bonding::ionic_runtime::IonicContractRegistry;
use crate::bonding::{BondType, BondingConstraint, TrustModel};
use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "ionic-bond",
        track: Track::Bonding,
        tier: Tier::Both,
        provenance_crate: "exp031_ionic_bond",
        provenance_date: "2026-05-21",
        description: "Ionic bond — full contract lifecycle (propose/accept/meter/terminate/seal)",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Bond Type Properties");
    phase_bond_type_properties(v);

    v.section("Phase 2: Contract Lifecycle (WS-1 protocol)");
    phase_contract_lifecycle(v);

    v.section("Phase 3: Policy Enforcement");
    phase_policy_enforcement(v);

    v.section("Phase 4: Live RPC Lifecycle (bonding.* + crypto.ionic_bond.*)");
    phase_live_discovery(v, ctx);
}

fn sample_proposal() -> IonicProposal {
    IonicProposal {
        proposer_identity: "did:key:flockgate-west".into(),
        requested_capabilities: vec![
            "compute.submit".into(),
            "compute.status".into(),
            "storage.retrieve".into(),
        ],
        duration_secs: 3600,
        trust_model: TrustModel::Contractual,
        attribution: AttributionTerms::default(),
        data_return_policy: DataReturnPolicy::DeleteOnTermination,
        rate_limit_rps: 50,
    }
}

fn compute_constraints() -> BondingConstraint {
    BondingConstraint {
        capability_allow: vec!["compute.*".into()],
        capability_deny: vec!["compute.admin.*".into()],
        bandwidth_limit_mbps: 100,
        max_concurrent_requests: 10,
    }
}

fn phase_bond_type_properties(v: &mut ValidationResult) {
    let bond = BondType::Ionic;
    v.check_bool(
        "ionic_description_non_empty",
        !bond.description().is_empty(),
        &format!("BondType::Ionic.description() — {}", bond.description()),
    );
    v.check_bool("ionic_is_metered", bond.is_metered(), "Ionic bonds are metered");
    v.check_bool(
        "ionic_no_shared_electrons",
        !bond.shares_electrons(),
        "Ionic bonds do not share Tower state",
    );
}

fn phase_contract_lifecycle(v: &mut ValidationResult) {
    let mut reg = IonicContractRegistry::new();

    let id = match reg.propose(sample_proposal()) {
        Ok(id) => {
            v.check_bool("propose_succeeds", true, &format!("Contract proposed: {id}"));
            id
        }
        Err(e) => {
            v.check_bool("propose_succeeds", false, &format!("Propose failed: {e}"));
            return;
        }
    };

    let resp = reg.accept(&id, compute_constraints());
    let accepted = resp.as_ref().is_ok_and(|r| r.accepted);
    v.check_bool(
        "accept_transitions_to_active",
        accepted,
        "Proposed → Active on accept",
    );

    let call_ok = reg.record_call(&id, "compute.submit", 2048);
    v.check_bool("metered_call_succeeds", call_ok.is_ok(), "Metered call within scope");

    match reg.get(&id) {
        Some(contract) => {
            let usage = &contract.usage;
            v.check_bool(
                "usage_metrics_increment",
                usage.total_calls == 1 && usage.total_bytes == 2048,
                &format!("Usage: {} calls, {} bytes", usage.total_calls, usage.total_bytes),
            );
        }
        None => {
            v.check_bool("usage_metrics_increment", false, "Contract not found after accept");
        }
    }

    let seal = reg.terminate(&TerminationRequest {
        contract_id: id.clone(),
        reason: TerminationReason::Complete,
    });
    v.check_bool(
        "terminate_produces_seal",
        seal.is_ok(),
        "Active → Sealed with provenance seal",
    );

    if let Ok(ref s) = seal {
        v.check_bool(
            "seal_has_merkle_root",
            !s.merkle_root.is_empty(),
            &format!("Seal merkle_root: {}", s.merkle_root),
        );
        v.check_bool(
            "seal_has_braid_id",
            !s.braid_id.is_empty(),
            &format!("Seal braid_id: {}", s.braid_id),
        );
    }
}

fn phase_policy_enforcement(v: &mut ValidationResult) {
    let mut reg = IonicContractRegistry::new();
    let id = match reg.propose(sample_proposal()) {
        Ok(id) => id,
        Err(e) => {
            v.check_bool("policy_propose", false, &format!("Propose failed: {e}"));
            return;
        }
    };
    if reg.accept(&id, compute_constraints()).is_err() {
        v.check_bool("policy_accept", false, "Accept failed");
        return;
    }

    let denied = reg.record_call(&id, "storage.store", 512);
    v.check_bool(
        "out_of_scope_denied",
        denied.is_err(),
        "Capability outside negotiated scope is denied",
    );

    let bad_proposal = IonicProposal {
        proposer_identity: String::new(),
        requested_capabilities: vec![],
        duration_secs: 0,
        trust_model: TrustModel::ZeroTrust,
        attribution: AttributionTerms::default(),
        data_return_policy: DataReturnPolicy::DeleteOnTermination,
        rate_limit_rps: 0,
    };
    let invalid = reg.propose(bad_proposal);
    v.check_bool(
        "invalid_proposal_rejected",
        invalid.is_err(),
        "Proposal with empty identity and ZeroTrust is rejected",
    );
}

#[expect(clippy::too_many_lines, reason = "live RPC lifecycle phases are sequential")]
fn phase_live_discovery(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "live:propose",
            "orchestration capability not available — cannot test live bonding RPC",
        );
        return;
    }

    let proposal = sample_proposal();
    let propose_params = serde_json::json!({
        "proposer_identity": proposal.proposer_identity,
        "requested_capabilities": proposal.requested_capabilities,
        "duration_secs": proposal.duration_secs,
        "trust_model": "Contractual",
        "attribution": {},
        "data_return_policy": "DeleteOnTermination",
        "rate_limit_rps": proposal.rate_limit_rps,
    });

    let contract_id = match ctx.call("orchestration", "bonding.propose", propose_params) {
        Ok(resp) => {
            let has_id = resp.get("contract_id").is_some();
            v.check_bool(
                "live:propose",
                has_id,
                &format!("bonding.propose → {resp}"),
            );
            resp.get("contract_id")
                .and_then(|c| c.as_str())
                .map(String::from)
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("live:propose", &format!("orchestration not reachable: {e}"));
            return;
        }
        Err(e) => {
            v.check_bool("live:propose", false, &format!("bonding.propose error: {e}"));
            return;
        }
    };

    let Some(contract_id) = contract_id else {
        v.check_skip("live:accept", "no contract_id from propose — skipping accept");
        return;
    };

    let accept_params = serde_json::json!({
        "contract_id": contract_id,
        "constraints": {
            "capability_allow": ["compute.*"],
            "capability_deny": ["compute.admin.*"],
            "bandwidth_limit_mbps": 100,
            "max_concurrent_requests": 10,
        },
    });

    match ctx.call("orchestration", "bonding.accept", accept_params) {
        Ok(resp) => {
            let accepted = resp
                .get("accepted")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            v.check_bool(
                "live:accept",
                accepted,
                &format!("bonding.accept → accepted={accepted}"),
            );
        }
        Err(e) => {
            v.check_bool("live:accept", false, &format!("bonding.accept error: {e}"));
            return;
        }
    }

    match ctx.call(
        "orchestration",
        "bonding.status",
        serde_json::json!({ "contract_id": contract_id }),
    ) {
        Ok(resp) => {
            let state = resp.get("state").and_then(|s| s.as_str()).unwrap_or("unknown");
            v.check_bool(
                "live:status",
                state == "Active" || state == "active",
                &format!("bonding.status → state={state}"),
            );
        }
        Err(e) => {
            v.check_bool("live:status", false, &format!("bonding.status error: {e}"));
        }
    }

    match ctx.call(
        "orchestration",
        "bonding.modify_scope",
        serde_json::json!({
            "contract_id": contract_id,
            "add_capabilities": ["storage.retrieve"],
        }),
    ) {
        Ok(resp) => {
            v.check_bool(
                "live:modify_scope",
                resp.is_object(),
                &format!("bonding.modify_scope → {resp}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "live:modify_scope",
                false,
                &format!("bonding.modify_scope error: {e}"),
            );
        }
    }

    match ctx.call(
        "orchestration",
        "bonding.terminate",
        serde_json::json!({ "contract_id": contract_id, "reason": "complete" }),
    ) {
        Ok(resp) => {
            let has_seal = resp.get("merkle_root").is_some() || resp.get("sealed_at").is_some();
            v.check_bool(
                "live:terminate",
                has_seal,
                &format!("bonding.terminate → seal={resp}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "live:terminate",
                false,
                &format!("bonding.terminate error: {e}"),
            );
        }
    }

    if ctx.has_capability("security") {
        match ctx.call(
            "security",
            "crypto.ionic_bond.verify_proposal",
            serde_json::json!({
                "signed_payload": "test-payload",
                "signature": "test-sig",
                "proposer_public_key": "ed25519-test-key",
            }),
        ) {
            Ok(resp) => {
                v.check_bool(
                    "live:crypto_verify",
                    resp.is_object(),
                    &format!("crypto.ionic_bond.verify_proposal responded: {resp}"),
                );
            }
            Err(e) if e.is_method_not_found() => {
                v.check_skip(
                    "live:crypto_verify",
                    &format!("crypto.ionic_bond.verify_proposal not yet in bearDog: {e}"),
                );
            }
            Err(e) if e.is_skippable() => {
                v.check_skip(
                    "live:crypto_verify",
                    &format!("bearDog not reachable: {e}"),
                );
            }
            Err(e) => {
                v.check_bool(
                    "live:crypto_verify",
                    false,
                    &format!("crypto.ionic_bond.verify_proposal error: {e}"),
                );
            }
        }
    } else {
        v.check_skip(
            "live:crypto_verify",
            "security capability not available — bearDog not in composition",
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ionic_bond_pass() {
        let mut v = ValidationResult::new("ionic-bond");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "ionic bond scenario had {} failures (use --nocapture for details)",
            v.failed
        );
    }
}
