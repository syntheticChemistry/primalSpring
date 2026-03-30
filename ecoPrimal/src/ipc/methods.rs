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
    /// Deploy a graph (biomeOS graph executor).
    pub const DEPLOY: &str = "graph.deploy";
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

/// BearDog crypto domain — real cryptographic operations.
pub mod crypto {
    /// Generate an Ed25519 keypair.
    pub const GENERATE_KEYPAIR: &str = "crypto.generate_keypair";
    /// Sign data with Ed25519.
    pub const SIGN_ED25519: &str = "crypto.sign_ed25519";
    /// Verify an Ed25519 signature.
    pub const VERIFY_ED25519: &str = "crypto.verify_ed25519";
    /// Blake3 hash of data.
    pub const BLAKE3_HASH: &str = "crypto.blake3_hash";
    /// SHA-256 hash of data.
    pub const SHA256_HASH: &str = "crypto.sha256_hash";
}

/// BirdSong beacon domain — encrypted peer discovery.
pub mod birdsong {
    /// Generate an encrypted discovery beacon.
    pub const GENERATE_ENCRYPTED_BEACON: &str = "birdsong.generate_encrypted_beacon";
    /// Decrypt a discovery beacon.
    pub const DECRYPT_BEACON: &str = "birdsong.decrypt_beacon";
    /// Verify lineage chain from a beacon.
    pub const VERIFY_LINEAGE: &str = "birdsong.verify_lineage";
    /// Encrypt discovery payload (BearDog-side).
    pub const ENCRYPT: &str = "birdsong.encrypt";
    /// Decrypt discovery payload (BearDog-side).
    pub const DECRYPT: &str = "birdsong.decrypt";
}

/// Genetic identity domain — family/lineage key derivation.
pub mod genetic {
    /// Derive a beacon key from lineage seed (HKDF, domain birdsong_beacon_v1).
    pub const DERIVE_LINEAGE_BEACON_KEY: &str = "genetic.derive_lineage_beacon_key";
    /// Verify a lineage chain.
    pub const VERIFY_LINEAGE: &str = "genetic.verify_lineage";
    /// Derive a per-domain key from lineage seed.
    pub const DERIVE_LINEAGE_KEY: &str = "genetic.derive_lineage_key";
    /// Generate a BLAKE3 lineage proof for family verification.
    pub const GENERATE_LINEAGE_PROOF: &str = "genetic.generate_lineage_proof";
}

/// Secrets domain — encrypted key-value storage (BearDog).
pub mod secrets {
    /// Store an encrypted secret.
    pub const STORE: &str = "secrets.store";
    /// Retrieve a stored secret.
    pub const RETRIEVE: &str = "secrets.retrieve";
}

/// NestGate storage domain — persistent object storage.
pub mod storage {
    /// Store an object.
    pub const STORE: &str = "storage.store";
    /// Retrieve an object.
    pub const RETRIEVE: &str = "storage.retrieve";
    /// List stored objects.
    pub const LIST: &str = "storage.list";
}

/// Game domain — ludoSpring game science methods.
pub mod game {
    /// Evaluate flow state (Csikszentmihalyi).
    pub const EVALUATE_FLOW: &str = "game.evaluate_flow";
    /// Dynamic difficulty adjustment.
    pub const DIFFICULTY_ADJUSTMENT: &str = "game.difficulty_adjustment";
    /// Wave Function Collapse step.
    pub const WFC_STEP: &str = "game.wfc_step";
    /// Fitts's law cost analysis.
    pub const FITTS_COST: &str = "game.fitts_cost";
    /// Engagement metrics.
    pub const ENGAGEMENT: &str = "game.engagement";
    /// UI analysis (accessibility evaluation).
    pub const ANALYZE_UI: &str = "game.analyze_ui";
    /// Accessibility evaluation.
    pub const ACCESSIBILITY: &str = "game.accessibility";
    /// Procedural noise generation.
    pub const GENERATE_NOISE: &str = "game.generate_noise";
}

/// Compute domain — toadStool dispatch methods.
pub mod compute {
    /// Submit a compute dispatch job.
    pub const DISPATCH_SUBMIT: &str = "compute.dispatch.submit";
    /// Execute a dispatch (coralReef delegation).
    pub const DISPATCH_EXECUTE: &str = "compute.dispatch.execute";
    /// Health check for compute substrate.
    pub const HEALTH: &str = "compute.health";
}

/// Shader domain — toadStool S168 shader pipeline.
pub mod shader {
    /// Dispatch compiled shader binary (base64/u8/compile_result).
    pub const DISPATCH: &str = "shader.dispatch";
    /// Compile WGSL shader source (coralReef).
    pub const COMPILE_WGSL: &str = "shader.compile.wgsl";
}

/// Webb (narrative) domain — esotericWebb V6 session methods.
pub mod webb {
    /// Resolve a scene: predicates, effects, NPC state.
    pub const RESOLVE_SCENE: &str = "webb.resolve_scene";
    /// Query NPC state.
    pub const NPC_STATE: &str = "webb.npc_state";
    /// Ability check.
    pub const ABILITY_CHECK: &str = "webb.ability_check";
    /// Session state.
    pub const SESSION_STATE: &str = "webb.session_state";
    /// Liveness probe.
    pub const LIVENESS: &str = "webb.liveness";
}

/// Session domain — shared session lifecycle (esotericWebb, ludoSpring).
pub mod session {
    /// Create a new session.
    pub const CREATE: &str = "session.create";
    /// Advance to next scene.
    pub const ADVANCE: &str = "session.advance";
    /// Complete / finalize a session.
    pub const COMPLETE: &str = "session.complete";
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
            graph::DEPLOY,
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
            route::REGISTER,
            composition::NUCLEUS_HEALTH,
            mcp::TOOLS_LIST,
            provenance::SESSION_CREATE,
            provenance::EVENT_APPEND,
            provenance::COMMIT,
            provenance::ATTRIBUTION_CLAIM,
            provenance::ATTRIBUTION_RESOLVE,
            crypto::GENERATE_KEYPAIR,
            crypto::SIGN_ED25519,
            crypto::VERIFY_ED25519,
            crypto::BLAKE3_HASH,
            crypto::SHA256_HASH,
            birdsong::GENERATE_ENCRYPTED_BEACON,
            birdsong::DECRYPT_BEACON,
            birdsong::VERIFY_LINEAGE,
            birdsong::ENCRYPT,
            birdsong::DECRYPT,
            genetic::DERIVE_LINEAGE_BEACON_KEY,
            genetic::VERIFY_LINEAGE,
            genetic::DERIVE_LINEAGE_KEY,
            genetic::GENERATE_LINEAGE_PROOF,
            secrets::STORE,
            secrets::RETRIEVE,
            storage::STORE,
            storage::RETRIEVE,
            storage::LIST,
            game::EVALUATE_FLOW,
            game::DIFFICULTY_ADJUSTMENT,
            game::WFC_STEP,
            game::FITTS_COST,
            game::ENGAGEMENT,
            game::ANALYZE_UI,
            game::ACCESSIBILITY,
            game::GENERATE_NOISE,
            compute::DISPATCH_SUBMIT,
            compute::DISPATCH_EXECUTE,
            compute::HEALTH,
            shader::DISPATCH,
            shader::COMPILE_WGSL,
            webb::RESOLVE_SCENE,
            webb::NPC_STATE,
            webb::ABILITY_CHECK,
            webb::SESSION_STATE,
            webb::LIVENESS,
            session::CREATE,
            session::ADVANCE,
            session::COMPLETE,
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
            route::REGISTER,
            crypto::GENERATE_KEYPAIR,
            birdsong::ENCRYPT,
            genetic::DERIVE_LINEAGE_KEY,
            secrets::STORE,
            storage::LIST,
            game::ENGAGEMENT,
            compute::DISPATCH_SUBMIT,
            shader::DISPATCH,
            webb::RESOLVE_SCENE,
            session::CREATE,
        ];
        for method in all {
            assert!(!method.starts_with('.'), "{method:?} starts with dot");
            assert!(!method.ends_with('.'), "{method:?} ends with dot");
        }
    }
}
