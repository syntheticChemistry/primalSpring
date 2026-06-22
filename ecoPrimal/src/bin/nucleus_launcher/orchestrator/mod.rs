// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! NUCLEUS orchestrator — dependency-ordered primal startup, health checks,
//! and discovery-provider registry seeding.

pub mod preflight;
mod registry;
mod spawn;
mod validate;

pub use validate::run_validation;

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
    /// Peer addresses for cross-gate mesh seeding (via discovery provider).
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
    clippy::needless_pass_by_value,
    reason = "config is consumed; caller never reuses it"
)]
pub fn run(config: LaunchConfig) -> LaunchResult {
    let primals = ordered_primals(config.atomic);
    let total = primals.len();

    print_banner(&config, &primals);

    if let Some(early) = phase_preflight(&config, &primals, total) {
        return early;
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
    let (started, healthy) = if config.seed_only {
        (0, 0)
    } else {
        phase_prepare_runtime(&config, &socket_dir);
        phase_stop_existing(&config, &primals);
        let s = phase_start_primals(
            &config,
            &primals,
            &mut nucleation,
            &family_seed_str,
            health_timeout,
        );
        let h = phase_health_sweep(&config, &primals, &nucleation, health_timeout);
        (s, h)
    };

    let registered = phase_registry_seeding(&config, &primals, &nucleation);
    phase_peer_seeding(&config, &nucleation);

    println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
    println!("\x1b[36m  NUCLEUS Ready\x1b[0m");
    println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
    println!();
    println!("  Composition: {:?}", config.atomic);
    println!("  Family:      {}", config.family_id);
    println!("  Node:        {}", config.node_id);
    println!();

    if config.validate && !config.dry_run {
        phase_validate(&config);
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

fn print_banner(config: &LaunchConfig, primals: &[&str]) {
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
}

fn phase_preflight(config: &LaunchConfig, primals: &[&str], total: usize) -> Option<LaunchResult> {
    if config.skip_preflight || config.dry_run {
        return None;
    }
    let pf = preflight::run_preflight(primals, config.uds_only);
    if pf.passed {
        return None;
    }
    println!("\x1b[31mPre-flight FAILED — aborting launch.\x1b[0m");
    if !pf.binary_missing.is_empty() {
        println!("  Missing binaries: {}", pf.binary_missing.join(", "));
        println!("  Run: plasmidbin sync   (or set ECOPRIMALS_ROOT)");
    }
    if !pf.port_conflicts.is_empty() {
        println!("  Port conflicts detected — fix ports.env");
    }
    println!();
    Some(LaunchResult {
        success: false,
        started: 0,
        healthy: 0,
        registered: 0,
        total,
    })
}

fn phase_prepare_runtime(config: &LaunchConfig, socket_dir: &std::path::Path) {
    let runtime_dir = tolerances::runtime_dir();
    info!("Phase 1: Prepare runtime");
    println!("=== Phase 1: Prepare runtime ===");
    println!("  Runtime: {runtime_dir}");
    println!("  Sockets: {}", socket_dir.display());
    if let Some(fed_port) = config.federation_port {
        println!("  Federation: TCP :{fed_port} (LAN mesh enabled)");
    } else {
        println!("  Federation: disabled (UDS-only, no LAN mesh)");
    }
    println!();
}

fn phase_stop_existing(config: &LaunchConfig, primals: &[&str]) {
    println!(
        "=== Phase 2: Stop existing primals (family: {}) ===",
        config.family_id
    );
    if !config.dry_run {
        for primal in primals {
            spawn::stop_existing_family(primal, &config.family_id);
        }
        std::thread::sleep(Duration::from_secs(1));
    }
    println!("  Cleared.");
    println!();
}

fn phase_start_primals(
    config: &LaunchConfig,
    primals: &[&str],
    nucleation: &mut SocketNucleation,
    family_seed_str: &str,
    health_timeout: Duration,
) -> usize {
    println!("=== Phase 3: Start primals (dependency order) ===");
    println!();

    let mut started = 0usize;
    for primal in primals {
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

        match spawn::spawn_primal(primal, port, &socket, config, family_seed_str) {
            Ok(()) => {
                std::thread::sleep(Duration::from_secs(3));
                if port > 0 {
                    match registry::health_check_tcp(port, health_timeout) {
                        registry::ProbeResult::Healthy => println!("\x1b[32mALIVE\x1b[0m"),
                        registry::ProbeResult::Reachable => {
                            println!("\x1b[32mALIVE\x1b[0m (no health method)");
                        }
                        registry::ProbeResult::Unreachable => {
                            println!("\x1b[33mSTARTED\x1b[0m (health probe pending)");
                        }
                    }
                } else {
                    println!("\x1b[32mSTARTED\x1b[0m (UDS)");
                }
                started += 1;
            }
            Err(e) => {
                println!("\x1b[31mFAIL\x1b[0m ({e})");
            }
        }
    }

    println!();
    println!("  Started: {started} / {}", primals.len());
    println!();
    started
}

fn phase_health_sweep(
    config: &LaunchConfig,
    primals: &[&str],
    nucleation: &SocketNucleation,
    health_timeout: Duration,
) -> usize {
    println!("=== Phase 4: Health sweep ===");
    let mut healthy = 0usize;

    for primal in primals {
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

        if port > 0 {
            match registry::health_check_tcp(port, health_timeout) {
                registry::ProbeResult::Healthy => {
                    println!("\x1b[32mHEALTHY\x1b[0m");
                    healthy += 1;
                }
                registry::ProbeResult::Reachable => {
                    println!("\x1b[32mALIVE\x1b[0m (no health method)");
                    healthy += 1;
                }
                registry::ProbeResult::Unreachable => {
                    let log_hint = PathBuf::from(tolerances::runtime_dir())
                        .join(primalspring::env_keys::BIOMEOS_SUBDIR)
                        .join("logs")
                        .join(format!("{primal}.log"));
                    println!("\x1b[31mUNREACHABLE\x1b[0m  (check {})", log_hint.display());
                }
            }
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
        }
    }

    println!();
    println!("  Healthy: {healthy} / {}", primals.len());
    println!();
    healthy
}

fn phase_registry_seeding(
    config: &LaunchConfig,
    primals: &[&str],
    nucleation: &SocketNucleation,
) -> usize {
    println!("=== Phase 5: Registry seeding (discovery ipc.register) ===");
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

    for primal in primals {
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
            || registry::register_with_discovery(songbird_port, &payload),
            |uds| registry::register_with_discovery_uds(uds, &payload),
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
    registered
}

#[expect(
    deprecated,
    reason = "SONGBIRD_PEERS fallback for backward compatibility"
)]
fn phase_peer_seeding(config: &LaunchConfig, nucleation: &SocketNucleation) {
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

    let peer_list: Vec<String> = if config.peers.is_empty() {
        std::env::var(env_keys::MESH_PEERS)
            .or_else(|_| std::env::var(env_keys::SONGBIRD_PEERS))
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        config.peers.clone()
    };

    if !peer_list.is_empty() {
        println!("=== Phase 5b: Peer seeding (cross-gate mesh) ===");
        let seeded = songbird_uds.map_or_else(
            || registry::seed_discovery_peers(songbird_port, &peer_list, &config.node_id),
            |uds| registry::seed_discovery_peers_uds(uds, &peer_list, &config.node_id),
        );
        if seeded > 0 {
            println!(
                "  \x1b[32mSeeded {seeded} peer(s)\x1b[0m: {}",
                peer_list.join(", ")
            );
        } else {
            println!(
                "  \x1b[31mFailed to seed peers\x1b[0m — discovery provider may not support mesh.init"
            );
            println!("  Peers requested: {}", peer_list.join(", "));
        }
        println!();
    }
}

fn phase_validate(config: &LaunchConfig) {
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

    let label = if family_id.is_empty() {
        "default"
    } else {
        family_id
    };
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
///
/// Reports results relative to the active composition profile (e.g.
/// Tower = 3 primals, not 13). A full green sweep means PASS for that profile.
pub fn show_status(primals: &[&str]) {
    let pid_dir = PathBuf::from(tolerances::runtime_dir())
        .join(primalspring::env_keys::BIOMEOS_SUBDIR)
        .join(".pids");
    let health_timeout = Duration::from_secs(3);

    let profile_label = AtomicType::from_primal_count(primals.len());

    println!(
        "=== NUCLEUS Status ({profile_label}: {}/{} primals) ===",
        primals.len(),
        primals.len()
    );
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

        let (health_ok, transport) = if registry::capability_probe(primal) {
            (true, "cap")
        } else if socket.exists() && registry::health_check_uds(&socket).is_alive() {
            (true, "uds")
        } else {
            let port = registry::effective_port(primal);
            if port > 0 && registry::health_check_tcp(port, health_timeout).is_alive() {
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
    if alive == total {
        println!("  \x1b[32mPASS\x1b[0m {alive}/{total} primals healthy ({profile_label})");
    } else {
        println!("  \x1b[33m{alive}/{total}\x1b[0m primals responding ({profile_label})");
    }
}
