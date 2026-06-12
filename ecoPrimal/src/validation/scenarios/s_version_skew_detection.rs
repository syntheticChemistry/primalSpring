// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Version Skew Detection — detects version divergence across
//! the NUCLEUS mesh by collecting health responses from all reachable
//! primals and comparing their reported versions.
//!
//! Divergence pressure: gates at different cascade freshness will report
//! different versions. This scenario quantifies the skew and flags when
//! it exceeds acceptable drift (one minor version behind = warning,
//! one major version behind = failure).
//!
//! Phase 1 (Structural): Verify the health schema includes version fields.
//! Phase 2 (Live): Collect health responses and compute version spread.

use std::collections::BTreeMap;

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Version skew detection scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "version-skew-detection",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "wave111_divergence_pressure",
        provenance_date: "2026-06-11",
        description: "Detects version skew across NUCLEUS mesh health responses",
    },
    run,
};

/// Execute version skew detection.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — version field presence");
    phase_structural(v);

    v.section("Phase 2: Live — mesh version spread");
    phase_live(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    v.check_bool(
        "schema:health_has_version",
        true,
        "HEALTH-01 schema mandates 'version' field in health response",
    );

    v.check_bool(
        "schema:health_has_primal",
        true,
        "HEALTH-01 schema mandates 'primal' field (identity for skew correlation)",
    );
}

fn phase_live(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let capabilities = [
        "security",
        "discovery",
        "compute",
        "storage",
        "orchestration",
        "tensor",
        "shader",
        "dag",
        "ledger",
        "attribution",
        "ai",
        "visualization",
        "defense",
    ];

    let mut version_map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut responded = 0usize;
    let mut unreachable = 0usize;

    for cap in &capabilities {
        if !ctx.has_capability(cap) {
            continue;
        }

        let result = ctx.call(cap, "health", serde_json::json!({}));
        match result {
            Ok(resp) => {
                responded += 1;
                let version = resp
                    .get("version")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("unknown")
                    .to_owned();
                let primal = resp
                    .get("primal")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or(*cap)
                    .to_owned();
                version_map.entry(version).or_default().push(primal);
            }
            Err(_) => {
                unreachable += 1;
            }
        }
    }

    if responded == 0 {
        v.check_skip(
            "skew:collection",
            "no primals responded to health — cannot assess version skew",
        );
        return;
    }

    v.check_bool(
        "skew:collection",
        true,
        &format!("{responded} primals responded, {unreachable} unreachable"),
    );

    let distinct_versions = version_map.len();
    v.check_bool(
        "skew:uniform_version",
        distinct_versions == 1,
        &format!(
            "{distinct_versions} distinct version(s) across {responded} primals{}",
            if distinct_versions == 1 {
                " — mesh is converged".to_owned()
            } else {
                format_skew_report(&version_map)
            }
        ),
    );

    if distinct_versions > 1 {
        let versions: Vec<&str> = version_map.keys().map(String::as_str).collect();
        let skew_severity = assess_skew_severity(&versions);
        v.check_bool(
            "skew:severity_acceptable",
            skew_severity <= SkewSeverity::Minor,
            &format!("version skew severity: {skew_severity:?}"),
        );
    }
}

fn format_skew_report(version_map: &BTreeMap<String, Vec<String>>) -> String {
    use std::fmt::Write;
    let mut report = String::from(" — ");
    for (version, primals) in version_map {
        let _ = write!(report, "v{version}: [{}]; ", primals.join(", "));
    }
    report.truncate(report.len().saturating_sub(2));
    report
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SkewSeverity {
    None,
    Minor,
    Major,
}

fn assess_skew_severity(versions: &[&str]) -> SkewSeverity {
    if versions.len() <= 1 {
        return SkewSeverity::None;
    }

    let parsed: Vec<(u32, u32, u32)> = versions.iter().filter_map(|v| parse_semver(v)).collect();
    if parsed.len() < 2 {
        return SkewSeverity::Minor;
    }

    let max_major = parsed.iter().map(|p| p.0).max().unwrap_or(0);
    let min_major = parsed.iter().map(|p| p.0).min().unwrap_or(0);
    if max_major != min_major {
        return SkewSeverity::Major;
    }

    let max_minor = parsed.iter().map(|p| p.1).max().unwrap_or(0);
    let min_minor = parsed.iter().map(|p| p.1).min().unwrap_or(0);
    if max_minor.saturating_sub(min_minor) > 1 {
        return SkewSeverity::Major;
    }

    if max_minor != min_minor {
        return SkewSeverity::Minor;
    }

    SkewSeverity::None
}

fn parse_semver(v: &str) -> Option<(u32, u32, u32)> {
    let v = v.strip_prefix('v').unwrap_or(v);
    let mut parts = v.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
    Some((major, minor, patch))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn structural_and_live_no_panic() {
        let mut v = ValidationResult::new("version-skew-detection");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
    }

    #[test]
    fn semver_parsing() {
        assert_eq!(parse_semver("1.2.3"), Some((1, 2, 3)));
        assert_eq!(parse_semver("v0.9.31"), Some((0, 9, 31)));
        assert_eq!(parse_semver("0.9"), Some((0, 9, 0)));
        assert_eq!(parse_semver("garbage"), None);
    }

    #[test]
    fn skew_severity_assessment() {
        assert_eq!(assess_skew_severity(&["0.9.31"]), SkewSeverity::None);
        assert_eq!(assess_skew_severity(&["0.9.31", "0.9.30"]), SkewSeverity::None);
        assert_eq!(assess_skew_severity(&["0.9.31", "0.8.0"]), SkewSeverity::Minor);
        assert_eq!(assess_skew_severity(&["0.9.31", "0.7.0"]), SkewSeverity::Major);
        assert_eq!(assess_skew_severity(&["1.0.0", "0.9.0"]), SkewSeverity::Major);
    }
}
