// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scenario registry — metadata, filtering, and execution.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;

/// Validation tier for scenario filtering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tier {
    /// Tier 1: Pure Rust structural validation — no IPC needed.
    Rust,
    /// Tier 2: Live NUCLEUS validation — requires deployed primals.
    Live,
    /// Both tiers: has structural and live phases.
    Both,
}

impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rust => write!(f, "rust"),
            Self::Live => write!(f, "live"),
            Self::Both => write!(f, "both"),
        }
    }
}

impl Tier {
    /// Parse a tier from a loose string (accepts aliases).
    #[must_use]
    pub fn from_str_loose(s: &str) -> Option<Self> {
        match s {
            "rust" | "structural" | "tier1" => Some(Self::Rust),
            "live" | "ipc" | "tier2" => Some(Self::Live),
            "both" | "all" => Some(Self::Both),
            _ => None,
        }
    }
}

/// Track taxonomy — groups related scenarios.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Track {
    /// Tower, Node, Nest, and Full NUCLEUS compositions.
    AtomicComposition,
    /// Graph execution: sequential, parallel, conditional DAG.
    GraphExecution,
    /// Cross-spring data flow and ecology.
    CrossSpring,
    /// Bonding model: covalent, ionic, metallic, weak.
    Bonding,
    /// IPC transport: sockets, TCP, protocol escalation.
    Transport,
    /// Security: bearer tokens, BTSP, method gate.
    Security,
    /// biomeOS deployment and Neural API.
    BiomeosDeploy,
    /// Infrastructure: deployment matrix, cellular graphs.
    Infrastructure,
    /// Composition lifecycle: reload, federation, parity.
    Lifecycle,
    /// Sovereignty: membrane composition, content routing, parity protocol.
    Sovereignty,
}

impl std::fmt::Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AtomicComposition => write!(f, "atomic-composition"),
            Self::GraphExecution => write!(f, "graph-execution"),
            Self::CrossSpring => write!(f, "cross-spring"),
            Self::Bonding => write!(f, "bonding"),
            Self::Transport => write!(f, "transport"),
            Self::Security => write!(f, "security"),
            Self::BiomeosDeploy => write!(f, "biomeos-deploy"),
            Self::Infrastructure => write!(f, "infrastructure"),
            Self::Lifecycle => write!(f, "lifecycle"),
            Self::Sovereignty => write!(f, "sovereignty"),
        }
    }
}

impl Track {
    /// Parse a track name from a string.
    #[must_use]
    pub fn from_str_loose(s: &str) -> Option<Self> {
        match s {
            "atomic-composition" | "atomic" => Some(Self::AtomicComposition),
            "graph-execution" | "graph" => Some(Self::GraphExecution),
            "cross-spring" | "cross" => Some(Self::CrossSpring),
            "bonding" => Some(Self::Bonding),
            "transport" => Some(Self::Transport),
            "security" => Some(Self::Security),
            "biomeos-deploy" | "biomeos" => Some(Self::BiomeosDeploy),
            "infrastructure" | "infra" => Some(Self::Infrastructure),
            "lifecycle" => Some(Self::Lifecycle),
            "sovereignty" | "sovereign" => Some(Self::Sovereignty),
            _ => None,
        }
    }
}

/// Scenario metadata — provenance, classification, and description.
#[derive(Debug, Clone)]
pub struct ScenarioMeta {
    /// Unique scenario identifier (e.g. `"tower-atomic"`).
    pub id: &'static str,
    /// Which track this scenario belongs to.
    pub track: Track,
    /// Which validation tier this scenario exercises.
    pub tier: Tier,
    /// Original experiment crate name for provenance (e.g. `"exp001_tower_atomic"`).
    pub provenance_crate: &'static str,
    /// Date of last significant update.
    pub provenance_date: &'static str,
    /// One-line description.
    pub description: &'static str,
}

/// A callable scenario: metadata + run function.
pub struct Scenario {
    /// Scenario metadata.
    pub meta: ScenarioMeta,
    /// The validation function.
    pub run: fn(&mut ValidationResult, &mut CompositionContext),
}

impl std::fmt::Debug for Scenario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scenario")
            .field("id", &self.meta.id)
            .field("track", &self.meta.track)
            .field("tier", &self.meta.tier)
            .finish_non_exhaustive()
    }
}

/// Registry of all absorbed validation scenarios.
pub struct ScenarioRegistry {
    scenarios: Vec<Scenario>,
}

impl ScenarioRegistry {
    /// Create an empty registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            scenarios: Vec::new(),
        }
    }

    /// Register a scenario.
    pub fn register(&mut self, scenario: Scenario) {
        self.scenarios.push(scenario);
    }

    /// All registered scenarios.
    #[must_use]
    pub fn all(&self) -> &[Scenario] {
        &self.scenarios
    }

    /// Filter scenarios by tier.
    pub fn filter_by_tier(&self, tier: Tier) -> impl Iterator<Item = &Scenario> {
        self.scenarios
            .iter()
            .filter(move |s| s.meta.tier == tier || s.meta.tier == Tier::Both || tier == Tier::Both)
    }

    /// Filter scenarios by track.
    pub fn filter_by_track(&self, track: Track) -> impl Iterator<Item = &Scenario> {
        self.scenarios.iter().filter(move |s| s.meta.track == track)
    }

    /// Find a scenario by ID.
    #[must_use]
    pub fn find(&self, id: &str) -> Option<&Scenario> {
        self.scenarios.iter().find(|s| s.meta.id == id)
    }

    /// Total number of registered scenarios.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.scenarios.len()
    }

    /// Whether the registry is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.scenarios.is_empty()
    }
}

impl Default for ScenarioRegistry {
    fn default() -> Self {
        Self::new()
    }
}
