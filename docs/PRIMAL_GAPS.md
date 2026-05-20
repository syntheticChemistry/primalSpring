# Primal Gap Registry

Structured inventory of known gaps per primal that block or degrade composable deployments.

> **Scope**: NUCLEUS primals only (13 core + compute/ecosystem primals).
> Downstream springs and gardens own their own debt and are NOT tracked here.
>
> **Current phase: INTERSTADIAL** — stadial gate cleared April 16, 2026.
> All 13 primals at modern async Rust parity: `async-trait` eliminated (13/13),
> enum dispatch (13/13), `cargo deny check bans` passes (13/13), Edition 2024 (13/13).
>
> **Last updated**: 2026-05-20 PM (Wave 30: sporePrint **15/15** complete, SP-1 auto-merge shipped, CM-3 cross-gate scenario (45th). wetSpring V182 ingested: UniBin consolidation (349→1 binary, 345 scenarios), WS-11 v3 MAPQ calibration, Tenaillon batch 0 COMPLETE (5/5). plasmidBin `validate/` v0.2.0 serde-typed. Wave 29 CM-1/CM-2/CM-3/CM-4 all RESOLVED.)
>
> **Full history**: archived in `fossilRecord/primal_gaps_phase60_may2026/PRIMAL_GAPS_FULL_HISTORY.md`

---

## Ecosystem Status (May 18, 2026)

**267+ PASS, 0 FAIL, 0 KNOWN_GAP** — projectNUCLEUS Phase 60+ validation, darkforest v0.2.1 (MEM-14-17 added). primalSpring: 45 scenarios (10 tracks, 3 tiers), 456 methods, 747 tests. Dark Forest Glacial Gate standard formalized (5-pillar security invariants). Sovereignty track added: membrane composition, sovereignty parity, content sovereignty. 14 atomic signal graphs. Wave 28 sporePrint: **15/15** primals contributed `sporeprint/` validation summaries. Wave 29 cellMembrane: **all gaps RESOLVED** (CM-1/CM-2/CM-3/CM-4). SP-1 auto-merge shipped. plasmidBin validate v0.2.0 serde-typed.

| Primal | Tests | JH-0 | BTSP P3 | Wire Std | Debt Status |
|--------|------:|:----:|:-------:|:--------:|-------------|
| bearDog | 14,784+ | **ADOPTED** | FULL | L2 | **CLEAN** — HSM mock `#[cfg(test)]` (Wave 98) |
| songbird | 7,178+ | **ADOPTED** | FULL | L3 | **CLEAN** — DF-3 CallerContext wired (TCP transport-aware) |
| toadStool | 23,000+ | **ADOPTED** | FULL | L3 | **CLEAN** — S265: `sovereign.pmu_investigate` RPC, VFIO PMU probe. Phase D factory wired (S254). AMD live, NV FECS-gated. 85 JSON-RPC methods. |
| biomeOS | 7,924+ | **ADOPTED** | FULL | consumer | **CLEAN** — v3.57: Neural API evolution complete. `signal.dispatch` composition collapse, `primal.announce` atomic self-registration, metrics tagging with signal namespaces, `capability.call` signal-tier interception. v3.54 `biomeos.spring_status` for Tier 2. |
| nestgate | 12,393+ | **ADOPTED** | FULL | L3 | **CLEAN** — S66 shadow S3 readiness: `content.resolve` index.html path normalization (trailing-slash + bare-path fallback), `resolved_in_ms`/`retrieved_in_ms` timing metadata for TTFB. |
| squirrel | 7,178 | **ADOPTED** | FULL | L2 | **CLEAN** — 1105L test split, inference dispatch (P7) |
| barraCuda | 4,422+ | **ADOPTED** | FULL | L2 | **CLEAN** — Sprint 68: 72-method coverage, TensorSession `sub`/`negate` (GAP-HS-027), registry assertion softened to `>= 70` |
| petalTongue | 6,297+ | **ADOPTED** | FULL | L2/L3 | **CLEAN** — v1.6.6: 55 IPC methods, S3 shadow parity, musl targets |
| rhizoCrypt | 1,642+ | **ADOPTED** | FULL | L3 | **CLEAN** — S68: `dag.session.get` enriched with `agents`/`genesis`/`frontier` (covers `dag_summary` proposal). GAP-36 aliases shipped. 93.88% coverage. |
| loamSpine | 1,523+ | **ADOPTED** | FULL | L3 | **CLEAN** — v0.9.16: 42 methods, chain anchoring spec (Bitcoin/Ethereum/RFC 3161) |
| sweetGrass | 1,553 | **ADOPTED** | FULL | L3 | **CLEAN** — v0.7.37: PID file, JH-0 gate + port 9850 canonical |
| coralReef | 4,506+ | **ADOPTED** | FULL | L2 | **CLEAN** — eprintln→tracing in 5 driver files (Iter 95) |
| skunkBat | 363+ | **ADOPTED** | FULL | L2 | **CLEAN** — JH-5 Phase 2 event instrumentation complete |

**13/13 at zero debt. Waves 1-29 complete. 45 scenarios (10 tracks), 456 methods. Zero panics in production. Wave 20: `primal.list` canonical schema, `capability.list` standardization, `s_schema_standard` + `s_nest_commit_live` scenarios, Thread 10 provenance wiring. All delta springs resolved Wave 20 debt (May 17). Dark Forest Glacial Gate: 5-pillar security invariant standard. Sovereignty track: membrane composition + parity + content sovereignty. Neural API Signal Elevation. Eukaryotic validation with shared helpers, atomic signal layer, bootstrap graph.**

---

## Debt Exposed by lithoSpore Downstream Audit (May 17, 2026)

lithoSpore is the ecosystem's first deployed consumer — a USB artifact with 75/75
science checks, 117 tests, and Tier 3 JSON-RPC provenance wiring. Their audit
surfaced 8 requests. R1–R4 resolved in primalSpring, R5–R8 require upstream action.

### Resolved (primalSpring local)

| # | Request | Resolution |
|---|---------|------------|
| R1 | Document degradation behavior | Degradation table in `CompositionContext` module docs |
| R2 | Freeze/version method names | `stability` tiers in `capability_registry.toml` |
| R3 | Trio transaction semantics | Partial completion rules in `PROVENANCE_TRIO_INTEGRATION_GUIDE.md` |
| R4 | UDS socket ownership | Ownership table in `CAPABILITY_BASED_DISCOVERY_STANDARD.md` |

### Upstream-Blocked (awaiting primal teams)

| # | Request | Owner | Priority |
|---|---------|-------|----------|
| ~~R5~~ | ~~`nest.store` signal dispatch~~ | biomeOS | ~~MEDIUM~~ **RESOLVED** — v3.63: all 16 signal methods promoted to first-class route table entries. 10 signal tests. |
| ~~R6~~ | ~~Ferment transcript braids~~ | wetSpring | ~~HIGH~~ **RESOLVED** — Barrick 2009 SEALED (7/7 clones, 486 sovereign variants, USB to lithoSpore May 19). Tenaillon 2016 queued (264 clones, 590 GB). |
| ~~R7~~ | ~~`spore.instantiate` atomic VM provisioning~~ | biomeOS | ~~LOW~~ **DEFERRED-TO-STADIAL** — v3.63: route/graph scaffold, handler includes `_deferred` context. Wire when lithoSpore Tier 3 ready. |
| ~~R8~~ | ~~`capability.list` complete inventory~~ | All primals | ~~LOW~~ **RESOLVED** — all primals now return canonical `{ capabilities, count, primal }` envelope (Wave 22 stadial push) |
| R9 | Stale socket cleanup on startup | biomeOS | MEDIUM — **ABSORBED** (biomeOS CHANGELOG confirms socket hygiene) |
| R10 | Stale socket cleanup on startup | songbird | LOW — **ABSORBED** (songbird CHANGELOG confirms socket hygiene) |
| R11 | PID file alongside socket | All primals | LOW — deprioritized (consumer-side connect-probe provides equivalent liveness; toadStool explicitly declined, others rely on unlink-before-bind) |
| ~~R12~~ | ~~`doctor.sh` stale socket checker~~ | plasmidBin | ~~LOW~~ **RESOLVED** — stale socket detection section added to `doctor.sh` (fuser + python3 fallback) |

### Resolved Locally (primalSpring — May 18, 2026)

| Issue | Resolution |
|-------|------------|
| Stale socket discovery (wetSpring report) | `socket_is_alive()` connect-probe replaces `path.exists()` in all discovery paths (`discover_primal`, `discover_by_capability`, `NeuralBridge::discover`). Dead socket negative cache (`DEAD_SOCKET_CACHE`) prevents repeated ~100ms probe costs. CAPABILITY_BASED_DISCOVERY_STANDARD updated to v1.3.0 (§5-6). |
| plasmidBin `doctor.sh` stale socket check (R12) | Stale socket detection section added — scans `$XDG_RUNTIME_DIR/biomeos/` and `/tmp/biomeos/` for `.sock` files without listeners. Uses `fuser` with `python3` connect-probe fallback. Reports live/stale counts, provides cleanup tip. JSON output includes `sockets_live`/`sockets_stale`. |
| plasmidBin `stop_gate.sh` post-kill cleanup | Cleans stale sockets from `biomeos/`, `ecoprimals/`, `/tmp/biomeos/` after killing primals. Prevents the exact scenario wetSpring observed (50+ sockets left after processes died). |
| plasmidBin `start_primal.sh` pre-start cleanup | Removes stale socket at `--socket` path (if no listener via `fuser`) before primal binds. Prevents `EADDRINUSE` on restart after crash. |

### Stale Socket Blurb Absorption (May 18, 2026 sweep)

**14/14 primals absorbed** the stale socket cleanup blurb. All confirmed `unlink()` before `bind()` at bind sites and/or implemented shutdown cleanup. barraCuda fix landed (Sprint 70 hotfix, `transport.rs`).

| Primal | Status | Notes |
|--------|--------|-------|
| bearDog | **ABSORBED** | unix_socket_fault_tests + integration tests |
| biomeOS | **ABSORBED** | CHANGELOG + CURRENT_STATUS confirms |
| coralReef | **ABSORBED** | ecosystem.rs + tarpc_transport + advanced tests |
| loamSpine | **ABSORBED** | CHANGELOG + uds.rs cleanup |
| nestgate | **ABSORBED** | socket_config.rs + isomorphic_ipc server |
| petalTongue | **ABSORBED** | unix_socket_server + server cleanup |
| rhizoCrypt | **ABSORBED** | CHANGELOG + uds.rs + uds_tests |
| skunkBat | **ABSORBED** | ipc/mod.rs cleanup |
| songbird | **ABSORBED** | CHANGELOG + platform/unix.rs + android.rs |
| sourDough | **ABSORBED** | scaffold template generates clean server.rs |
| squirrel | **ABSORBED** | CHANGELOG + DEPLOYMENT_GUIDE |
| sweetGrass | **ABSORBED** | CHANGELOG + uds.rs + roundtrip tests |
| toadStool | **ABSORBED** | S264: 6/6 bind sites audited, CLI daemon + DisplayServer gaps FIXED, 9,028 tests |
| barraCuda | **ABSORBED** | `transport.rs`: `remove_file` before `bind` at both sites + legacy symlink cleanup (post-Sprint 70 hotfix) |

**14/14 primals confirmed stale-socket-clean.** No remaining upstream action items.

### Downstream-Blocked (awaiting spring teams)

| Gap | Spring | What | Priority |
|-----|--------|------|----------|
| Ferment braids: Tenaillon 2016 | wetSpring | breseq on 264 genomes, trio provenance, braid handoff to lithoSpore. 590 GB, 312 accessions, 524 FASTQs. Requires `compute.fan_out` (toadStool). | HIGH |
| ~~Ferment braids: Barrick 2009~~ | wetSpring | ~~breseq on 7 clones~~ **SEALED** — 7/7 clones, 486 sovereign vs 569 breseq (0.85 ratio), USB handoff May 19. L1 vs L2 parity documented. | ~~HIGH~~ **DONE** |
| Cross-tier parity adoption | all springs with dual-language validation | `litho parity` pattern — Python vs Rust numerical agreement | MEDIUM |
| Thread 4 expression seeding | wetSpring / airSpring | Environmental genomics targets for projectFOUNDATION | MEDIUM |

### wetSpring Active Gaps (ingested May 19, 2026)

From `wetSpring/GAPS.md` — issues that route to primalSpring or upstream teams:

| # | Gap | Owner | Priority | Status |
|---|-----|-------|----------|--------|
| WS-1 | Ionic contract negotiation — no automated protocol for establishing/modifying/terminating bonds | primalSpring Track 4 | HIGH | Scaffolded, no protocol |
| WS-2 | Cross-spring data exchange (RootPulse semantic function) — no remote pull protocol for provenance-wrapped subsets | biomeOS + trio | HIGH | **IN PROGRESS** — biomeOS v3.64: `nest.sync` 6-node graph shipped. loamSpine: `spine.list`/`entry.list` RPC methods (42 methods total). Live orchestration wiring pending. |
| WS-3 | Public chain anchor — braids have no public verifiable ledger anchor | loamSpine | MEDIUM | **SPEC** — `specs/PUBLIC_TIMESTAMPING.md`: RFC 3161 TSA, Bitcoin OP_RETURN, Ethereum analyzed. `AnchorTarget::Rfc3161Tsa` variant added. Implementation timeline open. |
| WS-4 | petalTongue client-side WASM — all grammar rendering requires live HPC | petalTongue | MEDIUM | Not started |
| WS-9 | Cross-tier parity — L1 vs L2 documented (0 position overlap), L2 vs L3 pending, coordinate mismatch | wetSpring | MEDIUM | L1/L2 done, L3 pending |
| WS-11 | Variant caller parity — sovereign over-calls vs breseq | wetSpring | HIGH | **v2 deployed** (V180) — GPU min_depth wired, MAPQ≥10 filtering, ±5bp window matching, duplicate removal, CPU mapping threshold 250bp. Tenaillon batch 0: 2/5 clones validated. Re-measurement pending. |

**Note**: WS-8 (ferment transcript pipeline) and WS-10 (stale socket) are RESOLVED. WS-5 (ludoSpring), WS-6 (hotSpring physics), WS-7 (radiating attribution) are Phase 3-4 future work — not tracked here.

### sporePrint External Surface — Wave 28 (added May 20, 2026)

sporePrint (primals.eco) is treated as a validation target, not a separate
maintenance artifact. Each primal contributes incrementally via existing patterns.
Automation reaches sporePrint the same way it reached everything else — via glacial
pressure. primalSpring validates the surface structurally via `s_sporeprint_surface`.

**Per-primal contribution status**:

| Primal | `sporeprint/` dir | `notify-sporeprint.yml` | config.toml entity | Status |
|--------|:-----------------:|:----------------------:|:------------------:|--------|
| bearDog | **Yes** | Yes | Yes | **Complete** — 248+ tests, 126 methods, S1 shadow LIVE |
| songbird | **Yes** | Yes | Yes | **Complete** — 7,803 tests, 54 methods, VPS relay deployed |
| skunkBat | **Yes** | Yes | Yes | **Complete** — 382 tests, 17 methods, defense meta-primal |
| toadStool | **Yes** | Yes | Yes | **Complete** — 9,028+ tests, 85 methods, Node Atomic ready |
| barraCuda | **Yes** | Yes | Yes | **Complete** — 4,393+ tests, 75 methods, precision ladder |
| coralReef | **Yes** | Yes | Yes | **Complete** — 3,181 tests, 16 methods, A++ pure compiler |
| nestGate | **Yes** | Yes | Yes | **Complete** — 12,393 tests, S3 shadow ready |
| rhizoCrypt | **Yes** | Yes | Yes | **Complete** — 1,642 tests, 32 methods, 93.88% coverage |
| loamSpine | **Yes** | Yes | Yes | **Complete** — 1,523 tests, 42 methods, chain anchoring spec |
| sweetGrass | **Yes** | Yes | Yes | **Complete** — 1,553 tests, 37 methods, W3C PROV-O braids |
| biomeOS | **Yes** | Yes | Yes | **Complete** — 7,924+ tests, 27 domains, v3.64 |
| squirrel | **Yes** | Yes | Yes | **Complete** — 7,089+ tests, 38 methods, 90% coverage |
| petalTongue | **Yes** | Yes | Yes | **Complete** — 6,297+ tests, 55 methods, S3 shadow parity |
| sourDough | **Yes** | Yes | Yes | **Complete** — 281 tests, CLI meta-primal, 95% coverage |
| bingoCube | **Yes** | Yes | Yes | **Complete** — 73 tests, cryptographic commitment library |

**sporePrint infra gaps** (owned by sporePrint/primalSpring):

| # | Gap | Owner | Priority | Status |
|---|-----|-------|----------|--------|
| ~~SP-1~~ | ~~Auto-merge: Tier 2 content auto-commits after `spore-validate` passes~~ | sporePrint CI | ~~MEDIUM~~ | **RESOLVED** — auto-refresh.yml content job now auto-commits when spore-validate passes; falls back to PR on validation failure |
| SP-2 | Deploy status fields in `config.toml` (`last_push`, `shadow_status`, `deploy_locations`) | sporePrint | MEDIUM | Not started |
| SP-3 | `liveSpore.json` auto-ingest from trio-equipped deployments | sporePrint CI | LOW | Pipeline exists, feed source pending |
| SP-4 | Sovereign publish: `publish_sporeprint.sh` → NestGate `content.put` | projectNUCLEUS | LOW | Script exists — **blocked on bearDog `content.*` scope** (MethodGate rejects without `content.put` in session token) |

### cellMembrane Nest Expansion — Wave 29 (added May 20, 2026)

Expand cellMembrane VPS from Tower Atomic (3 primals) to Nest Atomic
(+ nestGate, rhizoCrypt, loamSpine, sweetGrass). Enables cross-boundary
composition testing and trio-verified deployments.

| # | Gap | Owner | Priority | Status |
|---|-----|-------|----------|--------|
| ~~CM-1~~ | ~~`deploy_membrane.sh --composition nest`~~ | plasmidBin | ~~MEDIUM~~ | **RESOLVED** — `--composition nest` added: fetches nestgate/rhizocrypt/loamspine/sweetgrass, generates systemd units, opens ports, wires Tower dependency chain |
| ~~CM-2~~ | ~~`membrane_provenance.sh` post-deploy trio hook~~ | projectNUCLEUS | ~~MEDIUM~~ | **RESOLVED** — `deploy/membrane_provenance.sh` shipped (5-phase remote trio verification, graceful degradation, report generation) |
| ~~CM-3~~ | ~~Cross-gate `capability.call` testing~~ | primalSpring + songbird | ~~LOW~~ | **RESOLVED** — `s_cross_gate_capability_call` scenario (45th): membrane relay channel, wire contract, local + cross-gate dispatch |
| ~~CM-4~~ | ~~darkforest MEM-14 through MEM-17 (Nest health)~~ | projectNUCLEUS | ~~LOW~~ | **RESOLVED** — MEM-14 through MEM-17 added to `darkforest_membrane.sh` (NestGate, rhizoCrypt, loamSpine, sweetGrass liveness checks, 17 PASS / 0 FAIL / 5 SKIP) |

---

## Upstream Gap Reconciliation (projectNUCLEUS May 9, 2026)

Post-deep-debt-sweep reconciliation from downstream `projectNUCLEUS`:

### Resolved

| ID | What | Resolution |
|----|------|------------|
| DF-2 | toadStool `TOADSTOOL_AUTH_MODE` env | toadStool S233 — `auth.mode` env + `eprintln` → `tracing` |
| DF-3 | songbird/squirrel silent on `auth.mode` TCP | songbird — `CallerContext` wired (TCP transport-aware) |
| U5 | sweetGrass port 39085 vs 9850 | sweetGrass v0.7.32 — port 9850 canonical |
| GAP-12 | 15 ludoSpring IPC methods need canonical registration | **RESOLVED** — 28 `game.*` methods in `config/capability_registry.toml` (456 total, zero drift) |
| U1 | CHECKSUMS stale after Phase 59 refactoring | **RESOLVED** — regenerated with 25 tracked files (UniBin, certification, scenarios, registry) |
| U2 | 5 deploy graphs missing `by_capability` | **FALSE POSITIVE** — only manifests (parameter tables, not node-bearing graphs) lack field; all actual `[[graph.nodes]]` graphs have `by_capability` |
| U3 | 8 profile graphs missing `bonding_policy` | **RESOLVED** — 9/9 profile graphs already have `bonding_policy` |

### Resolved (upstream evolution wave May 10, 2026)

| ID | Owner | What | Resolution |
|----|-------|------|------------|
| JH-11 | bearDog/biomeOS | Cross-primal token federation | **RESOLVED** — bearDog Wave 99 `auth.public_key` (Ed25519 key distribution) + biomeOS v3.51 `BearDogVerifier` (IPC-based cross-primal verification) |
| GAP-06 | rhizoCrypt | No UDS transport | **RESOLVED** — S66 confirms UDS operational since S23, provenance trio integration test added |
| GAP-03 | biomeOS | Cell graph live deploy not tested | **RESOLVED** — biomeOS v3.51 `composition.deploy` route alias for `graph.execute` |
| GAP-09 | biomeOS | Neural API registration endpoint | **RESOLVED** — biomeOS v3.51 `method.register` endpoint for spring method registration |

### Resolved (glacial debt escalation May 13, 2026)

| ID | Owner | What | Resolution |
|----|-------|------|------------|
| GAP-36 | rhizoCrypt | `provenance.*` methods returned -32601 | **RESOLVED** — S68 `normalize_method()` maps 21 `provenance.*` → `dag.*` aliases. 1,637 tests. |
| GAP-36 | loamSpine | `session.*` methods returned -32601 | **RESOLVED** — v0.9.16 `normalize_method` aliases `session.*` → `spine.*`. Handoff shipped. 1,522 tests. |
| GAP-36 | sweetGrass | `braid.attribution.create` returned -32601 | **RESOLVED** — v0.7.35 alias table, `dispatch_classified()`. 1,549 tests, 91.7% coverage. |
| GAP-35 | loamSpine | `entry.append` vs `session.create` | **RESOLVED** — both coexist: `entry.*` = entry CRUD, `spine.*` = ledger lifecycle, `session.*` = aliases. |
| GAP-34 | biomeOS/nestGate | `content.*` vs `storage.*` naming | **CONFIRMED INTENTIONAL** — distinct domains (CAS vs blob). biomeOS v3.53. |
| GAP-16 | Tower primals | Tower not deployed locally | **RESOLVED** — ludoSpring V70 live-validated 6/6 Tower capabilities (crypto fingerprint/sign/verify/hash, mesh peers, audit log) against running bearDog + songbird + skunkBat. Wire corrections: bearDog uses base64 `message` param, skunkBat routes via `security.audit_log`. |
| — | toadStool | Phase D factory not wired | **RESOLVED** — S254 `LocalDeviceFactory` wired. AMD live, NV FECS-gated. 74 methods, 22,900+ tests. |
| — | barraCuda | Framework parity benchmarks | **RESOLVED** — Sprint 63: LAMMPS + SciPy + Kokkos benches, DF64 GPU E2E tests. |
| — | coralReef | `naga::Module` direct ingest | **RESOLVED** — `compile_module`/`compile_module_full` shipped. 3,129 tests. |
| — | biomeOS | Shadow deploy preflight | **RESOLVED** — v3.53 `composition.deploy.shadow` (dry-run validation, 3 routing tests). |
| — | petalTongue | `backend=nestgate` | **RESOLVED** — v1.6.6 `GET /` → `content.resolve("/")` + live dashboard SSE. |
| — | bearDog | Ionic lease (H2) | **RESOLVED** — Wave 102 `ttl_seconds`/`expires_at` on `sign_contract`/`verify_contract`. |
| — | songbird | `capability.resolve` (H2) | **RESOLVED** — Wave 199-201 wire parity. |

### Resolved (Neural API evolution May 15, 2026 — biomeOS v3.55–v3.57)

| ID | Owner | What | Resolution |
|----|-------|------|------------|
| — | biomeOS | Merge conflicts from upstream evolution | **RESOLVED** — v3.55: 5 conflict files resolved (capability_translation, capability handler, path_builder). `cargo check` clean. |
| — | biomeOS | Signal dispatch not wired | **RESOLVED** — v3.56: `signal.dispatch`/`signal.list`/`signal.schema` routes + `capability.call` signal-tier interception. 7 integration tests. Composition collapse active. |
| — | biomeOS | Metrics lack signal context | **RESOLVED** — v3.57: `GraphExecutor` tags metrics with signal namespace, extracts primal_id/operation from graph nodes. `PathwayLearner` signal-aware. |
| — | biomeOS | No atomic self-registration | **RESOLVED** — v3.57: `primal.announce` single-RPC registration (lifecycle + capabilities + translations + signal tiers). See `wateringHole/PRIMAL_ANNOUNCE_PROTOCOL.md`. |
| — | squirrel | No signal planning mode | **RESOLVED** — `signal_plan` mode for `ai.query`: ingests `signal_tools.toml`, decomposes intent into structured signal step sequences. |

Also resolved by upstream teams (not previously tracked as gaps):

| What | Resolution |
| `biomeos.spring_status` (projectNUCLEUS proposal) | **IMPLEMENTED** — biomeOS v3.54: binary discovery, workload counts, topology version. 3 tests. |
| `nestgate.artifact_query` (projectNUCLEUS proposal) | **COVERED** — NestGate Session 62: `content.get`/`content.exists` return provenance metadata (`source`, `pipeline`, `stored_by`). No separate method needed. |
| `rhizocrypt.dag_summary` (projectNUCLEUS proposal) | **COVERED** — rhizoCrypt S68: `dag.session.get` enriched with `agents`, `genesis`, `frontier` fields. Serves as canonical session summary. |
| barraCuda registry test off-by-one | **RESOLVED** — Sprint 68: assertion softened to `>= 70` (was exact `== 71`), covers 72-method registry including `precision.route`. |
|------|------------|
| `composition.status` method | biomeOS v3.51 — `{ active_users, primal_health, resource_pressure }` |
| bearDog TLS + rate limiting (H2-10/H2-11) | bearDog Wave 100 — rustls X.509 termination + per-IP sliding-window rate limiter |
| petalTongue PT-1 through PT-5 (sovereignty) | All resolved — `--docroot`, `WebServeConfig`, `--ipc`, `--workers`, NestGate content backend (PT-13) |
| petalTongue notebook rendering | `.ipynb` → HTML with `metadata.title` + `strip_sources` |
| songbird NAT traversal (H2-13 through H2-16) | Wave 196-197 — STUN wire-compliant, RFC 5766 TURN client, Cloudflare DDNS, 5-tier `ConnectionFallbackChain` all live |
| biomeOS token forwarding | v3.50 — `_bearer_token` propagated through all capability routing paths |

### Downstream-Surfaced Primal Debt (projectNUCLEUS May 11, 2026)

The deep debt sweep and sovereignty pre-wire exposed gaps that only become visible
when primals are composed in production. This is the sentinel-stadial model working:
downstream pressure propagates upward to expose primal gaps at the gate.

#### NestGate content.put — Transport Parity Gap — RESOLVED (Session 60, May 11)

**Original finding**: `content.*` methods were implemented on the primary
`unix_socket_server/dispatch.rs` path but **not routed** on SemanticRouter,
isomorphic IPC, or HTTP API — callers on those paths got "Method not found."

**Resolution (NestGate Session 60):** All 8 `content.*` methods (`put`, `get`,
`exists`, `list`, `publish`, `resolve`, `promote`, `collections`) now wired on
**all 4 transport surfaces**:

| Transport | `content.*` available? |
|-----------|----------------------|
| Primary `unix_socket_server/dispatch.rs` | **YES** (existing) |
| `SemanticRouter::call_method` | **YES** (new `content.rs` module) |
| Isomorphic IPC `UnixSocketRpcHandler` | **YES** (new delegation block) |
| `nestgate-api` HTTP (`NestGateRpcHandler` + `NestGateJsonRpcHandler`) | **YES** (new `handle_content_method` + transport handlers) |

Additionally: `lifecycle.status` handler added on all 4 surfaces. Public
`content_ops` facade created for stateless cross-crate access.

**Unblocks**: petalTongue `backend=nestgate`, projectNUCLEUS Pillars 1-3,
`publish_sporeprint.sh`, sovereign content pipeline, plasmidBin hosting.

#### Other Per-Primal Composition Debt

| Priority | Primal | Finding | Status | Blocks |
|----------|--------|---------|--------|--------|
| ~~LOW~~ | NestGate | ~~`storage.list` accessible without auth~~ (opaque hashes — BLAKE3 content-addressed, no metadata leak). Gate tests added: `nestgate_storage_list_returns_opaque_hashes`, `nestgate_storage_list_content_addressed` | **RESOLVED** — validated as low-risk by design; BTSP scoping deferred to Phase 2b as stretch goal | — |
| ~~MEDIUM~~ | toadStool | ~~IPC callers see no env var expansion~~ | **RESOLVED** (S234 — IPC contract documented as pre-resolved only) | — |
| ~~MEDIUM~~ | squirrel | ~~`LocalProcessProvider` dev stub, delegation not wired~~ | **RESOLVED** (`RemoteComputeProvider` for toadStool IPC delegation shipped) | — |
| ~~LOW~~ | barraCuda | ~~Embedded crypto deps for BTSP framing~~ | **RESOLVED** (bearDog Wave 101 `crypto.hkdf_sha256` + `crypto.hmac_verify` IPC surface) | — |
| ~~MEDIUM~~ | loamSpine | ~~`session.commit` API contract mismatch~~ | **RESOLVED** (method aliases + hex hash acceptance confirmed) | — |
| ~~LOW~~ | petalTongue | ~~`backend=nestgate` blocked on NestGate transport parity~~ | **RESOLVED** (NestGate Session 60 shipped transport parity; SPA + CORS already shipped) | — |

#### primalSpring Validation Gap — Why This Wasn't Caught

**This gap propagated to projectNUCLEUS because primalSpring's gate validates
structural consistency (methods registered, health alive) but not semantic
correctness (methods actually work across transports).** Specific failures:

1. ~~**`content` not in `ALL_CAPS` routing table**~~ — **FIXED (W7-01)**: `content`
   added to `ALL_CAPS` and wired to NestGate in `capability_to_primal()`.

2. ~~**Zero `content.*` scenarios**~~ — **FIXED (W7-02)**: `s_nestgate_content_pipeline`
   exercises `content.put` → `content.get` round-trip (BLAKE3 hash match),
   `content.exists`, `content.list`, and `content.resolve`.

3. ~~**Zero `content.*` tests in `server_ecosystem_compose.rs`**~~ — **FIXED (W7-03)**:
   Content Gate 1-3 tests added (put/hash, get/roundtrip, list/includes).

4. **Composition parity scenario** (`s_composition_parity`) tests `storage.store` →
   `storage.retrieve` round-trip — different API surface from `content.*`.
   (Not a bug — storage and content are different domains.)

5. ~~**418-method registry unexercised**~~ — **FIXED (W7-06)**: `check_method_coverage.sh`
   (inverse drift detection) reports 125/418 methods registered but never referenced in
   any scenario, test, or graph. CI-gatable. `content.put/get/exists/list/resolve` are
   now exercised; `content.collections/promote/publish` remain unexercised.

6. ~~**No deploy graph steps invoke `content.*`**~~ — **FIXED (W7-04)**:
   `content_pipeline_smoke.toml` uses `by_capability = "content"` for
   put + get + list round-trip.

**Root cause**: The primalSpring gate is a **structural** gate (methods enumerated,
health alive, graphs coherent) but lacks **contract tests** for the full NestGate
capability surface. The `content` domain was registered but never exercised.

**Required primalSpring evolution** (see Wave 7 below).

### Previously Resolved Gaps (for reference)

| Priority | Primal | Issue | Status |
|----------|--------|-------|--------|
| 1 | coralReef | `eprintln!` → `tracing` | Done (Iter 95) |
| 2 | barraCuda | `unwrap()` → `?` in session/ops | Confirmed false positive (optional dep) |
| 3 | nestGate | `unwrap()` → `?` in rpc/discovery | Confirmed false positive (S59) |
| 4 | biomeOS | Mock helpers mixed with production code | Done (v3.49 `#[cfg(test)]` isolation) |
| 5 | bearDog | HSM mock not feature-gated | Done (Wave 98 `#[cfg(test)]`) |
| 6 | petalTongue | Bare `#[allow]` without reason | Done (P6 `#[expect(reason)]`) |
| 7 | squirrel | 1105-line test file | Done (P7 inference dispatch split) |

---

## Next Interstadial Wave — Evolution Goals

These items are the active evolution targets for the next stadial push.
Delta springs have completed the interstadial primordial extinction (8/8
eukaryotic UniBin, May 9, 2026). projectNUCLEUS and downstream products
should absorb the current patterns while these goals mature upstream.

### Wave 1: JH-11 — Cross-Primal Token Federation

**RESOLVED** (May 10, 2026)

- bearDog Wave 99: `auth.public_key` endpoint — Ed25519 verifying key in base64/hex/DID
  formats. Any primal can call once, cache key, verify ionic tokens locally.
- biomeOS v3.50: `_bearer_token` propagated through all capability routing paths.
- biomeOS v3.51: `BearDogVerifier` for IPC-based cross-primal token verification.
  Degrades gracefully to local parsing when bearDog unreachable.
- primalSpring: `TokenVerifier` trait, `scope_permits_method()`, `call_authenticated()`,
  scenarios `s_bearer_token`/`s_gate_failure`/`s_gate_routing`, experiments exp108-111.

**JH-5 (audit forwarding) and Tier 4 rewiring are now unblocked.**

---

### Wave 2: JH-5 — Cross-Primal Audit Log Forwarding

**RESOLVED** (May 11, 2026)

- skunkBat Phase 3: cross-primal audit event forwarding shipped (`forwarding.rs` —
  308 lines, forwards security events to rhizoCrypt + sweetGrass via IPC).
- rhizoCrypt S67: composition readiness + payload_ref wiring + pipeline tests.
- sweetGrass v0.7.34: composition readiness + provenance trio pipeline validation.
- All 8 springs wired with skunkBat Rust IPC modules.

**JH-5 is fully shipped. The provenance trio pipeline (skunkBat → rhizoCrypt →
sweetGrass) is operational.**

---

### Wave 3: Primordial Extinction — Delta Spring Pattern Evolution

**Owner**: All delta springs (hotSpring, wetSpring, neuralSpring, healthSpring,
ludoSpring, groundSpring, airSpring)
**Priority**: HIGH — the primary interstadial work for delta teams
**Target**: Before next stadial gate

**COMPLETED** (May 9, 2026) — All 8 springs have completed the primordial extinction:

1. **UniBin consolidation**: 8/8 — all springs have single unified binaries with
   `certify`/`validate`/`status`/`version` subcommands (most also have `serve`).
2. **Guidestone absorption**: 8/8 — certification engine absorbed as library organelle.
3. **Deprecated API cleanup**: 8/8 — zero bare `#[allow(deprecated)]` suppressions.
4. **primalSpring v0.9.25 pin**: 7/8 pinned (ludoSpring pinned, healthSpring upgraded).
5. **Fossil record**: 8/8 — `fossilRecord/` with dated provenance READMEs.
6. **Zero debt**: 8/8 — zero TODO/FIXME/HACK, zero clippy warnings, zero test failures.

| Spring | Post-Evolution State | Next Target |
|--------|---------------------|-------------|
| healthSpring | V64k, gS L5, UniBin, 1,014+ tests, Tier 4 (`default=[]`), B5 COMPLETE, Nest atomic COMPLETE, GAP-36 RESOLVED both sides, Tier 2 wired | Nest niche depth, Nest deploy |
| ludoSpring | V70, gS L4, UniBin, 858 tests, Tier 4 (`default=["ipc"]`), **Tower atomic LIVE** (6/6) | Tower niche depth, coralReef wiring |
| hotSpring | v0.6.32, gS L6, UniBin, 591 tests, Tier 4 (`default=[]`), LTEE B2, 3-GPU sovereign | Node atomic deployment (strandGate + biomeGate) |
| wetSpring | V166b, gS L4, UniBin, 1,962 tests, Tier 4 (`default=[]`), LTEE B7 Tier 2, PG-03/05 RESOLVED | close PG-02/04 (deployment), gS L5 |
| airSpring | v0.10.0, gS L4, UniBin, 1,429 tests, Tier 4 (`default=[]`), NestGate+Squirrel IPC wired | LTEE E3, gS L5+ |
| neuralSpring | S204, gS L5, UniBin, 907 tests, Tier 4 (`default=[]`), LTEE B1, ~~Gap 11~~ CLOSED | NestGate weights, Tier 2 depth |
| groundSpring | V140, gS L4, UniBin, 1,123 tests, Tier 4 (`default=[]`), LTEE B1-B4 **DONE** | lithoSpore integration, coralReef IPC |

**Wave 3 COMPLETED** (May 9). Post-interstadial push (May 10-11) achieved:
8/8 skunkBat Rust IPC, 8/8 `method.register`, 8/8 CI cross-sync 418,
8/8 `composition.status`, 8/8 NUCLEUS workload TOMLs. Tier 4 rewiring
and `CompositionContext` migration now **UNBLOCKED** by JH-11.

---

### Wave 4: PG-63 — Matplotlib Agg Guidance Reconciliation — RESOLVED

**Owner**: sporePrint / wateringHole docs
**Priority**: ~~LOW~~ — **DONE** (May 11, 2026)

Both `CONTENT_GUIDE.md` and `SPRING_EVOLUTION_TARGETS.md` now consistently say
**"do NOT set `matplotlib.use('Agg')`"** (breaks inline rendering in JupyterHub
and nbconvert CI). The original conflict was resolved during the Phase 59
documentation wave. All 4 references across wateringHole are aligned.

---

### Wave 5: PG-54 — Adaptive Composition Tick Model — RESOLVED

**Owner**: primalSpring composition library + biomeOS
**Priority**: ~~LOW~~ — **DONE** (May 11, 2026)

`nucleus_composition_lib.sh` now supports three tick modes:
- **fixed** — constant `POLL_INTERVAL` (default, backward-compatible)
- **adaptive** — scales between `TICK_MIN` and `TICK_MAX` based on activity
  (fast when busy, exponential backoff when idle)
- **event** — no polling; for compositions using sensor stream file descriptors

Domain scripts set `TICK_MODE`, `TICK_MIN`, `TICK_MAX` before their main loop
and call `tick_sleep` / `tick_mark_active` / `tick_mark_idle`. ludoSpring can
now use `TICK_MODE=adaptive TICK_MIN=0.016` for 60Hz floor with idle ceiling.

---

## Compliance Summary

All 13 primals share these invariants (regressions are rejected):

| Invariant | Status |
|-----------|--------|
| `async-trait` eliminated | **13/13** |
| Enum dispatch (finite implementors) | **13/13** |
| `cargo deny check bans` (ring/openssl/aws-lc-sys banned) | **13/13** |
| Edition 2024 | **13/13** |
| JH-0 MethodGate pre-dispatch authorization | **13/13** |
| BTSP Phase 3 (ChaCha20-Poly1305 AEAD) | **13/13** |
| Capability Wire Standard L2+ | **13/13** |
| `--bind` / localhost-default (PG-55) | **13/13** |
| plasmidBin musl-static ecoBin | **13/13** |
| `forbid(unsafe_code)` or justified opt-out | **13/13** |

---

## Portability Posture

| Class | Issue | Status |
|-------|-------|--------|
| C Crypto (`ring`) | BearDog pure-Rust delegation, `deny.toml` bans | **RESOLVED** (13/13) |
| GPU/Vulkan (`wgpu`) | barraCuda 4-tier fallback (GPU→CPU→IPC→scalar) | **RESOLVED** |
| Remaining C surfaces | All feature-gated or target-gated | **ACCEPTABLE** |
| `ring` lockfile ghost | Cargo v4 artifact, never compiled | **NOT ACTIONABLE** |

---

## Per-Primal Quick Reference

Detailed per-primal gap tables, BTSP compliance matrices, capability wire standard
levels, plasmidBin binary inventory, and historical resolution logs are archived in:

`fossilRecord/primal_gaps_phase60_may2026/PRIMAL_GAPS_FULL_HISTORY.md`

Historical per-primal handoffs are in `infra/wateringHole/handoffs/archive/`.
All primal-specific stadial gate responses (May 17) have been fossilized after
absorption into Wave 22 coordination docs. See `primalSpring/wateringHole/handoffs/`
for the living coordination handoffs:
- `WAVE22_UPSTREAM_PRIMAL_EVOLUTION_MAY17_2026.md` — per-primal action items
- `WAVE22_STADIAL_GATE_PRIMAL_BLURB_MAY17_2026.md` — stadial gate sweep

---

## Evolution Cycle Ownership Model

Every gap in the ecosystem belongs to exactly one layer of the evolution cycle.
When a gap is identified, it should be tagged with its owner layer. This prevents
ambiguity about who acts on what, and which gaps block downstream work.

### Sentinel-Stadial Model (May 11, 2026)

Primals are **sentinels** — the least composed, most climate-responsive entities
in the ecosystem. They feel shifts first and respond first. They are already in
their own **stadial cycle**, with primalSpring as their **external validation
gate**. This is analogous to how Cloudflare/Barrick are stadial gates for
downstream products.

```
L1 (Primals — sentinel-stadial)
  │ validated against
  ▼
L2 (primalSpring — stadial gate for primals)
  │ 456 registry, MethodGate enforcement, deploy graph coherence,
  │ guidestone certification, CompositionContext contracts
  │
  │ patterns flow downstream
  ▼
L3 (Springs — interstadial) → L4 (Products — interstadial) → L5 (Foundation)
```

The key distinction: **primals are ahead of the ecosystem**. They have shipped
their capabilities. primalSpring is the **pressure** that validates quality — any
primal not passing the gate creates upstream debt that blocks everything
downstream. The river delta and products are still interstadial, absorbing
primal capabilities into compositions and deployments.

### Layer 1: Upstream Primals — Sentinel-Stadial (13 core primals)

**Owner**: Individual primal teams (bearDog, songbird, toadStool, etc.)
**Scope**: Primal-internal code quality, capability correctness, IPC contracts
**Phase**: **Stadial** — capabilities shipped, responding to gate pressure
**Current**: **13/13 passing the primalSpring gate** on structural AND semantic
invariants (MethodGate, BTSP, Edition 2024, deny.toml, plasmidBin, content
transport parity). **Zero critical upstream gaps.** NestGate transport parity
resolved (Session 60, May 11). All downstream-surfaced debt resolved.
See "Downstream-Surfaced Primal Debt" section above for full audit findings.

**Stadial pressure on primals** (primalSpring as gate):
- 418-method canonical registry — drift is rejected
- MethodGate enforcement — **13/13 DONE**
- Deploy graph coherence — all primals must compose cleanly
- Guidestone certification — primals participate in spring gS levels
- Upstream crate extraction (stadial external) — wgsl-precision, proc-sysinfo
- Framework parity (stadial external) — Kokkos, LAMMPS, SciPy benchmarks

### Layer 2: primalSpring — The Stadial Gate

**Owner**: primalSpring team
**Scope**: Canonical capability registry (456 methods), deploy graph library,
composition validation, gap registry, `CompositionContext` API, two-tier
validation harness (Tier 1 Rust / Tier 2 Live IPC), guidestone certification,
atomic signal graphs, eukaryotic validation scenarios with shared helpers
**Role**: **Stadial gate for L1 primals.** The registry, MethodGate check,
graph coherence, and guidestone layers are the validation pressure that
primals must pass. Patterns validated here flow downstream to springs/products.
**Current**: 456 methods, 744 tests, zero local debt. All waves complete:
- Wave 8: Compute trio composition — 9/9 DONE
- Wave 9: Domain contract sweep — 24 scenarios, 77 deploy graphs
- Wave 10: Phase 32 atomic model — all fragments v3.0.0
- Wave 11: Local debt resolution — CompositionContext migration, btsp.capabilities, 27 scenarios, 307/419 methods exercised (73%), Thread 10 seeded
- ~~PG-54~~ **DONE** (adaptive tick model shipped)
- ~~PG-63~~ **DONE** (Agg guidance already reconciled)

### Layer 3: River Delta — Interstadial (8 springs)

**Owner**: Individual spring teams
**Scope**: Domain science, spring-internal debt, barraCuda coupling, gS levels,
foundation seeding, plasmidBin release readiness
**Phase**: **Interstadial** — absorbing primal capabilities, pre-wiring compositions
**Current**: Post-interstadial targets all green (8/8 on 5 axes). Per-spring:

| Spring | Version | gS | Tier 4 | Open Gaps | LTEE | Next Target |
|--------|---------|:--:|:------:|-----------|------|-------------|
| wetSpring | V179 | **L5** | Done | 2 (PG-02,04 — deployment-only) | **Barrick SEALED** (7/7, USB to lithoSpore) | Tenaillon 2016, variant caller parity, L3 cross-tier |
| hotSpring | v0.6.32 | L6 | Done | Titan V FECS, K80 livepatch | **B2 DONE** | Node atomic (`s_node_atomic` added). strandGate AMD+NV, biomeGate sovereign dispatch |
| neuralSpring | V159 | L5 | Done | Squirrel provider reg (upstream) | **B1 DONE** | NestGate weight persistence WIRED, Squirrel inference pipeline COMPLETE |
| airSpring | v0.10.0 | L4 | Done | ~~AG-005~~ **RESOLVED**. NestGate/Squirrel IPC wired | E3 queued | LTEE E3, gS L5+ |
| ludoSpring | V71 | L4 | Done | coralReef IPC (GAP-01) | **Tower atomic LIVE** (6/6) | MDA framework + BM-004/005 implemented. Foundation T9+T10 seeded |
| groundSpring | V141 | L4 | Done | coralReef IPC, PRNG Phase 2b | **B1-B4 DONE** | lithoSpore integration (B3+B4 INGESTED) |
| healthSpring | V64m | L5 | Done | ionic bridge (upstream) | B5 COMPLETE, Nest atomic COMPLETE | NestComposition facade, cell.toml deployed, Foundation T10 gap |

### Layer 4: Sovereignty Composition — Interstadial (projectNUCLEUS, gardens)

**Owner**: Product teams + primalSpring (schema ownership)
**Scope**: Membrane composition, content-aware routing, sovereignty parity,
calibrate-shadow-cutover protocol, darkforest alignment
**Phase**: **Interstadial** — shadow runs ACTIVE, sovereignty calibration underway

**primalSpring sovereignty track** (3 new scenarios):
- `membrane-composition` (Rust): structural validation of `graphs/membrane/tower_membrane.toml`
- `sovereignty-parity` (Both): routing config schema + live membrane boundary health
- `content-sovereignty` (Live): content pipeline through sovereign routing + SkunkBat audit

**primalSpring owns**:
- Canonical routing config schema: `config/routing_config_reference.toml`
- Membrane deploy graph: `graphs/membrane/tower_membrane.toml`
- 4-layer sovereignty validation (Layers 1-2 existing, Layers 3-4 new sovereignty track)

**Current** (projectNUCLEUS):
- Horizon 1: **COMPLETE** — external security, darkforest v0.2.1
- Horizon 2: **80%** — 2a done, 2b ready, 3a cell membrane live, 3b/3c upstream shipped, **H2-12 BearDog TLS shadow LIVE** (10ms vs 120ms tunnel), **DoT 10/10 FIXED**, tunnel baseline clarified
- Horizon 3: **20%** — H3-07/H3-08 unblocked, rest future
- Absorption targets: `composition.deploy(graph)`, Tier 4 rewiring, skunkBat in smaller compositions
- Forgejo as primary git host, membrane telemetry pipeline (`membrane_telemetry.sh`, `membrane_7day.toml`)

**Current** (lithoSpore/CATHEDRAL):
- **6/7 modules Tier 2 PASS** (51/51 checks). VM-validated via benchScale + agentReagents
  on fresh libvirt VM — different `hostname_hash` confirms geo-delocalized validation.
- ecoBin compliant: BLAKE3 `features = ["pure", "std"]`, zero C deps. `litho-core` library extracted (discovery, harness, stats).
- **14+ debt items resolved** across two CATHEDRAL sessions. Pillar 4 interstadial gate **EXCEEDED**.
- benchScale CLI now supports `--backend libvirt` (was hardcoded Docker). `russh` 0.58→0.60.
- agentReagents: new `lithoSpore-validation.yaml` template (Ubuntu 24.04, 2GB, musl-static).
- CATHEDRAL owns benchScale + agentReagents going forward.

**Upstream blockers (CATHEDRAL-exposed)**:

| ID | Blocker | Owner | Impact |
|----|---------|-------|--------|
| ~~UB-1~~ | ~~Songbird TURN client library~~ | songBird team | **SHIPPED** — Wave 205: `songbird-turn-client` crate, RFC 5766 TURN allocation + channel-bind + refresh. `primal.announce` wired. |
| ~~UB-2~~ | ~~BearDog FIDO2/CTAP2 support~~ | bearDog team | **SHIPPED** — Wave 103: `fido2.rs` handler, `beardog.fido2.discover`/`register`/`authenticate`. 126 methods, ctap2 feature gate. primalSpring `s_beardog_fido2` scenario validates. |
| ~~UB-3~~ | ~~genomeBin Tier 3 packaging for USB~~ | plasmidBin / primalSpring | **SHIPPED** — `stage_usb.sh` Tier 3 USB packaging implemented. |
| ~~UB-4~~ | ~~sporePrint pipeline wiring~~ | primalSpring / sporePrint | **SHIPPED** — `auto-refresh.yml` extended for `liveSpore.json` pipeline. |
| CC-2 | LTEE Guidestone handoff archived | primalSpring | File archived to `handoffs/archive/` — cross-reference added |

**Composition gaps (foundation-exposed, primal ownership)**:

| Priority | Gap | Owner | Status |
|----------|-----|-------|--------|
| ~~1~~ | ~~Sandbox `working_dir` passthrough~~ | toadStool | **RESOLVED** — S263 workload spec |
| ~~2~~ | ~~Env var expansion in workload TOMLs~~ | toadStool | **RESOLVED** — S263 documented as pre-resolved |
| ~~3~~ | ~~GPU API alignment (`submit_and_map`)~~ | barraCuda / coralReef | **RESOLVED** — coralReef: `precision_advice`, `adapter`, `dispatch_hints` fields. barraCuda Sprint 71: `TENSOR_WIRE_CONTRACT.md` documenting 3-hop sovereign dispatch. |
| ~~4~~ | ~~Data dependency declaration in TOML~~ | toadStool / nestGate | **RESOLVED** — S263 `DataDependency` field |
| ~~6~~ | ~~Hex string acceptance (loamSpine/rhizoCrypt)~~ | loamSpine / rhizoCrypt | **RESOLVED** — rhizoCrypt S69 `parse_hash32`, loamSpine `serde_content_hash` |
| ~~7~~ | ~~sweetGrass TCP without BTSP~~ | sweetGrass | **RESOLVED** — v0.7.36 rejects raw JSON-RPC on TCP when `FAMILY_ID` set |
| ~~8~~ | ~~Cross-gate dispatch via songBird~~ | songBird / biomeOS | **RESOLVED** — songbird Wave 211: `capability.call` handler with local UDS + remote mesh TCP forwarding. routing="local" hop prevention. |

### Layer 5: projectFOUNDATION (sporeGarden/projectFOUNDATION)

**Owner**: projectFOUNDATION team + contributing springs
**Scope**: Public data anchoring, provenance validation, thread coverage
**Current**: 10 domain threads, 100+ data sources. CI thread-index validation functional.
Validation reality: Threads 2, 6, 7 fully validated; Thread 1 WCM (10/25 hashed, FN-1 partial).
FN-1 (BLAKE3 backfill) **IN PROGRESS** — 10/25 sources hashed (NCBI, UniProt, KEGG; 15 need manual fetch). FN-5 (CI validation) **RESOLVED** — CI extended to 13 steps with hash regression gate, typed IPC, thread_registry.sh. Springs seeding:
- airSpring: Thread 6 (ag) — 36/36 targets validated
- hotSpring: Thread 2 seeded
- neuralSpring: Threads 5+7 documented, ready for contribution
- groundSpring: Thread 7 (Anderson) index fixed
- ludoSpring: Threads 9+10 seeded with expressions + targets
- healthSpring: Thread 10 (provenance) gap documented

### Gap Flow — Sentinel Model

```
L1 (Primals — sentinels, stadial-first)
  │
  │ validated against ↓
  │
L2 (primalSpring — stadial gate)
  │ 456 registry, MethodGate, deploy graphs, guidestone cert
  │
  │ patterns flow downstream ↓
  │
L3 (Springs — interstadial, absorbing primal capabilities)
  │ domain science, IPC rewiring, foundation seeding
  │
  │ compositions flow downstream ↓
  │
L4 (Products — interstadial, pre-wiring sovereignty)
  │ shadow runs, deployment, external-facing artifacts
  │
  │ data anchoring ↓
  │
L5 (projectFOUNDATION — knowledge layer, thread coverage)
```

Gaps propagate **upward** (springs expose primal gaps → primalSpring gates them
→ primals resolve). Patterns propagate **downward** (primals ship capabilities
→ primalSpring validates → springs absorb → products deploy).

---

## Wave 6: Targeted GuideStone (LTEE) — May 11, 2026

The ecosystem's first **deployable subsystem**: a self-contained, USB-portable
artifact that leaves ecosystem possession. The LTEE guideStone reproduces
Barrick/Lenski LTEE papers and generates new predictions via the Anderson
disorder framework. This is a **projectNUCLEUS subsystem**.

Standard: `infra/wateringHole/TARGETED_GUIDESTONE_STANDARD.md`
Handoff: `infra/wateringHole/handoffs/LTEE_GUIDESTONE_SUBSYSTEM_HANDOFF_MAY11_2026.md`

### Wave 6 Ownership

| Layer | Responsibility | Status |
|-------|---------------|--------|
| L2 (primalSpring) | Targeted GuideStone standard, scope graph schema, validation harness pattern | **DONE** — standard defined |
| L3 (springs) | LTEE paper queue items (36 assignments across 6 springs), binary builds, scenario implementations | **SEEDED** — queues populated, reproduction work begins |
| L4 (projectNUCLEUS) | Integration as subsystem, workload TOMLs, deployment testing, USB packaging | **ARCHITECTURE** — handoff created, phases 2-5 pending |
| L5 (projectFOUNDATION) | Thread 04 (enviro genomics) + Thread 02 (plasma physics) data anchoring for LTEE datasets | **PENDING** — awaiting spring reproductions |

### Wave 6 Paper-Spring Assignments

| Spring | Papers | Count |
|--------|--------|------:|
| wetSpring | B1–B8, E1, E5 | 10 |
| neuralSpring | B1–B4, B6–B9, E2–E5 | 12 |
| groundSpring | B1–B4, B6–B9 | 8 |
| hotSpring | B2, B9 | 2 |
| healthSpring | B5, E2, E4 | 3 |
| airSpring | E3 | 1 |
| **Total** | | **36** |

### Wave 6 Milestones

- [x] Phase 1: Architecture + queue seeding (THIS UPDATE)
- [ ] Phase 2: Spring reproductions (L3) — **INTERSTADIAL**
- [ ] Phase 3: Binary bundle + data assembly (L2 + L4) — **INTERSTADIAL**
- [ ] Phase 4: Integration + deployment testing (L4) — **STADIAL**
- [ ] Phase 5: External deployment to Barrick Lab (L4) — **STADIAL**

---

## Wave 7: Contract Testing — Semantic Gate Evolution (May 11, 2026)

**Owner**: primalSpring team
**Priority**: HIGH — exposed by NestGate transport parity gap (now **RESOLVED**)
**Target**: Before stadial (prevents this class of gap from recurring)
**Status**: **7/7 items DONE** — all Wave 7 items closed (May 11)

The NestGate `content.put` transport parity gap reached projectNUCLEUS because
primalSpring's gate validates **structural** consistency (methods registered, health
alive, graphs coherent) but not **semantic** correctness (methods actually serve
correct responses across all transports). This wave evolves the gate from structural
to contract-level validation.

### Wave 7 Items

| ID | What | Owner | Status |
|----|------|-------|--------|
| W7-01 | Add `content` to `ALL_CAPS` in `composition/routing.rs` and wire `capability_to_primal("content") → "nestgate"` | primalSpring | **DONE** (May 11) |
| W7-02 | New scenario `s_nestgate_content_pipeline`: `content.put` → `content.get` round-trip (BLAKE3 hash match), `content.list`, `content.exists`, `content.resolve`. SKIP when NestGate unreachable, FAIL when methods error. | primalSpring | **DONE** (May 11) |
| W7-03 | Extend `server_ecosystem_compose.rs` Gate tests: `content.put` stores bytes returns hash, `content.get` retrieves by hash matches original, `content.list` includes stored hash (Content Gate 1-3) | primalSpring | **DONE** (May 11) |
| W7-04 | Deploy graph `content_pipeline_smoke.toml`: `content.put` + `content.get` + `content.list` round-trip via `by_capability = "content"` | primalSpring | **DONE** (May 11) |
| W7-05 | Validate `content.resolve` for petalTongue backend: ensure NestGate path resolution returns correct content + MIME type (petalTongue `backend=nestgate` depends on this) | primalSpring | **DONE** (May 11) — NestGate Session 60 shipped transport parity; gate scenario covers `content.resolve` |
| W7-06 | Inverse drift detection: `tools/check_method_coverage.sh` flags methods registered in 418-registry but **never referenced** in any scenario, test, or graph. Currently shows 125/418 uncovered. CI-gatable. | primalSpring | **DONE** (May 11) |
| W7-07 | NestGate transport parity: verify `content.*` methods are reachable on SemanticRouter, isomorphic IPC adapter, and HTTP API — not just primary unix_socket_server dispatch | primalSpring + NestGate | **DONE** (May 11) — NestGate Session 60 wired all 8 `content.*` methods on all 4 transport surfaces |

### Lesson: Structural vs Semantic Gates

The primalSpring gate currently validates:
- **Structural**: methods enumerated in registry, deploy graphs reference correct capabilities, health checks pass, `storage.*` round-trips work
- **NEW (Wave 7)**: `content.*` contract tests (scenario, gate tests, deploy graph), inverse drift detection (125/418 methods uncovered — CI-gatable tool shipped)
- **Wave 9** (domain contract sweep): `secrets.*`, `bonding.*`, `defense.*`, `discovery.*`, `provenance.*`, `spine.*`, `network.*` all exercised via `s_domain_contract_sweep` scenario + `domain_contract_sweep.toml` graph. Coverage 288/418 → 302/418 (72%). Remaining 116 are test fixtures, domain-specific (game/nautilus/ml), or require external infrastructure
- **Resolved**: W7-07 transport parity verification (NestGate Session 60 shipped all surfaces)

The sentinel-stadial model correctly surfaced this gap — downstream composition
pressure (projectNUCLEUS trying to publish content) exposed that the upstream
sentinel (NestGate) had implemented the capability on one transport path but not
wired it on all paths, and the gate (primalSpring) was not testing the capability
semantically. **This gap is now fully resolved** — NestGate Session 60 shipped
transport parity, and Wave 7 added the semantic contract tests.

**Wave 7 closes this gap class permanently.** After Wave 7, any method registered
in the 418-method registry that lacks a contract test or is unreachable on any
transport will be flagged by primalSpring's gate.

---

## Wave 8: Compute Trio Composition — Node Atomic Evolution (May 11, 2026)

**Owner**: primalSpring team + upstream compute trio teams
**Priority**: HIGH — extends Node atomic from structural to semantic validation
**Target**: Before stadial (enables sovereign compute E2E in compositions)

The compute trio (coralReef + toadStool + barraCuda) forms the Node atomic's
compute layer. hotSpring's sovereign compute breakthrough (3 GPUs, warm-catch
pipeline, pure Rust) and the wateringHole handoff define a clear domain split:
coralReef (HOW — compiler), toadStool (WHERE — hardware), barraCuda (WHAT — math).

Wave 8 sketches the architecture locally and hands upstream to primal teams.

### Wave 8 Items

| ID | What | Owner | Status |
|----|------|-------|--------|
| W8-01 | Architecture document `docs/COMPUTE_TRIO_EVOLUTION.md` — HOW/WHERE/WHAT domain split, IPC contracts (`shader.compile.wgsl`, `compute.dispatch.submit`), 6-phase ember/glowplug absorption path, degradation tiers, upstream handoff matrix | primalSpring | **DONE** (May 11) |
| W8-02 | Evolve `s_compute_triangle` scenario — 5-phase validation: discovery, coralReef capabilities, toadStool capabilities, barraCuda math round-trip, sovereign dispatch E2E contract (compile → dispatch response shapes) | primalSpring | **DONE** (May 11) |
| W8-03 | Inverse drift audit — compute/tensor/shader domains: 5 compute-related methods uncovered (aliases/admin), critical dispatch path exercised | primalSpring | **DONE** (May 11) |
| W8-04 | Compute trio gate tests in `server_ecosystem_compose.rs` — Gate 1: coralReef `shader.compile.capabilities`, Gate 2: toadStool `compute.capabilities`, Gate 3: barraCuda `stats.mean` round-trip, Gate 4: sovereign E2E compile+dispatch | primalSpring | **DONE** (May 11) |
| W8-05 | Deploy graph `compute_trio_smoke.toml` — 6-phase health + capabilities + math round-trip for all three primals | primalSpring | **DONE** (May 11) |
| W8-06 | gen4 sketch `SOVEREIGN_COMPUTE_TRIO_SKETCH.md` — HOW/WHERE/WHAT as gen4 composition pattern, warm-catch as sovereignty pattern, era-agnostic compute, budding/absorption model | primalSpring | **DONE** (May 11) |
| W8-07 | toadStool ember/glowplug absorption (Phases 1-6) — absorb coral-ember + coral-glowplug + coral-driver hardware | toadStool | **DONE** — Phase C **COMPLETE** (S245-S250, batches 1-7, 520 cylinder tests, 8,809 workspace). Phase D plumbing in (local dispatch path, factory abstraction). `toadstool.validate` **IMPLEMENTED** (S250). `toadstool.list_workloads` **WIRED** (S245+). E2E sovereign dispatch awaits factory hook-up (stadial work). |
| W8-08 | coralReef domain boundary cleanup — extract hardware code to toadStool, retain compiler domain only (`shader.compile.*`) | coralReef | **DONE** — coral-ember/glowplug soft-deprecated. RDNA2 atomics fix shipped. Phase C/D transition markers in place. Sprint 7: FECS **STABILITY PROOF SHIPPED** (`boot_gr_falcons_with_recovery`, 3× retry + PMC GR reset, `GrBootOutcome` enum, 4790 tests). All sentinel blockers resolved. |
| W8-09 | barraCuda sovereign dispatch E2E wiring — wire `SovereignDevice` through trio IPC (compile + dispatch) | barraCuda | **DONE** (v0.4.0) — 15-tier PrecisionTier, sovereign dispatch wire extracted, IPC coverage sweep (71/71 methods), bearDog crypto audit confirmed non-redundant. Stadial gate release. |

### Upstream Handoff

primalSpring provides: architecture doc, IPC contracts, gate tests, deploy graphs, gen4 sketch.
Upstream teams implement: absorption (toadStool), domain cleanup (coralReef), E2E wiring (barraCuda).

See `docs/COMPUTE_TRIO_EVOLUTION.md` for full architecture and handoff matrix.

---

## Wave 10: Phase 32 Atomic Model Evolution + Temporal Review (May 12, 2026)

**Owner**: primalSpring team
**Priority**: HIGH — structural model alignment before stadial
**Status**: **DONE** (May 12)

The temporal ecosystem review identified structural drift between the Rust
`AtomicType` model and the graph fragment definitions. Phase 32 resolves this:

| Change | Before | After |
|--------|--------|-------|
| **Tower** | bearDog + songbird (2) | bearDog + songbird + **skunkBat** (3) |
| **Tower capabilities** | security, discovery | security, discovery, **defense** |
| **Node** | Tower(2) + compute trio (5) | Tower(3) + compute trio (**6**) |
| **Nest primals** | bearDog, songbird, nestGate, **squirrel** (4) | bearDog, songbird, **skunkBat**, nestGate, **rhizoCrypt, loamSpine, sweetGrass** (7) |
| **Nest capabilities** | security, discovery, storage, **ai** | security, discovery, **defense**, storage, **dag, ledger, attribution** |
| **NUCLEUS core** | 9 primals | **10** primals |
| **Fragment versions** | 2.0.0 | **3.0.0** |

### Artifacts

- `docs/TEMPORAL_ECOSYSTEM_REVIEW_MAY12_2026.md` — full ecosystem temporal review
- `docs/LIVE_SCIENCE_API.md` — Tier 2 wire contract (toadstool.validate, list_workloads)
- Updated: `config/deployment_matrix.toml`, all `graphs/fragments/*.toml`
- Updated: `ecoPrimal/src/coordination/mod.rs` (AtomicType + 689+ tests pass)

---

## Wave 12: Deep Debt Sweep — Safety, Idiom, Discovery (May 14, 2026)

**Owner**: primalSpring team
**Priority**: HIGH — zero-panic production, modern idiomatic Rust, capability-first
**Status**: **DONE** (May 14)

Comprehensive audit found zero unsafe blocks, zero production mocks, zero `todo!()`/
`unimplemented!()`, zero `Box<dyn Error>`, all files under 800 lines, and pure Rust
dependencies (no C/FFI crates; BLAKE3 uses `pure` feature).

### Panic/Expect Elimination

| Site | Was | Now |
|------|-----|-----|
| `certification/entropy.rs:generate_machine_seed` | `panic!("OS entropy unavailable")` | `Option<String>`, caller handles gracefully |
| `bin/primalspring_guidestone/entropy.rs` | Same panic | Same fix + `.unwrap_or_default()` |
| `ipc/transport.rs:call_encrypted` | `.expect("Phase 3 keys required")` ×2 | `.ok_or(IpcError::ProtocolError { .. })?` |
| `harness/mod.rs:generate_harness_mito_seed` | `.expect("HKDF expand")` | Graceful fallback (empty vec) |
| `harness/mod.rs:generate_harness_nuclear` | `.expect("HKDF expand")` | Silent `let _ =` (zeroed OKM accepted) |

### Hardcoding → Discovery

| Site | Was | Now |
|------|-----|-----|
| `certification/entropy.rs` | Hardcoded `"/tmp/ecoprimals"` | `ipc::discover::resolve_socket_dir()` (env-first) |
| `bin/.../entropy.rs` | Same hardcoded path | Same fix |
| `certification/entropy.rs` | Hardcoded `"x86_64-unknown-linux-musl"` arch | `current_target_triple()` compile-time dispatch |
| `ipc/method_gate.rs:BearDogVerifier` | Direct socket path + literal `"beardog"` | `discover_by_capability("security")` fallback chain + `primal_names::BEARDOG` constant |

### Idiomatic Rust Modernization

| Pattern | Was | Now |
|---------|-----|-----|
| `Vec<&String>` | `certification/btsp.rs` cleartext caps | `Vec<&str>` with `.as_str()` |
| Manual `Display + Error` | `JsonRpcError` (protocol.rs) | `#[derive(thiserror::Error)]` |
| Manual `Display + Error` | `UnknownPrimal` (primal_names.rs) | `#[derive(thiserror::Error)]` |
| `DeployError::Parse(String)` | String-erased TOML errors | `Parse { context, source: toml::de::Error }` — preserves error chain |

### Deprecated Production Path Cleanup

`composition/btsp.rs:upgrade_btsp_clients` — replaced `#[expect(deprecated)]`
bridge to `family_seed_from_env()` with `mito_beacon_from_env().key_bytes()`,
the genetics-aware non-deprecated path.

### New Discovery Infrastructure

Added `ipc::discover::resolve_socket_dir()` — canonical function for resolving the
ecoPrimals runtime socket directory. Priority: `$ECOPRIMALS_SOCKET_DIR` →
`$XDG_RUNTIME_DIR/ecoprimals` → `<temp_dir>/ecoprimals`. Replaces all inline
hardcoded `/tmp/ecoprimals` fallback patterns.

---

## Wave 11: Local Debt Resolution + Compute Trio Depth (May 14, 2026)

**Owner**: primalSpring team
**Priority**: HIGH — interstadial exit gate items
**Status**: **DONE** (May 14)

### CompositionContext Migration

Migrated all active validation and RPC paths from deprecated `probe_primal` to
`CompositionContext`-based probing. The deprecated functions remain for backward
compatibility but are no longer called by any handler or live validation path.

| File | Change |
|------|--------|
| `deploy/validation.rs` | `probe_graph_node` → `probe_graph_node_with_context`, new `validate_live_with_context` |
| `coordination/mod.rs` | new `validate_composition_ctx` (capability-keyed, context-aware) |
| `bin/primalspring_primal/handlers.rs` | All 4 handlers migrated to context-aware paths |

### btsp.capabilities Method

Registry method 419: `btsp.capabilities` (owner: bearDog). `upgrade_btsp_clients`
now probes this method before attempting BTSP handshake, preventing connection
failures in mixed deployments where some primals lack BTSP server listeners.

### New Scenarios (24 → 27)

| Scenario | What | Methods Exercised |
|----------|------|-------------------|
| `s_tier2_science_api` | Tier 2 wire contract exemplar | `toadstool.validate`, `toadstool.list_workloads`, `barracuda.precision.route`, `biomeos.spring_status` |
| `s_barracuda_precision` | Deep precision routing + TensorSession | `barracuda.precision.route` (multi-op), `tensor.create`, `stats.variance`, `stats.std` |
| `s_coralreef_shader_targets` | Dual-vendor GPU compilation | `shader.compile.capabilities`, `shader.compile.wgsl`, `shader.compile.module` (naga) |

### Method Coverage

307/419 (73.3%), up from 302/418 (72.2%). 112 uncovered remain — mostly test
fixtures, domain-specific (`game.*`, `nautilus.*`, `ml.*`), and external infra.

### Foundation Thread 10 Seeded

Thread 10 (Provenance/Economics) elevated from EMPTY → SEEDED. Expression:
provenance trio pipeline (skunkBat → rhizoCrypt → sweetGrass) as the economic
substrate for NFT/attestation models. Sources: BLAKE3 CAS hashes, ionic bond
contracts, attribution braids. Targets: content-addressed artifact lifecycle,
cross-family attestation chain, provenance-anchored economic exchange.

---

## Interstadial Exit Criteria (May 12, 2026)

The interstadial ends when sovereignty capabilities are structurally wired and
shadow runs can begin. Five pillars define the exit gate. Full details:
`infra/wateringHole/INTERSTADIAL_EXIT_CRITERIA.md`

### Interstadial Targets by Layer

| Layer | Interstadial Target | Gate Condition |
|-------|-------------------|----------------|
| **L1 (Primals)** | MethodGate parity 13/13 | MethodGate shipped for all primals |
| **L2 (primalSpring)** | CompositionContext coordination pass, lithoSpore standard | 2+ lithoSpore modules PASS Tier 1 |
| **L3 (Springs)** | 4+ springs `optional=true`, gS convergence (air/neural → L4), LTEE reproductions begin | wetSpring < 5 PG gaps, 2+ foundation threads seeded |
| **L4 (Products)** | H2 shadow runs (TLS/NAT/NestGate/BTSP auth), ABG WCM compositions | H2-2b/3a/3b/3c in shadow-run state |
| **L5 (Foundation)** | Threads 3, 5, 8, 10 sources/targets, LTEE data anchoring | 7+/10 threads with sources |

### Stadial Targets by Layer

| Layer | Stadial Target | External Driver |
|-------|---------------|-----------------|
| **L1 (Primals)** | Upstream crate extraction (wgsl-precision, proc-sysinfo) | crates.io community |
| **L2 (primalSpring)** | Framework parity benchmarks | Kokkos, LAMMPS, SciPy |
| **L3 (Springs)** | lithoSpore Phases 4-5, all springs Tier 4 | Barrick Lab USB, peer validation |
| **L4 (Products)** | H2 cutover (Cloudflare → sovereign), H3 begin | Cloudflare baselines, GitHub → Forgejo |
| **L5 (Foundation)** | All threads with validated targets, ABG in production | ABG users, faculty network |
