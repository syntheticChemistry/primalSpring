// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Profile constraint parsing — reads deployment profile TOML and extracts
//! evolution-relevant constraints for fitness evaluation.
//!
//! A profile declares what a deployment target can and cannot do:
//! - `max_memory_mb`: RAM ceiling
//! - `max_primals`: composition size limit
//! - `transport`: TCP-only, UDS-preferred, or mixed
//! - `read_only_root`: whether filesystem writes are restricted
//!
//! The fitness evaluator uses these constraints to classify targets
//! and determine whether a primal composition fits within the profile.

use super::pressure::PressureCategory;
use super::target::{CompositionTier, Target};

/// Parsed constraints from a deployment profile TOML.
#[derive(Debug, Clone)]
pub struct ProfileConstraints {
    /// Profile name (e.g. "fieldmouse", "graphenegate").
    pub name: String,
    /// Target triple (if declared).
    pub target_triple: Option<String>,
    /// Atomic type string (e.g. "tower", "micro").
    pub atomic_type: String,
    /// Transport mode (e.g. "`tcp_only`", "`tcp_enabled`").
    pub transport: String,
    /// Maximum primals (from `[composition.constraints]`).
    pub max_primals: Option<usize>,
    /// Maximum memory in MB.
    pub max_memory_mb: Option<usize>,
    /// Single-core restriction.
    pub single_core: bool,
    /// Read-only root filesystem.
    pub read_only_root: bool,
    /// Required primal slugs.
    pub required_primals: Vec<String>,
    /// Optional primal slugs.
    pub optional_primals: Vec<String>,
    /// Whether federation/mesh enrollment is enabled.
    pub mesh_enabled: bool,
}

impl ProfileConstraints {
    /// Parse constraints from profile TOML content.
    #[must_use]
    pub fn from_toml(content: &str) -> Option<Self> {
        let parsed: toml::Table = content.parse().ok()?;

        let gate = parsed.get("gate")?.as_table()?;
        let name = gate
            .get("name")
            .and_then(toml::Value::as_str)
            .unwrap_or("")
            .to_owned();
        let target_triple = gate
            .get("target_triple")
            .and_then(toml::Value::as_str)
            .filter(|s| !s.is_empty())
            .map(str::to_owned);

        let composition = parsed.get("composition")?.as_table()?;
        let atomic_type = composition
            .get("atomic_type")
            .and_then(|v| v.as_str())
            .unwrap_or("tower")
            .to_owned();
        let transport = composition
            .get("transport")
            .and_then(|v| v.as_str())
            .unwrap_or("tcp_enabled")
            .to_owned();

        let constraints = composition.get("constraints").and_then(|v| v.as_table());

        let max_primals = constraints
            .and_then(|c| c.get("max_primals"))
            .and_then(toml::Value::as_integer)
            .and_then(|v| usize::try_from(v).ok());

        let max_memory_mb = constraints
            .and_then(|c| c.get("max_memory_mb"))
            .and_then(toml::Value::as_integer)
            .and_then(|v| usize::try_from(v).ok());

        let single_core = constraints
            .and_then(|c| c.get("single_core"))
            .and_then(toml::Value::as_bool)
            .unwrap_or(false);

        let read_only_root = constraints
            .and_then(|c| c.get("read_only_root"))
            .and_then(toml::Value::as_bool)
            .unwrap_or(false);

        let primals = composition.get("primals").and_then(|v| v.as_table());
        let required_primals = primals
            .and_then(|p| p.get("required"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default();
        let optional_primals = primals
            .and_then(|p| p.get("optional"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default();

        let mesh = parsed.get("mesh").and_then(toml::Value::as_table);
        let mesh_enabled = mesh
            .and_then(|m| m.get("enroll"))
            .and_then(toml::Value::as_bool)
            .unwrap_or(false);

        Some(Self {
            name,
            target_triple,
            atomic_type,
            transport,
            max_primals,
            max_memory_mb,
            single_core,
            read_only_root,
            required_primals,
            optional_primals,
            mesh_enabled,
        })
    }

    /// Derive the composition tier from these constraints.
    #[must_use]
    #[expect(
        clippy::missing_const_for_fn,
        reason = "match on Option<usize> not const-stable"
    )]
    pub fn composition_tier(&self) -> CompositionTier {
        match self.max_primals {
            Some(1..=2) => CompositionTier::Micro,
            Some(3..=5) => CompositionTier::Light,
            Some(6..=10) => CompositionTier::Standard,
            _ => CompositionTier::Full,
        }
    }

    /// Infer active selection pressures from these constraints.
    #[must_use]
    pub fn active_pressures(&self) -> Vec<PressureCategory> {
        let mut pressures = Vec::new();

        if self.read_only_root {
            pressures.push(PressureCategory::Filesystem);
        }
        if self.transport == "tcp_only" {
            pressures.push(PressureCategory::IpcTransport);
        }
        if self.max_memory_mb.is_some_and(|m| m <= 512) {
            pressures.push(PressureCategory::Memory);
        }
        if self.single_core {
            pressures.push(PressureCategory::Concurrency);
        }
        if !self.mesh_enabled {
            pressures.push(PressureCategory::Network);
        }

        pressures
    }

    /// Whether a given primal count fits within this profile.
    #[must_use]
    pub fn fits_primal_count(&self, count: usize) -> bool {
        self.max_primals.is_none_or(|max| count <= max)
    }

    /// Resolve the target (if triple is declared).
    #[must_use]
    pub fn target(&self) -> Target {
        match self.target_triple.as_deref() {
            Some(t) if t.contains("aarch64") => Target::Aarch64Musl,
            Some(t) if t.contains("riscv64") => Target::Riscv64Musl,
            Some(t) if t.contains("wasm") => Target::Wasm32Wasi,
            _ => Target::X86_64Musl,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIELDMOUSE_TOML: &str = include_str!("../../../config/profiles/fieldmouse.toml");
    const GRAPHENEGATE_TOML: &str = include_str!("../../../config/profiles/graphenegate.toml");

    #[test]
    fn parse_fieldmouse_profile() {
        let constraints =
            ProfileConstraints::from_toml(FIELDMOUSE_TOML).expect("fieldmouse.toml should parse");
        assert_eq!(constraints.atomic_type, "micro");
        assert_eq!(constraints.transport, "tcp_only");
        assert_eq!(constraints.max_primals, Some(2));
        assert_eq!(constraints.max_memory_mb, Some(256));
        assert!(constraints.single_core);
        assert!(constraints.read_only_root);
        assert!(!constraints.mesh_enabled);
        assert_eq!(constraints.required_primals, vec!["beardog"]);
        assert_eq!(constraints.composition_tier(), CompositionTier::Micro);
    }

    #[test]
    fn parse_graphenegate_profile() {
        let constraints = ProfileConstraints::from_toml(GRAPHENEGATE_TOML)
            .expect("graphenegate.toml should parse");
        assert_eq!(constraints.atomic_type, "tower");
        assert_eq!(constraints.transport, "tcp_enabled");
        assert!(
            constraints
                .target_triple
                .as_deref()
                .unwrap()
                .contains("aarch64")
        );
        assert!(constraints.mesh_enabled);
        assert_eq!(constraints.target(), Target::Aarch64Musl);
    }

    #[test]
    fn fieldmouse_pressures() {
        let constraints = ProfileConstraints::from_toml(FIELDMOUSE_TOML).unwrap();
        let pressures = constraints.active_pressures();
        assert!(pressures.contains(&PressureCategory::Filesystem));
        assert!(pressures.contains(&PressureCategory::IpcTransport));
        assert!(pressures.contains(&PressureCategory::Memory));
        assert!(pressures.contains(&PressureCategory::Concurrency));
        assert!(pressures.contains(&PressureCategory::Network));
    }

    #[test]
    fn fits_primal_count() {
        let constraints = ProfileConstraints::from_toml(FIELDMOUSE_TOML).unwrap();
        assert!(constraints.fits_primal_count(1));
        assert!(constraints.fits_primal_count(2));
        assert!(!constraints.fits_primal_count(3));
    }
}
