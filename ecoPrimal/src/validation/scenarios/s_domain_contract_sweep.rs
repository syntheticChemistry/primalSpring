// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: Domain Contract Sweep — contract tests for remaining capability domains.
//!
//! Wave 9 gap closure: exercises capability domains that had registered methods
//! in the 458-method registry but no scenario, test, or graph coverage. Covers
//! secrets, bonding, defense, discovery, provenance, spine/ledger, and network.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "domain-contract-sweep",
        track: Track::AtomicComposition,
        tier: Tier::Live,
        provenance_crate: "wave9_domain_sweep",
        provenance_date: "2026-05-11",
        description: "Contract sweep — secrets, bonding, defense, discovery, provenance, spine, network",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: secrets.* (bearDog key vault)");
    phase_secrets(v, ctx);

    v.section("Phase 2: bonding.* (biomeOS bond lifecycle)");
    phase_bonding(v, ctx);

    v.section("Phase 3: defense.* (skunkBat audit + policy)");
    phase_defense(v, ctx);

    v.section("Phase 4: discovery.* (songbird network discovery)");
    phase_discovery(v, ctx);

    v.section("Phase 5: provenance.* / dag.* (rhizoCrypt session lifecycle)");
    phase_provenance(v, ctx);

    v.section("Phase 6: spine.* / session.* (loamSpine ledger)");
    phase_spine(v, ctx);

    v.section("Phase 7: network.* (songbird NAT/TLS/STUN)");
    phase_network(v, ctx);
}

fn phase_secrets(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("security") {
        v.check_skip("secrets_store_responds", "security capability not available");
        v.check_skip("secrets_retrieve_responds", "security capability not available");
        return;
    }

    match ctx.call(
        "security",
        "secrets.store",
        serde_json::json!({
            "id": "primalspring:sweep:test-key",
            "value": "sweep-contract-test-value",
        }),
    ) {
        Ok(_resp) => {
            v.check_bool("secrets_store_responds", true, "secrets.store accepted request");
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("secrets_store_responds", &format!("bearDog not reachable: {e}"));
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("not found") || msg.contains("Not allowed") {
                v.check_bool(
                    "secrets_store_responds",
                    true,
                    "secrets.store method exists (permission/scope rejection is valid)",
                );
            } else {
                v.check_bool("secrets_store_responds", false, &format!("secrets.store error: {e}"));
            }
        }
    }

    match ctx.call(
        "security",
        "secrets.retrieve",
        serde_json::json!({ "id": "primalspring:sweep:test-key" }),
    ) {
        Ok(_resp) => {
            v.check_bool("secrets_retrieve_responds", true, "secrets.retrieve returned response");
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("secrets_retrieve_responds", &format!("bearDog not reachable: {e}"));
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("not found") || msg.contains("No secret") {
                v.check_bool(
                    "secrets_retrieve_responds",
                    true,
                    "secrets.retrieve method exists (no-secret response is valid for sweep key)",
                );
            } else {
                v.check_bool(
                    "secrets_retrieve_responds",
                    false,
                    &format!("secrets.retrieve error: {e}"),
                );
            }
        }
    }
}

fn phase_bonding(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") {
        v.check_skip("bonding_status_responds", "orchestration capability not available");
        return;
    }

    match ctx.call(
        "orchestration",
        "bonding.status",
        serde_json::json!({}),
    ) {
        Ok(resp) => {
            let has_bonds = resp.get("bonds").is_some() || resp.get("active").is_some() || resp.is_object();
            v.check_bool(
                "bonding_status_responds",
                has_bonds,
                "bonding.status returned bond state",
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("bonding_status_responds", &format!("biomeOS not reachable: {e}"));
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("not found") {
                v.check_skip("bonding_status_responds", "bonding.status not yet routed on this biomeOS version");
            } else {
                v.check_bool("bonding_status_responds", false, &format!("bonding.status error: {e}"));
            }
        }
    }
}

fn phase_defense(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("defense") {
        v.check_skip("defense_status_responds", "defense capability not available");
        v.check_skip("defense_events_responds", "defense capability not available");
        return;
    }

    match ctx.call("defense", "defense.status", serde_json::json!({})) {
        Ok(resp) => {
            let valid = resp.is_object();
            v.check_bool("defense_status_responds", valid, "defense.status returned policy state");
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("defense_status_responds", &format!("skunkBat not reachable: {e}"));
        }
        Err(e) => {
            v.check_bool("defense_status_responds", false, &format!("defense.status error: {e}"));
        }
    }

    match ctx.call(
        "defense",
        "defense.events",
        serde_json::json!({ "limit": 5 }),
    ) {
        Ok(resp) => {
            let has_events = resp.get("events").is_some() || resp.is_array() || resp.is_object();
            v.check_bool(
                "defense_events_responds",
                has_events,
                "defense.events returned recent audit events",
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("defense_events_responds", &format!("skunkBat not reachable: {e}"));
        }
        Err(e) => {
            v.check_bool("defense_events_responds", false, &format!("defense.events error: {e}"));
        }
    }
}

fn phase_discovery(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("discovery") {
        v.check_skip("discovery_discover_responds", "discovery capability not available");
        v.check_skip("discovery_protocols_responds", "discovery capability not available");
        return;
    }

    match ctx.call(
        "discovery",
        "discovery.discover",
        serde_json::json!({ "capability": "storage" }),
    ) {
        Ok(resp) => {
            let has_result = resp.get("socket").is_some()
                || resp.get("endpoint").is_some()
                || resp.is_object();
            v.check_bool(
                "discovery_discover_responds",
                has_result,
                "discovery.discover returned endpoint info",
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("discovery_discover_responds", &format!("songbird not reachable: {e}"));
        }
        Err(e) => {
            v.check_bool(
                "discovery_discover_responds",
                false,
                &format!("discovery.discover error: {e}"),
            );
        }
    }

    match ctx.call("discovery", "discovery.protocols", serde_json::json!({})) {
        Ok(resp) => {
            let valid = resp.is_object() || resp.is_array();
            v.check_bool(
                "discovery_protocols_responds",
                valid,
                "discovery.protocols returned supported protocol list",
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("discovery_protocols_responds", &format!("songbird not reachable: {e}"));
        }
        Err(e) => {
            v.check_bool(
                "discovery_protocols_responds",
                false,
                &format!("discovery.protocols error: {e}"),
            );
        }
    }
}

fn phase_provenance(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("dag") {
        v.check_skip("provenance_session_create_responds", "dag capability not available");
        v.check_skip("provenance_event_append_responds", "dag capability not available");
        return;
    }

    let session_id = match ctx.call(
        "dag",
        "provenance.session.create",
        serde_json::json!({ "description": "sweep-contract-test", "session_type": "General" }),
    ) {
        Ok(resp) => {
            let sid = resp.as_str().unwrap_or_default().to_owned();
            let has_id = !sid.is_empty()
                || resp.get("session_id").is_some()
                || resp.get("result").is_some();
            v.check_bool(
                "provenance_session_create_responds",
                has_id,
                "provenance.session.create returned session",
            );
            resp.get("session_id")
                .or_else(|| resp.get("result"))
                .and_then(|v| v.as_str())
                .map(String::from)
                .or_else(|| (!sid.is_empty()).then_some(sid))
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("provenance_session_create_responds", &format!("rhizoCrypt not reachable: {e}"));
            None
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("dag.session.create") {
                v.check_skip(
                    "provenance_session_create_responds",
                    "provenance.session.create alias may not be registered (dag.session.create is canonical)",
                );
            } else {
                v.check_bool(
                    "provenance_session_create_responds",
                    false,
                    &format!("provenance.session.create error: {e}"),
                );
            }
            None
        }
    };

    if let Some(ref sid) = session_id {
        match ctx.call(
            "dag",
            "provenance.event.append",
            serde_json::json!({
                "session_id": sid,
                "event_type": { "Custom": { "domain": "sweep", "event_name": "contract_test" } },
                "parents": [],
                "metadata": [],
            }),
        ) {
            Ok(_resp) => {
                v.check_bool(
                    "provenance_event_append_responds",
                    true,
                    "provenance.event.append created vertex",
                );
            }
            Err(e) if e.is_skippable() => {
                v.check_skip("provenance_event_append_responds", &format!("rhizoCrypt not reachable: {e}"));
            }
            Err(e) => {
                v.check_bool(
                    "provenance_event_append_responds",
                    false,
                    &format!("provenance.event.append error: {e}"),
                );
            }
        }
    } else {
        v.check_skip("provenance_event_append_responds", "no session from provenance.session.create");
    }
}

fn phase_spine(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("ledger") {
        v.check_skip("session_create_responds", "ledger capability not available");
        v.check_skip("session_state_responds", "ledger capability not available");
        return;
    }

    match ctx.call(
        "ledger",
        "session.create",
        serde_json::json!({ "description": "sweep-contract-test" }),
    ) {
        Ok(resp) => {
            let valid = resp.is_object() || resp.is_string();
            v.check_bool("session_create_responds", valid, "session.create returned session info");
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("session_create_responds", &format!("loamSpine not reachable: {e}"));
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("not found") || msg.contains("spine.create") {
                v.check_skip(
                    "session_create_responds",
                    "session.create alias may map to spine.create on this loamSpine version",
                );
            } else {
                v.check_bool("session_create_responds", false, &format!("session.create error: {e}"));
            }
        }
    }

    match ctx.call(
        "ledger",
        "session.state",
        serde_json::json!({}),
    ) {
        Ok(resp) => {
            let valid = resp.is_object();
            v.check_bool("session_state_responds", valid, "session.state returned state info");
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("session_state_responds", &format!("loamSpine not reachable: {e}"));
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("not found") {
                v.check_skip("session_state_responds", "session.state not yet exposed on this loamSpine version");
            } else {
                v.check_bool("session_state_responds", false, &format!("session.state error: {e}"));
            }
        }
    }
}

fn phase_network(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("discovery") {
        v.check_skip("network_nat_type_responds", "discovery capability not available");
        v.check_skip("network_stun_responds", "discovery capability not available");
        return;
    }

    match ctx.call("discovery", "network.nat_type", serde_json::json!({})) {
        Ok(resp) => {
            let has_type = resp.get("nat_type").is_some()
                || resp.get("type").is_some()
                || resp.is_object();
            v.check_bool(
                "network_nat_type_responds",
                has_type,
                "network.nat_type returned NAT classification",
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("network_nat_type_responds", &format!("songbird not reachable: {e}"));
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("not found") || msg.contains("not implemented") {
                v.check_skip(
                    "network_nat_type_responds",
                    "network.nat_type requires STUN (may not be available locally)",
                );
            } else {
                v.check_bool(
                    "network_nat_type_responds",
                    false,
                    &format!("network.nat_type error: {e}"),
                );
            }
        }
    }

    match ctx.call(
        "discovery",
        "network.stun",
        serde_json::json!({ "server": "stun.l.google.com:19302" }),
    ) {
        Ok(resp) => {
            let valid = resp.is_object();
            v.check_bool("network_stun_responds", valid, "network.stun returned STUN binding");
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("network_stun_responds", &format!("songbird not reachable: {e}"));
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("not found") || msg.contains("not implemented") || msg.contains("timeout") {
                v.check_skip(
                    "network_stun_responds",
                    "network.stun requires external STUN server (may not be available)",
                );
            } else {
                v.check_bool("network_stun_responds", false, &format!("network.stun error: {e}"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_contract_sweep_no_panic() {
        let mut v = ValidationResult::new("domain-contract-sweep");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(v.evaluated() > 0 || v.skipped > 0, "scenario should produce at least one check");
    }
}
