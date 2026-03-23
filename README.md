# primalSpring

**Coordination and composition validation spring for the ecoPrimals ecosystem.**

| | |
|-|-|
| **Domain** | Primal coordination, atomic composition, graph execution, emergent systems, multi-node bonding + federation |
| **Version** | 0.7.0 |
| **Edition** | Rust 2024 (1.87+) |
| **License** | AGPL-3.0-or-later |
| **Tests** | 360 (unit + integration + doc-tests + proptest) |
| **Experiments** | 53 (10 tracks) |
| **Deploy Graphs** | 22 TOMLs (18 single-node + 4 multi-node) |
| **Compositions** | Tower + Nest + Node + NUCLEUS + Graph Overlays + Squirrel Discovery + Graph Execution + Provenance Trio + Multi-Node Bonding (87/87 gates) |
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
│   │   ├── ipc/                   # JSON-RPC 2.0 client, discovery, error, dispatch, extract, resilience, transport, probes, proptest
│   │   ├── launcher/              # Primal binary discovery, spawn, socket nucleation (sync biomeOS port)
│   │   ├── harness/               # Atomic test orchestration: spawn compositions, validate, RAII teardown
│   │   ├── niche.rs               # BYOB niche self-knowledge (capabilities, semantic mappings, registration)
│   │   ├── primal_names.rs        # Canonical display names ↔ discovery slugs (neuralSpring pattern)
│   │   ├── validation/            # Experiment harness (check_bool, check_skip, check_relative, OrExit, ValidationSink, NdjsonSink, skip-aware exit)
│   │   └── tolerances/            # Named latency and throughput bounds
│   ├── src/bin/
│   │   ├── primalspring_primal/   # UniBin: JSON-RPC 2.0 server with niche registration
│   │   └── validate_all/          # Meta-validator: runs all 51 experiments
│   └── tests/
│       ├── integration/           # Shared test helpers (guards, spawn, RPC)
│       ├── server_integration.rs  # 10 core auto tests
│       ├── server_ecosystem.rs    # Tower-related live tests (#[ignore])
│       └── server_ecosystem_compose.rs  # Nest/Node/Overlay/Squirrel live tests (#[ignore])
├── experiments/                   # 51 validation experiments (9 tracks)
├── config/                        # Launch profiles (primal_launch_profiles.toml)
├── graphs/                        # 22 biomeOS deploy graph TOMLs (18 single-node + 4 multi-node)
│   └── multi_node/               # Multi-node federation graphs (HPC, friend, idle, data)
├── niches/                        # BYOB niche deployment YAML
├── specs/                         # Architecture specs
└── wateringHole/                  # Docs and handoffs
```

## Key Design Principles

- **Capability-first discovery**: No hardcoded primal rosters. Discovery is
  capability-based (via `discover_by_capability()`) or Neural API-driven
  (via `neural-api-client-sync`). Deploy graphs use `by_capability` for
  loose coupling — callers ask for capabilities, not primal identities.
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

# Run all 51 experiments (meta-validator)
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

## Deploy Graphs

primalSpring ships 22 biomeOS deploy graph TOMLs (all nodes declare `by_capability`):

**Single-node graphs (18)**:

| Graph | Pattern | Primals |
|-------|---------|---------|
| `tower_atomic_bootstrap.toml` | Sequential | beardog, songbird |
| `tower_full_capability.toml` | Sequential | beardog, songbird (full caps) |
| `nest_deploy.toml` | Sequential | beardog, songbird, nestgate |
| `node_atomic_compute.toml` | Sequential | beardog, songbird, toadstool |
| `nucleus_complete.toml` | Sequential | beardog, songbird, nestgate, toadstool, sweetgrass |
| `tower_ai.toml` | Sequential | beardog, songbird, squirrel |
| `tower_ai_viz.toml` | Sequential | beardog, songbird, squirrel, petaltongue |
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

**Multi-node federation graphs (4)** — `graphs/multi_node/`:

| Graph | Scenario | Bond Type | Trust Model |
|-------|----------|-----------|-------------|
| `basement_hpc_covalent.toml` | LAN HPC mesh | Covalent | GeneticLineage |
| `friend_remote_covalent.toml` | Remote friend + NAT traversal | Covalent | GeneticLineage |
| `idle_compute_federation.toml` | Federated idle compute sharing | Covalent | GeneticLineage |
| `data_federation_cross_site.toml` | NestGate cross-site replication | Covalent | GeneticLineage |

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

## Discovery (5-Tier)

Discovery walks 5 tiers in priority order:
1. `{PRIMAL}_SOCKET` env override
2. `$XDG_RUNTIME_DIR/biomeos/{primal}-{family}.sock` (XDG convention)
3. `{temp_dir}/biomeos/{primal}-{family}.sock` (fallback)
4. Primal manifest: `$XDG_RUNTIME_DIR/ecoPrimals/manifests/{primal}.json`
5. Socket registry: `$XDG_RUNTIME_DIR/biomeos/socket-registry.json`

Tiers 4–5 absorbed from biomeOS v2.50 and Squirrel alpha.12.

## MCP Tools

primalSpring exposes 8 typed MCP tools via `mcp.tools.list` for Squirrel AI
coordination tool discovery. Each tool has a JSON Schema input definition.

## Live Atomic Harness

primalSpring absorbs primal coordination from biomeOS — binary discovery,
socket nucleation, process spawning, and wave-based startup — ported to
pure synchronous Rust (`std::process` + `std::thread`, no tokio).

| Module | Responsibility |
|--------|---------------|
| `launcher/` | `discover_binary()`, `spawn_primal()`, `spawn_neural_api()`, `wait_for_socket()`, `SocketNucleation`, `LaunchProfile`, `LaunchError` (incl. `HealthCheckFailed`) |
| `harness/` | `AtomicHarness::new()` / `::with_graph()`, `.start()` (topological waves), `.start_with_neural_api()`, `RunningAtomic` (capability-based `socket_for` / `client_for`, RAII lifecycle, NeuralBridge) |

Set `ECOPRIMALS_PLASMID_BIN` to point at `ecoPrimals/plasmidBin/` to enable
live primal spawning. Without it, experiments fall back to discovering
whatever is already running.

## Docs

- `wateringHole/README.md` — Track structure and cross-spring context
- `wateringHole/PRIMALSPRING_COMPOSITION_GUIDANCE.md` — Composition guidance
- `wateringHole/handoffs/` — Active + archived evolution handoffs
- `specs/CROSS_SPRING_EVOLUTION.md` — Evolution path (Phase 0–12, future 13–19)
- `specs/TOWER_STABILITY.md` — 87-gate acceptance criteria and progression
- `specs/PAPER_REVIEW_QUEUE.md` — Coordination patterns ready for validation
- `specs/CAPABILITY_ROUTING_TRACE.md` — Hardcoded → semantic routing evolution
- `specs/BARRACUDA_REQUIREMENTS.md` — barraCuda relationship (indirect only)
- `whitePaper/baseCamp/README.md` — baseCamp paper pointer

## Deployment Scripts

| Script | Purpose |
|--------|---------|
| `scripts/build_ecosystem_musl.sh` | Build all primals as `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` static binaries |
| `scripts/prepare_spore_payload.sh` | Assemble USB spore deployment payload (binaries + graphs + scripts + genetics) |
| `scripts/validate_remote_gate.sh` | Probe a remote gate's NUCLEUS health via TCP JSON-RPC |
| `scripts/validate_release.sh` | Release quality gate: fmt + clippy + deny + test floor + docs |

---

**License**: AGPL-3.0-or-later
