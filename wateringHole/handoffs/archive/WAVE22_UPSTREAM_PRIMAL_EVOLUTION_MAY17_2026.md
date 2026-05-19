# Wave 22: Upstream Primal Evolution and plasmidBin Hardening

**Date**: May 17, 2026 PM
**From**: primalSpring (coordination spring)
**To**: All 13 primal teams + sourDough + bingoCube
**Status**: All primals LIVE, 456-method registry (452+4 from wetSpring asks), 13/13 BTSP AEAD, Dark Forest Gate PASS

---

## Context

The interstadial exit gate is cleared (9.5/10). Delta springs are at zero
Wave 20 debt. Downstream gardens have absorbed Wave 21 patterns. wetSpring
is executing the Barrick 2009 E2E study on southGate and has elevated its
sovereign pipeline to live primal composition via the provenance trio.

**This wave focuses upstream**: hardening all 13 primals to full deployment
standard, resolving version drift between repos and plasmidBin manifest,
and preparing each primal for the stadial shadow runs where they will
need to operate under real external pressure — including pairing with
downstream teams.

### wetSpring Upstream Asks (May 17 PM)

wetSpring's sovereign resequencing pipeline (Exp382) has been elevated from
hand-wired provenance to live trio composition. Three concrete upstream asks
have been filed as handoff docs in `infra/wateringHole/handoffs/`:

| Ask | Team | Method | Handoff |
|-----|------|--------|---------|
| Partial Merkle root of sealed DAG nodes without closing session | **rhizoCrypt** | `dag.partial_dehydrate` | `WETSPRING_UPSTREAM_RHIZOCRYPT_PARTIAL_DEHYDRATE_MAY17_2026.md` |
| Braid update/complete signals via biomeOS + RootPulse propagation | **biomeOS** | `braid.partial_update`, `braid.complete` | `WETSPRING_UPSTREAM_BIOMEOS_BRAID_SIGNAL_MAY17_2026.md` |
| DAG-aware work-unit dispatch for clone-level parallelism | **toadStool** | `compute.fan_out` | `WETSPRING_UPSTREAM_TOADSTOOL_COMPUTE_FANOUT_MAY17_2026.md` |

All three degrade gracefully — science is never gated behind provenance.
These methods have been registered in `capability_registry.toml` as
`stability = "evolving"`.

> **UPDATE (May 18, 2026)**: All three upstream asks have been **IMPLEMENTED**:
> - rhizoCrypt S69: `dag.partial_dehydrate` with `PartialDehydrateResponse` type
> - biomeOS v3.60: `braid.partial_update` + `braid.complete` signal graphs
> - toadStool S263: `compute.fan_out` with substrate filter + DAG session wiring

---

## Ecosystem-Wide Standards Checklist

Every primal should self-audit against this checklist. Items marked with
a star are new or recently formalized in Wave 20/21.

### Runtime Contract (DEPLOYMENT_VALIDATION_STANDARD)

- [ ] **Health triad**: implements `health.liveness`, `health.readiness`,
      `health.check` on newline JSON-RPC (exact method names, canonical shapes)
- [ ] **UDS socket**: serves at `$XDG_RUNTIME_DIR/biomeos/<primal>.sock`
- [ ] **TCP fallback**: respects `ports.env` assignment when TCP enabled
- [ ] **CLI convergence**: `server` subcommand with `--port` for JSON-RPC
- [ ] **Standalone startup**: boots without `FAMILY_ID`/`NODE_ID`

### Discovery and Wire Format (CAPABILITY_WIRE_STANDARD)

- [ ] **`capabilities.list`**: returns canonical envelope
      `{ "capabilities": [...], "count": N, "primal": "<name>" }` *
- [ ] **`identity.get`**: returns canonical identity response
- [ ] **`primal.announce`**: implements self-registration via
      `{ "primal": "<name>", "socket": "<path>", ... }` *
- [ ] **Semantic naming**: all methods follow `{domain}.{operation}[.{variant}]`

### Security (DARK_FOREST_GLACIAL_GATE + BTSP)

- [ ] **BTSP handshake**: mandatory when `FAMILY_ID` is set (non-"default")
- [ ] **Cipher negotiation**: ChaCha20-Poly1305, HKDF with `btsp-v1`
- [ ] **Refuse impossible state**: `FAMILY_ID` + `BIOMEOS_INSECURE=1` = refuse to start
- [ ] **`btsp.capabilities`**: registered in capability response
- [ ] **Zero metadata leakage**: stripped binary, no path/hostname/username leaks
- [ ] **UDS-first default**: TCP off unless explicitly enabled
- [ ] **deny.toml**: `ring` and `openssl` banned (sovereignty requirement) *

### Build and Distribution (plasmidBin)

- [ ] **manifest.toml version matches repo**: `latest` field = actual released version
- [ ] **checksums.toml entry**: BLAKE3 hashes for all Tier 1 targets
- [ ] **seed_fingerprint**: BLAKE3 hash present and correct
- [ ] **notify-plasmidbin.yml**: workflow fires on release/tag push
- [ ] **CI green**: `ci.yml` passes on all targets
- [ ] **musl-static**: builds clean with `cargo build --target x86_64-unknown-linux-musl`
- [ ] **Edition 2024**: `edition = "2024"` in workspace Cargo.toml

### Documentation

- [ ] **README.md**: version matches manifest, key capabilities listed
- [ ] **CHANGELOG.md**: recent evolution documented
- [ ] **CONTEXT.md or equivalent**: current status, known gaps, remaining work

---

## Per-Primal Guidance

### bearDog (crypto spine) — v0.9.0

**Status**: Production Ready. 47 registered methods. Heaviest upstream
primal by method count.

**Action items**:
- [ ] Audit `rustls` dependency — workspace deps include `ring` features
      via rustls. Verify this doesn't violate deny.toml or confirm it's
      gated behind a feature flag that's off in the musl-static build
- [ ] Version alignment: manifest says `0.9.0`, README says `0.9.0` — OK
- [ ] Stadial prep: BearDog will own TLS shadow cutover from Cloudflare.
      Ensure ACME client integration path is documented even if not built yet

**Downstream pairing**: cellMembrane/projectNUCLEUS (TLS termination),
all primals (BTSP negotiation)

---

### songbird (network) — v0.2.1

**Status**: Production Ready S+. 46 registered methods.

**Action items**:
- [ ] Version alignment: manifest `0.2.1`, README `0.2.1` — OK
- [ ] GitHub repo name `songBird` vs local dir `songbird` — cosmetic but
      verify `sources.toml` entry matches
- [ ] Review `REMAINING_WORK.md` for any stadial blockers
- [ ] Stadial prep: Songbird owns NAT traversal cutover from cloudflared

**Downstream pairing**: cellMembrane (TURN relay), all gates (mesh discovery)

---

### skunkBat (defense) — v0.2.0-dev

**Status**: Development. 10 registered methods. Newest primal.

**Action items**:
- [ ] Version: manifest says `0.2.0-dev` — when releasing, update manifest
      `latest` to match tag
- [ ] plasmidBin README inventory table still says `0.1.0` — update when
      next release lands
- [ ] Method count is low (10) — review if `defense.*` methods should
      expand for stadial audit requirements

**Downstream pairing**: cellMembrane (VPS audit trail), lithoSpore (verification)

---

### toadStool (compute dispatch) — v0.1.0

**Status**: Active evolution. 23 registered methods (22+1 from wetSpring ask).

**Action items**:
- [ ] **UPSTREAM ASK**: Implement `compute.fan_out` — DAG-aware dispatch of
      clone-level work units to available substrate. See
      `WETSPRING_UPSTREAM_TOADSTOOL_COMPUTE_FANOUT_MAY17_2026.md` for
      proposed wire format and substrate routing table
- [ ] Version: manifest `0.1.0` — consider tagging current state as
      `0.2.0` given S262 phase evolution
- [ ] Review `NEXT_STEPS.md` — explicitly calls out remaining gaps

**Downstream pairing**: wetSpring (264-clone parallelism for Tenaillon 2016),
hotSpring (GPU dispatch), coralReef (shader compilation)

---

### barraCuda (GPU math) — v0.4.0

**Status**: Stadial gate release. 38 registered methods. 826 WGSL shaders.

**Action items**:
- [ ] Version alignment: manifest `0.4.0`, README `0.4.0` — OK
- [ ] `build_from_source = true` in manifest — verify this is intentional
      (WGSL compilation may require it)
- [ ] Most mature compute primal — ensure all 38 methods have stability
      tier annotations in registry

**Downstream pairing**: hotSpring (3-GPU sovereign), airSpring (cross-tier
parity), coralReef (compiler pipeline)

---

### coralReef (shader compiler) — v0.1.0

**Status**: Phase 10 Sprint 12. 11 registered methods.

**Action items**:
- [ ] Version: manifest `0.1.0` but README uses phase/sprint numbering —
      align on semver for manifest or document the dual scheme
- [ ] `build_from_source = true` — same as barraCuda, verify intentional
- [ ] ecobin_grade `A+` (not `A++`) — review if gap is addressable
- [ ] Smallest method count among compute primals — appropriate given
      compiler scope, but review `shader.*` namespace coverage

**Downstream pairing**: barraCuda (compilation target), hotSpring (VFIO dispatch)

---

### nestGate (storage) — v0.1.0

**Status**: Strong CI. 32 registered methods.

**Action items**:
- [ ] **Critical version drift**: README says `4.7.0-dev`, manifest says
      `0.1.0`. Reconcile — these are likely different versioning schemes
      (internal iteration vs public API). Pick one for manifest `latest`
- [ ] `PLASMIDBIN_PUSH_AUTOMATION_STANDARD.md` still says nestGate
      "does not yet have its own repository" — this is stale, notify-plasmidbin
      workflow exists, repo exists. Update the standard
- [ ] Vendored `rustls-rustcrypto` under `vendor/` — document why and
      ensure deny.toml is consistent

**Downstream pairing**: lithoSpore (USB storage), projectFOUNDATION
(validation evidence persistence)

---

### rhizoCrypt (working memory / DAG) — v0.14.0

**Status**: Phase 2 development. 18 registered methods (17+1 from wetSpring ask).

**Action items**:
- [ ] **UPSTREAM ASK**: Implement `dag.partial_dehydrate` — Merkle root of
      sealed-only nodes without closing the session. See
      `WETSPRING_UPSTREAM_RHIZOCRYPT_PARTIAL_DEHYDRATE_MAY17_2026.md` for
      proposed wire format and expected response
- [ ] Version: manifest `0.14.0`, README `0.14.0-dev` — minor `-dev` suffix
      drift, reconcile on next tag
- [ ] ecobin_grade `A+` — review gap to `A++`

**Downstream pairing**: wetSpring (DAG checkpointing, partial braids),
lithoSpore (provenance DAG), projectFOUNDATION (thread lineage)

---

### loamSpine (permanent ledger) — v0.9.16

**Status**: Mature. 18 registered methods.

**Action items**:
- [ ] Version alignment: manifest `0.9.16`, README `0.9.16` — OK
- [ ] ecobin_grade `A+` — review gap to `A++`
- [ ] `DEPENDENCY_EVOLUTION.md` in specs — review for any unresolved items
- [ ] GAP-36 wire reconciliation — verify fully closed from sweetGrass
      integration

**Downstream pairing**: sweetGrass (braid permanence), lithoSpore (ledger
verification), projectFOUNDATION (immutable evidence)

---

### sweetGrass (attribution/provenance) — v0.7.27 (manifest) / v0.7.35 (README)

**Status**: Active. 25 registered methods.

**Action items**:
- [ ] **Version drift**: manifest says `0.7.27`, README says `v0.7.35`.
      Harvest may not have refreshed — run notify-plasmidbin or manually
      update manifest `latest` to `0.7.35`
- [ ] ecobin_grade `A+` — review gap to `A++`
- [ ] `BEARDOG_INTEGRATION_GAP.md` in showcase — verify resolved
- [ ] Provenance trio wire format is stable — document as stable in README

**Downstream pairing**: lithoSpore (braid verification), wetSpring (ferment
transcript braids), projectFOUNDATION (attribution chain)

---

### biomeOS (orchestrator) — v0.1.0 (manifest) / v3.61 (README)

**Status**: Production Ready. 40 registered methods (38+2 from wetSpring ask). **STADIAL ABSORBED.**

**Action items**:
- [ ] **UPSTREAM ASK**: Register `braid.partial_update` and `braid.complete`
      as dispatchable signals. When a DAG node seals, wetSpring fires
      `braid.partial_update`; RootPulse propagates braids to lithoSpore.
      See `WETSPRING_UPSTREAM_BIOMEOS_BRAID_SIGNAL_MAY17_2026.md`
- [ ] **Version schemes**: manifest uses `0.1.0` (workspace meta), README
      uses `v3.59` (release train). Document the dual scheme explicitly in
      README or align manifest to release train version
- [ ] Rich evolution docs in `specs/` — review `EVOLUTION_ROADMAP.md` for
      stadial-phase items
- [ ] `is_orchestrator = true` in manifest — unique flag, verify biomeOS
      is the only primal with it

**Downstream pairing**: all springs (graph execution), all gardens
(composition deployment), projectNUCLEUS (sovereignty orchestration),
wetSpring (braid signal propagation via RootPulse)

---

### squirrel (AI/MCP) — v0.1.0

**Status**: Active. 20 registered methods.

**Action items**:
- [ ] Version alignment: manifest `0.1.0` — consider version bump if
      significant evolution has occurred since initial tag
- [ ] 7,213 tests — impressive test coverage, verify CI runs them all
- [ ] `ai.query` signal dispatch recently added — ensure registered in
      capability_registry.toml

**Downstream pairing**: esotericWebb (agentic AI), projectFOUNDATION
(AI-assisted analysis)

---

### petalTongue (UI/representation) — v1.6.6

**Status**: Production Ready. 17 registered methods.

**Action items**:
- [ ] Version alignment: manifest `1.6.6`, workspace `1.6.6` — OK
- [ ] Multiple modes (ui, tui, web, headless, live, server, status) —
      verify each mode has health triad support
- [ ] Optional native audio deps — document which platforms need them

**Downstream pairing**: esotericWebb (game UI), lithoSpore (validation
dashboard)

---

### sourDough (scaffolding) — v0.3.0 (manifest) / v0.1.0 (workspace)

**Status**: Active. Not one of the 13 canonical but in manifest.

**Action items**:
- [ ] **Version drift**: manifest says `0.3.0`, workspace Cargo.toml says
      `0.1.0`. Reconcile
- [ ] Clarify canonical status — is sourDough the 14th primal or a utility?
- [ ] Review if it needs plasmidBin binary publication or stays source-only

---

### bingoCube (demos) — v0.1.1

**Status**: Standalone demos workspace. Not canonical.

**Action items**:
- [ ] No `ci.yml` — only `notify-sporeprint.yml`. Add CI if shipping binaries
- [ ] Clarify role — demos/showcase vs production primal

---

## Manifest Drift Summary

| Primal | manifest.toml | Repo/README | Action |
|--------|--------------|-------------|--------|
| bearDog | 0.9.0 | 0.9.0 | OK |
| songbird | 0.2.1 | 0.2.1 | OK |
| skunkBat | 0.2.0-dev | 0.2.0-dev | OK (will update on release) |
| toadStool | 0.1.0 | 0.1.0/S262 | Consider version bump |
| barraCuda | 0.4.0 | 0.4.0 | OK |
| coralReef | 0.1.0 | Phase 10 Sprint 12 | Align semver |
| nestGate | 0.1.0 | 4.7.0-dev | **Reconcile** — large gap |
| rhizoCrypt | 0.14.0 | 0.14.0-dev | Drop `-dev` on release |
| loamSpine | 0.9.16 | 0.9.16 | OK |
| sweetGrass | 0.7.27 | 0.7.35 | **Update manifest** |
| biomeOS | 0.1.0 | v3.59 | **Document dual scheme** |
| squirrel | 0.1.0 | 0.1.0 | OK (consider bump) |
| petalTongue | 1.6.6 | 1.6.6 | OK |
| sourDough | 0.3.0 | 0.1.0 | **Reconcile** |

---

## Stadial Pairing Preview

When the stadial drives real external pressure, primals will need to pair
with downstream teams:

| Primal Cluster | Downstream Partner | Stadial Validation |
|---------------|-------------------|-------------------|
| Tower (bearDog + songbird + skunkBat) | cellMembrane, projectNUCLEUS | TLS cutover, NAT cutover, audit |
| Compute Trio (toadStool + barraCuda + coralReef) | hotSpring, wetSpring | GPU dispatch, sovereign pipelines |
| Provenance Trio (rhizoCrypt + loamSpine + sweetGrass) | lithoSpore, projectFOUNDATION | Braid verification, evidence chain |
| Meta (biomeOS + squirrel + petalTongue) | esotericWebb, all springs | Composition orchestration, AI, UI |
| Storage (nestGate) | lithoSpore, projectFOUNDATION | USB deployment, validation persistence |

---

## How to Use This Blurb

1. Self-audit against the standards checklist above
2. Fix your per-primal action items
3. Reconcile any manifest drift (update `plasmidBin/manifest.toml` or tag a new release)
4. Run `notify-plasmidbin` workflow to trigger auto-harvest
5. Push changes and notify primalSpring for the next pull/audit cycle

This is a self-evolution wave. Each primal team owns their hardening.
primalSpring will pull and audit results in the next cycle.
