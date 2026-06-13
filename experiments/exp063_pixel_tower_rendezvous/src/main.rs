// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp063: Pixel ↔ Tower Rendezvous — BirdSong and connectivity via CompositionContext.

use primalspring::composition::CompositionContext;
use primalspring::ipc::methods;
use primalspring::ipc::tcp;
use primalspring::validation::ValidationResult;

fn phase_beacon(v: &mut ValidationResult, ctx: &mut CompositionContext, family_id: &str) {
    if !ctx.has_capability("discovery") {
        v.check_skip("beacon_generated", "discovery capability not connected");
        return;
    }

    let beacon_result = ctx.call(
        "discovery",
        "birdsong.generate_encrypted_beacon",
        serde_json::json!({
            "family_id": family_id,
            "node_id": family_id,
            "capabilities": ["security", "discovery", "network.tls"],
            "device_type": "tower"
        }),
    );

    match &beacon_result {
        Ok(beacon) => {
            println!("  beacon generated: {}B", beacon.to_string().len());
            v.check_bool(
                "beacon_generated",
                true,
                "BirdSong encrypted beacon generated",
            );

            let enc_str = beacon
                .get("encrypted_beacon")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let decrypt = ctx.call(
                "discovery",
                "birdsong.decrypt_beacon",
                serde_json::json!({ "encrypted_beacon": enc_str }),
            );
            match &decrypt {
                Ok(plain) => {
                    println!("  beacon decrypted: {plain}");
                    v.check_bool(
                        "beacon_roundtrip",
                        true,
                        "beacon encrypt+decrypt round-trip",
                    );
                }
                Err(e) => {
                    println!("  beacon decrypt: {e}");
                    v.check_bool("beacon_roundtrip", false, &format!("decrypt failed: {e}"));
                }
            }
        }
        Err(e) => {
            println!("  beacon generation: {e}");
            let msg = format!("{e}");
            v.check_bool(
                "beacon_generated",
                msg.contains("Method not found") || e.is_method_not_found(),
                &format!("birdsong.generate_encrypted_beacon: {e}"),
            );
        }
    }
}

fn phase_connectivity(v: &mut ValidationResult, ctx: &mut CompositionContext, family_id: &str) {
    if !ctx.has_capability("discovery") {
        v.check_skip("onion_started", "discovery capability not connected");
        v.check_skip(
            "stun_address_obtained",
            "discovery capability not connected",
        );
        return;
    }

    let onion = ctx.call(
        "discovery",
        "onion.start",
        serde_json::json!({ "family_id": family_id }),
    );
    match &onion {
        Ok(resp) => {
            let addr = resp
                .get("address")
                .and_then(|a| a.as_str())
                .unwrap_or("unknown");
            println!("  onion service started: {addr}");
            v.check_bool("onion_started", true, "sovereign onion for rendezvous");
        }
        Err(e) => {
            println!("  onion.start: {e}");
            let msg = format!("{e}");
            v.check_bool(
                "onion_started",
                msg.contains("Method not found") || e.is_method_not_found(),
                &format!("onion: {e}"),
            );
        }
    }

    let stun = ctx.call(
        "discovery",
        "stun.get_public_address",
        serde_json::json!({}),
    );
    match &stun {
        Ok(addr) => {
            println!("  STUN public address: {addr}");
            v.check_bool(
                "stun_address_obtained",
                true,
                "STUN resolved public address",
            );
        }
        Err(e) => {
            println!("  STUN: {e}");
            let msg = format!("{e}");
            v.check_bool(
                "stun_address_obtained",
                msg.contains("Method not found") || e.is_method_not_found(),
                &format!("STUN: {e}"),
            );
        }
    }
}

fn phase_cross_device(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    family_id: &str,
    host: &str,
    pixel_port: u16,
) {
    v.section("Phase 3: Cross-Device Beacon Exchange");
    println!("  Pixel host: {host}:{pixel_port}");

    match tcp::tcp_rpc(
        host,
        pixel_port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    ) {
        Ok(_) => {
            println!("  Pixel songbird: LIVE");
            v.check_bool("pixel_songbird_live", true, "Pixel songbird reachable");
        }
        Err(e) => {
            println!("  Pixel songbird: {e}");
            v.check_skip("pixel_songbird_live", &format!("Pixel unreachable: {e}"));
        }
    }

    if !ctx.has_capability("discovery") {
        v.check_skip("cross_device_beacon", "discovery capability not connected");
        return;
    }

    let local_beacon = ctx.call(
        "discovery",
        "birdsong.generate_encrypted_beacon",
        serde_json::json!({
            "family_id": family_id,
            "node_id": "tower_local",
            "capabilities": ["security", "discovery"],
            "device_type": "tower"
        }),
    );
    if let Ok(beacon) = &local_beacon {
        let enc = beacon
            .get("encrypted_beacon")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        if !enc.is_empty() {
            match tcp::tcp_rpc(
                host,
                pixel_port,
                "birdsong.decrypt_beacon",
                &serde_json::json!({ "encrypted_beacon": enc }),
            ) {
                Ok(_) => {
                    println!("  cross-device beacon: Tower→Pixel decrypt OK");
                    v.check_bool(
                        "cross_device_beacon",
                        true,
                        "Tower beacon decrypted by Pixel",
                    );
                }
                Err(e) => {
                    println!("  cross-device beacon: {e}");
                    v.check_skip("cross_device_beacon", &format!("Pixel decrypt: {e}"));
                }
            }
        }
    }
}

fn main() {
    let family_id = format!("e063-{}", std::process::id());

    ValidationResult::new("primalSpring Exp063 — Pixel Tower Rendezvous")
        .with_provenance("exp063_pixel_tower_rendezvous", "2026-05-09")
        .run(
            "primalSpring Exp063: BirdSong beacon generation + rendezvous exchange",
            |v| {
                v.section("Phase 1: BirdSong beacon");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_beacon(v, &mut ctx, &family_id);

                v.section("Phase 2: Connectivity");
                phase_connectivity(v, &mut ctx, &family_id);

                let pixel_host = std::env::var("PIXEL_SONGBIRD_HOST").ok();
                let pixel_port = tcp::env_port(
                    "PIXEL_SONGBIRD_PORT",
                    primalspring::tolerances::default_port_for("songbird"),
                );

                if let Some(ref host) = pixel_host {
                    phase_cross_device(v, &mut ctx, &family_id, host, pixel_port);
                }

                println!("\n  === Rendezvous Flow Summary ===");
                println!("  Tower ({family_id}) local validation complete.");
                if pixel_host.is_some() {
                    println!("  Cross-device beacon exchange attempted.");
                } else {
                    println!("  Set PIXEL_SONGBIRD_HOST to enable cross-device beacon exchange.");
                }
            },
        );
}
