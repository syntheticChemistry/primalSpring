// SPDX-License-Identifier: AGPL-3.0-or-later

//! Canonical JSON-RPC method name constants for primalSpring.
//!
//! This crate catalogs **primalSpring's own coordination-domain methods** and
//! **universal ecosystem methods** (health, capabilities, Neural API routing,
//! discovery, topology, MCP, and similar). Foreign primals each own their RPC
//! names; duplicating them here would violate the self-knowledge principle.
//! Using constants instead of inline string literals prevents typo-induced
//! silent failures and gives a single inventory of what primalSpring
//! recognizes for routing and tests.

/// Health domain — all primals must respond to these.
pub mod health {
    /// Basic health check (returns status + version).
    pub const CHECK: &str = "health.check";
    /// Liveness probe (minimal latency, for monitoring).
    pub const LIVENESS: &str = "health.liveness";
    /// Readiness probe (are all subsystems ready to serve?).
    pub const READINESS: &str = "health.readiness";
}

/// Capability domain — discovering what a primal can do.
pub mod capabilities {
    /// List all capabilities this primal exposes.
    pub const LIST: &str = "capabilities.list";
}

/// Capability routing via Neural API.
pub mod capability {
    /// Discover primals for a given capability domain.
    pub const DISCOVER: &str = "capability.discover";
    /// Invoke an operation on a capability domain.
    pub const CALL: &str = "capability.call";
    /// Register a capability provider with the neural router.
    pub const REGISTER: &str = "capability.register";
    /// Unregister a primal's capabilities (used by biomeOS rollback).
    pub const UNREGISTER: &str = "capability.unregister";
    /// Route a request to a capability provider.
    pub const ROUTE: &str = "capability.route";
}

/// Lifecycle domain — primal lifecycle management.
pub mod lifecycle {
    /// Start a primal.
    pub const START: &str = "lifecycle.start";
    /// Stop a primal (used by biomeOS rollback).
    pub const STOP: &str = "lifecycle.stop";
    /// Current lifecycle status (starting, ready, degraded, stopping).
    pub const STATUS: &str = "lifecycle.status";
    /// Register a primal with the lifecycle manager.
    pub const REGISTER: &str = "lifecycle.register";
}

/// Coordination domain — primalSpring's own methods.
pub mod coordination {
    /// Validate a composition (Tower, Node, Nest, NUCLEUS).
    pub const VALIDATE_COMPOSITION: &str = "coordination.validate_composition";
    /// Probe a single primal.
    pub const PROBE_PRIMAL: &str = "coordination.probe_primal";
    /// Sweep discovery across all known primals.
    pub const DISCOVERY_SWEEP: &str = "coordination.discovery_sweep";
}

/// Graph domain — deploy graph operations.
pub mod graph {
    /// Execute a deploy graph (biomeOS neural-api routing target).
    pub const EXECUTE: &str = "graph.execute";
    /// Query graph execution status.
    pub const STATUS: &str = "graph.status";
    /// Roll back a deployed graph (reverse topological lifecycle.stop).
    pub const ROLLBACK: &str = "graph.rollback";
    /// Validate a deploy graph structurally.
    pub const VALIDATE: &str = "graph.validate";
    /// List known deploy graphs.
    pub const LIST: &str = "graph.list";
    /// Run a streaming pipeline graph.
    pub const PIPELINE: &str = "graph.pipeline";
    /// Run a continuous-tick graph.
    pub const CONTINUOUS: &str = "graph.continuous";
}

/// Federation domain — cross-gate federation operations.
pub mod federation {
    /// Configure a gate for federation (per-gate setup).
    pub const CONFIGURE: &str = "federation.configure";
    /// Join a gate to a federation.
    pub const JOIN: &str = "federation.join";
    /// Health check across all gates in a federation.
    pub const HEALTH_CHECK: &str = "federation.health_check";
}

/// Discovery domain — primal and service discovery.
pub mod discovery {
    /// Discover primals via all available mechanisms.
    pub const DISCOVER: &str = "discovery.discover";
    /// Discover all primals (exhaustive scan).
    pub const DISCOVER_ALL: &str = "discovery.discover_all";
    /// List available discovery protocols.
    pub const PROTOCOLS: &str = "discovery.protocols";
}

/// Topology domain — network and composition topology.
pub mod topology {
    /// Get the current topology graph.
    pub const GET: &str = "topology.get";
    /// Proprioceptive topology (self-aware network map).
    pub const PROPRIOCEPTION: &str = "topology.proprioception";
    /// Rescan for newly-registered primals (biomeOS v2.81+).
    pub const RESCAN: &str = "topology.rescan";
}

/// Route domain — batch capability registration.
pub mod route {
    /// Batch-register all capabilities for a remote primal.
    pub const REGISTER: &str = "route.register";
}

/// Composition domain — high-level composition health.
pub mod composition {
    /// Full NUCLEUS health assessment.
    pub const NUCLEUS_HEALTH: &str = "composition.nucleus_health";
}

/// MCP tool discovery.
pub mod mcp {
    /// List MCP tools available from a primal.
    pub const TOOLS_LIST: &str = "mcp.tools.list";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_constants_are_dotted() {
        let all = [
            health::CHECK,
            health::LIVENESS,
            health::READINESS,
            capabilities::LIST,
            capability::DISCOVER,
            capability::CALL,
            capability::REGISTER,
            capability::UNREGISTER,
            capability::ROUTE,
            lifecycle::START,
            lifecycle::STOP,
            lifecycle::STATUS,
            lifecycle::REGISTER,
            coordination::VALIDATE_COMPOSITION,
            coordination::PROBE_PRIMAL,
            coordination::DISCOVERY_SWEEP,
            graph::EXECUTE,
            graph::STATUS,
            graph::ROLLBACK,
            graph::VALIDATE,
            graph::LIST,
            graph::PIPELINE,
            graph::CONTINUOUS,
            federation::CONFIGURE,
            federation::JOIN,
            federation::HEALTH_CHECK,
            discovery::DISCOVER,
            discovery::DISCOVER_ALL,
            discovery::PROTOCOLS,
            topology::GET,
            topology::PROPRIOCEPTION,
            topology::RESCAN,
            route::REGISTER,
            composition::NUCLEUS_HEALTH,
            mcp::TOOLS_LIST,
        ];
        for method in all {
            assert!(
                method.contains('.'),
                "method {method:?} should be dotted (domain.operation)"
            );
        }
    }

    #[test]
    fn no_leading_or_trailing_dots() {
        let all = [
            health::CHECK,
            health::LIVENESS,
            capabilities::LIST,
            capability::DISCOVER,
            capability::UNREGISTER,
            lifecycle::STOP,
            coordination::VALIDATE_COMPOSITION,
            graph::ROLLBACK,
            federation::CONFIGURE,
            discovery::DISCOVER,
            topology::GET,
            topology::RESCAN,
            route::REGISTER,
            composition::NUCLEUS_HEALTH,
            mcp::TOOLS_LIST,
        ];
        for method in all {
            assert!(!method.starts_with('.'), "{method:?} starts with dot");
            assert!(!method.ends_with('.'), "{method:?} ends with dot");
        }
    }
}
