# Upstream Primal Cross-Talk & Downstream Absorption Patterns

**Date**: April 16, 2026
**From**: primalSpring v0.9.17 (Phase 45)
**Phase**: **INTERSTADIAL** — stadial gate cleared, downstream absorption open
**License**: AGPL-3.0-or-later

---

## Purpose

This document defines two interfaces:

1. **Upstream cross-talk** — how primals maintain composition compatibility
   with each other without compile-time coupling
2. **Downstream absorption** — how springs and gardens ingest NUCLEUS
   patterns for their domain work

---

## Part 1: Upstream Primal Cross-Talk

### The Rule

Primals never import each other. All cross-primal interaction happens via:
- JSON-RPC 2.0 over UDS or TCP (capability-routed by biomeOS Neural API)
- biomeOS graph execution (`graph.execute` sequences capability calls)
- Environment variable conventions for socket discovery

### Protocol Auto-Detection (Required)

Every primal accepting socket connections MUST implement first-byte peek
to auto-detect the incoming protocol:

```
Incoming byte | Protocol     | Action
──────────────┼──────────────┼─────────────────────────────
0x7B ('{')    | JSON-RPC     | Bypass BTSP, route to JSON-RPC handler
Other         | BTSP binary  | Proceed with BTSP handshake
```

**Why**: biomeOS routes `capability.call` RPCs as plain JSON-RPC over UDS/TCP.
If a primal enforces BTSP unconditionally, biomeOS forwarding breaks. The
first-byte peek allows both local composition (JSON-RPC) and remote/secure
connections (BTSP) on the same socket.

**Implemented in**: BearDog (`tcp_ipc/server.rs`), NestGate
(`isomorphic_ipc/server.rs`, `unix_socket_server/mod.rs`).

**Pattern** (Rust + tokio):
```rust
let mut reader = BufReader::new(stream);
let is_json_rpc = match reader.fill_buf().await {
    Ok(buf) if !buf.is_empty() => buf[0] == b'{',
    _ => false,
};
if is_json_rpc {
    handle_json_rpc(&mut reader, &mut writer, &handler).await
} else {
    handle_btsp_handshake(&mut reader, &mut writer, &handler).await
}
```

### Socket Naming Convention

Primals MUST resolve sockets via the `FAMILY_ID`-aware convention:

```
$XDG_RUNTIME_DIR/biomeos/{primal}-{family_id}.sock
```

When `--family-id` is passed as a CLI argument, it takes precedence over
`FAMILY_ID` environment variable. biomeOS's `SocketNucleation` uses this
for deterministic socket assignment.

### Capability Registration Contract

Every primal MUST expose:
- `health.liveness` — am I alive?
- `health.readiness` — am I ready to serve?
- `health.check` — full self-diagnostic
- `capabilities.list` — enumerate all exposed methods

biomeOS discovers primals via these probes and registers them in the
`CapabilityTranslationRegistry`. The registry maps semantic capabilities
(e.g., `storage.store`) to transport endpoints (UDS path or TCP address).

### Family-ID Propagation

When a primal receives `--family-id` via CLI, it MUST:
1. Use it for socket path resolution (not just `$FAMILY_ID` env var)
2. Pass it to any internal registries or translation loading
3. Thread it explicitly — do NOT rely on `std::env::set_var` (violates
   `#![forbid(unsafe_code)]`)

**Lesson learned**: biomeOS's `NeuralApiServer` initially failed to propagate
`--family-id` to its `CapabilityTranslationRegistry`, causing 4 capability
domains to route to `-default.sock`. The fix was explicit parameter threading.

### BTSP Two-Phase Genetics

Cross-primal connections requiring authentication follow the two-phase model:

| Phase | Purpose | Genetics Tier | When |
|-------|---------|---------------|------|
| Phase 1 | Mito-beacon tunnel (discovery) | Tier 1 (inherited, cloneable) | Always (establishes encrypted channel) |
| Phase 2 | Nuclear session (permissions) | Tier 2 (spawned fresh, !Clone) | Permission-bearing operations only |

Phase 1 never exposes Phase 2 credentials. Primals MUST NOT mix these tiers.

### Graph Executor Contract

When biomeOS executes a graph that spawns or calls a primal:
- `graph.execute` sends `capability_call` RPCs to registered providers
- Primals receive standard JSON-RPC calls — they don't know about "graphs"
- The graph executor reports per-node success/failure via `graph.status`
- `completed_nodes` and `failed_nodes` are returned with error messages

Primals should be stateless with respect to graph execution context. The
graph is the program; the primal is the instruction.

---

## Part 2: Downstream Absorption Patterns

### For Springs (Domain-Specific Applications)

Springs absorb NUCLEUS patterns through a maturity ladder:

```
Level 1: Python baseline        — reference implementation from peer-reviewed science
Level 2: Rust validation        — faithful port, pass/fail, tolerance-gated
Level 3: barraCuda CPU          — same math via primal IPC (WGSL CPU fallback)
Level 4: barraCuda GPU          — sovereign shader execution on hardware
Level 5: Primal composition     — ALL math via NUCLEUS primals (no local Rust math)
Level 6: Deploy graph validated — proto-nucleate satisfied, all nodes healthy via biomeOS
```

At Level 5+, the spring's binary becomes fossil record. The graph IS the product.

### Absorption Checklist for Springs

1. **Read your proto-nucleate**: `graphs/downstream/{yourspring}_*_proto_nucleate.toml`
2. **Identify required primals**: which are `required = true` for your domain?
3. **Wire IPC via CompositionContext**:
   ```rust
   let ctx = CompositionContext::from_live_discovery_with_fallback();
   ```
   This tries UDS discovery first, then falls back to TCP via env vars.
4. **Call by capability, not identity**: `ctx.call("tensor", "tensor.matmul", params)`
5. **Validate parity**: compare primal results against your Python/Rust baselines
6. **Deploy via biomeOS graph**: use `biomeos neural-api` with your deploy graph
7. **Document gaps**: hand back to primalSpring `docs/PRIMAL_GAPS.md`

### Fragment Composition

Springs compose NUCLEUS using fragments from `graphs/fragments/`:

| Fragment | Contains | Use When |
|----------|----------|----------|
| `tower_atomic` | BearDog + Songbird | Always (security + discovery) |
| `node_atomic` | Tower + ToadStool + barraCuda + coralReef | Your domain needs compute |
| `nest_atomic` | Tower + NestGate + Provenance Trio | Your domain needs storage |
| `nucleus` | Tower + Node + Nest (includes all three) | Full platform |
| `meta_tier` | biomeOS + Squirrel + petalTongue | AI + orchestration + UI |
| `provenance_trio` | rhizoCrypt + loamSpine + sweetGrass | Audit trails + attribution |

Spring deploy graphs declare fragments via `[graph.metadata] fragments = [...]`.

### Bonding Patterns for Multi-Site Deployment

| Bond | Trust Model | Use Case | Genetics Tier |
|------|-------------|----------|---------------|
| Covalent | `NuclearLineage` | Same family, same trust | Tier 2 (nuclear) |
| Ionic | `MitoBeaconFamily` | Cross-family capability sharing | Tier 1 (mito-beacon) |
| Metallic | `MitoBeaconFamily` | Shared compute fleet | Tier 1 (mito-beacon) |

See `graphs/bonding/` and `graphs/multi_node/` for deployment patterns.

### Key Library Patterns to Absorb from ecoPrimal

| Pattern | Module | What It Does |
|---------|--------|--------------|
| `CompositionContext` | `composition/mod.rs` | Capability-keyed IPC client set |
| `validate_parity` | `composition/mod.rs` | Compare local baseline vs primal result |
| `ValidationResult` | `validation/mod.rs` | Builder-pattern check harness |
| `CircuitBreaker` | `ipc/resilience.rs` | Fault tolerance for IPC calls |
| `Transport` | `ipc/transport.rs` | Unified Unix+TCP transport |
| `tcp_rpc_multi_protocol` | `ipc/tcp.rs` | Auto-detect raw TCP vs HTTP POST |
| `MitoBeacon` / `NuclearGenetics` | `genetics/` | Three-tier identity types |

### For Gardens (End Products)

Gardens (esotericWebb, helixVision) follow the pure composition model:
- `composition_model = "pure"` in graph metadata
- All nodes are `spawn = false` — biomeOS manages full lifecycle
- No garden binary ships — the graph IS the product
- The garden is a deploy graph + configuration, not a codebase

### Shell Composition Path (Interactive Exploration)

For rapid interactive exploration before committing to graph-based deployment,
springs can use the **shell composition library**:

```bash
# Copy the template, fill in domain hooks
cp primalSpring/tools/composition_template.sh my_composition.sh

# Launch NUCLEUS from plasmidBin
COMPOSITION_NAME=myspring primalSpring/tools/composition_nucleus.sh start

# Run your composition
COMPOSITION_NAME=myspring bash my_composition.sh
```

The shell composition library (`tools/nucleus_composition_lib.sh`) provides
the same capability stack as the Rust `CompositionContext`: discovery,
JSON-RPC transport, DAG sessions, ledger spines, braid provenance, and
petalTongue interaction — but in bash, with immediate feedback.

See `wateringHole/DOWNSTREAM_COMPOSITION_EXPLORER_GUIDE.md` for per-spring
exploration lanes and the convergent evolution model.

### Convergent Evolution Model

Springs explore complementary aspects of the composition pattern:
- **ludoSpring**: interaction fidelity, real-time feedback, petalTongue stress
- **hotSpring**: async computation, DAG memoization, scientific provenance
- **wetSpring**: data visualization, large-state navigation, storage integration
- **neuralSpring**: agentic composition, inference pipeline, AI provenance

Each spring discovers domain-specific patterns. primalSpring absorbs and
abstracts proven patterns back into the library for cross-domain reuse.
This is structured convergent evolution — messy but powerfully robust.

### What to Hand Back

When your spring or garden discovers a gap:
1. Document it in primalSpring `docs/PRIMAL_GAPS.md`
2. Propose the wire (JSON-RPC method signature)
3. Build a primalSpring experiment or graph that tests the gap
4. Submit via PR — primalSpring triages and routes to the responsible primal
5. Document discovered patterns — tick rates, interaction models, visualization
   approaches that worked for your domain (candidates for lib promotion)

---

## Part 3: Interstadial Standards (April 16, 2026)

The stadial gate has cleared — all 13 primals at modern async Rust parity.
The following standards are **permanent invariants** enforced going forward.
See `STADIAL_PARITY_GATE_APR16_2026.md` for the full specification.

### Non-negotiable invariants

- **No `async-trait`** — banned in `deny.toml`, use native `async fn` (RPITIT)
- **No `Box<dyn Trait>` for finite implementors** — use enum dispatch
- **`cargo deny check bans` PASS** — C/FFI bans enforced
- **Edition 2024** — all crates
- **First-byte peek** — every socket-accepting primal auto-detects JSON-RPC vs BTSP
- **Tower atomic delegation** — Songbird provides TLS, BearDog provides crypto.
  Primals do not pull `reqwest`, `hyper-rustls`, or `ring` directly.

### PR review checklist (for upstream primal teams)

- [ ] No `#[async_trait]` introduced
- [ ] No `Box<dyn T>` for finite-implementor traits
- [ ] `cargo deny check bans` passes
- [ ] No new banned crate transitive deps in `Cargo.lock`
- [ ] `#[expect(...)]` preferred over `#[allow(...)]`
- [ ] Public APIs use typed errors, not `Box<dyn Error>`
- [ ] New files < 1000 LOC
- [ ] Coverage maintained or improved

### Downstream absorption: now open

Springs and gardens may now absorb NUCLEUS patterns. The maturity ladder above
applies. Start with your proto-nucleate graph, wire IPC via `CompositionContext`,
validate parity against baselines, then deploy via biomeOS. Hand gaps back to
primalSpring.

---

## Part 4: Graph Deployment Subsystem Inventory (April 16, 2026)

primalSpring owns **71 TOML deploy graphs** (consolidated via template+manifest and fragment-first composition) — the standard subsystems for everything
downstream. All graphs use capability-first routing (`by_capability`), parse cleanly
(37 graph unit tests + 4 overlay composition tests), and are topologically acyclic.

### Atomic Fragments (6)

The building blocks every composition inherits:

| Fragment | Primals | Role |
|----------|---------|------|
| `tower_atomic` | BearDog + Songbird | Security + TLS |
| `node_atomic` | Tower + compute triangle | Compute capability |
| `nest_atomic` | Tower + Nest + provenance trio | Storage + provenance |
| `nucleus` | Full NUCLEUS | All 10+ primals |
| `provenance_trio` | rhizoCrypt + loamSpine + sweetGrass | Provenance chain |
| `meta_tier` | biomeOS + Squirrel + petalTongue | Orchestration + AI + dashboard |

### Composition Profiles (9)

Named profiles for `plasmidBin` deployment: tower, node, nest, nucleus, full,
tower_ai, node_ai, tower_viz, nest_viz.

### NUCLEUS Validation (13 spring_validation graphs)

7 composition subsystem validators (C1–C7: render, narration, session, game science,
persistence, proprioception, interactive), plus nucleus_atomics, rollback, federation
manifest, crypto negative, and 2 templates.

### Bonding Models (5 graphs)

All five `BondType` variants represented:
- **Covalent**: `organo_metal_salt_complex` (full 12-primal composition)
- **Ionic**: `ionic_capability_share` (cross-gate capability sharing)
- **Metallic**: `metallic_gpu_pool` (compute pooling)
- **Weak**: `defensive_mesh` (isolation + defense)
- **Mixed**: `albatross_multiplex` (multi-Songbird multiplexing)

Validated by exp030 (covalent), exp031 (ionic), exp032 (plasmodium/completeness),
exp033 (all 5 BondType variants), exp071 (idle compute policies), exp092 (dual tower
ionic), exp093 (covalent mesh), exp096 (cross-arch covalent+ionic).

### Downstream Proto-Nucleate (8 graphs)

One per downstream spring/garden: ludoSpring, esotericWebb, hotSpring, wetSpring,
airSpring, groundSpring, neuralSpring, healthSpring (enclave). Template: exp095.

### Multi-Node / Federation (6 graphs)

- `basement_hpc_covalent` — same-LAN high-performance
- `friend_remote_covalent` — cross-network friends
- `three_node_covalent_cross_network` — 3-node mesh
- `data_federation_cross_site` — data sharing
- `idle_compute_federation` — idle resource pooling
- `content_distribution_federation` — seeder/consumer model

### Execution Pattern Coverage

| Pattern | Graph | Experiment |
|---------|-------|------------|
| Sequential | `tower_atomic_bootstrap.toml` | exp010 |
| Parallel | `parallel_capability_burst.toml` | exp011 |
| Conditional DAG | `conditional_fallback.toml` | exp012 |
| Streaming Pipeline | `streaming_pipeline.toml` | exp013 |
| Continuous Tick | `continuous_tick.toml` | exp014 |
| Topological Waves | all graphs via `topological_waves()` | exp006, exp069, exp070 |

### Binary Name Consistency

All 71 graphs use canonical lowercase binary names: `beardog`, `songbird`, `biomeos`,
`toadstool`, `squirrel`, `nestgate`, `rhizocrypt`, `loamspine`, `sweetgrass`,
`petaltongue`, `barracuda`, `coralreef`, `skunkbat_primal`, `rootpulse`,
`primalspring_primal`, and `*spring_primal` for downstream springs.

---

**License**: AGPL-3.0-or-later
