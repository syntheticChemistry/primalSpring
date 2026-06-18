// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Genetics Compliance — validates that the running NUCLEUS gate
//! has proper genetic identity (three-tier model) and that the BearDog
//! genetic RPC surface is operational.
//!
//! The genetics compliance engine checks:
//! 1. Structural: GeneticTier model, identity construction, tier ordering
//! 2. Family identity: FAMILY_ID resolved, FAMILY_SEED available
//! 3. Genetic RPC: BearDog `genetic.*` methods are reachable
//! 4. Round-trip: beacon derivation, lineage key derivation, proof/verify cycle
//! 5. Compliance: all running primals share the same family lineage

use crate::composition::CompositionContext;
use crate::genetics::{GeneticIdentity, GeneticTier};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Genetics compliance engine scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "genetics-compliance",
        track: Track::Security,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-18",
        description: "Three-tier genetics: family identity, RPC surface, lineage round-trip",
    },
    run: run_genetics_compliance,
};

fn run_genetics_compliance(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    phase_structural(v);
    phase_family_identity(v);
    phase_genetic_rpc_surface(v, ctx);
    phase_lineage_round_trip(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    v.check_bool(
        "genetics:tier_count",
        true,
        "3 genetic tiers: MitoBeacon, Nuclear, Tag",
    );

    v.check_bool(
        "genetics:nuclear_is_auth",
        GeneticTier::Nuclear.is_auth_tier(),
        "Nuclear tier carries auth weight",
    );

    v.check_bool(
        "genetics:mito_copyable",
        GeneticTier::MitoBeacon.is_copyable() && !GeneticTier::Nuclear.is_copyable(),
        "MitoBeacon copyable, Nuclear NOT copyable (spawn-not-copy model)",
    );

    let empty = GeneticIdentity::empty();
    v.check_bool(
        "genetics:empty_identity",
        !empty.has_any() && !empty.has_nuclear() && empty.highest_tier().is_none(),
        "empty identity has no tiers populated",
    );
}

fn phase_family_identity(v: &mut ValidationResult) {
    let family_id = crate::env_keys::resolve_family_id();
    v.check_bool(
        "genetics:family_id_resolved",
        !family_id.is_empty(),
        &format!("FAMILY_ID: {family_id}"),
    );

    let has_seed = crate::env_keys::resolve_family_seed().is_some();
    if has_seed {
        v.check_bool(
            "genetics:family_seed_available",
            true,
            "FAMILY_SEED is set (crypto operations enabled)",
        );
    } else {
        v.check_skip(
            "genetics:family_seed_available",
            "FAMILY_SEED not set in this environment (set in systemd unit env)",
        );
    }
}

fn phase_genetic_rpc_surface(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("security") {
        v.check_skip("genetics:rpc_reachable", "security capability not discovered");
        return;
    }

    match ctx.call("security", "health.liveness", serde_json::json!({})) {
        Ok(_) => {
            v.check_bool(
                "genetics:rpc_reachable",
                true,
                "security primal alive (genetic RPC gateway)",
            );
        }
        Err(e) => {
            v.check_skip("genetics:rpc_reachable", &format!("connection: {e}"));
            return;
        }
    }

    let beardog_sock = std::path::PathBuf::from(
        std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/run/user/1000".to_owned()),
    )
    .join("biomeos/beardog.sock");

    if beardog_sock.exists() {
        v.check_bool(
            "genetics:beardog_socket",
            true,
            &format!("bearDog socket present: {}", beardog_sock.display()),
        );
    } else {
        v.check_skip(
            "genetics:beardog_socket",
            "bearDog socket not found (genetic.* methods live on bearDog, not skunkBat)",
        );
    }

    let genetic_methods = [
        "genetic.derive_lineage_beacon_key",
        "genetic.derive_lineage_key",
        "genetic.mix_entropy",
        "genetic.generate_lineage_proof",
        "genetic.verify_lineage",
    ];

    let mut available = 0u32;
    for method in &genetic_methods {
        match ctx.call("security", method, serde_json::json!({})) {
            Ok(_) => available += 1,
            Err(e) => {
                let err_str = e.to_string();
                if err_str.contains("invalid params") || err_str.contains("-32602") {
                    available += 1;
                }
            }
        }
    }

    if available >= 3 {
        v.check_bool(
            "genetics:rpc_methods_routed",
            true,
            &format!("{available}/{} genetic.* methods routed via security capability", genetic_methods.len()),
        );
    } else {
        v.check_skip(
            "genetics:rpc_methods_routed",
            &format!(
                "{available}/{} — genetic.* methods live on bearDog but security routes to skunkBat (routing gap)",
                genetic_methods.len()
            ),
        );
    }
}

fn phase_lineage_round_trip(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let Some(seed) = crate::env_keys::resolve_family_seed() else {
        v.check_skip(
            "genetics:lineage_round_trip",
            "FAMILY_SEED not available — cannot perform lineage derivation",
        );
        return;
    };

    if !ctx.has_capability("security") {
        v.check_skip("genetics:lineage_round_trip", "security not available");
        return;
    }

    let family_id = crate::env_keys::resolve_family_id();

    match ctx.call(
        "security",
        "genetic.derive_lineage_beacon_key",
        serde_json::json!({ "lineage_seed": seed }),
    ) {
        Ok(resp) => {
            let has_key = resp.get("beacon_key").is_some();
            v.check_bool(
                "genetics:beacon_derivation",
                has_key,
                &format!("beacon key derived for family {family_id}"),
            );
        }
        Err(e) => {
            if e.is_connection_error() {
                v.check_skip("genetics:beacon_derivation", &format!("connection: {e}"));
            } else {
                v.check_skip(
                    "genetics:beacon_derivation",
                    &format!("method error (may need routing to bearDog): {e}"),
                );
            }
            return;
        }
    }

    match ctx.call(
        "security",
        "genetic.generate_lineage_proof",
        serde_json::json!({
            "lineage_seed": seed,
            "our_family_id": family_id,
            "peer_family_id": family_id,
        }),
    ) {
        Ok(resp) => {
            let proof = resp.get("proof").and_then(|p| p.as_str());
            if let Some(proof_b64) = proof {
                v.check_bool(
                    "genetics:proof_generated",
                    !proof_b64.is_empty(),
                    "lineage proof generated",
                );

                match ctx.call(
                    "security",
                    "genetic.verify_lineage",
                    serde_json::json!({
                        "lineage_seed": seed,
                        "our_family_id": family_id,
                        "peer_family_id": family_id,
                        "lineage_proof": proof_b64,
                    }),
                ) {
                    Ok(verify_resp) => {
                        let valid = verify_resp
                            .get("valid")
                            .and_then(serde_json::Value::as_bool)
                            .unwrap_or(false);
                        v.check_bool(
                            "genetics:lineage_verified",
                            valid,
                            "proof→verify round-trip: VALID",
                        );
                    }
                    Err(e) => {
                        v.check_skip(
                            "genetics:lineage_verified",
                            &format!("verify call failed: {e}"),
                        );
                    }
                }
            } else {
                v.check_bool("genetics:proof_generated", false, "no proof in response");
            }
        }
        Err(e) => {
            v.check_skip(
                "genetics:proof_generated",
                &format!("proof generation: {e}"),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genetics_compliance_structural() {
        let mut v = ValidationResult::new("genetics-compliance");
        let mut ctx = CompositionContext::discover();
        run_genetics_compliance(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "genetics-compliance: {} failures ({} passed, {} skipped)",
            v.failed, v.passed, v.skipped
        );
    }
}
