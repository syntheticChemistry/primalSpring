// SPDX-License-Identifier: AGPL-3.0-or-later

//! Graph execution validation — all 5 coordination patterns.
//!
//! primalSpring validates Sequential, Parallel, `ConditionalDag`, Pipeline,
//! and Continuous graph execution with real primals (not mocks).

use serde::{Deserialize, Serialize};

/// biomeOS graph execution pattern for coordinating primals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoordinationPattern {
    /// Nodes executed in dependency order (A → B → C).
    Sequential,
    /// Independent nodes run concurrently.
    Parallel,
    /// DAG with `condition/skip_if` branching.
    ConditionalDag,
    /// Streaming via bounded mpsc channels.
    Pipeline,
    /// Fixed-timestep tick loop (e.g. 60 Hz).
    Continuous,
}

impl CoordinationPattern {
    /// Human-readable description of this execution pattern.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::Sequential => "Nodes in dependency order (A -> B -> C)",
            Self::Parallel => "Independent nodes concurrently",
            Self::ConditionalDag => "DAG with condition/skip_if branching",
            Self::Pipeline => "Streaming via bounded mpsc channels",
            Self::Continuous => "Fixed-timestep tick loop (e.g. 60 Hz)",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_patterns_have_descriptions() {
        let patterns = [
            CoordinationPattern::Sequential,
            CoordinationPattern::Parallel,
            CoordinationPattern::ConditionalDag,
            CoordinationPattern::Pipeline,
            CoordinationPattern::Continuous,
        ];
        for p in patterns {
            assert!(!p.description().is_empty());
        }
    }

    #[test]
    fn pattern_round_trip_json() {
        for p in [
            CoordinationPattern::Sequential,
            CoordinationPattern::Parallel,
            CoordinationPattern::ConditionalDag,
            CoordinationPattern::Pipeline,
            CoordinationPattern::Continuous,
        ] {
            let json = serde_json::to_string(&p).unwrap();
            let back: CoordinationPattern = serde_json::from_str(&json).unwrap();
            assert_eq!(p, back);
        }
    }
}
