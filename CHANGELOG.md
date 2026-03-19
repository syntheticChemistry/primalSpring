# Changelog

All notable changes to primalSpring are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased] — v0.3.0

### Added
- **Live Atomic Harness** — absorbed primal coordination from biomeOS, ported to pure
  synchronous Rust (no tokio). New modules:
  - `launcher/` — `discover_binary()` (5-tier search, 6 binary patterns), `spawn_primal()`,
    `wait_for_socket()`, `SocketNucleation` (deterministic socket assignment), `LaunchProfile`
    (data-driven TOML config), `PrimalProcess` (RAII child lifecycle), `LaunchError` (typed errors)
  - `harness/` — `AtomicHarness::start()` (topological wave startup), `RunningAtomic`
    (health checks, capability queries, validation, RAII teardown)
- `config/primal_launch_profiles.toml` — per-primal socket-passing conventions
- 3 live atomic integration tests (`tower_atomic_live_*`, `#[ignore]` — require plasmidBin)
- exp001 evolved to optionally spawn live primals via `AtomicHarness` when
  `ECOPRIMALS_PLASMID_BIN` is set
- Harvested stable binaries to `ecoPrimals/plasmidBin/primals/` (beardog, songbird,
  nestgate, toadstool, squirrel)
- 248 tests total (233 unit + 13 integration + 2 doc-tests), 3 ignored (live)
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
- 251 tests total (233 unit + 16 integration + 2 doc-tests), 6 ignored (live atomic + neural)

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
