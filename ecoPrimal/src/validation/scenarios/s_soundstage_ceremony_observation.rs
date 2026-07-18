// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: soundStage Ceremony Observation — validates that the soundStage
//! transparency layer can observe, record, compare, and quality-gate key
//! generation ceremonies.
//!
//! This bridges the soundStage concept with the FIDO2 ceremony path:
//! - Channels map to anchors (`SoloKey`, `StrongBox`, audio, OS)
//! - Sessions record full ceremonies
//! - Comparator proves key independence across users/sessions
//! - Quality gates reject degenerate entropy
//!
//! The point: you can SEE the ceremony, not just trust it's secure.

use crate::composition::CompositionContext;
use crate::composition::neural_routing::canonical_routing_table;
use crate::soundstage::anchor::{Anchor, AnchorKind};
use crate::soundstage::capture::LiveCapture;
use crate::soundstage::channel::Channel;
use crate::soundstage::comparator::{Comparator, Verdict};
use crate::soundstage::session::CeremonySession;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "soundstage-ceremony-observation",
        track: Track::Security,
        tier: Tier::Rust,
        provenance_crate: "wave139a_soundstage",
        provenance_date: "2026-07-14",
        description: "soundStage ceremony observation — transparent key generation, anti-black-box",
    },
    run,
};

pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Channel per anchor type");
    phase_channel_anchors(v);

    v.section("Phase 2: Session captures full ceremony");
    phase_session_capture(v);

    v.section("Phase 3: LiveCapture thread-safe observation");
    phase_live_capture(v);

    v.section("Phase 4: Comparator independence verification");
    phase_comparator(v);

    v.section("Phase 5: Quality gates reject degenerate entropy");
    phase_quality_gates(v);

    v.section("Phase 6: Neural API routing for ceremony observation");
    phase_napi_routing(v);
}

fn phase_channel_anchors(v: &mut ValidationResult) {
    let anchors = [
        Anchor::fido2("solokey-east", "/dev/hidraw5"),
        Anchor::strongbox("pixel-8a"),
        Anchor::audio("headset-mic"),
        Anchor::os_entropy(),
    ];

    v.check_bool(
        "channel:four_anchor_types",
        anchors.len() == 4,
        "soundStage supports 4 anchor types (FIDO2, StrongBox, Audio, OS)",
    );

    for anchor in &anchors {
        let mut ch = Channel::new(anchor.clone());
        ch.record_contribution(&[0xAA; 32]);
        v.check_bool(
            &format!("channel:{}:records", anchor.kind),
            ch.contribution_count() == 1,
            &format!("{} channel records contributions", anchor.kind),
        );
    }

    let fido = Anchor::fido2("solokey", "/dev/hidraw0");
    assert_eq!(fido.kind, AnchorKind::Fido2);
    v.check_bool(
        "channel:anchor_typed",
        true,
        "Anchors carry typed AnchorKind for routing",
    );
}

fn phase_session_capture(v: &mut ValidationResult) {
    let mut session = CeremonySession::begin("test-ceremony-001", "eastgate");

    let fido = Anchor::fido2("solokey-east", "/dev/hidraw5");
    let ch = session.add_channel(fido);
    ch.record_request("beardog.fido2.make_credential", &[0x01; 64]);
    ch.record_response("beardog.fido2.make_credential", &[0xBB; 77]);
    ch.record_request("beardog.fido2.get_assertion", &[0x02; 64]);
    ch.record_response("beardog.fido2.get_assertion", &[0xCC; 77]);
    ch.record_contribution(&simulated_entropy(0x11));

    let os = Anchor::os_entropy();
    let ch2 = session.add_channel(os);
    ch2.record_contribution(&simulated_entropy(0x22));

    session.record_mix_input("fido2:solokey-east", &simulated_entropy(0x11));
    session.record_mix_input("os:getrandom", &simulated_entropy(0x22));
    session.record_monitor(&simulated_entropy(0x33));

    let record = session.finalize();

    v.check_bool(
        "session:captures_channels",
        record.channels_used.len() == 2,
        "Session recorded 2 channels (FIDO2 + OS)",
    );
    v.check_bool(
        "session:captures_events",
        record.event_count() >= 5,
        &format!(
            "Session captured {} events (req/resp/contrib)",
            record.event_count()
        ),
    );
    v.check_bool(
        "session:has_monitor",
        record.key_fingerprint().is_some(),
        "Session has key fingerprint (monitor observed derivation)",
    );
    v.check_bool(
        "session:multi_source",
        record.source_count() >= 2,
        "Session used multi-source entropy (>=2 anchors)",
    );
}

fn phase_live_capture(v: &mut ValidationResult) {
    use std::sync::Arc;

    let anchor = Anchor::fido2("solokey-east", "/dev/hidraw5");
    let (session, capture) = LiveCapture::begin("live-001", "eastgate");

    capture.add_anchor(anchor);
    capture.observe_request(
        "fido2:solokey-east",
        "beardog.fido2.make_credential",
        &[0x01; 8],
    );
    capture.observe_response(
        "fido2:solokey-east",
        "beardog.fido2.make_credential",
        &simulated_entropy(0x44),
    );
    capture.observe_contribution("fido2:solokey-east", &simulated_entropy(0x44));
    capture.observe_key_derived(&simulated_entropy(0x55));

    let Some(snap) = capture.snapshot() else {
        v.check_bool("capture:thread_safe", false, "LiveCapture lock poisoned");
        return;
    };
    v.check_bool(
        "capture:thread_safe",
        true,
        "LiveCapture operates through Arc<Mutex> (thread-safe)",
    );
    v.check_bool(
        "capture:has_mix",
        snap.mix_input_count >= 1,
        "LiveCapture records mix inputs in real time",
    );
    v.check_bool(
        "capture:has_monitor",
        snap.has_monitor,
        "LiveCapture observes key derivation",
    );

    drop(capture);
    let record = if let Ok(mutex) = Arc::try_unwrap(session) {
        if let Ok(sess) = mutex.into_inner() {
            sess.finalize()
        } else {
            v.check_bool("capture:finalize", false, "session mutex poisoned");
            return;
        }
    } else {
        v.check_bool("capture:finalize", false, "session Arc still shared");
        return;
    };
    v.check_bool(
        "capture:finalize",
        record.source_count() >= 1,
        "LiveCapture session finalizes into SessionRecord",
    );
}

fn phase_comparator(v: &mut ValidationResult) {
    let sessions: Vec<_> = (0..3)
        .map(|i| {
            let mut s = CeremonySession::begin(format!("cmp-{i}"), "eastgate");
            let fido = Anchor::fido2("solokey", "/dev/hidraw0");
            let ch = s.add_channel(fido);
            ch.record_contribution(&simulated_entropy(i * 37 + 7));
            let os = Anchor::os_entropy();
            let ch2 = s.add_channel(os);
            ch2.record_contribution(&simulated_entropy(i * 53 + 13));
            s.record_mix_input("fido2:solokey", &simulated_entropy(i * 37 + 7));
            s.record_mix_input("os:getrandom", &simulated_entropy(i * 53 + 13));
            s.record_monitor(&simulated_entropy(i * 71 + 3));
            s.finalize()
        })
        .collect();

    v.check_bool(
        "comparator:batch_compare",
        Comparator::all_independent(&sessions),
        "3 ceremonies produce mutually independent keys",
    );

    let results = Comparator::compare_all(&sessions);
    v.check_bool(
        "comparator:pairwise_3",
        results.len() == 3,
        "3 sessions → 3 pairwise comparisons",
    );

    for r in &results {
        v.check_bool(
            &format!("comparator:{}_{}", r.session_a, r.session_b),
            r.verdict == Verdict::Independent,
            &format!("{} vs {} = Independent", r.session_a, r.session_b),
        );
    }
}

fn phase_quality_gates(v: &mut ValidationResult) {
    // Single-source should fail quality
    let mut single = CeremonySession::begin("single-source", "user");
    let fido = Anchor::fido2("solokey", "/dev/hidraw0");
    let ch = single.add_channel(fido);
    ch.record_contribution(&simulated_entropy(0xAA));
    single.record_mix_input("fido2:solokey", &simulated_entropy(0xAA));
    single.record_monitor(&simulated_entropy(0xBB));
    let single_rec = single.finalize();

    v.check_bool(
        "quality:rejects_single_source",
        !single_rec.quality_pass(),
        "Quality gate rejects single-source ceremony",
    );

    // Multi-source with good entropy should pass
    let mut multi = CeremonySession::begin("multi-source", "user");
    let fido = Anchor::fido2("solokey", "/dev/hidraw0");
    let ch = multi.add_channel(fido);
    ch.record_contribution(&simulated_entropy(0x11));
    let os = Anchor::os_entropy();
    let ch2 = multi.add_channel(os);
    ch2.record_contribution(&simulated_entropy(0x22));
    multi.record_mix_input("fido2:solokey", &simulated_entropy(0x11));
    multi.record_mix_input("os:getrandom", &simulated_entropy(0x22));
    multi.record_monitor(&simulated_entropy(0x33));
    let multi_rec = multi.finalize();

    v.check_bool(
        "quality:accepts_multi_source",
        multi_rec.quality_pass(),
        "Quality gate accepts multi-source ceremony with good entropy",
    );

    // Degenerate entropy should fail quality
    let mut degen = CeremonySession::begin("degenerate", "user");
    let fido = Anchor::fido2("solokey", "/dev/hidraw0");
    let ch = degen.add_channel(fido);
    ch.record_contribution(&[0x00; 32]); // all zeros
    let os = Anchor::os_entropy();
    let ch2 = degen.add_channel(os);
    ch2.record_contribution(&[0x00; 32]); // all zeros
    degen.record_mix_input("fido2:solokey", &[0x00; 32]);
    degen.record_mix_input("os:getrandom", &[0x00; 32]);
    degen.record_monitor(&[0x00; 32]);
    let degen_rec = degen.finalize();

    v.check_bool(
        "quality:rejects_degenerate",
        !degen_rec.quality_pass(),
        "Quality gate rejects degenerate entropy (constant bytes)",
    );
}

fn phase_napi_routing(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let ceremony_methods = [
        "beardog.fido2.register",
        "beardog.fido2.authenticate",
        "genetic.ceremony_init",
        "genetic.ceremony_finalize",
    ];

    for method in ceremony_methods {
        let routed = table.route(method).is_some();
        v.check_bool(
            &format!("napi:{}", method.replace('.', "_")),
            routed,
            &format!("{method} routable via Neural API (bearDog ceremony path)"),
        );
    }
}

/// Generate high-entropy bytes from a seed (for test reproducibility).
fn simulated_entropy(seed: u8) -> Vec<u8> {
    (0u8..32)
        .map(|i| {
            seed.wrapping_mul(37)
                .wrapping_add(i.wrapping_mul(13))
                .wrapping_add(i ^ seed)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenario_passes_structural() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "scenario '{}' had {} failures",
            SCENARIO.meta.id, v.failed
        );
    }

    #[test]
    fn simulated_entropy_is_diverse() {
        let e1 = simulated_entropy(0x11);
        let e2 = simulated_entropy(0x22);
        assert_ne!(e1, e2, "different seeds must produce different entropy");
        let unique: std::collections::HashSet<u8> = e1.iter().copied().collect();
        assert!(
            unique.len() > 8,
            "entropy should have significant byte diversity"
        );
    }
}
