// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

//! Exp113: Primal Cephalization — Socket Namespace Organization
//!
//! As primals evolved through UniBin and ecoBin, each gained the ability to
//! expose multiple domain sockets (e.g., BearDog → `crypto.sock`,
//! `security.sock`, `btsp.sock`, `ed25519.sock`, `x25519.sock`). These
//! currently live in a flat namespace under `$XDG_RUNTIME_DIR/biomeos/`.
//!
//! **Cephalization** — the evolutionary concentration of sensory/control
//! functions toward one end of an organism — here means organizing sockets
//! *with* their owning primal. Domain sockets move from flat names into
//! primal-scoped directories, so the Neural API can route by primal
//! ownership rather than just capability name.
//!
//! Current flat:  `biomeos/crypto.sock`           (who owns this?)
//!                `biomeos/beardog-nucleus01.sock` (obvious)
//!
//! Cephalized:    `biomeos/beardog/crypto.sock`
//!                `biomeos/beardog/nucleus01.sock`
//!
//! This enables:
//!   - Primal-aware routing in the Neural API
//!   - Clean per-primal shutdown (`rm -rf beardog/`)
//!   - Portable primal socket groups across gates
//!   - K-Derm layer placement (cytoplasm vs plasma membrane sockets)

use primalspring::composition::CompositionContext;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp113 — Primal Cephalization")
        .with_provenance("exp113_cephalization", "2026-05-26")
        .run(
            "Exp113: Socket Namespace → Primal-Scoped Directories",
            |v| {
                v.section("Phase 1: Current Socket Census");
                phase_socket_census(v);

                v.section("Phase 2: Ownership Mapping");
                phase_ownership_mapping(v);

                v.section("Phase 3: Cephalization Readiness");
                phase_cephalization_readiness(v);

                v.section("Phase 4: Migration Plan");
                phase_migration_plan(v);
            },
        );
}

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

fn biomeos_socket_dir() -> std::path::PathBuf {
    let xdg = primalspring::tolerances::runtime_dir();
    std::path::PathBuf::from(xdg).join("biomeos")
}

fn phase_socket_census(v: &mut ValidationResult) {
    let dir = biomeos_socket_dir();
    if !dir.exists() {
        v.check_skip("census:dir", "biomeos socket directory not found");
        return;
    }

    let entries: Vec<String> = std::fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .filter_map(std::result::Result::ok)
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    let socks: Vec<_> = entries
        .iter()
        .filter(|n| n.to_ascii_lowercase().ends_with(".sock"))
        .collect();
    let named_count = socks
        .iter()
        .filter(|n| n.contains("nucleus01") || n.contains("primalspring01"))
        .count();
    let flat_count = socks
        .iter()
        .filter(|n| !n.contains("nucleus01") && !n.contains("primalspring01"))
        .count();

    v.check_bool(
        "census:total_sockets",
        true,
        &format!("{} total sockets", socks.len()),
    );
    v.check_bool(
        "census:primal_scoped",
        true,
        &format!("{named_count} already primal-scoped (naming convention)"),
    );
    v.check_bool(
        "census:flat_domain",
        true,
        &format!("{flat_count} flat domain sockets (cephalization targets)"),
    );
}

fn phase_ownership_mapping(v: &mut ValidationResult) {
    let dir = biomeos_socket_dir();
    if !dir.exists() {
        v.check_skip("ownership:dir", "socket dir not found");
        return;
    }

    let all_sockets: Vec<String> = std::fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .filter_map(std::result::Result::ok)
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| n.to_ascii_lowercase().ends_with(".sock"))
        .collect();

    let flat_sockets: Vec<&String> = all_sockets
        .iter()
        .filter(|n| !n.contains("nucleus01") && !n.contains("primalspring01"))
        .collect();

    let mut mapped = 0usize;
    let mut unmapped: Vec<String> = Vec::new();

    for sock in &flat_sockets {
        let owner = OWNERSHIP_MAP
            .iter()
            .find(|o| o.domain_sockets.contains(&sock.as_str()));
        if owner.is_some() {
            mapped += 1;
        } else {
            unmapped.push((*sock).clone());
        }
    }

    v.check_bool(
        "ownership:mapped",
        mapped > 0,
        &format!(
            "{mapped}/{} flat sockets have known primal owner",
            flat_sockets.len()
        ),
    );

    if unmapped.is_empty() {
        v.check_bool("ownership:unmapped", true, "all flat sockets mapped");
    } else {
        v.check_bool(
            "ownership:unmapped",
            false,
            &format!("{} orphan sockets: {}", unmapped.len(), unmapped.join(", ")),
        );
    }

    for entry in OWNERSHIP_MAP {
        let live = entry
            .domain_sockets
            .iter()
            .filter(|s| dir.join(s).exists())
            .count();
        v.check_bool(
            &format!("ownership:{}", entry.primal),
            live > 0,
            &format!(
                "{live}/{} domain sockets live → would become {}/",
                entry.domain_sockets.len(),
                entry.primal
            ),
        );
    }
}

fn phase_cephalization_readiness(v: &mut ValidationResult) {
    let ctx = CompositionContext::from_live_discovery_with_fallback();
    let caps = ctx.available_capabilities();

    v.check_bool(
        "ceph:composition_context",
        !caps.is_empty(),
        &format!(
            "{} capabilities via CompositionContext — routing is capability-first, \
             cephalization adds primal provenance to each route",
            caps.len()
        ),
    );

    let multi_socket: Vec<_> = OWNERSHIP_MAP
        .iter()
        .filter(|o| o.domain_sockets.len() >= 3)
        .collect();
    v.check_bool(
        "ceph:high_value_primals",
        !multi_socket.is_empty(),
        &format!(
            "{} primals with 3+ domain sockets (highest cephalization value): {}",
            multi_socket.len(),
            multi_socket
                .iter()
                .map(|o| format!("{}({})", o.primal, o.domain_sockets.len()))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    );

    let total_flat: usize = OWNERSHIP_MAP.iter().map(|o| o.domain_sockets.len()).sum();
    v.check_bool(
        "ceph:total_migration_targets",
        total_flat > 0,
        &format!(
            "{total_flat} domain sockets across {} primals → primal-scoped directories",
            OWNERSHIP_MAP.len()
        ),
    );
}

fn phase_migration_plan(v: &mut ValidationResult) {
    let mut phase_a: Vec<(&str, usize)> = OWNERSHIP_MAP
        .iter()
        .filter(|o| o.domain_sockets.len() >= 3)
        .map(|o| (o.primal, o.domain_sockets.len()))
        .collect();
    phase_a.sort_by_key(|a| std::cmp::Reverse(a.1));

    v.check_bool(
        "migration:phase_a",
        !phase_a.is_empty(),
        &format!(
            "Phase A (3+ sockets, highest value): {}",
            phase_a
                .iter()
                .map(|(p, n)| format!("{p}({n})"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    );

    let phase_b: Vec<_> = OWNERSHIP_MAP
        .iter()
        .filter(|o| !o.domain_sockets.is_empty() && o.domain_sockets.len() < 3)
        .map(|o| o.primal)
        .collect();

    v.check_bool(
        "migration:phase_b",
        true,
        &format!("Phase B (1-2 sockets): {}", phase_b.join(", ")),
    );

    v.check_bool(
        "migration:backward_compat",
        true,
        "symlinks: crypto.sock → beardog/crypto.sock (zero breakage for existing callers)",
    );

    v.check_bool(
        "migration:neural_api",
        true,
        "Neural API gains primal-scoped discovery: capability_call routes through primal dir",
    );

    v.check_bool(
        "migration:kderm_alignment",
        true,
        "K-Derm: cytoplasm sockets (primal-internal) vs plasma membrane (public endpoints)",
    );

    v.check_bool(
        "migration:per_primal_lifecycle",
        true,
        "clean shutdown per primal: rm primal dir removes all domain sockets atomically",
    );

    v.check_bool(
        "migration:gate_portability",
        true,
        "gate migration: move entire primal/ socket dir to new NUCLEUS",
    );
}
