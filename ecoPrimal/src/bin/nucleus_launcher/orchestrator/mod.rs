// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! NUCLEUS orchestrator — dependency-ordered primal startup, health checks,
//! and Songbird registry seeding.

pub mod preflight;
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
    /// Skip Phase 0 pre-flight validation (degraded-mode escape hatch).
    pub skip_preflight: bool,
    /// Allow startup with < 100% healthy primals (50% threshold).
    pub allow_degraded: bool,
    /// Don't stop already-started primals on failure.
    pub no_rollback: bool,
}

/// Summary of the launch operation.
pub struct LaunchResult {
    pub success: bool,
    #[expect(
        dead_code,
        reason = "public API for callers that display detailed launch stats"
    )]
    pub started: usize,
    #[expect(
        dead_code,
        reason = "public API for callers that display detailed launch stats"
    )]
    pub healthy: usize,
    #[expect(
        dead_code,
        reason = "public API for callers that display detailed launch stats"
    )]
    pub registered: usize,
    #[expect(
        dead_code,
        reason = "public API for callers that display detailed launch stats"
    )]
    pub total: usize,
}

/// Ordered primals for a given composition type.
///
/// Attempts graph-driven topological ordering first (from the composition's
/// deploy graph). Falls back to capability-based resolution with `Primal::ALL`
/// ordering when no graph is available or parsing fails.
pub fn ordered_primals(atomic: AtomicType) -> Vec<&'static str> {
    if let Some(ordered) = graph_ordered_primals(atomic) {
        return ordered;
    }
    capability_ordered_primals(atomic)
}

/// Try to resolve startup order from the composition's deploy graph.
///
/// Uses `topological_waves()` to get dependency-ordered waves, then
/// flattens them while retaining only primals that are `Primal::ALL` members.
fn graph_ordered_primals(atomic: AtomicType) -> Option<Vec<&'static str>> {
    let graph_name = atomic.graph_name();
    let graph_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("../graphs/{graph_name}.toml"));

    let graph = primalspring::deploy::load_graph(&graph_path).ok()?;
    let waves = primalspring::deploy::topological_waves(&graph).ok()?;

    let valid_primals: std::collections::HashSet<&str> =
        primal_names::Primal::ALL.iter().map(|p| p.slug()).collect();

    let mut ordered = Vec::new();
    for wave in &waves {
        for name in wave {
            if let Some(&slug) = valid_primals.get(name.as_str()) {
                if !ordered.contains(&slug) {
                    ordered.push(slug);
                }
            }
        }
    }

    if ordered.is_empty() {
        return None;
    }
    Some(ordered)
}

/// Fallback: resolve primals from capabilities, ordered by `Primal::ALL`.
fn capability_ordered_primals(atomic: AtomicType) -> Vec<&'static str> {
    use primalspring::composition::capability_to_primal;

    let all_slugs: Vec<&str> = primal_names::Primal::ALL.iter().map(|p| p.slug()).collect();
    let caps = atomic.required_capabilities();
    let mut resolved: Vec<&str> = caps
        .iter()
        .map(|cap| capability_to_primal(cap))
        .filter(|primal| all_slugs.contains(primal))
        .collect();
    resolved.sort_by_key(|p| all_slugs.iter().position(|s| s == p).unwrap_or(usize::MAX));
    resolved.dedup();
    resolved
}

/// Execute the full NUCLEUS launch sequence.
#[expect(
    clippy::too_many_lines,
    reason = "orchestration phases are sequential; splitting loses readability"
)]
#[expect(
    clippy::needless_pass_by_value,
    reason = "config is consumed; caller never reuses it"
)]
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
    println!(
        "  Transport:   {}",
        if config.uds_only {
            "UDS-only (VPS standard)"
        } else {
            "UDS + TCP"
        }
    );
    println!("  Dark Forest: {}", config.dark_forest);
    println!();

    // Phase 0: Pre-flight validation
    if !config.skip_preflight && !config.dry_run {
        let pf = preflight::run_preflight(&primals, config.uds_only);
        if !pf.passed {
            println!("\x1b[31mPre-flight FAILED — aborting launch.\x1b[0m");
            if !pf.binary_missing.is_empty() {
                println!("  Missing binaries: {}", pf.binary_missing.join(", "));
                println!("  Run: plasmidbin sync   (or set ECOPRIMALS_ROOT)");
            }
            if !pf.port_conflicts.is_empty() {
                println!("  Port conflicts detected — fix ports.env");
            }
            println!();
            return LaunchResult {
                success: false,
                started: 0,
                healthy: 0,
                registered: 0,
                total,
            };
        }
    }

    let runtime_dir = tolerances::runtime_dir();
    let socket_dir = PathBuf::from(&runtime_dir).join(primalspring::env_keys::BIOMEOS_SUBDIR);
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

        println!("=== Phase 2: Stop existing primals (family: {}) ===", config.family_id);
        if !config.dry_run {
            for primal in &primals {
                spawn::stop_existing_family(primal, &config.family_id);
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
                        .join(primalspring::env_keys::BIOMEOS_SUBDIR)
                        .join("logs")
                        .join(format!("{primal}.log"));
                    println!(
                        "\x1b[31mSOCKET ABSENT\x1b[0m  (check {})",
                        log_hint.display()
                    );
                }
            } else {
                let log_hint = PathBuf::from(tolerances::runtime_dir())
                    .join(primalspring::env_keys::BIOMEOS_SUBDIR)
                    .join("logs")
                    .join(format!("{primal}.log"));
                println!("\x1b[31mUNREACHABLE\x1b[0m  (check {})", log_hint.display());
            }
        }

        println!();
        println!("  Healthy: {healthy} / {total}");
        println!();
    }

    println!("=== Phase 5: Registry seeding (Songbird ipc.register) ===");
    let discovery_owner = primalspring::composition::capability_to_primal("discovery");
    let songbird_port = registry::effective_port_for(discovery_owner, config.uds_only);
    let songbird_uds: Option<&std::path::Path> = if config.uds_only {
        nucleation
            .get(discovery_owner, &config.family_id)
            .filter(|s| s.exists())
            .map(std::path::PathBuf::as_path)
    } else {
        None
    };
    if config.uds_only {
        match songbird_uds {
            Some(path) => println!("  transport: UDS ({})", path.display()),
            None => println!(
                "  transport: UDS requested but socket not found — falling back to TCP :{songbird_port}"
            ),
        }
    }
    let mut registered = 0usize;
    let capability_map = registry::build_capability_map();

    for primal in &primals {
        if *primal == discovery_owner {
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

        let caps_str = caps.join(",");

        print!("  {primal:<14} [{caps_str}]  ");

        if config.dry_run {
            println!("\x1b[33m[dry-run]\x1b[0m");
            registered += 1;
            continue;
        }

        let bind_host = std::env::var(primalspring::env_keys::PRIMALSPRING_HOST)
            .unwrap_or_else(|_| primalspring::tolerances::DEFAULT_HOST.to_owned());
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "ipc.register",
            "params": {
                "name": primal,
                "capabilities": caps,
                "endpoint": format!("unix://{socket}"),
                "tcp_endpoint": format!("tcp://{bind_host}:{port}"),
                "family_id": config.family_id,
                "node_id": config.node_id,
            },
            "id": 99
        });
        let payload = serde_json::to_string(&request).unwrap_or_default();

        let result = songbird_uds.map_or_else(
            || registry::register_with_songbird(songbird_port, &payload),
            |uds| registry::register_with_songbird_uds(uds, &payload),
        );
        match result {
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
        let seeded = match songbird_uds {
            Some(uds) => registry::seed_songbird_peers_uds(uds, &config.peers, &config.node_id),
            None => registry::seed_songbird_peers(songbird_port, &config.peers, &config.node_id),
        };
        if seeded > 0 {
            println!(
                "  \x1b[32mSeeded {seeded} peer(s)\x1b[0m: {}",
                config.peers.join(", ")
            );
        } else {
            println!("  \x1b[31mFailed to seed peers\x1b[0m — Songbird may not support mesh.init");
            println!("  Peers requested: {}", config.peers.join(", "));
        }
        println!();
    } else if let Ok(env_peers) = std::env::var(env_keys::SONGBIRD_PEERS) {
        if !env_peers.is_empty() {
            println!("=== Phase 5b: Peer seeding (from SONGBIRD_PEERS env) ===");
            let peer_list: Vec<String> = env_peers
                .split(',')
                .map(|s| s.trim().to_owned())
                .filter(|s| !s.is_empty())
                .collect();
            let seeded = match songbird_uds {
                Some(uds) => registry::seed_songbird_peers_uds(uds, &peer_list, &config.node_id),
                None => registry::seed_songbird_peers(songbird_port, &peer_list, &config.node_id),
            };
            if seeded > 0 {
                println!("  \x1b[32mSeeded {seeded} peer(s)\x1b[0m: {env_peers}");
            } else {
                println!(
                    "  \x1b[31mFailed to seed peers\x1b[0m — Songbird may not support mesh.init"
                );
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

    let success = if config.dry_run {
        true
    } else if config.allow_degraded {
        started == total && healthy >= total / 2
    } else {
        started == total && healthy == total
    };

    if !success && !config.dry_run && !config.no_rollback {
        println!("\x1b[31m=== Launch FAILED — rolling back ===\x1b[0m");
        stop_all(&primals);
        println!();
    }

    LaunchResult {
        success,
        started,
        healthy,
        registered,
        total,
    }
}

/// Stop all primals in the given list (reverse dependency order).
///
/// Uses the environment `FAMILY_ID` to locate family-scoped PID files.
/// Falls back to unscoped PID files for backward compatibility.
pub fn stop_all(primals: &[&str]) {
    let family_id = std::env::var(env_keys::FAMILY_ID).unwrap_or_default();
    stop_all_family(primals, &family_id);
}

/// Stop all primals scoped to a specific family.
pub fn stop_all_family(primals: &[&str], family_id: &str) {
    let pid_dir = PathBuf::from(tolerances::runtime_dir())
        .join(primalspring::env_keys::BIOMEOS_SUBDIR)
        .join(".pids");

    let label = if family_id.is_empty() { "default" } else { family_id };
    println!("=== Stopping primals (family: {label}) ===");
    for primal in primals.iter().rev() {
        let pid_file = if family_id.is_empty() {
            pid_dir.join(format!("{primal}.pid"))
        } else {
            pid_dir.join(format!("{primal}-{family_id}.pid"))
        };

        if try_stop_pid_file(primal, &pid_file, "") {
            continue;
        }

        let legacy = pid_dir.join(format!("{primal}.pid"));
        if legacy != pid_file && try_stop_pid_file(primal, &legacy, " (legacy)") {
            continue;
        }

        println!("  {primal:<14} \x1b[90mnot running\x1b[0m");
    }

    std::thread::sleep(Duration::from_secs(1));
    println!("  Done.");
}

/// Read a PID file, signal the process, and remove the file. Returns `true` if stopped.
fn try_stop_pid_file(primal: &str, pid_file: &std::path::Path, suffix: &str) -> bool {
    let Ok(contents) = std::fs::read_to_string(pid_file) else {
        return false;
    };
    let Ok(pid) = contents.trim().parse::<u32>() else {
        return false;
    };
    print!("  {primal:<14} pid={pid:<8}{suffix} ");
    if spawn::signal_pid(pid).is_ok() {
        println!("\x1b[33mSIGTERM\x1b[0m");
    } else {
        println!("\x1b[31mFAILED\x1b[0m");
    }
    let _ = std::fs::remove_file(pid_file);
    true
}

/// Show status of all primals via PID files and UDS/TCP health probes.
///
/// Prefers UDS socket liveness (Tower Atomic default). Falls back to TCP
/// only when a socket is unavailable and a port is configured.
pub fn show_status(primals: &[&str]) {
    let pid_dir = PathBuf::from(tolerances::runtime_dir())
        .join(primalspring::env_keys::BIOMEOS_SUBDIR)
        .join(".pids");
    let health_timeout = Duration::from_secs(3);

    println!("=== NUCLEUS Status ===");
    println!();

    let mut alive = 0usize;
    let mut total = 0usize;

    for primal in primals {
        total += 1;
        let pid_file = pid_dir.join(format!("{primal}.pid"));
        let socket = registry::socket_path_for(primal);

        let pid_status = std::fs::read_to_string(&pid_file)
            .ok()
            .and_then(|c| c.trim().parse::<u32>().ok())
            .filter(|pid| std::path::Path::new(&format!("/proc/{pid}")).exists());

        let (health_ok, transport) = if socket.exists() && registry::health_check_uds(&socket) {
            (true, "uds")
        } else {
            let port = registry::effective_port(primal);
            if port > 0 && registry::health_check_tcp(port, health_timeout) {
                (true, "tcp")
            } else {
                (false, "---")
            }
        };

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
        println!("  {primal:<14} [{status}] pid={pid_str:<8} via={transport}");
    }

    println!();
    println!("  {alive}/{total} primals responding to health checks");
}
