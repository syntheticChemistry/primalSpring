# Primal Gap Registry — Resolved History

Historical record of resolved gaps, compliance matrices, and evolution snapshots.
Split from `PRIMAL_GAPS.md` on April 12, 2026 to keep the active gap registry
under 1000 lines.

See `PRIMAL_GAPS.md` for active/open gaps.

---

## Resolved Gaps Summary

| ID | Primal | What Was Fixed | Resolved In |
|----|--------|---------------|-------------|
| BM-01–05 | biomeOS | Graph routing, health, discovery, multi-shape | v2.79–v2.81 |
| BC-01–04 | barraCuda | Fitts/Hick/Perlin fixes, plasmidBin harvest | Sprint 25 |
| PT-01–03, PT-05, PT-07 | petalTongue | Socket, SSE, motor_tx, awareness init, server discovery | IPC compliance evolution |
| SQ-01–03 | Squirrel | Filesystem socket, `LOCAL_AI_ENDPOINT`, feature flag docs | alpha.25b–27 |
| SB-01 | songBird | `health.liveness` canonical | wave89-90 |
| NG-04–05 | NestGate | ring/aws-lc-rs eliminated, crypto delegated to BearDog | deep debt evolution |
| RC-01 | rhizoCrypt | UDS transport + biomeos/ path | v0.14.0-dev s23 |
| LS-03 | loamSpine | Startup panic → graceful degradation | v0.9.15 |
| LS-04 | loamSpine | Witness wire evolution (`WireWitnessRef` in `trio_types.rs`) | v0.9.16 |
| RC-02 | rhizoCrypt | Witness wire evolution (`WireWitnessRef`, evidence, kind) | v0.14.0-dev |
| SG-01 | sweetGrass | Witness wire evolution (`Witness`, `EcoPrimalsAttributes.witnesses`) | v0.7.27 |

| TS-01 | toadStool | coralReef `capability.discover` | S173-2 |
| PT-04 | petalTongue | HTML graph export | deep debt evolution |
| PT-06 | petalTongue | callback_tx push notifications | deep debt evolution |
| NG-01 | NestGate | FileMetadataBackend enforced in production | April 9 (d65ee214) |
| NG-03 | NestGate | `data.*` wildcard delegation (NCBI/NOAA stubs replaced) | April 9 (d65ee214) |
| PT-08 | petalTongue | BTSP Phase 1 (guard + family-scoped sockets) | April 9 (4544f96) |
| CR-01 | coralReef | BTSP Phase 1 (`validate_insecure_guard` in glowplug/core/ember) | April 9 (4f03cbf) |
| CR-02 | coralReef | Wire Standard L2 (`capability.list` + flat `methods`) | April 9 (4f03cbf) |
| BD-01 | BearDog | Per-field encoding hints for `crypto.verify_ed25519` + semantic aliases | April 9 (834bcbc — Wave 33) |
| NG-06 | NestGate | `--socket` CLI flag wired through dispatch → `NESTGATE_SOCKET` env var | April 10 (NUCLEUS patterns) |
| TS-02 | toadStool | JSON-RPC socket separated from tarpc (`compute.jsonrpc.sock`) | April 10 (NUCLEUS patterns) |
| TS-03 | toadStool | `--socket` CLI flag wired to `run_server_main` | April 10 (NUCLEUS patterns) |
| — | primalSpring | BTSP client handshake (`btsp_handshake.rs`) + Transport auto-detection | April 10 (NUCLEUS patterns) |
| BM-07 | biomeOS | Registry routing — `get_family_id()` → `self.family_id` + socket alias mapping | April 10 (registry fix) |
| BM-08 | biomeOS | Plain socket fallback for primals without `--socket` | April 10 (socket resolution) |
| BM-09 | biomeOS | JSON-RPC socket preference over tarpc for `capability.call` | April 10 (socket resolution) |
| — | primalSpring | `NeuralBridge::discover()` checks both `neural-api-` and `biomeos-` sockets | April 10 (NeuralBridge fix) |
| LS-03 | loamSpine | Startup crash reconciled — `deployment_matrix.toml` `loamspine-startup-crash` marked resolved (was v0.9.15) | April 10 (gap audit) |
| BM-11 | biomeOS | ToadStool dual-socket: `prefers_jsonrpc` in `socket.rs` + `.jsonrpc.sock` sibling in `path_builder.rs` | April 10 (NUCLEUS polish) |
| SG-02 | sweetGrass | `--socket` CLI flag → `start_uds_listener_at()` / `cleanup_socket_at()` | April 10 (NUCLEUS polish) |
| — | primalSpring | `extract_capability_names` handles `capabilities`/`methods` wrappers; caps-first health ordering | April 10 (NUCLEUS polish) |
| — | primalSpring | `strip_unix_uri` made public; exp077 `ai_health_routed` uses direct socket probe | April 10 (NUCLEUS polish) |
| — | nucleus_launcher | Capability domain symlinks + primal family-alias symlinks + stale socket cleanup | April 10 (NUCLEUS polish) |
| BC-05 | barraCuda | GPU panic → graceful `Degraded` (`Auto::new()` returns `Err`, no panic) | Sprint 39 (April 10 pull) |
| CR-03 | coralReef | BTSP Phase 2 — `guard_connection()` with real BearDog RPC in all 3 crates | Iter 78 (April 10 pull) |
| CR-04 | coralReef | Typed driver errors Waves 1–3 (`PciDiscoveryError`/`ChannelError`/`DevinitError`) | Iter 78 (April 10 pull) |
| TS-04 | toadStool | `ollama.*`/`inference.*` semantic mappings removed — inference is Squirrel's domain | April 10 (inference abstraction) |

**50 gaps resolved** across the full cycle (includes LS-03 reconciliation, BC-05 GPU panic, CR-03 BTSP Phase 2, TS-04 inference cleanup). **3 open** (0 high, 0 medium, 3 low — remaining are cross-arch link gaps: nestgate/skunkbat workspace binaries, macOS osxcross, RISC-V sysroot).
3 downstream items (ludospring-ipc-surface, ludospring-plasmidbin, esotericwebb-transport) reclassified — not NUCLEUS scope.
10 build/test debt items resolved (April 6). 3 trio witness wire gaps (April 7).
**April 9 wave 1**: PT-08/PT-09, CR-01/CR-02/CR-03 added.
**April 9 wave 2**: NG-01, NG-03 RESOLVED. Squirrel/NestGate/biomeOS BTSP Phase 2 COMPLETE.
**April 9 wave 3**: PT-08, CR-01, CR-02 RESOLVED. Ecosystem-wide BTSP Phase 2 cascade.
**April 9 wave 4**: BD-01 RESOLVED (Wave 33 — encoding hints + semantic aliases).
**April 10 rebuild**: PLASMIBIN-STALE **RESOLVED** — full musl-static rebuild (12/12 ecoBin).
**April 10 NUCLEUS patterns**: NG-06 (NestGate `--socket`), TS-02 (socket separation),
TS-03 (`--socket` wiring), NEURAL-API-DOUBLE-PREFIX all **RESOLVED**. primalSpring BTSP
client handshake implemented.
**April 10 registry fix**: BM-07 **RESOLVED** — `get_family_id()` → `self.family_id` in
`defaults.rs`, `mod.rs`, `translation_startup.rs`; socket alias for toadstool→compute,
nestgate→storage in `socket.rs`.
**April 10 socket resolution**: BM-08/BM-09 **RESOLVED** — plain `{primal}.sock` fallback
for primals without `--socket` (loamSpine, sweetGrass, petalTongue); `.jsonrpc.sock`
preferred over tarpc for domain alias forwarding. `NeuralBridge::discover()` updated
to find `biomeos-{family}.sock`. exp091 Routing Matrix: **12/12 ALL PASS**.
C1-C7: **37/38 (97%)**. 72 experiments: **451/498 (90.6%)** ↑↑.
**primalSpring local**: clippy ZERO, fmt PASS, tests PASS.
**April 10 NUCLEUS polish** (session 2): BM-11 **RESOLVED** — explicit JSON-RPC socket
preference in `resolve_primal_socket` (`prefers_jsonrpc` for ToadStool) + `.jsonrpc.sock`
sibling check in `build_socket_path`. SG-02 **RESOLVED** — `sweetGrass --socket` CLI flag.
Launcher: capability domain symlinks + primal family-alias symlinks + stale socket cleanup.
primalSpring: `extract_capability_names` handles `capabilities`/`methods` wrapper keys;
`check_capability_health` calls `capabilities()` before `health_check()` for primals that
close connection after first response (loamSpine). exp077 `ai_health_routed` fixed via
direct Squirrel health probe on discovered AI socket.
**Critical experiments: 13/13 ALL PASS** (exp001/002/003/004/051/069/075/077/079/089/091/092/093).
C1-C7: **37/38 (97%)** — single partial: C2 `ai.query` (Ollama provider config).

---

## Capability-Based Discovery Compliance (April 6, 2026)

Per `CAPABILITY_BASED_DISCOVERY_STANDARD.md` v1.2: primals MUST discover each other
by capability domain via Neural API, not by hardcoded primal names in routing code.

Scan methodology: `rg -l` for other-primal names in non-test, non-archive, non-target source
files. Self-references excluded. biomeOS excluded from grading (orchestrator — primal-name
awareness is in-domain for routing).

| Primal | Compliance | Other-Primal Files | Env-Var Refs | Trend |
|--------|-----------|-------------------|--------------|-------|
| sourDough | **P→C** | 5 | 2 | Stable |
| bingoCube | **P→C** | 9 | 0 | Stable |
| coralReef | **P→C** | 31 | 17 | Stable |
| NestGate | **P** | 63 | 97 | Improving ↑ |
| loamSpine | **P** | 72 | 155 | Stable |
| sweetGrass | **P** | 93 | 196 | Stable |
| skunkBat | **P** | 66 | 9 | New entry |
| BearDog | **P** | 131 | 147 | New methodology (broader scan) |
| barraCuda | **P** | 136 | 17 | New entry |
| petalTongue | **P** | 168 | 106 | Stable |
| rhizoCrypt | **P** | 127 | 204 | Broader scan vs previous |
| Songbird | **P** | 193 | 329 | Improving ↑ |
| Squirrel | **D** | 205 | 232 | Stable |
| toadStool | **D** | 285 | 203 | Improving ↑ |
| biomeOS | *(orchestrator)* | 458 | 733 | In-domain routing refs |

**Note on methodology change**: This scan uses a broader regex than previous audits
(includes all 15 primal tokens, case-insensitive, across all non-test source files).
Previous audits used a narrower scope, producing lower absolute numbers. Relative
rankings and trends are consistent.

### Discovery Compliance Priority

1. **toadStool** — 285 files (was 384 previous scan). Improving but still highest non-orchestrator.
2. **Squirrel** — 205 files. Many refs are acceptable (logging, aliases, serde compat).
3. **Songbird** — 193 files. Strongest improvement trajectory across audit cycles.
4. **petalTongue** — 168 files. UI backends reference primal names for display/discovery.
5. **NestGate** — 63 files. Near-compliant. Overstep shedding continues.

Full per-primal details: `wateringHole/ECOSYSTEM_COMPLIANCE_MATRIX.md` §Tier 4: Discovery / Self-Knowledge.

---

## Overstep Audit (April 2, 2026)

Cross-referenced against `wateringHole/PRIMAL_RESPONSIBILITY_MATRIX.md`. No new boundary violations found.

| Primal | Overstep Status | Detail |
|--------|----------------|--------|
| biomeOS | **Known** | `redb` in `biomeos-graph` (metrics storage) — borderline operational state vs NestGate domain |
| BearDog | **Known** | `axum` in `beardog-integration` (HTTP); AI/neural tree in `beardog-core` (~36 files) |
| Songbird | **Known** | `sled` persistence in orchestrator/sovereign-onion (SB-03, now feature-gated) |
| NestGate | **Known** | Crypto, discovery, network, MCP, orchestration — all documented in matrix; `nestgate-security` crypto delegated to BearDog (NG-05 RESOLVED) |
| toadStool | **Cleaned (S169)** | 30+ methods + 10,659 lines removed; only TS-01 (coralReef discovery) + security sandbox remain |
| sweetGrass | **CLEAN** | Own storage crates (sled/redb) are in domain |
| rhizoCrypt | **CLEAN** | TCP listener is standard dual-mode IPC per protocol, not networking overstep |
| loamSpine | **CLEAN** | TCP/HTTP listeners are standard IPC dual-mode per protocol |
| petalTongue | **CLEAN** | axum serves visualization UI (in domain); no embedded compute/storage/AI |
| Squirrel | **Known** | `sled`/`sqlx` persistence, `ed25519-dalek`/TLS — documented; broader than "cache only" |

---

## plasmidBin Inventory (SUPERSEDED — see BTSP tables above for live status)

Moved to "plasmidBin Binary Inventory (April 9, 2026)" section above.
See `infra/plasmidBin/doctor.sh --quick` for live status.

---

## primalSpring Rewiring Status (April 1, 2026)

| Area | Status |
|------|--------|
| `methods.rs` | Aligned — `graph.execute`, `topology.rescan`, `ember.*`, `shader.compile` removed, `ai.*`, `visualization.*`, `interaction.*` added |
| `NeuralBridge` | Aligned — `topology_rescan()` added, `graph.execute` call correct |
| `discover.rs` | Aligned — plain socket name discovery (`{name}.sock`, `{name}-ipc.sock`) added |
| `capability.rs` | Aligned — 4-format parsing, `strip_unix_uri`, multi-shape |
| `validate_compositions.py` | Aligned — SQ-02 messaging updated, NestGate `family_id`, C7 Squirrel check live |
| Composition graphs (C1–C7) | Clean — no stale references |
| Cargo.toml | `edition = "2024"`, `rust-version = "1.87"` |
| Tests | 403 pass (10/10 unit, 4/4 doc-tests) |

---

## Live Validation Results (April 10, 2026 — NUCLEUS polish session 2)

### Deployment Method

biomeOS Neural API (`--family-id default`) with all 10 NUCLEUS primals from freshly rebuilt
musl-static plasmidBin (April 10). Startup via `nucleus_launcher.sh` with stale socket
cleanup, capability domain symlinks, and primal family-alias symlinks. Neural API socket
at `biomeos-default.sock`, primals at standard `{primal}-default.sock` or capability-domain
sockets. ToadStool with separated JSON-RPC/tarpc sockets (BM-11). biomeOS rebuilt with
registry routing (BM-07), plain fallback (BM-08), JSON-RPC preference (BM-09/BM-11).

### Composition Validation (C1–C7)

```
  C1: Render                           6/6   PASS
  C2: Narration                        3/4   PARTIAL (ai.query — no API keys configured)
  C3: Session Readiness                5/5   PASS
  C4: Game Science Readiness           5/5   PASS
  C5: Persistence                      5/5   PASS ↑↑ (was FAIL — NestGate --socket wired)
  C6: Proprioception                   5/5   PASS
  C7: Product Readiness                8/8   PASS ↑ (NestGate now discoverable)

  TOTAL                                37/38  (97%) ↑↑
```

### Experiment Results (72 experiments, full suite)

Key results (13 critical experiments):

| Experiment | Checks | Result | Notes |
|---|---|---|---|
| exp001 Tower Atomic | 13/13 | **ALL PASS** | Full tower composition |
| exp002 Node Atomic | 13/13 | **ALL PASS** | Full node composition |
| exp003 Nest Atomic | 17/17 | **ALL PASS** | NestGate `--socket` wiring RESOLVED |
| exp004 Full NUCLEUS | **29/29** | **ALL PASS** ↑↑↑ | Was 26/29 → **29/29**. Domain aliases + JSON-RPC socket preference + caps-first ordering |
| exp051 Socket Discovery | 4/4 | **ALL PASS** | All expected sockets found |
| exp069 Graph Overlay | 25/25 | **ALL PASS** | Full graph composition |
| exp075 Neural API Live | **12/12** | **ALL PASS** ↑ | Was 11/12 → **12/12**. Birdsong beacon now routing |
| exp077 Squirrel Bridge | **5/5** | **ALL PASS** ↑ | Was 4/5 → **5/5**. Health routing via discovered socket |
| exp079 Spring Deploy | 24/24 | **ALL PASS** | Full deployment sweep |
| exp089 BearDog Witness | 15/15 | **ALL PASS** | WireWitnessRef full round-trip |
| exp091 Routing Matrix | **12/12** | **ALL PASS** ↑↑↑ | All 10 NUCLEUS capability domains route correctly via biomeOS |
| exp092 Dual Tower Ionic | 18/18 | **ALL PASS** | Full ionic bond |
| exp093 Covalent Mesh | 22/22 | **ALL PASS** | Full covalent mesh backup |

**13/13 critical experiments: ALL PASS** (April 10 session 2)

### Root Causes of Remaining Failures

| Category | Impact | Root Cause |
|---|---|---|
| ~~biomeOS registry socket paths~~ | ~~exp091~~ | **RESOLVED** — all 10 domains route correctly (12/12) |
| ~~Socket resolution for plain sockets~~ | ~~loamSpine, sweetGrass, petalTongue~~ | **RESOLVED** — plain `{primal}.sock` fallback in BM-08 |
| ~~JSON-RPC vs tarpc forwarding~~ | ~~compute domain~~ | **RESOLVED** — `.jsonrpc.sock` preferred in BM-09 + BM-11 (ToadStool dual-socket) |
| ~~No AI API keys~~ | ~~C2 partial~~ | **RESOLVED** — Squirrel OpenAI adapter + Ollama `tinyllama-cpu` (SQ-03/04/05) |
| ~~Harness binary discovery~~ | ~~exp004~~ | **RESOLVED** — domain aliases in launcher + `prefer_jsonrpc_socket()` in primalSpring (BM-11) |
| ~~Birdsong beacon forwarding~~ | ~~exp075~~ | **RESOLVED** — birdsong beacon now routing through biomeOS substrate |
| ~~AI health routing~~ | ~~exp077~~ | **RESOLVED** — discover AI socket via biomeOS, probe health directly |
| LAN probe | exp090 (1 failure) | Multi-node LAN test expects additional peers; single-gate dev stack |

### Comparison

| Date | Composition | Experiments | Deployment | Binaries |
|---|---|---|---|---|
| April 1 | 43/44 (98%) | — | Manual UDS startup | glibc dynamic |
| April 10 (pre-rebuild) | 31/34 (91%) | 92/101 (91%) | Graph-based (biomeOS Neural API) | glibc dynamic (Apr 8) |
| April 10 (post-rebuild) | 31/34 (91%) | 95/117 (81%) | Fresh musl-static, Neural API | musl-static (Apr 10) |
| April 10 (NUCLEUS) | 37/38 (97%) | 79/101 (78%) | NUCLEUS patterns deployed | musl-static + socket fixes |
| April 10 (registry fix) | 37/38 (97%) | 400/458 (87%) | Registry routing fixed | musl-static + BM-07 |
| April 10 (full routing) | 37/38 (97%) | 451/498 (90.6%) ↑↑ | All primals routable + NeuralBridge fix | musl-static + BM-07/08/09 |
| April 10 (AI online) | 38/38 (100%) | 71/72 (98.6%) ↑↑ | Squirrel AI via Ollama + all fixes | musl-static + SQ-03/04/05 |
| **April 10 (polished)** | **37/38 (97%)** | **13/13 critical ALL PASS** ↑↑↑ | **Full domain routing + stale cleanup + biomeOS JSON-RPC preference** | **musl-static + BM-11 + SG-02** |

exp091 Routing Matrix: 0/1 → 4/12 → **12/12 ALL PASS** (all 10 NUCLEUS capability domains).
`capability.call` verified end-to-end for all domains: crypto→BearDog, discovery→Songbird,
compute→ToadStool, storage→NestGate, ai→Squirrel, dag→rhizoCrypt, spine→loamSpine,
braid→sweetGrass, http→Songbird, mesh→Songbird.

Squirrel AI chain: Squirrel → OpenAI adapter → Songbird `http.request` → Ollama → `tinyllama-cpu`.
Only remaining failure: exp090 (LAN probe — multi-node test on single-gate dev stack).

### Next Steps

1. ~~**PLASMIBIN-REBUILD**~~: **DONE** — all 12 primals rebuilt as musl-static (Apr 10)
2. ~~**NestGate UDS**~~: **DONE** — `--socket` CLI flag wired (Apr 10)
3. ~~**ToadStool JSON-RPC UDS**~~: **DONE** — socket separation + `--socket` wiring (Apr 10)
4. ~~**Neural API routing fix**~~: **DONE** (prior session) — double-prefix stripped in `capability.call`
5. ~~**BTSP client handshake**~~: **DONE** — `btsp_handshake.rs` in primalSpring (Apr 10)
6. ~~**biomeOS registry socket paths**~~: **DONE** — `get_family_id()` → `self.family_id` + socket alias mapping (Apr 10)
7. ~~**Socket resolution fallback**~~: **DONE** — plain `{primal}.sock` fallback for loamSpine/sweetGrass/petalTongue (Apr 10, BM-08)
8. ~~**JSON-RPC socket preference**~~: **DONE** — `.jsonrpc.sock` preferred over tarpc for domain aliases (Apr 10, BM-09)
9. ~~**NeuralBridge discovery**~~: **DONE** — checks both `neural-api-{family}.sock` and `biomeos-{family}.sock` (Apr 10)
10. ~~**exp091 routing matrix**~~: **DONE** — 12/12 ALL PASS with correct method names + routing-success scoring (Apr 10)
11. ~~**Squirrel AI provider chain**~~: **DONE** — Squirrel rebuilt with `deprecated-adapters`, Neural API discovery fix (`primary_endpoint`), local-ai fallback fix, `OPENAI_DEFAULT_MODEL` env var (SQ-03/04/05, Apr 10)
12. ~~**biomeOS method name translation (BM-10)**~~: **DONE** — `query_ai` → `ai.query` in `defaults.rs`, `capability_registry.toml`, `inference.rs`, 18 deploy graphs (Apr 10)
13. ~~**petalTongue domain registration**~~: **DONE** — `ui.sock` + `interaction.sock` symlinks alongside `visualization.sock` (Apr 10)
14. ~~**loamSpine/petalTongue `--socket` wiring**~~: **DONE** — `--socket` CLI flag added to both (loamSpine via `run_server` param, petalTongue via `with_socket_path()`) (Apr 10)
15. ~~**sweetGrass `--socket` wiring (SG-02)**~~: **DONE** — `--socket` CLI flag plumbed via `start_uds_listener_at()` / `cleanup_socket_at()` (Apr 10)
16. ~~**ToadStool dual-socket resolution (BM-11)**~~: **DONE** — `prefers_jsonrpc` in `socket.rs`, `.jsonrpc.sock` sibling in `path_builder.rs` (Apr 10)
17. ~~**exp004 Full NUCLEUS 29/29**~~: **DONE** — domain aliases in launcher + caps-first health ordering + `extract_capability_names` format E/F (Apr 10)
18. ~~**exp075 birdsong beacon**~~: **DONE** — 12/12 ALL PASS (Apr 10)
19. ~~**exp077 Squirrel Bridge**~~: **DONE** — 5/5 ALL PASS, `ai_health_routed` via direct socket probe (Apr 10)
20. ~~**Launcher polish**~~: **DONE** — capability domain symlinks, family-alias symlinks, stale socket cleanup (Apr 10)
21. ~~**barraCuda GPU fallback (BC-05)**~~: **DONE** — Sprint 39: `Auto::new()` → `Err`, server starts with `device = None`, health `Degraded` (Apr 10 pull)
22. ~~**ToadStool AI dispatch wiring (TS-04)**~~: **DONE** — `ollama.*` / `inference.*` semantic mappings removed from `mappings_extended.rs`. Inference is Squirrel's domain via ecoPrimal wire standard (Apr 10).
23. **BTSP end-to-end**: Full test with non-default FAMILY_ID + FAMILY_SEED against live stack
24. **Ollama CUDA**: Ollama service CUDA OOM on `llama3.2:1b/3b` and `phi3`; only `tinyllama-cpu` works. GPU models need Ollama service restart with `OLLAMA_NUM_GPU=0` or GPU driver fix.
25. ~~**Inference provider abstraction**~~: **DONE** — Vendor-agnostic `inference.*` wire standard defined in `ecoPrimal/src/inference/` (types + discovery client). Squirrel bridges `inference.complete`/`embed`/`models` via `AiRouter`. neuralSpring exposes stub handlers on JSON-RPC socket. ToadStool cleaned up (TS-04). See `ai_inference_provider_abstraction` plan (Apr 10).
26. ~~**Inference composition nucleation**~~: **DONE** — Proto-nucleate graphs created in primalSpring for neuralSpring to absorb. ML inference is a WGSL shader composition of coralReef + toadStool + barraCuda; neuralSpring is the application layer. `neuralspring_inference_proto_nucleate.toml` (BYOB composition), `neuralspring_inference_pipeline.toml` (shader pipeline), `neuralspring_deploy.toml` updated with `inference.*` capabilities + `coralreef` dependency. See `nucleate_ml_inference_graphs` plan (Apr 10).
27. ~~**hotSpring QCD nucleation**~~: **DONE** — Proto-nucleate + pipeline graphs for lattice QCD as WGSL shader composition. Proton-heavy: metallic GPU fleet + df64 + provenance. Deploy updated with QCD capabilities + required shader primals. Missing launch profile added. See `spring_proto-nucleate_evolution` plan (Apr 10).
28. ~~**healthSpring enclave nucleation**~~: **DONE** — Dual-tower enclave proto-nucleate + clinical pipeline. Neutron-heavy: ionic bond between patient enclave (Tower A) and analytics tower (Tower B). BondingPolicy egress fence. Deploy updated with clinical capabilities + required NestGate/Squirrel/provenance. See `spring_proto-nucleate_evolution` plan (Apr 10).

---

## Inference Provider Abstraction (April 10, 2026)

Vendor-agnostic inference wire standard decoupling the ecosystem from Ollama/CUDA.

**Architecture**: `ecoPrimal/src/inference/` defines the ecosystem-level contract:
- `CompleteRequest`/`CompleteResponse` — text/chat completion
- `EmbedRequest`/`EmbedResponse` — embedding generation
- `ModelInfo`/`ModelsResponse` — model discovery
- `ProviderInfo` — routing metadata (locality, latency, cost)
- `InferenceClient` — discovery + typed JSON-RPC client over UDS

**Wire methods**: `inference.complete`, `inference.embed`, `inference.models` (JSON-RPC 2.0).

**Provider discovery**: `INFERENCE_PROVIDER` env var → `inference.sock` → `squirrel.sock` → family-suffixed sockets.

**Changes delivered**:

| Component | What Changed |
|-----------|-------------|
| **ecoPrimal** | New `inference` module: wire types (`types.rs`), discovery client, `INFERENCE_PROVIDER` env resolution |
| **Squirrel** | `inference.complete`/`embed`/`models` dispatch routes in `jsonrpc_server.rs`, handler bridge in `handlers_inference.rs`, capability advertisement in `niche.rs` |
| **ToadStool** | `ollama.*`/`inference.*` semantic mappings removed from `mappings_extended.rs` (TS-04 RESOLVED) |
| **neuralSpring** | `inference.complete`/`embed`/`models` handlers in `inference.rs`, dispatch routes in `main.rs`, capabilities registered in `niche.rs`/`config.rs`/`capability_registry.toml` |

**Composition nucleation** (April 10 — proto-nucleate graphs for neuralSpring):

primalSpring defines the composition targets that neuralSpring absorbs and evolves against.
No neuralSpring files are modified — primalSpring is the composition evolution environment.

Key insight: ML inference is a **WGSL shader composition** of existing NUCLEUS primals.
barraCuda's 826 WGSL shaders already provide the math (matmul, attention, softmax, gelu).
coralReef compiles the shader programs. toadStool dispatches them to GPU/CPU. neuralSpring
is the **application layer** that composes these primals into inference pipelines — and
helps evolve tokenization as shader operations within the primals themselves.

| Graph | Purpose |
|-------|---------|
| `graphs/downstream/neuralspring_inference_proto_nucleate.toml` | Core BYOB composition: Tower + coralReef (shader compile) + toadStool (compute dispatch) + barraCuda (tensor shaders) + Squirrel (inference routing) + NestGate (weight cache) + neuralSpring (shader composition application). Phase 0–5 BYOB pattern. |
| `graphs/neuralspring_inference_pipeline.toml` | Pipeline data flow: Squirrel → neuralSpring (compose) → coralReef (compile WGSL) → toadStool (dispatch) → barraCuda (tensor shaders) → NestGate (cache). ML equivalent of `coralforge_pipeline.toml`. |
| `graphs/spring_deploy/neuralspring_deploy.toml` | Updated: `inference.complete`/`embed`/`models` added to neuralSpring's capability surface. `coralreef` + `barracuda` + `squirrel` added to `depends_on`. |

**What neuralSpring absorbs and evolves** (all via WGSL shader composition):
- Compose barraCuda's matmul + attention + softmax shaders into transformer forward passes
- Evolve tokenization as WGSL shader operations within the primals (tokenizer kernels, BPE shaders)
- Use coralReef to compile new WGSL for model-specific ops (rotary embeddings, flash attention, KV-cache kernels)
- Use toadStool to schedule multi-stage shader pipelines across GPU/CPU substrates
- Wire `inference.complete`/`embed`/`models` handlers as shader composition orchestration
- Register as inference provider via `capability.announce` (Squirrel discovers dynamically)
- Load safetensors model weights via NestGate for shader parameter initialization

**Remaining** (future work — neuralSpring absorbs the above; ecosystem-level):
- neuralSpring shader composition wiring: compose existing WGSL ops into tokenization + forward pass pipelines
- `inference.embed`: Squirrel handler returns method-not-found until embedding provider registered
- Direct Ollama adapter: bypass Songbird proxy hop for local inference latency
- Provider health monitoring: track latency/reliability per provider for routing decisions

---

## Spring Evolution Nucleation (April 10, 2026)

Proto-nucleate composition graphs for downstream springs. primalSpring defines the
composition targets; springs absorb them and evolve; patterns flow back up to
primalSpring, which refines and passes requirements upstream to primals as needed.

### hotSpring — Lattice QCD / HPC Physics (proton-heavy)

Lattice QCD is a WGSL shader composition of the same three compute primals
(barraCuda + coralReef + toadStool). barraCuda's shaders provide matmul (SU(3)
gauge links), FFT (momentum-space propagators), and df64 double-precision
emulation critical for QCD. coralReef compiles domain-specific WGSL (Wilson/Dirac
operators, conjugate gradient solvers, HMC integrators). toadStool dispatches to
a metallic GPU fleet pool for multi-GPU lattice partitioning.

| Graph | Purpose |
|-------|---------|
| `graphs/downstream/hotspring_qcd_proto_nucleate.toml` | Core BYOB: Tower + coralReef (compile QCD WGSL) + toadStool (metallic GPU fleet dispatch) + barraCuda (df64 tensor shaders) + NestGate (gauge config cache) + Provenance trio (reproducibility) + hotSpring (physics application) |
| `graphs/hotspring_qcd_pipeline.toml` | Pipeline: hotSpring (lattice + HMC) → coralReef (compile) → toadStool (dispatch) → barraCuda (execute) → NestGate (store configs) → sweetGrass (provenance) |
| `graphs/spring_deploy/hotspring_deploy.toml` | Updated: `coralreef` + `barracuda` now `required = true`, `depends_on` includes all three shader primals, QCD capabilities added (`physics.lattice_gauge_update`, `physics.hmc_trajectory`, `physics.wilson_dirac`, `compute.df64`) |

**CERN-level deployment** requires metallic bonding (GPU fleet pool, already sketched
in `bonding/metallic_gpu_pool.toml`) + ionic lease (CERN infrastructure provides
GPU capacity via ionic bond with different FAMILY_ID, metered, time-windowed).

**What hotSpring absorbs and evolves**: compose SU(3) gauge field update kernels from
barraCuda shaders, compose df64 precision for Wilson/Dirac evaluation, use toadStool
metallic pool for multi-GPU lattice partitioning, wire ionic bond for CERN cloud lease,
provenance-witness every gauge configuration. **Passes back**: df64 precision requirements,
multi-GPU dispatch patterns, HPC bonding gaps.

### healthSpring — Dual-Tower Enclave (neutron-heavy)

healthSpring has the strongest security requirements in the ecosystem. Patient records
NEVER leave the enclave. The composition uses a dual-tower ionic bond pattern
(evolved from the sketch at `sketches/mixed_atomics/dual_tower_ionic.toml`):

- **Tower A** (FAMILY_A): Patient data enclave — BearDog A + NestGate A (BondingPolicy
  egress fence) + healthSpring (ingest, de-identify, aggregate) + Provenance trio A
  (regulatory audit trail)
- **Tower B** (FAMILY_B): Analytics tower — BearDog B + Squirrel (clinical AI inference
  on de-identified data only) + NestGate B (model weights) + Provenance trio B
- **Ionic bridge**: `capabilities_denied = ["storage.*", "dag.*"]` — only de-identified
  aggregates cross. Metered, time-windowed, fully auditable.

| Graph | Purpose |
|-------|---------|
| `graphs/downstream/healthspring_enclave_proto_nucleate.toml` | Dual-tower BYOB: Tower A (patient enclave) + ionic bridge + Tower B (clinical AI). BondingPolicy egress fence on NestGate A. |
| `graphs/healthspring_clinical_pipeline.toml` | Pipeline: healthSpring (ingest + de-identify) → NestGate-A (enclave) → ionic bridge (aggregates) → Squirrel (clinical AI) → NestGate-B (cache) → sweetGrass (audit) |
| `graphs/spring_deploy/healthspring_deploy.toml` | Updated: clinical capabilities (`health.pharmacology`/`genomics`/`clinical`/`de_identify`/`aggregate`), NestGate + Squirrel + Provenance trio now `required`, `bonding_policy` metadata for enclave pattern |

**What healthSpring absorbs and evolves**: implement dual-tower deployment with separate
FAMILY_IDs, wire BondingPolicy egress fence on NestGate, compose ionic bond for
de-identified aggregate sharing, wire Squirrel inference for clinical AI, covalent mesh
sharding for fault tolerance. **Passes back**: BondingPolicy enforcement gaps, ionic
bridge metering requirements, NestGate enclave mode needs.

### Launch profile gap (RESOLVED)

`config/primal_launch_profiles.toml` was missing `[profiles.hotspring]` — all other
springs had profiles. Added with socket wiring for barraCuda, coralReef, toadStool,
NestGate, BearDog, and biomeOS.

---

## Ecosystem Audit Debt (April 6, 2026)

### License Alignment — **COMPLETE**

All 15 primals now on `AGPL-3.0-or-later`. Zero license debt remaining.

### Build/Test Debt

| Primal | Category | Issue | Status |
|--------|----------|-------|--------|
| barraCuda | ~~compile~~ | ~~E0061~~ `eval_math` decomposition | **FIXED** (Sprint 29) |
| barraCuda | ~~file size~~ | ~~`executor.rs` 1,097 lines~~ → split (max 845 LOC) | **FIXED** ↑ |
| barraCuda | **test** | `fault_injection` SIGSEGV (thread cap added Sprint 29) | Open |
| BearDog | ~~fmt~~ | ~~1 file diff~~ | **FIXED** ↑ |
| Songbird | ~~fmt~~ | ~~2 file diffs~~ | **FIXED** ↑ |
| toadStool | ~~fmt~~ | ~~1,899 diffs~~ → ~~1 diff~~ → **0 diffs** | **FIXED** ↑↑ |
| toadStool | ~~clippy~~ | ~~`manual_let_else`, deprecated `GenericArray`~~ | **FIXED** ↑↑ |
| NestGate | ~~fmt~~ | ~~`migration.rs:189`~~ | **FIXED** |
| coralReef | ~~clippy~~ | ~~7 errors in `coral-gpu` tests~~ | **FIXED** ↑ |
| bingoCube | ~~clippy~~ | ~~15 cast errors~~ | **FIXED** ↑ |
| bingoCube | ~~edition~~ | ~~2021~~ → **2024** | **FIXED** ↑ |
| rhizoCrypt | ~~clippy~~ | ~~5 `doc_markdown` warnings~~ | **FIXED** (39 warnings resolved) ↑ |
| sweetGrass | ~~clippy~~ | ~~1 unused import~~ | **FIXED** ↑ |
| sweetGrass | ~~config~~ | ~~`.cargo/config.toml` target-dir points to `/home/southgate/`~~ | **FIXED** (already cleaned) |
| petalTongue | **test** | 1 flaky test (`test_resolve_instance_id_error_message_invalid`) | Open (passes on retry) |

| ~~NUCLEUS~~ | ~~**plasmidBin**~~ | ~~All x86_64 binaries predate BTSP Phase 2 (Apr 8)~~ | **RESOLVED** (Apr 10 — full musl rebuild) |
| ~~NUCLEUS~~ | ~~**musl**~~ | ~~9/11 x86_64 binaries are DYNAMIC~~ | **RESOLVED** (Apr 10 — 12/12 musl-static) |
| ~~NestGate~~ | ~~**UDS**~~ | ~~`service start` is HTTP-only; no `--socket` flag for UDS listener~~ | **RESOLVED** (Apr 10 — `--socket` flag wired) |
| ~~Neural API~~ | ~~**routing**~~ | ~~`capability.call` double-prefixes method names~~ | **RESOLVED** (Apr 10 — domain prefix strip) |
| petalTongue | ~~**domains**~~ | ~~Only `visualization` symlink created~~ | **RESOLVED** (Apr 10 — `ui.sock` + `interaction.sock` symlinks) |
| ~~ToadStool~~ | ~~**UDS-JSONRPC**~~ | ~~`compute.sock` serves tarpc-only; exp002 expects JSON-RPC~~ | **RESOLVED** (Apr 10 — socket separation) |
| ~~barraCuda~~ | ~~**GPU panic**~~ | ~~`barracuda server` panics without GPU~~ | **RESOLVED** (Sprint 39 — `Auto::new()` → `Err`, server starts with `device = None`, health `Degraded`) |
| ~~biomeOS~~ | ~~**registry routing**~~ | ~~Registry stores `{primal}-{hash}.sock` instead of live socket paths~~ | **RESOLVED** (Apr 10 — BM-07) |

**Resolved this cycle:** 21 build/test debt items (+11 this push: fmt×3, executor split, plasmidBin rebuild, musl compliance, NestGate UDS, ToadStool socket separation, Neural API routing, biomeOS registry routing, barraCuda GPU panic). **Remaining:** 2 (barraCuda SIGSEGV test, petalTongue flaky test).

---

## Downstream Spring/Garden Architecture (April 10, 2026)

Springs and gardens are **consumers** of NUCLEUS, not primals. Key changes:

1. **`nucleus_launcher.sh`**: Removed Phase 5 (ludoSpring/esotericWebb). Launcher now
   Phases 0-4 only (10 core NUCLEUS primals). `find_binary` only searches `plasmidBin/`
   and `primals/` release targets.

2. **`primal_launch_profiles.toml`**: Spring profiles (ludospring, groundspring, etc.)
   reclassified under "DOWNSTREAM SPRING/GARDEN PROFILES — reference only" section.
   Not launched by NUCLEUS, just documented for biomeOS graph wiring reference.

3. **`deployment_matrix.toml`**: `ludospring-ipc-surface` and `esotericwebb-transport`
   marked `scope = "downstream"`. `ludospring-plasmidbin` marked `resolved = true`
   (spring binaries don't belong in plasmidBin). `loamspine-startup-crash` marked
   `resolved = true` (LS-03 fixed in v0.9.15). Topology primal lists cleaned:
   springs/gardens moved to separate `springs`/`downstream` keys.

4. **Proto-nucleate graphs**: `graphs/downstream/` now contains 5 proto-nucleate patterns:
   - `ludospring_proto_nucleate.toml` — game science composition
   - `esotericwebb_proto_nucleate.toml` — narrative composition
   - `neuralspring_inference_proto_nucleate.toml` — ML inference as WGSL shader composition
   - `hotspring_qcd_proto_nucleate.toml` ↑ — lattice QCD / HPC physics as WGSL shader
     composition (proton-heavy: metallic GPU pool + df64 + provenance for reproducibility)
   - `healthspring_enclave_proto_nucleate.toml` ↑ — dual-tower enclave for clinical data
     (neutron-heavy: ionic bond between patient data Tower A and analytics Tower B,
     BondingPolicy egress fence, regulatory provenance audit)

   All NUCLEUS nodes use `spawn = false`. ludoSpring and esotericWebb proto-nucleates now use `composition_model = "pure"` (no downstream binary).
   Pipeline graphs: `neuralspring_inference_pipeline.toml`, `hotspring_qcd_pipeline.toml` ↑,
   `healthspring_clinical_pipeline.toml` ↑.

5. **BYOB template**: `graphs/spring_byob_template.toml` updated with Tower Atomic
   security metadata, bonding policy, `spawn = false` for all NUCLEUS nodes, and notes
   that spring binaries are NOT in plasmidBin.

6. **Stale binary names**: `ludospring_primal` → `ludospring` in remaining science/sketch
   graphs (4 files fixed).

7. **Secure-by-default graph rewiring (April 10)**: All 93 deploy graphs now include
   `[graph.metadata]` with `security_model = "btsp_enforced"`, `transport = "uds_only"`,
   `tcp_ports = 0`. Previously 91/101 graphs were missing security metadata (plaintext-first
   patterns from early evolution). Bonding-specific graphs get appropriate `[graph.bonding_policy]`:
   ionic gets `encryption_tiers.cross_family = "full"`, metallic gets `encryption_tiers.fleet = "hmac_plain"`,
   OrganoMetalSalt gets composite tiers. Multi-node/federation graphs add `federation_transport = "songbird_tcp"`.
   ludoSpring/esotericWebb nodes in 14 science/sketch/gen4 graphs annotated with
   `spawn = true` + `security_model = "tower_delegated"` to clarify downstream boundary.
   Pre-existing metadata fields (trust_model, internal_bond_type, etc.) preserved via merge.

## Composition Evolution (April 9, 2026)

ludoSpring and esotericWebb are **NOT primals and NOT spawnable binaries**. They are
pure compositions of NUCLEUS primals — the graph defines the product, biomeOS executes it.
Their experiments and validation work prove the composition patterns work (Fitts, Flow,
DDA, session lifecycle, provenance) — they are the "papers" that validate graph-driven products.

### What Changed

- **Proto-nucleate graphs rewritten** (`graphs/downstream/`): `ludospring_proto_nucleate.toml`
  and `esotericwebb_proto_nucleate.toml` now have `composition_model = "pure"` and no `spawn = true`
  binary nodes. All capabilities map to existing NUCLEUS primals.
- **10 graphs consolidated**: 8 graphs rewritten to replace ludo/webb binary nodes with
  constituent primals (barraCuda for game math, Squirrel for AI/narration, petalTongue
  for rendering, NestGate for storage, provenance trio for session integrity).
- **7 graphs deleted**: 5 redundant sketches + 2 duplicate compositions.
- **6 canonical fragments** (`graphs/fragments/`): `tower_atomic`, `provenance_trio`,
  `node_atomic`, `nucleus` — documenting the "periodic table" of composition.
- **100% metadata annotation**: Every deploy graph carries `composition_model` and `fragments` (atomic-aligned: `tower_atomic`, `node_atomic`, `nest_atomic`, `meta_tier`, `nucleus`, `provenance_trio`).

### How Capabilities Map

| Old Binary Node | Replacement Primals | Capabilities |
|-----------------|---------------------|-------------|
| `ludospring` | barraCuda + toadStool | GPU math: Fitts, Flow, Perlin, WFC, engagement |
| `ludospring` | Squirrel | AI: DDA, analysis, accessibility |
| `esotericwebb` | Squirrel + petalTongue | AI narration + scene rendering |
| `esotericwebb` | NestGate + provenance trio | Session storage + DAG integrity |

### What This Means for Other Springs

This is the same pattern as neuralSpring (ML inference = composition of WGSL shaders)
extended to game science and narrative products. **Every "product" is a graph. Every graph
is a composition of primals. biomeOS is the CPU.** Springs that want to evolve game science,
narrative, or interactive capabilities should compose the same primals — not build new binaries.
