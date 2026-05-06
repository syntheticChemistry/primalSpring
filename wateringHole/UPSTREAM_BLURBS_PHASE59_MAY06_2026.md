# Upstream Primal Blurbs — Phase 59 (May 6, 2026)

**From**: primalSpring v0.9.24 (Phase 59 — Foundation Absorption + Discovery Escalation)

Pull primalSpring and infra/wateringHole before starting.

---

## What Changed This Cycle

1. **Foundation absorption**: primalSpring now validates foundation composition
   graphs (12-node NUCLEUS for scientific sediment pipeline) through Rust IPC.
   New graph fields: `GraphNode.fallback` (`"skip"` for optional nodes),
   `GraphMetadata.purpose` (`"validation"`, `"foundation"`).

2. **Discovery Escalation Hierarchy**: 5-tier (Songbird → biomeOS Neural API →
   UDS → socket registry → TCP probing). Your primal is discovered top-down;
   no action required unless you want to register additional discovery metadata.

3. **Capability taxonomy standardized**: `"provenance"` removed as primary
   capability — now a routing alias for `"dag"`. All graph TOMLs use
   `by_capability = "dag"` for rhizoCrypt. No wire change needed; the alias
   still routes correctly.

4. **fieldMouse reclassified**: deployment class (biomeOS chimera), not a primal.
   Removed from primal registries.

---

## Per-Primal Blurbs

### BearDog — CLEAN

No upstream debt. BTSP Phase 3 FULL AEAD shipped. Seed fingerprints, purpose keys,
`btsp.negotiate` all live. Family-scoped socket resolution works.

**One longer-term ask**: `crypto.sign_contract` for cross-tower ionic bond
negotiation is not yet wired as a JSON-RPC method. This blocks cross-family
contract signing for ionic compositions. LOW priority — no spring is blocked today.

---

### Songbird — CLEAN

No upstream debt. W189-192 absorbed (socket field on `ipc.resolve`, identity
verification, whitespace-tolerant UDS detection, sovereign-onion frame guard).
Discovery escalation hierarchy's Tier 1 relies on Songbird — working correctly.

---

### toadStool — CLEAN

No upstream debt. S222-S223 absorbed (deep debt, smart-refactor, sleep-speed).
`compute.submit` and `compute.dispatch` wired and validated.

---

### NestGate — CLEAN (minor long-term)

No blocking debt. S54 Wire Standard L3 absorbed. Storage roundtrip works.

**Long-term**: `storage.retrieve` for large/streaming tensors and cross-spring
persistent storage IPC remain partial. Neither blocks any current composition.

---

### barraCuda — CLEAN (surface expansion requested)

No blocking debt. Sprint 47b absorbed. All 39 JSON-RPC methods validated.

**Requested by downstream**: 18 additional methods from neuralSpring V133 gap
analysis (`linalg.eigh`, `stats.pearson`, `stats.chi_squared`, `stats.shannon`,
etc.). This blocks full Level 5 for neuralSpring only — other springs unblocked.
LOW priority; neuralSpring can submit PRs per method.

---

### coralReef — CLEAN (Phase D pending)

No blocking debt. Iter 92 Wire Standard L3 absorbed. WGSL compilation works.

**Long-term**: Phase D (draw + compute + framebuffer mixed pipeline) is the
longest pole for real-time rendering. CPU path works for moderate frame rates.
LOW priority — no composition blocked.

---

### rhizoCrypt — CLEAN

No upstream debt. `dag.session.create`, `dag.event.append`, `dag.session.complete`
all validated. Capability taxonomy standardized to `"dag"` — `"provenance"` remains
as routing alias, no code change needed.

---

### loamSpine — CLEAN

No upstream debt. `spine.create`, `entry.append` validated. Tower-signed ledger
entries work. BTSP Phase 3 FULL AEAD on wire.

---

### sweetGrass — CLEAN

No upstream debt. `braid.create` validated. Port 9850 is canonical BTSP TCP;
39085 is deprecated legacy HTTP — bind HTTP on 9850 when convenient.

---

### petalTongue — CLEAN

No upstream debt. Tier-1 Songbird registration, BufReader split-path fix,
whitespace-tolerant detection all absorbed. Visualization channels work.

---

### squirrel — CLEAN (cosmetic)

No blocking debt. AI composition validated.

**Cosmetic**: GAP-06 — `discovery.register` naming uses lowercase where other
primals use slug format. Not a blocker.

---

### biomeOS — CLEAN

No upstream debt. v3.43 absorbed. Neural API discovery schema (`primary_endpoint`
+ `primals[].name`) works. Graph executor validated through guidestone.

---

### skunkBat — CLEAN

No upstream debt. Port corrected to 9140. Defense/recon capabilities validated.
Meta-tier enhancer pattern established.

---

---

## PG-55 through PG-59 — ALL RESOLVED (May 6, 2026)

All security gaps from the projectNUCLEUS Phase 2a pen test are closed:

| PG | Resolution |
|----|-----------|
| **PG-55** | All 13 primals default to `127.0.0.1`. Songbird, ToadStool, skunkBat, biomeOS, petalTongue: `--bind`. sweetGrass: bare `--port` = localhost. biomeOS nucleus forwards `--bind`. |
| **PG-56** | NestGate BTSP method-level auth gating. 10-method exempt whitelist. |
| **PG-57** | skunkBat multi-dimensional baseline (rate + volume + port diversity). |
| **PG-58** | Songbird `--bind` for HTTP, `--listen` for IPC (documented). |
| **PG-59** | sweetGrass `--http-address` and `--port` formats documented. |

---

## Ecosystem State

- **13/13** primals BTSP Phase 3 FULL AEAD
- **Zero open security gaps** — PG-55 through PG-59 all RESOLVED
- **Zero runtime blockers**
- Discovery escalation hierarchy live (5 tiers)
- Foundation layer validated through IPC (exp107)
- Pen test baseline: all primals survived input fuzzing, no hidden admin methods
- All 13 primals default to `127.0.0.1` bind
- 85 experiments, 613 tests passing, 74 deploy graphs

**Next cycle focus**: Foundation sediment pipeline live validation,
spring-side library-to-binary rewiring, notebook elevation contract.
