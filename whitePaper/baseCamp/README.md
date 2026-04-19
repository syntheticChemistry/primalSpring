# primalSpring baseCamp — Coordination and Composition Validation

**Date**: April 20, 2026
**Status**: Phase 44 — v0.9.16 — 75 experiments (17 tracks), 570 tests, 56 deploy graphs (fragment-first composition), **guideStone Level 4** (67/67 live NUCLEUS, 41/41 bare, BLAKE3 checksums P3), multi-tier genetics identity (Mito-Beacon / Nuclear / Tags), BTSP Phase 1–3 (ChaCha20-Poly1305), biomeOS-managed cross-arch deployment (Pixel aarch64 + GrapheneOS), content distribution federation, ionic bond RPC, BtspEnforcer deny semantics, graph consolidation (78→56 via template+manifest + fragment resolution), stadial parity (zero dyn, zero async-trait, Edition 2024), **plasmidBin depot pattern documented** — downstream springs can pull pre-built ecoBin binaries and deploy live NUCLEUS for primal proof validation, 12/12 primals ALIVE, 19/19 exp094 composition parity, 12/12 exp091 routing matrix, 14/15 cross-arch checks (HSM pending)

---

## What This Is

Where baseCamp papers for other springs explore scientific questions using the
ecoPrimals infrastructure, primalSpring's baseCamp explores **the infrastructure
itself**. The "papers" are the atomics. The "experiments" are composition patterns.
The validation target is biomeOS and the Neural API.

## The Paper

See `ecoPrimals/infra/whitePaper/gen3/baseCamp/README.md` (Paper 23 section) for
the full baseCamp paper documenting primalSpring's validation of ecosystem coordination.

## The baseCamp Pattern: Python -> Rust -> Primal

Every spring's baseCamp follows a three-stage validation pipeline that bridges
peer-reviewed science with production-grade primal compositions:

1. **Python baseline** — peer-reviewed, reproducible implementations using
   standard scientific libraries (NumPy, SciPy, etc.). These are the ground
   truth that domain scientists trust.
2. **Rust port** — matches the Python baseline within documented tolerance.
   Pure Rust, zero C dependencies, ecoBin compliant. This is what ships.
3. **Primal composition** — matches the Rust port via IPC through the NUCLEUS
   composition layer. Springs call `validate_parity()` to compare primal
   results against their local Rust baselines.

For primalSpring, this pattern is recursive: the "science" being validated IS
the coordination infrastructure. The "Python baseline" is the spec. The "Rust
port" is the implementation. The "Primal composition" is the live ecosystem.

## Experiments by Track

| Track | Domain | Experiments | Key Question |
|-------|--------|-------------|--------------|
| 1 | Atomic Composition | exp001–006 | Do atomics deploy correctly? |
| 2 | Graph Execution | exp010–015 | Do all 5 coordination patterns work? (3/5 live) |
| 3 | Emergent Systems | exp020–025 | Do Layer 3 systems emerge correctly? |
| 4 | Bonding & Plasmodium | exp030–034 | Does multi-gate coordination work? |
| 5 | coralForge | (exp025) | Does the neural object pipeline work? |
| 6 | Cross-Spring | exp040–044 | Do cross-spring data flows work? |
| 7 | Showcase-Mined | exp050–059 | Do mined coordination patterns hold? |
| 8 | Live Composition | exp060–070 | Tower + Squirrel AI + Nest + Node + NUCLEUS + Graph Overlays + Cross-Primal Discovery |
| 9 | Multi-Node Bonding | exp071–072 | Do bonding policies and data federation structures validate? |
| 10 | Cross-Gate Deployment | exp073–074 | Does cross-gate health probing and LAN covalent mesh work? |
| 11 | gen4 Deployment Evolution | exp075–080 | Does biomeOS substrate validate? Cross-gate routing? Spring deploy sweep? |
| 12 | Deployment Matrix | exp081 | Does the 43-cell deployment matrix validate across arch × topology × preset × transport? |
| 13 | Substrate Stress | exp082–084 | Chaos substrate, federation edge cases, provenance adversarial — does the stack survive? |
| 14 | E2E Composition | exp085–088 | BearDog crypto lifecycle, genetic identity E2E, Neural API routing, storytelling composition |
| 15 | LAN/Covalent + Mixed Composition | exp089–093 | Deployment graph sweep, Tower Atomic LAN probe, L0 routing matrix, L2 dual-tower ionic, L3 covalent mesh backup |
| 16 | Composition Parity | exp094 | Does full NUCLEUS composition produce correct results via IPC? 19/19 checks. |
| 17 | Cross-Architecture Deployment | exp095–096 | Does biomeOS-managed Tower bootstrap on aarch64 Pixel via Neural API `--tcp-only`? 6/9 checks. |

## Current State (v0.9.16)

| Metric | Value |
|--------|-------|
| Experiments | 75 (17 tracks) |
| Total tests | **570** (unit + integration + doc-tests + proptest) |
| Proptest fuzz tests | 22 (IPC protocol, extract, capability parsing, cross-cutting pipeline) |
| clippy (pedantic+nursery+unwrap/expect) | 0 warnings (all-targets) |
| cargo doc | 0 warnings |
| `#[allow()]` in production | 0 |
| unsafe_code | Workspace-level `forbid` |
| C dependencies | 0 (pure Rust, ecoBin compliant, `deny.toml` enforced) |
| Deploy graphs | **56 TOMLs** (6 fragments + 9 profiles + 5 multi-node + 4 spring validation + 2 spring deploy + 3 downstream + 5 bonding + 2 chaos + 2 cross-spring + 4 patterns + 1 federation + ~11 root), fragment-first composition with `resolve = true` |
| Composition subsystems | **7** (C1: Render, C2: Narration, C3: Session, C4: Game Science, C5: Persistence, C6: Proprioception, C7: Full Interactive) |
| Primal gap registry | All LD-01 through LD-10 RESOLVED. Pre-downstream gaps resolved. |
| Discovery | Capability-first: 6-tier + Neural API + `discover_by_capability()` + biomeOS `capability.discover` + `topology.rescan` |
| RPC endpoints | 17 methods (including `graph.waves`, `graph.capabilities`) |
| Niche self-knowledge | `niche.rs` — 47 capabilities, semantic mappings, cost estimates |
| MCP tools | 8 typed tools via `mcp.tools.list` for Squirrel AI |
| Validation harness | Builder `.run()`, `check_bool`, `check_skip`, `check_or_skip`, `check_relative`, `check_abs_or_rel`, `with_provenance()`, `NdjsonSink` |
| Provenance coverage | **100%** — all 75 experiments carry `with_provenance()` metadata |
| Dishonest scaffolding | 0 (all experiments use honest skip or real validation) |
| Tower Atomic | **FULLY UTILIZED** — 41/41 gates (24 core + 17 full utilization) |
| Nest Atomic | **VALIDATED** — 8/8 gates (nestgate storage, model cache) |
| Node Atomic | **VALIDATED** — 5/5 gates (toadstool compute, dual-protocol) |
| NUCLEUS | **VALIDATED** — 58/58 base gates (Tower + Nest + Node) |
| Graph Overlays | **VALIDATED** — 14/14 gates (tier-independent primals via deploy graphs) |
| Squirrel Discovery | **VALIDATED** — 5/5 gates (cross-primal env_sockets, capability.discover) |
| Graph Execution | **LIVE** — 6/6 gates (3/5 coordination patterns live) |
| Provenance Readiness | **STRUCTURAL** — 4/4 gates (launch profiles + deploy graph) |
| Total Gates | **87/87** |
| Squirrel AI | Composition validated (Tower + Squirrel + Anthropic Claude) |
| Subsystem validation | C1: 6/6, C2: 4/4, C3: 8/8, C4: 6/6, C5: **5/5**, C6: 5/5, C7: **10/10** PASS |
| Live pass rate | **43/44 (98%)** — up from 93% → 79% pre-evolution |
| petalTongue | v1.6.6+ integrated, zero-copy IPC, blake3, RenderingAwareness auto-init, PT-07 server discovery |
| Nucleated spring deploys | 6 proto graphs: airSpring, groundSpring, healthSpring, hotSpring, neuralSpring, wetSpring |

## What Changed — Phase 44 (Live NUCLEUS Validation + plasmidBin Depot)

### guideStone Level 4 — Live NUCLEUS (April 20, 2026)

The `primalspring_guidestone` binary now validates against a live 12-primal NUCLEUS
composition deployed from `plasmidBin` ecoBin binaries. **67/67 ALL PASS** (23 skips
for protocol boundaries: BTSP handshake, HTTP-framed UDS services).

Key evolution this phase:
- **BLAKE3 checksums (Property 3)**: `primalspring::checksums` module generates and
  verifies BLAKE3 manifests for all validation-critical source and graph files.
- **Family-aware discovery**: `discover_by_capability()` resolves
  `{capability}-{family}.sock` before falling back to plain sockets.
- **Protocol tolerance**: `IpcError::is_protocol_error()` classifies HTTP-on-UDS as
  reachable-but-incompatible (SKIP, not FAIL). Downstream springs inherit this.
- **plasmidBin depot pattern**: Documented for downstream consumption — pull pre-built
  musl-static ecoBin binaries, deploy NUCLEUS, run `guideStone` externally.
- **Graph TOML fixes**: Fragment ordering (`meta_tier`), profile deconfliction
  (`full`, `nest_viz`), validation graph restructuring.
- **72 clippy warnings eliminated**: All `doc_markdown`, `unwrap_or`, and match
  destructuring warnings resolved across 14 source files.
- **9 new gaps documented (PG-16–PG-23)**: 7 resolved in-session, 2 tracked for
  upstream evolution (BearDog family seed, biomeOS TCP port).

The validation pipeline is now: `Python baseline → Rust proof → guideStone bare →
guideStone NUCLEUS → primal proof`. Downstream springs can begin Level 5 primal
proof validation cycles using the plasmidBin depot and the three-tier composition
pattern (LOCAL → IPC-WIRED → FULL NUCLEUS).

## What Changed — Phase 43+ (guideStone Composition Certification)

### guideStone as Base Certification Layer (April 18, 2026)

The **Python → Rust → Primal** validation ladder now has a concrete deployment
artifact: the **guideStone**. Each spring evolves from a Rust validation binary
(Level 2, the "Rust proof") to a self-validating NUCLEUS node (Level 5, the
"primal proof"). primalSpring's own guideStone validates composition correctness
as the base certification layer.

**primalspring_guidestone binary** — 6-layer NUCLEUS composition certification:
- **Layer 0 (Bare)**: Deploy graph parsing, fragment resolution, manifest consistency, bonding type well-formedness. Runs on any clean machine without primals.
- **Layer 1 (Discovery)**: All primals in the graph discoverable via capability-based scan.
- **Layer 2 (Health)**: Tower, Node, Nest primals respond to `health.liveness`.
- **Layer 3 (Parity)**: `stats.mean`, `tensor.matmul`, storage roundtrip, shader capabilities via IPC.
- **Layer 4 (Cross-Atomic)**: Tower hash → Nest store → retrieve → verify (end-to-end proof).
- **Layer 5 (Bonding)**: All `BondType` variants well-formed, cipher policy ordering, metering.
- **Layer 6 (BTSP+Crypto)**: `crypto.hash` determinism, BTSP cipher policy, Ed25519 sign/verify roundtrip.

**Layered certification model**: Domain guideStones (hotSpring, healthSpring, etc.)
inherit the base certification and only validate their own domain science on top.
If primalSpring's guideStone passes, discovery, health, and crypto are certified.

**Composition API evolution**: `capability_to_primal()`, `method_to_capability_domain()`,
and `validate_liveness()` now form the canonical API for downstream springs building
their own guideStone binaries.

**Ecosystem guideStone readiness** (as of April 18, 2026):
- hotSpring v0.6.32: **Level 5 — Certified** (guideStone-v0.7.0)
- healthSpring V53, neuralSpring V133, wetSpring V145, ludoSpring V44: **Level 1** (validation exists)
- airSpring v0.10.0, groundSpring V124: **Level 0** (not started)
- primalSpring v0.9.15: **Level 1** (6-layer binary implemented, needs live NUCLEUS for full certification)

See `wateringHole/GUIDESTONE_COMPOSITION_STANDARD.md` for the full standard.

## What Changed — Phase 42–43 (Multi-Tier Genetics + Cross-Arch Deployment)

### Multi-Tier Genetics Architecture (April 14, 2026)

The plaintext `FAMILY_SEED` first draft evolved into the dark forest protocol:

| Tier | Genetic Name | Analogy | Role |
|------|-------------|---------|------|
| Mito-Beacon | Mitochondrial genetics | Inherited, shared | Group membership, discovery, NAT negotiation |
| Nuclear (Lineage DNA) | Nuclear genetics | Non-fungible, generational | Permissions, authentication — always spawn + mix, never copy |
| Tags | Open channels | Hashtags, chat rooms | Derived from plaintext seed heritage, open participation |

Key properties: Grandma can let a cousin know how to reach you without giving away your contacts. A school alum has a mito-beacon for their degree. A gamer has many from guilds. Nuclear genetics are permission-bearing and copy-resistant — lineage tracking ensures generational mixing at every step rather than cloning the nuclear material.

**BTSP Phase 3**: ChaCha20-Poly1305 encrypted post-handshake channel with HKDF-SHA256 key derivation. `BtspEnforcer` evolved with explicit deny semantics per `TrustModel`.

**Ionic Bond Protocol**: Cross-family capability sharing via `IonicBondClient` RPC wiring. `IonicPeerIdentity` + `DiscoveryMethod` types.

**Content Distribution Federation**: 8-phase pipeline graph (`content_distribution_federation.toml`) with 4 bonding tiers and BLAKE3 content addressing. Domain types: `ContentManifest`, `DistributionRole`, `SeederEnrollment`.

### Cross-Architecture Deployment via biomeOS (April 14, 2026)

**biomeOS as composition substrate** — stopped manual primal wiring. biomeOS `neural-api --tcp-only` bootstraps the Tower from `tower_atomic_bootstrap.toml`:

1. Cross-compiled `biomeos-unibin`, BearDog, Songbird for `aarch64-unknown-linux-musl`
2. Deployed to Pixel (GrapheneOS) via ADB
3. biomeOS spawned BearDog + Songbird as child processes
4. Neural API on TCP port 9000, BearDog on TCP 9900
5. `exp096` validated 6/9 checks through Neural API proxy

**Transport evolution**: `tcp_rpc_multi_protocol` tries raw TCP then HTTP POST fallback. Removed premature `shutdown(Write)` from `tcp_rpc`. Added `http_json_rpc` for Songbird's HTTP transport.

**Upstream biomeOS patch**: `bootstrap.rs` now inherits `BIOMEOS_PLASMID_BIN_DIR`, `ECOPRIMALS_PLASMID_BIN`, `XDG_RUNTIME_DIR`, `FAMILY_SEED` into `ExecutionContext`.

**Remaining upstream gaps**: TCP endpoint propagation in NeuralRouter, graph env variable substitution (`${FAMILY_ID}` etc.).

## What Changed — Phase 28 (BTSP + Inference Abstraction + Proto-Nucleate Graphs)

### Zero-Port Tower Atomic + BTSP Phase 2 + Inference + Nucleation (April 10, 2026)

**Zero-Port Tower Atomic achieved** — 0 TCP ports across all 10+ primals. Pure Unix domain socket IPC. BTSP (`btsp.handshake`) authenticates every connection with FAMILY_ID + nonce + HMAC.

**BTSP Phase 2 Cascade** — 11/13 primals enforce `btsp.handshake` on connection. Client-side `ipc::btsp_handshake` module added to ecoPrimal. All 100 deploy graphs and 4 fragments carry `[graph.metadata] secure_by_default = true` and `btsp_phase = 2`.

**Inference Provider Abstraction** — Vendor-agnostic wire standard in ecoPrimal:
- `inference.complete` / `inference.embed` / `inference.models` — JSON-RPC 2.0 methods
- `InferenceClient` discovers providers via socket probing or environment variable
- Squirrel bridge: routes `inference.*` calls through `AiRouter` (Ollama as OpenAI-compatible HTTP endpoint)
- No vendor lock-in to CUDA or Ollama — springs compose barraCuda WGSL shaders for native ML

**WGSL Shader Composition Model** — The unifying pattern across all science springs:
- barraCuda provides 826 WGSL compute shaders (matmul, attention, FFT, df64)
- coralReef compiles WGSL programs (tokenizer kernels, QCD operators)
- toadStool dispatches workloads to GPU/CPU substrates
- Springs are application layers composing these primals, not standalone compute engines
- Paper 23's mass-energy-information equivalence realized: same shaders compose into ML inference, QCD physics, biology

**Proto-Nucleate Graphs** (5 new in `graphs/downstream/`):
1. `neuralspring_inference_proto_nucleate.toml` — ML inference as WGSL shader composition (coralReef + toadStool + barraCuda)
2. `hotspring_qcd_proto_nucleate.toml` — Lattice QCD on metallic GPU pool (df64 precision, provenance for reproducibility)
3. `healthspring_enclave_proto_nucleate.toml` — Dual-tower enclave (ionic bond, egress fence, clinical AI via Squirrel)
4. Additional composition variants for multi-tenant and hybrid deployments

**Pipeline Graphs** (3 new):
1. `neuralspring_inference_pipeline.toml` — Squirrel → neuralSpring → coralReef → toadStool → barraCuda → NestGate
2. `hotspring_qcd_pipeline.toml` — hotSpring → coralReef → toadStool → barraCuda → NestGate → sweetGrass
3. `healthspring_clinical_pipeline.toml` — healthSpring → NestGate-A → ionic bridge → Squirrel → NestGate-B → sweetGrass

**ToadStool Semantic Cleanup** — Removed unimplemented `ollama.*` and `inference.*` method mappings. ToadStool is compute substrate, not inference provider.

**Metrics**: 404 tests, 72 experiments (15 tracks), 100 deploy graphs + 4 fragments. 12/12 plasmidBin ecoBin compliant.

## What Changed — Phase 26 (Mixed Composition + Live Validation Matrix)

### Particle Model + Layered Validation + Live Probing (April 7, 2026)

Adopted particle model from Paper 23 (Mass-Energy-Information Equivalence) for compositional reasoning:

| Particle | Atomic | Role | Property |
|----------|--------|------|----------|
| Electron | Tower (BearDog + Songbird) | Mediates bonds, trust boundary | `shares_electrons()` for Covalent/Metallic |
| Proton | Node (Tower + ToadStool) | Compute = energy, defines gate identity | Fungible across gates |
| Neutron | Nest (Tower + NestGate + Squirrel) | Data = energy at rest, stabilizes nucleus | Non-fungible, unique content |
| Atom | NUCLEUS | Complete system | All particles composed |

**Layered Validation Framework** (L0-L3):
1. **L0**: biomeOS Neural API + any single primal routing (`primal_routing_matrix.toml`)
2. **L1**: Each atomic composition independently (Tower, Node, Nest)
3. **L2**: Mixed atomics — a Node with a dedicated Tower separate from the atomic Tower (`dual_tower_ionic.toml`, `node_with_dedicated_tower.toml`, `nest_enclave.toml`)
4. **L3**: Bonding patterns on top of atomics — covalent mesh backup, ionic capability lease, organo-metal-salt complex

**Live Validation on Eastgate (April 7, 2026)**:
- BearDog: `health.liveness` LIVE PASS, `crypto.sign_ed25519` LIVE PASS, `crypto.blake3_hash` LIVE PASS, 9 capability groups
- Songbird: `health.liveness` LIVE PASS, HTTPS `ifconfig.me` LIVE PASS (HTTP 200, 283ms), Tower crypto confirmed
- Neural API: Running, detects sockets, but `capability.call` FAILS — **GAP-MATRIX-01** (0 capabilities registered from primals)

**6 GAP-MATRIX Items Documented** (see `specs/CROSS_SPRING_EVOLUTION.md`):
1. **GAP-MATRIX-01 (Critical)**: Neural API capability registration — primals advertise via `capabilities.list` but biomeOS probe returns 0 capabilities
2. **GAP-MATRIX-02 (Medium)**: biomeOS graph parser rejects `tower_atomic_bootstrap.toml`
3. **GAP-MATRIX-03 (Low)**: Songbird TLS cipher suite gaps for some HTTPS targets
4. **GAP-MATRIX-04 (Medium)**: NestGate HTTP REST IPC diverges from JSON-RPC over UDS model
5. **GAP-MATRIX-05 (Medium)**: Provenance trio + Squirrel + ToadStool not yet live-tested
6. **GAP-MATRIX-06 (Low)**: plasmidBin binary freshness varies across primals

**New Artifacts**:
- `specs/MIXED_COMPOSITION_PATTERNS.md` — particle model, gap inventory, spring specialization guide
- `specs/NUCLEUS_VALIDATION_MATRIX.md` — updated with live results and sketch cross-references
- 17 sketch graphs in `graphs/sketches/` (validation, mixed_atomics, bonding_patterns)
- exp091 (L0 routing), exp092 (L2 dual-tower), exp093 (L3 mesh backup)

**Metrics**: 404 tests, 72 experiments (15 tracks), 99 deploy graphs. 6 GAP-MATRIX items documented.

## What Changed — Phase 25 (Modernization Sweep)

### Pattern Cleanup + Tower Atomic Validation (April 7, 2026)

Comprehensive modernization sweep resolving all legacy patterns:

1. **Capability naming canonical** — `dag.dehydrate` → `dag.dehydration.trigger`, plus 5 other stale method names, across 17 graph files + `capability_registry.toml` + `niche.rs`
2. **Graph format unified (NA-016)** — Parser accepts `[[graph.node]]`, `[[graph.nodes]]`, `[[nodes]]` via serde alias + merge. All 87+ graphs migrated to `[[graph.nodes]]`. `GraphMeta` gains `id` field.
3. **`http_health_probe` deprecated (NA-009)** — Tower Atomic owns all HTTP. 4 experiments updated to `tcp_rpc` + `health.liveness`.
4. **`nest-deploy.toml` v4.0** — Gold standard: HTTPS validation phase (calls `http.get` through Tower Atomic), Songbird mesh capabilities.
5. **exp090 Tower Atomic LAN probe** — BirdSong mesh discovery, peer capability enumeration, HTTPS through Tower, STUN/NAT.
6. **exp073 modernized** — Neural API routing, `FAMILY_ID` genetic lineage, HTTPS through Tower.
7. **`basement_hpc_covalent.toml`** — Canonical capability names, HTTPS validation phase inserted.
8. **Cross-primal alignment** — Songbird TLS 1.3 fix rebased, trio naming confirmed, biomeOS graph format aligned.
9. **Resolved gaps** — NA-009 (capability naming), NA-016 (graph format).

**Metrics**: 404 tests, 69 experiments (15 tracks), 92 deploy graphs. 2 gaps resolved.

## What Changed — Phase 23g (Primal Rewiring + Gap Cleanup)

### Primal Evolution Rewiring (April 1, 2026)

Full primal audit and rewiring cycle. Reviewed all 10 primals, updated
primalSpring internals to match latest APIs, cleaned gap registry to
primal-only scope, validated at 98%.

**Code Rewiring**:
- `ipc/methods.rs` — `graph.execute` (was `graph.deploy`), `topology.rescan` (biomeOS v2.81), `ember.*` (toadStool S171), `ai.*` (Squirrel), `visualization.*`/`interaction.*` (petalTongue). Removed downstream modules (`game.*`, `webb.*`, `session.*`)
- `ipc/neural_bridge.rs` — `topology_rescan()` for biomeOS v2.81
- `ipc/discover.rs` — 6-tier discovery with plain socket names (`{name}.sock`, `{name}-ipc.sock`)
- `tools/validate_compositions.py` — SQ-02 resolved messaging, NestGate `family_id`, C7 live Squirrel check

**Gap Resolution** (5 newly resolved):
- SQ-02: `LOCAL_AI_ENDPOINT` wired into AiRouter (alpha.27)
- PT-05: `RenderingAwareness` auto-init in `UnixSocketServer`
- PT-07: periodic discovery refresh in server mode
- NG-04: ring/aws-lc-rs eliminated (TLS → system curl)
- NG-05: nestgate-security zero crypto deps (BearDog IPC delegation)

**Nucleated Spring Deploy Graphs** (6 new):
- `graphs/spring_deploy/` — airSpring, groundSpring, healthSpring, hotSpring, neuralSpring, wetSpring
- Each provides NUCLEUS base (biomeOS + BearDog + Songbird) + domain-specific primals
- Downstream tributaries consume these as proto-compositions

**Gap Registry Cleanup**:
- Scoped to primals only — removed downstream (esotericWebb, ludoSpring, cross-cutting)
- 8 open gaps (1 medium, 7 low), zero critical/high
- 18 resolved across all primals

## What Changed — Phase 23c (NUCLEUS Atomics + biomeOS Substrate)

### biomeOS Substrate Integration (March 28, 2026)

biomeOS Neural API is now an explicit substrate in every NUCLEUS composition.
`SubstrateHealth` in `CompositionResult` tracks Neural API health independently.
All 4 canonical atomic deploy graphs (Tower, Node, Nest, Full) include a Phase 0
`biomeos_neural_api` node (required, spawn=false, health.liveness) to verify the
orchestration substrate before any primals deploy.

**Nest Alignment**: Squirrel added to `AtomicType::Nest` (`required_primals` + `ai`
capability), matching biomeOS's `nucleus --mode nest` runtime behavior.

**Full NUCLEUS Alignment**: NestGate, ToadStool, Squirrel marked `required = true`
in `nucleus_complete.toml`, matching the 5 core primals biomeOS starts in Full mode.

**New Validation Graph**: `nucleus_atomics_validate.toml` — validates all 4 NUCLEUS
tiers + Tower+Squirrel overlay + structural graph validation + biomeOS substrate.

**composition.tower_squirrel_health**: Wired in `primalspring_primal` dispatch —
was advertised in capability registry but previously unimplemented.

**Metrics**: 402 tests, 67 experiments, 63 deploy graphs (11 validation), 63 method
constants, 47 capabilities in registry, 0 clippy warnings.

## What Changed — Phase 23 (Ecosystem Debt Resolution + Standards)

### Debt Resolution + Composition Standards (March 29, 2026)

Comprehensive ecosystem audit reclassified 11 findings from the initial deep audit into
proper categories: intentional design (2), deployment wiring (2), defensive coding (1),
documentation (4), test gaps (2). Executed all actionable debt across BearDog, Songbird,
and biomeOS.

**Upstream Fixes Driven by primalSpring Audit**:
- BearDog: `genetic.derive_lineage_beacon_key` registered, empty/zero/short seed rejection, federation label fix, HSM labeling correction
- Songbird: Dark Forest / legacy birdsong / dual broadcast env vars wired into BirdSongConfig
- biomeOS: eprintln→tracing in capability_domains.rs

**New Validation Graph**: `crypto_negative_validate.toml` — 9-node graph validating security
rejection paths (wrong-seed lineage, empty-seed rejection, tampered-payload detection, same-family beacon decrypt).

**exp086 Evolution**: Full generate-then-verify lineage round-trip with both positive (correct seed verifies) and negative (wrong seed fails) tests. Added `base64` dependency, `GENERATE_LINEAGE_PROOF` method constant.

**Standards Documents for wateringHole**:
- `COMPOSITION_PATTERNS.md` — canonical reference for both deploy graph formats, niche YAML, primal launch profiles, 8-step socket discovery
- `SPOREGARDEN_DEPLOYMENT_STANDARD.md` — BYOB model, esotericWebb reference, environment contract
- Per-primal team debt handoffs (BearDog, Songbird, biomeOS)
- Glossary updated with 6 composition terms

**Metrics**: 67 experiments, 402 tests, 63 deploy graphs (11 validation), 0 clippy warnings, 0 unsafe, 0 C deps.

## What Changed — Phase 22 (E2E Composition Testing)

### Track 14: E2E Composition (March 29, 2026)

4 new experiments (exp085–exp088) for end-to-end composition testing:
- exp085: BearDog crypto lifecycle (Ed25519, Blake3, BirdSong beacon, secrets)
- exp086: Genetic identity E2E (mito beacon seed vs nuclear lineage, family scoping)
- exp087: Neural API routing E2E (security, discovery, storage, compute, AI domains)
- exp088: Storytelling composition (ludoSpring + esotericWebb + Squirrel + petalTongue)

`ipc::methods` expanded to 16 domain modules. `validate_composition.sh` added.
ludoSpring game method gap handoff for esotericWebb contract.

## What Changed — Phase 21 (Deep Ecosystem Audit + Library Consolidation)

### Deep Audit Execution (March 29, 2026)

Comprehensive 8-axis audit against ecosystem standards (`wateringHole/`) with full
remediation execution. Zero TODOs/FIXMEs/HACKs remain. Zero clippy warnings (pedantic+nursery).

**Library Consolidation**:
- **`ipc::tcp`** — shared TCP RPC helper (`tcp_rpc`, `tcp_rpc_with_timeout`, `http_health_probe`, `env_port`) extracted from 8 experiments into library module
- **`ipc::methods`** — centralized JSON-RPC method name constants (`health::LIVENESS`, `capabilities::LIST`, `provenance::*`, etc.) — zero hardcoded method strings in experiments
- **`ipc::capability`** — capability discovery and routing extracted from `ipc/discover.rs`
- **`launcher/`** — smart refactor into 4 sub-modules: `discovery.rs`, `profiles.rs`, `spawn.rs`, `biomeos.rs` (public API preserved)

**Provenance Circuit Breaker Evolution**:
- Time-based half-open state with `TRIO_OPENED_AT` epoch tracking
- Probe token via `AtomicBool` — single probe admitted during half-open window
- Graceful mutex poisoning handling (circuit defaults to open on poison)

**Tracing Migration**: Library `println!`/`eprintln!` → `tracing::info!`/`tracing::error!` (harness, validation/or_exit)

**Experiment Consolidation**: 8 experiments (`exp063`, `exp073`, `exp074`, `exp076`, `exp081`–`exp084`) refactored to use library `ipc::tcp` helpers and `ipc::methods` constants. Hardcoded primal name strings replaced with `primal_names::*` in 4 experiments.

**Test Growth**: 385 → 411 tests (+26). New tests cover: `ipc::tcp` module (TCP RPC, health probe, env port), `ipc::methods` constants (health, capabilities, provenance, coordination), provenance circuit breaker half-open state, launcher sub-module APIs.

**Transport Unification**: `PrimalClient` now uses `Transport` enum internally — single code path for Unix + TCP IPC.

**Phase 21 Metrics**: 411 tests, 63 experiments, 59 deploy graphs, 0 clippy warnings (all-targets), 0 fmt diff, 0 doc warnings, 0 `#[allow()]` in production, 0 unsafe, 0 C deps.

## What Changed — Phase 20 (Deployment Matrix + Substrate Validation)

### Deployment Matrix + Validation Substrate (March 28, 2026)

Built a comprehensive deployment validation matrix and substrate validation system using
benchScale (Docker topologies) and agentReagents (image provisioning) from `ecoPrimals/infra/`.

**Deployment Matrix** (`config/deployment_matrix.toml`):
- **43 test cells** across x86_64/aarch64, 7 network presets, UDS/TCP transport modes
- Matrix runner: `scripts/validate_deployment_matrix.sh` with dry-run, per-cell, and tier modes
- Known blockers tracked: biomeOS TCP-only, Squirrel abstract socket, ludoSpring IPC surface

**benchScale Topologies** (15 new in `infra/benchScale/topologies/`):
- Niche/constrained: Alpine minimal, read-only FS, 256MB constrained
- Bonding models: Ionic 2-family, metallic fleet, organo-metal-salt
- Scale: 10-node federation, mixed-arch cluster
- Showcase: fieldMouse chimera, Albatross multiplex, skunkBat defensive, neuromorphic edge, gaming mesh
- Agentic: biomeOS + Squirrel + petalTongue tower, agentic fieldMouse
- Storytelling: esotericWebb + ludoSpring + Squirrel AI DM + petalTongue

**Graph Compositions** (23 new):
- `graphs/bonding/` (5): ionic, metallic, OMS, defensive mesh, albatross multiplex
- `graphs/chaos/` (2): partition recovery, slow-start convergence
- `graphs/science/` (10): coralForge federated, ecology provenance, reproducibility audit, fieldMouse ingestion, paper lifecycle, supply chain provenance, mixed entropy, gaming mesh, neuromorphic classify, RPGPT session provenance
- `graphs/gen4/` (6 new): agentic substrate, agentic fieldMouse, UI-orchestrator loop, storytelling full, storytelling minimal

**New Experiments** (4):
- exp081: deployment matrix sweep (structural)
- exp082: chaos substrate (kill-and-recover, half-open, rapid reconnect)
- exp083: federation edge cases (asymmetric latency, partial mesh)
- exp084: provenance adversarial (tampered DAG, replay attacks, attribution disputes)

**Chaos Engineering**: `scripts/chaos-inject.sh` — partition, kill, disk-fill, slow DNS, clock drift injection.

**Evolution Specs** (3 new):
- `specs/AGENTIC_TRIO_EVOLUTION.md` — biomeOS + Squirrel + petalTongue as the agentic loop (nervous system + brain + senses)
- `specs/STORYTELLING_EVOLUTION.md` — ludoSpring + esotericWebb AI DM storytelling stack
- `specs/SHOWCASE_MINING_REPORT.md` — patterns mined from primal showcases

**Primal Integration Analysis**:
- biomeOS: 285+ methods, 26 domains, 5 graph patterns — P0 gap: TCP-only `--port` ignored
- Squirrel: MCP hub, AI inference, context — P0 gap: abstract socket vs filesystem
- petalTongue: TUI/egui/web/headless, SSE events — P1 gap: dialogue-tree scene type
- ludoSpring: 8 game science IPC methods — P0 gap: 6 methods esotericWebb needs
- esotericWebb: CRPG narrative engine — P0 gap: TCP-first vs biomeOS UDS-first

## What Changed — Phase 19 (Gen4 Spring Scaffolding)

### Spring Primal Build + Deploy (March 28, 2026)

Resolved broken `path = "..."` dependencies across 7 spring repositories by creating local
symlinks for `barraCuda`, `bingoCube`, `toadStool`, `coralReef`, `loamSpine`, `rhizoCrypt`,
and `sweetGrass`. Patched upstream primal crates to align APIs with spring expectations
(feature-gating, missing fields, precision variants). Built 5 of 6 spring primal binaries.

| Spring | Binary | Status | Notes |
|--------|--------|--------|-------|
| groundSpring | `groundspring` | **BUILT** | `--no-default-features --features biomeos` |
| healthSpring | `healthspring_primal` | **BUILT** | Full features |
| ludoSpring | `ludospring` | **BUILT** | Full features |
| neuralSpring | `neuralspring` | **BUILT** | Full features |
| wetSpring | `wetspring` | **BUILT** | `--no-default-features` |
| airSpring | `airspring_primal` | **BLOCKED** | Internal `data::Provider` / `data::NestGateProvider` API drift |

**plasmidBin updates**: `manifest.toml`, `sources.toml`, `checksums.toml` (blake3),
`doctor.sh` spring inventory section — all 5 binaries stripped and registered.

**gen4_spring_composition.toml**: New master graph deploying Tower + biomeOS + all 5
spring primals with cross-spring validation node.

**Spring validation graphs**: All 7 updated to deploy biomeOS as substrate (`start_biomeos`
node, order 2) before germinating the spring primal.

**Launch profiles**: Added profiles for all 6 springs in `primal_launch_profiles.toml`
(env vars, socket mappings, dependent primal sockets).

**Upstream patches applied**:
- `barraCuda`: version bump 0.3.5→0.3.7, `F16` precision variant, GPU feature-gating
  for `plasma_dispersion` and `analyze_weight_matrix`, missing `DeviceCapabilities` methods,
  `rel_tolerance` field on `Check`, `PrecisionRoutingAdvice` re-export
- `bingoCube/nautilus`: no-op `json` feature gate, `input_dim` field on `ShellConfig`

## What Changed — Phase 18 (Live NUCLEUS + Cross-Gate Federation)

### Live Deployment Validation (March 28, 2026)

Full NUCLEUS deployed and validated on Eastgate with all 5 primals running concurrently
under biomeOS orchestration. Cross-gate federation demonstrated between Eastgate (x86_64)
and Pixel 8a (aarch64) via ADB TCP port forwarding.

**Eastgate NUCLEUS (Full Stack)**:
1. **biomeOS** — Neural API server running, 39+ deploy graphs loaded, `graph.list` and `capability.call` operational
2. **BearDog** — crypto/security via Unix socket, `health`, `generate_keypair`, `sha256` routed through biomeOS
3. **Songbird** — network orchestration, mesh initialized, STUN public address discovery (162.226.225.148), BirdSong beacons
4. **NestGate** — storage via Unix socket, store/retrieve round-trip validated through biomeOS
5. **Squirrel** — AI/MCP via abstract socket `@squirrel`, `ai.*` domain registered
6. **FAMILY_ID reconciliation** — all primals use seed-derived `8ff3b864a4bc589a` matching biomeOS internal routing

**Cross-Gate: Eastgate ↔ Pixel Federation**:
7. **Pixel Songbird (TCP)** — running v0.1.0 on TCP ports 9200/9901 (SELinux blocks Unix sockets)
8. **ADB port forwarding** — Pixel 9901 → Eastgate 19901, Pixel 9200 → Eastgate 19200
9. **`route.register`** — Pixel Songbird registered on Eastgate biomeOS as `gate: pixel8a` with 5 capabilities (network, discovery, http, mesh, birdsong)
10. **Cross-gate health** — Pixel Songbird health confirmed via `tcp://127.0.0.1:19901` from Eastgate
11. **Mesh initialized** — both Eastgate and Pixel mesh networks initialized, announce operational
12. **`primal.info` comparison** — Eastgate v0.2.1 (14 capabilities) vs Pixel v0.1.0 (8 capabilities), binary upgrade needed

**SELinux Mobile Gap (Critical)**:
13. **GrapheneOS blocks `sock_file` creation** — confirmed via audit log: `avc: denied { create } for name="beardog-pixel.sock" scontext=u:r:shell:s0 tcontext=u:object_r:shell_data_file:s0 tclass=sock_file permissive=0`
14. **BearDog** — server mode hard-exits if Unix socket fails; no `--listen` TCP fallback for mobile
15. **biomeOS api** — ignores `--port` flag, forces Unix socket ("HTTP mode deprecated")
16. **biomeOS nucleus** — waits for Unix socket from primals, times out on Android
17. **Songbird** — only primal with `--listen` TCP IPC mode; works correctly on Pixel
18. **Impact**: Tower atomic on Pixel runs degraded (Songbird only, no BearDog crypto, no biomeOS substrate)

### Known Gaps (Updated)
- BearDog needs `--listen <addr>` for TCP-only server mode on Android/mobile
- biomeOS `api` and `nucleus` modes need TCP transport for mobile substrates
- biomeOS `capability.call` does not implement gate-aware routing (`gate` param ignored, always uses primary endpoint)
- Pixel primal binaries are v0.1.0; need aarch64-musl static rebuilds from latest evolution
- Squirrel uses abstract sockets (`@squirrel`); biomeOS routes to filesystem sockets

## What Changed — Phase 17 (gen4 Deployment Evolution)

### biomeOS Substrate Validation (March 27, 2026)
1. **Native NeuralBridge** — replaced `neural-api-client-sync` compile-time dependency with runtime JSON-RPC via `PrimalClient` (zero cross-primal coupling)
2. **`spawn_biomeos()`** — refactored from `spawn_neural_api()`, discovers `biomeos` or `neural-api-server` binary with fallback
3. **biomeOS coordinated mode** — validated on Eastgate: 24 capability domains, 39 deploy graphs, routing `crypto.generate_keypair` and `beacon.generate` through biomeOS → BearDog
4. **Cross-gate routing** — Pixel BearDog/Songbird validated via TCP through ADB-forwarded ports
5. **Squirrel AI primal** — validated via abstract socket `@squirrel`, `ai.*` domain registered in biomeOS
6. **Spring deploy sweep** — all 7 sibling spring + 4 pipeline biomeOS deploy graphs loaded and validated
7. **Cross-spring ecology** — 9-node ET₀ → diversity → spectral pipeline validated structurally
8. **gen4 prototype graphs** — sovereign tower, science substrate, agentic tower, interactive substrate
9. **Spring validation graphs** — 7 per-spring + 2 cross-spring wrappers
10. **6 new experiments** (exp075–080): biomeOS live, cross-gate routing, Squirrel bridge, petalTongue viz, spring sweep, cross-spring ecology
11. **385 tests** (up from 378), **59 experiments** (up from 53), **35 deploy graphs** (up from 22)

## What Changed — Phase 16 (Deep Debt Audit + Centralized Tolerances)

### Comprehensive Audit (March 24, 2026)
Deep debt audit against all ecosystem standards (wateringHole/) and sibling spring conventions:
1. **Tolerance calibration notes** — all 7 latency/throughput constants updated from "pending" to document Phase 15 operational validation history
2. **Provenance trio resilience centralized** — `TRIO_CIRCUIT_THRESHOLD` removed from `ipc/provenance.rs`, trio retry params (`TRIO_RETRY_ATTEMPTS`, `TRIO_RETRY_BASE_DELAY_MS`) centralized in `tolerances/`
3. **Remote gate TCP port defaults** — `DEFAULT_BEARDOG_PORT` through `DEFAULT_SQUIRREL_PORT` centralized (was inline magic numbers in exp073/074)
4. **`extract_capability_names` deduplicated** — coordination delegates to ipc::discover 4-format parser (was local 2-format copy)
5. **Hardcoding evolved to capability-based** — exp010 string match → semantic check, exp073/074 inline ports → tolerances, coordination tests → `primal_names` slug constants
6. **Stale docs cleaned** — validate_all comment, validate_remote_gate.sh usage, niche version bump
7. **Coverage baseline measured** — `cargo llvm-cov` run, cast/dispatch/error/protocol 95–100%, coordination/discover/provenance 66–74%, launcher 21% (requires live primals)
8. **364 tests** (up from 361) — 3 new tolerance tests

## What Changed — Phase 15 (Cross-Ecosystem Absorption)

### Absorption Wave (March 24, 2026)
Absorbed patterns from 7 sibling springs + 10 primals:
1. **`primal_names` slug constants** — `BEARDOG`, `SONGBIRD`, etc. as `pub const` for zero-duplication
2. **Hardcoded primal names eliminated** — `coordination/mod.rs`, `ipc/probes.rs`, `bin/main.rs` use `primal_names::` constants
3. **`unwrap_used` / `expect_used` = `warn` workspace-wide** — healthSpring V42 / wetSpring V135 discipline, `cfg_attr(test, allow)` for tests
4. **`launcher/mod.rs` smart refactored** — tests extracted to `launcher/tests.rs` (802 → 699 LOC), env var names as constants
5. **`ipc/provenance.rs` docs updated** — rhizoCrypt sled→redb migration, capability-based env vars for trio
6. **`CONTRIBUTING.md` + `SECURITY.md`** — neuralSpring V124 ecosystem standard docs
7. **Zero clippy warnings on `--all-targets`** including unwrap/expect discipline

## What Changed — Phase 14 (Deep Debt + Builder Pattern + Full Provenance)

### Builder-Pattern Validation (March 24, 2026)
All experiments standardized on the builder-pattern `ValidationResult`:
1. **`ValidationResult::run()`** — consumes self, prints banner, executes checks, prints summary, exits
2. **All experiments carry structured provenance** — `with_provenance(source, date)` on every experiment
3. **`validation/tests.rs` extracted** — 493-line test module separated from production code (540 lines, was 1016)
4. **Zero `.unwrap()` in experiment binaries** — all replaced with `.or_exit("context")`
5. **Zero `#[allow()]` in production** — 3 integration test files evolved to `#[expect(reason)]`
6. **Doc and config fixes** — broken intra-doc link, stale socket path doc, capability_registry version sync
7. **361 tests** (up from 360), 0 clippy warnings, 0 doc warnings, 0 fmt diff

## What Changed — Phase 11–12 (Multi-Node Bonding + Federation)

### Provenance Trio Neural API (Phase 11)
1. **`ipc::provenance` module** — full RootPulse pipeline via `capability.call` (zero compile coupling)
2. **4 experiments evolved** — exp020 (6-phase commit), exp021 (branch/merge), exp022 (diff/federate), exp041 (E2E chain)
3. **Live probing** — sweetGrass LIVE, rhizoCrypt LIVE (TCP), loamSpine BROKEN (runtime panic)
4. **4 gaps documented** — wire format mismatches, param schema drift

### Multi-Node Bonding + Federation (Phase 12)
5. **BondType expanded** — Covalent, Metallic, Ionic, Weak, OrganoMetalSalt (5 variants)
6. **TrustModel** — GeneticLineage, Contractual, Organizational, ZeroTrust
7. **BondingConstraint + BondingPolicy** — capability masks, bandwidth limits, time windows, concurrency
8. **4 multi-node deploy graphs** — basement HPC, friend remote, idle compute, data federation
9. **`graph_metadata.rs`** — parse + validate `[graph.metadata]` and `[graph.bonding_policy]` from TOML
10. **`stun_tiers.rs`** — 4-tier STUN config parser, sovereignty-first escalation validation
11. **exp071 + exp072** — idle compute policy and data federation validation
12. **303 tests**, 51 experiments, 22 deploy graphs (at time of Phase 12)

### Ecosystem Absorption Wave (Phase 12.1, March 23, 2026)
Absorbed patterns from all 7 sibling springs into primalSpring core:
13. **`deny.toml` ban list** — merged groundSpring V120 + wetSpring V132 C-dep bans (aws-lc-sys, cmake, cc, pkg-config, vcpkg)
14. **Cast discipline lints** — neuralSpring S170 / airSpring V010 clippy cast_* lints workspace-wide
15. **`ValidationSink` enrichment** — `section()` + `write_summary()` from groundSpring V120
16. **`exit_code_skip_aware()`** — 3-way exit from wetSpring V132 (0=pass, 1=fail, 2=all-skipped)
17. **`proptest_ipc` module** — 7 cross-cutting property tests fuzzing the IPC pipeline (healthSpring V41)
18. **`primal_names` module** — canonical display↔slug mapping for 23 primals/springs (neuralSpring pattern)
19. **Provenance trio circuit breaker** — epoch-based breaker + exponential backoff in `ipc::provenance` (healthSpring V41)
20. **303 tests** (up from 280) — zero clippy warnings, zero TODO/FIXME in production

### Ecosystem Absorption Wave (Phase 12.2, March 23, 2026)
Absorbed deeper patterns from all 7 sibling springs into primalSpring core:
21. **`normalize_method()`** — ecosystem-wide JSON-RPC dispatch standard (groundSpring V121, neuralSpring V122, wetSpring V133, healthSpring V42)
22. **`check_relative()` + `check_abs_or_rel()`** — robust numeric validation for both relative and absolute tolerance (groundSpring V120, healthSpring V42)
23. **`NdjsonSink`** — streaming newline-delimited JSON validation output (groundSpring V121, wetSpring V133, neuralSpring V122)
24. **`IpcError::is_recoverable()`** — broader recovery classification beyond `is_retriable()` (neuralSpring V122, wetSpring V133)
25. **`Transport` enum (Unix + Tcp)** — cross-platform IPC layer (airSpring V010, healthSpring V42, groundSpring V121)
26. **`ipc::probes`** — `OnceLock`-cached runtime resource probes for test parallelism (hotSpring V0.6.32, neuralSpring V122)
27. **`validate_release.sh`** — release quality gate: fmt + clippy + deny + test floor (320) + docs
28. **`missing_docs` → `deny`** — all public items documented, lint level upgraded from warn
29. **Server `normalize_method()` dispatch** — prefix-agnostic routing for all ecosystem callers
30. **360 tests** (up from 303) — zero clippy warnings, zero missing docs

### Cross-Gate Deployment Tooling (Phase 13, March 23, 2026)
Built deployment pipeline for live multi-gate LAN covalent deployment:
31. **`build_ecosystem_musl.sh`** — build all 6 core primals as x86_64 + aarch64 musl static binaries
32. **`prepare_spore_payload.sh`** — assemble USB spore deployment payload (binaries + graphs + genetics)
33. **`validate_remote_gate.sh`** — probe remote gate NUCLEUS health via TCP JSON-RPC
34. **exp073_lan_covalent_mesh** — cross-gate Songbird mesh + BirdSong beacon exchange via TCP
35. **exp074_cross_gate_health** — remote per-primal TCP health + capabilities + composition assessment
36. **exp063 evolved** — cross-device Pixel beacon exchange via `PIXEL_SONGBIRD_HOST` + TCP
37. **`basement_hpc_covalent.toml`** — annotated with full gate inventory from HARDWARE.md
38. **LAN_COVALENT_DEPLOYMENT_GUIDE** — step-by-step handoff for all gate operators
39. **53 experiments** (up from 51), **10 tracks** (up from 9) — later expanded to 59/11 in Phase 17

## What Changed (v0.6.0 -> v0.7.0)

### Graph-Driven Overlay Composition
1. **Tier-independent primals** — Squirrel, petalTongue, biomeOS compose at any atomic tier via deploy graphs
2. **`spawn` field on GraphNode** — distinguishes primal nodes from validation nodes
3. **5 new overlay deploy graphs** — tower_ai, tower_ai_viz, nest_viz, node_ai, full_overlay
4. **`merge_graphs()`** — merge base + overlay deploy graphs at runtime
5. **exp069** — end-to-end overlay validation (25/25 checks)

### Squirrel Cross-Primal Discovery
6. **9 env_sockets** — Squirrel discovers NestGate, ToadStool, Songbird, BearDog via explicit env vars
7. **full_overlay.toml** — Tower + Nest + Node + Squirrel (all capability domains)
8. **exp070** — cross-primal discovery validation
9. **4 new integration tests** — squirrel_discovers_sibling_primals, tool_list, context_create, ai_query

### Graph Execution Patterns (3/5 Live)
10. **exp010 Sequential** — live Tower composition with ordering verification
11. **exp011 Parallel** — live 4-primal burst (beardog+songbird+nestgate+toadstool)
12. **exp012 ConditionalDag** — live toadstool/CPU fallback branching
13. exp013/014 — awaiting provenance trio (sweetGrass, rhizoCrypt, loamSpine)

### Provenance Readiness
14. **Launch profiles** — sweetGrass, loamSpine, rhizoCrypt socket wiring
15. **provenance_overlay.toml** — Tower + RootPulse deploy graph
16. **Handoffs delivered** — provenance trio team + all teams

## What Changed (v0.5.0 -> v0.6.0)

### NUCLEUS Composition (v0.6.0)
1. **Nest Atomic** — nestgate storage primal integrated
2. **Node Atomic** — toadstool compute primal integrated
3. **NUCLEUS Composition** — all 3 atomic layers compose together (58/58 gates)
4. **3 new experiments** — exp066, exp067, exp068
5. **12 new integration tests** — 8 Nest + 4 Node

## Co-Evolution Strategy

| Phase | Focus | Partners | Gate Target | Status |
|---|---|---|---|---|
| Tower Stability | All 24 Tower gates | beardog, songbird, biomeOS | Gates 1–6 (24/24) | **DONE** |
| Tower + Squirrel | AI composition | + squirrel | AI gates | **DONE** |
| Tower Full Utilization | Subsystems + viz | + petalTongue | Gates 7–11 (41/41) | **DONE** |
| Nest Atomic | Storage gates | + nestgate | Gates 12–13 (8/8) | **DONE** |
| Node Atomic | Compute gates | + toadstool | Gates 14–15 (5/5) | **DONE** |
| NUCLEUS Composition | All layers compose | Tower + Nest + Node | Gate 16 (4/4) | **DONE** |
| Graph Overlays | Tier-independent primals | + squirrel, petalTongue | Gates 17–20 (14/14) | **DONE** |
| Squirrel Discovery | Cross-primal wiring | + all primals | Gate 21 (5/5) | **DONE** |
| Graph Execution | 3/5 patterns live | Tower + Nest + Node | Gate 22 (6/6) | **DONE** |
| Provenance Readiness | Structural prep | sweetGrass/loamSpine/rhizoCrypt | Gate 23 (4/4) | **DONE** |
| Provenance Trio Neural API | ipc::provenance wired | + sweetGrass/loamSpine/rhizoCrypt | Neural API gates | **DONE** |
| Multi-Node Bonding | BondType, BondingPolicy, STUN, federation graphs | + Songbird, NestGate | Bonding gates | **DONE** |
| Emergent E2E | RootPulse + coralForge live | + provenance trio running | Emergent gates | **NEXT** |

See `specs/TOWER_STABILITY.md` for the full 87-gate acceptance criteria.

## Hardware Validation (March 22, 2026)

primalSpring validated against physical hardware: Pixel 8a (aarch64), 3 USB
spores (biomeOS1, LiveSpore, ColdSpore), and SoloKey 2.

| Target | Result |
|--------|--------|
| aarch64-unknown-linux-musl cross-compile | primalspring 0.7.0 runs on Pixel |
| Pixel primal execution | beardog 0.9.0, songbird 3.33.0, squirrel 0.1.0, toadstool 0.1.0, nestgate 2.1.0 all execute |
| USB primal execution | 5/6 primals run (nestgate has corrupt static build on USB) |
| Host validate_all | 47/49 pass (exp060/061 blocked by external primal socket timeouts) |
| genomeBin packages | Zero .genome self-extractors exist; raw binaries only |
| plasmidBin aarch64 | Missing; only x86_64 in ecosystem plasmidBin |

**Blockers for full Pixel atomic deployment:**
- SELinux `sock_file` creation denied for all primals in `shell` context (GrapheneOS)
- BearDog lacks `--listen` TCP-only server mode (hard-exits on socket bind failure)
- biomeOS `api`/`nucleus` modes ignore `--port`, force Unix sockets
- Pixel binaries stale (v0.1.0); need aarch64-musl static rebuilds from latest waves
- biomeOS `capability.call` lacks gate-aware routing

**What works on Pixel (validated March 28, 2026):**
- Songbird TCP mode (`--listen 127.0.0.1:9901`) — health, mesh.init, mesh.announce
- ADB port forwarding for cross-gate communication
- biomeOS `route.register` to register remote Pixel capabilities on Eastgate
- biomeOS aarch64 binary cross-compiles and starts (fails only at socket bind)

## What Remains

### Structural Debt (plan but don't block handoff)
- **Bonding experiments (exp030-034)**: 13 skipped live checks requiring benchScale Docker labs with 2+ FAMILY_IDs
- **benchScale ecoPrimals integration**: 5 gaps in deploy pipeline (`ECOPRIMALS_INTEGRATION.md`)
- **biomeOS DOWN**: 11/12 primals during testing (Neural API composition validation skips)

### Forward Evolution
- Live multi-node validation with benchScale Docker topologies
- Protocol escalation (JSON-RPC -> tarpc sidecar)
- biomeOS self-composition (runtime graph generation)
- genomeBin packaging: run sourDough to produce .genome self-extractors
- ecoBin compliance: rebuild all primals as static musl for both x86_64 and aarch64
- Anchoring + Economics: sweetGrass anchoring to BTC/ETH

---

**License**: AGPL-3.0-or-later
