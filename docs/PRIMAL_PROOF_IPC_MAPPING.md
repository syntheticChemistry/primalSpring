<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# primalSpring — Primal Proof IPC Mapping

**Date**: May 13, 2026
**Version**: v0.9.25 (Phase 32 — Tier 2 converged, GAP-36 resolved, Phase D wired)
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
| `content.put` | nestGate | `s_nestgate_content_pipeline.rs` | Content-addressed storage (CAS) |
| `content.get` | nestGate | `s_nestgate_content_pipeline.rs` | Content retrieval by BLAKE3 hash |
| `content.exists` | nestGate | `s_nestgate_content_pipeline.rs` | Content existence check |
| `content.list` | nestGate | `s_nestgate_content_pipeline.rs` | Enumerate stored content hashes |
| `content.resolve` | nestGate | `s_nestgate_content_pipeline.rs` | Resolve content path + MIME type |
| `provenance.session.create` | rhizoCrypt | `s_domain_contract_sweep.rs` | DAG session (alias: `dag.session.create`) |
| `provenance.event.append` | rhizoCrypt | `s_domain_contract_sweep.rs` | DAG event (alias: `dag.event.append`) |
| `session.create` | loamSpine | `s_domain_contract_sweep.rs` | Ledger session on `ledger` capability |
| `session.state` | loamSpine | `s_domain_contract_sweep.rs` | Ledger session state query |

> **Wire name note (GAP-34/35/36 — all RESOLVED upstream May 13)**:
> `content.*` (nestGate CAS) and `storage.*` (generic blob API) are distinct
> domains — confirmed intentional by biomeOS v3.53 (GAP-34). `provenance.*` is
> the sweep's wire name; `dag.*` is canonical in rhizoCrypt — S68 `normalize_method`
> maps both (GAP-36). loamSpine v0.9.16 aliases `session.*` → `spine.*` (GAP-35/36).
> sweetGrass v0.7.35 aliases `braid.attribution.create` → `braid.create` (GAP-36).

### Tower + Defense + Orchestration

| Method | Primal | Module | Purpose |
|--------|--------|--------|---------|
| `secrets.store` | bearDog | `s_domain_contract_sweep.rs` | Key vault store |
| `secrets.retrieve` | bearDog | `s_domain_contract_sweep.rs` | Key vault retrieve |
| `defense.status` | skunkBat | `s_domain_contract_sweep.rs` | Defense policy state |
| `defense.events` | skunkBat | `s_domain_contract_sweep.rs` | Recent audit events |
| `discovery.discover` | songbird | `s_domain_contract_sweep.rs` | Capability-based endpoint discovery |
| `discovery.protocols` | songbird | `s_domain_contract_sweep.rs` | Supported protocol enumeration |
| `network.nat_type` | songbird | `s_domain_contract_sweep.rs` | NAT classification |
| `network.stun` | songbird | `s_domain_contract_sweep.rs` | STUN binding request |
| `bonding.status` | biomeOS | `s_domain_contract_sweep.rs` | Bond state query |

> **Wire name note (Tower live validation — ludoSpring V70, May 13)**:
> skunkBat's primary audit method is `security.audit_log` (not `defense.audit`).
> The `defense.status` and `defense.events` methods are separate admin endpoints.
> ludoSpring wires `security.audit_log` for runtime audit trail; primalSpring's
> sweep tests `defense.status`/`defense.events` for policy + event admin queries.
> Both domains are valid — `security.*` for data-plane audit, `defense.*` for
> control-plane policy. bearDog expects base64-encoded `message` param (not raw
> string `data`) for signing operations.

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
