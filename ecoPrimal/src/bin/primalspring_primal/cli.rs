// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "primalspring",
    version,
    about = "Coordination and composition validation primal"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the JSON-RPC 2.0 IPC server
    Server {
        /// Transport bind mode: uds_only (default), tcp_only, fallback, auto.
        /// Overrides PRIMAL_BIND_MODE env var. If omitted, auto-detects
        /// via PlatformCapabilities::detect().
        #[arg(long, value_name = "MODE")]
        bind_mode: Option<String>,

        /// TCP port override. Only used when bind mode selects TCP.
        /// Falls back to PRIMALSPRING_PORT env or ports.toml default.
        #[arg(long, short, value_name = "PORT")]
        port: Option<u16>,
    },
    /// Show health and capability info
    Status,
    /// Show version info
    Version,
}
