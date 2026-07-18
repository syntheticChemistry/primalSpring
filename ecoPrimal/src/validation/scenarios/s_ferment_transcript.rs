// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: Ferment Transcript (NFT) — Novel Ferment Transcript lifecycle.
//!
//! Exercises the time-extended provenance pattern (playbook Artifact 2):
//! `dag.session.create` → N × `dag.event.append` → `dag.dehydration.trigger`
//! → `certificate.mint` (`type=ferment_transcript`).
//!
//! Unlike the trio pipeline (single-shot), the ferment transcript accumulates
//! events over time before dehydrating into a sealed hash and minting a
//! certificate. This validates session management, dehydration, and the
//! cert mint path.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "ferment-transcript",
        track: Track::AtomicComposition,
        tier: Tier::Live,
        provenance_crate: "nucleus_playbook_artifact2",
        provenance_date: "2026-05-16",
        description: "Ferment transcript: session → N events → dehydrate → certificate.mint",
    },
    run,
};

/// Run the ferment transcript validation.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Capability discovery — DAG + certificate");
    phase_discovery(v, ctx);

    v.section("Phase 2: dag.session.create");
    let session = phase_session_create(v, ctx);

    v.section("Phase 3: dag.event.append × 3 (accumulate events)");
    phase_append_events(v, ctx, session.as_deref());

    v.section("Phase 4: dag.dehydration.trigger");
    let dehydrated = phase_dehydrate(v, ctx, session.as_deref());

    v.section("Phase 5: certificate.mint (ferment_transcript)");
    phase_mint_cert(v, ctx, session.as_deref(), dehydrated.as_deref());
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    for (cap, label) in [
        ("dag", "rhizoCrypt DAG"),
        ("certificate", "loamSpine certificates"),
    ] {
        let found = ctx.has_capability(cap);
        v.check_bool(
            &format!("ferment:discover:{cap}"),
            found,
            &format!(
                "{label} — {}",
                if found {
                    "resolved"
                } else {
                    "not discoverable"
                }
            ),
        );
    }
}

fn phase_session_create(v: &mut ValidationResult, ctx: &mut CompositionContext) -> Option<String> {
    match ctx.call("dag", "dag.session.create", serde_json::json!({})) {
        Ok(resp) => {
            let session_id = resp
                .get("session_id")
                .and_then(|s| s.as_str())
                .unwrap_or("");
            v.check_bool(
                "ferment:session_create:id",
                !session_id.is_empty(),
                &format!("session_id: {session_id}"),
            );
            if session_id.is_empty() {
                None
            } else {
                Some(session_id.to_owned())
            }
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "ferment:session_create:id",
                &format!("rhizoCrypt not available: {e}"),
            );
            None
        }
        Err(e) => {
            v.check_bool(
                "ferment:session_create:id",
                false,
                &format!("dag.session.create error: {e}"),
            );
            None
        }
    }
}

fn phase_append_events(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    session_id: Option<&str>,
) {
    let Some(session_id) = session_id else {
        v.check_skip("ferment:append:count", "no session from dag.session.create");
        return;
    };

    let mut appended = 0;
    for i in 0..3 {
        let payload = format!(
            "ferment-event-{i}-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );
        match ctx.call(
            "dag",
            "dag.event.append",
            serde_json::json!({
                "session_id": session_id,
                "event": { "type": "data", "payload": payload },
            }),
        ) {
            Ok(_) => appended += 1,
            Err(e) if e.is_skippable() => {
                v.check_skip(
                    "ferment:append:count",
                    &format!("rhizoCrypt not available: {e}"),
                );
                return;
            }
            Err(e) => {
                v.check_bool(
                    "ferment:append:count",
                    false,
                    &format!("event {i} error: {e}"),
                );
                return;
            }
        }
    }

    v.check_bool(
        "ferment:append:count",
        appended == 3,
        &format!("{appended}/3 events appended to session {session_id}"),
    );
}

fn phase_dehydrate(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    session_id: Option<&str>,
) -> Option<String> {
    let Some(session_id) = session_id else {
        v.check_skip(
            "ferment:dehydrate:hash",
            "no session from dag.session.create",
        );
        return None;
    };

    match ctx.call(
        "dag",
        "dag.dehydration.trigger",
        serde_json::json!({ "session_id": session_id }),
    ) {
        Ok(resp) => {
            let hash = resp
                .get("dehydrated_hash")
                .or_else(|| resp.get("hash"))
                .and_then(|h| h.as_str())
                .unwrap_or("");
            v.check_bool(
                "ferment:dehydrate:hash",
                !hash.is_empty(),
                &format!("dehydrated hash: {}...", &hash[..hash.len().min(16)]),
            );
            if hash.is_empty() {
                None
            } else {
                Some(hash.to_owned())
            }
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "ferment:dehydrate:hash",
                &format!("rhizoCrypt not available: {e}"),
            );
            None
        }
        Err(e) => {
            v.check_bool(
                "ferment:dehydrate:hash",
                false,
                &format!("dehydration.trigger error: {e}"),
            );
            None
        }
    }
}

fn phase_mint_cert(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    session_id: Option<&str>,
    dehydrated_hash: Option<&str>,
) {
    let (Some(session_id), Some(hash)) = (session_id, dehydrated_hash) else {
        v.check_skip("ferment:cert_mint:id", "missing session or dehydrated hash");
        return;
    };

    match ctx.call(
        "certificate",
        "certificate.mint",
        serde_json::json!({
            "session_id": session_id,
            "spine_hash": hash,
            "cert_type": "ferment_transcript",
        }),
    ) {
        Ok(resp) => {
            let cert_id = resp.get("cert_id").and_then(|c| c.as_str()).unwrap_or("");
            v.check_bool(
                "ferment:cert_mint:id",
                !cert_id.is_empty() || resp.get("id").is_some(),
                &format!(
                    "cert response keys: {:?}",
                    resp.as_object()
                        .map(|o| o.keys().collect::<Vec<_>>())
                        .unwrap_or_default()
                ),
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "ferment:cert_mint:id",
                &format!("loamSpine not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "ferment:cert_mint:id",
                false,
                &format!("certificate.mint error: {e}"),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ferment_transcript_no_panic() {
        let mut v = ValidationResult::new("ferment-transcript");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.evaluated() > 0 || v.skipped > 0,
            "scenario should produce checks"
        );
    }
}
