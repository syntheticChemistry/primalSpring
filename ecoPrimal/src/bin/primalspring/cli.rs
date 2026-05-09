// SPDX-License-Identifier: AGPL-3.0-or-later

//! UniBin CLI — clap subcommands for the eukaryotic primalspring binary.

use clap::{Parser, Subcommand};

/// primalSpring UniBin — coordination, certification, and validation.
#[derive(Parser)]
#[command(
    name = "primalspring",
    version,
    about = "Eukaryotic coordination primal — certification, validation, and IPC server"
)]
pub struct Cli {
    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands.
#[derive(Subcommand)]
pub enum Commands {
    /// Run composition certification (absorbed guidestone, L0-L8).
    Certify {
        /// Maximum certification layer (0-8, default 8).
        #[arg(long, value_name = "N")]
        layer: Option<u8>,
        /// Run only Layer 0 (bare structural validation, no primals needed).
        #[arg(long, default_value_t = false)]
        bare: bool,
    },
    /// Run validation scenarios (absorbed experiments).
    Validate {
        /// Filter by track (e.g. atomic-composition, bonding, security).
        #[arg(long)]
        track: Option<String>,
        /// Run a single scenario by ID.
        #[arg(long)]
        scenario: Option<String>,
        /// Filter by tier: rust (structural), live (IPC), both.
        #[arg(long)]
        tier: Option<String>,
        /// List all available scenarios without running them.
        #[arg(long, default_value_t = false)]
        list: bool,
    },
    /// Start the JSON-RPC 2.0 IPC server (cell membrane).
    Serve,
    /// Show composition health and capability discovery status.
    Status,
    /// Show version information.
    Version,
}
