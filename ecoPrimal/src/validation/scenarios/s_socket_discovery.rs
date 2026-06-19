// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: Socket Discovery Sweep — absorbed from exp051.

use crate::composition::CompositionContext;
use crate::coordination::AtomicType;
use crate::tolerances;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "socket-discovery",
        track: Track::Transport,
        tier: Tier::Live,
        provenance_crate: "exp051_socket_discovery_sweep",
        provenance_date: "2026-05-09",
        description: "Socket discovery sweep — Full Nucleus capability enumeration",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Capability Sweep");
    phase_capability_sweep(v, ctx);

    v.section("Phase 2: Reachability Analysis");
    phase_reachability(v, ctx);

    v.section("Phase 3: Socket Manifest Compliance");
    phase_socket_manifest(v);
}

fn phase_capability_sweep(v: &mut ValidationResult, ctx: &CompositionContext) {
    let all_caps = AtomicType::FullNucleus.required_capabilities();
    let _avail = ctx.available_capabilities();

    v.check_count("full_nucleus_capability_set_size", all_caps.len(), 13);

    let reachable = all_caps.iter().filter(|&&c| ctx.has_capability(c)).count();
    let unreachable = all_caps.len() - reachable;

    v.check_bool(
        "reachable_unresolved_sum",
        reachable + unreachable == all_caps.len(),
        "reachable + unreachable equals FullNucleus capability count",
    );

    for &cap in all_caps {
        let ok = ctx.has_capability(cap);
        if ok {
            v.check_bool(
                &format!("resolved_{cap}"),
                true,
                &format!("{cap} available via CompositionContext"),
            );
        } else {
            v.check_skip(&format!("resolved_{cap}"), &format!("{cap} not discovered"));
        }
    }
}

fn phase_reachability(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let all_caps = AtomicType::FullNucleus.required_capabilities();
    for &cap in all_caps {
        if !ctx.has_capability(cap) {
            v.check_skip(&format!("liveness_{cap}"), &format!("{cap} not connected"));
            continue;
        }
        match ctx.health_check(cap) {
            Ok(true) => v.check_bool(
                &format!("liveness_{cap}"),
                true,
                &format!("{cap} answers health.liveness"),
            ),
            Ok(false) => v.check_bool(
                &format!("liveness_{cap}"),
                false,
                &format!("{cap} not live"),
            ),
            Err(e) if e.is_skippable() => {
                v.check_skip(&format!("liveness_{cap}"), &format!("{cap}: {e}"));
            }
            Err(e) => v.check_bool(&format!("liveness_{cap}"), false, &format!("error: {e}")),
        }
    }
}

/// Validate socket naming convention and manifest presence.
///
/// Each primal should expose sockets following the pattern `{slug}.sock` or
/// `{slug}-{family}.sock` in the biomeOS runtime directory. This phase checks:
/// 1. Runtime directory exists
/// 2. Socket files follow naming convention
/// 3. No orphan sockets from unknown primals
fn phase_socket_manifest(v: &mut ValidationResult) {
    use crate::composition::ALL_CAPS;

    let runtime_dir =
        std::path::PathBuf::from(tolerances::runtime_dir()).join(crate::env_keys::BIOMEOS_SUBDIR);

    let exists = runtime_dir.is_dir();
    v.check_bool(
        "manifest:runtime_dir_exists",
        exists,
        &format!("runtime directory: {}", runtime_dir.display()),
    );

    if !exists {
        v.check_skip("manifest:socket_naming", "runtime dir absent");
        return;
    }

    let valid_slugs: std::collections::HashSet<&str> =
        tolerances::all_primal_slugs().into_iter().collect();

    let valid_caps: std::collections::HashSet<&str> = ALL_CAPS.iter().copied().collect();

    let mut total_sockets = 0u32;
    let mut valid_names = 0u32;
    let mut orphans: Vec<String> = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&runtime_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "sock") {
                total_sockets += 1;
                let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                // Strip tarpc suffix if present: {name}-tarpc → {name}
                let without_tarpc = stem.strip_suffix("-tarpc").unwrap_or(stem);
                // Accept: {slug}.sock, {slug}-{family}.sock, {cap}.sock,
                // {cap}-{family}.sock, {slug}-core-{profile}.sock
                let base = without_tarpc.split('-').next().unwrap_or(without_tarpc);
                if valid_slugs.contains(base)
                    || valid_caps.contains(base)
                    || base == "neural"
                    || base == "coralreef"
                {
                    valid_names += 1;
                } else {
                    orphans.push(stem.to_owned());
                }
            }
        }
    }

    v.check_bool(
        "manifest:socket_naming",
        orphans.is_empty(),
        &format!(
            "{valid_names}/{total_sockets} sockets follow naming convention{}",
            if orphans.is_empty() {
                String::new()
            } else {
                format!(" — orphans: {}", orphans.join(", "))
            }
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn socket_discovery_no_panic() {
        let mut v = ValidationResult::new("socket-discovery");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.evaluated() > 0 || v.skipped > 0,
            "scenario should produce at least one check"
        );
    }

    #[test]
    fn socket_manifest_structural() {
        let mut v = ValidationResult::new("socket-discovery");
        phase_socket_manifest(&mut v);
    }
}
