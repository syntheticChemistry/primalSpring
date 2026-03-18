// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp001: Tower Atomic — BearDog + Songbird validation.
//!
//! Validates the minimal NUCLEUS composition: BearDog (crypto) and Songbird (mesh).
//! Discovers sockets at runtime, connects via JSON-RPC, and validates health +
//! capabilities. Gracefully skips checks when primals are not running.

use primalspring::coordination::{AtomicType, check_primal_health, validate_composition};
use primalspring::ipc::discover::{discover_primal, neural_api_healthy};
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::run_experiment(
        "primalSpring Exp001 — Tower Atomic",
        "primalSpring Exp001: Tower Atomic (BearDog + Songbird)",
        |v| {
            let tower_primals = AtomicType::Tower.required_primals();
            v.check_count(
                "tower_required_count",
                tower_primals.len(),
                AtomicType::Tower.required_primals().len(),
            );

            let beardog = discover_primal("beardog");
            v.check_or_skip(
                "socket_beardog",
                beardog.socket.as_ref(),
                "beardog socket not found — primal not running",
                |path, v| {
                    v.check_bool(
                        "socket_beardog",
                        true,
                        &format!("beardog socket found at {}", path.display()),
                    );
                },
            );

            let songbird = discover_primal("songbird");
            v.check_or_skip(
                "socket_songbird",
                songbird.socket.as_ref(),
                "songbird socket not found — primal not running",
                |path, v| {
                    v.check_bool(
                        "socket_songbird",
                        true,
                        &format!("songbird socket found at {}", path.display()),
                    );
                },
            );

            check_primal_health(v, "beardog");
            check_primal_health(v, "songbird");

            let neural_ok = neural_api_healthy();
            if neural_ok {
                v.check_bool("neural_api", true, "Neural API reachable");
                let comp = validate_composition(AtomicType::Tower);
                v.check_bool(
                    "composition_healthy",
                    comp.all_healthy,
                    "Tower composition all healthy",
                );
                v.check_bool(
                    "composition_discovery",
                    comp.discovery_ok,
                    "Tower discovery complete",
                );
                v.check_minimum("composition_caps", comp.total_capabilities, 2);
            } else {
                v.check_skip(
                    "neural_api",
                    "Neural API not reachable — biomeOS not running",
                );
                v.check_skip("composition_healthy", "requires Neural API");
                v.check_skip("composition_discovery", "requires Neural API");
                v.check_skip("composition_caps", "requires Neural API");
            }
        },
    );
}
