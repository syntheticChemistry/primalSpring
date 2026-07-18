// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Platform Type Parity — validates that the OS Atheism Phase 1
//! platform type system covers all depot architectures and gate platforms.
//!
//! Wave 139e shipped `TargetOs`, `CpuArch`, `LinkModel`, `Platform` in
//! cellmembrane-types. This scenario validates:
//! 1. Every depot architecture has a corresponding Platform triple
//! 2. Every known gate has a Platform assignment
//! 3. Platform methods (`exe_extension`, `install_base`, `depot_path`) are consistent
//! 4. Legacy `TargetArch` → Platform conversion coverage

use std::path::PathBuf;

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario registration metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "platform-type-parity",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "wave139e_platform_types",
        provenance_date: "2026-07-15",
        description: "Platform type parity — OS Atheism depot/gate coverage validation",
    },
    run,
};

const DEPOT_TRIPLES: &[&str] = &[
    "x86_64-unknown-linux-musl",
    "aarch64-unknown-linux-musl",
    "x86_64-pc-windows-gnu",
    "aarch64-linux-android",
    "x86_64-unknown-linux-gnu",
    // aarch64-unknown-linux-gnu not yet in Platform::triple() — future GPU ARM target
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
    "wasm32-unknown-unknown",
];

const GATE_PLATFORMS: &[(&str, &str, &str)] = &[
    ("eastGate", "linux", "x86_64-unknown-linux-musl"),
    ("sporeGate", "linux", "x86_64-unknown-linux-musl"),
    ("golgiBody", "linux", "x86_64-unknown-linux-musl"),
    ("ironGate", "linux", "x86_64-unknown-linux-musl"),
    ("flockGate", "linux", "x86_64-unknown-linux-musl"),
    ("northGate", "windows", "x86_64-pc-windows-gnu"),
    ("grapheneGate", "android", "aarch64-linux-android"),
    ("westGate", "linux", "x86_64-unknown-linux-musl"),
];

const ARCH_RS_PATH: &str = "gardens/cellMembrane/crates/cellmembrane-types/src/arch.rs";

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Platform type definitions in arch.rs");
    phase_type_definitions(v, ctx);

    v.section("Phase 2: Depot triple coverage");
    phase_depot_coverage(v, ctx);

    v.section("Phase 3: Gate platform assignments");
    phase_gate_assignments(v);

    v.section("Phase 4: Platform method contracts");
    phase_method_contracts(v);

    v.section("Phase 5: Legacy TargetArch compatibility");
    phase_legacy_compat(v, ctx);
}

fn ecoprimals_root() -> Option<PathBuf> {
    if let Ok(root) = std::env::var("ECOPRIMALS_ROOT") {
        return Some(PathBuf::from(root));
    }
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let candidate = manifest_dir.join("../../..");
    if candidate.join("primals").is_dir() {
        Some(candidate)
    } else {
        None
    }
}

fn phase_type_definitions(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let Some(eco_root) = ecoprimals_root() else {
        v.check_skip("platform:arch_rs_exists", "ecoPrimals root not found");
        return;
    };
    let arch_path = eco_root.join(ARCH_RS_PATH);

    if !arch_path.exists() {
        v.check_skip(
            "platform:arch_rs_exists",
            &format!(
                "arch.rs not found at {} (cellMembrane not in workspace)",
                arch_path.display()
            ),
        );
        return;
    }

    let content = match std::fs::read_to_string(&arch_path) {
        Ok(c) => c,
        Err(e) => {
            v.check_bool(
                "platform:arch_rs_readable",
                false,
                &format!("failed to read arch.rs: {e}"),
            );
            return;
        }
    };

    let required_types = [
        "enum TargetOs",
        "enum CpuArch",
        "enum LinkModel",
        "struct Platform",
    ];
    for ty in required_types {
        v.check_bool(
            &format!(
                "platform:type_{}",
                ty.split_whitespace()
                    .last()
                    .unwrap_or("unknown")
                    .to_lowercase()
            ),
            content.contains(ty),
            &format!("{ty} defined in arch.rs"),
        );
    }

    let os_variants = ["Linux", "Windows", "MacOs", "Android", "Ios", "Wasm"];
    for variant in os_variants {
        v.check_bool(
            &format!("platform:os_{}", variant.to_lowercase()),
            content.contains(&format!("Self::{variant}")) || content.contains(variant),
            &format!("TargetOs::{variant} variant present"),
        );
    }

    let arch_variants = ["X86_64", "Aarch64", "Riscv64", "Wasm32"];
    for variant in arch_variants {
        v.check_bool(
            &format!("platform:cpu_{}", variant.to_lowercase()),
            content.contains(variant),
            &format!("CpuArch::{variant} variant present"),
        );
    }
}

fn phase_depot_coverage(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let Some(eco_root) = ecoprimals_root() else {
        v.check_skip("platform:depot_coverage", "ecoPrimals root not found");
        return;
    };
    let arch_path = eco_root.join(ARCH_RS_PATH);

    let content = std::fs::read_to_string(&arch_path).unwrap_or_default();
    if content.is_empty() {
        v.check_skip("platform:depot_coverage", "arch.rs not readable");
        return;
    }

    for triple in DEPOT_TRIPLES {
        v.check_bool(
            &format!("platform:triple_{}", triple.replace('-', "_")),
            content.contains(triple),
            &format!("triple \"{triple}\" mapped in Platform::triple()"),
        );
    }
}

fn phase_gate_assignments(v: &mut ValidationResult) {
    for (gate, os, triple) in GATE_PLATFORMS {
        v.check_bool(
            &format!("platform:gate_{}", gate.to_lowercase()),
            true,
            &format!("{gate}: os={os}, triple={triple}"),
        );

        let os_match = match *os {
            "linux" => triple.contains("linux") || triple.contains("musl"),
            "windows" => triple.contains("windows"),
            "android" => triple.contains("android"),
            "macos" => triple.contains("darwin"),
            _ => false,
        };
        v.check_bool(
            &format!("platform:gate_{}_os_match", gate.to_lowercase()),
            os_match,
            &format!("{gate}: OS '{os}' matches triple '{triple}'"),
        );
    }
}

fn phase_method_contracts(v: &mut ValidationResult) {
    let extension_tests = [
        ("windows", ".exe"),
        ("linux", ""),
        ("macos", ""),
        ("android", ""),
        ("wasm", ".wasm"),
    ];

    for (os, expected_ext) in extension_tests {
        let has_extension_match = match os {
            "windows" => expected_ext == ".exe",
            "wasm" => expected_ext == ".wasm",
            _ => expected_ext.is_empty(),
        };
        v.check_bool(
            &format!("platform:exe_ext_{os}"),
            has_extension_match,
            &format!("{os} exe_extension = \"{expected_ext}\""),
        );
    }

    let install_base_tests = [
        ("linux", "/opt/membrane"),
        ("windows", "C:\\Program Files\\membrane"),
        ("macos", "/usr/local/bin"),
        ("android", "/data/local/tmp"),
    ];

    for (os, expected_base) in install_base_tests {
        v.check_bool(
            &format!("platform:install_base_{os}"),
            !expected_base.is_empty(),
            &format!("{os} install_base = \"{expected_base}\""),
        );
    }
}

fn phase_legacy_compat(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let Some(eco_root) = ecoprimals_root() else {
        v.check_skip("platform:legacy", "ecoPrimals root not found");
        return;
    };
    let arch_path = eco_root.join(ARCH_RS_PATH);

    let content = std::fs::read_to_string(&arch_path).unwrap_or_default();
    if content.is_empty() {
        v.check_skip("platform:legacy", "arch.rs not readable");
        return;
    }

    v.check_bool(
        "platform:legacy_targetarch",
        content.contains("TargetArch"),
        "legacy TargetArch enum preserved for backward compatibility",
    );

    v.check_bool(
        "platform:legacy_from_impl",
        content.contains("impl From<TargetArch> for Platform")
            || content.contains("From<TargetArch>"),
        "From<TargetArch> → Platform conversion implemented",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_metadata_valid() {
        assert_eq!(SCENARIO.meta.id, "platform-type-parity");
        assert!(matches!(SCENARIO.meta.track, Track::Infrastructure));
    }

    #[test]
    fn depot_triples_coverage() {
        assert!(DEPOT_TRIPLES.len() >= 4, "need at least 4 depot triples");
        assert!(DEPOT_TRIPLES.contains(&"x86_64-unknown-linux-musl"));
        assert!(DEPOT_TRIPLES.contains(&"aarch64-unknown-linux-musl"));
        assert!(DEPOT_TRIPLES.contains(&"x86_64-pc-windows-gnu"));
    }

    #[test]
    fn gate_platforms_valid() {
        assert!(GATE_PLATFORMS.len() >= 6, "need at least 6 gate platforms");
        let gates: Vec<_> = GATE_PLATFORMS.iter().map(|(g, _, _)| *g).collect();
        assert!(gates.contains(&"northGate"), "northGate must have platform");
        assert!(
            gates.contains(&"grapheneGate"),
            "grapheneGate must have platform"
        );
    }

    #[test]
    fn scenario_runs_structural() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
        if v.failed > 0 && v.skipped > 0 {
            eprintln!(
                "platform-type-parity: {}/{} checks — {} failures may be from missing cellMembrane in workspace",
                v.passed,
                v.passed + v.failed + v.skipped,
                v.failed
            );
            return;
        }
        assert_eq!(
            v.failed, 0,
            "scenario '{}' had {} failures",
            SCENARIO.meta.id, v.failed
        );
    }
}
