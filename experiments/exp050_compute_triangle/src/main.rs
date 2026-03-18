// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp050: Compute Triangle — validates coralReef → toadStool → barraCuda pipeline.

use std::time::Instant;

use primalspring::cast;
use primalspring::ipc::client::PrimalClient;
use primalspring::ipc::discover::{
    DiscoverySource, discover_primal, extract_capability_names, socket_path,
};
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

fn probe_primal_with_liveness_readiness(
    name: &str,
    socket: &std::path::Path,
) -> Option<(bool, bool, u64, Vec<String>)> {
    let mut client = PrimalClient::connect(socket, name).ok()?;
    let start = Instant::now();
    let liveness = client.health_liveness().unwrap_or(false);
    let readiness = if liveness {
        client.health_readiness().unwrap_or(false)
    } else {
        false
    };
    let caps = client.capabilities().ok();
    let latency_us = cast::micros_u64(start.elapsed());
    let capability_names = extract_capability_names(caps);
    Some((liveness, readiness, latency_us, capability_names))
}

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp050 — Compute Triangle");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp050: coralReef → toadStool → barraCuda Pipeline");
    println!("{}", "=".repeat(72));

    let toadstool = discover_primal("toadstool");
    v.check_bool(
        "toadstool_discovery",
        toadstool.primal == "toadstool",
        "discover_primal returns DiscoveryResult for toadstool",
    );

    let coralreef = discover_primal("coralreef");
    let coralreef_attempted = coralreef.source == DiscoverySource::NotFound
        || coralreef.source == DiscoverySource::XdgConvention
        || coralreef.source == DiscoverySource::TempFallback
        || coralreef.source == DiscoverySource::EnvOverride;
    v.check_bool(
        "coralreef_socket_discovery_attempted",
        coralreef_attempted,
        "attempt to discover coralreef socket (may be NotFound if not running)",
    );

    let path_toadstool = socket_path("toadstool");
    let path_coralreef = socket_path("coralreef");
    let path_barracuda = socket_path("barracuda");
    let valid_paths = path_toadstool.to_string_lossy().contains("biomeos")
        && path_toadstool.to_string_lossy().contains("toadstool")
        && path_toadstool.to_string_lossy().ends_with(".sock")
        && path_coralreef.to_string_lossy().contains("biomeos")
        && path_coralreef.to_string_lossy().contains("coralreef")
        && path_barracuda.to_string_lossy().contains("biomeos")
        && path_barracuda.to_string_lossy().contains("barracuda");
    v.check_bool(
        "socket_path_valid_for_all_three",
        valid_paths,
        "socket_path returns valid-looking paths for toadstool, coralreef, barracuda",
    );

    for (primal, display_name) in [
        ("toadstool", "toadStool"),
        ("coralreef", "coralReef"),
        ("barracuda", "barraCuda"),
    ] {
        let discovery = discover_primal(primal);
        v.check_or_skip(
            &format!("{primal}_probe"),
            discovery.socket.as_ref(),
            &format!("{display_name} not reachable"),
            |path, v| {
                if let Some((liveness, readiness, latency_us, caps)) =
                    probe_primal_with_liveness_readiness(primal, path)
                {
                    v.check_bool(
                        &format!("{primal}_liveness"),
                        liveness,
                        &format!("{display_name} health.liveness"),
                    );
                    v.check_bool(
                        &format!("{primal}_readiness"),
                        readiness,
                        &format!("{display_name} health.readiness"),
                    );
                    v.check_latency(
                        &format!("{primal}_latency"),
                        latency_us,
                        tolerances::HEALTH_CHECK_MAX_US,
                    );
                    v.check_minimum(&format!("{primal}_capabilities"), caps.len(), 1);
                } else {
                    v.check_bool(
                        &format!("{primal}_connect"),
                        false,
                        &format!("{display_name} connection failed"),
                    );
                }
            },
        );
    }

    v.check_skip(
        "compile_dispatch_pipeline",
        "actual compile+dispatch pipeline needs live primals",
    );

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
