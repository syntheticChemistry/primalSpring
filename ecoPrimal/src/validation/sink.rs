// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Pluggable output sinks for the validation harness.
//!
//! The [`ValidationSink`] trait allows experiments to redirect check output
//! (e.g. to a test harness capture buffer, tracing pipeline, or NDJSON
//! stream instead of stdout).

use std::sync::Arc;

use super::CheckOutcome;

/// Pluggable output destination for validation check results.
///
/// Converged ecosystem pattern from airSpring, rhizoCrypt, and groundSpring.
/// Default implementation writes to stdout; test harnesses can
/// substitute a capture buffer.
///
/// The `section` and `write_summary` methods have default no-op impls
/// for backward compatibility (absorbed from groundSpring V120).
pub trait ValidationSink: std::fmt::Debug + Send + Sync {
    /// Emit a single check result.
    fn on_check(&self, outcome: CheckOutcome, name: &str, detail: &str);

    /// Begin a named section of checks (e.g. "IPC health", "Graph validation").
    ///
    /// Absorbed from groundSpring V120 `ValidationSink::section()`. Sinks that
    /// support structured output can use this to group checks.
    fn section(&self, _name: &str) {}

    /// Write a summary footer after all checks are emitted.
    ///
    /// Absorbed from groundSpring V120 `ValidationSink::write_summary()`.
    fn write_summary(&self, _passed: u32, _failed: u32, _skipped: u32) {}
}

/// Default sink that writes check results to stdout.
#[derive(Debug)]
pub struct StdoutSink;

impl ValidationSink for StdoutSink {
    fn on_check(&self, outcome: CheckOutcome, name: &str, detail: &str) {
        let tag = match outcome {
            CheckOutcome::Pass => "PASS",
            CheckOutcome::Fail => "FAIL",
            CheckOutcome::Skip => "SKIP",
        };
        println!("  [{tag}] {name}: {detail}");
    }

    fn section(&self, name: &str) {
        println!("\n--- {name} ---");
    }

    fn write_summary(&self, passed: u32, failed: u32, skipped: u32) {
        let total = passed + failed;
        print!("  Summary: {passed}/{total} passed");
        if skipped > 0 {
            print!(" ({skipped} skipped)");
        }
        println!();
    }
}

/// Sink that routes check results through `tracing` instead of stdout.
///
/// PASS and SKIP results are logged at `info` level; failures at `warn`.
/// Section headers use `debug`. Useful for structured log pipelines and
/// embedded deployments where stdout is not a primary output channel.
#[derive(Debug)]
pub struct TracingSink;

impl ValidationSink for TracingSink {
    fn on_check(&self, outcome: CheckOutcome, name: &str, detail: &str) {
        match outcome {
            CheckOutcome::Pass => tracing::info!(check = name, outcome = "PASS", "{detail}"),
            CheckOutcome::Fail => tracing::warn!(check = name, outcome = "FAIL", "{detail}"),
            CheckOutcome::Skip => tracing::info!(check = name, outcome = "SKIP", "{detail}"),
        }
    }

    fn section(&self, name: &str) {
        tracing::debug!(section = name, "validation section");
    }

    fn write_summary(&self, passed: u32, failed: u32, skipped: u32) {
        let total = passed + failed;
        tracing::info!(passed, failed, skipped, total, "validation summary");
    }
}

/// Sink that discards all output (useful for tests that only inspect counts).
#[derive(Debug)]
pub struct NullSink;

impl ValidationSink for NullSink {
    fn on_check(&self, _: CheckOutcome, _: &str, _: &str) {}
}

/// Newline-delimited JSON sink for streaming validation output.
///
/// Absorbed from groundSpring V121 / wetSpring V133 / neuralSpring V122.
/// Emits one JSON object per check, one per line — ideal for log aggregation,
/// CI pipelines, and cross-process streaming. Each line is independently
/// parseable, unlike a single JSON array.
#[derive(Debug)]
pub struct NdjsonSink<W: std::io::Write + std::fmt::Debug + Send + Sync> {
    writer: std::sync::Mutex<W>,
}

impl<W: std::io::Write + std::fmt::Debug + Send + Sync> NdjsonSink<W> {
    /// Create a new NDJSON sink writing to the given destination.
    pub const fn new(writer: W) -> Self {
        Self {
            writer: std::sync::Mutex::new(writer),
        }
    }
}

impl NdjsonSink<std::io::Stdout> {
    /// Create a sink that writes NDJSON to stdout.
    #[must_use]
    pub fn stdout() -> Self {
        Self::new(std::io::stdout())
    }
}

impl<W: std::io::Write + std::fmt::Debug + Send + Sync> ValidationSink for NdjsonSink<W> {
    fn on_check(&self, outcome: CheckOutcome, name: &str, detail: &str) {
        let tag = match outcome {
            CheckOutcome::Pass => "pass",
            CheckOutcome::Fail => "fail",
            CheckOutcome::Skip => "skip",
        };
        let line = format!(
            "{{\"outcome\":\"{tag}\",\"name\":{},\"detail\":{}}}",
            serde_json::to_string(name).unwrap_or_else(|_| format!("\"{name}\"")),
            serde_json::to_string(detail).unwrap_or_else(|_| format!("\"{detail}\"")),
        );
        if let Ok(mut w) = self.writer.lock() {
            let _ = writeln!(w, "{line}");
        }
    }

    fn section(&self, name: &str) {
        let line = format!(
            "{{\"section\":{}}}",
            serde_json::to_string(name).unwrap_or_else(|_| format!("\"{name}\"")),
        );
        if let Ok(mut w) = self.writer.lock() {
            let _ = writeln!(w, "{line}");
        }
    }

    fn write_summary(&self, passed: u32, failed: u32, skipped: u32) {
        let line = format!(
            "{{\"summary\":{{\"passed\":{passed},\"failed\":{failed},\"skipped\":{skipped}}}}}"
        );
        if let Ok(mut w) = self.writer.lock() {
            let _ = writeln!(w, "{line}");
        }
    }
}

/// Create the default validation output sink (stdout).
pub(super) fn default_sink() -> Arc<dyn ValidationSink> {
    Arc::new(StdoutSink)
}
