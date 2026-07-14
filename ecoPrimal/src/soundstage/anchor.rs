// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Hardware anchor abstraction — the physical trust roots.

use std::fmt;

/// A hardware trust anchor — the physical source of entropy or key material.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Anchor {
    pub kind: AnchorKind,
    pub label: String,
    pub device_path: Option<String>,
}

/// Categories of trust anchors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnchorKind {
    /// FIDO2 token (SoloKey, YubiKey) — CTAP2 hmac-secret for entropy.
    Fido2,
    /// Hardware security module (Titan M2 StrongBox) — key storage + attestation.
    StrongBox,
    /// Audio capture (mic/headset) — environmental entropy.
    Audio,
    /// OS entropy pool (getrandom/urandom) — software baseline.
    OsEntropy,
    /// Custom/future anchor type.
    Custom,
}

impl Anchor {
    pub fn fido2(label: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            kind: AnchorKind::Fido2,
            label: label.into(),
            device_path: Some(path.into()),
        }
    }

    pub fn strongbox(label: impl Into<String>) -> Self {
        Self {
            kind: AnchorKind::StrongBox,
            label: label.into(),
            device_path: None,
        }
    }

    pub fn audio(label: impl Into<String>) -> Self {
        Self {
            kind: AnchorKind::Audio,
            label: label.into(),
            device_path: None,
        }
    }

    pub fn os_entropy() -> Self {
        Self {
            kind: AnchorKind::OsEntropy,
            label: "getrandom".into(),
            device_path: None,
        }
    }
}

impl fmt::Display for Anchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.kind, self.label)
    }
}

impl fmt::Display for AnchorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fido2 => write!(f, "fido2"),
            Self::StrongBox => write!(f, "strongbox"),
            Self::Audio => write!(f, "audio"),
            Self::OsEntropy => write!(f, "os"),
            Self::Custom => write!(f, "custom"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchor_display() {
        let sk = Anchor::fido2("solokey-east", "/dev/hidraw5");
        assert_eq!(sk.to_string(), "fido2:solokey-east");
        assert_eq!(sk.device_path.as_deref(), Some("/dev/hidraw5"));
    }

    #[test]
    fn anchor_kinds_distinct() {
        let a = Anchor::fido2("key1", "/dev/hidraw0");
        let b = Anchor::strongbox("pixel");
        assert_ne!(a.kind, b.kind);
    }
}
