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
            crypto::GENERATE_KEYPAIR,
            birdsong::ENCRYPT,
            genetic::DERIVE_LINEAGE_KEY,
            secrets::STORE,
            storage::LIST,
            game::ENGAGEMENT,
        ];
        for method in all {
            assert!(!method.starts_with('.'), "{method:?} starts with dot");
            assert!(!method.ends_with('.'), "{method:?} ends with dot");
        }
    }
}
