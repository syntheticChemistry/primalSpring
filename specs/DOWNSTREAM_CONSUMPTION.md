# SPDX-License-Identifier: AGPL-3.0-or-later

# projectNUCLEUS Downstream Consumption Surface

This document defines the stable interface that **projectNUCLEUS** depends on
from the `primalspring` library crate, graph artifacts, and configuration
schema. It is the contract between primalSpring (experimentation laboratory)
and projectNUCLEUS (polished agnostic deployment product).

## Library Crate (`primalspring`)

projectNUCLEUS depends on the `primalspring` crate as a Cargo dependency for
composition validation, coordination types, and certification.

### Stable Public API Surface

| Module | Consumed Items | Purpose |
|--------|---------------|---------|
| `composition` | `CompositionContext`, `validate_parity`, `validate_liveness` | Core composition validation engine |
| `coordination` | `AtomicType` (Tower, Node, Nest, FullNucleus) | Atomic composition type definitions |
| `validation` | `ValidationResult`, `ValidationSink`, `NdjsonSink` | Structured experiment output |
| `tolerances` | `*` (all named bounds) | Latency, throughput, and parity thresholds |
| `bonding` | `BondType`, `BondingPolicy`, `TrustModel` | Multi-gate bonding definitions |
| `certification` | `certify`, `CertificationResult` | L0-L8 guideStone certification engine |
| `deploy` | `load_graph`, `validate_structural`, `topological_waves` | Graph parsing and structural checks |
| `checksums` | `generate_manifest`, `verify_manifest` | BLAKE3 integrity verification |

### Not Consumed (primalSpring-internal)

| Module | Reason |
|--------|--------|
| `launcher` | projectNUCLEUS uses biomeOS `nucleus start`, not direct spawning |
| `harness` | Deprecated; local experimentation only |
| `ipc` | projectNUCLEUS wires its own IPC client to biomeOS |
| `niche` | primalSpring server self-knowledge; irrelevant to NUCLEUS |
| `emergent` | Experimentation-specific emergent system types |

## Graph Artifacts

### `graphs/cells/`

Cell deployment manifests consumed by `biomeos deploy`. Each cell defines a
minimal NUCLEUS composition for a specific workload. projectNUCLEUS inherits
these as deployment templates.

### `graphs/fragments/`

Atomic building blocks (tower, node, nest, nucleus, meta, provenance) used
with `resolve = true` for fragment-first composition. projectNUCLEUS extends
these fragments with its own workload-specific nodes.

### `graphs/compositions/`

Atomic composition graphs defining the Neural API composition layer. These
map high-level operations to primal capability graphs. projectNUCLEUS
consumes these as-is for biomeOS graph-backed execution.

## Configuration Schema

### `config/capability_registry.toml`

The method truth table. Maps 490+ capability methods to primal domains.
projectNUCLEUS reads this to understand what capabilities exist and which
primals provide them.

### `config/primal_launch_profiles.toml`

Profile schema defining per-primal CLI flags, env vars, and socket wiring.
projectNUCLEUS uses this schema for its own profile instances (different
values, same structure).

## Certification as Quality Gate

projectNUCLEUS CI should run primalSpring certification before release:

```bash
# L0 structural — no primals needed, fast
primalspring certify --bare

# Full L0-L8 — requires primals running (integration gate)
primalspring certify
```

A passing `certify --bare` is a **hard gate** for projectNUCLEUS releases.
Full L0-L8 certification is a **soft gate** (requires live infrastructure).

## What projectNUCLEUS Owns Independently

projectNUCLEUS is not a fork — it is a consumer. It owns:

- **Routing config instances** — specific routing tables for its deployment targets
- **Workload TOMLs** — application-specific deploy graphs built on primalSpring fragments
- **Deploy automation policy** — scheduling, rollback, canary logic
- **Notebook packaging** — sporePrint notebook integration and presentation
- **User-facing polish** — CLI UX, documentation, branding, error messages
- **Platform abstraction** — OS-specific paths, service management, desktop integration

## Version Compatibility

primalSpring follows semver for the library crate. projectNUCLEUS should
pin to `primalspring = "0.9"` and track minor releases. Breaking changes
(if any) will go through a deprecation cycle with `#[deprecated]` attributes
before removal.

## Feedback Loop

When projectNUCLEUS integration reveals composition gaps:

1. projectNUCLEUS files a gap in `docs/PRIMAL_GAPS.md` (or reports via blurb)
2. primalSpring adds a scenario to `validation/scenarios/` covering the gap
3. primalSpring evolves the library to pass the scenario
4. projectNUCLEUS pulls the updated crate and revalidates
