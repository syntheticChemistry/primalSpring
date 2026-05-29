# Cross-Gate Graph Executor — Design Spec

**Owner**: biomeOS (runtime) + primalSpring (validation)
**Status**: Spec — Wave 65+ implementation target
**Date**: May 29, 2026 (Wave 60)
**Coordination Domain**: All three (quorumSignal, rootPulse, waterFall)

---

## Problem

Today, `graph.execute` runs entirely within one NUCLEUS instance. All
nodes in a signal graph are dispatched to local primals via UDS IPC.
Cross-gate communication exists (songbird mesh, TURN relay, Forgejo SSH)
but is not accessible from within graph execution — it requires manual
wiring or bash scripts.

The triad needs cross-gate graph execution:
- **quorumSignal**: `tower.discover` should sense across the Plasmodium mesh
- **rootPulse**: `rootpulse.federate` must synchronize DAG state between gates
- **waterFall**: `ecosystem.pull/push/check` orchestrate repos across gates

## Design: Gate and Relay Hints on Graph Nodes

### Node-Level Hints

Each graph node gains two optional fields: `gate` and `relay`.

```toml
[[graph.nodes]]
name = "federate_dag"
binary = "rhizocrypt"
order = 4
required = true
spawn = false
depends_on = ["diff_remote"]
by_capability = "dag"
capabilities = ["dag.federate"]

# Cross-gate hints (new)
gate = "${TARGET_PEER}"           # execute on this gate's NUCLEUS
relay = "songbird"                # transport: songbird | turn | direct
```

#### `gate` field

- **Omitted or `"local"`**: execute on the local NUCLEUS (current behavior)
- **`"${VAR}"`**: resolved from graph environment at runtime (e.g. `TARGET_PEER=westGate`)
- **`"<gateName>"`**: hardcoded target gate (rare — most graphs should be parameterized)
- **`"any"`**: biomeOS selects the best gate based on routing weights (capability availability, latency, load)

#### `relay` field

Controls how the executor reaches the remote gate:

- **`"songbird"`** (default for cross-gate): route via songbird mesh (TCP federation or TURN relay as needed)
- **`"turn"`**: force TURN relay path (for NAT-traversal when direct mesh unavailable)
- **`"direct"`**: direct UDS/TCP to known endpoint (for VPS periplasm with stable addressing)
- **Omitted**: local UDS IPC (current behavior)

### Execution Model

```
graph.execute("rootpulse_federate.toml", env)
    │
    ├─ wave[0]: discover_peer     gate=local     relay=none     → local songbird
    ├─ wave[1]: diff_remote       gate=local     relay=none     → local rhizoCrypt
    ├─ wave[2]: replicate_content gate=${TARGET}  relay=songbird → remote nestGate
    ├─ wave[3]: federate_dag      gate=${TARGET}  relay=songbird → remote rhizoCrypt
    ├─ wave[4]: sync_braids       gate=${TARGET}  relay=songbird → remote sweetGrass
    └─ wave[5]: sign_receipt      gate=local     relay=none     → local bearDog
```

The executor:
1. Resolves `gate` hints against the graph environment
2. For `gate=local` nodes, dispatches via existing UDS `capability.call`
3. For remote gates, uses `try_relay_dispatch()` (CG-8) to route through songbird
4. Maintains the dependency graph — wave ordering is preserved across gates
5. Returns aggregated results as if all nodes were local

### Transport Resolution

```
gate hint → relay hint → transport selection
─────────────────────────────────────────────
local      → (ignored)  → UDS (existing path)
<gateName> → songbird   → songbird mesh TCP → remote NUCLEUS UDS
<gateName> → turn       → songbird TURN relay → remote NUCLEUS UDS
<gateName> → direct     → TCP to known endpoint (VPS periplasm)
any        → songbird   → biomeOS routing weights select gate, then songbird mesh
```

### Error Handling

- **Remote gate unreachable**: if `required = true`, graph fails; if `required = false`, node is skipped with a warning
- **Partial failure**: results from completed nodes are preserved; the graph returns partial results with error annotations
- **Timeout**: per-node `timeout_ms` applies; remote nodes get an additional `relay_timeout_ms` for transport overhead (default: 5000ms)
- **Circuit breaker**: biomeOS's existing `RoutingWeightTable` circuit breaker applies to remote gates — 5 consecutive failures open the circuit

### Security Model

Cross-gate graph execution requires **bilateral trust**:

1. The local NUCLEUS must have a valid songbird mesh connection to the remote gate
2. Both gates must share the same MitoBeacon family seed (genetic lineage trust)
3. BearDog on the remote gate must verify the requesting gate's identity via Dark Forest protocol
4. The remote primal must accept the `capability.call` — existing BTSP policies apply

This means cross-gate graphs work automatically within a bonded Plasmodium
(where genetic trust is already established) and fail safely against
unbonded gates.

### Graph Metadata Extensions

```toml
[graph.metadata]
cross_gate = true              # signals that this graph may execute remotely
transport = "uds_and_mesh"     # transport requirements: uds_only | uds_and_mesh | mesh_only
relay_timeout_ms = 5000        # extra timeout budget for relay transport
```

### Existing Infrastructure Used

| Component | Role in cross-gate execution |
|-----------|------------------------------|
| `try_relay_dispatch()` (CG-8) | Routes `capability.call` through songbird mesh to remote NUCLEUS |
| `RoutingWeightTable` | Selects optimal gate for `gate = "any"` nodes |
| `mesh.find_path` | Discovers route to target gate through the songbird topology |
| `TURN relay` | NAT traversal for WAN gates (flockGate, swiftGate, kinGate) |
| `MitoBeacon` | Family-level trust verification for cross-gate access |
| `BearDog Dark Forest` | Per-call identity verification on remote gate |

## Impact on Each Domain

### quorumSignal (sense)

- `tower.discover` gains `gate = "any"` — discover capabilities across the Plasmodium
- `meta.observe` gains `gate = "any"` — observe all gate health from any node
- Quorum validation can span gates: Tower quorum = 3 primals, but they don't need to be co-located

### rootPulse (action)

- `rootpulse.federate` uses `gate = "${TARGET_PEER}"` — already designed for this
- `rootpulse.commit` remains `gate = local` — commits are always local
- Future: `rootpulse.merge` could merge branches from different gates

### waterFall (sync)

- `ecosystem.pull` remains local (pulling is a local operation)
- `ecosystem.push` remains local (pushing from local to remote)
- `ecosystem.check` gains `gate = "any"` — check freshness of any gate from any gate
- Future: biomeOS orchestrates a fleet-wide pull by dispatching `ecosystem.pull` to all gates

## Implementation Phases

### Phase A: Spec + Validation (Wave 60 — this document)
- [x] Spec the `gate` and `relay` node hints
- [x] Document transport resolution and error handling
- [x] Define security model for cross-gate execution
- [ ] primalSpring validation scenario: graph with cross-gate hints parses correctly

### Phase B: Executor Extension (Wave 65)
- [ ] biomeOS `graph.execute` checks `gate` hint on each node
- [ ] For remote gates, delegate to `try_relay_dispatch()` instead of local UDS
- [ ] Wire `relay_timeout_ms` into dispatch timeout
- [ ] Test with `rootpulse_federate.toml` as first real cross-gate graph

### Phase C: Routing Integration (Wave 66)
- [ ] `gate = "any"` support via `RoutingWeightTable` scoring
- [ ] Circuit breaker for remote gate health
- [ ] `neural_api.route_explain` returns cross-gate routing decisions

### Phase D: Fleet Orchestration (Wave 67+)
- [ ] biomeOS can dispatch the same graph to multiple gates in parallel
- [ ] Plasmodium-level graph: "pull on all gates" = fan-out of `ecosystem.pull`
- [ ] Dashboard: cross-gate graph execution visualization via petalTongue

## Scope Boundary

**In scope**: spec, TOML schema extensions, transport resolution design,
security model, validation scenarios.

**Out of scope**: biomeOS runtime implementation (that's biomeOS team),
live Plasmodium testing, multi-region VPS mesh, UI dashboard.
