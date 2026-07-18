// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comparator — diff and verify sessions for independence and quality.
//!
//! The comparator answers the key questions:
//! - Did two ceremonies produce independent keys? (fingerprint divergence)
//! - Does the same ceremony reproduce? (determinism check with same inputs)
//! - Is one anchor type producing degenerate entropy? (entropy quality)
//! - Are multiple users getting independent key material? (multi-user safety)

use super::session::SessionRecord;

/// Comparison result between two sessions.
#[derive(Debug, Clone)]
pub struct ComparisonResult {
    pub session_a: String,
    pub session_b: String,
    pub keys_independent: bool,
    pub entropy_quality_a: f64,
    pub entropy_quality_b: f64,
    pub shared_anchors: Vec<String>,
    pub verdict: Verdict,
}

/// Summary judgment from a comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// Both sessions produced independent, high-quality keys.
    Independent,
    /// Sessions share key material (CRITICAL — same entropy source reused).
    Collision,
    /// One or both sessions have low entropy quality.
    DegenerateEntropy,
    /// Insufficient data to compare (one session missing monitor).
    Incomplete,
}

/// The comparator engine.
pub struct Comparator;

impl Comparator {
    /// Compare two completed sessions.
    #[must_use]
    pub fn compare(a: &SessionRecord, b: &SessionRecord) -> ComparisonResult {
        let fp_a = a.key_fingerprint();
        let fp_b = b.key_fingerprint();

        let (keys_independent, verdict) = match (fp_a, fp_b) {
            (Some(fa), Some(fb)) => {
                let independent = fa != fb;
                let verdict = if independent {
                    Verdict::Independent
                } else {
                    Verdict::Collision
                };
                (independent, verdict)
            }
            _ => (false, Verdict::Incomplete),
        };

        let entropy_quality_a = avg_entropy_quality(a);
        let entropy_quality_b = avg_entropy_quality(b);

        let verdict = if verdict == Verdict::Independent
            && (entropy_quality_a < 4.0 || entropy_quality_b < 4.0)
        {
            Verdict::DegenerateEntropy
        } else {
            verdict
        };

        let shared_anchors = find_shared_anchors(a, b);

        ComparisonResult {
            session_a: a.id.clone(),
            session_b: b.id.clone(),
            keys_independent,
            entropy_quality_a,
            entropy_quality_b,
            shared_anchors,
            verdict,
        }
    }

    /// Batch compare: all pairs in a set of sessions.
    #[must_use]
    pub fn compare_all(sessions: &[SessionRecord]) -> Vec<ComparisonResult> {
        let mut results = Vec::new();
        for i in 0..sessions.len() {
            for j in (i + 1)..sessions.len() {
                results.push(Self::compare(&sessions[i], &sessions[j]));
            }
        }
        results
    }

    /// Check that all sessions in a batch are mutually independent.
    #[must_use]
    pub fn all_independent(sessions: &[SessionRecord]) -> bool {
        Self::compare_all(sessions)
            .iter()
            .all(|r| r.verdict == Verdict::Independent)
    }
}

fn avg_entropy_quality(session: &SessionRecord) -> f64 {
    if session.mix_inputs.is_empty() {
        return 0.0;
    }
    let total: f64 = session.mix_inputs.iter().map(|m| m.entropy_estimate).sum();
    total / session.mix_inputs.len() as f64
}

fn find_shared_anchors(a: &SessionRecord, b: &SessionRecord) -> Vec<String> {
    a.channels_used
        .iter()
        .filter(|ch| b.channels_used.contains(ch))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::soundstage::anchor::Anchor;
    use crate::soundstage::session::CeremonySession;

    fn high_entropy_bytes(seed: u8) -> Vec<u8> {
        (0u8..32)
            .map(|i| {
                seed.wrapping_mul(37)
                    .wrapping_add(i.wrapping_mul(13))
                    .wrapping_add(i ^ seed)
            })
            .collect()
    }

    fn make_session(id: &str, user: &str, seed: u8) -> SessionRecord {
        let key_material = high_entropy_bytes(seed);
        let os_material = high_entropy_bytes(seed.wrapping_add(100));
        let mut session = CeremonySession::begin(id, user);

        let anchor = Anchor::fido2("solokey", "/dev/hidraw5");
        let ch = session.add_channel(anchor);
        ch.record_contribution(&key_material);

        let os = Anchor::os_entropy();
        let ch2 = session.add_channel(os);
        ch2.record_contribution(&os_material);

        session.record_mix_input("fido2:solokey", &key_material);
        session.record_mix_input("os:getrandom", &os_material);
        session.record_monitor(&key_material);

        session.finalize()
    }

    #[test]
    fn independent_sessions() {
        let a = make_session("s1", "user1", 0x11);
        let b = make_session("s2", "user2", 0x22);

        let result = Comparator::compare(&a, &b);
        assert!(result.keys_independent);
        assert_eq!(result.verdict, Verdict::Independent);
    }

    #[test]
    fn collision_detected() {
        let a = make_session("s1", "user1", 0x11);
        let b = make_session("s2", "user2", 0x11);

        let result = Comparator::compare(&a, &b);
        assert!(!result.keys_independent);
        assert_eq!(result.verdict, Verdict::Collision);
    }

    #[test]
    fn batch_independence() {
        let sessions: Vec<SessionRecord> = (0..5)
            .map(|i| make_session(&format!("s{i}"), "user", (i * 31 + 7) as u8))
            .collect();

        assert!(Comparator::all_independent(&sessions));
    }

    #[test]
    fn degenerate_entropy_detected() {
        let mut session = CeremonySession::begin("degen", "user");

        let anchor = Anchor::fido2("solokey", "/dev/hidraw5");
        let ch = session.add_channel(anchor);
        ch.record_contribution(&[0x00; 32]);

        let os = Anchor::os_entropy();
        let ch2 = session.add_channel(os);
        ch2.record_contribution(&[0x00; 32]);

        session.record_mix_input("fido2:solokey", &[0x00; 32]);
        session.record_mix_input("os:getrandom", &[0x00; 32]);
        session.record_monitor(&[0x00; 32]);

        let degen = session.finalize();
        let good = make_session("good", "user", 0xAA);

        let result = Comparator::compare(&degen, &good);
        assert_eq!(result.verdict, Verdict::DegenerateEntropy);
    }

    #[test]
    fn shared_anchors_found() {
        let a = make_session("s1", "user1", 0x11);
        let b = make_session("s2", "user2", 0x22);

        let result = Comparator::compare(&a, &b);
        assert!(!result.shared_anchors.is_empty());
    }
}
