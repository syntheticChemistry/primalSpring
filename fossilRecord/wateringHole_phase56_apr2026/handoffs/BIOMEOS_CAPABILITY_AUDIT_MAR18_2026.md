# biomeOS Capability-Based Compliance Audit

**From**: primalSpring coordination (v0.3.5)  
**To**: biomeOS team  
**Date**: 2026-03-18  
**Severity**: High — the Neural API provider bypasses its own Neural API

## Executive Summary

biomeOS provides the Neural API capability translation layer but does not use it internally. 12+ production code paths bypass the Neural API with direct beardog/songbird socket connections. The capability translation registry is also missing `genetic.*` and `lineage.*` domains.

## Critical Fixes

### 1. Replace `DirectBeardogCaller` with Neural API routing

**Definition**: `crates/biomeos-spore/src/beacon_genetics/capability.rs:141-230`  
**Used in**: enrollment, dark forest beacon, lineage derivation

| Usage Site | Raw Methods | Replacement |
|---|---|---|
| `modes/enroll.rs:131,166` | `genetic.derive_lineage_key`, `crypto.sign` | `NeuralApiCapabilityCaller` |
| `dark_forest/beacon.rs:82` | `crypto.chacha20_poly1305_encrypt/decrypt`, `crypto.blake3_hash`, `genetic.*` | `capability.call()` |
| `beacon_genetics/derivation/lineage_deriver.rs` | `genetic.derive_lineage_key`, `crypto.sign`, `genetic.mix_entropy`, `genetic.generate_lineage_proof`, `genetic.verify_lineage` | `capability.call()` |

**Keep** `DirectBeardogCaller` only for bootstrap (before Neural API is running).

### 2. Replace identity-based discovery functions

| Function | Location | Replacement |
|---|---|---|
| `discover_beardog_socket()` | `modes/enroll.rs`, `nucleus/identity.rs`, `nucleus/trust.rs`, `federation/subfederation/beardog.rs` | `discover_by_capability("security")` |
| `discover_songbird_socket()` | `federation/discovery/mod.rs`, `nucleus/discovery.rs` | `discover_by_capability("discovery")` |
| `discover_beardog_socket(&env)` | `graph/executor/node_handlers.rs` | `discover_capability_socket("security", &env)` |

### 3. Complete the capability translation registry

**Missing domains** (used by `DirectBeardogCaller` but not in registry):

| Method | Domain | Notes |
|---|---|---|
| `genetic.derive_lineage_key` | `genetic` | Used by `LineageDeriver` |
| `genetic.mix_entropy` | `genetic` | Used by `LineageDeriver` |
| `genetic.verify_lineage` | `genetic` | Used by `LineageDeriver` and `DarkForestBeacon` |
| `genetic.generate_lineage_proof` | `genetic` | Used by `LineageDeriver` and `DarkForestBeacon` |
| `lineage.verify_siblings` | `lineage` | Used by `node_handlers.rs` |
| `lineage.verify_members` | `lineage` | Used by `subfederation/beardog.rs` |
| `crypto.derive_subfederation_key` | `crypto` | Used by `subfederation/beardog.rs` |
| `crypto.derive_child_seed` | `crypto` | Used by `node_handlers.rs` |

### 4. Remove hardcoded primal rosters

| Location | Pattern | Replacement |
|---|---|---|
| `orchestrator.rs:42-44` | `vec!["beardog-server", "songbird-orchestrator"]` | Graph-based primal list |
| `beacon_verification.rs:282` | `["beardog", "songbird"]` | `discover_beacon_providers_by_capability()` |
| `primal_coordinator.rs:161+` | `vec!["beardog", "songbird"]` | Deploy graph primals |
| `genomebin-v3/composer.rs:94+` | Hardcoded beardog + songbird | Capability-based composer |

### 5. Eat your own dogfood — route through Neural API

12+ sites bypass the Neural API in production:

| Location | Bypass | Fix |
|---|---|---|
| `modes/enroll.rs:166` | `DirectBeardogCaller::new` | `NeuralApiCapabilityCaller` |
| `graph/executor/node_handlers.rs:115-166` | `UnixStream::connect` → `crypto.derive_child_seed` | `capability.call("crypto", "derive_child_seed")` |
| `federation/subfederation/beardog.rs:46-91` | `UnixStream::connect` → `lineage.verify_members` | `capability.call("lineage", "verify_members")` |
| `api/beacon_verification.rs:111` | `AtomicClient::unix` → `crypto.blake3_hash` | `capability.call("crypto", "hash")` |
| `nucleus/identity.rs:98` | `discover_beardog_socket()` → direct | `capability.call("security", "verify_identity")` |
| `atomic-deploy/primal_communication.rs:117` | `AtomicClient::unix(beardog_socket)` | Neural API routing |
| `atomic-deploy/beardog_jwt_client.rs:49` | `AtomicClient::unix` | Neural API routing |
| `atomic-deploy/health_check.rs:179` | `AtomicClient::unix` | Neural API routing |

## What's Good

- Pure Rust, minimal unsafe (test-only `env::set_var`)
- `capability_registry.toml` with 285+ translations
- Neural API server works (COORDINATED MODE detected in live test)
- `CapabilityTaxonomy` partially used in federation/nucleus
- `unwrap_used = "deny"`, `expect_used = "deny"` in workspace
- No `todo!()` or `unimplemented!()` in production
