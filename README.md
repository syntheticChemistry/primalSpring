# primalSpring

[![CI](https://github.com/syntheticChemistry/primalSpring/actions/workflows/ci.yml/badge.svg)](https://github.com/syntheticChemistry/primalSpring/actions/workflows/ci.yml)
[![License: AGPL-3.0](https://img.shields.io/badge/license-AGPL--3.0-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Tests](https://img.shields.io/badge/tests-1198_pass-brightgreen.svg)](#validation-scenarios-166-across-12-tracks)
[![Rust](https://img.shields.io/badge/rust-1.87%2B-orange.svg)](https://www.rust-lang.org)
[![Unsafe: forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance)

**NUCLEUS evolution arena — validates primal compositions in the ecoPrimals ecosystem.**

| | |
|-|-|
| **Domain** | Primal experimentation — atomic composition, graph execution, emergent systems, bonding models, mesh behavior |
| **Version** | 0.9.33 |
| **Edition** | Rust 2024 (1.87+) |
| **License** | AGPL-3.0-or-later |
| **Tests** | 1133 lib + 17 doc + experiment (1170 workspace total) |
| **Experiments** | 96 (21 tracks) — 166 validation scenarios (12 tracks, 3 tiers) |
| **Deploy Graphs** | 102 graph TOMLs (16 directories) — fragment-first with `resolve = true` |
| **Coverage** | Method coverage against 492+ registered capability methods; line coverage via llvm-cov |
| **Compositions** | Tower + Nest + Node + NUCLEUS + Graph Overlays + Squirrel Discovery + Graph Execution + Provenance Trio + Multi-Node Bonding + biomeOS Substrate + Cross-Gate + Deployment Matrix + Substrate Stress + Pure Composition (ludoSpring + esotericWebb as graph-defined products) + **7 Decomposed Subsystems (C1-C7)** + **Mixed Atomics (L2) + Bonding Patterns (L3)** (87/87 gates). **exp091 12/12 routing, exp094 19/19 parity, exp096 14/15 cross-arch** (HSM cfg-gated) |
| **Mesh** | 5-node sovereign relay (golgi ↔ sporeGate ↔ eastGate ↔ flockGate ↔ ironGate). WG overlay 10.13.37.0/24. TOML-driven topology (`config/mesh_topology.toml`). K-Derm cytoplasm zones: backbone/house2/garage/wan. Three-hub triangle topology. Live overlay validation (s_mesh_overlay). |
| **Subsystems** | C1: Render (petalTongue) + C2: Narration (Squirrel) + C3: Session (esotericWebb) + C4: Game Science (ludoSpring) + C5: Persistence (NestGate) + C6: Proprioception (petalTongue) + C7: Full Interactive |
| **Provenance** | All 96 experiments carry structured `with_provenance()` metadata |
| **Clippy** | 0 warnings — pedantic + nursery clean (`cargo clippy --all-targets`). `#![warn(missing_docs)]` enforced. |
| **guideStone** | Level 8 — **live NUCLEUS** (certification engine absorbed as UniBin organelle) (13/13 BTSP authenticated), 41/41 bare, P3 CHECKSUMS (BLAKE3), seed provenance (Layer 0.5), BTSP default everywhere (Layer 1.5), cellular deployment (Layer 7, 9 cells BTSP-enforced), **46 cross-arch binaries (6 targets, Tier 1 39/39)**, **provenance-elevated checksums** (Layer 2: composite fingerprint + sweetGrass braids) |
| **Unsafe** | Workspace-level `deny` via `[workspace.lints.rust]`, `#![forbid(unsafe_code)]` on all 88 crate roots — zero unsafe blocks |
| **C deps** | Zero in default build (ecoBin compliant, `deny.toml` enforced). `ureq` (HTTPS) feature-gated behind `cross-membrane`. |
| **Runtime deps** | 16 (was 17; `hostname` eliminated Wave 54b). Pure Rust crypto stack for BTSP bootstrap. |

---

## What Is primalSpring?

primalSpring is the **NUCLEUS evolution arena**. Where hotSpring validates
physics and wetSpring validates biology, primalSpring validates how primals
compose — the atomic patterns, bonding models, mesh behaviors, and emergent
properties that arise when primals work together.

**primalSpring is NOT a primal.** It does not serve on a socket, does not
register with biomeOS, and does not appear in NUCLEUS compositions. It is
a pure CLI tool + IPC client that validates compositions from the outside.

Its "papers" are the atomics. Its "experiments" are composition patterns.
Its validation targets are the compositions themselves.

Deployment pipelines, VPS ops, and upstream team coordination have been
handed off to cellMembrane and the wateringHole overwatch (Wave 82c).
primalSpring focuses on what springs do: run experiments. Validated
patterns flow downstream to **cellMembrane** (binary evolution + VPS ops)
and **projectNUCLEUS** (polished agnostic deployment product).

## Architecture

```
primalSpring/
├── ecoPrimal/                     # Library crate + UniBin binaries
│   ├── src/
│   │   ├── lib.rs                 # Library root (unsafe_code = deny)
│   │   ├── cast.rs                # Safe numeric casts (saturating boundary)
│   │   ├── coordination/          # Atomic composition, health probing, composition validation
│   │   ├── deploy/                # Deploy graph parsing, structural + live validation
│   │   ├── graphs/                # Graph execution pattern types (5 patterns)
│   │   ├── emergent/              # Emergent system validation (RootPulse, RPGPT, CoralForge)
│   │   ├── bonding/               # Multi-gate bonding models (Covalent, Metallic, Ionic, Weak, OMS) + graph metadata + STUN tiers
│   │   ├── ipc/                   # JSON-RPC 2.0 client, discovery, capability, error, dispatch, extract, resilience, transport, tcp, methods, provenance, proptest, btsp_handshake
│   │   ├── launcher/              # Primal binary discovery, spawn, profiles, socket nucleation (sync biomeOS port)
│   │   ├── harness/               # Atomic test orchestration: spawn compositions, validate, RAII teardown
│   │   ├── primal_names.rs        # Canonical slug constants, display names ↔ discovery slugs (neuralSpring pattern)
│   │   ├── validation/            # Experiment harness (check_bool, check_skip, check_relative, OrExit, ValidationSink, NdjsonSink, builder .run())
│   │   ├── validation/helpers.rs  # Shared validation helpers (graph parsing, Dark Forest, capability cross-ref)
│   │   ├── validation/scenarios/  # 122 validation scenarios (12 tracks, 3 tiers: Rust/Live/Both)
│   │   ├── tolerances/            # Named latency and throughput bounds
│   │   ├── certification/         # Certification engine (absorbed guidestone, L0-L8)
│   ├── src/bin/
│   │   ├── primalspring/          # UniBin: certify + validate + status + checksums + registry + version
│   │   └── nucleus_launcher/   # Rust NUCLEUS lifecycle (start/stop/status + federation)
│   └── tests/
│       ├── integration/           # Shared test helpers (guards, spawn, RPC)
│       ├── server_integration.rs  # 10 core auto tests
│       ├── server_ecosystem.rs    # Tower atomic + Squirrel AI (#[ignore])
│       ├── server_ecosystem_songbird.rs  # Songbird IPC surface (#[ignore])
│       ├── server_ecosystem_genetics.rs  # Three-tier genetics (#[ignore])
│       ├── server_ecosystem_compose.rs   # Nest/Node composition (#[ignore])
│       └── server_ecosystem_overlay.rs   # Graph-driven overlays (#[ignore])
├── experiments/                   # 93 validation experiments (21 tracks)
├── config/                        # Launch profiles, capability registry, composition tools
├── graphs/                        # 82 deploy graph TOMLs + 32 atomic composition graphs
│   ├── compositions/             # 32 atomic composition graphs (tower/node/nest/meta/rootpulse/ecosystem/impulse/sync)
│   ├── fragments/                # 6 atomic building blocks (tower, node, nest, nucleus, meta, provenance)
│   ├── profiles/                 # 9 thin compositions (fragment refs + delta nodes, resolve = true)
│   ├── patterns/                 # 4 coordination patterns: parallel, conditional, streaming, continuous
│   ├── bonding/                  # 5 bonding model graphs: ionic, metallic, OMS, defensive, albatross
│   ├── chaos/                    # 2 chaos engineering: partition recovery, slow start
│   ├── multi_node/               # 5 multi-node federation graphs
│   ├── spring_validation/        # 9 files: template + manifest + nucleus_atomics + crypto_negative + gaming_niche + domain_contract_sweep + content_pipeline_smoke + compute_trio_smoke + deploy_pipeline_smoke
│   ├── spring_deploy/            # 2 files: template + manifest (5 springs parameterized)
│   ├── downstream/               # 3 TOML + 2 docs: template + manifest + healthspring_enclave
│   ├── cross_spring/             # 2 cross-spring validators
│   └── federation/               # 1 content distribution
├── docs/                          # Structured gap registry and subsystem documentation
│   └── PRIMAL_GAPS.md            # Per-primal gap inventory with severity and fix paths
├── notebooks/                     # Jupyter evidence notebooks (scaffolded, data gen pending)
├── benchScale/                    # Local mesh topology configs for live_mesh scenarios
├── niches/                        # BYOB niche deployment YAML
├── specs/                         # Architecture specs
├── wateringHole/                  # Fossilized — see infra/wateringHole/
└── fossilRecord/                  # Pointer to external fossilRecord repo
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

# Primal binaries come from plasmidBin (auto-detected from infra/plasmidBin/)
# First-time bootstrap (if git checkout unavailable):
membrane plasmid.fetch

# Run live atomic tests (requires plasmidBin binaries)
cargo test --ignored

# Run scenario validation (UniBin)
cargo run --release --bin primalspring -- validate

# Certify ecosystem composition
cargo run --release --bin primalspring -- certify

# === UniBin (eukaryotic) ===
# Run certification (absorbed guidestone)
cargo run --bin primalspring -- certify

# Run bare certification (no primals needed)
cargo run --bin primalspring -- certify --bare

# Run all validation scenarios (absorbed experiments)
cargo run --bin primalspring -- validate

# List available scenarios
cargo run --bin primalspring -- validate --list

# Filter by track or tier
cargo run --bin primalspring -- validate --track atomic-composition
cargo run --bin primalspring -- validate --tier rust
```

### Code coverage ([cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov))

Install the tool (`cargo install cargo-llvm-cov --locked` or a release binary) and `rustup component add llvm-tools-preview`, then:

```bash
cargo coverage
```

This runs LLVM source-based coverage for the whole workspace, skips paths matching `tests/` in the report. Release gate (`primalspring release`) enforces a **70%** floor. For HTML output, run `cargo llvm-cov --workspace --html` (see upstream docs for `--open`, `--lcov`, CI, etc.).

## Validation Scenarios (122 across 12 tracks)

primalSpring ships 122 validation scenarios organized into 12 tracks:

| Track | Scenarios | Tier |
|-------|-----------|------|
| atomic-composition | Tower, Node, Nest, NUCLEUS atomics | Rust/Live |
| mesh-topology | WireGuard overlay, zone topology, cross-gate calls | Live |
| bonding-models | Covalent, ionic, metallic bond validation | Rust |
| certification | GuideStone L0-L8, BTSP enforcement | Rust/Live |
| deployment | Deploy graph structural validation, pipeline depth | Rust |
| cross-primal | Direct socket probing (11 primals + 9 sub-caps) | Live |
| ai-pipeline | Squirrel AI + provenance tracking | Live |
| defense | SkunkBat threat detection, attestation | Rust/Live |
| sovereignty | Audit chain, genetics, ledger verification | Rust |
| convergence | Capability convergence, ecosystem drift | Rust |
| observatory | Parity, freshness, routing consistency | Rust |
| emergent | RootPulse, RPGPT, CoralForge, agentic tower | Rust |

## Validation Gates

primalSpring provides two Rust-native validation gates (Wave 82c, replacing
the former bash scripts):

```bash
# Release quality gate (fmt, clippy, deny, tests, docs, depot health)
cargo run --bin primalspring -- release

# NUCLEUS deployment validation (pre-flight, launch, health, federation)
cargo run --bin primalspring -- nucleus
cargo run --bin primalspring -- nucleus --full   # includes lifecycle (Tier 4)
```

Both gates output human-readable text by default and machine-readable JSON
with `--json`. The `release` gate calls `nucleus` automatically unless
`--skip-nucleus` is passed.

## Deploy Graphs

primalSpring ships 82 deploy graph TOMLs + 32 atomic composition graphs using fragment-first composition (all nodes declare `by_capability`):

**Root-level graphs (14)**:

| Graph | Pattern | Primals |
|-------|---------|---------|
| `tower_atomic_bootstrap.toml` | Sequential | beardog, songbird, skunkbat |
| `tower_ai.toml` | Sequential | beardog, songbird, squirrel |
| `tower_ai_viz.toml` | Sequential | beardog, songbird, squirrel, petaltongue |
| `node_atomic_compute.toml` | Sequential | beardog, songbird, toadstool |
| `node_ai.toml` | Sequential | beardog, songbird, toadstool, squirrel |
| `nest_viz.toml` | Sequential | beardog, songbird, nestgate, petaltongue |
| `nucleus_complete.toml` | Sequential | biomeos, beardog, songbird, skunkbat, nestgate, toadstool, squirrel (+compute trio +provenance trio) |
| `provenance_overlay.toml` | Sequential | beardog, songbird, rhizocrypt, loamspine, sweetgrass |
| `coralforge_pipeline.toml` | Pipeline | beardog, songbird, nestgate, toadstool, sweetgrass |
| `hotspring_qcd_pipeline.toml` | Pipeline | hotSpring QCD composition |
| `neuralspring_inference_pipeline.toml` | Pipeline | neuralSpring ML inference composition |
| `healthspring_clinical_pipeline.toml` | Pipeline | healthSpring clinical composition |
| `tower_agent.toml` | Sequential | beardog, songbird, skunkbat, squirrel (agentic Tower) |
| `spring_byob_template.toml` | Sequential | template for new springs |

**Multi-node federation graphs (5)** — `graphs/multi_node/`:

| Graph | Scenario | Bond Type | Trust Model |
|-------|----------|-----------|-------------|
| `basement_hpc_covalent.toml` | LAN HPC mesh | Covalent | GeneticLineage |
| `friend_remote_covalent.toml` | Remote friend + NAT traversal | Covalent | GeneticLineage |
| `idle_compute_federation.toml` | Federated idle compute sharing | Covalent | GeneticLineage |
| `data_federation_cross_site.toml` | NestGate cross-site replication | Covalent | GeneticLineage |
| `three_node_covalent_cross_network.toml` | 3-node cross-network mesh | Covalent | GeneticLineage |

**Spring validation graphs (9)** — `graphs/spring_validation/`: parameterized
`spring_validate_template.toml` + `spring_validate_manifest.toml` (9 compositions covering
all 7 springs plus products), `nucleus_atomics_validate.toml` (all 4 NUCLEUS tiers),
`crypto_negative_validate.toml` (negative security boundary tests),
`gaming_niche_validate.toml` (ludoSpring gaming niche validation),
`domain_contract_sweep.toml` (cross-domain method exercise),
`content_pipeline_smoke.toml` (NestGate content round-trip),
and `compute_trio_smoke.toml` (compute trio health + capabilities + math + sovereign dispatch).
All graphs include biomeOS Neural API as orchestration substrate.

**Composition graphs (1)** — `graphs/compositions/`: `foundation_validation.toml` —
full NUCLEUS composition for scientific validation via the foundation sediment pipeline
(12 nodes, 3 optional with `fallback = "skip"`). Purpose: `"foundation"`.

**Cross-spring graphs (2)** — `graphs/cross_spring/`: ecology validation
(ET₀ → diversity → spectral) and full sweep across all springs.

**Bonding model graphs (5)** — `graphs/bonding/`: ionic capability sharing,
metallic GPU pool, organo-metal-salt complex, defensive mesh (skunkBat),
Albatross multiplex (Songbird fleet).

**Chaos engineering graphs (2)** — `graphs/chaos/`: network partition recovery
and slow-start composition convergence.

**Spring deploy graphs (2)** — `graphs/spring_deploy/`: parameterized template +
manifest covering 5 per-spring science compositions. Each includes biomeOS Phase 0
substrate, Tower base, optional ToadStool for GPU-compute springs, and the spring
primal.

**Downstream graphs (3)** — `graphs/downstream/`: parameterized template + manifest
for 7 proto-nucleate compositions, plus `healthspring_enclave_proto_nucleate.toml`
(unique dual-tower ionic bridge pattern).

**Patterns (4)** — `graphs/patterns/`: streaming pipeline, continuous tick,
parallel capability burst, conditional fallback.

All graphs have `by_capability` on every node and are structurally validated +
topologically sorted at test time. Multi-node graphs include `[graph.metadata]`
and `[graph.bonding_policy]` sections validated by `graph_metadata.rs`.

## Atomic Composition Graphs

primalSpring ships 32 atomic composition graphs under `graphs/compositions/` that define
the Neural API composition layer. Each composition maps a high-level operation
(e.g. `tower.publish`, `nest.store`, `meta.deploy`) to a graph of primal
capabilities, enabling biomeOS to decompose semantic intent into concrete IPC calls.

Compositions are organized by tier:
- **Tower** (5): `publish`, `authenticate`, `discover`, `health`, `bootstrap`
- **Node** (1): `compute`
- **Nest** (4): `store`, `commit`, `retrieve`, `ingest_spore`
- **rootPulse** (5): `commit`, `branch`, `merge`, `diff`, `federate`
- **Meta** (5): `observe`, `intent`, `render`, `health`, `deploy`
- **Ecosystem** (3): `pull`, `push`, `check`
- **Impulse/Potential** (4): `impulse.post`, `impulse.ack`, `potential.sense`, `potential.check`
- **Agentic Sync** (3): `sync.diverge`, `sync.resolve`, `sync.resolve.crossgate`
- **Context** (2): `context.weave_anchored`, `impulse.post_signed`

The `tower.bootstrap` composition defines the two-phase cold-start sequence that
resolves the bootstrap paradox: Phase 1 (static, no biomeOS) brings up BearDog,
Songbird, and SkunkBat; Phase 2 (graph-driven) lets biomeOS discover and seed
the running Tower. See `infra/whitePaper/neuralAPI/02_ARCHITECTURE.md`.

Composition tool definitions for Squirrel AI consumption live in `config/composition_tools.toml`.

**Composition dispatch**: `CompositionContext::composition()` dispatches atomic compositions
via `signal.dispatch` (preferred, biomeOS wire method) with `capability.call` fallback for
older biomeOS versions. biomeOS v3.55–v3.57 intercepts `capability.call` transparently when
the target matches a composition tier, enabling seamless graph-backed execution.
`primal.announce` provides atomic self-registration (lifecycle + capabilities +
translations + composition-tier membership in a single RPC). Squirrel's `composition_plan`
mode decomposes natural-language intent into structured composition step sequences.
See `wateringHole/PRIMAL_ANNOUNCE_PROTOCOL.md` for the wire format.

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

**Post-primordial**: all primal binaries come from `plasmidBin` — the single
source of truth. The git checkout at `infra/plasmidBin/primals/` is auto-detected
by the launcher. Run `membrane plasmid.fetch` for first-time bootstrap if the
git checkout is unavailable. Override with `NUCLEUS_BIN_DIR`. Without binaries,
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

**gen4 prototypes** (archived to `fossilRecord/stale_graphs_apr12_2026/gen4/`):
sovereign tower (Dark Forest ready), science substrate (multi-spring pipeline),
agentic tower (AI-orchestrated), interactive substrate (full UI + AI + crypto +
mesh surface), **spring composition** (Tower + biomeOS + 5 spring primals +
cross-spring capability routing).

**Spring Gen4 Scaffolding** (Phase 19): 5 of 6 spring primal binaries built
and deployed to `plasmidBin/springs/` (groundspring, healthspring_primal,
ludospring, neuralspring, wetspring). All spring validation graphs updated to
deploy biomeOS as substrate. Launch profiles added for all 6 springs.

See `specs/CROSS_SPRING_EVOLUTION.md` for full evolution path.

## Phase 56: Desktop Substrate + The Rhizome (April 28, 2026)

- **8 new experiments** (exp099–exp106) across Track 18 — Desktop Substrate:
  - exp099: Agentic loop substrate (petalTongue ↔ biomeOS ↔ Squirrel feedback loop)
  - exp100: MCP ecosystem tools (Squirrel `tool.list` across springs)
  - exp101: fieldMouse AI triage (sensor → NestGate → Squirrel → petalTongue)
  - exp102: Storytelling session loop (esotericWebb → ludoSpring → Squirrel → petalTongue → trio)
  - exp103: ludoSpring expanded IPC (14 IPC methods esotericWebb needs)
  - exp104: RPGPT provenance replay (DAG → ledger → replay verification)
  - exp105: **The Rhizome micro-game** — full roguelike loop on NUCLEUS (Barracuda noise, game loop, save/load, provenance, narration)
  - exp106: **Micro-desktop shell** — desktop composition wrapping The Rhizome (biomeOS routing, 12/13 primal health bar, multi-session petalTongue, provenance sidebar)
- **4 desktop app deploy graphs**: `desktop_shell.toml`, `app_esotericwebb.toml`, `app_system_monitor.toml`, `app_rhizome.toml` — biomeOS-managed desktop applications with continuous coordination
- **`desktop_nucleus.sh`**: 13-primal NUCLEUS launcher with auto-symlink for petalTongue discovery, FAMILY_ID management, validate mode
- **Live gap harvesting**: 23 gaps documented in `fossilRecord/wateringHole_phase56_apr2026/LIVE_DEPLOYMENT_GAP_REPORT_PHASE56.md` — socket naming, capability routing, IPC parameter schemas, biomeOS graph parser inconsistencies
- **Provenance trio E2E fully resolved**: rhizoCrypt DAG + loamSpine ledger + sweetGrass attribution — all parameter schemas documented and corrected
- **The Rhizome**: Procedural roguelike game running on NUCLEUS — Barracuda Perlin noise for biome generation, deterministic floor layout, 5 biome types, creature/item systems, TOML save format, gap-tolerant validation

## BTSP, Inference Abstraction, and Proto-Nucleate Graphs (April 10, 2026)

- **Zero-Port Tower Atomic achieved**: 0 TCP ports across all 10+ primals. Pure Unix domain socket IPC with BTSP handshake authentication.
- **BTSP Phase 2 ecosystem cascade → Phase 3 full convergence (May 2, 2026)**: 13/13 capabilities BTSP-authenticated (was 5/13 pre-escalation). Phase 3 ChaCha20-Poly1305 AEAD achieved across all 13 primals — every primal ships `btsp.negotiate` returning `cipher: "chacha20-poly1305"` with HKDF-SHA256 key derivation and encrypted framing wire format. JSON-line BTSP auto-detection wired in all relay primals. Phase 45c resolved final upstream debt: Songbird `SecurityRpcClient::new_direct()` (Wave 169), ToadStool post-handshake connection persistence, loamSpine `btsp.negotiate` non-fatal fallback, petalTongue BearDog field alignment. Client-side `ipc::btsp_handshake` module with 15-second relay timeout. `upgrade_btsp_clients()` two-pass strategy (cleartext probe + BTSP-first for enforcing primals). All deploy graphs carry `secure_by_default = true` metadata.
- **Inference provider abstraction**: Vendor-agnostic `inference.complete`/`embed`/`models` wire standard in ecoPrimal. Squirrel bridge routes through `AiRouter` (Ollama as OpenAI-compatible HTTP endpoint). No vendor lock-in to CUDA or Ollama.
- **WGSL shader composition model**: ML inference, QCD physics, and biology are compositions of existing barraCuda WGSL shaders (826 kernels: matmul, attention, FFT, df64) compiled by coralReef and dispatched by toadStool.
- **5 proto-nucleate graphs** (`graphs/downstream/`): neuralSpring ML inference, hotSpring QCD (metallic GPU pool, df64, provenance), healthSpring dual-tower enclave (ionic bond, egress fence, clinical AI).
- **3 pipeline graphs**: neuralSpring inference pipeline, hotSpring QCD pipeline, healthSpring clinical pipeline — modeling end-to-end data flow through primal compositions.
- **13/13 critical experiments ALL PASS** — 93 total experiments across 21 tracks.
- **46 cross-architecture binaries** (6 target triples, Tier 1 39/39) — genomeBin v5.1, zero C dependencies.

## Fragment-First Graph Consolidation (April 16, 2026)

- **78 → 56 deploy graph TOMLs**: Eliminated isomorphic duplication through template+manifest parameterization and fragment-first `resolve = true` composition.
- **Template+manifest pattern**: Spring validation (13 → 4: template + manifest + 2 unique), spring deploy (5 → 2: template + manifest), downstream proto-nucleate (7 → 3: template + manifest + healthspring enclave).
- **Fragment resolution in `load_graph()`**: Profiles declaring `resolve = true` in `[graph.metadata]` inherit nodes from `graphs/fragments/*.toml` as a base layer, then apply only their delta nodes. Profiles trimmed from ~40 lines to ~15 lines each.
- **Removed**: `primalspring_deploy.toml` (absorbed into `nucleus_complete.toml`), `full_overlay.toml` (absorbed into `profiles/full.toml`), `fossilRecord/graphs/` stale snapshots, 9 per-spring validation wrappers, 5 per-spring deploy files, 7 individual proto-nucleate files.
- **Zero-regression**: All 631 tests at time of consolidation (585 passed + 46 ignored), 0 clippy warnings. Current: 887 lib tests pass (2 ignored) + 17 doc tests.

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
- **12 sketch graphs** (archived to `fossilRecord/stale_graphs_apr12_2026/sketches/`): L0 primal routing matrix, L2 dual-tower ionic / dedicated tower / nest enclave, L3 covalent mesh backup / ionic lease / organo-metal-salt.
- **3 new experiments** (exp091–093): L0 primal routing matrix, L2 dual-tower ionic structural, L3 covalent mesh backup structural.
- **Live validation on Eastgate**: Tower Atomic (BearDog + Songbird) fully validated — crypto, HTTPS, discovery all LIVE PASS. Neural API running, capability registration resolved (GAP-09 shipped in biomeOS v3.51 `method.register`). All 6 GAP-MATRIX items from Phase 56 are now resolved or superseded by eukaryotic architecture.

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

**57/57 (100%)** — all scenarios passing. See `docs/PRIMAL_GAPS.md` for the structured gap registry (13/13 zero debt, Waves 1–67 complete).

## Live Integration Status (June 28, 2026)

**13/13 primals ALIVE** on eastGate + flockGate (plasmidBin-only, post-primordial).
Zero debt, 13/13 BTSP Phase 3 FULL AEAD, 13/13 MethodGate adopted.
Zero-port Tower Atomic standard: UDS-only default, TCP opt-in via `PRIMALSPRING_TCP_TIER5=1`.
5-node WireGuard mesh (golgi ↔ sporeGate ↔ eastGate ↔ flockGate ↔ ironGate).
sporeGate role: ephemeral compute (hot-pluggable, no network-critical services).
Songbird TCP :7700 federation. Config-driven topology (`config/mesh_topology.toml`).
Validation scenarios derive mesh topology from SSOT (zero hardcoded IPs in production paths).

| Primal | Status | Notes |
|--------|--------|-------|
| BearDog | healthy (Unix socket + TCP) | `--listen` available, TCP unblocked since Wave 8 |
| Songbird | healthy (Unix socket + TCP + mesh) | Cross-gate federation operational |
| NestGate | healthy (Unix socket + TCP) | Content-addressed storage, 8 `content.*` methods |
| Squirrel | healthy (abstract socket) | AI inference dispatch |
| biomeOS | healthy (neural-api, 39+ graphs) | `api --port` supported |
| ToadStool | healthy (compute dispatch) | AMD live, NV FECS-gated |
| barraCuda | healthy (precision routing) | v0.4.0, TensorSession, 87 methods (Sprint 73) |
| coralReef | healthy (shader compilation) | Dual-vendor GPU (PTX + RDNA) |
| skunkBat | healthy (defense + audit) | JH-5 Phase 2 event instrumentation |
| rhizoCrypt | healthy (DAG provenance) | `dag.session.get` enriched |
| loamSpine | healthy (Merkle ledger) | Zero debt |
| sweetGrass | healthy (attribution braids) | JH-0 gate + port 9850 canonical |
| petalTongue | healthy (viz + rendering) | `#[expect(reason)]` complete |

See [fossilRecord](https://github.com/ecoPrimals/fossilRecord) → `springs/primalSpring/wateringHole_phase58_59_may2026/` for historical cross-gate analysis.

## Docs

- `docs/PRIMAL_GAPS.md` — Structured gap registry (Waves 1–55b, 13/13 zero debt)
- `docs/CROSS_SPRING_PARITY_SCORECARD.md` — Cross-spring parity scorecard
- `docs/DISCOVERY_WIRE_CONTRACT.md` — Songbird + biomeOS discovery wire format
- `infra/wateringHole/PRIMAL_ANNOUNCE_PROTOCOL.md` — `primal.announce` wire format (canonical, external)
- `infra/wateringHole/CRYPTO_CONSUMPTION_HIERARCHY.md` — Crypto posture per primal role (canonical, external)
- `infra/wateringHole/METHOD_GATE_STANDARD.md` — MethodGate authorization standard (canonical, external)
- `specs/CROSS_SPRING_EVOLUTION.md` — Evolution path (Phase 0–25+ done — live validation matrix, GAP-MATRIX items all resolved)
- `specs/NUCLEUS_VALIDATION_MATRIX.md` — NUCLEUS capability validation matrix with particle model
- `specs/MIXED_COMPOSITION_PATTERNS.md` — Particle model, layered validation (L0-L3), gap inventory, spring specialization guide
- `specs/archive/TOWER_STABILITY.md` — 87-gate acceptance criteria and progression (FOSSIL RECORD — archived Wave 49)
- `specs/archive/CAPABILITY_ROUTING_TRACE.md` — Hardcoded → semantic routing evolution (FOSSIL RECORD — archived Wave 49)
- ~~`specs/GEN4_COMPOSITION_AUDIT.md`~~ — fossilized to `fossilRecord/springs/primalSpring/docs_wave35_may2026/`
- `specs/archive/PAPER_REVIEW_QUEUE.md` — Coordination patterns (archived Wave 49)
- `specs/archive/BARRACUDA_REQUIREMENTS.md` — barraCuda relationship (archived Wave 49)
- `specs/AGENTIC_TRIO_EVOLUTION.md` — biomeOS + Squirrel + petalTongue evolution guidance for the agentic loop
- `specs/STORYTELLING_EVOLUTION.md` — ludoSpring + esotericWebb evolution for AI DM storytelling
- `specs/RHIZOME_MICRO_GAME.md` — The Rhizome roguelike game spec: world gen, biomes, tile rendering, save format, primal usage
- `specs/MICRO_DESKTOP_COMPOSITION.md` — Micro-desktop shell: layout, session model, provenance sidebar
- `specs/DESKTOP_NUCLEUS_DEPLOYMENT.md` — Desktop NUCLEUS deployment spec
- `specs/DESKTOP_SESSION_MODEL.md` — Desktop session model (petalTongue + biomeOS)
- `specs/LIVE_GUI_COMPOSITION_PATTERN.md` — Live GUI composition patterns
- ~~`specs/SHOWCASE_MINING_REPORT.md`~~ — fossilized to `fossilRecord/springs/primalSpring/docs_wave35_may2026/`
- `whitePaper/baseCamp/README.md` — baseCamp paper pointer

## Scripts and Tools

**`scripts/`** (5 lab scripts) — experimentation execution layer:

| Script | Purpose |
|--------|---------|
| `scripts/validate_composition.sh` | Validate NUCLEUS composition health and binary presence |
| `scripts/validate_local_lab.sh` | Quick local lab validation for benchScale topologies |
| `scripts/chaos-inject.sh` | Inject chaos conditions (partition, kill, disk-fill, slow DNS, clock drift) |
| `scripts/pixel_cross_arch_lab.sh` | Cross-arch validation lab for Pixel/Android targets |
| `scripts/lan_covalent_lab.sh` | LAN covalent bonding lab for multi-node federation |

Former CI/validation scripts (`validate_release.sh`, `validate_nucleus_deployment.sh`,
`validate_deployment_matrix.sh`, etc.) have been replaced by the `primalspring release`
and `primalspring nucleus` Rust subcommands (Wave 82c deep debt sprint).

**`tools/`** — removed (all tooling absorbed into Rust as of Wave 120).

All tools (25+ shell scripts, GDScript, Python) were deleted or fossilized to
`fossilRecord/` during deep debt sprints (Waves 82c, 120). Shell composition library,
NUCLEUS launchers, method audit tools, and desktop launchers are all absorbed into
idiomatic Rust (`nucleus_launcher`, `primalspring` subcommands).

---

**License**: AGPL-3.0-or-later
