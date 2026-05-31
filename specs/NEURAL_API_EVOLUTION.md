# Neural API — Subsystem Evolution Spec

**Owner**: biomeOS (substrate primal) + primalSpring (observatory + validation)
**Status**: Operational — evolving toward layered semantic network
**Date**: May 24, 2026 (Wave 47)

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

## Current Architecture (biomeOS v3.70)

```
biomeos neural-api
├── routing.rs              — 44+ neural_api.* aliases + capability.call + utilization
├── capability_call.rs      — resolve + dispatch + try_relay_dispatch (CG-8) + utilization recording
├── signal.rs               — signal dispatch + graph env injection
├── execute.rs              — graph execution with env merge
├── neural_router/
│   ├── weights.rs          — RoutingWeightTable (redb-persistent) + CapabilityUtilizationTracker
│   └── composition.rs      — CompositionTier + CompositionPatternRegistry
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

### primalSpring Observatory Surface

primalSpring's domain IS primal coordination. It studies biomeOS's routing
intelligence and pushes evolution upstream — the same pattern other springs
use for their domain science.

Observatory tools:
- `NeuralBridge` (`ipc/neural_bridge.rs`) — zero-coupling bridge
  - `routing_weights()` — study adaptive routing convergence
  - `route_explain()` — study provider selection decisions
  - `composition_patterns()` — validate pattern consistency
  - `plan_tier()` — study tier deployment blueprints
  - `capability_call_instrumented()` — bridge round-trip with `BridgeOutcome` feedback
- `NeuralRoutingTable` — local static model for structural analysis
- `NeuralDispatcher` — dispatch metrics collection + bridge outcome ingestion
  - `record_bridge_outcome()` — ingest outcomes from external bridge calls
  - `dispatch_instrumented()` — dispatch with bridge-level timing automatically recorded
- `s_biomeos_neural_api` scenario — live health + graph execution
- `s_signal_dispatch_parity` — signal routing correctness
- `s_primal_announce` — semantic_mappings on announce
- `s_neural_routing_surface` — 17-check structural validation
- `coordination.neural_api_status` RPC method
- `CompositionContext` — Tier 2-4 Neural API discovery

---

## Evolution Track

### Wave 39: Absorption + Foundation
- [x] Absorb bearDog Wave 109 (ionic verify), songbird (TURN relay),
  biomeOS v3.66 (cross-gate), toadStool S269 (fan_out)
- [x] Wire `bonding.*` handlers through IonicContractRegistry
- [x] Add `science.*` routing (neuralSpring → 6 methods)
- [x] Registry: 445 → 452 methods
- [x] Document Neural API as first-class subsystem (this document)

### Wave 40: Neural Routing Layer
- [x] `NeuralRoutingTable` — data-driven routing table from `capability_registry.toml`
  - O(1) method → owner/domain/tier lookup across all 452 methods
  - 7 composition tiers (Tower/Node/Nest/Nucleus/Meta/Orchestration/Standalone)
  - Signal graph detection from `[signals.*]` sections
  - Named composition patterns (rootpulse_commit, tower_atomic_bootstrap,
    nest_store, ionic_bond_lifecycle)
- [x] `NeuralDispatcher` — high-level dispatch surface
  - `dispatch()` for single methods via `capability.call`
  - `dispatch_pattern()` for graph-backed compositions
  - Per-dispatch metrics (latency, success, route path) — Layer 4 training signal
  - `status_report()` returns full routing health as JSON
- [x] `coordination.neural_api_status` enhanced with full routing table summary
- [x] Scenario S46 `neural-routing-surface` — 17 structural checks
- [x] 775 tests passing (27 new: 14 routing + 12 dispatch + 1 scenario)

### Wave 40: Adaptive Routing Weights (biomeOS v3.67)
- [x] `RoutingWeightTable` in `neural_router/weights.rs` — per-provider EWMA
  latency, error rate, affinity, cost hints, circuit breaker
- [x] `ProviderWeight::score()` — scoring function: `affinity * reliability *
  latency_factor - cost_penalty` with exploration bonus for cold providers
- [x] Circuit breaker: 5 consecutive failures → open, 30s cooldown → half-open
- [x] `capability.call` feedback loop — every forward records outcome into weights
- [x] `primal.announce` accepts `cost_hints` and `latency_estimates` for
  self-reporting. Cooperative primals get affinity 0.6 (vs neutral 0.5)
- [x] `neural_api.routing_weights` RPC — full weight table snapshot
- [x] `neural_api.route_explain` RPC — routing decision explanation
- [x] 1290 biomeOS tests, 775 primalSpring tests — all passing

### Wave 41: Observatory Posture + Composition Abstraction
- [x] biomeOS v3.68 — `CompositionTier::classify()` maps domain + provider to
  atomic tier at runtime (Tower/Node/Nest/Nucleus/Meta/Orchestration/Standalone)
- [x] `CompositionPatternRegistry` — canonical patterns as first-class runtime
  objects (rootpulse_commit, tower_atomic_bootstrap, nest_store, tower_publish,
  meta_observe, ionic_bond_lifecycle). Extensible via primal.announce.
- [x] `plan_tier()` — deployment blueprints per tier (required primals, domains,
  available patterns)
- [x] `NeuralBridge` observatory methods — primalSpring consumes biomeOS routing
  intelligence: routing_weights, route_explain, composition_patterns, plan_tier
- [x] Registry: 454 → 456 methods (+2 neural_api.composition_patterns, neural_api.plan_tier)
- [x] 1303 biomeOS tests, 775 primalSpring tests — all passing

### Wave 42: Operational Data Deepening + Full Deployment
- [x] Wire `NeuralBridge.capability_call_instrumented` — primalSpring bridge
  records round-trip latency + success into `NeuralDispatcher` metrics
- [x] `NeuralDispatcher.record_bridge_outcome()` — ingest external bridge outcomes
- [x] `NeuralDispatcher.dispatch_instrumented()` — dispatch with bridge-level timing
- [x] Capability utilization tracking — `CapabilityUtilizationTracker` in biomeOS:
  hot/cold method analysis, `neural_api.utilization` RPC endpoint
- [x] Weight table persistence — redb-backed `RoutingWeightTable` survives restarts
  (`RoutingWeightTable::open()`, `NeuralRouter::with_persistent_weights()`)
- [x] `WAVE42_NEURAL_API_DEPLOYMENT_GUIDE.md` — `primal.announce` adoption guide
  for all 13 primal teams with v3.68 schema, cost/latency hints, signal tiers
- [x] `TEAM_OWNERSHIP_MATRIX.md` — cellMembrane/projectNUCLEUS/primalSpring ownership
- [x] Registry: 456 → 460 methods (+1 neural_api.utilization, +1 neural_api.weight_health, +2 Wave 55b)
- [x] 1311 biomeOS tests, 791 primalSpring tests — all passing
- [ ] Graph execution timing per-node (PathwayLearner → weight table)
- [ ] Cross-gate latency baselines (local UDS vs remote TURN)

### Wave 60: Coordination Triad + Ecosystem Signal Tier
- [x] `CoordinationDomain` enum: `Signal` / `Pulse` / `Fall` in `graphs/mod.rs`
- [x] Triad taxonomy documented (quorumSignal / rootPulse / waterFall)
- [x] 5 rootPulse signal graphs materialized (commit/branch/merge/diff/federate)
- [x] `ecosystem` signal tier introduced — 5th tier for membrane-level sync
- [x] 3 ecosystem signals: `ecosystem.pull`, `ecosystem.push`, `ecosystem.check`
- [x] `cascade-pull.sh` evolved to manifest-driven (reads `ecosystem_manifest.toml`)
- [x] `--source auto` (forgejo-first, origin fallback), `--check` parity, `--parallel N`
- [ ] Signal graph registration in capability registry (23 signals total: 15 original + 5 rootPulse + 3 ecosystem)
- [x] Cross-gate graph executor spec: `specs/CROSS_GATE_GRAPH_EXECUTOR.md` (gate/relay hints on graph nodes)

### Wave 63: impulsePotential — Inter-Gate Coordination Substrate
- [x] `signal.*` commands renamed to `impulse.*` / `potential.*` in membrane-shadow
- [x] Triad mapping: `impulse.post/ack` → rootPulse (ACTION), `potential.sense/check` → quorumSignal (SENSE), `impulse.archive` + propagation → waterFall (SYNC)
- [x] `impulse.post` auto-populates `[from].ref` with project HEAD SHA (rootPulse DAG provenance)
- [x] `potential.sense --count` — lightweight integer output for cascade-pull integration
- [x] `potential.check` — membrane gradient health (expired, unacked, volume per wave)
- [x] `cascade-pull.sh` auto-triggers `potential.sense` after sync
- [x] `signals/` directory → `impulses/` in wateringHole
- [x] `SIGNAL_FRAGO_STANDARD.md` → `IMPULSE_POTENTIAL_STANDARD.md`
- [x] Backward compatibility: `signal.*` aliases print deprecation notice for one wave
- [x] TOML parser reads both `[signal]` and `[impulse]` table names (no migration required)
- [ ] Phase 2: Forgejo webhook → peptidoglycan impulse-relay → Songbird mesh.publish

### Wave 43+: Provider Selection + Graph Optimization
- [ ] Weighted provider selection in `discover_capability` (currently first-match)
- [ ] Co-occurrence analysis (which capabilities are called together)
- [ ] Graph pre-staging (dependency validation pattern, already prototyped)
- [ ] PathwayLearner suggestions → actual weight table updates

### Wave 46+: Learned Routing (horizon)
- [ ] Weight matrix as input to gradient-free optimizer
- [ ] A/B shadow comparison (already prototyped in `validation::shadow`)
- [ ] Self-healing: automatic rerouting on primal failure
- [ ] Self-announcing primals dynamically reshape routing topology

---

## Coordination Domains: quorumSignal / rootPulse / waterFall

The Neural API orchestrates three coordination domains — the ecosystem's
nervous system. Each domain uses the 5 `CoordinationPattern` variants
(Sequential, Parallel, ConditionalDag, Pipeline, Continuous) as execution
strategies, but serves a distinct biological purpose.

### quorumSignal — SENSE (afferent)

How the ecosystem observes, discovers, and reacts. Named after bacterial
quorum sensing: a signal only has meaning when enough primals participate
in the composition to form consensus. The quorum is the minimum primal
set for an atomic operation to be valid — Tower quorum is 3 (bearDog +
songbird + skunkBat), Nest quorum is 4 (nestGate + trio), Full NUCLEUS
quorum is 13.

- 23 atomic signal graphs in `graphs/signals/` across 6 tiers (Tower/Node/Nest/Meta/rootPulse/Ecosystem)
- `signal.dispatch` collapses N-squared primal IPC to one semantic call
- Primarily Sequential and Parallel coordination patterns
- `CoordinationDomain::Signal` in `graphs/mod.rs`

### rootPulse — ACTION (efferent)

How the ecosystem creates, mutates, and proves. Emergent VCS over the
provenance trio — nothing in-tree implements `git`; version control
emerges from graph-orchestrated capability calls across sovereign primals.

- 6 primals compose: rhizoCrypt (DAG) + loamSpine (ledger) + sweetGrass (attribution) + bearDog (signing) + nestGate (storage) + songbird (federation)
- 5 logical operations: commit, branch, merge, diff, federate
- `nest.commit` signal graph + `rootpulse_commit` composition pattern
- Sequential today; federation could use Pipeline (streaming cross-site)
- `CoordinationDomain::Pulse` in `graphs/mod.rs`

### waterFall — SYNC (autonomic)

How the ecosystem maintains coherence across gates. Multi-repo alignment,
freshness, federation through the VPS periplasm (golgiBody). Autonomic
because gates should sync without manual intervention — like a heartbeat
keeping code coherent.

- Currently bash (`cascade-pull.sh`) + Forgejo SSH + `ecosystem_manifest.toml`
- Gate profiles drive scoped pulls: eastGate (38 repos), ironGate (~20), etc.
- Evolving toward Neural API `ecosystem` signal tier (ecosystem.pull/push/check)
- Target: Parallel (concurrent repo pulls) with ConditionalDag (skip unchanged)
- `CoordinationDomain::Fall` in `graphs/mod.rs`

### impulsePotential — Cross-Domain Coordination Substrate

The inter-gate coordination primitive that spans all three triad domains.
Named after biological action potentials across cell membranes:

- **Impulse** = discrete event (rootPulse fires it via `impulse.post`)
- **Potential** = measurable state (quorumSignal senses it via `potential.sense`)
- **Propagation** = transport through membrane (waterFall carries it via git push)

```
impulse.post (rP ACTION) → impulses/active/*.toml → git push (wF SYNC)
                                    ↑
potential.sense (qS SENSE) ─────────┘
potential.check (qS SENSE) ─── gradient health across mesh
impulse.ack (rP+wF) ──────── receptor binding + propagation
impulse.archive (wF) ──────── discharge spent impulses
```

Detailed standard: `infra/wateringHole/IMPULSE_POTENTIAL_STANDARD.md`

### Triad Relationship

```
                    quorumSignal (sense)
                         |
                    signal.dispatch (23 atomic graphs)
                    potential.sense / potential.check
                         |
          +--------------+--------------+
          |                             |
    rootPulse (action)          waterFall (sync)
    nest.commit / federate      ecosystem.pull / push
    impulse.post / impulse.ack  impulse.archive / cascade-pull
    provenance trio             gate-profile cascade
    within NUCLEUS              across gates
```

Layer 2 (graph composition) serves all three domains. The distinction is
operational: quorumSignal is reactive sensing, rootPulse is creative
mutation, waterFall is autonomous coherence. All three collapse to
`graph.execute` at runtime. impulsePotential is the cross-domain substrate
that enables inter-gate coordination to flow through the triad.

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
