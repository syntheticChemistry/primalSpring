# primalSpring

**Coordination and composition validation spring for the ecoPrimals ecosystem.**

| | |
|-|-|
| **Domain** | Primal coordination, atomic composition, graph execution, emergent systems, multi-node bonding + federation |
| **Version** | 0.9.4 |
| **Edition** | Rust 2024 (1.87+) |
| **License** | AGPL-3.0-or-later |
| **Tests** | 404 (unit + integration + doc-tests + proptest) |
| **Experiments** | 72 (15 tracks) |
| **Deploy Graphs** | 93 TOMLs (7 root + 9 profiles + 12 sketches + 5 multi-node + 13 spring validation + 2 cross-spring + 10 gen4 + 5 bonding + 2 chaos + 13 science + 5 spring deploy + 5 downstream proto-nucleate + 3 pipeline + 4 patterns) + 6 fragment definitions |
| **Coverage** | 72.5% library line coverage (llvm-cov) |
| **Compositions** | Tower + Nest + Node + NUCLEUS + Graph Overlays + Squirrel Discovery + Graph Execution + Provenance Trio + Multi-Node Bonding + biomeOS Substrate + Cross-Gate + Deployment Matrix + Substrate Stress + Pure Composition (ludoSpring + esotericWebb as graph-defined products) + **7 Decomposed Subsystems (C1-C7)** + **Mixed Atomics (L2) + Bonding Patterns (L3)** (87/87 gates) |
| **Subsystems** | C1: Render (petalTongue) + C2: Narration (Squirrel) + C3: Session (esotericWebb) + C4: Game Science (ludoSpring) + C5: Persistence (NestGate) + C6: Proprioception (petalTongue) + C7: Full Interactive |
| **Provenance** | All 72 experiments carry structured `with_provenance()` metadata |
| **Clippy** | 0 warnings (pedantic + nursery + cast discipline + unwrap/expect discipline) |
| **Unsafe** | Workspace-level `forbid` via `[workspace.lints.rust]` |
| **C deps** | Zero (ecoBin compliant, `deny.toml` enforced) |

---

## What Is primalSpring?

primalSpring is the spring whose domain IS coordination. Where other springs
validate domain science (hotSpring validates physics, wetSpring validates
biology), primalSpring validates the ecosystem itself — the coordination,
composition, and emergent behavior that biomeOS and the Neural API produce
when primals work together.

Its "papers" are the atomics. Its "experiments" are composition patterns.
Its validation target is biomeOS itself.

## Architecture

```
primalSpring/
├── ecoPrimal/                     # Library crate + UniBin binaries
│   ├── src/
│   │   ├── lib.rs                 # Library root (unsafe_code = forbid)
│   │   ├── cast.rs                # Safe numeric casts (saturating boundary)
│   │   ├── coordination/          # Atomic composition, health probing, composition validation
│   │   ├── deploy/                # Deploy graph parsing, structural + live validation
│   │   ├── graphs/                # Graph execution pattern types (5 patterns)
│   │   ├── emergent/              # Emergent system validation (RootPulse, RPGPT, CoralForge)
│   │   ├── bonding/               # Multi-gate bonding models (Covalent, Metallic, Ionic, Weak, OMS) + graph metadata + STUN tiers
│   │   ├── ipc/                   # JSON-RPC 2.0 client, discovery, capability, error, dispatch, extract, resilience, transport, tcp, methods, probes, provenance, proptest, btsp_handshake
│   │   ├── inference/             # Vendor-agnostic inference wire types + InferenceClient (inference.complete/embed/models)
│   │   ├── launcher/              # Primal binary discovery, spawn, profiles, socket nucleation (sync biomeOS port)
│   │   ├── harness/               # Atomic test orchestration: spawn compositions, validate, RAII teardown
│   │   ├── niche.rs               # BYOB niche self-knowledge (capabilities, semantic mappings, registration)
│   │   ├── primal_names.rs        # Canonical slug constants, display names ↔ discovery slugs (neuralSpring pattern)
│   │   ├── validation/            # Experiment harness (check_bool, check_skip, check_relative, OrExit, ValidationSink, NdjsonSink, builder .run())
│   │   └── tolerances/            # Named latency and throughput bounds
│   ├── src/bin/
│   │   ├── primalspring_primal/   # UniBin: JSON-RPC 2.0 server with niche registration
│   │   └── validate_all/          # Meta-validator: runs all 72 experiments
│   └── tests/
│       ├── integration/           # Shared test helpers (guards, spawn, RPC)
│       ├── server_integration.rs  # 10 core auto tests
│       ├── server_ecosystem.rs    # Tower-related live tests (#[ignore])
│       └── server_ecosystem_compose.rs  # Nest/Node/Overlay/Squirrel live tests (#[ignore])
├── experiments/                   # 72 validation experiments (15 tracks)
├── config/                        # Launch profiles, deployment matrix, capability registry
├── graphs/                        # 93 deploy graph TOMLs + 6 fragment definitions
│   ├── profiles/                 # NUCLEUS atomic profiles: tower, node, nest, nucleus, full + meta overlays (9)
│   ├── patterns/                 # Execution patterns: parallel, conditional, streaming, continuous (4)
│   ├── fragments/                # Atomic architecture: tower_atomic, node_atomic, nest_atomic, meta_tier, nucleus, provenance_trio (6)
│   ├── sketches/                 # Particle-model validation & mixed composition sketches (12)
│   │   ├── validation/           # L0 primal routing matrix + ludo/webb validates (5)
│   │   ├── mixed_atomics/        # L2 dual-tower ionic, dedicated tower, nest enclave (3)
│   │   └── bonding_patterns/     # L3 covalent mesh backup, ionic lease, organo-metal-salt (3)
│   ├── bonding/                  # Bonding model graphs: ionic, metallic, OMS, defensive, albatross (5)
│   ├── chaos/                    # Chaos engineering: partition recovery, slow start (2)
│   ├── multi_node/               # Multi-node federation graphs (5)
│   ├── science/                  # Science + showcase graphs: provenance, fieldMouse, gaming, neuro (13)
│   ├── spring_validation/        # Template + manifest + composition + security validation (13)
│   ├── cross_spring/             # Cross-spring ecology + full sweep (2)
│   ├── gen4/                     # gen4: sovereign, science, agentic, storytelling, UI loop (10)
│   ├── spring_deploy/            # Per-spring science compositions (5)
│   └── downstream/               # Proto-nucleate graphs for spring evolution (5)
├── docs/                          # Structured gap registry and subsystem documentation
│   └── PRIMAL_GAPS.md            # Per-primal gap inventory with severity and fix paths
├── tools/                         # Operational tooling
│   ├── nucleus_launcher.sh       # Start/stop/restart full NUCLEUS stack
│   ├── ws_gateway.py             # Thin WebSocket-to-IPC bridge (no business logic)
│   └── validate_compositions.py  # Live subsystem composition validator (C1-C7)
├── web/
│   └── play.html                 # Composition monitor / debug dashboard (not primary UI)
├── niches/                        # BYOB niche deployment YAML
├── specs/                         # Architecture specs
└── wateringHole/                  # Docs and handoffs
```

## Key Design Principles

- **Capability-first discovery**: No hardcoded primal rosters. Discovery is
  capability-based (via `discover_by_capability()`) or routed through the
  biomeOS Neural API substrate (via `NeuralBridge`). Deploy graphs use
  `by_capability` for loose coupling — callers ask for capabilities, not
  primal identities.
- **Graphs as source of truth**: Deploy graph TOMLs define what capabilities
  a composition needs. `topological_waves()` computes startup ordering from
  dependency edges. `graph_required_capabilities()` extracts the capability
  roster from graph nodes.
- **Graceful degradation**: Experiments honestly skip checks when providers
  aren't running — never fake a pass.
- **Zero domain knowledge**: primalSpring validates coordination, not math.
  Zero barraCuda, zero WGSL, zero domain science.
- **BYOB niche model**: Full self-knowledge via `niche.rs` — capabilities,
  semantic mappings, operation dependencies, cost estimates, and runtime
  registration with biomeOS.

## Quick Start

```bash
# Build everything
cargo build --workspace

# Run all tests (auto + ignored live tests)
cargo test --workspace

# Run live atomic tests (requires plasmidBin binaries)
ECOPRIMALS_PLASMID_BIN=../plasmidBin cargo test --ignored

# Run all 72 experiments (meta-validator)
cargo run --release --bin validate_all

# Run exp001 with live primals (harness auto-starts them)
ECOPRIMALS_PLASMID_BIN=../plasmidBin cargo run --bin exp001_tower_atomic

# Start the primalSpring JSON-RPC server
cargo run --bin primalspring_primal -- server

# Show ecosystem status
cargo run --bin primalspring_primal -- status
```

### Code coverage ([cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov))

Install the tool (`cargo install cargo-llvm-cov --locked` or a release binary) and `rustup component add llvm-tools-preview`, then:

```bash
cargo coverage
```

This runs LLVM source-based coverage for the whole workspace, skips paths matching `tests/` in the report, and fails if **line** coverage is below **90%**. For HTML output, run `cargo llvm-cov --workspace --html` (see upstream docs for `--open`, `--lcov`, CI, etc.).

## Server Mode

The `primalspring_primal` binary exposes coordination capabilities via JSON-RPC 2.0:

| Method | Description |
|--------|-------------|
| `health.check` | Self health status |
| `health.liveness` | Kubernetes-style liveness probe |
| `health.readiness` | Readiness probe (Neural API + discovered primals) |
| `capabilities.list` | Niche capabilities + semantic mappings + cost estimates |
| `coordination.validate_composition` | Validate an atomic composition (capability-based by default) |
| `coordination.validate_composition_by_capability` | Explicitly capability-based validation |
| `coordination.discovery_sweep` | Enumerate capabilities in a composition |
| `coordination.probe_capability` | Probe a single capability provider |
| `coordination.neural_api_status` | Neural API reachability |
| `graph.list` | Structurally validate all deploy graphs |
| `graph.validate` | Validate a specific graph (structural or live) |
| `graph.waves` | Compute topological startup waves from a deploy graph |
| `graph.capabilities` | Extract required capabilities from a deploy graph |
| `lifecycle.status` | Primal status report |
| `mcp.tools.list` | MCP tool definitions for Squirrel AI |

## Deployment Matrix

primalSpring includes a deployment validation matrix (`config/deployment_matrix.toml`) that
defines **43 test cells** across architectures (x86_64, aarch64), topologies, network presets,
and transport modes. Each cell validates a specific primal composition under specific conditions.

```bash
# Run all cells (dry-run to see what would execute)
scripts/validate_deployment_matrix.sh --dry-run --all

# Run a specific cell
scripts/validate_deployment_matrix.sh --cell tower-x86-homelan

# Run all cells in an experiment group
scripts/validate_deployment_matrix.sh --tier docker
```

Topology categories: Tower (2-node), NUCLEUS (3-node), Federation (10-node), Bonding (ionic, metallic, OMS),
Showcase (fieldMouse, Albatross, skunkBat, neuromorphic, gaming), Agentic (biomeOS+Squirrel+petalTongue),
Storytelling (esotericWebb+ludoSpring+Squirrel+petalTongue).

## Deploy Graphs

primalSpring ships 93 deploy graph TOMLs + 6 atomic-aligned fragments (all nodes declare `by_capability`):

**Single-node graphs (17)**:

| Graph | Pattern | Primals |
|-------|---------|---------|
| `tower_atomic_bootstrap.toml` | Sequential | beardog, songbird |
| `tower_full_capability.toml` | Sequential | beardog, songbird (full caps) |
| `nest_deploy.toml` | Sequential | beardog, songbird, nestgate, squirrel |
| `node_atomic_compute.toml` | Sequential | beardog, songbird, toadstool |
| `nucleus_complete.toml` | Sequential | biomeos, beardog, songbird, nestgate, toadstool, squirrel (+trio) |
| `tower_ai.toml` | Sequential | beardog, songbird, squirrel |
| `nest_viz.toml` | Sequential | beardog, songbird, nestgate, petaltongue |
| `node_ai.toml` | Sequential | beardog, songbird, toadstool, squirrel |
| `full_overlay.toml` | Sequential | beardog, songbird, nestgate, toadstool, squirrel |
| `provenance_overlay.toml` | Sequential | beardog, songbird, rhizocrypt, loamspine, sweetgrass |
| `parallel_capability_burst.toml` | Parallel | beardog, songbird, nestgate, toadstool |
| `conditional_fallback.toml` | ConditionalDag | beardog, songbird, toadstool |
| `streaming_pipeline.toml` | Pipeline | beardog, nestgate, sweetgrass |
| `continuous_tick.toml` | Continuous | all 7 primals |
| `coralforge_pipeline.toml` | Pipeline | beardog, songbird, nestgate, toadstool, sweetgrass |
| `primalspring_deploy.toml` | Sequential | primalspring coordination |
| `spring_byob_template.toml` | Sequential | template for new springs |

**Multi-node federation graphs (5)** — `graphs/multi_node/`:

| Graph | Scenario | Bond Type | Trust Model |
|-------|----------|-----------|-------------|
| `basement_hpc_covalent.toml` | LAN HPC mesh | Covalent | GeneticLineage |
| `friend_remote_covalent.toml` | Remote friend + NAT traversal | Covalent | GeneticLineage |
| `idle_compute_federation.toml` | Federated idle compute sharing | Covalent | GeneticLineage |
| `data_federation_cross_site.toml` | NestGate cross-site replication | Covalent | GeneticLineage |

**Composition subsystem graphs (3)** — `graphs/compositions/`: each subfunction
decomposed into a separately deployable biomeOS graph with its own validation:
`narration_ai` (C2: Squirrel), `persistence_standalone` (C5: NestGate),
`proprioception_loop` (C6: petalTongue interaction).

**Spring validation graphs (17)** — `graphs/spring_validation/`: per-spring validation
wrappers (7 springs) plus `crypto_negative_validate` (negative security boundary tests),
`rollback_validate` (biomeOS graph rollback lifecycle), `federation_manifest_validate`
(federation configure→join→health), `nucleus_atomics_validate` (all 4 NUCLEUS tiers),
3 product validation (esotericwebb_tower, esotericwebb_composed, ludospring_game),
7 composition subsystem validation (C1-C7: render, narration, session, game_science,
persistence, proprioception, interactive).
All graphs include biomeOS Neural API as orchestration substrate.

**Cross-spring graphs (2)** — `graphs/cross_spring/`: ecology validation
(ET₀ → diversity → spectral) and full sweep across all springs.

**Bonding model graphs (5)** — `graphs/bonding/`: ionic capability sharing,
metallic GPU pool, organo-metal-salt complex, defensive mesh (skunkBat),
Albatross multiplex (Songbird fleet).

**Chaos engineering graphs (2)** — `graphs/chaos/`: network partition recovery
and slow-start composition convergence.

**Science + showcase graphs (13)** — `graphs/science/`: coralForge federated, ecology
provenance, reproducibility audit, fieldMouse ingestion, paper lifecycle, supply chain
provenance, mixed entropy hierarchy, gaming mesh chimera, neuromorphic classify,
RPGPT session provenance.

**gen4 graphs (10)** — `graphs/gen4/`: sovereign tower, science substrate, agentic tower,
interactive substrate, spring composition, **agentic substrate** (biomeOS+Squirrel+petalTongue),
**agentic fieldMouse**, **UI-orchestrator loop**, **storytelling full** (esotericWebb+ludoSpring+Squirrel AI DM),
**storytelling minimal** (offline play).

**Spring science deploy graphs (6)** — `graphs/spring_deploy/`: per-spring compositions
for deploying domain science as biomeOS compositions. Each includes biomeOS Phase 0
substrate, Tower base, optional ToadStool for GPU-compute springs (hotSpring, neuralSpring,
wetSpring, groundSpring), and the spring primal. Springs reference primalSpring's
wateringHole patterns for composition standards.

All graphs have `by_capability` on every node and are structurally validated +
topologically sorted at test time. Multi-node graphs include `[graph.metadata]`
and `[graph.bonding_policy]` sections validated by `graph_metadata.rs`.

## IPC Resilience

Converged IPC resilience stack absorbed from 7 sibling springs: `IpcError` (8
typed variants with `is_retriable()`, `is_recoverable()`, `is_timeout_likely()`,
etc.), `CircuitBreaker`, `RetryPolicy`, `resilient_call()`, `DispatchOutcome<T>`,
`Transport` enum (Unix + Tcp), `normalize_method()` for prefix-agnostic dispatch,
`OnceLock`-cached runtime probes, and centralized `extract_rpc_result` /
`extract_rpc_dispatch` for JSON-RPC result extraction. Capability parsing handles
all 4 ecosystem wire formats (A/B/C/D). Provenance trio calls use an epoch-based
circuit breaker with exponential backoff (absorbed from healthSpring V41).

## Discovery (6-Tier)

Discovery walks 6 tiers in priority order:
1. `{PRIMAL}_SOCKET` env override
2. `$XDG_RUNTIME_DIR/biomeos/{primal}-{family}.sock` (XDG convention)
3. Plain socket name: `{primal}.sock` or `{primal}-ipc.sock`
4. `{temp_dir}/biomeos/{primal}-{family}.sock` (temp fallback)
5. Primal manifest: `$XDG_RUNTIME_DIR/ecoPrimals/manifests/{primal}.json`
6. Socket registry: `$XDG_RUNTIME_DIR/biomeos/socket-registry.json`

Tiers 5–6 absorbed from biomeOS v2.50 and Squirrel alpha.12.

## MCP Tools

primalSpring exposes 8 typed MCP tools via `mcp.tools.list` for Squirrel AI
coordination tool discovery. Each tool has a JSON Schema input definition.

## Live Atomic Harness

primalSpring absorbs primal coordination from biomeOS — binary discovery,
socket nucleation, process spawning, and wave-based startup — ported to
pure synchronous Rust (`std::process` + `std::thread`, no tokio).

| Module | Responsibility |
|--------|---------------|
| `launcher/` | `discover_binary()`, `spawn_primal()`, `spawn_biomeos()`, `wait_for_socket()`, `SocketNucleation`, `LaunchProfile`, `LaunchError` (incl. `HealthCheckFailed`) |
| `harness/` | `AtomicHarness::new()` / `::with_graph()`, `.start()` (topological waves), `.start_with_neural_api()`, `RunningAtomic` (capability-based `socket_for` / `client_for`, RAII lifecycle, NeuralBridge) |

Set `ECOPRIMALS_PLASMID_BIN` to point at `ecoPrimals/plasmidBin/` to enable
live primal spawning. Without it, experiments fall back to discovering
whatever is already running.

## gen4 Deployment Evolution

primalSpring validates the biomeOS substrate model end-to-end: biomeOS as the
neural-api orchestrator, capability routing across primals (crypto, beacon,
mesh, AI, visualization), and cross-gate deployment to heterogeneous hardware.

**Phase 18 (done)**: Full NUCLEUS deployed and validated on Eastgate (biomeOS +
BearDog + Songbird + NestGate + Squirrel, all running concurrently under seed-derived
FAMILY_ID `8ff3b864a4bc589a`). Cross-gate federation: Pixel Songbird TCP via ADB
port forwarding, registered on Eastgate biomeOS via `route.register`. Mobile
SELinux gap documented — GrapheneOS blocks `sock_file` creation for `shell` context,
requiring TCP-only transport modes for all primals on Android.

**Phase 17 (done)**: biomeOS neural-api validated on Eastgate in coordinated mode
with 24 capability domains and 39 deploy graphs. Cross-gate routing to Pixel
via ADB-forwarded TCP. Squirrel AI primal validated. Spring deploy sweep confirms
all 7 sibling springs' biomeOS graphs load correctly.

**gen4 prototypes** (graphs/gen4/): sovereign tower (Dark Forest ready), science
substrate (multi-spring pipeline), agentic tower (AI-orchestrated), interactive
substrate (full UI + AI + crypto + mesh surface), **spring composition**
(Tower + biomeOS + 5 spring primals + cross-spring capability routing).

**Spring Gen4 Scaffolding** (Phase 19): 5 of 6 spring primal binaries built
and deployed to `plasmidBin/springs/` (groundspring, healthspring_primal,
ludospring, neuralspring, wetspring). All spring validation graphs updated to
deploy biomeOS as substrate. Launch profiles added for all 6 springs.

See `specs/CROSS_SPRING_EVOLUTION.md` for full evolution path.

## BTSP, Inference Abstraction, and Proto-Nucleate Graphs (April 10, 2026)

- **Zero-Port Tower Atomic achieved**: 0 TCP ports across all 10+ primals. Pure Unix domain socket IPC with BTSP handshake authentication.
- **BTSP Phase 2 ecosystem cascade**: 11/13 primals enforce `btsp.handshake` on connection. Client-side `ipc::btsp_handshake` module added to ecoPrimal. All deploy graphs carry `secure_by_default = true` metadata.
- **Inference provider abstraction**: Vendor-agnostic `inference.complete`/`embed`/`models` wire standard in ecoPrimal. Squirrel bridge routes through `AiRouter` (Ollama as OpenAI-compatible HTTP endpoint). No vendor lock-in to CUDA or Ollama.
- **WGSL shader composition model**: ML inference, QCD physics, and biology are compositions of existing barraCuda WGSL shaders (826 kernels: matmul, attention, FFT, df64) compiled by coralReef and dispatched by toadStool.
- **5 proto-nucleate graphs** (`graphs/downstream/`): neuralSpring ML inference, hotSpring QCD (metallic GPU pool, df64, provenance), healthSpring dual-tower enclave (ionic bond, egress fence, clinical AI).
- **3 pipeline graphs**: neuralSpring inference pipeline, hotSpring QCD pipeline, healthSpring clinical pipeline — modeling end-to-end data flow through primal compositions.
- **13/13 critical experiments ALL PASS** — 72 total experiments across 15 tracks.
- **12/12 plasmidBin musl-static ecoBin compliant** — zero C dependencies.

## Graph Consolidation + Composition Evolution (April 9, 2026)

- **ludoSpring and esotericWebb evolved to pure compositions**: No longer spawnable binaries — the graph IS the product, biomeOS IS the execution engine. All prior "binary" work is validation that proves composition patterns work.
- **6 atomic-aligned fragments** (`graphs/fragments/`): `tower_atomic` (electron), `node_atomic` (proton), `nest_atomic` (neutron), `nucleus` (full atom), `meta_tier` (cross-atomic), `provenance_trio` (Nest sub-pattern) — aligned to the 3 NUCLEUS atomics.
- **7 graphs deleted** (5 redundant sketches + 2 duplicate compositions): `ludospring_game_deploy`, `esotericwebb_tower_deploy`, `game_science_standalone`, `session_standalone`, `esotericwebb_composed_deploy`, `render_standalone`, `tower_ai_viz`.
- **10 graphs rewritten**: 2 proto-nucleates + 8 science/gen4/sketch graphs — all ludo/webb binary nodes replaced with constituent NUCLEUS primals.
- **100% fragment + composition_model metadata**: Every deploy graph annotated with `fragments` and `composition_model` (pure/nucleated/validation).
- **Gen4 naming normalized**: All 10 gen4 graphs use canonical `biomeos_neural_api` node name.
- Deploy graph count: 100 → 93 (+ 6 atomic-aligned fragment definitions). 8 root-level subset graphs → 9 profiles. 6 per-spring validates → 1 template + manifest. 3 compositions absorbed. hotspring_deploy merged into proto-nucleate. 4 execution patterns moved to `patterns/`.

## Mixed Composition + Live Validation Matrix (April 7, 2026)

- **Particle model adopted**: Tower = electron (trust boundary, mediates bonds), Node = proton (compute, fungible), Nest = neutron (data at rest, non-fungible), NUCLEUS = atom. Grounded in Paper 23 (Mass-Energy-Information Equivalence). See `specs/MIXED_COMPOSITION_PATTERNS.md`.
- **Layered validation framework**: L0 (biomeOS + any primal), L1 (each atomic), L2 (mixed atomics), L3 (bonding patterns on top of atomics). Full NUCLEUS Validation Matrix at `specs/NUCLEUS_VALIDATION_MATRIX.md`.
- **12 sketch graphs** (`graphs/sketches/`): L0 primal routing matrix, L2 dual-tower ionic / dedicated tower / nest enclave, L3 covalent mesh backup / ionic lease / organo-metal-salt.
- **3 new experiments** (exp091–093): L0 primal routing matrix, L2 dual-tower ionic structural, L3 covalent mesh backup structural.
- **Live validation on Eastgate**: Tower Atomic (BearDog + Songbird) fully validated — crypto, HTTPS, discovery all LIVE PASS. Neural API running but capability registration gap identified (GAP-MATRIX-01). See `specs/CROSS_SPRING_EVOLUTION.md` for 6 gap items.
- **6 GAP-MATRIX items documented**: Neural API capability registration (Critical), biomeOS graph parsing (Medium), Songbird TLS cipher suites (Low), NestGate IPC model (Medium), untested primals (Medium), plasmidBin freshness (Low).

## Modernization Sweep (April 7, 2026)

- **Graph format unified**: All 92+ graphs migrated from `[[graph.node]]` to canonical `[[graph.nodes]]`. Parser accepts both via serde alias; `GraphMeta` gains optional `id` field for biomeOS `GraphId` compatibility.
- **Capability names canonical**: `dag.dehydration.trigger`, `dag.session.create`, `dag.event.append`, `dag.merkle.root`, `session.commit`, `entry.append` — cleaned across 17 graph files, `capability_registry.toml`, and `niche.rs`.
- **`http_health_probe` deprecated**: All primals expose JSON-RPC `health.liveness`. Four experiments (exp073, exp074, exp076, exp081) updated to use `tcp_rpc` instead.
- **`nest-deploy.toml` v4.0**: Gold standard graph — BearDog + Songbird + NestGate + Squirrel + HTTPS validation phase + composition validation.
- **exp090 Tower Atomic LAN probe**: BirdSong mesh discovery, peer capability enumeration, HTTPS through Tower Atomic, STUN/NAT detection.
- **exp073 covalent bonding modernized**: Neural API routing validation, `FAMILY_ID` genetic lineage, end-to-end HTTPS.
- **Basement HPC graph aligned**: `basement_hpc_covalent.toml` updated with canonical capability names and HTTPS validation phase.
- **NA-009, NA-016 resolved** in `specs/CROSS_SPRING_EVOLUTION.md`.

## Live Composition Validation (April 1, 2026)

7 decomposed subsystem compositions validated independently against live stack:

| Composition | Result | Notes |
|-------------|--------|-------|
| C1: Render (petalTongue) | **6/6 PASS** | Dashboard render, SVG export, SceneGraph storage, session awareness |
| C2: Narration (Squirrel) | 3/4 PARTIAL | SQ-02 resolved (code wired); `ai.query` fails only because no local Ollama running |
| C3: Session (esotericWebb) | **8/8 PASS** | Full session lifecycle, actions, act, graph |
| C4: Game Science (ludoSpring) | **6/6 PASS** | Flow, Fitts, WFC, engagement |
| C5: Persistence (NestGate) | **5/5 PASS** | Store, retrieve round-trip, list with `family_id` |
| C6: Proprioception (petalTongue) | **5/5 PASS** | Subscribe, apply, poll, showing |
| C7: Full Interactive | **10/10 PASS** | Full cross-subsystem: session→render→export, game science, Squirrel health, NestGate |

**43/44 (98%)** — up from 93%. See `docs/PRIMAL_GAPS.md` for the structured gap registry (8 open, zero critical).

## Live Integration Status (March 28, 2026)

| Primal | Eastgate | Pixel (ADB) | Notes |
|--------|----------|-------------|-------|
| BearDog | healthy v0.9.0 (Unix socket) | **BLOCKED** (no TCP mode) | Needs `--listen` for Android SELinux |
| Songbird | healthy v0.2.1 (Unix socket + mesh) | healthy v0.1.0 (TCP :9901) | `--listen` works on Android |
| NestGate | healthy (Unix socket, store/retrieve) | not deployed | Needs `--listen` for Android |
| Squirrel | alive v0.1.0 (abstract `@squirrel`) | not deployed | Abstract sockets TBD on Android |
| biomeOS | neural-api (39+ graphs, route.register) | **BLOCKED** (forces Unix socket) | `api --port` ignored |
| ToadStool | structural | not deployed | CLI-only, no server |

**Cross-gate federation**: Pixel Songbird registered on Eastgate biomeOS as
`gate: pixel8a` with capabilities [network, discovery, http, mesh, birdsong].
ADB forwards Pixel TCP 9901 → Eastgate 19901 for JSON-RPC IPC.

See `infra/wateringHole/handoffs/CROSS_GATE_MOBILE_TCP_TRANSPORT_GAP_HANDOFF_MAR28_2026.md`
for the full SELinux gap analysis and per-primal remediation plan.

## Docs

- `wateringHole/README.md` — Track structure and cross-spring context
- `wateringHole/PRIMALSPRING_COMPOSITION_GUIDANCE.md` — Composition guidance
- `wateringHole/handoffs/` — Active + archived evolution handoffs
- `specs/CROSS_SPRING_EVOLUTION.md` — Evolution path (Phase 0–25+ done — includes live validation matrix + 6 GAP-MATRIX items)
- `specs/NUCLEUS_VALIDATION_MATRIX.md` — NUCLEUS capability validation matrix with particle model
- `specs/MIXED_COMPOSITION_PATTERNS.md` — Particle model, layered validation (L0-L3), gap inventory, spring specialization guide
- `specs/TOWER_STABILITY.md` — 87-gate acceptance criteria and progression
- `specs/CAPABILITY_ROUTING_TRACE.md` — Hardcoded → semantic routing evolution (incl. gen4 categories 8–11)
- `specs/GEN4_COMPOSITION_AUDIT.md` — Shortcomings audit: primalSpring vs esotericWebb gen4 needs
- `specs/PAPER_REVIEW_QUEUE.md` — Coordination patterns ready for validation
- `specs/BARRACUDA_REQUIREMENTS.md` — barraCuda relationship (indirect only)
- `specs/AGENTIC_TRIO_EVOLUTION.md` — biomeOS + Squirrel + petalTongue evolution guidance for the agentic loop
- `specs/STORYTELLING_EVOLUTION.md` — ludoSpring + esotericWebb evolution for AI DM storytelling
- `specs/SHOWCASE_MINING_REPORT.md` — Showcase patterns mined from primals for substrate validation
- `config/deployment_matrix.toml` — 43-cell deployment validation matrix
- `whitePaper/baseCamp/README.md` — baseCamp paper pointer

## Deployment Scripts

| Script | Purpose |
|--------|---------|
| `scripts/validate_deployment_matrix.sh` | Run deployment matrix cells: topology × arch × preset × transport validation |
| `scripts/chaos-inject.sh` | Inject chaos conditions (partition, kill, disk-fill, slow DNS, clock drift) into benchScale labs |
| `scripts/validate_local_lab.sh` | Quick local lab validation for benchScale topologies |
| `scripts/build_ecosystem_musl.sh` | Build all primals as `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` static binaries |
| `scripts/prepare_spore_payload.sh` | Assemble USB spore deployment payload (binaries + graphs + scripts + genetics) |
| `scripts/validate_remote_gate.sh` | Probe a remote gate's NUCLEUS health via TCP JSON-RPC |
| `scripts/validate_release.sh` | Release quality gate: fmt + clippy + deny + test floor + docs |

---

**License**: AGPL-3.0-or-later
