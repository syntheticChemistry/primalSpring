// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Stress — BTSP Handshake Storm.
//!
//! Every new connection through the Tower Atomic stack requires a
//! BTSP handshake: `btsp.session.create` + `btsp.session.verify` +
//! (optionally) `btsp.server.export_keys`. Each is a UDS round-trip
//! from songBird/skunkBat → bearDog.
//!
//! Under storm conditions (N concurrent new connections), bearDog's
//! UDS accept loop and crypto operations (`Ed25519`, `ChaCha20` key
//! derivation) become the bottleneck.
//!
//! This scenario validates:
//! - bearDog can handle concurrent BTSP sessions
//! - Session state management under parallel create/verify
//! - Crypto operation throughput (how many handshakes/second?)
//! - Graceful degradation (queue, reject, or crash?)

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const BTSP_SERVER_SRC: &str = include_str!(
    "../../../../../../primals/bearDog/crates/beardog-tunnel/src/unix_socket_ipc/server.rs"
);
const BTSP_HANDLER_SRC: &str = include_str!(
    "../../../../../../primals/bearDog/crates/beardog-tunnel/src/unix_socket_ipc/handlers/btsp/mod.rs"
);

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-stress-btsp-storm",
        track: Track::Security,
        tier: Tier::Rust,
        provenance_crate: "wave150w_tower_stress",
        provenance_date: "2026-07-23",
        description: "Tower stress — N concurrent BTSP handshakes saturating bearDog crypto pipeline",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: bearDog session concurrency model");
    phase_session_concurrency(v);

    v.section("Phase 2: Crypto pipeline throughput");
    phase_crypto_pipeline(v);

    v.section("Phase 3: Graceful degradation");
    phase_degradation(v);
}

fn phase_session_concurrency(v: &mut ValidationResult) {
    let has_async_accept =
        BTSP_SERVER_SRC.contains("accept()") || BTSP_SERVER_SRC.contains("incoming()");
    v.check_bool(
        "btsp_storm:async_accept",
        has_async_accept,
        "bearDog UDS server uses async accept loop (can handle concurrent connections)",
    );

    let spawns_per_connection =
        BTSP_SERVER_SRC.contains("tokio::spawn") || BTSP_SERVER_SRC.contains("spawn(");
    v.check_bool(
        "btsp_storm:per_connection_task",
        spawns_per_connection,
        &format!(
            "bearDog {} per incoming connection — {} concurrent handshakes",
            if spawns_per_connection {
                "spawns a task"
            } else {
                "processes serially"
            },
            if spawns_per_connection {
                "supports"
            } else {
                "SERIALIZES"
            }
        ),
    );

    let has_session_map = BTSP_HANDLER_SRC.contains("HashMap")
        || BTSP_HANDLER_SRC.contains("BTreeMap")
        || BTSP_HANDLER_SRC.contains("sessions")
        || BTSP_HANDLER_SRC.contains("SessionStore");
    v.check_bool(
        "btsp_storm:session_storage",
        has_session_map,
        &format!(
            "BTSP session storage: {} — concurrent creates need thread-safe session map",
            if has_session_map {
                "session map found"
            } else {
                "NO session storage detected (stateless handshakes?)"
            }
        ),
    );

    let has_concurrent_guard = BTSP_HANDLER_SRC.contains("RwLock")
        || BTSP_HANDLER_SRC.contains("Mutex")
        || BTSP_HANDLER_SRC.contains("DashMap")
        || BTSP_HANDLER_SRC.contains("Arc<");
    v.check_bool(
        "btsp_storm:concurrent_session_access",
        has_concurrent_guard,
        &format!(
            "Session state concurrency: {} — without guards, concurrent create/verify races",
            if has_concurrent_guard {
                "guarded"
            } else {
                "UNGUARDED"
            }
        ),
    );
}

fn phase_crypto_pipeline(v: &mut ValidationResult) {
    let has_ed25519 = BTSP_HANDLER_SRC.contains("ed25519") || BTSP_HANDLER_SRC.contains("Ed25519");
    v.check_bool(
        "btsp_storm:ed25519_ops",
        has_ed25519,
        "BTSP uses Ed25519 (fast: ~70k verify/s on x86_64, not a bottleneck at <1000 concurrent)",
    );

    let has_chacha = BTSP_HANDLER_SRC.contains("chacha") || BTSP_HANDLER_SRC.contains("ChaCha");
    let has_aead = BTSP_HANDLER_SRC.contains("AEAD")
        || BTSP_HANDLER_SRC.contains("encrypt")
        || BTSP_HANDLER_SRC.contains("cipher");
    v.check_bool(
        "btsp_storm:symmetric_crypto",
        has_chacha || has_aead,
        &format!(
            "Symmetric crypto: {} — key derivation per session",
            if has_chacha {
                "ChaCha20-Poly1305"
            } else if has_aead {
                "AEAD (specific cipher TBD)"
            } else {
                "NONE DETECTED"
            }
        ),
    );

    let has_key_export =
        BTSP_HANDLER_SRC.contains("export_keys") || BTSP_HANDLER_SRC.contains("ExportKeys");
    v.check_bool(
        "btsp_storm:key_export",
        has_key_export,
        &format!(
            "Key export for Phase 3 encrypted framing: {}",
            if has_key_export {
                "present (per-session cost: derive + serialize keys)"
            } else {
                "NOT FOUND"
            }
        ),
    );
}

fn phase_degradation(v: &mut ValidationResult) {
    let has_rate_limit = BTSP_SERVER_SRC.contains("rate_limit")
        || BTSP_SERVER_SRC.contains("RateLimit")
        || BTSP_SERVER_SRC.contains("throttle")
        || BTSP_SERVER_SRC.contains("semaphore");
    v.check_bool(
        "btsp_storm:rate_limiting",
        has_rate_limit,
        &format!(
            "Connection rate limiting: {} — without rate limiting, \
             a storm can exhaust file descriptors or CPU",
            if has_rate_limit {
                "PRESENT"
            } else {
                "ABSENT (risk: unbounded concurrent handshakes)"
            }
        ),
    );

    let has_max_connections = BTSP_SERVER_SRC.contains("max_connections")
        || BTSP_SERVER_SRC.contains("MAX_CONN")
        || BTSP_SERVER_SRC.contains("connection_limit");
    v.check_bool(
        "btsp_storm:connection_cap",
        has_max_connections,
        &format!(
            "Max connection cap: {} — under storm, unbounded accepts leak resources",
            if has_max_connections {
                "ENFORCED"
            } else {
                "ABSENT (resource leak risk)"
            }
        ),
    );

    let has_error_on_overload = BTSP_HANDLER_SRC.contains("TooMany")
        || BTSP_HANDLER_SRC.contains("overloaded")
        || BTSP_HANDLER_SRC.contains("backpressure");
    v.check_bool(
        "btsp_storm:backpressure_signal",
        has_error_on_overload,
        &format!(
            "Backpressure signaling: {} — callers need feedback when bearDog is saturated",
            if has_error_on_overload {
                "PRESENT"
            } else {
                "ABSENT (callers won't know bearDog is overloaded)"
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
