# Upstream Primal Cross-Talk & Downstream Absorption Patterns

**Date**: April 16, 2026
**From**: primalSpring v0.9.14 (Phase 43+)
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

### What to Hand Back

When your spring or garden discovers a gap:
1. Document it in primalSpring `docs/PRIMAL_GAPS.md`
2. Propose the wire (JSON-RPC method signature)
3. Build a primalSpring experiment or graph that tests the gap
4. Submit via PR — primalSpring triages and routes to the responsible primal

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

**License**: AGPL-3.0-or-later
