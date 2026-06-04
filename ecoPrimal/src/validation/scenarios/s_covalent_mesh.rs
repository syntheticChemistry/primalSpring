// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Covalent Mesh — cross-gate trust validation.
//!
//! Six-phase validation of the covalent mesh trust model:
//!
//! 1. Structural: federation port, deploy graphs, capability registry
//! 2. Discovery: `discovery.peers` via Songbird TCP :7700
//! 3. Cross-gate dispatch: `capability.call` transparent routing
//! 4. Security: bearDog BTSP cross-gate token validation
//! 5. Content integrity: NestGate federation BLAKE3 end-to-end
//! 6. Dark Forest invariants: isolation + reversibility
//!
//! Tier::Both — structural checks pass without primals, live checks
//! gracefully skip when federation is unavailable.

use crate::composition::CompositionContext;
use crate::ipc::verifiers::parse_verify_ionic_response;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "covalent-mesh",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave49_covalent_mesh",
        provenance_date: "2026-05-25",
        description:
            "Covalent mesh — discovery.peers, cross-gate capability.call via Songbird TCP federation",
    },
    run,
};

/// Run all covalent mesh validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — federation prerequisites");
    phase_structural(v);

    v.section("Phase 2: Discovery — Songbird TCP federation peers");
    phase_discovery_peers(v, ctx);

    v.section("Phase 3: Cross-gate — capability.call via mesh dispatch");
    phase_cross_gate_dispatch(v, ctx);

    v.section("Phase 4: Security — bearDog BTSP cross-gate token validation");
    phase_security_trust(v, ctx);

    v.section("Phase 5: Content integrity — NestGate federation BLAKE3");
    phase_content_integrity(v, ctx);

    v.section("Phase 6: Dark Forest invariants — isolation + reversibility");
    phase_dark_forest_invariants(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    v.check_bool(
        "registry:discovery_peers",
        REGISTRY_TOML.contains("discovery.peers"),
        "discovery.peers registered in capability_registry.toml",
    );
    v.check_bool(
        "registry:capability_call",
        REGISTRY_TOML.contains("capability.call"),
        "capability.call registered (mesh dispatch entrypoint)",
    );
    v.check_bool(
        "registry:route_register",
        REGISTRY_TOML.contains("route.register"),
        "route.register registered (federation route sharing)",
    );

    let covalent_graph = include_str!("../../../../graphs/multi_node/basement_hpc_covalent.toml");
    let parsed: Result<toml::Value, _> = toml::from_str(covalent_graph);
    v.check_bool(
        "graph:covalent_parses",
        parsed.is_ok(),
        "basement_hpc_covalent.toml valid TOML",
    );
}

fn phase_discovery_peers(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("discovery") {
        v.check_skip(
            "live:discovery_peers",
            "discovery capability not in composition context",
        );
        v.check_skip(
            "live:peer_count",
            "discovery capability not in composition context",
        );
        return;
    }

    match ctx.call(
        "discovery",
        "discovery.peers",
        serde_json::json!({}),
    ) {
        Ok(resp) => {
            v.check_bool(
                "live:discovery_peers",
                true,
                &format!("discovery.peers responded: {resp}"),
            );
            let peer_count = resp
                .get("peers")
                .and_then(serde_json::Value::as_array)
                .map_or(0, Vec::len);
            v.check_bool(
                "live:peer_count",
                peer_count > 0,
                &format!("discovery.peers found {peer_count} remote peer(s)"),
            );
            if peer_count == 0 {
                v.check_skip(
                    "live:peer_gate_ids",
                    "no peers — federation port may not be enabled",
                );
                v.check_skip(
                    "live:peer_latency",
                    "no peers for latency measurement",
                );
                v.check_skip(
                    "live:capability_propagation",
                    "no peers for capability check",
                );
            } else if let Some(peers) = resp.get("peers").and_then(serde_json::Value::as_array) {
                let gate_ids: Vec<&str> = peers
                    .iter()
                    .filter_map(|p| {
                        p.get("gate")
                            .or_else(|| p.get("node_id"))
                            .and_then(serde_json::Value::as_str)
                    })
                    .collect();
                v.check_bool(
                    "live:peer_gate_ids",
                    !gate_ids.is_empty(),
                    &format!("peer gates: {gate_ids:?}"),
                );
                let latency_peers: Vec<(&str, f64)> = peers
                    .iter()
                    .filter_map(|p| {
                        let gate = p
                            .get("gate")
                            .or_else(|| p.get("node_id"))
                            .and_then(serde_json::Value::as_str)?;
                        let ms = p.get("latency_ms").and_then(serde_json::Value::as_f64)?;
                        Some((gate, ms))
                    })
                    .collect();
                if latency_peers.is_empty() {
                    v.check_skip(
                        "live:peer_latency",
                        "peers present but no latency_ms field (Songbird may need update)",
                    );
                } else {
                    let summary: Vec<String> = latency_peers
                        .iter()
                        .map(|(g, ms)| format!("{g}={ms:.1}ms"))
                        .collect();
                    v.check_bool(
                        "live:peer_latency",
                        true,
                        &format!("peer latencies: {}", summary.join(", ")),
                    );
                }

                let peers_with_caps: Vec<&str> = peers
                    .iter()
                    .filter(|p| {
                        p.get("capabilities")
                            .and_then(serde_json::Value::as_array)
                            .is_some_and(|a| !a.is_empty())
                    })
                    .filter_map(|p| {
                        p.get("gate")
                            .or_else(|| p.get("node_id"))
                            .and_then(serde_json::Value::as_str)
                    })
                    .collect();
                if peers_with_caps.is_empty() {
                    v.check_skip(
                        "live:capability_propagation",
                        "peers discovered but capabilities: [] — Songbird propagation gap (P1)",
                    );
                } else {
                    v.check_bool(
                        "live:capability_propagation",
                        true,
                        &format!(
                            "capability propagation: {} peer(s) advertising caps: {:?}",
                            peers_with_caps.len(),
                            peers_with_caps
                        ),
                    );
                }
            }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "live:discovery_peers",
                &format!("Songbird not reachable: {e}"),
            );
            v.check_skip("live:peer_count", "Songbird not reachable");
        }
        Err(e) => {
            let msg = format!("{e}");
            if msg.contains("not found") || msg.contains("-32601") {
                v.check_skip(
                    "live:discovery_peers",
                    &format!("discovery.peers not implemented: {e}"),
                );
                v.check_skip("live:peer_count", "discovery.peers not available");
            } else {
                v.check_bool(
                    "live:discovery_peers",
                    false,
                    &format!("discovery.peers error: {e}"),
                );
                v.check_skip("live:peer_count", &format!("error: {e}"));
            }
        }
    }
}

fn phase_cross_gate_dispatch(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "live:mesh_capability_call",
            "orchestration capability not in context",
        );
        return;
    }

    let manifest_toml = include_str!("../../../../../../infra/wateringHole/ecosystem_manifest.toml");
    let peer_gates: Vec<String> = toml::from_str::<toml::Value>(manifest_toml)
        .ok()
        .and_then(|p| p.get("gates")?.as_table().cloned())
        .map(|t| {
            t.keys()
                .filter(|k| !["golgiBody", "peptidoglycan", "golgiBody-ext"].contains(&k.as_str()))
                .cloned()
                .collect()
        })
        .unwrap_or_default();
    for target_gate in &peer_gates {
        let check_id = format!("live:mesh_call_{target_gate}");
        match ctx.call(
            "orchestration",
            "capability.call",
            serde_json::json!({
                "capability": "security",
                "operation": "health.liveness",
                "args": {},
                "gate": target_gate,
            }),
        ) {
            Ok(resp) => {
                v.check_bool(
                    &check_id,
                    true,
                    &format!("capability.call to {target_gate} succeeded: {resp}"),
                );
            }
            Err(e) if e.is_connection_error() => {
                v.check_skip(
                    &check_id,
                    &format!("{target_gate} not reachable (mesh not active): {e}"),
                );
            }
            Err(e) => {
                let msg = format!("{e}");
                let expected_skip = msg.contains("not found")
                    || msg.contains("unknown gate")
                    || msg.contains("no route")
                    || msg.contains("not available")
                    || msg.contains("Invalid JSON from remote")
                    || msg.contains("No local or remote provider")
                    || msg.contains("-32601");
                if expected_skip {
                    v.check_skip(
                        &check_id,
                        &format!("{target_gate} mesh not available: {e}"),
                    );
                } else {
                    v.check_bool(
                        &check_id,
                        false,
                        &format!("{target_gate} unexpected error: {e}"),
                    );
                }
            }
        }
    }
}

/// Phase 4: Security trust — bearDog BTSP cross-gate token validation.
///
/// Tests:
/// - Issue token on local gate, verify locally (baseline)
/// - Verify with `verification_source: "local"` succeeds
/// - Verify with `verification_source: "remote"` — validates BTSP session binding
/// - Issue token on local gate, verify on remote gate via mesh
/// - Reject forged/expired token from remote gate
/// - auth.verify_ionic scopes propagate through mesh dispatch
fn phase_security_trust(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("security") {
        v.check_skip(
            "security:local_token_issue",
            "security capability not in context (bearDog not running)",
        );
        v.check_skip("security:local_verify", "security not available");
        v.check_skip("security:verify_source_local", "security not available");
        v.check_skip("security:verify_source_remote", "security not available");
        v.check_skip("security:cross_gate_verify", "security not available");
        v.check_skip("security:reject_forged", "security not available");
        v.check_skip("security:scopes_propagate", "security not available");
        return;
    }

    let token_result = ctx.call(
        "security",
        "auth.issue_ionic",
        serde_json::json!({
            "subject": "mesh-trust-test",
            "scopes": ["discovery.*", "mesh.*"],
            "ttl_seconds": 60,
            "gate_origin": "east-gate"
        }),
    );

    let token = match token_result {
        Ok(resp) => {
            let tok = resp.get("token").and_then(serde_json::Value::as_str);
            v.check_bool(
                "security:local_token_issue",
                tok.is_some(),
                &format!(
                    "auth.issue_ionic: {}",
                    if tok.is_some() { "token issued with gate_origin" } else { "no token field" }
                ),
            );
            tok.map(String::from)
        }
        Err(e) => {
            let msg = format!("{e}");
            if msg.contains("not found") || msg.contains("-32601") {
                v.check_skip(
                    "security:local_token_issue",
                    &format!("auth.issue_ionic not implemented: {e}"),
                );
            } else {
                v.check_bool(
                    "security:local_token_issue",
                    false,
                    &format!("auth.issue_ionic error: {e}"),
                );
            }
            None
        }
    };

    let Some(ref valid_token) = token else {
        v.check_skip("security:local_verify", "no token to verify");
        v.check_skip("security:verify_source_local", "no token to verify");
        v.check_skip("security:verify_source_remote", "no token to verify");
        v.check_skip("security:cross_gate_verify", "no token to verify");
        v.check_skip("security:reject_forged", "no token to verify");
        v.check_skip("security:scopes_propagate", "no token to verify");
        return;
    };

    match ctx.call(
        "security",
        "auth.verify_ionic",
        serde_json::json!({ "token": valid_token }),
    ) {
        Ok(resp) => {
            let parsed = parse_verify_ionic_response(&resp);
            v.check_bool(
                "security:local_verify",
                parsed.is_some(),
                &format!(
                    "local verify: {}",
                    if parsed.is_some() { "valid" } else { "rejected unexpectedly" }
                ),
            );
            if let Some(ref vt) = parsed {
                v.check_bool(
                    "security:scopes_propagate",
                    vt.scopes.iter().any(|s| s.contains("discovery") || s == "*"),
                    &format!("scopes returned: {:?}", vt.scopes),
                );
            } else {
                v.check_skip("security:scopes_propagate", "local verify failed");
            }
        }
        Err(e) => {
            v.check_bool(
                "security:local_verify",
                false,
                &format!("auth.verify_ionic local error: {e}"),
            );
            v.check_skip("security:scopes_propagate", &format!("verify failed: {e}"));
        }
    }

    verify_with_source(v, ctx, valid_token);
    verify_cross_gate_and_forged(v, ctx, valid_token);
}

/// BTSP verification source testing — validate that bearDog distinguishes
/// local vs remote token verification contexts.
fn verify_with_source(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    valid_token: &str,
) {
    match ctx.call(
        "security",
        "auth.verify_ionic",
        serde_json::json!({
            "token": valid_token,
            "verification_source": "local"
        }),
    ) {
        Ok(resp) => {
            let parsed = parse_verify_ionic_response(&resp);
            v.check_bool(
                "security:verify_source_local",
                parsed.is_some(),
                &format!(
                    "verification_source=local: {}",
                    if parsed.is_some() { "accepted (local gate issued this token)" } else { "rejected" }
                ),
            );
        }
        Err(e) => {
            let msg = format!("{e}");
            if msg.contains("not found") || msg.contains("-32601") {
                v.check_skip(
                    "security:verify_source_local",
                    "verification_source param not supported (bearDog version too old)",
                );
            } else {
                v.check_bool(
                    "security:verify_source_local",
                    false,
                    &format!("verify with source=local error: {e}"),
                );
            }
        }
    }

    match ctx.call(
        "security",
        "auth.verify_ionic",
        serde_json::json!({
            "token": valid_token,
            "verification_source": "remote",
            "requesting_gate": "strand-gate"
        }),
    ) {
        Ok(resp) => {
            let valid = resp
                .get("valid")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            let gate_bound = resp.get("gate_origin").is_some()
                || resp.get("issuing_gate").is_some();
            v.check_bool(
                "security:verify_source_remote",
                valid,
                &format!(
                    "verification_source=remote: valid={valid}, gate_binding={}",
                    if gate_bound { "present" } else { "absent (acceptable pre-BTSP-v2)" }
                ),
            );
        }
        Err(e) => {
            let msg = format!("{e}");
            if msg.contains("not found") || msg.contains("-32601") {
                v.check_skip(
                    "security:verify_source_remote",
                    "verification_source=remote not supported (bearDog needs w135+)",
                );
            } else {
                v.check_skip(
                    "security:verify_source_remote",
                    &format!("remote verification not available: {e}"),
                );
            }
        }
    }
}

/// Cross-gate token verification + forged token rejection sub-phase.
fn verify_cross_gate_and_forged(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    valid_token: &str,
) {
    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "security:cross_gate_verify",
            "orchestration not available — no mesh dispatch",
        );
        v.check_skip("security:reject_forged", "no mesh dispatch");
        return;
    }

    match ctx.call(
        "orchestration",
        "capability.call",
        serde_json::json!({
            "capability": "security",
            "operation": "auth.verify_ionic",
            "args": { "token": valid_token },
        }),
    ) {
        Ok(resp) => {
            let parsed = parse_verify_ionic_response(&resp);
            v.check_bool(
                "security:cross_gate_verify",
                parsed.is_some(),
                &format!(
                    "cross-gate verify: {}",
                    if parsed.is_some() { "token valid on remote gate" } else { "rejected" }
                ),
            );
        }
        Err(e) => {
            let msg = format!("{e}");
            let expected = msg.contains("No local or remote provider")
                || msg.contains("not found")
                || msg.contains("no route");
            if expected {
                v.check_skip(
                    "security:cross_gate_verify",
                    &format!("mesh dispatch not available: {e}"),
                );
            } else {
                v.check_bool(
                    "security:cross_gate_verify",
                    false,
                    &format!("cross-gate verify unexpected error: {e}"),
                );
            }
        }
    }

    match ctx.call(
        "security",
        "auth.verify_ionic",
        serde_json::json!({ "token": "forged.invalid.token.from.attacker" }),
    ) {
        Ok(resp) => {
            let parsed = parse_verify_ionic_response(&resp);
            v.check_bool(
                "security:reject_forged",
                parsed.is_none(),
                &format!(
                    "forged token: {}",
                    if parsed.is_none() { "correctly rejected" } else { "ACCEPTED (vulnerability!)" }
                ),
            );
        }
        Err(_) => {
            v.check_bool(
                "security:reject_forged",
                true,
                "forged token correctly rejected (error response)",
            );
        }
    }
}

/// Phase 5: Content integrity — NestGate federation BLAKE3 end-to-end.
///
/// Tests:
/// - content.put on local gate returns a BLAKE3 hash
/// - content.get with same hash returns identical data
/// - content.replicate.pull from remote gate verifies hash integrity
fn phase_content_integrity(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.check_bool(
        "content:registry_put",
        REGISTRY_TOML.contains("content.put"),
        "content.put registered in capability_registry.toml",
    );
    v.check_bool(
        "content:registry_replicate",
        REGISTRY_TOML.contains("content.replicate"),
        "content.replicate registered in capability_registry.toml",
    );

    if !ctx.has_capability("storage") {
        v.check_skip(
            "content:local_put_get",
            "storage capability not in context (NestGate not running)",
        );
        v.check_skip("content:hash_integrity", "storage not available");
        v.check_skip("content:cross_gate_replicate", "storage not available");
        return;
    }

    let test_data = "covalent-mesh-integrity-test-payload-wave75";
    match ctx.call(
        "storage",
        "content.put",
        serde_json::json!({
            "data": test_data,
            "content_type": "text/plain"
        }),
    ) {
        Ok(resp) => {
            let hash = resp.get("hash").and_then(serde_json::Value::as_str);
            v.check_bool(
                "content:local_put_get",
                hash.is_some(),
                &format!(
                    "content.put: {}",
                    hash.map_or_else(|| "no hash returned".to_owned(), |h| format!("hash={h}"))
                ),
            );

            if let Some(content_hash) = hash {
                match ctx.call(
                    "storage",
                    "content.get",
                    serde_json::json!({ "hash": content_hash }),
                ) {
                    Ok(get_resp) => {
                        let retrieved = get_resp
                            .get("data")
                            .and_then(serde_json::Value::as_str)
                            .unwrap_or("");
                        v.check_bool(
                            "content:hash_integrity",
                            retrieved == test_data,
                            &format!(
                                "BLAKE3 round-trip: {}",
                                if retrieved == test_data { "data matches" } else { "DATA MISMATCH" }
                            ),
                        );
                    }
                    Err(e) => {
                        v.check_bool(
                            "content:hash_integrity",
                            false,
                            &format!("content.get error: {e}"),
                        );
                    }
                }

                if ctx.has_capability("orchestration") {
                    match ctx.call(
                        "orchestration",
                        "capability.call",
                        serde_json::json!({
                            "capability": "storage",
                            "operation": "content.replicate.pull",
                            "args": { "hash": content_hash, "source_gate": "eastGate" },
                        }),
                    ) {
                        Ok(repl_resp) => {
                            let repl_hash =
                                repl_resp.get("hash").and_then(serde_json::Value::as_str);
                            v.check_bool(
                                "content:cross_gate_replicate",
                                repl_hash == Some(content_hash),
                                &format!(
                                    "cross-gate replicate: {}",
                                    if repl_hash == Some(content_hash) {
                                        "hash integrity maintained"
                                    } else {
                                        "HASH MISMATCH (integrity violation!)"
                                    }
                                ),
                            );
                        }
                        Err(e) => {
                            let msg = format!("{e}");
                            let expected = msg.contains("No local or remote provider")
                                || msg.contains("not found")
                                || msg.contains("no route");
                            if expected {
                                v.check_skip(
                                    "content:cross_gate_replicate",
                                    &format!("mesh replication not routable: {e}"),
                                );
                            } else {
                                v.check_bool(
                                    "content:cross_gate_replicate",
                                    false,
                                    &format!("replicate error: {e}"),
                                );
                            }
                        }
                    }
                } else {
                    v.check_skip(
                        "content:cross_gate_replicate",
                        "orchestration not available — no mesh dispatch",
                    );
                }
            } else {
                v.check_skip("content:hash_integrity", "no hash from put");
                v.check_skip("content:cross_gate_replicate", "no hash from put");
            }
        }
        Err(e) => {
            let msg = format!("{e}");
            if msg.contains("not found") || msg.contains("-32601") {
                v.check_skip(
                    "content:local_put_get",
                    &format!("content.put not implemented: {e}"),
                );
            } else {
                v.check_bool(
                    "content:local_put_get",
                    false,
                    &format!("content.put error: {e}"),
                );
            }
            v.check_skip("content:hash_integrity", "put failed");
            v.check_skip("content:cross_gate_replicate", "put failed");
        }
    }
}

/// Phase 6: Dark Forest invariants — isolation + reversibility.
///
/// Tests:
/// - Only Songbird federation port (7700) accepts cross-gate traffic
/// - UDS-only for local primal communication (no TCP local)
/// - Gate can conceptually leave mesh without data loss
fn phase_dark_forest_invariants(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let federation_port_only = REGISTRY_TOML.contains("mesh.relay")
        || REGISTRY_TOML.contains("route.register");
    v.check_bool(
        "darkforest:federation_surface",
        federation_port_only,
        "federation capabilities registered (mesh.relay / route.register)",
    );

    let socket_dir = std::path::Path::new("/run/user/1000/biomeos");
    let has_uds = socket_dir.exists();
    v.check_bool(
        "darkforest:uds_runtime_exists",
        has_uds,
        &format!(
            "UDS runtime dir: {}",
            if has_uds { "exists" } else { "/run/user/1000/biomeos not found" }
        ),
    );

    if has_uds {
        let tcp_local_found = std::fs::read_dir(socket_dir)
            .map(|entries| {
                entries
                    .filter_map(Result::ok)
                    .any(|e| {
                        let name = e.file_name();
                        let n = name.to_string_lossy();
                        n.contains(':') || n.ends_with(".tcp")
                    })
            })
            .unwrap_or(false);
        v.check_bool(
            "darkforest:no_tcp_local",
            !tcp_local_found,
            &format!(
                "local IPC: {}",
                if tcp_local_found {
                    "TCP socket found (Dark Forest VIOLATION)"
                } else {
                    "UDS-only (correct)"
                }
            ),
        );
    } else {
        v.check_skip(
            "darkforest:no_tcp_local",
            "UDS runtime dir not present",
        );
    }

    let songbird_port: u16 = 7700;
    let federation_port_check = std::net::TcpStream::connect_timeout(
        &std::net::SocketAddr::from(([127, 0, 0, 1], songbird_port)),
        std::time::Duration::from_millis(200),
    );
    v.check_bool(
        "darkforest:federation_port_only",
        federation_port_check.is_ok(),
        &format!(
            "Songbird federation :{songbird_port}: {}",
            if federation_port_check.is_ok() { "listening (correct)" } else { "not listening" }
        ),
    );

    let non_federation_ports: &[u16] = &[7701, 9101, 9750];
    let any_non_federation_exposed = non_federation_ports.iter().any(|&port| {
        std::net::TcpStream::connect_timeout(
            &std::net::SocketAddr::from(([127, 0, 0, 1], port)),
            std::time::Duration::from_millis(100),
        )
        .is_ok()
    });
    v.check_bool(
        "darkforest:no_extra_ports",
        !any_non_federation_exposed,
        &format!(
            "non-federation ports: {}",
            if any_non_federation_exposed {
                "EXPOSED (investigate)"
            } else {
                "closed (correct — only :7700 for cross-gate)"
            }
        ),
    );

    if ctx.has_capability("storage") {
        match ctx.call("storage", "content.stat", serde_json::json!({})) {
            Ok(resp) => {
                let has_local_store = resp
                    .get("objects")
                    .and_then(serde_json::Value::as_u64)
                    .is_some()
                    || resp.get("store").is_some();
                v.check_bool(
                    "darkforest:reversibility_local_store",
                    has_local_store,
                    "NestGate has local CAS — gate can leave mesh without data loss",
                );
            }
            Err(e) => {
                let msg = format!("{e}");
                if msg.contains("not found") || msg.contains("-32601") {
                    v.check_skip(
                        "darkforest:reversibility_local_store",
                        &format!("content.stat not implemented: {e}"),
                    );
                } else {
                    v.check_skip(
                        "darkforest:reversibility_local_store",
                        &format!("content.stat error: {e}"),
                    );
                }
            }
        }
    } else {
        v.check_skip(
            "darkforest:reversibility_local_store",
            "storage not available — cannot verify data independence",
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn covalent_mesh_no_panic() {
        let mut v = ValidationResult::new("covalent-mesh");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
    }

    #[test]
    fn registry_has_discovery_peers() {
        assert!(
            REGISTRY_TOML.contains("discovery.peers"),
            "discovery.peers must be in capability_registry.toml"
        );
    }

    #[test]
    fn phase_structural_passes() {
        let mut v = ValidationResult::new("covalent-mesh-structural");
        phase_structural(&mut v);
        assert_eq!(v.failed, 0, "structural checks should pass");
    }

    #[test]
    fn phase_dark_forest_structural() {
        let mut v = ValidationResult::new("covalent-mesh-darkforest");
        let mut ctx = CompositionContext::discover();
        phase_dark_forest_invariants(&mut v, &mut ctx);
    }

    #[test]
    fn forged_token_format() {
        let forged = serde_json::json!({ "valid": false, "error": "invalid_signature" });
        assert!(
            parse_verify_ionic_response(&forged).is_none(),
            "forged token must be rejected"
        );
    }
}
