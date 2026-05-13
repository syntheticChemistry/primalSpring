<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# primalSpring — Primal Proof IPC Mapping

**Date**: May 13, 2026
**Version**: v0.9.25 (Phase 32 — Tier 2 converged)
**Role**: Coordination spring — validates compositions, does not perform domain science

---

## Overview

primalSpring is unique among springs: it is the **coordination primal**,
not a domain science spring. Its IPC surface is the validation library
that all other springs consume via `ecoPrimal` crate imports and the
coordination JSON-RPC server (`primalspring serve`).

This document maps primalSpring's outbound IPC calls (to primals it
validates) and its inbound IPC surface (methods it serves to consumers).

---

## Outbound IPC — Primals Called During Validation

primalSpring calls primals via `CompositionContext::call()` during
composition validation. These are the methods exercised across 22
validation scenarios.

### Pre-flight (Tier 2 Science API)

| Method | Primal | Module | Purpose |
|--------|--------|--------|---------|
| `toadstool.validate` | toadStool | `s_compute_triangle.rs` | Workload pre-flight — can this hardware run the workload? |
| `toadstool.list_workloads` | toadStool | `s_compute_triangle.rs` | Enumerate available workloads |
| `barracuda.precision.route` | barraCuda | `s_compute_triangle.rs` | Optimal precision strategy for operation class |

### Health + Discovery

| Method | Primal | Module | Purpose |
|--------|--------|--------|---------|
| `health.liveness` | All 13 | `probes.rs`, scenarios | Kubernetes-style liveness |
| `health.readiness` | All 13 | `probes.rs` | Readiness probe |
| `health.check` | All 13 | `probes.rs` | Self health status |
| `capabilities.list` | All 13 | `capability.rs` | Niche capabilities + semantic mappings |
| `primal.capabilities` | All 13 | `capability.rs` | Alt format for capability listing |
| `rpc.methods` | All 13 | `capability.rs` | Method enumeration |

### Compute Trio

| Method | Primal | Module | Purpose |
|--------|--------|--------|---------|
| `stats.mean` | barraCuda | `s_compute_triangle.rs` | Math roundtrip validation |
| `shader.compile.wgsl` | coralReef | `s_compute_triangle.rs` | Sovereign shader compile |
| `compute.dispatch.submit` | toadStool | `s_compute_triangle.rs` | GPU dispatch |

### Nest + Provenance

| Method | Primal | Module | Purpose |
|--------|--------|--------|---------|
| `content.put` | nestGate | `s_nestgate_content_pipeline.rs` | Content storage |
| `content.get` | nestGate | `s_nestgate_content_pipeline.rs` | Content retrieval |
| `content.exists` | nestGate | `s_nestgate_content_pipeline.rs` | Content existence check |
| `dag.session.create` | rhizoCrypt | `s_domain_contract_sweep.rs` | DAG session |
| `dag.event.append` | rhizoCrypt | `s_domain_contract_sweep.rs` | DAG event |
| `ledger.entry.append` | loamSpine | `s_domain_contract_sweep.rs` | Ledger append |
| `braid.attribution.create` | sweetGrass | `s_domain_contract_sweep.rs` | Attribution braid |

### Tower

| Method | Primal | Module | Purpose |
|--------|--------|--------|---------|
| `crypto.seed_fingerprint` | bearDog | `s_domain_contract_sweep.rs` | Seed provenance |
| `crypto.sign` | bearDog | `s_domain_contract_sweep.rs` | Signing |
| `btsp.negotiate` | bearDog | `btsp_handshake.rs` | BTSP Phase 3 handshake |
| `mesh.peers` | songbird | `s_domain_contract_sweep.rs` | Peer discovery |
| `defense.audit` | skunkBat | `s_domain_contract_sweep.rs` | Audit forwarding |

---

## Inbound IPC — Methods Served by primalSpring

These methods are served via `primalspring serve` (JSON-RPC 2.0 over UDS).

| Method | Purpose |
|--------|---------|
| `health.check` | Self health status |
| `health.liveness` | Liveness probe |
| `health.readiness` | Readiness probe (Neural API + discovered primals) |
| `capabilities.list` | Niche capabilities + semantic mappings + cost estimates |
| `coordination.validate_composition` | Validate an atomic composition |
| `coordination.validate_composition_by_capability` | Capability-based validation |
| `coordination.discovery_sweep` | Enumerate capabilities in a composition |
| `coordination.probe_capability` | Probe a single capability provider |
| `coordination.neural_api_status` | Neural API reachability |
| `graph.list` | Structurally validate all deploy graphs |
| `graph.validate` | Validate a specific graph (structural or live) |
| `graph.waves` | Compute topological startup waves |
| `graph.capabilities` | Extract required capabilities from graph |
| `lifecycle.status` | Primal status report |
| `mcp.tools.list` | MCP tool definitions for Squirrel AI |

---

## Feature Flag

primalSpring does not use a `primal-proof` feature flag because it has no
library-linked domain math to gate. All primal interaction is already IPC
via `CompositionContext::call()`. The coordination library (`ecoPrimal`)
is consumed by sibling springs as a Rust crate dependency for its
`ValidationResult`, `CompositionContext`, and IPC client infrastructure.

---

## Fallback Behavior

When a primal is not running, `CompositionContext::call()` returns
`IpcError::ConnectionRefused` and the calling scenario emits
`check_skip()` — never fakes a pass. The `CircuitBreaker` prevents
repeated connection attempts to known-down primals.
