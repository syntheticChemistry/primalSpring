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
- **graphs/** — 62 deploy graph TOMLs (single-node + multi-node + spring validation + cross-spring + gen4 + bonding + chaos + science)
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
63 method constants across 18 domains. MCP tool surface for Squirrel AI.
Capability-based discovery via Neural API or 5-tier filesystem probing.

## Status

v0.8.0 Phase 23b — 87/87 gates, 399 tests, 72.5% library coverage,
67 experiments, 62 deploy graphs, 5 spring primal binaries in plasmidBin.
Phase 23b: biomeOS v2.78 rewire — all 4 blocking biomeOS debt items resolved
upstream. NeuralBridge gains graph_deploy/status/rollback + discover_domain.
20 new method constants (federation.*, discovery.*, topology.*, graph.deploy/
status/rollback/pipeline/continuous, lifecycle.start/stop/register,
capability.register/unregister/route, route.register). Two new validation
graphs: rollback_validate.toml and federation_manifest_validate.toml.
Deploy graph count 60→62, validation graphs 8→10.

## Ecosystem Position

primalSpring validates biomeOS composition patterns so that other
springs and gen4 products can trust the coordination layer. It
contributes ValidationSink, deploy graphs, overlays, MCP tools,
bonding metadata, and STUN tier definitions back to the ecosystem.
