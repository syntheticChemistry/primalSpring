# Neural API — Subsystem Evolution Spec

**Owner**: biomeOS (substrate primal) + primalSpring (validation + patterns)
**Status**: Operational — evolving toward layered semantic network
**Date**: May 22, 2026 (Wave 39)

---

## What Is the Neural API

The Neural API is biomeOS's composition orchestration subsystem. It is not a
single endpoint — it is the **semantic routing layer** that collapses complex
inter-primal interactions into emergent systems.

When a primal advertises `capabilities.list`, the Neural API ingests those
capabilities and their semantic translations. When a consumer calls
`capability.call`, the Neural API resolves the target primal, routes the
call, and returns the result. When a graph is deployed via `graph.execute`,
the Neural API orchestrates topological dispatch across multiple primals.

**The key insight**: the Neural API already acts as a learned routing
function. Its translation tables (`[translations.*]` in TOML) are the
weights. Its dispatch paths are the forward pass. As more operational data
flows through (call latencies, error rates, capability utilizations), these
tables can evolve from static configuration into adaptive routing — and
eventually into a neural network whose API surface IS the composition layer.

---

## How It Collapses Complexity

### Layer 0: Direct IPC (no Neural API)
```
bearDog ←UDS→ songbird ←UDS→ toadStool
       ←UDS→ nestgate  ←UDS→ rhizoCrypt
```
Every consumer must know every primal's socket, method names, and wire
format. N² coupling. This is what we started with.

### Layer 1: Capability Routing (current Neural API)
```
consumer → biomeOS.capability.call("crypto", "hash", data)
                    ↓ semantic_translation
           bearDog.crypto.hash(data)
                    ↓ response
consumer ← result
```
The consumer names a **capability**, not a primal. biomeOS resolves the
routing. Primals can be swapped, upgraded, or distributed without consumer
changes.

### Layer 2: Graph Composition (current — RootPulse, RPGPT)
```
consumer → biomeOS.graph.execute("rootpulse_commit.toml")
                    ↓ topological_waves()
           wave[0]: bearDog.crypto.sign(commit_hash)
           wave[1]: rhizoCrypt.dag.event.append(signed_event)
           wave[2]: sweetGrass.braid.anchor(dag_id)
           wave[3]: loamSpine.spine.commit(braid_id)
                    ↓ provenance seal
consumer ← ProvenanceSeal { merkle_root, commit_id, braid_id }
```
A single graph call orchestrates 4 primals across 3 domains. The consumer
sees one call. This is **emergent composition** — RootPulse does not exist
as code. It exists as a graph that the Neural API executes.

### Layer 3: Cross-Gate Composition (v3.66 — just shipped)
```
consumer → biomeOS.capability.call("storage", "sync", {gate: "westGate"})
                    ↓ try_relay_dispatch()
           songbird.relay.allocate(westGate)
                    ↓ TURN relay
           westGate.nestgate.storage.sync(params)
                    ↓ relay response
consumer ← result
```
The Neural API now routes across physical gates via songbird TURN relay.
The consumer doesn't know which gate serves the capability. Geographic
distribution is transparent.

### Layer 4: Adaptive Routing (next evolution)

The Neural API accumulates operational data:
- Call latencies per primal per method
- Error rates and circuit breaker states
- Capability utilization (which methods are hot)
- Gate health and availability

This data becomes the training signal for routing optimization:
- Route `crypto.hash` to the gate with lowest p95 latency
- Prefer local primals when available, fall back to remote
- Pre-warm capabilities that commonly co-occur in graphs
- Learn graph execution patterns and pre-stage dependencies

### Layer 5: Neural Network as API (horizon)

With enough operational data and layers, the routing function itself becomes
a neural network:
- **Input**: capability request (method, params, context)
- **Hidden layers**: routing tables, translation maps, gate topology,
  historical performance, current load
- **Output**: optimal dispatch plan (which primal, which gate, which
  transport, what timeout)

The API surface doesn't change. `capability.call` still works. But the
routing behind it evolves from static TOML tables to learned weights.
The Neural API becomes a neural network whose inference IS the API.

---

## Current Architecture (biomeOS v3.66)

```
biomeos neural-api
├── routing.rs         — 40+ neural_api.* aliases + capability.call
├── capability_call.rs — resolve + dispatch + try_relay_dispatch (CG-8)
├── signal.rs          — signal dispatch + graph env injection
├── execute.rs         — graph execution with env merge
└── config/capability_registry.toml — [translations.*] sections
```

### Existing Semantic Translations

biomeOS maps between domain vocabularies via `[translations.*]` TOML
sections. These are the "weights" of Layer 1:

| Source domain | Translation | Target |
|---------------|-------------|--------|
| `crypto.*` | → `security` | bearDog |
| `network.*` | → `discovery` | songbird |
| `compute.*` | → toadStool dispatch | toadStool |
| `storage.*` | → content/CAS | nestgate |
| `dag.*` | → provenance | rhizoCrypt |
| `spine.*` | → ledger | loamSpine |
| `braid.*` | → attribution | sweetGrass |
| `ai.*` | → inference | squirrel |
| `science.*` | → domain compute | neuralSpring (new, Wave 39) |

### primalSpring Validation Surface

primalSpring validates the Neural API via:
- `NeuralBridge` (`ipc/neural_bridge.rs`) — zero-coupling bridge
- `s_biomeos_neural_api` scenario — live health + graph execution
- `s_signal_dispatch_parity` — signal routing correctness
- `s_primal_announce` — semantic_mappings on announce
- `coordination.neural_api_status` RPC method
- `CompositionContext` — Tier 2-4 Neural API discovery

---

## Evolution Track

### Wave 39 (current): Absorption + Foundation
- [x] Absorb bearDog Wave 109 (ionic verify), songbird (TURN relay),
  biomeOS v3.66 (cross-gate), toadStool S269 (fan_out)
- [x] Wire `bonding.*` handlers through IonicContractRegistry
- [x] Add `science.*` routing (neuralSpring → 6 methods)
- [x] Registry: 445 → 452 methods
- [ ] Document Neural API as first-class subsystem (this document)

### Wave 40-42: Operational Data Collection
- [ ] Instrument `NeuralBridge` calls with latency + error metrics
- [ ] Add `neural_api.metrics` method to biomeOS for operational telemetry
- [ ] Graph execution timing per-node (which primal/wave is slow)
- [ ] Capability utilization tracking (hot methods, cold methods)
- [ ] Cross-gate latency baselines (local UDS vs remote TURN)

### Wave 43-45: Adaptive Routing
- [ ] Latency-aware `capability.call` routing (prefer fastest gate)
- [ ] Circuit breaker integration with routing decisions
- [ ] Graph pre-staging (dependency validation pattern, already prototyped)
- [ ] Co-occurrence analysis (which capabilities are called together)

### Wave 46+: Learned Routing (horizon)
- [ ] Routing table as weight matrix
- [ ] Gradient-free optimization of dispatch plans from operational data
- [ ] A/B shadow comparison (already prototyped in `validation::shadow`)
- [ ] Self-healing: automatic rerouting on primal failure

---

## Emergent Systems Already Demonstrating Collapse

These systems exist because the Neural API collapses their complexity:

| System | Primals involved | Neural API role |
|--------|-----------------|-----------------|
| **RootPulse** | bearDog + rhizoCrypt + sweetGrass + loamSpine | Graph orchestration of 4-primal provenance pipeline |
| **RPGPT** | squirrel + bearDog + rhizoCrypt | 60Hz tick loop with AI inference + provenance |
| **Tower Atomic** | bearDog + songbird + skunkBat | 3-primal security + discovery + defense |
| **Nest Atomic** | Tower + nestgate + trio | 7-primal storage + provenance composition |
| **FlockGate** | All + cellMembrane | Cross-WAN distributed compute via TURN relay |
| **Ionic Bonds** | bearDog + primalSpring + consumer | Contract lifecycle across trust boundaries |

None of these systems exist as monolithic code. They emerge from graph
composition through the Neural API. As the API learns, these systems
get faster, more resilient, and more adaptive — without changing their
graph definitions.

---

## Relationship to Deployment Systems

The Neural API IS the deployment system's runtime brain. When
`deploy_membrane.sh --composition nest` deploys primals to a gate, the
Neural API discovers them via `primal.announce` and makes them available
for `capability.call`. The deployment system handles placement; the Neural
API handles routing.

As we expand to multi-gate (westGate, northGate, strandGate), the Neural
API's cross-gate dispatch (v3.66) becomes the composition layer that makes
the cluster feel like one system. The deployment matrix
(`config/deployment_matrix.toml`) defines what goes where; the Neural API
makes it work at runtime.

---

*The Neural API doesn't replace primals. It makes them composable. And as
it learns from operational data, it makes them composable in ways we didn't
design — it discovers the compositions that work.*
