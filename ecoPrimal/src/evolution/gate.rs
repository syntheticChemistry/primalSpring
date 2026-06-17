// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Gate readiness matrix — tracks deployment posture per gate.
//!
//! Each gate in the ecosystem has a readiness state determined by:
//! - Primal liveness (how many of 13 are running)
//! - Depot freshness (how old are the binaries)
//! - VCS sync status (is it on HEAD)
//! - Mesh connectivity (federation peers reachable)
//! - Identity configuration (GATE_NAME, FAMILY_SEED)

use std::fmt;

/// Readiness classification for a single gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReadinessLevel {
    /// Gate is unreachable or powered off.
    Offline,
    /// Gate is reachable but not running primals.
    Reachable,
    /// Gate has partial primal coverage.
    Partial,
    /// Gate is fully operational with all primals alive.
    Full,
    /// Gate is fully operational and verified (all checks pass).
    Verified,
}

impl fmt::Display for ReadinessLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Offline => write!(f, "OFFLINE"),
            Self::Reachable => write!(f, "REACHABLE"),
            Self::Partial => write!(f, "PARTIAL"),
            Self::Full => write!(f, "FULL"),
            Self::Verified => write!(f, "VERIFIED"),
        }
    }
}

/// Status snapshot for a single gate in the ecosystem.
#[derive(Debug, Clone)]
pub struct GateStatus {
    /// Gate identifier (e.g. "eastGate", "sporeGate", "golgi").
    pub name: String,
    /// Readiness classification.
    pub readiness: ReadinessLevel,
    /// Number of primals alive.
    pub primals_alive: u8,
    /// Total primals expected.
    pub primals_expected: u8,
    /// Whether depot is fresh (< 72h old).
    pub depot_fresh: bool,
    /// Whether VCS is synced with remotes.
    pub vcs_synced: bool,
    /// Number of mesh peers connected.
    pub mesh_peers: u8,
    /// Optional last-seen timestamp (Unix epoch seconds).
    pub last_seen: Option<u64>,
    /// Free-form notes.
    pub notes: String,
}

impl GateStatus {
    /// Create a new gate status with minimal information.
    #[must_use]
    pub fn new(name: impl Into<String>, readiness: ReadinessLevel) -> Self {
        Self {
            name: name.into(),
            readiness,
            primals_alive: 0,
            primals_expected: 13,
            depot_fresh: false,
            vcs_synced: false,
            mesh_peers: 0,
            last_seen: None,
            notes: String::new(),
        }
    }

    /// Derive readiness from observed metrics.
    #[must_use]
    pub const fn derived_readiness(&self) -> ReadinessLevel {
        if self.primals_alive == 0 {
            if self.last_seen.is_some() {
                ReadinessLevel::Reachable
            } else {
                ReadinessLevel::Offline
            }
        } else if self.primals_alive < self.primals_expected {
            ReadinessLevel::Partial
        } else if self.vcs_synced && self.depot_fresh {
            ReadinessLevel::Verified
        } else {
            ReadinessLevel::Full
        }
    }

    /// Update readiness to match derived state.
    pub const fn reconcile(&mut self) {
        self.readiness = self.derived_readiness();
    }
}

/// The full ecosystem gate readiness matrix.
#[derive(Debug, Clone, Default)]
pub struct GateMatrix {
    /// All tracked gates in the ecosystem.
    pub gates: Vec<GateStatus>,
}

impl GateMatrix {
    /// Create a new empty matrix.
    #[must_use]
    pub const fn new() -> Self {
        Self { gates: Vec::new() }
    }

    /// Build the current ecosystem matrix from known gates.
    #[must_use]
    pub fn ecosystem_snapshot() -> Self {
        let mut m = Self::new();
        m.gates.push(gate_from_env("eastGate"));
        m.gates.push(gate_from_env("sporeGate"));
        m.gates.push(gate_from_env("golgi"));
        m.gates.push(gate_from_env("pepti"));
        m.gates.push(gate_from_env("northGate"));
        m.gates.push(gate_from_env("fieldGate"));
        m.gates.push(gate_from_env("flockGate"));
        m
    }

    /// Count of gates at or above a given readiness level.
    #[must_use]
    pub fn count_at_level(&self, minimum: ReadinessLevel) -> usize {
        self.gates.iter().filter(|g| g.readiness >= minimum).count()
    }

    /// Total primals alive across all gates.
    #[must_use]
    pub fn total_primals_alive(&self) -> u32 {
        self.gates.iter().map(|g| u32::from(g.primals_alive)).sum()
    }

    /// Summary line suitable for display.
    #[must_use]
    pub fn summary(&self) -> String {
        let total = self.gates.len();
        let verified = self.count_at_level(ReadinessLevel::Verified);
        let full = self.count_at_level(ReadinessLevel::Full);
        let partial = self.count_at_level(ReadinessLevel::Partial);
        let alive = self.total_primals_alive();
        format!(
            "{total} gates: {verified} verified, {full} full+, {partial} partial+ | {alive} primals alive"
        )
    }
}

impl fmt::Display for GateMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "┌─ Gate Readiness Matrix ─────────────────────┐")?;
        for g in &self.gates {
            writeln!(
                f,
                "│ {:12} {:10} {:2}/{:2} primals {:>5} │",
                g.name,
                g.readiness.to_string(),
                g.primals_alive,
                g.primals_expected,
                if g.vcs_synced { "sync" } else { "drift" },
            )?;
        }
        writeln!(f, "├─────────────────────────────────────────────┤")?;
        writeln!(f, "│ {} │", self.summary())?;
        write!(f, "└─────────────────────────────────────────────┘")
    }
}

fn gate_from_env(name: &str) -> GateStatus {
    let env_key = format!("GATE_{}_STATUS", name.to_uppercase());
    let status_str = std::env::var(&env_key).unwrap_or_default();

    let readiness = match status_str.to_lowercase().as_str() {
        "verified" => ReadinessLevel::Verified,
        "full" => ReadinessLevel::Full,
        "partial" => ReadinessLevel::Partial,
        "reachable" => ReadinessLevel::Reachable,
        _ => ReadinessLevel::Offline,
    };

    GateStatus::new(name, readiness)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readiness_ordering() {
        assert!(ReadinessLevel::Verified > ReadinessLevel::Full);
        assert!(ReadinessLevel::Full > ReadinessLevel::Partial);
        assert!(ReadinessLevel::Partial > ReadinessLevel::Reachable);
        assert!(ReadinessLevel::Reachable > ReadinessLevel::Offline);
    }

    #[test]
    fn derived_readiness_logic() {
        let mut g = GateStatus::new("test", ReadinessLevel::Offline);
        assert_eq!(g.derived_readiness(), ReadinessLevel::Offline);

        g.last_seen = Some(1234);
        assert_eq!(g.derived_readiness(), ReadinessLevel::Reachable);

        g.primals_alive = 5;
        assert_eq!(g.derived_readiness(), ReadinessLevel::Partial);

        g.primals_alive = 13;
        assert_eq!(g.derived_readiness(), ReadinessLevel::Full);

        g.vcs_synced = true;
        g.depot_fresh = true;
        assert_eq!(g.derived_readiness(), ReadinessLevel::Verified);
    }

    #[test]
    fn matrix_snapshot_structure() {
        let m = GateMatrix::ecosystem_snapshot();
        assert_eq!(m.gates.len(), 7);
        assert!(m.gates.iter().any(|g| g.name == "eastGate"));
        assert!(m.gates.iter().any(|g| g.name == "golgi"));
    }

    #[test]
    fn matrix_summary_format() {
        let m = GateMatrix::ecosystem_snapshot();
        let s = m.summary();
        assert!(s.contains("7 gates"));
    }
}
