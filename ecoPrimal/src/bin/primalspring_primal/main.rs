// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]

//! primalSpring `UniBin` — coordination and composition primal.
//!
//! Runs as a JSON-RPC 2.0 server over a Unix domain socket, exposing
//! coordination capabilities: composition validation, health aggregation,
//! and discovery sweep.

mod cli;
mod dispatch;
mod handlers;
mod server;

use clap::Parser;

use primalspring::coordination::AtomicType;
use primalspring::ipc::discover::{discover_capabilities_for, neural_api_healthy};
use primalspring::{PRIMAL_DOMAIN, PRIMAL_NAME};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let parsed = cli::Cli::parse();

    match parsed.command {
        cli::Commands::Server { bind_mode, port } => {
            server::run_server(bind_mode.as_deref(), port);
        }
        cli::Commands::Status => print_status(),
        cli::Commands::Version => {
            println!("primalspring {}", env!("CARGO_PKG_VERSION"));
        }
    }
}

fn print_status() {
    println!("{PRIMAL_NAME} v{}", env!("CARGO_PKG_VERSION"));
    println!("domain: {PRIMAL_DOMAIN}");
    println!("tracks: 6 (atomic, graph, emergent, bonding, coralforge, cross-spring)");
    println!(
        "local methods: {} | routed: {}",
        primalspring::niche::LOCAL_CAPABILITIES.len(),
        primalspring::niche::ROUTED_CAPABILITIES.len(),
    );

    let neural_ok = neural_api_healthy();
    println!(
        "neural_api: {}",
        if neural_ok { "reachable" } else { "not found" }
    );

    let capabilities = AtomicType::FullNucleus.required_capabilities();
    let discovered = discover_capabilities_for(capabilities);
    let found = discovered.iter().filter(|d| d.socket.is_some()).count();
    println!("capabilities: {found}/{} discovered", capabilities.len());

    for d in &discovered {
        let status = if d.socket.is_some() { "UP" } else { "DOWN" };
        let provider = d.resolved_primal.as_deref().unwrap_or("unresolved");
        println!("  [{status}] {} (via {provider})", d.capability);
    }
}
