// SPDX-License-Identifier: AGPL-3.0-or-later

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

use super::routing::{capability_to_primal, method_to_capability_domain};

/// Composition tier — which atomic deployment a method participates in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
            | "beacon" | "lineage" | "tls" | "birdsong" | "identity" => Self::Tower,
            "discovery" | "network" | "stun" | "onion" | "tor" | "mesh" => Self::Tower,
            "defense" | "recon" | "threat" => Self::Tower,
            "compute" | "dispatch" | "toadstool" | "sovereign" => Self::Node,
            "tensor" | "math" | "ode" | "ml" | "nautilus" | "rng"
            | "stats" | "linalg" | "spectral" | "noise" | "shader" => Self::Node,
            "storage" | "content" | "secrets" => Self::Nest,
            "dag" | "spine" | "event" | "entry" | "session"
            | "certificate" | "permanence" | "proof" => Self::Nest,
            "braid" | "anchoring" | "provenance" | "attribution"
            | "contribution" | "anchor" => Self::Nest,
            "visualization" | "render" | "viz" | "interaction"
            | "proprioception" => Self::Meta,
            "ai" | "inference" | "squirrel" | "context" => Self::Meta,
            "science" => Self::Meta,
            "orchestration" | "federation" | "biomeos" | "primal"
            | "signal" | "topology" | "route" => Self::Orchestration,
            "health" | "capabilities" | "lifecycle" | "mcp"
            | "tool" | "tools" | "rpc" | "system" => Self::Orchestration,
            "coordination" | "composition" | "graph" | "nucleus" => Self::Orchestration,
            "bonding" | "ionic" => Self::Standalone,
            "game" | "webb" => Self::Standalone,
            _ => {
                match owner {
                    "beardog" | "songbird" | "skunkbat" => Self::Tower,
                    "toadstool" | "barracuda" | "coralreef" => Self::Node,
                    "nestgate" | "rhizocrypt" | "loamspine" | "sweetgrass" => Self::Nest,
                    "petaltongue" | "squirrel" => Self::Meta,
                    "biomeos" => Self::Orchestration,
                    _ => Self::Standalone,
                }
            }
        }
    }
}

/// A single method's routing entry in the neural routing table.
#[derive(Debug, Clone)]
pub struct RouteEntry {
    /// The JSON-RPC method string (e.g., "crypto.hash").
    pub method: String,
    /// The primal that owns this method.
    pub owner: String,
    /// The capability domain (e.g., "crypto", "storage").
    pub domain: String,
    /// Which composition tier this method participates in.
    pub tier: CompositionTier,
    /// Whether this method has a signal graph (e.g., "nest.store" → nest_store.toml).
    pub has_signal_graph: bool,
    /// Semantic aliases that map to this method.
    pub aliases: Vec<String>,
}

/// A composition pattern — a named sequence of methods that form an
/// emergent system when executed as a graph.
#[derive(Debug, Clone)]
pub struct CompositionPattern {
    /// Pattern name (e.g., "rootpulse_commit").
    pub name: String,
    /// Ordered method sequence (topological).
    pub methods: Vec<String>,
    /// Primals involved.
    pub primals: Vec<String>,
    /// Which tier this pattern belongs to.
    pub tier: CompositionTier,
}

/// The neural routing table — primalSpring's model of the capability method surface.
///
/// Built from the capability registry, this table provides O(1) lookup by
/// method, domain, tier, or primal. It also holds composition patterns
/// extracted from deploy graphs, showing how methods combine into emergent
/// systems.
pub struct NeuralRoutingTable {
    /// method string → route entry
    method_index: HashMap<String, RouteEntry>,
    /// domain → list of method strings
    domain_index: HashMap<String, Vec<String>>,
    /// tier → list of method strings
    tier_index: HashMap<CompositionTier, Vec<String>>,
    /// primal → list of method strings
    primal_index: HashMap<String, Vec<String>>,
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
        let mut domain_index: HashMap<String, Vec<String>> = HashMap::new();
        let mut tier_index: HashMap<CompositionTier, Vec<String>> = HashMap::new();
        let mut primal_index: HashMap<String, Vec<String>> = HashMap::new();

        let skip_sections = ["test_fixtures", "false_positives"];

        let mut signal_methods: Vec<String> = Vec::new();

        if let Some(table) = parsed.as_table() {
            // First pass: collect signal methods from [signals.*] sections.
            if let Some(signals) = table.get("signals").and_then(|v| v.as_table()) {
                for (tier_name, tier_val) in signals {
                    if let Some(sigs) = tier_val.get("signals").and_then(|v| v.as_array()) {
                        for sig in sigs {
                            if let Some(name) = sig.get("name").and_then(|v| v.as_str()) {
                                signal_methods.push(format!("{tier_name}.{name}"));
                            }
                        }
                    }
                }
            }

            // Second pass: register all methods from domain sections.
            for (section, value) in table {
                if skip_sections.contains(&section.as_str())
                    || section == "signals"
                {
                    continue;
                }
                let owner = value
                    .get("owner")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_owned();
                if owner == "none" || owner == "tests" {
                    continue;
                }

                let methods = value
                    .get("methods")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();

                for method in &methods {
                    let domain = method_to_capability_domain(method).to_owned();
                    let routed_owner = if owner == "all" {
                        "all".to_owned()
                    } else {
                        let routed = capability_to_primal(&domain).to_owned();
                        if routed == domain { owner.clone() } else { routed }
                    };
                    let tier = if owner == "all" {
                        CompositionTier::Orchestration
                    } else {
                        CompositionTier::from_domain(&domain, &routed_owner)
                    };

                    let is_signal = signal_methods.iter().any(|s| s == method);

                    let entry = RouteEntry {
                        method: method.clone(),
                        owner: routed_owner.clone(),
                        domain: domain.clone(),
                        tier,
                        has_signal_graph: is_signal,
                        aliases: Vec::new(),
                    };

                    method_index.insert(method.clone(), entry);
                    domain_index.entry(domain).or_default().push(method.clone());
                    tier_index.entry(tier).or_default().push(method.clone());
                    primal_index
                        .entry(routed_owner)
                        .or_default()
                        .push(method.clone());
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
        self.method_index.get(method)
    }

    /// All methods in a capability domain.
    #[must_use]
    pub fn methods_in_domain(&self, domain: &str) -> &[String] {
        self.domain_index
            .get(domain)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    /// All methods owned by a primal.
    #[must_use]
    pub fn methods_for_primal(&self, primal: &str) -> &[String] {
        self.primal_index
            .get(primal)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    /// All methods in a composition tier.
    #[must_use]
    pub fn methods_in_tier(&self, tier: CompositionTier) -> &[String] {
        self.tier_index
            .get(&tier)
            .map(Vec::as_slice)
            .unwrap_or(&[])
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
        self.primal_index.keys().map(String::as_str)
    }

    /// All domain names in the routing table.
    pub fn domains(&self) -> impl Iterator<Item = &str> {
        self.domain_index.keys().map(String::as_str)
    }

    /// Methods that have signal graph shortcuts.
    pub fn signal_methods(&self) -> impl Iterator<Item = &RouteEntry> {
        self.method_index.values().filter(|e| e.has_signal_graph)
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
    pub fn patterns_involving(&self, primal: &str) -> Vec<&CompositionPattern> {
        self.patterns
            .iter()
            .filter(|p| p.primals.iter().any(|pr| pr == primal))
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
        let mut primals: Vec<String> = methods
            .iter()
            .filter_map(|m| self.route(m).map(|e| e.owner.clone()))
            .collect();
        primals.sort();
        primals.dedup();

        let mut domains: Vec<String> = methods
            .iter()
            .filter_map(|m| self.route(m).map(|e| e.domain.clone()))
            .collect();
        domains.sort();
        domains.dedup();

        TierComposition {
            tier,
            method_count: methods.len(),
            primals,
            domains,
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
    let toml_str = std::fs::read_to_string(&registry_path).unwrap_or_default();
    let mut table = NeuralRoutingTable::from_registry(&toml_str);

    table.register_pattern(CompositionPattern {
        name: "rootpulse_commit".to_owned(),
        methods: vec![
            "crypto.sign".to_owned(),
            "dag.event.append".to_owned(),
            "braid.anchor".to_owned(),
            "spine.commit".to_owned(),
        ],
        primals: vec![
            "beardog".to_owned(),
            "rhizocrypt".to_owned(),
            "sweetgrass".to_owned(),
            "loamspine".to_owned(),
        ],
        tier: CompositionTier::Nest,
    });

    table.register_pattern(CompositionPattern {
        name: "tower_atomic_bootstrap".to_owned(),
        methods: vec![
            "crypto.sign_ed25519".to_owned(),
            "discovery.announce".to_owned(),
            "security.audit_event".to_owned(),
        ],
        primals: vec![
            "beardog".to_owned(),
            "songbird".to_owned(),
            "skunkbat".to_owned(),
        ],
        tier: CompositionTier::Tower,
    });

    table.register_pattern(CompositionPattern {
        name: "nest_store".to_owned(),
        methods: vec![
            "content.put".to_owned(),
            "dag.event.append".to_owned(),
            "spine.seal".to_owned(),
            "braid.create".to_owned(),
        ],
        primals: vec![
            "nestgate".to_owned(),
            "rhizocrypt".to_owned(),
            "loamspine".to_owned(),
            "sweetgrass".to_owned(),
        ],
        tier: CompositionTier::Nest,
    });

    table.register_pattern(CompositionPattern {
        name: "ionic_bond_lifecycle".to_owned(),
        methods: vec![
            "bonding.propose".to_owned(),
            "crypto.ionic_bond.verify_proposal".to_owned(),
            "bonding.accept".to_owned(),
            "bonding.status".to_owned(),
            "bonding.terminate".to_owned(),
        ],
        primals: vec![
            "primalspring".to_owned(),
            "beardog".to_owned(),
        ],
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
        assert_eq!(entry.owner, "beardog");
        assert_eq!(entry.domain, "security");
        assert_eq!(entry.tier, CompositionTier::Tower);
    }

    #[test]
    fn storage_store_routes_to_nestgate() {
        let table = test_table();
        let entry = table.route("storage.store").expect("storage.store should exist");
        assert_eq!(entry.owner, "nestgate");
        assert_eq!(entry.tier, CompositionTier::Nest);
    }

    #[test]
    fn compute_dispatch_routes_to_toadstool() {
        let table = test_table();
        let entry = table
            .route("compute.dispatch")
            .expect("compute.dispatch should exist");
        assert_eq!(entry.owner, "toadstool");
        assert_eq!(entry.tier, CompositionTier::Node);
    }

    #[test]
    fn science_methods_route_to_neuralspring() {
        let table = test_table();
        let entry = table
            .route("science.eigensolve")
            .expect("science.eigensolve should exist");
        assert_eq!(entry.owner, "neuralspring");
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
        assert!(tower.contains(&"crypto.hash".to_owned()));
        assert!(tower.contains(&"crypto.sign".to_owned()));
    }

    #[test]
    fn nest_tier_contains_storage_and_provenance() {
        let table = test_table();
        let nest = table.methods_in_tier(CompositionTier::Nest);
        assert!(nest.contains(&"storage.store".to_owned()));
        assert!(nest.contains(&"dag.event.append".to_owned()));
        assert!(nest.contains(&"braid.create".to_owned()));
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
        let rootpulse = patterns.iter().find(|p| p.name == "rootpulse_commit");
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
            .find(|p| p.name == "ionic_bond_lifecycle")
            .expect("ionic_bond_lifecycle should exist");
        assert!(ionic.primals.contains(&"beardog".to_owned()));
        assert!(ionic.primals.contains(&"primalspring".to_owned()));
    }

    #[test]
    fn tier_composition_tower_has_three_primals() {
        let table = test_table();
        let comp = table.tier_composition(CompositionTier::Tower);
        assert!(
            comp.primals.contains(&"beardog".to_owned()),
            "Tower should include beardog"
        );
        assert!(
            comp.primals.contains(&"songbird".to_owned()),
            "Tower should include songbird"
        );
        assert!(
            comp.primals.contains(&"skunkbat".to_owned()),
            "Tower should include skunkbat"
        );
    }

    #[test]
    fn signal_tier_patterns_extracted() {
        let table = test_table();
        // Signal graphs (nest.store, tower.publish, etc.) are composition
        // patterns over methods, not individual method entries. Verify that
        // our composition patterns cover the signal-tier methods.
        let nest_store = table
            .patterns()
            .iter()
            .find(|p| p.name == "nest_store");
        assert!(nest_store.is_some(), "nest_store pattern should exist");
        let ns = nest_store.unwrap();
        assert!(ns.methods.contains(&"content.put".to_owned()));
        assert!(ns.methods.contains(&"dag.event.append".to_owned()));
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
