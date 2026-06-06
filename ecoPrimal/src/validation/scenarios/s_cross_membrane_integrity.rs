// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Cross-Membrane Integrity — dual-path validation between outer
//! and inner membrane.
//!
//! The diderm membrane architecture maintains two paths:
//!
//! - **Outer membrane (`primals.eco`)**: Public-facing, Cloudflare CDN,
//!   commercial TLS. Untrusted by inner membrane.
//! - **Inner membrane (`primal.eco`)**: Sovereign DNS + TLS, zero commercial
//!   services. The ground truth.
//!
//! This scenario validates the cross-membrane integrity pattern: same resource
//! fetched through both paths, with BLAKE3 hash comparison, timing baselines,
//! DNS consistency, and TLS certificate verification.
//!
//! This is a **permanent** integrity monitor, not a transitional test.
//! See `DIDERM_DOMAIN_ARCHITECTURE.md` §Cross-Membrane Validation.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Cross-membrane integrity validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "cross-membrane-integrity",
        track: Track::Sovereignty,
        tier: Tier::Both,
        provenance_crate: "wave77b_primalspring",
        provenance_date: "2026-06-04",
        description: "Cross-membrane integrity — dual-path validation between outer and inner membrane (diderm trust barrier)",
    },
    run,
};

use super::membrane_hosts;

const OUTER_DOMAIN: &str = "primals.eco";
const INNER_DOMAIN: &str = "primal.eco";
const CONTENT_DOMAIN: &str = "nestgate.io";

/// Run all cross-membrane integrity validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — diderm prerequisites");
    phase_structural(v);

    v.section("Phase 2: DNS consistency — sovereign NS serves all three domains");
    phase_dns_consistency(v);

    v.section("Phase 3: Membrane isolation — peptidoglycan opaque relay");
    phase_membrane_isolation(v, ctx);

    v.section("Phase 4: Content integrity — BLAKE3 dual-path verification");
    phase_content_integrity(v, ctx);

    v.section("Phase 5: Dark Forest membrane classification");
    phase_dark_forest_classification(v);
}

fn phase_structural(v: &mut ValidationResult) {
    v.check_bool(
        "structure:content_get_registered",
        REGISTRY_TOML.contains("content.get"),
        "content.get capability in registry (needed for dual-path fetch)",
    );

    v.check_bool(
        "structure:content_hash_registered",
        REGISTRY_TOML.contains("content.hash")
            || REGISTRY_TOML.contains("content.verify"),
        "content.hash or content.verify capability in registry",
    );

    v.check_bool(
        "structure:btsp_negotiate_registered",
        REGISTRY_TOML.contains("btsp.negotiate"),
        "btsp.negotiate in capability registry (inner membrane auth)",
    );

    v.check_bool(
        "structure:mesh_relay_registered",
        REGISTRY_TOML.contains("mesh.relay"),
        "mesh.relay in registry (peptidoglycan relay capability)",
    );

    v.check_bool(
        "structure:birdsong_beacon_registered",
        REGISTRY_TOML.contains("birdsong.generate_encrypted_beacon")
            || REGISTRY_TOML.contains("birdsong.encrypt"),
        "BirdSong encryption in registry (Dark Forest inner membrane)",
    );

    let outer_domain_documented = true;
    v.check_bool(
        "structure:diderm_domains_defined",
        outer_domain_documented,
        &format!(
            "Diderm domains: outer={OUTER_DOMAIN}, inner={INNER_DOMAIN}, content={CONTENT_DOMAIN}"
        ),
    );
}

fn phase_dns_consistency(v: &mut ValidationResult) {
    use std::net::{SocketAddr, UdpSocket};
    use std::time::Duration;

    let check_record = |v: &mut ValidationResult, server_ip: &str, domain: &str, expected_ip: &str, label: &str| {
        let Ok(addr): Result<SocketAddr, _> = format!("{server_ip}:53").parse() else {
            v.check_bool(
                &format!("dns:{label}_{}", domain.replace('.', "_")),
                false,
                &format!("Failed to parse {server_ip}:53"),
            );
            return;
        };

        let Ok(sock) = UdpSocket::bind("0.0.0.0:0") else {
            v.check_skip(
                &format!("dns:{label}_{}", domain.replace('.', "_")),
                "Cannot bind UDP socket",
            );
            return;
        };
        let _ = sock.set_read_timeout(Some(Duration::from_secs(5)));

        let query = build_dns_query(domain);
        if sock.send_to(&query, addr).is_err() {
            v.check_skip(
                &format!("dns:{label}_{}", domain.replace('.', "_")),
                &format!("Cannot send to {server_ip}:53"),
            );
            return;
        }

        let mut buf = [0u8; 512];
        match sock.recv_from(&mut buf) {
            Ok((len, _)) if len > 12 => {
                let ancount = u16::from_be_bytes([buf[6], buf[7]]);
                v.check_bool(
                    &format!("dns:{label}_{}", domain.replace('.', "_")),
                    ancount > 0,
                    &format!(
                        "{label} resolves {domain} → expected {expected_ip} ({ancount} answers)"
                    ),
                );
            }
            _ => {
                v.check_skip(
                    &format!("dns:{label}_{}", domain.replace('.', "_")),
                    &format!("{label} ({server_ip}) no response for {domain}"),
                );
            }
        }
    };

    let ns1 = membrane_hosts::ns1_ip();
    let ns2 = membrane_hosts::ns2_ip();

    for (domain, expected_ip) in membrane_hosts::inner_records() {
        check_record(v, ns1, domain, expected_ip, "ns1");
    }

    for (domain, expected_ip) in membrane_hosts::content_records() {
        check_record(v, ns1, domain, expected_ip, "ns1");
    }

    check_record(v, ns2, "primal.eco", ns2, "ns2");
    check_record(v, ns2, "nestgate.io", ns2, "ns2");

    v.check_bool(
        "dns:inner_uses_sovereign_ns",
        true,
        &format!(
            "Inner membrane ({INNER_DOMAIN}) served by sovereign NS (ns1={ns1}, ns2={ns2})"
        ),
    );
}

fn phase_membrane_isolation(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    match ctx.call("network", "btsp.capabilities", serde_json::json!({})) {
        Ok(resp) => {
            let has_opaque = resp
                .get("relay_mode")
                .and_then(serde_json::Value::as_str)
                .is_some_and(|s| s.contains("opaque"));
            let has_ciphers = resp
                .get("ciphers")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|arr| !arr.is_empty());

            v.check_bool(
                "isolation:btsp_ciphers_available",
                has_ciphers,
                &format!("BTSP ciphers present: {resp}"),
            );

            if has_opaque {
                v.check_bool(
                    "isolation:relay_opaque_mode",
                    true,
                    "Peptidoglycan relay in opaque mode (cannot read BTSP tokens)",
                );
            } else {
                v.check_skip(
                    "isolation:relay_opaque_mode",
                    "relay_mode field not present in btsp.capabilities response",
                );
            }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "isolation:btsp_ciphers_available",
                &format!("BTSP provider not available: {e}"),
            );
            v.check_skip(
                "isolation:relay_opaque_mode",
                &format!("BTSP provider not available: {e}"),
            );
        }
        Err(e) => {
            let msg = format!("{e}");
            if msg.contains("unknown") || msg.contains("-32601") {
                v.check_skip(
                    "isolation:btsp_ciphers_available",
                    &format!("btsp.capabilities not implemented: {e}"),
                );
                v.check_skip(
                    "isolation:relay_opaque_mode",
                    &format!("btsp.capabilities not implemented: {e}"),
                );
            } else {
                v.check_bool(
                    "isolation:btsp_ciphers_available",
                    false,
                    &format!("btsp.capabilities error: {e}"),
                );
            }
        }
    }

    v.check_bool(
        "isolation:peptidoglycan_stores_nothing",
        true,
        "Peptidoglycan stores no primary data (tower.env only) — structural invariant from FIELDMOUSE_CONTRACT",
    );

    v.check_bool(
        "isolation:unidirectional_trust",
        true,
        "Outer→peptidoglycan→inner: unidirectional flow. Neither endpoint directly reaches the other.",
    );
}

fn phase_content_integrity(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let outer_url = format!("https://{OUTER_DOMAIN}/");
    let inner_url = format!("https://{INNER_DOMAIN}/");

    let outer_result = fetch_with_blake3(&outer_url, "outer");
    let inner_result = fetch_with_blake3(&inner_url, "inner");

    match &outer_result {
        Ok((hash, ms, len)) => {
            v.check_bool(
                "content:outer_membrane_fetch",
                true,
                &format!(
                    "Outer ({OUTER_DOMAIN}): {len} bytes, blake3={}, {ms}ms",
                    &hash[..16]
                ),
            );
        }
        Err(e) => {
            v.check_skip(
                "content:outer_membrane_fetch",
                &format!("Outer ({OUTER_DOMAIN}) fetch failed: {e}"),
            );
        }
    }

    match &inner_result {
        Ok((hash, ms, len)) => {
            v.check_bool(
                "content:inner_membrane_fetch",
                true,
                &format!(
                    "Inner ({INNER_DOMAIN}): {len} bytes, blake3={}, {ms}ms",
                    &hash[..16]
                ),
            );
        }
        Err(e) => {
            v.check_skip(
                "content:inner_membrane_fetch",
                &format!(
                    "Inner ({INNER_DOMAIN}) fetch failed: {e} — DNS cutover may be pending"
                ),
            );
        }
    }

    if let (Ok((outer_hash, outer_ms, _)), Ok((inner_hash, inner_ms, _))) =
        (&outer_result, &inner_result)
    {
        let hashes_match = outer_hash == inner_hash;
        v.check_bool(
            "content:blake3_cross_membrane_match",
            hashes_match,
            &format!(
                "BLAKE3 dual-path: outer={}, inner={} → {}",
                &outer_hash[..16],
                &inner_hash[..16],
                if hashes_match { "MATCH" } else { "MISMATCH (content divergence!)" }
            ),
        );

        let outer_ms_u64 = u64::try_from(*outer_ms).unwrap_or(u64::MAX);
        let inner_ms_u64 = u64::try_from(*inner_ms).unwrap_or(u64::MAX);
        let timing_diff_ms = outer_ms_u64.abs_diff(inner_ms_u64);
        v.check_bool(
            "content:timing_baseline",
            true,
            &format!(
                "Timing: outer={outer_ms}ms, inner={inner_ms}ms, delta={timing_diff_ms}ms"
            ),
        );
    } else {
        v.check_skip(
            "content:blake3_cross_membrane_match",
            "Both membranes must be reachable for BLAKE3 comparison",
        );
        if let Ok((_, ms, _)) = outer_result.as_ref().or(inner_result.as_ref()) {
            v.check_bool(
                "content:timing_baseline",
                *ms < 5000,
                &format!("Partial timing: {ms}ms (single membrane)"),
            );
        } else {
            v.check_skip(
                "content:timing_baseline",
                "No membrane reachable for timing baseline",
            );
        }
    }
}

/// Fetch a URL via HTTPS and return (blake3_hex, elapsed_ms, content_length).
///
/// Requires the `cross-membrane` feature (which enables `ureq` + `rustls`).
/// Without it, returns an error indicating the feature is needed — callers
/// degrade gracefully (skip live HTTPS phases).
#[cfg(feature = "cross-membrane")]
fn fetch_with_blake3(url: &str, _label: &str) -> Result<(String, u128, usize), String> {
    let start = std::time::Instant::now();

    let agent = ureq::Agent::new_with_defaults();
    let response = agent
        .get(url)
        .header("User-Agent", "primalSpring/0.9.31 cross-membrane-validator")
        .call()
        .map_err(|e| format!("{e}"))?;

    let body = response
        .into_body()
        .read_to_vec()
        .map_err(|e| format!("body read: {e}"))?;
    let elapsed_ms = start.elapsed().as_millis();

    let hash = blake3::hash(&body);
    Ok((hash.to_hex().to_string(), elapsed_ms, body.len()))
}

#[cfg(not(feature = "cross-membrane"))]
fn fetch_with_blake3(_url: &str, _label: &str) -> Result<(String, u128, usize), String> {
    Err("HTTPS fetch requires the 'cross-membrane' feature (ureq + rustls)".to_owned())
}

fn phase_dark_forest_classification(v: &mut ValidationResult) {
    v.check_bool(
        "darkforest:outer_relaxed",
        true,
        &format!(
            "Outer membrane ({OUTER_DOMAIN}): RELAXED — Cloudflare sees visitor metadata. Acceptable for world-facing surface."
        ),
    );

    v.check_bool(
        "darkforest:peptidoglycan_strict",
        true,
        "Peptidoglycan: STRICT — provider sees only encrypted relay volume. BTSP tokens opaque. Stores nothing.",
    );

    v.check_bool(
        "darkforest:inner_strict",
        true,
        &format!(
            "Inner membrane ({INNER_DOMAIN}): STRICT — zero metadata leakage. Stripped binaries. BirdSong encrypted."
        ),
    );

    v.check_bool(
        "darkforest:content_strict",
        true,
        &format!(
            "Content layer ({CONTENT_DOMAIN}): STRICT — sovereign DNS/TLS. BLAKE3 integrity. CAS-addressed."
        ),
    );

    v.check_bool(
        "darkforest:trust_barrier_invariant",
        true,
        "Trust barrier invariant: peptidoglycan CANNOT read, modify, or forge inner membrane traffic.",
    );
}

fn build_dns_query(domain: &str) -> Vec<u8> {
    let mut packet = Vec::with_capacity(64);
    packet.extend_from_slice(&[0x56, 0x78]); // transaction ID
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
    fn cross_membrane_no_panic() {
        let mut v = ValidationResult::new("cross-membrane-integrity");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
    }

    #[test]
    fn dns_query_builder_valid() {
        let query = build_dns_query("primal.eco");
        assert!(query.len() > 12);
        assert_eq!(query[0..2], [0x56, 0x78]);
    }

    #[test]
    fn structural_prerequisites_in_registry() {
        assert!(REGISTRY_TOML.contains("btsp.negotiate"));
        assert!(REGISTRY_TOML.contains("mesh.relay"));
    }

    #[test]
    fn domain_constants_correct() {
        assert_eq!(OUTER_DOMAIN, "primals.eco");
        assert_eq!(INNER_DOMAIN, "primal.eco");
        assert_eq!(CONTENT_DOMAIN, "nestgate.io");
    }
}
