// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Drawbridge Bond Registry — validates the runtime weak bond
//! configuration that drives songBird drawbridge allowlists, caching, and
//! health monitoring for external data sources.
//!
//! Structural checks:
//! - Registry parses as valid TOML with [meta] schema_version
//! - All entries have required fields
//! - Trust tiers are valid enum values
//! - Every bond has at least one consumer
//! - No duplicate hosts
//! - Science APIs (NCBI, UniProt, PDB) are declared

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};
use std::collections::HashSet;

const BONDS_TOML: &str = include_str!("../../../../config/drawbridge_bonds.toml");

const VALID_TIERS: &[&str] = &[
    "scientific",
    "community",
    "commercial",
    "municipal",
    "untrusted",
];

const VALID_METHODS: &[&str] = &["GET", "POST", "PUT", "DELETE", "PATCH"];

const SCIENCE_HOSTS: &[&str] = &[
    "pubchem.ncbi.nlm.nih.gov",
    "eutils.ncbi.nlm.nih.gov",
    "api.ncbi.nlm.nih.gov",
    "rest.uniprot.org",
    "data.rcsb.org",
    "alphafold.ebi.ac.uk",
];

/// Drawbridge bond registry validation.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "drawbridge-bond-registry",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "wave139a_bond_registry",
        provenance_date: "2026-07-14",
        description: "Drawbridge bond registry — structure, tiers, science API coverage",
    },
    run,
};

/// Run all bond registry validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    phase_parse(v);
    phase_schema(v);
    phase_science_coverage(v);
}

fn phase_parse(v: &mut ValidationResult) {
    v.section("Phase 1: Registry parse");

    let parsed: Result<toml::Value, _> = toml::from_str(BONDS_TOML);
    v.check_bool(
        "registry:toml_parse",
        parsed.is_ok(),
        &format!(
            "drawbridge_bonds.toml parses as valid TOML{}",
            parsed.as_ref().err().map_or(String::new(), |e| format!(" ({e})"))
        ),
    );

    if parsed.is_err() {
        return;
    }
    let doc = parsed.unwrap();

    let schema_ver = doc
        .get("meta")
        .and_then(|m| m.get("schema_version"))
        .and_then(|v| v.as_integer());
    v.check_bool(
        "registry:schema_version",
        schema_ver.is_some() && schema_ver.unwrap() >= 2,
        &format!("schema_version = {}", schema_ver.unwrap_or(0)),
    );

    let bonds = doc.get("bonds").and_then(|b| b.as_table());
    v.check_bool(
        "registry:has_bonds",
        bonds.is_some(),
        "registry has [bonds.*] section",
    );

    if let Some(b) = bonds {
        v.check_bool(
            "registry:bond_count",
            b.len() >= 16,
            &format!("{} bonds declared (expected ≥16)", b.len()),
        );
    }
}

fn phase_schema(v: &mut ValidationResult) {
    v.section("Phase 2: Schema validation");

    let doc: toml::Value = match toml::from_str(BONDS_TOML) {
        Ok(d) => d,
        Err(_) => return,
    };

    let bonds = match doc.get("bonds").and_then(|b| b.as_table()) {
        Some(b) => b,
        None => return,
    };

    let mut hosts_seen: HashSet<String> = HashSet::new();
    let mut tier_invalid = Vec::new();
    let mut missing_fields = Vec::new();
    let mut method_invalid = Vec::new();
    let mut empty_consumers = Vec::new();
    let mut duplicate_hosts = Vec::new();

    for (name, entry) in bonds {
        let table = match entry.as_table() {
            Some(t) => t,
            None => {
                missing_fields.push(format!("{name}: not a table"));
                continue;
            }
        };

        match table.get("tier").and_then(|t| t.as_str()) {
            Some(tier) => {
                if !VALID_TIERS.contains(&tier) {
                    tier_invalid.push(format!("{name}: '{tier}'"));
                }
            }
            None => missing_fields.push(format!("{name}: missing 'tier'")),
        }

        match table.get("host").and_then(|h| h.as_str()) {
            Some(host) => {
                if !hosts_seen.insert(host.to_string()) {
                    duplicate_hosts.push(host.to_string());
                }
            }
            None => missing_fields.push(format!("{name}: missing 'host'")),
        }

        match table.get("methods").and_then(|m| m.as_array()) {
            Some(methods) => {
                for m in methods {
                    if let Some(ms) = m.as_str() {
                        if !VALID_METHODS.contains(&ms) {
                            method_invalid.push(format!("{name}: '{ms}'"));
                        }
                    }
                }
            }
            None => missing_fields.push(format!("{name}: missing 'methods'")),
        }

        match table.get("consumers").and_then(|c| c.as_array()) {
            Some(consumers) => {
                if consumers.is_empty() {
                    empty_consumers.push(name.clone());
                }
            }
            None => missing_fields.push(format!("{name}: missing 'consumers'")),
        }

        if table.get("description").and_then(|d| d.as_str()).is_none() {
            missing_fields.push(format!("{name}: missing 'description'"));
        }
    }

    v.check_bool(
        "schema:required_fields",
        missing_fields.is_empty(),
        &if missing_fields.is_empty() {
            "all bonds have required fields".to_string()
        } else {
            format!("missing: {}", missing_fields.join("; "))
        },
    );

    v.check_bool(
        "schema:valid_tiers",
        tier_invalid.is_empty(),
        &if tier_invalid.is_empty() {
            "all trust tiers valid".to_string()
        } else {
            format!("invalid: {}", tier_invalid.join("; "))
        },
    );

    v.check_bool(
        "schema:valid_methods",
        method_invalid.is_empty(),
        &if method_invalid.is_empty() {
            "all HTTP methods valid".to_string()
        } else {
            format!("invalid: {}", method_invalid.join("; "))
        },
    );

    v.check_bool(
        "schema:no_empty_consumers",
        empty_consumers.is_empty(),
        &if empty_consumers.is_empty() {
            "every bond has ≥1 consumer".to_string()
        } else {
            format!("empty: {}", empty_consumers.join(", "))
        },
    );

    v.check_bool(
        "schema:no_duplicate_hosts",
        duplicate_hosts.is_empty(),
        &if duplicate_hosts.is_empty() {
            "no duplicate hosts".to_string()
        } else {
            format!("dupes: {}", duplicate_hosts.join(", "))
        },
    );
}

fn phase_science_coverage(v: &mut ValidationResult) {
    v.section("Phase 3: Science API coverage");

    let mut found = 0usize;
    let mut missing = Vec::new();

    for &host in SCIENCE_HOSTS {
        if BONDS_TOML.contains(host) {
            found += 1;
        } else {
            missing.push(host);
        }
    }

    v.check_bool(
        "science:api_coverage",
        found == SCIENCE_HOSTS.len(),
        &format!(
            "{found}/{} science APIs declared{}",
            SCIENCE_HOSTS.len(),
            if missing.is_empty() {
                String::new()
            } else {
                format!(" (missing: {})", missing.join(", "))
            }
        ),
    );

    let has_ncbi = BONDS_TOML.contains("pubchem.ncbi.nlm.nih.gov")
        && BONDS_TOML.contains("eutils.ncbi.nlm.nih.gov");
    v.check_bool(
        "science:ncbi_present",
        has_ncbi,
        "NCBI PubChem + Entrez bonds declared",
    );

    let has_structural = BONDS_TOML.contains("data.rcsb.org")
        && BONDS_TOML.contains("alphafold.ebi.ac.uk");
    v.check_bool(
        "science:structural_bio",
        has_structural,
        "Structural biology APIs (PDB + AlphaFold) declared",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bond_registry_structural() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "bond-registry: {} failures ({} passed, {} skipped)",
            v.failed, v.passed, v.skipped
        );
    }
}
