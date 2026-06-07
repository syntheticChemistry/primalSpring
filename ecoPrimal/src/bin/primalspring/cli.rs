// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! UniBin CLI — clap subcommands for the eukaryotic primalspring binary.

use clap::{Parser, Subcommand, ValueEnum};

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

/// Output format for validation and certification results.
#[derive(Clone, Copy, Default, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable text output (default).
    #[default]
    Text,
    /// Machine-readable NDJSON output.
    Json,
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
        /// Output format: text (default) or json.
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Run validation scenarios (absorbed experiments).
    Validate {
        /// Filter by track (e.g. atomic-composition, bonding, security).
        #[arg(long)]
        track: Option<String>,
        /// Run a single scenario by ID.
        #[arg(long)]
        scenario: Option<String>,
        /// Filter by tier: rust, live, both (aliases: structural, ipc, tier1, tier2, all).
        #[arg(long)]
        tier: Option<String>,
        /// List all available scenarios without running them.
        #[arg(long, default_value_t = false)]
        list: bool,
        /// Output format: text (default) or json.
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
        /// Write provenance artifacts (results.json, provenance.toml) to this
        /// directory. Used by projectFOUNDATION Thread 10 workload.
        #[arg(long)]
        provenance_dir: Option<String>,
    },
    /// Start the JSON-RPC 2.0 IPC server (cell membrane).
    Serve,
    /// Show composition health and capability discovery status.
    Status,
    /// Regenerate BLAKE3 checksums manifest (validation/CHECKSUMS).
    Checksums {
        /// Output file (default: validation/CHECKSUMS).
        #[arg(long, default_value = "validation/CHECKSUMS")]
        output: String,
    },
    /// Lint capability registry for method-string drift.
    Registry {
        /// Check mode: source (Rust strings vs registry), graphs (TOML methods),
        /// coverage (registered but never used), all.
        #[arg(long, default_value = "all")]
        check: String,
    },
    /// Run release validation pipeline (fmt, clippy, tests, docs, depot).
    Release {
        /// Skip code coverage check.
        #[arg(long, default_value_t = false)]
        skip_coverage: bool,
        /// Skip NUCLEUS deployment gate.
        #[arg(long, default_value_t = false)]
        skip_nucleus: bool,
        /// Output JSON summary.
        #[arg(long, default_value_t = false)]
        json: bool,
    },
    /// Validate NUCLEUS deployment from plasmidBin depot (replaces bash gate).
    Nucleus {
        /// Run Tier 0-4 including lifecycle (shutdown + restart).
        #[arg(long, default_value_t = false)]
        full: bool,
        /// Skip NUCLEUS launch — validate against running instance.
        #[arg(long, default_value_t = false)]
        skip_launch: bool,
        /// Output JSON summary.
        #[arg(long, default_value_t = false)]
        json: bool,
    },
    /// Show version information.
    Version,
}
