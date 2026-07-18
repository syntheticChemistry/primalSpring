// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Channel — a single observable entropy signal from an anchor.
//!
//! Each channel represents one flow of entropy from a hardware source.
//! The channel captures every byte that flows through it, timestamped,
//! so you can see exactly what the hardware produced.

use super::anchor::Anchor;
use std::time::{Duration, Instant};

/// A live signal observation from a channel.
#[derive(Debug, Clone)]
pub struct Signal {
    /// Raw bytes observed (entropy contribution).
    pub data: Vec<u8>,
    /// BLAKE3 fingerprint of the signal data.
    pub fingerprint: [u8; 32],
    /// Byte length of the signal.
    pub len: usize,
    /// Shannon entropy estimate (bits per byte, max 8.0).
    pub entropy_estimate: f64,
}

/// An event on a channel — something observable happened.
#[derive(Debug, Clone)]
pub struct ChannelEvent {
    /// Which anchor produced this event.
    pub anchor: Anchor,
    /// What kind of event.
    pub kind: EventKind,
    /// When it happened (relative to session start).
    pub offset: Duration,
    /// The signal data, if this is a data event.
    pub signal: Option<Signal>,
    /// IPC method that was called (e.g. "beardog.fido2.authenticate").
    pub method: Option<String>,
}

/// Event types on a channel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    /// Request sent to hardware (outbound).
    Request,
    /// Response received from hardware (inbound).
    Response,
    /// Entropy contribution flowing into mix bus.
    Contribute,
    /// Error or timeout from hardware.
    Error,
    /// Hardware acknowledged (e.g., user touched key).
    Acknowledge,
}

/// A channel — one observable entropy source during a ceremony.
#[derive(Debug)]
pub struct Channel {
    pub anchor: Anchor,
    events: Vec<ChannelEvent>,
    start: Instant,
}

impl Channel {
    #[must_use]
    pub fn new(anchor: Anchor) -> Self {
        Self {
            anchor,
            events: Vec::new(),
            start: Instant::now(),
        }
    }

    /// Record a request being sent to the hardware.
    pub fn record_request(&mut self, method: &str, data: &[u8]) {
        self.events.push(ChannelEvent {
            anchor: self.anchor.clone(),
            kind: EventKind::Request,
            offset: self.start.elapsed(),
            signal: Some(make_signal(data)),
            method: Some(method.to_string()),
        });
    }

    /// Record a response received from the hardware.
    pub fn record_response(&mut self, method: &str, data: &[u8]) {
        self.events.push(ChannelEvent {
            anchor: self.anchor.clone(),
            kind: EventKind::Response,
            offset: self.start.elapsed(),
            signal: Some(make_signal(data)),
            method: Some(method.to_string()),
        });
    }

    /// Record an entropy contribution flowing to the mix bus.
    pub fn record_contribution(&mut self, data: &[u8]) {
        self.events.push(ChannelEvent {
            anchor: self.anchor.clone(),
            kind: EventKind::Contribute,
            offset: self.start.elapsed(),
            signal: Some(make_signal(data)),
            method: None,
        });
    }

    /// Record an error or timeout.
    pub fn record_error(&mut self, method: &str) {
        self.events.push(ChannelEvent {
            anchor: self.anchor.clone(),
            kind: EventKind::Error,
            offset: self.start.elapsed(),
            signal: None,
            method: Some(method.to_string()),
        });
    }

    /// Record user acknowledgement (touch, biometric, etc).
    pub fn record_acknowledge(&mut self) {
        self.events.push(ChannelEvent {
            anchor: self.anchor.clone(),
            kind: EventKind::Acknowledge,
            offset: self.start.elapsed(),
            signal: None,
            method: None,
        });
    }

    /// All events captured on this channel.
    #[must_use]
    pub fn events(&self) -> &[ChannelEvent] {
        &self.events
    }

    /// Consume the channel and return its events.
    #[must_use]
    pub fn into_events(self) -> Vec<ChannelEvent> {
        self.events
    }

    /// Total data bytes observed on this channel.
    #[must_use]
    pub fn total_bytes(&self) -> usize {
        self.events
            .iter()
            .filter_map(|e| e.signal.as_ref())
            .map(|s| s.len)
            .sum()
    }

    /// Number of contributions (entropy flowing to mix bus).
    #[must_use]
    pub fn contribution_count(&self) -> usize {
        self.events
            .iter()
            .filter(|e| e.kind == EventKind::Contribute)
            .count()
    }
}

/// Shannon entropy (public for use by session module).
#[must_use]
pub fn shannon_entropy_pub(data: &[u8]) -> f64 {
    shannon_entropy(data)
}

fn make_signal(data: &[u8]) -> Signal {
    let fingerprint = blake3::hash(data).into();
    let entropy_estimate = shannon_entropy(data);
    Signal {
        data: data.to_vec(),
        fingerprint,
        len: data.len(),
        entropy_estimate,
    }
}

/// Shannon entropy in bits per byte (0.0 = all same, 8.0 = perfectly random).
fn shannon_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mut counts = [0u64; 256];
    for &b in data {
        counts[b as usize] += 1;
    }
    let len = data.len() as f64;
    counts
        .iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / len;
            -p * p.log2()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::soundstage::anchor::Anchor;

    #[test]
    fn channel_records_events() {
        let anchor = Anchor::fido2("test-key", "/dev/hidraw0");
        let mut ch = Channel::new(anchor);

        ch.record_request("beardog.fido2.authenticate", &[0x01, 0x02, 0x03]);
        ch.record_response("beardog.fido2.authenticate", &[0xAA; 32]);
        ch.record_contribution(&[0xBB; 32]);

        assert_eq!(ch.events().len(), 3);
        assert_eq!(ch.contribution_count(), 1);
        assert_eq!(ch.total_bytes(), 3 + 32 + 32);
    }

    #[test]
    fn shannon_entropy_uniform() {
        let data: Vec<u8> = (0..=255).collect();
        let e = shannon_entropy(&data);
        assert!(
            (e - 8.0).abs() < 0.001,
            "uniform data should have ~8.0 bits/byte, got {e}"
        );
    }

    #[test]
    fn shannon_entropy_constant() {
        let data = vec![0x42; 100];
        let e = shannon_entropy(&data);
        assert!(
            e < 0.001,
            "constant data should have ~0.0 bits/byte, got {e}"
        );
    }

    #[test]
    fn signal_fingerprint_deterministic() {
        let data = b"test entropy data";
        let s1 = make_signal(data);
        let s2 = make_signal(data);
        assert_eq!(s1.fingerprint, s2.fingerprint);
    }

    #[test]
    fn signal_fingerprint_differs_for_different_data() {
        let s1 = make_signal(b"source A");
        let s2 = make_signal(b"source B");
        assert_ne!(s1.fingerprint, s2.fingerprint);
    }
}
