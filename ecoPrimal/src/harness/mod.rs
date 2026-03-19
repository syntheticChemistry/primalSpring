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
//! let running = AtomicHarness::start(AtomicType::Tower, "test-1")
//!     .expect("tower atomic start");
//! // primals are now running — connect via running.socket_for("security")
//! drop(running);
//! // all primals killed and sockets cleaned up
//! ```

use std::path::{Path, PathBuf};

use crate::coordination::AtomicType;
use crate::ipc::client::PrimalClient;
use crate::ipc::discover::extract_capability_names;
use crate::ipc::NeuralBridge;
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
}

impl RunningAtomic {
    /// Get the socket path for a given primal name.
    #[must_use]
    pub fn socket_for_primal(&self, primal: &str) -> Option<&PathBuf> {
        self.nucleation.get(primal, &self.family_id)
    }

    /// Connect a [`PrimalClient`] to the named primal.
    ///
    /// # Errors
    ///
    /// Returns `None` if the primal is not in this composition or if
    /// the connection fails.
    #[must_use]
    pub fn client_for(&self, primal: &str) -> Option<PrimalClient> {
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
                    .client_for(&p.name)
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
                    .client_for(&p.name)
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
    /// Records results on the provided [`ValidationResult`].
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

    /// The runtime directory used for sockets.
    #[must_use]
    pub fn runtime_dir(&self) -> &Path {
        &self.runtime_dir
    }
}

impl Drop for RunningAtomic {
    fn drop(&mut self) {
        if let Some(neural) = self.neural_api_process.take() {
            println!(
                "[harness] stopping {} (pid {})",
                neural.name,
                neural.pid()
            );
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
pub struct AtomicHarness;

impl AtomicHarness {
    /// Start all primals for an [`AtomicType`] composition.
    ///
    /// Creates an isolated runtime directory, assigns sockets via
    /// nucleation, and spawns primals in dependency order (beardog
    /// first, since all other primals depend on security).
    ///
    /// # Errors
    ///
    /// Returns [`LaunchError`] if any binary cannot be found, any
    /// process fails to spawn, or any socket times out.
    pub fn start(
        atomic: AtomicType,
        family_id: &str,
    ) -> Result<RunningAtomic, LaunchError> {
        let primals = atomic.required_primals();

        let runtime_dir = std::env::temp_dir().join(format!(
            "primalspring-harness-{}-{}",
            family_id,
            std::process::id()
        ));
        let _ = std::fs::create_dir_all(&runtime_dir);

        let mut nucleation = SocketNucleation::new(runtime_dir.clone());

        nucleation.assign_batch(primals, family_id);

        let mut processes = Vec::with_capacity(primals.len());

        for primal in primals {
            println!("[harness] starting {primal} ({}/{})", processes.len() + 1, primals.len());
            let process = launcher::spawn_primal(primal, family_id, &mut nucleation)?;
            processes.push(process);
        }

        println!(
            "[harness] {} primals running for {:?}",
            processes.len(),
            atomic
        );

        Ok(RunningAtomic {
            processes,
            neural_api_process: None,
            nucleation,
            family_id: family_id.to_owned(),
            runtime_dir,
        })
    }

    /// Start primals for an [`AtomicType`] AND the Neural API server.
    ///
    /// Primals are started first (beardog → songbird), then the Neural
    /// API server is launched. The Neural API server detects the
    /// already-running Tower and enters companion mode.
    ///
    /// `graphs_dir` should point to the directory containing deploy
    /// graph TOMLs (e.g. `primalSpring/graphs/`).
    ///
    /// # Errors
    ///
    /// Returns [`LaunchError`] if any binary cannot be found, any
    /// process fails to spawn, or any socket times out.
    pub fn start_with_neural_api(
        atomic: AtomicType,
        family_id: &str,
        graphs_dir: &Path,
    ) -> Result<RunningAtomic, LaunchError> {
        let primals = atomic.required_primals();

        let runtime_dir = std::env::temp_dir().join(format!(
            "primalspring-harness-{}-{}",
            family_id,
            std::process::id()
        ));
        let _ = std::fs::create_dir_all(&runtime_dir);

        let mut nucleation = SocketNucleation::new(runtime_dir.clone());

        nucleation.assign_batch(primals, family_id);

        let mut processes = Vec::with_capacity(primals.len());

        for primal in primals {
            println!(
                "[harness] starting {primal} ({}/{})",
                processes.len() + 1,
                primals.len()
            );
            let process = launcher::spawn_primal(primal, family_id, &mut nucleation)?;
            processes.push(process);
        }

        println!(
            "[harness] {} primals running, starting Neural API server...",
            processes.len()
        );

        let neural_api =
            launcher::spawn_neural_api(family_id, &nucleation, graphs_dir)?;

        println!(
            "[harness] Tower + Neural API running ({} primals + neural-api-server)",
            processes.len()
        );

        Ok(RunningAtomic {
            processes,
            neural_api_process: Some(neural_api),
            nucleation,
            family_id: family_id.to_owned(),
            runtime_dir,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn running_atomic_drops_cleanly_even_if_empty() {
        let dir = std::env::temp_dir().join(format!(
            "primalspring-harness-empty-{}",
            std::process::id()
        ));
        let _ = std::fs::create_dir_all(&dir);
        let nuc = SocketNucleation::new(dir.clone());
        let running = RunningAtomic {
            processes: vec![],
            neural_api_process: None,
            nucleation: nuc,
            family_id: "test".to_owned(),
            runtime_dir: dir.clone(),
        };
        assert_eq!(running.primal_count(), 0);
        drop(running);
        assert!(!dir.exists(), "runtime dir should be removed on drop");
    }
}
