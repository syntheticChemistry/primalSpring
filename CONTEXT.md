# SPDX-License-Identifier: AGPL-3.0-or-later

# primalSpring — Context

## What

primalSpring is the coordination and composition validation spring for
the ecoPrimals ecosystem. Its domain IS the ecosystem itself — exclusively:
atomic composition (Tower, Node, Nest, Full NUCLEUS), graph execution
patterns, emergent systems, multi-node bonding, and cross-spring interaction.

primalSpring is NOT a science spring. It does not own domain-specific
validation — that belongs to each gate's springs (hotSpring → physics,
wetSpring → biology, etc.). primalSpring validates that compositions of
primals work together; it is the bonding mechanics, not the atoms.
See the ecosystem `GATE_SPRING_OWNERSHIP.md` for the canonical gate-spring
routing SSOT (published via wateringHole cascade).

## Role

primalSpring sits upstream of all springs and gardens but downstream of
primals. Where other springs validate domain science (hotSpring → physics,
wetSpring → biology), primalSpring validates the coordination layer
that biomeOS and the Neural API produce when primals work together.
It has self-knowledge of coordination patterns and discovers other
primals at runtime via capability-based discovery.

Downstream tributaries (springs, gardens) consume our patterns from
`wateringHole/`. As they validate, they expose new gaps that flow
back upstream to primals and primalSpring.

## Architecture

- **ecoPrimal/** — library crate (`primalspring`) + 3 binaries:
  `primalspring` (eukaryotic UniBin: certify + validate + serve + status + checksums + registry + version),
  `primalspring_primal` (JSON-RPC IPC server / cell membrane),
  `nucleus_launcher` (Rust NUCLEUS lifecycle: start/stop/status with PID tracking + federation)
- **experiments/** — 93 validation binaries covering 21 tracks
- **graphs/** — ~80 deploy graph TOMLs + 33 atomic composition graphs using fragment-first
  composition (14 root + 9 profiles + 6 fragments + 9 spring validation + 5 multi-node +
  5 bonding + 4 patterns + 4 desktop + 3 downstream + 2 spring deploy + 2 chaos +
  2 cross-spring + 1 federation + 1 foundation + 12 cell graphs + `compositions/` tier)
- **docs/** — structured gap registry (`PRIMAL_GAPS.md`), wire contracts (discovery, storage, crypto), migration guides
- **tools/** — desktop NUCLEUS, composition library + template, TTT reference, Godot bridge, WS gateway (6 CI scripts deprecated → Rust subcommands Wave 54b, 4 NUCLEUS launchers deprecated → Rust nucleus_launcher Wave 51→54)
- **config/** — capability registry, launch profiles
- **niches/** — BYOB niche YAML for biomeOS scheduling
- **specs/** — architecture and evolution specs

## Key Modules

| Module | Purpose |
|--------|---------|
| `coordination` | Atomic composition definitions, health probing |
| `deploy` | Deploy graph parsing, structural + live validation |
| `ipc` | JSON-RPC 2.0 client, Neural API bridge, socket discovery, BTSP handshake |
| `launcher` | Binary discovery, process spawn, socket nucleation |
| `harness` | Spawn compositions, validate, RAII teardown |
| `bonding` | Multi-gate bonding models + STUN tiers + ionic RPC + content distribution |
| `btsp` | BTSP Phase 1–3: handshake, cipher negotiation, encrypted channels |
| `validation` | Experiment harness with structured output (`ValidationResult`, `ValidationSink`) |
| `validation/helpers` | Shared graph parsing, Dark Forest, capability cross-ref helpers |
| `validation/scenarios` | 57 absorbed experiment scenarios (10 tracks, 3 tiers: Rust/Live/Both) |
| `composition/neural_routing` | NeuralRoutingTable — Arc\<str\>-interned model of 490+ method routing surface |
| `composition/neural_dispatch` | NeuralDispatcher — dispatch surface with metrics + bridge outcome ingestion |
| `tolerances` | Named latency and throughput bounds |
| `niche` | Capability table, semantic mappings, registration |

## Boundaries

- **No barraCuda dependency** — coordination, not compute
- **No WGSL shaders** — GPU work stays in domain springs
- **No cross-spring imports** — discovers primals via IPC at runtime
- **Pure Rust** — zero C dependencies (ecoBin compliant)

## IPC

JSON-RPC 2.0 over Unix domain sockets (TCP fallback) and HTTP POST.
`tcp_rpc_multi_protocol` tries raw TCP then HTTP for transport-agnostic probing.
Method constants across 20+ primal domains. MCP tool surface for Squirrel AI.
Capability-based discovery via Neural API or 6-tier filesystem probing.

## Status

v0.9.31 Wave 67 (June 1, 2026) — 57 scenarios (10 tracks, 3 tiers),
490+ registered capability methods, 93 experiments (21 tracks), 113 graph TOMLs
(~80 deploy + 33 compositions), 44-cell deployment matrix. 835 tests passing
(804 lib + 10 integration + 4 binary + 17 doc). 13/13 BTSP convergence.
**Wave 67 vocabulary evolution**: `signal` → `composition` across all code, configs,
and docs. `Arc<str>` interning in `NeuralRoutingTable`. `FromStr` for `AtomicType`.
`FEDERATION_PORTS` centralized. `DispatchError::Ipc` → `Arc<IpcError>`.
`ordered_primals()` evolved to capability-based routing.
**Post-primordial / deep-debt**: `#![forbid(unsafe_code)]` on all 88 crate roots +
6 integration tests. Centralized `PORT_REGISTRY` in tolerances. Deny-by-default
`SecurityVerifier` (capability-based, no hardcoded primal names). `required_primals()`
deprecated → capability discovery. `membrane temporal.cascade` (Rust) replaces bash.
S1 TLS shadow PASSED (13 days). 20/20 primal services on golgiBody.

**Wave 54: Provenance-elevated checksums + sweetGrass braid integration.**
plasmidBin now produces a two-layer checksum model:
Layer 1 (unchanged): `checksums.toml` — `blake3(stripped_binary_bytes)` for download integrity.
Layer 2 (new): `provenance.toml` — composite fingerprint `blake3(content_hash || source_commit || build_timestamp || rustc_version || target)` that changes whenever ANY input changes, even if binary bytes are identical. sweetGrass `braid.create` emitted post-harvest (UDS when available, `.braid-pending.json` sidecar when offline). `plasmidbin verify-provenance` subcommand validates the full provenance chain. primalSpring consumer tooling rewired: `fetch_primals.sh` downloads provenance.toml, `s_deployment_pipeline` validates it at Stage 2.5, `validate_release.sh` checks Layer 2, `build_ecosystem_genomeBin.sh` uses provenance-aware Rust CLI, `desktop_nucleus.sh` validates provenance in its `validate` mode, `gen_seed_fingerprints.sh` enriches output with source_commit from provenance.toml. Prepares for cellMembrane Forgejo sovereignty (forge identity recorded in provenance, braids cross-referenceable across forges).

Wave 54 prep: 3 new absorbed scenarios (cephalization, tower-cns, kderm-boundary).
4 gates operational (eastGate, ironGate, southGate, biomeGate).
hotSpring NUCLEUS braid evolution absorbed: pseudoSpore v1.6.1 braid schema evolved
to NUCLEUS-ready (provenance trio slots, three-era deployment model, `biomeos nucleus ingest`
three-tier fallback, ownership boundary split per SPORE_OWNERSHIP_MATRIX). pseudoSpore 2.0
compChem target: publish on sporePrint, host via HPC LAN (biomeGate).

**Wave 58b: Deep debt sprint** — dispatch telemetry persistence (`flush_metrics_to_file()`,
`DispatchMetric` now `Serialize`), `NoopVerifier` → `PermissiveVerifier`, `blake3_hash` correctness
fix (was SHA-256), primal name constants in orchestrator/routing, `#[expect]` everywhere (zero
`#[allow]` remaining), env key centralization in `tolerances`, `entropy`, `orchestrator`.
Neural API evolution: Layer 4/5 data collection foundation — JSON-lines telemetry ready for
single-layer perceptron routing evolution. NC-1 COMPLETE (code), NC-3 CONSUMED, NC-4 ADVANCING,
NC-5 UNBLOCKED. biomeOS v3.84, bearDog/songbird env debt RESOLVED, squirrel IN PROGRESS.

**Waves 1-55b complete**. Key milestones: post-primordial (W49), covalent mesh (W48), Neural API deployment (W42), observatory
posture (W41), neural routing layer (W40), upstream absorption (W39), behavioral
convergence (W47), provenance-elevated checksums (W54), eukaryotic UniBin (W55b).
Wave-by-wave detail fossilized to `fossilRecord/`.

Zero DEBT markers, zero unsafe blocks, zero panics, zero clippy warnings (pedantic + nursery clean).
Security gate: MethodGate 13/13, BTSP AEAD 13/13, Edition 2024 13/13.
`Vec<&String>` → `Vec<&str>`. `JsonRpcError`/`UnknownPrimal` → `thiserror` derives.
`DeployError::Parse` now wraps `toml::de::Error` source for error chains. Deprecated
`family_seed_from_env()` → `mito_beacon_from_env().key_bytes()`. Hardcoded arch
triple → `current_target_triple()` compile-time dispatch.
**Zero-port Tower Atomic standard**: Tier 5 TCP discovery gated behind
`PRIMALSPRING_TCP_TIER5=1` (opt-in). UDS-only is the default. NestGate/Squirrel
port-swap bug fixed in scripts and `ports.env`.
See `fossilRecord/springs/primalSpring/docs_wave35_may2026/TEMPORAL_ECOSYSTEM_REVIEW_MAY12_2026.md` for the archived ecosystem audit.
**plasmidBin Wave 52 evolution**: `build --commit` for reproducible builds, `harvest --version-tag`
for manifest auto-update, `fetch` skips unshipped primals on `--all` (checksums-based), `gh` CLI
calls have 15s timeout (no more indefinite hangs). sourDough v0.3.0 first harvest. biomeOS UniBin
naming debt resolved (`biomeos-cli` → `biomeos` primal + `biome` CLI helper).
**Full NUCLEUS live on eastGate** — 13/13 primals running from plasmidBin deployment, 19/19
sockets alive, `plasmidbin doctor` 35/35 pass, `plasmidbin validate` 100/100 pass. Certify
175/193 (18 live-tier/BTSP config-dependent). K-Derm topology + bonding model standards
published to wateringHole. Gen4 K-Derm reconciliation bridges gram-negative → absolute layer
naming. Deploy graph validation: fragment-aware structural checks (profiles with `fragments`
exempt from duplicate-order rule). Provenance checksums regenerated (24 files, BLAKE3).
**plasmidBin decoupled** — all direct filesystem coupling to `../plasmidBin` removed (20 files).
Binary discovery standardized: `$ECOPRIMALS_PLASMID_BIN` → `$XDG_DATA_HOME/ecoPrimals/plasmidBin`.
`membrane plasmid.fetch` bootstraps binaries from sovereign mirrors. plasmidBin CI/CD
auto-harvests on primal push via `repository_dispatch`. GAP-27 (stale biomeOS) resolved.
**genomeBin v5.1** — 46 cross-architecture binaries across 6 target triples (Tier 1: 39/39),
`build_ecosystem_genomeBin.sh` replaces musl-only script with full 9-target matrix.

Live validation: **13/13 primals ALIVE** (plasmidBin v2026.05.26 on eastGate, all from plasmidBin deployment),
**guidestone Level 8** (BTSP alias routing + flex key fixes shipped Phase 58; 13 failures resolved),
**13/13 BTSP authenticated**, 9 cellular graphs BTSP-enforced, bonding model ALL PASS,
**19/19 exp094 composition parity**, **12/12 exp091 routing PASS**, **14/15 exp096 cross-arch**
(HSM cfg-gated). ludoSpring parity: exp068 **6/6**, exp067 **18/19**, exp072 **24/31**.
Full NUCLEUS validated across all 3 atomics (Tower + Node + Nest) + cross-atomic pipeline.
benchScale Docker lab: 13 binaries deployed and version-verified.
biomeOS substrate: Neural API liveness and graph executor validated via guidestone Layer 1.5.
**Neural API evolution** (biomeOS v3.55–v3.57): `signal.dispatch` as preferred atomic
signal dispatch path (composition collapse), `capability.call` signal-tier interception,
`primal.announce` atomic self-registration protocol, metrics tagging with signal
namespaces, Squirrel `composition_plan` mode for intent-to-composition decomposition. Tier 2
validation dynamically checks `signal.list` counts and `signal.schema` tool definitions.
See `wateringHole/PRIMAL_ANNOUNCE_PROTOCOL.md`.

Multi-tier genetics identity system: Mitochondrial (Mito-Beacon for discovery
and NAT negotiation), Nuclear (lineage DNA for non-fungible permissions with
generational mixing), Tags (open participation from plaintext seed heritage).
Three-tier BTSP: Phase 1 (FAMILY_SEED auth), Phase 2 (secure-by-default
cascade across 13/13 primals), Phase 3 (ChaCha20-Poly1305 encrypted channel —
13/13 FULL AEAD, converged May 2, 2026).
BtspEnforcer with explicit deny semantics per TrustModel.

Cross-architecture deployment: plasmidBin serves as genomeBin depot per ecoBin
Architecture Standard v3.0. Tier 1 MUST: x86_64 + aarch64 + armv7 musl-static.
Tier 2 SHOULD: Windows (barraCuda), Android (5 primals), macOS (8/14 check-pass).
Tier 3 NICE: RISC-V (all cargo-check pass, primalSpring itself linked).
14/15 cross-arch checks pass (beardog HSM cfg-gated in upstream Session 43).

Particle model adopted: Tower = electron, Node = proton, Nest = neutron,
NUCLEUS = atom. Layered validation: L0 (primal routing) → L1 (atomic) →
L2 (mixed atomics) → L3 (bonding patterns).

guideStone composition certification: `primalspring certify` (UniBin subcommand,
formerly standalone `primalspring_guidestone` — removed Wave 32) validates
NUCLEUS composition correctness across 9 layers (bare properties, seed provenance,
discovery, BTSP escalation, atomic health, capability parity, cross-atomic pipeline,
bonding, BTSP/crypto, cellular deployment). Layer 1.5 reports per-atomic security
posture (BTSP default on all tiers — cleartext is FAIL). biomeOS substrate
health (neural-api liveness + graph.list) validated as first-class check.
Domain guideStones (hotSpring, healthSpring, etc.) inherit this base certification
and only validate their own science. See [fossilRecord](https://github.com/ecoPrimals/fossilRecord) → `springs/primalSpring/wateringHole_phase56_apr2026/GUIDESTONE_COMPOSITION_STANDARD.md`.

BTSP convergence achieved: 13/13 capabilities BTSP-authenticated across all NUCLEUS
tiers. `upgrade_btsp_clients()` uses a two-pass strategy — cleartext probe first,
then BTSP-first for enforcing primals that reject cleartext. Published seed fingerprints
prove binary authenticity at Layer 0.5. All upstream primals now implement the 4-step
handshake server protocol. Key convergence fixes: Songbird `SecurityRpcClient::new_direct()`
(Wave 169), ToadStool post-handshake connection persistence, loamSpine `btsp.negotiate`
non-fatal fallback, petalTongue BearDog field alignment. `nucleus_launcher.sh` starts
biomeOS with `BIOMEOS_BTSP_ENFORCE=0` (cleartext bootstrap before Tower is alive).

Bonding models validated (structural): Covalent, Ionic, Metallic, Weak,
OrganoMetalSalt. Content distribution federation graph with 4 bonding tiers.
Ionic bond protocol RPC wiring for cross-family capability sharing.

## Shell Composition Library

`tools/nucleus_composition_lib.sh` — 41 reusable bash functions for interactive
NUCLEUS composition via IPC. Covers capability discovery, JSON-RPC transport,
petalTongue motor/scene/interaction/proprioception, rhizoCrypt DAG, loamSpine
ledger, sweetGrass braids, discrete sensor event isolation (click vs hover vs
keypress), and startup/teardown lifecycle. Springs source this library and
implement domain hooks. `tools/composition_template.sh` is the minimal starter,
`tools/ttt_composition.sh` is the reference implementation with branching game
states, and `tools/composition_nucleus.sh` is the parameterized NUCLEUS launcher.

## Gate Deployment

| Field | Value |
|-------|-------|
| **Gate** | eastGate (primary) — coordination hub |
| **Composition** | Full NUCLEUS (13/13 primals) |
| **NUCLEUS status** | operational — VALIDATED |
| **Songbird federation** | port 7700 |
| **LAN mesh** | ready — covalent linking via Songbird TCP |

primalSpring lives on eastGate as the ecosystem coordination spring.
Other gates evolve their own springs independently — see
`wateringHole/GATE_SPRING_OWNERSHIP.md` for the full routing table.
eastGate handles BTSP convergence, plasmidBin CI/CD, upstream primal
coordination, and ecosystem-wide rollout.

See `infra/whitePaper/gen3/about/HARDWARE.md` for full cluster topology
(11 towers, 4 HBM2 cards, 3 NPUs, ~1 TB aggregate RAM).

## Ecosystem Position

primalSpring validates biomeOS composition patterns so that other
springs and gen4 products can trust the coordination layer. It
contributes ValidationSink, deploy graphs, overlays, MCP tools,
bonding metadata, STUN tier definitions, pure-primal proto-nucleate
graphs, and the shell composition library back to the ecosystem.
Downstream tributaries reference `wateringHole/` for patterns and
standards. Per-spring exploration lanes guide convergent evolution:
ludoSpring (interaction fidelity), hotSpring (async compute/DAG
memoization), wetSpring (data visualization), neuralSpring (agentic
composition).

## Glacial Checkpoint — Current and Remaining (May 24, 2026)

### Completed
- **Waves 1–49 complete**: 13/13 primals stadial-gate absorbed, all upstream blockers shipped
- 490+ method registry (100% exercised), 57 scenarios, 110 deploy graphs
- 13/13 BTSP AEAD, 13/13 behavioral convergence, 12/12 primal.announce
- lithoSpore v1.0.0, all 8 springs at Wave 20+, 10/10 foundation threads active
- 45+ handoffs fossilized, zero local debt across all springs
- Wave 20-21 detail (per-spring PM items, garden absorption) fossilized to `fossilRecord/`

### Garden & Infrastructure Evolution (Waves 17-22, fossilized)

Garden evolution (lithoSpore v1.0.0, projectNUCLEUS V3, esotericWebb V8,
projectFOUNDATION), infrastructure review (repo membrane boundary, env audit),
wateringHole fossilization (numeric drift fixed, 18 gen4 docs organized),
and Wave 22 upstream primal evolution (13/13 stadial-gate absorbed, 4 new
methods registered) — all detail fossilized to `fossilRecord/`.

### Remaining (updated May 27, 2026)

**Resolved since last review:**
- ~~**Thread 4**~~ now active (expression + data sources in projectFOUNDATION)
- ~~**biomeOS primal.list implementation**~~ **RESOLVED** — shipped in biomeOS v3.65 (Wave 33)
- ~~**biomeOS nest.store signal dispatch**~~ **RESOLVED** — R5 promoted in biomeOS v3.63
- ~~**biomeOS spore.instantiate**~~ **DEFERRED-TO-STADIAL** — R7 in biomeOS v3.63
- ~~**Primal-blocked gaps**~~ (toadStool sandbox, barraCuda/coralReef, ionic bridge, sweetGrass TCP) — **RESOLVED** Wave 22 stadial gate + Wave 44 fixes
- ~~**sporePrint bash scripts**~~ — **RESOLVED** Wave 47: `render-notebooks` + `fetch-refresh` subcommands in `spore-validate`
- ~~**SP-2 deploy status**~~ — **RESOLVED** Wave 47: fields added to sporePrint `config.toml`

**Neural API (upstream — ALL RESOLVED):**
- ~~**songbird outbound announce**~~ — **RESOLVED** commit `4a8f4cdc`: outbound push + capability alignment shipped
- ~~**bearDog attestation field rename**~~ — **RESOLVED** commit `2a94f2d6d` (Wave 111): field renamed, verified by biomeOS v3.70

**Gate Deployment Intel (Wave 47 — ALL RESOLVED):**
- ~~loamSpine Tokio crash~~ — misdiagnosis: was `serve`→`server` CLI error, fixed in plasmidBin
- ~~toadStool `health.liveness`~~ — S272: always returns `"alive"` now
- ~~NestGate `--socket` flag~~ — S71: CLI flag added
- **DEPLOYMENT_BEHAVIOR_STANDARD published** → 13/13 primals compliant
- **4 gates operational**: eastGate, ironGate, southGate, biomeGate (strandGate hardware ready, not deployed)
- plasmidBin `plasmidbin start` (Rust CLI) normalized for all 13 primals

**Glacial shift horizons:**
- **Covalent bonding** — **Wave 51: `discovery.peers` VALIDATED LIVE**. Songbird mesh+registry merge, `SONGBIRD_PEERS` auto-seeding on boot, `mesh_seed` module (7+ tests). Dedicated primalSpring NUCLEUS (primalspring01 :7701) meshed with nucleus01 (:7700) on eastGate. **NEXT**: `s_covalent_mesh` + `s_cross_gate_capability_call` scenario validation → Plasmodium collective (3+ gates meshed).
- **Sovereignty cutover** — cellMembrane: Forgejo-primary, sovereign DNS (knot-dns), Forgejo Releases (S5). **Wave 54: provenance.toml + sweetGrass braids now provide the provenance chain for Forgejo migration.**
- **Neural API evolution** — adaptive routing weights → learned routing (biomeOS + primalSpring). biomeOS v3.75 mesh routing is a prerequisite step.
- ~~**Method coverage 80%**~~ — **RESOLVED** Wave 47: pushed to 460/460 (100%) via 3 new scenarios + coverage graph + script regex fix
- ~~**plasmidBin Rust elevation**~~ — **RESOLVED** Wave 47-51: `nucleus_launcher` Rust binary + full `plasmidbin` Rust CLI (build, harvest, fetch, validate, doctor, start, launch). All CI workflows migrated.
- ~~**Provenance-elevated checksums**~~ — **RESOLVED** Wave 54: two-layer checksum model (content hash + composite fingerprint), sweetGrass braid integration, `verify-provenance` subcommand, primalSpring consumer tooling rewired.
- **sporePrint living content (S6)** — NestGate `content.put` pipeline for dynamic site

**Downstream / cross-spring (async — springs/gardens own):**
- ludoSpring 6-method IPC expansion for esotericWebb
- esotericWebb provenance E2E on biomeOS (GAP-024)
- Foundation validate elevation to CompositionContext + Rust crates
- wetSpring ferment transcripts: Barrick 2009 SEALED (7/7), Tenaillon 2016 batch 0 COMPLETE
