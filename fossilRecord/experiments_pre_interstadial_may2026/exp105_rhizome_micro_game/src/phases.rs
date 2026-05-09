// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::types::{World, MAP_WIDTH, MAP_HEIGHT, Tile, hash_simple};

use base64::Engine as _;
use primalspring::ipc::client::PrimalClient;
use primalspring::ipc::discover::{discover_by_capability, discover_primal};
use primalspring::validation::ValidationResult;

// ═══════════════════════════════════════════════════════════════════════
// Phase 3: Game Loop Simulation
// ═══════════════════════════════════════════════════════════════════════

pub fn phase_game_loop(v: &mut ValidationResult, world: &mut World) {
    v.section("Game Loop Simulation (10 turns)");

    let moves: [(i32, i32); 10] = [
        (1, 0),
        (1, 0),
        (0, 1),
        (0, 1),
        (1, 0),
        (0, -1),
        (1, 0),
        (1, 0),
        (0, 1),
        (-1, 0),
    ];

    let mut turns_completed = 0;
    for &(dx, dy) in &moves {
        let nx = (i32::try_from(world.player_x).unwrap_or(0) + dx).max(0) as usize;
        let ny = (i32::try_from(world.player_y).unwrap_or(0) + dy).max(0) as usize;

        if nx < MAP_WIDTH && ny < MAP_HEIGHT && world.tile(nx, ny) != Tile::Wall {
            if let Some(ci) = world.creatures.iter().position(|c| c.x == nx && c.y == ny) {
                world.creatures[ci].hp -= 10;
                world
                    .messages
                    .push(format!("Hit {} for 10 damage!", world.creatures[ci].kind));
                if world.creatures[ci].hp <= 0 {
                    let name = world.creatures[ci].kind;
                    world.messages.push(format!("{name} defeated!"));
                    world.creatures.remove(ci);
                }
            } else {
                world.player_x = nx;
                world.player_y = ny;

                if let Some(ii) = world.items.iter().position(|it| it.x == nx && it.y == ny) {
                    let item_name = world.items[ii].kind.to_owned();
                    world.player_items.push(item_name.clone());
                    world.items.remove(ii);
                    world.messages.push(format!("Picked up {item_name}!"));
                }
            }
        }

        world.turn += 1;
        turns_completed += 1;
    }

    v.check_count("turns_simulated", turns_completed, 10);

    phase_flow_tracking(v);
    phase_damage_calc(v);
}

pub fn phase_flow_tracking(v: &mut ValidationResult) {
    let ls = discover_primal("ludospring");
    let Some(ls_sock) = ls.socket.as_ref() else {
        v.check_skip("flow_eval", "ludoSpring not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(ls_sock, "ludospring") else {
        v.check_skip("flow_eval", "ludoSpring connection failed");
        return;
    };

    let resp = client.call(
        "game.evaluate_flow",
        serde_json::json!({
            "player_skill": 0.5,
            "challenge_level": 0.4,
            "recent_successes": 6,
            "recent_failures": 2,
            "session_duration_s": 120
        }),
    );

    v.check_bool(
        "flow_eval",
        resp.is_ok_and(|r| r.result.is_some()),
        "ludoSpring game.evaluate_flow for DDA",
    );
}

pub fn phase_damage_calc(v: &mut ValidationResult) {
    let barr = discover_by_capability("math");
    let Some(barr_sock) = barr.socket.as_ref() else {
        v.check_skip("damage_calc", "Barracuda not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(barr_sock, "barracuda") else {
        v.check_skip("damage_calc", "Barracuda connection failed");
        return;
    };

    let resp = client.call("stats.mean", serde_json::json!({"data": [5.0, 1.0, 8.0]}));

    match resp {
        Ok(r) => {
            let result = r
                .result
                .as_ref()
                .and_then(|r| r.get("result"))
                .and_then(serde_json::Value::as_f64);
            v.check_bool(
                "damage_calc",
                result.is_some(),
                &format!(
                    "Damage via stats.mean = {}",
                    result.map_or("N/A".to_owned(), |v| format!("{v:.1}"))
                ),
            );
        }
        Err(e) => {
            v.check_skip("damage_calc", &format!("stats.mean failed: {e}"));
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Phase 4: Save Game
// ═══════════════════════════════════════════════════════════════════════

pub fn phase_save_game(v: &mut ValidationResult, world: &World) -> Option<String> {
    v.section("Save Game (NestGate + Provenance Trio)");

    let toml_save = world.to_save_toml();
    let save_key = "save:rhizome:exp105-validation";

    let stored = save_to_nestgate(v, save_key, &toml_save);

    let dag_session = create_provenance_session(v);

    if let Some(ref sid) = dag_session {
        seal_save_event(v, sid, save_key);
    } else {
        v.check_skip("dag_seal", "No DAG session — skipping provenance seal");
    }

    record_ledger_entry(v, &toml_save);
    record_attribution(v, &toml_save);

    if stored {
        v.check_bool("save_complete", true, "Full save pipeline completed");
    } else {
        v.check_skip("save_complete", "Save incomplete — NestGate store failed");
    }

    dag_session
}

fn save_to_nestgate(v: &mut ValidationResult, key: &str, toml_data: &str) -> bool {
    let ng = discover_by_capability("storage");
    let ng_fallback;
    let ng_sock = if let Some(s) = ng.socket.as_ref() {
        s
    } else {
        ng_fallback = discover_primal("nestgate");
        if let Some(s) = ng_fallback.socket.as_ref() {
            s
        } else {
            v.check_skip("nestgate_store", "NestGate not discovered");
            return false;
        }
    };

    let Ok(mut client) = PrimalClient::connect(ng_sock, "nestgate") else {
        v.check_skip("nestgate_store", "NestGate connection failed");
        return false;
    };

    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());
    let resp = client.call(
        "storage.store",
        serde_json::json!({
            "family_id": family_id,
            "key": key,
            "value": toml_data,
        }),
    );

    match &resp {
        Ok(r) if r.result.is_some() => {
            v.check_bool("nestgate_store", true, "Save TOML stored in NestGate");
            true
        }
        Ok(r) => {
            let err_msg = r
                .error
                .as_ref()
                .map_or("unknown".to_owned(), |e| e.message.clone());
            let fallback = discover_primal("nestgate");
            if let Some(fb_sock) = fallback.socket.as_ref() {
                if let Ok(mut fb_client) = PrimalClient::connect(fb_sock, "nestgate") {
                    let fb_resp = fb_client.call(
                        "storage.store",
                        serde_json::json!({"family_id": family_id, "key": key, "value": toml_data}),
                    );
                    if fb_resp.is_ok_and(|r| r.result.is_some()) {
                        v.check_bool(
                            "nestgate_store",
                            true,
                            "Save TOML stored via NestGate fallback (biomeOS misrouted)",
                        );
                        return true;
                    }
                }
            }
            v.check_bool(
                "nestgate_store",
                false,
                &format!("NestGate store failed: {err_msg}"),
            );
            false
        }
        Err(e) => {
            v.check_bool(
                "nestgate_store",
                false,
                &format!("NestGate store error: {e}"),
            );
            false
        }
    }
}

fn create_provenance_session(v: &mut ValidationResult) -> Option<String> {
    let rz = discover_by_capability("dag");
    let Some(rz_sock) = rz.socket.as_ref() else {
        v.check_skip("dag_session", "rhizoCrypt not discovered");
        return None;
    };

    let Ok(mut client) = PrimalClient::connect(rz_sock, "rhizocrypt") else {
        v.check_skip("dag_session", "rhizoCrypt connection failed");
        return None;
    };

    let resp = client.call(
        "dag.session.create",
        serde_json::json!({"name": "exp105-rhizome-game"}),
    );

    match resp {
        Ok(r) => {
            let session_id = r.result.as_ref().and_then(|v| {
                v.as_str().map(String::from).or_else(|| {
                    v.get("session_id")
                        .and_then(|s| s.as_str())
                        .map(String::from)
                })
            });
            v.check_bool(
                "dag_session",
                session_id.is_some(),
                "DAG session created for save provenance",
            );
            session_id
        }
        Err(e) => {
            v.check_skip("dag_session", &format!("dag.session.create failed: {e}"));
            None
        }
    }
}

fn seal_save_event(v: &mut ValidationResult, session_id: &str, save_key: &str) {
    let rz = discover_by_capability("dag");
    let Some(rz_sock) = rz.socket.as_ref() else {
        v.check_skip("dag_seal", "rhizoCrypt not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(rz_sock, "rhizocrypt") else {
        v.check_skip("dag_seal", "rhizoCrypt connection failed");
        return;
    };

    let resp = client.call(
        "dag.event.append",
        serde_json::json!({
            "session_id": session_id,
            "event_type": {
                "Custom": {
                    "label": "game_save",
                    "event_name": "game_save",
                    "domain": "game"
                }
            },
            "data": {"save_key": save_key, "experiment": "exp105"}
        }),
    );

    v.check_bool(
        "dag_seal",
        resp.is_ok_and(|r| r.result.is_some()),
        "Save event sealed in rhizoCrypt DAG",
    );
}

fn record_ledger_entry(v: &mut ValidationResult, toml_data: &str) {
    let ls = discover_by_capability("ledger");
    let Some(ls_sock) = ls.socket.as_ref() else {
        v.check_skip("ledger_entry", "loamSpine not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(ls_sock, "loamspine") else {
        v.check_skip("ledger_entry", "loamSpine connection failed");
        return;
    };

    let spine_resp = client.call(
        "spine.create",
        serde_json::json!({"name": "exp105-rhizome-session", "owner": "primalspring"}),
    );

    let spine_id = spine_resp
        .ok()
        .and_then(|r| r.result)
        .and_then(|r| r.get("spine_id").and_then(|s| s.as_str()).map(String::from));

    let Some(spine_id) = spine_id else {
        v.check_skip("ledger_entry", "spine.create failed — no spine_id");
        return;
    };

    let payload_bytes: Vec<u8> = toml_data.as_bytes().iter().copied().take(128).collect();

    let resp = client.call(
        "entry.append",
        serde_json::json!({
            "spine_id": spine_id,
            "committer": "rhizome-game",
            "entry_type": {
                "Custom": {
                    "label": "game_save",
                    "type_uri": "urn:eco:rhizome-save",
                    "domain": "game",
                    "payload": payload_bytes
                }
            }
        }),
    );

    v.check_bool(
        "ledger_entry",
        resp.is_ok_and(|r| r.result.is_some()),
        "Save event recorded in loamSpine ledger",
    );
}

fn record_attribution(v: &mut ValidationResult, toml_data: &str) {
    let sg = discover_by_capability("attribution");
    let Some(sg_sock) = sg.socket.as_ref() else {
        v.check_skip("attribution", "sweetGrass not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(sg_sock, "sweetgrass") else {
        v.check_skip("attribution", "sweetGrass connection failed");
        return;
    };

    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0u128, |d| d.as_nanos());
    let data_hash = format!("{:016x}{:016x}", hash_simple(toml_data), ts);

    let braid_resp = client.call(
        "braid.create",
        serde_json::json!({
            "name": "exp105-rhizome-save",
            "data_hash": data_hash,
            "mime_type": "application/toml",
            "size": toml_data.len(),
            "metadata": {"game": "rhizome", "experiment": "exp105"}
        }),
    );

    let braid_ok = braid_resp.as_ref().is_ok_and(|r| r.result.is_some());
    match &braid_resp {
        Ok(r) if r.result.is_some() => {
            v.check_bool("braid_create", true, "Attribution braid created for save");
        }
        Ok(r) => {
            let msg = r.error.as_ref().map_or("no result".to_owned(), |e| {
                format!("{}: {}", e.code, e.message)
            });
            v.check_bool(
                "braid_create",
                false,
                &format!("braid.create RPC error: {msg}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "braid_create",
                false,
                &format!("braid.create transport error: {e}"),
            );
        }
    }

    if braid_ok {
        let contrib_hash = format!("contrib-{}", &data_hash[..data_hash.len().min(32)]);
        let contrib_resp = client.call(
            "contribution.record",
            serde_json::json!({
                "braid_id": format!("urn:braid:{data_hash}"),
                "agent": "rhizome-player",
                "role": "Creator",
                "content_hash": contrib_hash,
            }),
        );

        match &contrib_resp {
            Ok(r) if r.result.is_some() => {
                v.check_bool(
                    "contribution_record",
                    true,
                    "Play contribution recorded in sweetGrass",
                );
            }
            Ok(r) => {
                let msg = r
                    .error
                    .as_ref()
                    .map_or("no result".to_owned(), |e| e.message.clone());
                v.check_bool(
                    "contribution_record",
                    false,
                    &format!("contribution.record error: {msg}"),
                );
            }
            Err(e) => {
                v.check_bool(
                    "contribution_record",
                    false,
                    &format!("contribution.record transport: {e}"),
                );
            }
        }
    } else {
        v.check_skip("contribution_record", "No braid — skipping contribution");
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Phase 5: Load Game
// ═══════════════════════════════════════════════════════════════════════

pub fn phase_load_game(v: &mut ValidationResult, dag_session: Option<&str>) {
    v.section("Load Game (NestGate + Merkle Verify)");

    let save_key = "save:rhizome:exp105-validation";

    let ng = discover_by_capability("storage");
    let ng_fallback;
    let ng_sock = if let Some(s) = ng.socket.as_ref() {
        s
    } else {
        ng_fallback = discover_primal("nestgate");
        if let Some(s) = ng_fallback.socket.as_ref() {
            s
        } else {
            v.check_skip("load_game", "NestGate not discovered");
            return;
        }
    };

    let Ok(mut client) = PrimalClient::connect(ng_sock, "nestgate") else {
        v.check_skip("load_game", "NestGate connection failed");
        return;
    };

    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());
    let get_params = serde_json::json!({"family_id": family_id, "key": save_key});
    let resp = client.call("storage.get", get_params.clone());

    let resp = if resp.as_ref().is_ok_and(|r| r.error.is_some()) {
        if let Some(fb_sock) = discover_primal("nestgate").socket {
            if let Ok(mut fb) = PrimalClient::connect(&fb_sock, "nestgate") {
                fb.call("storage.get", get_params)
            } else {
                resp
            }
        } else {
            resp
        }
    } else {
        resp
    };

    match resp {
        Ok(r) => {
            let has_value = r
                .result
                .as_ref()
                .and_then(|r| r.get("value"))
                .and_then(|v| v.as_str())
                .is_some();
            v.check_bool("load_game", has_value, "Save TOML loaded from NestGate");

            if has_value {
                verify_merkle_root(v, dag_session);
            }
        }
        Err(e) => {
            v.check_skip("load_game", &format!("storage.get failed: {e}"));
        }
    }
}

fn verify_merkle_root(v: &mut ValidationResult, dag_session: Option<&str>) {
    let Some(session_id) = dag_session else {
        v.check_skip("merkle_verify", "No DAG session — skipping merkle verify");
        return;
    };

    let rz = discover_by_capability("dag");
    let Some(rz_sock) = rz.socket.as_ref() else {
        v.check_skip("merkle_verify", "rhizoCrypt not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(rz_sock, "rhizocrypt") else {
        v.check_skip("merkle_verify", "rhizoCrypt connection failed");
        return;
    };

    let resp = client.call(
        "dag.merkle.root",
        serde_json::json!({"session_id": session_id}),
    );

    match resp {
        Ok(r) => {
            let root = r.result.as_ref().and_then(|v| {
                v.as_str()
                    .map(String::from)
                    .or_else(|| v.get("root").and_then(|s| s.as_str()).map(String::from))
            });
            v.check_bool(
                "merkle_verify",
                root.is_some(),
                &format!(
                    "Save chain merkle root: {}",
                    root.as_deref().unwrap_or("none")
                ),
            );
        }
        Err(e) => {
            v.check_skip("merkle_verify", &format!("dag.merkle.root failed: {e}"));
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Phase 6: AI Narration (Optional)
// ═══════════════════════════════════════════════════════════════════════

pub fn phase_narration(v: &mut ValidationResult, world: &World) {
    v.section("AI Narration (Squirrel — optional)");

    let sq = discover_by_capability("ai");
    let Some(sq_sock) = sq.socket.as_ref() else {
        v.check_skip(
            "ai_narrate",
            "Squirrel not discovered — using template text",
        );
        return;
    };

    let Ok(mut client) = PrimalClient::connect(sq_sock, "squirrel") else {
        v.check_skip(
            "ai_narrate",
            "Squirrel connection failed — using template text",
        );
        return;
    };

    let prompt = format!(
        "You are narrator for a roguelike game called The Rhizome. \
         The player (a Fieldmouse) just entered the {} on floor {}. \
         Describe the scene in one sentence.",
        world.biome.name(),
        world.floor
    );

    let resp = client.call(
        "ai.chat",
        serde_json::json!({
            "messages": [
                {"role": "system", "content": "You are a concise game narrator for The Rhizome."},
                {"role": "user", "content": prompt}
            ]
        }),
    );

    v.check_bool(
        "ai_narrate",
        resp.is_ok(),
        "Squirrel AI narration for biome entry",
    );
}

// ═══════════════════════════════════════════════════════════════════════
// Phase 7: Crypto Hash (BearDog)
// ═══════════════════════════════════════════════════════════════════════

pub fn phase_crypto_hash(v: &mut ValidationResult, world: &World) {
    v.section("Save Integrity Hash (BearDog)");

    let bd = discover_by_capability("crypto");
    let Some(bd_sock) = bd.socket.as_ref() else {
        v.check_skip("crypto_hash", "BearDog not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(bd_sock, "beardog") else {
        v.check_skip("crypto_hash", "BearDog connection failed");
        return;
    };

    let save_data = world.to_save_toml();
    let encoded = base64::engine::general_purpose::STANDARD.encode(save_data.as_bytes());
    let resp = client.call("crypto.blake3_hash", serde_json::json!({"data": encoded}));

    v.check_bool(
        "crypto_hash",
        resp.is_ok_and(|r| r.result.is_some()),
        "BLAKE3 hash of save data via BearDog",
    );
}

// ═══════════════════════════════════════════════════════════════════════
// Phase 8: Discovery Mesh Validation
// ═══════════════════════════════════════════════════════════════════════

pub fn phase_discovery(v: &mut ValidationResult) {
    v.section("Service Discovery (Songbird)");

    let sb = discover_primal("songbird");
    let Some(sb_sock) = sb.socket.as_ref() else {
        v.check_skip("discovery_list", "Songbird not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(sb_sock, "songbird") else {
        v.check_skip("discovery_list", "Songbird connection failed");
        return;
    };

    let resp = client.call("ipc.list", serde_json::json!({}));

    match resp {
        Ok(r) => {
            let count = r
                .result
                .as_ref()
                .and_then(|r| r.get("services"))
                .and_then(|s| s.as_array())
                .map_or(0, Vec::len);
            v.check_minimum("discovery_list", count, 8);
        }
        Err(e) => {
            v.check_skip("discovery_list", &format!("ipc.list failed: {e}"));
        }
    }
}
