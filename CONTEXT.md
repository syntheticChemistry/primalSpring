# SPDX-License-Identifier: AGPL-3.0-or-later

# primalSpring — Context

## What

primalSpring is the coordination and composition validation spring for
the ecoPrimals ecosystem. Its domain IS the ecosystem itself: atomic
composition (Tower, Node, Nest, Full NUCLEUS), graph execution patterns,
emergent systems, multi-node bonding, and cross-spring interaction.

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

- **ecoPrimal/** — library crate (`primalspring`) + 3 binaries
  (`primalspring_primal` server, `primalspring_guidestone` 9-layer composition
  certification (Layers 0–7 + 0.5 + 1.5), `validate_all` runner)
- **experiments/** — 84 validation binaries covering 18 tracks
- **graphs/** — 71 deploy graph TOMLs using fragment-first composition (6 fragments +
  9 profiles + 4 patterns + 5 spring validation + 2 cross-spring + 5 bonding +
  2 chaos + 2 spring deploy + 3 downstream + 5 multi-node + 13 root-level +
  1 federation + 12 cell graphs + 4 desktop app graphs)
- **docs/** — structured gap registry (`PRIMAL_GAPS.md`), wire contracts (discovery, storage, crypto), migration guides
- **tools/** — desktop NUCLEUS launcher, nucleus launcher, composition library + template, TTT reference implementation, Godot bridge, thin WS gateway, composition validator
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
| `validation` | Experiment harness with structured output |
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

v0.9.24 Phase 56c — 631 tests (585 passed + 46 ignored), 84 experiments (18 tracks), 71 deploy graphs (fragment-first composition). **Desktop NUCLEUS**: 12-primal live substrate with `desktop_nucleus.sh`, 11/12 healthy on heartbeat, 4 desktop app graphs (shell, system monitor, esotericWebb, The Rhizome). **Phase 56 Desktop Substrate**: 8 new experiments (exp099–exp106) including The Rhizome roguelike micro-game and micro-desktop shell composition. **Live gap harvest**: 23 gaps documented (LIVE_DEPLOYMENT_GAP_REPORT_PHASE56.md). **Provenance trio E2E fully resolved**: corrected parameter schemas for rhizoCrypt, loamSpine, sweetGrass. **Two-tier crypto architecture**: published seed fingerprints → HKDF base keys → family keys → per-atomic purpose keys. **All 12 primals resolved** — zero upstream asks remaining. Upstream absorbed: NestGate v0.4.70 S48 (encrypt-at-rest), biomeOS v3.30 (deep debt), Songbird W178 (anyhow), Squirrel AN (HTTP providers + discovery + crypto), BearDog W75 (purpose-key module extraction), barraCuda Sprint 47b (self-registration + role-based naming), sweetGrass v0.7.28 (braid + anchor signing delegation), loamSpine (Tower-signed ledger entries), ToadStool S205-S208 (encrypted dispatch + self-registration + deep debt). Full IPC method map (`docs/NUCLEUS_IPC_METHOD_MAP.md`). Crypto bootstrap (`tools/nucleus_crypto_bootstrap.sh`).
**plasmidBin decoupled** — all direct filesystem coupling to `../plasmidBin` removed (20 files).
Binary discovery standardized: `$ECOPRIMALS_PLASMID_BIN` → `$XDG_DATA_HOME/ecoPrimals/plasmidBin`.
`tools/fetch_primals.sh` bootstraps binaries from GitHub Releases. plasmidBin CI/CD
auto-harvests on primal push via `repository_dispatch`. GAP-27 (stale biomeOS) resolved.
**genomeBin v5.1** — 46 cross-architecture binaries across 6 target triples (Tier 1: 39/39),
`build_ecosystem_genomeBin.sh` replaces musl-only script with full 9-target matrix.

Live validation: **12/12 primals ALIVE**, **187/187 guidestone ALL PASS** (**13/13 BTSP
authenticated** — full NUCLEUS BTSP convergence achieved, 8 cellular graphs BTSP-enforced),
**19/19 exp094 composition parity**, **12/12 exp091 routing PASS**, **14/15 exp096 cross-arch**
(HSM cfg-gated). ludoSpring parity: exp068 **6/6**, exp067 **18/19**, exp072 **24/31**.
Full NUCLEUS validated across all 3 atomics (Tower + Node + Nest) + cross-atomic pipeline.
benchScale Docker lab: 12 binaries deployed and version-verified.
biomeOS substrate: Neural API liveness and graph executor validated via guidestone Layer 1.5.

Multi-tier genetics identity system: Mitochondrial (Mito-Beacon for discovery
and NAT negotiation), Nuclear (lineage DNA for non-fungible permissions with
generational mixing), Tags (open participation from plaintext seed heritage).
Three-tier BTSP: Phase 1 (FAMILY_SEED auth), Phase 2 (secure-by-default
cascade across 12/12 primals), Phase 3 (ChaCha20-Poly1305 encrypted channel).
BtspEnforcer with explicit deny semantics per TrustModel.

Cross-architecture deployment: plasmidBin serves as genomeBin depot per ecoBin
Architecture Standard v3.0. Tier 1 MUST: x86_64 + aarch64 + armv7 musl-static.
Tier 2 SHOULD: Windows (barraCuda), Android (5 primals), macOS (8/14 check-pass).
Tier 3 NICE: RISC-V (all cargo-check pass, primalSpring itself linked).
14/15 cross-arch checks pass (beardog HSM cfg-gated in upstream Session 43).

Particle model adopted: Tower = electron, Node = proton, Nest = neutron,
NUCLEUS = atom. Layered validation: L0 (primal routing) → L1 (atomic) →
L2 (mixed atomics) → L3 (bonding patterns).

guideStone composition certification: `primalspring_guidestone` binary validates
NUCLEUS composition correctness across 9 layers (bare properties, seed provenance,
discovery, BTSP escalation, atomic health, capability parity, cross-atomic pipeline,
bonding, BTSP/crypto, cellular deployment). Layer 1.5 reports per-atomic security
posture (BTSP default on all tiers — cleartext is FAIL). biomeOS substrate
health (neural-api liveness + graph.list) validated as first-class check.
Domain guideStones (hotSpring, healthSpring, etc.) inherit this base certification
and only validate their own science. See `wateringHole/GUIDESTONE_COMPOSITION_STANDARD.md`.

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
