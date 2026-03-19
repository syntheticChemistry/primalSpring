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
use crate::launcher::{self, LaunchError, PrimalProcess, SocketNucleation};

/// A running atomic composition with RAII lifecycle management.
///
/// Holds live [`PrimalProcess`] instances. When dropped, primals are
/// killed in reverse startup order and sockets are cleaned up.
pub struct RunningAtomic {
    processes: Vec<PrimalProcess>,
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
            v.check_minimum(&format!("{name}_capabilities"), caps.len(), 1);
        }
    }

    /// Number of running primals.
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
            nucleation: nuc,
            family_id: "test".to_owned(),
            runtime_dir: dir.clone(),
        };
        assert_eq!(running.primal_count(), 0);
        drop(running);
        assert!(!dir.exists(), "runtime dir should be removed on drop");
    }
}
