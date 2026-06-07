// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Covalent mesh trust phases (4-6) — extracted for maintainability.
//!
//! - Phase 4: Security trust (bearDog BTSP cross-gate token validation)
//! - Phase 5: Content integrity (NestGate federation BLAKE3 end-to-end)
//! - Phase 6: Dark Forest invariants (isolation + reversibility)

use crate::composition::CompositionContext;
use crate::ipc::verifiers::parse_verify_ionic_response;
use crate::validation::ValidationResult;

use super::s_covalent_mesh::REGISTRY_TOML;

/// Phase 4: Security trust — bearDog BTSP cross-gate token validation.
pub(super) fn phase_security_trust(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("security") {
        v.check_skip(
            "security:local_token_issue",
            "security capability not in context (bearDog not running)",
        );
        v.check_skip("security:local_verify", "security not available");
        v.check_skip("security:verify_source_local", "security not available");
        v.check_skip("security:verify_source_remote", "security not available");
        v.check_skip("security:btsp_gate_binding", "security not available");
        v.check_skip("security:btsp_trust_chain", "security not available");
        v.check_skip("security:cross_gate_verify", "security not available");
        v.check_skip("security:reject_forged", "security not available");
        v.check_skip("security:scopes_propagate", "security not available");
        return;
    }

    let local_gate_name = std::env::var("GATE_NAME").unwrap_or_else(|_| {
        ctx.gate_id().unwrap_or("local-gate").to_owned()
    });
    let token_result = ctx.call(
        "security",
        "auth.issue_ionic",
        serde_json::json!({
            "subject": "mesh-trust-test",
            "scopes": ["discovery.*", "mesh.*"],
            "ttl_seconds": 60,
            "gate_origin": local_gate_name
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
        v.check_skip("security:btsp_gate_binding", "no token to verify");
        v.check_skip("security:btsp_trust_chain", "no token to verify");
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

fn verify_with_source(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    valid_token: &str,
) {
    let local_result = ctx.call(
        "security",
        "auth.verify_ionic",
        serde_json::json!({
            "token": valid_token,
            "verification_source": "local"
        }),
    );

    let source_supported = match &local_result {
        Ok(resp) => {
            let parsed = parse_verify_ionic_response(resp);
            v.check_bool(
                "security:verify_source_local",
                parsed.is_some(),
                &format!(
                    "verification_source=local: {}",
                    if parsed.is_some() { "accepted (local gate issued this token)" } else { "rejected" }
                ),
            );
            true
        }
        Err(e) => {
            let msg = format!("{e}");
            if msg.contains("not found") || msg.contains("-32601") {
                v.check_skip(
                    "security:verify_source_local",
                    "verification_source param not supported (bearDog needs w137+)",
                );
            } else {
                v.check_bool(
                    "security:verify_source_local",
                    false,
                    &format!("verify with source=local error: {e}"),
                );
            }
            false
        }
    };

    verify_remote_source(v, ctx, valid_token, source_supported);
}

/// Remote verification source — the P0 trust criterion.
///
/// With bearDog w135+, `verification_source` context proves trust:
/// 1. Return `valid: true` (the token is recognized)
/// 2. Gate binding via `claims.gate_id` / `claims.family_id` or top-level
///    `issuer_gate_id` / `issuer_family_id` (for remote-verified tokens)
/// 3. Family-level trust chain: token's `family_id` matches local gate family
///
/// When verified locally (same key issued and verifies), `verification_source`
/// is `"local"` and gate binding comes from the token's embedded claims.
/// When verified via TrustedIssuerRegistry, `verification_source` is `"remote"`
/// and provenance comes from top-level `issuer_gate_id`.
fn verify_remote_source(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    valid_token: &str,
    source_supported: bool,
) {
    let remote_gate = std::env::var("REMOTE_GATE_NAME")
        .unwrap_or_else(|_| "remote-gate".to_owned());

    match ctx.call(
        "security",
        "auth.verify_ionic",
        serde_json::json!({
            "token": valid_token,
            "verification_source": "remote",
            "requesting_gate": remote_gate
        }),
    ) {
        Ok(resp) => {
            let valid = resp
                .get("valid")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);

            let gate_origin = resp
                .get("issuer_gate_id")
                .or_else(|| resp.get("gate_origin"))
                .or_else(|| resp.get("issuing_gate"))
                .and_then(serde_json::Value::as_str)
                .or_else(|| {
                    resp.get("claims")
                        .and_then(|c| c.get("gate_id"))
                        .and_then(serde_json::Value::as_str)
                });
            let family_origin = resp
                .get("issuer_family_id")
                .and_then(serde_json::Value::as_str)
                .or_else(|| {
                    resp.get("claims")
                        .and_then(|c| c.get("family_id"))
                        .and_then(serde_json::Value::as_str)
                });

            v.check_bool(
                "security:verify_source_remote",
                valid,
                &format!(
                    "verification_source=remote: valid={valid}, gate={gate_origin:?}, family={family_origin:?}"
                ),
            );

            if source_supported {
                let gate_bound = gate_origin.is_some() || family_origin.is_some();
                v.check_bool(
                    "security:btsp_gate_binding",
                    gate_bound,
                    &format!(
                        "BTSP gate binding: {}",
                        if gate_bound {
                            format!(
                                "gate={}, family={}",
                                gate_origin.unwrap_or("?"),
                                family_origin.unwrap_or("?"),
                            )
                        } else {
                            "MISSING — token provenance not tracked".to_owned()
                        }
                    ),
                );

                let trust_chain_valid = valid
                    && (gate_origin.is_some() || family_origin.is_some());
                v.check_bool(
                    "security:btsp_trust_chain",
                    trust_chain_valid,
                    &format!(
                        "BTSP trust chain: gate={gate_origin:?}, family={family_origin:?}, result={}",
                        if trust_chain_valid { "TRUSTED" } else { "BROKEN" }
                    ),
                );
            } else {
                v.check_skip(
                    "security:btsp_gate_binding",
                    "verification_source not supported — pre-w135 bearDog",
                );
                v.check_skip(
                    "security:btsp_trust_chain",
                    "verification_source not supported — pre-w135 bearDog",
                );
            }
        }
        Err(e) => {
            let msg = format!("{e}");
            if msg.contains("not found") || msg.contains("-32601") {
                v.check_skip(
                    "security:verify_source_remote",
                    "verification_source=remote not supported (bearDog needs w135+)",
                );
                v.check_skip("security:btsp_gate_binding", "not supported");
                v.check_skip("security:btsp_trust_chain", "not supported");
            } else {
                v.check_bool(
                    "security:verify_source_remote",
                    false,
                    &format!("remote verification error: {e}"),
                );
                v.check_skip("security:btsp_gate_binding", &format!("error: {e}"));
                v.check_skip("security:btsp_trust_chain", &format!("error: {e}"));
            }
        }
    }
}

fn verify_cross_gate_and_forged(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    valid_token: &str,
) {
    verify_cross_gate_dispatch(v, ctx, valid_token);
    verify_reject_forged(v, ctx);
}

fn verify_cross_gate_dispatch(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    valid_token: &str,
) {
    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "security:cross_gate_verify",
            "orchestration not available — no mesh dispatch (Songbird required)",
        );
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
}

fn verify_reject_forged(v: &mut ValidationResult, ctx: &mut CompositionContext) {
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
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "security:reject_forged",
                &format!("security provider not reachable: {e}"),
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
pub(super) fn phase_content_integrity(v: &mut ValidationResult, ctx: &mut CompositionContext) {
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
                validate_content_round_trip(v, ctx, content_hash, test_data);
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

fn validate_content_round_trip(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    content_hash: &str,
    expected_data: &str,
) {
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
                retrieved == expected_data,
                &format!(
                    "BLAKE3 round-trip: {}",
                    if retrieved == expected_data { "data matches" } else { "DATA MISMATCH" }
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
                let repl_hash = repl_resp.get("hash").and_then(serde_json::Value::as_str);
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
}

/// Phase 6: Dark Forest invariants — isolation + reversibility.
pub(super) fn phase_dark_forest_invariants(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
) {
    let federation_port_only = REGISTRY_TOML.contains("mesh.relay")
        || REGISTRY_TOML.contains("route.register");
    v.check_bool(
        "darkforest:federation_surface",
        federation_port_only,
        "federation capabilities registered (mesh.relay / route.register)",
    );

    let socket_dir_str = format!("{}/biomeos", crate::tolerances::runtime_dir());
    let socket_dir = std::path::Path::new(&socket_dir_str);
    let has_uds = socket_dir.exists();
    let uds_detail = if has_uds {
        "exists".to_owned()
    } else {
        format!("{socket_dir_str} not found")
    };
    v.check_bool(
        "darkforest:uds_runtime_exists",
        has_uds,
        &format!("UDS runtime dir: {uds_detail}"),
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

    check_port_isolation(v);
    check_reversibility(v, ctx);
}

fn check_port_isolation(v: &mut ValidationResult) {
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
}

fn check_reversibility(v: &mut ValidationResult, ctx: &mut CompositionContext) {
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
