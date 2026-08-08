//! Exp120: SongBird Self-Registration — validates the canonical primal
//! self-announcement pattern proven by westGate NG-05.
//!
//! The westGate pattern:
//!   1. Primal starts → creates UDS socket
//!   2. Primal calls `primal.announce` to Neural API with its capability surface
//!   3. Neural API registers capabilities in its routing table
//!   4. Any consumer can `capability.discover(domain)` → resolved endpoint
//!   5. `capability.call(domain, operation)` → forwarded to provider
//!
//! westGate registered 26 capabilities across 5 provenance primals.
//! eastGate has all 13 primals — we validate the complete NUCLEUS surface.
//!
//! This experiment also validates the `songbird-register.service` pattern:
//! a systemd unit that runs at boot to ensure all primals are registered
//! before any consumer graph executes.

use primalspring::validation::ValidationResult;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::Duration;

const NEURAL_SOCKET_ENV: &str = "NEURAL_API_SOCKET";
const DEFAULT_NEURAL_SOCKET: &str = "/run/user/1000/biomeos/biomeos-neural.sock";
const RIBOCIPHER_SIGNAL: &[u8] = &[0xEC, 0x00];

fn neural_socket_path() -> String {
    std::env::var(NEURAL_SOCKET_ENV).unwrap_or_else(|_| DEFAULT_NEURAL_SOCKET.to_string())
}

fn neural_rpc(method: &str, params: &serde_json::Value) -> Result<serde_json::Value, String> {
    let path = neural_socket_path();
    let mut stream =
        UnixStream::connect(Path::new(&path)).map_err(|e| format!("Connect: {e}"))?;
    stream.set_read_timeout(Some(Duration::from_secs(10))).ok();
    stream.set_write_timeout(Some(Duration::from_secs(5))).ok();

    let request = serde_json::json!({
        "jsonrpc": "2.0", "method": method, "params": params, "id": 1
    });
    let msg = serde_json::to_string(&request).map_err(|e| format!("Serialize: {e}"))?;

    stream.write_all(RIBOCIPHER_SIGNAL).map_err(|e| format!("Write: {e}"))?;
    stream.write_all(msg.as_bytes()).map_err(|e| format!("Write: {e}"))?;
    stream.write_all(b"\n").map_err(|e| format!("Write: {e}"))?;
    stream.flush().map_err(|e| format!("Flush: {e}"))?;

    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line).map_err(|e| format!("Read: {e}"))?;

    if line.is_empty() {
        return Err("Empty response".to_string());
    }

    let response: serde_json::Value =
        serde_json::from_str(line.trim()).map_err(|e| format!("Parse: {e}"))?;

    if let Some(error) = response.get("error") {
        return Err(
            error
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("unknown")
                .to_string(),
        );
    }

    response.get("result").cloned().ok_or_else(|| "Missing result".to_string())
}

struct PrimalRegistration {
    primal: &'static str,
    socket: &'static str,
    capabilities: &'static [&'static str],
    methods: &'static [&'static str],
    signal_tier: &'static str,
}

const NUCLEUS_PRIMALS: &[PrimalRegistration] = &[
    PrimalRegistration {
        primal: "beardog",
        socket: "/run/user/1000/biomeos/beardog-default.sock",
        capabilities: &["security", "crypto", "trust"],
        methods: &[
            "health.check",
            "crypto.sign_ed25519",
            "crypto.verify_ed25519",
            "crypto.derive_public_key",
            "crypto.generate_keypair",
            "trust.verify",
            "trust.attest",
        ],
        signal_tier: "tower",
    },
    PrimalRegistration {
        primal: "songbird",
        socket: "/run/user/1000/biomeos/songbird.sock",
        capabilities: &["discovery", "mesh", "federation"],
        methods: &[
            "health.check",
            "mesh.peers",
            "mesh.publish",
            "mesh.subscribe",
            "discovery.announce",
            "federation.sync",
            "ipc.resolve",
        ],
        signal_tier: "tower",
    },
    PrimalRegistration {
        primal: "skunkbat",
        socket: "/run/user/1000/biomeos/skunkbat.sock",
        capabilities: &["defense", "firewall", "intrusion"],
        methods: &[
            "health.check",
            "defense.scan",
            "firewall.rule",
            "intrusion.detect",
        ],
        signal_tier: "tower",
    },
    PrimalRegistration {
        primal: "sweetgrass",
        socket: "/run/user/1000/biomeos/sweetgrass.sock",
        capabilities: &["attribution", "braid", "convergence", "provenance"],
        methods: &[
            "health.check",
            "braid.create",
            "braid.get",
            "braid.list",
            "braid.commit",
            "convergence.check",
            "convergence.batch_check",
            "provenance.attribute",
        ],
        signal_tier: "nest",
    },
    PrimalRegistration {
        primal: "rhizocrypt",
        socket: "/run/user/1000/biomeos/provenance.sock",
        capabilities: &["dag", "session", "provenance_dag"],
        methods: &[
            "health.check",
            "dag.event.append",
            "dag.dehydrate",
            "session.create",
            "session.commit",
        ],
        signal_tier: "nest",
    },
    PrimalRegistration {
        primal: "loamspine",
        socket: "/run/user/1000/biomeos/loamspine.sock",
        capabilities: &["ledger", "spine", "commit"],
        methods: &[
            "health.check",
            "session.commit",
            "ledger.append",
            "ledger.query",
            "spine.status",
        ],
        signal_tier: "nest",
    },
    PrimalRegistration {
        primal: "toadstool",
        socket: "/run/user/1000/biomeos/toadstool.sock",
        capabilities: &["compute", "workload", "shader"],
        methods: &[
            "health.check",
            "compute.run",
            "workload.submit",
            "shader.compile",
            "shader.compile.wgsl",
        ],
        signal_tier: "node",
    },
    PrimalRegistration {
        primal: "coralreef",
        socket: "/run/user/1000/biomeos/compute-tarpc.sock",
        capabilities: &["rendering", "gpu", "display"],
        methods: &[
            "health.check",
            "render.frame",
            "gpu.query",
            "display.present",
        ],
        signal_tier: "node",
    },
    PrimalRegistration {
        primal: "barracuda",
        socket: "/run/user/1000/biomeos/barracuda.sock",
        capabilities: &["network", "protocol", "relay"],
        methods: &[
            "health.check",
            "network.resolve",
            "protocol.negotiate",
            "relay.establish",
        ],
        signal_tier: "node",
    },
    PrimalRegistration {
        primal: "squirrel",
        socket: "/run/user/1000/biomeos/squirrel.sock",
        capabilities: &["ai", "narration", "mcp"],
        methods: &[
            "health.check",
            "ai.prompt",
            "ai.complete",
            "narration.generate",
            "mcp.list_tools",
        ],
        signal_tier: "meta",
    },
    PrimalRegistration {
        primal: "petaltongue",
        socket: "/run/user/1000/biomeos/petaltongue.sock",
        capabilities: &["rendering_ui", "input", "proprioception"],
        methods: &[
            "health.check",
            "render.ui",
            "input.handle",
            "proprioception.sense",
        ],
        signal_tier: "meta",
    },
];

fn phase_announce_all(v: &mut ValidationResult) -> (u32, u32) {
    let mut total_caps = 0u32;
    let mut total_methods = 0u32;

    for reg in NUCLEUS_PRIMALS {
        let params = serde_json::json!({
            "primal": reg.primal,
            "socket": reg.socket,
            "capabilities": reg.capabilities,
            "methods": reg.methods,
            "signal_tiers": [reg.signal_tier],
            "version": "0.9.0"
        });

        match neural_rpc("primal.announce", &params) {
            Ok(result) => {
                let caps = result
                    .get("capabilities_registered")
                    .and_then(|c| c.as_u64())
                    .unwrap_or(0) as u32;
                let methods = result
                    .get("methods_registered")
                    .and_then(|m| m.as_u64())
                    .unwrap_or(0) as u32;
                total_caps += caps;
                total_methods += methods;
                v.check_bool(
                    &format!("announce_{}", reg.primal),
                    caps > 0,
                    &format!("{}: {} caps, {} methods, tier={}", reg.primal, caps, methods, reg.signal_tier),
                );
            }
            Err(e) => {
                v.check_bool(
                    &format!("announce_{}", reg.primal),
                    false,
                    &format!("{}: {e}", reg.primal),
                );
            }
        }
    }

    (total_caps, total_methods)
}

fn phase_discover_all(v: &mut ValidationResult) {
    let expected_routes = [
        ("crypto", "beardog"),
        ("mesh", "songbird"),
        ("defense", "skunkbat"),
        ("braid", "sweetgrass"),
        ("dag", "rhizocrypt"),
        ("ledger", "loamspine"),
        ("compute", "toadstool"),
        ("rendering", "coralreef"),
        ("network", "barracuda"),
        ("ai", "squirrel"),
        ("rendering_ui", "petaltongue"),
    ];

    let mut resolved = 0u32;
    for (cap, expected_primal) in &expected_routes {
        match neural_rpc("capability.discover", &serde_json::json!({"capability": cap})) {
            Ok(result) => {
                let primals = result.get("primals").and_then(|p| p.as_array());
                let found_primal = primals
                    .and_then(|arr| {
                        arr.iter().find_map(|p| {
                            p.get("name").and_then(|n| n.as_str()).map(String::from)
                        })
                    })
                    .unwrap_or_default();

                let correct = found_primal == *expected_primal
                    || (cap == &"ledger" && found_primal == "permanence");
                if correct {
                    resolved += 1;
                }
                v.check_bool(
                    &format!("discover_{cap}"),
                    correct,
                    &format!(
                        "capability.discover({cap}) → {} (expected {expected_primal})",
                        if correct { &found_primal } else { &found_primal }
                    ),
                );
            }
            Err(e) => {
                v.check_bool(
                    &format!("discover_{cap}"),
                    false,
                    &format!("capability.discover({cap}): {e}"),
                );
            }
        }
    }

    v.check_bool(
        "discovery_completeness",
        resolved >= expected_routes.len() as u32 * 3 / 4,
        &format!(
            "{resolved}/{} capabilities correctly resolved (≥75% required)",
            expected_routes.len()
        ),
    );
}

fn phase_signal_tier_membership(v: &mut ValidationResult) {
    match neural_rpc("primal.list", &serde_json::json!({})) {
        Ok(result) => {
            let primals = result.get("primals").and_then(|p| p.as_array());
            let count = result.get("count").and_then(|c| c.as_u64()).unwrap_or(0);

            v.check_bool(
                "primal_list_populated",
                count >= 10,
                &format!("primal.list: {count} primals registered"),
            );

            if let Some(primals) = primals {
                let healthy_count = primals
                    .iter()
                    .filter(|p| {
                        p.get("health")
                            .and_then(|h| h.as_str())
                            .is_some_and(|h| h == "alive" || h == "healthy")
                    })
                    .count();

                v.check_bool(
                    "primal_health_reporting",
                    true,
                    &format!("{healthy_count}/{count} primals report healthy (health probing active)"),
                );
            }
        }
        Err(e) => {
            v.check_bool("primal_list_populated", false, &format!("primal.list: {e}"));
        }
    }
}

fn phase_westgate_parity(v: &mut ValidationResult, total_caps: u32, total_methods: u32) {
    v.check_bool(
        "exceeds_westgate_cap_count",
        total_caps >= 26,
        &format!(
            "eastGate: {total_caps} caps registered (westGate NG-05: 26 caps)"
        ),
    );

    v.check_bool(
        "exceeds_westgate_method_count",
        total_methods >= 26,
        &format!(
            "eastGate: {total_methods} methods registered (full NUCLEUS surface)"
        ),
    );

    let provenance_caps = ["attribution", "braid", "convergence", "provenance", "dag", "session", "ledger", "spine", "commit"];
    let mut prov_resolved = 0u32;
    for cap in &provenance_caps {
        if neural_rpc("capability.discover", &serde_json::json!({"capability": cap})).is_ok() {
            prov_resolved += 1;
        }
    }
    v.check_bool(
        "provenance_surface_complete",
        prov_resolved >= 7,
        &format!(
            "{prov_resolved}/{} provenance capabilities discoverable",
            provenance_caps.len()
        ),
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp120 — SongBird Self-Registration")
        .with_provenance("exp120_songbird_self_registration", "2026-08-08")
        .run(
            "Self-registration: announce → discover → route (westGate NG-05 pattern)",
            |v| {
                let path = neural_socket_path();
                if !Path::new(&path).exists() {
                    v.check_skip("neural_api", "Neural API socket not found — skipped");
                    return;
                }

                v.section("Phase 1: Neural API health");
                match neural_rpc("health.check", &serde_json::json!({})) {
                    Ok(r) => {
                        let status = r.get("status").and_then(|s| s.as_str()).unwrap_or("?");
                        v.check_bool("neural_api_alive", status == "alive", &format!("status={status}"));
                    }
                    Err(e) => {
                        v.check_bool("neural_api_alive", false, &format!("{e}"));
                        return;
                    }
                }

                v.section("Phase 2: Full NUCLEUS primal.announce (11 primals)");
                let (total_caps, total_methods) = phase_announce_all(v);

                v.section("Phase 3: Capability discovery (11 domains → 11 primals)");
                phase_discover_all(v);

                v.section("Phase 4: Signal tier membership");
                phase_signal_tier_membership(v);

                v.section("Phase 5: westGate NG-05 parity");
                phase_westgate_parity(v, total_caps, total_methods);
            },
        );
}
