// SPDX-License-Identifier: AGPL-3.0-or-later
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

    v.section("Phase 4: Live Discovery + Health");
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
    v.check_bool(
        "accept_transitions_to_active",
        resp.is_ok() && resp.as_ref().unwrap().accepted,
        "Proposed → Active on accept",
    );

    let call_ok = reg.record_call(&id, "compute.submit", 2048);
    v.check_bool("metered_call_succeeds", call_ok.is_ok(), "Metered call within scope");

    let usage = &reg.get(&id).unwrap().usage;
    v.check_bool(
        "usage_metrics_increment",
        usage.total_calls == 1 && usage.total_bytes == 2048,
        &format!("Usage: {} calls, {} bytes", usage.total_calls, usage.total_bytes),
    );

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
    let id = reg.propose(sample_proposal()).unwrap();
    reg.accept(&id, compute_constraints()).unwrap();

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

fn phase_live_discovery(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    for cap in ["security", "discovery"] {
        if !ctx.has_capability(cap) {
            v.check_skip(
                &format!("health_liveness_{cap}"),
                &format!("capability {cap} not in composition context"),
            );
            continue;
        }
        match ctx.health_check(cap) {
            Ok(true) => v.check_bool(
                &format!("health_liveness_{cap}"),
                true,
                &format!("{cap} health.liveness ok"),
            ),
            Ok(false) => v.check_bool(
                &format!("health_liveness_{cap}"),
                false,
                &format!("{cap} not live"),
            ),
            Err(e) if e.is_connection_error() => v.check_skip(
                &format!("health_liveness_{cap}"),
                &format!("{cap} not reachable: {e}"),
            ),
            Err(e) => v.check_bool(
                &format!("health_liveness_{cap}"),
                false,
                &format!("error: {e}"),
            ),
        }
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
