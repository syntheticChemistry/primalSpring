# The Rhizome -- Micro-Game Design Specification

**Date**: April 28, 2026
**Status**: Phase 56 -- Desktop Substrate
**Related**: `DESKTOP_SESSION_MODEL.md`, `STORYTELLING_EVOLUTION.md`, `MICRO_DESKTOP_COMPOSITION.md`

---

## Overview

The Rhizome is an ecoPrimals-themed procedural roguelike that exercises every
NUCLEUS primal end-to-end. The player is a **Fieldmouse** exploring underground
biome caverns, each themed after a primal domain. The game runs as a
biomeOS-managed desktop application, rendered through petalTongue's scene graph,
with saves stored in NestGate and sealed by the provenance trio.

The Rhizome serves two purposes:
1. A playable micro-game demonstrating the NUCLEUS as a stable application substrate
2. A gap-finding harness that documents every IPC mismatch, missing method, and
   routing failure encountered during real gameplay

---

## World Structure

### Biomes (5 types)

Each biome is a procedurally generated cavern floor. Biome type is determined by
Perlin noise sampling from Barracuda (`noise.perlin2d`).

| Biome | Theme | Tile Color | Creatures | Items |
|-------|-------|-----------|-----------|-------|
| Rhizome Network | Root tunnels, mycelium | Green | Spore Drifters, Root Weavers | Mycelium Thread, Spore Capsule |
| Tensor Caves | Crystalline math halls | Blue | Matrix Crawlers, Eigen Bats | Tensor Shard, Spectral Lens |
| Crypto Tunnels | Encrypted passages | Red | Hash Hounds, Cipher Moths | Key Fragment, Nonce Stone |
| Provenance Vaults | Sealed archive chambers | Gold | Ledger Golems, Merkle Wisps | Seal Ring, Chain Link |
| Discovery Passages | Echoing mesh corridors | Cyan | Songbird Echoes, Mesh Spiders | Beacon Orb, Route Map |

### Floor Generation

Each floor is a 40x25 tile grid generated in two phases:

1. **Structure** via ludoSpring `game.wfc_step`:
   - Input: `grid_width=40, grid_height=25`
   - Output: tile constraint grid (walls, floors, doors, corridors)

2. **Biome assignment** via Barracuda `noise.perlin2d`:
   - Sample noise at each floor tile position
   - Threshold to biome type based on noise value ranges

3. **Population** via Barracuda `rng.uniform`:
   - Place creatures (density scaled by floor depth)
   - Place items (rarity scaled by ludoSpring `game.difficulty_adjustment`)
   - Place stairs down (always one per floor)

### Tile Legend

```
@ = Player (Fieldmouse)
# = Wall
. = Floor
> = Stairs down
+ = Door
~ = Water/mycelium
* = Item
A-Z = Creatures (letter = type)
```

---

## Game Mechanics

### Turn-Based Loop

Each player action (move, pickup, wait) consumes one turn. On each turn:

1. Player action resolves
2. All creatures take one action (simple chase/wander AI)
3. Visibility recalculated (field of view)
4. Flow state evaluated via ludoSpring `game.evaluate_flow`
5. Scene pushed to petalTongue

### Combat

Simple bump-to-attack system:
- Player HP starts at 60, regenerates 1/turn when not in combat
- Creature HP varies by type (10-40)
- Damage = `Barracuda stats.mean([base_attack, floor_depth, rng_roll])`
- DDA via ludoSpring: `game.difficulty_adjustment` tweaks creature spawn density

### Encounters (AI Narration)

When the player enters a new biome or encounters a named creature:
- Squirrel `ai.chat` generates a 1-2 sentence description
- Falls back to template text if Squirrel unavailable (HTTP provider gap)
- Narration recorded in rhizoCrypt DAG as `AgentAction` event

### Items

Items are passive stat modifiers stored in the player inventory array:

| Item | Effect | Biome |
|------|--------|-------|
| Mycelium Thread | +5 max HP | Rhizome |
| Tensor Shard | +2 damage | Tensor |
| Key Fragment | Opens crypto doors | Crypto |
| Seal Ring | Saves cost 0 turns | Provenance |
| Beacon Orb | Reveals full floor map | Discovery |
| Spore Capsule | Heals 20 HP | Rhizome |
| Spectral Lens | See invisible creatures | Tensor |
| Nonce Stone | Avoid one attack | Crypto |
| Chain Link | +1 armor | Provenance |
| Route Map | Shows stairs location | Discovery |

---

## Rendering Contract

### Scene Graph Structure

The game renders via `visualization.render.scene` with this node hierarchy:

```
root
  +-- map_layer        (40x25 Text primitives for terrain)
  +-- creature_layer   (Text primitives for visible creatures)
  +-- item_layer       (Text primitives for visible items)
  +-- player_node      (single Text "@" at player position)
  +-- hud_layer
  |     +-- hp_bar     (Text: "HP: 45/60")
  |     +-- floor_info (Text: "Floor 3 | Tensor Caves | Turn 247")
  |     +-- message    (Text: last game message)
  +-- narration_box    (Text: AI narration, bottom of screen)
```

### Primitive Format

Each tile is a Text primitive with grid-aligned transform:

```json
{
  "Text": {
    "x": 0, "y": 0,
    "content": "#",
    "font_size": 14,
    "color": {"r": 0.6, "g": 0.6, "b": 0.6, "a": 1.0},
    "anchor": "TopLeft",
    "bold": false,
    "italic": false,
    "data_id": null
  }
}
```

Grid positioning uses the node transform:
- `tx = col * CELL_WIDTH` (CELL_WIDTH = 10 pixels)
- `ty = row * CELL_HEIGHT` (CELL_HEIGHT = 16 pixels)

### Scene Session

The game creates a petalTongue session (`session_id = "rhizome-game"`)
and pushes full scene updates on every turn.

---

## Save System

### Format (TOML)

```toml
[save]
version = "1.0.0"
session_id = "019dd5de-dfcd-7050-a9cc-692b839b9f69"
dag_session_id = "019dd5de-dfcd-7050-a9cc-692b839b9f69"
player_name = "Fieldmouse"
floor = 3
turn = 247
seed = 42

[save.player]
x = 12
y = 8
hp = 45
max_hp = 60
attack = 5
armor = 1
items = ["rhizome_key", "tensor_shard", "spore_capsule"]

[save.world]
biome = "tensor_caves"
width = 40
height = 25
tiles = "base64-encoded-byte-array"
creature_count = 7
item_count = 3

[[save.creatures]]
kind = "MatrixCrawler"
x = 15
y = 10
hp = 20

[[save.items]]
kind = "SpectralLens"
x = 22
y = 14
```

### Storage Pipeline

1. Serialize world state to TOML string
2. NestGate `storage.store`: key=`save:rhizome:{session_id}`, namespace=`rhizome`, value=TOML string
3. rhizoCrypt `dag.event.append`: event_type=`Custom{label:"save", event_name:"game_save", domain:"game"}`, data includes save hash
4. loamSpine `entry.append`: committer="rhizome-game", entry_type=`Custom{label:"save", type_uri:"urn:eco:rhizome-save", domain:"game", payload:[bytes]}`
5. sweetGrass `contribution.record`: agent="rhizome-player", role="Creator", content_hash=save_hash

### Load Pipeline

1. NestGate `storage.get`: key=`save:rhizome:{session_id}`, namespace=`rhizome`
2. Parse TOML string back to game state
3. Verify: rhizoCrypt `dag.merkle.root` matches expected chain
4. Resume game from loaded state

---

## Primal Integration Map

| Primal | Method | Purpose | Fallback |
|--------|--------|---------|----------|
| Barracuda | `noise.perlin2d` | Biome noise map | Hardcoded biome pattern |
| Barracuda | `rng.uniform` | Loot/creature RNG | Local PRNG |
| Barracuda | `stats.mean` | Damage calculation | Simple average |
| ludoSpring | `game.wfc_step` | Floor structure gen | Hardcoded room templates |
| ludoSpring | `game.evaluate_flow` | Flow state tracking | Skip (cosmetic) |
| ludoSpring | `game.difficulty_adjustment` | DDA scaling | Linear floor scaling |
| Squirrel | `ai.chat` | Encounter narration | Template strings |
| NestGate | `storage.store/get` | Save/load | Skip save feature |
| rhizoCrypt | `dag.session.create` | Provenance session | Skip provenance |
| rhizoCrypt | `dag.event.append` | Save event recording | Skip provenance |
| rhizoCrypt | `dag.merkle.root` | Save chain integrity | Skip verification |
| loamSpine | `spine.create` | Session ledger | Skip ledger |
| loamSpine | `entry.append` | Play events | Skip ledger |
| sweetGrass | `braid.create` | Attribution | Skip attribution |
| sweetGrass | `contribution.record` | Play credit | Skip attribution |
| petalTongue | `visualization.render.scene` | Tile rendering | Print to stdout |
| petalTongue | `proprioception.get` | Frame rate check | Skip |
| Songbird | `ipc.list` | Discovery | Direct socket paths |
| BearDog | `crypto.blake3_hash` | Save hash | Local hash |
| biomeOS | `capability.call` | Semantic routing | Direct primal calls |

---

## Experiment Structure

The game is implemented as `exp105_rhizome_micro_game`, a primalSpring experiment
using the `ValidationResult` framework. Each primal integration point is a
validation check that either succeeds or degrades gracefully.

The experiment runs as a single-pass validation:
1. Generate a world (exercises Barracuda + ludoSpring)
2. Render initial scene (exercises petalTongue)
3. Simulate 10 turns (exercises game loop)
4. Save game (exercises NestGate + provenance trio)
5. Load game (exercises NestGate + rhizoCrypt verify)
6. Report gaps

This validates the full stack without requiring interactive input (the validation
simulates player actions deterministically).
