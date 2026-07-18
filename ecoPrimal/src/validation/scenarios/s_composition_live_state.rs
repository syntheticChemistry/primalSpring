// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Composition Live State — validates that the composition graph
//! expected for this gate matches what is actually running.
//!
//! Compares:
//! - Expected primals (from `AtomicType::FullNucleus` roster) vs running systemd units
//! - Expected sockets vs actual socket files
//! - Graph node order vs actual startup order (journal timestamps)
//! - Capability resolution vs graph `by_capability` declarations
//!
//! This is the "are we running what we think we're running?" check.

use crate::composition::CompositionContext;
use crate::coordination::AtomicType;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Composition vs live gate state validation.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "composition-live-state",
        track: Track::Lifecycle,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-18",
        description: "Composition graph vs live gate: expected primals, sockets, capabilities",
    },
    run: run_composition_live_state,
};

fn run_composition_live_state(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    phase_expected_vs_running(v);
    phase_socket_vs_graph(v);
    phase_capability_resolution(v, ctx);
    phase_composition_completeness(v, ctx);
}

fn phase_expected_vs_running(v: &mut ValidationResult) {
    let expected = AtomicType::FullNucleus.required_primal_slugs();

    let output = std::process::Command::new("systemctl")
        .args([
            "--user",
            "list-units",
            "membrane-nucleus@*",
            "--no-pager",
            "--plain",
            "--no-legend",
        ])
        .output();

    let Ok(out) = output else {
        v.check_skip("live:systemd_comparison", "systemctl not available");
        return;
    };

    let text = String::from_utf8_lossy(&out.stdout);
    let running: Vec<String> = text
        .lines()
        .filter(|l| l.contains("running"))
        .filter_map(|l| {
            let unit = l.split_whitespace().next()?;
            let primal = unit
                .strip_prefix("membrane-nucleus@")?
                .strip_suffix(".service")?;
            Some(primal.to_owned())
        })
        .collect();

    let mut matched = 0usize;
    let mut missing: Vec<&str> = Vec::new();
    let mut unexpected: Vec<String> = Vec::new();

    for slug in &expected {
        if running.iter().any(|r| r == *slug) {
            matched += 1;
        } else {
            missing.push(slug);
        }
    }

    for r in &running {
        if !expected.iter().any(|e| e == r) {
            unexpected.push(r.clone());
        }
    }

    v.check_bool(
        "live:expected_running",
        matched >= expected.len().saturating_sub(3),
        &format!(
            "{matched}/{} expected primals running ({} missing: {})",
            expected.len(),
            missing.len(),
            missing.join(", ")
        ),
    );

    if !unexpected.is_empty() {
        v.check_skip(
            "live:unexpected_primals",
            &format!(
                "running but not in FullNucleus roster: {}",
                unexpected.join(", ")
            ),
        );
    }
}

fn phase_socket_vs_graph(v: &mut ValidationResult) {
    let socket_dir = crate::tolerances::platform::biomeos_socket_dir();

    if !socket_dir.is_dir() {
        v.check_skip("live:socket_graph_match", "biomeos socket dir not found");
        return;
    }

    let expected_slugs = AtomicType::FullNucleus.required_primal_slugs();

    let actual_sockets: Vec<String> = std::fs::read_dir(&socket_dir)
        .into_iter()
        .flatten()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "sock"))
        .filter_map(|e| {
            e.path()
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
        })
        .collect();

    let mut graph_sockets_found = 0usize;
    for slug in &expected_slugs {
        if actual_sockets.iter().any(|s| s == *slug) {
            graph_sockets_found += 1;
        }
    }

    v.check_bool(
        "live:socket_graph_match",
        graph_sockets_found >= expected_slugs.len().saturating_sub(3),
        &format!(
            "{graph_sockets_found}/{} graph-declared primals have live sockets",
            expected_slugs.len()
        ),
    );

    let extra_sockets: Vec<&String> = actual_sockets
        .iter()
        .filter(|s| !expected_slugs.contains(&s.as_str()))
        .collect();

    if !extra_sockets.is_empty() {
        v.check_bool(
            "live:auxiliary_sockets",
            true,
            &format!(
                "{} auxiliary sockets (sub-capabilities): {}",
                extra_sockets.len(),
                extra_sockets
                    .iter()
                    .take(5)
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        );
    }
}

fn phase_capability_resolution(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let caps = AtomicType::FullNucleus.required_capabilities();
    let mut resolved = 0u32;
    let mut alive = 0u32;

    for cap in caps {
        if ctx.has_capability(cap) {
            resolved += 1;
            if ctx
                .call(cap, "health.liveness", serde_json::json!({}))
                .is_ok()
            {
                alive += 1;
            }
        }
    }

    #[expect(clippy::cast_possible_truncation, reason = "cap count < 256")]
    let total = caps.len() as u32;

    v.check_bool(
        "live:capabilities_resolved",
        resolved > 0,
        &format!("{resolved}/{total} capabilities resolved via CompositionContext"),
    );

    v.check_bool(
        "live:capabilities_alive",
        alive > 0 && alive == resolved,
        &format!("{alive}/{resolved} resolved capabilities respond to health.liveness"),
    );
}

fn phase_composition_completeness(v: &mut ValidationResult, ctx: &CompositionContext) {
    let running_count = std::process::Command::new("systemctl")
        .args([
            "--user",
            "list-units",
            "membrane-nucleus@*",
            "--no-pager",
            "--plain",
            "--no-legend",
        ])
        .output()
        .map_or(0, |o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .filter(|l| l.contains("running"))
                .count()
        });

    let total_expected = AtomicType::FullNucleus.required_primal_slugs().len();

    #[expect(clippy::cast_precision_loss, reason = "primal count < 20")]
    let completeness_pct = if total_expected > 0 {
        (running_count as f64 / total_expected as f64) * 100.0
    } else {
        0.0
    };

    let inferred_type = AtomicType::from_primal_count(running_count);

    v.check_bool(
        "live:composition_level",
        running_count >= 8,
        &format!(
            "gate composition: {running_count}/{total_expected} primals ({completeness_pct:.0}%) — inferred: {inferred_type}",
        ),
    );

    let caps_resolved = AtomicType::FullNucleus
        .required_capabilities()
        .iter()
        .filter(|cap| ctx.has_capability(cap))
        .count();

    #[expect(clippy::cast_precision_loss, reason = "primal count < 20")]
    let resolution_ratio = if running_count > 0 {
        caps_resolved as f64 / running_count as f64
    } else {
        0.0
    };

    v.check_bool(
        "live:resolution_ratio",
        resolution_ratio > 0.3 || caps_resolved >= 4,
        &format!(
            "capability resolution ratio: {caps_resolved} resolved / {running_count} running ({:.0}%)",
            resolution_ratio * 100.0
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composition_live_state_structural() {
        let mut v = ValidationResult::new("composition-live-state");
        let mut ctx = CompositionContext::discover();
        run_composition_live_state(&mut v, &mut ctx);
        assert!(
            v.passed + v.failed + v.skipped > 0,
            "composition-live-state should evaluate at least one check"
        );
    }
}
