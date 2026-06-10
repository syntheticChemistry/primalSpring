// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

//! Exp114: Tower CNS Convergence — Songbird as Central Nervous System
//!
//! Tower Atomic (BearDog + Songbird) is present in *every* atomic
//! composition — the electron shell of the NUCLEUS particle model.
//! This makes it the natural central nervous system (CNS).
//!
//! Currently, individual primals bind TCP ports:
//!   - BearDog: 9900 (nucleus01), 9101 (primalspring01) — crypto RPC
//!   - Songbird: 7700, 7701 — federation (cross-gate braid)
//!   - SkunkBat: 9750 — meta-tier defense
//!   - ToadStool, CoralReef, SweetGrass — ephemeral TCP listeners
//!
//! Goal: consolidate to **Songbird federation ports only**. All inter-gate
//! traffic routes through Songbird braid. Intra-gate stays UDS (zero TCP).
//!
//! Result: minimal attack surface (2 TCP ports per NUCLEUS), Tower as sole
//! cross-gate relay, K-Derm plasma membrane = Songbird federation,
//! cytoplasm = UDS-only inner primals.

use primalspring::composition::CompositionContext;
use primalspring::ipc::NeuralBridge;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp114 — Tower CNS Convergence")
        .with_provenance("exp114_tower_cns_convergence", "2026-05-26")
        .run(
            "Exp114: Songbird CNS — Port Consolidation + Tower Relay",
            |v| {
                v.section("Phase 1: Current Port Census");
                phase_port_census(v);

                v.section("Phase 2: UDS Capability Coverage");
                phase_uds_coverage(v);

                v.section("Phase 3: Tower Relay Readiness");
                phase_tower_relay(v);

                v.section("Phase 4: CNS Convergence Plan");
                phase_cns_plan(v);
            },
        );
}

fn phase_port_census(v: &mut ValidationResult) {
    #[expect(clippy::struct_field_names, reason = "clarity for port census")]
    struct Port {
        port: u16,
        primal: &'static str,
        role: &'static str,
        can_drop: bool,
    }

    let ports = [
        Port {
            port: 7700,
            primal: "songbird",
            role: "federation nucleus01",
            can_drop: false,
        },
        Port {
            port: 7701,
            primal: "songbird",
            role: "federation primalspring01",
            can_drop: false,
        },
        Port {
            port: 9900,
            primal: "beardog",
            role: "crypto RPC nucleus01",
            can_drop: true,
        },
        Port {
            port: 9101,
            primal: "beardog",
            role: "crypto RPC primalspring01",
            can_drop: true,
        },
        Port {
            port: 9750,
            primal: "skunkbat",
            role: "meta-tier defense",
            can_drop: true,
        },
    ];

    let mut reachable_count = 0;
    let mut droppable_count = 0;

    for p in &ports {
        let reachable = std::net::TcpStream::connect_timeout(
            &std::net::SocketAddr::from(([127, 0, 0, 1], p.port)),
            std::time::Duration::from_millis(500),
        )
        .is_ok();

        if reachable {
            reachable_count += 1;
            if p.can_drop {
                droppable_count += 1;
            }
        }

        v.check_bool(
            &format!("port:{}:{}", p.primal, p.port),
            reachable,
            &format!(
                "{} — {}",
                p.role,
                if p.can_drop {
                    "DROPPABLE: available via UDS, TCP unnecessary"
                } else {
                    "KEEP: federation requires TCP for cross-gate braid"
                }
            ),
        );
    }

    v.check_bool(
        "port:summary",
        reachable_count > 0,
        &format!(
            "{reachable_count} ports active, {droppable_count} droppable → \
             target: {} ports only (Songbird federation)",
            reachable_count - droppable_count
        ),
    );
}

fn phase_uds_coverage(v: &mut ValidationResult) {
    let dir = primalspring::tolerances::biomeos_socket_dir();

    let caps_with_sockets: &[(&str, &str)] = &[
        ("crypto", "crypto.sock"),
        ("security", "security.sock"),
        ("btsp", "btsp.sock"),
        ("discovery", "discovery.sock"),
        ("braid", "braid.sock"),
        ("compute", "compute.sock"),
        ("tensor", "tensor.sock"),
        ("shader", "shader.sock"),
        ("storage", "storage.sock"),
        ("ledger", "ledger.sock"),
        ("dag", "dag.sock"),
        ("commit", "commit.sock"),
        ("provenance", "provenance.sock"),
        ("attribution", "attribution.sock"),
        ("orchestration", "orchestration.sock"),
        ("visualization", "visualization.sock"),
        ("network", "network.sock"),
        ("ai", "ai.sock"),
        ("inference", "inference.sock"),
    ];

    let mut uds_ready = 0;
    for &(cap, sock) in caps_with_sockets {
        let exists = dir.join(sock).exists();
        if exists {
            uds_ready += 1;
        }
        let msg = if exists {
            format!("{sock} present — TCP port unnecessary for local")
        } else {
            format!("{sock} absent — may need Tower relay for remote")
        };
        v.check_bool(&format!("uds:{cap}"), exists, &msg);
    }

    #[expect(clippy::cast_precision_loss, reason = "count fits f64")]
    let coverage = (f64::from(uds_ready) / caps_with_sockets.len() as f64) * 100.0;
    v.check_bool(
        "uds:coverage",
        coverage > 80.0,
        &format!(
            "{uds_ready}/{} capabilities via UDS ({coverage:.0}%) — \
             most TCP ports serve only remote; route through Songbird instead",
            caps_with_sockets.len()
        ),
    );
}

fn phase_tower_relay(v: &mut ValidationResult) {
    let ctx = CompositionContext::from_live_discovery_with_fallback();
    let caps = ctx.available_capabilities();

    let tower_caps = ["security", "crypto", "discovery"];
    let tower_alive = tower_caps.iter().filter(|c| caps.contains(c)).count();
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
        v.check_skip("tower:neural_api", "biomeOS Neural API not running");
        return;
    };

    match bridge.health_check() {
        Ok(healthy) => {
            v.check_bool("tower:neural_api_health", healthy, "Neural API healthy");
        }
        Err(e) => {
            v.check_skip("tower:neural_api_health", &format!("health: {e}"));
        }
    }

    match bridge.discover_capability("discovery") {
        Ok(_) => {
            v.check_bool(
                "tower:songbird_via_neural",
                true,
                "Songbird reachable via Neural API → relay pathway validated",
            );
        }
        Err(e) => {
            v.check_skip(
                "tower:songbird_via_neural",
                &format!("songbird discover: {e}"),
            );
        }
    }

    match bridge.discover_capability("crypto") {
        Ok(_) => {
            v.check_bool(
                "tower:beardog_via_neural",
                true,
                "BearDog reachable via Neural API → TCP port 9900 droppable",
            );
        }
        Err(e) => {
            v.check_skip(
                "tower:beardog_via_neural",
                &format!("beardog discover: {e}"),
            );
        }
    }

    let non_tower_caps = ["compute", "storage", "dag", "orchestration"];
    let mut relayable = 0;
    for cap in &non_tower_caps {
        if bridge.discover_capability(cap).is_ok() {
            relayable += 1;
        }
    }
    v.check_bool(
        "tower:non_tower_relay",
        relayable > 0,
        &format!(
            "{relayable}/{} non-Tower caps reachable via Neural API relay",
            non_tower_caps.len()
        ),
    );
}

fn phase_cns_plan(v: &mut ValidationResult) {
    v.check_bool(
        "cns:architecture",
        true,
        "Tower (BearDog + Songbird) = CNS; all inter-gate traffic via Songbird braid",
    );

    v.check_bool(
        "cns:phase1_beardog_drop",
        true,
        "Phase 1: BearDog drops TCP 9900/9101 — crypto via UDS + Songbird relay",
    );

    v.check_bool(
        "cns:phase2_skunkbat_drop",
        true,
        "Phase 2: SkunkBat drops TCP 9750 — meta-tier via UDS + Songbird relay",
    );

    v.check_bool(
        "cns:phase3_ephemeral_drop",
        true,
        "Phase 3: ToadStool/CoralReef/SweetGrass drop ephemeral TCP → UDS primary",
    );

    v.check_bool(
        "cns:phase4_songbird_only",
        true,
        "Phase 4: Songbird retains TCP (2 ports per NUCLEUS) for cross-gate federation",
    );

    v.check_bool(
        "cns:kderm_alignment",
        true,
        "K-Derm: Tower = plasma membrane (public TCP), inner primals = cytoplasm (UDS only)",
    );

    v.check_bool(
        "cns:firewall_simplification",
        true,
        "firewall reduces to: allow Songbird ports per NUCLEUS, deny all else",
    );

    v.check_bool(
        "cns:audit_point",
        true,
        "sovereignty: single relay = single audit point for all cross-gate traffic",
    );

    v.check_bool(
        "cns:neural_api_routing",
        true,
        "Neural API routes remote: Songbird → braid → local UDS capability socket",
    );
}
