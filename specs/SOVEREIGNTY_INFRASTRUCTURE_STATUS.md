# Sovereignty Infrastructure Status

**Date**: May 28, 2026 (Wave 58b)
**Status**: Active — tracking sovereignty cutover progress
**Audience**: All teams (cellMembrane, projectNUCLEUS, primalSpring)

---

## Overview

The sovereignty stack replaces commercial dependencies with
primal-composed alternatives following the Calibrate → Shadow → Cutover
protocol (see `SOVEREIGNTY_STANDARDS.md`). This document tracks the
current state of each layer, who owns it, and what remains.

---

## Layer Status

### S0: Hardware + Network (COMPLETE)

| Component | Status | Owner |
|-----------|--------|-------|
| Active gate hardware | LIVE | Family (physical) |
| Cat6 LAN backbone | LIVE | Family (physical) |
| Unmanaged switch | LIVE | Family (physical) |
| 10G elevation | PLANNED (not blocking) | Family (physical) |

The hardware backbone is deployed. Standard Cat6 LAN on an unmanaged
switch provides the covalent mesh. 10G is an elevation goal, not a
blocker — all current compositions operate within Cat6 bandwidth.

### S1: TLS + Certificate Management (IN PROGRESS)

| Component | Status | Owner | Notes |
|-----------|--------|-------|-------|
| Caddy TLS termination | LIVE on VPS | cellMembrane | Reverse proxy, auto-HTTPS |
| Let's Encrypt certificates | LIVE (external) | cellMembrane | Standard ACME flow |
| bearDog ACME daemon | **LIVE** (Wave 112) | bearDog team | Sovereign cert issuance/renewal |
| ACME → sovereign cutover | Shadow ready | cellMembrane | Shadow: run bearDog ACME in parallel with LE |

**Priority**: MEDIUM — bearDog ACME daemon is live since Wave 112. Shadow period
(run bearDog in parallel with Let's Encrypt) can begin. Cutover when confident.

### S2: DNS (IN PROGRESS)

| Component | Status | Owner | Notes |
|-----------|--------|-------|-------|
| Commercial DNS (registrar) | LIVE (external) | cellMembrane | Domain registration + NS records still at registrar |
| knot-dns on VPS | **DEPLOYED** | cellMembrane | Running on VPS, DNSSEC enabled (cellMembrane Wave 56) |
| Registrar NS cutover | **PENDING** | cellMembrane | NS records must point to sovereign knot-dns (NC-3.3) |

**Priority**: HIGH — knot-dns is deployed and serving; the remaining step is
registrar NS record cutover to make sovereign DNS authoritative.

### S3: Remote Access (COMPLETE)

| Component | Status | Owner | Notes |
|-----------|--------|-------|-------|
| RustDesk self-hosted | LIVE on VPS | cellMembrane | Sovereign alternative to TeamViewer/RDP |
| Songbird TURN relay | LIVE | Tower team | P2P relay for cross-gate communication |

Both remote access channels are sovereign. RustDesk handles human
operators; Songbird TURN handles machine-to-machine relay.

### S4: Source Code Hosting (COMPLETE)

| Component | Status | Owner | Notes |
|-----------|--------|-------|-------|
| Forgejo (self-hosted) | LIVE (32 repos, 3 orgs) | projectNUCLEUS | PRIMARY source of truth |
| GitHub mirror | LIVE (dual-push) | projectNUCLEUS | Public mirror, CI offload |

Forgejo is primary for **git hosting**. GitHub is an observed outer membrane mirror.
**CI/CD**: Still runs on GitHub Actions (74 workflows). CI inversion to Forgejo
Actions or self-hosted runner-first is a sovereignty gap — not blocking stadial
entry but required for full S4 satisfaction.

### S5: Binary Distribution (COMPLETE)

| Component | Status | Owner | Notes |
|-----------|--------|-------|-------|
| plasmidBin genomeBin v3 | LIVE | projectNUCLEUS | 14/14 primals, 46 release assets, provenance-elevated (Wave 54) |
| GitHub Releases | LIVE | projectNUCLEUS | Public distribution channel |
| Forgejo releases | PLANNED | projectNUCLEUS | Sovereign binary channel |

Functional but partially external (GitHub Releases). Adding Forgejo
releases as the sovereign channel is a natural extension.

### S6: Content Publishing (IN PROGRESS)

| Component | Status | Owner | Notes |
|-----------|--------|-------|-------|
| sporePrint (primals.eco) | LIVE (static Zola) | cellMembrane | Registry + public content |
| Living content evolution | PLANNED | cellMembrane | Dynamic content, primal-served pages |
| GitHub Pages fallback | LIVE | cellMembrane | Outer membrane fallback |

sporePrint works but is static. Evolution toward living content
(primal-served, dynamically updated) is cellMembrane's responsibility.

### S7: Deployment Orchestration (COMPLETE)

| Component | Status | Owner | Notes |
|-----------|--------|-------|-------|
| Neural API graph execution | LIVE (biomeOS v3.70) | biomeOS team | Graph-driven deployment |
| deploy_membrane.sh | LIVE | cellMembrane (operational) | VPS provisioning + channel deployment |
| deploy.sh / deploy_graph.sh | LIVE | projectNUCLEUS | Gate-level deployment |
| Composition graphs | LIVE (80 TOMLs) | primalSpring | Deployment blueprints |

The deployment stack is functional. Rust-native deployment (replacing
bash scripts) is an ongoing elevation in projectNUCLEUS.

### S8: Neural API (OPERATIONAL)

| Component | Status | Owner | Notes |
|-----------|--------|-------|-------|
| Capability routing | LIVE | biomeOS | 460 methods, 84+ domains |
| Adaptive routing weights | LIVE (redb-persistent) | biomeOS | EWMA latency, error rate, cost, circuit breakers |
| Utilization tracking | LIVE (Wave 42) | biomeOS | Hot/cold method analysis |
| Composition intelligence | LIVE | biomeOS | CompositionTier, CompositionPatternRegistry |
| Observatory feedback loop | LIVE (Wave 42) | primalSpring | Bridge outcome recording into dispatch metrics |
| `primal.announce` adoption | **12/12** | All primal teams | Wave 47: all compliant. biomeOS v3.75 mesh dispatch live. |

The Neural API infrastructure is mature. `primal.announce` adoption is
complete (12/12). Remaining evolution: learned routing weights (biomeOS)
and cross-gate mesh dispatch validation on the 4-gate covalent mesh.

---

## Ownership Mapping

See `wateringHole/TEAM_OWNERSHIP_MATRIX.md` for full details.

| Layer | Primary Owner | Secondary |
|-------|--------------|-----------|
| S0 Hardware | Family (physical) | — |
| S1 TLS | cellMembrane | bearDog (upstream ACME daemon) |
| S2 DNS | cellMembrane | — |
| S3 Remote Access | cellMembrane | Tower team (Songbird) |
| S4 Source Hosting | projectNUCLEUS | — |
| S5 Binary Dist | projectNUCLEUS | — |
| S6 Content | cellMembrane | — |
| S7 Deployment | projectNUCLEUS + cellMembrane | primalSpring (graphs) |
| S8 Neural API | biomeOS | primalSpring (observatory) |

---

## Remaining Work

### Immediate (cellMembrane team)

1. ~~Stand up knot-dns as secondary DNS on VPS~~ **DONE** (Wave 56) — cutover registrar NS records
2. Add Forgejo releases as sovereign binary channel
3. Begin sporePrint living content evolution

### Upstream Asks (async, not blocking)

1. ~~bearDog: ACME renewal daemon for sovereign TLS~~ **RESOLVED** (Wave 112)
2. biomeOS: `composition_model = "membrane"` in `composition.deploy(graph)`
3. ~~All primals: `primal.announce` with v3.68 schema~~ **RESOLVED** (12/12, Wave 47)
4. ~~plasmidBin: provenance-elevated checksums~~ **RESOLVED** (Wave 54) — `provenance.toml` + sweetGrass braids

### Multi-Gate Expansion (cellMembrane, later)

1. westGate VPS provisioning
2. northGate VPS provisioning
3. Cross-gate DNS (knot-dns zone replication)
4. Cross-gate TURN relay (Songbird mesh)

---

## Async Evolution Model

Each layer evolves independently:

```
S0 ████████████████████████ COMPLETE (hardware deployed)
S1 ████████████████████████ COMPLETE (BearDog ACME renewal daemon live, Wave 112)
S2 ████████████░░░░░░░░░░░░ DEPLOYED (knot-dns live on VPS; registrar NS cutover pending)
S3 ████████████████████████ COMPLETE (RustDesk + Songbird live)
S4 ████████████████████░░░░ GIT COMPLETE (Forgejo primary); CI still GitHub Actions
S5 ████████████████████░░░░ MOSTLY (Forgejo releases pending; provenance-elevated Wave 54)
S6 ████████████░░░░░░░░░░░░ IN PROGRESS (static Zola, living content pending)
S7 ████████████████████████ COMPLETE (Neural API + scripts + graphs)
S8 ████████████████████████ COMPLETE (12/12 primal.announce, biomeOS v3.75 mesh)
S9 ████████████████████████ NEW (Wave 54: provenance chain — content hash + composite fingerprint + braids)
```

No layer blocks another. Teams evolve orthogonally and synchronize at
wave review checkpoints.
