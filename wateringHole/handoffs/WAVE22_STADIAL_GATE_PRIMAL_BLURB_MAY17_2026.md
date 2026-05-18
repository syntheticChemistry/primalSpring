# Wave 22: Stadial Gate — All Upstream Primals

**Date**: May 17, 2026 (PM)
**From**: primalSpring (coordination spring)
**To**: All 13 primal teams + sourDough + bingoCube
**Registry**: 456 methods (canonical, `capability_registry.toml`)
**Gate**: Interstadial exit CLEARED (9.5/10). Stadial transition ready.

---

## What This Blurb Is

The interstadial exit gate is cleared. The stadial drives **real external
pressure**: Cloudflare baselines, Barrick Lab USB deployments, upstream crate
extraction, community framework parity. Every primal will need to operate under
that pressure — including pairing with downstream teams (springs, gardens,
lithoSpore, projectNUCLEUS).

This blurb is a **final debt sweep and stadial readiness checklist** for every
primal. Resolve all items here so we enter the stadial clean.

**Reference docs** (pull from `infra/wateringHole/`):
- `DEPLOYMENT_VALIDATION_STANDARD.md` — runtime contract (health triad, UDS, TCP)
- `CAPABILITY_WIRE_STANDARD.md` — discovery and envelope shapes
- `DARK_FOREST_GLACIAL_GATE_STANDARD.md` — 5-pillar security invariants
- `BTSP_PROTOCOL_STANDARD.md` — cipher negotiation, handshake
- `SEMANTIC_METHOD_NAMING_STANDARD.md` — `{domain}.{operation}[.{variant}]`
- `PLASMIDBIN_PUSH_AUTOMATION_STANDARD.md` — manifest, checksums, CI
- `INTERSTADIAL_EXIT_CRITERIA.md` — what stadial entry means
- `SOVEREIGNTY_STANDARDS.md` — deny.toml, Forgejo, dual-push
- `ECOSYSTEM_EVOLUTION_CYCLE.md` — sentinel-stadial model

---

## Universal Standards Checklist (all primals)

Every primal should self-audit against this list. If you're green on all items,
you're stadial-ready.

### Runtime
- [ ] Health triad: `health.liveness`, `health.readiness`, `health.check`
- [ ] UDS socket at `$XDG_RUNTIME_DIR/biomeos/<primal>.sock`
- [ ] TCP fallback respects `ports.env` assignment
- [ ] `server` subcommand with `--port` for JSON-RPC
- [ ] Standalone startup without `FAMILY_ID`/`NODE_ID`

### Discovery
- [ ] `capabilities.list` returns `{ "capabilities": [...], "count": N, "primal": "<name>" }`
- [ ] `identity.get` returns canonical identity response
- [ ] `primal.announce` implements self-registration
- [ ] All methods follow `{domain}.{operation}[.{variant}]` naming

### Security
- [ ] BTSP handshake mandatory when `FAMILY_ID` is set (non-"default")
- [ ] ChaCha20-Poly1305 + HKDF with `btsp-v1`
- [ ] `FAMILY_ID` + `BIOMEOS_INSECURE=1` = refuse to start
- [ ] `btsp.capabilities` registered in capability response
- [ ] Zero metadata leakage (stripped binary, no path/hostname/username)
- [ ] UDS-first default (TCP off unless explicitly enabled)
- [ ] `deny.toml` bans `ring`, `openssl`, `aws-lc-sys`

### Build / plasmidBin
- [ ] `manifest.toml` version matches actual released tag
- [ ] `checksums.toml` entry with BLAKE3 hashes for Tier 1 targets
- [ ] `seed_fingerprint` BLAKE3 hash present and correct
- [ ] `notify-plasmidbin.yml` workflow fires on release/tag push
- [ ] CI green on all targets
- [ ] musl-static clean: `cargo build --target x86_64-unknown-linux-musl`
- [ ] `edition = "2024"` in workspace Cargo.toml

### Documentation
- [ ] README.md version matches manifest
- [ ] CHANGELOG.md documents recent evolution
- [ ] CONTEXT.md or equivalent: current status, known gaps

### Composition Readiness (stadial-specific)
- [ ] Stability tiers annotated for all registered methods
- [ ] Degradation behavior documented (what happens when this primal is down)
- [ ] Downstream pairing documented (which springs/gardens depend on you)

---

## Low-Debt Group: bearDog, songbird, squirrel, petalTongue, loamSpine, barraCuda, skunkBat

These 7 primals are structurally clean — zero debt in the gap registry, versions
mostly aligned, CI green. Your work is: **run the checklist above, close any
remaining cosmetic items, and confirm stadial readiness**.

Per-primal items:

### bearDog (crypto spine) — v0.9.0 — 47 methods — CLEAN

- [ ] Audit `rustls` dependency — workspace deps include `ring` features via
      rustls. Confirm this doesn't violate deny.toml or is feature-gated off
      in the musl-static build
- [ ] Document ACME client integration path for stadial TLS shadow cutover
      (doesn't need to be built yet — just documented)
- [ ] **Stadial pairing**: cellMembrane/projectNUCLEUS (TLS termination),
      all primals (BTSP negotiation)
- **Composition gap**: None

### songbird (network) — v0.2.1 — 46 methods — CLEAN

- [ ] GitHub repo name `songBird` vs local dir `songbird` — verify
      `sources.toml` entry matches (cosmetic)
- [ ] Review `REMAINING_WORK.md` for stadial blockers
- [ ] **Composition gap**: Cross-gate dispatch via songBird (Phase 2, low priority)
- [ ] **Stadial pairing**: cellMembrane (TURN relay), all gates (mesh discovery)

### squirrel (AI/MCP) — v0.1.0 — 20 methods — CLEAN

- [ ] Consider version bump if significant evolution has occurred since initial tag
- [ ] 7,213 tests — verify CI runs them all (not just a subset)
- [ ] Confirm `ai.query` is registered in `capability_registry.toml`
- [ ] **Stadial pairing**: esotericWebb (agentic AI), projectFOUNDATION
      (AI-assisted analysis)
- **Composition gap**: None

### petalTongue (UI/representation) — v1.6.6 — 17 methods — CLEAN

- [ ] Multiple modes (ui, tui, web, headless, live, server, status) — verify
      each mode supports the health triad
- [ ] Document which platforms need optional native audio deps
- [ ] **Stadial pairing**: esotericWebb (game UI), lithoSpore (validation dashboard)
- **Composition gap**: None (PT-1 through PT-5 all resolved)

### loamSpine (permanent ledger) — v0.9.16 — 18 methods — CLEAN

- [ ] ecobin_grade `A+` — review gap to `A++`
- [ ] Review `DEPENDENCY_EVOLUTION.md` in specs for unresolved items
- [ ] Verify GAP-36 wire reconciliation fully closed (sweetGrass integration)
- [ ] **Composition gap**: Hex string acceptance (`loamSpine` + `rhizoCrypt`)
      — verify hex/base64 input flexibility for ledger operations
- [ ] **Stadial pairing**: sweetGrass (braid permanence), lithoSpore (ledger
      verification), projectFOUNDATION (immutable evidence)

### barraCuda (GPU math) — v0.4.0 — 38 methods — CLEAN

- [ ] `build_from_source = true` in manifest — verify intentional (WGSL
      compilation may require it)
- [ ] Confirm all 38 methods have stability tier annotations in registry
- [ ] **Composition gap**: GPU API alignment (`submit_and_map`) — align with
      wetSpring sovereign pipeline expectations
- [ ] **Stadial pairing**: hotSpring (3-GPU sovereign), airSpring (cross-tier
      parity), coralReef (compiler pipeline)

### skunkBat (defense) — v0.2.0-dev — 10 methods — CLEAN

- [ ] When releasing, update manifest `latest` to match tag (currently `-dev`)
- [ ] plasmidBin README inventory table still says `0.1.0` — update on release
- [ ] Review if `defense.*` method count (10) should expand for stadial audit
      requirements — VPS audit trail, cross-gate defense posture
- [ ] **Stadial pairing**: cellMembrane (VPS audit trail), lithoSpore (verification)
- **Composition gap**: None

---

## Focused: nestGate (storage) — v0.1.0 (manifest) / 4.7.0-dev (README)

**Debt level**: MEDIUM — critical version drift, stale doc references

### Action Items

- [ ] **CRITICAL: Version drift** — README says `4.7.0-dev`, manifest says
      `0.1.0`. These are likely different versioning schemes (internal iteration
      vs public API). **Pick one for manifest `latest`** and document the scheme.
      This is the largest version gap in the ecosystem
- [ ] Vendored `rustls-rustcrypto` under `vendor/` — document **why** and
      ensure deny.toml is consistent with vendored deps
- [ ] Verify `notify-plasmidbin.yml` fires correctly and the auto-harvest
      pipeline picks up the right version
- [ ] Review all 32 registered methods for stability tier annotations

### Composition Gaps (owned or shared)

- [ ] Data dependency declaration in workload TOMLs (shared with toadStool)

### Stadial Pairing

- lithoSpore (USB storage — nestGate is the persistence layer for USB artifacts)
- projectFOUNDATION (validation evidence persistence)

---

## Focused: sweetGrass (attribution/provenance) — v0.7.27 (manifest) / v0.7.35 (README)

**Debt level**: MEDIUM — manifest version drift, TCP/BTSP gap

### Action Items

- [ ] **Version drift**: manifest says `0.7.27`, README says `v0.7.35`. Run
      `notify-plasmidbin` or manually update manifest `latest` to `0.7.35`.
      Checksums need regeneration
- [ ] ecobin_grade `A+` — review gap to `A++`
- [ ] Verify `BEARDOG_INTEGRATION_GAP.md` in showcase is resolved
- [ ] Document provenance trio wire format as **stable** in README — it is the
      canonical provenance substrate for all downstream science

### Composition Gaps

- [ ] **TCP without BTSP** — sweetGrass currently allows TCP connections without
      BTSP handshake. This must be closed before stadial: TCP + no-BTSP = plaintext
      provenance data on the wire. Either enforce BTSP on TCP or document as
      UDS-only with TCP explicitly disabled by default

### Stadial Pairing

- lithoSpore (braid verification — sweetGrass braids are the primary verification artifact)
- wetSpring (ferment transcript braids — live science provenance)
- projectFOUNDATION (attribution chain for all thread evidence)

---

## Focused: rhizoCrypt (working memory / DAG) — v0.14.0

**Debt level**: MEDIUM — upstream ask from wetSpring + minor version drift

### Action Items

- [ ] **UPSTREAM ASK**: Implement `dag.partial_dehydrate` — Merkle root of
      sealed-only DAG nodes without closing the session. Enables cryptographically
      valid partial braids as clones complete in long-running computations.
      See `infra/wateringHole/handoffs/WETSPRING_UPSTREAM_RHIZOCRYPT_PARTIAL_DEHYDRATE_MAY17_2026.md`
      for proposed wire format:
      ```
      → { "method": "dag.partial_dehydrate", "params": { "session_id": "..." } }
      ← { "merkle_root": "<blake3>", "sealed_count": N, "open_count": M }
      ```
      **Degradation**: If unavailable, partial braids emit with empty Merkle root —
      per-clone BLAKE3 hashes remain verifiable. Science is not gated.
- [ ] Version: manifest `0.14.0`, README `0.14.0-dev` — drop `-dev` suffix on
      next tag
- [ ] ecobin_grade `A+` — review gap to `A++`

### Composition Gaps

- [ ] Hex string acceptance — verify hex/base64 input flexibility for DAG
      operations (shared with loamSpine)

### Stadial Pairing

- wetSpring (DAG checkpointing for 264-clone Tenaillon 2016 — **highest priority**)
- lithoSpore (provenance DAG — the DAG is the verification substrate)
- projectFOUNDATION (thread lineage — DAG sessions anchor thread evidence)

---

## Focused: biomeOS (orchestrator) — v0.1.0 (manifest) / v3.59 (README)

**Debt level**: MEDIUM-HIGH — upstream asks + version scheme alignment + orchestrator role

### Action Items

- [ ] **UPSTREAM ASK**: Register `braid.partial_update` and `braid.complete`
      as dispatchable signals in the signal dispatch table. When a DAG node
      seals, wetSpring fires `braid.partial_update`; RootPulse propagates the
      partial braid to lithoSpore. `braid.complete` fires when the full session
      closes.
      See `infra/wateringHole/handoffs/WETSPRING_UPSTREAM_BIOMEOS_BRAID_SIGNAL_MAY17_2026.md`
      for proposed wire format:
      ```
      → { "method": "signal.dispatch", "params": {
            "signal": "braid.partial_update",
            "payload": { "session_id": "...", "merkle_root": "...", "sealed_count": N }
          }}
      ```
      **Degradation**: If unavailable, braids are written to disk and pushed
      manually via git. RootPulse propagation is enrichment, not gate.
- [ ] **Version schemes**: manifest uses `0.1.0` (workspace meta), README uses
      `v3.59` (release train). **Pick one or document the dual scheme explicitly
      in README.** The orchestrator is the most visible primal — version confusion
      propagates to every downstream consumer
- [ ] Review `EVOLUTION_ROADMAP.md` in `specs/` for stadial-phase items
- [ ] `is_orchestrator = true` in manifest — unique flag. Verify biomeOS is the
      only primal with it (it should be)

### Composition Gaps

- [ ] Cross-gate dispatch via songBird (shared with songBird — Phase 2, lower priority)
- [ ] `nest.store` signal dispatch (lithoSpore ask R5 — MEDIUM priority)
- [ ] `spore.instantiate` atomic VM provisioning (lithoSpore ask R7 — LOW priority)

### Stadial Pairing

- **All springs** (graph execution — biomeOS is the composition engine)
- **All gardens** (composition deployment)
- **projectNUCLEUS** (sovereignty orchestration — biomeOS drives deploy graphs)
- **wetSpring** (braid signal propagation via RootPulse — urgent for Tenaillon 2016)

---

## Focused: toadStool (compute dispatch) — v0.1.0

**Debt level**: MEDIUM-HIGH — upstream ask + 3 composition gaps + version stale

### Action Items

- [ ] **UPSTREAM ASK**: Implement `compute.fan_out` — DAG-aware dispatch of
      clone-level work units to available compute substrate. Each completion
      triggers `record_step` + loamSpine aglet. Natural map for distributed
      LTEE processing (264 clones for Tenaillon 2016).
      See `infra/wateringHole/handoffs/WETSPRING_UPSTREAM_TOADSTOOL_COMPUTE_FANOUT_MAY17_2026.md`
      for proposed wire format:
      ```
      → { "method": "compute.fan_out", "params": {
            "work_units": [...],
            "substrate_filter": { "min_cores": 4, "gpu_required": false },
            "dag_session_id": "..."
          }}
      ← { "dispatch_id": "...", "assigned": [...], "queued": [...] }
      ```
      **Degradation**: If unavailable, sequential clone processing produces
      identical results. Performance enrichment only.
- [ ] **Version**: manifest `0.1.0` but significant evolution has occurred
      (S262+, Phase D factory, 22,900+ tests). **Tag `0.2.0`** to reflect
      current maturity
- [ ] Review `NEXT_STEPS.md` — it explicitly calls out remaining gaps

### Composition Gaps (3 owned)

- [ ] **Sandbox `working_dir` passthrough** — workloads need to specify working
      directory for sandboxed execution. Currently no TOML field for this
- [ ] **Env var expansion in workload TOMLs** — `$HOME` and similar vars not
      expanded in workload definitions. Document as pre-resolved or add expansion
- [ ] **Data dependency declaration in TOML** — workloads need to declare input
      data dependencies (shared with nestGate for data staging)

### Stadial Pairing

- wetSpring (264-clone parallelism for Tenaillon 2016 — **highest priority**)
- hotSpring (GPU dispatch — sovereign compute pipeline)
- coralReef (shader compilation dispatch)

---

## Focused: coralReef (shader compiler) — v0.1.0

**Debt level**: MEDIUM — version alignment, quality grade

### Action Items

- [ ] **Version alignment**: manifest `0.1.0` but README uses Phase 10 Sprint 12
      numbering. **Align on semver for manifest** or document the dual scheme.
      Consider tagging `0.2.0` given significant Phase 10 evolution
- [ ] `build_from_source = true` — verify intentional (WGSL compilation pipeline
      may genuinely require source builds)
- [ ] ecobin_grade `A+` (not `A++`) — review if the gap is addressable
- [ ] Smallest method count among compute primals (11 methods) — appropriate
      given compiler scope, but review if `shader.*` namespace needs expansion
      for stadial (e.g., `shader.validate`, `shader.optimize`)

### Composition Gaps

- None owned (hardware code extracted to toadStool per Wave 8)

### Stadial Pairing

- barraCuda (compilation target — coralReef compiles, barraCuda executes)
- hotSpring (VFIO dispatch — sovereign GPU pipeline)

---

## Non-Canonical: sourDough + bingoCube

These are not in the 13-primal core but appear in the plasmidBin manifest.

### sourDough (scaffolding) — v0.3.0 (manifest) / v0.1.0 (workspace)

- [ ] **Version drift**: manifest `0.3.0`, workspace Cargo.toml `0.1.0`. Reconcile
- [ ] **Identity**: Clarify whether sourDough is the 14th canonical primal or a
      utility/scaffolding tool. This affects whether it needs the full stadial
      checklist or just binary publication
- [ ] Decide: plasmidBin binary publication or source-only?

### bingoCube (demos) — v0.1.1

- [ ] No `ci.yml` — only `notify-sporeprint.yml`. Add CI if shipping binaries
- [ ] Clarify role — demo showcase vs production primal
- [ ] If demo-only: exempt from stadial checklist but keep manifest current

---

## Manifest Drift Summary (resolve before stadial)

| Primal | manifest.toml | Repo/README | Action |
|--------|--------------|-------------|--------|
| bearDog | 0.9.0 | 0.9.0 | OK — ring/rustls policy reconciled, ACME design doc written |
| songbird | 0.2.1 | 0.2.1-wave208 | OK — btsp.capabilities, primal.announce, aws-lc-sys banned |
| skunkBat | — | 0.2.0 | **RESOLVED** — released v0.2.0 (was -dev) |
| toadStool | — | 0.2.0 | **RESOLVED** — tagged v0.2.0 (S263) |
| barraCuda | 0.4.0 | 0.4.0 | OK — Sprint 70, 75 methods |
| coralReef | 0.2.0 | 0.2.0 | **RESOLVED** — aligned manifest + Cargo |
| nestGate | 0.1.0 | 4.7.0-dev | **Documented** — dual scheme intentional, unify on first public tag |
| rhizoCrypt | 0.14.0 | 0.14.0-dev | Minor — drop `-dev` on next tag |
| loamSpine | 0.9.16 | 0.9.16 | OK — stadial 23/23 PASS |
| sweetGrass | — | 0.7.36 | **RESOLVED** — v0.7.36 (was 0.7.27/0.7.35 drift) |
| biomeOS | 0.1.0 | v3.60 | **RESOLVED** — dual scheme documented in EVOLUTION_ROADMAP |
| squirrel | 0.1.0 | 0.1.0 | OK — 38 methods, stadial hardened |
| petalTongue | 1.6.6 | 1.6.6 | OK — checksums.toml created, 55 methods |
| sourDough | 0.3.0 | 0.1.0 (workspace) | **UNRESOLVED** — docs say 0.3.0, Cargo says 0.1.0 |

---

## Composition Gaps (primal-owned, open)

These gaps were exposed by downstream composition pressure (projectNUCLEUS,
lithoSpore, springs). They block or degrade real deployments.

| # | Gap | Owner | Priority |
|---|-----|-------|----------|
| ~~1~~ | ~~Sandbox `working_dir` passthrough~~ | toadStool | **RESOLVED** (S263) |
| ~~2~~ | ~~Env var expansion in workload TOMLs~~ | toadStool | **RESOLVED** (S263) |
| 3 | GPU API alignment (`submit_and_map`) | barraCuda / wetSpring | MEDIUM |
| ~~4~~ | ~~Data dependency declaration in TOML~~ | toadStool / nestGate | **RESOLVED** (S263) |
| ~~6~~ | ~~Hex string acceptance~~ | loamSpine / rhizoCrypt | **RESOLVED** (v0.9.16 / S69) |
| ~~7~~ | ~~sweetGrass TCP without BTSP~~ | sweetGrass | **RESOLVED** (v0.7.36) |
| 8 | Cross-gate dispatch | songBird / biomeOS | LOW (Phase 2) — songBird still evolving |

---

## Stadial Pairing Preview

When the stadial drives real external pressure, primals pair with downstream:

| Primal Cluster | Downstream Partner | Stadial Validation |
|---------------|-------------------|-------------------|
| Tower (bearDog + songbird + skunkBat) | cellMembrane, projectNUCLEUS | TLS cutover, NAT cutover, audit |
| Compute Trio (toadStool + barraCuda + coralReef) | hotSpring, wetSpring | GPU dispatch, sovereign pipelines, 264-clone fan-out |
| Provenance Trio (rhizoCrypt + loamSpine + sweetGrass) | lithoSpore, projectFOUNDATION | Braid verification, evidence chain, partial dehydrate |
| Meta (biomeOS + squirrel + petalTongue) | esotericWebb, all springs | Composition orchestration, AI, UI |
| Storage (nestGate) | lithoSpore, projectFOUNDATION | USB deployment, validation persistence |

---

## How to Use This Blurb

1. Run the universal standards checklist (top of this doc)
2. Fix your per-primal action items (low-debt group or focused section)
3. Close any composition gaps you own
4. Reconcile manifest drift (update `plasmidBin/manifest.toml` or tag a new release)
5. Run `notify-plasmidbin` workflow to trigger auto-harvest
6. Push changes and notify primalSpring for the next pull/audit cycle

This is a self-evolution wave. Each primal team owns their hardening.
primalSpring will pull and audit results in the next cycle.
