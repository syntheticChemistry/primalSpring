// SPDX-License-Identifier: AGPL-3.0-or-later

//! NUCLEUS orchestrator — dependency-ordered primal startup, health checks,
//! and Songbird registry seeding.

mod registry;
mod spawn;

use std::path::PathBuf;
use std::time::Duration;

use tracing::info;

use primalspring::coordination::AtomicType;
use primalspring::env_keys;
use primalspring::launcher::SocketNucleation;
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
    /// UDS-only mode: suppress TCP port allocation (VPS standard).
    pub uds_only: bool,
    pub federation_port: Option<u16>,
    /// Peer addresses for cross-gate Songbird mesh seeding.
    pub peers: Vec<String>,
}

/// Summary of the launch operation.
pub struct LaunchResult {
    pub success: bool,
    #[expect(dead_code, reason = "public API for callers that display detailed launch stats")]
    pub started: usize,
    #[expect(dead_code, reason = "public API for callers that display detailed launch stats")]
    pub healthy: usize,
    #[expect(dead_code, reason = "public API for callers that display detailed launch stats")]
    pub registered: usize,
    #[expect(dead_code, reason = "public API for callers that display detailed launch stats")]
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

/// Ordered primals for a given composition type, filtered against the startup order.
///
/// Resolves primals from `required_capabilities()` via the capability registry
/// routing table (capability → primal owner). Falls back to the deprecated
/// `required_primals()` if the routing table is empty.
pub fn ordered_primals(atomic: AtomicType) -> Vec<&'static str> {
    use primalspring::composition::capability_to_primal;

    let caps = atomic.required_capabilities();
    let mut resolved: Vec<&str> = caps
        .iter()
        .map(|cap| capability_to_primal(cap))
        .filter(|primal| STARTUP_ORDER.contains(primal))
        .collect();
    resolved.sort_unstable();
    resolved.dedup();

    if resolved.is_empty() {
        #[allow(deprecated)]
        let required = atomic.required_primals();
        return STARTUP_ORDER
            .iter()
            .copied()
            .filter(|p| required.contains(p))
            .collect();
    }

    STARTUP_ORDER
        .iter()
        .copied()
        .filter(|p| resolved.contains(p))
        .collect()
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
    println!("  Transport:   {}", if config.uds_only { "UDS-only (VPS standard)" } else { "UDS + TCP" });
    println!("  Dark Forest: {}", config.dark_forest);
    println!();

    let runtime_dir = tolerances::runtime_dir();
    let socket_dir = PathBuf::from(&runtime_dir).join("biomeos");
    let _ = std::fs::create_dir_all(&socket_dir);

    let family_seed = spawn::resolve_family_seed(&socket_dir);
    let family_seed_str = String::from_utf8_lossy(&family_seed).to_string();

    let seed_file = socket_dir.join(".family.seed");
    let _ = std::fs::write(&seed_file, &family_seed);

    let mut nucleation = SocketNucleation::new(PathBuf::from(&runtime_dir));
    nucleation.set_family_seed(family_seed);

    let health_timeout = Duration::from_secs(config.health_timeout_secs);
    let mut started = 0usize;
    let mut healthy = 0usize;

    if !config.seed_only {
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

        println!("=== Phase 2: Stop existing primals ===");
        if !config.dry_run {
            for primal in &primals {
                spawn::stop_existing(primal);
            }
            std::thread::sleep(Duration::from_secs(1));
        }
        println!("  Cleared.");
        println!();

        println!("=== Phase 3: Start primals (dependency order) ===");
        println!();

        for primal in &primals {
            let port = registry::effective_port_for(primal, config.uds_only);
            let socket = nucleation.assign(primal, &config.family_id);

            if config.uds_only {
                print!("  {primal:<14} uds-only    ");
            } else {
                print!("  {primal:<14} tcp={port:<5} ");
            }

            if config.dry_run {
                println!("\x1b[33m[dry-run]\x1b[0m");
                started += 1;
                continue;
            }

            match spawn::spawn_primal(primal, port, &socket, &config, &family_seed_str) {
                Ok(()) => {
                    std::thread::sleep(Duration::from_secs(3));
                    if port > 0 && registry::health_check_tcp(port, health_timeout) {
                        println!("\x1b[32mALIVE\x1b[0m");
                    } else if port == 0 {
                        println!("\x1b[32mSTARTED\x1b[0m (UDS)");
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

        println!("=== Phase 4: Health sweep ===");
        for primal in &primals {
            let port = registry::effective_port_for(primal, config.uds_only);

            if config.uds_only {
                print!("  {primal:<14} uds  ");
            } else {
                print!("  {primal:<14} :{port}  ");
            }

            if config.dry_run {
                println!("\x1b[33m[dry-run]\x1b[0m");
                healthy += 1;
                continue;
            }

            if port > 0 && registry::health_check_tcp(port, health_timeout) {
                println!("\x1b[32mHEALTHY\x1b[0m");
                healthy += 1;
            } else if config.uds_only {
                let socket = nucleation.get(primal, &config.family_id);
                let alive = socket.as_ref().is_some_and(|s| s.exists());
                if alive {
                    println!("\x1b[32mSOCKET LIVE\x1b[0m");
                    healthy += 1;
                } else {
                    let log_hint = PathBuf::from(tolerances::runtime_dir())
                        .join("biomeos/logs")
                        .join(format!("{primal}.log"));
                    println!(
                        "\x1b[31mSOCKET ABSENT\x1b[0m  (check {})",
                        log_hint.display()
                    );
                }
            } else {
                let log_hint = PathBuf::from(tolerances::runtime_dir())
                    .join("biomeos/logs")
                    .join(format!("{primal}.log"));
                println!(
                    "\x1b[31mUNREACHABLE\x1b[0m  (check {})",
                    log_hint.display()
                );
            }
        }

        println!();
        println!("  Healthy: {healthy} / {total}");
        println!();
    }

    println!("=== Phase 5: Registry seeding (Songbird ipc.register) ===");
    let songbird_port = registry::effective_port_for(primal_names::SONGBIRD, config.uds_only);
    let mut registered = 0usize;
    let capability_map = registry::build_capability_map();

    for primal in &primals {
        if *primal == primal_names::SONGBIRD {
            continue;
        }

        let caps = capability_map.get(*primal);
        let empty_caps = Vec::new();
        let caps = caps.unwrap_or(&empty_caps);
        if caps.is_empty() {
            continue;
        }

        let port = registry::effective_port_for(primal, config.uds_only);
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

        match registry::register_with_songbird(songbird_port, &payload) {
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

    if !config.peers.is_empty() {
        println!("=== Phase 5b: Peer seeding (cross-gate mesh) ===");
        let seeded = registry::seed_songbird_peers(songbird_port, &config.peers, &config.node_id);
        if seeded > 0 {
            println!("  \x1b[32mSeeded {seeded} peer(s)\x1b[0m: {}", config.peers.join(", "));
        } else {
            println!("  \x1b[31mFailed to seed peers\x1b[0m — Songbird may not support mesh.init");
            println!("  Peers requested: {}", config.peers.join(", "));
        }
        println!();
    } else if let Ok(env_peers) = std::env::var(env_keys::SONGBIRD_PEERS) {
        if !env_peers.is_empty() {
            println!("=== Phase 5b: Peer seeding (from SONGBIRD_PEERS env) ===");
            let peer_list: Vec<String> = env_peers.split(',').map(|s| s.trim().to_owned()).filter(|s| !s.is_empty()).collect();
            let seeded = registry::seed_songbird_peers(songbird_port, &peer_list, &config.node_id);
            if seeded > 0 {
                println!("  \x1b[32mSeeded {seeded} peer(s)\x1b[0m: {env_peers}");
            } else {
                println!("  \x1b[31mFailed to seed peers\x1b[0m — Songbird may not support mesh.init");
            }
            println!();
        }
    }

    println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
    println!("\x1b[36m  NUCLEUS Ready\x1b[0m");
    println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
    println!();
    println!("  Composition: {:?}", config.atomic);
    println!("  Family:      {}", config.family_id);
    println!("  Node:        {}", config.node_id);
    println!();

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

/// Stop all primals in the given list (reverse dependency order).
pub fn stop_all(primals: &[&str]) {
    let pid_dir = PathBuf::from(tolerances::runtime_dir())
        .join("biomeos")
        .join(".pids");

    println!("=== Stopping primals ===");
    for primal in primals.iter().rev() {
        let pid_file = pid_dir.join(format!("{primal}.pid"));
        if let Ok(contents) = std::fs::read_to_string(&pid_file) {
            if let Ok(pid) = contents.trim().parse::<u32>() {
                print!("  {primal:<14} pid={pid:<8} ");
                if spawn::signal_pid(pid).is_ok() {
                    println!("\x1b[33mSIGTERM\x1b[0m");
                } else {
                    println!("\x1b[31mFAILED\x1b[0m");
                }
                let _ = std::fs::remove_file(&pid_file);
                continue;
            }
        }
        println!("  {primal:<14} \x1b[90mnot running\x1b[0m");
    }

    std::thread::sleep(Duration::from_secs(1));
    println!("  Done.");
}

/// Show status of all primals via PID files and TCP health probes.
pub fn show_status(primals: &[&str]) {
    let pid_dir = PathBuf::from(tolerances::runtime_dir())
        .join("biomeos")
        .join(".pids");
    let health_timeout = Duration::from_secs(3);

    println!("=== NUCLEUS Status ===");
    println!();

    let mut alive = 0usize;
    let mut total = 0usize;

    for primal in primals {
        total += 1;
        let pid_file = pid_dir.join(format!("{primal}.pid"));
        let port = registry::effective_port(primal);

        let pid_status = std::fs::read_to_string(&pid_file)
            .ok()
            .and_then(|c| c.trim().parse::<u32>().ok())
            .filter(|pid| std::path::Path::new(&format!("/proc/{pid}")).exists());

        let health_ok = port > 0 && registry::health_check_tcp(port, health_timeout);

        let status = match (pid_status, health_ok) {
            (Some(_), true) => {
                alive += 1;
                "\x1b[32mALIVE\x1b[0m"
            }
            (Some(_), false) => "\x1b[33mSTARTED\x1b[0m",
            (None, true) => {
                alive += 1;
                "\x1b[32mALIVE\x1b[0m (no PID file)"
            }
            (None, false) => "\x1b[31mDOWN\x1b[0m",
        };

        let pid_str = pid_status.map_or_else(|| "-".to_owned(), |p| p.to_string());
        println!("  {primal:<14} [{status}] pid={pid_str:<8} tcp={port}");
    }

    println!();
    println!("  {alive}/{total} primals responding to health checks");
}
