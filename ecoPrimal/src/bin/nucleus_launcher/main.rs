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

    ManifestOverrides {
        family_id,
        composition,
        federation_port,
        peers,
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

    match cli.command {
        Some(NucleusCommand::Stop) => {
            let primals = orchestrator::ordered_primals(atomic);
            let family_id = cli.family_id.as_deref().unwrap_or("");
            orchestrator::stop_all_family(&primals, family_id);
        }
        Some(NucleusCommand::Status) => {
            let primals = orchestrator::ordered_primals(atomic);
            orchestrator::show_status(&primals);
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

            let federation_port = federation_port.or_else(|| {
                manifest_overrides
                    .as_ref()
                    .and_then(|m| m.federation_port)
            });

            let mut merged_peers = peers;
            if let Some(ref m) = manifest_overrides {
                for p in &m.peers {
                    if !merged_peers.contains(p) {
                        merged_peers.push(p.clone());
                    }
                }
            }

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
