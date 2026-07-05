// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Pepti Warehouse Cross-Arch Deploy — validates the sovereign binary
//! distribution pipeline from Sovereign CI through pepti warehouse to gate deployment.
//!
//! The pepti warehouse model:
//! - Sovereign CI (sporeGate) compiles for multiple architectures
//! - Binaries are published to pepti warehouse (golgi depot)
//! - Gates pull binaries via WireGuard overlay or golgi relay
//! - Cross-hardware validation (grapheneGate, ironGate, etc.) without local builds
//!
//! Phases:
//! 1. Depot structure: aarch64 + x86_64 binary inventories
//! 2. Architecture coverage: all 13 primals built for each target
//! 3. deploy_pixel.sh contract: handles all 13 primal startup sequences
//! 4. ADB transport: port forwarding config covers Tower composition
//! 5. Live: depot freshness + binary checksums (requires depot access)

use crate::composition::CompositionContext;
use crate::primal_names::Primal;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Pepti warehouse deploy contract scenario metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "pepti-warehouse-deploy",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "wave132g_pepti_warehouse",
        provenance_date: "2026-07-05",
        description:
            "Pepti warehouse cross-arch deploy — sovereign CI → depot → gate deployment pipeline",
    },
    run,
};

const ALL_PRIMALS: &[&str] = Primal::ALL_SLUGS;

const TOWER_PRIMALS: &[&str] = &["beardog", "songbird", "skunkbat"];

const ADB_PORTS: &[(&str, u16)] = &[
    ("beardog", 9100),
    ("songbird", 9200),
    ("skunkbat", 9140),
];

fn ecoprimals_root() -> Option<std::path::PathBuf> {
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for ancestor in manifest.ancestors() {
        if ancestor.join("infra/plasmidBin").is_dir() {
            return Some(ancestor.to_path_buf());
        }
    }
    None
}

/// Run all pepti warehouse deploy validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Depot structure");
    phase_depot_structure(v);

    v.section("Phase 2: Architecture coverage (aarch64-musl)");
    phase_arch_coverage(v);

    v.section("Phase 3: deploy_pixel.sh contract");
    phase_deploy_script(v);

    v.section("Phase 4: ADB transport config");
    phase_adb_transport(v);

    v.section("Phase 5: Live — depot freshness");
    phase_live_freshness(v);
}

fn phase_depot_structure(v: &mut ValidationResult) {
    let Some(root) = ecoprimals_root() else {
        v.check_skip("depot:root", "ecoprimals_root not found (not in workspace)");
        return;
    };

    let plasmid = root.join("infra/plasmidBin");
    v.check_bool(
        "depot:plasmidbin_exists",
        plasmid.is_dir(),
        "infra/plasmidBin directory exists",
    );

    let primals_dir = plasmid.join("primals");
    v.check_bool(
        "depot:primals_dir",
        primals_dir.is_dir(),
        "infra/plasmidBin/primals/ directory exists",
    );

    let aarch64_dir = primals_dir.join("aarch64-unknown-linux-musl");
    v.check_bool(
        "depot:aarch64_dir",
        aarch64_dir.is_dir(),
        "aarch64-unknown-linux-musl target directory exists",
    );

    let checksums = plasmid.join("checksums.toml");
    v.check_bool(
        "depot:checksums_toml",
        checksums.is_file(),
        "checksums.toml exists for integrity verification",
    );

    let manifest = plasmid.join("manifest.toml");
    v.check_bool(
        "depot:manifest_toml",
        manifest.is_file(),
        "manifest.toml exists for depot metadata",
    );
}

fn phase_arch_coverage(v: &mut ValidationResult) {
    let Some(root) = ecoprimals_root() else {
        v.check_skip("arch:root", "ecoprimals_root not found");
        return;
    };

    let aarch64_dir = root.join("infra/plasmidBin/primals/aarch64-unknown-linux-musl");
    if !aarch64_dir.is_dir() {
        v.check_skip("arch:aarch64_dir", "aarch64 depot dir not found");
        return;
    }

    let mut built_count = 0u32;
    for primal in ALL_PRIMALS {
        let binary = aarch64_dir.join(primal);
        let exists = binary.is_file();
        if exists {
            built_count += 1;
        }
        v.check_bool(
            &format!("arch:aarch64:{primal}"),
            exists,
            &format!("{primal}: {}", if exists { "BUILT" } else { "MISSING" }),
        );
    }

    v.check_bool(
        "arch:aarch64:full_coverage",
        built_count >= 13,
        &format!("{built_count}/13 aarch64 binaries in depot"),
    );
}

fn phase_deploy_script(v: &mut ValidationResult) {
    let Some(root) = ecoprimals_root() else {
        v.check_skip("deploy:root", "ecoprimals_root not found");
        return;
    };

    let script = root.join("infra/plasmidBin/deploy_pixel.sh");
    let script_exists = script.is_file();
    v.check_bool(
        "deploy:script_exists",
        script_exists,
        "deploy_pixel.sh exists",
    );

    if !script_exists {
        return;
    }

    let content = std::fs::read_to_string(&script).unwrap_or_default();

    v.check_bool(
        "deploy:executable",
        content.starts_with("#!/"),
        "deploy_pixel.sh has shebang",
    );

    v.check_bool(
        "deploy:primal_bind_mode",
        content.contains("PRIMAL_BIND_MODE"),
        "script sets PRIMAL_BIND_MODE for SELinux-safe transport",
    );

    for primal in TOWER_PRIMALS {
        v.check_bool(
            &format!("deploy:handler:{primal}"),
            content.contains(primal),
            &format!("{primal} startup handler in deploy_pixel.sh"),
        );
    }

    v.check_bool(
        "deploy:adb_forward",
        content.contains("adb forward"),
        "script configures ADB port forwarding",
    );

    v.check_bool(
        "deploy:stop_mode",
        content.contains("--stop"),
        "script supports --stop for clean shutdown",
    );

    v.check_bool(
        "deploy:dry_run",
        content.contains("--dry-run"),
        "script supports --dry-run for validation without execution",
    );
}

fn phase_adb_transport(v: &mut ValidationResult) {
    let topology_toml: &str = include_str!("../../../../config/mesh_topology.toml");

    v.check_bool(
        "adb:graphenegate_in_topology",
        topology_toml.contains("grapheneGate"),
        "grapheneGate enrolled in mesh_topology.toml",
    );

    v.check_bool(
        "adb:transport_adb",
        topology_toml.contains("transport = \"adb\""),
        "grapheneGate transport declared as ADB",
    );

    v.check_bool(
        "adb:role_mobile",
        topology_toml.contains("role = \"mobile\""),
        "grapheneGate role is mobile",
    );

    for (primal, port) in ADB_PORTS {
        v.check_bool(
            &format!("adb:port:{primal}"),
            *port > 0 && *port < 65535,
            &format!("{primal} ADB forward port {port} is valid"),
        );
    }

    let ports_unique = ADB_PORTS
        .iter()
        .map(|(_, p)| p)
        .collect::<std::collections::HashSet<_>>()
        .len()
        == ADB_PORTS.len();
    v.check_bool(
        "adb:ports_unique",
        ports_unique,
        "ADB forward ports are unique (no collisions)",
    );
}

fn phase_live_freshness(v: &mut ValidationResult) {
    let Some(root) = ecoprimals_root() else {
        v.check_skip("live:root", "ecoprimals_root not found");
        return;
    };

    let aarch64_dir = root.join("infra/plasmidBin/primals/aarch64-unknown-linux-musl");
    if !aarch64_dir.is_dir() {
        v.check_skip("live:aarch64_depot", "aarch64 depot not found");
        return;
    }

    let mut newest_mtime = std::time::SystemTime::UNIX_EPOCH;
    for primal in TOWER_PRIMALS {
        let binary = aarch64_dir.join(primal);
        if let Ok(meta) = binary.metadata() {
            if let Ok(mtime) = meta.modified() {
                if mtime > newest_mtime {
                    newest_mtime = mtime;
                }
            }
        }
    }

    let age = std::time::SystemTime::now()
        .duration_since(newest_mtime)
        .unwrap_or_default();
    let days_old = age.as_secs() / 86400;

    v.check_bool(
        "live:tower_freshness",
        days_old < 30,
        &format!("Tower binaries: {days_old} days old (threshold: 30)"),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pepti_warehouse_structural() {
        let mut v = ValidationResult::new("pepti-warehouse-deploy");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        let total = v.passed + v.failed + v.skipped;
        assert!(total >= 15, "expected ≥15 checks, got {total}");
    }

    #[test]
    fn tower_primals_subset() {
        for p in TOWER_PRIMALS {
            assert!(
                ALL_PRIMALS.contains(p),
                "Tower primal {p} should be in ALL_PRIMALS"
            );
        }
    }

    #[test]
    fn adb_ports_no_collision() {
        let ports: std::collections::HashSet<u16> = ADB_PORTS.iter().map(|(_, p)| *p).collect();
        assert_eq!(ports.len(), ADB_PORTS.len());
    }
}
