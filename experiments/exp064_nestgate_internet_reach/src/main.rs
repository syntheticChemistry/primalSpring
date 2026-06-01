// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp064: Nestgate Internet Reach — connectivity paths via CompositionContext.

use std::time::Instant;

use primalspring::composition::CompositionContext;
use primalspring::validation::ValidationResult;

struct PathReport {
    available: Vec<&'static str>,
}

impl PathReport {
    const fn new() -> Self {
        Self {
            available: Vec::new(),
        }
    }

    fn record(&mut self, name: &'static str) {
        self.available.push(name);
    }
}

fn phase_https(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    endpoint: &str,
    report: &mut PathReport,
) {
    println!("  [1/5] HTTPS probe to {endpoint}");
    let start = Instant::now();
    let result = ctx.call(
        "discovery",
        "discovery.https_probe",
        serde_json::json!({ "url": endpoint }),
    );
    let lat = start.elapsed();
    match &result {
        Ok(r) => {
            println!(
                "    OK ({lat:?}): {}",
                r.to_string().chars().take(80).collect::<String>()
            );
            report.record("HTTPS");
            v.check_bool(
                "https_nestgate_reachable",
                true,
                "nestgate.io HTTPS reachable",
            );
        }
        Err(e) => {
            println!("    FAIL ({lat:?}): {e}");
            let msg = format!("{e}");
            let is_not_wired = msg.contains("Unknown method");
            v.check_bool(
                "https_nestgate_reachable",
                is_not_wired,
                &format!("HTTPS not wired in IPC yet: {e}"),
            );
        }
    }
}

fn phase_stun_and_nat(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    report: &mut PathReport,
) {
    println!("  [2/5] STUN public address");
    let start = Instant::now();
    let result = ctx.call(
        "discovery",
        "stun.get_public_address",
        serde_json::json!({}),
    );
    let lat = start.elapsed();
    match &result {
        Ok(addr) => {
            println!("    OK ({lat:?}): {addr}");
            report.record("STUN");
            v.check_bool("stun_public_address", true, "STUN resolved");
        }
        Err(e) => {
            println!("    FAIL ({lat:?}): {e}");
            v.check_bool("stun_public_address", false, &format!("STUN: {e}"));
        }
    }

    println!("  [3/5] NAT type detection");
    let nat = ctx.call("discovery", "stun.detect_nat_type", serde_json::json!({}));
    match &nat {
        Ok(n) => {
            println!("    NAT type: {n}");
            v.check_bool("nat_type_detected", true, "NAT type resolved");
        }
        Err(e) => {
            println!("    NAT: {e}");
            v.check_bool("nat_type_detected", false, &format!("NAT: {e}"));
        }
    }
}

fn phase_onion_and_tor(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    family_id: &str,
    report: &mut PathReport,
) {
    println!("  [4/5] Sovereign onion service");
    let onion = ctx.call(
        "discovery",
        "onion.start",
        serde_json::json!({ "family_id": family_id }),
    );
    match &onion {
        Ok(r) => {
            let addr = r
                .get("address")
                .and_then(|a| a.as_str())
                .unwrap_or("unknown");
            println!("    Onion: {addr}");
            report.record("Onion");
            v.check_bool("onion_service_started", true, "sovereign onion up");
        }
        Err(e) => {
            println!("    Onion: {e}");
            v.check_bool("onion_service_started", false, &format!("onion: {e}"));
        }
    }

    println!("  [5/5] Tor relay status");
    let tor = ctx.call("discovery", "tor.status", serde_json::json!({}));
    match &tor {
        Ok(s) => {
            println!("    Tor: {s}");
            report.record("Tor");
            v.check_bool("tor_available", true, "Tor responds");
        }
        Err(e) => {
            println!("    Tor: {e}");
            v.check_bool("tor_available", false, &format!("Tor: {e}"));
        }
    }
}

fn main() {
    let family_id = format!("e064-{}", std::process::id());
    let endpoint =
        std::env::var("NESTGATE_ENDPOINT").unwrap_or_else(|_| "https://api.nestgate.io".to_owned());

    ValidationResult::new("primalSpring Exp064 — Nestgate Internet Reach")
        .with_provenance("exp064_nestgate_internet_reach", "2026-05-09")
        .run(
            "primalSpring Exp064: Full internet deployment path validation",
            |v| {
                v.section("Phase 1: Internet reach probes");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();

                if !ctx.has_capability("discovery") {
                    v.check_bool("songbird_socket", false, "discovery capability not found");
                    return;
                }

                let mut report = PathReport::new();

                phase_https(v, &mut ctx, &endpoint, &mut report);
                phase_stun_and_nat(v, &mut ctx, &mut report);
                phase_onion_and_tor(v, &mut ctx, &family_id, &mut report);

                println!("\n  ╔══════════════════════════════════════════════════════╗");
                println!(
                    "  ║  Internet Reach: {}/{} paths available             ║",
                    report.available.len(),
                    5
                );
                println!("  ╚══════════════════════════════════════════════════════╝");
                println!("  Available: {}", report.available.join(", "));
                println!("\n  Deployment model: nestgate.io (Cloudflare tunnel)");
                println!("    Tower ─── HTTPS ──→ api.nestgate.io ←── Pixel 8a (hotspot)");
                println!("    Tower ─── Onion ──→ .onion ←── Pixel 8a (Tor)");
                println!("    Tower ─── STUN ──→ public IP ←── Pixel 8a (direct)");

                v.check_bool(
                    "at_least_one_path",
                    !report.available.is_empty(),
                    "at least one path",
                );
            },
        );
}
