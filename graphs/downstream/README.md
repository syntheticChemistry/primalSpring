# Downstream Proto-Nucleate Graphs

**Date**: April 13, 2026
**Owner**: primalSpring (intermediary for all springs)
**License**: AGPL-3.0-or-later

---

## What This Directory Contains

Proto-nucleate deployment graphs that downstream springs absorb into their
own repos. Each graph defines how a spring composes NUCLEUS primals for its
niche domain.

**Springs do not ship binaries at the composition level.** They validate
their science by composing primal capabilities via IPC.

## The Pattern

Every spring follows the same NUCLEUS base:

```
Tower (BearDog + Songbird) → always present
  ↓
Node (barraCuda + coralReef + toadStool) → math/shader/compute
  ↓
Nest (NestGate + provenance trio) → storage/lineage
  ↓
Spring Application (spring-specific binary) → domain composition
```

All NUCLEUS nodes have `spawn = false` — they're already running. The spring's
own binary is the only `spawn = true` node.

## Current Graphs

Proto-nucleates are consolidated via **template + manifest**. `downstream_manifest.toml`
parameterizes 7 springs using `proto_nucleate_template.toml`. One standalone graph is
kept for its unique dual-tower ionic bridge architecture.

| File | Type | Springs/Purpose |
|------|------|-----------------|
| `proto_nucleate_template.toml` | Template | Structural skeleton for all proto-nucleates |
| `downstream_manifest.toml` | Manifest | Parameters for airSpring, groundSpring, hotSpring, wetSpring, neuralSpring, ludoSpring, esotericWebb |
| `healthspring_enclave_proto_nucleate.toml` | Standalone | Unique dual-tower ionic bridge pattern (healthSpring) |

## Composition Parity Validation

After absorbing a graph, each spring should validate using `primalspring::composition`:

```rust
use primalspring::composition::{CompositionContext, validate_parity};
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

let mut ctx = CompositionContext::from_live_discovery();
let mut v = ValidationResult::new("mySpring — NUCLEUS parity");

// Example: validate stats.mean parity against a known baseline
// NOTE: barraCuda expects "data" param, not "values"
validate_parity(
    &mut ctx, &mut v,
    "my_niche_mean",
    "tensor",                                    // capability
    "stats.mean",                                // method
    serde_json::json!({"data": [1.0, 2.0, 3.0]}),
    "result",                                    // result key (not "mean")
    2.0_f64,                                     // expected
    tolerances::CPU_GPU_PARITY_TOL,              // tolerance
);

// Wire method param schemas (validated April 13, 2026):
//   stats.mean:         { "data": [f64...] }              → { "result": f64 }
//   tensor.create:      { "shape": [usize], "data": [f64] } → { "tensor_id": str }
//   storage.store:      { "key": str, "value": str }      → { "status": "stored" }
//   dag.session.create: { "label": str }                   → session_id (bare UUID)
//   dag.event.append:   { "session_id": str, "event_type": { "DataCreate": {...} }, "payload": {} }
//   braid.create:       { "data_hash": str, "agent": str, "mime_type": str, "size": u64 }
//   spine.create:       { "name": str, "owner": str }     → { "spine_id": UUID }
//   crypto.hash:        { "data": base64, "algorithm": "blake3" } → { "hash": base64(44) }
//
// NOTE: loamSpine health.check requires {"include_details": true} — unlike all
// other primals which accept empty params. CompositionContext::health_check()
// handles this automatically for ledger/spine/merkle capabilities.
```

See `experiments/exp094_composition_parity/` for the canonical full NUCLEUS
parity experiment that all springs can reference.

## rhizoCrypt `dag.event.append` — Event Type Reference

The `event_type` parameter uses Rust-style tagged enum syntax, **not** plain
strings. Each variant is a JSON object with a single key (the variant name)
whose value contains the required fields for that variant.

### Common Variants with Examples

**DataCreate** — record creation of a new data artifact:
```json
{
  "session_id": "019d875d-...",
  "event_type": {
    "DataCreate": {
      "data_id": "artifact-001",
      "description": "Raw sensor readings batch"
    }
  },
  "payload": {"source": "sensor_array_north"}
}
```

**AgentAction** — record an agent performing an action:
```json
{
  "event_type": {
    "AgentAction": {
      "agent_id": "squirrel-east",
      "action": "inference",
      "description": "Ran classification model"
    }
  }
}
```

**ExperimentStart** — begin a tracked experiment:
```json
{
  "event_type": {
    "ExperimentStart": {
      "experiment_id": "exp094",
      "description": "NUCLEUS composition parity"
    }
  }
}
```

**Custom** — freeform event for domain-specific use:
```json
{
  "event_type": {
    "Custom": {
      "kind": "my_domain_event",
      "data": {"key": "value"}
    }
  }
}
```

### Full Variant List

| Variant | Domain | Required Fields |
|---------|--------|----------------|
| `SessionStart` | lifecycle | `session_id` |
| `SessionEnd` | lifecycle | `session_id` |
| `AgentJoin` | agents | `agent_id` |
| `AgentLeave` | agents | `agent_id` |
| `AgentAction` | agents | `agent_id`, `action`, `description` |
| `DataCreate` | data | `data_id`, `description` |
| `DataModify` | data | `data_id`, `description` |
| `DataDelete` | data | `data_id` |
| `DataTransfer` | data | `data_id`, `target` |
| `SliceCheckout` | slicing | `slice_id` |
| `SliceOperation` | slicing | `slice_id`, `operation` |
| `SliceResolve` | slicing | `slice_id` |
| `GameEvent` | ludoSpring | `event_name` |
| `ItemLoot` | ludoSpring | `item_id` |
| `ItemDrop` | ludoSpring | `item_id` |
| `ItemTransfer` | ludoSpring | `item_id`, `target` |
| `Combat` | ludoSpring | `participants` |
| `Extraction` | ludoSpring | `zone_id` |
| `ExperimentStart` | science | `experiment_id`, `description` |
| `Observation` | science | `observation_id` |
| `Analysis` | science | `analysis_id` |
| `Result` | science | `result_id` |
| `DocumentEdit` | collaboration | `document_id` |
| `CommentAdd` | collaboration | `comment_id` |
| `ApprovalGrant` | governance | `approval_id` |
| `ApprovalRevoke` | governance | `approval_id` |
| `Custom` | any | `kind`, `data` |

## Content Distribution Federation

For content distribution (game assets, datasets, knowledge bases), see the
federation graph at `graphs/federation/content_distribution_federation.toml`.
This composes NestGate content-addressed storage, metallic seeder pools,
ionic consumer downloads, and Songbird relay for NAT traversal. Supporting
types are in `primalspring::bonding::content_distribution`.

## Bonding & BTSP Enforcement

When composing across NUCLEUS families (multi-node / cross-family), bonding
policies control cipher negotiation at the BTSP layer. Key things downstream
springs need to know:

**`BtspEnforcer::evaluate_connection` performs cipher upgrade.** If a peer's
cipher is weaker than the bond minimum, the cipher is auto-upgraded.

**`BtspEnforcer::evaluate_connection_with_trust` enforces deny semantics.**
When a peer's trust tier does not meet the `BondingPolicy` requirements, the
connection is rejected with `allowed: false`. For example, a `MitoBeaconFamily`
peer attempting a `Covalent` bond (which requires `NuclearLineage`) will be
denied at handshake time.

| Bond Type | Min Cipher | Trust Model | Behaviour |
|-----------|------------|-------------|-----------|
| Covalent | Null | SharedFamilySeed | Same-family, full trust — requires NuclearLineage |
| Metallic | Aes256Gcm | DelocElectronSea | Org-level trust, requires MitoBeaconFamily |
| Ionic | ChaCha20Poly1305 | ContractBased | Cross-family, metered — requires Contractual |
| Weak | ChaCha20Poly1305 | ZeroTrust | Unknown peer — cipher upgraded, ZeroTrust accepted |
| OrganoMetalSalt | ChaCha20Poly1305 | HybridTrust | Hybrid — Contractual minimum |

Springs that need trust-tier enforcement should use `evaluate_connection_with_trust`
and pass the peer's authenticated `TrustModel`.

## Upstream Gap Status (April 13, 2026)

All previously identified gaps (LD-01 through LD-10) are **RESOLVED** upstream.
The NUCLEUS stack runs **12/12 primals ALIVE** with **19/19 exp094 parity checks
PASS, 0 FAIL, 0 SKIP**.

Key resolutions that downstream springs benefit from:

| Resolution | Primal | What Changed |
|------------|--------|-------------|
| crypto.hash base64 normalized | BearDog | BTSP transport encodes hashes consistently |
| ipc.resolve returns `native_endpoint` / `virtual_endpoint` | Songbird | Full programmatic capability discovery works |
| Persistent UDS connections | NestGate, ToadStool | Multi-request sessions over a single socket |
| BTSP auto-detect on all transports | ToadStool | Accepts both raw JSON-RPC and BTSP-framed |
| No port conflict on startup | barraCuda | Binds `math-{family}.sock` without `--unix` override |
| UDS socket at `rhizocrypt-{family}.sock` | rhizoCrypt | DAG capability discoverable via standard naming |
| JSON-RPC over BTSP guard line | barraCuda | Full JSON-RPC wire support alongside tarpc |
| loamSpine TCP opt-in via `--listen` | loamSpine | UDS-first with opt-in TCP for replication |
| petalTongue `--socket` flag | petalTongue | Correct UDS path via CLI |

For the complete gap history, see `primalSpring/docs/PRIMAL_GAPS.md`.
