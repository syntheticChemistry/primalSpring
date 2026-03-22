# primalSpring — Composition Guidance for Springs and Primals

**Date**: March 22, 2026
**From**: primalSpring v0.7.0
**License**: AGPL-3.0-or-later

---

## Purpose

This document describes how primalSpring's coordination validation capabilities
can be leveraged across composition layers. primalSpring is unique: it validates
the ecosystem itself — the coordination, composition, and emergent behavior that
biomeOS and the Neural API produce when primals work together.

Each primal in the ecosystem should write an equivalent document. No primal knows
about another at compile time — all composition happens at runtime via
capability-based discovery through biomeOS.

---

## Capabilities Table

| Capability | Description |
|------------|-------------|
| `coordination.validate_composition` | Validate atomic compositions (Tower/Node/Nest/FullNucleus) |
| `coordination.discovery_sweep` | Discover all primals in a composition |
| `coordination.neural_api_status` | Neural API health and reachability |
| `health.check` | Full self health status |
| `health.liveness` | Kubernetes-style liveness probe — am I alive? |
| `health.readiness` | Kubernetes-style readiness probe — ready to serve? |
| `capabilities.list` | List coordination capabilities |
| `lifecycle.status` | Primal status report (version, domain, status) |

---

## 1. Standalone — primalSpring Alone

primalSpring can run as a standalone primal without any other primals. In this
mode it validates coordination patterns, discovery sweep logic, and Neural
API health.

### What Standalone primalSpring Can Do

| Capability | Use Case |
|-------------|----------|
| **Discovery sweep** | Enumerate primals in a composition (returns empty when none running) |
| **Neural API status** | Check if biomeOS is reachable |
| **Health probes** | `health.liveness` always succeeds; `health.readiness` reports Neural API + discovered primals |
| **Validate coordination patterns** | Run experiments that probe primals — honest skip when primals absent |
| **4-format capability parsing** | Parse capability responses from any primal (Format A/B/C/D) |

### When to Use Standalone

- CI pipelines that validate primalSpring itself
- Discovery sweep testing without live primals
- Neural API connectivity checks
- Validation binary scaffolding (OrExit, ValidationSink, check_or_skip)

---

## 2. Tower — primalSpring + BearDog + Songbird

**Atomic**: Tower provides crypto identity and mesh discovery.

### What primalSpring Validates

| Validation | Experiment | Purpose |
|------------|------------|---------|
| **Tower Atomic bootstrap** | exp001 | BearDog socket created, Songbird mesh reachable |
| **Crypto capabilities** | exp001 | `crypto.sign`, `crypto.verify`, `crypto.keygen` respond |
| **Startup ordering** | exp006 | BearDog starts before primals that depend on it |
| **Graceful degradation** | exp005 | Removal of BearDog causes honest skip, not fake pass |

### Novel Patterns

primalSpring probes `health.liveness` and `health.readiness` on BearDog and
Songbird. Uses `resilient_call()` with `CircuitBreaker` and `RetryPolicy` for
transient failures. `DispatchOutcome::should_retry()` guides retry decisions.

---

## 3. Node — Tower + ToadStool

**Atomic**: Node adds GPU/CPU compute dispatch.

### What primalSpring Validates

| Validation | Experiment | Purpose |
|------------|------------|---------|
| **Compute pipeline coordination** | exp002 | ToadStool `compute.execute` capability routing |
| **Discovery within Node** | exp002 | Tower primals + ToadStool discovered via FAMILY_ID-aware sweep |
| **GPU dispatch** | exp050 | coralReef → toadStool → barraCuda compute triangle |

### Novel Patterns

primalSpring validates that ToadStool receives and routes compute requests
correctly. Uses `extract_rpc_result` and `extract_rpc_dispatch` for typed
JSON-RPC result extraction. `IpcError::is_method_not_found()` distinguishes
capability mismatches from connection failures.

---

## 4. Nest — Tower + NestGate

**Atomic**: Nest adds content-addressed storage.

### What primalSpring Validates

| Validation | Experiment | Purpose |
|------------|------------|---------|
| **Storage pipeline coordination** | exp003 | NestGate `storage.store` + `storage.retrieve` round-trip |
| **Discovery within Nest** | exp003 | Tower + NestGate discovered |
| **fieldMouse ingestion** | exp042 | fieldMouse frames → NestGate → sweetGrass |

### Novel Patterns

primalSpring validates storage pipeline composition. NestGate health probes
(`health.liveness`, `health.readiness`) confirm readiness before storage
operations. `safe_cast::micros_u64` for latency metrics in validation reports.

---

## 5. Full NUCLEUS — All 8 Primals

**Atomic**: BearDog, Songbird, ToadStool, NestGate, rhizoCrypt, loamSpine,
sweetGrass, Squirrel.

### What primalSpring Validates

| Validation | Experiment | Purpose |
|------------|------------|---------|
| **Full composition** | exp004 | All 8 primals start, discover peers, respond to capability calls |
| **Squirrel AI coordination** | exp044 | Multi-MCP routing, `ai.query`, `ai.analyze`, `ai.suggest` |
| **Neural API integration** | exp004 | Composition-driven discovery via Neural API |

### Novel Patterns

primalSpring's `validate_composition(AtomicType::FullNucleus)` probes every
required primal. `health.readiness` on primalSpring itself reports
`primals_discovered` and `primals_total` for orchestration visibility.

---

## 6. Provenance Trio — rhizoCrypt + loamSpine + sweetGrass (RootPulse)

**Composition**: primalSpring coordinates validation of the RootPulse workflow.

### What primalSpring Validates

| Validation | Experiment | Purpose |
|------------|------------|---------|
| **6-phase commit** | exp020 | health → dehydrate → sign → store → commit → attribute |
| **Branch + merge** | exp021 | Branch creation, merge commit, seal |
| **Merkle diff + federation** | exp022 | Cross-gate Merkle comparison |
| **Provenance for science** | exp041 | Any spring experiment → provenance trio |

### Novel Patterns

primalSpring validates that the provenance trio (rhizoCrypt, loamSpine,
sweetGrass) composes correctly for RootPulse. Uses `extract_capability_names`
to handle all 4 capability wire formats from each primal. `CircuitBreaker`
prevents cascading failures when one trio member is down.

---

## 7. Cross-Spring — airSpring → wetSpring → neuralSpring

**Composition**: Ecology pipeline across springs.

### What primalSpring Validates

| Validation | Experiment | Purpose |
|------------|------------|---------|
| **Cross-spring data flow** | exp024 | airSpring → wetSpring → neuralSpring ecology pipeline |
| **Capability routing** | exp024 | Data flows through biomeOS capability graph |
| **Provenance trio for science** | exp041 | Cross-spring experiments → provenance attribution |

### Novel Patterns

primalSpring validates that springs never import each other — coordination
happens via shared barraCuda primitives and biomeOS capability discovery.
`coordination.neural_api_status` confirms biomeOS is routing correctly.

---

## 8. Sovereign Compute Triangle — coralReef → toadStool → barraCuda

**Composition**: GPU compute pipeline.

### What primalSpring Validates

| Validation | Experiment | Purpose |
|------------|------------|---------|
| **Compute triangle** | exp050 | coralReef → toadStool → barraCuda pipeline |
| **Health probes** | exp050 | `health.liveness`, `health.readiness` on each primal |
| **Probe patterns** | exp050 | FAMILY_ID-aware discovery, Neural API health checks |

### Novel Patterns

primalSpring validates the sovereign compute stack: coralReef compiles WGSL,
toadStool dispatches, barraCuda executes. No vendor SDK lock-in. Uses
`ValidationSink` (StdoutSink/NullSink) for CI/headless validation runs.

---

## Discovery Protocol

All compositions above are **runtime-discovered**. primalSpring never imports
another primal. The discovery chain:

1. primalSpring starts → registers capabilities via `capabilities.list`
2. biomeOS discovers primalSpring → adds to niche capability registry
3. Any primal calls `coordination.validate_composition` → primalSpring probes
   required primals via `AtomicType::required_primals()`
4. primalSpring discovers primals via: `{PRIMAL}_SOCKET`, XDG convention,
   temp fallback, Neural API
5. No compile-time coupling. Primals come and go. Capabilities are the contract.

---

## For Other Primals Writing This Document

primalSpring's composition guidance differs from compute springs: primalSpring
**validates** compositions rather than **providing** compute. Focus on:

1. **What you validate** — which composition patterns you probe
2. **What each atomic unlocks** — Tower, Node, Nest, Full NUCLEUS
3. **What emergent systems you validate** — RootPulse, coralForge, cross-spring
4. **What probe patterns you use** — health.liveness, health.readiness,
   resilient_call, DispatchOutcome
5. **What capabilities you expose** — coordination, health, lifecycle

Remember: complexity through coordination, not coupling.

---

## 9. Cross-Architecture Deployment (v0.7.0)

**Key Insight**: Rust's `cargo build --target` produces correct binaries for
any architecture from a single codebase. Every composition pattern above
works identically on x86_64 and aarch64 — the coordination logic is
architecture-agnostic; only the binary needs rebuilding.

### Validated Targets

| Target | Linking | Binary Size | Runs On |
|--------|---------|-------------|---------|
| `x86_64-unknown-linux-musl` | static-pie | 3.0 MB | Desktop, server, USB spore |
| `aarch64-unknown-linux-musl` | static | 2.99 MB | Pixel 8a, ARM server |

### What Changes Per Architecture

Nothing in the coordination or composition logic changes. The only
architecture-dependent concerns are:

| Concern | x86_64 | aarch64 (Android) |
|---------|--------|--------------------|
| Socket transport | Unix filesystem | Abstract sockets (`@name`) |
| Working directory | `$XDG_RUNTIME_DIR` | `/data/local/tmp/` |
| Discovery tiers | Standard 5-tier | Explicit env vars (tiers 2-3 unavailable) |
| Binary format | ELF x86-64, static-pie | ELF aarch64, static |

### Composition on Pixel

The full Tower atomic (beardog + songbird + primalSpring) has been validated
on Pixel 8a via ADB. The coordination protocol (JSON-RPC 2.0 over socket)
is identical — only the transport layer (abstract vs filesystem socket) and
the binary target differ.

For Android deployment, each primal needs:
- `aarch64-unknown-linux-musl` build (static, stripped)
- Abstract socket support (`@biomeos/{primal}` namespace)
- Explicit `FAMILY_ID`, `NODE_ID`, `{PRIMAL}_SOCKET` env vars
- Writable CWD (`/data/local/tmp/`)

### genomeBin: Architecture-Agnostic Packaging

The genomeBin standard (wateringHole `GENOMEBIN_ARCHITECTURE_STANDARD.md`)
solves the cross-arch distribution problem: a single `.genome` self-extractor
embeds binaries for all supported architectures and selects the right one at
deploy time. See `ECOBIN_GENOMEBIN_EVOLUTION_GUIDANCE_MAR22_2026.md` for the
evolution roadmap.

---

## 10. Graph-Driven Overlay Composition (v0.7.0)

**Key Insight**: Tier-independent primals (Squirrel, petalTongue, biomeOS) are
not locked to `FullNucleus`. Any atomic tier can compose them as **optional
overlays** via deploy graphs.

### The Overlay Model

Instead of fixed `AtomicType` enum gating, deploy graph TOMLs define the full
primal set. The harness spawns **all nodes with `spawn = true`** and uses the
base tier's `required_primals()` as the minimum guarantee.

```
Base Tier (required)    +    Overlay (optional, from graph)
────────────────────        ─────────────────────────────
Tower: beardog, songbird    + squirrel (AI alignment)
Nest:  + nestgate           + petaltongue (storage dashboards)
Node:  + toadstool          + squirrel (AI-driven compute)
```

### Deploy Graph TOMLs

| Graph | Base Tier | Overlay Primals | Use Case |
|-------|-----------|-----------------|----------|
| `tower_ai.toml` | Tower | Squirrel | AI task alignment at Tower level |
| `tower_ai_viz.toml` | Tower | Squirrel + petalTongue | Full user-facing with dashboards |
| `nest_viz.toml` | Nest | petalTongue | Storage dashboards |
| `node_ai.toml` | Node | Squirrel | AI-directed compute workloads |

### Graph Node Fields

Each graph node supports:

```toml
[[graph.node]]
name = "squirrel"
binary = "squirrel_primal"
order = 3
required = false           # composition doesn't fail if missing
spawn = true               # harness spawns this as a process (default: true)
depends_on = ["beardog"]   # topological ordering
by_capability = "ai"       # capability routing key
capabilities = ["ai.query", "ai.complete"]
```

Set `spawn = false` for validation/coordination nodes that the harness should
not spawn (e.g. `validate_tower`, `primalspring`).

### Capability Resolution

The harness resolves capabilities in two layers:

1. **Static base tier** — `AtomicType::required_capabilities()` maps to
   `required_primals()` (e.g. `"security"` → `"beardog"`)
2. **Dynamic overlay** — graph nodes with `spawn = true` and `by_capability`
   populate an overlay map (e.g. `"ai"` → `"squirrel"`)

`RunningAtomic::socket_for("ai")` transparently resolves through both layers.

### Graph Merge/Compose

biomeOS can compose graphs at runtime:

```rust
use primalspring::deploy::{load_graph, merge_graphs};

let base = load_graph("graphs/tower_atomic_bootstrap.toml")?;
let overlay = load_graph("graphs/tower_ai.toml")?;
let merged = merge_graphs(&base, &overlay);
// merged.graph.name = "tower_atomic_bootstrap+tower_ai"
```

Merge semantics: overlay nodes with the same name override base nodes; new
nodes are appended. The result is topologically validated.

### biomeOS Graph Self-Composition

biomeOS itself is a tier-independent primal that can **compose its own graphs**:

- A running biomeOS instance can call `graph.compose` to merge a base tier
  graph with an overlay fragment
- biomeOS can spawn nested biomeOS instances as VM-like systems, each with
  their own deploy graph
- Graph fragments are the unit of composition — biomeOS reads, merges, and
  deploys them

This pattern enables:
- Dynamic composition changes without restart
- Nested biomeOS topologies
- Self-modifying deploy graphs (biomeOS updates its own graph)

### Experiments and Tests

| Artifact | Purpose |
|----------|---------|
| `exp069_graph_overlay_composition` | End-to-end overlay validation |
| `overlay_tower_ai_*` integration tests | Tower + Squirrel via graph |
| `overlay_nest_viz_*` integration tests | Nest + petalTongue via graph |
| `overlay_node_ai_*` integration tests | Node + Squirrel via graph |
| `overlay_graph_merge_base_plus_ai` | Graph merge validation |

---

## 11. Squirrel Free-Roaming Coordination (v0.7.0)

Squirrel is the AI coordinator primal — it moves freely across the
ecosystem, discovering sibling primals and routing tool/AI/context
requests through them. Unlike domain-bound primals, Squirrel has no
fixed tier; it attaches to any composition via deploy graph overlays.

### Discovery Mechanism

Squirrel uses 3-tier capability discovery:

1. **Explicit env var** (`{CAPABILITY}_PROVIDER_SOCKET=/path/to/socket`) — fastest
2. **Registry query** — Neural API capability registry
3. **Socket scan** — `$XDG_RUNTIME_DIR/biomeos/` directory (fallback)

The primalSpring harness provides all three: env vars via `env_sockets`
in `primal_launch_profiles.toml`, Neural API via the composition, and
the shared `biomeos/` runtime directory for socket scanning.

### Wired Capabilities

Squirrel's launch profile maps 9 capability-provider socket pairs:

| Env Var | Provider | Capabilities |
|---------|----------|-------------|
| `STORAGE_STORE_PROVIDER_SOCKET` | nestgate | `storage.store` |
| `STORAGE_GET_PROVIDER_SOCKET` | nestgate | `storage.retrieve` |
| `STORAGE_LIST_PROVIDER_SOCKET` | nestgate | `storage.list` |
| `MODEL_REGISTER_PROVIDER_SOCKET` | nestgate | `model.register` |
| `MODEL_LOCATE_PROVIDER_SOCKET` | nestgate | `model.locate` |
| `COMPUTE_EXECUTE_PROVIDER_SOCKET` | toadstool | `compute.execute` |
| `COMPUTE_DISPATCH_SUBMIT_PROVIDER_SOCKET` | toadstool | `compute.dispatch.submit` |
| `HTTP_REQUEST_PROVIDER_SOCKET` | songbird | `http.request` (cloud AI) |
| `CRYPTO_SIGN_PROVIDER_SOCKET` | beardog | `crypto.sign` |

### Full Overlay Graph

`graphs/full_overlay.toml` composes all capability domains:

```
beardog (security) → songbird (discovery) → nestgate (storage)
                                          → toadstool (compute)
                                          → squirrel (ai)
```

Squirrel discovers all siblings via env vars and can:
- **`capability.discover`** — report known sibling capabilities
- **`tool.list`** — aggregate tools from all discovered primals
- **`context.create`** — manage AI context (backed by nestgate storage)
- **`ai.query`** — route queries through Songbird's `http.request` to cloud AI

### Experiments and Tests

| Artifact | Purpose |
|----------|---------|
| `exp070_squirrel_cross_primal_discovery` | Cross-primal discovery validation |
| `squirrel_discovers_sibling_primals` | Squirrel finds NestGate/ToadStool/Songbird |
| `squirrel_tool_list_aggregates` | Aggregated tool listing from multiple primals |
| `squirrel_context_create` | Context management via composition |
| `squirrel_ai_query_routes_via_songbird` | Cloud AI routing through Songbird |

---

## 12. Expectations for Composed Primals

For any primal to participate in primalSpring-validated compositions, it must
meet these baseline requirements. primalSpring's integration tests and
experiments enforce these at runtime.

### JSON-RPC Surface

Every primal must register: `health.liveness`, `health.readiness`,
`health.check`, `capabilities.list`. primalSpring probes these during
composition validation — missing methods cause gate failures.

### Discovery

Primals must resolve peers via capability, not identity. No hardcoded peer
names or socket paths in production code. The 5-tier discovery chain
(env var → XDG → temp → manifest → Neural API) is the standard; Android
replaces tiers 2-3 with abstract sockets.

### Build Targets

Both `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` are
required for ecoBin compliance. `[profile.release] strip = true, lto = true`.
See `PRIMAL_CAPABILITY_STATUS_MAR22_2026.md` for per-primal compliance.

### Honest Reporting

primalSpring uses `check_skip()` when a primal is absent — never a fake
pass. Composed primals should follow the same pattern: report what you
actually observed, not what you hope is true.

---

**License**: AGPL-3.0-or-later
