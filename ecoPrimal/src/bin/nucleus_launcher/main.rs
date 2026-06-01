// SPDX-License-Identifier: AGPL-3.0-or-later

//! `nucleus_launcher` — Rust replacement for `plasmidBin/nucleus_launcher.sh`.
//!
//! Orchestrates primal startup in dependency order, performs health checks,
//! and seeds Songbird's registry with capability domains.
//!
//! Wave 47: initial Rust elevation from bash launcher.

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
        /// UDS-only mode: no TCP ports (VPS/cellMembrane standard).
        ///
        /// When set, all primals start with port 0 (UDS only).
        /// This is the standard for VPS deployments where TCP port
        /// exposure is a metadata leak. Desktop deployments that
        /// need TCP should omit this flag.
        #[arg(long)]
        uds_only: bool,
        /// Songbird TCP federation port for LAN mesh.
        #[arg(long)]
        federation_port: Option<u16>,
        /// Comma-separated peer addresses for cross-gate mesh.
        #[arg(long, value_delimiter = ',')]
        peers: Vec<String>,
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
    match composition {
        "tower" => AtomicType::Tower,
        "node" => AtomicType::Node,
        "nest" => AtomicType::Nest,
        "nucleus" | "full" => AtomicType::FullNucleus,
        other => {
            eprintln!("Unknown composition type: {other}");
            eprintln!("Valid: tower, node, nest, nucleus, full");
            std::process::exit(1);
        }
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    let atomic = resolve_atomic(&cli.composition);

    match cli.command {
        Some(NucleusCommand::Stop) => {
            let primals = orchestrator::ordered_primals(atomic);
            orchestrator::stop_all(&primals);
        }
        Some(NucleusCommand::Status) => {
            let primals = orchestrator::ordered_primals(atomic);
            orchestrator::show_status(&primals);
        }
        cmd => {
            let (dark_forest, seed_only, health_timeout, dry_run, validate, uds_only, federation_port, peers) =
                match cmd {
                    Some(NucleusCommand::Start {
                        dark_forest,
                        seed_only,
                        health_timeout,
                        dry_run,
                        validate,
                        uds_only,
                        federation_port,
                        peers,
                    }) => (dark_forest, seed_only, health_timeout, dry_run, validate, uds_only, federation_port, peers),
                    _ => (false, false, 20, false, false, false, None, Vec::new()),
                };
            let family_id = cli.family_id.unwrap_or_else(|| {
                eprintln!("error: --family-id is required for start");
                std::process::exit(1);
            });
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
                peers,
            };
            let result = orchestrator::run(config);
            std::process::exit(i32::from(!result.success));
        }
    }
}
