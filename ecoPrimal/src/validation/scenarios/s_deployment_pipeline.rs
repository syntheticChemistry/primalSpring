// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Deployment Pipeline — validates the full depot-to-deploy chain.
//!
//! Ensures that any gate (LAN or WAN, VPS or desktop) can reliably deploy
//! from the plasmidBin depot with consistent structural guarantees:
//!
//! 1. Depot Layout — binary directory exists with the correct target triple
//! 2. Checksums — `checksums.toml` parses with per-primal BLAKE3 hashes
//! 3. Provenance — `provenance.toml` has commit metadata for each primal
//! 4. Cell Readiness — VPS-standard cell graphs pass structural validation
//! 5. Live (optional) — depot binaries resolve and health-check

use std::path::{Path, PathBuf};

use crate::composition::CompositionContext;
use crate::deploy;
use crate::tolerances::current_target_triple;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Deployment-pipeline validation: depot layout, checksums, provenance, cell readiness, live deploy.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "deployment-pipeline",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "wave93_primalspring",
        provenance_date: "2026-06-07",
        description: "Deployment pipeline — depot layout, checksums, provenance, cell readiness, live deploy",
    },
    run,
};

use crate::primal_names::Primal;

const EXPECTED_PRIMALS: &[&str] = Primal::ALL_SLUGS;

fn resolve_depot_root() -> Option<PathBuf> {
    if let Ok(bin) = std::env::var(crate::env_keys::ECOPRIMALS_PLASMID_BIN) {
        let p = PathBuf::from(&bin);
        if p.is_dir() {
            return Some(p);
        }
    }

    if std::env::var(crate::env_keys::ECOPRIMALS_ROOT).is_ok() {
        let triple = crate::tolerances::current_target_triple();
        let p = PathBuf::from(crate::tolerances::plasmidbin_depot_root())
            .join("primals")
            .join(&triple);
        if p.is_dir() {
            return Some(p);
        }
    }

    None
}

fn resolve_plasmidbin_root() -> Option<PathBuf> {
    let p = PathBuf::from(crate::tolerances::plasmidbin_depot_root());
    if p.join("checksums.toml").exists() {
        return Some(p);
    }

    let relative = Path::new("../../infra/plasmidBin");
    if relative.join("checksums.toml").exists() {
        return Some(relative.to_path_buf());
    }

    None
}

/// Execute the full deployment-pipeline validation across five phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Depot layout");
    let depot_root = phase_depot_layout(v);

    v.section("Phase 2: Checksums integrity");
    phase_checksums(v);

    v.section("Phase 3: Provenance metadata");
    phase_provenance(v);

    v.section("Phase 4: Cell graph deployment readiness");
    phase_cell_readiness(v);

    v.section("Phase 5: Live — depot binary resolution");
    phase_live_binary_resolution(v, ctx, depot_root.as_ref());
}

fn phase_depot_layout(v: &mut ValidationResult) -> Option<PathBuf> {
    let depot_root = resolve_depot_root();

    if let Some(root) = &depot_root {
        v.check_bool(
            "depot:directory_exists",
            true,
            &format!("depot at {}", root.display()),
        );

        let mut found = 0u32;
        let mut missing = Vec::new();
        for &primal in EXPECTED_PRIMALS {
            let bin = root.join(primal);
            if bin.exists() {
                found += 1;
            } else {
                missing.push(primal);
            }
        }

        v.check_bool(
            "depot:primal_count",
            found as usize == EXPECTED_PRIMALS.len(),
            &format!(
                "{found}/{} primals in depot{}",
                EXPECTED_PRIMALS.len(),
                if missing.is_empty() {
                    String::new()
                } else {
                    format!(" (missing: {})", missing.join(", "))
                }
            ),
        );
    } else {
        v.check_skip(
            "depot:directory_exists",
            "no depot found ($ECOPRIMALS_PLASMID_BIN or $ECOPRIMALS_ROOT/infra/plasmidBin)",
        );
        v.check_skip("depot:primal_count", "no depot");
    }

    depot_root
}

fn phase_checksums(v: &mut ValidationResult) {
    let pb_root = resolve_plasmidbin_root();

    let Some(root) = pb_root else {
        v.check_skip("checksums:parse", "plasmidBin root not found");
        return;
    };

    let checksums_path = root.join("checksums.toml");
    match std::fs::read_to_string(&checksums_path) {
        Ok(content) => {
            let parsed: Result<toml::Value, _> = toml::from_str(&content);
            match parsed {
                Ok(table) => {
                    v.check_bool("checksums:parse", true, "checksums.toml valid TOML");

                    let triple = current_target_triple();
                    let entries = table
                        .get(&triple)
                        .and_then(|t| t.as_table())
                        .map_or(0, toml::value::Table::len);

                    v.check_bool(
                        "checksums:entry_count",
                        entries >= EXPECTED_PRIMALS.len(),
                        &format!(
                            "{entries} entries for {triple} (expected {}+)",
                            EXPECTED_PRIMALS.len()
                        ),
                    );

                    if let Some(target_table) = table.get(&triple).and_then(|t| t.as_table()) {
                        for &primal in EXPECTED_PRIMALS {
                            let has_hash = target_table
                                .get(primal)
                                .and_then(|v| v.get("blake3"))
                                .and_then(|v| v.as_str())
                                .is_some_and(|h| !h.is_empty());
                            v.check_bool(
                                &format!("checksums:{primal}"),
                                has_hash,
                                if has_hash {
                                    "blake3 present"
                                } else {
                                    "missing blake3 hash"
                                },
                            );
                        }
                    }
                }
                Err(e) => {
                    v.check_bool("checksums:parse", false, &format!("parse error: {e}"));
                }
            }
        }
        Err(e) => {
            v.check_bool(
                "checksums:parse",
                false,
                &format!("cannot read {}: {e}", checksums_path.display()),
            );
        }
    }
}

fn phase_provenance(v: &mut ValidationResult) {
    let pb_root = resolve_plasmidbin_root();

    let Some(root) = pb_root else {
        v.check_skip("provenance:parse", "plasmidBin root not found");
        return;
    };

    let prov_path = root.join("provenance.toml");
    match std::fs::read_to_string(&prov_path) {
        Ok(content) => {
            let parsed: Result<toml::Value, _> = toml::from_str(&content);
            match parsed {
                Ok(table) => {
                    v.check_bool("provenance:parse", true, "provenance.toml valid TOML");

                    let mut with_commit = 0u32;
                    for &primal in EXPECTED_PRIMALS {
                        let has_commit = table
                            .get(primal)
                            .and_then(|v| v.get("commit"))
                            .and_then(|v| v.as_str())
                            .is_some_and(|c| !c.is_empty());
                        if has_commit {
                            with_commit += 1;
                        }
                    }

                    v.check_bool(
                        "provenance:commit_coverage",
                        with_commit as usize >= EXPECTED_PRIMALS.len() - 1,
                        &format!(
                            "{with_commit}/{} primals have commit hashes",
                            EXPECTED_PRIMALS.len()
                        ),
                    );
                }
                Err(e) => {
                    v.check_bool("provenance:parse", false, &format!("parse error: {e}"));
                }
            }
        }
        Err(e) => {
            v.check_bool(
                "provenance:parse",
                false,
                &format!("cannot read {}: {e}", prov_path.display()),
            );
        }
    }
}

fn phase_cell_readiness(v: &mut ValidationResult) {
    let cells_dir = Path::new("graphs/cells");
    if !cells_dir.exists() {
        v.check_skip("cells:readiness", "graphs/cells/ not found");
        return;
    }

    let manifest_path = cells_dir.join("cells_manifest.toml");
    let vps_cells: Vec<String> = std::fs::read_to_string(&manifest_path)
        .ok()
        .and_then(|content| content.parse::<toml::Value>().ok())
        .and_then(|table| {
            table.get("cells").and_then(|c| c.as_array()).map(|arr| {
                arr.iter()
                    .filter(|c| {
                        c.get("vps_standard")
                            .and_then(toml::Value::as_bool)
                            .unwrap_or(false)
                    })
                    .filter_map(|c| c.get("file").and_then(|f| f.as_str()).map(String::from))
                    .collect()
            })
        })
        .unwrap_or_default();

    v.check_bool(
        "cells:vps_standard_count",
        !vps_cells.is_empty(),
        &format!("{} VPS-standard cells in manifest", vps_cells.len()),
    );

    let mut structurally_clean = 0u32;
    let total = u32::try_from(vps_cells.len()).unwrap_or(u32::MAX);

    for cell_file in &vps_cells {
        let cell_path = cells_dir.join(cell_file);
        if !cell_path.exists() {
            v.check_bool(
                &format!("cells:exists:{cell_file}"),
                false,
                "manifest references missing file",
            );
            continue;
        }

        let result = deploy::validate_structure(&cell_path);
        if result.parsed && result.issues.is_empty() {
            structurally_clean += 1;
        } else if !result.parsed {
            v.check_bool(
                &format!("cells:parse:{cell_file}"),
                false,
                "failed to parse",
            );
        }
    }

    v.check_bool(
        "cells:vps_structural_clean",
        structurally_clean == total,
        &format!("{structurally_clean}/{total} VPS cells structurally clean"),
    );
}

fn phase_live_binary_resolution(
    v: &mut ValidationResult,
    _ctx: &mut CompositionContext,
    depot_root: Option<&PathBuf>,
) {
    let Some(_root) = depot_root else {
        v.check_skip("live:binary_resolution", "no depot — skipping live phase");
        return;
    };

    let mut resolved = 0u32;
    let mut unresolved = Vec::new();

    for &primal in EXPECTED_PRIMALS {
        match crate::launcher::discover_binary(primal) {
            Ok(_) => resolved += 1,
            Err(_) => unresolved.push(primal),
        }
    }

    v.check_bool(
        "live:binary_resolution",
        resolved as usize == EXPECTED_PRIMALS.len(),
        &format!(
            "{resolved}/{} primals resolved via discover_binary{}",
            EXPECTED_PRIMALS.len(),
            if unresolved.is_empty() {
                String::new()
            } else {
                format!(" (unresolved: {})", unresolved.join(", "))
            }
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_runs_without_panic() {
        let mut v = ValidationResult::new("test-deployment-pipeline");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        let total = v.passed + v.failed + v.skipped;
        assert!(total > 0, "deployment pipeline should produce checks");
    }

    #[test]
    fn target_triple_is_reasonable() {
        let triple = current_target_triple();
        assert!(
            triple.contains("linux") || triple.contains("darwin") || triple.contains("windows"),
            "unexpected target triple: {triple}"
        );
    }

    #[test]
    fn expected_primals_sorted() {
        let mut sorted = EXPECTED_PRIMALS.to_vec();
        sorted.sort_unstable();
        assert_eq!(
            EXPECTED_PRIMALS,
            &sorted[..],
            "EXPECTED_PRIMALS should be alphabetically sorted"
        );
    }
}
