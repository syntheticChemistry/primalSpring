// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! primalSpring — the ecosystem intermediary.
//!
//! primalSpring starts and validates all shared NUCLEUS compositions (atomics,
//! bonding, chimeras). It is the bridge between upstream primals and downstream
//! springs: primals expose capabilities, primalSpring proves they compose
//! correctly, springs validate their domain science through those compositions.
//!
//! # Core Role
//!
//! - **Start** NUCLEUS compositions (Tower, Node, Nest, FullNucleus) via biomeOS
//! - **Validate** that primal compositions produce correct results (parity with baselines)
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
//!
//! ## Supporting — IPC, discovery, and ecosystem wiring
//! - [`ipc`] — JSON-RPC 2.0 client, Neural API bridge, 6-tier socket discovery, resilience
//! - [`launcher`] — primal binary discovery, spawn, socket nucleation
//! - [`bonding`] — multi-gate bonding models (Covalent, Metallic, Ionic, Weak, `OrganoMetalSalt`)
//! - [`btsp`] — BTSP wire types and cipher policy
//! - [`graphs`] — coordination pattern types (Sequential, Parallel, DAG, Pipeline, Continuous)
//! - [`emergent`] — emergent system types (`RootPulse`, RPGPT, `CoralForge`)
//! - [`niche`] — BYOB capability registration for the primalSpring server
//! - [`primal_names`] — canonical display names and discovery slug mapping
//! - [`cast`] — safe numeric casts for metrics

/// Canonical primal name — single source of truth for self-knowledge.
pub const PRIMAL_NAME: &str = "primalspring";

/// Capability domain this primal serves.
pub const PRIMAL_DOMAIN: &str = "coordination";

pub mod bonding;
pub mod btsp;
pub mod cast;
pub mod composition;
pub mod coordination;
pub mod deploy;
pub mod emergent;
pub mod graphs;
pub mod harness;
pub mod ipc;
pub mod launcher;
pub mod niche;
pub mod primal_names;
pub mod tolerances;
pub mod validation;
