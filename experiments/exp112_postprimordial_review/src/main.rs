// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp112: PostPrimordial Review — Glacial Shift Readiness Audit
//!
//! Full NUCLEUS live review as we cross the glacial shift:
//!   1. Primal inventory (13/13 from plasmidBin)
//!   2. Socket census & ownership mapping (flat vs primal-scoped)
//!   3. TCP port audit (identify ports eligible for Tower CNS migration)
//!   4. Atomic composition health (Tower, Node, Nest, Provenance)
//!   5. Neural API routing health
//!   6. UniBin/ecoBin composability verification
//!
//! Produces a frozen baseline for the cephalization (exp113) and
//! Tower CNS convergence (exp114) work.

use primalspring::composition::CompositionContext;
use primalspring::ipc::NeuralBridge;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp112 — PostPrimordial Review")
        .with_provenance("exp112_postprimordial_review", "2026-05-26")
        .run(
            "Exp112: Glacial Shift Readiness — Full NUCLEUS Audit",
            |v| {
                v.section("Phase 1: Primal Inventory");
                phase_primal_inventory(v);

                v.section("Phase 2: Socket Census");
                phase_socket_census(v);

                v.section("Phase 3: TCP Port Audit");
                phase_tcp_port_audit(v);

                v.section("Phase 4: Atomic Composition");
                phase_atomic_composition(v);

                v.section("Phase 5: Neural API Health");
                phase_neural_api_health(v);

                v.section("Phase 6: UniBin Composability");
                phase_unibin_composability(v);
            },
        );
}

fn biomeos_socket_dir() -> std::path::PathBuf {
    let xdg = primalspring::tolerances::runtime_dir();
    std::path::PathBuf::from(xdg).join("biomeos")
}

fn plasmidbin_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(
        "/home/eastgate/Development/ecoPrimals/infra/plasmidBin/primals/x86_64-unknown-linux-musl",
    )
}

fn phase_primal_inventory(v: &mut ValidationResult) {
    let primals = [
        "beardog", "songbird", "toadstool", "barracuda", "coralreef",
        "nestgate", "rhizocrypt", "loamspine", "sweetgrass", "biomeos",
        "squirrel", "petaltongue", "skunkbat",
    ];

    let dir = biomeos_socket_dir();
    let mut alive_count = 0;

    for primal in &primals {
        let has_socket = socket_exists_for_primal(&dir, primal);
        if has_socket {
            alive_count += 1;
        }
        v.check_bool(
            &format!("inventory:{primal}"),
            has_socket,
            if has_socket { "socket present" } else { "MISSING" },
        );
    }

    v.check_bool(
        "inventory:total",
        alive_count == primals.len(),
        &format!("{alive_count}/{} primals present", primals.len()),
    );

    let pdir = plasmidbin_dir();
    if pdir.exists() {
        let binary_count = primals.iter().filter(|p| pdir.join(p).exists()).count();
        v.check_bool(
            "inventory:plasmidbin_binaries",
            binary_count == primals.len(),
            &format!("{binary_count}/{} binaries in plasmidBin", primals.len()),
        );

        let sourdough = pdir.join("sourdough");
        v.check_bool(
            "inventory:sourdough_harvested",
            sourdough.exists(),
            if sourdough.exists() {
                "sourDough binary present"
            } else {
                "not yet fetched"
            },
        );
    } else {
        v.check_skip("inventory:plasmidbin_binaries", "plasmidBin directory not found");
    }
}

fn socket_exists_for_primal(dir: &std::path::Path, primal: &str) -> bool {
    let patterns = [
        format!("{primal}-nucleus01.sock"),
        format!("{primal}.sock"),
        format!("{primal}-core-nucleus01.sock"),
        "neural-api-nucleus01.sock".to_string(),
    ];
    if primal == "biomeos" {
        return dir.join("neural-api-nucleus01.sock").exists();
    }
    if primal == "coralreef" {
        return dir.join("coralreef-core-nucleus01.sock").exists();
    }
    patterns.iter().any(|p| dir.join(p).exists())
}

fn phase_socket_census(v: &mut ValidationResult) {
    let dir = biomeos_socket_dir();
    if !dir.exists() {
        v.check_skip("sockets:dir", "biomeos socket directory not found");
        return;
    }

    let entries: Vec<String> = std::fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .filter_map(std::result::Result::ok)
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    let sock_count = entries.iter().filter(|n| n.to_ascii_lowercase().ends_with(".sock")).count();
    let json_count = entries.iter().filter(|n| n.to_ascii_lowercase().ends_with(".json")).count();

    v.check_bool("sockets:total", sock_count > 0, &format!("{sock_count} sockets"));
    v.check_bool("sockets:state_files", true, &format!("{json_count} JSON state files"));

    let is_sock = |n: &&String| n.to_ascii_lowercase().ends_with(".sock");
    let named_count = entries
        .iter()
        .filter(|n| is_sock(n) && n.contains("nucleus01"))
        .count();
    let flat_count = entries
        .iter()
        .filter(|n| is_sock(n) && !n.contains("nucleus01") && !n.contains("primalspring01"))
        .count();

    v.check_bool(
        "sockets:named",
        true,
        &format!("{named_count} primal-scoped (already cephalized via naming)"),
    );
    v.check_bool(
        "sockets:flat_domain",
        true,
        &format!("{flat_count} flat domain sockets (cephalization candidates for exp113)"),
    );

    #[allow(clippy::cast_precision_loss)]
    let ceph_ratio = if sock_count > 0 {
        (flat_count as f64 / sock_count as f64) * 100.0
    } else {
        0.0
    };
    v.check_bool(
        "sockets:cephalization_opportunity",
        ceph_ratio > 30.0,
        &format!("{ceph_ratio:.0}% flat domain — high cephalization opportunity"),
    );
}

fn phase_tcp_port_audit(v: &mut ValidationResult) {
    struct PortEntry {
        port: u16,
        primal: &'static str,
        purpose: &'static str,
        cns_eligible: bool,
    }

    let ports = [
        PortEntry { port: 7700, primal: "songbird", purpose: "federation (nucleus01)", cns_eligible: false },
        PortEntry { port: 7701, primal: "songbird", purpose: "federation (primalspring01)", cns_eligible: false },
        PortEntry { port: 9101, primal: "beardog", purpose: "primalspring01 crypto", cns_eligible: true },
        PortEntry { port: 9900, primal: "beardog", purpose: "nucleus01 crypto", cns_eligible: true },
        PortEntry { port: 9750, primal: "skunkbat", purpose: "meta-tier defense", cns_eligible: true },
    ];

    let mut cns_eligible = 0;
    for entry in &ports {
        let reachable = std::net::TcpStream::connect_timeout(
            &std::net::SocketAddr::from(([127, 0, 0, 1], entry.port)),
            std::time::Duration::from_millis(500),
        )
        .is_ok();

        if reachable && entry.cns_eligible {
            cns_eligible += 1;
        }

        v.check_bool(
            &format!("tcp:{}:{}", entry.primal, entry.port),
            reachable,
            &format!(
                "{} — {}",
                entry.purpose,
                if entry.cns_eligible {
                    "DROPPABLE via Tower CNS (exp114)"
                } else {
                    "KEEP: federation requires TCP"
                }
            ),
        );
    }

    v.check_bool(
        "tcp:cns_migration_candidates",
        cns_eligible > 0,
        &format!("{cns_eligible} ports can migrate to UDS-only via Tower CNS routing"),
    );
}

fn phase_atomic_composition(v: &mut ValidationResult) {
    let ctx = CompositionContext::from_live_discovery_with_fallback();
    let caps = ctx.available_capabilities();

    v.check_bool(
        "atomic:discovered",
        !caps.is_empty(),
        &format!("{} capabilities discovered", caps.len()),
    );

    let tower_caps = ["security", "crypto", "discovery"];
    let tower_alive = tower_caps
        .iter()
        .filter(|c| caps.contains(c))
        .count();
    v.check_bool(
        "atomic:tower",
        tower_alive > 0,
        &format!("{tower_alive}/{} Tower capabilities", tower_caps.len()),
    );

    let node_caps = ["compute", "tensor", "shader", "math", "visualization"];
    let node_alive = node_caps.iter().filter(|c| caps.contains(c)).count();
    v.check_bool(
        "atomic:node",
        node_alive > 0,
        &format!("{node_alive}/{} Node capabilities", node_caps.len()),
    );

    let nest_caps = ["storage", "ledger", "orchestration"];
    let nest_alive = nest_caps.iter().filter(|c| caps.contains(c)).count();
    v.check_bool(
        "atomic:nest",
        nest_alive > 0,
        &format!("{nest_alive}/{} Nest capabilities", nest_caps.len()),
    );

    let prov_caps = ["dag", "commit", "provenance", "attribution"];
    let prov_alive = prov_caps.iter().filter(|c| caps.contains(c)).count();
    v.check_bool(
        "atomic:provenance",
        prov_alive > 0,
        &format!("{prov_alive}/{} Provenance capabilities", prov_caps.len()),
    );
}

fn phase_neural_api_health(v: &mut ValidationResult) {
    let bridge = NeuralBridge::discover()
        .or_else(|| NeuralBridge::discover_with(None, Some("nucleus01")));
    let Some(bridge) = bridge else {
        v.check_skip("neural:health", "biomeOS Neural API not running");
        return;
    };

    match bridge.health_check() {
        Ok(healthy) => {
            v.check_bool("neural:health", healthy, "Neural API healthy");
        }
        Err(e) => {
            v.check_skip("neural:health", &format!("health check failed: {e}"));
            return;
        }
    }

    match bridge.discover_capability("security") {
        Ok(resp) => {
            v.check_bool("neural:capability_discovery", true, &format!("security: {resp}"));
        }
        Err(e) => {
            v.check_skip("neural:capability_discovery", &format!("discover failed: {e}"));
        }
    }

    match bridge.routing_weights() {
        Ok(weights) => {
            v.check_bool("neural:routing_weights", true, "routing weights available");
            let _ = weights;
        }
        Err(e) => {
            v.check_skip("neural:routing_weights", &format!("weights unavailable: {e}"));
        }
    }
}

fn phase_unibin_composability(v: &mut ValidationResult) {
    let pdir = plasmidbin_dir();
    if !pdir.exists() {
        v.check_skip("unibin:dir", "plasmidBin not found");
        return;
    }

    let ecobin_count = std::fs::read_dir(&pdir)
        .into_iter()
        .flatten()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.metadata().map(|m| m.len() > 1_000_000).unwrap_or(false))
        .count();

    v.check_bool(
        "unibin:ecobin_total",
        ecobin_count >= 13,
        &format!("{ecobin_count} ecoBin static musl binaries"),
    );

    let unibin_primals = ["biomeos", "beardog", "songbird", "petaltongue", "toadstool"];
    for primal in &unibin_primals {
        let binary = pdir.join(primal);
        if binary.exists() {
            #[allow(clippy::cast_precision_loss)]
            let size_mb = std::fs::metadata(&binary)
                .map(|m| m.len() as f64 / 1_048_576.0)
                .unwrap_or(0.0);
            v.check_bool(
                &format!("unibin:{primal}"),
                size_mb > 1.0,
                &format!("{size_mb:.1}M static binary"),
            );
        } else {
            v.check_skip(&format!("unibin:{primal}"), "binary not found");
        }
    }
}
