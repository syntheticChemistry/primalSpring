// SPDX-License-Identifier: AGPL-3.0-or-later

//! `nucleus_launcher` — Rust replacement for `plasmidBin/nucleus_launcher.sh`.
//!
//! Orchestrates primal startup in dependency order, performs health checks,
//! and seeds Songbird's registry with capability domains.
//!
//! Wave 47: initial Rust elevation from bash launcher.

#![deny(unsafe_code)]

mod orchestrator;

use clap::Parser;

use primalspring::coordination::AtomicType;

/// NUCLEUS Launcher — start a full composition of primals.
#[derive(Parser, Debug)]
#[command(name = "nucleus_launcher", version, about)]
#[expect(clippy::struct_excessive_bools, reason = "CLI args naturally have many boolean flags")]
struct Cli {
    /// Family identifier (required for socket and seed naming).
    #[arg(long)]
    family_id: String,

    /// Node identifier (defaults to hostname).
    #[arg(long)]
    node_id: Option<String>,

    /// Composition type: tower, node, nest, nucleus (full).
    #[arg(long, default_value = "nucleus")]
    composition: String,

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

    /// Songbird TCP federation port for LAN mesh (enables cross-gate discovery).
    #[arg(long)]
    federation_port: Option<u16>,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    let atomic = match cli.composition.as_str() {
        "tower" => AtomicType::Tower,
        "node" => AtomicType::Node,
        "nest" => AtomicType::Nest,
        "nucleus" | "full" => AtomicType::FullNucleus,
        other => {
            eprintln!("Unknown composition type: {other}");
            eprintln!("Valid: tower, node, nest, nucleus, full");
            std::process::exit(1);
        }
    };

    let node_id = cli.node_id.unwrap_or_else(|| {
        hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| "nucleus".to_owned())
    });

    let config = orchestrator::LaunchConfig {
        family_id: cli.family_id,
        node_id,
        atomic,
        dark_forest: cli.dark_forest,
        seed_only: cli.seed_only,
        health_timeout_secs: cli.health_timeout,
        dry_run: cli.dry_run,
        validate: cli.validate,
        federation_port: cli.federation_port,
    };

    let result = orchestrator::run(config);

    std::process::exit(i32::from(!result.success));
}
