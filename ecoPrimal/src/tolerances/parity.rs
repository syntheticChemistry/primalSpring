// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Composition parity tolerances for cross-implementation validation.
//!
//! Used by springs to validate that primal composition output matches
//! local Rust math (which was already validated against Python baselines).
//!
//! Ordering: EXACT < `DETERMINISTIC_FLOAT` < `DF64_PARITY` < `CPU_GPU_PARITY`
//!           <= `IPC_ROUND_TRIP` < `WGSL_SHADER` <= `STOCHASTIC_SEED`

/// Exact integer parity — zero tolerance.
///
/// For deterministic integer math where composition must produce bit-identical
/// results (e.g. hash computations, index calculations).
pub const EXACT_PARITY_TOL: f64 = 0.0;

/// Deterministic floating-point parity.
///
/// For pure CPU f64 operations where the only divergence is instruction
/// ordering. IEEE 754 guarantees identical results for identical operation
/// sequences; this covers minor reordering.
pub const DETERMINISTIC_FLOAT_TOL: f64 = 1e-15;

/// Double-float (df64) emulated precision parity.
///
/// barraCuda's df64 path uses Dekker/Knuth error-free transforms to achieve
/// ~30 digits of precision. Tolerance covers the ~1 ULP error inherent in
/// the emulation scheme.
pub const DF64_PARITY_TOL: f64 = 1e-14;

/// CPU vs GPU floating-point parity.
///
/// GPU shader execution may reorder FMA, use different rounding modes,
/// or flush denormals. This tolerance covers the expected divergence
/// between a CPU f64 path and a GPU WGSL f32→f64 promotion path.
pub const CPU_GPU_PARITY_TOL: f64 = 1e-10;

/// IPC round-trip serialization parity.
///
/// JSON serialization of f64 values may lose the least significant bits
/// depending on the serializer's precision setting. `serde_json` uses
/// sufficient precision for f64 round-trips, but this tolerance covers
/// edge cases in intermediate Value representations.
pub const IPC_ROUND_TRIP_TOL: f64 = 1e-10;

/// WGSL shader computation parity.
///
/// WGSL shaders execute in f32 by default. When comparing f32 shader output
/// against f64 Rust baselines, this tolerance covers the expected precision
/// loss from the narrower mantissa (23 vs 52 bits).
pub const WGSL_SHADER_TOL: f64 = 1e-6;

/// Stochastic algorithm parity (seed-fixed).
///
/// For algorithms with pseudorandom components (Monte Carlo, HMC, etc.)
/// where the seed is fixed. Different implementations of the same PRNG
/// may diverge after many iterations due to floating-point accumulation.
pub const STOCHASTIC_SEED_TOL: f64 = 1e-6;
