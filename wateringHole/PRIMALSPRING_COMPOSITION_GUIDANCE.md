# primalSpring — Composition Guidance for Springs and Primals

**Date**: March 18, 2026
**From**: primalSpring v0.2.0
**License**: AGPL-3.0-or-later

---

## Purpose

This document describes how primalSpring's coordination validation capabilities
can be leveraged across composition layers. primalSpring is unique: it validates
the ecosystem itself — the coordination, composition, and emergent behavior that
biomeOS and the Neural API produce when primals work together.

Each primal in the ecosystem should write an equivalent document. No primal knows
about another at compile time — all composition happens at runtime via
capability-based discovery through biomeOS.

---

## Capabilities Table

| Capability | Description |
|------------|-------------|
| `coordination.validate_composition` | Validate atomic compositions (Tower/Node/Nest/FullNucleus) |
| `coordination.discovery_sweep` | Discover all primals in a composition |
| `coordination.neural_api_status` | Neural API health and reachability |
| `health.check` | Full self health status |
| `health.liveness` | Kubernetes-style liveness probe — am I alive? |
| `health.readiness` | Kubernetes-style readiness probe — ready to serve? |
| `capabilities.list` | List coordination capabilities |
| `lifecycle.status` | Primal status report (version, domain, status) |

---

## 1. Standalone — primalSpring Alone

primalSpring can run as a standalone primal without any other primals. In this
mode it validates coordination patterns, discovery sweep logic, and Neural
API health.

### What Standalone primalSpring Can Do

| Capability | Use Case |
|-------------|----------|
| **Discovery sweep** | Enumerate primals in a composition (returns empty when none running) |
| **Neural API status** | Check if biomeOS is reachable |
| **Health probes** | `health.liveness` always succeeds; `health.readiness` reports Neural API + discovered primals |
| **Validate coordination patterns** | Run experiments that probe primals — honest skip when primals absent |
| **4-format capability parsing** | Parse capability responses from any primal (Format A/B/C/D) |

### When to Use Standalone

- CI pipelines that validate primalSpring itself
- Discovery sweep testing without live primals
- Neural API connectivity checks
- Validation binary scaffolding (OrExit, ValidationSink, check_or_skip)

---

## 2. Tower — primalSpring + BearDog + Songbird

**Atomic**: Tower provides crypto identity and mesh discovery.

### What primalSpring Validates

| Validation | Experiment | Purpose |
|------------|------------|---------|
| **Tower Atomic bootstrap** | exp001 | BearDog socket created, Songbird mesh reachable |
| **Crypto capabilities** | exp001 | `crypto.sign`, `crypto.verify`, `crypto.keygen` respond |
| **Startup ordering** | exp006 | BearDog starts before primals that depend on it |
| **Graceful degradation** | exp005 | Removal of BearDog causes honest skip, not fake pass |

### Novel Patterns

primalSpring probes `health.liveness` and `health.readiness` on BearDog and
Songbird. Uses `resilient_call()` with `CircuitBreaker` and `RetryPolicy` for
transient failures. `DispatchOutcome::should_retry()` guides retry decisions.

---

## 3. Node — Tower + ToadStool

**Atomic**: Node adds GPU/CPU compute dispatch.

### What primalSpring Validates

| Validation | Experiment | Purpose |
|------------|------------|---------|
| **Compute pipeline coordination** | exp002 | ToadStool `compute.execute` capability routing |
| **Discovery within Node** | exp002 | Tower primals + ToadStool discovered via FAMILY_ID-aware sweep |
| **GPU dispatch** | exp050 | coralReef → toadStool → barraCuda compute triangle |

### Novel Patterns

primalSpring validates that ToadStool receives and routes compute requests
correctly. Uses `extract_rpc_result` and `extract_rpc_dispatch` for typed
JSON-RPC result extraction. `IpcError::is_method_not_found()` distinguishes
capability mismatches from connection failures.

---

## 4. Nest — Tower + NestGate

**Atomic**: Nest adds content-addressed storage.

### What primalSpring Validates

| Validation | Experiment | Purpose |
|------------|------------|---------|
| **Storage pipeline coordination** | exp003 | NestGate `storage.store` + `storage.retrieve` round-trip |
| **Discovery within Nest** | exp003 | Tower + NestGate discovered |
| **fieldMouse ingestion** | exp042 | fieldMouse frames → NestGate → sweetGrass |

### Novel Patterns

primalSpring validates storage pipeline composition. NestGate health probes
(`health.liveness`, `health.readiness`) confirm readiness before storage
operations. `safe_cast::micros_u64` for latency metrics in validation reports.

---

## 5. Full NUCLEUS — All 8 Primals

**Atomic**: BearDog, Songbird, ToadStool, NestGate, rhizoCrypt, loamSpine,
sweetGrass, Squirrel.

### What primalSpring Validates

| Validation | Experiment | Purpose |
|------------|------------|---------|
| **Full composition** | exp004 | All 8 primals start, discover peers, respond to capability calls |
| **Squirrel AI coordination** | exp044 | Multi-MCP routing, `ai.query`, `ai.analyze`, `ai.suggest` |
| **Neural API integration** | exp004 | Composition-driven discovery via Neural API |

### Novel Patterns

primalSpring's `validate_composition(AtomicType::FullNucleus)` probes every
required primal. `health.readiness` on primalSpring itself reports
`primals_discovered` and `primals_total` for orchestration visibility.

---

## 6. Provenance Trio — rhizoCrypt + loamSpine + sweetGrass (RootPulse)

**Composition**: primalSpring coordinates validation of the RootPulse workflow.

### What primalSpring Validates

| Validation | Experiment | Purpose |
|------------|------------|---------|
| **6-phase commit** | exp020 | health → dehydrate → sign → store → commit → attribute |
| **Branch + merge** | exp021 | Branch creation, merge commit, seal |
| **Merkle diff + federation** | exp022 | Cross-gate Merkle comparison |
| **Provenance for science** | exp041 | Any spring experiment → provenance trio |

### Novel Patterns

primalSpring validates that the provenance trio (rhizoCrypt, loamSpine,
sweetGrass) composes correctly for RootPulse. Uses `extract_capability_names`
to handle all 4 capability wire formats from each primal. `CircuitBreaker`
prevents cascading failures when one trio member is down.

---

## 7. Cross-Spring — airSpring → wetSpring → neuralSpring

**Composition**: Ecology pipeline across springs.

### What primalSpring Validates

| Validation | Experiment | Purpose |
|------------|------------|---------|
| **Cross-spring data flow** | exp024 | airSpring → wetSpring → neuralSpring ecology pipeline |
| **Capability routing** | exp024 | Data flows through biomeOS capability graph |
| **Provenance trio for science** | exp041 | Cross-spring experiments → provenance attribution |

### Novel Patterns

primalSpring validates that springs never import each other — coordination
happens via shared barraCuda primitives and biomeOS capability discovery.
`coordination.neural_api_status` confirms biomeOS is routing correctly.

---

## 8. Sovereign Compute Triangle — coralReef → toadStool → barraCuda

**Composition**: GPU compute pipeline.

### What primalSpring Validates

| Validation | Experiment | Purpose |
|------------|------------|---------|
| **Compute triangle** | exp050 | coralReef → toadStool → barraCuda pipeline |
| **Health probes** | exp050 | `health.liveness`, `health.readiness` on each primal |
| **Probe patterns** | exp050 | FAMILY_ID-aware discovery, Neural API health checks |

### Novel Patterns

primalSpring validates the sovereign compute stack: coralReef compiles WGSL,
toadStool dispatches, barraCuda executes. No vendor SDK lock-in. Uses
`ValidationSink` (StdoutSink/NullSink) for CI/headless validation runs.

---

## Discovery Protocol

All compositions above are **runtime-discovered**. primalSpring never imports
another primal. The discovery chain:

1. primalSpring starts → registers capabilities via `capabilities.list`
2. biomeOS discovers primalSpring → adds to niche capability registry
3. Any primal calls `coordination.validate_composition` → primalSpring probes
   required primals via `AtomicType::required_primals()`
4. primalSpring discovers primals via: `{PRIMAL}_SOCKET`, XDG convention,
   temp fallback, Neural API
5. No compile-time coupling. Primals come and go. Capabilities are the contract.

---

## For Other Primals Writing This Document

primalSpring's composition guidance differs from compute springs: primalSpring
**validates** compositions rather than **providing** compute. Focus on:

1. **What you validate** — which composition patterns you probe
2. **What each atomic unlocks** — Tower, Node, Nest, Full NUCLEUS
3. **What emergent systems you validate** — RootPulse, coralForge, cross-spring
4. **What probe patterns you use** — health.liveness, health.readiness,
   resilient_call, DispatchOutcome
5. **What capabilities you expose** — coordination, health, lifecycle

Remember: complexity through coordination, not coupling.
