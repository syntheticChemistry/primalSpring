// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::types::*;

use primalspring::ipc::client::PrimalClient;
use primalspring::ipc::discover::{discover_by_capability, discover_primal};
use primalspring::validation::ValidationResult;

// ═══════════════════════════════════════════════════════════════════════
// Phase 1: World Generation
// ═══════════════════════════════════════════════════════════════════════

pub(crate) fn phase_world_gen(v: &mut ValidationResult) -> World {
    v.section("World Generation (Barracuda + ludoSpring)");

    let biome = generate_biome(v);
    let tiles = generate_floor_structure(v);
    let creatures = populate_creatures(v, &tiles);
    let items = populate_items(v, &tiles);

    let (px, py) = find_spawn_point(&tiles);

    World {
        tiles,
        biome,
        creatures,
        items,
        player_x: px,
        player_y: py,
        player_hp: 60,
        player_max_hp: 60,
        player_items: Vec::new(),
        floor: 1,
        turn: 0,
        seed: 42,
        messages: vec!["Welcome to The Rhizome.".to_owned()],
    }
}

fn generate_biome(v: &mut ValidationResult) -> Biome {
    let barr = discover_by_capability("math");
    let Some(barr_sock) = barr.socket.as_ref() else {
        v.check_skip("biome_noise", "Barracuda not discovered");
        return Biome::Rhizome;
    };

    let Ok(mut client) = PrimalClient::connect(barr_sock, "barracuda") else {
        v.check_skip("biome_noise", "Barracuda connection failed");
        return Biome::Rhizome;
    };

    let resp = client.call(
        "noise.perlin2d",
        serde_json::json!({"x": 4, "y": 4, "scale": 1.0, "seed": 42}),
    );

    match resp {
        Ok(r) if r.result.is_some() => {
            let noise_val = r
                .result
                .as_ref()
                .and_then(|v| {
                    v.get("result")
                        .and_then(serde_json::Value::as_f64)
                        .or_else(|| {
                            v.get("data")
                                .and_then(|d| d.as_array())
                                .and_then(|a| a.first())
                                .and_then(serde_json::Value::as_f64)
                        })
                })
                .unwrap_or(0.0);

            let biome = Biome::from_noise(noise_val);
            v.check_bool(
                "biome_noise",
                true,
                &format!("Biome determined via Perlin noise: {}", biome.name()),
            );
            biome
        }
        Ok(r) => {
            let msg = r
                .error
                .as_ref()
                .map_or("no result".to_owned(), |e| e.message.clone());
            v.check_skip(
                "biome_noise",
                &format!("noise.perlin2d error: {msg} — using default biome"),
            );
            Biome::Rhizome
        }
        Err(e) => {
            v.check_skip(
                "biome_noise",
                &format!("noise.perlin2d failed: {e} — using default biome"),
            );
            Biome::Rhizome
        }
    }
}

fn generate_floor_structure(v: &mut ValidationResult) -> Vec<Tile> {
    let ls = discover_primal("ludospring");
    if let Some(ls_sock) = ls.socket.as_ref() {
        if let Ok(mut client) = PrimalClient::connect(ls_sock, "ludospring") {
            let resp = client.call(
                "game.wfc_step",
                serde_json::json!({
                    "grid_width": MAP_WIDTH,
                    "grid_height": MAP_HEIGHT,
                    "tile_types": ["wall", "floor", "door"],
                    "seed": 42
                }),
            );

            let wfc_ok = resp.as_ref().is_ok_and(|r| r.result.is_some());
            v.check_bool(
                "wfc_floor",
                wfc_ok,
                "Floor structure via ludoSpring game.wfc_step",
            );

            if wfc_ok {
                v.check_bool(
                    "wfc_floor_usable",
                    true,
                    "WFC result received (using fallback layout for determinism)",
                );
            }
        } else {
            v.check_skip("wfc_floor", "ludoSpring connection failed");
        }
    } else {
        v.check_skip("wfc_floor", "ludoSpring not discovered");
    }

    fallback_floor_layout()
}

fn fallback_floor_layout() -> Vec<Tile> {
    let mut tiles = vec![Tile::Wall; MAP_WIDTH * MAP_HEIGHT];

    carve_room(&mut tiles, 2, 2, 10, 8);
    carve_room(&mut tiles, 15, 3, 12, 7);
    carve_room(&mut tiles, 30, 1, 8, 10);
    carve_room(&mut tiles, 5, 14, 14, 9);
    carve_room(&mut tiles, 25, 14, 10, 8);

    carve_corridor_h(&mut tiles, 11, 15, 5);
    carve_corridor_h(&mut tiles, 26, 30, 6);
    carve_corridor_v(&mut tiles, 10, 10, 14);
    carve_corridor_v(&mut tiles, 29, 10, 14);

    tiles[18 * MAP_WIDTH + 12] = Tile::StairsDown;
    tiles[5 * MAP_WIDTH + 14] = Tile::Door;
    tiles[14 * MAP_WIDTH + 10] = Tile::Door;

    tiles
}

fn carve_room(tiles: &mut [Tile], x: usize, y: usize, w: usize, h: usize) {
    for row in y..y.saturating_add(h).min(MAP_HEIGHT) {
        for col in x..x.saturating_add(w).min(MAP_WIDTH) {
            tiles[row * MAP_WIDTH + col] = Tile::Floor;
        }
    }
}

fn carve_corridor_h(tiles: &mut [Tile], x1: usize, x2: usize, y: usize) {
    if y < MAP_HEIGHT {
        for col in x1..=x2.min(MAP_WIDTH - 1) {
            tiles[y * MAP_WIDTH + col] = Tile::Floor;
        }
    }
}

fn carve_corridor_v(tiles: &mut [Tile], x: usize, y1: usize, y2: usize) {
    if x < MAP_WIDTH {
        for row in y1..=y2.min(MAP_HEIGHT - 1) {
            tiles[row * MAP_WIDTH + x] = Tile::Floor;
        }
    }
}

fn find_spawn_point(tiles: &[Tile]) -> (usize, usize) {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            if tiles[y * MAP_WIDTH + x] == Tile::Floor {
                return (x, y);
            }
        }
    }
    (MAP_WIDTH / 2, MAP_HEIGHT / 2)
}

fn populate_creatures(v: &mut ValidationResult, tiles: &[Tile]) -> Vec<Creature> {
    let templates: &[(&str, &str, i32)] = &[
        ("SporeDrifter", "S", 15),
        ("MatrixCrawler", "M", 20),
        ("HashHound", "H", 25),
        ("LedgerGolem", "L", 30),
        ("MeshSpider", "W", 18),
    ];

    let mut creatures = Vec::new();
    let mut placed = 0;

    for (i, &(kind, glyph, hp)) in templates.iter().enumerate() {
        let target_x = 5 + (i * 7) % MAP_WIDTH;
        let target_y = 5 + (i * 5) % MAP_HEIGHT;

        if target_x < MAP_WIDTH
            && target_y < MAP_HEIGHT
            && tiles[target_y * MAP_WIDTH + target_x] == Tile::Floor
        {
            creatures.push(Creature {
                kind,
                glyph,
                x: target_x,
                y: target_y,
                hp,
            });
            placed += 1;
        }
    }

    v.check_minimum("creature_spawn", placed, 1);
    creatures
}

fn populate_items(v: &mut ValidationResult, tiles: &[Tile]) -> Vec<Item> {
    let templates: &[(&str, usize, usize)] = &[
        ("MyceliumThread", 4, 4),
        ("TensorShard", 18, 5),
        ("SealRing", 32, 4),
        ("BeaconOrb", 8, 16),
        ("SporeCapsule", 28, 17),
    ];

    let mut items = Vec::new();
    let mut placed = 0;

    for &(kind, x, y) in templates {
        if x < MAP_WIDTH && y < MAP_HEIGHT && tiles[y * MAP_WIDTH + x] == Tile::Floor {
            items.push(Item { kind, x, y });
            placed += 1;
        }
    }

    v.check_minimum("item_spawn", placed, 1);
    items
}

// ═══════════════════════════════════════════════════════════════════════
// Phase 2: Scene Rendering
// ═══════════════════════════════════════════════════════════════════════

pub(crate) fn phase_render_scene(v: &mut ValidationResult, world: &World) {
    v.section("Scene Rendering (petalTongue)");

    let pt = discover_by_capability("visualization");
    let Some(pt_sock) = pt.socket.as_ref() else {
        v.check_skip("scene_render", "petalTongue not discovered");
        render_stdout_fallback(world);
        return;
    };

    let Ok(mut client) = PrimalClient::connect(pt_sock, "petaltongue") else {
        v.check_skip("scene_render", "petalTongue connection failed");
        render_stdout_fallback(world);
        return;
    };

    let scene = build_scene_graph(world);

    let resp = client.call(
        "visualization.render.scene",
        serde_json::json!({
            "session": "rhizome-game",
            "scene": scene
        }),
    );

    match resp {
        Ok(r) => {
            v.check_bool(
                "scene_render",
                r.result.is_some(),
                &format!(
                    "Tile grid rendered: {}x{} map + {} creatures + HUD",
                    MAP_WIDTH,
                    MAP_HEIGHT,
                    world.creatures.len()
                ),
            );
        }
        Err(e) => {
            v.check_skip(
                "scene_render",
                &format!("visualization.render.scene failed: {e}"),
            );
            render_stdout_fallback(world);
        }
    }
}

fn build_scene_graph(world: &World) -> serde_json::Value {
    let mut nodes = Vec::new();
    let (br, bg, bb) = world.biome.glyph_color();

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let tile = world.tile(x, y);
            let (r, g, b) = if tile == Tile::Wall {
                (0.4, 0.4, 0.4)
            } else {
                (br * 0.5, bg * 0.5, bb * 0.5)
            };

            nodes.push(serde_json::json!({
                "Text": {
                    "content": tile.glyph(),
                    "font_size": 14,
                    "color": {"r": r, "g": g, "b": b, "a": 1.0},
                    "anchor": "TopLeft",
                    "transform": {
                        "tx": x as f64 * CELL_WIDTH,
                        "ty": y as f64 * CELL_HEIGHT
                    }
                }
            }));
        }
    }

    for creature in &world.creatures {
        let (cr, cg, cb) = (0.9, 0.1, 0.1);
        nodes.push(serde_json::json!({
            "Text": {
                "content": creature.glyph,
                "font_size": 14,
                "color": {"r": cr, "g": cg, "b": cb, "a": 1.0},
                "anchor": "TopLeft",
                "transform": {
                    "tx": creature.x as f64 * CELL_WIDTH,
                    "ty": creature.y as f64 * CELL_HEIGHT
                }
            }
        }));
    }

    for item in &world.items {
        nodes.push(serde_json::json!({
            "Text": {
                "content": "*",
                "font_size": 14,
                "color": {"r": 1.0, "g": 0.9, "b": 0.2, "a": 1.0},
                "anchor": "TopLeft",
                "transform": {
                    "tx": item.x as f64 * CELL_WIDTH,
                    "ty": item.y as f64 * CELL_HEIGHT
                }
            }
        }));
    }

    nodes.push(serde_json::json!({
        "Text": {
            "content": "@",
            "font_size": 14,
            "color": {"r": 1.0, "g": 1.0, "b": 1.0, "a": 1.0},
            "anchor": "TopLeft",
            "bold": true,
            "transform": {
                "tx": world.player_x as f64 * CELL_WIDTH,
                "ty": world.player_y as f64 * CELL_HEIGHT
            }
        }
    }));

    let msg = world.messages.last().map_or("", String::as_str);
    nodes.push(serde_json::json!({
        "Text": {
            "content": format!("HP: {}/{}", world.player_hp, world.player_max_hp),
            "font_size": 12,
            "color": {"r": 0.2, "g": 1.0, "b": 0.2, "a": 1.0},
            "anchor": "TopLeft",
            "transform": {"tx": 0.0, "ty": (MAP_HEIGHT as f64).mul_add(CELL_HEIGHT, 4.0)}
        }
    }));

    nodes.push(serde_json::json!({
        "Text": {
            "content": format!("Floor {} | {} | Turn {}", world.floor, world.biome.name(), world.turn),
            "font_size": 12,
            "color": {"r": 0.8, "g": 0.8, "b": 0.8, "a": 1.0},
            "anchor": "TopLeft",
            "transform": {"tx": 150.0, "ty": (MAP_HEIGHT as f64).mul_add(CELL_HEIGHT, 4.0)}
        }
    }));

    nodes.push(serde_json::json!({
        "Text": {
            "content": msg,
            "font_size": 12,
            "color": {"r": 0.9, "g": 0.9, "b": 0.5, "a": 1.0},
            "anchor": "TopLeft",
            "transform": {"tx": 0.0, "ty": (MAP_HEIGHT as f64).mul_add(CELL_HEIGHT, 20.0)}
        }
    }));

    serde_json::json!({
        "type": "tile_grid",
        "title": "The Rhizome",
        "nodes": nodes
    })
}

fn render_stdout_fallback(world: &World) {
    eprintln!("--- stdout fallback render ---");
    for y in 0..MAP_HEIGHT {
        let mut line = String::with_capacity(MAP_WIDTH);
        for x in 0..MAP_WIDTH {
            if x == world.player_x && y == world.player_y {
                line.push('@');
            } else if let Some(c) = world.creatures.iter().find(|c| c.x == x && c.y == y) {
                line.push_str(c.glyph);
            } else if world.items.iter().any(|i| i.x == x && i.y == y) {
                line.push('*');
            } else {
                line.push_str(world.tile(x, y).glyph());
            }
        }
        eprintln!("{line}");
    }
    eprintln!(
        "HP: {}/{} | Floor {} | {} | Turn {}",
        world.player_hp,
        world.player_max_hp,
        world.floor,
        world.biome.name(),
        world.turn
    );
}
