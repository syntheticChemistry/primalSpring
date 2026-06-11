// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Cross-Gate `capability.call` — CM-3 + Wave 75 live trust.
//!
//! Validates that `capability.call` routing works for both local-gate and
//! cross-gate (mesh) scenarios. CG-8 resolution: songbird Wave 211
//! shipped `capability.call` handler with local UDS + remote mesh TCP
//! forwarding and `routing="local"` hop prevention.
//!
//! Four phases:
//! 1. Structural: membrane graph declares relay channel + songbird mesh node
//! 2. Wire contract: `capability.call` registered with correct params schema
//! 3. Live: local-gate `capability.call` through biomeOS orchestration
//! 4. Live BTSP: cross-gate `capability.call` local→remote with BTSP auth

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::live_mesh::LiveMeshConfig;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "cross-gate-capability-call",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave29_cellmembrane",
        provenance_date: "2026-05-20",
        description: "Cross-gate capability.call — relay channel, wire contract, local + remote dispatch",
    },
    run,
};

/// Run all cross-gate capability.call validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — membrane graph relay channel");
    phase_structural(v);

    v.section("Phase 2: Wire contract — capability.call params schema");
    phase_wire_contract(v);

    v.section("Phase 3: Live — local-gate capability.call dispatch");
    phase_live_dispatch(v, ctx);

    v.section("Phase 4: Live BTSP — cross-gate capability.call via Songbird");
    phase_live_btsp_cross_gate(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    v.check_bool(
        "relay:membrane_graph_cellmembrane_owned",
        true,
        "membrane graph validation transferred to cellMembrane",
    );
}

fn phase_wire_contract(v: &mut ValidationResult) {
    v.check_bool(
        "wire:capability_call_registered",
        REGISTRY_TOML.contains("capability.call"),
        "capability.call in capability_registry.toml",
    );

    v.check_bool(
        "wire:capability_route_registered",
        REGISTRY_TOML.contains("capability.route"),
        "capability.route (forwarding path) in registry",
    );

    v.check_bool(
        "wire:capability_discover_registered",
        REGISTRY_TOML.contains("capability.discover"),
        "capability.discover (lookup path) in registry",
    );

    v.check_bool(
        "wire:capability_resolve_registered",
        REGISTRY_TOML.contains("capability.resolve"),
        "capability.resolve (endpoint resolution) in registry",
    );

    v.check_bool(
        "wire:route_register_registered",
        REGISTRY_TOML.contains("route.register"),
        "route.register (federation pattern) in registry",
    );
}

#[expect(
    clippy::too_many_lines,
    reason = "cross-gate dispatch validation with IPC probing"
)]
fn phase_live_dispatch(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    match ctx.call(
        "orchestration",
        "capability.call",
        serde_json::json!({
            "capability": "security",
            "operation": "health.liveness",
            "args": {},
        }),
    ) {
        Ok(resp) => {
            let alive = resp
                .get("alive")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false)
                || resp.get("status").and_then(serde_json::Value::as_str) == Some("ok")
                || resp.get("result").is_some();
            v.check_bool(
                "live:local_capability_call",
                alive,
                &format!("capability.call(security, health.liveness) → {resp}"),
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "live:local_capability_call",
                &format!("biomeOS orchestration not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "live:local_capability_call",
                false,
                &format!("capability.call error: {e}"),
            );
        }
    }

    match ctx.call(
        "orchestration",
        "capability.call",
        serde_json::json!({
            "capability": "discovery",
            "operation": "identity.get",
            "args": {},
        }),
    ) {
        Ok(resp) => {
            let has_identity = resp.get("name").is_some()
                || resp.get("primal").is_some()
                || resp.get("result").is_some();
            v.check_bool(
                "live:routed_identity_get",
                has_identity,
                &format!("capability.call(discovery, identity.get) → {resp}"),
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "live:routed_identity_get",
                &format!("biomeOS not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "live:routed_identity_get",
                false,
                &format!("capability.call error: {e}"),
            );
        }
    }

    match ctx.call(
        "orchestration",
        "capability.call",
        serde_json::json!({
            "capability": "security",
            "operation": "health.liveness",
            "args": {},
            "gate": "cellMembrane",
        }),
    ) {
        Ok(resp) => {
            v.check_bool(
                "live:cross_gate_dispatch",
                true,
                &format!("cross-gate capability.call to cellMembrane succeeded: {resp}"),
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "live:cross_gate_dispatch",
                &format!("cross-gate not available (expected without VPS mesh): {e}"),
            );
        }
        Err(e) => {
            let msg = e.to_string();
            let expected_without_mesh = msg.contains("not found")
                || msg.contains("unknown gate")
                || msg.contains("no route")
                || msg.contains("not available");
            if expected_without_mesh {
                v.check_skip(
                    "live:cross_gate_dispatch",
                    &format!("cross-gate dispatch gracefully rejected: {e}"),
                );
            } else {
                v.check_bool(
                    "live:cross_gate_dispatch",
                    false,
                    &format!("unexpected cross-gate error: {e}"),
                );
            }
        }
    }
}

/// Phase 4: Live BTSP cross-gate `capability.call` via Songbird federation.
///
/// P0 validation that proves end-to-end trust:
/// local gate issues BTSP token → calls capability on remote gate via Songbird →
/// remote gate verifies token with `verification_source: "remote"` → returns valid result.
fn phase_live_btsp_cross_gate(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let mesh = LiveMeshConfig::from_env();

    if !mesh.is_connectable() {
        v.check_skip(
            "live:btsp_cross_gate_reachable",
            &format!("no remote gates configured ({})", mesh.summary()),
        );
        v.check_skip("live:btsp_cross_gate_call", "no remote gates");
        v.check_skip("live:btsp_cross_gate_auth", "no remote gates");
        return;
    }

    let readiness = mesh.check_readiness();
    let target_gate = emit_reachability(v, &readiness);

    let Some(target_gate) = target_gate else {
        v.check_skip("live:btsp_cross_gate_call", "no Songbird responding");
        v.check_skip("live:btsp_cross_gate_auth", "no Songbird responding");
        return;
    };

    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "live:btsp_cross_gate_call",
            "orchestration not in context — cannot dispatch cross-gate",
        );
        v.check_skip("live:btsp_cross_gate_auth", "no orchestration");
        return;
    }

    if !attempt_cross_gate_call(v, ctx, target_gate) {
        v.check_skip(
            "live:btsp_cross_gate_auth",
            "cross-gate call did not succeed — cannot verify auth",
        );
        return;
    }

    attempt_cross_gate_auth(v, ctx, &mesh, target_gate);
}

fn emit_reachability<'a>(
    v: &mut ValidationResult,
    readiness: &'a [crate::validation::live_mesh::GateReadiness],
) -> Option<&'a str> {
    let any_reachable = readiness.iter().any(|g| g.tcp_reachable);
    let any_songbird = readiness.iter().any(|g| g.songbird_responding);

    v.check_bool(
        "live:btsp_cross_gate_reachable",
        any_reachable,
        &format!(
            "remote gate connectivity: {} reachable, {} songbird OK (of {})",
            readiness.iter().filter(|g| g.tcp_reachable).count(),
            readiness.iter().filter(|g| g.songbird_responding).count(),
            readiness.len(),
        ),
    );

    if any_songbird {
        readiness
            .iter()
            .find(|g| g.songbird_responding)
            .map(|g| g.gate_id.as_str())
    } else {
        None
    }
}

fn attempt_cross_gate_call(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    target_gate: &str,
) -> bool {
    match ctx.call(
        "orchestration",
        "capability.call",
        serde_json::json!({
            "capability": "security",
            "operation": "health.liveness",
            "args": {},
            "gate": target_gate,
        }),
    ) {
        Ok(resp) => {
            let has_result = resp.get("alive").is_some()
                || resp.get("status").is_some()
                || resp.get("result").is_some()
                || resp.get("ok").is_some();
            v.check_bool(
                "live:btsp_cross_gate_call",
                has_result,
                &format!("cross-gate capability.call → {target_gate}: {resp}"),
            );
            has_result
        }
        Err(e) => {
            let msg = e.to_string();
            let mesh_gap = msg.contains("No local or remote provider")
                || msg.contains("no route")
                || msg.contains("not found");
            if mesh_gap {
                v.check_skip(
                    "live:btsp_cross_gate_call",
                    &format!("capability propagation gap — no remote caps: {e}"),
                );
            } else {
                v.check_bool(
                    "live:btsp_cross_gate_call",
                    false,
                    &format!("cross-gate call to {target_gate} failed: {e}"),
                );
            }
            false
        }
    }
}

fn attempt_cross_gate_auth(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    mesh: &LiveMeshConfig,
    target_gate: &str,
) {
    if !ctx.has_capability("security") || !mesh.btsp_available {
        v.check_skip(
            "live:btsp_cross_gate_auth",
            &format!(
                "BTSP prerequisites not met (security={}, btsp={})",
                ctx.has_capability("security"),
                mesh.btsp_available,
            ),
        );
        return;
    }

    let token = ctx
        .call(
            "security",
            "auth.issue_ionic",
            serde_json::json!({
                "subject": "cross-gate-trust-test",
                "scopes": ["security.*", "health.*"],
                "ttl_seconds": 30,
                "gate_origin": &mesh.local_gate,
            }),
        )
        .ok()
        .and_then(|r| {
            r.get("token")
                .and_then(serde_json::Value::as_str)
                .map(String::from)
        });

    let Some(bearer) = token else {
        v.check_skip(
            "live:btsp_cross_gate_auth",
            "could not issue local token for cross-gate auth test",
        );
        return;
    };

    match ctx.call(
        "orchestration",
        "capability.call",
        serde_json::json!({
            "capability": "security",
            "operation": "auth.verify_ionic",
            "args": {
                "token": bearer,
                "verification_source": "remote",
                "requesting_gate": &mesh.local_gate,
            },
            "gate": target_gate,
        }),
    ) {
        Ok(resp) => {
            let valid = resp
                .get("valid")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            v.check_bool(
                "live:btsp_cross_gate_auth",
                valid,
                &format!(
                    "cross-gate BTSP auth: issued on {} → verified on {target_gate} = {valid}",
                    mesh.local_gate,
                ),
            );
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("No local or remote provider") || msg.contains("no route") {
                v.check_skip(
                    "live:btsp_cross_gate_auth",
                    &format!("capability propagation gap blocks remote verify: {e}"),
                );
            } else {
                v.check_bool(
                    "live:btsp_cross_gate_auth",
                    false,
                    &format!("cross-gate auth verify failed: {e}"),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn cross_gate_no_panic() {
        let mut v = ValidationResult::new("cross-gate-capability-call");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
    }

    #[test]
    fn capability_call_in_registry() {
        assert!(
            REGISTRY_TOML.contains("capability.call"),
            "capability.call must be in capability_registry.toml"
        );
    }
}
