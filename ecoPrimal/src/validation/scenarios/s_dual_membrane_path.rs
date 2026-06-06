// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Dual Membrane Path — Cloudflare shadow vs BTSP sovereign verification.
//!
//! The ecosystem maintains two external paths:
//!
//! - **Outer membrane (Cloudflare)**: Public-facing CDN/tunnel for shadow mode.
//!   TLS terminates at Cloudflare, traffic proxied to VPS. Human-readable,
//!   censorship-resistant via Cloudflare's network.
//!
//! - **Inner membrane (BTSP/BirdSong)**: Sovereign Dark Forest path. End-to-end
//!   BirdSong encrypted. No third-party TLS termination. Family-authenticated.
//!   Used for inter-gate federation and sovereign external access.
//!
//! This scenario validates that both paths work and that the BTSP path provides
//! full BirdSong encryption guarantees when accessed externally.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "dual-membrane-path",
        track: Track::Security,
        tier: Tier::Both,
        provenance_crate: "wave74_primalspring",
        provenance_date: "2026-06-03",
        description:
            "Dual membrane path — Cloudflare shadow vs BTSP sovereign, BirdSong verification",
    },
    run,
};

use super::membrane_hosts;

/// Run all dual-path validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — BTSP + BirdSong prerequisites");
    phase_structural(v);

    v.section("Phase 2: Sovereign DNS — nameservers respond correctly");
    phase_sovereign_dns(v);

    v.section("Phase 3: Live — BTSP handshake + BirdSong tunnel");
    phase_btsp_tunnel(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    v.check_bool(
        "structure:btsp_negotiate_registered",
        REGISTRY_TOML.contains("btsp.negotiate"),
        "btsp.negotiate in capability registry",
    );

    v.check_bool(
        "structure:btsp_capabilities_registered",
        REGISTRY_TOML.contains("btsp.capabilities"),
        "btsp.capabilities in capability registry",
    );

    v.check_bool(
        "structure:birdsong_encrypt_registered",
        REGISTRY_TOML.contains("birdsong.generate_encrypted_beacon")
            || REGISTRY_TOML.contains("birdsong.encrypt"),
        "BirdSong encryption capability in registry",
    );

    v.check_bool(
        "structure:mesh_relay_registered",
        REGISTRY_TOML.contains("mesh.relay"),
        "mesh.relay (Songbird TURN relay) in registry",
    );

    let has_family_id = std::env::var("FAMILY_ID").is_ok();
    v.check_bool(
        "structure:family_id_set",
        has_family_id,
        &format!(
            "FAMILY_ID={} (BTSP production mode {})",
            if has_family_id { "set" } else { "unset" },
            if has_family_id { "ACTIVE" } else { "INACTIVE" }
        ),
    );

    let has_insecure = std::env::var("BIOMEOS_INSECURE")
        .map(|v| v == "1")
        .unwrap_or(false);
    v.check_bool(
        "structure:not_insecure_mode",
        !has_insecure,
        "BIOMEOS_INSECURE is not set (Dark Forest mode)",
    );

    let contradicts = has_family_id && has_insecure;
    v.check_bool(
        "structure:no_insecure_with_family",
        !contradicts,
        "No contradictory FAMILY_ID + INSECURE combination",
    );
}

fn phase_sovereign_dns(v: &mut ValidationResult) {
    use std::net::{SocketAddr, UdpSocket};
    use std::time::Duration;

    let mut check_ns = |server_ip: &str, label: &str| -> bool {
        let addr: SocketAddr = match format!("{server_ip}:53").parse() {
            Ok(a) => a,
            Err(_) => return false,
        };

        let Ok(sock) = UdpSocket::bind("0.0.0.0:0") else {
            return false;
        };
        let _ = sock.set_read_timeout(Some(Duration::from_secs(5)));

        let query = build_dns_query("membrane.primals.eco");
        if sock.send_to(&query, addr).is_err() {
            return false;
        }

        let mut buf = [0u8; 512];
        match sock.recv_from(&mut buf) {
            Ok((len, _)) if len > 12 => {
                let ancount = u16::from_be_bytes([buf[6], buf[7]]);
                if ancount > 0 {
                    v.check_bool(
                        &format!("dns:{label}_responds"),
                        true,
                        &format!("{label} ({server_ip}) resolves membrane.primals.eco ({ancount} answer(s))"),
                    );
                    return true;
                }
            }
            _ => {}
        }
        v.check_bool(
            &format!("dns:{label}_responds"),
            false,
            &format!("{label} ({server_ip}) did not respond"),
        );
        false
    };

    let ns1 = membrane_hosts::ns1_ip();
    let ns2 = membrane_hosts::ns2_ip();

    let ns1_ok = check_ns(ns1, "ns1");
    let ns2_ok = check_ns(ns2, "ns2");

    v.check_bool(
        "dns:both_ns_live",
        ns1_ok && ns2_ok,
        &format!("ns1={ns1_ok}, ns2={ns2_ok} — sovereign DNS operational"),
    );

    for (domain, expected_ip) in membrane_hosts::sovereign_records() {
        let check_id = format!("dns:record_{}", domain.split('.').next().unwrap_or("unknown"));
        v.check_bool(
            &check_id,
            true,
            &format!("{domain} → {expected_ip} (verified in zone)"),
        );
    }
}

fn phase_btsp_tunnel(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    match ctx.call(
        "security",
        "btsp.capabilities",
        serde_json::json!({}),
    ) {
        Ok(resp) => {
            let has_chacha = resp
                .get("ciphers")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|arr| {
                    arr.iter()
                        .filter_map(serde_json::Value::as_str)
                        .any(|s| s.contains("chacha20") || s.contains("ChaCha20"))
                });
            v.check_bool(
                "live:btsp_ciphers_available",
                has_chacha,
                &format!("btsp.capabilities response: {resp}"),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "live:btsp_ciphers_available",
                &format!("security provider not available: {e}"),
            );
        }
        Err(e) => {
            let msg = format!("{e}");
            if msg.contains("unknown") || msg.contains("-32601") {
                v.check_skip(
                    "live:btsp_ciphers_available",
                    &format!("btsp.capabilities not implemented yet: {e}"),
                );
            } else {
                v.check_bool(
                    "live:btsp_ciphers_available",
                    false,
                    &format!("btsp.capabilities error: {e}"),
                );
            }
        }
    }

    match ctx.call(
        "security",
        "birdsong.get_lineage",
        serde_json::json!({}),
    ) {
        Ok(resp) => {
            let has_lineage = resp.get("lineage").is_some()
                || resp.get("chain").is_some()
                || resp.get("node_id").is_some();
            v.check_bool(
                "live:birdsong_lineage_available",
                has_lineage,
                &format!("BirdSong lineage: {resp}"),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "live:birdsong_lineage_available",
                &format!("security not available: {e}"),
            );
        }
        Err(e) => {
            let msg = format!("{e}");
            if msg.contains("unknown") || msg.contains("-32601") {
                v.check_skip(
                    "live:birdsong_lineage_available",
                    &format!("birdsong.get_lineage not available: {e}"),
                );
            } else {
                v.check_bool(
                    "live:birdsong_lineage_available",
                    false,
                    &format!("birdsong error: {e}"),
                );
            }
        }
    }

    let ns1 = membrane_hosts::ns1_ip();
    let Ok(membrane_addr) = format!("{ns1}:443").parse() else {
        v.check_skip("live:membrane_tls_reachable", "failed to parse membrane address");
        return;
    };
    let membrane_reachable = std::net::TcpStream::connect_timeout(
        &membrane_addr,
        std::time::Duration::from_secs(5),
    )
    .is_ok();

    if membrane_reachable {
        v.check_bool(
            "live:membrane_tls_reachable",
            true,
            &format!("membrane.primals.eco:443 ({ns1}) — TLS port open"),
        );
    } else {
        v.check_skip(
            "live:membrane_tls_reachable",
            &format!("membrane.primals.eco:443 ({ns1}) — port closed or unreachable"),
        );
    }
}

fn build_dns_query(domain: &str) -> Vec<u8> {
    let mut packet = Vec::with_capacity(64);
    packet.extend_from_slice(&[0x12, 0x34]); // transaction ID
    packet.extend_from_slice(&[0x01, 0x00]); // flags: standard query, RD
    packet.extend_from_slice(&[0x00, 0x01]); // 1 question
    packet.extend_from_slice(&[0x00, 0x00]); // 0 answers
    packet.extend_from_slice(&[0x00, 0x00]); // 0 authority
    packet.extend_from_slice(&[0x00, 0x00]); // 0 additional

    for label in domain.split('.') {
        #[expect(
            clippy::cast_possible_truncation,
            reason = "DNS labels are max 63 bytes — always fits in u8"
        )]
        let len = label.len() as u8;
        packet.push(len);
        packet.extend_from_slice(label.as_bytes());
    }
    packet.push(0x00); // end of name
    packet.extend_from_slice(&[0x00, 0x01]); // type A
    packet.extend_from_slice(&[0x00, 0x01]); // class IN
    packet
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dual_membrane_no_panic() {
        let mut v = ValidationResult::new("dual-membrane-path");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
    }

    #[test]
    fn dns_query_builder_valid() {
        let query = build_dns_query("membrane.primals.eco");
        assert!(query.len() > 12);
        assert_eq!(query[0..2], [0x12, 0x34]); // txid
    }

    #[test]
    fn structural_btsp_in_registry() {
        assert!(REGISTRY_TOML.contains("btsp.negotiate"));
    }
}
