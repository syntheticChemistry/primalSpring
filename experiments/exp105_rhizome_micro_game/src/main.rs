// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]

//! exp105 — The Rhizome Micro-Game
//!
//! Validates a full roguelike game loop running on the NUCLEUS substrate:
//! world gen (Barracuda + ludoSpring) → render (petalTongue) → save/load
//! (NestGate + provenance trio) → narration (Squirrel) → flow tracking
//!
//! Phase 56 — Desktop Substrate (RHIZOME_MICRO_GAME.md)

mod phases;
mod types;
mod world;

use phases::{
    phase_crypto_hash, phase_discovery, phase_game_loop, phase_load_game, phase_narration,
    phase_save_game,
};
use primalspring::composition::CompositionContext;
use primalspring::validation::ValidationResult;
use world::{phase_render_scene, phase_world_gen};

fn main() {
    ValidationResult::new("primalSpring Exp105 — The Rhizome Micro-Game")
        .with_provenance("exp105_rhizome_micro_game", "2026-05-09")
        .run("Exp105: Full roguelike game loop on Desktop NUCLEUS", |v| {
            let mut ctx = CompositionContext::discover();

            v.section("Phase 1: World / Scene Setup");
            let mut world = phase_world_gen(v, &mut ctx);
            phase_render_scene(v, &world, &mut ctx);

            v.section("Phase 2: Game Loop");
            phase_game_loop(v, &mut world, &mut ctx);
            phase_render_scene(v, &world, &mut ctx);

            v.section("Phase 3: Save / Provenance");
            let dag_session = phase_save_game(v, &world, &mut ctx);
            phase_load_game(v, dag_session.as_deref(), &mut ctx);

            v.section("Phase 4: Narration / Integrity / Discovery");
            phase_narration(v, &world, &mut ctx);
            phase_crypto_hash(v, &world, &mut ctx);
            phase_discovery(v, &mut ctx);
        });
}
