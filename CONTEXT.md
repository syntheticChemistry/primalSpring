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
- **graphs/** — 59 deploy graph TOMLs (single-node + multi-node + spring validation + cross-spring + gen4 + bonding + chaos + science)
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
37 registered capabilities. MCP tool surface for Squirrel AI.
Capability-based discovery via Neural API or 5-tier filesystem probing.

## Status

v0.7.0 Phase 22 — 87/87 gates, 413 tests, 72.5% library coverage,
67 experiments, 59 deploy graphs, 5 spring primal binaries in plasmidBin.
Phase 22: E2E composition testing (Track 14) — exp085–exp088 (BearDog crypto lifecycle,
genetic identity, Neural API routing, storytelling composition), ipc::methods domain modules,
ipc::tcp neural API helpers, tolerances aliases, validate_composition.sh, ludoSpring game
method gap handoff; experiment count 63→67, test count 411→413.

## Ecosystem Position

primalSpring validates biomeOS composition patterns so that other
springs and gen4 products can trust the coordination layer. It
contributes ValidationSink, deploy graphs, overlays, MCP tools,
bonding metadata, and STUN tier definitions back to the ecosystem.
