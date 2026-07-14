// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! `soundStage` — transparent observation of ephemeral key generation.
//!
//! An ecoPrimals concept: **see the ceremony, don't trust the black box.**
//!
//! `soundStage` provides live visibility into the key generation process:
//! every entropy source, every mixing operation, every derivation step is
//! observable, recordable, and comparable. If you can't see it working,
//! you're just trusting it's secure.
//!
//! # Why
//!
//! Hardware security modules are opaque. You plug in a key, call an API,
//! get bytes back. How do you know those bytes are actually entropic?
//! How do you know the mixing isn't degenerate? How do you know two
//! ceremonies produce independent keys?
//!
//! `soundStage` answers: watch it happen. Record the stream. Compare
//! sessions. Diff the outputs. See the entropy flowing from each source,
//! through mixing, into the key.
//!
//! # Concepts
//!
//! - **Channel**: A single entropy source (SoloKey FIDO2, Pixel StrongBox,
//!   audio mic, OS getrandom). Each channel has a live signal.
//! - **Mix bus**: Where channels converge. Shows the mixing operation
//!   and its output in real time.
//! - **Monitor**: The derived key material, observable for comparison
//!   but never exported raw (hash/fingerprint only).
//! - **Session**: A complete ceremony recording — all channels, mix,
//!   and monitor output timestamped.
//! - **Comparator**: Diffs two sessions to prove independence, or
//!   replays a session to prove determinism.
//!
//! # Multi-anchor / Multi-user
//!
//! Each anchor (SoloKey, YubiKey, StrongBox, audio) is a channel.
//! Each user gets their own session. Sessions can be compared across
//! users and across anchor types to validate independence.

pub mod anchor;
pub mod capture;
pub mod channel;
pub mod comparator;
pub mod session;

pub use anchor::{Anchor, AnchorKind};
pub use capture::LiveCapture;
pub use channel::{Channel, ChannelEvent, Signal};
pub use comparator::Comparator;
pub use session::{CeremonySession, SessionRecord};
