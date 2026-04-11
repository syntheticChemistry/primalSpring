# primalSpring — Ecosystem Leverage Guide

**Date**: April 11, 2026
**Version**: v0.9.9
**License**: AGPL-3.0-or-later

---

## What primalSpring Offers

primalSpring validates coordination itself. Its leverage surface is the
patterns, harnesses, and IPC infrastructure that all springs and primals
can absorb or compose against.

### Standalone Use

| Capability | How | When |
|------------|-----|------|
| **Coordination validation** | `cargo run --bin primalspring_primal -- server` | Validate atomic compositions live |
| **Deploy graph validation** | `graph.list` / `graph.validate` RPC | CI: ensure deploy TOMLs are structurally valid |
| **Health probing** | `health.liveness` / `health.readiness` | Kubernetes-style health checks |
| **MCP tool discovery** | `mcp.tools.list` | Squirrel AI routes coordination requests |
| **Meta-validation** | `cargo run --bin validate_all` | Run all 72 experiments in sequence |
| **Remote gate probe** | `./scripts/validate_remote_gate.sh <host>` | Per-primal TCP health check on any gate |
| **Musl build** | `./scripts/build_ecosystem_musl.sh` | Static x86_64+aarch64 binaries for deployment |
| **Spore prep** | `./scripts/prepare_spore_payload.sh <dir>` | USB payload assembly |

### Library Patterns to Absorb

| Pattern | Module | Description |
|---------|--------|-------------|
| **ValidationResult** | `validation/mod.rs` | `check_bool`, `check_skip`, `check_or_skip`, `check_latency`, `check_minimum`, `check_count` + `finish()` + `exit_code()` |
| **Structured Provenance** | `validation/mod.rs` | `Provenance { source, baseline_date, description }` on validation results |
| **OrExit** | `validation/or_exit.rs` | `.or_exit("reason")` for zero-panic binary startup |
| **ValidationSink** | `validation/mod.rs` | `StdoutSink`, `NullSink` for pluggable output |
| **IPC resilience** | `ipc/resilience.rs` | `CircuitBreaker`, `RetryPolicy`, `resilient_call()` |
| **IpcError** | `ipc/error.rs` | 8 typed variants with `is_retriable()`, `is_connection_error()`, etc. |
| **IpcErrorPhase** | `ipc/error.rs` | Phase-aware error context: Connect/Serialize/Send/Receive/Parse |
| **PhasedIpcError** | `ipc/error.rs` | `error.in_phase(IpcErrorPhase::Receive)` for diagnostics |
| **DispatchOutcome** | `ipc/dispatch.rs` | Three-way: `Success(T)`, `ProtocolError`, `ApplicationError` |
| **extract_rpc_result** | `ipc/extract.rs` | Centralized JSON-RPC result extraction with typed errors |
| **4-format capability parsing** | `ipc/discover.rs` | Handles Format A/B/C/D wire formats from any primal |
| **6-tier discovery** | `ipc/discover.rs` | env/XDG/plain/temp/manifest/socket-registry (+ Neural API sweep) |
| **MCP tool definitions** | `ipc/mcp.rs` | `McpTool { name, description, input_schema }` with JSON Schema |
| **Spring tool discovery** | `ipc/mcp.rs` | `discover_remote_tools(socket, primal)` to find other springs' MCP tools |
| **Safe cast** | `cast.rs` | Saturating numeric casts (`usize_u32`, `u64_usize`, `micros_u64`) |
| **Named tolerances** | `tolerances/mod.rs` | All latency/throughput bounds as named constants with provenance |
| **Capability registry** | `config/capability_registry.toml` | Single source of truth, sync-tested against code |
| **Primal display names** | `primal_names.rs` | `display_name()` / `discovery_slug()` round-trip (neuralSpring pattern) |
| **Skip-aware exit** | `validation/mod.rs` | `exit_code_skip_aware()`: 0=pass, 1=fail, 2=all-skipped (wetSpring pattern) |
| **Provenance resilience** | `ipc/provenance.rs` | Epoch-based circuit breaker + exponential backoff for trio calls |
| **Cross-cutting proptest** | `ipc/proptest_ipc.rs` | Pipeline-spanning property tests (healthSpring pattern) |
| **normalize_method()** | `ipc/mod.rs` | Ecosystem-wide JSON-RPC method normalization — strips legacy prefixes |
| **check_relative()** | `validation/mod.rs` | Relative-tolerance numeric validation (groundSpring/healthSpring pattern) |
| **check_abs_or_rel()** | `validation/mod.rs` | Combined absolute-or-relative tolerance (avoids false negatives near zero) |
| **NdjsonSink** | `validation/mod.rs` | Streaming NDJSON validation output for CI/log aggregation |
| **is_recoverable()** | `ipc/error.rs` | Broader recovery classification: retriable + server-recoverable errors |
| **Transport** | `ipc/transport.rs` | Unified Unix+Tcp transport with `connect_transport()` address parsing |
| **OnceLock probes** | `ipc/probes.rs` | Cached runtime resource probes for parallel test execution |
| **Release gate** | `scripts/validate_release.sh` | fmt + clippy + deny + test floor + docs CI gate |
| **BTSP handshake** | `ipc/btsp_handshake.rs` | Client-side BTSP authentication (FAMILY_ID + nonce + HMAC) for secure socket connections |
| **InferenceClient** | `inference/mod.rs` | Vendor-agnostic inference client — `complete`, `embed`, `models` via socket discovery |
| **Inference wire types** | `inference/types.rs` | `CompleteRequest`, `EmbedRequest`, `ModelsResponse`, `ProviderInfo` — no vendor lock-in |

### Composition Patterns

| Layer | What primalSpring Validates |
|-------|-----------------------------|
| **Tower Atomic** | BearDog + Songbird discover, health, capabilities |
| **Node Atomic** | Tower + ToadStool + barraCuda + coralReef (proton: dispatch, execute, compile) |
| **Nest Atomic** | Tower + NestGate + rhizoCrypt + loamSpine + sweetGrass (neutron: storage + provenance) |
| **Full NUCLEUS** | Tower + Node + Nest — 9 core primals (atom) |
| **Meta-Tier** | biomeOS + Squirrel + petalTongue — cross-atomic orchestration, AI, UI |
| **Graph Execution** | 5 coordination patterns (Sequential, Parallel, DAG, Pipeline, Continuous) |
| **Emergent Systems** | RootPulse, RPGPT, coralForge pipeline |
| **Bonding** | Covalent, Ionic, Plasmodium multi-gate |
| **Cross-Spring** | Data flow, provenance trio, fieldMouse, petalTongue, Squirrel AI |
| **WGSL Shader Composition** | Springs compose barraCuda/coralReef/toadStool for domain compute (ML, QCD, biology) |
| **Proto-Nucleate Graphs** | `graphs/downstream/*.toml` — target compositions for spring evolution |
| **Pipeline Graphs** | End-to-end data flow models through primal compositions |
| **Dual-Tower Enclave** | Ionic bond between patient-data and analytics towers (healthSpring pattern) |
| **Metallic GPU Pool** | Shared compute fleet via toadStool metallic bonding (hotSpring pattern) |

### What primalSpring Absorbs From

| Source | What | Where in primalSpring |
|--------|------|----------------------|
| hotSpring | Provenance patterns, tolerance structure | `tolerances/`, `validation/mod.rs` |
| wetSpring | IPC resilience stack, cast module, MCP tools, skip_with_code | `ipc/`, `cast.rs`, `ipc/mcp.rs`, `validation/mod.rs` |
| airSpring | deny.toml merged bans, ecoBin enforcement, cast lints | `deny.toml`, `Cargo.toml` |
| groundSpring V120 | ValidationSink (section + write_summary), typed errors, OrExit, deny.toml merge | `validation/`, `ipc/error.rs`, `deny.toml` |
| neuralSpring S170 | Capability registry TOML, discovery module, primal_names::display, cast lint policy | `config/`, `ipc/discover.rs`, `primal_names.rs`, `Cargo.toml` |
| healthSpring V41 | Proptest IPC consolidated module, provenance circuit breaker, MCP tools | `ipc/proptest_ipc.rs`, `ipc/provenance.rs`, `ipc/mcp.rs` |
| ludoSpring V29 | with_provenance(), #[expect(reason)], XDG sockets | `validation/mod.rs`, `Cargo.toml` |
| biomeOS v2.66 | IpcErrorPhase, manifest discovery, socket registry, aligned 6-tier | `ipc/error.rs`, `ipc/discover.rs` |
| Squirrel alpha.21 | Spring tool discovery, socket registry | `ipc/mcp.rs`, `ipc/discover.rs` |

### Inference Wire Standard (v0.9.4)

Springs and primals that handle AI/ML requests use the `inference.*` wire:

| Method | Purpose | When to Use |
|--------|---------|-------------|
| `inference.complete` | Text generation (chat/completion) | Any spring serving or consuming LLM responses |
| `inference.embed` | Vector embedding | Similarity search, RAG, classification |
| `inference.models` | List available models + providers | Discovery: what's available on this node? |

Squirrel is the current bridge (routes to Ollama via `AiRouter`). As springs evolve native WGSL-based inference, the wire standard stays the same — only the provider changes.

### WGSL Shader Composition Pattern (v0.9.4)

The unifying compute pattern: **springs compose barraCuda/coralReef/toadStool, they don't build their own math.**

```
Spring (application layer — defines the problem)
    → coralReef (compiles WGSL programs for the domain)
    → toadStool (dispatches to GPU/CPU substrate)
    → barraCuda (executes 826 WGSL compute shaders)
```

This applies to ML inference (neuralSpring), QCD physics (hotSpring), biology (wetSpring), and any future compute domain. Same shaders, different compositions.

### Proto-Nucleate Absorption Workflow (v0.9.4)

How a spring picks up a proto-nucleate graph and evolves against it:

1. **Read** `graphs/downstream/{yourspring}_*_proto_nucleate.toml` — your target composition
2. **Understand dependencies** — which primals are `required = true` for your domain
3. **Wire IPC** — use ecoPrimal's `PrimalClient` or `InferenceClient` to call primals
4. **Compose** — build your domain logic as orchestration of primal capability calls
5. **Validate** — run primalSpring experiments to verify your composition works
6. **Hand back** — document gaps/patterns discovered, hand back to primalSpring

### Primal Composition Validation Layer (v0.9.9)

The next evolution beyond Rust validation: validate that your spring's domain
logic works **as a composition of primal capabilities**, not just as Rust code.

**The validation maturity ladder**:

```
Level 1: Python baseline        — reference implementation, documented provenance
Level 2: Rust validation        — faithful port, pass/fail exit code, tolerance-gated
Level 3: barraCuda CPU          — primal math (WGSL CPU fallback), same tolerances
Level 4: barraCuda GPU          — sovereign shader execution, performance validated
Level 5: Primal composition     — domain logic as IPC calls to NUCLEUS primals
Level 6: Deploy graph validated — proto-nucleate satisfied, all nodes healthy
```

**What composition validation proves**:
- Your domain logic **delegates** to primals instead of reimplementing
- IPC round-trips stay within latency budgets (named tolerances)
- The NUCLEUS atomics your proto-nucleate declares are **live and healthy**
- Graceful degradation works when optional primals are absent

**Standardized patterns** for composition validation are documented in
`infra/wateringHole/SPRING_COMPOSITION_PATTERNS.md`. Every spring MUST adopt:
- Method normalization (§1)
- Capability registration as contract (§2)
- Tiered socket discovery (§3)
- Graceful degradation for provenance (§7)
- Niche identity with dependencies (§11)

### BTSP Client Handshake Pattern (v0.9.4)

All socket connections to BTSP Phase 2 primals must authenticate:

```rust
use primalspring::ipc::btsp_handshake;
// After connecting to a primal socket:
btsp_handshake::perform(&mut stream, family_id, nonce)?;
// Connection is now authenticated — proceed with capability calls
```

### Upstream/Downstream Evolution Cycle

```
primals (base capabilities)
    ↓ expose capabilities
primalSpring (composition patterns + proto-nucleate graphs)
    ↓ graphs/downstream/*.toml
springs (domain applications — absorb + evolve)
    ↓ discover gaps + new patterns
primalSpring (absorbs patterns, refines compositions)
    ↓ primal-level gaps
primals (evolve to close gaps)
    ↓ ... cycle continues
```

### NUCLEUS Atomic Alignment (Phase 31)

All proto-nucleate graphs, deploy graphs, and validation infrastructure now
use the Phase 31 atomic model. Springs should reference this model when
building their compositions:

| Atomic | Particle | Primals | Fragment |
|--------|----------|---------|----------|
| Tower | Electron | BearDog + Songbird | `tower_atomic` |
| Node | Proton | Tower + ToadStool + barraCuda + coralReef | `node_atomic` |
| Nest | Neutron | Tower + NestGate + rhizoCrypt + loamSpine + sweetGrass | `nest_atomic` |
| NUCLEUS | Atom | Tower + Node + Nest (9 unique primals) | `nucleus` |
| Meta-tier | — | biomeOS + Squirrel + petalTongue | `meta_tier` |

Deploy graphs declare their atomics via `fragments = [...]` metadata.
Bonding policies document how atomics bind within cross-atomic compositions.

Each spring solving its domain unlocks patterns for all others. hotSpring's
GPU work drove coralReef evolution. neuralSpring's ML patterns will flow to
every spring that needs inference. healthSpring's enclave pattern applies to
any spring handling sensitive data.

---

## How to Compose with primalSpring

### As a Primal Consumer (springs, primals)

1. Discover primalSpring: `discover_primal("primalspring")`
2. Check health: `health.liveness` → `health.readiness`
3. Request coordination: `coordination.validate_composition { "atomic": "Tower" }`
4. List deploy graphs: `graph.list`
5. Validate a graph: `graph.validate { "path": "...", "live": true }`

### As a Squirrel AI Consumer

1. Discover tools: `mcp.tools.list` → 8 typed tools with JSON Schema
2. Route requests: tool name maps 1:1 to JSON-RPC methods
3. Use `tool_to_method()` for name resolution

### As a biomeOS Graph Node

primalSpring ships 93 deploy graph TOMLs (all nodes declare `by_capability`).
biomeOS orchestrates the niche directly from these graphs. `topological_waves()`
computes startup ordering. primalSpring participates as a validator node that
probes other nodes by capability and reports composition health.

---

## Cross-Architecture Leverage

primalSpring is the reference implementation for cross-architecture deployment.
Every pattern above works identically on `x86_64` and `aarch64`.

### What primalSpring Proves

| Capability | Proven |
|------------|--------|
| `aarch64-unknown-linux-musl` cross-compile | 2.99 MB static, runs on Pixel 8a |
| Full workspace cross-compile (67 experiments + server) | 1.27s incremental |
| JSON-RPC coordination over abstract sockets | Same protocol, different transport |
| Zero architecture-specific code | All arch concerns in env/transport config |

### What Other Primals Should Absorb

1. **musl target**: `cargo build --release --target aarch64-unknown-linux-musl`
2. **Release profile**: `strip = true`, `lto = true` in `[profile.release]`
3. **Abstract socket support**: `@biomeos/{primal}` for Android deployment
4. **Env-first config**: `FAMILY_ID`, `NODE_ID`, `{PRIMAL}_SOCKET` — no filesystem assumptions

### Reference Documents

- `PRIMAL_CAPABILITY_STATUS_MAR22_2026.md` — per-primal open items and compliance
- `ECOBIN_GENOMEBIN_EVOLUTION_GUIDANCE_MAR22_2026.md` — ecoBin/genomeBin evolution roadmap
- `PRIMALSPRING_V070_HARDWARE_VALIDATION_HANDOFF_MAR22_2026.md` — full hardware audit

---

## How Springs Help Primals Evolve

Each spring is a **domain laboratory**. When a spring solves its domain problem
by composing NUCLEUS primals, it simultaneously stress-tests and evolves those
primals. The patterns discovered feed back through primalSpring to the primal
teams. This section maps each spring's domain work to the primal evolution it
drives.

### The Feedback Protocol

When a spring discovers a gap or pattern, it should:

1. **Document the gap** — what capability is missing, what the workaround is
2. **Propose the wire** — what JSON-RPC method signature would close the gap
3. **Build validation** — a primalSpring experiment or graph that tests the gap
4. **Hand back** — PR to primalSpring `docs/PRIMAL_GAPS.md` + graph in `graphs/downstream/`

primalSpring triages, refines, and routes the gap to the responsible primal.

### Per-Spring Evolution Map

| Spring | Domain | Primary Atomic | Primals Evolved | What They Teach |
|--------|--------|----------------|-----------------|-----------------|
| **hotSpring** | QCD / physics | Node (proton-heavy) | barraCuda, coralReef, toadStool | df64 GPU precision, shader pipeline scaling, multi-GPU federation, compute budget management |
| **neuralSpring** | ML / AI | Node + Meta | barraCuda, coralReef, Squirrel | Tokenization as WGSL shader, inference pipeline composition, model routing, embedding workflows |
| **wetSpring** | Biology / ecology | Node + Nest | barraCuda, NestGate, provenance trio | Sensor ingestion patterns, time-series storage, biodiversity attribution, fieldMouse edge composition |
| **airSpring** | Weather / atmosphere | Node | barraCuda, coralReef | Atmospheric simulation shaders, real-time sensor fusion, streaming pipeline patterns |
| **groundSpring** | Soil / geology | Node + Nest | barraCuda, NestGate | Long-duration storage patterns, geospatial indexing, soil model shaders |
| **healthSpring** | Clinical / compliance | Nest (neutron-heavy) | NestGate, BearDog, provenance trio | Ionic bonding enforcement, data egress fences, dual-tower enclave, HIPAA audit trails |
| **ludoSpring** | Game science | Node + Meta | barraCuda, petalTongue, Squirrel | 60Hz tick budgets, real-time shader composition, AI narration latency, pure composition model |
| **esotericWebb** | CRPG / narrative | Full NUCLEUS + Meta | All primals | End-to-end product validation, session lifecycle, persistence + provenance + AI + render as single graph |

### What Each Spring Hands Back

**hotSpring → barraCuda / coralReef**
- df64 emulation patterns (double-precision on consumer GPU)
- Shader pipeline scaling beyond single-GPU dispatch
- Compute budget allocation for long-running QCD lattice sweeps
- Multi-node GPU federation via metallic bonding

**neuralSpring → barraCuda / coralReef / Squirrel**
- Tokenization as a WGSL compute shader (not a CPU-side library)
- Attention/FFN forward pass shaders for inference
- Model weight storage patterns (NestGate as model registry)
- `inference.*` wire evolution: streaming responses, batch embedding

**wetSpring → NestGate / provenance trio**
- Time-series storage optimizations for high-frequency sensor data
- Provenance patterns for biodiversity attribution
- fieldMouse chimera patterns (Tower-only edge deployment)
- Cross-spring ecology pipeline (data federation)

**healthSpring → BearDog / NestGate**
- Ionic bond runtime enforcement (cross-family trust negotiation)
- `crypto.sign_contract` capability for bond establishment
- Data egress fence implementation in NestGate bonding policy
- Regulatory audit trail patterns via provenance trio

**ludoSpring / esotericWebb → petalTongue / Squirrel / barraCuda**
- 60Hz composition budget: how fast can a graph execute end-to-end?
- AI narration latency: Squirrel response time under game-tick pressure
- Pure composition proof: complex product with zero downstream binary
- Session lifecycle: create → save → restore → fork across compositions

### Evolution Priority by Primal

| Primal | Primary Spring Drivers | Key Evolution |
|--------|----------------------|---------------|
| **barraCuda** | hotSpring, neuralSpring, ludoSpring | df64 precision, ML shaders, tick-budget dispatch |
| **coralReef** | hotSpring, neuralSpring | Domain-specific shader compilation, pipeline optimization |
| **toadStool** | hotSpring, ludoSpring | Multi-GPU dispatch, tick-budget scheduling, federation |
| **NestGate** | healthSpring, wetSpring, groundSpring | Egress fences, time-series storage, geospatial indexing |
| **BearDog** | healthSpring | Ionic bond contracts, cross-family trust, regulatory crypto |
| **Songbird** | all (federation) | NAT traversal improvements, mesh scaling, relay protocols |
| **Provenance trio** | healthSpring, wetSpring | Audit trail patterns, attribution granularity, federation |
| **Squirrel** | neuralSpring, ludoSpring | Inference routing, model discovery, real-time AI under latency constraints |
| **petalTongue** | ludoSpring, esotericWebb | Scene rendering composition, dashboard patterns, TUI push |
| **biomeOS** | all (orchestration) | Graph execution performance, tick-loop scheduling, deploy class resolution |

### Starting a Spring Evolution Session

```
1. Read your proto-nucleate:
   graphs/downstream/{yourspring}_*_proto_nucleate.toml

2. Check the deployment matrix for your topology:
   config/deployment_matrix.toml → find cells matching your atomic

3. Wire your domain logic to primal capabilities via IPC:
   use ecoPrimal::ipc::PrimalClient;

4. Run primalSpring validation experiments:
   cargo run --bin validate_all -- --filter {yourspring}

5. Document gaps in primalSpring:
   docs/PRIMAL_GAPS.md → add your spring's findings

6. Propose new graph patterns:
   graphs/downstream/ or graphs/science/ → submit via PR

7. Hand back patterns to primalSpring for refinement
```

Each cycle makes the whole ecosystem stronger. A gap discovered by hotSpring
in barraCuda's df64 handling benefits neuralSpring's ML precision.
A bonding model hardened by healthSpring's compliance work protects every
spring's data sovereignty. The atomics are shared infrastructure — springs
are domain laboratories that evolve it.

---

**License**: AGPL-3.0-or-later
