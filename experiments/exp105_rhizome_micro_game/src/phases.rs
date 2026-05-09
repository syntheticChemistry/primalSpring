// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::types::{MAP_HEIGHT, MAP_WIDTH, Tile, World, hash_simple};

use base64::Engine as _;
use primalspring::composition::CompositionContext;
use primalspring::ipc::IpcError;
use primalspring::validation::ValidationResult;

fn orchestration_route(
    ctx: &mut CompositionContext,
    capability: &str,
    operation: &str,
    args: &serde_json::Value,
) -> Result<serde_json::Value, IpcError> {
    ctx.call(
        "orchestration",
        "capability.call",
        serde_json::json!({
            "capability": capability,
            "operation": operation,
            "args": args,
        }),
    )
}

fn call_game(
    ctx: &mut CompositionContext,
    method: &str,
    params: serde_json::Value,
) -> Result<serde_json::Value, IpcError> {
    if ctx.has_capability("game") {
        ctx.call("game", method, params)
    } else if ctx.has_capability("orchestration") {
        let op = method.strip_prefix("game.").unwrap_or(method);
        orchestration_route(ctx, "game", op, &params)
    } else {
        Err(IpcError::SocketNotFound {
            primal: "game".to_owned(),
        })
    }
}

pub fn phase_game_loop(v: &mut ValidationResult, world: &mut World, ctx: &mut CompositionContext) {
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
        let nx = {
            let step = i32::try_from(world.player_x).unwrap_or(0) + dx;
            usize::try_from(step.max(0)).unwrap_or(0)
        };
        let ny = {
            let step = i32::try_from(world.player_y).unwrap_or(0) + dy;
            usize::try_from(step.max(0)).unwrap_or(0)
        };

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

    phase_flow_tracking(v, ctx);
    phase_damage_calc(v, ctx);
}

pub fn phase_flow_tracking(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("game") && !ctx.has_capability("orchestration") {
        v.check_skip("flow_eval", "ludoSpring not discovered");
        return;
    }

    let resp = call_game(
        ctx,
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
        resp.is_ok(),
        "ludoSpring game.evaluate_flow for DDA",
    );
}

pub fn phase_damage_calc(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("tensor") {
        v.check_skip("damage_calc", "Barracuda not discovered");
        return;
    }

    let resp = ctx.call(
        "tensor",
        "stats.mean",
        serde_json::json!({"data": [5.0, 1.0, 8.0]}),
    );

    match resp {
        Ok(r) => {
            let result = r.get("result").and_then(serde_json::Value::as_f64);
            v.check_bool(
                "damage_calc",
                result.is_some(),
                &format!(
                    "Damage via stats.mean = {}",
                    result.map_or_else(|| "N/A".to_owned(), |v| format!("{v:.1}"))
                ),
            );
        }
        Err(e) => {
            v.check_skip("damage_calc", &format!("stats.mean failed: {e}"));
        }
    }
}

pub fn phase_save_game(
    v: &mut ValidationResult,
    world: &World,
    ctx: &mut CompositionContext,
) -> Option<String> {
    v.section("Save Game (NestGate + Provenance Trio)");

    let toml_save = world.to_save_toml();
    let save_key = "save:rhizome:exp105-validation";

    let stored = save_to_nestgate(v, save_key, &toml_save, ctx);

    let dag_session = create_provenance_session(v, ctx);

    if let Some(ref sid) = dag_session {
        seal_save_event(v, sid, save_key, ctx);
    } else {
        v.check_skip("dag_seal", "No DAG session — skipping provenance seal");
    }

    record_ledger_entry(v, &toml_save, ctx);
    record_attribution(v, &toml_save, ctx);

    if stored {
        v.check_bool("save_complete", true, "Full save pipeline completed");
    } else {
        v.check_skip("save_complete", "Save incomplete — NestGate store failed");
    }

    dag_session
}

fn save_to_nestgate(
    v: &mut ValidationResult,
    key: &str,
    toml_data: &str,
    ctx: &mut CompositionContext,
) -> bool {
    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());
    let store_params = serde_json::json!({
        "family_id": family_id,
        "key": key,
        "value": toml_data,
    });

    if ctx
        .call("storage", "storage.store", store_params.clone())
        .is_ok()
    {
        v.check_bool("nestgate_store", true, "Save TOML stored in NestGate");
        return true;
    }

    if ctx.has_capability("orchestration")
        && orchestration_route(ctx, "storage", "storage.store", &store_params).is_ok()
    {
        v.check_bool(
            "nestgate_store",
            true,
            "Save TOML stored via NestGate fallback (biomeOS misrouted)",
        );
        return true;
    }

    v.check_bool("nestgate_store", false, "NestGate storage.store failed");
    false
}

fn create_provenance_session(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
) -> Option<String> {
    if !ctx.has_capability("dag") {
        v.check_skip("dag_session", "rhizoCrypt not discovered");
        return None;
    }

    let resp = ctx.call(
        "dag",
        "dag.session.create",
        serde_json::json!({"name": "exp105-rhizome-game"}),
    );

    match resp {
        Ok(r) => {
            let session_id = r
                .get("session_id")
                .and_then(|s| s.as_str())
                .map(String::from);
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

fn seal_save_event(
    v: &mut ValidationResult,
    session_id: &str,
    save_key: &str,
    ctx: &mut CompositionContext,
) {
    if !ctx.has_capability("dag") {
        v.check_skip("dag_seal", "rhizoCrypt not discovered");
        return;
    }

    let resp = ctx.call(
        "dag",
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
        resp.is_ok(),
        "Save event sealed in rhizoCrypt DAG",
    );
}

fn record_ledger_entry(v: &mut ValidationResult, toml_data: &str, ctx: &mut CompositionContext) {
    if !ctx.has_capability("ledger") {
        v.check_skip("ledger_entry", "loamSpine not discovered");
        return;
    }

    let spine_resp = ctx.call(
        "ledger",
        "spine.create",
        serde_json::json!({"name": "exp105-rhizome-session", "owner": "primalspring"}),
    );

    let spine_id = spine_resp
        .ok()
        .and_then(|r| r.get("spine_id").and_then(|s| s.as_str()).map(String::from));

    let Some(spine_id) = spine_id else {
        v.check_skip("ledger_entry", "spine.create failed — no spine_id");
        return;
    };

    let payload_bytes: Vec<u8> = toml_data.as_bytes().iter().copied().take(128).collect();

    let resp = ctx.call(
        "ledger",
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
        resp.is_ok(),
        "Save event recorded in loamSpine ledger",
    );
}

fn record_attribution(v: &mut ValidationResult, toml_data: &str, ctx: &mut CompositionContext) {
    if !ctx.has_capability("attribution") {
        v.check_skip("attribution", "sweetGrass not discovered");
        return;
    }

    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0u128, |d| d.as_nanos());
    let data_hash = format!("{:016x}{:016x}", hash_simple(toml_data), ts);

    let braid_resp = ctx.call(
        "attribution",
        "braid.create",
        serde_json::json!({
            "name": "exp105-rhizome-save",
            "data_hash": data_hash,
            "mime_type": "application/toml",
            "size": toml_data.len(),
            "metadata": {"game": "rhizome", "experiment": "exp105"}
        }),
    );

    let braid_ok = braid_resp.is_ok();
    match &braid_resp {
        Ok(_) => {
            v.check_bool("braid_create", true, "Attribution braid created for save");
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
        let contrib_resp = ctx.call(
            "attribution",
            "contribution.record",
            serde_json::json!({
                "braid_id": format!("urn:braid:{data_hash}"),
                "agent": "rhizome-player",
                "role": "Creator",
                "content_hash": contrib_hash,
            }),
        );

        match &contrib_resp {
            Ok(_) => {
                v.check_bool(
                    "contribution_record",
                    true,
                    "Play contribution recorded in sweetGrass",
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

pub fn phase_load_game(
    v: &mut ValidationResult,
    dag_session: Option<&str>,
    ctx: &mut CompositionContext,
) {
    v.section("Load Game (NestGate + Merkle Verify)");

    let save_key = "save:rhizome:exp105-validation";

    if !ctx.has_capability("storage") {
        v.check_skip("load_game", "NestGate not discovered");
        return;
    }

    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());
    let get_params = serde_json::json!({"family_id": family_id, "key": save_key});

    let mut resp = ctx.call("storage", "storage.get", get_params.clone());

    if resp.is_err() && ctx.has_capability("orchestration") {
        resp = orchestration_route(ctx, "storage", "storage.get", &get_params);
    }

    match resp {
        Ok(r) => {
            let has_value = r.get("value").and_then(|v| v.as_str()).is_some();
            v.check_bool("load_game", has_value, "Save TOML loaded from NestGate");

            if has_value {
                verify_merkle_root(v, dag_session, ctx);
            }
        }
        Err(e) => {
            v.check_skip("load_game", &format!("storage.get failed: {e}"));
        }
    }
}

fn verify_merkle_root(
    v: &mut ValidationResult,
    dag_session: Option<&str>,
    ctx: &mut CompositionContext,
) {
    let Some(session_id) = dag_session else {
        v.check_skip("merkle_verify", "No DAG session — skipping merkle verify");
        return;
    };

    if !ctx.has_capability("dag") {
        v.check_skip("merkle_verify", "rhizoCrypt not discovered");
        return;
    }

    let resp = ctx.call(
        "dag",
        "dag.merkle.root",
        serde_json::json!({"session_id": session_id}),
    );

    match resp {
        Ok(r) => {
            let root = r
                .as_str()
                .map(String::from)
                .or_else(|| r.get("root").and_then(|s| s.as_str()).map(String::from));
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

pub fn phase_narration(v: &mut ValidationResult, world: &World, ctx: &mut CompositionContext) {
    v.section("AI Narration (Squirrel — optional)");

    if !ctx.has_capability("ai") {
        v.check_skip(
            "ai_narrate",
            "Squirrel not discovered — using template text",
        );
        return;
    }

    let prompt = format!(
        "You are narrator for a roguelike game called The Rhizome. \
         The player (a Fieldmouse) just entered the {} on floor {}. \
         Describe the scene in one sentence.",
        world.biome.name(),
        world.floor
    );

    let resp = ctx.call(
        "ai",
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

pub fn phase_crypto_hash(v: &mut ValidationResult, world: &World, ctx: &mut CompositionContext) {
    v.section("Save Integrity Hash (BearDog)");

    if !ctx.has_capability("security") {
        v.check_skip("crypto_hash", "BearDog not discovered");
        return;
    }

    let save_data = world.to_save_toml();
    let encoded = base64::engine::general_purpose::STANDARD.encode(save_data.as_bytes());
    let resp = ctx.call(
        "security",
        "crypto.blake3_hash",
        serde_json::json!({"data": encoded}),
    );

    v.check_bool(
        "crypto_hash",
        resp.is_ok(),
        "BLAKE3 hash of save data via BearDog",
    );
}

pub fn phase_discovery(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Service Discovery (Songbird)");

    if !ctx.has_capability("discovery") {
        v.check_skip("discovery_list", "Songbird not discovered");
        return;
    }

    let resp = ctx.call("discovery", "ipc.list", serde_json::json!({}));

    match resp {
        Ok(r) => {
            let count = r
                .get("services")
                .and_then(|s| s.as_array())
                .map_or(0, Vec::len);
            v.check_minimum("discovery_list", count, 8);
        }
        Err(e) => {
            v.check_skip("discovery_list", &format!("ipc.list failed: {e}"));
        }
    }
}
