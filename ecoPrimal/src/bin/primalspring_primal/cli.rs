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
    Server,
    /// Show health and capability info
    Status,
    /// Show version info
    Version,
}
