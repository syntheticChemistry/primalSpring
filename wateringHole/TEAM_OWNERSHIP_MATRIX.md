# Team Ownership Matrix

> **Version**: 1.0 (Wave 42 — May 2026)
> **Status**: Active — restructuring in effect

This document defines team ownership boundaries for the ecoPrimals
ecosystem. Each team owns specific repos, systems, and evolution tracks.

---

## cellMembrane Team (New — Spinning Up)

The cellMembrane team is a dedicated ops team being spun up to own all
membrane, interface, and public-facing infrastructure. They handle the
operational surface that makes sovereign deployment real.

### Ownership

| What | Repo / Path | Description |
|------|-------------|-------------|
| **cellMembrane** | `gardens/cellMembrane` | VPS state, runbooks, credentials, IP/key inventory |
| **sporePrint** | `gardens/sporePrint` (primals.eco) | Ecosystem registry + public content; evolving from static Zola toward living content |
| **Membrane channels** | `infra/plasmidBin/deploy_membrane.sh` | Operational execution of Signal/DNS, Relay, Surface/TLS channels |
| **Caddy TLS** | VPS deployments | TLS termination, certificate management, reverse proxy |
| **Sovereign DNS** | knot-dns on VPS | DNS resolution for `primals.eco` and sovereign subdomains |
| **RustDesk** | VPS deployments | Self-hosted remote access (sovereign shadow for commercial RDP/TeamViewer) |
| **Multi-gate expansion** | westGate, northGate provisioning | New gate deployments on additional VPS or hardware |

### Responsibilities

- VPS provisioning and lifecycle (DigitalOcean, future hardware)
- Membrane channel deployment and monitoring
- TLS certificate issuance and rotation
- Public DNS zone management
- Interface system uptime (RustDesk, sporePrint/Zola builds)
- Credential rotation and security hardening

### Handoff from projectNUCLEUS

The following systems transfer from projectNUCLEUS to cellMembrane:

1. VPS provisioning workflows
2. `deploy_membrane.sh` channel management
3. TLS shadow configuration (Caddy + Let's Encrypt)
4. IP/key inventory in `cellMembrane/` repo
5. sporePrint content publishing pipeline

projectNUCLEUS retains gate-level *validation* (Dark Forest checks,
membrane provenance) but cellMembrane team owns *operational execution*.

---

## projectNUCLEUS (Refocused)

projectNUCLEUS focuses on deployment pipelines, big compute orchestration,
and gate validation. It no longer directly manages VPS interface systems.

### Ownership

| What | Repo / Path | Description |
|------|-------------|-------------|
| **Deployment pipelines** | `gardens/projectNUCLEUS/deploy/` | `deploy.sh`, `deploy_graph.sh`, graph-driven deployment |
| **Gate inventory** | `gardens/projectNUCLEUS/gates/` | TOML-defined gate configurations (irongate, etc.) |
| **Big compute support** | Orchestration graphs | Science dispatch, toadStool fan_out, distributed compute |
| **Gate validation** | Dark Forest + sovereignty checks | 33 Dark Forest PASS, membrane provenance, sovereignty shadow |
| **Forgejo/GitHub mirror** | CI/CD infrastructure | Dual-push mirror management, release automation |
| **genomeBin production** | plasmidBin releases | Binary packaging and distribution for all 13 primals |

### Responsibilities

- Deployment pipeline evolution (Rust-native deployment replacing bash)
- Gate configuration management (TOML gate specs)
- Graph-driven deployment orchestration
- Cross-gate compute routing (toadStool fan_out through biomeOS)
- Binary release pipeline (genomeBin v3, checksums, attestations)
- Dark Forest and sovereignty validation suites

### Retained from Previous Scope

- Gate validation remains with projectNUCLEUS (they validate, cellMembrane deploys)
- Forgejo administration remains with projectNUCLEUS
- Binary distribution (plasmidBin) remains with projectNUCLEUS

---

## primalSpring (Unchanged)

primalSpring continues as the ecosystem's coordination science surface and
pattern observatory.

### Ownership

| What | Repo / Path | Description |
|------|-------------|-------------|
| **Capability registry** | `config/capability_registry.toml` | 457 methods, authoritative method surface |
| **Validation scenarios** | `ecoPrimal/src/validation/scenarios/` | 46 scenarios, 10 tracks, 3 tiers |
| **Deploy graphs** | `graphs/` | 80 deploy TOMLs + 14 atomic signal graphs |
| **Routing schema** | `config/routing_config_reference.toml` | Canonical membrane routing configuration |
| **Neural API observatory** | `ecoPrimal/src/composition/` + `ipc/neural_bridge.rs` | Studies biomeOS routing intelligence, pushes evolution |
| **wateringHole** | `wateringHole/` | Ecosystem standards, handoffs, team guidance |
| **Experiment tracks** | `experiments/` | 89 experiments, 20 tracks |

### Responsibilities

- Upstream pattern definition and dissemination
- Neural API observatory (bridge feedback loop, dispatch metrics)
- Ecosystem-wide validation (method coverage, composition patterns)
- Cross-spring parity assessment
- Standards evolution (BTSP, method gate, announce protocol)
- Handoff generation for upstream and downstream teams

---

## biomeOS (Upstream — Substrate Primal)

biomeOS is the ecosystem substrate — it orchestrates but is not
"owned" by any downstream team. primalSpring studies its behavior;
projectNUCLEUS deploys it; cellMembrane provides the infrastructure it
runs on.

### Key Interfaces for Downstream Teams

| System | Version | What It Provides |
|--------|---------|------------------|
| Neural API | v3.68 | Capability routing, graph execution, `primal.announce` |
| Adaptive routing | Layer 4 | `RoutingWeightTable` with EWMA, circuit breakers, cost hints |
| Composition intelligence | v3.68 | `CompositionTier`, `CompositionPatternRegistry`, `plan_tier()` |
| Utilization tracking | Wave 42 | Hot/cold method tracking via `neural_api.utilization` |
| Weight persistence | Wave 42 | redb-backed routing weights survive restarts |

---

## Cross-Team Coordination

### Signal Flow

```
primalSpring (standards)
    ↓ patterns + handoffs
upstream primals (implement primal.announce + methods)
    ↓ operational data
biomeOS (routes + learns)
    ↓ deployment graphs
projectNUCLEUS (deploys)
    ↓ gate configs
cellMembrane (operates infrastructure)
```

### Async Evolution Model

Teams evolve independently on orthogonal tracks:

- **cellMembrane**: Infrastructure ops, multi-gate expansion, DNS
- **projectNUCLEUS**: Deployment pipelines, compute orchestration, Rust-native deploy
- **primalSpring**: Validation, observatory, standards dissemination
- **Upstream primals**: `primal.announce` adoption, method compliance, niche science

Wave reviews (every ~20 waves) synchronize state and reassess priorities.
