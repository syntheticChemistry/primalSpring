# primalSpring v0.9.25 Evolution Handoff — May 9, 2026

**From**: primalSpring (syntheticChemistry)
**To**: Upstream primal teams (ecoPrimals), delta spring teams (syntheticChemistry), downstream products (sporeGarden)
**Scope**: Primal usage review, evolution debt, composition patterns, NUCLEUS deployment, spring absorption guidance

---

## 1. primalSpring's Primal Consumption — Current State

primalSpring validates all 13 NUCLEUS primals via IPC. It does NOT link any primal as a Rust dependency — all interaction is over JSON-RPC via UDS or TCP. This is the pattern all springs should converge toward.

### Per-Primal Usage Summary

| Primal | Role in primalSpring | Key Methods Used | Validated By |
|--------|---------------------|------------------|-------------|
| **BearDog** | Trust anchor — crypto, BTSP handshake, ionic tokens | `crypto.hash`, `crypto.sign_ed25519`, `btsp.session.*`, `auth.*` | exp085, exp086, exp094 |
| **Songbird** | Network — discovery, mesh, HTTPS, BirdSong | `discovery.resolve`, `mesh.init`, `mesh.announce`, `http.get` | exp063, exp073, exp090, exp094 |
| **ToadStool** | Compute substrate — dispatch, workload execution | `compute.dispatch`, `compute.status`, `ember.*` | exp067, exp087, exp094 |
| **barraCuda** | Math/ML — stats, tensor, ODE, ML, nautilus | `stats.mean`, `tensor.matmul`, `ml.*`, `nautilus.*` | exp067, exp091, exp094 |
| **coralReef** | GPU — shader compilation and dispatch | `shader.compile`, `shader.dispatch`, `shader.capabilities` | exp067, exp094 |
| **NestGate** | Storage — content-addressed store/retrieve | `storage.store`, `storage.retrieve`, `storage.list` | exp068, exp094, exp107 |
| **rhizoCrypt** | DAG — provenance sessions, dehydration | `dag.session.create`, `dag.add_node`, `dag.dehydration.trigger` | exp020-022, exp094, exp107 |
| **loamSpine** | Ledger — commit, audit, spine management | `ledger.commit`, `ledger.audit`, `ledger.spine.create` | exp020, exp094, exp107 |
| **sweetGrass** | Attribution — braid, witness, anchor | `attribution.braid`, `attribution.witness`, `attribution.anchor` | exp020, exp094, exp107 |
| **Squirrel** | AI/MCP — inference, context, tool execution | `ai.query`, `inference.complete`, `mcp.tools.list` | exp069-070, exp094 |
| **petalTongue** | UI — scene graph, rendering, interaction | `visualization.render_scene`, `interaction.*`, `proprioception.*` | exp088, exp099-106 |
| **biomeOS** | Orchestrator — Neural API, graph execution | `capability.call`, `graph.list`, `graph.execute`, `topology.rescan` | exp075-080, exp094, exp107 |
| **skunkBat** | Defense — log aggregation, audit | `defense.log`, `defense.audit` | exp094, guidestone |

### Composition Experiments

| Experiment | Composition | What It Validates |
|-----------|-------------|-------------------|
| exp094 | Full NUCLEUS (13 primals) | Tower+Node+Nest+Cross-Atomic pipeline: hash→store→retrieve→verify |
| exp085 | BearDog solo | Crypto lifecycle (Ed25519 keygen, sign, verify, BLAKE3, BirdSong beacon) |
| exp086 | BearDog + Songbird | Genetic identity E2E (mito beacon, nuclear lineage, family scoping) |
| exp087 | Neural API → 5 domains | Security, discovery, storage, compute, AI routing through biomeOS |
| exp088 | ludoSpring + Squirrel + petalTongue | Storytelling composition (AI DM, scene rendering, interaction) |
| exp107 | Foundation pipeline | 8-phase: structural→discovery→health→provenance→storage→compute→ledger→attribution |

---

## 2. Upstream Primal Evolution Debt

Items primalSpring has identified that need primal-team attention.

### Critical / High

| Primal | Gap | Impact | Recommended Fix |
|--------|-----|--------|----------------|
| **biomeOS** | `nucleus --mode full` only launches 5 primals | Full 13-primal NUCLEUS requires manual launch or `composition_nucleus.sh` | Extend `nucleus` mode to handle all 13 via `nucleus_complete.toml` graph |
| **biomeOS** | TCP transport for mobile (Android/GrapheneOS) | SELinux blocks UDS — Pixel deployments degraded | `--tcp-only` needs to propagate TCP endpoints to all spawned primals |
| **biomeOS** | `capability.call` lacks gate-aware routing | Multi-gate federation can't route to specific gates | Honor `gate` param in `capability.call`, route via registered gate endpoints |

### Medium

| Primal | Gap | Impact | Recommended Fix |
|--------|-----|--------|----------------|
| **BearDog** | No `--listen <addr>` TCP-only server mode | Mobile/SELinux environments can't use BearDog | Add TCP listen flag alongside UDS |
| **petalTongue** | BTSP server convergence late | Was last primal to converge on BTSP Phase 3 | Resolved — monitor for regression |
| **coralReef** | Uses tarpc transport, not family-namespaced sockets | Discovery inconsistency with UDS convention | Align socket naming to `{name}-{family}.sock` pattern |
| **Songbird** | BirdSong beacon privacy model incomplete | Cross-family beacon leakage not prevented | Implement encrypted beacon payloads per mito-tier spec |

### Low / Monitoring

| Primal | Note |
|--------|------|
| **All 13** | JH-0 MethodGate adopted 13/13 — monitor for regressions on new method additions |
| **All 13** | BTSP Phase 3 AEAD converged 13/13 — no cleartext paths remain |
| **All 13** | 389 methods registered in `capability_registry.toml` — new methods must be registered before shipping |

---

## 3. Spring Delta Team Handoffs

### Universal Guidance (All 7 Delta Springs)

**barraCuda path dep migration**: All springs still link barraCuda as a Rust library dependency. The sovereign pattern is IPC via `CompositionContext`:

```rust
use ecoprimals::composition::CompositionContext;

let ctx = CompositionContext::discover().await?;
let result = ctx.call("stats.mean", json!({"values": data})).await?;
```

4 of 7 springs are actively building IPC clients: hotSpring (9 probes), healthSpring (2/11 via `math_dispatch`), neuralSpring (`IpcMathClient`, 9 methods), wetSpring (5 primals).

**`deny.toml` hardening**: Ban `ring`, `openssl`, `aws-lc-sys`, `cmake`, `cc`, `pkg-config`. primalSpring's `deny.toml` is the reference.

**Guidestone adoption**: Build a `{spring}_guidestone` binary following primalSpring's 9-layer model. hotSpring is Level 5 (certified). All others should target Level 3+ (IPC parity validation).

**Capability registry sync**: Each spring should cross-test its method strings against `primalSpring/config/capability_registry.toml` (389 methods). Use `tools/check_method_strings.sh` as a template.

### Per-Spring Notes

| Spring | Key Evolution Target | primalSpring Pattern to Absorb |
|--------|---------------------|-------------------------------|
| **hotSpring** | Level 5 guidestone certified. Lead spring. | Maintain — absorb `CompositionContext` for barraCuda IPC |
| **healthSpring** | 2/11 IPC methods via `math_dispatch` | Expand to full barraCuda IPC; absorb `check_relative()` for PK/PD tolerance |
| **neuralSpring** | `IpcMathClient` with 9 methods wired | Nearest to IPC parity — absorb `CompositionContext` wrapper |
| **wetSpring** | 5 primals in composition, largest test suite (5,707+) | Absorb `NdjsonSink` for CI integration; add composition parity experiment |
| **airSpring** | `data::Provider` / `data::NestGateProvider` API drift | Fix internal API, then adopt IPC pattern |
| **groundSpring** | 965+ tests, strong measurement science | Add guidestone binary; absorb `ValidationSink` enrichments from primalSpring |
| **ludoSpring** | 8 game science IPC methods | Complete `session.*` and `rules.*` method surface for esotericWebb |

---

## 4. Composition Patterns for NUCLEUS

### The primalSpring Composition Model

```
Python baseline → Rust port → Primal composition (IPC)
                                      ↓
                              Deploy graph (TOML)
                                      ↓
                              biomeOS Neural API
                                      ↓
                              NUCLEUS (13 primals)
```

### Deploy Graph Pattern (Fragment-First)

primalSpring pioneered **fragment-first composition**: small atomic graphs (tower, node, nest, meta, provenance) composed into profiles via `resolve = true`. This is now the canonical pattern:

```toml
# profiles/nucleus.toml — thin composition
[graph]
name = "NUCLEUS Complete"
id = "nucleus-complete"
resolve = true

[[graph.nodes]]
name = "tower_fragment"
source = "fragments/tower_atomic.toml"

[[graph.nodes]]
name = "node_fragment"
source = "fragments/node_atomic.toml"

[[graph.nodes]]
name = "nest_fragment"
source = "fragments/nest_atomic.toml"
```

Springs define **cell graphs** that overlay domain logic on top of NUCLEUS:

```toml
# cells/hotspring_cell.toml
[graph]
name = "hotSpring Cell"
id = "hotspring-cell"
resolve = true
coordination = "continuous"

[[graph.nodes]]
name = "nucleus"
source = "profiles/nucleus.toml"

[[graph.nodes]]
name = "hotspring_primal"
binary = "hotspring"
capabilities = ["physics.plasma", "physics.qcd", "physics.spectral"]
```

### Neural API Deployment

biomeOS's Neural API is the deployment surface. The pipeline:

1. `tools/fetch_primals.sh` — download plasmidBin binaries
2. `tools/desktop_nucleus.sh` — launch 13-primal NUCLEUS (biomeOS-first, fallback to `composition_nucleus.sh`)
3. biomeOS loads cell graph, spawns primals, registers capabilities
4. Spring connects via `CompositionContext::discover()` and calls methods via Neural API routing

### Shell Composition Library

`tools/nucleus_composition_lib.sh` provides 41 reusable bash functions for rapid interactive prototyping against live NUCLEUS. Springs can `source` this library for exploration before committing to graph-based deployments. See `tools/composition_template.sh` for the minimal starter.

---

## 5. Downstream Product Guidance

### esotericWebb (sporeGarden)

esotericWebb is a CRPG narrative engine that composes as a graph-defined product. primalSpring provides:

- **Cell graph template**: `graphs/cells/` — esotericWebb should define its own cell graph overlaying on NUCLEUS
- **Composition subsystems C1-C7**: primalSpring validated all 7 subsystems (Render, Narration, Session, Game Science, Persistence, Proprioception, Full Interactive). esotericWebb consumes C1/C3/C4/C7 directly.
- **ludoSpring IPC contract**: 8 game science methods (`session.*`, `rules.*`, `narrative.*`) — esotericWebb is the primary consumer
- **petalTongue scene graph**: `visualization.render_scene` + `interaction.*` for UI. See `tools/ttt_composition.sh` as reference implementation.

### projectNUCLEUS (sporeGarden)

projectNUCLEUS is the multi-user deployment product. primalSpring's JH-0 through JH-5 hardening work directly enables it:

- **JH-0**: MethodGate capability check — 13/13 primals adopted. Every RPC call is pre-authorized against capability whitelist.
- **JH-1**: BearDog ionic tokens (Ed25519-signed, scoped, expiry). Session management for multi-user.
- **JH-2**: Token-carried resource envelopes (biomeOS + ToadStool enforcement). Per-user compute quotas.
- **JH-3**: Composition hot-reload (`composition.reload`). Runtime graph updates without restart.
- **JH-4**: Auth UX (`auth.issue_session`). BearDog + primalSpring token issuance flow.
- **JH-5**: skunkBat log aggregation + provenance pipeline. Audit trail for multi-user.

### sporePrint (ecoPrimals/infra)

primalSpring's `sporeprint/validation-summary.md` auto-syncs to primals.eco via the auto-refresh pipeline. Keep this file current. Notebooks in `notebooks/` render to lab pages. The CI content job (fixed today) will auto-PR new content.

---

## 6. What We Learned

### Composition Convergence Patterns

1. **Fragment-first scales**: 74 graphs from 6 fragments. New compositions are thin profiles, not new graphs.
2. **Capability-first discovery eliminates hardcoding**: `discover_by_capability()` + `primal_names::` constants = zero hardcoded socket paths in experiments.
3. **BTSP must be default, not opt-in**: The Phase 45c "BTSP default everywhere" decision caught 8 relay primals that weren't doing AEAD. Cleartext should always FAIL.
4. **MethodGate is the access control boundary**: JH-0 adoption took 13/13 primals from "all methods public" to "capability-whitelisted". This is the foundation for multi-user.
5. **Daily CI catch-up prevents dispatch drops**: `plasmidBin` auto-harvest added daily cron after discovering GitHub `repository_dispatch` rate limiting silently drops events (~10/min/repo).
6. **The meta-spring pattern works**: primalSpring's "science IS coordination" approach caught 14 upstream sovereignty gaps, drove JH-0 adoption across all primals, and established the composition parity toolkit that springs now use.

### Known Limitations

- **No compile-time primal API verification**: All IPC is string-based JSON-RPC. Method signature drift is caught by `check_method_strings.sh`, not the type system.
- **48 ignored tests**: These require live primals. They pass when NUCLEUS is running but are skipped in CI.
- **barraCuda path dep universal**: Despite IPC being the sovereign pattern, all 7 springs still link barraCuda as a Rust library. Full migration is the single largest ecosystem-wide evolution remaining.

---

**Generated**: May 9, 2026 — primalSpring v0.9.25, Phase 60+
**License**: CC-BY-SA 4.0
