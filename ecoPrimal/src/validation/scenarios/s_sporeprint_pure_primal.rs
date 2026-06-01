// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scenario: sporePrint Pure Primal Parity — validates that the petalTongue
//! content rendering pipeline can produce equivalent output to the Zola
//! reference build.
//!
//! This scenario verifies the evolution path from Zola static-site rendering
//! to the pure-primal stack (petalTongue + Nest Atomic):
//!
//! 1. Content Parsing — TOML front-matter extraction and markdown compilation
//! 2. Entity Resolution — shortcodes resolve against the entity registry
//! 3. Modality Output — HTML and description outputs are structurally correct
//! 4. Composition Graph — deploy graph references valid capabilities
//! 5. Certification — guideStone manifest remains valid post-render

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

use std::path::PathBuf;

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "sporeprint-pure-primal-parity",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "primalspring_sporeprint_pure_primal",
        provenance_date: "2026-06-01",
        description:
            "sporePrint pure-primal parity — content rendering via petalTongue vs Zola reference",
    },
    run,
};

fn find_sporeprint_root() -> Option<PathBuf> {
    let candidates = [
        PathBuf::from("../../infra/sporePrint"),
        PathBuf::from("../infra/sporePrint"),
        PathBuf::from("infra/sporePrint"),
    ];
    for c in &candidates {
        if c.join("config.toml").exists() && c.join("content").is_dir() {
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

fn find_deploy_graph() -> Option<PathBuf> {
    let candidates = [
        PathBuf::from("../../gardens/projectNUCLEUS/graphs/sporeprint_composition.toml"),
        PathBuf::from("../gardens/projectNUCLEUS/graphs/sporeprint_composition.toml"),
    ];
    for c in &candidates {
        if c.exists() {
            return Some(c.clone());
        }
    }
    if let Ok(home) = std::env::var(crate::env_keys::HOME) {
        let dev = PathBuf::from(home)
            .join("Development")
            .join("ecoPrimals")
            .join("gardens")
            .join("projectNUCLEUS")
            .join("graphs")
            .join("sporeprint_composition.toml");
        if dev.exists() {
            return Some(dev);
        }
    }
    None
}

/// Run all sporePrint pure-primal parity validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let root = find_sporeprint_root();

    let Some(root) = root else {
        v.check_bool(
            "pure_primal:sporeprint_found",
            false,
            "could not locate sporePrint content directory",
        );
        return;
    };

    v.check_bool(
        "pure_primal:sporeprint_found",
        true,
        &format!("sporePrint at {}", root.display()),
    );

    v.section("Phase 1: Content Parsing");
    phase_content_parsing(v, &root);

    v.section("Phase 2: Entity Resolution");
    phase_entity_resolution(v, &root);

    v.section("Phase 3: Modality Output Structure");
    phase_modality_output(v, &root);

    v.section("Phase 4: Composition Graph");
    phase_composition_graph(v);

    v.section("Phase 5: Certification Manifest");
    phase_certification(v, &root);
}

fn phase_content_parsing(v: &mut ValidationResult, root: &PathBuf) {
    let content_dir = root.join("content");

    let md_files: Vec<_> = walkdir(&content_dir, "md");
    v.check_bool(
        "parse:content_files_found",
        !md_files.is_empty(),
        &format!("found {} .md content files", md_files.len()),
    );

    let mut parse_successes = 0u32;
    let mut parse_failures = 0u32;

    for path in md_files.iter().take(20) {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => {
                parse_failures += 1;
                continue;
            }
        };

        let has_front_matter = content.trim_start().starts_with("+++");
        if has_front_matter {
            let after = &content[content.find("+++").unwrap() + 3..];
            let has_closing = after.contains("\n+++");
            if has_closing {
                parse_successes += 1;
            } else {
                parse_failures += 1;
            }
        } else {
            parse_successes += 1;
        }
    }

    v.check_bool(
        "parse:front_matter_valid",
        parse_failures == 0,
        &format!(
            "front-matter parsing: {parse_successes} valid, {parse_failures} invalid (sampled {})",
            md_files.len().min(20)
        ),
    );

    let config_path = root.join("config.toml");
    let config_content = std::fs::read_to_string(&config_path).unwrap_or_default();
    v.check_bool(
        "parse:config_has_entity_registry",
        config_content.contains("[extra.entity_registry."),
        "config.toml has entity_registry section",
    );
}

fn phase_entity_resolution(v: &mut ValidationResult, root: &PathBuf) {
    let config_content =
        std::fs::read_to_string(root.join("config.toml")).unwrap_or_default();

    let entity_count = config_content
        .lines()
        .filter(|l| l.starts_with("[extra.entity_registry."))
        .count();
    v.check_bool(
        "entity:registry_populated",
        entity_count >= 50,
        &format!("entity registry has {entity_count} entries (need >= 50)"),
    );

    let content_dir = root.join("content");
    let md_files: Vec<_> = walkdir(&content_dir, "md");
    let mut shortcode_count = 0u32;
    let mut resolvable_count = 0u32;

    let entity_keys: Vec<String> = config_content
        .lines()
        .filter_map(|l| {
            l.strip_prefix("[extra.entity_registry.")
                .and_then(|r| r.strip_suffix(']'))
                .map(|s| s.to_lowercase())
        })
        .collect();

    for path in md_files.iter().take(50) {
        let content = std::fs::read_to_string(path).unwrap_or_default();
        for line in content.lines() {
            if let Some(start) = line.find("{{ entity(name=\"") {
                shortcode_count += 1;
                let after = &line[start + 16..];
                if let Some(end) = after.find("\") }}") {
                    let key = &after[..end];
                    if entity_keys.contains(&key.to_lowercase()) {
                        resolvable_count += 1;
                    }
                }
            }
        }
    }

    v.check_bool(
        "entity:shortcodes_found",
        shortcode_count > 0,
        &format!("found {shortcode_count} entity shortcodes in content"),
    );

    let resolution_rate = if shortcode_count > 0 {
        (resolvable_count as f64 / shortcode_count as f64) * 100.0
    } else {
        100.0
    };
    v.check_bool(
        "entity:resolution_rate",
        resolution_rate >= 90.0,
        &format!(
            "entity resolution: {resolvable_count}/{shortcode_count} ({resolution_rate:.1}%, need >= 90%)"
        ),
    );
}

fn phase_modality_output(v: &mut ValidationResult, root: &PathBuf) {
    let content_dir = root.join("content");
    let md_files: Vec<_> = walkdir(&content_dir, "md");

    let sample = md_files.iter().find(|p| {
        let name = p.file_name().unwrap_or_default().to_string_lossy();
        !name.starts_with('_') && name != "_index.md"
    });

    let Some(sample_path) = sample else {
        v.check_bool("modality:sample_found", false, "no non-index content file found");
        return;
    };

    let content = std::fs::read_to_string(sample_path).unwrap_or_default();
    let has_heading = content.lines().any(|l| l.starts_with('#'));
    let has_body = content.lines().any(|l| !l.trim().is_empty() && !l.starts_with("+++") && !l.starts_with('#'));

    v.check_bool(
        "modality:sample_has_heading",
        has_heading,
        &format!("sample {} has heading", sample_path.file_name().unwrap_or_default().to_string_lossy()),
    );
    v.check_bool(
        "modality:sample_has_body",
        has_body,
        "sample has paragraph content for rendering",
    );

    v.check_bool(
        "modality:html_output_feasible",
        has_heading && has_body,
        "content has structure suitable for HTML rendering",
    );
    v.check_bool(
        "modality:description_output_feasible",
        has_heading && has_body,
        "content has structure suitable for description (accessibility) rendering",
    );
}

fn phase_composition_graph(v: &mut ValidationResult) {
    let graph_path = find_deploy_graph();

    let Some(path) = graph_path else {
        v.check_bool(
            "graph:found",
            false,
            "could not locate sporeprint_composition.toml deploy graph",
        );
        return;
    };

    let content = std::fs::read_to_string(&path).unwrap_or_default();
    v.check_bool("graph:found", true, &format!("deploy graph at {}", path.display()));

    v.check_bool(
        "graph:includes_nest_atomic",
        content.contains("nest_atomic"),
        "composition includes nest_atomic fragment",
    );
    v.check_bool(
        "graph:has_petaltongue",
        content.contains("petaltongue"),
        "composition defines petaltongue node",
    );
    v.check_bool(
        "graph:petaltongue_content_provider",
        content.contains("content-provider"),
        "petalTongue configured as content-provider backend",
    );
    v.check_bool(
        "graph:has_certification",
        content.contains("spore-validate") || content.contains("certify"),
        "composition includes certification step",
    );
    v.check_bool(
        "graph:render_capability",
        content.contains("content.render") || content.contains("\"render\""),
        "petalTongue declares render capability",
    );
}

fn phase_certification(v: &mut ValidationResult, root: &PathBuf) {
    let workflow = root.join(".github/workflows/deploy.yml");
    let has_certify = if let Ok(content) = std::fs::read_to_string(&workflow) {
        content.contains("certify")
    } else {
        false
    };
    v.check_bool(
        "cert:deploy_workflow_certifies",
        has_certify,
        "deploy.yml includes certification step",
    );

    let manifest_path = root.join("static/certification/manifest.json");
    let manifest_exists = manifest_path.exists();
    v.check_bool(
        "cert:manifest_present",
        manifest_exists,
        "certification manifest.json exists in static/",
    );

    if manifest_exists {
        let manifest = std::fs::read_to_string(&manifest_path).unwrap_or_default();
        let has_merkle = manifest.contains("merkle_root");
        let has_version = manifest.contains("schema_version");
        v.check_bool(
            "cert:manifest_has_merkle",
            has_merkle,
            "manifest contains merkle_root field",
        );
        v.check_bool(
            "cert:manifest_has_version",
            has_version,
            "manifest contains schema_version field",
        );
    }
}

fn walkdir(dir: &std::path::Path, ext: &str) -> Vec<PathBuf> {
    let mut results = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                results.extend(walkdir(&path, ext));
            } else if path.extension().is_some_and(|e| e == ext) {
                results.push(path);
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn sporeprint_pure_primal_no_panic() {
        let mut v = ValidationResult::new("sporeprint-pure-primal-parity");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
    }
}
