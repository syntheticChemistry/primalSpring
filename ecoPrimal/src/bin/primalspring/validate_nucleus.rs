// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! NUCLEUS deployment validation gate — pure Rust replacement for
//! `validate_nucleus_deployment.sh`.
//!
//! Validates a live NUCLEUS launch from the plasmidBin depot using the
//! library's existing discovery, health-check, and launcher infrastructure.

#![forbid(unsafe_code)]

use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use primalspring::launcher::discover_binary;
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

const EXPECTED_PRIMALS: &[&str] = &[
    primal_names::BEARDOG,
    primal_names::SONGBIRD,
    primal_names::SKUNKBAT,
    primal_names::TOADSTOOL,
    primal_names::BARRACUDA,
    primal_names::CORALREEF,
    primal_names::NESTGATE,
    primal_names::RHIZOCRYPT,
    primal_names::LOAMSPINE,
    primal_names::SWEETGRASS,
    primal_names::SQUIRREL,
    primal_names::PETALTONGUE,
];

const FEDERATION_PORT: u16 = 7700;
const LAUNCH_TIMEOUT: Duration = Duration::from_secs(90);
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(15);

struct NucleusGate {
    v: ValidationResult,
    depot_dir: PathBuf,
    socket_dir: PathBuf,
    gate_name: String,
    full_mode: bool,
    skip_launch: bool,
    json_output: bool,
    nucleus_child: Option<Child>,
}

pub struct GateArgs {
    pub full: bool,
    pub skip_launch: bool,
    pub json: bool,
}

pub fn run(args: GateArgs) {
    let mut gate = NucleusGate::new(args);
    gate.tier0_preflight();
    gate.tier1_launch();
    gate.tier2_health();
    gate.tier3_federation();
    if gate.full_mode {
        gate.tier4_lifecycle();
    }
    gate.summary();
}

impl NucleusGate {
    fn new(args: GateArgs) -> Self {
        let depot_dir = resolve_depot_dir();
        let socket_dir = resolve_socket_dir();
        let gate_name = std::env::var("GATE_NAME")
            .unwrap_or_else(|_| {
                Command::new("hostname").arg("-s").output()
                    .ok()
                    .and_then(|o| String::from_utf8(o.stdout).ok())
                    .map(|s| s.trim().to_owned())
                    .unwrap_or_else(|| "unknown".into())
            });

        let v = ValidationResult::new("NUCLEUS Deployment Validation Gate");

        Self {
            v,
            depot_dir,
            socket_dir,
            gate_name,
            full_mode: args.full,
            skip_launch: args.skip_launch,
            json_output: args.json,
            nucleus_child: None,
        }
    }

    fn tier0_preflight(&mut self) {
        self.v.section("Tier 0: Pre-flight");

        self.v.check_bool(
            "depot-exists",
            self.depot_dir.is_dir(),
            &format!("depot directory: {}", self.depot_dir.display()),
        );
        if !self.depot_dir.is_dir() {
            return;
        }

        let mut present = 0u32;
        let mut missing = Vec::new();
        for &p in EXPECTED_PRIMALS {
            let bin = self.depot_dir.join(p);
            if bin.is_file() {
                present += 1;
            } else {
                missing.push(p);
            }
        }
        self.v.check_bool(
            "depot-binary-count",
            present >= EXPECTED_PRIMALS.len() as u32,
            &format!("{present}/{} primals in depot", EXPECTED_PRIMALS.len()),
        );
        if !missing.is_empty() {
            self.v.check_bool(
                "depot-missing-primals",
                false,
                &format!("missing: {}", missing.join(", ")),
            );
        }

        let biomeos = self.depot_dir.join(primal_names::BIOMEOS);
        self.v.check_bool(
            "biomeos-executable",
            biomeos.is_file() && is_executable(&biomeos),
            "biomeos binary executable",
        );

        let checksums = self.depot_dir.parent()
            .and_then(|p| p.parent())
            .map(|root| root.join("checksums.toml"))
            .unwrap_or_else(|| self.depot_dir.join("../../checksums.toml"));
        self.v.check_bool(
            "checksums-present",
            checksums.is_file(),
            "checksums.toml present",
        );

        if !self.socket_dir.exists() {
            let _ = std::fs::create_dir_all(&self.socket_dir);
        }
        self.v.check_bool(
            "socket-dir-exists",
            self.socket_dir.is_dir(),
            &format!("socket directory: {}", self.socket_dir.display()),
        );
    }

    fn tier1_launch(&mut self) {
        self.v.section("Tier 1: NUCLEUS Launch");

        if self.skip_launch {
            self.v.check_bool("launch-skipped", true, "skipped (--skip-launch)");
            return;
        }

        kill_existing_nucleus();
        clean_socket_dir(&self.socket_dir);

        let biomeos_path = match discover_binary(primal_names::BIOMEOS) {
            Ok(p) => p,
            Err(e) => {
                self.v.check_bool("biomeos-discovered", false, &format!("{e}"));
                return;
            }
        };

        let family_seed = std::env::var("FAMILY_SEED").unwrap_or_else(|_| {
            let seed_path = dirs_home().join(".family.seed");
            std::fs::read_to_string(&seed_path).unwrap_or_default().trim().to_owned()
        });

        let mut cmd = Command::new(&biomeos_path);
        cmd.args(["nucleus", "start", "--node-id", &self.gate_name])
            .env("ECOPRIMALS_ROOT", std::env::var("ECOPRIMALS_ROOT").unwrap_or_default())
            .env("ECOPRIMALS_PLASMID_BIN", self.depot_dir.to_str().unwrap_or(""))
            .env("GATE_NAME", &self.gate_name)
            .env("FAMILY_SEED", &family_seed)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                self.v.check_bool("nucleus-spawn", false, &format!("spawn failed: {e}"));
                return;
            }
        };

        let pid = child.id();
        self.v.check_bool("nucleus-spawn", true, &format!("PID {pid}"));

        self.nucleus_child = Some(child);

        let start = Instant::now();
        let mut neural_up = false;
        while start.elapsed() < LAUNCH_TIMEOUT {
            if let Some(ref mut child) = self.nucleus_child {
                if let Ok(Some(status)) = child.try_wait() {
                    self.v.check_bool(
                        "nucleus-alive-during-startup",
                        false,
                        &format!("exited prematurely with {status}"),
                    );
                    self.nucleus_child = None;
                    return;
                }
            }
            std::thread::sleep(Duration::from_secs(2));

            if self.socket_dir.join("neural-api.sock").exists() {
                neural_up = true;
                break;
            }
        }

        self.v.check_bool(
            "neural-api-startup",
            neural_up,
            &format!("Neural API socket appeared in {:.0}s", start.elapsed().as_secs_f64()),
        );

        std::thread::sleep(Duration::from_secs(5));

        if let Some(ref mut child) = self.nucleus_child {
            match child.try_wait() {
                Ok(Some(status)) => {
                    self.v.check_bool(
                        "nucleus-stable-after-startup",
                        false,
                        &format!("BIO-ORPHAN-01: exited with {status} after startup"),
                    );
                    self.nucleus_child = None;
                }
                Ok(None) => {
                    self.v.check_bool("nucleus-stable-after-startup", true, &format!("PID {pid} alive"));
                }
                Err(e) => {
                    self.v.check_bool(
                        "nucleus-stable-after-startup",
                        false,
                        &format!("wait error: {e}"),
                    );
                }
            }
        }
    }

    fn tier2_health(&mut self) {
        self.v.section("Tier 2: Primal Health");

        let sockets = list_sockets(&self.socket_dir);
        self.v.check_bool(
            "socket-count",
            !sockets.is_empty(),
            &format!("{} socket(s) in {}", sockets.len(), self.socket_dir.display()),
        );

        let mut health_pass = 0u32;
        let mut health_fail = 0u32;

        for sock_path in &sockets {
            let name = sock_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");

            let result = primalspring::coordination::probe_primal_at_socket(name, sock_path);
            if result.health_ok {
                health_pass += 1;
                self.v.check_bool(
                    &format!("health-{name}"),
                    true,
                    &format!("{name}: alive ({}µs)", result.latency_us),
                );
            } else if name == "neural-api" {
                self.v.check_bool(
                    &format!("health-{name}"),
                    true,
                    &format!("{name}: API socket (no health.liveness — normal)"),
                );
            } else {
                health_fail += 1;
                self.v.check_bool(
                    &format!("health-{name}"),
                    false,
                    &format!("{name}: unreachable"),
                );
            }
        }

        self.v.check_bool(
            "health-summary",
            health_fail == 0,
            &format!("{health_pass} pass, {health_fail} fail"),
        );
    }

    fn tier3_federation(&mut self) {
        self.v.section("Tier 3: Federation & Mesh Readiness");

        let fed_live = TcpStream::connect_timeout(
            &format!("127.0.0.1:{FEDERATION_PORT}").parse().unwrap(),
            Duration::from_secs(3),
        )
        .is_ok();

        self.v.check_bool(
            "federation-port",
            fed_live,
            &format!("songbird :{FEDERATION_PORT} {}", if fed_live { "LIVE" } else { "NOT listening" }),
        );
    }

    fn tier4_lifecycle(&mut self) {
        self.v.section("Tier 4: Lifecycle (graceful shutdown + restart)");

        if let Some(ref mut child) = self.nucleus_child.take() {
            let pid = child.id();

            #[cfg(unix)]
            {
                use nix::sys::signal::{Signal, kill};
                use nix::unistd::Pid;
                let _ = kill(Pid::from_raw(pid as i32), Signal::SIGTERM);
            }
            #[cfg(not(unix))]
            {
                let _ = child.kill();
            }

            let start = Instant::now();
            let clean_exit = loop {
                if start.elapsed() > SHUTDOWN_TIMEOUT {
                    break false;
                }
                match child.try_wait() {
                    Ok(Some(_)) => break true,
                    _ => std::thread::sleep(Duration::from_millis(500)),
                }
            };

            self.v.check_bool(
                "graceful-shutdown",
                clean_exit,
                &format!(
                    "NUCLEUS PID {pid} {} in {:.0}s",
                    if clean_exit { "exited cleanly" } else { "did NOT exit" },
                    start.elapsed().as_secs_f64()
                ),
            );

            if !clean_exit {
                let _ = child.kill();
                let _ = child.wait();
            }

            std::thread::sleep(Duration::from_secs(2));

            let orphans = count_depot_processes(&self.depot_dir);
            self.v.check_bool(
                "orphan-check",
                orphans == 0,
                &format!("{orphans} primal process(es) survived shutdown"),
            );

            if orphans > 0 {
                kill_depot_processes(&self.depot_dir);
            }
        } else {
            self.v.check_bool("lifecycle-skipped", true, "no NUCLEUS to test (not launched by us)");
        }
    }

    fn summary(&mut self) {
        self.v.section("RESULT");

        if self.json_output {
            if let Ok(j) = self.v.to_json() {
                println!("{j}");
                return;
            }
        }

        self.v.finish();

        if let Some(ref mut child) = self.nucleus_child.take() {
            let pid = child.id();
            eprintln!("NUCLEUS left running (PID {pid}). Stop with: kill {pid}");
        }

        std::process::exit(self.v.exit_code());
    }
}

impl Drop for NucleusGate {
    fn drop(&mut self) {
        if !self.full_mode {
            if let Some(ref mut child) = self.nucleus_child.take() {
                let pid = child.id();
                eprintln!("NUCLEUS left running (PID {pid}). Stop with: kill {pid}");
            }
        }
    }
}

fn resolve_depot_dir() -> PathBuf {
    if let Ok(bin) = std::env::var("ECOPRIMALS_PLASMID_BIN") {
        return PathBuf::from(bin);
    }
    if let Ok(root) = std::env::var("ECOPRIMALS_ROOT") {
        let triple = host_triple();
        let depot = PathBuf::from(&root).join("infra/plasmidBin/primals").join(&triple);
        if depot.is_dir() {
            return depot;
        }
        let flat = PathBuf::from(&root).join("infra/plasmidBin/primals");
        if flat.is_dir() {
            return flat;
        }
    }
    PathBuf::from("/opt/ecoPrimals/plasmidBin/primals")
}

fn resolve_socket_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("BIOMEOS_SOCKET_DIR") {
        return PathBuf::from(dir);
    }
    #[cfg(unix)]
    {
        let uid = nix::unistd::getuid();
        PathBuf::from(format!("/run/user/{}/biomeos", uid))
    }
    #[cfg(not(unix))]
    {
        std::env::temp_dir().join(primalspring::env_keys::BIOMEOS_SUBDIR)
    }
}

fn host_triple() -> String {
    let arch = std::env::consts::ARCH;
    let os = std::env::consts::OS;
    match os {
        "linux" => format!("{arch}-unknown-linux-musl"),
        "macos" => format!("{arch}-apple-darwin"),
        _ => format!("{arch}-unknown-{os}"),
    }
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/root"))
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    path.metadata()
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}

fn list_sockets(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    entries
        .flatten()
        .filter(|e| {
            e.path().extension().and_then(|s| s.to_str()) == Some("sock")
                && e.path().exists()
        })
        .map(|e| e.path())
        .collect()
}

fn kill_existing_nucleus() {
    #[cfg(unix)]
    {
        let output = Command::new("pgrep")
            .args(["-f", "biomeos nucleus"])
            .output();
        if let Ok(out) = output {
            let pids: Vec<u32> = String::from_utf8_lossy(&out.stdout)
                .lines()
                .filter_map(|l| l.trim().parse().ok())
                .collect();
            let had_pids = !pids.is_empty();
            for pid in pids {
                let _ = nix::sys::signal::kill(
                    nix::unistd::Pid::from_raw(pid as i32),
                    nix::sys::signal::Signal::SIGTERM,
                );
            }
            if had_pids {
                std::thread::sleep(Duration::from_secs(2));
            }
        }
    }
}

fn clean_socket_dir(dir: &Path) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "sock" || ext == "pid" || ext == "json" {
                    let _ = std::fs::remove_file(&path);
                }
            }
        }
    }
}

fn count_depot_processes(depot: &Path) -> usize {
    let depot_str = depot.to_string_lossy();
    Command::new("pgrep")
        .args(["-f", &depot_str])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).lines().count())
        .unwrap_or(0)
}

fn kill_depot_processes(depot: &Path) {
    let depot_str = depot.to_string_lossy();
    let _ = Command::new("pkill").args(["-f", &*depot_str]).status();
    std::thread::sleep(Duration::from_secs(2));
    let _ = Command::new("pkill").args(["-9", "-f", &*depot_str]).status();
}
