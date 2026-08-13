// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! `nucleus_launcher` — local experimentation NUCLEUS lifecycle manager.
//!
//! **Ownership**: Local experimentation launcher for eastGate lab only.
//! Production NUCLEUS lifecycle is biomeOS via the cellMembrane pipeline
//! (`biomeos nucleus start`). This binary validates composition startup
//! patterns that are then consumed by projectNUCLEUS.
//!
//! Orchestrates primal startup in dependency order, performs health checks,
//! and seeds Songbird's registry with capability domains.

#![forbid(unsafe_code)]

mod orchestrator;

use clap::Parser;

use primalspring::coordination::AtomicType;

use clap::Subcommand;

/// NUCLEUS Launcher — lifecycle management for primal compositions.
#[derive(Parser, Debug)]
#[command(name = "nucleus_launcher", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<NucleusCommand>,

    /// Family identifier (required for socket and seed naming).
    #[arg(long, global = true)]
    family_id: Option<String>,

    /// Node identifier (defaults to hostname).
    #[arg(long, global = true)]
    node_id: Option<String>,

    /// Composition type: tower, node, nest, nucleus (full).
    #[arg(long, global = true, default_value = "nucleus")]
    composition: String,

    /// Proto-nucleate manifest (TOML) — overrides family_id, composition, and
    /// mesh parameters from a structured deployment template.
    #[arg(long, global = true)]
    manifest: Option<std::path::PathBuf>,

    /// NUCLEUS biome.yaml manifest (v1 schema from toadStool).
    /// Overrides composition ordering with manifest-driven dependency graphs.
    #[arg(long, global = true)]
    biome: Option<std::path::PathBuf>,

    /// Named profile: tower, nest, compute, edge, full.
    /// Resolves to config/profiles/{name}.toml (convenience for --manifest).
    #[arg(long, global = true)]
    profile: Option<String>,
}

#[derive(Subcommand, Debug)]
enum NucleusCommand {
    /// Start primals in dependency order (default when no subcommand given).
    Start {
        /// Enable Dark Forest beacon mode.
        #[arg(long)]
        dark_forest: bool,
        /// Skip startup, only run Phase 5 registry seeding.
        #[arg(long)]
        seed_only: bool,
        /// Per-primal health timeout in seconds.
        #[arg(long, default_value = "20")]
        health_timeout: u64,
        /// Show plan without executing.
        #[arg(long)]
        dry_run: bool,
        /// Run composition validation after startup.
        #[arg(long)]
        validate: bool,
        /// Allow TCP ports (standalone/desktop mode).
        ///
        /// By default, all primals start UDS-only (port 0).
        /// Songbird handles cross-gate routing via federation.
        /// Pass `--tcp` to allocate TCP ports from the port
        /// registry (standalone debugging, desktop dev only).
        #[arg(long)]
        tcp: bool,
        /// Songbird TCP federation port for LAN mesh.
        #[arg(long)]
        federation_port: Option<u16>,
        /// Comma-separated peer addresses for cross-gate mesh.
        #[arg(long, value_delimiter = ',')]
        peers: Vec<String>,
        /// Skip Phase 0 pre-flight validation (degraded-mode escape hatch).
        #[arg(long)]
        skip_preflight: bool,
        /// Allow startup with degraded health (50% threshold instead of 100%).
        #[arg(long)]
        allow_degraded: bool,
        /// Don't stop already-started primals on failure.
        #[arg(long)]
        no_rollback: bool,
    },
    /// Stop running primals via PID files (graceful SIGTERM).
    Stop,
    /// Show status of running primals (PID files + health probes).
    Status,
    /// Run validation scenarios against a live NUCLEUS instance.
    ///
    /// Discovers the composition via standard IPC, then runs the scenario
    /// suite appropriate for the active composition type. Useful for
    /// post-deployment verification and interaction testing.
    Validate {
        /// Run only a specific scenario by ID (default: run all for composition).
        #[arg(long)]
        scenario: Option<String>,
        /// Only run Tier::Rust (structural) checks, skip live probes.
        #[arg(long)]
        structural_only: bool,
    },
    /// Reconcile a biome.yaml manifest against live NUCLEUS state.
    ///
    /// Loads the manifest, resolves composition graphs, then probes the
    /// running gate to report which primals are alive, missing, or extra.
    Reconcile,
    /// Assess the deploy→gossip→verify lifecycle for all primals.
    ///
    /// Probes socket existence, swarmVine gossip registration, and biomeOS
    /// mesh routability to report each primal's lifecycle phase.
    Lifecycle,
    /// Query fleet deployment health from swarmVine gossip.
    ///
    /// Connects to swarmVine and queries `deploy.result` gossip events
    /// emitted by biomeOS after each `composition.orchestrate` cycle.
    /// Aggregates per-gate health into a fleet summary.
    FleetHealth {
        /// Output as JSON instead of human-readable text.
        #[arg(long)]
        json: bool,
    },
}

fn resolve_node_id(cli_node_id: Option<String>) -> String {
    cli_node_id.unwrap_or_else(|| {
        std::env::var(primalspring::env_keys::HOSTNAME)
            .or_else(|_| std::env::var(primalspring::env_keys::HOST))
            .unwrap_or_else(|_| "nucleus".to_owned())
    })
}

fn resolve_atomic(composition: &str) -> AtomicType {
    match composition.parse() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}

/// Load proto-nucleate manifest and extract overridable fields.
struct ManifestOverrides {
    family_id: Option<String>,
    composition: Option<String>,
    federation_port: Option<u16>,
    peers: Vec<String>,
    /// If `true`, manifest explicitly requests TCP transport.
    tcp_enabled: bool,
    /// If `true`, allow degraded startup (50% health threshold).
    allow_degraded: bool,
}

fn load_manifest(path: &std::path::Path) -> ManifestOverrides {
    let contents = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: cannot read manifest {}: {e}", path.display());
            std::process::exit(1);
        }
    };
    let parsed: toml::Value = match toml::from_str(&contents) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: cannot parse manifest {}: {e}", path.display());
            std::process::exit(1);
        }
    };

    let family_id = parsed
        .get("gate")
        .and_then(|g| g.get("family_id"))
        .and_then(toml::Value::as_str)
        .filter(|s| !s.is_empty())
        .map(str::to_owned);

    let composition = parsed
        .get("composition")
        .and_then(|c| c.get("atomic_type"))
        .and_then(toml::Value::as_str)
        .filter(|s| !s.is_empty())
        .map(str::to_owned);

    let federation_port = parsed
        .get("mesh")
        .and_then(|m| m.get("federation_port"))
        .and_then(toml::Value::as_integer)
        .and_then(|p| u16::try_from(p).ok());

    let peers = parsed
        .get("mesh")
        .and_then(|m| m.get("peers"))
        .and_then(toml::Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();

    let tcp_enabled = parsed
        .get("composition")
        .and_then(|c| c.get("transport"))
        .and_then(toml::Value::as_str)
        .is_some_and(|t| t == "tcp_enabled" || t == "tcp");

    let allow_degraded = parsed
        .get("validation")
        .and_then(|v| v.get("allow_degraded"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);

    ManifestOverrides {
        family_id,
        composition,
        federation_port,
        peers,
        tcp_enabled,
        allow_degraded,
    }
}

#[expect(clippy::too_many_lines, reason = "CLI dispatch — single entry point")]
fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    let manifest_path = cli.manifest.clone().or_else(|| {
        cli.profile.as_ref().map(|name| {
            let profile_name = match name.as_str() {
                "tower" => "tower_atomic",
                "nest" => "nest_atomic",
                "compute" => "compute_heavy",
                "edge" => "edge_light",
                "full" => "full_nucleus",
                "eastgate-shared" | "shared" => "eastgate_shared",
                "eastgate-primalspring" | "primalspring" => "eastgate_primalspring",
                "fieldgate-canary" | "fieldgate" => "fieldgate_canary",
                "graphenegate" | "graphene" | "pixel" => "graphenegate",
                other => other,
            };
            std::path::PathBuf::from(format!("config/profiles/{profile_name}.toml"))
        })
    });
    let manifest_overrides = manifest_path.as_deref().map(load_manifest);

    let composition_str = manifest_overrides
        .as_ref()
        .and_then(|m| m.composition.as_deref())
        .unwrap_or(&cli.composition);
    let atomic = resolve_atomic(composition_str);

    let biome_manifest = cli.biome.as_deref().map(|path| {
        match primalspring::composition::manifest::load_biome_manifest(path) {
            Ok(m) => {
                println!("  Biome manifest: {} (v{})", m.metadata.name, m.metadata.version);
                m
            }
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
    });

    match cli.command {
        Some(NucleusCommand::Reconcile) => {
            let manifest = biome_manifest.unwrap_or_else(|| {
                let default_path = std::path::Path::new("config/biome-eastgate.yaml");
                match primalspring::composition::manifest::load_biome_manifest(default_path) {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("error: --biome required for reconcile (or config/biome-eastgate.yaml): {e}");
                        std::process::exit(1);
                    }
                }
            });
            let recon = primalspring::composition::manifest::reconcile_with_live(&manifest);
            println!();
            println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
            println!("\x1b[36m  Manifest Reconciliation — {}\x1b[0m", recon.gate);
            println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
            println!();
            println!("  Declared: {}", recon.declared);
            println!("  Alive:    {}", recon.alive);
            if !recon.missing.is_empty() {
                println!("  \x1b[31mMissing:  {}\x1b[0m", recon.missing.join(", "));
            }
            if !recon.extra.is_empty() {
                println!("  Extra:    {}", recon.extra.join(", "));
            }
            println!();
            for comp in &recon.compositions {
                let status = if comp.ready { "\x1b[32mREADY\x1b[0m" } else { "\x1b[31mNOT READY\x1b[0m" };
                println!("  {} ({}) — {status}", comp.name, comp.kind);
                if !comp.unhealthy_members.is_empty() {
                    println!("    unhealthy: {}", comp.unhealthy_members.join(", "));
                }
            }
            println!();
        }
        Some(NucleusCommand::Lifecycle) => {
            let manifest = biome_manifest.unwrap_or_else(|| {
                let default_path = std::path::Path::new("config/biome-eastgate.yaml");
                match primalspring::composition::manifest::load_biome_manifest(default_path) {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("error: --biome required for lifecycle (or config/biome-eastgate.yaml): {e}");
                        std::process::exit(1);
                    }
                }
            });
            let report = primalspring::composition::lifecycle::assess_lifecycle(&manifest);
            println!();
            println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
            println!("\x1b[36m  Lifecycle Assessment — {}\x1b[0m", report.gate);
            println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
            println!();
            println!("  Total: {}  |  Verified: {}  |  Gossip: {}  |  Deployed: {}  |  Missing: {}",
                report.summary.total,
                report.summary.verified,
                report.summary.gossip_only,
                report.summary.deployed_only,
                report.summary.not_deployed,
            );
            println!("  Elapsed: {}ms", report.elapsed_ms);
            println!();
            for state in &report.primals {
                let icon = match state.phase {
                    primalspring::composition::lifecycle::LifecyclePhase::Verified => "\x1b[32m✓\x1b[0m",
                    primalspring::composition::lifecycle::LifecyclePhase::GossipRegistered => "\x1b[33m◉\x1b[0m",
                    primalspring::composition::lifecycle::LifecyclePhase::Deployed => "\x1b[34m●\x1b[0m",
                    primalspring::composition::lifecycle::LifecyclePhase::NotDeployed => "\x1b[31m✗\x1b[0m",
                };
                let phase_label = state.phase.label();
                println!("  {icon} {:<14} {phase_label}", state.slug);
                if !state.declared_gossip_events.is_empty() && state.confirmed_gossip_events.is_empty() {
                    println!("      gossip: {} events declared, 0 confirmed", state.declared_gossip_events.len());
                }
            }
            println!();
        }
        Some(NucleusCommand::FleetHealth { json }) => {
            let fleet = primalspring::composition::deploy_health::query_fleet_health();

            if json {
                match serde_json::to_string_pretty(&fleet) {
                    Ok(s) => println!("{s}"),
                    Err(e) => {
                        eprintln!("error: {e}");
                        std::process::exit(1);
                    }
                }
            } else {
                println!();
                println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
                println!("\x1b[36m  Fleet Deployment Health\x1b[0m");
                println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
                println!();
                if fleet.gates_reporting == 0 {
                    println!("  No deploy.result gossip events found.");
                    println!("  biomeOS emits these after composition.orchestrate.");
                    println!("  Ensure swarmVine is running and biomeOS has deployed.");
                } else {
                    let status_icon = if fleet.is_fleet_healthy() {
                        "\x1b[32m●\x1b[0m"
                    } else {
                        "\x1b[31m●\x1b[0m"
                    };
                    println!("  {status_icon} Fleet: {}/{} healthy  ({:.0}%)",
                        fleet.gates_healthy, fleet.gates_reporting,
                        fleet.health_ratio() * 100.0);
                    if fleet.gates_failed > 0 {
                        println!("  \x1b[31m  Failed: {}\x1b[0m", fleet.gates_failed);
                    }
                    if fleet.gates_stale > 0 {
                        println!("  \x1b[33m  Stale:  {}\x1b[0m", fleet.gates_stale);
                    }
                    println!();
                    for (gate, health) in &fleet.gates {
                        let icon = if health.latest.success {
                            "\x1b[32m✓\x1b[0m"
                        } else {
                            "\x1b[31m✗\x1b[0m"
                        };
                        println!("  {icon} {gate:<14} {}/{} primals  {}ms  ({}s ago)",
                            health.latest.primals_alive,
                            health.latest.primals_expected,
                            health.latest.deploy_ms,
                            health.staleness_secs);
                        if let Some(ref err) = health.latest.error {
                            println!("      error: {err}");
                        }
                    }
                }
                println!();
            }
        }
        Some(NucleusCommand::Stop) => {
            let primals = orchestrator::ordered_primals(atomic);
            let family_id = cli
                .family_id
                .as_deref()
                .or_else(|| {
                    manifest_overrides
                        .as_ref()
                        .and_then(|m| m.family_id.as_deref())
                })
                .unwrap_or("");
            orchestrator::stop_all_family(&primals, family_id);
        }
        Some(NucleusCommand::Status) => {
            let primals = orchestrator::ordered_primals(atomic);
            orchestrator::show_status(&primals);
        }
        Some(NucleusCommand::Validate {
            scenario,
            structural_only,
        }) => {
            orchestrator::run_validation(atomic, scenario.as_deref(), structural_only);
        }
        cmd => {
            let (
                dark_forest,
                seed_only,
                health_timeout,
                dry_run,
                validate,
                uds_only,
                federation_port,
                peers,
                skip_preflight,
                allow_degraded,
                no_rollback,
            ) = match cmd {
                Some(NucleusCommand::Start {
                    dark_forest,
                    seed_only,
                    health_timeout,
                    dry_run,
                    validate,
                    tcp,
                    federation_port,
                    peers,
                    skip_preflight,
                    allow_degraded,
                    no_rollback,
                }) => (
                    dark_forest,
                    seed_only,
                    health_timeout,
                    dry_run,
                    validate,
                    !tcp,
                    federation_port,
                    peers,
                    skip_preflight,
                    allow_degraded,
                    no_rollback,
                ),
                _ => (
                    false,
                    false,
                    20,
                    false,
                    false,
                    true,
                    None,
                    Vec::new(),
                    false,
                    false,
                    false,
                ),
            };
            let family_id = cli
                .family_id
                .or_else(|| {
                    manifest_overrides
                        .as_ref()
                        .and_then(|m| m.family_id.clone())
                })
                .unwrap_or_else(|| {
                    eprintln!("error: --family-id is required for start (or set in manifest)");
                    std::process::exit(1);
                });

            let uds_only = if manifest_overrides.as_ref().is_some_and(|m| m.tcp_enabled) {
                false
            } else {
                uds_only
            };

            let federation_port = federation_port
                .or_else(|| manifest_overrides.as_ref().and_then(|m| m.federation_port));

            let mut merged_peers = peers;
            if let Some(ref m) = manifest_overrides {
                for p in &m.peers {
                    if !merged_peers.contains(p) {
                        merged_peers.push(p.clone());
                    }
                }
            }

            let allow_degraded = allow_degraded
                || manifest_overrides
                    .as_ref()
                    .is_some_and(|m| m.allow_degraded);

            let config = orchestrator::LaunchConfig {
                family_id,
                node_id: resolve_node_id(cli.node_id),
                atomic,
                dark_forest,
                seed_only,
                health_timeout_secs: health_timeout,
                dry_run,
                validate,
                uds_only,
                federation_port,
                peers: merged_peers,
                skip_preflight,
                allow_degraded,
                no_rollback,
            };
            let result = orchestrator::run(config);
            std::process::exit(i32::from(!result.success));
        }
    }
}
