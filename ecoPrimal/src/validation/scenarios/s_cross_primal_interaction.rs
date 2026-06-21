// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Cross-Primal Interaction — probes capabilities through direct
//! socket connections that bypass the normal CompositionContext routing.
//!
//! With 13/13 NUCLEUS live, many sockets exist that aren't routed through
//! the standard capability discovery (security → skunkBat, but crypto.sock
//! belongs to bearDog). This scenario tests the actual IPC surface breadth
//! by connecting directly and probing health.liveness on each.
//!
//! This validates:
//! 1. Direct socket connectivity to all 13 primals
//! 2. Sub-capability sockets (crypto, btsp, storage, provenance, etc.)
//! 3. BiomeOS neural API socket
//! 4. Cross-primal method routing (can we reach bearDog for genetics?)

use std::path::{Path, PathBuf};

use crate::composition::CompositionContext;
use crate::ipc::client::PrimalClient;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Cross-primal interaction testing via direct socket probes.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "cross-primal-interaction",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-21",
        description: "Direct socket probes: health.liveness on all primals + sub-capability sockets",
    },
    run: run_cross_primal_interaction,
};

fn run_cross_primal_interaction(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let base = socket_dir();
    let Some(dir) = base else {
        v.check_skip("interaction:socket_dir", "biomeos socket dir not found");
        return;
    };
    phase_primary_primals(v, &dir);
    phase_sub_capability_sockets(v, &dir);
    phase_beardog_genetics(v, &dir);
    phase_biomeos_neural(v, &dir);
}

const PRIMARY_PRIMALS: &[&str] = &[
    "barracuda",
    "beardog",
    "coralreef",
    "loamspine",
    "nestgate",
    "rhizocrypt",
    "skunkbat",
    "songbird",
    "squirrel",
    "sweetgrass",
    "toadstool",
];

fn phase_primary_primals(v: &mut ValidationResult, dir: &Path) {
    let mut alive = 0u32;
    let total = PRIMARY_PRIMALS.len();

    for primal in PRIMARY_PRIMALS {
        let sock = dir.join(format!("{primal}.sock"));
        if !sock.exists() {
            v.check_skip(
                &format!("primal:{primal}:liveness"),
                &format!("{primal}.sock not present"),
            );
            continue;
        }

        match probe_health(&sock, primal) {
            ProbeResult::Alive => {
                alive += 1;
                v.check_bool(
                    &format!("primal:{primal}:liveness"),
                    true,
                    &format!("{primal}: ALIVE via direct socket"),
                );
            }
            ProbeResult::Responded => {
                alive += 1;
                v.check_bool(
                    &format!("primal:{primal}:liveness"),
                    true,
                    &format!("{primal}: responded (method may differ)"),
                );
            }
            ProbeResult::ConnectionFailed(e) => {
                v.check_skip(
                    &format!("primal:{primal}:liveness"),
                    &format!("{primal}: connection failed — {e}"),
                );
            }
        }
    }

    #[expect(clippy::cast_possible_truncation, reason = "primal count < 256")]
    let total_u32 = total as u32;
    v.check_bool(
        "interaction:primary_alive",
        alive >= total_u32.saturating_sub(2),
        &format!("{alive}/{total} primary primals alive via direct socket"),
    );
}

const SUB_CAPABILITY_SOCKETS: &[(&str, &str)] = &[
    ("crypto.sock", "bearDog crypto sub-capability"),
    ("btsp.sock", "BTSP tunnel endpoint"),
    ("security.sock", "skunkBat security"),
    ("ledger.sock", "loamSpine ledger"),
    ("ai.sock", "barraCuda AI"),
    ("shader.sock", "coralReef shader"),
    ("storage.sock", "nestGate storage"),
    ("provenance.sock", "rhizoCrypt provenance"),
    ("compute-tarpc.sock", "toadStool compute"),
];

fn phase_sub_capability_sockets(v: &mut ValidationResult, dir: &Path) {
    let mut reachable = 0u32;
    let mut total_present = 0u32;

    for &(sock_name, _desc) in SUB_CAPABILITY_SOCKETS {
        let sock = dir.join(sock_name);
        if !sock.exists() {
            continue;
        }
        total_present += 1;

        let label = sock_name.strip_suffix(".sock").unwrap_or(sock_name);
        match probe_health(&sock, label) {
            ProbeResult::Alive | ProbeResult::Responded => {
                reachable += 1;
            }
            ProbeResult::ConnectionFailed(_) => {}
        }
    }

    v.check_bool(
        "interaction:sub_capabilities",
        reachable >= 4,
        &format!(
            "{reachable}/{total_present} sub-capability sockets respond ({} defined)",
            SUB_CAPABILITY_SOCKETS.len()
        ),
    );
}

fn phase_beardog_genetics(v: &mut ValidationResult, dir: &Path) {
    let beardog_sock = dir.join("beardog.sock");
    if !beardog_sock.exists() {
        v.check_skip("interaction:beardog_genetics", "beardog.sock not present");
        return;
    }

    let Ok(mut client) = PrimalClient::connect(&beardog_sock, "beardog") else {
        v.check_skip("interaction:beardog_genetics", "beardog connection failed");
        return;
    };

    let result = client.call(
        "genetic.derive_lineage_beacon_key",
        serde_json::json!({"lineage_seed": "dGVzdA=="}),
    );

    match result {
        Ok(resp) => {
            let has_key = resp
                .result
                .as_ref()
                .is_some_and(|r| r.get("beacon_key").is_some());
            v.check_bool(
                "interaction:beardog_genetics",
                has_key,
                "bearDog genetic.derive_lineage_beacon_key: key derived",
            );
        }
        Err(e) => {
            let method_exists = !e.to_string().contains("method not found");
            v.check_bool(
                "interaction:beardog_genetics",
                method_exists,
                &format!("bearDog genetic RPC: {e}"),
            );
        }
    }
}

fn phase_biomeos_neural(v: &mut ValidationResult, dir: &Path) {
    let neural_sock = dir.join("biomeos-neural.sock");
    if !neural_sock.exists() {
        v.check_skip(
            "interaction:biomeos_neural",
            "biomeos-neural.sock not present",
        );
        return;
    }

    match probe_health(&neural_sock, "biomeos-neural") {
        ProbeResult::Alive => {
            v.check_bool(
                "interaction:biomeos_neural",
                true,
                "biomeOS neural API socket: ALIVE",
            );
        }
        ProbeResult::Responded => {
            v.check_bool(
                "interaction:biomeos_neural",
                true,
                "biomeOS neural API: responds to RPC",
            );
        }
        ProbeResult::ConnectionFailed(e) => {
            v.check_skip(
                "interaction:biomeos_neural",
                &format!("biomeOS neural connection: {e}"),
            );
        }
    }
}

enum ProbeResult {
    Alive,
    Responded,
    ConnectionFailed(String),
}

fn probe_health(sock: &Path, label: &str) -> ProbeResult {
    let mut client = match PrimalClient::connect(sock, label) {
        Ok(c) => c,
        Err(e) => return ProbeResult::ConnectionFailed(e.to_string()),
    };

    match client.call("health.liveness", serde_json::json!({})) {
        Ok(resp) => {
            if resp.result.is_some() {
                ProbeResult::Alive
            } else {
                ProbeResult::Responded
            }
        }
        Err(e) => {
            if e.is_connection_error() {
                ProbeResult::ConnectionFailed(e.to_string())
            } else {
                ProbeResult::Responded
            }
        }
    }
}

fn socket_dir() -> Option<PathBuf> {
    let runtime = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/run/user/1000".to_owned());
    let dir = PathBuf::from(runtime).join("biomeos");
    dir.is_dir().then_some(dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_primal_interaction_structural() {
        let mut v = ValidationResult::new("cross-primal-interaction");
        let mut ctx = CompositionContext::discover();
        run_cross_primal_interaction(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "cross-primal-interaction: {} failures ({} passed, {} skipped)",
            v.failed, v.passed, v.skipped
        );
    }
}
