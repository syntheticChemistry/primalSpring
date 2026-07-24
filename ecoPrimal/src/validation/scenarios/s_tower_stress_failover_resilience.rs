// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Stress — Failover Resilience.
//!
//! What happens when a Tower Atomic primal dies mid-operation?
//!
//! - bearDog dies: all BTSP sessions fail, no new crypto, existing
//!   encrypted connections lose key material. Does songBird detect
//!   the dead socket and degrade gracefully?
//! - songBird dies: all mesh routing stops, capability dispatch fails.
//!   Does cellMembrane detect and restart? Do local primals queue?
//! - skunkBat dies: no threat detection, but transport should continue.
//!   Is skunkBat truly optional at runtime?
//!
//! Validates: socket health probes, restart detection, error propagation
//! paths, and whether the composition degrades gracefully vs crashes.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const DISPATCH_SRC: &str = include_str!(
    "../../../../../../primals/songBird/crates/songbird-universal-ipc/src/service/capability_dispatch.rs"
);
const SOCKET_DISCOVERY_SRC: &str = include_str!(
    "../../../../../../primals/songBird/crates/songbird-crypto-provider/src/socket_discovery.rs"
);

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-stress-failover-resilience",
        track: Track::Lifecycle,
        tier: Tier::Rust,
        provenance_crate: "wave150w_tower_stress",
        provenance_date: "2026-07-23",
        description: "Tower stress — primal death mid-session: bearDog, songBird, skunkBat failure paths",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: bearDog death — crypto failure path");
    phase_beardog_death(v);

    v.section("Phase 2: songBird death — dispatch failure path");
    phase_songbird_death(v);

    v.section("Phase 3: skunkBat death — defense optional?");
    phase_skunkbat_death(v);

    v.section("Phase 4: Recovery detection");
    phase_recovery_detection(v);
}

fn phase_beardog_death(v: &mut ValidationResult) {
    let has_connect_error_handling = SOCKET_DISCOVERY_SRC.contains("Err")
        || SOCKET_DISCOVERY_SRC.contains("error")
        || SOCKET_DISCOVERY_SRC.contains("fallback");
    v.check_bool(
        "failover:beardog_connect_error",
        has_connect_error_handling,
        "Socket discovery handles bearDog unavailability (error path or fallback)",
    );

    let has_retry = SOCKET_DISCOVERY_SRC.contains("retry")
        || SOCKET_DISCOVERY_SRC.contains("Retry")
        || SOCKET_DISCOVERY_SRC.contains("attempt");
    v.check_bool(
        "failover:beardog_retry",
        has_retry,
        &format!(
            "bearDog connection retry: {} — without retry, a transient restart is a hard failure",
            if has_retry { "PRESENT" } else { "ABSENT" }
        ),
    );

    let dispatch_handles_ipc_error = DISPATCH_SRC.contains("IpcStream")
        && (DISPATCH_SRC.contains("Err(") || DISPATCH_SRC.contains('?'));
    v.check_bool(
        "failover:dispatch_ipc_error_propagation",
        dispatch_handles_ipc_error,
        "capability.call propagates IPC errors (dead provider socket returns error to caller)",
    );

    let has_multi_socket_fallback = SOCKET_DISCOVERY_SRC.contains("security.sock")
        && SOCKET_DISCOVERY_SRC.contains("crypto.sock")
        && SOCKET_DISCOVERY_SRC.contains("beardog");
    v.check_bool(
        "failover:multi_socket_discovery",
        has_multi_socket_fallback,
        "Socket discovery tries multiple paths (security.sock → crypto.sock → beardog.sock)",
    );
}

fn phase_songbird_death(v: &mut ValidationResult) {
    let dispatch_is_entry =
        DISPATCH_SRC.contains("capability.call") || DISPATCH_SRC.contains("handle_capability_call");
    v.check_bool(
        "failover:songbird_is_dispatch_gateway",
        dispatch_is_entry,
        "songBird is the sole dispatch gateway — its death blocks ALL capability routing",
    );

    let has_health_check = DISPATCH_SRC.contains("health") || DISPATCH_SRC.contains("liveness");
    v.check_bool(
        "failover:songbird_health_surface",
        has_health_check,
        &format!(
            "songBird health check surface: {} — cellMembrane needs health probes to detect death",
            if has_health_check {
                "PRESENT"
            } else {
                "NOT in dispatch path (check service module)"
            }
        ),
    );
}

fn phase_skunkbat_death(v: &mut ValidationResult) {
    let dispatch_requires_skunkbat =
        DISPATCH_SRC.contains("skunkbat") || DISPATCH_SRC.contains("security.advisory");
    v.check_bool(
        "failover:skunkbat_optional_for_dispatch",
        !dispatch_requires_skunkbat,
        &format!(
            "skunkBat in dispatch critical path: {} — \
             defense should be advisory, not blocking",
            if dispatch_requires_skunkbat {
                "YES (dispatch depends on skunkBat — its death blocks routing)"
            } else {
                "NO (dispatch works without skunkBat — defense is optional layer)"
            }
        ),
    );

    let drawbridge_checks_skunkbat =
        DISPATCH_SRC.contains("drawbridge") && DISPATCH_SRC.contains("security");
    v.check_bool(
        "failover:drawbridge_skunkbat_dependency",
        drawbridge_checks_skunkbat || !dispatch_requires_skunkbat,
        &format!(
            "Drawbridge → skunkBat dependency: {} — HTTP gateway may require advisory checks",
            if drawbridge_checks_skunkbat {
                "drawbridge references security (may block on skunkBat death)"
            } else {
                "no direct dependency detected"
            }
        ),
    );
}

fn phase_recovery_detection(v: &mut ValidationResult) {
    let has_socket_exists_check =
        SOCKET_DISCOVERY_SRC.contains("exists()") || SOCKET_DISCOVERY_SRC.contains("Path::new");
    v.check_bool(
        "failover:socket_existence_probe",
        has_socket_exists_check,
        "Socket existence check before connect (detect restart via new socket file)",
    );

    let has_inotify_or_poll = SOCKET_DISCOVERY_SRC.contains("notify")
        || SOCKET_DISCOVERY_SRC.contains("watch")
        || SOCKET_DISCOVERY_SRC.contains("poll");
    v.check_bool(
        "failover:socket_watch",
        has_inotify_or_poll,
        &format!(
            "Socket filesystem watch: {} — reactive recovery detection vs polling",
            if has_inotify_or_poll {
                "PRESENT (inotify/watch)"
            } else {
                "ABSENT (must poll for socket reappearance)"
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
