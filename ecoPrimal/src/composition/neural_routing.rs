// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Neural routing layer — semantic method resolution over the capability method surface.
//!
//! The Neural API in biomeOS collapses complex inter-primal interactions into
//! emergent systems via `capability.call` and `graph.execute`. This module
//! provides primalSpring's model of that routing layer: a registry of every
//! method, the primal that owns it, the composition tier it participates in,
//! and the routing metadata needed for adaptive dispatch.
//!
//! # Architecture
//!
//! ```text
//! NeuralRoutingTable
//! ├── method_index: "crypto.hash" → RouteEntry { owner, tier, domain, ... }
//! ├── domain_index: "crypto" → [methods...]
//! ├── tier_index:   Tower → [methods...]
//! ├── primal_index: "beardog" → [methods...]
//! └── composition_patterns: "rootpulse" → [method sequence]
//! ```
//!
//! The routing table is built from `config/capability_registry.toml` at
//! initialization. As primals announce via `primal.announce`, the table
//! can be updated with runtime routing data (latencies, error rates,
//! socket endpoints). Graphs define composition patterns — sequences of
//! methods that form emergent systems like RootPulse.
//!
//! This is Layer 1-2 of the Neural API evolution model. Layers 3-5
//! (cross-gate, adaptive, learned) build on this foundation.

use std::collections::HashMap;
use std::sync::Arc;

use super::routing::{capability_to_primal, method_to_capability_domain};

/// Composition tier — which atomic deployment a method participates in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CompositionTier {
    /// Tower: security + discovery + defense (bearDog, songbird, skunkBat)
    Tower,
    /// Node: Tower + compute (+ toadStool, barraCuda, coralReef)
    Node,
    /// Nest: Tower + storage + provenance (+ nestgate, rhizoCrypt, loamSpine, sweetGrass)
    Nest,
    /// NUCLEUS: all 13 primals composed
    Nucleus,
    /// Meta: observability + AI (petalTongue, squirrel)
    Meta,
    /// Infrastructure: biomeOS orchestration
    Orchestration,
    /// Standalone: method works with a single primal, no composition required
    Standalone,
}

impl CompositionTier {
    fn from_domain(domain: &str, owner: &str) -> Self {
        match domain {
            "crypto" | "security" | "auth" | "btsp" | "fido2" | "genetic"
            | "beacon" | "lineage" | "tls" | "birdsong" | "identity"
            | "discovery" | "network" | "stun" | "onion" | "tor" | "mesh"
            | "defense" | "recon" | "threat" => Self::Tower,
            "compute" | "dispatch" | "toadstool" | "sovereign"
            | "tensor" | "math" | "ode" | "ml" | "nautilus" | "rng"
            | "stats" | "linalg" | "spectral" | "noise" | "shader" => Self::Node,
            "storage" | "content" | "secrets"
            | "dag" | "spine" | "event" | "entry" | "session"
            | "certificate" | "permanence" | "proof"
            | "braid" | "anchoring" | "provenance" | "attribution"
            | "contribution" | "anchor" => Self::Nest,
            "visualization" | "render" | "viz" | "interaction"
            | "proprioception" | "ai" | "inference" | "squirrel" | "context"
            | "science" => Self::Meta,
            "orchestration" | "federation" | "biomeos" | "primal"
            | "signal" | "topology" | "route"
            | "health" | "capabilities" | "lifecycle" | "mcp"
            | "tool" | "tools" | "rpc" | "system"
            | "coordination" | "composition" | "graph" | "nucleus" => Self::Orchestration,
            "bonding" | "ionic" | "game" | "webb" => Self::Standalone,
            _ => {
                use crate::primal_names;
                match owner {
                    primal_names::BEARDOG | primal_names::SONGBIRD | primal_names::SKUNKBAT => Self::Tower,
                    primal_names::TOADSTOOL | primal_names::BARRACUDA | primal_names::CORALREEF => Self::Node,
                    primal_names::NESTGATE | primal_names::RHIZOCRYPT | primal_names::LOAMSPINE | primal_names::SWEETGRASS => Self::Nest,
                    primal_names::PETALTONGUE | primal_names::SQUIRREL => Self::Meta,
                    primal_names::BIOMEOS => Self::Orchestration,
                    _ => Self::Standalone,
                }
            }
        }
    }
}

/// A single method's routing entry in the neural routing table.
///
/// Uses `Arc<str>` for method/owner/domain strings — these are interned once
/// during table build and shared across all four indexes without allocation.
#[derive(Debug, Clone)]
pub struct RouteEntry {
    /// The JSON-RPC method string (e.g., "crypto.hash").
    pub method: Arc<str>,
    /// The primal that owns this method.
    pub owner: Arc<str>,
    /// The capability domain (e.g., "crypto", "storage").
    pub domain: Arc<str>,
    /// Which composition tier this method participates in.
    pub tier: CompositionTier,
    /// Whether this method has a composition graph (e.g., "nest.store" → nest_store.toml).
    pub has_composition_graph: bool,
    /// Semantic aliases that map to this method.
    pub aliases: Vec<Arc<str>>,
}

/// A composition pattern — a named sequence of methods that form an
/// emergent system when executed as a graph.
#[derive(Debug, Clone)]
pub struct CompositionPattern {
    /// Pattern name (e.g., "rootpulse_commit").
    pub name: Arc<str>,
    /// Ordered method sequence (topological).
    pub methods: Vec<Arc<str>>,
    /// Primals involved.
    pub primals: Vec<Arc<str>>,
    /// Which tier this pattern belongs to.
    pub tier: CompositionTier,
}

/// The neural routing table — primalSpring's model of the capability method surface.
///
/// Built from the capability registry, this table provides O(1) lookup by
/// method, domain, tier, or primal. Uses `Arc<str>` interning so shared
/// strings (method keys, owner slugs, domains) are allocated once and referenced.
pub struct NeuralRoutingTable {
    /// method string → route entry
    method_index: HashMap<Arc<str>, RouteEntry>,
    /// domain → list of method strings
    domain_index: HashMap<Arc<str>, Vec<Arc<str>>>,
    /// tier → list of method strings
    tier_index: HashMap<CompositionTier, Vec<Arc<str>>>,
    /// primal → list of method strings
    primal_index: HashMap<Arc<str>, Vec<Arc<str>>>,
    /// named composition patterns
    patterns: Vec<CompositionPattern>,
}

impl NeuralRoutingTable {
    /// Build the routing table from the capability registry TOML.
    #[must_use]
    pub fn from_registry(registry_toml: &str) -> Self {
        let parsed: toml::Value = match toml::from_str(registry_toml) {
            Ok(v) => v,
            Err(_) => return Self {
                method_index: HashMap::new(),
                domain_index: HashMap::new(),
                tier_index: HashMap::new(),
                primal_index: HashMap::new(),
                patterns: Vec::new(),
            },
        };
        let mut method_index = HashMap::new();
        let mut domain_index: HashMap<Arc<str>, Vec<Arc<str>>> = HashMap::new();
        let mut tier_index: HashMap<CompositionTier, Vec<Arc<str>>> = HashMap::new();
        let mut primal_index: HashMap<Arc<str>, Vec<Arc<str>>> = HashMap::new();

        let skip_sections = ["test_fixtures", "false_positives"];

        let mut composition_methods: Vec<String> = Vec::new();

        if let Some(table) = parsed.as_table() {
            // First pass: collect composition methods from [compositions.*] sections.
            if let Some(compositions) = table.get("compositions").and_then(|v| v.as_table()) {
                for (tier_name, tier_val) in compositions {
                    if let Some(sigs) = tier_val.get("compositions").and_then(|v| v.as_array()) {
                        for sig in sigs {
                            if let Some(name) = sig.get("name").and_then(|v| v.as_str()) {
                                composition_methods.push(format!("{tier_name}.{name}"));
                            }
                        }
                    }
                }
            }

            // Second pass: register all methods from domain sections.
            for (section, value) in table {
                if skip_sections.contains(&section.as_str())
                    || section == "compositions"
                {
                    continue;
                }
                let owner_raw = value
                    .get("owner")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                if owner_raw == "none" || owner_raw == "tests" {
                    continue;
                }

                let methods = value
                    .get("methods")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();

                for method_str in &methods {
                    let domain_str = method_to_capability_domain(method_str);
                    let routed_owner_str = if owner_raw == "all" {
                        "all"
                    } else {
                        let routed = capability_to_primal(domain_str);
                        if routed == domain_str { owner_raw } else { routed }
                    };
                    let tier = if owner_raw == "all" {
                        CompositionTier::Orchestration
                    } else {
                        CompositionTier::from_domain(domain_str, routed_owner_str)
                    };

                    let is_composition = composition_methods.iter().any(|s| s == method_str);

                    let method: Arc<str> = Arc::from(*method_str);
                    let domain: Arc<str> = Arc::from(domain_str);
                    let routed_owner: Arc<str> = Arc::from(routed_owner_str);

                    domain_index.entry(Arc::clone(&domain)).or_default().push(Arc::clone(&method));
                    tier_index.entry(tier).or_default().push(Arc::clone(&method));
                    primal_index
                        .entry(Arc::clone(&routed_owner))
                        .or_default()
                        .push(Arc::clone(&method));

                    let entry = RouteEntry {
                        method: Arc::clone(&method),
                        owner: routed_owner,
                        domain,
                        tier,
                        has_composition_graph: is_composition,
                        aliases: Vec::new(),
                    };
                    method_index.insert(Arc::clone(&entry.method), entry);
                }
            }
        }

        Self {
            method_index,
            domain_index,
            tier_index,
            primal_index,
            patterns: Vec::new(),
        }
    }

    /// Total methods in the routing table.
    #[must_use]
    pub fn method_count(&self) -> usize {
        self.method_index.len()
    }

    /// Look up a method's routing entry.
    #[must_use]
    pub fn route(&self, method: &str) -> Option<&RouteEntry> {
        self.method_index.get(method as &str)
    }

    /// All methods in a capability domain.
    #[must_use]
    pub fn methods_in_domain(&self, domain: &str) -> &[Arc<str>] {
        self.domain_index
            .get(domain as &str)
            .map_or(&[], Vec::as_slice)
    }

    /// All methods owned by a primal.
    #[must_use]
    pub fn methods_for_primal(&self, primal: &str) -> &[Arc<str>] {
        self.primal_index
            .get(primal as &str)
            .map_or(&[], Vec::as_slice)
    }

    /// All methods in a composition tier.
    #[must_use]
    pub fn methods_in_tier(&self, tier: CompositionTier) -> &[Arc<str>] {
        self.tier_index
            .get(&tier)
            .map_or(&[], Vec::as_slice)
    }

    /// Number of distinct capability domains.
    #[must_use]
    pub fn domain_count(&self) -> usize {
        self.domain_index.len()
    }

    /// Number of distinct primals in the routing table.
    #[must_use]
    pub fn primal_count(&self) -> usize {
        self.primal_index.len()
    }

    /// All primal names in the routing table.
    pub fn primals(&self) -> impl Iterator<Item = &str> {
        self.primal_index.keys().map(|s| &**s)
    }

    /// All domain names in the routing table.
    pub fn domains(&self) -> impl Iterator<Item = &str> {
        self.domain_index.keys().map(|s| &**s)
    }

    /// Methods that have signal graph shortcuts.
    pub fn composition_methods(&self) -> impl Iterator<Item = &RouteEntry> {
        self.method_index.values().filter(|e| e.has_composition_graph)
    }

    /// Register a composition pattern.
    pub fn register_pattern(&mut self, pattern: CompositionPattern) {
        self.patterns.push(pattern);
    }

    /// All registered composition patterns.
    #[must_use]
    pub fn patterns(&self) -> &[CompositionPattern] {
        &self.patterns
    }

    /// Find patterns that involve a specific primal.
    #[must_use]
    pub fn patterns_involving(&self, primal: &str) -> Vec<&CompositionPattern> {
        self.patterns
            .iter()
            .filter(|p| p.primals.iter().any(|pr| &**pr == primal))
            .collect()
    }

    /// Generate a tier summary — how many methods per tier.
    #[must_use]
    pub fn tier_summary(&self) -> HashMap<&'static str, usize> {
        let mut summary = HashMap::new();
        for (tier, methods) in &self.tier_index {
            let label = match tier {
                CompositionTier::Tower => "tower",
                CompositionTier::Node => "node",
                CompositionTier::Nest => "nest",
                CompositionTier::Nucleus => "nucleus",
                CompositionTier::Meta => "meta",
                CompositionTier::Orchestration => "orchestration",
                CompositionTier::Standalone => "standalone",
            };
            summary.insert(label, methods.len());
        }
        summary
    }

    /// Resolve a composition for a given tier — which primals and methods
    /// are needed to bring up that tier.
    #[must_use]
    pub fn tier_composition(&self, tier: CompositionTier) -> TierComposition {
        let methods = self.methods_in_tier(tier);
        let mut primals: Vec<&str> = methods
            .iter()
            .filter_map(|m| self.route(m).map(|e| &*e.owner))
            .collect();
        primals.sort_unstable();
        primals.dedup();

        let mut domains: Vec<&str> = methods
            .iter()
            .filter_map(|m| self.route(m).map(|e| &*e.domain))
            .collect();
        domains.sort_unstable();
        domains.dedup();

        TierComposition {
            tier,
            method_count: methods.len(),
            primals: primals.into_iter().map(ToOwned::to_owned).collect(),
            domains: domains.into_iter().map(ToOwned::to_owned).collect(),
        }
    }
}

/// Summary of what a composition tier requires.
#[derive(Debug, Clone)]
pub struct TierComposition {
    /// Which tier.
    pub tier: CompositionTier,
    /// How many methods belong to this tier.
    pub method_count: usize,
    /// Which primals must be deployed.
    pub primals: Vec<String>,
    /// Which capability domains are covered.
    pub domains: Vec<String>,
}

// Signal detection is now data-driven from [signals.*] sections in the TOML.

/// Build the canonical routing table from the workspace registry.
///
/// Reads `config/capability_registry.toml` relative to the workspace root.
#[must_use]
pub fn canonical_routing_table() -> NeuralRoutingTable {
    let registry_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../config/capability_registry.toml");
    let toml_str = match std::fs::read_to_string(&registry_path) {
        Ok(s) => s,
        Err(e) => {
            tracing::warn!(
                path = %registry_path.display(),
                error = %e,
                "capability registry not found — routing table will be empty"
            );
            String::new()
        }
    };
    let mut table = NeuralRoutingTable::from_registry(&toml_str);

    table.register_pattern(CompositionPattern {
        name: "rootpulse_commit".into(),
        methods: vec![
            "crypto.sign".into(),
            "dag.event.append".into(),
            "braid.anchor".into(),
            "spine.commit".into(),
        ],
        primals: vec!["beardog".into(), "rhizocrypt".into(), "sweetgrass".into(), "loamspine".into()],
        tier: CompositionTier::Nest,
    });

    table.register_pattern(CompositionPattern {
        name: "tower_atomic_bootstrap".into(),
        methods: vec![
            "crypto.sign_ed25519".into(),
            "discovery.announce".into(),
            "security.audit_event".into(),
        ],
        primals: vec!["beardog".into(), "songbird".into(), "skunkbat".into()],
        tier: CompositionTier::Tower,
    });

    table.register_pattern(CompositionPattern {
        name: "nest_store".into(),
        methods: vec![
            "content.put".into(),
            "dag.event.append".into(),
            "spine.seal".into(),
            "braid.create".into(),
        ],
        primals: vec!["nestgate".into(), "rhizocrypt".into(), "loamspine".into(), "sweetgrass".into()],
        tier: CompositionTier::Nest,
    });

    table.register_pattern(CompositionPattern {
        name: "ionic_bond_lifecycle".into(),
        methods: vec![
            "bonding.propose".into(),
            "crypto.ionic_bond.verify_proposal".into(),
            "bonding.accept".into(),
            "bonding.status".into(),
            "bonding.terminate".into(),
        ],
        primals: vec!["primalspring".into(), "beardog".into()],
        tier: CompositionTier::Standalone,
    });

    table
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_table() -> NeuralRoutingTable {
        canonical_routing_table()
    }

    #[test]
    fn routing_table_loads_all_methods() {
        let table = test_table();
        assert!(
            table.method_count() >= 450,
            "expected 450+ methods, got {}",
            table.method_count()
        );
    }

    #[test]
    fn crypto_hash_routes_to_beardog() {
        let table = test_table();
        let entry = table.route("crypto.hash").expect("crypto.hash should exist");
        assert_eq!(&*entry.owner, "beardog");
        assert_eq!(&*entry.domain, "security");
        assert_eq!(entry.tier, CompositionTier::Tower);
    }

    #[test]
    fn storage_store_routes_to_nestgate() {
        let table = test_table();
        let entry = table.route("storage.store").expect("storage.store should exist");
        assert_eq!(&*entry.owner, "nestgate");
        assert_eq!(entry.tier, CompositionTier::Nest);
    }

    #[test]
    fn compute_dispatch_routes_to_toadstool() {
        let table = test_table();
        let entry = table
            .route("compute.dispatch")
            .expect("compute.dispatch should exist");
        assert_eq!(&*entry.owner, "toadstool");
        assert_eq!(entry.tier, CompositionTier::Node);
    }

    #[test]
    fn science_methods_route_to_neuralspring() {
        let table = test_table();
        let entry = table
            .route("science.eigensolve")
            .expect("science.eigensolve should exist");
        assert_eq!(&*entry.owner, "neuralspring");
        assert_eq!(entry.tier, CompositionTier::Meta);
    }

    #[test]
    fn tower_tier_contains_security_methods() {
        let table = test_table();
        let tower = table.methods_in_tier(CompositionTier::Tower);
        assert!(
            tower.len() >= 50,
            "Tower tier should have 50+ methods, got {}",
            tower.len()
        );
        assert!(tower.iter().any(|s| &**s == "crypto.hash"));
        assert!(tower.iter().any(|s| &**s == "crypto.sign"));
    }

    #[test]
    fn nest_tier_contains_storage_and_provenance() {
        let table = test_table();
        let nest = table.methods_in_tier(CompositionTier::Nest);
        assert!(nest.iter().any(|s| &**s == "storage.store"));
        assert!(nest.iter().any(|s| &**s == "dag.event.append"));
        assert!(nest.iter().any(|s| &**s == "braid.create"));
    }

    #[test]
    fn tier_summary_covers_all_methods() {
        let table = test_table();
        let summary = table.tier_summary();
        let total: usize = summary.values().sum();
        assert_eq!(
            total,
            table.method_count(),
            "tier summary total should equal method count"
        );
    }

    #[test]
    fn rootpulse_pattern_registered() {
        let table = test_table();
        let patterns = table.patterns();
        let rootpulse = patterns.iter().find(|p| &*p.name == "rootpulse_commit");
        assert!(rootpulse.is_some(), "rootpulse_commit pattern should exist");
        let rp = rootpulse.unwrap();
        assert_eq!(rp.primals.len(), 4);
        assert_eq!(rp.tier, CompositionTier::Nest);
    }

    #[test]
    fn ionic_bond_pattern_spans_beardog_and_primalspring() {
        let table = test_table();
        let ionic = table
            .patterns()
            .iter()
            .find(|p| &*p.name == "ionic_bond_lifecycle")
            .expect("ionic_bond_lifecycle should exist");
        assert!(ionic.primals.iter().any(|s| &**s == "beardog"));
        assert!(ionic.primals.iter().any(|s| &**s == "primalspring"));
    }

    #[test]
    fn tier_composition_tower_has_three_primals() {
        let table = test_table();
        let comp = table.tier_composition(CompositionTier::Tower);
        assert!(
            comp.primals.iter().any(|s| &**s == "beardog"),
            "Tower should include beardog"
        );
        assert!(
            comp.primals.iter().any(|s| &**s == "songbird"),
            "Tower should include songbird"
        );
        assert!(
            comp.primals.iter().any(|s| &**s == "skunkbat"),
            "Tower should include skunkbat"
        );
    }

    #[test]
    fn composition_tier_patterns_extracted() {
        let table = test_table();
        // Signal graphs (nest.store, tower.publish, etc.) are composition
        // patterns over methods, not individual method entries. Verify that
        // our composition patterns cover the signal-tier methods.
        let nest_store = table
            .patterns()
            .iter()
            .find(|p| &*p.name == "nest_store");
        assert!(nest_store.is_some(), "nest_store pattern should exist");
        let ns = nest_store.unwrap();
        assert!(ns.methods.iter().any(|s| &**s == "content.put"));
        assert!(ns.methods.iter().any(|s| &**s == "dag.event.append"));
    }

    #[test]
    fn beardog_owns_crypto_and_security_methods() {
        let table = test_table();
        let beardog_methods = table.methods_for_primal("beardog");
        assert!(
            beardog_methods.len() >= 30,
            "beardog should own 30+ methods, got {}",
            beardog_methods.len()
        );
    }

    #[test]
    fn patterns_involving_beardog() {
        let table = test_table();
        let involving = table.patterns_involving("beardog");
        assert!(
            involving.len() >= 2,
            "beardog should participate in 2+ patterns"
        );
    }
}
