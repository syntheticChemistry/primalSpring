# Composition Broker — Validated Patterns from the Ecosystem

**Owner**: primalSpring (validation) — patterns proven by gate teams
**Status**: Operational — continuously validated
**Wave**: 157a (Aug 7, 2026)
**Evidence**: `whitePaper/subGen/COMPOSITION_BROKER_THRESHOLD.md`, gate AARs

---

## What the Composition Broker Is

biomeOS acts as the composition broker for all primal coordination within a
gate. It maps semantic capability calls to physical primal endpoints, executes
composition graphs, and propagates BTSP trust across atomic boundaries.

primalSpring validates that the broker works correctly. This spec codifies the
patterns that gate teams have already proven in production and that primalSpring
must exercise via experiments.

---

## Broker Threshold (CROSSED — Wave 155i)

The composition broker threshold was crossed on Jul 29, 2026. Evidence:

| Metric | Value | Source |
|--------|-------|--------|
| Capabilities registered | **704+** | biomeOS COORDINATED mode |
| Capability domains | 10+ (security, discovery, compute, storage, dag, spine, braid, ai, mesh, http, ...) | `config/capability_registry.toml` |
| Mode | **COORDINATED** (not Bootstrap) | biomeOS runtime state |
| E2E routing proven | `content.put` → nestGate via capability.call | westGate deployment |
| Signal graphs | 27+ across Tower/Node/Nest/NUCLEUS/Meta tiers | biomeOS `config/signal_tools.toml` |
| Provenance E2E | **7/7** (bearDog sign → full trio chain) | westGate + blueGate validation |

### Atomic Signal Collapse

The 490+ method surface collapses into ~32 atomic signals across composition
tiers. Springs call one signal; biomeOS decomposes into primal graphs:

| Tier | Signal examples | Primals orchestrated |
|------|----------------|---------------------|
| Tower | `tower.publish`, `tower.health` | bearDog + songBird + skunkBat |
| Node | `node.compute`, `node.compile` | + toadStool + barraCuda + coralReef |
| Nest | `nest.store`, `nest.commit` | + nestGate + rhizoCrypt + loamSpine + sweetGrass |
| NUCLEUS | `nucleus.deploy`, `nucleus.health` | All 13 |
| Meta | `meta.intent`, `meta.render` | squirrel + petalTongue |

The collapse is Phase A of the Neural API evolution. Phase B (NeuralRouter
signal-tier dispatch) wires `signal.dispatch("tower", "publish")` to graph
loading. Phase A is complete; Phase B is pending in biomeOS.

primalSpring validates this via `CompositionContext::composition(tier, signal, params)`
which prefers `signal.dispatch`, falling back to `capability.call`.

---

## Session-Scoped Provenance Model

Lesson from westGate (Aug 4-6, 2026): per-file spine commits are pathological.
The canonical provenance flow is session-scoped:

```
Per file:   BLAKE3 hash → nestGate CAS → rhizoCrypt dag.event.append (~4ms)
Per session: dehydrate → session.commit → bearDog.sign → sweetGrass.braid
```

### Why session-scoped

westGate processes 11M+ files across 153 datasets. Per-file spine commits
caused:
- Convoy throughput: 0.3 files/sec (pathological)
- Spine bloat: one immutable entry per file
- Attribution noise: one braid per file instead of per-dataset

Session-scoped commits fixed this:
- Convoy throughput: **217 files/sec** (~700x improvement)
- Spine entries: one per dataset session
- Braids: meaningful per-dataset attribution

### Canonical provenance flow

```
1. session.create → rhizoCrypt allocates ephemeral DAG session
2. Per file: content.put → nestGate CAS (BLAKE3 addressed)
3. Per file: dag.event.append → rhizoCrypt ephemeral DAG (~4ms)
4. session.commit → rhizoCrypt dehydrates DAG → Merkle root
5. session.commit → loamSpine permanent spine entry (immutable)
6. crypto.sign → bearDog Ed25519 signs commit
7. braid.create → sweetGrass W3C PROV-O attribution braid
```

Phases 1-3 are per-file (fast, lock-free). Phases 4-7 are per-session
(one-time cost amortized across all files in the session).

### primalSpring validation

- `trio_ops` helper crate exercises the session-scoped pattern
- `exp020_rootpulse_commit` validates the full 7-phase flow
- `exp041_provenance_trio_science` validates cross-domain provenance

---

## Compute Trio IPC Pattern

Proven by hotSpring on strandGate (Dual EPYC + RTX 3090 + RX 6950 XT).
The compute trio coordinates exclusively via IPC — no cross-primal Rust imports.

| Primal | Role | Analogy |
|--------|------|---------|
| barraCuda | WHAT — math, WGSL shaders, linear algebra | The equation |
| coralReef | HOW — compile WGSL to native GPU code | The compiler |
| toadStool | WHERE — discover hardware, dispatch work | The scheduler |

### Compute trio flow

```
Spring → shader.compile.wgsl → coralReef → compiled binary
      → compute.dispatch.submit → toadStool → GPU execution
      → compute.dispatch.result → verify + BLAKE3 witness
      → rhizoCrypt dag.event (provenance)
```

Key IPC methods:
- `shader.compile.wgsl` (coralReef) — compile WGSL shader source to native
- `compute.dispatch.capabilities` (toadStool) — discover available hardware
- `compute.dispatch.submit` (toadStool) — submit compiled shader for execution
- `compute.dispatch.result` (toadStool) — collect results with verification

### primalSpring validation

- `exp117_compute_trio_routing` validates the full flow through Neural API
- `exp050_compute_triangle` validates structural compute trio composition
- `exp094_composition_parity` includes cross-atomic hash→store pipeline

---

## Provenance Trio Architecture

Three primals compose the provenance stack. Each owns one temporal model:

| Primal | Temporal model | Role | Biological analogy |
|--------|---------------|------|--------------------|
| rhizoCrypt | Ephemeral DAG (present/future) | Lock-free staging, Merkle roots | Mycelium — exploratory, branching |
| loamSpine | Immutable linear history (past) | Append-only, provable | Fossil record — permanent, ordered |
| sweetGrass | W3C PROV-O attribution | Semantic braids, not line counts | Root system — nutrient attribution |

### Two-tier temporal architecture

```
DAG (ephemeral, fast) → dehydration → linear spine (permanent)
```

Dehydration is the temporal collapse from flexible present (rhizoCrypt DAG) to
immutable past (loamSpine spine). This separation is fundamental: the DAG allows
speculative work without committing to history; dehydration freezes the
speculation into permanent record.

### strandGate convergence

strandGate (QCD) uses the same BLAKE3 CAS + braid structure as westGate (NAS)
but with different access patterns (~100 large configs vs 11M+ small files).
Cross-gate provenance is achieved by hash reference: westGate data braids can
reference strandGate compute braids via BLAKE3 hash → full provenance chain.

---

## Cross-References

| Document | Relationship |
|----------|-------------|
| `whitePaper/subGen/COMPOSITION_BROKER_THRESHOLD.md` | Evidence: broker threshold crossed |
| `whitePaper/subGen/PROVENANCE_TRIO_ARCHITECTURE.md` | Session-scoped commit model |
| `whitePaper/subGen/DATA_FEDERATION_STATUS.md` | 519 GB federated, declare→acquire→complete |
| `whitePaper/RootPulse/` | Canonical RootPulse graphs and architecture |
| `infra/wateringHole/handoffs/WESTGATE_STORAGE_TIERS_AUG05_2026.md` | 4-tier storage, convoy fix |
| `infra/wateringHole/handoffs/HOTSPRING_PRIMAL_DEDUPLICATION_HANDOFF_AUG07.md` | Compute trio patterns |
| `specs/STAGE2_ACTIVATION.md` | G67 activation tasks |
| `specs/MIXED_COMPOSITION_PATTERNS.md` | Particle model (Tower=electron, Node=proton, Nest=neutron) |
