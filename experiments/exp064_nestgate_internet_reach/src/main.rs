// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp064: Nestgate Internet Reach — test full internet deployment paths.
//!
//! Spawns Tower and exercises every connectivity path: STUN public address,
//! HTTPS probe to `api.nestgate.io` (or configurable endpoint), sovereign
//! onion service, and Tor status. Reports which paths are available for
//! the Pixel 8a / USB / nestgate.io deployment model.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::{Duration, Instant};

use primalspring::coordination::AtomicType;
use primalspring::harness::AtomicHarness;
use primalspring::validation::ValidationResult;

fn rpc_call(
    socket: &Path,
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    let mut stream = UnixStream::connect(socket).map_err(|e| format!("connect: {e}"))?;
    stream.set_read_timeout(Some(Duration::from_secs(15))).ok();
    stream.set_write_timeout(Some(Duration::from_secs(5))).ok();

    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    });
    let msg = format!("{req}\n");
    stream
        .write_all(msg.as_bytes())
        .map_err(|e| format!("write: {e}"))?;
    let _ = stream.shutdown(std::net::Shutdown::Write);

    let reader = BufReader::new(&stream);
    for line in reader.lines().map_while(Result::ok) {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&line) {
            if let Some(result) = parsed.get("result") {
                return Ok(result.clone());
            }
            if let Some(error) = parsed.get("error") {
                return Err(format!("RPC error: {error}"));
            }
        }
    }
    Err("no response".to_owned())
}

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

fn probe_https(
    v: &mut primalspring::validation::ValidationResult,
    socket: &Path,
    endpoint: &str,
    report: &mut PathReport,
) {
    println!("  [1/5] HTTPS probe to {endpoint}");
    let start = Instant::now();
    let result = rpc_call(
        socket,
        "discovery.https_probe",
        &serde_json::json!({ "url": endpoint }),
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
            v.check_bool("https_nestgate_reachable", false, &format!("HTTPS: {e}"));
        }
    }
}

fn probe_stun_and_nat(
    v: &mut primalspring::validation::ValidationResult,
    socket: &Path,
    report: &mut PathReport,
) {
    println!("  [2/5] STUN public address");
    let start = Instant::now();
    let result = rpc_call(socket, "stun.get_public_address", &serde_json::json!({}));
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
    let nat = rpc_call(socket, "stun.detect_nat_type", &serde_json::json!({}));
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

fn probe_onion_and_tor(
    v: &mut primalspring::validation::ValidationResult,
    socket: &Path,
    family_id: &str,
    report: &mut PathReport,
) {
    println!("  [4/5] Sovereign onion service");
    let onion = rpc_call(
        socket,
        "onion.start",
        &serde_json::json!({ "family_id": family_id }),
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
    let tor = rpc_call(socket, "tor.status", &serde_json::json!({}));
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
    let graphs_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../graphs");
    let family_id = format!("exp064-reach-{}", std::process::id());
    let endpoint =
        std::env::var("NESTGATE_ENDPOINT").unwrap_or_else(|_| "https://api.nestgate.io".to_owned());

    ValidationResult::run_experiment(
        "primalSpring Exp064 — Nestgate Internet Reach",
        "primalSpring Exp064: Full internet deployment path validation",
        |v| {
            let running = AtomicHarness::new(AtomicType::Tower)
                .start_with_neural_api(&family_id, &graphs_dir)
                .expect("tower + neural-api should start");

            let songbird_socket = running
                .socket_for("discovery")
                .or_else(|| running.socket_for_primal("songbird"))
                .expect("songbird socket");

            let mut report = PathReport::new();

            probe_https(v, songbird_socket, &endpoint, &mut report);
            probe_stun_and_nat(v, songbird_socket, &mut report);
            probe_onion_and_tor(v, songbird_socket, &family_id, &mut report);

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
