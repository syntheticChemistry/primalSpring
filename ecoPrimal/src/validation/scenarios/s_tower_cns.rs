// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower CNS Convergence — Songbird as central nervous system.
//!
//! Validates TCP port consolidation toward Songbird-only federation.
//! All inter-gate traffic routes through Songbird braid. Intra-gate
//! stays UDS (zero TCP for non-Tower primals).
//!
//! Tier::Both — structural port mapping always runs; live port
//! reachability checks require deployed primals.

use crate::composition::CompositionContext;
use crate::ipc::NeuralBridge;
use crate::tolerances::FEDERATION_PORTS;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-cns",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "exp114_tower_cns_convergence",
        provenance_date: "2026-05-26",
        description: "Tower CNS convergence — Songbird as sole cross-gate relay, TCP port consolidation",
    },
    run,
};

/// Run all Tower CNS convergence validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — port inventory");
    phase_structural(v);

    v.section("Phase 2: UDS capability coverage");
    phase_uds_coverage(v);

    v.section("Phase 3: Tower relay readiness");
    phase_tower_relay(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    let droppable = FEDERATION_PORTS.iter().filter(|p| p.droppable).count();
    let keep = FEDERATION_PORTS.iter().filter(|p| !p.droppable).count();
    v.check_bool(
        "struct:port_inventory",
        true,
        &format!(
            "{} known ports: {keep} federation (keep), {droppable} droppable (UDS available)",
            FEDERATION_PORTS.len()
        ),
    );

    for p in FEDERATION_PORTS {
        let label = if p.droppable { "DROPPABLE" } else { "KEEP" };
        let reachable = std::net::TcpStream::connect_timeout(
            &std::net::SocketAddr::from(([127, 0, 0, 1], p.port)),
            std::time::Duration::from_millis(300),
        )
        .is_ok();
        v.check_bool(
            &format!("port:{}:{}", p.primal, p.port),
            reachable,
            &format!(
                "{} {} — {} ({})",
                p.profile,
                p.role,
                label,
                if reachable { "active" } else { "inactive" }
            ),
        );
    }
}

fn phase_uds_coverage(v: &mut ValidationResult) {
    let dir = crate::tolerances::biomeos_socket_dir();

    let caps_sockets: &[(&str, &str)] = &[
        ("crypto", "crypto.sock"),
        ("security", "security.sock"),
        ("btsp", "btsp.sock"),
        ("discovery", "discovery.sock"),
        ("braid", "braid.sock"),
        ("compute", "compute.sock"),
        ("storage", "storage.sock"),
        ("ledger", "ledger.sock"),
        ("dag", "dag.sock"),
        ("orchestration", "orchestration.sock"),
        ("visualization", "visualization.sock"),
        ("network", "network.sock"),
        ("ai", "ai.sock"),
    ];

    let mut uds_ready = 0u32;
    for &(cap, sock) in caps_sockets {
        let exists = dir.join(sock).exists();
        if exists {
            uds_ready += 1;
        }
        v.check_bool(
            &format!("uds:{cap}"),
            exists,
            &format!(
                "{sock} — {}",
                if exists {
                    "TCP unnecessary for local"
                } else {
                    "needs Tower relay for remote"
                }
            ),
        );
    }

    #[expect(clippy::cast_precision_loss, reason = "socket count fits f64 exactly")]
    let total = caps_sockets.len() as f64;
    let coverage = (f64::from(uds_ready) / total) * 100.0;
    v.check_bool(
        "uds:coverage",
        coverage > 80.0,
        &format!(
            "{uds_ready}/{} via UDS ({coverage:.0}%)",
            caps_sockets.len()
        ),
    );
}

#[expect(
    clippy::needless_pass_by_ref_mut,
    reason = "ctx reserved for future relay probes"
)]
fn phase_tower_relay(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let caps = ctx.available_capabilities();
    let tower_caps = ["security", "crypto", "discovery"];
    let tower_alive = tower_caps.iter().filter(|c| caps.contains(*c)).count();
    v.check_bool(
        "tower:capabilities",
        tower_alive > 0,
        &format!(
            "{tower_alive}/{} Tower capabilities discovered",
            tower_caps.len()
        ),
    );

    let bridge =
        NeuralBridge::discover().or_else(|| NeuralBridge::discover_with(None, Some("nucleus01")));
    let Some(bridge) = bridge else {
        v.check_skip("tower:neural_api", "Neural API not running");
        return;
    };

    match bridge.health_check() {
        Ok(healthy) => v.check_bool(
            "tower:neural_api",
            healthy,
            "Neural API healthy — relay pathway available",
        ),
        Err(e) => v.check_skip("tower:neural_api", &format!("health: {e}")),
    }

    for cap in &["crypto", "discovery", "compute", "storage"] {
        match bridge.discover_capability(cap) {
            Ok(_) => v.check_bool(
                &format!("tower:relay:{cap}"),
                true,
                &format!("{cap} reachable via Neural API relay"),
            ),
            Err(e) => v.check_skip(&format!("tower:relay:{cap}"), &format!("{cap}: {e}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn tower_cns_no_panic() {
        let mut v = ValidationResult::new("tower-cns");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
    }

    #[test]
    fn port_inventory_consistent() {
        assert!(
            FEDERATION_PORTS
                .iter()
                .filter(|p| !p.droppable)
                .all(|p| p.primal == "songbird"),
            "only Songbird ports should be non-droppable",
        );
        assert!(
            FEDERATION_PORTS.iter().any(|p| p.droppable),
            "should have droppable ports",
        );
    }
}
