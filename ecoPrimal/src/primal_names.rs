// SPDX-License-Identifier: AGPL-3.0-or-later

//! Canonical primal identity — typed enum replacing loose string constants.
//!
//! Every primal in the ecosystem has exactly one [`Primal`] variant. The enum
//! provides compile-time exhaustiveness checks, zero-cost slug/display
//! conversion, and [`FromStr`] parsing that catches typos at boundaries
//! rather than silently routing to `other => other`.
//!
//! Absorbed from neuralSpring S170 `primal_names::display` pattern,
//! elevated from `&str` constants to a proper type.

use std::fmt;
use std::str::FromStr;

/// Every known primal in the ecosystem.
///
/// The variants are exhaustive — adding a new primal forces updates to all
/// match arms across the codebase (routing, deployment, niche, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Primal {
    /// Trust & crypto (Tower electron).
    BearDog,
    /// Discovery & NAT traversal (Tower electron).
    Songbird,
    /// Compute dispatch (Node proton).
    ToadStool,
    /// Content storage (Nest neutron).
    NestGate,
    /// AI inference (Meta-tier).
    Squirrel,
    /// Tensor math & GPU compute (Node proton).
    BarraCuda,
    /// Shader compilation (Node proton).
    CoralReef,
    /// Orchestration & federation (Meta-tier).
    BiomeOS,
    /// Visualization & rendering (Meta-tier).
    PetalTongue,
    /// DAG provenance (Nest neutron).
    RhizoCrypt,
    /// Merkle ledger (Nest neutron).
    LoamSpine,
    /// Attribution & commit braids (Nest neutron).
    SweetGrass,
    /// Defense, audit & threat assessment (Tower electron).
    SkunkBat,
}

/// Every known spring in the ecosystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Spring {
    /// Ecosystem coordination & validation.
    PrimalSpring,
    /// Thermal science & compute trio integration.
    HotSpring,
    /// Geology & terrain science.
    GroundSpring,
    /// Neural network & ML science.
    NeuralSpring,
    /// Fluid dynamics & water science.
    WetSpring,
    /// Atmospheric & weather science.
    AirSpring,
    /// Medical & biological science.
    HealthSpring,
    /// Game engine & interactive science.
    LudoSpring,
}

/// NUCLEUS atomic compositions — typed membership.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Atomic {
    /// Electron: BearDog + Songbird + SkunkBat (trust boundary).
    Tower,
    /// Proton: Tower + ToadStool + BarraCuda + CoralReef (compute).
    Node,
    /// Neutron: Tower + NestGate + RhizoCrypt + LoamSpine + SweetGrass (storage + lineage).
    Nest,
    /// Meta-tier: BiomeOS + Squirrel + PetalTongue (orchestration + AI + viz).
    Meta,
}

impl Primal {
    /// Lowercase discovery slug used in socket paths, env vars, and IPC.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::BearDog => "beardog",
            Self::Songbird => "songbird",
            Self::ToadStool => "toadstool",
            Self::NestGate => "nestgate",
            Self::Squirrel => "squirrel",
            Self::BarraCuda => "barracuda",
            Self::CoralReef => "coralreef",
            Self::BiomeOS => "biomeos",
            Self::PetalTongue => "petaltongue",
            Self::RhizoCrypt => "rhizocrypt",
            Self::LoamSpine => "loamspine",
            Self::SweetGrass => "sweetgrass",
            Self::SkunkBat => "skunkbat",
        }
    }

    /// Mixed-case display name for dashboards and handoffs.
    #[must_use]
    pub const fn display(self) -> &'static str {
        match self {
            Self::BearDog => "BearDog",
            Self::Songbird => "Songbird",
            Self::ToadStool => "ToadStool",
            Self::NestGate => "NestGate",
            Self::Squirrel => "Squirrel",
            Self::BarraCuda => "barraCuda",
            Self::CoralReef => "coralReef",
            Self::BiomeOS => "biomeOS",
            Self::PetalTongue => "petalTongue",
            Self::RhizoCrypt => "rhizoCrypt",
            Self::LoamSpine => "LoamSpine",
            Self::SweetGrass => "sweetGrass",
            Self::SkunkBat => "skunkBat",
        }
    }

    /// All primals in canonical order.
    pub const ALL: &'static [Primal] = &[
        Self::BearDog,
        Self::Songbird,
        Self::SkunkBat,
        Self::ToadStool,
        Self::BarraCuda,
        Self::CoralReef,
        Self::NestGate,
        Self::RhizoCrypt,
        Self::LoamSpine,
        Self::SweetGrass,
        Self::BiomeOS,
        Self::Squirrel,
        Self::PetalTongue,
    ];

    /// All primals belonging to a given atomic.
    #[must_use]
    pub fn for_atomic(atomic: Atomic) -> &'static [Primal] {
        match atomic {
            Atomic::Tower => &[Self::BearDog, Self::Songbird, Self::SkunkBat],
            Atomic::Node => &[
                Self::BearDog,
                Self::Songbird,
                Self::SkunkBat,
                Self::ToadStool,
                Self::BarraCuda,
                Self::CoralReef,
            ],
            Atomic::Nest => &[
                Self::BearDog,
                Self::Songbird,
                Self::SkunkBat,
                Self::NestGate,
                Self::RhizoCrypt,
                Self::LoamSpine,
                Self::SweetGrass,
            ],
            Atomic::Meta => &[Self::BiomeOS, Self::Squirrel, Self::PetalTongue],
        }
    }

    /// The NUCLEUS atom — all 10 foundation primals (Tower + Node + Nest, deduplicated).
    pub const NUCLEUS: &'static [Primal] = &[
        Self::BearDog,
        Self::Songbird,
        Self::SkunkBat,
        Self::ToadStool,
        Self::BarraCuda,
        Self::CoralReef,
        Self::NestGate,
        Self::RhizoCrypt,
        Self::LoamSpine,
        Self::SweetGrass,
    ];
}

impl fmt::Display for Primal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.display())
    }
}

impl FromStr for Primal {
    type Err = UnknownPrimal;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "beardog" | "BearDog" => Ok(Self::BearDog),
            "songbird" | "Songbird" => Ok(Self::Songbird),
            "toadstool" | "ToadStool" => Ok(Self::ToadStool),
            "nestgate" | "NestGate" => Ok(Self::NestGate),
            "squirrel" | "Squirrel" => Ok(Self::Squirrel),
            "barracuda" | "barraCuda" => Ok(Self::BarraCuda),
            "coralreef" | "coralReef" => Ok(Self::CoralReef),
            "biomeos" | "biomeOS" => Ok(Self::BiomeOS),
            "petaltongue" | "petalTongue" => Ok(Self::PetalTongue),
            "rhizocrypt" | "rhizoCrypt" => Ok(Self::RhizoCrypt),
            "loamspine" | "LoamSpine" => Ok(Self::LoamSpine),
            "sweetgrass" | "sweetGrass" => Ok(Self::SweetGrass),
            "skunkbat" | "skunkBat" => Ok(Self::SkunkBat),
            _ => Err(UnknownPrimal(s.to_owned())),
        }
    }
}

/// Error returned when a string doesn't match any known primal.
#[derive(Debug, Clone)]
pub struct UnknownPrimal(pub String);

impl fmt::Display for UnknownPrimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown primal: '{}'", self.0)
    }
}

impl std::error::Error for UnknownPrimal {}

impl Spring {
    /// Lowercase discovery slug.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::PrimalSpring => "primalspring",
            Self::HotSpring => "hotspring",
            Self::GroundSpring => "groundspring",
            Self::NeuralSpring => "neuralspring",
            Self::WetSpring => "wetspring",
            Self::AirSpring => "airspring",
            Self::HealthSpring => "healthspring",
            Self::LudoSpring => "ludospring",
        }
    }

    /// Mixed-case display name.
    #[must_use]
    pub const fn display(self) -> &'static str {
        match self {
            Self::PrimalSpring => "primalSpring",
            Self::HotSpring => "hotSpring",
            Self::GroundSpring => "groundSpring",
            Self::NeuralSpring => "neuralSpring",
            Self::WetSpring => "wetSpring",
            Self::AirSpring => "airSpring",
            Self::HealthSpring => "healthSpring",
            Self::LudoSpring => "ludoSpring",
        }
    }
}

impl fmt::Display for Spring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.display())
    }
}

impl FromStr for Spring {
    type Err = UnknownPrimal;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "primalspring" | "primalSpring" => Ok(Self::PrimalSpring),
            "hotspring" | "hotSpring" => Ok(Self::HotSpring),
            "groundspring" | "groundSpring" => Ok(Self::GroundSpring),
            "neuralspring" | "neuralSpring" => Ok(Self::NeuralSpring),
            "wetspring" | "wetSpring" => Ok(Self::WetSpring),
            "airspring" | "airSpring" => Ok(Self::AirSpring),
            "healthspring" | "healthSpring" => Ok(Self::HealthSpring),
            "ludospring" | "ludoSpring" => Ok(Self::LudoSpring),
            _ => Err(UnknownPrimal(s.to_owned())),
        }
    }
}

// ── Backward compatibility ─────────────────────────────────────────
// Legacy `&str` constants and free functions. These allow incremental
// migration: existing code continues to compile while callers are
// upgraded module-by-module to use the `Primal` enum directly.

/// Legacy slug constant — prefer `Primal::BearDog.slug()`.
pub const BEARDOG: &str = "beardog";
/// Legacy slug constant — prefer `Primal::Songbird.slug()`.
pub const SONGBIRD: &str = "songbird";
/// Legacy slug constant — prefer `Primal::ToadStool.slug()`.
pub const TOADSTOOL: &str = "toadstool";
/// Legacy slug constant — prefer `Primal::NestGate.slug()`.
pub const NESTGATE: &str = "nestgate";
/// Legacy slug constant — prefer `Primal::Squirrel.slug()`.
pub const SQUIRREL: &str = "squirrel";
/// Legacy slug constant — prefer `Primal::RhizoCrypt.slug()`.
pub const RHIZOCRYPT: &str = "rhizocrypt";
/// Legacy slug constant — prefer `Primal::LoamSpine.slug()`.
pub const LOAMSPINE: &str = "loamspine";
/// Legacy slug constant — prefer `Primal::SweetGrass.slug()`.
pub const SWEETGRASS: &str = "sweetgrass";
/// Legacy slug constant — prefer `Primal::PetalTongue.slug()`.
pub const PETALTONGUE: &str = "petaltongue";
/// Legacy slug constant — prefer `Primal::BiomeOS.slug()`.
pub const BIOMEOS: &str = "biomeos";
/// Legacy slug constant — prefer `Primal::BarraCuda.slug()`.
pub const BARRACUDA: &str = "barracuda";
/// Legacy slug constant — prefer `Primal::CoralReef.slug()`.
pub const CORALREEF: &str = "coralreef";
/// Legacy slug constant — prefer `Primal::SkunkBat.slug()`.
pub const SKUNKBAT: &str = "skunkbat";

/// Lowercase discovery slug → mixed-case display name (legacy wrapper).
#[must_use]
pub fn display_name(slug: &str) -> &str {
    match slug.parse::<Primal>() {
        Ok(p) => p.display(),
        Err(_) => match slug.parse::<Spring>() {
            Ok(s) => s.display(),
            Err(_) => slug,
        },
    }
}

/// Mixed-case display name → lowercase discovery slug (legacy wrapper).
#[must_use]
pub fn discovery_slug(display: &str) -> &str {
    match display.parse::<Primal>() {
        Ok(p) => p.slug(),
        Err(_) => match display.parse::<Spring>() {
            Ok(s) => s.slug(),
            Err(_) => display,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_primals_have_display_names() {
        assert_eq!(display_name("beardog"), "BearDog");
        assert_eq!(display_name("biomeos"), "biomeOS");
        assert_eq!(display_name("barracuda"), "barraCuda");
        assert_eq!(display_name("primalspring"), "primalSpring");
    }

    #[test]
    fn unknown_slug_passes_through() {
        assert_eq!(display_name("unknown_primal"), "unknown_primal");
    }

    #[test]
    fn display_to_slug_round_trips() {
        for p in Primal::ALL {
            let display = p.display();
            let back = discovery_slug(display);
            assert_eq!(
                back,
                p.slug(),
                "round-trip failed for {:?}: {} -> {} -> {}",
                p,
                p.slug(),
                display,
                back,
            );
        }
    }

    #[test]
    fn unknown_display_passes_through() {
        assert_eq!(discovery_slug("UnknownPrimal"), "UnknownPrimal");
    }

    #[test]
    fn spring_names_are_canonical() {
        assert_eq!(display_name("hotspring"), "hotSpring");
        assert_eq!(display_name("wetspring"), "wetSpring");
        assert_eq!(display_name("airspring"), "airSpring");
        assert_eq!(display_name("healthspring"), "healthSpring");
        assert_eq!(display_name("ludospring"), "ludoSpring");
        assert_eq!(display_name("neuralspring"), "neuralSpring");
        assert_eq!(display_name("groundspring"), "groundSpring");
    }

    #[test]
    fn enum_parse_from_slug() {
        assert_eq!("beardog".parse::<Primal>().unwrap(), Primal::BearDog);
        assert_eq!("coralreef".parse::<Primal>().unwrap(), Primal::CoralReef);
        assert_eq!("skunkbat".parse::<Primal>().unwrap(), Primal::SkunkBat);
    }

    #[test]
    fn enum_parse_from_display() {
        assert_eq!("BearDog".parse::<Primal>().unwrap(), Primal::BearDog);
        assert_eq!("barraCuda".parse::<Primal>().unwrap(), Primal::BarraCuda);
        assert_eq!("biomeOS".parse::<Primal>().unwrap(), Primal::BiomeOS);
    }

    #[test]
    fn enum_parse_rejects_unknown() {
        assert!("unknown".parse::<Primal>().is_err());
        assert!("BEARDOG".parse::<Primal>().is_err());
    }

    #[test]
    fn all_primals_count() {
        assert_eq!(Primal::ALL.len(), 13);
    }

    #[test]
    fn nucleus_count() {
        assert_eq!(Primal::NUCLEUS.len(), 10);
    }

    #[test]
    fn tower_atomic_membership() {
        let tower = Primal::for_atomic(Atomic::Tower);
        assert_eq!(tower.len(), 3);
        assert!(tower.contains(&Primal::BearDog));
        assert!(tower.contains(&Primal::Songbird));
        assert!(tower.contains(&Primal::SkunkBat));
    }

    #[test]
    fn node_includes_tower() {
        let node = Primal::for_atomic(Atomic::Node);
        for tp in Primal::for_atomic(Atomic::Tower) {
            assert!(node.contains(tp), "Node should include Tower primal {tp}");
        }
    }

    #[test]
    fn display_format_matches_display_method() {
        for p in Primal::ALL {
            assert_eq!(format!("{p}"), p.display());
        }
    }

    #[test]
    fn slug_constants_match_enum() {
        assert_eq!(BEARDOG, Primal::BearDog.slug());
        assert_eq!(SONGBIRD, Primal::Songbird.slug());
        assert_eq!(TOADSTOOL, Primal::ToadStool.slug());
        assert_eq!(NESTGATE, Primal::NestGate.slug());
        assert_eq!(SQUIRREL, Primal::Squirrel.slug());
        assert_eq!(BARRACUDA, Primal::BarraCuda.slug());
        assert_eq!(CORALREEF, Primal::CoralReef.slug());
        assert_eq!(BIOMEOS, Primal::BiomeOS.slug());
        assert_eq!(PETALTONGUE, Primal::PetalTongue.slug());
        assert_eq!(RHIZOCRYPT, Primal::RhizoCrypt.slug());
        assert_eq!(LOAMSPINE, Primal::LoamSpine.slug());
        assert_eq!(SWEETGRASS, Primal::SweetGrass.slug());
        assert_eq!(SKUNKBAT, Primal::SkunkBat.slug());
    }
}
