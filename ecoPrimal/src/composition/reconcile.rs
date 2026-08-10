// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Live reconciliation of a `BiomeManifest` against running NUCLEUS state.
//!
//! Probes socket existence (biomeOS socket directory) to determine which
//! primals declared in the manifest are alive, missing, or extra. This is
//! a fast structural check — it does not perform RPC health probes.

use std::collections::HashSet;

use super::manifest::{
    BiomeManifest, CompositionReadinessResult, ManifestReconciliation,
};

/// Reconcile a manifest against a live NUCLEUS state by checking socket
/// existence. Returns a summary of what matches and what diverges.
pub fn reconcile_with_live(manifest: &BiomeManifest) -> ManifestReconciliation {
    let socket_dir = crate::tolerances::biomeos_socket_dir();

    let declared: HashSet<String> = manifest
        .primals
        .keys()
        .filter(|name| manifest.primals[name.as_str()].enabled)
        .cloned()
        .collect();

    let mut alive_set = HashSet::new();
    let mut missing = Vec::new();

    for primal_slug in &declared {
        let sock = socket_dir.join(format!("{primal_slug}.sock"));
        let tarpc_sock = socket_dir.join(format!("{primal_slug}.tarpc.sock"));
        let neural_sock = socket_dir.join(format!("{primal_slug}-neural.sock"));
        if sock.exists() || tarpc_sock.exists() || neural_sock.exists() {
            alive_set.insert(primal_slug.clone());
        } else if primal_slug == "biomeos" {
            let bio_neural = socket_dir.join("biomeos-neural.sock");
            let bio_api = std::fs::read_dir(&socket_dir)
                .ok()
                .map(|entries| {
                    entries.filter_map(Result::ok).any(|e| {
                        e.file_name()
                            .to_string_lossy()
                            .starts_with("biomeos-api-")
                    })
                })
                .unwrap_or(false);
            if bio_neural.exists() || bio_api {
                alive_set.insert(primal_slug.clone());
            } else {
                missing.push(primal_slug.clone());
            }
        } else {
            missing.push(primal_slug.clone());
        }
    }
    missing.sort();

    let mut extra = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&socket_dir) {
        for entry in entries.filter_map(Result::ok) {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.ends_with(".sock") {
                continue;
            }
            let slug = name
                .trim_end_matches(".sock")
                .trim_end_matches(".tarpc")
                .trim_end_matches("-health")
                .trim_end_matches("-neural")
                .trim_end_matches("-default");
            if let Some(slug) = slug.split('-').next() {
                if !declared.contains(slug) && !slug.starts_with("biomeos") {
                    let s = slug.to_string();
                    if !extra.contains(&s) {
                        extra.push(s);
                    }
                }
            }
        }
    }
    extra.sort();

    let compositions = manifest
        .compositions
        .iter()
        .map(|comp| {
            let mut healthy = Vec::new();
            let mut unhealthy = Vec::new();
            for member in &comp.members {
                if alive_set.contains(member) {
                    healthy.push(member.clone());
                } else {
                    unhealthy.push(member.clone());
                }
            }
            let ready = comp
                .readiness
                .as_ref()
                .map_or(unhealthy.is_empty(), |r| {
                    r.require_healthy
                        .iter()
                        .all(|name| alive_set.contains(name))
                });
            CompositionReadinessResult {
                name: comp.name.clone(),
                kind: format!("{:?}", comp.kind),
                ready,
                healthy_members: healthy,
                unhealthy_members: unhealthy,
            }
        })
        .collect();

    ManifestReconciliation {
        gate: manifest.metadata.name.clone(),
        declared: declared.len(),
        alive: alive_set.len(),
        missing,
        extra,
        compositions,
    }
}
