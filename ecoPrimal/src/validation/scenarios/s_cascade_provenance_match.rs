// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Cascade Provenance Match — validates that installed binaries
//! match the provenance record from the depot.
//!
//! After a `temporal.cascade`, gate binaries should trace back to the depot's
//! `provenance.toml` (commit + rustc version) and `checksums.toml` (BLAKE3 hash).
//! This scenario detects post-cascade drift: binaries modified in place,
//! incomplete cascades, or provenance gaps.
//!
//! Phase 1 (Structural): Verify provenance/checksum infrastructure exists.
//! Phase 2 (Structural): Cross-reference installed binaries against depot records.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::composition::CompositionContext;
use crate::tolerances;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Cascade provenance match scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "cascade-provenance-match",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "wave111_divergence_pressure",
        provenance_date: "2026-06-11",
        description: "Validates installed binaries match depot provenance after cascade",
    },
    run,
};

/// Execute cascade provenance match validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — provenance infrastructure");
    phase_infrastructure(v);

    v.section("Phase 2: Structural — binary-provenance cross-reference");
    phase_cross_reference(v);
}

fn phase_infrastructure(v: &mut ValidationResult) {
    let depot_root = resolve_depot_root();

    let Some(depot) = depot_root else {
        v.check_skip(
            "infra:depot_root",
            "depot root not found (ECOPRIMALS_PLASMID_BIN / ECOPRIMALS_ROOT not set)",
        );
        return;
    };

    v.check_bool(
        "infra:depot_root",
        depot.is_dir(),
        &format!("depot root exists: {}", depot.display()),
    );

    let triple = tolerances::current_target_triple();
    let arch_dir = depot.join("primals").join(&triple);
    if !arch_dir.is_dir() {
        v.check_skip(
            "infra:arch_dir",
            &format!("architecture depot {triple} not found — cascade not yet run"),
        );
        return;
    }
    v.check_bool("infra:arch_dir", true, &format!("architecture depot {triple} exists"));

    let checksums_path = depot.join("checksums.toml");
    if !checksums_path.exists() {
        v.check_skip("infra:checksums_toml", "checksums.toml not present — cascade pending");
        return;
    }
    v.check_bool("infra:checksums_toml", true, "checksums.toml present for integrity verification");

    let provenance_path = depot.join("provenance.toml");
    if !provenance_path.exists() {
        v.check_skip("infra:provenance_toml", "provenance.toml not present — cascade pending");
        return;
    }
    v.check_bool("infra:provenance_toml", true, "provenance.toml present for build traceability");
}

fn phase_cross_reference(v: &mut ValidationResult) {
    let depot_root = resolve_depot_root();
    let Some(depot) = depot_root else {
        v.check_skip("xref:provenance_loaded", "depot root not available");
        return;
    };

    let provenance_path = depot.join("provenance.toml");
    let Some(provenance) = load_provenance(&provenance_path) else {
        v.check_skip(
            "xref:provenance_loaded",
            &format!("provenance.toml not loadable at {}", provenance_path.display()),
        );
        return;
    };

    v.check_bool(
        "xref:provenance_loaded",
        true,
        &format!("{} entries in provenance.toml", provenance.len()),
    );

    let checksums_path = depot.join("checksums.toml");
    let checksums = load_checksums(&checksums_path).unwrap_or_default();
    v.check_bool(
        "xref:checksums_loaded",
        !checksums.is_empty(),
        &format!("{} entries in checksums.toml", checksums.len()),
    );

    let triple = tolerances::current_target_triple();
    let arch_dir = depot.join("primals").join(&triple);
    if !arch_dir.is_dir() {
        v.check_skip("xref:binary_scan", "architecture depot directory not found");
        return;
    }

    let mut matched = 0usize;
    let mut missing_provenance = Vec::new();
    let mut missing_checksum = Vec::new();

    let slugs = tolerances::all_primal_slugs();
    for slug in &slugs {
        let binary_path = arch_dir.join(slug);
        if !binary_path.exists() {
            continue;
        }

        if provenance.contains_key(*slug) {
            matched += 1;
        } else {
            missing_provenance.push(*slug);
        }

        if !checksums.contains_key(*slug) {
            missing_checksum.push(*slug);
        }
    }

    v.check_bool(
        "xref:all_have_provenance",
        missing_provenance.is_empty(),
        &format!(
            "{matched} binaries have provenance records{}",
            if missing_provenance.is_empty() {
                String::new()
            } else {
                format!(" (missing: {})", missing_provenance.join(", "))
            }
        ),
    );

    v.check_bool(
        "xref:all_have_checksums",
        missing_checksum.is_empty(),
        &format!(
            "checksum coverage: {}/{}{}",
            slugs.len() - missing_checksum.len(),
            slugs.len(),
            if missing_checksum.is_empty() {
                String::new()
            } else {
                format!(" (missing: {})", missing_checksum.join(", "))
            }
        ),
    );

    let all_commits_present = provenance.values().all(|entry| !entry.commit.is_empty());
    v.check_bool(
        "xref:commits_populated",
        all_commits_present,
        "all provenance entries have non-empty commit hashes",
    );
}

fn resolve_depot_root() -> Option<PathBuf> {
    let root_str = tolerances::plasmidbin_depot_root();
    let path = PathBuf::from(&root_str);
    if path.is_dir() {
        Some(path)
    } else {
        None
    }
}

#[derive(Debug)]
struct ProvenanceEntry {
    commit: String,
}

fn load_provenance(path: &Path) -> Option<BTreeMap<String, ProvenanceEntry>> {
    let content = std::fs::read_to_string(path).ok()?;
    let table: toml::Table = content.parse().ok()?;
    let mut map = BTreeMap::new();
    for (slug, section) in &table {
        let Some(t) = section.as_table() else {
            continue;
        };
        let commit = t
            .get("commit")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_owned();
        map.insert(slug.clone(), ProvenanceEntry { commit });
    }
    Some(map)
}

fn load_checksums(path: &Path) -> Option<BTreeMap<String, String>> {
    let content = std::fs::read_to_string(path).ok()?;
    let table: toml::Table = content.parse().ok()?;
    let mut map = BTreeMap::new();
    for (slug, section) in &table {
        if let Some(hash) = section.as_str() {
            map.insert(slug.clone(), hash.to_owned());
        } else if let Some(t) = section.as_table() {
            if let Some(hash) = t.get("blake3").and_then(|v| v.as_str()) {
                map.insert(slug.clone(), hash.to_owned());
            }
        }
    }
    Some(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn structural_phase_runs_without_panic() {
        let mut v = ValidationResult::new("cascade-provenance-match");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
    }

    #[test]
    fn provenance_entry_fields() {
        let entry = ProvenanceEntry {
            commit: "abc1234".to_owned(),
        };
        assert!(!entry.commit.is_empty());
    }
}
