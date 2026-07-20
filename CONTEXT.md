# SPDX-License-Identifier: AGPL-3.0-or-later

# primalSpring — Context

## Identity

**primalSpring is the composition experimentation laboratory.**

Like every other spring, primalSpring runs experiments. Where hotSpring
experiments with physics and wetSpring experiments with biology,
primalSpring experiments with primals — atomic compositions (Tower,
Node, Nest, Full NUCLEUS), graph execution patterns, emergent systems,
multi-node bonding, mesh behavior, and cross-gate interaction.

The compositions are the experiments. The deploy graphs are the
protocols. The bonding models are the hypotheses.

## Ownership Boundaries

### What primalSpring Owns

- **Composition validation library** — `CompositionContext`, routing,
  mesh topology, parity checks, liveness probing
- **Certification engine** — guideStone L0-L8 (170 scenarios, 12-track
  validation), `primalspring certify`
- **Bonding proofs** — multi-gate bonding models, STUN tiers, ionic
  RPC, content distribution federation
- **Graph authorship** — deploy graph structural validation, fragment
  composition, topological sorting
- **Gap discovery loop** — experiment failures drive upstream primal
  evolution via structured gap reports

### What primalSpring Does NOT Own

| Responsibility | Owner |
|----------------|-------|
| Deployment pipelines, VPS ops, binary harvesting | **cellMembrane** |
| Workload packaging, user-facing deployment polish | **projectNUCLEUS** |
| Team coordination, evolution blurbs, FRAGOs | **wateringHole overwatch** |
| Upstream primal code | **Individual primal teams** |

## Three-Tier Consumption Model

```
primalSpring ──validated patterns──▶ wateringHole/ ──▶ cellMembrane (deploys)
      │                                                     │
      │──composition library + certification──▶ projectNUCLEUS (packages)
      │                                                     │
      ◀──revalidation after downstream integration──────────┘
```

1. **primalSpring** publishes validated composition patterns, library
   APIs, and certification gates to `wateringHole/`
2. **cellMembrane** consumes those patterns for VPS deployment, binary
   evolution, and membrane ops
3. **projectNUCLEUS** consumes the composition library and certification
   engine as a polished, agnostic deployment product
4. **primalSpring** revalidates patterns after downstream integration,
   closing the feedback loop

## Role

primalSpring sits alongside other springs, downstream of primals.
It consumes primal capabilities via NUCLEUS compositions and validates
that those compositions produce correct emergent behavior.

Other springs consume primalSpring's patterns (composition types,
bonding models, IPC client) from `wateringHole/`. As they validate
their own domains, they expose new composition gaps that flow back
to primalSpring as experiment targets.

## Architecture

- **ecoPrimal/** — library crate (`primalspring`) + 2 binaries:
  `primalspring` (eukaryotic UniBin: certify + validate + status + checksums + registry + version),
  `nucleus_launcher` (Rust NUCLEUS lifecycle: start/stop/status with PID tracking + federation)
- **experiments/** — 96 validation binaries covering 21 tracks
- **graphs/** — ~102 deploy graph TOMLs + 33 atomic composition graphs using fragment-first
  composition (14 root + 9 profiles + 6 fragments + 9 spring validation + 5 multi-node +
  5 bonding + 4 patterns + 4 desktop + 3 downstream + 2 spring deploy + 2 chaos +
  2 cross-spring + 1 federation + 1 foundation + 12 cell graphs + `compositions/` tier)
- **docs/** — structured gap registry (`PRIMAL_GAPS.md`), wire contracts (discovery, storage, crypto), migration guides
- **tools/** — empty (all tooling absorbed into Rust subcommands, Wave 82c–120 deep debt sprints)
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
| `validation/scenarios` | 170 validation scenarios (12 tracks, 3 tiers: Rust/Live/Both) |
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

v0.9.41 Wave 150o (July 20, 2026) — 171 scenarios (12 tracks, 3 tiers),
492+ registered capability methods, 93 experiments (21 tracks), 102 graph TOMLs
(16 directories). 1206 lib tests (0 failures, 2 ignored). **5-gate active mesh.
USB enrollment ready. 43 repos audited (100k+ ecosystem tests). 55 depot binaries,
re-harvest pending (56 expected).** Config-driven topology
(`config/mesh_topology.toml` — SSOT for gate roster, zones, mesh addresses).
Evolution module: CytoplasmZone model (backbone/house2/garage/wan), three-hub
triangle topology, gate enrollment pipeline, convergence monitoring.
Zero clippy warnings, zero unsafe blocks, zero hardcoded primal assumptions in
production paths (mesh addresses, zones, ports all TOML-driven).
Role formalized: composition experimentation laboratory. Deployment ops handed
to cellMembrane, workload packaging to projectNUCLEUS.
**Wave 82c Deep Debt Sprint COMPLETE**: 3 fossil tool scripts deleted,
`validate_nucleus_deployment.sh` and `validate_release.sh` replaced by
`primalspring nucleus` and `primalspring release` Rust subcommands,
hardcoded routing patterns eliminated (TOML-driven), `Command::new("kill")`
replaced with `nix` crate safe signals, default auth mode evolved from
Permissive to Enforced (fail-closed). 5 lab scripts remain, 1 deprecated
tool (`ws_gateway.py`). Zero bash in production path.
13/13 BTSP convergence. All 4 upstream gaps RESOLVED.
**Wave 82 Deep Debt Sprint COMPLETE (16/16 tasks)**: Type-safe dispatch (CapabilityDomain
newtypes, table-driven handlers), TOML-driven config (ports, bind flags, VPS IPs, env keys),
real health checks (AtomicU64 drain, capability discovery readiness), tracing migration
(TracingSink), deprecation cleanup (required_primals → required_primal_slugs, LEGACY_PREFIXES
→ enum-derived), shell absorption (nucleus_composition_lib.sh + crypto_bootstrap.sh deprecated
with Rust replacement guides), pure-Rust crypto bootstrap (3-tier HMAC-SHA256), 36 new tests,
deploy pipeline hardened (multi-binary overrides, self_refresh.rs), ureq/rustls pinned
(ring wrapper-allowed for cross-membrane only). plasmidBin ownership: cellMembrane → projectNUCLEUS.
**Wave 79 UDS-only stadial gate**: `nucleus_launcher` defaults port-free (`--tcp`
opt-in), `discover_with_fallback()` TCP gated behind `tcp_tier5_enabled()`, all 6
legacy graphs migrated `uds_only`, deploy profiles suppress port env for UDS graphs,
`show_status` UDS-first probing, config reference endpoint UDS. 3 new stadial gate
validation phases. BD-TRUST-01 resolved (Songbird auto trust in mesh.init).
Songbird Wave 81 deep debt absorbed (hardcoded ports → constants, prod stubs hardened).
**Wave 77c deep debt**: `ring` C dependency eliminated (feature-gated behind `cross-membrane`),
6+ routing tables consolidated into single TOML-driven source, `nucleus_launcher` profile-driven
(no primal special-casing), 3 near-threshold files split, 33 certification unit tests added,
4 hardcoded values cleaned for gate-agnostic deployment, deprecated APIs cleaned up.
Large function split in `s_ecosystem_freshness`. fossilRecord scripts cleaned.
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

## Shell Composition Library (DEPRECATED — Wave 82)

`tools/nucleus_composition_lib.sh` — 41 reusable bash functions, now deprecated.
All business logic absorbed into Rust (ecoPrimal crate). See deprecation headers
in each script for Rust replacement mappings. The shell library remains as fossil
record for pattern reference only.

## Gate Deployment

| Field | Value |
|-------|-------|
| **Gate** | eastGate (primary) — experimentation |
| **Composition** | Full NUCLEUS (13/13 primals) |
| **NUCLEUS status** | operational — VALIDATED |
| **Songbird federation** | port 7700 |
| **LAN mesh** | ready — covalent linking via Songbird TCP |

primalSpring lives on eastGate. VPS deployment, binary refresh, and
membrane ops are owned by cellMembrane. See
`wateringHole/GATE_SPRING_OWNERSHIP.md` for the full routing table.

## Ecosystem Position

primalSpring contributes validated composition patterns, bonding models,
and IPC abstractions to the ecosystem via `wateringHole/`. Other springs
and gardens consume these to build their domain-specific compositions.

Contributions: ValidationSink, deploy graphs, overlays, MCP tools,
bonding metadata, STUN tier definitions, proto-nucleate graphs,
certification engine (guideStone).

Downstream experiment lanes that exercise compositions:
ludoSpring (interaction fidelity), hotSpring (async compute/DAG
memoization), wetSpring (data fetch + visualization), neuralSpring
(agentic composition), tideGlass (gen5 drug repurposing via NUCLEUS).

## Glacial Checkpoint — Current and Remaining (July 16, 2026 — Wave 145a)

### Completed
- **Waves 1–49 complete**: 13/13 primals stadial-gate absorbed, all upstream blockers shipped
- **Silicon Atheism Phase 1** (Wave 142a): 14/14 primals cross-compile for 4 architectures
- **Silicon Atheism Phase 2** (Wave 145a): 14/14 primals shipped platform-agnostic transport
- **Content-Addressed Convergence** (Wave 144a): 6/6 layers complete
- 492+ method registry (100% exercised), 170 scenarios, 102 deploy graphs, 1203 lib tests
- 13/13 BTSP AEAD, 13/13 behavioral convergence, 12/12 primal.announce
- 59 depot binaries (14 per architecture × 4 targets), all BLAKE3 + Ed25519 signed
- lithoSpore v1.0.0, all 8 springs at Wave 20+, 10/10 foundation threads active
- 45+ handoffs fossilized, zero local debt across all springs
- Wave 20-21 detail (per-spring PM items, garden absorption) fossilized to `fossilRecord/`

### Garden & Infrastructure Evolution (Waves 17-22, fossilized)

Garden evolution (lithoSpore v1.0.0, projectNUCLEUS V3, esotericWebb V8,
projectFOUNDATION), infrastructure review (repo membrane boundary, env audit),
wateringHole fossilization (numeric drift fixed, 18 gen4 docs organized),
and Wave 22 upstream primal evolution (13/13 stadial-gate absorbed, 4 new
methods registered) — all detail fossilized to `fossilRecord/`.

### Remaining (updated June 4, 2026)

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
