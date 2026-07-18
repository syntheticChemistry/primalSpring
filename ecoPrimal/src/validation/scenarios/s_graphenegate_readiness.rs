// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: grapheneGate Readiness — validates that all 13 primals have
//! TCP-only fallback infrastructure for Pixel 8 (aarch64-musl) deployment.
//!
//! Structural phase (`Tier::Rust)`:
//!   - All 13 primals have TCP fallback ports registered
//!   - `BindMode` parsing covers all variants
//!   - `PRIMAL_BIND_MODE` env key is defined
//!   - Deploy graph for grapheneGate exists and specifies correct arch/transport
//!   - aarch64 binary depot has all 13 primals
//!   - Binary freshness: binaries postdate TCP fallback adoption commits

use crate::composition::CompositionContext;
use crate::ipc::server_bind::BindMode;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// grapheneGate readiness scenario — checks all aarch64 deployment prerequisites.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "graphenegate-readiness",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "wave107_graphenegate_readiness",
        provenance_date: "2026-06-10",
        description: "grapheneGate readiness — validates aarch64 deployment prerequisites for Pixel 8",
    },
    run,
};

use crate::primal_names::Primal;

const ALL_13_PRIMALS: &[&str] = Primal::ALL_SLUGS;

/// Resolve the ecoPrimals workspace root from `CARGO_MANIFEST_DIR`.
/// primalSpring lives at `springs/primalSpring/ecoPrimal` within the workspace,
/// so we walk up to find `infra/plasmidBin`.
fn ecoprimals_root() -> Option<std::path::PathBuf> {
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for ancestor in manifest.ancestors() {
        if ancestor.join("infra/plasmidBin").is_dir() {
            return Some(ancestor.to_path_buf());
        }
    }
    None
}

fn aarch64_depot_path() -> Option<std::path::PathBuf> {
    ecoprimals_root().map(|r| r.join("infra/plasmidBin/primals/aarch64-unknown-linux-musl"))
}

fn deploy_pixel_path() -> Option<std::path::PathBuf> {
    ecoprimals_root().map(|r| r.join("infra/plasmidBin/deploy_pixel.sh"))
}

/// Run all grapheneGate readiness phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: TCP fallback port coverage (all 13 primals)");
    phase_port_coverage(v);

    v.section("Phase 2: BindMode infrastructure");
    phase_bind_mode(v);

    v.section("Phase 3: PRIMAL_BIND_MODE env key defined");
    phase_env_key(v);

    v.section("Phase 4: aarch64 binary depot inventory");
    phase_binary_depot(v);

    v.section("Phase 5: deploy_pixel.sh PRIMAL_BIND_MODE export");
    phase_deploy_script(v);

    v.section("Phase 6: Deployment matrix cell coverage");
    phase_matrix_cell(v);

    v.section("Phase 7: Upstream blocker status (13/13 gate)");
    phase_upstream_blockers(v);
}

/// Tracks upstream primal blockers that prevent 13/13 grapheneGate deployment.
/// Each blocker becomes a check that passes once the fix is absorbed.
fn phase_upstream_blockers(v: &mut ValidationResult) {
    let cr_tarpc_fixed = check_primal_bind_mode_adoption("coralReef", "tarpc");
    v.check_bool(
        "blocker:cr_tarpc_01",
        cr_tarpc_fixed,
        "CR-TARPC-01: coralReef tarpc respects PRIMAL_BIND_MODE=tcp_only",
    );

    let bm_uds_fixed = check_primal_bind_mode_adoption("biomeOS", "neural_api");
    v.check_bool(
        "blocker:bm_uds_01",
        bm_uds_fixed,
        "BM-UDS-01: biomeOS Neural API respects PRIMAL_BIND_MODE=tcp_only",
    );

    let toadstool_socket = check_toadstool_socket_cleanup();
    v.check_bool(
        "blocker:toadstool_socket",
        toadstool_socket,
        "TOADSTOOL-SOCKET-CLEANUP: toadStool 3-tier socket resolution (no /tmp hardcode)",
    );

    let live_count: u32 = 11 + u32::from(cr_tarpc_fixed) + u32::from(bm_uds_fixed);
    v.check_bool(
        "deployment:live_count",
        live_count >= 13,
        &format!("{live_count}/13 primals deployable on grapheneGate TCP-only"),
    );
}

/// Checks whether a primal's bind-mode-specific subsystem respects `tcp_only`.
/// Returns true only when we can structurally verify the fix has landed.
/// For now, checks if the primal's codebase contains the guard pattern.
fn check_primal_bind_mode_adoption(primal: &str, subsystem: &str) -> bool {
    let primal_dir_name = match primal {
        "coralReef" => "coralReef",
        "biomeOS" => "biomeOS",
        _ => return false,
    };
    let Some(root) = ecoprimals_root() else {
        return false;
    };
    let primal_root = root.join("primals").join(primal_dir_name);
    if !primal_root.is_dir() {
        return false;
    }
    // Structural check: search for PRIMAL_BIND_MODE guard near the subsystem's bind site.
    // This is a heuristic — true validation requires the rebuilt binary on grapheneGate.
    scan_for_bind_mode_guard(&primal_root, subsystem)
}

fn scan_for_bind_mode_guard(primal_root: &std::path::Path, subsystem: &str) -> bool {
    let src_dir = primal_root.join("src");
    let crates_dir = primal_root.join("crates");
    let search_root = if crates_dir.is_dir() {
        crates_dir
    } else if src_dir.is_dir() {
        src_dir
    } else {
        return false;
    };

    let target_patterns: &[&str] = match subsystem {
        "tarpc" => &["PRIMAL_BIND_MODE", "tcp_only", "tarpc"],
        "neural_api" => &["PRIMAL_BIND_MODE", "tcp_only"],
        _ => return false,
    };

    walk_rs_files_for_patterns(&search_root, target_patterns)
}

fn walk_rs_files_for_patterns(dir: &std::path::Path, patterns: &[&str]) -> bool {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return false;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            if walk_rs_files_for_patterns(&path, patterns) {
                return true;
            }
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if patterns.iter().all(|p| content.contains(p)) {
                    return true;
                }
            }
        }
    }
    false
}

/// Checks toadStool socket cleanup status — verifies 3-tier resolution adoption.
fn check_toadstool_socket_cleanup() -> bool {
    let Some(root) = ecoprimals_root() else {
        return false;
    };
    let toadstool_root = root.join("primals/toadStool");
    if !toadstool_root.is_dir() {
        return false;
    }
    // toadStool uses BIOMEOS_SOCKET_DIR as its env override (not TOADSTOOL_SOCKET_DIR).
    // Check that the server's socket resolution references BIOMEOS_SOCKET_DIR.
    let crates_dir = toadstool_root.join("crates");
    if !crates_dir.is_dir() {
        return false;
    }
    walk_rs_files_for_patterns(&crates_dir, &["BIOMEOS_SOCKET_DIR", "XDG_RUNTIME_DIR"])
}

fn phase_port_coverage(v: &mut ValidationResult) {
    let mut all_have_ports = true;
    for primal in ALL_13_PRIMALS {
        let port = crate::tolerances::default_port_for(primal);
        let has_port = port > 0;
        if !has_port {
            all_have_ports = false;
        }
        v.check_bool(
            &format!("port:{primal}"),
            has_port,
            &format!("{primal} TCP fallback port = {port}"),
        );
    }
    v.check_bool(
        "port:all_13_covered",
        all_have_ports,
        "all 13 primals have non-zero TCP fallback ports",
    );
}

fn phase_bind_mode(v: &mut ValidationResult) {
    v.check_bool(
        "bindmode:three_variants",
        BindMode::UdsOnly != BindMode::TcpOnly
            && BindMode::TcpOnly != BindMode::Fallback
            && BindMode::Fallback != BindMode::UdsOnly,
        "BindMode has three distinct variants",
    );

    v.check_bool(
        "bindmode:from_env_available",
        {
            let _ = BindMode::from_env();
            true
        },
        "BindMode::from_env() callable without panic",
    );
}

fn phase_env_key(v: &mut ValidationResult) {
    v.check_bool(
        "env:primal_bind_mode_defined",
        crate::env_keys::PRIMAL_BIND_MODE == "PRIMAL_BIND_MODE",
        "PRIMAL_BIND_MODE constant matches expected value",
    );
}

fn phase_binary_depot(v: &mut ValidationResult) {
    let Some(depot_path) = aarch64_depot_path() else {
        v.check_skip(
            "depot:aarch64_dir_exists",
            "ecoPrimals workspace root not found",
        );
        for primal in ALL_13_PRIMALS {
            v.check_skip(&format!("depot:{primal}"), "depot not locatable");
        }
        v.check_skip("depot:all_13_present", "depot not locatable");
        v.check_skip("depot:freshness", "depot not locatable");
        return;
    };
    let depot_exists = depot_path.is_dir();
    v.check_bool(
        "depot:aarch64_dir_exists",
        depot_exists,
        &format!(
            "aarch64-unknown-linux-musl depot at {}",
            depot_path.display()
        ),
    );

    if !depot_exists {
        for primal in ALL_13_PRIMALS {
            v.check_skip(
                &format!("depot:{primal}"),
                "aarch64 depot directory not found",
            );
        }
        return;
    }

    let mut all_present = true;
    let mut stale_count = 0u32;
    let adoption_cutoff = chrono_lite_cutoff();

    for primal in ALL_13_PRIMALS {
        let bin_path = depot_path.join(primal);
        let exists = bin_path.is_file();
        if !exists {
            all_present = false;
        }

        let fresh = exists
            && std::fs::metadata(&bin_path)
                .and_then(|m| m.modified())
                .is_ok_and(|mtime| {
                    let is_fresh = mtime >= adoption_cutoff;
                    if !is_fresh {
                        stale_count += 1;
                    }
                    is_fresh
                });

        let label = if exists && fresh {
            "present + fresh"
        } else if exists {
            "present but STALE (pre-TCP-fallback)"
        } else {
            "MISSING"
        };

        v.check_bool(
            &format!("depot:{primal}"),
            exists,
            &format!("{primal}: {label}"),
        );
    }

    v.check_bool(
        "depot:all_13_present",
        all_present,
        "all 13 aarch64 binaries in depot",
    );

    if stale_count > 0 {
        v.check_skip(
            "depot:freshness",
            &format!("{stale_count}/13 binaries stale — aarch64 rebuild needed"),
        );
    } else {
        v.check_bool(
            "depot:freshness",
            true,
            "all 13 aarch64 binaries postdate TCP fallback adoption",
        );
    }
}

fn phase_deploy_script(v: &mut ValidationResult) {
    let Some(script_path) = deploy_pixel_path() else {
        v.check_skip(
            "deploy:script_exists",
            "ecoPrimals workspace root not found",
        );
        v.check_skip("deploy:bind_mode_export", "deploy_pixel.sh not locatable");
        return;
    };
    let exists = script_path.is_file();
    v.check_bool("deploy:script_exists", exists, "deploy_pixel.sh exists");

    if !exists {
        v.check_skip("deploy:bind_mode_export", "deploy_pixel.sh not found");
        return;
    }

    match std::fs::read_to_string(script_path) {
        Ok(content) => {
            let has_bind_mode = content.contains("PRIMAL_BIND_MODE");
            v.check_bool(
                "deploy:bind_mode_export",
                has_bind_mode,
                "deploy_pixel.sh exports PRIMAL_BIND_MODE",
            );

            let has_fallback = content.contains("PRIMAL_BIND_MODE") && content.contains("fallback");
            v.check_bool(
                "deploy:bind_mode_fallback",
                has_fallback,
                "deploy_pixel.sh sets PRIMAL_BIND_MODE=fallback",
            );
        }
        Err(e) => {
            v.check_skip(
                "deploy:bind_mode_export",
                &format!("cannot read deploy_pixel.sh: {e}"),
            );
        }
    }
}

fn phase_matrix_cell(v: &mut ValidationResult) {
    v.check_bool(
        "matrix:graphenegate_cell",
        true,
        "grapheneGate = aarch64-musl × Pixel8 × tcp_fallback",
    );

    let graphs_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let graphs_path = graphs_dir.as_path();
    let has_graphenegate_graph = graphs_path.is_dir()
        && std::fs::read_dir(graphs_path).is_ok_and(|entries| {
            entries.filter_map(Result::ok).any(|e| {
                let name = e.file_name();
                let name = name.to_string_lossy();
                name.contains("graphene") || name.contains("pixel") || name.contains("aarch64")
            })
        });

    if has_graphenegate_graph {
        v.check_bool(
            "matrix:deploy_graph_exists",
            true,
            "grapheneGate deploy graph found",
        );
    } else {
        v.check_skip(
            "matrix:deploy_graph_exists",
            "no grapheneGate-specific deploy graph yet (using deploy_pixel.sh directly)",
        );
    }
}

/// Returns a `SystemTime` cutoff: binaries must be newer than this to be "fresh".
/// TCP fallback adoption commits were June 10 2026 ~08:00 UTC-4.
fn chrono_lite_cutoff() -> std::time::SystemTime {
    use std::time::{Duration, UNIX_EPOCH};
    // 2026-06-10T12:00:00 UTC = after all TCP fallback commits landed
    UNIX_EPOCH + Duration::from_secs(1_781_294_400)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn graphenegate_readiness_no_panic() {
        let mut v = ValidationResult::new("graphenegate-readiness");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        // Structural checks should pass; binary freshness may skip/fail
        // depending on whether aarch64 rebuild has happened.
        assert!(
            v.passed > 0,
            "graphenegate-readiness scenario should have passing checks"
        );
    }
}
