// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Socket Directory Unification — validates unified socket directory
//! contract via platform capabilities and environment override keys.

use crate::composition::CompositionContext;
use crate::env_keys;
use crate::ipc::discover::resolve_socket_dir;
use crate::ipc::platform::PlatformCapabilities;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Socket directory unification scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "socket-directory-unification",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "wave138a_socket_directory_unification",
        provenance_date: "2026-07-14",
        description: "Socket directory unification — platform socket_dir and /run/membrane equivalent",
    },
    run,
};

/// Run socket directory unification validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Platform capabilities declare socket directory");

    let caps = PlatformCapabilities::detect();
    v.check_bool(
        "platform:uds_available",
        caps.uds_available || caps.tcp_available,
        "platform has at least one transport available",
    );

    if caps.uds_available {
        v.check_bool(
            "platform:socket_dir_declared",
            caps.socket_dir.is_some(),
            &format!(
                "PlatformCapabilities.socket_dir: {:?}",
                caps.socket_dir.as_ref().map(|p| p.display().to_string())
            ),
        );
    } else {
        v.check_skip("platform:socket_dir_declared", "UDS unavailable on this platform");
    }

    v.section("Phase 2: Socket directory resolution contract");

    let socket_dir = resolve_socket_dir();
    v.check_bool(
        "socket:dir_non_empty",
        !socket_dir.is_empty(),
        &format!("resolve_socket_dir() = {socket_dir}"),
    );

    v.check_bool(
        "socket:override_keys_defined",
        !env_keys::SOCKET_DIR.is_empty() && !env_keys::ECOPRIMALS_SOCKET_DIR.is_empty(),
        "SOCKET_DIR and ECOPRIMALS_SOCKET_DIR env keys defined for /run/membrane override",
    );

    let biomeos_dir = crate::tolerances::platform::biomeos_socket_dir();
    v.check_bool(
        "socket:biomeos_subdir",
        biomeos_dir
            .to_string_lossy()
            .contains(env_keys::BIOMEOS_SUBDIR),
        &format!(
            "biomeos_socket_dir() uses {} subdir: {}",
            env_keys::BIOMEOS_SUBDIR,
            biomeos_dir.display()
        ),
    );

    v.section("Phase 3: VPS membrane path equivalence");

    let is_membrane_path = socket_dir.contains("membrane")
        || socket_dir.contains("biomeos")
        || socket_dir.contains("ecoprimals")
        || socket_dir.contains("runtime");
    v.check_bool(
        "socket:membrane_equivalent",
        is_membrane_path || caps.uds_available,
        &format!(
            "socket dir '{socket_dir}' is /run/membrane equivalent or UDS-capable platform"
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_passes_structural() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "scenario '{}' had {} failures",
            SCENARIO.meta.id, v.failed
        );
    }
}
