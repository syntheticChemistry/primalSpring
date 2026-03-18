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
//! - [`graphs`] — graph execution pattern validation (Sequential, Parallel, DAG, Pipeline, Continuous)
//! - [`emergent`] — emergent system validation (`RootPulse`, RPGPT, `CoralForge`)
//! - [`bonding`] — multi-gate bonding models (Covalent, Ionic, Weak)
//! - [`ipc`] — JSON-RPC 2.0 client + Neural API bridge + socket discovery
//! - [`validation`] — experiment validation harness with structured output
//! - [`tolerances`] — named latency and throughput bounds

pub mod bonding;
pub mod coordination;
pub mod emergent;
pub mod graphs;
pub mod ipc;
pub mod tolerances;
pub mod validation;
