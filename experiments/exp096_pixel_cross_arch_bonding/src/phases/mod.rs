// SPDX-License-Identifier: AGPL-3.0-or-later

//! Pixel cross-architecture bonding validation phases.
//!
//! Split into per-concern submodules to keep each under the 800-line threshold.

mod beacon_bonding_stun;
mod btsp_hsm;
mod tower_genetics;

pub use beacon_bonding_stun::{
    validate_beacon_exchange, validate_bonding_model, validate_stun_nat,
};
pub use btsp_hsm::{validate_btsp_phase3_readiness, validate_hsm_capabilities};
pub use tower_genetics::{validate_cross_arch_genetics, validate_pixel_tower_health};
