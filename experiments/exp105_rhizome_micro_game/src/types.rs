// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt::Write;

pub const MAP_WIDTH: usize = 40;
pub const MAP_HEIGHT: usize = 25;
pub const CELL_WIDTH: f64 = 10.0;
pub const CELL_HEIGHT: f64 = 16.0;

// ─── Biome types ────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub enum Biome {
    Rhizome,
    TensorCaves,
    CryptoTunnels,
    ProvenanceVaults,
    DiscoveryPassages,
}

impl Biome {
    pub(crate) const fn glyph_color(self) -> (f64, f64, f64) {
        match self {
            Self::Rhizome => (0.2, 0.8, 0.3),
            Self::TensorCaves => (0.3, 0.5, 0.9),
            Self::CryptoTunnels => (0.9, 0.3, 0.2),
            Self::ProvenanceVaults => (0.9, 0.8, 0.2),
            Self::DiscoveryPassages => (0.2, 0.8, 0.8),
        }
    }

    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Rhizome => "Rhizome Network",
            Self::TensorCaves => "Tensor Caves",
            Self::CryptoTunnels => "Crypto Tunnels",
            Self::ProvenanceVaults => "Provenance Vaults",
            Self::DiscoveryPassages => "Discovery Passages",
        }
    }

    pub(crate) fn from_noise(val: f64) -> Self {
        if val < -0.3 {
            Self::CryptoTunnels
        } else if val < 0.0 {
            Self::TensorCaves
        } else if val < 0.3 {
            Self::Rhizome
        } else if val < 0.6 {
            Self::ProvenanceVaults
        } else {
            Self::DiscoveryPassages
        }
    }
}

// ─── Tile types ─────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
    StairsDown,
    Door,
    #[allow(dead_code)]
    Water,
}

impl Tile {
    pub(crate) const fn glyph(self) -> &'static str {
        match self {
            Self::Wall => "#",
            Self::Floor => ".",
            Self::StairsDown => ">",
            Self::Door => "+",
            Self::Water => "~",
        }
    }
}

// ─── Creature ───────────────────────────────────────────────────────

#[derive(Clone)]
pub struct Creature {
    pub(crate) kind: &'static str,
    pub(crate) glyph: &'static str,
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) hp: i32,
}

// ─── Item ───────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct Item {
    pub(crate) kind: &'static str,
    pub(crate) x: usize,
    pub(crate) y: usize,
}

// ─── World state ────────────────────────────────────────────────────

pub struct World {
    pub(crate) tiles: Vec<Tile>,
    pub(crate) biome: Biome,
    pub(crate) creatures: Vec<Creature>,
    pub(crate) items: Vec<Item>,
    pub(crate) player_x: usize,
    pub(crate) player_y: usize,
    pub(crate) player_hp: i32,
    pub(crate) player_max_hp: i32,
    pub(crate) player_items: Vec<String>,
    pub(crate) floor: u32,
    pub(crate) turn: u32,
    pub(crate) seed: u64,
    pub(crate) messages: Vec<String>,
}

impl World {
    pub(crate) fn tile(&self, x: usize, y: usize) -> Tile {
        self.tiles[y * MAP_WIDTH + x]
    }

    #[allow(dead_code)]
    pub(crate) fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.tiles[y * MAP_WIDTH + x] = tile;
    }

    pub(crate) fn to_save_toml(&self) -> String {
        let items_str: Vec<String> = self
            .player_items
            .iter()
            .map(|i| format!("\"{i}\""))
            .collect();
        let mut toml = format!(
            r#"[save]
version = "1.0.0"
session_id = "exp105-validation"
player_name = "Fieldmouse"
floor = {floor}
turn = {turn}
seed = {seed}

[save.player]
x = {px}
y = {py}
hp = {hp}
max_hp = {max_hp}
items = [{items}]

[save.world]
biome = "{biome}"
width = {w}
height = {h}
creature_count = {cc}
item_count = {ic}
"#,
            floor = self.floor,
            turn = self.turn,
            seed = self.seed,
            px = self.player_x,
            py = self.player_y,
            hp = self.player_hp,
            max_hp = self.player_max_hp,
            items = items_str.join(", "),
            biome = self.biome.name(),
            w = MAP_WIDTH,
            h = MAP_HEIGHT,
            cc = self.creatures.len(),
            ic = self.items.len(),
        );

        for c in &self.creatures {
            let _ = write!(
                toml,
                "\n[[save.creatures]]\nkind = \"{}\"\nx = {}\ny = {}\nhp = {}\n",
                c.kind, c.x, c.y, c.hp
            );
        }
        for item in &self.items {
            let _ = write!(
                toml,
                "\n[[save.items]]\nkind = \"{}\"\nx = {}\ny = {}\n",
                item.kind, item.x, item.y
            );
        }
        toml
    }
}

pub fn hash_simple(data: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in data.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}
