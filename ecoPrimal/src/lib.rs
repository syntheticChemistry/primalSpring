// SPDX-License-Identifier: AGPL-3.0-or-later

//! primalSpring — coordination and composition validation library.
//!
//! The spring whose domain IS the ecosystem. Where other springs validate
//! domain science (physics, biology, chemistry), primalSpring validates
//! the coordination layer: atomic composition, graph execution, emergent
//! systems, bonding, and cross-spring interaction patterns.
//!
//! # Architecture
//!
//! primalSpring follows sovereign design: it has self-knowledge of coordination
//! patterns and discovers other primals at runtime via the Neural API or
//! direct socket probing. No hardcoded primal rosters.
//!
//! # Modules
//!
//! - [`coordination`] — atomic composition definitions (Tower, Node, Nest, Full NUCLEUS)
//! - [`deploy`] — deploy graph parsing, structural validation, and live probing
//! - [`graphs`] — graph execution pattern validation (Sequential, Parallel, DAG, Pipeline, Continuous)
//! - [`emergent`] — emergent system validation (`RootPulse`, RPGPT, `CoralForge`)
//! - [`bonding`] — multi-gate bonding models (Covalent, Metallic, Ionic, Weak, OrganoMetalSalt)
//! - [`ipc`] — JSON-RPC 2.0 client + Neural API bridge + socket discovery
//! - [`launcher`] — primal binary discovery, spawn, and socket lifecycle (sync port from biomeOS)
//! - [`harness`] — atomic test orchestration: spawn compositions, validate, tear down
//! - [`niche`] — BYOB niche self-knowledge (capabilities, semantic mappings, registration)
//! - [`validation`] — experiment validation harness with structured output
//! - [`tolerances`] — named latency and throughput bounds

/// Canonical primal name — single source of truth for self-knowledge.
pub const PRIMAL_NAME: &str = "primalspring";

/// Capability domain this primal serves.
pub const PRIMAL_DOMAIN: &str = "coordination";

pub mod bonding;
pub mod cast;
pub mod coordination;
pub mod deploy;
pub mod emergent;
pub mod graphs;
pub mod harness;
pub mod ipc;
pub mod launcher;
pub mod niche;
pub mod tolerances;
pub mod validation;
