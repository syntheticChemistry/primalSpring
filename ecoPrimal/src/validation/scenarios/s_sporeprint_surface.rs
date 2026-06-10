// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: sporePrint External Surface — structural validation of the
//! ecosystem's public-facing composition at primals.eco.
//!
//! Validates that sporePrint's entity registry (`config.toml`) and source
//! registry (`sources.toml`) are structurally consistent with the ecosystem.
//! The external surface is a composition like any other — it should reflect
//! every primal and spring tracked by primalSpring.
//!
//! Three phases:
//!   1. Entity Coverage — config.toml entity_registry covers all primals + springs
//!   2. Source Consistency — sources.toml entries match entity_registry
//!   3. Content Contribution — springs with content="true" contribute to the surface

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "sporeprint-surface",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "primalspring_sporeprint_surface",
        provenance_date: "2026-05-20",
        description: "sporePrint external surface — entity coverage, source consistency, content contribution",
    },
    run,
};

const EXPECTED_PRIMALS: &[&str] = &[
    "beardog",
    "songbird",
    "skunkbat",
    "toadstool",
    "barracuda",
    "coralreef",
    "nestgate",
    "rhizocrypt",
    "loamspine",
    "sweetgrass",
    "biomeos",
    "squirrel",
    "petaltongue",
    "sourdough",
    "bingocube",
];

const EXPECTED_SPRINGS: &[&str] = &[
    "hotspring",
    "airspring",
    "wetspring",
    "groundspring",
    "neuralspring",
    "healthspring",
    "ludospring",
    "primalspring",
];

fn find_sporeprint_root() -> Option<PathBuf> {
    let candidates = [
        PathBuf::from("../../infra/sporePrint"),
        PathBuf::from("../infra/sporePrint"),
        PathBuf::from("infra/sporePrint"),
    ];
    for c in &candidates {
        if c.join("config.toml").exists() && c.join("sources.toml").exists() {
            return Some(c.clone());
        }
    }
    if let Ok(home) = std::env::var(crate::env_keys::HOME) {
        let dev = PathBuf::from(home)
            .join("Development")
            .join("ecoPrimals")
            .join("infra")
            .join("sporePrint");
        if dev.join("config.toml").exists() {
            return Some(dev);
        }
    }
    None
}

fn parse_entity_ids(content: &str) -> HashSet<String> {
    let mut ids = HashSet::new();
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("[extra.entity_registry.") {
            if let Some(id) = rest.strip_suffix(']') {
                ids.insert(id.to_lowercase());
            }
        }
    }
    ids
}

fn parse_source_ids(content: &str) -> HashSet<String> {
    let mut ids = HashSet::new();
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("[sources.") {
            if let Some(id) = rest.strip_suffix(']') {
                ids.insert(id.to_lowercase());
            }
        }
    }
    ids
}

fn phase_entity_coverage(v: &mut ValidationResult, config: &str) {
    let entities = parse_entity_ids(config);

    for primal in EXPECTED_PRIMALS {
        let present = entities.contains(*primal);
        v.check_bool(
            &format!("entity:primal:{primal}"),
            present,
            &format!("config.toml entity_registry contains primal '{primal}'"),
        );
    }

    for spring in EXPECTED_SPRINGS {
        let present = entities.contains(*spring);
        v.check_bool(
            &format!("entity:spring:{spring}"),
            present,
            &format!("config.toml entity_registry contains spring '{spring}'"),
        );
    }

    let total = EXPECTED_PRIMALS.len() + EXPECTED_SPRINGS.len();
    let found = EXPECTED_PRIMALS
        .iter()
        .chain(EXPECTED_SPRINGS.iter())
        .filter(|id| entities.contains(**id))
        .count();
    v.check_bool(
        "entity:coverage",
        found == total,
        &format!("entity coverage: {found}/{total} expected entities present"),
    );
}

fn phase_source_consistency(v: &mut ValidationResult, config: &str, sources: &str) {
    let entities = parse_entity_ids(config);
    let sources_set = parse_source_ids(sources);

    let entities_without_source: Vec<_> = entities
        .iter()
        .filter(|e| {
            EXPECTED_PRIMALS.contains(&e.as_str()) || EXPECTED_SPRINGS.contains(&e.as_str())
        })
        .filter(|e| !sources_set.contains(e.as_str()))
        .collect();

    v.check_bool(
        "source:all_entities_have_source",
        entities_without_source.is_empty(),
        &format!(
            "entities without source entry: {}",
            if entities_without_source.is_empty() {
                "none".to_string()
            } else {
                entities_without_source
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        ),
    );

    let sources_without_entity: Vec<_> = sources_set
        .iter()
        .filter(|s| !entities.contains(s.as_str()))
        .collect();

    v.check_bool(
        "source:all_sources_have_entity",
        sources_without_entity.is_empty(),
        &format!(
            "sources without entity entry: {}",
            if sources_without_entity.is_empty() {
                "none".to_string()
            } else {
                sources_without_entity
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        ),
    );

    for primal in EXPECTED_PRIMALS {
        let in_both = entities.contains(*primal) && sources_set.contains(*primal);
        v.check_bool(
            &format!("source:primal:{primal}"),
            in_both,
            &format!("primal '{primal}' in both entity_registry and sources"),
        );
    }
}

fn parse_source_types(content: &str) -> (usize, usize, usize) {
    let mut primals = 0usize;
    let mut springs = 0usize;
    let mut products = 0usize;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("type") && trimmed.contains('=') {
            if trimmed.contains("\"primal\"") {
                primals += 1;
            } else if trimmed.contains("\"spring\"") {
                springs += 1;
            } else if trimmed.contains("\"product\"") {
                products += 1;
            }
        }
    }
    (primals, springs, products)
}

fn phase_content_contribution(v: &mut ValidationResult, base: &Path) {
    let sources_content = std::fs::read_to_string(base.join("sources.toml")).unwrap_or_default();

    let (primals, springs, products) = parse_source_types(&sources_content);
    let total = primals + springs + products;

    v.check_bool(
        "content:source_primals",
        primals >= EXPECTED_PRIMALS.len(),
        &format!(
            "{primals} primal sources (expect >= {})",
            EXPECTED_PRIMALS.len()
        ),
    );
    v.check_bool(
        "content:source_springs",
        springs >= EXPECTED_SPRINGS.len(),
        &format!(
            "{springs} spring sources (expect >= {})",
            EXPECTED_SPRINGS.len()
        ),
    );
    let expected_min = EXPECTED_PRIMALS.len() + EXPECTED_SPRINGS.len();
    v.check_bool(
        "content:source_total",
        total >= expected_min,
        &format!(
            "{total} total sources (primals: {primals}, springs: {springs}, products: {products})"
        ),
    );

    let config_content = std::fs::read_to_string(base.join("config.toml")).unwrap_or_default();
    let has_totals = config_content.contains("[extra.totals]");
    v.check_bool(
        "content:totals_section",
        has_totals,
        "config.toml has [extra.totals] aggregate metrics",
    );

    let entity_count = parse_entity_ids(&config_content).len();
    v.check_bool(
        "content:entity_count",
        entity_count >= (EXPECTED_PRIMALS.len() + EXPECTED_SPRINGS.len()),
        &format!(
            "entity_registry has {entity_count} entities (expect >= {})",
            EXPECTED_PRIMALS.len() + EXPECTED_SPRINGS.len()
        ),
    );
}

/// Run all sporePrint surface validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let root = find_sporeprint_root();

    let Some(root) = root else {
        v.check_bool(
            "surface:sporeprint_found",
            false,
            "could not locate sporePrint (tried infra/sporePrint and $HOME paths)",
        );
        return;
    };

    v.check_bool(
        "surface:sporeprint_found",
        true,
        &format!("sporePrint at {}", root.display()),
    );

    let config = std::fs::read_to_string(root.join("config.toml")).unwrap_or_default();
    let sources = std::fs::read_to_string(root.join("sources.toml")).unwrap_or_default();

    v.check_bool(
        "surface:config_readable",
        !config.is_empty(),
        "config.toml is readable and non-empty",
    );
    v.check_bool(
        "surface:sources_readable",
        !sources.is_empty(),
        "sources.toml is readable and non-empty",
    );

    if config.is_empty() || sources.is_empty() {
        return;
    }

    v.section("Phase 1: Entity Coverage");
    phase_entity_coverage(v, &config);

    v.section("Phase 2: Source Consistency");
    phase_source_consistency(v, &config, &sources);

    v.section("Phase 3: Content Contribution");
    phase_content_contribution(v, &root);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn sporeprint_surface_no_panic() {
        let mut v = ValidationResult::new("sporeprint-surface");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
    }

    #[test]
    fn parse_entity_ids_extracts_correctly() {
        let sample = r#"
[extra.entity_registry.beardog]
display = "BearDog"
[extra.entity_registry.songbird]
display = "Songbird"
"#;
        let ids = parse_entity_ids(sample);
        assert!(ids.contains("beardog"));
        assert!(ids.contains("songbird"));
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn parse_source_ids_extracts_correctly() {
        let sample = r#"
[sources.beardog]
repo = "ecoPrimals/bearDog"
[sources.hotspring]
repo = "syntheticChemistry/hotSpring"
"#;
        let ids = parse_source_ids(sample);
        assert!(ids.contains("beardog"));
        assert!(ids.contains("hotspring"));
        assert_eq!(ids.len(), 2);
    }
}
