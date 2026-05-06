// SPDX-License-Identifier: AGPL-3.0-or-later

//! Canonical primal names and display formatting.
//!
//! Absorbed from neuralSpring S170 `primal_names::display` pattern.
//! Discovery uses lowercase slugs (`beardog`, `songbird`); handoffs and
//! dashboards need mixed-case display names (`BearDog`, `Songbird`).
//!
//! Single source of truth — no duplicated name tables across modules.

/// Discovery slug for `BearDog`.
pub const BEARDOG: &str = "beardog";
/// Discovery slug for Songbird.
pub const SONGBIRD: &str = "songbird";
/// Discovery slug for `ToadStool`.
pub const TOADSTOOL: &str = "toadstool";
/// Discovery slug for `NestGate`.
pub const NESTGATE: &str = "nestgate";
/// Discovery slug for Squirrel.
pub const SQUIRREL: &str = "squirrel";
/// Discovery slug for rhizoCrypt.
pub const RHIZOCRYPT: &str = "rhizocrypt";
/// Discovery slug for `LoamSpine`.
pub const LOAMSPINE: &str = "loamspine";
/// Discovery slug for sweetGrass.
pub const SWEETGRASS: &str = "sweetgrass";
/// Discovery slug for petalTongue.
pub const PETALTONGUE: &str = "petaltongue";
/// Discovery slug for biomeOS.
pub const BIOMEOS: &str = "biomeos";
/// Discovery slug for barraCuda.
pub const BARRACUDA: &str = "barracuda";
/// Discovery slug for coralReef.
pub const CORALREEF: &str = "coralreef";
/// Discovery slug for skunkBat.
pub const SKUNKBAT: &str = "skunkbat";

/// Lowercase discovery slug → mixed-case display name.
///
/// Returns the display name for known ecosystem primals, or passes
/// through the input unchanged for unknown names.
#[must_use]
pub fn display_name(slug: &str) -> &str {
    match slug {
        "beardog" => "BearDog",
        "songbird" => "Songbird",
        "toadstool" => "ToadStool",
        "nestgate" => "NestGate",
        "squirrel" => "Squirrel",
        "barracuda" => "barraCuda",
        "coralreef" => "coralReef",
        "biomeos" => "biomeOS",
        "petaltongue" => "petalTongue",
        "rhizocrypt" => "rhizoCrypt",
        "loamspine" => "LoamSpine",
        "sweetgrass" => "sweetGrass",
        "skunkbat" => "skunkBat",
        "sourdough" => "sourDough",
        "primalspring" => "primalSpring",
        "hotspring" => "hotSpring",
        "groundspring" => "groundSpring",
        "neuralspring" => "neuralSpring",
        "wetspring" => "wetSpring",
        "airspring" => "airSpring",
        "healthspring" => "healthSpring",
        "ludospring" => "ludoSpring",
        _ => slug,
    }
}

/// Mixed-case display name → lowercase discovery slug.
///
/// Inverse of [`display_name`]. For known primals, returns the canonical
/// lowercase slug used in socket paths and env var lookups.
#[must_use]
pub fn discovery_slug(display: &str) -> &str {
    match display {
        "BearDog" => "beardog",
        "Songbird" => "songbird",
        "ToadStool" => "toadstool",
        "NestGate" => "nestgate",
        "Squirrel" => "squirrel",
        "barraCuda" => "barracuda",
        "coralReef" => "coralreef",
        "biomeOS" => "biomeos",
        "petalTongue" => "petaltongue",
        "rhizoCrypt" => "rhizocrypt",
        "LoamSpine" => "loamspine",
        "sweetGrass" => "sweetgrass",
        "skunkBat" => "skunkbat",
        "sourDough" => "sourdough",
        "primalSpring" => "primalspring",
        "hotSpring" => "hotspring",
        "groundSpring" => "groundspring",
        "neuralSpring" => "neuralspring",
        "wetSpring" => "wetspring",
        "airSpring" => "airspring",
        "healthSpring" => "healthspring",
        "ludoSpring" => "ludospring",
        _ => display,
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
        let slugs = [
            "beardog",
            "songbird",
            "toadstool",
            "nestgate",
            "squirrel",
            "barracuda",
            "coralreef",
            "biomeos",
            "petaltongue",
            "rhizocrypt",
            "loamspine",
            "sweetgrass",
            "primalspring",
        ];
        for slug in slugs {
            let display = display_name(slug);
            let back = discovery_slug(display);
            assert_eq!(
                back, slug,
                "round-trip failed for {slug} -> {display} -> {back}"
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
}
