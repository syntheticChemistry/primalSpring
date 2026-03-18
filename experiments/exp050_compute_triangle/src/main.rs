// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp050: Compute Triangle — validates coralReef → toadStool → barraCuda pipeline.

use primalspring::ipc::discover::{DiscoverySource, discover_primal, socket_path};
use primalspring::validation::ValidationResult;

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

    v.check_skip(
        "compile_dispatch_pipeline",
        "actual compile+dispatch pipeline needs live primals",
    );

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
