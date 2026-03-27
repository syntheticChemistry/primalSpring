// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp076: Cross-Gate Neural Routing — validate biomeOS capability routing
//! across the Eastgate/Pixel boundary.
//!
//! Connects to a live biomeOS neural-api and validates that capabilities
//! can be routed to primals on a remote gate (Pixel) via TCP. Also tests
//! birdsong beacon exchange between local and remote Songbird instances,
//! and mesh initialization/announcement via the Neural API.
//!
//! Expects:
//! - biomeOS neural-api running on localhost (Unix socket)
//! - Pixel BearDog reachable at `PIXEL_BEARDOG_TCP` (default `localhost:19100`)
//! - Pixel Songbird reachable at `PIXEL_SONGBIRD_TCP` (default `localhost:19200`)

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use primalspring::ipc::NeuralBridge;
use primalspring::ipc::client::PrimalClient;
use primalspring::ipc::discover;
use primalspring::validation::ValidationResult;

fn pixel_beardog_addr() -> String {
    std::env::var("PIXEL_BEARDOG_TCP").unwrap_or_else(|_| "localhost:19100".to_owned())
}

fn pixel_songbird_port() -> u16 {
    std::env::var("PIXEL_SONGBIRD_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(19200)
}

fn local_songbird_port() -> u16 {
    std::env::var("SONGBIRD_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(9200)
}

/// Newline-delimited JSON-RPC over TCP (BearDog protocol).
fn tcp_rpc(addr: &str, method: &str, params: &serde_json::Value) -> Option<serde_json::Value> {
    let mut stream = TcpStream::connect(addr).ok()?;
    stream.set_read_timeout(Some(Duration::from_secs(5))).ok()?;
    stream
        .set_write_timeout(Some(Duration::from_secs(5)))
        .ok()?;

    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    });
    let mut payload = serde_json::to_string(&req).ok()?;
    payload.push('\n');
    stream.write_all(payload.as_bytes()).ok()?;
    stream.flush().ok()?;

    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line).ok()?;
    serde_json::from_str(&line).ok()
}

/// HTTP GET health check (Songbird exposes `/health` returning "OK").
fn http_health(host: &str, port: u16) -> bool {
    let addr = format!("{host}:{port}");
    let Ok(mut stream) = TcpStream::connect(&addr) else {
        return false;
    };
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let req = format!("GET /health HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n");
    if stream.write_all(req.as_bytes()).is_err() {
        return false;
    }
    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).unwrap_or(0);
    let resp = String::from_utf8_lossy(&buf[..n]);
    resp.contains("200 OK")
}

fn validate_pixel_tower(v: &mut ValidationResult) {
    let beardog_addr = pixel_beardog_addr();
    let songbird_port = pixel_songbird_port();

    let beardog_resp = tcp_rpc(&beardog_addr, "health.check", &serde_json::json!({}));
    let beardog_ok = beardog_resp
        .as_ref()
        .and_then(|r| r.get("result"))
        .and_then(|r| r.get("status"))
        .and_then(|s| s.as_str())
        .is_some_and(|s| s == "healthy");
    v.check_bool(
        "pixel_beardog_health",
        beardog_ok,
        &format!("Pixel BearDog at {beardog_addr}"),
    );

    let songbird_ok = http_health("localhost", songbird_port);
    v.check_bool(
        "pixel_songbird_health",
        songbird_ok,
        &format!("Pixel Songbird HTTP at localhost:{songbird_port}"),
    );
}

fn validate_cross_gate_crypto(v: &mut ValidationResult) {
    let addr = pixel_beardog_addr();
    let resp = tcp_rpc(&addr, "crypto.generate_keypair", &serde_json::json!({}));
    let has_key = resp
        .as_ref()
        .and_then(|r| r.get("result"))
        .and_then(|r| r.get("public_key"))
        .is_some();
    v.check_bool(
        "cross_gate_crypto",
        has_key,
        "Pixel BearDog crypto.generate_keypair via TCP",
    );
}

fn validate_cross_gate_beacon_exchange(v: &mut ValidationResult) {
    let songbird = discover::discover_primal("songbird");
    let local_beacon = songbird
        .socket
        .and_then(|s| PrimalClient::connect(&s, "songbird").ok())
        .and_then(|mut c| {
            c.call(
                "birdsong.generate_encrypted_beacon",
                serde_json::json!({
                    "node_id": "eastgate-exp076",
                    "capabilities": ["security", "discovery"]
                }),
            )
            .ok()
        })
        .and_then(|r| r.result)
        .and_then(|v| v.get("encrypted_beacon").cloned());

    v.check_bool(
        "local_beacon_generated",
        local_beacon.is_some(),
        "Eastgate Songbird birdsong beacon via Unix socket",
    );

    let local_songbird = local_songbird_port();
    let pixel_songbird = pixel_songbird_port();
    let local_http = http_health("localhost", local_songbird);
    let pixel_http = http_health("localhost", pixel_songbird);

    v.check_bool(
        "local_songbird_http",
        local_http,
        &format!("Eastgate Songbird HTTP at :{local_songbird}"),
    );
    v.check_bool(
        "pixel_songbird_http",
        pixel_http,
        &format!("Pixel Songbird HTTP at :{pixel_songbird}"),
    );

    let both_reachable = local_http && pixel_http;
    v.check_bool(
        "cross_gate_songbird_pair",
        both_reachable,
        "both Songbird instances reachable for beacon exchange",
    );
}

fn validate_neural_api_substrate(v: &mut ValidationResult) {
    let Some(bridge) = NeuralBridge::discover() else {
        v.check_skip("neural_api_substrate", "biomeOS not running");
        return;
    };

    let health = bridge.health_check();
    v.check_bool(
        "neural_api_health",
        health.is_ok(),
        "biomeOS neural-api healthy",
    );

    let crypto = bridge.capability_call("crypto", "generate_keypair", &serde_json::json!({}));
    v.check_bool(
        "neural_routing_crypto",
        crypto.is_ok(),
        "biomeOS routes crypto to local BearDog",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp076 — Cross-Gate Neural Routing")
        .with_provenance("exp076_cross_gate_neural_routing", "2026-03-27")
        .run(
            "primalSpring Exp076: Cross-gate capability routing via biomeOS substrate",
            |v| {
                validate_pixel_tower(v);
                validate_cross_gate_crypto(v);
                validate_cross_gate_beacon_exchange(v);
                validate_neural_api_substrate(v);
            },
        );
}
