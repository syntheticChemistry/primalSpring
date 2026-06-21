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

/// Cytoplasm zone — physical topology grouping within the plasma membrane.
///
/// Maps to the K-Derm model where the cytoplasm is segmented into zones
/// by physical location and switching fabric. Gates in the same zone share
/// L2 connectivity; cross-zone traffic traverses backbone links.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CytoplasmZone {
    /// Hub 1: CRS310 backbone, sporeGate plasma membrane, 10G fabric.
    Backbone,
    /// Hub 2: Omada SX3008F (standalone L2), Flint 2 WiFi, house 2 gates.
    House2,
    /// Hub 3: Garage, planned compute + outdoor WiFi.
    Garage,
    /// WAN: gates outside the plasma membrane (VPS, offsite, public internet).
    Wan,
    /// Unknown or unassigned zone.
    Unassigned,
}

impl CytoplasmZone {
    /// Derive zone from gate name using `config/mesh_topology.toml`.
    ///
    /// Falls back to `Unassigned` for gates not in the topology config.
    #[must_use]
    pub fn for_gate(gate_name: &str) -> Self {
        MESH_REGISTRY
            .iter()
            .find(|e| e.name == gate_name)
            .map_or(Self::Unassigned, |e| Self::parse_zone(&e.zone))
    }

    /// Parse a zone string from TOML config.
    fn parse_zone(s: &str) -> Self {
        match s {
            "Backbone" => Self::Backbone,
            "House2" => Self::House2,
            "Garage" => Self::Garage,
            "Wan" => Self::Wan,
            _ => Self::Unassigned,
        }
    }

    /// Short label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Backbone => "backbone",
            Self::House2 => "house2",
            Self::Garage => "garage",
            Self::Wan => "wan",
            Self::Unassigned => "unassigned",
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
    /// Cytoplasm zone (physical topology grouping).
    pub zone: CytoplasmZone,
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
        let n: String = name.into();
        let zone = CytoplasmZone::for_gate(&n);
        Self {
            name: n,
            readiness,
            zone,
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

/// Embedded mesh topology TOML (single source of truth for address assignments).
const MESH_TOML: &str = include_str!("../../../config/mesh_topology.toml");

/// TOML-derived mesh address registry, built once at first access.
static MESH_REGISTRY: std::sync::LazyLock<Vec<MeshEntry>> = std::sync::LazyLock::new(|| {
    let Ok(parsed) = MESH_TOML.parse::<toml::Table>() else {
        return Vec::new();
    };
    let Some(gates) = parsed.get("gate").and_then(toml::Value::as_array) else {
        return Vec::new();
    };
    gates
        .iter()
        .filter_map(|g| {
            let t = g.as_table()?;
            let name = t.get("name")?.as_str()?.to_owned();
            let address = t.get("address").and_then(|v| v.as_str()).unwrap_or("").to_owned();
            let role = t.get("role").and_then(|v| v.as_str()).unwrap_or("").to_owned();
            let zone = t.get("zone").and_then(|v| v.as_str()).unwrap_or("").to_owned();
            Some(MeshEntry { name, address, role, zone })
        })
        .collect()
});

/// A single gate's mesh topology entry.
#[derive(Debug, Clone)]
pub struct MeshEntry {
    /// Gate name (e.g. `"eastGate"`).
    pub name: String,
    /// WireGuard overlay address (e.g. `"10.13.37.5"`).
    pub address: String,
    /// Gate role (e.g. `"meta"`, `"hub"`, `"tower"`).
    pub role: String,
    /// K-Derm zone (e.g. `"Backbone"`, `"Wan"`).
    pub zone: String,
}

/// WireGuard mesh address assignments (10.13.37.0/24 overlay).
///
/// Reads from `config/mesh_topology.toml` (SSOT). Adding a gate means adding
/// one `[[gate]]` entry to the TOML — no code changes required.
/// Returns `None` for gates that haven't been peered yet (no address field).
#[must_use]
pub fn mesh_address(gate_name: &str) -> Option<&'static str> {
    MESH_REGISTRY
        .iter()
        .find(|e| e.name == gate_name)
        .map(|e| e.address.as_str())
        .filter(|addr| !addr.is_empty())
}

/// All gates with assigned mesh addresses.
#[must_use]
pub fn all_mesh_gates() -> &'static [MeshEntry] {
    &MESH_REGISTRY
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

    /// Enumerate all known gates in the ecosystem readiness matrix.
    ///
    /// Alias for [`Self::ecosystem_snapshot`] — used by validation scenarios
    /// that iterate gates for cross-gate capability checks.
    #[must_use]
    pub fn enumerate() -> Self {
        Self::ecosystem_snapshot()
    }

    /// Build the current ecosystem matrix from `config/mesh_topology.toml`.
    ///
    /// All gates in the topology config are included regardless of mesh status.
    #[must_use]
    pub fn ecosystem_snapshot() -> Self {
        let mut m = Self::new();
        for entry in MESH_REGISTRY.iter() {
            m.gates.push(gate_from_env(&entry.name));
        }
        m
    }

    /// Build the local gate assessment and add it to the ecosystem matrix.
    #[must_use]
    pub fn with_local_assessment() -> Self {
        let mut m = Self::ecosystem_snapshot();
        let local = local_assessment();
        if let Some(existing) = m.gates.iter_mut().find(|g| g.name == local.name) {
            *existing = local;
        } else {
            m.gates.push(local);
        }
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

/// Probe the local machine and build a self-assessment `GateStatus`.
///
/// Checks:
/// - Gate name from `GATE_NAME` env var (fallback: hostname)
/// - Socket presence in biomeos runtime dir (primal liveness)
/// - Git workspace cleanliness (VCS sync proxy)
/// - Mesh peers from env configuration
#[must_use]
pub fn local_assessment() -> GateStatus {
    let name = std::env::var("GATE_NAME")
        .or_else(|_| crate::tolerances::platform::hostname().ok_or(()))
        .unwrap_or_else(|()| "local".to_owned());

    let runtime_dir = crate::tolerances::platform::runtime_dir();
    let biomeos_dir = std::path::Path::new(&runtime_dir).join(crate::env_keys::BIOMEOS_SUBDIR);

    let primals_alive = if biomeos_dir.is_dir() {
        std::fs::read_dir(&biomeos_dir).map_or(0, |rd| {
            rd.filter_map(Result::ok)
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "sock"))
                .count()
        })
    } else {
        0
    };

    let vcs_synced = std::process::Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .is_ok_and(|o| o.status.success() && o.stdout.is_empty());

    let mesh_peers = std::env::var(crate::env_keys::MESH_PEERS)
        .or_else(|_| {
            #[expect(deprecated, reason = "fallback for backward compatibility")]
            std::env::var(crate::env_keys::SONGBIRD_PEERS)
        })
        .map_or(0, |p| p.split(',').filter(|s| !s.is_empty()).count());

    #[expect(
        clippy::cast_possible_truncation,
        reason = "primal/peer counts fit in u8"
    )]
    let mut status = GateStatus {
        zone: CytoplasmZone::for_gate(&name),
        name,
        readiness: ReadinessLevel::Offline,
        primals_alive: primals_alive as u8,
        primals_expected: 13,
        depot_fresh: true,
        vcs_synced,
        mesh_peers: mesh_peers as u8,
        last_seen: Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        ),
        notes: "local self-assessment".to_owned(),
    };

    status.reconcile();
    status
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
        assert_eq!(m.gates.len(), 11);
        assert!(m.gates.iter().any(|g| g.name == "eastGate"));
        assert!(m.gates.iter().any(|g| g.name == "golgi"));
        assert!(m.gates.iter().any(|g| g.name == "ironGate"));
        assert!(m.gates.iter().any(|g| g.name == "swiftGate"));
    }

    #[test]
    fn matrix_summary_format() {
        let m = GateMatrix::ecosystem_snapshot();
        let s = m.summary();
        assert!(s.contains("11 gates"));
    }
}
