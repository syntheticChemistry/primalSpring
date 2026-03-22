// SPDX-License-Identifier: AGPL-3.0-or-later

//! Atomic test orchestration harness.
//!
//! [`AtomicHarness`] spawns a set of primals for an atomic composition,
//! respecting topological startup ordering from a deploy graph, and
//! tears them down on drop.
//!
//! # Usage
//!
//! ```rust,no_run
//! use primalspring::harness::AtomicHarness;
//! use primalspring::coordination::AtomicType;
//!
//! let running = AtomicHarness::new(AtomicType::Tower)
//!     .start("test-1")
//!     .expect("tower atomic start");
//! // primals are now running — connect via running.socket_for("security")
//! drop(running);
//! // all primals killed and sockets cleaned up
//! ```

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::coordination::AtomicType;
use crate::deploy;
use crate::ipc::NeuralBridge;
use crate::ipc::client::PrimalClient;
use crate::ipc::discover::extract_capability_names;
use crate::launcher::{self, LaunchError, PrimalProcess, SocketNucleation};

/// A running atomic composition with RAII lifecycle management.
///
/// Holds live [`PrimalProcess`] instances. When dropped, primals are
/// killed in reverse startup order and sockets are cleaned up.
pub struct RunningAtomic {
    processes: Vec<PrimalProcess>,
    neural_api_process: Option<PrimalProcess>,
    nucleation: SocketNucleation,
    family_id: String,
    runtime_dir: PathBuf,
    atomic: AtomicType,
}

impl RunningAtomic {
    /// Get the socket path for a capability (e.g. `"security"` → beardog's socket).
    ///
    /// Maps capability to primal using the composition's parallel
    /// `required_capabilities()` / `required_primals()` arrays.
    #[must_use]
    pub fn socket_for(&self, capability: &str) -> Option<&PathBuf> {
        let primal = self.capability_to_primal(capability)?;
        self.nucleation.get(primal, &self.family_id)
    }

    /// Get the socket path for a given primal name.
    #[must_use]
    pub fn socket_for_primal(&self, primal: &str) -> Option<&PathBuf> {
        self.nucleation.get(primal, &self.family_id)
    }

    /// Connect a [`PrimalClient`] to the provider of `capability`.
    ///
    /// Returns `None` if the capability is not in this composition or
    /// if the connection fails.
    #[must_use]
    pub fn client_for(&self, capability: &str) -> Option<PrimalClient> {
        let primal = self.capability_to_primal(capability)?;
        let socket = self.nucleation.get(primal, &self.family_id)?;
        PrimalClient::connect(socket, primal).ok()
    }

    /// Connect a [`PrimalClient`] to a primal by name.
    #[must_use]
    pub fn client_for_primal(&self, primal: &str) -> Option<PrimalClient> {
        let socket = self.socket_for_primal(primal)?;
        PrimalClient::connect(socket, primal).ok()
    }

    /// Run `health.liveness` on every primal in the composition.
    ///
    /// Returns a vec of `(primal_name, is_live)` tuples.
    #[must_use]
    pub fn health_check_all(&self) -> Vec<(String, bool)> {
        self.processes
            .iter()
            .map(|p| {
                let live = self
                    .client_for_primal(&p.name)
                    .and_then(|mut c| c.health_liveness().ok())
                    .unwrap_or(false);
                (p.name.clone(), live)
            })
            .collect()
    }

    /// Run `capabilities.list` on every primal and return
    /// `(primal_name, Vec<capability_name>)` tuples.
    #[must_use]
    pub fn capabilities_all(&self) -> Vec<(String, Vec<String>)> {
        self.processes
            .iter()
            .map(|p| {
                let caps = self
                    .client_for_primal(&p.name)
                    .and_then(|mut c| c.capabilities().ok())
                    .map(|v| extract_capability_names(Some(v)))
                    .unwrap_or_default();
                (p.name.clone(), caps)
            })
            .collect()
    }

    /// Validate the composition: liveness + capabilities for every primal.
    ///
    /// Liveness is required (fail if not live). Capabilities are best-effort
    /// (skip if the primal doesn't implement `capabilities.list`).
    /// When a Neural API is running, also validates the Neural API bridge.
    /// Records results on the provided [`crate::validation::ValidationResult`].
    pub fn validate(&self, v: &mut crate::validation::ValidationResult) {
        for (name, live) in self.health_check_all() {
            v.check_bool(
                &format!("{name}_liveness"),
                live,
                &format!("{name} health.liveness"),
            );
        }
        for (name, caps) in self.capabilities_all() {
            if caps.is_empty() {
                v.check_skip(
                    &format!("{name}_capabilities"),
                    &format!("{name} does not implement capabilities.list"),
                );
            } else {
                v.check_minimum(&format!("{name}_capabilities"), caps.len(), 1);
            }
        }
        if let Some(bridge) = self.neural_bridge() {
            let neural_ok = bridge.health_check().is_ok();
            v.check_bool("neural_api_health", neural_ok, "Neural API health check");
        }
    }

    /// Whether the Neural API server is running in this composition.
    #[must_use]
    pub const fn has_neural_api(&self) -> bool {
        self.neural_api_process.is_some()
    }

    /// Get a [`NeuralBridge`] to the running Neural API server.
    ///
    /// Returns `None` if no Neural API was started with this composition.
    #[must_use]
    pub fn neural_bridge(&self) -> Option<NeuralBridge> {
        let proc = self.neural_api_process.as_ref()?;
        let socket_str = proc.socket_path.to_string_lossy();
        NeuralBridge::discover_with(Some(&socket_str), Some(&self.family_id))
    }

    /// Number of running primals (excluding Neural API server).
    #[must_use]
    pub const fn primal_count(&self) -> usize {
        self.processes.len()
    }

    /// Collect all child PIDs (primals + optional Neural API server).
    #[must_use]
    pub fn pids(&self) -> Vec<u32> {
        let mut pids: Vec<u32> = self.processes.iter().map(PrimalProcess::pid).collect();
        if let Some(ref neural) = self.neural_api_process {
            pids.push(neural.pid());
        }
        pids
    }

    /// The runtime directory used for sockets.
    #[must_use]
    pub fn runtime_dir(&self) -> &Path {
        &self.runtime_dir
    }

    /// The atomic type of this composition.
    #[must_use]
    pub const fn atomic_type(&self) -> AtomicType {
        self.atomic
    }

    /// Map a capability name to the primal that provides it in this composition.
    fn capability_to_primal(&self, capability: &str) -> Option<&'static str> {
        let caps = self.atomic.required_capabilities();
        let primals = self.atomic.required_primals();
        caps.iter()
            .zip(primals.iter())
            .find(|&(cap, _)| *cap == capability)
            .map(|(_, &primal)| primal)
    }
}

impl Drop for RunningAtomic {
    fn drop(&mut self) {
        if let Some(neural) = self.neural_api_process.take() {
            println!("[harness] stopping {} (pid {})", neural.name, neural.pid());
            drop(neural);
        }
        while let Some(process) = self.processes.pop() {
            println!(
                "[harness] stopping {} (pid {})",
                process.name,
                process.pid()
            );
            drop(process);
        }
        let _ = std::fs::remove_dir_all(&self.runtime_dir);
    }
}

/// Harness for spawning and managing atomic compositions.
///
/// Constructed with an [`AtomicType`] and an optional deploy graph path.
/// When a graph path is provided, [`start`](Self::start) uses
/// [`topological_waves`](crate::deploy::topological_waves) to determine
/// startup ordering. Without a graph, primals start in the static order
/// from [`AtomicType::required_primals`].
pub struct AtomicHarness {
    atomic: AtomicType,
    graph_path: Option<PathBuf>,
}

impl AtomicHarness {
    /// Create a harness for the given composition (no graph-driven ordering).
    #[must_use]
    pub const fn new(atomic: AtomicType) -> Self {
        Self {
            atomic,
            graph_path: None,
        }
    }

    /// Create a harness with graph-driven topological startup ordering.
    ///
    /// `graph_path` should point to a deploy graph TOML (e.g.
    /// `graphs/tower_atomic_bootstrap.toml`). The graph's
    /// `topological_waves()` determines startup order; only primals in
    /// [`AtomicType::required_primals`] are actually spawned.
    #[must_use]
    pub fn with_graph(atomic: AtomicType, graph_path: impl AsRef<Path>) -> Self {
        Self {
            atomic,
            graph_path: Some(graph_path.as_ref().to_path_buf()),
        }
    }

    /// Start all primals for this composition.
    ///
    /// Creates an isolated runtime directory, assigns sockets via
    /// nucleation, and spawns primals in dependency order — either from
    /// topological waves (when a graph was provided) or from the static
    /// [`AtomicType::required_primals`] ordering.
    ///
    /// # Errors
    ///
    /// Returns [`LaunchError`] if any binary cannot be found, any
    /// process fails to spawn, or any socket times out.
    pub fn start(&self, family_id: &str) -> Result<RunningAtomic, LaunchError> {
        let spawn_order = self.compute_spawn_order()?;

        let runtime_dir = std::env::temp_dir().join(format!(
            "primalspring-harness-{}-{}",
            family_id,
            std::process::id()
        ));
        let _ = std::fs::create_dir_all(&runtime_dir);

        let mut nucleation = SocketNucleation::new(runtime_dir.clone());

        let primal_refs: Vec<&str> = spawn_order.iter().map(String::as_str).collect();
        nucleation.assign_batch(&primal_refs, family_id);

        let mut processes = Vec::with_capacity(spawn_order.len());

        for primal in &spawn_order {
            println!(
                "[harness] starting {primal} ({}/{})",
                processes.len() + 1,
                spawn_order.len()
            );
            let process = launcher::spawn_primal(primal, family_id, &mut nucleation)?;
            processes.push(process);
        }

        println!(
            "[harness] {} primals running for {:?}",
            processes.len(),
            self.atomic
        );

        Ok(RunningAtomic {
            processes,
            neural_api_process: None,
            nucleation,
            family_id: family_id.to_owned(),
            runtime_dir,
            atomic: self.atomic,
        })
    }

    /// Start primals for this composition AND the Neural API server.
    ///
    /// Primals are started first (in topological or static order), then
    /// the Neural API server is launched. The Neural API server detects
    /// the already-running primals and enters companion mode.
    ///
    /// `graphs_dir` should point to the directory containing deploy
    /// graph TOMLs (e.g. `primalSpring/graphs/`).
    ///
    /// # Errors
    ///
    /// Returns [`LaunchError`] if any binary cannot be found, any
    /// process fails to spawn, or any socket times out.
    pub fn start_with_neural_api(
        &self,
        family_id: &str,
        graphs_dir: &Path,
    ) -> Result<RunningAtomic, LaunchError> {
        let spawn_order = self.compute_spawn_order()?;

        let runtime_dir = std::env::temp_dir().join(format!(
            "primalspring-harness-{}-{}",
            family_id,
            std::process::id()
        ));
        let _ = std::fs::create_dir_all(&runtime_dir);

        let mut nucleation = SocketNucleation::new(runtime_dir.clone());

        let primal_refs: Vec<&str> = spawn_order.iter().map(String::as_str).collect();
        nucleation.assign_batch(&primal_refs, family_id);

        let mut processes = Vec::with_capacity(spawn_order.len());

        for primal in &spawn_order {
            println!(
                "[harness] starting {primal} ({}/{})",
                processes.len() + 1,
                spawn_order.len()
            );
            let process = launcher::spawn_primal(primal, family_id, &mut nucleation)?;
            processes.push(process);
        }

        println!(
            "[harness] {} primals running, starting Neural API server...",
            processes.len()
        );

        let neural_api = launcher::spawn_neural_api(family_id, &nucleation, graphs_dir)?;

        println!(
            "[harness] {:?} + Neural API running ({} primals + neural-api-server)",
            self.atomic,
            processes.len()
        );

        Ok(RunningAtomic {
            processes,
            neural_api_process: Some(neural_api),
            nucleation,
            family_id: family_id.to_owned(),
            runtime_dir,
            atomic: self.atomic,
        })
    }

    /// Determine spawn order: graph-driven topological waves when a graph
    /// path was provided, otherwise the static `required_primals()` order.
    ///
    /// When using topological waves, only primals in `required_primals()`
    /// are included (graph may contain validation nodes we don't spawn).
    /// Any required primals missing from the graph are appended at the end.
    fn compute_spawn_order(&self) -> Result<Vec<String>, LaunchError> {
        let required: Vec<&str> = self.atomic.required_primals().to_vec();

        let Some(ref graph_path) = self.graph_path else {
            return Ok(required.iter().map(|s| (*s).to_owned()).collect());
        };

        let graph = deploy::load_graph(graph_path).map_err(|e| {
            LaunchError::ProfileParseError(format!("deploy graph {}: {e}", graph_path.display()))
        })?;

        let waves = deploy::topological_waves(&graph).map_err(|e| {
            LaunchError::ProfileParseError(format!(
                "topological sort of {}: {e}",
                graph_path.display()
            ))
        })?;

        let required_set: HashSet<&str> = required.iter().copied().collect();
        let mut ordered: Vec<String> = waves
            .into_iter()
            .flatten()
            .filter(|name| required_set.contains(name.as_str()))
            .collect();

        for &r in &required {
            if !ordered.iter().any(|o| o == r) {
                ordered.push(r.to_owned());
            }
        }

        Ok(ordered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn running_atomic_drops_cleanly_even_if_empty() {
        let dir =
            std::env::temp_dir().join(format!("primalspring-harness-empty-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let nuc = SocketNucleation::new(dir.clone());
        let running = RunningAtomic {
            processes: vec![],
            neural_api_process: None,
            nucleation: nuc,
            family_id: "test".to_owned(),
            runtime_dir: dir.clone(),
            atomic: AtomicType::Tower,
        };
        assert_eq!(running.primal_count(), 0);
        drop(running);
        assert!(!dir.exists(), "runtime dir should be removed on drop");
    }

    #[test]
    fn capability_to_primal_mapping() {
        let dir =
            std::env::temp_dir().join(format!("primalspring-harness-cap-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let nuc = SocketNucleation::new(dir.clone());
        let running = RunningAtomic {
            processes: vec![],
            neural_api_process: None,
            nucleation: nuc,
            family_id: "test".to_owned(),
            runtime_dir: dir,
            atomic: AtomicType::Tower,
        };
        assert_eq!(running.capability_to_primal("security"), Some("beardog"));
        assert_eq!(running.capability_to_primal("discovery"), Some("songbird"));
        assert_eq!(running.capability_to_primal("nonexistent"), None);
        drop(running);
    }

    #[test]
    fn compute_spawn_order_without_graph() {
        let harness = AtomicHarness::new(AtomicType::Tower);
        let order = harness.compute_spawn_order().unwrap();
        assert_eq!(order, vec!["beardog", "songbird"]);
    }

    #[test]
    fn compute_spawn_order_with_graph() {
        let graph_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs/tower_atomic_bootstrap.toml");
        let harness = AtomicHarness::with_graph(AtomicType::Tower, &graph_path);
        let order = harness.compute_spawn_order().unwrap();
        assert!(
            order.contains(&"beardog".to_owned()),
            "should include beardog"
        );
        assert!(
            order.contains(&"songbird".to_owned()),
            "should include songbird"
        );
        let beardog_pos = order.iter().position(|n| n == "beardog").unwrap();
        let songbird_pos = order.iter().position(|n| n == "songbird").unwrap();
        assert!(
            beardog_pos < songbird_pos,
            "beardog should start before songbird (topological order)"
        );
    }

    #[test]
    fn compute_spawn_order_node_with_graph() {
        let graph_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs/node_atomic_compute.toml");
        let harness = AtomicHarness::with_graph(AtomicType::Node, &graph_path);
        let order = harness.compute_spawn_order().unwrap();
        assert_eq!(order.len(), 3, "Node = beardog + songbird + toadstool");
        assert!(order.contains(&"beardog".to_owned()));
        assert!(order.contains(&"songbird".to_owned()));
        assert!(order.contains(&"toadstool".to_owned()));
    }

    #[test]
    fn harness_new_creates_without_graph() {
        let harness = AtomicHarness::new(AtomicType::Tower);
        assert!(harness.graph_path.is_none());
    }

    #[test]
    fn harness_with_graph_stores_path() {
        let harness = AtomicHarness::with_graph(AtomicType::Tower, "/tmp/test.toml");
        assert_eq!(
            harness.graph_path.as_deref(),
            Some(Path::new("/tmp/test.toml"))
        );
    }
}
