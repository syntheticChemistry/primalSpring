// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! LiveCapture — real-time observation stream from a ceremony.
//!
//! `LiveCapture` wraps a ceremony session and provides a stream of
//! observable events. This is what you'd display in a browser UI or
//! terminal to watch the key generation happening in real time.

use super::anchor::Anchor;
use super::session::CeremonySession;
use std::sync::{Arc, Mutex};

/// A live capture stream — observes a ceremony in real time.
///
/// Holds a reference to the session and can be polled for new events.
/// Multiple observers can watch the same session simultaneously.
pub struct LiveCapture {
    session: Arc<Mutex<CeremonySession>>,
}

impl LiveCapture {
    /// Create a new capture observing a shared session.
    pub fn new(session: Arc<Mutex<CeremonySession>>) -> Self {
        Self { session }
    }

    /// Start a new ceremony and return both the session handle and a capture.
    pub fn begin(id: impl Into<String>, user: impl Into<String>) -> (Arc<Mutex<CeremonySession>>, Self) {
        let session = Arc::new(Mutex::new(CeremonySession::begin(id, user)));
        let capture = Self::new(Arc::clone(&session));
        (session, capture)
    }

    /// Add a channel to the observed session.
    pub fn add_anchor(&self, anchor: Anchor) {
        if let Ok(mut s) = self.session.lock() {
            s.add_channel(anchor);
        }
    }

    /// Record a request on a channel (call this when sending to hardware).
    pub fn observe_request(&self, anchor_label: &str, method: &str, data: &[u8]) {
        if let Ok(mut s) = self.session.lock() {
            if let Some(ch) = s.channel(anchor_label) {
                ch.record_request(method, data);
            }
        }
    }

    /// Record a response from hardware (call this when receiving from hardware).
    pub fn observe_response(&self, anchor_label: &str, method: &str, data: &[u8]) {
        if let Ok(mut s) = self.session.lock() {
            if let Some(ch) = s.channel(anchor_label) {
                ch.record_response(method, data);
            }
        }
    }

    /// Record entropy contribution to mix bus.
    pub fn observe_contribution(&self, anchor_label: &str, data: &[u8]) {
        if let Ok(mut s) = self.session.lock() {
            if let Some(ch) = s.channel(anchor_label) {
                ch.record_contribution(data);
            }
            s.record_mix_input(anchor_label, data);
        }
    }

    /// Record the final key derivation (fingerprint observation).
    pub fn observe_key_derived(&self, key_material: &[u8]) {
        if let Ok(mut s) = self.session.lock() {
            s.record_monitor(key_material);
        }
    }

    /// Get a snapshot summary of the current session state.
    pub fn snapshot(&self) -> Option<CaptureSnapshot> {
        let s = self.session.lock().ok()?;
        Some(CaptureSnapshot {
            id: s.id.clone(),
            user: s.user.clone(),
            mix_input_count: s.mix_input_count(),
            has_monitor: s.has_monitor(),
        })
    }
}

/// Point-in-time snapshot of a live capture.
#[derive(Debug, Clone)]
pub struct CaptureSnapshot {
    pub id: String,
    pub user: String,
    pub mix_input_count: usize,
    pub has_monitor: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_capture_observe_flow() {
        let anchor = Anchor::fido2("solokey", "/dev/hidraw5");
        let (session, capture) = LiveCapture::begin("cap-001", "eastgate");

        capture.add_anchor(anchor);

        capture.observe_request("fido2:solokey", "beardog.fido2.authenticate", &[0x01; 8]);
        capture.observe_response("fido2:solokey", "beardog.fido2.authenticate", &[0xAA; 32]);
        capture.observe_contribution("fido2:solokey", &[0xAA; 32]);
        capture.observe_key_derived(&[0xFF; 32]);

        let snap = capture.snapshot().unwrap();
        assert_eq!(snap.mix_input_count, 1);
        assert!(snap.has_monitor);

        drop(capture);
        let record = Arc::try_unwrap(session)
            .unwrap()
            .into_inner()
            .unwrap()
            .finalize();
        assert_eq!(record.source_count(), 1);
    }
}
