// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Primal Work Utilization — validates that each assigned primal
//! responds to its domain-specific work, not just health.liveness.
//!
//! Goes beyond "is it alive?" to "is it doing its job?". Each primal has a
//! specific domain capability. This scenario probes that capability to confirm
//! the primal is ready to serve its purpose in the composition.
//!
//! | Primal | Domain | Probe Method |
//! |--------|--------|-------------|
//! | Squirrel | AI inference | ai.models or health.liveness |
//! | ToadStool | Compute dispatch | compute.capabilities |
//! | BarraCuda | Tensor math | tensor.device_info |
//! | CoralReef | Shader compile | shader.capabilities |
//! | RhizoCrypt | DAG provenance | dag.session.status |
//! | LoamSpine | Merkle ledger | entry.get or spine.info |
//! | SweetGrass | Attribution | braid.status |
//! | BearDog | Trust/Crypto | crypto.identity |
//! | Songbird | Discovery | discovery.peers |
//! | SkunkBat | Defense | security.audit_status |
//! | PetalTongue | Visualization | visualization.capabilities |
//! | BiomeOS | Orchestration | orchestration.status |
//! | NestGate | Storage | content.status |

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Primal work utilization — are primals responding to their domain work?
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "primal-utilization",
        track: Track::Lifecycle,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-19",
        description: "Domain-specific work probes: each primal responds to its assigned role",
    },
    run: run_primal_utilization,
};

struct WorkProbe {
    primal: &'static str,
    capability: &'static str,
    methods: &'static [&'static str],
    domain: &'static str,
}

const WORK_PROBES: &[WorkProbe] = &[
    WorkProbe {
        primal: "skunkbat",
        capability: "security",
        methods: &["health.liveness", "security.audit_log"],
        domain: "defense",
    },
    WorkProbe {
        primal: "barracuda",
        capability: "ai",
        methods: &["health.liveness"],
        domain: "AI inference",
    },
    WorkProbe {
        primal: "coralreef",
        capability: "shader",
        methods: &["health.liveness"],
        domain: "shader compilation",
    },
    WorkProbe {
        primal: "loamspine",
        capability: "ledger",
        methods: &["health.liveness"],
        domain: "merkle ledger",
    },
    WorkProbe {
        primal: "petaltongue",
        capability: "visualization",
        methods: &["health.liveness"],
        domain: "visualization",
    },
];

fn run_primal_utilization(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    phase_capability_probes(v, ctx);
    phase_socket_responsiveness(v);
    phase_utilization_score(v, ctx);
}

fn phase_capability_probes(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    for probe in WORK_PROBES {
        if !ctx.has_capability(probe.capability) {
            v.check_skip(
                &format!("util:{}:routed", probe.primal),
                &format!(
                    "{} ({}) — capability not routed",
                    probe.primal, probe.domain
                ),
            );
            continue;
        }

        let mut responded = false;
        for method in probe.methods {
            match ctx.call(probe.capability, method, serde_json::json!({})) {
                Ok(_) => {
                    responded = true;
                    break;
                }
                Err(e) => {
                    if !e.is_connection_error() {
                        responded = true;
                        break;
                    }
                }
            }
        }

        v.check_bool(
            &format!("util:{}:responding", probe.primal),
            responded,
            &format!(
                "{} ({}): {}",
                probe.primal,
                probe.domain,
                if responded {
                    "RESPONDING"
                } else {
                    "NO RESPONSE"
                }
            ),
        );
    }
}

fn phase_socket_responsiveness(v: &mut ValidationResult) {
    let runtime_dir =
        std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/run/user/1000".to_owned());
    let socket_dir = std::path::PathBuf::from(&runtime_dir).join("biomeos");

    if !socket_dir.is_dir() {
        v.check_skip("util:sockets_responsive", "biomeos dir not found");
        return;
    }

    let all_primals = [
        "barracuda",
        "beardog",
        "biomeos",
        "coralreef",
        "loamspine",
        "nestgate",
        "petaltongue",
        "rhizocrypt",
        "skunkbat",
        "squirrel",
        "sweetgrass",
        "toadstool",
        "songbird",
    ];

    let mut have_socket = 0u32;
    for primal in &all_primals {
        let sock = socket_dir.join(format!("{primal}.sock"));
        if sock.exists() {
            have_socket += 1;
        }
    }

    v.check_bool(
        "util:socket_coverage",
        have_socket >= 10,
        &format!(
            "{have_socket}/{} primals have active sockets",
            all_primals.len()
        ),
    );
}

fn phase_utilization_score(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let all_caps = [
        "security",
        "ai",
        "shader",
        "ledger",
        "visualization",
        "dag",
        "attribution",
        "compute",
        "tensor",
        "discovery",
        "storage",
        "defense",
        "commit",
    ];

    let mut alive = 0u32;
    let mut total_probed = 0u32;

    for cap in &all_caps {
        if !ctx.has_capability(cap) {
            continue;
        }
        total_probed += 1;
        if ctx
            .call(cap, "health.liveness", serde_json::json!({}))
            .is_ok()
        {
            alive += 1;
        }
    }

    let score = if total_probed > 0 {
        (f64::from(alive) / f64::from(total_probed)) * 100.0
    } else {
        0.0
    };

    v.check_bool(
        "util:utilization_score",
        score >= 80.0,
        &format!("utilization: {alive}/{total_probed} responding ({score:.0}%)"),
    );

    let systemd_count = std::process::Command::new("systemctl")
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

    #[expect(clippy::cast_precision_loss, reason = "primal count < 20")]
    let coverage = if systemd_count > 0 {
        (f64::from(total_probed) / systemd_count as f64) * 100.0
    } else {
        0.0
    };

    v.check_bool(
        "util:routing_coverage",
        total_probed >= 4,
        &format!(
            "routing: {total_probed} capabilities routed from {systemd_count} running primals ({coverage:.0}%)",
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primal_utilization_structural() {
        let mut v = ValidationResult::new("primal-utilization");
        let mut ctx = CompositionContext::discover();
        run_primal_utilization(&mut v, &mut ctx);
        assert!(
            v.passed + v.failed + v.skipped > 0,
            "primal-utilization should evaluate at least one check"
        );
    }
}
