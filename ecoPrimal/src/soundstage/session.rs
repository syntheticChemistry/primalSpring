// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Session — a complete ceremony observation from start to finish.
//!
//! A session captures everything: which channels were active, what data
//! flowed through them, what the mix bus produced, and what the final
//! key fingerprint looks like. Sessions are the unit of comparison.

use super::anchor::Anchor;
use super::channel::{Channel, ChannelEvent, EventKind};
use std::collections::HashMap;
use std::time::Instant;

/// A live ceremony session — records all channels and mixing in real time.
#[derive(Debug)]
pub struct CeremonySession {
    /// Unique session identifier.
    pub id: String,
    /// User/operator who initiated the ceremony.
    pub user: String,
    channels: HashMap<String, Channel>,
    mix_inputs: Vec<MixInput>,
    monitor: Option<Monitor>,
    start: Instant,
}

/// An entropy contribution that went into the mix bus.
#[derive(Debug, Clone)]
pub struct MixInput {
    /// Which anchor produced this entropy.
    pub anchor_label: String,
    /// BLAKE3 fingerprint of the contributed bytes.
    pub fingerprint: [u8; 32],
    /// Byte length of the contribution.
    pub len: usize,
    /// Shannon entropy estimate (bits/byte, max 8.0).
    pub entropy_estimate: f64,
}

/// The final output observation — fingerprint only, never raw key material.
#[derive(Debug, Clone)]
pub struct Monitor {
    /// BLAKE3 fingerprint of the derived key (not the key itself).
    pub key_fingerprint: [u8; 32],
    /// Number of entropy sources that contributed.
    pub source_count: usize,
    /// Total entropy bytes mixed.
    pub total_entropy_bytes: usize,
    /// Whether the key passed basic quality checks.
    pub quality_pass: bool,
}

/// A completed session recording — serializable, comparable.
#[derive(Debug, Clone)]
pub struct SessionRecord {
    /// Unique session identifier.
    pub id: String,
    /// User/operator who ran the ceremony.
    pub user: String,
    /// Labels of all channels that participated.
    pub channels_used: Vec<String>,
    /// All captured events across all channels.
    pub events: Vec<ChannelEvent>,
    /// Entropy contributions that entered the mix bus.
    pub mix_inputs: Vec<MixInput>,
    /// Final key observation (fingerprint only, never raw material).
    pub monitor: Option<Monitor>,
    /// Wall-clock duration of the ceremony in milliseconds.
    pub duration_ms: u128,
}

impl CeremonySession {
    /// Start a new ceremony session.
    pub fn begin(id: impl Into<String>, user: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            user: user.into(),
            channels: HashMap::new(),
            mix_inputs: Vec::new(),
            monitor: None,
            start: Instant::now(),
        }
    }

    /// Add a channel (entropy source) to this session.
    pub fn add_channel(&mut self, anchor: Anchor) -> &mut Channel {
        let label = anchor.to_string();
        self.channels
            .entry(label)
            .or_insert_with(|| Channel::new(anchor))
    }

    /// Get a channel by label.
    pub fn channel(&mut self, label: &str) -> Option<&mut Channel> {
        self.channels.get_mut(label)
    }

    /// Record entropy flowing into the mix bus from a channel.
    pub fn record_mix_input(&mut self, anchor_label: &str, data: &[u8]) {
        let fingerprint = blake3::hash(data).into();
        let entropy_estimate = super::channel::shannon_entropy_pub(data);
        self.mix_inputs.push(MixInput {
            anchor_label: anchor_label.to_string(),
            fingerprint,
            len: data.len(),
            entropy_estimate,
        });
    }

    /// Record the final key observation (fingerprint only — not the actual key).
    pub fn record_monitor(&mut self, key_material: &[u8]) {
        let key_fingerprint = blake3::hash(key_material).into();
        let source_count = self.mix_inputs.len();
        let total_entropy_bytes: usize = self.mix_inputs.iter().map(|m| m.len).sum();

        let quality_pass = source_count >= 2
            && total_entropy_bytes >= 32
            && self.mix_inputs.iter().all(|m| m.entropy_estimate > 4.0);

        self.monitor = Some(Monitor {
            key_fingerprint,
            source_count,
            total_entropy_bytes,
            quality_pass,
        });
    }

    /// Number of mix inputs recorded so far.
    #[must_use]
    pub const fn mix_input_count(&self) -> usize {
        self.mix_inputs.len()
    }

    /// Whether a monitor (key fingerprint) has been recorded.
    #[must_use]
    pub const fn has_monitor(&self) -> bool {
        self.monitor.is_some()
    }

    /// Finalize the session into a serializable record.
    pub fn finalize(self) -> SessionRecord {
        let duration_ms = self.start.elapsed().as_millis();
        let channels_used: Vec<String> = self.channels.keys().cloned().collect();
        let events: Vec<ChannelEvent> = self
            .channels
            .into_values()
            .flat_map(Channel::into_events)
            .collect();

        SessionRecord {
            id: self.id,
            user: self.user,
            channels_used,
            events,
            mix_inputs: self.mix_inputs,
            monitor: self.monitor,
            duration_ms,
        }
    }
}

impl SessionRecord {
    /// Total events captured in this session.
    #[must_use]
    pub const fn event_count(&self) -> usize {
        self.events.len()
    }

    /// How many distinct sources contributed entropy.
    #[must_use]
    pub const fn source_count(&self) -> usize {
        self.mix_inputs.len()
    }

    /// Whether the ceremony produced a quality key.
    #[must_use]
    pub fn quality_pass(&self) -> bool {
        self.monitor.as_ref().is_some_and(|m| m.quality_pass)
    }

    /// Key fingerprint (for comparison with other sessions).
    #[must_use]
    pub fn key_fingerprint(&self) -> Option<[u8; 32]> {
        self.monitor.as_ref().map(|m| m.key_fingerprint)
    }

    /// All response events (data flowing back FROM hardware).
    #[must_use]
    pub fn responses(&self) -> Vec<&ChannelEvent> {
        self.events
            .iter()
            .filter(|e| e.kind == EventKind::Response)
            .collect()
    }

    /// All contribution events (data flowing INTO mix bus).
    #[must_use]
    pub fn contributions(&self) -> Vec<&ChannelEvent> {
        self.events
            .iter()
            .filter(|e| e.kind == EventKind::Contribute)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::soundstage::anchor::Anchor;

    #[test]
    fn session_lifecycle() {
        let mut session = CeremonySession::begin("test-001", "eastgate");

        let anchor = Anchor::fido2("solokey", "/dev/hidraw5");
        let ch = session.add_channel(anchor);
        ch.record_request("beardog.fido2.authenticate", &[0x01; 8]);
        ch.record_response("beardog.fido2.authenticate", &[0xAA; 32]);
        ch.record_contribution(&[0xAA; 32]);

        let os = Anchor::os_entropy();
        let ch2 = session.add_channel(os);
        ch2.record_contribution(&[0xBB; 32]);

        session.record_mix_input("fido2:solokey", &[0xAA; 32]);
        session.record_mix_input("os:getrandom", &[0xBB; 32]);
        session.record_monitor(&[0xCC; 32]);

        let record = session.finalize();

        assert_eq!(record.source_count(), 2);
        assert_eq!(record.channels_used.len(), 2);
        assert!(record.key_fingerprint().is_some());
    }

    #[test]
    fn quality_requires_multi_source() {
        let mut session = CeremonySession::begin("test-002", "eastgate");

        let anchor = Anchor::fido2("solokey", "/dev/hidraw5");
        let ch = session.add_channel(anchor);
        ch.record_contribution(&[0xAA; 32]);

        // Only one source — quality should fail
        session.record_mix_input("fido2:solokey", &[0xAA; 32]);
        session.record_monitor(&[0xDD; 32]);

        let record = session.finalize();
        assert!(
            !record.quality_pass(),
            "single-source ceremony should not pass quality"
        );
    }

    #[test]
    fn different_sessions_different_fingerprints() {
        let mut s1 = CeremonySession::begin("s1", "user1");
        s1.record_monitor(&[0x11; 32]);
        let r1 = s1.finalize();

        let mut s2 = CeremonySession::begin("s2", "user1");
        s2.record_monitor(&[0x22; 32]);
        let r2 = s2.finalize();

        assert_ne!(
            r1.key_fingerprint(),
            r2.key_fingerprint(),
            "different key material must produce different fingerprints"
        );
    }
}
