// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Multi-gate mesh awareness for composition validation.
//!
//! Now that the mesh is live, compositions span multiple gates. This module
//! models which primals are deployed on which gates, enabling validation to
//! verify cross-gate capability routing paths and detect deployment gaps.
//!
//! # Design
//!
//! A [`MeshTopology`] is built from live `discovery.peers` data combined with
//! the gate manifest. It answers questions like:
//!
//! - Which gate hosts a given capability?
//! - Is a cross-gate route available for a capability.call?
//! - Which capabilities are unreachable due to mesh gaps?

use std::collections::{BTreeMap, BTreeSet};

/// A gate in the mesh with its assigned primals and capabilities.
#[derive(Debug, Clone)]
pub struct GateNode {
    /// Gate identifier (e.g., "east-gate", "strand-gate").
    pub gate_id: String,
    /// Network address (host:port for Songbird federation).
    pub address: Option<String>,
    /// Whether this gate is reachable via `mesh.health_check`.
    pub healthy: bool,
    /// Primals deployed on this gate.
    pub primals: BTreeSet<String>,
    /// Capabilities available from this gate.
    pub capabilities: BTreeSet<String>,
}

/// Cross-gate capability routing path.
#[derive(Debug, Clone)]
pub struct MeshRoute {
    /// The capability being routed.
    pub capability: String,
    /// Source gate (requester).
    pub from_gate: String,
    /// Destination gate (provider).
    pub to_gate: String,
    /// Whether the route is currently healthy.
    pub healthy: bool,
}

/// Multi-gate mesh topology model.
///
/// Built from live discovery data and gate manifests. Supports both
/// structural validation (which gates SHOULD have which capabilities) and
/// live validation (which gates DO respond).
#[derive(Debug, Clone, Default)]
pub struct MeshTopology {
    /// All known gates in the mesh.
    gates: BTreeMap<String, GateNode>,
    /// The local gate identity.
    local_gate: Option<String>,
}

impl MeshTopology {
    /// Create a new empty topology.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the local gate identity.
    pub fn set_local_gate(&mut self, gate_id: impl Into<String>) {
        self.local_gate = Some(gate_id.into());
    }

    /// Register a gate with its capabilities.
    pub fn register_gate(
        &mut self,
        gate_id: impl Into<String>,
        address: Option<String>,
        primals: impl IntoIterator<Item = impl Into<String>>,
        capabilities: impl IntoIterator<Item = impl Into<String>>,
    ) {
        let id = gate_id.into();
        let node = GateNode {
            gate_id: id.clone(),
            address,
            healthy: false,
            primals: primals.into_iter().map(Into::into).collect(),
            capabilities: capabilities.into_iter().map(Into::into).collect(),
        };
        self.gates.insert(id, node);
    }

    /// Mark a gate as healthy (reachable via mesh).
    pub fn mark_healthy(&mut self, gate_id: &str, healthy: bool) {
        if let Some(node) = self.gates.get_mut(gate_id) {
            node.healthy = healthy;
        }
    }

    /// Find which gate provides a capability.
    ///
    /// Returns the first healthy gate that advertises the capability,
    /// preferring the local gate if it has the capability.
    #[must_use]
    pub fn resolve_capability(&self, capability: &str) -> Option<&GateNode> {
        if let Some(local_id) = &self.local_gate {
            if let Some(local) = self.gates.get(local_id) {
                if local.capabilities.contains(capability) {
                    return Some(local);
                }
            }
        }
        self.gates
            .values()
            .find(|g| g.healthy && g.capabilities.contains(capability))
    }

    /// All capabilities reachable from the mesh (union of all healthy gates).
    #[must_use]
    pub fn reachable_capabilities(&self) -> BTreeSet<&str> {
        self.gates
            .values()
            .filter(|g| g.healthy)
            .flat_map(|g| g.capabilities.iter().map(String::as_str))
            .collect()
    }

    /// Capabilities that exist in the manifest but have no healthy provider.
    #[must_use]
    pub fn unreachable_capabilities(&self) -> BTreeSet<&str> {
        let all: BTreeSet<&str> = self
            .gates
            .values()
            .flat_map(|g| g.capabilities.iter().map(String::as_str))
            .collect();
        let reachable = self.reachable_capabilities();
        all.difference(&reachable).copied().collect()
    }

    /// Compute all cross-gate routes needed for a given capability.
    #[must_use]
    pub fn routes_for_capability(&self, capability: &str) -> Vec<MeshRoute> {
        let providers: Vec<&str> = self
            .gates
            .values()
            .filter(|g| g.capabilities.contains(capability))
            .map(|g| g.gate_id.as_str())
            .collect();

        let requesters: Vec<&str> = self
            .gates
            .keys()
            .filter(|id| !providers.contains(&id.as_str()))
            .map(String::as_str)
            .collect();

        let mut routes = Vec::new();
        for from in &requesters {
            for to in &providers {
                let from_healthy = self
                    .gates
                    .get(*from)
                    .is_some_and(|g| g.healthy);
                let to_healthy = self
                    .gates
                    .get(*to)
                    .is_some_and(|g| g.healthy);
                routes.push(MeshRoute {
                    capability: capability.to_owned(),
                    from_gate: (*from).to_owned(),
                    to_gate: (*to).to_owned(),
                    healthy: from_healthy && to_healthy,
                });
            }
        }
        routes
    }

    /// Number of gates in the topology.
    #[must_use]
    pub fn gate_count(&self) -> usize {
        self.gates.len()
    }

    /// Number of healthy gates.
    #[must_use]
    pub fn healthy_gate_count(&self) -> usize {
        self.gates.values().filter(|g| g.healthy).count()
    }

    /// All gate nodes.
    #[must_use]
    pub const fn gates(&self) -> &BTreeMap<String, GateNode> {
        &self.gates
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_topology() -> MeshTopology {
        let mut topo = MeshTopology::new();
        topo.set_local_gate("east-gate");
        topo.register_gate(
            "east-gate",
            Some("192.168.1.144:7700".to_owned()),
            ["beardog", "songbird", "nestgate"],
            ["security", "discovery", "storage"],
        );
        topo.register_gate(
            "strand-gate",
            Some("192.168.1.132:7700".to_owned()),
            ["toadstool", "barracuda", "coralreef"],
            ["compute", "tensor", "shader"],
        );
        topo.register_gate(
            "west-gate",
            Some("192.168.1.200:7700".to_owned()),
            ["skunkbat"],
            ["defense"],
        );
        topo.mark_healthy("east-gate", true);
        topo.mark_healthy("strand-gate", true);
        topo.mark_healthy("west-gate", false);
        topo
    }

    #[test]
    fn gate_count() {
        let topo = sample_topology();
        assert_eq!(topo.gate_count(), 3);
        assert_eq!(topo.healthy_gate_count(), 2);
    }

    #[test]
    fn resolve_local_capability() {
        let topo = sample_topology();
        let node = topo.resolve_capability("security").unwrap();
        assert_eq!(node.gate_id, "east-gate");
    }

    #[test]
    fn resolve_remote_capability() {
        let topo = sample_topology();
        let node = topo.resolve_capability("compute").unwrap();
        assert_eq!(node.gate_id, "strand-gate");
    }

    #[test]
    fn unreachable_capability_on_unhealthy_gate() {
        let topo = sample_topology();
        assert!(topo.resolve_capability("defense").is_none());
        let unreachable = topo.unreachable_capabilities();
        assert!(unreachable.contains("defense"));
    }

    #[test]
    fn reachable_capabilities_union() {
        let topo = sample_topology();
        let reachable = topo.reachable_capabilities();
        assert!(reachable.contains("security"));
        assert!(reachable.contains("compute"));
        assert!(!reachable.contains("defense"));
    }

    #[test]
    fn cross_gate_routes() {
        let topo = sample_topology();
        let routes = topo.routes_for_capability("compute");
        assert!(!routes.is_empty());
        let east_to_strand = routes
            .iter()
            .find(|r| r.from_gate == "east-gate" && r.to_gate == "strand-gate");
        assert!(east_to_strand.is_some());
        assert!(east_to_strand.unwrap().healthy);
    }

    #[test]
    fn routes_for_local_capability_empty_for_host() {
        let topo = sample_topology();
        let routes = topo.routes_for_capability("security");
        let self_route = routes.iter().find(|r| r.from_gate == "east-gate");
        assert!(self_route.is_none());
    }
}
