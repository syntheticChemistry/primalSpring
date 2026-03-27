// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp078: petalTongue Viz Surface — validate the universal UI primal
//! integration with biomeOS substrate.
//!
//! Connects to a running petalTongue instance (discovered via standard
//! socket or `PETALTONGUE_SOCKET`) and validates the `visualization.*`,
//! `ui.*`, and `interaction.*` capability domains.
//!
//! Expects:
//! - petalTongue running (discovered via socket convention)
//! - Optional: biomeOS neural-api running for capability routing

use primalspring::ipc::NeuralBridge;
use primalspring::ipc::client::PrimalClient;
use primalspring::ipc::discover;
use primalspring::validation::ValidationResult;

fn validate_petaltongue_direct(v: &mut ValidationResult) -> bool {
    let result = discover::discover_primal("petaltongue");
    let Some(socket) = result.socket else {
        v.check_skip(
            "petaltongue_direct",
            "petalTongue not discovered — start petaltongue server",
        );
        return false;
    };

    let mut client = match PrimalClient::connect(&socket, "petaltongue") {
        Ok(c) => c,
        Err(e) => {
            v.check_bool("petaltongue_connect", false, &format!("{e}"));
            return false;
        }
    };

    let health = client.health_check().unwrap_or(false);
    v.check_bool("petaltongue_health", health, "petalTongue health check");

    let caps = client.capabilities();
    v.check_bool(
        "petaltongue_capabilities",
        caps.is_ok(),
        "petalTongue capabilities.list",
    );

    health
}

fn validate_viz_domains(v: &mut ValidationResult) {
    let Some(bridge) = NeuralBridge::discover() else {
        v.check_skip(
            "viz_biomeos_routing",
            "biomeOS not running — visualization routing not tested",
        );
        return;
    };

    for domain in &["visualization", "ui", "interaction"] {
        let result = bridge.discover_capability(domain);
        let key = format!("{domain}_domain");
        match result {
            Ok(val) => {
                let registered = val.get("primary_socket").is_some();
                v.check_bool(
                    &key,
                    registered,
                    &format!("{domain} domain registered in biomeOS"),
                );
            }
            Err(_) => v.check_skip(&key, &format!("{domain} domain not yet registered")),
        }
    }
}

fn validate_tower_ai_viz_graph(v: &mut ValidationResult) {
    let graph_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../graphs/tower_ai_viz.toml");

    v.check_bool(
        "tower_ai_viz_file",
        graph_path.exists(),
        "graphs/tower_ai_viz.toml exists",
    );

    let Some(bridge) = NeuralBridge::discover() else {
        v.check_skip(
            "tower_ai_viz_loaded",
            "biomeOS not running — graph validation via Neural API skipped",
        );
        return;
    };

    let mut client = match PrimalClient::connect(bridge.socket_path(), "biomeos") {
        Ok(c) => c,
        Err(e) => {
            v.check_bool("biomeos_connect", false, &format!("{e}"));
            return;
        }
    };

    let resp = client.call("graph.list", serde_json::json!({}));
    let graphs: Vec<serde_json::Value> = match resp {
        Ok(r) => r
            .result
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default(),
        Err(e) => {
            v.check_bool("graph_list", false, &format!("{e}"));
            return;
        }
    };

    let ui_graph = graphs
        .iter()
        .find(|g| g.get("id").and_then(|i| i.as_str()) == Some("ui_atomic"));
    v.check_bool(
        "tower_ai_viz_loaded",
        ui_graph.is_some(),
        "ui_atomic graph loaded in biomeOS",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp078 — petalTongue Viz Surface")
        .with_provenance("exp078_petaltongue_viz_surface", "2026-03-27")
        .run(
            "primalSpring Exp078: Universal UI primal integration with biomeOS substrate",
            |v| {
                let live = validate_petaltongue_direct(v);
                if live {
                    validate_viz_domains(v);
                }
                validate_tower_ai_viz_graph(v);
            },
        );
}
