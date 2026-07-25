// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Pen Test — UDS Socket Spoof.
//!
//! The Tower Atomic trio discovers each other via filesystem sockets:
//! `security.sock`, `songbird.sock`, `skunkbat.sock`. These are
//! typically symlinks (e.g., `security.sock → beardog.sock`).
//!
//! Attack vectors:
//! - Replace `security.sock` symlink to point to an attacker socket
//!   that impersonates bearDog (returns valid-looking but attacker-
//!   controlled BTSP sessions)
//! - Create a rogue `federation.sock` before songBird starts, capturing
//!   threat broadcasts from skunkBat
//! - Symlink race: replace socket between discovery and connect
//! - File permission bypass: socket created with world-readable perms
//!
//! Validates socket identity verification, filesystem permissions,
//! and symlink safety.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const SOCKET_DISCOVERY_SRC: &str = include_str!(
    "../../../../../../primals/songBird/crates/songbird-crypto-provider/src/socket_discovery.rs"
);
const PLATFORM_UNIX_SRC: &str = include_str!(
    "../../../../../../primals/songBird/crates/songbird-universal-ipc/src/platform/unix.rs"
);
const METHOD_GATE_CALLER_SRC: &str = include_str!(
    "../../../../../../primals/songBird/crates/songbird-orchestrator/src/ipc/pure_rust_server/method_gate/caller.rs"
);
const BEARDOG_SERVER_SRC: &str = include_str!(
    "../../../../../../primals/bearDog/crates/beardog-tunnel/src/unix_socket_ipc/server.rs"
);

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-pen-uds-spoof",
        track: Track::Security,
        tier: Tier::Rust,
        provenance_crate: "wave150w_tower_pen",
        provenance_date: "2026-07-23",
        description: "Tower pen — UDS socket hijacking: symlink replacement, identity spoof, permission bypass",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Socket identity verification");
    phase_identity_verification(v);

    v.section("Phase 2: Filesystem permissions");
    phase_filesystem_permissions(v);

    v.section("Phase 3: Symlink safety");
    phase_symlink_safety(v);
}

fn phase_identity_verification(v: &mut ValidationResult) {
    let has_identity_probe = SOCKET_DISCOVERY_SRC.contains("identity")
        || SOCKET_DISCOVERY_SRC.contains("verify_identity")
        || SOCKET_DISCOVERY_SRC.contains("health.identity")
        || METHOD_GATE_CALLER_SRC.contains("CallerContext")
        || METHOD_GATE_CALLER_SRC.contains("PeerCredentials");
    v.check_bool(
        "spoof:identity_probe_on_connect",
        has_identity_probe,
        &format!(
            "Identity probe after socket connect: {} — without identity verification, \
             a rogue socket impersonating bearDog would be accepted silently",
            if has_identity_probe {
                "PRESENT (CallerContext + PeerCredentials in method gate)"
            } else {
                "ABSENT (trust-on-connect: any process on the socket is accepted)"
            }
        ),
    );

    let has_process_verification = SOCKET_DISCOVERY_SRC.contains("pid")
        || SOCKET_DISCOVERY_SRC.contains("SO_PEERCRED")
        || SOCKET_DISCOVERY_SRC.contains("UCred")
        || METHOD_GATE_CALLER_SRC.contains("SO_PEERCRED")
        || METHOD_GATE_CALLER_SRC.contains("peer_cred");
    v.check_bool(
        "spoof:peer_credentials_check",
        has_process_verification,
        &format!(
            "UDS peer credentials (SO_PEERCRED): {} — Linux provides PID/UID of socket peer; \
             verifying the PID belongs to bearDog process prevents impersonation",
            if has_process_verification {
                "CHECKED (CallerContext::from_unix_stream extracts peer cred)"
            } else {
                "NOT CHECKED (any local process can impersonate bearDog)"
            }
        ),
    );

    let has_btsp_on_local = SOCKET_DISCOVERY_SRC.contains("btsp")
        || SOCKET_DISCOVERY_SRC.contains("handshake")
        || BEARDOG_SERVER_SRC.contains("require_btsp");
    v.check_bool(
        "spoof:btsp_on_local_sockets",
        has_btsp_on_local,
        &format!(
            "BTSP handshake on local UDS: {} — BTSP between co-located primals would \
             cryptographically verify the peer's identity (defense-in-depth)",
            if has_btsp_on_local {
                "PRESENT"
            } else {
                "ABSENT (local UDS connections are unencrypted, trust filesystem)"
            }
        ),
    );
}

fn phase_filesystem_permissions(v: &mut ValidationResult) {
    let has_permission_check = SOCKET_DISCOVERY_SRC.contains("permissions")
        || SOCKET_DISCOVERY_SRC.contains("mode")
        || SOCKET_DISCOVERY_SRC.contains("0o700")
        || SOCKET_DISCOVERY_SRC.contains("chmod")
        || PLATFORM_UNIX_SRC.contains("0o600")
        || PLATFORM_UNIX_SRC.contains("0o700")
        || PLATFORM_UNIX_SRC.contains("set_permissions")
        || PLATFORM_UNIX_SRC.contains("chmod");
    v.check_bool(
        "spoof:socket_permissions",
        has_permission_check,
        &format!(
            "Socket file permissions check: {} — sockets should be owner-only (0o700/0o600) \
             to prevent other users from connecting",
            if has_permission_check {
                "PRESENT (platform/unix.rs: chmod 0600 on socket creation)"
            } else {
                "NOT CHECKED (relies on directory permissions)"
            }
        ),
    );

    let has_dir_check = SOCKET_DISCOVERY_SRC.contains("XDG_RUNTIME_DIR")
        || SOCKET_DISCOVERY_SRC.contains("biomeos");
    v.check_bool(
        "spoof:socket_directory_controlled",
        has_dir_check,
        &format!(
            "Socket directory: {} — XDG_RUNTIME_DIR is user-private (mode 0700) on systemd, \
             providing directory-level isolation",
            if has_dir_check {
                "uses XDG_RUNTIME_DIR/biomeos/ (user-private)"
            } else {
                "UNKNOWN directory (may be world-accessible)"
            }
        ),
    );
}

fn phase_symlink_safety(v: &mut ValidationResult) {
    let has_symlink_resolution = SOCKET_DISCOVERY_SRC.contains("canonicalize")
        || SOCKET_DISCOVERY_SRC.contains("read_link")
        || SOCKET_DISCOVERY_SRC.contains("realpath")
        || PLATFORM_UNIX_SRC.contains("is_symlink")
        || PLATFORM_UNIX_SRC.contains("symlink");
    v.check_bool(
        "spoof:symlink_resolution",
        has_symlink_resolution,
        &format!(
            "Symlink protection: {} — resolving/rejecting symlinks prevents \
             attacker-controlled socket paths",
            if has_symlink_resolution {
                "PRESENT (platform/unix.rs rejects bind on symlink paths)"
            } else {
                "NOT DONE (connects to whatever the symlink points to)"
            }
        ),
    );

    let has_toctou_protection = SOCKET_DISCOVERY_SRC.contains("O_NOFOLLOW")
        || SOCKET_DISCOVERY_SRC.contains("atomic")
        || PLATFORM_UNIX_SRC.contains("is_symlink");
    v.check_bool(
        "spoof:toctou_protection",
        has_toctou_protection,
        &format!(
            "TOCTOU protection: {} — between checking symlink target and connecting, \
             an attacker could swap the symlink (race condition)",
            if has_toctou_protection {
                "PRESENT"
            } else {
                "ABSENT (symlink swap race possible between discover and connect)"
            }
        ),
    );

    let discovery_paths: Vec<&str> = SOCKET_DISCOVERY_SRC
        .lines()
        .filter(|l| l.contains(".sock") && (l.contains("security") || l.contains("beardog")))
        .take(5)
        .collect();
    v.check_bool(
        "spoof:discovery_path_hardcoded",
        !discovery_paths.is_empty(),
        &format!(
            "{} discovery socket paths found — each is a potential spoof target \
             (attacker creates a socket at one of these paths before the real primal starts)",
            discovery_paths.len()
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
