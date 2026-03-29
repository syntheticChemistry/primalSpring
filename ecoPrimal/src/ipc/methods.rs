// SPDX-License-Identifier: AGPL-3.0-or-later

//! Canonical JSON-RPC method name constants.
//!
//! Centralizes the stringly-typed method names used across coordination,
//! discovery, health, and provenance. Using constants instead of inline
//! string literals prevents typo-induced silent failures and provides a
//! single inventory of the ecosystem's RPC surface.

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
}

/// Lifecycle domain — primal lifecycle introspection.
pub mod lifecycle {
    /// Current lifecycle status (starting, ready, degraded, stopping).
    pub const STATUS: &str = "lifecycle.status";
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
    /// Validate a deploy graph structurally.
    pub const VALIDATE: &str = "graph.validate";
    /// List known deploy graphs.
    pub const LIST: &str = "graph.list";
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

/// Provenance trio operations (via Neural API capability routing).
pub mod provenance {
    /// Create a DAG session (rhizoCrypt).
    pub const SESSION_CREATE: &str = "provenance.session.create";
    /// Append an event to a DAG session (rhizoCrypt).
    pub const EVENT_APPEND: &str = "provenance.event.append";
    /// Commit a provenance record (loamSpine).
    pub const COMMIT: &str = "provenance.commit";
    /// Claim attribution (sweetGrass).
    pub const ATTRIBUTION_CLAIM: &str = "attribution.claim";
    /// Resolve an attribution dispute (sweetGrass).
    pub const ATTRIBUTION_RESOLVE: &str = "attribution.resolve";
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
            lifecycle::STATUS,
            coordination::VALIDATE_COMPOSITION,
            coordination::PROBE_PRIMAL,
            coordination::DISCOVERY_SWEEP,
            graph::VALIDATE,
            graph::LIST,
            composition::NUCLEUS_HEALTH,
            mcp::TOOLS_LIST,
            provenance::SESSION_CREATE,
            provenance::EVENT_APPEND,
            provenance::COMMIT,
            provenance::ATTRIBUTION_CLAIM,
            provenance::ATTRIBUTION_RESOLVE,
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
            coordination::VALIDATE_COMPOSITION,
        ];
        for method in all {
            assert!(!method.starts_with('.'), "{method:?} starts with dot");
            assert!(!method.ends_with('.'), "{method:?} ends with dot");
        }
    }
}
