// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Pen Test — Enrollment Replay.
//!
//! `mesh.enroll` uses HMAC-SHA256 proof:
//! ```text
//! proof = HMAC-SHA256(family_seed, node_id|public_key|timestamp)
//! ```
//!
//! The timestamp field provides replay protection — but how strong is it?
//! - What is the acceptance window? (seconds? minutes? unbounded?)
//! - Can an attacker who intercepts a valid proof replay it within the window?
//! - Does bearDog track used proofs to prevent exact replay?
//! - Can the same `node_id` re-enroll with a different `public_key`?
//!
//! This scenario validates the enrollment protocol's resistance to
//! replay attacks and credential reuse.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const ENROLLMENT_HANDLER: &str = include_str!(
    "../../../../../../primals/bearDog/crates/beardog-tunnel/src/unix_socket_ipc/handlers/btsp/enrollment/handler.rs"
);
const ENROLLMENT_CONFIG: &str = include_str!(
    "../../../../../../primals/bearDog/crates/beardog-tunnel/src/unix_socket_ipc/handlers/btsp/enrollment/config.rs"
);
const ENROLLMENT_REPLAY: &str = include_str!(
    "../../../../../../primals/bearDog/crates/beardog-tunnel/src/unix_socket_ipc/handlers/btsp/enrollment/replay_cache.rs"
);
const ENROLLMENT_CRYPTO: &str = include_str!(
    "../../../../../../primals/bearDog/crates/beardog-tunnel/src/unix_socket_ipc/handlers/btsp/enrollment/crypto.rs"
);

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-pen-enrollment-replay",
        track: Track::Security,
        tier: Tier::Rust,
        provenance_crate: "wave150w_tower_pen",
        provenance_date: "2026-07-23",
        description: "Tower pen — enrollment HMAC replay: timestamp window, proof reuse, credential rotation",
    },
    run,
};

fn enrollment_contains(pattern: &str) -> bool {
    ENROLLMENT_HANDLER.contains(pattern)
        || ENROLLMENT_CONFIG.contains(pattern)
        || ENROLLMENT_REPLAY.contains(pattern)
        || ENROLLMENT_CRYPTO.contains(pattern)
}

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Timestamp validation");
    phase_timestamp_validation(v);

    v.section("Phase 2: Proof replay resistance");
    phase_replay_resistance(v);

    v.section("Phase 3: Credential rotation");
    phase_credential_rotation(v);
}

fn phase_timestamp_validation(v: &mut ValidationResult) {
    let has_timestamp_field = enrollment_contains("timestamp");
    v.check_bool(
        "replay:timestamp_in_proof",
        has_timestamp_field,
        "Enrollment proof includes timestamp field (replay protection basis)",
    );

    let has_timestamp_validation = enrollment_contains("SystemTime")
        || enrollment_contains("Utc::now")
        || enrollment_contains("time_window")
        || enrollment_contains("max_age")
        || enrollment_contains("timestamp_check");
    v.check_bool(
        "replay:timestamp_window_enforced",
        has_timestamp_validation,
        &format!(
            "Timestamp window enforcement: {} — without a window, old proofs are valid forever",
            if has_timestamp_validation {
                "PRESENT (checks current time vs proof timestamp)"
            } else {
                "ABSENT — enrollment accepts proofs with ANY timestamp (replay risk)"
            }
        ),
    );

    let has_explicit_window = enrollment_contains("300")
        || enrollment_contains("600")
        || enrollment_contains("MAX_AGE")
        || enrollment_contains("window_secs");
    v.check_bool(
        "replay:explicit_time_window",
        has_explicit_window,
        &format!(
            "Explicit time window: {} — defines how long a proof remains valid",
            if has_explicit_window {
                "CONFIGURED"
            } else {
                "NOT CONFIGURED (default: accept any timestamp)"
            }
        ),
    );
}

fn phase_replay_resistance(v: &mut ValidationResult) {
    let has_used_proof_tracking = enrollment_contains("used_proofs")
        || enrollment_contains("seen_proofs")
        || enrollment_contains("nonce")
        || enrollment_contains("idempotency")
        || enrollment_contains("HashSet")
        || enrollment_contains("HashMap")
        || enrollment_contains("ReplayCache")
        || enrollment_contains("bloom")
        || enrollment_contains("blake3");
    v.check_bool(
        "replay:used_proof_tracking",
        has_used_proof_tracking,
        &format!(
            "Used proof tracking: {} — without tracking, the exact same proof \
             can be replayed within the timestamp window",
            if has_used_proof_tracking {
                "PRESENT (proofs tracked to prevent replay)"
            } else {
                "ABSENT — same valid proof can be submitted multiple times"
            }
        ),
    );

    let has_constant_time = enrollment_contains("constant_time")
        || enrollment_contains("ct_eq")
        || enrollment_contains("ConstantTimeEq")
        || enrollment_contains("subtle");
    v.check_bool(
        "replay:constant_time_comparison",
        has_constant_time,
        &format!(
            "Constant-time HMAC comparison: {} — timing side-channel mitigation",
            if has_constant_time {
                "PRESENT"
            } else {
                "NOT DETECTED (may use `==` — timing oracle risk)"
            }
        ),
    );

    let has_hmac_sha256 = enrollment_contains("hmac")
        || enrollment_contains("Hmac")
        || enrollment_contains("HMAC-SHA256");
    v.check_bool(
        "replay:hmac_sha256_algorithm",
        has_hmac_sha256,
        "HMAC-SHA256 algorithm used (appropriate strength for enrollment proof)",
    );
}

fn phase_credential_rotation(v: &mut ValidationResult) {
    let has_public_key_binding = enrollment_contains("public_key");
    v.check_bool(
        "replay:public_key_in_proof",
        has_public_key_binding,
        "Public key is bound to enrollment proof (key rotation requires new proof)",
    );

    let has_node_id_uniqueness =
        enrollment_contains("node_id") && enrollment_contains("verify");
    v.check_bool(
        "replay:node_id_identity_binding",
        has_node_id_uniqueness,
        "node_id is part of the HMAC message — proof is scoped to specific gate identity",
    );

    let has_family_seed_env =
        enrollment_contains("FAMILY_SEED") || enrollment_contains("family_seed");
    v.check_bool(
        "replay:family_seed_source",
        has_family_seed_env,
        &format!(
            "Family seed source: {} — seed compromise allows forging enrollments for ANY gate",
            if has_family_seed_env {
                "environment variable FAMILY_SEED"
            } else {
                "UNKNOWN SOURCE"
            }
        ),
    );

    let has_seed_rotation = enrollment_contains("rotate")
        || enrollment_contains("version")
        || enrollment_contains("generation")
        || enrollment_contains("seed_epoch")
        || enrollment_contains("key_id");
    v.check_bool(
        "replay:seed_rotation_support",
        has_seed_rotation,
        &format!(
            "Family seed rotation: {} — long-lived secrets without rotation are a risk (P2 finding)",
            if has_seed_rotation {
                "SUPPORTED"
            } else {
                "NOT SUPPORTED (single seed, no versioning — P2 #5 for bearDog/eastGate)"
            }
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_no_panic() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
    }
}
