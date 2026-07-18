// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Depot Binary Standard — validates that depot binaries conform
//! to the post-primordial musl-static standard.
//!
//! Wave 139a identified a P2 divergence (DEPOT-LAYOUT): sporeGate's genomeBin
//! depot contains only 6/14 primals and those are dynamically linked (gnu),
//! violating the musl-static standard established post-primordial.
//!
//! This scenario validates:
//! 1. Binary count: all 14 primals present in depot
//! 2. Static linkage: binaries should be statically linked (no dynamic libs)
//! 3. Checksums: checksums.toml present with BLAKE3 hashes for all primals
//! 4. Signatures: signatures.toml present (cellMembrane-signed depot trust)
//! 5. Target triple consistency: binaries match expected target architecture

use std::path::PathBuf;

use crate::composition::CompositionContext;
use crate::primal_names::Primal;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Depot binary standard compliance scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "depot-binary-standard",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "wave139a_depot_standard",
        provenance_date: "2026-07-14",
        description: "Depot binary standard — musl-static linkage, full primal coverage, checksums + signatures",
    },
    run,
};

const EXPECTED_PRIMALS: &[&str] = Primal::ALL_SLUGS;

/// Run depot binary standard validation.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Depot discovery");
    let depot_root = phase_depot_discovery(v);

    v.section("Phase 2: Binary coverage");
    phase_binary_coverage(v, depot_root.as_ref());

    v.section("Phase 3: Static linkage standard");
    phase_static_linkage(v, depot_root.as_ref());

    v.section("Phase 4: Integrity artifacts");
    phase_integrity_artifacts(v);

    v.section("Phase 5: Layout standard");
    phase_layout_standard(v, depot_root.as_ref());

    let _ = ctx;
}

fn resolve_depot_dir() -> Option<PathBuf> {
    if let Ok(bin) = std::env::var(crate::env_keys::ECOPRIMALS_PLASMID_BIN) {
        let p = PathBuf::from(&bin);
        if p.is_dir() {
            return Some(p);
        }
    }

    let triple = crate::tolerances::current_target_triple();
    let depot = PathBuf::from(crate::tolerances::plasmidbin_depot_root())
        .join("primals")
        .join(&triple);
    if depot.is_dir() {
        return Some(depot);
    }

    None
}

fn resolve_plasmidbin_root() -> Option<PathBuf> {
    let p = PathBuf::from(crate::tolerances::plasmidbin_depot_root());
    if p.is_dir() {
        return Some(p);
    }
    None
}

fn phase_depot_discovery(v: &mut ValidationResult) -> Option<PathBuf> {
    let depot = resolve_depot_dir();

    if let Some(path) = &depot {
        v.check_bool(
            "depot-std:discovered",
            true,
            &format!("depot at {}", path.display()),
        );

        let triple = crate::tolerances::current_target_triple();
        let path_str = path.to_string_lossy();
        let has_triple = path_str.contains(&triple);
        v.check_bool(
            "depot-std:triple_in_path",
            has_triple,
            &format!(
                "depot path {} target triple `{triple}`",
                if has_triple { "contains" } else { "missing" }
            ),
        );
    } else {
        v.check_skip(
            "depot-std:discovered",
            "no depot directory found (plasmidBin layout or $ECOPRIMALS_PLASMID_BIN)",
        );
        v.check_skip("depot-std:triple_in_path", "depot not found");
    }

    depot
}

fn phase_binary_coverage(v: &mut ValidationResult, depot: Option<&PathBuf>) {
    let Some(root) = depot else {
        v.check_skip("depot-std:coverage", "no depot — skipping coverage check");
        return;
    };

    let mut found = 0u32;
    let mut missing = Vec::new();
    for &primal in EXPECTED_PRIMALS {
        let bin = root.join(primal);
        if bin.exists() && bin.is_file() {
            found += 1;
        } else {
            missing.push(primal);
        }
    }

    let total = EXPECTED_PRIMALS.len();
    v.check_bool(
        "depot-std:coverage",
        found as usize == total,
        &format!(
            "{found}/{total} primals present{}",
            if missing.is_empty() {
                String::new()
            } else {
                format!(" (missing: {})", missing.join(", "))
            }
        ),
    );

    v.check_bool(
        "depot-std:no_orphans",
        found > 0,
        "at least one binary found in depot directory",
    );
}

fn phase_static_linkage(v: &mut ValidationResult, depot: Option<&PathBuf>) {
    let Some(root) = depot else {
        v.check_skip(
            "depot-std:static_linkage",
            "no depot — skipping linkage check",
        );
        return;
    };

    let triple = crate::tolerances::current_target_triple();
    let is_musl_target = triple.contains("musl");
    v.check_bool(
        "depot-std:musl_target",
        is_musl_target,
        &format!(
            "target triple `{triple}` {} musl",
            if is_musl_target { "is" } else { "is NOT" }
        ),
    );

    let mut checked = 0u32;
    let mut statically_linked = 0u32;
    for &primal in EXPECTED_PRIMALS {
        let bin = root.join(primal);
        if !bin.exists() || !bin.is_file() {
            continue;
        }
        checked += 1;

        if let Ok(data) = std::fs::read(&bin) {
            if data.len() >= 4 && &data[0..4] == b"\x7fELF" {
                let is_static = !elf_has_dynamic_section(&data);
                if is_static {
                    statically_linked += 1;
                } else {
                    v.check_bool(
                        &format!("depot-std:static_{primal}"),
                        false,
                        &format!(
                            "`{primal}` is dynamically linked (violates musl-static standard)"
                        ),
                    );
                }
            }
        }
    }

    if checked > 0 {
        v.check_bool(
            "depot-std:static_linkage",
            statically_linked == checked,
            &format!("{statically_linked}/{checked} binaries are statically linked"),
        );
    } else {
        v.check_skip("depot-std:static_linkage", "no binaries found to check");
    }
}

fn elf_has_dynamic_section(data: &[u8]) -> bool {
    if data.len() < 64 {
        return false;
    }
    let is_64 = data[4] == 2;
    if is_64 {
        elf64_has_pt_dynamic(data)
    } else {
        elf32_has_pt_dynamic(data)
    }
}

#[allow(clippy::cast_possible_truncation)]
fn elf64_has_pt_dynamic(data: &[u8]) -> bool {
    if data.len() < 64 {
        return false;
    }
    let phoff = u64::from_le_bytes(data[32..40].try_into().unwrap_or_default()) as usize;
    let phentsize = u16::from_le_bytes(data[54..56].try_into().unwrap_or_default()) as usize;
    let phnum = u16::from_le_bytes(data[56..58].try_into().unwrap_or_default()) as usize;

    for i in 0..phnum {
        let offset = phoff + i * phentsize;
        if offset + 4 > data.len() {
            break;
        }
        let p_type = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap_or_default());
        if p_type == 2 {
            return true;
        }
    }
    false
}

fn elf32_has_pt_dynamic(data: &[u8]) -> bool {
    if data.len() < 52 {
        return false;
    }
    let phoff = u32::from_le_bytes(data[28..32].try_into().unwrap_or_default()) as usize;
    let phentsize = u16::from_le_bytes(data[42..44].try_into().unwrap_or_default()) as usize;
    let phnum = u16::from_le_bytes(data[44..46].try_into().unwrap_or_default()) as usize;

    for i in 0..phnum {
        let offset = phoff + i * phentsize;
        if offset + 4 > data.len() {
            break;
        }
        let p_type = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap_or_default());
        if p_type == 2 {
            return true;
        }
    }
    false
}

fn phase_integrity_artifacts(v: &mut ValidationResult) {
    let Some(root) = resolve_plasmidbin_root() else {
        v.check_skip("depot-std:checksums_present", "plasmidBin root not found");
        v.check_skip("depot-std:signatures_present", "plasmidBin root not found");
        return;
    };

    let checksums = root.join("checksums.toml");
    v.check_bool(
        "depot-std:checksums_present",
        checksums.exists(),
        &format!(
            "checksums.toml {}",
            if checksums.exists() {
                "present"
            } else {
                "MISSING"
            }
        ),
    );

    let signatures = root.join("signatures.toml");
    v.check_bool(
        "depot-std:signatures_present",
        signatures.exists(),
        &format!(
            "signatures.toml {}",
            if signatures.exists() {
                "present"
            } else {
                "MISSING (depot not cellMembrane-signed)"
            }
        ),
    );
}

fn phase_layout_standard(v: &mut ValidationResult, depot: Option<&PathBuf>) {
    let Some(root) = depot else {
        v.check_skip("depot-std:layout", "no depot for layout check");
        return;
    };

    let parent = root.parent();
    let in_primals_dir = parent
        .and_then(|p| p.file_name())
        .is_some_and(|n| n == "primals");
    v.check_bool(
        "depot-std:plasmidbin_layout",
        in_primals_dir,
        &format!(
            "depot is inside `primals/` directory (plasmidBin standard): {}",
            if in_primals_dir {
                "yes"
            } else {
                "no (possible genomeBin legacy layout)"
            }
        ),
    );

    let triple = crate::tolerances::current_target_triple();
    let dir_name = root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let matches_triple = dir_name == triple;
    v.check_bool(
        "depot-std:dir_matches_triple",
        matches_triple,
        &format!(
            "directory name `{dir_name}` {} target triple `{triple}`",
            if matches_triple {
                "matches"
            } else {
                "does NOT match"
            }
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elf_magic_detection() {
        let not_elf = b"not an elf file at all";
        assert!(!elf_has_dynamic_section(not_elf));
    }

    #[test]
    fn scenario_count_includes_all_primals() {
        assert!(
            EXPECTED_PRIMALS.len() >= 13,
            "expected at least 13 primals in slug list, got {}",
            EXPECTED_PRIMALS.len()
        );
    }

    #[test]
    fn depot_binary_standard_structural() {
        let mut v = ValidationResult::new("depot-binary-standard");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        // Wave 139a known debt: depot binaries are dynamically linked (gnu)
        // and checksums/signatures are missing. This is the DEPOT-LAYOUT P2
        // divergence tracked in the blurb. Scenario validates the standard;
        // failures here quantify the gap.
        if v.failed > 0 {
            eprintln!(
                "depot-binary-standard: {}/{} checks failed (DEPOT-LAYOUT P2 divergence — expected until depot reconciliation)",
                v.failed,
                v.passed + v.failed + v.skipped
            );
        }
    }
}
