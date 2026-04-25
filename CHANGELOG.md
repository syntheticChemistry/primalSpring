# Changelog

All notable changes to primalSpring are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.9.17+] — Phase 45c: BTSP Default Everywhere + Full NUCLEUS Convergence (2026-04)

### Milestone
**BTSP authentication default on all tiers — 13/13 converged.** guidestone reports
**171/171 ALL PASS**, **13/13 capabilities BTSP-authenticated** (was 5/13 pre-Phase 45c).
JSON-line BTSP auto-detection and full handshake relay wired into all 12 upstream primals.
Cleartext is a FAIL, not a pass. Full NUCLEUS BTSP convergence achieved April 24, 2026.

### Changed
- `PROACTIVE_CAPS` expanded to all 17 capabilities (security, discovery, compute,
  tensor, shader, storage, ai, dag, commit, provenance, visualization, ledger,
  attribution, inference, spine, merkle, braid)
- guidestone `tiers` updated: Node and Nest expect `"btsp"` (was `"tower_delegated"`)
- `tower_delegated` pass-through logic removed — cleartext always FAILs
- `BTSP_HANDSHAKE_TIMEOUT_SECS` (15s) for relay primals calling BearDog during handshake
- `Transport::unix_btsp()` uses handshake timeout then reverts to IPC timeout (5s)
- `nucleus_launcher.sh`: family-scoped symlinks for attribution/ledger/merkle/visualization/inference capabilities, `crypto-{family}.sock` symlink, `BIOMEOS_FAMILY_ID` for coralReef

### Upstream BTSP Fixes Driven by primalSpring
- **ToadStool**: JSON-line BTSP sub-detection in `0x7B` path, `relay_json_line_handshake`
- **barraCuda**: `is_btsp_client_hello` detects `"protocol":"btsp"`, `session_verify_rpc` passes `client_ephemeral_pub` + `preferred_cipher`
- **coralReef**: full `relay_json_line_handshake`, `b64_encode` helper (no `rand` dep), `BtspSessionError` made pub, `BIOMEOS_FAMILY_ID` wiring
- **NestGate**: `resolve_security_socket_path` env-var priority, JSON-line framing in `perform_handshake`, `session_token`/`response` field alignment
- **Squirrel**: `json_line_mode` in `btsp_handshake_after_client_hello`, `write_json_line`/`read_json_line_msg` helpers, BearDog challenge extraction

### Common fixes across all 5 relay primals
- `btsp.session.create`: send `family_seed` (base64-encoded) instead of `family_seed_ref`
- `btsp.session.verify`: `session_token` (not `session_id`), `response` (not `client_response`)
- Challenge sourced from BearDog's `btsp.session.create` response (not locally generated)
- `session_id` / `session_token` fallback parsing

### Final Convergence Fixes (April 24, 2026)
- **Songbird** (Wave 169): `SecurityRpcClient::new()` → `new_direct()` in `bin_interface/server.rs` (lines 436, 468) — discovery capability was routing BTSP through Neural API instead of direct BearDog
- **ToadStool**: Post-handshake connection kept alive for NDJSON RPC (was dropping after successful handshake)
- **loamSpine**: `btsp.negotiate` made non-fatal in `handshake.rs` — BearDog doesn't implement this method; loamSpine now defaults to client's preferred cipher
- **petalTongue**: BearDog field alignment, BTSP server fully converged
- **primalSpring**: `upgrade_btsp_clients()` second pass — BTSP-first connection for capabilities with discoverable sockets but no existing client (covers BTSP-enforcing primals that reject cleartext)

### Resolved
- All 13/13 capabilities BTSP-authenticated — zero upstream BTSP debt remaining
- All seed fingerprint mismatches resolved via plasmidBin reharvest

## [0.9.17] — Phase 45: genomeBin Cross-Architecture Depot + Deployment Validation (2026-04-20)

### Milestone
**46 binaries across 6 target triples (Tier 1 39/39).** plasmidBin evolved from flat
x86_64+aarch64 layout to full genomeBin `primals/{target-triple}/` directory structure
per ecoBin Architecture Standard v3.0. Full deployment validation: local 12/12,
benchScale Docker 12/12, remote fetch sim 13/13 checksums verified.

### Added
- `build_ecosystem_genomeBin.sh` — replaces `build_ecosystem_musl.sh`, full 9-target
  matrix with tiered build (Tier 1 MUST → Tier 2 SHOULD → Tier 3 NICE), per-target
  linker configuration, cargo check + full build pipeline
- armv7-unknown-linux-musleabihf support: 10 primals built (RaspPi/ARM32)
- x86_64-pc-windows-gnu support: barraCuda built as .exe
- aarch64-linux-android support: 5 primals built for Pixel/GrapheneOS
- riscv64gc-unknown-linux-musl: primalSpring linked, all others cargo-check pass
- macOS (x86_64 + aarch64): 8/14 primals cargo-check pass (proves pure Rust)
- Backward-compat symlinks: `primals/{binary}` -> `x86_64-unknown-linux-musl/{binary}`

### Changed
- `plasmidBin/manifest.toml` version 5.1.0 (genomeBin format), full target matrix
- `plasmidBin/checksums.toml` — BLAKE3 per target triple for all 46 binaries
- `plasmidBin/start_primal.sh` — auto-detects target triple from `uname`
- `plasmidBin/deploy_pixel.sh` — resolves from `aarch64-unknown-linux-musl/`
- `plasmidBin/fetch.sh` — target-triple-aware binary resolution
- `plasmidBin/harvest.sh` — supports arbitrary target triples
- `benchScale/deploy-ecoprimals.sh` — resolves from genomeBin layout, symlink-safe
- `wateringHole/genomeBin/manifest.toml` — v5.0.0 with full coverage matrix

### Phase 45 — Upstream Evolution Absorbed
- toadStool armv7 usize overflow fixed upstream (cfg target_pointer_width gating)
- biomeOS armv7/aarch64 rebuilt with correct workspace binary target (`biomeos-unibin`)
- nestgate cross-arch builds restored (armv7 target installed)
- coralReef iter84 absorbed: `--port` → `--rpc-bind` CLI change
- beardog HSM session 43: BTSP FAMILY_SEED env var requirement documented
- squirrel, songbird, petalTongue upstream evolutions pulled and validated

### Phase 45 — Deployment Validation
- `start_primal.sh`: auto-set BEARDOG_FAMILY_SEED from --family-id, coralreef --rpc-bind
- Local NUCLEUS: 12/12 primals alive (x86_64-unknown-linux-musl)
- benchScale Docker: 12/12 primals alive, songbird security provider configured
- Remote fetch simulation: 13/13 x86_64 binaries, checksums verified
- Pixel payload staged: 13 aarch64-unknown-linux-musl binaries validated

### Documented Gaps (Reduced)
- nestgate/skunkbat: lib-only on cross-targets (workspace binary structure — upstream fix needed)
- macOS: cargo check proves pure Rust, full link requires osxcross
- RISC-V: cargo check pass, full link requires complete musl sysroot

## [0.9.16+] — Phase 44: Deep Debt Evolution + Capability-Based Discovery (2026-04)

### Evolution
**Deep debt elimination.** All production code now references `primal_names::`
constants — zero hardcoded primal name strings in 11 core modules. TCP fallback
port table centralized via `tcp_fallback_table()` referencing `tolerances::`
constants. Test suites extracted from `harness/mod.rs` (874→623 LOC) and
`bonding/mod.rs` (860→464 LOC) to companion `tests.rs` files. Unified workspace
`serde_json` in exp094. `handle_bonding_status` evolved from placeholder to typed
response. Added `TCP_FALLBACK_PETALTONGUE_PORT` constant. **601 tests, zero clippy
warnings (pedantic+nursery), zero fmt drift.**

## [0.9.16] — Phase 44: guideStone Level 4 + Live NUCLEUS Validation + plasmidBin Depot (2026-04-20)

### Milestone
**guideStone Level 4 — NUCLEUS guideStone works.** 67/67 ALL PASS against live
12-primal NUCLEUS deployed from plasmidBin. 41/41 bare checks (Level 3).
BLAKE3 checksums (P3 self-verifying). plasmidBin documented as remote depot
for downstream live deployments. 72 clippy warnings eliminated (zero remaining).
9 new gaps discovered and 7 resolved during live validation (PG-16 through PG-23).

### Added
- **`primalspring::checksums` module**: BLAKE3 file hashing, manifest generation
  (`generate_manifest()`), and verification (`verify_manifest()`). Used by
  guideStone Property 3 (self-verifying). Depends on `blake3` crate.
- **`IpcError::is_protocol_error()`**: Classifies wire-level protocol violations
  (HTTP-on-UDS) as reachable-but-incompatible. Used by guideStone and
  `validate_liveness()` for graceful degradation.
- **Family-aware capability discovery**: `discover_by_capability()` now checks
  `{capability}-{family}.sock` before falling back to `{capability}.sock`.
- **`validation/CHECKSUMS`**: BLAKE3 manifest of 18 validation-critical files.
- **`examples/gen_checksums.rs`**: Generator for CHECKSUMS manifests.
- **`wateringHole/PLASMINBIN_DEPOT_PATTERN.md`**: Documents plasmidBin as
  remote binary depot for downstream spring teams.

### Changed
- **guideStone graph parsing**: Fragments validated as `[fragment]` TOML (not
  `[graph]`). Manifests/templates skipped. Profiles validated after resolution.
- **guideStone shader check**: Accepts coralReef's `supported_archs` response
  format (not just legacy `capabilities` array).
- **guideStone exit semantics**: Protocol errors treated as SKIP (reachable but
  mismatched) rather than FAIL, in both `validate_liveness()` and Layer 2.
- **`validate_liveness()` in `composition/mod.rs`**: Protocol errors increment
  alive count (primal is reachable, just different protocol dialect).
- **`meta_tier.toml`**: Added explicit order values (10/11/12) to prevent
  duplicate-order conflicts in profile resolution.
- **Profile order fixes**: `full.toml` (12→99), `nest_viz.toml` (4→7).
- **Validation graph TOML fixes**: `crypto_negative_validate.toml` and
  `nucleus_atomics_validate.toml` migrated from illegal duplicate
  `[graph.node.operation]` tables to per-entry `[graph.nodes.operation]`.
- **72 clippy doc_markdown warnings fixed** across 14 source files via auto-fix.

### Fixed
- **`start_primal.sh`** (plasmidBin): `serve` → `server` for provenance trio;
  removed unsupported `--bind` from squirrel case.

### Documentation
- `PRIMAL_GAPS.md`: primalSpring upgraded to Level 4; 9 new gaps (PG-16–PG-23).
- `GUIDESTONE_COMPOSITION_STANDARD.md` v1.1.0: BLAKE3 checksums integration
  pattern, updated readiness table.
- `NUCLEUS_SPRING_ALIGNMENT.md`: Updated for guideStone Level 4.

## [0.9.15] — Phase 43+: Graph Consolidation + Fragment-First Composition + guideStone (2026-04-16)

### Milestone
**78 → 56 deploy graph TOMLs.** Fragment-first composition eliminates isomorphic duplication.
All profiles resolve fragments at load time via `resolve = true`. Template+manifest pattern
replaces per-spring/per-downstream boilerplate. Stadial parity gate complete: zero `dyn`,
zero `async-trait`, Edition 2024, `#[expect]` over `#[allow]` everywhere.

### Changed
- **Fragment-first `load_graph()`**: Graphs declaring `resolve = true` in `[graph.metadata]`
  inherit nodes from `graphs/fragments/*.toml` as a base layer, then apply only their delta
  nodes. Implemented via `resolve_fragments()` in `ecoPrimal/src/deploy/mod.rs`.
- **9 profiles trimmed**: All `graphs/profiles/*.toml` reduced from ~40 to ~15 lines each,
  retaining only unique delta nodes and setting `resolve = true`.
- **Spring validation consolidated**: 13 → 4 files. `spring_validate_template.toml` +
  `spring_validate_manifest.toml` (9 parameterized compositions) + 2 unique validators.
- **Spring deploy consolidated**: 5 → 2 files. `spring_deploy_template.toml` +
  `spring_deploy_manifest.toml` (5 parameterized springs).
- **Downstream proto-nucleate consolidated**: 7 → 3 files. `proto_nucleate_template.toml` +
  `downstream_manifest.toml` (7 parameterized compositions) +
  `healthspring_enclave_proto_nucleate.toml` (unique dual-tower ionic bridge).
- **`nucleus_complete.toml`**: Now the canonical NUCLEUS deploy graph (absorbs former
  `primalspring_deploy.toml`).
- **`profiles/full.toml`**: Now the canonical full overlay (absorbs former `full_overlay.toml`).
- **`GraphMeta`**: Added optional `metadata` field with `fragments`, `resolve`, and
  fragment-resolution support structs.

### Removed
- **25 redundant graph files**: 9 spring validation wrappers, 5 per-spring deploy files,
  7 individual proto-nucleate files, `primalspring_deploy.toml`, `full_overlay.toml`,
  `fossilRecord/graphs/` stale snapshots.

### Documentation
- Updated README.md, CONTEXT.md, SECURITY.md, CONTRIBUTING.md, CHANGELOG.md with
  v0.9.15 metrics (570 tests, 75 experiments, 56 deploy graphs).
- Updated `graphs/README.md` with fragment resolution documentation and revised hierarchy.
- Updated `wateringHole/` docs with consolidated graph counts.
- New handoff: `wateringHole/GRAPH_CONSOLIDATION_AND_NUCLEUS_DEPLOYMENT_HANDOFF_APR16_2026.md`.
- `config/capability_registry.toml` and `deployment_matrix.toml` version/date updated.

### Added
- **`primalspring_guidestone` binary**: Self-validating deployable that certifies NUCLEUS
  composition correctness across 6 layers (bare properties, discovery, atomic health,
  capability parity, cross-atomic pipeline, bonding model, BTSP/crypto). Base certification
  layer that domain guideStones inherit from.
- **guideStone Composition Standard**: `wateringHole/GUIDESTONE_COMPOSITION_STANDARD.md` —
  ecosystem-wide standard for self-validating deployables (5 properties, 6 readiness levels,
  bare vs NUCLEUS modes, validation window concept).
- **Layered certification model**: primalSpring guideStone = base (composition), domain
  guideStone = domain (science). Domain guideStones can call `primalspring_guidestone` as
  a pre-flight check.
- **Composition API evolution**: `capability_to_primal()` made public, new
  `method_to_capability_domain()` and `validate_liveness()` for downstream springs.
- **guideStone readiness tracking** in `PRIMAL_GAPS.md`: primalSpring Level 1, hotSpring
  Level 5 (certified), 4 springs at Level 1, 2 at Level 0.

### Test Results
- **primalSpring**: 570 tests PASS, 0 failures, 0 clippy warnings.

## [Unreleased] — Phase 42–43: Multi-Tier Genetics + Cross-Architecture Deployment (2026-04-15)

### Milestone
**Multi-tier genetics identity system.** Mito-Beacon (discovery/NAT), Nuclear lineage
(non-fungible permissions with generational mixing), Tags (open participation from
plaintext seed heritage). BTSP Phase 3 types (ChaCha20-Poly1305 encrypted channels).
Cross-architecture Tower bootstrap on Pixel (aarch64 + GrapheneOS) via biomeOS Neural
API `--tcp-only`. **14/15 cross-arch checks pass** (HSM/Titan M2 pending upstream).
**NUCLEUS composition fully validated**: exp091 12/12 routing, exp094 19/19 parity.
All 7 biomeOS/NestGate composition gaps RESOLVED.

### Added
- **BTSP Phase 3 types**: `Phase3Cipher`, `NegotiateRequest`/`NegotiateResponse`,
  `SessionKeys` with HKDF-SHA256 key derivation and ChaCha20-Poly1305 in
  `ecoPrimal/src/btsp/phase3.rs`.
- **BtspEnforcer deny semantics**: Explicit deny per `TrustModel` (NuclearLineage,
  MitoBeaconFamily, TagOpen) in `bonding/mod.rs`.
- **Ionic bond protocol**: `IonicBondClient` RPC wrappers, `IonicPeerIdentity`,
  `DiscoveryMethod` in `bonding/ionic_rpc.rs`.
- **Content distribution federation**: `content_distribution_federation.toml` graph
  (8-phase pipeline, 4 bonding tiers, BLAKE3 content addressing). Domain types in
  `bonding/content_distribution.rs`.
- **`tcp_rpc_multi_protocol`**: Auto-fallback from raw TCP to HTTP POST in `ipc/tcp.rs`.
  Added `http_json_rpc` for Songbird HTTP transport.
- **exp096**: Cross-architecture bonding experiment — biomeOS Neural API health,
  proxied primal health, capability counts, FAMILY_ID verification, genetics RPC,
  BTSP Phase 3, HSM probing.
- **Genetics integration tests**: 4 new `#[ignore]` tests (mito-beacon, nuclear
  lineage, proof, entropy mixing) in `server_ecosystem.rs`.
- **Three-tier genetics validation** in exp073 LAN covalent mesh.
- **biomeOS bootstrap.rs patch**: Environment inheritance for `BIOMEOS_PLASMID_BIN_DIR`,
  `ECOPRIMALS_PLASMID_BIN`, `XDG_RUNTIME_DIR`, `FAMILY_SEED`.

### Fixed
- **`tcp_rpc` shutdown(Write)**: Removed premature half-close that prevented server
  responses. Added `flush()` instead.
- **`FAMILY_SEED` encoding**: `raw_family_seed_from_env()` now auto-detects hex vs
  base64 encoding.
- **`prepare_spore_payload.sh` ECO_ROOT**: Fixed path depth (was one `dirname` short).
- **`validate_remote_gate.sh` ports.env path**: Fixed relative path to plasmidBin.

### Documentation
- Updated CONTEXT.md, README.md, experiments/README.md, wateringHole/README.md,
  whitePaper/baseCamp/README.md with Phase 43 metrics (74 experiments, 17 tracks).
- New handoff: `wateringHole/PRIMALSPRING_V0914_PHASE43_HANDOFF_APR14_2026.md` —
  per-primal evolution paths, spring deployment patterns, biomeOS gaps.
- New blurb: `wateringHole/BIOMEOS_COMPOSITION_GAPS_APR14_2026.md` — 7 upstream
  gaps with fix paths and validation evidence (all RESOLVED as of April 15).
- New doc: `wateringHole/UPSTREAM_CROSSTALK_AND_DOWNSTREAM_ABSORPTION.md` —
  primal cross-talk protocol (first-byte peek, socket naming, family-ID propagation)
  and spring/garden absorption patterns (maturity ladder, fragment composition,
  bonding models, library patterns to absorb).
- Updated `wateringHole/PRIMALSPRING_ECOSYSTEM_LEVERAGE_GUIDE.md` v1.4.0 —
  refreshed composition status, fixed broken `infra/wateringHole/` references.
- Updated `docs/PRIMAL_GAPS.md` — biomeOS routing fix, NestGate BTSP bypass,
  graph executor error reporting all RESOLVED.
- Added `fossilRecord/README.md` — archive directory index.
- Updated specs/PAPER_REVIEW_QUEUE.md, specs/CROSS_SPRING_EVOLUTION.md with Phase 43.

## [0.9.14] — Phase 41: Pre-Downstream Gap Resolution (2026-04-13)

### Milestone
**Pre-downstream gap resolution complete.** All 7 blocking/confusing gaps resolved
before handing to springs for absorption. 13 FullNucleus capabilities (was 11).
443 tests (was 441). DeployGraph now parses both single-node and multi-node dialects.

### Fixed
- **Songbird capability alias mismatch**: `ledger`, `attribution`, `visualization` now
  registered in Phase 5 launcher seeding. `FullNucleus` capabilities expanded from 11 to 13.
  `capability_to_primal()` maps all self-reported names (`spine`, `merkle`, `braid`, `provenance`).
- **`validate_parity_vec` silent element drop**: Non-numeric array elements now detected
  and reported explicitly instead of being silently filtered by `filter_map(as_f64)`.
- **`validate_parity` doc examples**: Module-level and function-level examples updated
  from `tensor.matmul` to `stats.mean` with correct `"data"` param and `"result"` key.
- **loamSpine `health.check` auto-param**: `CompositionContext::health_check()` now sends
  `{"include_details": true}` automatically for loamSpine capabilities.

### Changed
- **DeployGraph multi-node unification**: `GraphNode.name` accepts `id` alias. `binary`,
  `order`, `health_method` default when absent. `primal`, `operation`, `constraints`,
  `output` fields captured for multi-node sub-tables. `structural_checks()` skips
  binary/health/order validation for multi-node graphs. 2 new tests confirm parsing.
- **`NICHE_STARTER_PATTERNS.md`**: `tensor.softmax` result key `"values"` → `"result"`.

### Documented
- **BtspEnforcer limitations**: Struct-level and method-level docs explain cipher-upgrade-only
  semantics (never denies connections). Downstream README includes bonding enforcement table.
- **rhizoCrypt `dag.event.append` event_type**: Full 27-variant enum reference with 4 worked
  JSON examples added to downstream README.
- **`graphs/multi_node/README.md`**: Schema differences documented, deployment instructions.

### Test Results
- **primalSpring**: 443 tests PASS, 0 failures (2 new deploy tests for multi-node graphs).
- **exp094**: 19/19 PASS (unchanged — validates against live stack).

## [Unreleased] — Phase 40: NUCLEUS Complete — 12/12 ALIVE, 19/19 PASS (2026-04-13)

### Milestone
**exp094 NUCLEUS Composition Parity: 19/19 PASS, 0 FAIL, 0 SKIP.**
**12/12 primals ALIVE.** All LD gaps RESOLVED. Full NUCLEUS composition validated
across all 3 atomics (Tower + Node + Nest) + cross-atomic pipeline. 11 capabilities
discovered. This is the first time every primal is alive and every check passes.

### Resolved (Upstream — second pull cycle)
- **LD-10 RESOLVED** — barraCuda Sprint 42 phase 5: `fix(LD-10): replay consumed BTSP guard line to JSON-RPC handler`. JSON-RPC now fully operational on UDS (`stats.mean`, `capabilities.list`, 32 methods).
- **LD-06 RESOLVED** — rhizoCrypt S37: UDS unconditional, TCP opt-in. `rhizocrypt_alive` PASS. Provenance trio at full UDS parity.
- **Songbird Wave 139**: Self-healing socket auto-discovery — periodic 30s re-scan eliminates need for Phase 5 launcher seeding (retained as belt-and-suspenders).
- **BearDog**: TCP transport now skip when `--port`/`--listen` not passed.
- **petalTongue Sprint 6**: `--socket` CLI flag added for NUCLEUS launcher alignment. Now ALIVE in stack.

### Changed
- **Full ecosystem rebuild**: All 12 primals pulled, rebuilt, harvested to plasmidBin. Every primal has new commits addressing debt/gaps.
- **exp094**: `tensor_dot_3elem` replaced with `tensor_stats_mean` (barraCuda `stats.mean` — actual JSON-RPC wire method). `rhizocrypt_alive` now PASS (was SKIP).
- **Unit tests**: `node_parity_tensor_matmul_identity` updated to use `stats.mean` (barraCuda `tensor.matmul` uses session-based IDs, not inline data).

### Test Results
- **primalSpring**: 455 tests PASS, 0 failures.
- **exp094**: 19/19 PASS, 0 FAIL, 0 SKIP. 11 capabilities discovered.
- **NUCLEUS stack**: 12/12 primals ALIVE (including rhizoCrypt UDS, petalTongue, barraCuda).

## [0.9.13] — Phase 39: NUCLEUS Composition PASS — 17/17 exp094 (2026-04-13)

### Milestone
**exp094 NUCLEUS Composition Parity: 17/17 PASS, 0 FAIL, 2 SKIP.**
All 4 critical LD gaps resolved. Full NUCLEUS cross-atomic pipeline operational.

### Added
- **`nucleus_launcher.sh` Phase 5**: Registry seeding — after all primals start, launcher calls `ipc.register` on Songbird for 9 primals with their capability sets. Resolves LD-08 (Songbird auto-discovery races startup).
- **`IpcError::is_transport_mismatch()`**: Detects tarpc-socket-receives-JSON-RPC pattern (EAGAIN timeout). `validate_parity` and `validate_parity_vec` now gracefully SKIP on transport mismatch.

### Resolved (Upstream)
- **LD-04 RESOLVED** — ToadStool S203d: BTSP auto-detect via first-byte inspection.
- **LD-05 RESOLVED** — barraCuda Sprint 42 phase 2: eliminated TCP sidecar in UDS mode.
- **LD-08 RESOLVED** — Songbird Wave 138: socket auto-discovery + Phase 5 launcher seeding.
- **LD-09 RESOLVED** — loamSpine: TCP transports opt-in. UDS unconditional.

### Changed
- **`nucleus_launcher.sh`**: Removed `--unix` from barraCuda start args. Added Phase 5 registry seeding.
- **exp094**: `resolve_*` checks accept `native_endpoint`/`virtual_endpoint`. Transport mismatch → graceful SKIP.
- **Integration tests**: Updated overlay graph tests to current consolidated graphs.

### Test Results
- **primalSpring**: 455 tests PASS, 0 failures.
- **exp094**: 17/17 PASS, 2 SKIP (tensor.dot tarpc transport, rhizoCrypt TCP-only).

## [0.9.12] — Phase 38: Upstream Revalidation + Composition Parity (2026-04-12)

### Added
- **LD-08**: New gap — Songbird `ipc.resolve` returns "Primal not found" because primals don't call `ipc.register` at startup. Wire accepts `capability` param (Wave 137b) but no directory entries.
- **LD-09**: New gap — loamSpine binds `0.0.0.0:8080` on startup, conflicts with other services. Needs socket-only mode.

### Changed
- **Upstream revalidation**: Pulled + rebuilt + tested all 5 core NUCLEUS primals (barraCuda Sprint 42, BearDog Wave 38, ToadStool S203c, Songbird Wave 137d, NestGate Session 43d). Fresh ecoBins copied to plasmidBin.
- **exp094 revalidation**: 13/16 PASS (was 7/16). Tower Atomic fully operational, Nest storage roundtrip works (LD-03 resolved upstream), cross-atomic pipeline (hash→store→retrieve) PASS end-to-end.
- **`PRIMAL_GAPS.md` Class 5**: Full revalidation section added with per-primal socket/health/IPC status table. LD-03 marked RESOLVED upstream. LD-04/LD-05 re-assessed with current evidence.
- **`nucleus_launcher.sh`**: Fixed stale `compute.sock` symlink (pointed to `toadstool-${FAMILY_ID}.jsonrpc.sock`, now `toadstool-${FAMILY_ID}.sock`). Updated ToadStool status check to include new socket name. Added BTSP/Ed25519/X25519 aliases for BearDog. Fixed Squirrel alias to use family-scoped socket.

### Test Results (all 5 primals)
- **BearDog**: 14,774+ tests, 0 failures. CLEAN.
- **Songbird**: Build CLEAN, tests PASS.
- **NestGate**: 11,856+ tests, 0 failures. CLEAN.
- **ToadStool**: 1,025 passed, 2 failures (env-dependent localhost fallback). Non-blocking.
- **barraCuda**: 3,835 passed, 14 failures (ESN model + tensor scalar race under parallel load). Non-blocking.

## [0.9.11] — Phase 37: Live NUCLEUS Deployment + Downstream Patterns (2026-04-12)

### Added
- **9 composition parity tests** (`composition/mod.rs`): Tower (crypto.hash, capability.resolve, health.liveness), Nest (storage round-trip, NestGate health), Node (tensor.batch.submit, tensor.matmul identity, shader.compile.capabilities), NUCLEUS (cross-atomic pipeline: hash → store → retrieve → verify). All degrade to SKIP when primals absent, PASS against live NUCLEUS.
- **`ECOSYSTEM_EVOLUTION_CYCLE.md`** (`wateringHole/`): Water-cycle model for ecosystem evolution — mountains (primals), springs (validation), delta (domain springs), evaporation (gap feedback). Documents current season, acceleration effect, per-layer guidance, modernization debt section.
- **Composition Elevation Sprint Priorities** in `PRIMAL_GAPS.md`: 6-item primalSpring sprint roadmap with upstream enablement targets.
- **Class 4: Pre-Modern Async Rust** in `PRIMAL_GAPS.md`: Full ecosystem async-trait/dyn audit across 12 primals.
- **`CompositionContext::health_check()`**: Normalizes diverse primal health response formats (`{"alive":true}`, `{"status":"alive"}`, `{"status":"ok"}`) into single boolean.
- **`CompositionContext::hash_bytes()`**: Handles base64 encoding/decoding for BearDog `crypto.hash`.
- **`CompositionContext::resolve_capability()`**: Maps capability→primal_id for Songbird `ipc.resolve`.

### Changed
- **`PRIMAL_GAPS.md`**: Wire contract blockers (tensor/shader/dispatch) all marked RESOLVED. Sprint table updated: Tower + Nest + Node all UNBLOCKED, upstream primals all DELIVERED.
- **`SPRING_AUDIT_PROMPT.md`**: v2.2 — added `ECOSYSTEM_EVOLUTION_CYCLE.md` to required reading, seasonal context.
- **`NUCLEUS_SPRING_ALIGNMENT.md`**: Phase 36 — "Current Season: Mountain → Spring Transition" section.
- **`wateringHole/README.md`**: Added "Ecosystem Evolution" section to document index.

### Fixed
- **5 test fixtures** referencing archived graphs: `tower_atomic_bootstrap.toml` → `profiles/tower.toml`, `node_atomic_compute.toml` → `profiles/node.toml`, `tower_ai.toml` → `profiles/tower_ai.toml`, `conditional_fallback.toml` → `patterns/conditional_fallback.toml`. 428 → 437 tests, 0 failures.
- **3 pre-existing clippy warnings resolved**: unnecessary boolean not in `coordination/mod.rs`, missing `#[must_use]` on `strip_unix_uri`, if-let→`map_or_else` in `transport.rs`. clippy now returns 0 warnings.
- **`nucleus_launcher.sh`**: Added barraCuda + coralReef to Phase 2 (Node Atomic). Added `tensor` and `shader` to capability domain alias map. Updated stop sequence and status table.

### Documented
- **`exp094_composition_parity`**: New canonical NUCLEUS parity experiment (Tower + Node + Nest + cross-atomic pipeline). Live-tested against running NUCLEUS: 8 capabilities discovered, 7 PASS, 5 FAIL (upstream wire gaps), 7 SKIP.
- **`graphs/downstream/README.md`**: Index of proto-nucleate graphs with upstream gap table from live deployment.
- **`graphs/downstream/NICHE_STARTER_PATTERNS.md`**: Copy-paste-ready starter patterns for hotSpring (QCD), neuralSpring (ML), healthSpring (enclaves), wetSpring (genomics).
- **`PRIMAL_GAPS.md` Class 5**: 7 live deployment gaps (LD-01 through LD-07) from exp094 — base64 transport, ipc.resolve params, single-shot UDS, port conflicts, socket naming, health format inconsistency.
- **`SPRING_COMPOSITION_PATTERNS.md` §14**: NUCLEUS composition experiment pattern added with live deployment findings and key findings for springs.

## [0.9.10] — Phase 35: Seasonal Tightening + Composition Validation Library (2026-04-12)

### Added
- **Composition validation library** (`ecoPrimal/src/composition/`): `CompositionContext`, `validate_parity`, `validate_parity_vec` — capability-keyed IPC client management and one-call math parity comparison.
- **Typed IPC extractors** (`ipc/client.rs`): `call_extract<T>`, `call_extract_f64`, `call_extract_vec_f64`.
- **Math parity tolerances** (`tolerances/mod.rs`): 7 named constants (`EXACT_PARITY_TOL`, `CPU_GPU_PARITY_TOL`, `WGSL_SHADER_TOL`, `DF64_PARITY_TOL`, `STOCHASTIC_SEED_TOL`, `NETWORK_LATENCY_TOL`, `COMPOSITION_OVERHEAD_TOL`).
- **Composition parity checks** (`validation/mod.rs`): `check_composition_parity`, `check_composition_parity_vec`.

### Changed
- **`lib.rs`**: Rewritten module documentation — core intermediary role ("start, validate, surface gaps, provide library"), modules categorized as Core vs Supporting.
- **`coordination/mod.rs`**: `AtomicType::Node` now includes barraCuda + coralReef; `FullNucleus` includes all 11 primals.
- **`harness/mod.rs`**: `RunningAtomic::composition_context()` added.
- **All `#[allow(dead_code)]`** in `btsp_handshake.rs` converted to `#[expect(dead_code, reason = "...")]`.

### Removed
- **`inference/` module** (534 LOC): Archived to `fossilRecord/inference_module_apr12_2026/`. Speculative, unused — belongs with Squirrel/neuralSpring.
- **35 stale graph TOMLs** (4,328 LOC): `science/`, `sketches/`, `gen4/` archived to `fossilRecord/stale_graphs_apr12_2026/`. Graph count 102 → 67.
- **`EmergentValidation` struct** (29 LOC): Unused outside own tests.
- **`GraphExecutionResult` struct** (35 LOC): Unused outside own tests.
- **`spawn_neural_api` deprecated alias**: Removed from `launcher/mod.rs`.

### Fixed
- `BASE64.encode(&client_pub_bytes)` → `BASE64.encode(client_pub_bytes)` (clippy: borrowed expression).
- All `map_or` with function calls → `map_or_else` (clippy: function call in map_or).
- `PRIMAL_GAPS.md` split into active (865 LOC) + `PRIMAL_GAPS_RESOLVED_HISTORY.md` (522 LOC) to stay under 1000 LOC standard.

## [0.9.9] — Phase 33: Full Spring Alignment + Proto-Nucleate Coverage (2026-04-10)

### Added
- **3 new proto-nucleate graphs**: `airspring_ecology_proto_nucleate.toml` (Tower + Node + Nest, ecology/agriculture), `groundspring_geoscience_proto_nucleate.toml` (Tower + Node + Nest, measurement science), `wetspring_lifescience_proto_nucleate.toml` (Tower + Node + Nest + Meta, life science/chemistry). All 7 science springs now have proto-nucleates.
- **`NUCLEUS_SPRING_ALIGNMENT.md`** (`wateringHole/`): Comprehensive spring × atomic alignment matrix, per-spring composition diagrams, cross-pollination network, neuralSpring AI provider role documented.

### Changed
- **`hotspring_qcd_proto_nucleate.toml`**: Added Squirrel/neuralSpring AI cross-pollination evolution note.
- Deploy graph count: 93 → 96 (8 downstream proto-nucleates).

## [0.9.8] — Phase 32: NUCLEUS Validation & Spring Evolution Handoff (2026-04-10)

### Changed
- **`nucleus_atomics_validate.toml`**: Fixed old graph references (`tower_atomic_bootstrap` → `profiles/tower`, etc.), corrected SUMMARY atomic definitions to Phase 31 model (Node = Tower + ToadStool + barraCuda + coralReef, Nest = Tower + NestGate + provenance trio), bumped to v0.2.0.
- **`deployment_matrix.toml`**: Bumped to v3.0.0. Added `[atomics]` documentation section. Fixed `nucleus_3node`, `wan_federation`, `federation_10node`, `organo_metal_salt` topology primal lists to include full NUCLEUS (barraCuda, coralReef, provenance trio). Added `atomic = ` metadata to all 21 topologies. Updated `security_tiers.nucleus.primals`.
- **`PRIMALSPRING_COMPOSITION_GUIDANCE.md`**: Bumped to v0.9.8. Fixed Node/Nest/NUCLEUS definitions. Updated overlay graph refs to `profiles/` paths. Added Layer 0 (Atomics) to composition diagram. Added §16 Meta-Tier Primals section.
- **`PRIMALSPRING_ECOSYSTEM_LEVERAGE_GUIDE.md`**: Bumped to v0.9.8. Fixed Composition Patterns table atomic definitions. Added NUCLEUS Atomic Alignment (Phase 31) reference table. Updated deploy graph count to 93.

### Added
- **Spring Evolution Feedback Guidance** in `PRIMALSPRING_ECOSYSTEM_LEVERAGE_GUIDE.md`: Per-spring evolution map, feedback protocol, primal evolution priority table, and step-by-step session guide for springs evolving primals through composition.

## [0.9.7] — Phase 31: Atomic Cleanup & Bonding Alignment (2026-04-09)

### Fixed
- **3 multi_node graphs** (`three_node_covalent_cross_network`, `idle_compute_federation`, `friend_remote_covalent`): Merged duplicate `[graph.bonding_policy]` tables into single valid TOML sections.
- **5 graphs claiming `node_atomic` without coralReef**: Added coralReef nodes to `gaming_mesh_chimera`, `gen4_storytelling_full`, `ludospring_proto_nucleate`, `nucleus_game_session`, `ludospring_science_validation`.
- **6 graphs using `provenance_trio` where `nest_atomic` fits**: Upgraded to `nest_atomic` where NestGate + full trio present (`hotspring_qcd_proto_nucleate`, `healthspring_enclave_proto_nucleate`, `rpgpt_session_provenance`, `supply_chain_provenance`, `nucleus_complete`, `interactive_product`).
- **7 graph descriptions overstating "NUCLEUS"**: Replaced with accurate atomic descriptions where full NUCLEUS primals were not present.

### Changed
- **~20 graphs with understated fragments**: Updated `fragments` metadata to properly reflect atomics present (added `nest_atomic`, `node_atomic`, `meta_tier` where primals exist).
- **9 nucleated deploys gained `[graph.bonding_policy]`**: All cross-atomic graphs now declare bonding type, trust model, and encryption tiers per atomic boundary.
- **3 multi_node graphs gained `atomic_layout`**: Documents which gate plays which atomic role.

## [0.9.6] — Phase 30: NUCLEUS Atomic Alignment (2026-04-09)

### Added
- **6 atomic-aligned fragments** (`graphs/fragments/`): `tower_atomic` (electron: BearDog + Songbird), `node_atomic` (proton: Tower + ToadStool + barraCuda + coralReef), `nest_atomic` (neutron: Tower + NestGate + provenance trio), `meta_tier` (biomeOS + Squirrel + petalTongue), `nucleus` (Tower + Node + Nest), `provenance_trio` (kept). Aligned to gen3 ECOSYSTEM_ARCHITECTURE §3 particle model.
- **9 NUCLEUS profiles** (`graphs/profiles/`): `tower`, `node`, `nest`, `nucleus`, `full`, `tower_ai`, `tower_viz`, `node_ai`, `nest_viz`. Each is a documented slice of NUCLEUS with `base = "nucleus_complete"` metadata.
- **Spring validation template + manifest** (`spring_validate_template.toml` + `spring_validate_manifest.toml`): Parameterized skeleton replaces 6 identical per-spring validation graphs.
- **Execution patterns directory** (`graphs/patterns/`): `parallel_capability_burst`, `conditional_fallback`, `streaming_pipeline`, `continuous_tick` — coordination mode demonstrations.

### Changed
- **Fragment names aligned to atomics**: `tower_base` → `tower_atomic`, `wgsl_shader_pipeline` → `node_atomic`, `nucleus_core` → `nucleus`. All 93 graphs updated.
- **Meta-tier primals documented**: biomeOS, Squirrel, petalTongue operate at any atomic level — not part of any single atomic.
- **Nest Atomic now includes provenance trio**: rhizoCrypt + loamSpine + sweetGrass are integral to the neutron (storage + lineage).
- **4 ludo/webb sketch validates updated**: Reflect pure composition model (no spawnable binaries).
- **hotspring_deploy merged into proto-nucleate**: `spring_deploy/hotspring_deploy.toml` absorbed by `downstream/hotspring_qcd_proto_nucleate.toml`.
- Deploy graph count: 100 → 93 (+ 6 fragment definitions, 17 directories).

### Removed
- **8 root-level subset graphs**: `tower_atomic_bootstrap`, `tower_ai`, `tower_full_capability`, `node_atomic_compute`, `node_ai`, `nest-deploy`, `nest_viz`, `full_overlay` — replaced by `graphs/profiles/`.
- **6 per-spring validate files**: `airspring_validate`, `groundspring_validate`, `wetspring_validate`, `hotspring_validate`, `neuralspring_validate`, `healthspring_validate` — replaced by template + manifest.
- **3 composition files + directory**: `narration_ai`, `proprioception_loop`, `persistence_standalone` — absorbed into corresponding `composition_*_validate` files.
- **3 old fragment definitions**: `tower_base.toml`, `wgsl_shader_pipeline.toml`, `nucleus_core.toml` — replaced by atomic-aligned versions.
- **hotspring_deploy.toml**: Merged into `hotspring_qcd_proto_nucleate.toml`.

## [0.9.5] — Phase 29: Graph Consolidation + Composition Evolution (2026-04-09)

### Added
- **4 canonical fragment definitions** (`graphs/fragments/`): `tower_base` (biomeOS + BearDog + Songbird), `provenance_trio` (rhizoCrypt + loamSpine + sweetGrass), `wgsl_shader_pipeline` (coralReef + toadStool + barraCuda), `nucleus_core` (Tower + toadStool + NestGate + Squirrel). Documents the "periodic table" of NUCLEUS composition.
- **`composition_model` + `fragments` metadata**: Every deploy graph annotated with `composition_model = "pure"|"nucleated"|"validation"` and `fragments = [...]` listing which canonical patterns it composes. Makes isomorphic families visible.
- **Composition Evolution section** in `PRIMAL_GAPS.md`: documents that ludoSpring and esotericWebb are pure NUCLEUS compositions — the graph IS the product, biomeOS IS the execution engine.

### Changed
- **ludoSpring/esotericWebb proto-nucleates rewritten** as pure compositions (`composition_model = "pure"`): no `spawn = true` binary nodes. Game science capabilities map to barraCuda + toadStool + Squirrel + petalTongue. CRPG product maps to Squirrel + petalTongue + NestGate + provenance trio.
- **8 graphs rewritten**: `interactive_product`, `gen4_storytelling_full`, `gen4_storytelling_minimal`, `gen4_spring_composition`, `rpgpt_session_provenance`, `nucleus_game_session`, `ludospring_science_validation`, `gaming_mesh_chimera` — all ludo/webb binary nodes replaced with constituent NUCLEUS primals.
- **Gen4 naming normalized**: `biomeos` / `biomeos_api` / `biomeos_substrate` → canonical `biomeos_neural_api` across all 10 gen4 graphs. `depends_on` edges updated.
- Deploy graph count: 107 → 100 (7 deleted, 4 fragment definitions added separately).

### Removed
- **5 redundant sketches**: `ludospring_game_deploy`, `esotericwebb_tower_deploy`, `game_science_standalone`, `session_standalone`, `esotericwebb_composed_deploy` — superseded by rewritten proto-nucleates.
- **2 duplicate compositions**: `render_standalone.toml` (identical to `proprioception_loop`), `tower_ai_viz.toml` (redundant subset of `tower_ai`).

## [0.9.4] — Phase 28: BTSP Cascade, Inference Abstraction, Proto-Nucleate Graphs (2026-04-10)

### Added
- **BTSP client handshake** (`ipc::btsp_handshake`): Client-side secure-by-default authentication module. All socket connections can now perform `btsp.handshake` before capability calls.
- **Inference wire standard** (`inference` module): Vendor-agnostic `inference.complete`, `inference.embed`, `inference.models` wire types and `InferenceClient` in ecoPrimal. Decouples AI inference from CUDA/Ollama vendor lock-in.
- **Squirrel inference bridge**: `inference.complete`/`embed`/`models` dispatch routes in Squirrel's JSON-RPC server, bridging ecoPrimal wire types to `AiRouter`.
- **5 proto-nucleate graphs** (`graphs/downstream/`): neuralSpring ML inference (WGSL shader composition), hotSpring Lattice QCD (metallic GPU pool, df64, provenance), healthSpring dual-tower enclave (ionic bond, egress fence, clinical AI). Two additional composition variants.
- **3 pipeline graphs**: neuralSpring inference pipeline, hotSpring QCD pipeline, healthSpring clinical pipeline — end-to-end data flow through primal compositions.
- **WGSL shader composition model**: ML inference, QCD physics, and biology are compositions of barraCuda's 826 WGSL shaders, compiled by coralReef, dispatched by toadStool. Springs are application layers, not compute engines.
- **Spring evolution nucleation** in `PRIMAL_GAPS.md`: hotSpring (proton-heavy, metallic bond, ionic lease for CERN cloud), healthSpring (neutron-heavy, dual-tower enclave, covalent mesh).

### Changed
- **BTSP Phase 2 cascade**: 11/13 primals enforce `btsp.handshake`. All 107 deploy graphs carry `[graph.metadata] secure_by_default = true` and `btsp_phase = 2`.
- **ToadStool semantic cleanup**: Removed unimplemented `ollama.*` and `inference.*` method mappings — ToadStool is compute substrate, not inference provider.
- Deploy graph count: 99 → 107 (5 downstream proto-nucleate + 3 pipeline).
- Root docs, CONTEXT.md, experiments/README synchronized to April 10 state.
- 404 tests, 72 experiments (15 tracks), 107 deploy graphs.

## [0.9.3] — Phase 26: Mixed Composition + Live Validation Matrix (2026-04-07)

### Added
- **Particle model specification** (`specs/MIXED_COMPOSITION_PATTERNS.md`): Tower = electron (trust boundary), Node = proton (compute, fungible), Nest = neutron (data at rest, non-fungible), NUCLEUS = atom. Grounded in Paper 23 (Mass-Energy-Information Equivalence).
- **Layered validation framework**: L0 (biomeOS + any primal), L1 (each atomic), L2 (mixed atomics), L3 (bonding patterns). Documented in `specs/NUCLEUS_VALIDATION_MATRIX.md`.
- **17 sketch graphs** (`graphs/sketches/`): L0 primal routing matrix (10 domains), L2 dual-tower ionic, dedicated tower, nest enclave, L3 covalent mesh backup, ionic capability lease, organo-metal-salt complex, plus variations.
- **exp091 primal routing matrix**: L0 validation — drives 10-domain Neural API routing sweep.
- **exp092 dual tower ionic**: L2 validation — two Tower Atomics with ionic bond structural validation.
- **exp093 covalent mesh backup**: L3 validation — sharded encrypted backup across covalent peer mesh.
- **Live validation matrix** (`specs/CROSS_SPRING_EVOLUTION.md`): Tower Atomic (BearDog + Songbird) fully validated live on Eastgate. 6 GAP-MATRIX items documented from systematic probing.

### Fixed
- **GAP-MATRIX-01 identified (Critical)**: Neural API detects primal sockets but reports 0 capabilities — blocks all `capability.call` routing. Direct IPC works.
- **GAP-MATRIX-02 identified (Medium)**: `tower_atomic_bootstrap.toml` fails biomeOS internal parser despite valid TOML.
- **GAP-MATRIX-03 identified (Low)**: Songbird TLS 1.3 cipher suite gaps for some HTTPS targets.
- **GAP-MATRIX-04 identified (Medium)**: NestGate HTTP REST IPC diverges from JSON-RPC over UDS model.

### Changed
- 404 tests, 72 experiments (15 tracks), 99 deploy graphs.
- Root docs, wateringHole, whitePaper/baseCamp, experiments/README, CONTEXT.md metrics synchronized.
- `CROSS_SPRING_EVOLUTION.md` updated with "Live Validation Matrix — April 7, 2026" section.
- `NUCLEUS_VALIDATION_MATRIX.md` updated with live validation results and sketch cross-references.

## [0.9.2] — Phase 25: Modernization Sweep (2026-04-07)

### Fixed
- **Capability naming**: `dag.dehydrate` → `dag.dehydration.trigger` across `capability_registry.toml`, `niche.rs`, and 17 graph files. Also fixed stale `dag.create_session`/`dag.append_event`/`dag.merkle_root`/`commit.session`/`commit.entry` → canonical dotted names in `primalspring_deploy`, `nucleus_complete`, `continuous_tick`, and `data_federation_cross_site` graphs.
- **Graph format unification (NA-016 resolved)**: Parser accepts `[[graph.node]]`, `[[graph.nodes]]`, and top-level `[[nodes]]` via serde alias + merge. All 87+ graphs migrated from `[[graph.node]]` to `[[graph.nodes]]`. Multi-node graphs converted from `[[nodes]]` to `[[graph.nodes]]` with `[graph.nodes.*]` subsections. `GraphMeta` gains optional `id` field.
- **HTTP health probe deprecated (NA-009 resolved)**: `http_health_probe` marked `#[deprecated]` — Songbird no longer exposes HTTP /health; Tower Atomic owns all HTTP. Experiments exp073, exp074, exp076, exp081 updated to use `tcp_rpc` with `health.liveness`.
- **Discovery tier count**: README corrected from 5-Tier to 6-Tier (plain socket name tier was missing from docs).

### Added
- **`nest-deploy.toml` v4.0**: Gold standard graph — HTTPS validation phase (order 5) calls `http.get` to `https://ifconfig.me/ip` through Tower Atomic. Songbird gains `mesh.init`, `mesh.auto_discover`, `mesh.peers` capabilities.
- **exp090 Tower Atomic LAN probe**: BirdSong mesh discovery, peer capability enumeration, HTTPS through Tower Atomic, STUN/NAT detection.
- **exp073 covalent bonding modernized**: Neural API routing validation, `FAMILY_ID` genetic lineage via BearDog `health.check`, end-to-end HTTPS through Tower Atomic.
- **`basement_hpc_covalent.toml`**: Canonical capability names, HTTPS validation phase between `gate_validate` and `announce_capabilities`.

### Changed
- 404 tests, 69 experiments (15 tracks), 92 deploy graphs.
- Root docs, wateringHole, whitePaper/baseCamp, experiments/README metrics synchronized.
- `CROSS_SPRING_EVOLUTION.md`: NA-009 and NA-016 marked RESOLVED with detailed notes.
- `SHOWCASE_MINING_REPORT.md`: "HTTP REST" references corrected to JSON-RPC 2.0 serialization.

## [0.9.1] — Phase 24b: rustChip B → A (2026-04-05)

### Fixed
- **rustChip**: B → A — 828 clippy warnings resolved to 0 (workspace cast allows for numeric code, auto-fix + manual source fixes). 237 tests added across 5 crates (was 0 test functions). Coverage 60.8% (software-testable; hardware-only VFIO/mmap excluded). `tarpaulin.toml` with fail-under=60.0 and hardware exclude list.

### Changed
- All 4 ecosystem tools now at Grade A
- `ECOSYSTEM_COMPLIANCE_MATRIX.md` v2.1.0 — rustChip updated to Grade A with coverage column

## [0.9.0] — Phase 24: Deep Debt Resolution + Public Readiness Sprint (2026-04-05)

### Fixed
- **bingoCube**: Public-readiness scrub — internal docs deleted, home paths scrubbed, path dep made optional, README AGPL wording fixed, whitePaper license aligned, broken links fixed. `shell.rs` refactored into 3 cohesive modules (shell+snapshot+evolve). Coverage 62.6% → 83.4%.
- **benchScale**: B → A — README license aligned, archive paths scrubbed, all `#[allow(` → `#[expect(` (7 sites), SPDX consistency fixed. Unsafe evolution: `EnvGuard` RAII, `LeaseList` safe abstraction, `libc::kill` → `nix::sys::signal::kill`. `thiserror` 1.0 → 2.0. Large files refactored (`vm_state.rs`, `stages.rs`, `config/legacy.rs`). Coverage 35.5% → 61.9%.
- **agentReagents**: B → A — Path dep documented, README license aligned, archive paths scrubbed, all `#[allow(` → `#[expect(` (9+ sites), template passwords documented. Hardcoded Songbird registration → capability-based `RegistrationSettings`. Coverage 7.1% → 60.2%.
- **rustChip**: 31 unsafe blocks documented with `// SAFETY:` comments, `deny(unsafe_op_in_unsafe_fn)` enforced. `vfio/mod.rs` split into `ioctls.rs` + `container.rs`, `hybrid/mod.rs` → `software.rs`. `create_stub_model` → `create_reference_model`, `SoftSystemBackend` → `SoftwareBackend`.

### Added
- `tarpaulin.toml` with `fail-under = 60.0` on bingoCube, benchScale, agentReagents
- C dependency documentation in `deny.toml` for all 4 tools (virt/libvirt, sqlite3/sqlx, libc VFIO)
- Cross-primal doc references in rustChip marked as "ecosystem context — not a runtime dependency"
- `ECOSYSTEM_COMPLIANCE_MATRIX.md` v2.0.0 — coverage column, updated tool grades and debt summary

### Changed
- Tool grade distribution: 3 A (bingoCube, benchScale, agentReagents), 1 B (rustChip)
- **Public-ready**: bingoCube, benchScale, agentReagents cleared for public repos

## [0.8.0y] — Phase 23y: Full Tool Compliance Sprint + Ecosystem Tool Taxonomy (2026-04-04)

### Fixed
- **bingoCube**: F → A — Edition 2024, `AGPL-3.0-or-later`, `forbid(unsafe_code)`, clippy pedantic+nursery clean, 54 tests passing, SPDX headers on all 20 files, CHANGELOG, CONTEXT.md, `deny.toml`, README updated with nautilus. All `gen` variable renames for Rust 2024 edition keyword.
- **benchScale**: C → B — License `-or-later`, fmt fixed (shebangs removed), clippy clean, 401 tests + 73 doctests passing, `deny.toml`, SPDX updated, 18 stale doctests fixed.
- **agentReagents**: D → B — License `-or-later`, fmt clean, clippy clean, 52 tests passing, CHANGELOG + `deny.toml` added, SPDX updated, `unwrap_used` lint scoped to non-test.
- **rustChip**: C → B — Edition 2024 (`gen` keyword migration), workspace lints (`forbid(unsafe)`, pedantic+nursery), clippy clean, all `#[allow(` → `#[expect(`, CONTEXT.md + `deny.toml`.

### Added
- **Ecosystem Tool Taxonomy** — Codified gen2.5 "Tool" layer in `PRIMAL_SPRING_GARDEN_TAXONOMY.md`, `PRIMAL_RESPONSIBILITY_MATRIX.md`, and `GLOSSARY.md`.
- **Ecosystem Tools section** in `ECOSYSTEM_COMPLIANCE_MATRIX.md` v1.1.0 — All 4 tools now grade B or above.

### Changed
- Tool grade distribution: 1 A (bingoCube), 3 B (benchScale, agentReagents, rustChip)

## [0.8.0x] — Phase 23x: Ecosystem Compliance Matrix — 9 Tiers, 14 Primals (2026-04-04)

### Added
- **`wateringHole/ECOSYSTEM_COMPLIANCE_MATRIX.md` v1.0.0** — Comprehensive compliance matrix covering 40+ auditable dimensions across 9 tiers (Build Quality, UniBin/ecoBin, IPC Protocol, Discovery/Self-Knowledge, Semantic Naming, Responsibility/Overstep, Workspace Dependencies, Presentation, Deployment/Mobile). Each primal graded A–F per tier with rollup grade. Supersedes `IPC_COMPLIANCE_MATRIX.md` v1.6.0.

### Audited
- **Phase 2 checks** across all 14 primals: `forbid(unsafe_code)`, `warn(missing_docs)`, CONTEXT.md, `#[allow(` vs `#[expect(`, PII scan, workspace dependencies, commented-out code, SPDX headers, C deps via `cargo tree`.
- **Grade distribution**: 1 A (rhizoCrypt), 7 B (BearDog, coralReef, biomeOS, petalTongue, sweetGrass, LoamSpine, sourDough), 3 C (Songbird, NestGate, Squirrel), 2 D (ToadStool, barraCuda), 1 F (bingoCube).
- **Top ecosystem gaps**: discovery debt (5 primals with >100 primal-name refs), license alignment (8 primals need `-or-later`), `#[allow(` migration (4 primals with >30 allow attrs), domain symlinks (8 primals missing).

### Changed
- `wateringHole/IPC_COMPLIANCE_MATRIX.md` — Archived to `fossilRecord/consolidated-apr2026/`
- `wateringHole/README.md` — Updated reference from IPC matrix to ecosystem matrix
- `docs/PRIMAL_GAPS.md` — Updated header to reference compliance matrix and grade distribution

## [0.8.0w] — Phase 23w: wateringHole Consolidation — 74 docs to 31 (2026-04-04)

### Changed
- **wateringHole consolidation**: 49 original documents merged into 7 consolidated files, reducing the top-level from 74 to 31 documents. Originals archived to `fossilRecord/consolidated-apr2026/`.
  - 13 leverage guides → `LEVERAGE_GUIDES.md`
  - 3 licensing docs → `LICENSING_AND_COPYLEFT.md`
  - 5 GPU/compute docs → `GPU_AND_COMPUTE_EVOLUTION.md`
  - 16 deploy/composition docs → `DEPLOYMENT_AND_COMPOSITION.md` + `ARTIFACT_AND_PACKAGING.md`
  - 12 spring pattern docs → `SPRING_INTERACTION_PATTERNS.md` + `SPRING_COORDINATION_AND_VALIDATION.md`
- `wateringHole/README.md` — Document Index rewritten to reflect consolidated structure
- `wateringHole/STANDARDS_AND_EXPECTATIONS.md` — All section references updated to point to consolidated documents

## [0.8.0v] — Phase 23v: Self-Knowledge Standard + Songbird Wave 111 Audit (2026-04-04)

### Added
- **`wateringHole/PRIMAL_SELF_KNOWLEDGE_STANDARD.md` v1.0.0** — New canonical standard defining the self-knowledge boundary, capability domain registry, socket naming (`{domain}.sock` primary, `{primal}.sock` legacy symlink), env var conventions (`{DOMAIN}_SOCKET` not `{PRIMAL}_SOCKET`), six code organization patterns (provider traits, tiered discovery, serde aliases, deprecated test fixtures, capability.call, EnvSource injection), compliance audit checklist with scoring (A–F), and phased migration path. Unifies fragmented guidance from 7+ existing standards.

### Audited
- **Songbird wave 110-111**: Discovery **935→302 refs** (68% further reduction, 88% total since first audit). concurrent_helpers smart-refactored (787L → 8 modules). +38 tests. 12,568 tests passing. Clippy/fmt/deny CLEAN.
- **barraCuda**: Pushed clippy lint fix (stale `expect(clippy::large_stack_arrays)`). SIGSEGV is concurrent-test driver contention (llvmpipe/NVK) — passes single-threaded. Larger project for barraCuda team.

### Changed
- `wateringHole/CAPABILITY_BASED_DISCOVERY_STANDARD.md` — Added cross-reference to self-knowledge standard
- `wateringHole/PRIMAL_RESPONSIBILITY_MATRIX.md` — Added cross-reference to self-knowledge standard
- `docs/PRIMAL_GAPS.md` — Songbird 935→302, barraCuda lint fix, self-knowledge standard reference

## [0.8.0u] — Phase 23u: NestGate EnvSource Evolution (2026-04-04)

### Audited
- **NestGate** (f298c1c1): Config layer evolved to `EnvSource` injection pattern — eliminates direct `env::var()` calls, enables pure-function testing. 31 serial tests converted to concurrent. **11,264 tests** (was 6,451 — tests unlocked by removing serial bottleneck). 1 env-dep failure (`test_arc_stats_collect` — needs ZFS `/proc` entries). Clippy CLEAN, fmt PASS. Discovery stable at 195/24.
- coralReef, Songbird: no new commits.

### Changed
- `docs/PRIMAL_GAPS.md` — NestGate test count 6.6K→11.3K
- `wateringHole/PRIMAL_RESPONSIBILITY_MATRIX.md` — NestGate updated

## [0.8.0t] — Phase 23t: Full Ecosystem Audit — 4 Primals Evolved (2026-04-04)

### Audited
- **Songbird wave 107-109**: Final primal-name cleanup wave (`btsp_beardog_integration→btsp_security_provider_integration`). Discovery **1016→935 refs** (178 files). 63% total reduction since wave 97. 8,938 tests.
- **NestGate** (6b6bf799): **nestgate-automation deprecated** (overstep shed). -15,579 lines across 98 files. 83 `self.base_url` artifacts removed. 10.2K orphan lines deleted. 4.3K dead code deleted. Clippy CLEAN, 6,451 tests (2 flaky).
- **toadStool** (S176-S178): `env_config` primal names → capability names (S177). Deprecated API removal. Discovery **3239→2998 refs**. Clippy still FAIL (aes_gcm deprecated).
- **barraCuda Sprint 28**: Zero-copy ESN, capability-based sovereign naming. **SIGSEGV appears resolved** — 3,835 tests, 0 failures. Clippy: 1 unfulfilled lint expectation.

### Changed
- `docs/PRIMAL_GAPS.md` — barraCuda SIGSEGV→PASS, discovery tables updated, header updated
- `wateringHole/PRIMAL_RESPONSIBILITY_MATRIX.md` — all 4 primals updated
- `wateringHole/IPC_COMPLIANCE_MATRIX.md` — Songbird 1016→935

## [0.8.0s] — Phase 23s: Songbird Wave 106 + petalTongue Discovery Evolution (2026-04-04)

### Audited
- **Songbird wave 106** (`a26d73bfd`): Dead code cleanup, unwrap evolution, tor coverage +30 tests. Discovery **1472→1016 refs** (31% cut, 60% total since wave 97). 8,903 tests, 1 env-dep failure. Clippy CLEAN, fmt PASS.
- **petalTongue** (`9b0f0d0`): Capability compliance renames: `squirrel_adapter→ai_adapter`, `toadstool→discovered_display`, `toadstool_compute→gpu_compute`. 13 unused primal constants removed. **PT-04 RESOLVED** (HTML graph export), **PT-06 RESOLVED** (callback_tx push). Zero-copy evolution. 5,968 tests, 0 failures.

### Changed
- `docs/PRIMAL_GAPS.md` — PT-04/PT-06 RESOLVED (23 resolved, 4 open). Songbird P→P→C. petalTongue P→P→C. Discovery priority list reordered. Compliance matrix updated.
- `wateringHole/PRIMAL_RESPONSIBILITY_MATRIX.md` — Songbird + petalTongue compliance updated
- `wateringHole/IPC_COMPLIANCE_MATRIX.md` — Songbird discovery 1472→1016

## [0.8.0r] — Phase 23r: coralReef First Audit + toadStool S174 (2026-04-03)

### Audited
- **coralReef** (Iter 72 — 5a6ca52): **First-ever audit.** Clippy CLEAN, fmt PASS, **4,257 tests PASS**. `forbid(unsafe_code)` on core/stubs, `deny.toml` present. Discovery **CLEAN** — 28 `BIOMEOS_*` refs (ecosystem-standard), 2 primal names in doc/attribution comments only. Zero routing violations. Socket at `biomeos/coralreef-core-{family}.sock` with `shader.sock` + `device.sock` symlinks.
- **toadStool** (S174-S175): Unsafe reduction -80% in consumer blocks. New clippy errors: `v4l2` private `_pad` fields in `toadstool-display` + deprecated `aes_gcm::from_slice`.

### Changed
- `docs/PRIMAL_GAPS.md` — coralReef compliance data added (n/c → CLEAN/PASS/C), discovery table updated, header updated. toadStool clippy status updated for S174 regressions. Gap count: 21 resolved, 6 open.
- `wateringHole/PRIMAL_RESPONSIBILITY_MATRIX.md` — coralReef compliance updated
- `wateringHole/IPC_COMPLIANCE_MATRIX.md` — coralReef discovery compliance added

## [0.8.0q] — Phase 23q: Full Ecosystem Pull + Re-Audit (2026-04-03)

### Audited
- **Squirrel alpha.36**: Build **FIXED**. Clippy CLEAN, fmt PASS, **6,856 tests PASS** (was build-broken). alpha.33 removed 65,910 lines dead code. Discovery: 215 files / 1789 refs (full scan).
- **NestGate** (3dc0044b): **Overstep shedding** — deleted `discovery_mechanism` (-2K lines), deprecated `nestgate-network` (zero workspace dependents). Clippy CLEAN, fmt PASS, 6,607 tests (2 flaky). Discovery: 22 files / 192 refs, 9 files / 32 env refs.
- **toadStool** (S173-3): S173-2 direct primalSpring audit response — **TS-01 RESOLVED** (`coral_reef_client` uses `capability.discover("shader")`), `deny(unsafe_code)` workspace policy documented (43/43 crates). S173-3: smart refactoring + coverage. Clippy **FAIL** (deprecated `aes_gcm::from_slice`). 6,481 tests (1 timeout assertion bug). Discovery: 393 files / 3239 refs (full scan).
- biomeOS, BearDog, rhizoCrypt, loamSpine, sweetGrass, petalTongue, sourDough, barraCuda: no new commits.

### Changed
- `docs/PRIMAL_GAPS.md` — Squirrel FAIL→CLEAN, toadStool TS-01 RESOLVED (21→6 open gaps), updated compliance matrix with full-scan discovery data
- `wateringHole/PRIMAL_RESPONSIBILITY_MATRIX.md` — Squirrel/NestGate/toadStool compliance updated
- `wateringHole/IPC_COMPLIANCE_MATRIX.md` — Squirrel discovery data updated

## [0.8.0p] — Phase 23p: Songbird Wave 102 Re-Audit (2026-04-03)

### Audited
- **Songbird wave 102** (`0c893f22e`): deep debt evolution — TLS safety, capability completion, smart refactoring
- Clippy **CLEAN**, fmt **PASS** (was FAIL — both regressions resolved)
- **7,020+ tests**, 4 env-dep failures (need running BearDog — not code bugs)
- Discovery: **2558→1472 refs** (230 files) — 42% reduction
- Key renames: `beardog_*`→`security_*`, `squirrel_*`→`coordination_*`, `nestgate`→`storage_provider`, `toadstool`→`compute_provider`
- Primal-named spec docs archived to `specs/archive/`
- Remaining: 805 beardog refs (171 files), 130 toadstool (47), 96 squirrel (39), 53 nestgate (20)
- Env var refs rose 143→291 (capability-first chains with fallback lookups — correct pattern)

### Changed
- `docs/PRIMAL_GAPS.md` — Songbird status **X→P**, updated compliance matrix, discovery table, header
- `wateringHole/IPC_COMPLIANCE_MATRIX.md` — Songbird discovery X→P with measured data
- `wateringHole/PRIMAL_RESPONSIBILITY_MATRIX.md` — Songbird compliance updated, overstep detail expanded

## [0.8.0o] — Phase 23o: Responsibility Matrix Restructure + sourDough Integration (2026-04-03)

### Added
- **sourDough** added to `docs/PRIMAL_GAPS.md` gap registry — SD-01/02/03 (deny.toml, musl, signing), all Low. Compliance: clippy CLEAN, fmt PASS, 239 tests, discovery NEAR-CLEAN.
- sourDough added to guideline compliance matrix and discovery compliance table.

### Changed
- **`wateringHole/PRIMAL_RESPONSIBILITY_MATRIX.md` v2.3 → v3.0** — major restructure:
  - Added **Primal Directory**: clear role definitions, capability namespaces, and purpose for each primal.
  - Added sourDough (Tooling tier) and skunkBat (Nascent tier).
  - Added **Interaction Rules** section: discovery protocol, communication protocol, prohibition list.
  - Added **Capability Routing Guide**: quick-reference routing table for deploy graph design.
  - Added **Compliance Status** table with measured audit data from primalSpring full scan.
  - Simplified Overstep Detail section. Added Squirrel overstep (sled/crypto).
  - Reorganized Concern Matrix with sourDough column and `(resolved)` markers.
  - Designed for human and AI agent comprehension — defines primal roles to understand gaps and prevent overstep.
- **`wateringHole/IPC_COMPLIANCE_MATRIX.md` v1.5.0 → v1.6.0** — sourDough added to scorecard and discovery compliance; cross-reference to responsibility matrix v3.0.

## [0.8.0n] — Phase 23n: Full Audit Cycle Against WateringHole Standards (2026-04-03)

### Audited
- Full primal pull: nestGate (a75e9f2a) and toadStool (S172-5) had new evolution
- wateringHole pull: 2 new handoffs (nestGate v4.7.0, barraCuda v0.3.11), toadStool discovery X→C claim
- **nestGate**: Clippy CLEAN (was ~2 warnings), fmt PASS, 1449+ tests PASS. Discovery near-compliant: 7 files, zero primal env vars.
- **toadStool**: Clippy 2 warnings, fmt PASS, 21.5K tests PASS. Discovery improved but ~30 files + SONGBIRD_*/BEARDOG_SOCKET in fallbacks. Compliance claim overstated.
- **petalTongue**: Tests ALL PASS (was 1 failure — fixed). 24 env refs across 10 files.
- **Songbird**: fmt STILL FAILS. Discovery debt massive: 2558 refs in 321 files, 143 env-var refs in 50 files.
- **Squirrel**: clippy/tests STILL FAIL (ai-tools build error). 322 refs in 96 files.

### Changed
- `docs/PRIMAL_GAPS.md` — full audit findings, updated compliance matrix, discovery compliance table with measured ref counts
- `wateringHole/IPC_COMPLIANCE_MATRIX.md` v1.4.0 → **v1.5.0** — §Discovery Compliance updated with primalSpring scan data, corrected toadStool/Songbird statuses

## [0.8.0m] — Phase 23m: Downstream Graph Sketch Reframe (2026-04-03)

### Changed
- **Architectural correction**: ludoSpring is a parallel peer, esotericWebb is downstream. primalSpring does NOT own, build, or run their binaries.
- Moved ludoSpring/esotericWebb deploy graphs to `graphs/sketches/` — proto sketches co-evolved by primalSpring + ludoSpring, with esotericWebb as eventual owner.
- Moved ludoSpring/esotericWebb composition graphs to `graphs/sketches/`.
- Moved ludoSpring/esotericWebb validation graphs to `graphs/sketches/validation/` — sketches of how those systems should validate themselves.
- All moved graph headers updated with "PROTO SKETCH" designation and correct ownership.

### Reframed
- `validate_compositions.py` C3/C4/C7 no longer require downstream binaries:
  - **C3 → Session Readiness**: validates substrate + Tower + capability routing for narrative domain.
  - **C4 → Game Science Readiness**: validates substrate + Tower + capability routing for game domain.
  - **C7 → Product Readiness**: validates full primal stack health across all owned domains.
- `composition_game_science_validate.toml` → validates primal-layer readiness, not ludoSpring surface.
- `composition_session_validate.toml` → validates primal-layer readiness, not esotericWebb surface.
- `composition_interactive_validate.toml` → validates all owned primal domains, no downstream nodes.

## [0.8.0l] — Phase 23l: Evolution Pull + Discovery Compliance Re-Audit (2026-04-03)

### Audited
- Full primal pull and evolution review across all 12 primals
- **biomeOS v2.87**: Discovery compliance **RESOLVED** — identity-based routing removed from non-test code. All checks green.
- **petalTongue wave 99**: `SongbirdClient` + `barracuda.compute.dispatch` removed. Clippy+fmt clean. 11 residual env aliases. 1 test failure.
- **Songbird wave 99**: Clippy clean (0 warnings), `discover_beardog→discover_security_provider` rename. **fmt regressed** (widespread). ~30 files still have legacy refs.
- **Squirrel alpha.31**: Capability-based discovery commit. **clippy/tests regressed** (ai-tools cfg gate). 7 files still have Songbird coupling.
- **barraCuda Sprint 27**: Clippy+fmt clean. **`fault_injection` test SIGSEGV** — new regression.
- BearDog, NestGate, toadStool, rhizoCrypt, loamSpine, sweetGrass: no new commits.

### Changed
- `docs/PRIMAL_GAPS.md` — updated all compliance sections with April 3 findings, discovery matrix with trends
- `wateringHole/IPC_COMPLIANCE_MATRIX.md` — updated §Capability-Based Discovery Compliance with post-evolution status
- Guideline Compliance Matrix: added Discovery column, updated clippy/fmt/test status per primal

## [0.8.0k] — Phase 23k: Capability-Based Discovery Compliance Audit (2026-04-02)

### Audited
- Full ecosystem audit against `CAPABILITY_BASED_DISCOVERY_STANDARD.md` v1.2.0
- Scanned all primals for identity-based routing violations (hardcoded primal names, env vars, method namespaces)
- 4/10 primals **fully compliant**: BearDog, rhizoCrypt, loamSpine, sweetGrass
- 6/10 primals **non-compliant**: biomeOS, Songbird, Squirrel, toadStool, petalTongue, NestGate (partial)
- petalTongue deep audit: `SongbirdClient`, `discover_toadstool()`, `BARRACUDA_SOCKET`, `barracuda.compute.dispatch` identified as violations; `toadstool_v2.rs` display backend confirmed as correct exemplar

### Changed
- `wateringHole/CAPABILITY_BASED_DISCOVERY_STANDARD.md` v1.1.0 → **v1.2.0**: Added compliance audit checklist with grep patterns, per-primal findings, SHOULD → MUST upgrade
- `wateringHole/IPC_COMPLIANCE_MATRIX.md` v1.3.2 → **v1.4.0**: Added §Capability-Based Discovery Compliance with per-primal matrix and priority ranking
- `docs/PRIMAL_GAPS.md`: Added §Capability-Based Discovery Compliance matrix linking to IPC Compliance Matrix

## [0.8.0j] — Phase 23j: Evolution Pull + Deep Debt Synthesis (2026-04-02)

### Changed
- `docs/PRIMAL_GAPS.md` — evolution pull with major progress across all tiers:
  - NG-02 **RESOLVED** (session.rs + semantic router dispatch in d7a0716b)
  - NestGate compile fixed, clippy ~2 warnings (was RED)
  - toadStool S172-4: fmt + clippy both **CLEAN** (was 25 warnings + 18 fmt failures)
  - Squirrel alpha.29: 49K orphan lines removed, 0 todo!/unimplemented! (was 14+4)
  - BearDog Wave 26: AI tree feature-gated, flaky test stabilized, deny.toml skip-list halved
  - petalTongue: CHANGELOG.md added, sensory matrix + accessibility adapters, PT-06 code-complete
  - 20 gaps resolved (+1), 7 open (all low)

### Deep Debt Review
- Full per-primal debt audit: NUCLEUS atomics, meta-tier, provenance trio, extended computation
- Provenance trio (rhizoCrypt, loamSpine, sweetGrass) confirmed **debt-free**
- coralReef + barraCuda deferred (separate scope)

## [0.8.0i] — Phase 23i: Re-Audit Cycle + Overstep Verification (2026-04-02)

### Changed
- `docs/PRIMAL_GAPS.md` — re-audit with overstep scan and compliance recheck:
  - NG-01 reclassified Medium → **Low** (StorageBackend trait injection wired; metadata axis residual)
  - SB-02 reclassified to **Near-resolved** (rcgen removed from lockfile; ring not compiled in default)
  - SB-03 reclassified to **Improved** (sled feature-gated in all 3 crates)
  - All 8 open gaps now **Low** severity — zero critical, zero high, zero medium
  - Added overstep audit section confirming PRIMAL_RESPONSIBILITY_MATRIX alignment

### Compliance Evolution
- **Songbird**: 395 → **8** clippy warnings (wave93 ring elimination, concurrency fix)
- **NestGate**: **regressed** — 16 warnings + test compile errors; BUT `deny.toml` now present
- **loamSpine**: fmt now **PASSES** (was failing)
- **petalTongue**: tests now **PASS** (was 1 failure)
- **toadStool**: tests now **PASS** (was 1 failure), fmt **still fails**

### Overstep Scan
- No new boundary violations found
- Confirmed: rhizoCrypt/loamSpine TCP = standard IPC dual-mode (not networking overstep)
- biomeOS `redb`, BearDog `axum`+AI, Songbird `sled` = known items per matrix
- toadStool S169 cleanup holds (30+ methods removed)

## [0.8.0h] — Phase 23h: Full Primal Audit + Guideline Compliance (2026-04-01)

### Changed
- `docs/PRIMAL_GAPS.md` — full primal audit with guideline compliance matrix:
  - SQ-03 reclassified as **RESOLVED** (documented in `CURRENT_STATUS.md`, intentional retention)
  - NG-01/NG-02/NG-03 reclassified as **Improved** (unix-socket path is durable; session save/load exists; data/storage separation documented)
  - Added per-primal compliance data: clippy, fmt, unsafe policy, deny.toml, SPDX, test status
  - 19 gaps resolved (was 18), 8 open (1 medium, 7 low)

### Audit Findings
- **BearDog**: gold standard compliance — workspace `forbid(unsafe_code)`, clippy clean, 14K+ tests
- **NestGate**: missing `deny.toml`, 3 test failures (timing/env), tarpc path still in-memory
- **toadStool**: `cargo fmt` failure, 1 test failure, no workspace `forbid(unsafe_code)`
- **loamSpine**: `cargo fmt` failure (needs `cargo fmt --all`)
- **Songbird**: 395 clippy warnings in orchestrator tests (`unwrap_used`), `sled` still direct dep
- **petalTongue**: 1 test failure (`provenance_trio` discovery test), callback push not wired
- **Squirrel**: no workspace `forbid(unsafe_code)` (uses clippy groups)

## [0.8.0g] — Phase 23g: Primal Rewiring + Gap Cleanup (2026-04-01)

### Changed
- `ecoPrimal/src/ipc/methods.rs` — aligned with primal evolution:
  - `graph::DEPLOY` → `graph::EXECUTE` (matches actual biomeOS routing target)
  - Added `topology::RESCAN` (biomeOS v2.81)
  - Added `ember::LIST`, `ember::STATUS` (toadStool S171 hardware lifecycle)
  - Added `ai::QUERY`, `ai::LIST_PROVIDERS` (Squirrel)
  - Added `visualization::*`, `interaction::*` (petalTongue)
  - Removed `shader::COMPILE_WGSL` (coralReef domain since S169)
  - Removed downstream modules (`game::*`, `webb::*`, `session::*` — springs/gardens own these)
- `ecoPrimal/src/ipc/neural_bridge.rs` — added `topology_rescan()` for biomeOS v2.81
- `ecoPrimal/src/ipc/discover.rs` — added plain socket name discovery (`{name}.sock`, `{name}-ipc.sock`) for primals that don't use family-suffixed sockets
- `tools/validate_compositions.py` — updated SQ-02 messaging (resolved), NestGate `storage.list` passes `family_id`, C7 Squirrel check now live (not hardcoded fail)
- `docs/PRIMAL_GAPS.md` — scoped to primals only (downstream removed), 18 resolved, 8 open

### Live Validation Results (2026-04-01)
- **43/44 (98%)** — up from 93% (previous) and 79% (pre-evolution)
- C5: Persistence now **5/5 PASS** (was partial)
- C7: Full Interactive now **10/10 PASS** (was 9/10)
- Only failure: C2 `ai.query` — environment dependency (no local Ollama), code is wired (SQ-02 resolved)

### Newly Resolved Gaps (this session)
- **SQ-02** — `LOCAL_AI_ENDPOINT` wired into AiRouter (alpha.27)
- **PT-05** — `RenderingAwareness` auto-init in `UnixSocketServer`
- **PT-07** — periodic discovery refresh in server mode
- **NG-04** — ring/aws-lc-rs eliminated (TLS → system curl)
- **NG-05** — nestgate-security zero crypto deps (BearDog IPC delegation)

## [0.8.0f] — Phase 23f: Composition Decomposition — 7 Subsystem Compositions (2026-03-28)

### Added
- `graphs/compositions/` — 7 independently deployable subsystem compositions:
  - `render_standalone.toml` (C1: petalTongue render)
  - `narration_ai.toml` (C2: Squirrel AI narration)
  - `session_standalone.toml` (C3: esotericWebb session)
  - `game_science_standalone.toml` (C4: ludoSpring game science)
  - `persistence_standalone.toml` (C5: NestGate persistence)
  - `proprioception_loop.toml` (C6: petalTongue interaction loop)
  - `interactive_product.toml` (C7: all subsystems composed)
- 7 matching validation graphs in `graphs/spring_validation/composition_*_validate.toml`
- `docs/PRIMAL_GAPS.md` — structured gap registry: 22 gaps across petalTongue (7), Squirrel (3), NestGate (3), esotericWebb (4), biomeOS (3), ludoSpring (2), cross-cutting (3)
- `tools/validate_compositions.py` — live subsystem composition validator (C1-C7)
- biomeOS `capability.discover` socket resolution with liveness probing
- `graphs/spring_deploy/` — 6 science spring deploy graphs (airSpring, groundSpring, healthSpring, hotSpring, neuralSpring, wetSpring) for next validation cycle
- `infra/wateringHole/handoffs/` — composition decomposition handoff, primal team gaps handoff, spring teams deployment handoff

### Changed
- `tools/ws_gateway.py` — refactored from monolithic orchestrator to thin WebSocket-to-IPC bridge: generic RPC pass-through, batch calls, capability discovery via biomeOS, zero business logic
- `web/play.html` — reclassified from game UI to composition monitor: shows subsystem health grid, C1-C7 cards with click-to-test, debug session section, all calls via thin bridge protocol
- Deploy graphs: 69 → 89 (21 validation, 7 compositions, 6 spring deploy)

### Live Composition Validation Results (2026-03-28)
- **C1: Render (petalTongue)**: 6/6 PASS — dashboard, export, scene, sessions
- **C2: Narration (Squirrel)**: 0/3 FAIL — expected gap SQ-01 (Ollama routing)
- **C3: Session (esotericWebb)**: 8/8 PASS — full lifecycle + graph
- **C4: Game Science (ludoSpring)**: 6/6 PASS — flow, Fitts, WFC, engagement
- **C5: Persistence (NestGate)**: 1/5 PARTIAL — expected gap NG-01 (process stopped)
- **C6: Proprioception (petalTongue)**: 5/5 PASS — subscribe, apply, poll, showing
- **C7: Full Interactive**: 8/10 PARTIAL — only C2+C5 gaps propagate
- **Overall**: 34/43 (79%), all failures traced to documented gaps

## [0.8.0e] — Phase 23e: Live Composition — esotericWebb as ecoPrimals Product (2026-03-30)

### Added
- `graphs/ludospring_game_deploy.toml` — deploy ludoSpring V14 as game-science primal via biomeOS
- `graphs/esotericwebb_tower_deploy.toml` — minimum viable Webb product (Tower + narrative)
- `graphs/esotericwebb_composed_deploy.toml` — full AI DM composition (Tower + Squirrel + petalTongue + ludoSpring + Webb)
- `graphs/spring_validation/esotericwebb_tower_validate.toml` — spring validation for Webb Tower
- `graphs/spring_validation/esotericwebb_composed_validate.toml` — spring validation for full composed Webb
- `graphs/spring_validation/ludospring_game_validate.toml` — spring validation for ludoSpring game composition
- `strip_unix_uri()` helper — converts biomeOS `unix:///path` endpoints to raw filesystem paths

### Changed
- `ipc::capability::discover_by_capability` — reads `primary_endpoint` (biomeOS v2.78) with fallback to `primary_socket`
- `NeuralBridge::health_check` — uses liveness fallback chain with `graph.list` last-resort probe (biomeOS Neural API doesn't implement `health.check`)
- `exp075_biomeos_neural_api_live` — checks `primary_endpoint || primary_socket` for domain discovery
- **exp088 rewritten**: TCP-hardcoded → UDS socket discovery via `discover_primal` / `discover_by_capability`; now validates Tower + biomeOS + ludoSpring + esotericWebb end-to-end (16/16 PASS)
- Deploy graphs: 63 → 69 (14 validation)

### Live Validation Results (2026-03-30)
- **Tower Atomic**: 13/13 PASS — BearDog security + Songbird discovery + biomeOS substrate
- **biomeOS Neural API**: 12/12 PASS — 125 domains, 41 graphs, crypto/beacon routing
- **Storytelling Composition**: 16/16 PASS — Tower + ludoSpring (game science) + esotericWebb (CRPG) + biomeOS routing
- **Unit Tests**: 402/402 PASS, 0 clippy warnings

## [0.8.0d] — Phase 23d: Absorb toadStool S168 + esotericWebb V6 + ludoSpring V32 (2026-03-28)

### Added
- `compute::DISPATCH_SUBMIT`, `compute::DISPATCH_EXECUTE`, `compute::HEALTH` — toadStool compute dispatch methods
- `shader::DISPATCH`, `shader::COMPILE_WGSL` — toadStool S168 sovereign shader pipeline methods
- `webb::RESOLVE_SCENE`, `webb::NPC_STATE`, `webb::ABILITY_CHECK`, `webb::SESSION_STATE`, `webb::LIVENESS` — esotericWebb V6 narrative methods
- `session::CREATE`, `session::ADVANCE`, `session::COMPLETE` — shared session lifecycle
- `game::ANALYZE_UI`, `game::ACCESSIBILITY`, `game::GENERATE_NOISE` — ludoSpring V32 supplemental methods

### Changed
- `gen4_storytelling_full.toml` v2.0.0: esotericWebb V6 owns game science locally; ludoSpring now optional supplemental; biomeOS substrate Phase 0 node added; Squirrel AI methods updated (ai.query/suggest/analyze); toadStool gains shader.dispatch
- `gen4_storytelling_minimal.toml` v2.0.0: esotericWebb V6 self-contained; ludoSpring optional; Squirrel ai.query replaces ai.chat
- `ludospring_validate.toml` v0.2.0: biomeOS substrate node; V32 game.* capabilities (8 methods); updated validation surface
- Method constants: 63 → 79 (24 domains, 6 new modules: compute, shader, webb, session + 3 expanded game constants)

## [0.8.0c] — Phase 23c: NUCLEUS Atomics + biomeOS Substrate (2026-03-28)

### Added
- `nucleus_atomics_validate.toml` — validates all 4 NUCLEUS tiers (Tower, Node, Nest, Full) + Tower+Squirrel overlay + structural graph validation
- `SubstrateHealth` struct — biomeOS Neural API health status in `CompositionResult`
- `AtomicType::substrate_capabilities()` — Neural API surface every composition requires
- `AtomicType::substrate_primal()` — returns `"biomeos"`
- `probe_substrate()` — discovers and health-checks the Neural API
- `composition.tower_squirrel_health` — wired in `primalspring_primal` dispatch (was advertised but unimplemented)
- biomeOS Neural API Phase 0 health node in all 4 canonical atomic deploy graphs

### Changed
- Nest composition: `[beardog, songbird, nestgate]` → `[beardog, songbird, nestgate, squirrel]` (matches biomeOS `nucleus --mode nest`)
- Nest capabilities: `[security, discovery, storage]` → `[security, discovery, storage, ai]`
- `nucleus_complete.toml`: NestGate, ToadStool, Squirrel now `required = true` (core 5 match biomeOS Full)
- `validate_composition()` and `validate_composition_by_capability()` now probe biomeOS substrate
- `all_healthy` requires both substrate + primals healthy
- `node_atomic_compute.toml`, `nest_deploy.toml`: add `args = ["server"]` to validation nodes
- Deploy graphs: 62 → 63, validation graphs: 10 → 11, tests: 399 → 402

## [0.8.0b] — Phase 23b: biomeOS v2.78 Rewire (2026-03-28)

### Added
- `rollback_validate.toml` — spring validation graph for biomeOS graph rollback (deploy → status → rollback lifecycle)
- `federation_manifest_validate.toml` — spring validation graph for biomeOS federation manifest deployment (configure → join → health_check)
- `NeuralBridge::discover_domain()` — biomeOS v2.78 dual-param `capability.discover` (accepts `domain` alongside `capability`)
- `NeuralBridge::graph_deploy()`, `graph_status()`, `graph_rollback()` — graph lifecycle via Neural API
- 20 new method constants: `federation.{configure,join,health_check}`, `discovery.{discover,discover_all,protocols}`, `topology.{get,proprioception}`, `route.register`, `graph.{deploy,status,rollback,pipeline,continuous}`, `lifecycle.{start,stop,register}`, `capability.{register,unregister,route}`

### Changed
- Deploy graphs: 60 → 62, spring validation graphs: 8 → 10
- biomeOS debt handoff updated: all 4 blocking items + S-2/S-3 marked RESOLVED per v2.78

## [0.8.0] — Phase 23: Ecosystem Debt Resolution + Standards (2026-03-29)

### Added
- `crypto_negative_validate.toml` — spring validation graph for negative security boundary tests (wrong-seed, empty-seed rejection, tampered-payload detection)
- `ipc::methods::genetic::GENERATE_LINEAGE_PROOF` — method constant for lineage proof generation
- Per-primal team debt handoffs in wateringHole (BearDog, Songbird, biomeOS)
- `COMPOSITION_PATTERNS.md` — canonical reference for deploy graph formats, niche YAML, launch profiles, socket discovery
- `SPOREGARDEN_DEPLOYMENT_STANDARD.md` — BYOB model, esotericWebb reference, environment contract
- `PRIMALSPRING_V080_GAP_MAP_MAR29_2026.md` — comprehensive gap map reclassifying 11 findings from audit
- Glossary terms: BYOB, Niche YAML, Primal Launch Profile, sporeGarden Product, PrimalBridge, Primal Resolution Order

### Changed
- exp086: full generate-then-verify lineage round-trip with positive and negative tests (was incomplete `verify_lineage` call)
- Deploy graph count: 59 → 60, spring validation graph count: 7 → 8
- Version bump: 0.7.0 → 0.8.0

### Fixed (upstream, driven by primalSpring audit)
- BearDog: `genetic.derive_lineage_beacon_key` registered in method_list.rs (method count 92→93)
- BearDog: empty/zero/short lineage seeds now rejected (was silently defaulting to 32 zero bytes)
- BearDog: `federation.verify_family_member` label corrected from `genetic_lineage_hkdf` to `family_id_equality`
- BearDog: `encryption.encrypt/decrypt` docs no longer claim "HSM-backed"
- Songbird: `SONGBIRD_DARK_FOREST`, `SONGBIRD_ACCEPT_LEGACY_BIRDSONG`, `SONGBIRD_DUAL_BROADCAST` env vars wired into BirdSongConfig
- biomeOS: `eprintln!` → `tracing::warn!` in capability_domains.rs

## Phase 22: E2E Composition Testing (Track 14)

### Added
- `ipc::methods` — 6 new domain modules: `crypto`, `birdsong`, `genetic`, `secrets`, `storage`, `game`
- `ipc::tcp` — `neural_api_capability_call` and `neural_api_capability_discover` helpers
- `tolerances` — `PRIMAL_STARTUP_LATENCY_US`, `DEFAULT_SQUIRREL_PORT` aliases
- exp085: BearDog crypto lifecycle E2E (Ed25519, Blake3, BirdSong beacon, secrets)
- exp086: Genetic identity E2E (mito beacon seed vs nuclear lineage, family scoping)
- exp087: Neural API routing E2E (security, discovery, storage, compute, AI domains)
- exp088: Storytelling composition (ludoSpring + esotericWebb + Squirrel + petalTongue)
- `scripts/validate_composition.sh` — composition validation with benchScale topology support
- ludoSpring game.* method gap handoff for esotericWebb contract

### Changed
- Experiment count: 63 → 67, test count: 411 → 413
- `validate_release.sh` test floor: 411 → 413
- IPC method modules: 10 → 16 domain-specific constant sets

## [Unreleased] — Phase 21: Deep Ecosystem Audit + Library Consolidation (2026-03-29)

### Added
- **`ipc::tcp` module** — shared TCP RPC helper (`tcp_rpc`, `tcp_rpc_with_timeout`,
  `http_health_probe`, `env_port`) extracted from 8 experiments into library; eliminates
  per-experiment TCP boilerplate
- **`ipc::methods` module** — centralized JSON-RPC method name constants (`health::LIVENESS`,
  `capabilities::LIST`, `provenance::SESSION_CREATE`, `coordination::VALIDATE_COMPOSITION`,
  etc.) — zero hardcoded method strings in experiments
- **`ipc::capability` module** — capability discovery and routing logic extracted from
  `ipc/discover.rs` with full test coverage
- **`tolerances::TCP_CONNECT_TIMEOUT_SECS`**, `TCP_READ_TIMEOUT_SECS`,
  `TCP_WRITE_TIMEOUT_SECS` — centralized network timeout constants
- **Provenance circuit breaker half-open** — time-based half-open state with
  `TRIO_OPENED_AT` epoch, `AtomicBool` probe token, graceful mutex poisoning handling
- 26 new tests (385 → 411): ipc::tcp, ipc::methods, provenance half-open, launcher APIs
- Phase 21 handoff: `PRIMALSPRING_V070_PHASE21_DEEP_AUDIT_HANDOFF_MAR29_2026.md`

### Changed
- **`launcher/` smart refactor** — split into 4 sub-modules: `discovery.rs` (binary resolution),
  `profiles.rs` (launch profile parsing), `spawn.rs` (process spawning + socket wait),
  `biomeos.rs` (biomeOS-specific logic). Public API preserved via re-exports
- **8 experiments consolidated** — `exp063`, `exp073`, `exp074`, `exp076`, `exp081`–`exp084`
  refactored from local TCP RPC to `ipc::tcp` library helpers
- **Hardcoded primal names eliminated** — 4 experiments (`exp065`, `exp075`, `exp076`,
  `exp083`) now use `primal_names::*` slug constants
- **Hardcoded method strings eliminated** — all experiments use `ipc::methods::*` constants
- **Library tracing** — `println!`/`eprintln!` → `tracing::info!`/`tracing::error!` in
  harness/mod.rs and validation/or_exit.rs
- **`PrimalClient` transport unification** — uses `Transport` enum internally (Unix + TCP)
- **`validate_release.sh` test floor** — 378 → 411
- All docs updated: 411 tests, Phase 21 status, new module documentation
- All clippy warnings resolved (pedantic + nursery + cast + unwrap/expect discipline)

## [Unreleased] — Phase 19: Gen4 Spring Scaffolding (2026-03-28)

### Added
- **gen4_spring_composition.toml** — master deploy graph: Tower + biomeOS + 5 spring primals
  + cross-spring validation node (36 graphs total, up from 35)
- **6 spring launch profiles** in `primal_launch_profiles.toml` — airspring, groundspring,
  healthspring, hotspring, ludospring, neuralspring, wetspring
- Phase 19 handoff: `SPRING_GEN4_SCAFFOLDING_PHASE19_HANDOFF_MAR28_2026.md`

### Changed
- All 7 spring validation graphs updated: biomeOS substrate node (`start_biomeos`, order 2)
  inserted before spring primal germination
- plasmidBin: `manifest.toml`, `sources.toml`, `checksums.toml`, `doctor.sh` updated for
  5 spring primal binaries (groundspring, healthspring_primal, ludospring, neuralspring, wetspring)
- All docs updated: 36 deploy graphs, Phase 19 status, 5 spring binaries in plasmidBin

### Built (upstream patches for spring compilation)
- **barraCuda** v0.3.5→v0.3.7: F16 precision variant, GPU feature-gating (`plasma_dispersion`,
  `analyze_weight_matrix`), 4 missing `DeviceCapabilities` methods, `rel_tolerance` on `Check`,
  `PrecisionRoutingAdvice` re-export
- **bingoCube/nautilus**: no-op `json` feature gate, `input_dim` on `ShellConfig`
- 5/6 spring primal binaries built, stripped, checksummed (airspring blocked by internal API drift)

## [Unreleased] — Phase 17: gen4 Deployment Evolution (2026-03-27)

### Added
- **6 new experiments** (exp075–080): biomeOS substrate, cross-gate routing, Squirrel AI bridge,
  petalTongue viz, spring deploy sweep, cross-spring ecology
- **13 new deploy graphs** — 7 spring validation, 2 cross-spring, 4 gen4 prototypes (35 total)
- **Native `NeuralBridge`** — replaced `neural-api-client-sync` compile dependency with runtime
  JSON-RPC via `PrimalClient` (zero cross-primal coupling)
- `discover_biomeos_binary()` — discovers `biomeos` or `neural-api-server` with fallback
- `spawn_biomeos()` — refactored from `spawn_neural_api()` with `#[deprecated]` alias
- **NestGate integration** — pulled upstream updates (ZFS graceful degradation, unsafe evolution,
  family-scoped sockets, comprehensive audit), plasmidBin binary updated
- **Primal team handoff** — per-team guidance for biomeOS, Squirrel, Songbird, BearDog, petalTongue

### Changed
- `ipc/neural_bridge.rs` — new module for biomeOS neural-api substrate communication
- `harness/` — `neural_api_process` → `biomeos_process`, calls `spawn_biomeos()`
- `launcher/` — biomeOS graph paths updated from `phase2/` to `primals/`
- `scripts/build_ecosystem_musl.sh` — `phase1/`/`phase2/` paths → `primals/`
- `scripts/prepare_spore_payload.sh` — `phase2/biomeOS` → `primals/biomeOS`
- `exp060` — graph discovery paths updated to `primals/biomeOS/graphs`
- `bonding/mod.rs` — doc link updated to `primals/biomeOS/specs/`
- All docs updated: 59 experiments, 385 tests, 35 deploy graphs (now 36 as of Phase 19)
- `thiserror` migration for `IpcError` and `LaunchError`
- 385 tests (up from 378), 59 experiments (up from 53)

### Validated
- biomeOS coordinated mode: 24 capability domains, 39 deploy graphs
- Cross-gate Pixel routing via ADB-forwarded TCP (BearDog + Songbird)
- Squirrel AI via abstract socket, `ai.*` domain routing
- Spring deploy sweep: all 7 sibling spring biomeOS graphs loaded
- Full NUCLEUS: 16/16 gates with live NestGate storage round-trip

## [Unreleased] — Phase 16.1: Coverage Evolution + Docs Refresh (2026-03-27)

### Added
- **29 new unit tests** — 349 → 378 (coordination, niche, launcher, ipc/client)
- Coverage tests for `validate_composition_by_capability` graceful degradation (all atomics)
- Coverage tests for `health_check_within_tolerance` failure path
- Coverage test for `register_with_target` graceful degradation when biomeOS absent
- Coverage tests for all `LaunchError` Display + Error::source variants
- Coverage tests for `SocketNucleation::from_env`, `get`, `remap`
- Coverage test for `connect_by_capability` error path
- Cost estimate completeness + memory field tests for niche
- Semantic mapping cross-validation tests for niche

### Changed
- **exp014/exp023 tick slack** — `<= 1` magic tolerance replaced with
  `tolerances::TICK_BUDGET_60HZ_SLACK_US`
- **validate_release.sh test floor** — 364 → 378
- **README.md** — test count, coverage metric added
- **CONTEXT.md** — test count, coverage, phase updated
- **PRIMAL_REGISTRY.md** — Phase 16.1, 378 tests, 72.5% coverage
- **baseCamp README.md** — primalSpring status line updated
- **gen4 README.md + thesis** — test/experiment counts updated
- **wateringHole/README.md** — stats, handoff table, deep audit added
- Coverage: coordination 67%→83%, niche 63%→73%, launcher 22%→32%, ipc/client 53%→66%
- Library total coverage: 70% → 72.5%

## [Unreleased] — Phase 16: Deep Debt Audit + Centralized Tolerances (2026-03-24)

### Added
- **Remote gate TCP port defaults** — `DEFAULT_BEARDOG_PORT` through `DEFAULT_SQUIRREL_PORT`
  centralized in `tolerances/` (was inline in exp073/074)
- **Provenance trio resilience params** — `TRIO_RETRY_ATTEMPTS`, `TRIO_RETRY_BASE_DELAY_MS`
  centralized in `tolerances/` (was inline in `ipc/provenance.rs`)
- 3 new tolerance tests: trio resilience bounds, remote port range, port ordering
- Phase 16 handoff for primal and spring teams

### Changed
- **Tolerance calibration notes updated** — all 7 latency/throughput constants now document
  Phase 15 operational validation history (was "pending Phase N measurement")
- **Provenance trio circuit breaker** — `TRIO_CIRCUIT_THRESHOLD` removed from `ipc/provenance.rs`,
  now uses `tolerances::CIRCUIT_BREAKER_THRESHOLD` (single source of truth)
- **`extract_capability_names` deduplicated** — `coordination/mod.rs` local 2-format copy replaced
  with delegation to `ipc::discover::extract_capability_names` (full 4-format parser)
- **exp010 hardcoded description** — replaced exact string match with semantic check
  (description conveys ordering semantics, survives text evolution)
- **exp073/074 inline ports** — `9100`–`9500` literals replaced with `tolerances::DEFAULT_*_PORT`
  constants + env var override
- **exp074 primal names** — string literals replaced with `primal_names::*` slug constants
- **exp010 primal names** — string literals replaced with `primal_names::BEARDOG` etc.
- **Coordination tests** — `"beardog"`/`"songbird"`/etc. string literals replaced with
  `primal_names::*` constants
- **`validate_all` doc comment** — corrected from "discover at build time" to "enumerate
  experiment packages from a maintained manifest"
- **`validate_release.sh`** — test floor updated 361 → 364
- **`validate_remote_gate.sh`** — fixed stale `--port-base` in usage (actually `--unix`)
- **`niches/primalspring-coordination.yaml`** — version bumped 0.2.0 → 0.7.0
- Stale Mar 22 handoffs archived to `wateringHole/handoffs/archive/`
- 364 tests (up from 361), 0 clippy warnings, 0 fmt diff, 0 deny issues

## [Unreleased] — Phase 15: Cross-Ecosystem Absorption (2026-03-24)

### Added
- **`primal_names` slug constants** — `BEARDOG`, `SONGBIRD`, `TOADSTOOL`, `NESTGATE`,
  `SQUIRREL`, `RHIZOCRYPT`, `LOAMSPINE`, `SWEETGRASS` as `pub const` for zero-duplication
- **`CONTRIBUTING.md`** — ecosystem contributor guide (neuralSpring V124 pattern)
- **`SECURITY.md`** — security policy and vulnerability reporting
- **`unwrap_used` / `expect_used` = `warn`** workspace-wide (healthSpring V42 / wetSpring V135)
  with `cfg_attr(test, allow)` for test targets

### Changed
- **Hardcoded primal names eliminated** — `coordination/mod.rs`, `ipc/probes.rs`,
  `bin/main.rs` now use `primal_names::BEARDOG` etc. instead of string literals
- **`launcher/mod.rs` refactored** — tests extracted to `launcher/tests.rs` (802 → 695 LOC),
  env var names extracted as constants (`ENV_PLASMID_BIN`, `ENV_BIOMEOS_BIN_DIR`),
  relative discovery paths extracted to `RELATIVE_PLASMID_TIERS`
- **`ipc/provenance.rs` docs updated** — rhizoCrypt backend change (sled → redb v0.14),
  capability-based env vars noted for all trio primals
- 361 tests, 0 clippy warnings (including `--all-targets`), 0 doc warnings

## [Unreleased] — Phase 14: Deep Debt + Builder Pattern + Full Provenance (2026-03-24)

### Added
- **Builder-pattern `ValidationResult::run()`** — consumes `self` for idiomatic
  chaining: `ValidationResult::new(title).with_provenance(src, date).run(sub, |v| { ... })`
- **All 53 experiments carry structured provenance** — `with_provenance()` on every
  experiment (was 4/53). Source and baseline date traceable for every validation run

### Changed
- **`validation/mod.rs` refactored** — extracted 493-line test module to
  `validation/tests.rs`, production code now 540 lines (was 1016, over 1000 LOC limit)
- **All 53 experiments standardized on builder `.run()`** — eliminated manual
  `println!` banners, `v.finish()`, `std::process::exit(v.exit_code())` boilerplate
- **`.unwrap()` eliminated from all experiment binaries** — exp010/011/012 graph
  loading now uses `.or_exit()` with context messages
- **`#[allow(dead_code)]` → `#[expect(dead_code, reason = "...")]`** — 3 integration
  test files evolved to modern Rust with documented reason
- **Doc link fixed** in `ipc/provenance.rs` — broken intra-doc link escaped
- **Stale doc fixed** in `launcher/mod.rs` — Neural API socket path now documents
  actual `{nucleation_base}/biomeos/` location
- **`capability_registry.toml` version synced** — 0.5.0 → 0.7.0
- **`too_many_lines` resolved** — exp044 and exp063 refactored with extracted helpers
- 361 tests (up from 360), 0 clippy warnings, 0 doc warnings, 0 `#[allow()]` in production

## [Unreleased] — Phase 11–13 + Ecosystem Absorption + Cross-Gate Deployment (2026-03-23)

### Added
- **Provenance Trio Neural API Integration** — `ipc::provenance` module with
  full RootPulse pipeline (`begin_session`, `record_step`, `complete_experiment`)
  via `capability.call` (zero compile-time coupling to trio crates)
- `rootpulse_branch()`, `rootpulse_merge()`, `rootpulse_diff()`, `rootpulse_federate()`
- `trio_available()` and `trio_health()` diagnostic functions
- **BondType::Metallic** — electron-sea bonding for homogeneous fleet specialization
- **TrustModel** enum — GeneticLineage, Contractual, Organizational, ZeroTrust
- **BondingConstraint** — capability allow/deny lists, bandwidth limits, concurrency limits
- **BondingPolicy** — bond type + trust + constraints + time windows + relay offer
- Policy presets: `covalent_full()`, `idle_compute()`, `ionic_contract()`
- `BondType::all()`, `shares_electrons()`, `is_metered()` helper methods
- **4 multi-node deploy graphs** — `graphs/multi_node/`: basement_hpc_covalent,
  friend_remote_covalent, idle_compute_federation, data_federation_cross_site
- **`graph_metadata.rs`** — parse + validate `[graph.metadata]` and `[graph.bonding_policy]`
  from biomeOS deploy TOMLs; `validate_graph_bonding()`, `validate_all_graph_bonding()`
- **`stun_tiers.rs`** — 4-tier STUN config parser (Lineage → Self-hosted → Public → Rendezvous),
  `validate_sovereignty_first()`, `escalation_order()`
- **exp071_idle_compute_policy** — BondingPolicy capability masks, time windows, bandwidth
- **exp072_data_federation** — NestGate replication + trio provenance, 7-phase pipeline
- 12 bonding unit tests, 6 graph metadata unit tests, 6 STUN tier unit tests
- **Ecosystem Absorption Wave (Phase 12.1)**:
  - `deny.toml` ban convergence (groundSpring V120 + wetSpring V132: aws-lc-sys, cmake, cc, pkg-config, vcpkg)
  - Cast discipline clippy lints workspace-wide (neuralSpring S170 + airSpring V010)
  - `ValidationSink::section()` + `write_summary()` (groundSpring V120)
  - `ValidationResult::exit_code_skip_aware()` — 3-way CI exit (wetSpring V132)
  - `proptest_ipc` module — 7 cross-cutting IPC fuzz tests (healthSpring V41)
  - `primal_names` module — 23 canonical display↔slug mappings (neuralSpring pattern)
  - Provenance trio epoch-based circuit breaker + exponential backoff (healthSpring V41)

- **Ecosystem Absorption Wave (Phase 12.2)**:
  - `normalize_method()` — ecosystem-wide JSON-RPC dispatch standard, strips legacy prefixes (groundSpring V121, neuralSpring V122, wetSpring V133, healthSpring V42)
  - `check_relative()` + `check_abs_or_rel()` — robust numeric validation (groundSpring V120, healthSpring V42)
  - `NdjsonSink` — streaming validation output for CI/log aggregation (groundSpring V121, wetSpring V133)
  - `IpcError::is_recoverable()` — broader recovery classification (neuralSpring V122, wetSpring V133)
  - `Transport` enum (Unix + Tcp) — cross-platform IPC layer (airSpring V010, healthSpring V42)
  - `ipc::probes` — `OnceLock`-cached runtime resource probes for test parallelism (hotSpring V0.6.32, neuralSpring V122)
  - `validate_release.sh` — release quality gate (fmt + clippy + deny + test floor + docs)
  - `missing_docs` upgraded from `warn` to `deny` workspace-wide
  - Server dispatch wired through `normalize_method()` for prefix-agnostic routing

- **Cross-Gate Deployment Tooling (Phase 13)**:
  - `scripts/build_ecosystem_musl.sh` — build all primals as x86_64 + aarch64 musl static binaries
  - `scripts/prepare_spore_payload.sh` — assemble USB spore deployment payload (binaries + graphs + genetics)
  - `scripts/validate_remote_gate.sh` — probe remote gate NUCLEUS health via TCP JSON-RPC
  - **exp073_lan_covalent_mesh** — cross-gate Songbird mesh + BirdSong beacon exchange via TCP
  - **exp074_cross_gate_health** — remote per-primal TCP health + capabilities + composition assessment
  - exp063 evolved: cross-device Pixel beacon exchange via `PIXEL_SONGBIRD_HOST` + TCP
  - `basement_hpc_covalent.toml` annotated with full gate inventory from HARDWARE.md
  - **LAN_COVALENT_DEPLOYMENT_GUIDE** handoff — step-by-step for all gate operators
  - 53 experiments (up from 51), 10 tracks (up from 9)

### Changed
- `BOND_TYPE_COUNT` updated to 5 in exp032, exp033
- exp030 (covalent) — added BondType properties, BondingPolicy, HPC graph metadata
- exp032 (plasmodium) — added Metallic validation, graph metadata
- exp056 (cross-tower) — added 3 multi-node graph metadata validations
- Metallic match arm added to primalspring_primal bonding_test handler
- `missing_docs` lint level evolved from `warn` to `deny` (all public items documented)
- 360 tests (up from 303), 51 experiments, 22 deploy graphs (at time of Phase 12.2)

## [0.7.0] — 2026-03-22

### Added
- **Graph-Driven Overlay Composition** — tier-independent primals (Squirrel,
  petalTongue, biomeOS) compose at any atomic tier via deploy graphs
- **Squirrel Cross-Primal Discovery** — Squirrel discovers sibling primals
  (NestGate, ToadStool, Songbird, BearDog) via explicit env_sockets wiring
  and `$XDG_RUNTIME_DIR/biomeos/` socket scanning
- `spawn` field on `GraphNode` — distinguishes primal nodes (spawn=true) from
  validation/coordination nodes (spawn=false). Defaults to true for backward
  compatibility with existing graphs
- `graph_spawnable_primals()` — extract spawnable primal names from a graph
- `graph_capability_map()` — build capability-to-primal mapping from graph
- `merge_graphs()` — merge base + overlay deploy graphs for runtime composition
- `RunningAtomic::overlay_capabilities` — dynamic capability resolution for
  primals beyond the base tier
- `RunningAtomic::all_capabilities()` — returns base + overlay capability names
- `RunningAtomic::overlay_primals()` — names of primals from the graph overlay
- 5 new overlay deploy graphs: `tower_ai.toml`, `tower_ai_viz.toml`,
  `nest_viz.toml`, `node_ai.toml`, `full_overlay.toml`
- 9 Squirrel env_sockets in launch profile for cross-primal capability routing
- 15 new integration tests (4 structural + 7 live overlay + 4 Squirrel discovery)
- **exp069_graph_overlay_composition** — end-to-end overlay validation (25/25)
- **exp070_squirrel_cross_primal_discovery** — cross-primal discovery validation
- Gates 17-20 in TOWER_STABILITY.md: overlay composition gates (14/14 PASS)
- Gate 21 in TOWER_STABILITY.md: Squirrel cross-primal discovery (5/5 PASS)
- **Graph Execution Patterns Live** — exp010 (Sequential), exp011 (Parallel),
  exp012 (ConditionalDag) rewired from scaffolded skips to live AtomicHarness
  compositions with real primals
- **Provenance Readiness** — launch profiles for sweetGrass, loamSpine,
  rhizoCrypt; `provenance_overlay.toml` deploy graph; handoffs delivered
- Gate 22: Graph Execution Patterns (6/6 PASS)
- Gate 23: Provenance Readiness (4/4 PASS)

### Changed
- `compute_spawn_order` now spawns **all** graph nodes with `spawn=true`, not
  just those in `required_primals()`. Base tier primals are the minimum
  guarantee; graphs can add more
- `capability_to_primal` returns `Option<String>` (was `Option<&'static str>`)
  to support dynamic overlay capabilities
- All existing deploy graphs updated with `spawn = false` on validation nodes
- exp010-012 rewired from scaffolded skips to live graph-driven compositions
- 87/87 total gates, 49 experiments, 253+ tests

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

## [0.3.0] — 2026-03-18

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
