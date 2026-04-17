// SPDX-License-Identifier: AGPL-3.0-or-later

//! Emergent system validation — Layer 3 systems that arise from graph
//! execution over primals.
//!
//! `RootPulse` (version control), RPGPT (game engine), coralForge (structure
//! prediction), and cross-spring ecology pipelines are all emergent systems
//! that biomeOS composes from primal capabilities. primalSpring validates
//! that they emerge correctly.

use serde::{Deserialize, Serialize};

/// Layer 3 emergent systems that arise from graph execution over primals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmergentSystem {
    /// Distributed VCS from provenance trio coordination.
    RootPulse,
    /// Sovereign RPG engine at 60 Hz with provenance.
    Rpgpt,
    /// Structure prediction pipeline (neural object).
    CoralForge,
    /// Ecology pipeline: airSpring → wetSpring → neuralSpring.
    CrossSpringEcology,
}

impl EmergentSystem {
    /// Human-readable description of this emergent system.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::RootPulse => "Distributed VCS from provenance trio coordination",
            Self::Rpgpt => "Sovereign RPG engine at 60 Hz + provenance",
            Self::CoralForge => "Structure prediction pipeline (neural object)",
            Self::CrossSpringEcology => "Ecology pipeline: airSpring -> wetSpring -> neuralSpring",
        }
    }

    /// biomeOS graph names this system requires for execution.
    #[must_use]
    pub const fn required_graphs(self) -> &'static [&'static str] {
        match self {
            Self::RootPulse => &[
                "rootpulse_commit",
                "rootpulse_branch",
                "rootpulse_merge",
                "rootpulse_diff",
                "rootpulse_federate",
            ],
            Self::Rpgpt => &["game_engine_tick"],
            Self::CoralForge => &["coralforge_pipeline"],
            Self::CrossSpringEcology => &["cross_spring_ecology"],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_systems_have_descriptions() {
        let systems = [
            EmergentSystem::RootPulse,
            EmergentSystem::Rpgpt,
            EmergentSystem::CoralForge,
            EmergentSystem::CrossSpringEcology,
        ];
        for s in systems {
            assert!(!s.description().is_empty());
        }
    }

    #[test]
    fn all_systems_have_required_graphs() {
        let systems = [
            EmergentSystem::RootPulse,
            EmergentSystem::Rpgpt,
            EmergentSystem::CoralForge,
            EmergentSystem::CrossSpringEcology,
        ];
        for s in systems {
            assert!(!s.required_graphs().is_empty());
        }
    }

    #[test]
    fn rootpulse_requires_five_graphs() {
        assert_eq!(EmergentSystem::RootPulse.required_graphs().len(), 5);
    }

    #[test]
    fn system_round_trip_json() {
        for s in [
            EmergentSystem::RootPulse,
            EmergentSystem::Rpgpt,
            EmergentSystem::CoralForge,
            EmergentSystem::CrossSpringEcology,
        ] {
            let json = serde_json::to_string(&s).unwrap();
            let back: EmergentSystem = serde_json::from_str(&json).unwrap();
            assert_eq!(s, back);
        }
    }
}
