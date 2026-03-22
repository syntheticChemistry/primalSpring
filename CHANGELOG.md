# Changelog

All notable changes to primalSpring are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.6.0] — 2026-03-22

### Added
- **NUCLEUS Composition VALIDATED** — all 58/58 gates pass across Tower + Nest + Node
- **Nest Atomic** — nestgate storage primal integrated: socket-only mode (no ZFS required),
  storage.store/retrieve round-trip, model.register/locate, discover_capabilities
- **Node Atomic** — toadstool compute primal integrated: dual-protocol socket (tarpc + JSON-RPC),
  toadstool.health, toadstool.query_capabilities (4 workload types, 24 CPU cores)
- **exp066_nest_atomic** — Nest Atomic storage validation, 13/13 PASS
- **exp067_node_atomic** — Node Atomic compute validation, 13/13 PASS
- **exp068_full_nucleus** — all 3 atomic layers composing together, 16/16 PASS
- 12 new integration tests (8 Nest + 4 Node), all passing in parallel with Tower tests
- `subcommand` field in `LaunchProfile` to override default `"server"` subcommand
- `jsonrpc_socket_suffix` field in `LaunchProfile` for dual-protocol primals (toadstool)
- `SocketNucleation::remap()` for post-spawn socket path remapping
- Health liveness fallback chain: `health.liveness` → `health.check` → `health` → `{primal}.health`

### Fixed
- **NestGate ZFS hard-fail** — nestgate now degrades to filesystem mode when ZFS kernel module
  is not loaded (was: crash on startup). Fixed in `StorageState::new()` fallback to dev config
- **NestGate `socket_only` pattern match** — fixed pre-existing compile error in `cli.rs`
  where `Commands::Daemon` destructure was missing `socket_only` field
- **ToadStool socket discovery** — toadstool ignores `--socket` CLI flag, uses `TOADSTOOL_SOCKET`
  env var. Harness now passes socket via env and waits for `.jsonrpc.sock` suffix file

## [0.5.0] — 2026-03-21

### Added
- **Tower Full Utilization VALIDATED** — all 41/41 gates pass (24 core + 17 full utilization)
- **exp062_tower_subsystem_sweep** — probes all songbird JSON-RPC subsystems (Tor, STUN,
  BirdSong, Onion, Federation, Discovery), reports 11/12 UP (tor.connect expected DOWN)
- **exp063_pixel_tower_rendezvous** — BirdSong beacon encrypt/decrypt round-trip, sovereign
  onion service, STUN public address — ALL PASS
- **exp064_nestgate_internet_reach** — STUN, Onion, Tor internet paths validated (3/5 available)
- **exp065_petaltongue_tower_dashboard** — petalTongue headless server, dashboard render,
  Grammar of Graphics expression render — ALL PASS
- 6 new songbird subsystem integration tests, all passing in parallel
- `graphs/tower_full_capability.toml` — complete Tower deploy graph
- petalTongue v1.6.6 harvested to `plasmidBin/primals/petaltongue`
- `[profiles.petaltongue]` launch profile (headless server mode)
- `extra_args` field in `LaunchProfile` for verbatim CLI arguments
- 12 new capabilities in registry + federation translations in biomeOS

### Fixed
- **Songbird port contention** — added `--port 0` (ephemeral OS-assigned) support in songbird
  config validation and `bind_with_fallback`. All 19 integration tests now run in parallel (~1s)
  instead of requiring sequential execution (~30s)
- **BirdSong beacon API** — fixed `node_id` parameter requirement and `encrypted_beacon`
  field name for decrypt round-trip
- **petalTongue IPC** — use `PETALTONGUE_SOCKET` env var (not `--socket` CLI flag) for socket path
- **Grammar of Graphics** — corrected enum casing (`Cartesian`, `Bar`, `X`/`Y`)
- **Socket path length** — shortened experiment family IDs to prevent `SUN_LEN` overflow

### Changed
- 44 experiments, 270 tests total — all passing
- `TOWER_STABILITY.md` gates 7-11: PENDING → PASS (all validated live)

## [0.4.0] — 2026-03-21

### Added
- **Tower Stability Sprint** — all 24 Tower Atomic gates now pass (was 15/24)
- **Squirrel AI Composition** — full Tower + Squirrel composition (beardog + songbird + squirrel)
  with AI inference via Anthropic Claude routed through Neural API capability system
- **exp060_biomeos_tower_deploy** — biomeOS-orchestrated Tower deployment via `neural-api-server`
  and `tower_atomic_bootstrap.toml` graph (validates graph-driven germination)
- **exp061_squirrel_ai_composition** — 3-primal composition (Tower + Squirrel) with live
  AI `ai.query` calls, API key passthrough from `testing-secrets/api-keys.toml`, and
  post-query Tower health validation
- 7 new integration tests: `tower_zombie_check` (Gate 1.5), `tower_discovery_peer_list`
  (Gate 3.5), `tower_tls_handshake` (Gate 4.1), `tower_tls_internet_reach` (Gate 4.2),
  `tower_tls_routing_audit` (Gate 4.3), `tower_squirrel_ai_query`, `tower_squirrel_composition_health`
- `PrimalProcess::from_parts()` — construct from pre-spawned components (custom spawn logic)
- `RunningAtomic::pids()` — collect all child PIDs for lifecycle assertions
- `LaunchProfile::passthrough_env` — forward parent env vars to child processes
- `ai.query`, `ai.health`, `composition.tower_squirrel_health` — new capabilities in registry
- 40 experiments (38 → 40), 264 tests total (239 unit + 23 integration + 2 doc-tests)
- Rebuilt Squirrel from source and harvested to `plasmidBin/primals/squirrel`

### Changed (cross-primal, executed by primalSpring team)
- **beardog** — 5-tier `biomeos/` socket discovery in `tower-atomic/discovery.rs` and
  `neural_registration.rs`; removed hardcoded `/tmp/beardog-default.sock` fallback
- **biomeOS** — enrollment uses `NeuralApiCapabilityCaller` (fallback to
  `DirectBeardogCaller` for bootstrap only); graph executor and federation use
  `capability.call` via Neural API; all `discover_beardog_socket()` /
  `discover_songbird_socket()` replaced with capability-based discovery
- **songbird** — new `songbird-crypto-provider` shared crate extracted from
  `songbird-http-client`; `tor-protocol`, `orchestrator`, `nfc`, `sovereign-onion`,
  and `quic` crates now route all crypto through Neural API; removed 7/8-tier
  identity-based socket discovery in favor of Neural API socket discovery
- Rebuilt and harvested updated beardog, songbird, and neural-api-server binaries
  to `plasmidBin/primals/`

### Fixed
- Unresolved doc link to `ValidationResult`
- `cargo fmt` formatting drift in 4 files
- Version drift (Cargo.toml 0.2.0 → 0.4.0 across all workspace members)
- `.gitignore` now excludes `audit.log` and `sqlite:/` test artifacts

## [Unreleased] — v0.3.0

### Added
- **Live Atomic Harness** — absorbed primal coordination from biomeOS, ported to pure
  synchronous Rust (no tokio). New modules:
  - `launcher/` — `discover_binary()` (5-tier search, 6 binary patterns), `spawn_primal()`,
    `wait_for_socket()`, `SocketNucleation` (deterministic socket assignment), `LaunchProfile`
    (data-driven TOML config), `PrimalProcess` (RAII child lifecycle), `LaunchError` (typed errors
    including `HealthCheckFailed`)
  - `harness/` — `AtomicHarness::new(atomic)` / `::with_graph(atomic, path)` constructors,
    `start(family_id)` with topological wave startup from deploy graphs, `RunningAtomic`
    (capability-based `socket_for(cap)` / `client_for(cap)`, health checks, validation, RAII teardown)
- `config/primal_launch_profiles.toml` — per-primal socket-passing conventions
- 6 live atomic integration tests (`tower_atomic_live_*` + `tower_neural_api_*`, `#[ignore]`)
- exp001 evolved to optionally spawn live primals via `AtomicHarness` when
  `ECOPRIMALS_PLASMID_BIN` is set
- Harvested stable binaries to `ecoPrimals/plasmidBin/primals/` (beardog, songbird,
  nestgate, toadstool, squirrel)
- 262 tests total (239 unit + 21 integration + 2 doc-tests), 11 ignored (live)
- **Capability-first architecture** — all RPC handlers, discovery, and experiments default
  to capability-based resolution; identity-based is retained as `mode: "identity"` fallback
- `topological_waves()` — Kahn's algorithm startup wave computation from deploy graph DAGs
- `graph_required_capabilities()` — graph-as-source-of-truth capability extraction
- `validate_live_by_capability()` — live validation using capability-first node probing
- `check_capability_health()` — capability-based analog of `check_primal_health()`
- `graph.waves` RPC endpoint — topological startup ordering from deploy graphs
- `graph.capabilities` RPC endpoint — required capabilities extracted from graph nodes
- `coordination.probe_capability` RPC endpoint — probe a single capability provider
- `coordination.validate_composition_by_capability` RPC endpoint
- `by_capability` on all 11 deploy graph TOML nodes (enforced by test)
- `IpcErrorPhase` and `PhasedIpcError` — phase-aware IPC error context
- `discover_remote_tools()` — spring tool discovery via `mcp.tools.list`
- `deny.toml` — ecoBin 14-crate C-dep ban (aligned with airSpring, wetSpring, groundSpring)
- `LICENSE` file — AGPL-3.0-or-later full text at repo root
- `CHANGELOG.md` — this file
- `ValidationResult::with_provenance()` — structured provenance metadata on validation results
- `ValidationResult::run_experiment()` / `print_banner()` — shared experiment boilerplate helpers
- MCP tool definitions — `mcp.tools.list` method for Squirrel AI coordination tool discovery
- `config/capability_registry.toml` — single source of truth for 21 niche capabilities
- Manifest discovery fallback — `$XDG_RUNTIME_DIR/ecoPrimals/manifests/*.json`
- Socket registry fallback — `$XDG_RUNTIME_DIR/biomeos/socket-registry.json`
- Resilience constants in `tolerances/` — circuit breaker, retry, cost-estimate named constants
- `JSONRPC_VERSION` constant — eliminates `"2.0"` string repetition
- Proptest IPC fuzz expansion — `extract_rpc_result`, `classify_response`, capability parsing
- 11 new deploy tests — topological waves, cycle detection, all-graphs-acyclic, by_capability enforcement
- `spawn_neural_api()` — dedicated Neural API server launcher (absolute path resolution, CWD with graphs)
- `AtomicHarness::start_with_neural_api()` — full Tower + Neural API startup, NeuralBridge access
- `RunningAtomic::neural_bridge()` — connect to live Neural API via harness
- 3 Neural API integration tests (`tower_neural_api_*`, `#[ignore]`)
- exp001 evolved: spawns Tower + Neural API, validates via NeuralBridge
- `AtomicHarness` refactored to struct with `new()` / `with_graph()` constructors
- `AtomicHarness::start()` uses `topological_waves()` for graph-driven startup ordering
- `RunningAtomic::socket_for(capability)` — capability-based socket lookup (security → beardog)
- `RunningAtomic::client_for(capability)` — capability-based client connection
- `LaunchError::HealthCheckFailed` — typed error for post-spawn health failures
- 262 tests total (239 unit + 21 integration + 2 doc-tests), 11 ignored (live atomic + neural + stability)

### Changed
- `handle_validate_composition` — defaults to capability-based validation
- `handle_discovery_sweep` — returns capabilities by default (mode=capability)
- `handle_deploy_atomic` — uses `validate_composition_by_capability()`
- `handle_bonding_test` — discovers by capability instead of primal roster
- `handle_nucleus_lifecycle` — emits `required_capabilities` instead of `required_primals`
- `print_status` — shows capability discovery status with provider names
- exp001–004 — evolved from identity-based to capability-based discovery
- exp006 — evolved from primal subset checks to `topological_waves()` from real graphs
- exp051 — evolved from `discover_for()` to `discover_capabilities_for()`
- `deploy::validate_live()` — `.expect()` replaced with proper `Result` propagation
- `coordination/mod.rs` — circuit breaker and retry parameters extracted to named constants
- `protocol.rs` — `"2.0"` literals replaced with `JSONRPC_VERSION`
- `niche.rs` — biomeOS registration target configurable via `BIOMEOS_PRIMAL` env var
- Formatting — `cargo fmt --all` applied (import ordering, line wrapping)

### Fixed
- TOCTOU panic in `validate_live()` when graph file mutates between parse calls

## [0.2.0] — 2026-03-18

### Added
- IPC resilience stack absorbed from 7 sibling springs
- `IpcError` (8 typed variants with query helpers)
- `CircuitBreaker` and `RetryPolicy` with `resilient_call()`
- `DispatchOutcome<T>` — three-way dispatch outcome model
- `extract_rpc_result<T>()` and `extract_rpc_dispatch<T>()`
- 4-format capability parsing (Formats A/B/C/D)
- `health.liveness` and `health.readiness` Kubernetes-style probes
- `safe_cast` module (absorbed from airSpring/healthSpring/groundSpring)
- `OrExit<T>` trait for zero-panic validation binaries
- `ValidationSink` trait with `StdoutSink` and `NullSink`
- `PRIMAL_NAME` and `PRIMAL_DOMAIN` constants
- FAMILY_ID-aware discovery
- Neural API health checks via `neural-api-client-sync`
- Proptest for IPC protocol fuzzing (5 property tests)
- 132 unit tests (up from 69), zero warnings
- All 38 experiments evolved with real probe patterns

### Changed
- Version 0.1.0 → 0.2.0

## [0.1.0] — 2026-03-17

### Added
- Neural API integration via `neural-api-client-sync` path dep
- `KNOWN_PRIMALS` removed — sovereignty fix
- Discovery evolved: composition-driven + Neural API
- Server mode: JSON-RPC 2.0 over Unix socket
- `probe_primal()`, `validate_composition()`, `health_check()`
- `check_or_skip()`, JSON output mode, `exit_code()`
- Workspace lints centralized
- 69 unit tests
- exp001 + exp004 IPC-wired with graceful degradation
- Zero warnings: check, clippy (pedantic+nursery), doc, fmt

## [0.0.1] — 2026-03-02

### Added
- Initial scaffolding — 38 experiments across 7 tracks
- Workspace compiles
- Coordination domain definition
