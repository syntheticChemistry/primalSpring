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
- **experiments/** — 69 validation binaries covering 15 tracks
- **graphs/** — 92 deploy graph TOMLs (single-node + multi-node +
  21 spring validation + cross-spring + gen4 + bonding + chaos +
  science + 7 composition subsystems + 6 nucleated spring deploy)
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
| `ipc` | JSON-RPC 2.0 client, Neural API bridge, socket discovery |
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

v0.9.2 Phase 24 — 404 tests, 69 experiments, 92 deploy graphs.

Live validation: **43/44 (98%)**. C1 6/6, C3 8/8, C4 6/6, C5 5/5,
C6 5/5, C7 10/10. Only C2 `ai.query` fails (no local Ollama — code
is wired, SQ-02 resolved).

Primal gap registry: **8 open** (1 medium, 7 low), zero critical/high.
18 gaps resolved this cycle. Downstream gaps (gardens, springs) removed
from our scope — they own their own debt.

Rewired for latest primal evolution: biomeOS v2.81 (`topology.rescan`),
toadStool S171 (`ember.*`, `shader.compile` removed), petalTongue
(awareness init, server discovery), NestGate (crypto delegation),
Squirrel (local AI endpoint).

## Ecosystem Position

primalSpring validates biomeOS composition patterns so that other
springs and gen4 products can trust the coordination layer. It
contributes ValidationSink, deploy graphs, overlays, MCP tools,
bonding metadata, STUN tier definitions, and nucleated spring deploy
graphs back to the ecosystem. Downstream tributaries reference
`wateringHole/` for patterns and standards.
