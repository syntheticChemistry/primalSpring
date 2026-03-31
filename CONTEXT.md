# SPDX-License-Identifier: AGPL-3.0-or-later

# primalSpring — Context

## What

primalSpring is the coordination and composition validation spring for
the ecoPrimals ecosystem. Its domain IS the ecosystem itself: atomic
composition (Tower, Node, Nest, Full NUCLEUS), graph execution patterns,
emergent systems, multi-node bonding, and cross-spring interaction.

## Role

Where other springs validate domain science (hotSpring → physics,
wetSpring → biology), primalSpring validates the coordination layer
that biomeOS and the Neural API produce when primals work together.
It has self-knowledge of coordination patterns and discovers other
primals at runtime via capability-based discovery.

## Architecture

- **ecoPrimal/** — library crate (`primalspring`) + 2 binaries
  (`primalspring_primal` server, `validate_all` runner)
- **experiments/** — 67 validation binaries covering 14 tracks
  (atomics, graphs, emergent, bonding, IPC, provenance, deployment, gen4 substrate, deployment matrix, substrate stress)
- **graphs/** — 83 deploy graph TOMLs (single-node + multi-node + 21 spring validation + cross-spring + gen4 + bonding + chaos + science + 7 composition subsystems + ludoSpring deploy + esotericWebb product)
- **docs/** — structured gap registry (`PRIMAL_GAPS.md`)
- **tools/** — nucleus launcher, thin WS gateway, composition validator
- **config/** — capability registry, launch profiles
- **niches/** — BYOB niche YAML for biomeOS scheduling
- **specs/** — architecture and evolution specs
- **wateringHole/** — handoffs and ecosystem documentation

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
79 method constants across 24 domains. MCP tool surface for Squirrel AI.
Capability-based discovery via Neural API or 5-tier filesystem probing.

## Status

v0.8.0f Phase 23f — 87/87 gates, 402 tests, 72.5% library coverage,
67 experiments, 89 deploy graphs (21 validation + 7 compositions + 6 spring deploy), 5 spring primal binaries.
Phase 23f: Composition decomposition — 7 subsystem compositions (C1-C7)
independently deployable and validated. docs/PRIMAL_GAPS.md gap registry
(22 gaps across 6 primals + cross-cutting). Gateway refactored to thin
WebSocket-to-IPC bridge (zero business logic). web/play.html reclassified
as composition monitor. Live validation: C1 6/6, C3 8/8, C4 6/6, C6 5/5
(C2/C5 expected gaps — Squirrel Ollama routing, NestGate persistence).
Phase 23e: esotericWebb as ecoPrimals product, exp088 UDS rewrite,
capability discovery fix, NeuralBridge health fallback.
Live validated: Tower 13/13, Neural API 12/12, Storytelling 16/16.
79 method constants across 24 domains.

## Ecosystem Position

primalSpring validates biomeOS composition patterns so that other
springs and gen4 products can trust the coordination layer. It
contributes ValidationSink, deploy graphs, overlays, MCP tools,
bonding metadata, and STUN tier definitions back to the ecosystem.
