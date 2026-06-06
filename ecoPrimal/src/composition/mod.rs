// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Composition parity validation — the bridge between coordination and math.
//!
//! Springs validate domain science through a pipeline:
//!
//! 1. **Python baseline** — peer-reviewed, reproducible
//! 2. **Rust port** — matches Python within documented tolerance
//! 3. **Primal composition** — matches Rust via IPC
//!
//! This module provides [`CompositionContext`] as a single entry point for
//! step 3. Springs call math through the composition layer and compare
//! results against their local Rust baselines without understanding primal
//! internals, socket paths, or JSON-RPC response schemas.
//!
//! # Example
//!
//! ```rust,no_run
//! use primalspring::composition::CompositionContext;
//! use primalspring::validation::ValidationResult;
//! use primalspring::tolerances;
//!
//! let mut ctx = CompositionContext::discover();
//! let mut v = ValidationResult::new("hotSpring Composition Parity");
//!
//! // stats.mean: param key is "data", result key is "result"
//! primalspring::composition::validate_parity(
//!     &mut ctx, &mut v,
//!     "sample_mean",
//!     "tensor",           // capability — resolves to barraCuda
//!     "stats.mean",
//!     serde_json::json!({"data": [1.0, 2.0, 3.0, 4.0, 5.0]}),
//!     "result",
//!     3.0,
//!     tolerances::CPU_GPU_PARITY_TOL,
//! );
//! ```

mod btsp;
mod context;
mod context_discovery;
mod context_helpers;
pub mod mesh;
pub mod neural_dispatch;
pub mod neural_routing;
mod parity;
mod routing;

pub use context::{CompositionContext, DiscoveryPath};
pub use parity::{
    call_or_skip, is_skip_error, validate_liveness, validate_parity, validate_parity_flex,
    validate_parity_vec, validate_parity_vec_flex,
};
pub use btsp::tcp_fallback_table;
pub use routing::{
    ALL_CAPS, BTSP_EXTRA_CAPS, CapabilityDomain, capability_to_primal,
    capability_to_primal_typed, method_to_capability_domain,
};

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
