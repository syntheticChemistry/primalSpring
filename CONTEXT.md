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

- **ecoPrimal/** — library crate (`primalspring`) + 2 binaries
  (`primalspring_primal` server, `validate_all` runner)
- **experiments/** — 72 validation binaries covering 15 tracks
- **graphs/** — 93 deploy graph TOMLs + 6 atomic-aligned fragments (9 profiles +
  4 patterns + 13 spring validation + cross-spring + gen4 + bonding + chaos +
  science + 5 spring deploy + 12 sketches + 5 downstream proto-nucleate +
  3 pipeline + 5 multi-node + 7 root-level)
- **docs/** — structured gap registry (`PRIMAL_GAPS.md`)
- **tools/** — nucleus launcher, thin WS gateway, composition validator
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
| `bonding` | Multi-gate bonding models + STUN tiers |
| `validation` | Experiment harness with structured output |
| `tolerances` | Named latency and throughput bounds |
| `niche` | Capability table, semantic mappings, registration |

## Boundaries

- **No barraCuda dependency** — coordination, not compute
- **No WGSL shaders** — GPU work stays in domain springs
- **No cross-spring imports** — discovers primals via IPC at runtime
- **Pure Rust** — zero C dependencies (ecoBin compliant)

## IPC

JSON-RPC 2.0 over Unix domain sockets (TCP fallback).
Method constants across 20+ primal domains. MCP tool surface for Squirrel AI.
Capability-based discovery via Neural API or 6-tier filesystem probing.

## Status

v0.9.14 Phase 41 — 443 tests, 73 experiments, 67 deploy graphs + 6 atomic-aligned fragments.

Live validation: **12/12 primals ALIVE**, **19/19 exp094 composition parity PASS**.
All LD-01 through LD-10 gaps RESOLVED. Full NUCLEUS composition validated
across all 3 atomics (Tower + Node + Nest) + cross-atomic pipeline.
13 FullNucleus capabilities. Pre-downstream gap resolution complete:
Songbird alias registration, validate_parity_vec non-numeric guard,
BtspEnforcer cipher-only enforcement documented, DeployGraph multi-node
schema unified, loamSpine health.check auto-param, rhizoCrypt event_type
reference documented.

Particle model adopted: Tower = electron, Node = proton, Nest = neutron,
NUCLEUS = atom. Layered validation: L0 (primal routing) → L1 (atomic) →
L2 (mixed atomics) → L3 (bonding patterns).

Bonding models validated (structural): Covalent, Ionic, Metallic, Weak,
OrganoMetalSalt. 13 live multi-node checks skipped (require benchScale
Docker labs with 2+ FAMILY_IDs).

## Ecosystem Position

primalSpring validates biomeOS composition patterns so that other
springs and gen4 products can trust the coordination layer. It
contributes ValidationSink, deploy graphs, overlays, MCP tools,
bonding metadata, STUN tier definitions, and nucleated spring deploy
graphs back to the ecosystem. Downstream tributaries reference
`wateringHole/` for patterns and standards.
