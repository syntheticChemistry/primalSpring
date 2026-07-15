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

    let Ok(doc) = parsed else {
        return;
    };

    let schema_ver = doc
        .get("meta")
        .and_then(|m| m.get("schema_version"))
        .and_then(toml::Value::as_integer);
    v.check_bool(
        "registry:schema_version",
        schema_ver.unwrap_or(0) >= 2,
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

struct SchemaAudit {
    hosts_seen: HashSet<String>,
    tier_invalid: Vec<String>,
    missing_fields: Vec<String>,
    method_invalid: Vec<String>,
    empty_consumers: Vec<String>,
    duplicate_hosts: Vec<String>,
}

impl SchemaAudit {
    fn new() -> Self {
        Self {
            hosts_seen: HashSet::new(),
            tier_invalid: Vec::new(),
            missing_fields: Vec::new(),
            method_invalid: Vec::new(),
            empty_consumers: Vec::new(),
            duplicate_hosts: Vec::new(),
        }
    }

    fn audit_bond(&mut self, name: &str, table: &toml::map::Map<String, toml::Value>) {
        match table.get("tier").and_then(|t| t.as_str()) {
            Some(tier) if !VALID_TIERS.contains(&tier) => {
                self.tier_invalid.push(format!("{name}: '{tier}'"));
            }
            None => self.missing_fields.push(format!("{name}: missing 'tier'")),
            _ => {}
        }

        match table.get("host").and_then(|h| h.as_str()) {
            Some(host) => {
                if !self.hosts_seen.insert(host.to_string()) {
                    self.duplicate_hosts.push(host.to_string());
                }
            }
            None => self.missing_fields.push(format!("{name}: missing 'host'")),
        }

        if let Some(methods) = table.get("methods").and_then(|m| m.as_array()) {
            for m in methods {
                if let Some(ms) = m.as_str() {
                    if !VALID_METHODS.contains(&ms) {
                        self.method_invalid.push(format!("{name}: '{ms}'"));
                    }
                }
            }
        } else {
            self.missing_fields.push(format!("{name}: missing 'methods'"));
        }

        if let Some(consumers) = table.get("consumers").and_then(|c| c.as_array()) {
            if consumers.is_empty() {
                self.empty_consumers.push(name.to_owned());
            }
        } else {
            self.missing_fields.push(format!("{name}: missing 'consumers'"));
        }

        if table.get("description").and_then(|d| d.as_str()).is_none() {
            self.missing_fields.push(format!("{name}: missing 'description'"));
        }
    }

    fn emit(self, v: &mut ValidationResult) {
        v.check_bool(
            "schema:required_fields",
            self.missing_fields.is_empty(),
            &if self.missing_fields.is_empty() {
                "all bonds have required fields".to_string()
            } else {
                format!("missing: {}", self.missing_fields.join("; "))
            },
        );
        v.check_bool(
            "schema:valid_tiers",
            self.tier_invalid.is_empty(),
            &if self.tier_invalid.is_empty() {
                "all trust tiers valid".to_string()
            } else {
                format!("invalid: {}", self.tier_invalid.join("; "))
            },
        );
        v.check_bool(
            "schema:valid_methods",
            self.method_invalid.is_empty(),
            &if self.method_invalid.is_empty() {
                "all HTTP methods valid".to_string()
            } else {
                format!("invalid: {}", self.method_invalid.join("; "))
            },
        );
        v.check_bool(
            "schema:no_empty_consumers",
            self.empty_consumers.is_empty(),
            &if self.empty_consumers.is_empty() {
                "every bond has ≥1 consumer".to_string()
            } else {
                format!("empty: {}", self.empty_consumers.join(", "))
            },
        );
        v.check_bool(
            "schema:no_duplicate_hosts",
            self.duplicate_hosts.is_empty(),
            &if self.duplicate_hosts.is_empty() {
                "no duplicate hosts".to_string()
            } else {
                format!("dupes: {}", self.duplicate_hosts.join(", "))
            },
        );
    }
}

fn phase_schema(v: &mut ValidationResult) {
    v.section("Phase 2: Schema validation");

    let Ok(doc) = toml::from_str::<toml::Value>(BONDS_TOML) else {
        return;
    };
    let Some(bonds) = doc.get("bonds").and_then(|b| b.as_table()) else {
        return;
    };

    let mut audit = SchemaAudit::new();
    for (name, entry) in bonds {
        let Some(table) = entry.as_table() else {
            audit.missing_fields.push(format!("{name}: not a table"));
            continue;
        };
        audit.audit_bond(name, table);
    }
    audit.emit(v);
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
