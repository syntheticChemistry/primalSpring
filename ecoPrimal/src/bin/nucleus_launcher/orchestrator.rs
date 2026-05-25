// SPDX-License-Identifier: AGPL-3.0-or-later

//! NUCLEUS orchestrator — dependency-ordered primal startup, health checks,
//! and Songbird registry seeding.

use std::path::PathBuf;
use std::time::Duration;

use tracing::info;

use primalspring::coordination::AtomicType;
use primalspring::ipc::tcp::env_port;
use primalspring::launcher::{SocketNucleation, discover_binary};
use primalspring::primal_names;
use primalspring::tolerances;

/// Full configuration for a NUCLEUS launch.
#[expect(clippy::struct_excessive_bools, reason = "maps directly to CLI flags")]
pub struct LaunchConfig {
    pub family_id: String,
    pub node_id: String,
    pub atomic: AtomicType,
    pub dark_forest: bool,
    pub seed_only: bool,
    pub health_timeout_secs: u64,
    pub dry_run: bool,
    pub validate: bool,
    pub federation_port: Option<u16>,
    /// Peer addresses for cross-gate Songbird mesh seeding.
    pub peers: Vec<String>,
}

/// Summary of the launch operation.
pub struct LaunchResult {
    pub success: bool,
    #[allow(dead_code)]
    pub started: usize,
    #[allow(dead_code)]
    pub healthy: usize,
    #[allow(dead_code)]
    pub registered: usize,
    #[allow(dead_code)]
    pub total: usize,
}

/// Dependency-ordered startup sequence (crypto spine first, orchestrator last).
const STARTUP_ORDER: &[&str] = &[
    primal_names::BEARDOG,
    primal_names::SONGBIRD,
    primal_names::SKUNKBAT,
    primal_names::TOADSTOOL,
    primal_names::BARRACUDA,
    primal_names::CORALREEF,
    primal_names::NESTGATE,
    primal_names::RHIZOCRYPT,
    primal_names::LOAMSPINE,
    primal_names::SWEETGRASS,
    primal_names::BIOMEOS,
    primal_names::SQUIRREL,
    primal_names::PETALTONGUE,
];

/// Maps primal name → capability domains for Songbird registry seeding.
fn capability_domains(primal: &str) -> &'static [&'static str] {
    match primal {
        "beardog" => &["security", "crypto", "btsp", "birdsong", "lineage", "entropy", "jwt"],
        "songbird" => &["discovery", "http", "tls", "mesh", "stun", "relay", "onion"],
        "skunkbat" => &["defense", "audit", "firewall"],
        "toadstool" => &["compute", "cpu", "gpu", "npu", "wasm", "orchestration"],
        "barracuda" => &["tensor", "linalg", "spectral", "stats", "fhe", "wgsl"],
        "coralreef" => &["shader", "spirv", "wgsl", "glsl", "naga", "compile", "vfio"],
        "nestgate" => &["storage", "provenance", "compression"],
        "rhizocrypt" => &["dag", "session", "ephemeral"],
        "loamspine" => &["ledger", "permanent", "audit"],
        "sweetgrass" => &["attribution", "prov-o"],
        "biomeos" => &["orchestration", "graph", "deploy", "nucleus", "spore", "niche"],
        "squirrel" => &["ai", "inference", "mcp"],
        "petaltongue" => &["visualization", "ui", "interaction", "representation"],
        _ => &[],
    }
}

/// Maps primal name → its default TCP port.
fn default_port(primal: &str) -> u16 {
    match primal {
        "beardog" => tolerances::TCP_FALLBACK_BEARDOG_PORT,
        "songbird" => tolerances::TCP_FALLBACK_SONGBIRD_PORT,
        "skunkbat" => tolerances::TCP_FALLBACK_SKUNKBAT_PORT,
        "toadstool" => tolerances::TCP_FALLBACK_TOADSTOOL_PORT,
        "barracuda" => tolerances::TCP_FALLBACK_BARRACUDA_PORT,
        "coralreef" => tolerances::TCP_FALLBACK_CORALREEF_PORT,
        "nestgate" => tolerances::TCP_FALLBACK_NESTGATE_PORT,
        "rhizocrypt" => tolerances::TCP_FALLBACK_RHIZOCRYPT_PORT,
        "loamspine" => tolerances::TCP_FALLBACK_LOAMSPINE_PORT,
        "sweetgrass" => tolerances::TCP_FALLBACK_SWEETGRASS_PORT,
        "biomeos" => tolerances::TCP_FALLBACK_BIOMEOS_PORT,
        "squirrel" => tolerances::TCP_FALLBACK_SQUIRREL_PORT,
        "petaltongue" => tolerances::TCP_FALLBACK_PETALTONGUE_PORT,
        _ => 0,
    }
}

/// Maps primal name → the env var name for its port override.
fn port_env_key(primal: &str) -> &'static str {
    match primal {
        "beardog" => "BEARDOG_PORT",
        "songbird" => "SONGBIRD_PORT",
        "skunkbat" => "SKUNKBAT_PORT",
        "toadstool" => "TOADSTOOL_PORT",
        "barracuda" => "BARRACUDA_PORT",
        "coralreef" => "CORALREEF_PORT",
        "nestgate" => "NESTGATE_PORT",
        "rhizocrypt" => "RHIZOCRYPT_PORT",
        "loamspine" => "LOAMSPINE_PORT",
        "sweetgrass" => "SWEETGRASS_PORT",
        "biomeos" => "BIOMEOS_PORT",
        "squirrel" => "SQUIRREL_PORT",
        "petaltongue" => "PETALTONGUE_PORT",
        _ => "",
    }
}

/// Resolve the effective TCP port for a primal.
fn effective_port(primal: &str) -> u16 {
    let key = port_env_key(primal);
    if key.is_empty() {
        return 0;
    }
    env_port(key, default_port(primal))
}

/// Ordered primals for a given composition type, filtered against the startup order.
fn ordered_primals(atomic: AtomicType) -> Vec<&'static str> {
    let required = atomic.required_primals();
    STARTUP_ORDER
        .iter()
        .copied()
        .filter(|p| required.contains(p))
        .collect()
}

/// Resolve or generate a family seed.
fn resolve_family_seed(socket_dir: &std::path::Path) -> Vec<u8> {
    if let Ok(val) = std::env::var("BEARDOG_FAMILY_SEED") {
        return val.into_bytes();
    }
    if let Ok(val) = std::env::var("FAMILY_SEED") {
        return val.into_bytes();
    }
    let seed_file = socket_dir.join(".family.seed");
    if let Ok(contents) = std::fs::read_to_string(&seed_file) {
        let trimmed = contents.trim();
        if !trimmed.is_empty() {
            return trimmed.as_bytes().to_vec();
        }
    }
    let mut buf = [0u8; 32];
    if getrandom::fill(&mut buf).is_err() {
        return b"fallback-seed-entropy-unavailable".to_vec();
    }
    let mut hex_seed = String::with_capacity(64);
    for b in buf {
        use std::fmt::Write;
        let _ = write!(hex_seed, "{b:02x}");
    }
    hex_seed.into_bytes()
}

/// Perform a JSON-RPC health check on a primal via TCP.
fn health_check_tcp(port: u16, timeout: Duration) -> bool {
    use std::io::{Read, Write};
    use std::net::{SocketAddr, TcpStream};

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let payload = r#"{"jsonrpc":"2.0","method":"health.check","params":{},"id":1}"#;

    let Ok(stream) = TcpStream::connect_timeout(&addr, timeout) else {
        return false;
    };

    if stream.set_read_timeout(Some(timeout)).is_err() || stream.set_write_timeout(Some(timeout)).is_err() {
        return false;
    }

    let mut s = stream;
    if s.write_all(payload.as_bytes()).is_err() {
        return false;
    }
    if s.write_all(b"\n").is_err() {
        return false;
    }

    let mut buf = [0u8; 4096];
    match s.read(&mut buf) {
        Ok(n) if n > 0 => {
            let response = String::from_utf8_lossy(&buf[..n]);
            response.contains("\"jsonrpc\"")
        }
        _ => false,
    }
}

/// Execute the full NUCLEUS launch sequence.
#[expect(clippy::too_many_lines, reason = "orchestration phases are sequential; splitting loses readability")]
#[expect(clippy::needless_pass_by_value, reason = "config is consumed; caller never reuses it")]
pub fn run(config: LaunchConfig) -> LaunchResult {
    let primals = ordered_primals(config.atomic);
    let total = primals.len();

    println!();
    println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
    println!("\x1b[36m  NUCLEUS Launcher (Rust)\x1b[0m");
    println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
    println!();
    println!("  Family:      {}", config.family_id);
    println!("  Node:        {}", config.node_id);
    println!("  Composition: {:?}", config.atomic);
    println!("  Primals:     {}", primals.join(", "));
    println!("  Dark Forest: {}", config.dark_forest);
    println!();

    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .unwrap_or_else(|_| "/tmp".to_owned());
    let socket_dir = PathBuf::from(&runtime_dir).join("biomeos");
    let _ = std::fs::create_dir_all(&socket_dir);

    let family_seed = resolve_family_seed(&socket_dir);
    let family_seed_str = String::from_utf8_lossy(&family_seed).to_string();

    let seed_file = socket_dir.join(".family.seed");
    let _ = std::fs::write(&seed_file, &family_seed);

    let mut nucleation = SocketNucleation::new(PathBuf::from(&runtime_dir));
    nucleation.set_family_seed(family_seed);

    let health_timeout = Duration::from_secs(config.health_timeout_secs);
    let mut started = 0usize;
    let mut healthy = 0usize;

    if !config.seed_only {
        // Phase 1: Prepare
        info!("Phase 1: Prepare runtime");
        println!("=== Phase 1: Prepare runtime ===");
        println!("  Runtime: {runtime_dir}");
        println!("  Sockets: {}", socket_dir.display());
        if let Some(fed_port) = config.federation_port {
            println!("  Federation: Songbird TCP :{fed_port} (LAN mesh enabled)");
        } else {
            println!("  Federation: disabled (UDS-only, no LAN mesh)");
        }
        println!();

        // Phase 2: Stop existing
        println!("=== Phase 2: Stop existing primals ===");
        if !config.dry_run {
            for primal in &primals {
                stop_existing(primal);
            }
            std::thread::sleep(Duration::from_secs(1));
        }
        println!("  Cleared.");
        println!();

        // Phase 3: Start primals in dependency order
        println!("=== Phase 3: Start primals (dependency order) ===");
        println!();

        for primal in &primals {
            let port = effective_port(primal);
            let socket = nucleation.assign(primal, &config.family_id);

            print!("  {primal:<14} tcp={port:<5} ");

            if config.dry_run {
                println!("\x1b[33m[dry-run]\x1b[0m");
                started += 1;
                continue;
            }

            match spawn_primal(primal, port, &socket, &config, &family_seed_str) {
                Ok(()) => {
                    std::thread::sleep(Duration::from_secs(3));
                    if health_check_tcp(port, health_timeout) {
                        println!("\x1b[32mALIVE\x1b[0m");
                    } else {
                        println!("\x1b[33mSTARTED\x1b[0m (health probe pending)");
                    }
                    started += 1;
                }
                Err(e) => {
                    println!("\x1b[31mFAIL\x1b[0m ({e})");
                }
            }
        }

        println!();
        println!("  Started: {started} / {total}");
        println!();

        // Phase 4: Health sweep
        println!("=== Phase 4: Health sweep ===");
        for primal in &primals {
            let port = effective_port(primal);
            print!("  {primal:<14} :{port}  ");

            if config.dry_run {
                println!("\x1b[33m[dry-run]\x1b[0m");
                healthy += 1;
                continue;
            }

            if health_check_tcp(port, health_timeout) {
                println!("\x1b[32mHEALTHY\x1b[0m");
                healthy += 1;
            } else {
                println!("\x1b[31mUNREACHABLE\x1b[0m  (check /tmp/{primal}.log)");
            }
        }

        println!();
        println!("  Healthy: {healthy} / {total}");
        println!();
    }

    // Phase 5: Registry seeding
    println!("=== Phase 5: Registry seeding (Songbird ipc.register) ===");
    let songbird_port = effective_port("songbird");
    let mut registered = 0usize;

    for primal in &primals {
        if *primal == "songbird" {
            continue;
        }

        let caps = capability_domains(primal);
        if caps.is_empty() {
            continue;
        }

        let port = effective_port(primal);
        let socket = nucleation
            .get(primal, &config.family_id)
            .map(|p| p.display().to_string())
            .unwrap_or_default();

        let caps_json: Vec<String> = caps.iter().map(|c| format!("\"{c}\"")).collect();
        let caps_str = caps_json.join(",");

        print!("  {primal:<14} [{caps_str}]  ");

        if config.dry_run {
            println!("\x1b[33m[dry-run]\x1b[0m");
            registered += 1;
            continue;
        }

        let payload = format!(
            r#"{{"jsonrpc":"2.0","method":"ipc.register","params":{{"name":"{primal}","capabilities":[{caps_str}],"endpoint":"unix://{socket}","tcp_endpoint":"tcp://127.0.0.1:{port}","family_id":"{}","node_id":"{}"}},"id":99}}"#,
            config.family_id, config.node_id
        );

        match register_with_songbird(songbird_port, &payload) {
            Ok(()) => {
                println!("\x1b[32mOK\x1b[0m");
                registered += 1;
            }
            Err(e) => {
                println!("\x1b[31mFAIL\x1b[0m ({e})");
            }
        }
    }

    println!();
    println!("  Registered: {registered}");
    println!();

    // Phase 5b: Peer seeding (cross-gate mesh)
    if !config.peers.is_empty() {
        println!("=== Phase 5b: Peer seeding (cross-gate mesh) ===");
        let seeded = seed_songbird_peers(songbird_port, &config.peers, &config.node_id);
        if seeded > 0 {
            println!("  \x1b[32mSeeded {seeded} peer(s)\x1b[0m: {}", config.peers.join(", "));
        } else {
            println!("  \x1b[31mFailed to seed peers\x1b[0m — Songbird may not support mesh.init");
            println!("  Peers requested: {}", config.peers.join(", "));
        }
        println!();
    } else if let Ok(env_peers) = std::env::var("SONGBIRD_PEERS") {
        if !env_peers.is_empty() {
            println!("=== Phase 5b: Peer seeding (from SONGBIRD_PEERS env) ===");
            let peer_list: Vec<String> = env_peers.split(',').map(|s| s.trim().to_owned()).filter(|s| !s.is_empty()).collect();
            let seeded = seed_songbird_peers(songbird_port, &peer_list, &config.node_id);
            if seeded > 0 {
                println!("  \x1b[32mSeeded {seeded} peer(s)\x1b[0m: {env_peers}");
            } else {
                println!("  \x1b[31mFailed to seed peers\x1b[0m — Songbird may not support mesh.init");
            }
            println!();
        }
    }

    // Summary
    println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
    println!("\x1b[36m  NUCLEUS Ready\x1b[0m");
    println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
    println!();
    println!("  Composition: {:?}", config.atomic);
    println!("  Family:      {}", config.family_id);
    println!("  Node:        {}", config.node_id);
    println!();

    // Phase 6 (optional): Validation
    if config.validate && !config.dry_run {
        println!("\x1b[36m=== Phase 6: Composition Validation ===\x1b[0m");
        let result = primalspring::coordination::validate_composition_ctx(config.atomic);
        if result.all_healthy {
            println!("  \x1b[32mPASS\x1b[0m — all primals healthy");
        } else {
            println!("  \x1b[31mFAIL\x1b[0m — some primals unhealthy");
            for p in &result.primals {
                let status = if p.health_ok { "UP" } else { "DOWN" };
                println!("    [{status}] {}", p.name);
            }
        }
        println!();
    }

    let success = config.dry_run || (started == total && healthy >= total / 2);

    LaunchResult {
        success,
        started,
        healthy,
        registered,
        total,
    }
}

/// Attempt to stop any running instance of a primal.
fn stop_existing(primal: &str) {
    use std::process::Command;
    let _ = Command::new("pkill")
        .args(["-f", &format!("primals/.*{primal}")])
        .output();
}

/// Spawn a primal process using its discovered binary.
fn spawn_primal(
    primal: &str,
    port: u16,
    socket: &std::path::Path,
    config: &LaunchConfig,
    family_seed: &str,
) -> Result<(), String> {
    let binary = discover_binary(primal).map_err(|e| e.to_string())?;

    let mut cmd = std::process::Command::new(&binary);
    cmd.arg("server");
    cmd.arg("--socket").arg(socket);
    cmd.arg("--port").arg(port.to_string());
    cmd.arg("--family-id").arg(&config.family_id);

    cmd.env("FAMILY_ID", &config.family_id);
    cmd.env("FAMILY_SEED", family_seed);
    cmd.env("BEARDOG_FAMILY_SEED", family_seed);

    if config.dark_forest {
        cmd.arg("--dark-forest");
    }

    if primal == primal_names::SONGBIRD {
        if let Some(fed_port) = config.federation_port {
            cmd.arg("--federation-port").arg(fed_port.to_string());
            cmd.arg("--bind").arg("0.0.0.0");
        }
        cmd.env("SONGBIRD_SECURITY_SOCKET", socket);
    }

    let log_path = format!("/tmp/{primal}.log");
    let log_file = std::fs::File::create(&log_path)
        .map_err(|e| format!("cannot create log file {log_path}: {e}"))?;
    let log_err = log_file.try_clone()
        .map_err(|e| format!("cannot clone log file: {e}"))?;

    cmd.stdout(log_file);
    cmd.stderr(log_err);

    cmd.spawn().map_err(|e| format!("spawn failed: {e}"))?;

    info!(primal, binary = %binary.display(), "spawned");
    Ok(())
}

/// Seed Songbird with known peer addresses for cross-gate mesh discovery.
fn seed_songbird_peers(port: u16, peers: &[String], node_id: &str) -> usize {
    use std::io::{Read, Write};
    use std::net::{SocketAddr, TcpStream};

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let timeout = Duration::from_secs(5);
    let mut seeded = 0;

    let peers_json: Vec<String> = peers.iter().map(|p| format!("\"{p}\"")).collect();
    let payload = format!(
        r#"{{"jsonrpc":"2.0","method":"mesh.init","params":{{"node_id":"{node_id}","bootstrap_peers":[{}]}},"id":2}}"#,
        peers_json.join(",")
    );

    let Ok(stream) = TcpStream::connect_timeout(&addr, timeout) else {
        return 0;
    };
    let _ = stream.set_read_timeout(Some(timeout));
    let _ = stream.set_write_timeout(Some(timeout));
    let mut s = stream;

    if s.write_all(payload.as_bytes()).is_ok() && s.write_all(b"\n").is_ok() {
        let mut buf = [0u8; 4096];
        if let Ok(n) = s.read(&mut buf) {
            if n > 0 {
                let resp = String::from_utf8_lossy(&buf[..n]);
                if resp.contains("\"result\"") {
                    seeded = peers.len();
                }
            }
        }
    }

    seeded
}

/// Send a register payload to Songbird.
fn register_with_songbird(port: u16, payload: &str) -> Result<(), String> {
    use std::io::{Read, Write};
    use std::net::{SocketAddr, TcpStream};

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let timeout = Duration::from_secs(5);

    let stream = TcpStream::connect_timeout(&addr, timeout)
        .map_err(|e| format!("Songbird :{port} unreachable: {e}"))?;

    let _ = stream.set_read_timeout(Some(timeout));
    let _ = stream.set_write_timeout(Some(timeout));

    let mut s = stream;
    s.write_all(payload.as_bytes())
        .map_err(|e| format!("write: {e}"))?;
    s.write_all(b"\n")
        .map_err(|e| format!("write newline: {e}"))?;

    let mut buf = [0u8; 4096];
    match s.read(&mut buf) {
        Ok(n) if n > 0 => {
            let resp = String::from_utf8_lossy(&buf[..n]);
            if resp.contains("\"result\"") {
                Ok(())
            } else {
                Err(format!("non-standard response: {}", &resp[..resp.len().min(80)]))
            }
        }
        Ok(_) => Err("empty response".to_owned()),
        Err(e) => Err(format!("read: {e}")),
    }
}
