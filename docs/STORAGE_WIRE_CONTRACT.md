# NestGate Storage Wire Contract

> Interface specification for downstream springs coding against NestGate's JSON-RPC surfaces.
> Date: April 15, 2026

## Transport

| Channel | Protocol | Auto-detection |
|---------|----------|----------------|
| UDS | JSON-RPC 2.0 over raw TCP framing | First-byte peek: `{` → JSON-RPC, else BTSP binary |
| TCP | JSON-RPC 2.0 (HTTP POST `/jsonrpc` for legacy subset) | Peek on raw TCP; HTTP for API path |

Socket path: `/run/user/$UID/primal-nestgate-$FAMILY_ID.sock`

Method prefix normalization: `nestgate.` prefix is stripped automatically — both
`nestgate.storage.store` and `storage.store` resolve to the same handler.

---

## Core Storage Operations

| Method | Params | Response |
|--------|--------|----------|
| `storage.store` / `storage.put` | `{ key: string, value: any, family_id?: string }` | `{ status: "stored", key: string, family_id: string }` |
| `storage.retrieve` / `storage.get` | `{ key: string, family_id?: string }` | `{ value: any, data: any, key: string, family_id: string }` |
| `storage.exists` | `{ key: string, family_id?: string }` | `{ exists: bool }` |
| `storage.delete` | `{ key: string, family_id?: string }` | `{ status: "deleted", key: string }` |
| `storage.list` | `{ prefix?: string, family_id?: string }` | `{ keys: [string] }` |
| `storage.stats` | `{ family_id?: string }` | `{ key_count: u64, blob_count: u64, family_id: string }` |
| `storage.namespaces.list` | `{ family_id?: string }` | Namespace enumeration |

---

## Blob Operations (base64-encoded binary)

| Method | Params | Response |
|--------|--------|----------|
| `storage.store_blob` | `{ key: string, blob: base64 }` | `{ status: "stored", key: string, family_id: string, size: u64 }` |
| `storage.retrieve_blob` | `{ key: string }` | `{ blob: base64, key: string, family_id: string, size: u64 }` |
| `storage.retrieve_range` | `{ key: string, offset?: u64, length: u64 }` | `{ data: base64, offset: u64, length: u64, total_size: u64, encoding: "base64" }` |
| `storage.object.size` | `{ key: string }` | `{ exists: bool, size: u64, storage_type: "blob"\|"object"\|"none" }` |
| `storage.fetch_external` | `{ url: string, cache_key?: string }` | Fetched payload + provenance metadata |

---

## Bonding Ledger Persistence

Used by BearDog for ionic bond state persistence.

| Method | Params | Response |
|--------|--------|----------|
| `bonding.ledger.store` | `{ bond_id: string, data: object }` | `{ status: "stored" }` |
| `bonding.ledger.retrieve` | `{ bond_id: string }` | `{ data: object }` |
| `bonding.ledger.list` | `{}` | `{ bonds: [string] }` |

---

## Session Persistence

| Method | Params | Response |
|--------|--------|----------|
| `session.save` | `{ session_id: string, data: object }` | `{ status: "saved", session_id: string }` |
| `session.load` | `{ session_id: string }` | `{ data: object, session_id: string }` |
| `session.list` | `{}` | `{ sessions: [string] }` (isomorphic adapter only) |
| `session.delete` | `{ session_id: string }` | `{ status: "deleted" }` (isomorphic adapter only) |

---

## Model Cache Operations

| Method | Params | Response |
|--------|--------|----------|
| `model.register` | `{ model_id: string, path: string, metadata?: object }` | `{ registered: bool }` |
| `model.exists` | `{ model_id: string }` | `{ exists: bool }` |
| `model.locate` | `{ model_id: string }` | `{ path: string }` |
| `model.metadata` | `{ model_id: string }` | `{ metadata: object }` |

---

## Template Operations

| Method | Params | Response |
|--------|--------|----------|
| `templates.store` | `{ template_id: string, content: object }` | `{ status: "stored" }` |
| `templates.retrieve` | `{ template_id: string }` | `{ content: object }` |
| `templates.list` | `{ prefix?: string }` | `{ templates: [string] }` |
| `templates.community_top` | `{ limit?: u32 }` | `{ templates: [object] }` |

---

## NAT / Beacon Persistence

| Method | Params | Response |
|--------|--------|----------|
| `nat.store_traversal_info` | `{ peer_id: string, info: object }` | `{ stored: bool }` |
| `nat.retrieve_traversal_info` | `{ peer_id: string }` | `{ info: object }` |
| `beacon.store` | `{ beacon_id: string, data: object }` | `{ stored: bool }` |
| `beacon.retrieve` | `{ beacon_id: string }` | `{ data: object }` |
| `beacon.list` / `nat.beacon` | `{}` | `{ beacons: [string] }` |
| `beacon.delete` | `{ beacon_id: string }` | `{ deleted: bool }` |

---

## Audit

| Method | Params | Response |
|--------|--------|----------|
| `audit.store_execution` | `{ execution_id: string, record: object }` | `{ stored: bool }` |

---

## ZFS Operations (subprocess-backed)

| Method | Params | Response |
|--------|--------|----------|
| `zfs.pool.list` | `{}` | `{ status: "success", pools: [object] }` |
| `zfs.pool.get` | `{ pool: string }` | `{ status: "success", pool: object }` |
| `zfs.pool.health` | `{}` | `{ status: "success", health: object }` |
| `zfs.dataset.list` | `{ pool?: string }` | `{ status: "success", datasets: [object] }` |
| `zfs.dataset.get` | `{ dataset: string }` | `{ status: "success", dataset: object }` |
| `zfs.snapshot.list` | `{ dataset?: string }` | `{ status: "success", snapshots: [object] }` |
| `zfs.health` | `{}` | `{ zfs_available: bool, status: string }` |

---

## Discovery / Health

| Method | Response |
|--------|----------|
| `health.liveness` | `{ status: "alive", primal: "nestgate" }` |
| `health.readiness` | `{ status: "ready"\|"not_ready", storage: "initialized"\|"not_initialized" }` |
| `health.check` / `health` | `{ status: "healthy", version: string, primal: "nestgate" }` |
| `identity.get` | `{ primal: "nestgate", version: string, domain: "storage", license: "AGPL-3.0-or-later", family_id: string }` |
| `capabilities.list` / `capability.list` | Method enumeration + capabilities array |
| `discover_capabilities` | Full discovery payload |
| `discovery.capability.register` | `{ capability: string, endpoint: string }` → `{ success: bool }` |

---

## `data.*` Namespace (delegation stub)

Any method matching `data.*` is accepted but returns a delegation/not-implemented
error. This namespace is reserved for future live data feed providers that NestGate
will route to capability providers rather than handle directly.

---

## Downstream Usage Pattern

```rust
use ecoPrimal::composition::CompositionContext;

let ctx = CompositionContext::from_live_discovery_with_fallback().await?;
let result = ctx.storage_store("my-key", &json!({"data": "value"})).await?;
let retrieved = ctx.storage_retrieve("my-key").await?;
```
