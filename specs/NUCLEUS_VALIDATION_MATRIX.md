# NUCLEUS Validation Matrix

**Date**: April 8, 2026  
**Phase**: 27 (All 10 primals BTSP Phase 1 + Wire Standard L2 ecosystem — Run 6 ready)  
**Purpose**: Define the validation matrix for NUCLEUS composition patterns across downstream springs and sporeGarden products, based on gen4 architecture (`infra/whitePaper/gen4/architecture/COMPOSITION_PATTERNS.md`) and primalSpring's Phase 25-26 modernization results.

---

## Context

primalSpring validates the coordination layer. Phase 25 cleaned all legacy patterns and confirmed Tower Atomic HTTPS works end-to-end. The next evolution step is validating that these patterns compose correctly in downstream springs and gen4 products (sporeGarden).

This matrix defines what each spring and product must demonstrate to confirm NUCLEUS readiness.

---

## Meta-Patterns to Nucleate

These are the composition patterns proven in primalSpring that downstream systems must absorb:

| Pattern | primalSpring Reference | What It Proves |
|---------|----------------------|----------------|
| **Tower Atomic** | `nest-deploy.toml` Phase 1-2 | BearDog + Songbird compose; TLS 1.3 works |
| **Nest Atomic** | `nest-deploy.toml` Phase 1-4 | Tower + NestGate + Squirrel; storage + AI |
| **Node Atomic** | `node_atomic_compute.toml` | Tower + ToadStool; GPU compute |
| **Full NUCLEUS** | `nucleus_complete.toml` | Tower + Nest + Node; all capability domains |
| **Graph-Deployed Composition** | `[[graph.nodes]]` format | biomeOS deploys primals via TOML graphs |
| **Capability Routing** | `capability.call` via Neural API | biomeOS routes method calls to correct primal |
| **HTTPS Through Tower** | `validate_https` node | End-to-end TLS via BearDog→Songbird, no external TLS |
| **Covalent Bonding** | `basement_hpc_covalent.toml` | Multi-node with shared `FAMILY_ID`, mesh discovery |
| **Graceful Degradation** | gen4 COMPOSITION_PATTERNS §III | Product runs even if primals absent |
| **health.liveness** | All primals | Universal JSON-RPC health check (no HTTP) |

---

## Validation Matrix

### Columns

| Column | What to Validate | Method |
|--------|-----------------|--------|
| **A: Graph Format** | Uses `[[graph.nodes]]` with `id` field | Structural parse |
| **B: Capability Names** | All methods use canonical dotted names | Registry cross-check |
| **C: health.liveness** | All primals respond to `health.liveness` | JSON-RPC probe |
| **D: HTTPS Validation** | HTTPS through Tower Atomic works | `http.get` via Neural API |
| **E: Nest Atomic** | NestGate storage round-trip | `storage.store` + `storage.retrieve` |
| **F: Node Atomic** | ToadStool compute available | `compute.submit` |
| **G: AI Routing** | Squirrel `ai.query` via Neural API | `capability.call` |
| **H: Covalent Ready** | Multi-node graph, `FAMILY_ID`, mesh | Graph structure + exp073 pattern |
| **I: Graceful Degradation** | Product runs standalone (no primals) | Launch without stack |
| **J: sporeGarden Deploy** | BYOB deploy graph, plasmidBin binaries | `prepare_spore_payload.sh` |

### Rows (Springs)

| Spring | Domain | A | B | C | D | E | F | G | H | I | J |
|--------|--------|---|---|---|---|---|---|---|---|---|---|
| **primalSpring** | Coordination | **PASS** | **PASS** | **LIVE** (Tower) | **LIVE** (ifconfig.me 200) | structural | structural | structural | structural | n/a | structural | **52 caps, 7/9 call PASS** |
| **wetSpring** | Biology | pending | pending | pending | pending | pending | pending | pending | pending | n/a | pending |
| **hotSpring** | Physics | pending | pending | pending | pending | pending | **likely** | pending | pending | n/a | pending |
| **airSpring** | Agriculture | pending | pending | pending | pending | pending | pending | pending | pending | n/a | pending |
| **groundSpring** | Uncertainty | pending | pending | pending | pending | pending | pending | pending | pending | n/a | pending |
| **neuralSpring** | ML/Neural | pending | pending | pending | pending | pending | pending | pending | pending | n/a | pending |
| **healthSpring** | Health | pending | pending | pending | pending | pending | pending | pending | pending | n/a | pending |
| **ludoSpring** | Game Science | pending | pending | pending | pending | pending | pending | pending | pending | n/a | pending |

### Rows (sporeGarden Products)

| Product | Domain | A | B | C | D | E | F | G | H | I | J |
|---------|--------|---|---|---|---|---|---|---|---|---|---|
| **esotericWebb** | CRPG Engine | pending | pending | pending | pending | pending | pending | pending | pending | **required** | pending |
| **helixVision** | Genomics | planned | planned | planned | planned | planned | planned | planned | planned | **required** | planned |

### Extended Columns (Particle Model & Mixed Composition)

| Column | What to Validate | Method |
|--------|-----------------|--------|
| **K: Particle Profile** | Which particle the spring primarily exercises (proton-heavy, neutron-heavy, balanced) | Architectural analysis per `MIXED_COMPOSITION_PATTERNS.md` §5 |
| **L: Mixed Atomic** | Can the spring deploy L2 patterns (dual tower, dedicated tower, enclave) | Graph sketch structural validation |
| **M: Bonding Patterns** | Which L3 bonding patterns the spring requires | Domain analysis |
| **N: Sharding Ready** | Covalent mesh backup applicable (L3 `covalent_mesh_backup.toml`) | Structural + BondingPolicy validation |
| **O: Enclave Ready** | BondingPolicy data egress fence applicable (L2 `nest_enclave.toml`) | BondingPolicy structural validation |
| **P: Wire Standard** | Primal's `capabilities.list` follows Capability Wire Standard v1.0 | Level 1/2/3 audit per `infra/wateringHole/CAPABILITY_WIRE_STANDARD.md` |

### Extended Rows (Springs)

| Spring | Domain | K: Particle | L: Mixed Atomic | M: Bonding | N: Sharding | O: Enclave | P: Wire Std |
|--------|--------|-------------|-----------------|------------|-------------|------------|-------------|
| **primalSpring** | Coordination | balanced | structural | all (test arena) | structural | structural | L2+ (ref) — BD L2, SB L2, NG L3, TS L3, BC L2, SQ L2. **All 10 BTSP Phase 1.** |
| **wetSpring** | Biology | balanced | nest enclave | covalent mesh | planned | planned | pending |
| **hotSpring** | Physics | proton-heavy | node+dedicated tower | metallic, ionic lease | n/a | n/a | pending |
| **airSpring** | Agriculture | balanced | nest enclave | covalent mesh | planned | planned | pending |
| **groundSpring** | Uncertainty | proton-heavy | node+dedicated tower | ionic lease | n/a | n/a | pending |
| **neuralSpring** | ML/Neural | balanced | nest enclave | ionic lease | n/a | **required** | pending |
| **healthSpring** | Health | neutron-heavy | dual tower + enclave | covalent mesh | **required** | **required** | pending |
| **ludoSpring** | Game Science | proton-heavy | node+dedicated tower | organo-metal-salt | planned | n/a | pending |

### Extended Rows (sporeGarden Products)

| Product | Domain | K: Particle | L: Mixed Atomic | M: Bonding | N: Sharding | O: Enclave | P: Wire Std |
|---------|--------|-------------|-----------------|------------|-------------|------------|-------------|
| **esotericWebb** | CRPG Engine | proton-heavy | node+dedicated tower | covalent, ionic | planned | n/a | pending |
| **helixVision** | Genomics | neutron-heavy | dual tower + enclave | covalent mesh | **required** | **required** | pending |

---

## Layered Validation Model

The matrix columns map to a four-layer validation model defined in `specs/MIXED_COMPOSITION_PATTERNS.md`:

| Layer | Scope | Matrix Columns | Key Graph |
|-------|-------|---------------|-----------|
| **L0** | biomeOS + any primal | A, B, C | `graphs/sketches/validation/primal_routing_matrix.toml` |
| **L1** | Each atomic (Tower/Node/Nest) | C, D, E, F, G | Existing: `nest-deploy.toml`, `node_atomic_compute.toml` |
| **L2** | Mixed atomics | L, O | `graphs/sketches/mixed_atomics/*.toml` |
| **L3** | Bonding patterns | H, M, N | `graphs/sketches/bonding_patterns/*.toml` |

### Particle Model Reference

The atomic-to-particle mapping (Paper 23, `gen3/baseCamp/23_mass_energy_information_equivalence.md`):

| Atomic | Particle | Property | Fungibility |
|--------|----------|----------|-------------|
| **Tower** | Electron | Trust boundary, mediates all bonding | n/a (medium) |
| **Node** | Proton | Compute = energy | Fungible |
| **Nest** | Neutron | Data = energy at rest | Non-fungible |
| **NUCLEUS** | Atom | Complete composition | — |

Column K (Particle Profile) characterizes each spring's emphasis:
- **Proton-heavy**: compute-dominated (hotSpring, groundSpring, ludoSpring)
- **Neutron-heavy**: data-dominated (healthSpring, helixVision)
- **Balanced**: equal compute + data emphasis (wetSpring, airSpring, neuralSpring, primalSpring)

---

## Validation Approach per Spring

Each spring has a `validate_nucleus_*` binary or equivalent. The matrix cells are validated by:

1. **Structural**: Spring has a biomeOS deploy graph in `graphs/spring_deploy/` (primalSpring has these for all 7). Validate it parses with `[[graph.nodes]]` format.
2. **Live**: Spring's deploy graph is executed on Eastgate with live primals. The spring primal starts, discovers NUCLEUS primals, and performs its domain validation.
3. **Product**: For sporeGarden products, the full composition pipeline runs — PrimalBridge connects to all required primals, graceful degradation works, standalone mode functional.

---

## Priority Order

### Phase A: Graph Format Compliance (columns A + B)

All springs have proto-nucleate graphs (pure primal NUCLEUS compositions) in `primalSpring/graphs/downstream/`. Springs validate *against* these compositions externally — they do not appear as graph nodes. Spring deploy graphs in `spring_deploy/` are for Rust-proof integration testing only.

Action: Each spring team should verify their local graph files (if any) match `[[graph.nodes]]` format.

### Phase B: Health + HTTPS Validation (columns C + D)

Deploy each spring's NUCLEUS graph and validate:
- All primals respond to `health.liveness`
- HTTPS through Tower Atomic returns a valid response

This requires live primals. primalSpring's `AtomicHarness` can drive this.

### Phase C: Storage + Compute + AI (columns E + F + G)

Validate domain-specific primal interactions:
- NestGate `storage.store`/`storage.retrieve` for experiment data
- ToadStool `compute.submit` for GPU workloads (hotSpring, airSpring, groundSpring)
- Squirrel `ai.query` for AI-assisted analysis

### Phase D: Multi-Node + Covalent (column H)

Validate covalent bonding readiness:
- Each spring's graph can extend to multi-node deployment
- `FAMILY_ID` propagation works across gates
- BirdSong mesh discovery finds peer spring instances

### Phase E: sporeGarden Deployment (columns I + J)

For products:
- Graceful degradation validated (standalone mode works)
- BYOB deploy graph defines full primal topology
- `prepare_spore_payload.sh` produces deployable payload

---

## Integration with Existing Infrastructure

| Component | Role in Matrix |
|-----------|---------------|
| `primalSpring/graphs/spring_deploy/*.toml` | Nucleated deploy graphs for all 7 springs |
| `primalSpring/config/deployment_matrix.toml` | 43-cell deployment matrix (arch × topology × preset × transport) |
| `primalSpring/scripts/validate_deployment_matrix.sh` | Matrix runner |
| `primalSpring/scripts/validate_remote_gate.sh` | Remote gate NUCLEUS health probe |
| `primalSpring/scripts/prepare_spore_payload.sh` | USB spore payload assembly |
| `primalSpring/ecoPrimal/src/harness/` | `AtomicHarness` for live composition |
| `primalSpring/ecoPrimal/src/bonding/` | `BondType`, `BondingPolicy`, `BondingConstraint` |
| `primalSpring/experiments/exp090_tower_atomic_lan_probe/` | LAN discovery validation |
| `primalSpring/experiments/exp091_primal_routing_matrix/` | L0 routing matrix validation |
| `primalSpring/experiments/exp092_dual_tower_ionic/` | L2 dual tower + ionic validation |
| `primalSpring/experiments/exp093_covalent_mesh_backup/` | L3 covalent mesh backup validation |
| `primalSpring/specs/MIXED_COMPOSITION_PATTERNS.md` | Particle model, layered validation, gap inventory |
| `primalSpring/graphs/sketches/mixed_atomics/` | L2 mixed atomic graph sketches |
| `primalSpring/graphs/sketches/bonding_patterns/` | L3 bonding pattern graph sketches |
| `infra/whitePaper/gen4/architecture/COMPOSITION_PATTERNS.md` | gen4 patterns (PrimalBridge, graceful degradation, deploy graphs) |
| `infra/whitePaper/gen3/baseCamp/23_mass_energy_information_equivalence.md` | Paper 23: particle model theoretical foundation |

---

## Relationship to gen4

The gen4 vision (`COMPOSITION_PATTERNS.md`) introduces two patterns that extend the NUCLEUS matrix:

1. **Dual Surface** (Creator + Developer): The Creator surface (YAML/CLI) requires graceful degradation (column I). The Developer surface (Rust/PrimalBridge) requires all columns A-H.

2. **PrimalBridge**: Each gen4 product has a bridge that connects to 8+ primal domains. The matrix validates that these domains are reachable through NUCLEUS composition.

The NUCLEUS validation matrix is the gen3→gen4 bridge checkpoint: when all springs pass columns A-H, products can trust the composition layer.

---

## Live Validation Results (April 7, 2026)

### Run 1: biomeOS v2.81 (pre-fix)

Tower Atomic (BearDog + Songbird) validated live on Eastgate. Neural API running but capability registration gap identified — 0 capabilities discovered from running primals.

### Run 2: biomeOS v2.92 (post-fix — probe_endpoint + prefix matching)

biomeOS v2.92 includes real JSON-RPC probing (`identity.get` + `capabilities.list`), 4-format capability parser alignment, and domain prefix matching (GAP-019). Songbird (14 caps) registered. BearDog (Format E) still unrecognized. Proxy forwarding broken for all methods.

### Run 3: biomeOS v2.93 — BearDog + Songbird (2 primals)

biomeOS v2.93 resolves GAP-MATRIX-07, 01b, 02. 52 capabilities from 2 primals. 7/9 BearDog capability.call methods forwarded end-to-end.

### Run 4: biomeOS v2.93 — Full Tower + Provenance Trio (6 primals)

Trio pushed GAP-MATRIX-05 resolution commits: `identity.get` + biomeOS-parseable `capabilities.list` (rhizoCrypt Format E, loamSpine Format A, sweetGrass Format B). Rebuilt from source and validated with all 6 primals running:

| Probe | Result | Notes |
|-------|--------|-------|
| **Auto-discovery** | **LIVE PASS** | **162 capabilities from 6 primals** |
| rhizoCrypt discovered | **LIVE PASS** | 33 capabilities (Format E, 5 groups) |
| loamSpine discovered | **LIVE PASS** | 21 capabilities (Format A) |
| sweetGrass discovered | **LIVE PASS** | 28 capabilities (Format B) |
| provenance (symlink) discovered | **LIVE PASS** | 28 capabilities via provenance.sock → sweetgrass.sock |
| Songbird discovered | **LIVE PASS** | 14 capabilities (Format A) |
| BearDog discovered | **LIVE PASS** | 38 capabilities (Format E) |
| capability.discover dag | **LIVE PASS** | → rhizocrypt, correct endpoint |
| capability.discover permanence | **LIVE PASS** | → loamspine, correct endpoint |
| capability.discover braid | **LIVE PASS** | → provenance (sweetgrass), correct endpoint |
| capability.discover provenance | **LIVE PASS** | → provenance (sweetgrass), correct endpoint |
| capability.discover crypto | **LIVE PASS** | → beardog, correct endpoint |
| capability.discover network | **LIVE PASS** | → songbird, correct endpoint |
| **capability.call dag.session.create** | **LIVE PASS** | Neural API → rhizoCrypt → UUID `019d6a61-...` |
| **capability.call dag.session.list** | **LIVE PASS** | Neural API → rhizoCrypt → 2 active sessions |
| **capability.call health.liveness** | **LIVE PASS** | Neural API → rhizoCrypt → alive |
| **capability.call spine.create** | **LIVE PASS** | Neural API → loamSpine → spine_id + genesis_hash |
| **capability.call health.check** | **LIVE PASS** | Neural API → loamSpine → healthy, running |
| **capability.call braid.query** | **LIVE PASS** | Neural API → sweetGrass → empty result set |
| **capability.call crypto.blake3_hash** | **LIVE PASS** | Neural API → BearDog → BLAKE3 hash |
| **capability.call crypto.sign_ed25519** | **LIVE PASS** | Neural API → BearDog → Ed25519 signature |
| **capability.call crypto.hmac_sha256** | **LIVE PASS** | Neural API → BearDog → HMAC result |
| **capability.call security.evaluate** | **LIVE PASS** | Neural API → BearDog → trust evaluation |
| **capability.call trust.evaluate** | **LIVE PASS** | Neural API → BearDog → trust evaluation |
| **capability.call tls.derive_secrets** | **LIVE PASS** | Neural API → BearDog → TLS key derivation |
| capability.call braid.create | **FAIL** | biomeOS translates to `provenance.create_braid` (wrong method name) |
| capability.call with wrong params | **FAIL** | GAP-MATRIX-07b: primal -32602 errors swallowed as "Failed to forward" |
| NestGate | NOT STARTED | GAP-MATRIX-04: HTTP REST, not JSON-RPC/UDS |
| ToadStool, Squirrel | NOT STARTED | GAP-MATRIX-05 partial: need daemon mode |

### biomeOS v2.93 Validated (Runs 3+4)

- **GAP-MATRIX-07 (Critical) → RESOLVED**: `unix://` URI scheme parsed. 12/14 capability.call tests pass across 4 primals.
- **GAP-MATRIX-01 + 01b → RESOLVED**: 5-format parser (A-E). 162 capabilities from 6 primals auto-discovered.
- **GAP-MATRIX-05 → PARTIALLY RESOLVED**: Provenance trio live-validated through Neural API. rhizoCrypt (dag), loamSpine (permanence), sweetGrass (braid/provenance) all routing correctly. Squirrel + ToadStool remain untested.

### Resolved in biomeOS v2.94

- **GAP-MATRIX-07b → RESOLVED (v2.94)**: `forward_request()` preserves primal JSON-RPC error codes via `try_call()` + downcast in `dispatch()`. No more swallowed `-32601`/`-32602`.
- **GAP-MATRIX-08 → RESOLVED (v2.94)**: `NeuralRouter.self_socket_path` excludes own socket from `lazy_rescan_sockets()`, eliminating self-registration pollution.
- **GAP-MATRIX-02b → RESOLVED (v2.94)**: `graph.list` falls back to `biomeos_graph::GraphLoader` when neural parser fails, so `DeploymentGraph`-format TOMLs appear in listings.

### Resolved in Songbird Wave 123

- **GAP-MATRIX-03 → RESOLVED (Wave 123)**: TLS 1.3 ClientHello now includes 32-byte legacy session ID for middlebox compatibility (RFC 8446 Appendix D.4) and expanded RSA-PSS signature algorithms (`rsa_pss_rsae_sha384`, `rsa_pss_rsae_sha512`, `rsa_pss_pss_*`). Fixes handshake failures against CDN-fronted hosts (httpbin.org via Cloudflare).
- **Songbird method gap → RESOLVED (Wave 123)**: New `capabilities.methods` endpoint returns token→method mapping (`CAPABILITY_METHOD_MAP`). Capability tokens like `network.discovery` now normalize to callable methods (`discovery.peers`, `discovery.announce`, `discovery.list_peers`) via `normalize_json_rpc_method_name()`.

### Resolved in Primal Catch-Up Sprint (April 8, 2026)

- **GAP-MATRIX-10 → LARGELY RESOLVED**: Wire Standard adoption sprint — all atomic primals shipped compliance:
  - **BearDog (Wave 30)**: L2 complete — `{primal, version, methods, provided_capabilities}` + `identity.get`. Deep debt sweep: -654 lines dead code.
  - **Songbird (Wave 125)**: L2 complete — `{primal: "songbird", version, methods}` with 73 callable methods via `CALLABLE_METHODS` const + `identity.get`.
  - **NestGate (S36)**: **L3 complete** — full envelope with 9 capability groups, 46+ methods, `consumed_capabilities`, `protocol`, `transport`. Plus `identity.get`.
  - **ToadStool (S190-191)**: **L3 complete** — full envelope with `cost_estimates`, `operation_dependencies`, `consumed_capabilities`. `health.liveness` adds `"status": "alive"`.
  - **rhizoCrypt, loamSpine, sweetGrass**: previously validated (Run 4). sweetGrass near-L2, rhizoCrypt/loamSpine partial-L2.
  - Remaining: rhizoCrypt needs flat `methods`; loamSpine needs top-level `methods` + `identity.get`; sweetGrass needs `provided_capabilities` grouping for L3.
- **GAP-MATRIX-04 → RESOLVED (NestGate S36)**: NestGate now implements JSON-RPC 2.0 over Unix domain sockets alongside HTTP REST. `UnixListener::bind` + full method dispatch. Dual transport: `["uds", "http"]`.
- **GAP-MATRIX-05 → PARTIALLY RESOLVED (ToadStool S189)**: Daemon mode documented, `SERVER_METHODS.md` rewritten (67 methods, 11 namespaces), `DAEMON_MODE_USER_GUIDE.md` updated. Socket activation works via normal `UnixListener::bind` (not systemd `LISTEN_FDS`). Squirrel still untested.
- **NestGate → Tower Atomic composition**: `storage.fetch_external` now delegates HTTPS to Songbird via biomeOS `capability.call` (`http.request`). NestGate does NOT terminate TLS — BearDog remains the single auditable crypto boundary. This is the **Nest Atomic composition pattern** working end-to-end.

### Live Validation Run 5: biomeOS v2.95 — 9 primals, 607 capabilities (April 8, 2026)

All 9 primals started (fresh ecoBin builds). biomeOS auto-discovered **607 capabilities from 7 sockets** (beardog, crypto, songbird, nestgate, storage, sweetgrass, provenance, toadstool.jsonrpc). loamSpine TCP-only (no UDS).

| Probe | Result | Detail |
|-------|--------|--------|
| BearDog `crypto.blake3_hash` via Neural API | **PASS** | `beardog` domain → BLAKE3 hash returned |
| BearDog `crypto.sign_ed25519` via Neural API | **PASS** | Ed25519 signature returned |
| BearDog `crypto.hmac_sha256` via Neural API | **PASS** | HMAC-SHA256 returned |
| BearDog `identity.get` (direct) | **PASS** | `{primal: "beardog-tunnel", version: "0.9.0", domain: "crypto", license}` |
| BearDog `health.liveness` (direct) | **PASS** | `{status: "alive"}` |
| NestGate `identity.get` (direct) | **PASS (partial)** | `{primal: "nestgate", version: "0.1.0", family_id}` — missing `domain`, `license` |
| NestGate `health.liveness` (direct) | **PASS** | `{status: "alive"}` |
| NestGate `capabilities.list` (direct) | **PASS** | L3 envelope: methods array, provided_capabilities |
| NestGate `storage.list` (direct) | **PASS** | Returns stored keys |
| ToadStool `identity.get` (direct) | **PASS** | Full L3: domain, license, methods, capabilities |
| ToadStool `health.liveness` (direct) | **PASS** | `{status: "alive"}`, uptime, workload counts |
| ToadStool `capabilities.list` (direct) | **PASS** | L3: cost_estimates, consumed_capabilities |
| rhizoCrypt `dag.session.create` (direct) | **PASS** | Session UUID returned |
| sweetGrass `braid.query` (direct) | **PASS** | Empty result set (correct) |
| sweetGrass `capabilities.list` (direct) | **PASS** | Format B: methods + capabilities array |
| Songbird `capabilities.list` (direct) | **FAIL** | `Unknown method: capabilities.list` on orchestrator socket |
| Storage/Compute/DAG via Neural API | **FAIL** | GAP-MATRIX-11: graph-boot socket paths don't match actual sockets |
| NestGate `fetch_external` → Tower | **FAIL** | Looks for `neural-api-<family_id>.sock` which doesn't exist |

### New Gap: GAP-MATRIX-11 (Medium) — Graph-Boot Socket Path Mismatch

biomeOS graph-based bootstrap registers capabilities at FAMILY_ID-suffixed socket paths (e.g., `nestgate-8ff3b864a4bc589a.sock`, `toadstool-8ff3b864a4bc589a.sock`) which don't exist. Primals create sockets at simple names (`nestgate.sock`, `toadstool.jsonrpc.sock`). Graph paths take routing precedence over auto-discovered paths for taxonomy domains (`storage`, `compute`, `dag`, `braid`). The `beardog` domain works because it was registered from auto-discovery, not graph. `security.evaluate` worked in Run 4 for the same reason.

Impact: Neural API `capability.call` fails for any domain whose graph-boot registration overrides auto-discovery. Direct primal probes all work.

Recommended fix: biomeOS should either (a) auto-discovery results override graph-boot when socket exists, or (b) graph-boot should probe for actual socket existence before registering.

### Songbird Socket Gap (Medium)

Songbird's Wire Standard L2 methods (`capabilities.list`, `identity.get`, `health.liveness`) are implemented in the universal-ipc dispatch handler, but the `songbird.sock` socket is served by the orchestrator handler which doesn't expose these methods. biomeOS probes `songbird.sock` and gets "Unknown method" errors.

Impact: Songbird's L2 compliance exists in code but is unreachable on the socket biomeOS discovers.

Recommended fix: Songbird should wire the universal-ipc dispatch (Wire Standard methods) through the orchestrator's socket handler, or expose a second socket for universal-ipc.

### Resolved in biomeOS v2.96–v2.97 (April 8, 2026)

- **GAP-MATRIX-02 → RESOLVED (v2.96)**: `biomeos deploy` now accepts both `DeploymentGraph` (`[[graph.nodes]]`) and `neural_graph` (`[[nodes]]`) formats via dual-parser fallback. `tower_atomic_bootstrap.toml` loadable via `biomeos deploy --validate`.
- **GAP-MATRIX-09 → RESOLVED (v2.96)**: Attribution domain translations corrected — `provenance.create_braid` → `braid.create`, `provenance.get_braid` → `braid.get` to match sweetGrass v0.7.5 wire methods. Aliases now emit correct targets.
- **biomeOS v2.97 safety hardening**: `#![forbid(unsafe_code)]` on all 20+ binary entry points. Smart refactors: discovery.rs 843→94 LOC (split into registry/primal/composite/tests), orchestrator.rs 836→36 LOC, genome-deploy/lib.rs 860→35 LOC. 8 niche template IDs → `primal_names::` constants.

### Resolved in BarraCuda Sprint 33 + Squirrel alpha.43 (April 8, 2026)

- **GAP-MATRIX-10 → FURTHER RESOLVED**: Wire Standard adoption expands:
  - **BarraCuda (Sprint 33→42)**: **L2 complete** — `{primal, version, methods, provided_capabilities}` + `identity.get`. JSON-RPC dispatch over UDS (primary) + tarpc (secondary). 32 methods (tensor.matmul, tensor.create, stats.mean, compute.dispatch, noise.perlin2d, fhe.ntt, etc.), 4,187+ tests. `provided_capability_groups()` derives structured groups from dispatch table — zero hardcoded domain catalog. Full ecobin primal ready for spring IPC consumption.
  - **Squirrel (alpha.43)**: **L2 complete** — `capabilities.list`/`identity.get`/`health.liveness` aligned to Wire Standard. `reqwest` banned in `deny.toml` (Tower Atomic pattern). Production mock elimination: 791 lines dead orchestration code removed, SDK MCP `OperationHandler` returns honest errors.

### Resolved in Latest Sprint (April 8, 2026 — post-Run 5)

- **Songbird socket gap → RESOLVED (Wave 128)**: Wire Standard L2 methods (`capabilities.list`, `identity.get`, `health.liveness`) now wired through orchestrator socket handler — the same `songbird.sock` that biomeOS discovers.
- **GAP-MATRIX-12 → LARGELY RESOLVED**: BTSP Phase 1 adoption sprint — all primals now have INSECURE guard:
  - **BearDog (Wave 31-32)**: Full BTSP handshake enforcement. 4-step X25519+HMAC-SHA256 handshake, encrypted framing (ChaCha20-Poly1305/HMAC-Plain/Null), socket listener enforcement when FAMILY_ID set. **First primal live-encrypted.** 96 crypto methods, 14,366+ tests.
  - **ToadStool (S192)**: `validate_insecure_guard()` + `is_btsp_required()` + startup logging. Hooks into both unibin server and CLI daemon. 11 new tests, 21,526+ total.
  - **BarraCuda (Sprint 34-35)**: FAMILY_ID socket scoping (`BARRACUDA_FAMILY_ID` → `FAMILY_ID`), BIOMEOS_SOCKET_DIR support, INSECURE guard with typed `BarracudaCoreError`. 20 BTSP compliance tests, 4,207+ total.
  - **rhizoCrypt (S30)**: `family_scoped_socket_path()` (`rhizocrypt-{fid}.sock`), `btsp_env_guard()`, typed `BtspConfigError`. 16 new tests, 1,441 total.
  - **sweetGrass (Phase 11)**: INSECURE guard with typed `BtspGuardViolation`, FAMILY_ID chain (`SWEETGRASS_FAMILY_ID` → `BIOMEOS_FAMILY_ID` → `FAMILY_ID`). 5 DI-based tests, 1,218 total.
  - **loamSpine (S38)**: Domain-based socket (`loamspine.sock` → `permanence.sock`), family-scoped naming, INSECURE guard, legacy symlink with shutdown cleanup. **Now has UDS** (was TCP-only). 1,316 total.
  - **biomeOS (v2.98)**: `btsp_client` module with `validate_insecure_guard()` + `log_security_posture()` wired into all 4 startup paths.
  - **Squirrel (alpha.46)**: INSECURE guard + family-scoped socket naming (`squirrel-{fid}.sock`) + env resolution chain (`SQUIRREL_FAMILY_ID` → `BIOMEOS_FAMILY_ID` → `FAMILY_ID`). 6,875+ tests. Fresh ecoBin harvested.
- **GAP-MATRIX-06 → RESOLVED**: All ecoBins fresh in plasmidBin (April 8). No stale binaries remain.
- **biomeOS v2.99**: Zero-debt audit — 0 unsafe, 0 mocks, 0 TODO, 0 hardcoded names, 0 C deps. 7,695 tests.
- **ToadStool S193-194**: Headless GPU architecture (`TOADSTOOL_HEADLESS=1`), capability-based naming evolution across production code.

### What Remains

**Medium:**
- **GAP-MATRIX-11 (Medium)**: Socket naming alignment implemented in all primals, but NOT YET LIVE VALIDATED. Run 6 needed with all fresh ecoBins + biomeOS v2.99 to confirm family-scoped sockets resolve graph-boot mismatch end-to-end.
- **BearDog socket_config gap (Medium)**: BearDog `socket_config.rs` Tiers 2-4 still produce `beardog.sock` (not family-scoped `beardog-{fid}.sock`). Handshake enforcement works regardless via `BtspSecurityMode`, and auto-discovery uses `crypto.sock` symlink. But graph-boot path mismatch for `beardog` domain persists.

**Low / Residual:**
- **GAP-MATRIX-10 (Low, RESIDUAL)**: Wire Standard L2/L3 convergence. sweetGrass needs `provided_capabilities` grouping + `identity.get` for L3. rhizoCrypt needs flat `methods` array for full L2. loamSpine needs `identity.get` for full L2.
- **GAP-MATRIX-05 (Low, RESIDUAL)**: Squirrel + ToadStool not yet routed through Neural API (blocked by GAP-11 until Run 6).

**Ecosystem-wide Status:**
- **All 10 primals** have BTSP INSECURE guard + family-scoped socket naming. Zero BTSP holdouts.
- BearDog is the first primal with live BTSP encrypted socket enforcement.
- **All ecoBins fresh** (April 8). Zero stale binaries.
- Direct primal probes pass across all 9 tested primals (8 on UDS, Squirrel on abstract socket).
- Neural API routing confirmed for `beardog` domain. Other domains need Run 6 live validation.
- Wire Standard L2+ achieved: BD L2, SB L2, NG L3, TS L3, BC L2, SQ L2, rC L2 partial, lS L2 partial, sG L2 partial.

---

## New Sketches & Experiments (Phase 25+)

| Artifact | Layer | Purpose |
|----------|-------|---------|
| `graphs/sketches/validation/primal_routing_matrix.toml` | L0 | 10-domain Neural API routing sweep |
| `graphs/sketches/mixed_atomics/dual_tower_ionic.toml` | L2 | Two electron shells, ionic bridge |
| `graphs/sketches/mixed_atomics/node_with_dedicated_tower.toml` | L2 | Proton with dedicated electron |
| `graphs/sketches/mixed_atomics/nest_enclave.toml` | L2 | Neutron-heavy isotope, policy fence |
| `graphs/sketches/bonding_patterns/covalent_mesh_backup.toml` | L3 | Sharded encrypted backup across peers |
| `graphs/sketches/bonding_patterns/ionic_capability_lease.toml` | L3 | Metered electron transfer |
| `graphs/sketches/bonding_patterns/organo_metal_salt.toml` | L3 | Multi-bond compound (covalent + ionic + weak) |
| `experiments/exp091_primal_routing_matrix/` | L0 | Drives routing matrix graph |
| `experiments/exp092_dual_tower_ionic/` | L2 | Structural dual-tower + ionic bond validation |
| `experiments/exp093_covalent_mesh_backup/` | L3 | Structural shard model + covalent policy validation |
| `specs/MIXED_COMPOSITION_PATTERNS.md` | All | Particle model, layered validation, gap inventory |

---

## Secure Socket Architecture (Phase 26)

**Status**: Phase 1 complete. Phases 2-4 implemented.

| Phase | Scope | Status |
|-------|-------|--------|
| Phase 1: Socket Naming | All primals honor `FAMILY_ID` for `{primal}-{fid}.sock` naming | ✓ Implemented |
| Phase 2: BTSP Spec | `BTSP_PROTOCOL_STANDARD.md` in wateringHole | ✓ Written |
| Phase 3: BTSP Session Methods | BearDog: `btsp.session.create/verify/negotiate` | ✓ Implemented |
| Phase 3: biomeOS BTSP Client | Neural API BTSP-aware forwarding, INSECURE guard | ✓ Implemented |
| Phase 4: BondingPolicy Enforcement | `BtspEnforcer` — connection-time + per-request checks | ✓ Implemented |

**Key Changes:**
- BearDog `socket_config.rs`: XDG/BIOMEOS_SOCKET_DIR tiers now family-scoped when `FAMILY_ID` set
- Songbird `env_config.rs`: auto-family-scoped socket when `FAMILY_ID` set (no `SONGBIRD_MULTI_FAMILY` needed)
- NestGate `socket_config.rs`: `FAMILY_ID` fallback added, family-scoped tiers 2-3
- All primals: `BIOMEOS_INSECURE` guard (refuse start when both `FAMILY_ID` + `INSECURE` set)
- biomeOS: `btsp_client` module, security posture logging, family-scoped socket detection
- primalSpring ecoPrimal: `btsp` module (cipher suites, security mode, enforcement), `BtspEnforcer` in bonding

**Resolves**: GAP-MATRIX-11 (biomeOS graph-boot socket path mismatch) — pending live Run 6 validation.

---

## BTSP Adoption Tiers

The Secure Socket Architecture (Phase 26) defined the protocol and implemented it in the Tower Atomic primals. Remaining primals need phased adoption:

| Tier | Primals | Status | What Remains |
|------|---------|--------|--------------|
| **1: BTSP-native** | BearDog | ✓ **Live encrypted** | Full handshake enforcement (Wave 31). Socket listener requires BTSP when FAMILY_ID set. socket_config Tiers 2-4 still need family-scoped naming. |
| **1: BTSP Phase 1** | Songbird, NestGate, ToadStool, BarraCuda, rhizoCrypt, sweetGrass, loamSpine, biomeOS, Squirrel | ✓ **All complete** | FAMILY_ID socket ✓, INSECURE guard ✓. Handshake client not yet implemented (BearDog is the only handshake server). |

---

## Wire Standard Compliance Summary (April 8, 2026)

| Primal | Level | `identity.get` | `capabilities.list` envelope | `provided_capabilities` | `health.liveness` | Notes |
|--------|-------|-----------------|------------------------------|-------------------------|-------------------|-------|
| BearDog | **L2** | ✓ | ✓ `{primal, version, methods}` | ✓ | ✓ | Wave 30 |
| Songbird | **L2** | ✓ | ✓ `{primal, version, methods}` | — | ✓ | Wave 128. L2 on discoverable socket ✓ |
| NestGate | **L3** | ✓ | ✓ full envelope | ✓ 9 groups | ✓ | S36. `consumed_capabilities`, `protocol`, `transport` |
| ToadStool | **L3** | ✓ | ✓ full envelope | ✓ | ✓ | S191. `cost_estimates`, `operation_dependencies` |
| BarraCuda | **L2** | ✓ | ✓ `{primal, version, methods}` | ✓ groups | ✓ | Sprint 33. JSON-RPC + tarpc |
| Squirrel | **L2** | ✓ | ✓ `{primal, version, methods}` | — | ✓ | alpha.46. BTSP Phase 1 ✓, family-scoped socket ✓ |
| rhizoCrypt | **L2 partial** | needs rebuild | Format E (groups) | ✓ 5 groups | ✓ | Needs flat `methods` array for full L2 |
| loamSpine | **L2 partial** | needs impl | Format A | — | ✓ | UDS now ✓ (`permanence.sock`). Needs top-level `methods` + `identity.get` |
| sweetGrass | **L2 partial** | needs impl | Format B | — | ✓ | Needs `provided_capabilities` grouping for L3 |

---

## Next Steps

### Immediate (Run 6)
1. **Run 6 live validation** on Eastgate with all fresh ecoBins + biomeOS v2.99. Validate family-scoped socket naming resolves GAP-11 end-to-end. All 10 primals now have BTSP Phase 1 + fresh ecoBins.

### Short-term (primal convergence)
3. **BearDog socket_config**: add family-scoped naming to Tiers 2-4 (`beardog-{fid}.sock`). Handshake works but graph-boot path mismatch for `beardog` domain persists.
4. **Wire Standard L2 convergence**: rhizoCrypt flat `methods` array, loamSpine `identity.get`, sweetGrass `identity.get` + `provided_capabilities` grouping.
5. **BTSP handshake clients**: Songbird, NestGate, ToadStool, biomeOS need client-side handshake to connect to BearDog's encrypted socket in production mode.

### Medium-term (composition validation)
6. Run exp091 (L0 routing matrix) to validate all 10 primal domains through Neural API.
7. Implement dual-Tower coexistence in AtomicHarness (L2 gap).
8. Extend exp090 pattern to validate each spring's NUCLEUS health remotely.
9. Wire Standard audit — validate each primal against Level 2 checklist (column P).

### Long-term (downstream readiness)
10. Implement erasure coding as barraCuda primitive for L3 sharding.
11. Automate the full matrix (columns A-P) as a CI pipeline per spring.
12. Run live Phase B validation on Eastgate for each spring's deploy graph.
