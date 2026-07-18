// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Selection pressure categories — the forces that drive primal evolution.
//!
//! Each deployment target applies a unique combination of pressures.
//! Primals that fail under a specific pressure carry evolution debt
//! in that category until they adapt.

use super::target::Target;

/// Categories of selection pressure applied by deployment targets.
///
/// These are the axes along which platform theology manifests. A primal
/// that assumes writable `/var/run` has `Filesystem` theology. One that
/// requires UDS has `IpcTransport` theology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PressureCategory {
    /// Path assumptions, directory creation, read-only filesystems.
    /// Example: Songbird EROFS on grapheneGate.
    Filesystem,
    /// UDS/TCP/relay transport assumptions.
    /// Example: `SkunkBat` UDS-only crash on `SELinux`.
    IpcTransport,
    /// Heap allocation patterns, caching strategies, OOM behavior.
    Memory,
    /// Data serialization byte-order assumptions.
    Endianness,
    /// Struct packing, pointer alignment requirements.
    Alignment,
    /// Linux-specific syscalls, ioctl, procfs assumptions.
    SyscallAbi,
    /// LAN-only assumptions, latency tolerance, NAT traversal.
    Network,
    /// Thread count assumptions, async runtime requirements.
    Concurrency,
    /// Clock monotonicity, NTP availability, timezone.
    Time,
    /// Security policy restrictions (`SELinux`, `AppArmor`, sandboxing).
    SecurityPolicy,
    /// Not yet classified (pending investigation).
    Unknown,
}

impl PressureCategory {
    /// Short label for display in fitness reports.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Filesystem => "filesystem",
            Self::IpcTransport => "ipc-transport",
            Self::Memory => "memory",
            Self::Endianness => "endianness",
            Self::Alignment => "alignment",
            Self::SyscallAbi => "syscall-abi",
            Self::Network => "network",
            Self::Concurrency => "concurrency",
            Self::Time => "time",
            Self::SecurityPolicy => "security-policy",
            Self::Unknown => "unknown",
        }
    }

    /// Which pressures are active for a given target?
    #[must_use]
    pub fn active_for(target: Target) -> Vec<Self> {
        match target {
            Target::X86_64Musl => vec![Self::Network],
            Target::Aarch64Musl => vec![
                Self::Filesystem,
                Self::IpcTransport,
                Self::SecurityPolicy,
                Self::Memory,
                Self::Network,
            ],
            Target::Riscv64Musl => vec![
                Self::Alignment,
                Self::Endianness,
                Self::Concurrency,
                Self::Memory,
            ],
            Target::Wasm32Wasi => vec![
                Self::Filesystem,
                Self::IpcTransport,
                Self::SyscallAbi,
                Self::Network,
                Self::Concurrency,
                Self::Time,
            ],
            Target::BareMetal(_) => vec![
                Self::Filesystem,
                Self::IpcTransport,
                Self::SyscallAbi,
                Self::Network,
                Self::Concurrency,
                Self::Time,
                Self::Memory,
            ],
        }
    }

    /// How many unique pressures does this target apply?
    #[must_use]
    pub fn pressure_count(target: Target) -> usize {
        Self::active_for(target).len()
    }
}
