// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Deployment targets — the architectures, OS environments, and constraint
//! surfaces that primals must survive to prove silicon-atheism.

use std::fmt;

/// A deployment target — specific architecture + OS + constraint surface.
///
/// Each target represents a unique set of selection pressures. A primal
/// that passes validation on ALL its declared targets has proven its
/// computation is universal math, not silicon theology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Target {
    /// Standard x86_64 Linux with musl (eastGate, fieldGate, ironGate).
    X86_64Musl,
    /// ARM64 Linux with musl (grapheneGate — SELinux, restricted dirs).
    Aarch64Musl,
    /// RISC-V 64-bit Linux with musl (toadStool gateway, SiFive boards).
    Riscv64Musl,
    /// WebAssembly + WASI (browser/serverless, sandboxed, no filesystem).
    Wasm32Wasi,
    /// Bare metal — ecoPrimals as its own OS, no Linux.
    BareMetal(Arch),
}

/// CPU architecture for bare-metal targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arch {
    /// x86_64 bare metal.
    X86_64,
    /// ARM64 bare metal.
    Aarch64,
    /// RISC-V 64 bare metal.
    Riscv64,
}

impl Target {
    /// The Rust target triple string for this target (for cross-compilation).
    #[must_use]
    pub const fn triple(self) -> &'static str {
        match self {
            Self::X86_64Musl => "x86_64-unknown-linux-musl",
            Self::Aarch64Musl => "aarch64-unknown-linux-musl",
            Self::Riscv64Musl => "riscv64gc-unknown-linux-musl",
            Self::Wasm32Wasi => "wasm32-wasip1",
            Self::BareMetal(Arch::X86_64) => "x86_64-unknown-none",
            Self::BareMetal(Arch::Aarch64) => "aarch64-unknown-none",
            Self::BareMetal(Arch::Riscv64) => "riscv64gc-unknown-none-elf",
        }
    }

    /// Whether this target requires Linux (kernel + syscall ABI).
    #[must_use]
    pub const fn requires_linux(self) -> bool {
        matches!(
            self,
            Self::X86_64Musl | Self::Aarch64Musl | Self::Riscv64Musl
        )
    }

    /// Whether this target has filesystem access.
    #[must_use]
    pub const fn has_filesystem(self) -> bool {
        matches!(
            self,
            Self::X86_64Musl | Self::Aarch64Musl | Self::Riscv64Musl
        )
    }

    /// Whether this target supports Unix domain sockets.
    #[must_use]
    pub const fn has_uds(self) -> bool {
        matches!(self, Self::X86_64Musl | Self::Riscv64Musl)
    }

    /// Whether TCP networking is available.
    #[must_use]
    pub const fn has_tcp(self) -> bool {
        !matches!(self, Self::BareMetal(_))
    }

    /// The deployment tier (how constrained this target is).
    #[must_use]
    pub const fn tier(self) -> DeploymentTier {
        match self {
            Self::X86_64Musl => DeploymentTier::Permissive,
            Self::Aarch64Musl => DeploymentTier::Restricted,
            Self::Riscv64Musl => DeploymentTier::Constrained,
            Self::Wasm32Wasi => DeploymentTier::Sandboxed,
            Self::BareMetal(_) => DeploymentTier::Bare,
        }
    }

    /// Detect the current host target.
    #[must_use]
    pub fn current() -> Self {
        let triple = crate::tolerances::platform::current_target_triple();
        if triple.contains("x86_64") && triple.contains("linux") {
            Self::X86_64Musl
        } else if triple.contains("aarch64") && triple.contains("linux") {
            Self::Aarch64Musl
        } else if triple.contains("riscv64") {
            Self::Riscv64Musl
        } else if triple.contains("wasm") {
            Self::Wasm32Wasi
        } else {
            Self::X86_64Musl
        }
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.triple())
    }
}

/// How constrained a deployment tier is — determines which selection
/// pressures apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeploymentTier {
    /// Full Linux, writable filesystem, UDS, permissive security (eastGate).
    Permissive,
    /// Linux but with SELinux/AppArmor restrictions (grapheneGate).
    Restricted,
    /// Linux but with severe resource limits (fieldMouse: ≤256MB, single-core).
    Constrained,
    /// No OS — capabilities granted explicitly (WASI, capability-based).
    Sandboxed,
    /// No OS at all — hardware-direct, primals ARE the processes.
    Bare,
}

/// Composition deployment tier — what size NUCLEUS can run on this target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositionTier {
    /// Full 13-primal NUCLEUS.
    Full,
    /// 7-10 primals (Nest/Node atomic).
    Standard,
    /// 3-5 primals (Tower Atomic).
    Light,
    /// 1-2 primals (Micro — embedded fieldMouse).
    Micro,
}

impl CompositionTier {
    /// Maximum primal count for this tier.
    #[must_use]
    pub const fn max_primals(self) -> usize {
        match self {
            Self::Full => 13,
            Self::Standard => 10,
            Self::Light => 5,
            Self::Micro => 2,
        }
    }

    /// Determine composition tier from available resources.
    #[must_use]
    pub const fn from_target(target: Target) -> Self {
        match target {
            Target::X86_64Musl | Target::Riscv64Musl => Self::Full,
            Target::Aarch64Musl => Self::Light,
            Target::Wasm32Wasi | Target::BareMetal(_) => Self::Micro,
        }
    }
}
