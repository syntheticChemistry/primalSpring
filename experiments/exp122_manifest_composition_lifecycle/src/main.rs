// SPDX-License-Identifier: AGPL-3.0-or-later
//! exp122 — Manifest-driven composition lifecycle.
//!
//! Validates that primalSpring can:
//! 1. Parse and validate a biome.yaml manifest (toadStool v1 schema)
//! 2. Resolve composition sub-graphs with topological dependency ordering
//! 3. Reconcile the manifest against live NUCLEUS state on eastGate
//! 4. Report composition readiness per sub-graph
//!
//! This is the foundation for `nucleus.start` sub-graph execution.

#![forbid(unsafe_code)]

use primalspring::composition::manifest::{
    self, BiomeManifest, CompositionKind, ManifestReconciliation,
};
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("exp122: Manifest Composition Lifecycle");

    let manifest_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../config/biome-eastgate.yaml");

    let manifest = match manifest::load_biome_manifest(&manifest_path) {
        Ok(m) => {
            v.check_bool("manifest_parse", true, "biome-eastgate.yaml parses as v1 schema");
            m
        }
        Err(e) => {
            v.check_bool("manifest_parse", false, &format!("failed to parse: {e}"));
            v.summary();
            return;
        }
    };

    validate_structure(&manifest, &mut v);
    validate_ordering(&manifest, &mut v);
    validate_resolution(&manifest, &mut v);
    let recon = validate_reconciliation(&manifest, &mut v);

    println!();
    if let Some(r) = &recon {
        println!("  Gate:       {}", r.gate);
        println!("  Declared:   {}", r.declared);
        println!("  Alive:      {}", r.alive);
        println!("  Missing:    {:?}", r.missing);
        println!();
        for comp in &r.compositions {
            let status = if comp.ready { "READY" } else { "NOT READY" };
            println!(
                "  {} ({}) — {status} [{}/{}]",
                comp.name,
                comp.kind,
                comp.healthy_members.len(),
                comp.healthy_members.len() + comp.unhealthy_members.len()
            );
        }
    }
    println!();
    v.summary();
}

fn validate_structure(manifest: &BiomeManifest, v: &mut ValidationResult) {
    v.check_bool("api_version", manifest.api_version == "v1",
        &format!("api_version = {}", manifest.api_version));
    v.check_bool("kind", manifest.kind == "Biome",
        &format!("kind = {}", manifest.kind));
    v.check_bool("gate_name", manifest.metadata.name == "eastgate",
        &format!("name = {}", manifest.metadata.name));
    v.check_bool("primals_14", manifest.primals.len() == 14,
        &format!("{} primals declared", manifest.primals.len()));
    v.check_bool("compositions_3", manifest.compositions.len() == 3,
        &format!("{} compositions", manifest.compositions.len()));

    let has_tower = manifest.compositions.iter().any(|c| c.kind == CompositionKind::Tower);
    let has_nest = manifest.compositions.iter().any(|c| c.kind == CompositionKind::Nest);
    let has_node = manifest.compositions.iter().any(|c| c.kind == CompositionKind::Node);
    v.check_bool("has_tower_nest_node", has_tower && has_nest && has_node,
        "Tower + Nest + Node compositions present");

    let crypto_required = manifest.security.as_ref().is_some_and(|s| s.crypto_required);
    v.check_bool("security_crypto", crypto_required, "security.crypto_required = true");

    let has_federation = manifest.federation.as_ref().is_some_and(|f| f.enabled);
    v.check_bool("federation_enabled", has_federation, "federation enabled with peers");
}

fn validate_ordering(manifest: &BiomeManifest, v: &mut ValidationResult) {
    for comp in &manifest.compositions {
        let name = &comp.name;
        match manifest::topological_order(comp) {
            Ok(order) => {
                v.check_bool(&format!("{name}_topo_sort"), true,
                    &format!("{name}: {}", order.join(" -> ")));

                for (dep, requires) in &comp.dependencies {
                    let dep_pos = order.iter().position(|s| s == dep);
                    for req in requires {
                        let req_pos = order.iter().position(|s| s == req);
                        if let (Some(d), Some(r)) = (dep_pos, req_pos) {
                            v.check_bool(&format!("{name}_{req}_before_{dep}"), r < d,
                                &format!("{req} (pos {r}) starts before {dep} (pos {d})"));
                        }
                    }
                }
            }
            Err(e) => {
                v.check_bool(&format!("{name}_topo_sort"), false, &format!("sort failed: {e}"));
            }
        }

        match manifest::topological_waves(comp) {
            Ok(waves) => {
                let wave_strs: Vec<String> = waves.iter().enumerate()
                    .map(|(i, w)| format!("W{i}: [{}]", w.join(", ")))
                    .collect();
                v.check_bool(&format!("{name}_waves"), true,
                    &format!("{} waves: {}", waves.len(), wave_strs.join(" | ")));
            }
            Err(e) => {
                v.check_bool(&format!("{name}_waves"), false, &format!("{e}"));
            }
        }
    }
}

fn validate_resolution(manifest: &BiomeManifest, v: &mut ValidationResult) {
    match manifest::resolve_compositions(manifest) {
        Ok(resolved) => {
            v.check_bool("resolve_count", resolved.len() == 3,
                &format!("{} compositions resolved", resolved.len()));
            v.check_bool("priority_tower_first",
                resolved.first().is_some_and(|r| r.graph.kind == CompositionKind::Tower),
                "Tower starts first (priority 0)");
        }
        Err(e) => {
            v.check_bool("resolve_compositions", false, &format!("{e}"));
        }
    }

    match manifest::global_start_order(manifest) {
        Ok(order) => {
            v.check_bool("global_start_biomeos_first",
                order.first().is_some_and(|s| s == "biomeos"),
                &format!("global start: {} primals, biomeos first", order.len()));
            let unique = order.iter().collect::<std::collections::HashSet<_>>().len();
            v.check_bool("global_no_duplicates", unique == order.len(),
                &format!("{unique} unique out of {}", order.len()));
        }
        Err(e) => {
            v.check_bool("global_start_order", false, &format!("{e}"));
        }
    }
}

fn validate_reconciliation(
    manifest: &BiomeManifest,
    v: &mut ValidationResult,
) -> Option<ManifestReconciliation> {
    let recon = manifest::reconcile_with_live(manifest);

    v.check_bool("recon_gate", recon.gate == "eastgate",
        &format!("gate = {}", recon.gate));
    v.check_bool("recon_declared_14", recon.declared == 14,
        &format!("{} declared", recon.declared));

    let alive_ratio = if recon.declared > 0 {
        #[expect(clippy::cast_precision_loss)]
        let r = recon.alive as f64 / recon.declared as f64 * 100.0;
        r
    } else {
        0.0
    };

    v.check_bool("recon_alive_majority", recon.alive > recon.declared / 2,
        &format!("{}/{} alive ({alive_ratio:.0}%)", recon.alive, recon.declared));

    if recon.missing.is_empty() {
        v.check_bool("recon_no_missing", true, "all declared primals alive");
    } else {
        v.check_skip("recon_no_missing",
            &format!("missing: {}", recon.missing.join(", ")));
    }

    for comp in &recon.compositions {
        let status = if comp.ready { "READY" } else { "NOT READY" };
        v.check_bool(&format!("recon_{}_ready", comp.name), comp.ready,
            &format!("{} — {status} ({}/{} healthy)",
                comp.name,
                comp.healthy_members.len(),
                comp.healthy_members.len() + comp.unhealthy_members.len()));
    }

    Some(recon)
}
