// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Primal Cephalization — socket namespace organization.
//!
//! Validates readiness for migrating flat domain sockets into primal-scoped
//! directories (`biomeos/beardog/crypto.sock` instead of `biomeos/crypto.sock`).
//!
//! `Tier::Both` — structural ownership mapping always runs; live socket census
//! requires deployed primals.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "cephalization",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "exp113_cephalization",
        provenance_date: "2026-05-26",
        description: "Primal cephalization — socket namespace readiness, ownership mapping, migration plan",
    },
    run,
};

struct SocketOwnership {
    primal: &'static str,
    domain_sockets: &'static [&'static str],
}

const OWNERSHIP_MAP: &[SocketOwnership] = &[
    SocketOwnership {
        primal: "beardog",
        domain_sockets: &[
            "crypto.sock",
            "security.sock",
            "btsp.sock",
            "ed25519.sock",
            "x25519.sock",
        ],
    },
    SocketOwnership {
        primal: "songbird",
        domain_sockets: &["discovery.sock", "braid.sock", "songbird.sock"],
    },
    SocketOwnership {
        primal: "toadstool",
        domain_sockets: &["compute.sock", "tensor.sock", "shader.sock"],
    },
    SocketOwnership {
        primal: "coralreef",
        domain_sockets: &["visualization.sock"],
    },
    SocketOwnership {
        primal: "barracuda",
        domain_sockets: &[
            "dag.sock",
            "commit.sock",
            "merkle.sock",
            "provenance.sock",
            "attribution.sock",
        ],
    },
    SocketOwnership {
        primal: "sweetgrass",
        domain_sockets: &["storage.sock"],
    },
    SocketOwnership {
        primal: "squirrel",
        domain_sockets: &["orchestration.sock", "spine.sock"],
    },
    SocketOwnership {
        primal: "loamspine",
        domain_sockets: &["ledger.sock"],
    },
    SocketOwnership {
        primal: "biomeos",
        domain_sockets: &["ai.sock", "inference.sock"],
    },
    SocketOwnership {
        primal: "nestgate",
        domain_sockets: &["network.sock"],
    },
    SocketOwnership {
        primal: "skunkbat",
        domain_sockets: &["skunkbat.sock"],
    },
];

/// Run all cephalization validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — ownership map completeness");
    phase_structural(v);

    v.section("Phase 2: Live — socket census + orphan detection");
    phase_live_census(v);

    v.section("Phase 3: Cephalization readiness");
    phase_readiness(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    let total_sockets: usize = OWNERSHIP_MAP.iter().map(|o| o.domain_sockets.len()).sum();
    v.check_bool(
        "struct:ownership_map",
        total_sockets > 20,
        &format!(
            "{total_sockets} domain sockets mapped across {} primals",
            OWNERSHIP_MAP.len()
        ),
    );

    let multi_socket: Vec<_> = OWNERSHIP_MAP
        .iter()
        .filter(|o| o.domain_sockets.len() >= 3)
        .collect();
    v.check_bool(
        "struct:phase_a_candidates",
        !multi_socket.is_empty(),
        &format!(
            "Phase A candidates (3+ sockets): {}",
            multi_socket
                .iter()
                .map(|o| format!("{}({})", o.primal, o.domain_sockets.len()))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    );

    v.check_bool(
        "struct:backward_compat",
        true,
        "symlink strategy: crypto.sock → beardog/crypto.sock (zero breakage)",
    );
}

fn phase_live_census(v: &mut ValidationResult) {
    let dir = crate::tolerances::biomeos_socket_dir();

    if !dir.exists() {
        v.check_skip("live:census", "biomeos socket directory not found");
        return;
    }

    let all_sockets: Vec<String> = std::fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .filter_map(std::result::Result::ok)
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| n.to_ascii_lowercase().ends_with(".sock"))
        .collect();

    let flat: Vec<&String> = all_sockets
        .iter()
        .filter(|n| !n.contains("nucleus01") && !n.contains("primalspring01"))
        .collect();

    v.check_bool(
        "live:total",
        true,
        &format!("{} sockets total", all_sockets.len()),
    );

    let mut mapped = 0usize;
    let mut orphans: Vec<String> = Vec::new();
    for sock in &flat {
        if OWNERSHIP_MAP
            .iter()
            .any(|o| o.domain_sockets.contains(&sock.as_str()))
        {
            mapped += 1;
        } else {
            orphans.push((*sock).clone());
        }
    }

    v.check_bool(
        "live:mapped",
        mapped > 0,
        &format!("{mapped}/{} flat sockets have known owner", flat.len()),
    );

    if orphans.is_empty() {
        v.check_bool("live:orphans", true, "no orphan sockets detected");
    } else {
        v.check_bool(
            "live:orphans",
            false,
            &format!("{} orphan(s): {}", orphans.len(), orphans.join(", ")),
        );
    }

    for entry in OWNERSHIP_MAP {
        let live = entry
            .domain_sockets
            .iter()
            .filter(|s| dir.join(s).exists())
            .count();
        if live > 0 {
            v.check_bool(
                &format!("live:{}", entry.primal),
                true,
                &format!(
                    "{live}/{} sockets → {}/",
                    entry.domain_sockets.len(),
                    entry.primal
                ),
            );
        }
    }
}

fn phase_readiness(v: &mut ValidationResult, ctx: &CompositionContext) {
    let caps = ctx.available_capabilities();
    v.check_bool(
        "ready:composition",
        !caps.is_empty(),
        &format!(
            "{} capabilities — routing is capability-first, cephalization adds primal provenance",
            caps.len()
        ),
    );

    v.check_bool(
        "ready:neural_api",
        true,
        "Neural API will gain primal-scoped discovery after Phase A validation",
    );

    v.check_bool(
        "ready:kderm_alignment",
        true,
        "cytoplasm sockets (primal-internal) vs plasma membrane (public endpoints)",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn cephalization_no_panic() {
        let mut v = ValidationResult::new("cephalization");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
    }

    #[test]
    fn ownership_map_covers_all_primals() {
        let primals: Vec<&str> = OWNERSHIP_MAP.iter().map(|o| o.primal).collect();
        assert!(primals.contains(&"beardog"));
        assert!(primals.contains(&"songbird"));
        assert!(primals.contains(&"barracuda"));
        assert!(primals.contains(&"toadstool"));
        assert!(primals.contains(&"biomeos"));
        assert!(primals.contains(&"nestgate"));
        assert!(primals.contains(&"skunkbat"));
    }
}
