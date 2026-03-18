// SPDX-License-Identifier: AGPL-3.0-or-later

//! Safe numeric casts for metrics and graph execution.
//!
//! Absorbed from airSpring/healthSpring/groundSpring. Avoids `as` casts
//! that silently truncate or wrap. primalSpring currently only needs
//! `u128_to_u64` for `Instant::elapsed().as_micros()` results; the other
//! helpers are included for graph execution metrics in future phases.

/// Saturating cast from `u128` to `u64`.
#[must_use]
pub const fn u128_to_u64(v: u128) -> u64 {
    if v > u64::MAX as u128 {
        u64::MAX
    } else {
        v as u64
    }
}

/// Convert `Duration::as_micros()` (`u128`) to `u64`, saturating on overflow.
#[must_use]
pub const fn micros_u64(d: std::time::Duration) -> u64 {
    u128_to_u64(d.as_micros())
}

/// Saturating cast from `usize` to `u32`.
#[must_use]
pub const fn usize_to_u32(v: usize) -> u32 {
    if v > u32::MAX as usize {
        u32::MAX
    } else {
        v as u32
    }
}

/// Saturating cast from `usize` to `u64`.
#[must_use]
pub const fn usize_to_u64(v: usize) -> u64 {
    v as u64
}

/// Saturating cast from `f64` to `usize` (clamps negatives to 0, NaN to 0).
#[must_use]
#[expect(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    reason = "this function IS the safe cast boundary"
)]
pub fn f64_to_usize(v: f64) -> usize {
    if v.is_nan() || v < 0.0 {
        0
    } else if v > usize::MAX as f64 {
        usize::MAX
    } else {
        v as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u128_to_u64_within_range() {
        assert_eq!(u128_to_u64(42), 42);
        assert_eq!(u128_to_u64(u128::from(u64::MAX)), u64::MAX);
    }

    #[test]
    fn u128_to_u64_saturates() {
        assert_eq!(u128_to_u64(u128::MAX), u64::MAX);
    }

    #[test]
    fn usize_to_u32_within_range() {
        assert_eq!(usize_to_u32(100), 100);
    }

    #[test]
    fn usize_to_u64_identity() {
        assert_eq!(usize_to_u64(999), 999);
    }

    #[test]
    fn f64_to_usize_normal() {
        assert_eq!(f64_to_usize(42.7), 42);
    }

    #[test]
    fn f64_to_usize_negative_clamps() {
        assert_eq!(f64_to_usize(-5.0), 0);
    }

    #[test]
    fn f64_to_usize_nan_clamps() {
        assert_eq!(f64_to_usize(f64::NAN), 0);
    }
}
