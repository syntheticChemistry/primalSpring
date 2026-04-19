# primalSpring — Composition Guidance for Springs and Primals

**Date**: April 18, 2026
**From**: primalSpring v0.9.15
**License**: AGPL-3.0-or-later

---

## The Composition Principle

**Complex functions emerge from composing base primals via Neural API graphs.
You never build a new primal to achieve a higher-order capability — you compose
existing ones.**

Primals are atoms. Each owns one responsibility domain and exposes it as
JSON-RPC capabilities. biomeOS's Neural API executes TOML workflow graphs
that sequence capability calls across primals. The graph is the program;
the primals are the instruction set; the emergent function is the product.

```
Layer 3: Emergent Systems (RootPulse, science pipelines, game engines)
         Defined by TOML graphs, not new code
         |
Layer 2: biomeOS Neural API (graph execution + capability routing)
         graph.execute → capability_call → route to provider
         |
Layer 1: Primals & Springs (autonomous, self-describing)
         niche.rs self-knowledge, health.check, capability.list
         |
Layer 0: NUCLEUS Atomics (Tower / Node / Nest + Meta-tier)
         Canonical compositions from graphs/fragments/
```

**RootPulse** demonstrates this concretely: it is not a primal. It is a biomeOS
CLI mode (`biomeos rootpulse commit`) that sends `graph.execute("rootpulse_commit")`
to the Neural API. The Neural API executes `rootpulse_commit.toml` — a Sequential
graph over the provenance trio (rhizoCrypt → loamSpine → sweetGrass) with BearDog
signing. The trio primals know nothing about "RootPulse." They expose `dag.*`,
`session.commit`, `braid.*` capabilities. The graph composes them into version
control.

**wetSpring's NUCLEUS** follows the same pattern: 11 nodes in
`wetspring_science_nucleus.toml` compose Tower + Node + Nest + Provenance Trio +
petalTongue into a full science pipeline with auth, compute, storage, provenance,
and visualization. No spring code imports any primal. Every interaction is a
`capability_call` routed by Neural API.

### What this means for springs

1. **New function = new graph, not new primal.** Cross-spring provenance,
   federated data, multi-pipeline orchestration — all are graph design problems.
2. **Compose, don't import.** Springs never import primal source. All
   coordination is IPC over JSON-RPC routed by Neural API.
3. **Provenance is composition.** The provenance trio is three independent
   primals composed by a graph. Springs get provenance by including trio nodes
   in their deploy graph with `fallback = "skip"` for graceful degradation.
4. **Don't build what exists.** If a capability exists, call it. Don't reimplement.

---

## Purpose

primalSpring validates the ecosystem itself — the coordination, composition,
and emergent behavior that biomeOS and the Neural API produce when primals
work together. It is:

- **Upstream of springs** — defines proto graphs and coordination patterns
- **Downstream of primals** — validates primal compositions via plasmidBin deployments
- **Gap resolver** — when a spring exposes a composition failure, primalSpring
  determines whether the fix belongs in the graph, the primal, or the standard

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

## 3. Node — Tower + ToadStool + barraCuda + coralReef

Node adds GPU/CPU compute — dispatch (ToadStool), execution (barraCuda), and shader compilation (coralReef).

**Atomic**: Node = Tower + ToadStool + barraCuda + coralReef (proton).

### What primalSpring Validates

| Validation | Experiment | Purpose |
|------------|------------|---------|
| **Compute pipeline coordination** | exp002 | ToadStool `compute.execute` capability routing |
| **Discovery within Node** | exp002 | Tower primals + ToadStool discovered via FAMILY_ID-aware sweep |
| **Compute triangle** | exp050 | coralReef → toadStool → barraCuda pipeline |

### Novel Patterns

primalSpring validates that ToadStool receives and routes compute requests
correctly. Uses `extract_rpc_result` and `extract_rpc_dispatch` for typed
JSON-RPC result extraction. `IpcError::is_method_not_found()` distinguishes
capability mismatches from connection failures.

---

## 4. Nest — Tower + NestGate + Provenance Trio

Nest adds content-addressed storage (NestGate) and full provenance lineage (rhizoCrypt + loamSpine + sweetGrass).

**Atomic**: Nest = Tower + NestGate + rhizoCrypt + loamSpine + sweetGrass (neutron).

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

## 5. Full NUCLEUS — 9 Core Primals + Meta-Tier

**Atomic**: BearDog, Songbird, ToadStool, barraCuda, coralReef, NestGate, rhizoCrypt, loamSpine, sweetGrass.

Meta-tier primals (biomeOS, Squirrel, petalTongue) operate at any atomic level and are not counted in the NUCLEUS core.

### What primalSpring Validates

| Validation | Experiment | Purpose |
|------------|------------|---------|
| **Full composition** | exp004 | All 9 core primals start, discover peers, respond to capability calls |
| **Squirrel AI coordination** | exp044 | Multi-MCP routing, `ai.query`, `ai.analyze`, `ai.suggest` |
| **Neural API integration** | exp004 | Composition-driven discovery via Neural API |

### Novel Patterns

primalSpring's `validate_composition(AtomicType::FullNucleus)` probes every
required primal. `health.readiness` on primalSpring itself reports
`primals_discovered` and `primals_total` for orchestration visibility.

---

## 6. Provenance Trio — rhizoCrypt + loamSpine + sweetGrass

**Composition**: The provenance trio demonstrates the composition principle.
Three independent primals — each owning one temporal model — compose into
RootPulse (version control), science provenance (experiment attribution), and
any future lineage-bearing workflow. The primals know nothing about "RootPulse"
or "science provenance." They expose atomic capabilities:

| Primal | Temporal Model | Key Capabilities |
|--------|---------------|------------------|
| rhizoCrypt | Ephemeral DAG (mutable present) | `dag.session.create`, `dag.dehydration.trigger`, `dag.event.append` |
| loamSpine | Permanent linear history (immutable past) | `session.commit`, `anchor.publish`, `anchor.verify` |
| sweetGrass | Semantic attribution (who contributed what) | `contribution.record_dehydration`, `braid.create`, `pipeline.attribute` |

biomeOS composes them via workflow graphs:
- `rootpulse_commit.toml` → 6-phase commit: health → dehydrate → sign → store → commit → attribute
- `provenance_trio_deploy.toml` → startup ordering: loamSpine first (others anchor to it)
- Spring deploy graphs include trio nodes with `fallback = "skip"` for graceful degradation

### What primalSpring Validates

| Validation | Experiment | Purpose |
|------------|------------|---------|
| **6-phase commit** | exp020 | health → dehydrate → sign → store → commit → attribute |
| **Branch + merge** | exp021 | Branch creation, merge commit, seal |
| **Merkle diff + federation** | exp022 | Cross-gate Merkle comparison |
| **Provenance for science** | exp041 | Any spring experiment → provenance trio |

### Novel Patterns

primalSpring validates that the trio composes correctly across multiple
graph patterns. Uses `extract_capability_names` to handle all 4 capability
wire formats from each primal. `CircuitBreaker` prevents cascading failures
when one trio member is down. Wire types (`DehydrationWireSummary`,
`PipelineRequest`) carry `niche` / `session_type` metadata for
workflow-specific tagging without changing the primal code.

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
happens via biomeOS capability discovery and IPC calls to ecobin primals.
barraCuda is a full ecobin primal (32 JSON-RPC methods over UDS) — springs
call `tensor.matmul`, `stats.mean`, etc. by capability, not by library import.
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
| Discovery tiers | Standard 6-tier | Explicit env vars (tiers 2-3 unavailable) |
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
| `profiles/tower_ai.toml` | Tower | Squirrel | AI task alignment at Tower level |
| `profiles/tower_viz.toml` | Tower | Squirrel + petalTongue | Full user-facing with dashboards |
| `profiles/nest_viz.toml` | Nest | petalTongue | Storage dashboards |
| `profiles/node_ai.toml` | Node | Squirrel | AI-directed compute workloads |
| `profiles/full.toml` | NUCLEUS | Squirrel + petalTongue (optional) | Full platform overlay |

### Graph Node Fields

Each graph node supports:

```toml
[[graph.nodes]]
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
let overlay = load_graph("graphs/profiles/tower_ai.toml")?;
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

`graphs/profiles/full.toml` composes all capability domains:

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

## 12. Proto-Nucleate Graph Pattern (v0.9.4)

Proto-nucleate graphs define **target compositions** that downstream springs
absorb and evolve against. They live in `graphs/downstream/` and follow a
standard structure:

```toml
[graph]
name = "yourspring_domain_proto_nucleate"
description = "Proto-nucleate: YourSpring domain composition"
pattern = "ProtoNucleate"
version = "1.0.0"

[graph.metadata]
particle_model = "balanced"       # proton-heavy | neutron-heavy | balanced
bonding_primary = "covalent"      # default same-family bonding (requires NuclearLineage trust)
secure_by_default = true
btsp_phase = 2                    # Phase 1 = mito-beacon tunnel, Phase 2 = nuclear session
genetics_tier = "nuclear"         # nuclear | mito_beacon | tag (determines trust model)

[[graph.nodes]]
name = "coralreef"
required = true                   # shader compiler is mandatory for compute springs
by_capability = "shader"

[[graph.nodes]]
name = "toadstool"
required = true                   # dispatch substrate is mandatory
by_capability = "compute"

[[graph.nodes]]
name = "barracuda"
required = true                   # GPU/CPU math execution
by_capability = "tensor"
```

### Proto-Nucleate Absorption Workflow (v0.9.15 — guideStone Pattern)

**Proto-nucleate graphs are self-validating NUCLEUS compositions.** Each graph
includes a **guideStone node** — a domain-specific self-validation entry point
that discovers primals on startup, runs benchmarks, then serves domain capabilities.

The validation ladder:
- Level 2 (Rust proof): Spring binary proved Python → Rust parity — DONE
- Level 5 (guideStone): Self-validating NUCLEUS node, IPC to primals — THIS
- Level 6 (deployment): biomeOS deploys graph, guideStone validates and serves

The guideStone pattern (proven by hotSpring-guideStone-v0.7.0):
- 5 certified properties: deterministic, traceable, self-verifying,
  environment-agnostic, tolerance-documented
- Bare guideStone: properties hold without any primals running
- NUCLEUS guideStone: additive layer (BearDog signing, rhizoCrypt DAG, toadStool reporting)
- See `wateringHole/GUIDESTONE_COMPOSITION_STANDARD.md` for the full standard.

How a spring evolves toward guideStone deployment:

1. **Read** `graphs/downstream/downstream_manifest.toml` — find your `[[downstream]]` entry.
   (Exception: `healthspring_enclave_proto_nucleate.toml` is standalone — dual-tower ionic bridge.)
   Your `guidestone_binary` is the self-validating deployable. Your `validation_capabilities`
   are the primal IPC methods the guideStone checks on startup.
2. **Build the guideStone binary** — a standalone executable that satisfies the 5 properties,
   discovers primals, validates IPC parity, then serves domain capabilities.
3. **Deploy the NUCLEUS** — `biomeos deploy --graph <your_proto_nucleate>`. The guideStone
   node starts alongside primals and self-validates.
4. **Document gaps** — IPC methods that don't exist, response schemas that differ, tolerances
   that fail. Hand back to primalSpring for evaporation to primal teams.
5. **Iterate** — as primal teams fix gaps, re-validate. The cycle accelerates.

The IPC client each spring currently has (`NucleusContext`, `IpcMathClient`,
`math_dispatch`, `rpc_call`) is a **validation window** — temporary tooling that
proves the math works through NUCLEUS. The guideStone binary is the permanent
deployable that replaces it. Springs keep their `barracuda` library dependency
for Level 2 comparison; the guideStone uses pure IPC.

### Three-Tier Composition Validation (Emerged Pattern — April 17, 2026)

Four springs (hotSpring, healthSpring, neuralSpring, wetSpring) have independently
converged on a three-tier validation structure. This is now the **recommended pattern**
for any spring entering NUCLEUS composition:

```
Tier 1: LOCAL_CAPABILITIES (honest local dispatch)
        Spring owns a set of domain capabilities that can execute locally via Rust code.
        These are listed in the spring's niche.rs or equivalent.
        No IPC required. Always passes on the developer's machine.
        Examples: hotSpring 13 LOCAL_CAPABILITIES, healthSpring niche.rs

Tier 2: IPC-WIRED (live primal delegation with honest skip)
        Spring attempts IPC calls to NUCLEUS primals (Tower, Node, Nest).
        Uses `check_skip()` / `check_or_skip()` when primals are absent.
        Parity comparison: local Rust result vs primal IPC result.
        Examples: neuralSpring validate_science_composition, wetSpring Exp401/402

Tier 3: FULL NUCLEUS (proto-nucleate deployed via biomeOS)
        Proto-nucleate graph deployed by biomeOS with guideStone node.
        All primal nodes healthy, started in topological order.
        guideStone self-validates on startup: IPC calls to primals,
        results compared against Python baselines.
        This is the primal proof — science runs through NUCLEUS.
        See wateringHole/GUIDESTONE_COMPOSITION_STANDARD.md for the
        5 certified properties every guideStone must satisfy.
```

**Why three tiers**: Tier 1 is always green and lets the spring CI pass without infrastructure.
Tier 2 catches composition regressions when primals are running (integration test).
Tier 3 proves deployment correctness (acceptance test). Springs should NOT skip Tier 2
and jump to Tier 3 — the honest skip/pass distinction in Tier 2 is critical evidence
for identifying cross-primal protocol gaps.

### The guideStone Binary Pattern (April 18, 2026)

Three springs (hotSpring, healthSpring, wetSpring) independently converged on
the same harness pattern for Level 5 primal proof. This becomes the **guideStone
binary** — a self-validating deployable that runs in-graph or standalone:

```rust
use primalspring::composition::{
    CompositionContext, validate_parity, validate_parity_vec,
    capability_to_primal, method_to_capability_domain,
};
use primalspring::validation::ValidationResult;
use primalspring::tolerances;

fn main() {
    let mut ctx = CompositionContext::from_live_discovery();
    let mut v = ValidationResult::new("myspring guideStone");

    // Exit 2 if no NUCLEUS is running
    if ctx.available_capabilities().is_empty() {
        eprintln!("No NUCLEUS primals discovered. Deploy from plasmidBin and rerun.");
        std::process::exit(2);
    }

    // For each validation_capability in your manifest entry:
    validate_parity(
        &mut ctx, &mut v,
        "sample_mean",
        "tensor",           // capability domain — resolves to barraCuda
        "stats.mean",       // JSON-RPC method on the ecobin primal
        serde_json::json!({"data": [1.0, 2.0, 3.0, 4.0, 5.0]}),
        "result",           // key in the JSON-RPC response
        3.0,                // expected value (from Python baseline)
        tolerances::CPU_GPU_PARITY_TOL,
    );

    v.finish();  // exit 0 if all pass, exit 1 if any fail
}
```

Key conventions:
- **Exit 0** = all guideStone checks passed (NUCLEUS is valid)
- **Exit 1** = at least one check failed (deployment is broken)
- **Exit 2** = no NUCLEUS deployed (bare guideStone properties still apply)
- Use `method_to_capability_domain()` to resolve which domain to pass to `call()`
- Use `capability_to_primal()` to understand which primal serves a domain
- Use `check_skip()` (not fake passes) when a primal is absent
- Compare IPC results against Python baselines, not just against library results
- Binary name: `<spring>_guidestone` (e.g. `hotspring_guidestone`, `wetspring_guidestone`)
- The guideStone satisfies all 5 properties from `GUIDESTONE_COMPOSITION_STANDARD.md`
- The spring binary (with lib dep) validates the guideStone works; the guideStone deploys

### Layered Certification: Base + Domain (April 2026)

GuideStone certification is **layered**. primalSpring provides the **base
certification** (composition correctness), and domain springs provide
**domain certification** (science correctness). A domain guideStone inherits
the base and only validates its own science on top.

```
                 ┌─────────────────────────────────┐
                 │  Domain guideStone (hotSpring,   │
                 │  healthSpring, wetSpring, ...)   │
                 │  Validates: domain science       │
                 │  (QCD, clinical math, hydro)     │
                 └────────────┬────────────────────┘
                              │ inherits
                 ┌────────────▼────────────────────┐
                 │  primalSpring guideStone         │
                 │  Validates: composition          │
                 │  correctness (discovery, health, │
                 │  IPC parity, cross-atomic,       │
                 │  bonding, BTSP/crypto)           │
                 └─────────────────────────────────┘
```

**For domain guideStone developers**: if the primalSpring guideStone passes
(exit 0), you can assume discovery works, health checks pass, basic math
IPC is correct, storage roundtrips, bonding policies are well-formed, and
crypto produces deterministic results. Your guideStone only needs to
validate domain-specific science through the IPC methods listed in your
`validation_capabilities` manifest entry.

**Pre-flight usage**: domain guideStones can optionally run
`primalspring_guidestone` as a pre-flight check before their own validation.
Exit 0 means the composition is sound; exit 1 means composition is broken
(domain validation would be meaningless).

See `wateringHole/GUIDESTONE_COMPOSITION_STANDARD.md` for the full layered
certification specification and the 6 validation layers.

**Observed ecosystem blockers** (April 18, 2026):
- `crypto.sign_contract` (ionic bond negotiation) — BearDog, affects cross-tower compositions
- BTSP Phase 3 (encrypted post-handshake channel) — all primals
- ~~`compute.dispatch` standardization~~ — **RESOLVED** (toadStool S203 wire contract + PG-31 JSON-RPC routing fix)
- Squirrel provider registration — affects springs needing AI capabilities
- `storage.fetch_external` (cross-spring data) — NestGate, affects cross-spring pipelines
- barraCuda IPC rewiring — **spring-side, actively in progress**: hotSpring (9 probes wired via `validate_primal_proof`), healthSpring (`math_dispatch.rs` feature-gated routing, exp122 parity), wetSpring (Exp403 `validate_primal_parity_v1` calling 5 primals). Pattern absorbed into `primalspring::composition` as public API (`capability_to_primal`, `method_to_capability_domain`)

---

## 13. Dual-Tower Enclave Pattern (v0.9.4)

For springs handling sensitive data (healthSpring, regulatory, financial),
the **dual-tower enclave** separates data custody from analytics:

```
Tower A (Data Custody)              Tower B (Analytics)
┌─────────────────────┐            ┌─────────────────────┐
│ healthSpring         │            │ Squirrel (AI)       │
│ NestGate-A (egress   │ ══ionic══ │ NestGate-B (model   │
│   fence enforced)    │  bridge   │   weights cache)    │
│ Provenance Trio A    │            │ Provenance Trio B   │
└─────────────────────┘            └─────────────────────┘
```

**Key properties**:
- Different `FAMILY_ID` per tower (ionic bond, not covalent)
- Ionic bond requires `TrustModel::MitoBeaconFamily` minimum — each tower shares a mito-beacon for discovery but maintains separate nuclear lineages for permissions
- `capabilities_denied = ["storage.*", "dag.*"]` on the bridge — Tower B cannot access raw data
- NestGate-A enforces `BondingPolicy` egress fence — data cannot leave Tower A except as de-identified aggregates
- Both towers carry full provenance trios for regulatory audit trails
- Nuclear genetics (Tier 2) are never shared across the ionic bridge — each tower spawns its own lineage. Only mito-beacon membership (Tier 1) crosses the boundary

This pattern applies to any spring handling data with compliance requirements.
See `graphs/downstream/healthspring_enclave_proto_nucleate.toml` for the canonical
dual-tower ionic bridge pattern.

---

## 14. Pure Composition Pattern (ludoSpring/esotericWebb Model)

When a downstream spring or garden can be expressed entirely as a composition of
existing NUCLEUS primals, it should NOT have its own binary. The graph IS the product.

**All proto-nucleate graphs are now `pure_nucleus`:**
```toml
[graph.metadata]
composition_model = "pure_nucleus"
fragments = ["tower_atomic", "node_atomic"]
validated_by = "ludospring"
validation_capabilities = ["tensor.matmul", "compute.dispatch", "inference.complete"]

[[graph.nodes]]
name = "barracuda"
binary = "barracuda"
spawn = false
capabilities = ["tensor.fitts", "tensor.perlin", "tensor.wfc"]
```

Key properties:
- `composition_model = "pure_nucleus"` — all nodes are primals, no spring binaries
- All nodes are `spawn = false` — biomeOS manages the full lifecycle
- `validated_by` names the spring that validates externally (not a graph node)
- `validation_capabilities` lists primal IPC methods the spring calls
- The spring's Rust binary was the "Rust proof" — it proved Python → Rust parity
- The proto-nucleate is the "primal proof" — it proves the science runs through NUCLEUS

This pattern applies to ALL proto-nucleate graphs. Springs never appear as nodes.

See `graphs/fragments/README.md` for the full fragment model.

---

## 15. Three-Tier Genetics Identity (v0.9.14)

Compositions authenticate and authorize through a three-tier genetics model.
Each tier serves a distinct security function; compositions select the tier
appropriate to their bonding model.

```
Tier 1: Mito-Beacon Genetics (discovery, NAT, metadata)
        Inherited group membership. Freely cloneable. Multiple per system.
        Dark forest protocol — zero metadata leakage to observers.
        Sufficient for: Metallic bonds, ionic bridge discovery
        |
Tier 2: Nuclear Genetics (permissions, authorization, sessions)
        Fresh per generation — NEVER copied. Generational DNA mixing.
        Copy-resistant lineage tracking. ZeroizeOnDrop key material.
        Required for: Covalent bonds, authenticated sessions
        |
Tier 3: Genetic Tags (open participation channels)
        Legacy FAMILY_SEED transformed. Freely cloneable. No key material.
        Use for: Public subgroups, chat, hashtag comms within a family
```

### Two-Phase BTSP Connection

Phase 1 (mito-beacon tunnel) establishes encrypted communication using inherited
beacon key material. Phase 2 (nuclear session) spawns a fresh generation within
the tunnel for permission-bearing operations. This separation ensures that
discovery never exposes authorization credentials.

### Bonding × Genetics

| Bond Type | Minimum Trust Model | Genetics Tier |
|-----------|-------------------|---------------|
| Covalent | `NuclearLineage` | Tier 2 (nuclear, !Clone) |
| Metallic | `MitoBeaconFamily` | Tier 1 (mito-beacon) |
| Ionic | `MitoBeaconFamily` | Tier 1 (cross-family beacon) |

### For Primals and Springs

- **Covalent compositions** must spawn fresh nuclear genetics at each step.
  Use `genetics::rpc::derive_lineage_key()` through BearDog.
- **Ionic bridges** (dual-tower, cross-family) share mito-beacons for
  discovery but never nuclear credentials across the boundary.
- **Legacy FAMILY_SEED** is automatically bridged: `GeneticTag::from_legacy_family_seed()`
  wraps it as a Tier 3 tag. `mito_beacon_from_env()` wraps it as a Tier 1
  beacon for backward compatibility during migration.

---

## 16. Expectations for Composed Primals

For any primal to participate in primalSpring-validated compositions, it must
meet these baseline requirements. primalSpring's integration tests and
experiments enforce these at runtime.

### JSON-RPC Surface

Every primal must register: `health.liveness`, `health.readiness`,
`health.check`, `capabilities.list`. primalSpring probes these during
composition validation — missing methods cause gate failures.

### Discovery

Primals must resolve peers via capability, not identity. No hardcoded peer
names or socket paths in production code. The 6-tier discovery chain
(env var → XDG → plain socket → temp → manifest → socket-registry; Neural API for ecosystem sweep) is the standard; Android
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

## 17. Meta-Tier Primals — biomeOS + Squirrel + petalTongue

Meta-tier primals operate at **any** atomic level. They are not part of Tower,
Node, or Nest — they overlay any composition.

| Primal | Role | Key Capabilities |
|--------|------|------------------|
| biomeOS | Orchestration | `graph.deploy`, `graph.execute`, `capability.route`, `capability.discover` |
| Squirrel | AI coordination | `ai.query`, `ai.complete`, `tool.list`, `context.create` |
| petalTongue | UI/rendering | `render.scene`, `render.dashboard`, `tui.push` |

Meta-tier primals appear in deploy graphs as optional overlays. Any atomic
tier (Tower, Node, Nest, NUCLEUS) can compose with any subset of meta-tier
primals. The `meta_tier` fragment defines the canonical triple.

### Why Meta-Tier Matters for Springs

Springs consume meta-tier primals without depending on specific atomic tiers:
- A Tower + Squirrel composition gives AI without compute or storage
- A Nest + petalTongue composition gives storage dashboards without GPU
- NUCLEUS + full meta-tier gives the complete platform

This separation ensures springs can start minimal and compose up.

---

**License**: AGPL-3.0-or-later
