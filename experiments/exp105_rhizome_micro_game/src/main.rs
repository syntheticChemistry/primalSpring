// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]
#![allow(
    clippy::cast_precision_loss,
    clippy::option_if_let_else,
    clippy::or_fun_call,
    clippy::cast_sign_loss
)]

//! exp105 — The Rhizome Micro-Game
//!
//! Validates a full roguelike game loop running on the NUCLEUS substrate:
//! world gen (Barracuda + ludoSpring) → render (petalTongue) → save/load
//! (NestGate + provenance trio) → narration (Squirrel) → flow tracking
//!
//! Phase 56 — Desktop Substrate (RHIZOME_MICRO_GAME.md)

mod types;
mod world;
mod phases;

use phases::{
    phase_crypto_hash, phase_discovery, phase_game_loop, phase_load_game, phase_narration,
    phase_save_game,
};
use primalspring::validation::ValidationResult;
use world::{phase_render_scene, phase_world_gen};

fn main() {
    ValidationResult::new("primalSpring Exp105 — The Rhizome Micro-Game")
        .with_provenance("exp105_rhizome_micro_game", "2026-04-28")
        .run("Exp105: Full roguelike game loop on Desktop NUCLEUS", |v| {
            let mut world = phase_world_gen(v);
            phase_render_scene(v, &world);
            phase_game_loop(v, &mut world);
            phase_render_scene(v, &world);
            let dag_session = phase_save_game(v, &world);
            phase_load_game(v, dag_session.as_deref());
            phase_narration(v, &world);
            phase_crypto_hash(v, &world);
            phase_discovery(v);
        });
}
