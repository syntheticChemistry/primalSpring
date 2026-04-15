# Discovery Wire Contract (Songbird + biomeOS)

> Interface specification for downstream springs coding against Songbird's IPC/discovery
> and biomeOS Neural API's capability routing surfaces.
> Date: April 15, 2026

## Overview

Discovery has two layers:
1. **Songbird** — peer discovery, IPC registration, HTTP proxy, mesh networking
2. **biomeOS Neural API** — capability routing, graph orchestration, topology

Springs typically interact with biomeOS Neural API for capability routing and graph
deployment, and indirectly with Songbird through biomeOS's `capability.route` forwarding.

---

# Songbird Wire Surface

## Transport

Socket path: `/run/user/$UID/primal-songbird-$FAMILY_ID.sock`

All methods accept JSON-RPC 2.0 over raw TCP on UDS or TCP.

---

### `ipc.*` — Service Registry

| Method | Params | Response |
|--------|--------|----------|
| `ipc.register` | `{ primal_id: string, capabilities: [string], endpoint: string }` | `{ virtual_endpoint: string, registered_at: string }` |
| `ipc.resolve` | `{ primal_id?: string, capability?: string }` (at least one) | `{ virtual_endpoint: string, native_endpoint: string, capabilities: [string] }` |
| `ipc.discover` | `{ capability: string }` | `{ providers: [{ primal_id, endpoint, capabilities }] }` |
| `ipc.list` | `{}` | `{ services: [{ primal_id, virtual_endpoint, capabilities }] }` |

**Aliases**: `capability.discover` → `ipc.discover`, `net.discovery.find_by_capability` → `ipc.discover`

---

### `discovery.*` — Peer Discovery

| Method | Params | Response |
|--------|--------|----------|
| `discovery.peers` / `discovery.find_primals` | `{}` | `{ peers: [{ node_id, family_id, address, tcp_port, capabilities, last_seen }], total_count: u32 }` |
| `discovery.announce` / `announce_presence` | `{ family_id?: string, capabilities?: [string] }` | `{ announced: true, family_id: string, capabilities: [string] }` |
| `discovery.get_peer` | `{ peer_id: string }` | Peer info object (handler exists but not in main dispatch) |

---

### `http.*` — HTTP Proxy

| Method | Params | Response |
|--------|--------|----------|
| `http.request` | `{ url: string, method?: string, headers?: object, body?: string, timeout_ms?: u32 }` | `{ status_code: u16, headers: object, body: string, elapsed_ms: u64 }` |
| `http.get` | `{ url: string, headers?: object }` | Same response shape |
| `http.post` | `{ url: string, body: string, content_type?: string, headers?: object }` | Same response shape |
| `http.put` | `{ url: string, body: string }` | Same (orchestrator path only) |
| `http.delete` | `{ url: string }` | Same (orchestrator path only) |

---

### `mesh.*` — Onion Mesh Networking

| Method | Params | Response |
|--------|--------|----------|
| `mesh.init` | `{ node_id: string, bootstrap_onions?: [string] }` | `{ initialized: true, node_id: string }` |
| `mesh.status` | `{}` | `{ node_id, reachable_peers, paths: object }` |
| `mesh.find_path` | `{ target_node_id: string }` | Path object or `{ found: false }` |
| `mesh.announce` | `{ as_relay?: bool }` | Relay announcement result |
| `mesh.peers` | `{}` | `{ peers: [object] }` |
| `mesh.topology` | `{}` | `{ nodes: [object], edges: [object] }` |
| `mesh.health_check` | `{}` | `{ healthy: bool, details: object }` |
| `mesh.auto_discover` | `{}` | `{ discovered: u32 }` |

---

### Health

| Method | Response |
|--------|----------|
| `health.liveness` | `{ status: "alive", primal: "songbird" }` |
| `health.readiness` | `{ status: "ready", capabilities_count: u32 }` |
| `health.check` | `{ status: "healthy", version: string }` |

---

# biomeOS Neural API Wire Surface

## Transport

| Channel | Binding |
|---------|---------|
| TCP | `--tcp-only` binds `0.0.0.0:$PORT` (default 9100) |
| UDS | `/run/user/$UID/primal-biomeos-$FAMILY_ID.sock` |

---

### `capability.*` — Routing and Discovery

| Method | Params | Response |
|--------|--------|----------|
| `capability.register` | `{ capability: string, primal: string, socket: string, source?: string }` | `{ success: true, capability, primal, socket }` |
| `capability.discover` | `{ capability: string }` or `{ domain: string }` | `{ capability, atomic_type, primals: [object], primary_endpoint }` |
| `capability.resolve` | `{ capability: string }` or `{ domain: string }` | `{ resolved: bool, endpoint, primal, provider_count }` |
| `capability.route` | `{ capability: string, method: string, params?: object }` | Forwarded primal JSON result |
| `capability.call` | `{ capability: string, operation?: string, args?: object, gate?: string }` | Semantic routing / translation / forward result |
| `capability.list` / `capabilities.list` | `{}` | Full capability registry listing |
| `capability.providers` | `{ capability: string }` | `{ providers: [object], count: u32 }` |
| `capability.metrics` | `{}` | Routing metrics summary |
| `capability.discover_translations` | `{ capability: string }` | `{ translations: [object], count: u32 }` |
| `capability.list_translations` | `{}` | All registered translations |
| `route.register` | Batch registration params | Batch result |

**Semantic fallback**: Any `domain.operation` method not in the route table is
automatically rewritten to `capability.call { capability: "domain", operation: "operation" }`.

---

### `graph.*` — Graph Orchestration

| Method | Params | Response |
|--------|--------|----------|
| `graph.list` | `{}` | Array of graph summaries |
| `graph.get` / `graph.load` | `{ graph_id: string }` | Full graph JSON |
| `graph.save` | Graph JSON, or `{ toml: string }`, or `{ graph, nodes }` | `{ graph_id, location: "runtime" }` |
| `graph.execute` | `{ graph_id: string, family_id?: string, params?: object }` | `{ execution_id: string, status: string }` |
| `graph.status` | `{ execution_id: string }` | `{ status, completed_nodes: [string], failed_nodes: [(string, string)], error? }` |
| `graph.execute_pipeline` | `{ graph_id: string, channel_capacity?: u32 }` | Pipeline execution result |
| `graph.start_continuous` | `{ graph_id: string }` | Continuous session info |
| `graph.pause_continuous` | `{ session_id: string }` | `{ paused: bool }` |
| `graph.resume_continuous` | `{ session_id: string }` | `{ resumed: bool }` |
| `graph.stop_continuous` | `{ session_id: string }` | `{ stopped: bool }` |
| `graph.suggest_optimizations` | `{ graph_id: string, min_samples?: u32 }` | Optimization suggestions |
| `graph.protocol_map` | `{}` | Protocol map from ProtocolHandler |

---

### `topology.*` — Ecosystem Topology

| Method | Params | Response |
|--------|--------|----------|
| `topology.get` | `{}` | `{ primals: [object], connections: [object], timestamp: string }` |
| `topology.primals` | `{}` | Listed primals |
| `topology.proprioception` | `{}` | Self-awareness payload with neural_api section |
| `topology.metrics` | `{}` | System metrics |
| `topology.rescan` | `{}` | `{ rescanned: bool, new_capabilities_registered: u32, total_capabilities: u32 }` |

---

### Health

| Method | Response |
|--------|----------|
| `health.liveness` | `{ status: "alive", primal: "biomeos" }` |
| `health.readiness` | `{ status: "ready" }` |

---

### Legacy `neural_api.*` Aliases

All legacy names are routed to their canonical equivalents:

| Legacy | Canonical |
|--------|-----------|
| `neural_api.list_graphs` | `graph.list` |
| `neural_api.get_graph` | `graph.get` |
| `neural_api.save_graph` | `graph.save` |
| `neural_api.execute_graph` | `graph.execute` |
| `neural_api.get_execution_status` | `graph.status` |
| `neural_api.discover_capability` | `capability.discover` |
| `neural_api.route_to_primal` | `capability.route` |
| `neural_api.get_topology` | `topology.get` |
| `neural_api.get_primals` | `topology.primals` |
| `neural_api.get_proprioception` | `topology.proprioception` |
| `neural_api.get_metrics` | `topology.metrics` |

---

## Downstream Usage Pattern

```rust
use ecoPrimal::neural::NeuralBridge;

let bridge = NeuralBridge::connect("tcp://localhost:9100").await?;

// Capability routing
let result = bridge.capability_route("compute", "compute.dispatch", &params).await?;

// Graph deployment
let exec = bridge.graph_execute("nucleus_complete", None).await?;
let status = bridge.graph_status(&exec.execution_id).await?;

// Topology
let topo = bridge.topology_get().await?;
```
