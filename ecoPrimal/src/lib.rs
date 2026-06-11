// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(
    test,
    expect(
        clippy::unwrap_used,
        clippy::expect_used,
        reason = "test code permits panicking unwraps for assertion-style failures"
    )
)]

//! primalSpring — NUCLEUS evolution arena.
//!
//! primalSpring validates primal compositions (atomics, bonding, chimeras).
//! It is the bridge between upstream primals and downstream springs: primals
//! expose capabilities, primalSpring proves they compose correctly, springs
//! validate their domain science through those compositions.
//!
//! primalSpring is NOT a primal. It does not serve on a socket, register with
//! biomeOS, or appear in NUCLEUS compositions. It is a pure CLI + IPC client
//! that validates compositions from the outside.
//!
//! # Core Role
//!
//! - **Validate** that primal compositions produce correct results (parity with baselines)
//! - **Launch** NUCLEUS compositions (Tower, Node, Nest, `FullNucleus`) for testing
//! - **Surface** upstream gaps so primal teams know what to evolve
//! - **Provide** the composition validation library for downstream springs
//!
//! # Modules
//!
//! ## Core — composition validation spine
//! - [`coordination`] — atomic definitions (Tower, Node, Nest, Full NUCLEUS) and live probing
//! - [`composition`] — composition parity: call math through primals, compare against baselines
//! - [`validation`] — experiment harness with structured pass/fail/skip output
//! - [`tolerances`] — named bounds for parity, latency, and coordination
//! - [`deploy`] — deploy graph parsing, structural validation, and live probing
//! - [`harness`] — spawn compositions, validate, tear down
//! - [`checksums`] — BLAKE3 manifest generation and verification for guideStone P3 (self-verifying)
//!
//! ## Supporting — IPC, discovery, and ecosystem wiring
//! - [`ipc`] — JSON-RPC 2.0 client, Neural API bridge, 6-tier socket discovery, resilience
//! - [`launcher`] — primal binary discovery, spawn, socket nucleation
//! - [`bonding`] — multi-gate bonding models (Covalent, Metallic, Ionic, Weak, `OrganoMetalSalt`)
//! - [`btsp`] — BTSP wire types and cipher policy
//! - [`graphs`] — coordination pattern types (Sequential, Parallel, DAG, Pipeline, Continuous)
//! - [`emergent`] — emergent system types (`RootPulse`, RPGPT, `CoralForge`)
//! - [`primal_names`] — canonical display names and discovery slug mapping
//! - [`cast`] — safe numeric casts for metrics

/// Arena name — used in logging, provenance, and IPC client identity.
pub const NAME: &str = "primalspring";

pub mod bonding;
pub mod btsp;
pub mod cast;
pub mod certification;
pub mod checksums;
pub mod composition;
pub mod coordination;
pub mod deploy;
pub mod emergent;
pub mod env_keys;
pub mod genetics;
pub mod graphs;
pub mod harness;
pub mod ipc;
pub mod launcher;
pub mod primal_names;
pub mod tolerances;
pub mod validation;
