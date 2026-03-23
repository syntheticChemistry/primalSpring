# Provenance Trio — Live Probing Results

**Date**: March 23, 2026
**From**: primalSpring v0.7.0, Phase 11.1
**To**: sweetGrass team, loamSpine team, rhizoCrypt team
**License**: AGPL-3.0-or-later

---

## Summary

primalSpring built release binaries for all three trio workspaces and ran
live JSON-RPC probes against each. sweetGrass and rhizoCrypt are functional.
loamSpine has a runtime bug. Several wire-format gaps exist between
primalSpring's `ipc::provenance` module and the actual trio APIs.

---

## Per-Primal Results

### sweetGrass (v0.7.22) — LIVE

- **Unix socket**: Works (`SWEETGRASS_SOCKET` env var creates UDS listener)
- **HTTP JSON-RPC**: Works (random high port, `/jsonrpc` path)
- **health.liveness**: `{"alive":true}`
- **capability.list**: 24 methods across 9 domains (braid, anchoring, provenance,
  attribution, compression, contribution, pipeline, health, capability)
- **braid.create**: Works. Required params: `{data_hash, name, description, mime_type, size}`.
  Returns JSON-LD with `@context`, PROV-O `@type`, `was_attributed_to` DID, signature (unsigned/pending).
- **pipeline.attribute**: Works. Required params: `{session_id, merkle_root, source_primal, agent_did}`.
  Returns `{commit_ref, dehydration_merkle_root}`.
- **consumed_capabilities**: Reports `crypto.sign`, `crypto.verify`,
  `storage.artifact.store/get`, `dag.session.create`, `dag.dehydration.trigger`,
  `spine.create`, `commit.session` — matches primalSpring graph expectations.
- **transport**: Reports `["http", "uds"]`.

### rhizoCrypt (v0.13.0-dev) — LIVE (TCP only)

- **Unix socket**: NOT supported. `RHIZOCRYPT_SOCKET` env var is ignored.
  Server binds TCP port 9400 (tarpc) + 9401 (JSON-RPC HTTP).
- **JSON-RPC path**: `/rpc` (not `/` or `/jsonrpc`).
- **health.liveness**: `{"alive":true}`
- **capability.list**: dag.* methods + capability.list.
- **dag.session.create**: Works. Accepts `{name}`, returns UUID.
- **dag.event.append**: Works. Required params: `{session_id, event_type, payload}`.
  `event_type` is an **externally tagged struct variant enum**, not a string:
  ```json
  {"ExperimentStart": {"hypothesis": "...", "protocol": "..."}}
  {"Observation": {"metric": "...", "value": "...", "instrument": "..."}}
  {"Result": {"conclusion": "...", "confidence_percent": 99}}
  {"Custom": {"label": "...", "domain": "..."}}
  ```
  26 variant types supported.
- **dag.merkle.root**: Works. Returns 64-char hex hash after events appended.
- **dag.frontier.get**: Works. Returns array of frontier vertex hashes.
- **dag.session.get**: Works. Returns `{id, state, vertex_count, created_at, ...}`.

### loamSpine (v0.9.6) — BROKEN

- **Panic on startup**: `Cannot start a runtime from within a runtime` at
  `crates/loam-spine-core/src/service/infant_discovery.rs:234`.
  The `block_on` call in infant discovery conflicts with the tokio runtime.
- **No socket created**, no JSON-RPC available.
- **Fix**: Replace `block_on()` in infant discovery with proper async flow
  (`.await` instead of `block_on`, or spawn a dedicated discovery thread).

---

## Gaps Between primalSpring and Trio APIs

### Gap 1: rhizoCrypt has no Unix socket support

primalSpring's `ipc::provenance` routes via Neural API `capability.call`,
which expects Unix socket backends. rhizoCrypt only binds TCP.

**Options for resolution**:
- rhizoCrypt adds `--socket` flag / `RHIZOCRYPT_SOCKET` env var for UDS
- Neural API learns to proxy TCP JSON-RPC backends
- primalSpring adds TCP fallback to `capability_call` routing

### Gap 2: loamSpine runtime panic

`infant_discovery.rs:234` calls `block_on()` inside an async context.
This is a hard crash, no workaround from primalSpring side.

**Fix**: loamSpine team needs to refactor infant discovery to use `.await`.

### Gap 3: Event type wire format

rhizoCrypt `dag.event.append` expects struct-variant enums:
```json
{"event_type": {"ExperimentStart": {"hypothesis": "...", "protocol": "..."}}}
```

primalSpring's `ipc::provenance::record_experiment_step()` currently sends
generic JSON payloads. Needs alignment to the typed enum format.

### Gap 4: Param schema differences

| Method | primalSpring sends | Trio expects |
|--------|-------------------|-------------|
| `braid.create` | `{name, description}` | `{data_hash, name, description, mime_type, size}` |
| `pipeline.attribute` | `{session_id, merkle_root}` | `{session_id, merkle_root, source_primal, agent_did}` |
| `contribution.record_dehydration` | `{session_id, merkle_root}` | `{session_id, merkle_root, vertex_count, ...}` |

These are not blockers — `ipc::provenance` already uses `serde_json::json!`
for params, so the fix is updating the JSON payloads in the module.

---

## What Works Today (Without Neural API)

Even without biomeOS routing, direct-connection validation proves:

1. **sweetGrass** can receive braid creation and attribution requests over UDS
2. **rhizoCrypt** can run a full DAG session lifecycle over HTTP
3. The wire contract (JSON-RPC 2.0) is compatible
4. Capability lists are rich and match the graph expectations
5. PROV-O response format from sweetGrass is standards-compliant

## What's Needed Next

| Priority | Item | Owner |
|----------|------|-------|
| P0 | loamSpine infant_discovery panic fix | loamSpine team |
| P1 | rhizoCrypt Unix socket support | rhizoCrypt team |
| P1 | Update ipc::provenance param schemas | primalSpring |
| P1 | Update ipc::provenance event_type format | primalSpring |
| P2 | Neural API routing for TCP backends | biomeOS team |
| P2 | End-to-end pipeline: rhizo → loam → sweet | All teams |

---

## Cross-Spring Evolution Note

These gaps are expected and healthy. The point of live probing is to discover
exactly these kinds of contract mismatches before they become load-bearing.
Each gap is a focused evolution task, not a blocker. The architecture
(JSON on the wire, capability.call routing, graceful degradation) means
primalSpring continues to pass all 46/49 experiments regardless.

When wetSpring wants to track genetic data through the trio, it will
encounter its own domain-specific gaps — and that's the inherent point
of the cross-spring evolution model.

---

**License**: AGPL-3.0-or-later
