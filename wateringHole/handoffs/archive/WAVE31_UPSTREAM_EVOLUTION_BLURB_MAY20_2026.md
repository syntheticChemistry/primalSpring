# Wave 31: Upstream Evolution Blurb — All Primals + Springs

**Date**: May 20, 2026
**From**: primalSpring (coordination spring)
**To**: All 13 primal teams + sourDough + bingoCube + 8 delta springs + 4 gardens
**Registry**: 456 methods (canonical, stable)
**Scenarios**: 45 (primalSpring) + 345 (wetSpring) — eukaryotic pattern proven at scale
**Gate**: Stadial. All primals zero debt. All waves 1-31 complete.

---

## What Happened Since the Last Blurb (Wave 22, May 17)

**Waves 23-31 compressed 9 waves in 3 days.** Here's the delta:

| Wave | What | Impact |
|------|------|--------|
| 23 | wetSpring Barrick 2009 E2E — 7/7 clones SEALED | First science dataset fully processed through sovereign pipeline |
| 24 | Shadow runs S1-S3 LIVE | TLS termination 10ms sovereign vs 120ms Cloudflare |
| 28 | sporePrint **15/15** — all primals have Zola validation summaries | primals.eco content surface complete |
| 29 | cellMembrane CM-1/CM-2/CM-3/CM-4 all RESOLVED | VPS membrane composition validated |
| 30 | SP-1 auto-merge, CM-3 cross-gate scenario (45th) | sporePrint CI automates, capability.call tested |
| 31 | Pattern absorption: `validation::numeric`, migration guide | Eukaryotic migration path formalized for all springs |

**plasmidBin validate v0.2.0**: Full serde-typed refactor — edition 2024, 21 tests,
cross-validation drift now FAIL. If your checksums or sources are missing from
manifest, CI will catch it.

**wetSpring V182 UniBin**: 349 prokaryotic binaries → 1 binary, 345 scenarios.
Build time 25min → 1m44s. This is the reference implementation for the eukaryotic
migration pattern.

---

## What's Good — No Action Required

If your primal is in this table and all columns are green, you are **stadial-current**.
No action needed from you right now.

| Primal | Tests | BTSP P3 | sporePrint | Zero Debt | Status |
|--------|------:|:-------:|:----------:|:---------:|--------|
| bearDog | 14,784+ | FULL | Complete | Clean | Stadial-current |
| songbird | 7,178+ | FULL | Complete | Clean | Stadial-current |
| skunkBat | 363+ | FULL | Complete | Clean | Stadial-current |
| toadStool | 23,000+ | FULL | Complete | Clean | Stadial-current |
| barraCuda | 4,422+ | FULL | Complete | Clean | Stadial-current |
| coralReef | 4,506+ | FULL | Complete | Clean | Stadial-current |
| nestGate | 12,393+ | FULL | Complete | Clean | Stadial-current |
| rhizoCrypt | 1,642+ | FULL | Complete | Clean | Stadial-current |
| loamSpine | 1,523+ | FULL | Complete | Clean | Stadial-current |
| sweetGrass | 1,553 | FULL | Complete | Clean | Stadial-current |
| biomeOS | 7,924+ | FULL | Complete | Clean | Stadial-current |
| squirrel | 7,089+ | FULL | Complete | Clean | Stadial-current |
| petalTongue | 6,297+ | FULL | Complete | Clean | Stadial-current |
| sourDough | 281 | N/A | Complete | Clean | CLI meta-primal |
| bingoCube | 73 | N/A | Complete | Clean | Library |

**13/13 primals at zero debt. 15/15 sporePrint complete. All composition gaps RESOLVED.**

---

## Remaining Horizons — Per-Team Action Items

These are the open evolution targets. Work at your own cadence — push when ready,
and primalSpring will ingest downstream.

### bearDog

| Item | Priority | What |
|------|----------|------|
| `content.*` scope expansion | MEDIUM | `content.put` in MethodGate session token scope — currently blocked. Unblocks SP-4 sovereign publish to NestGate. |
| ACME Phase 2 production | LOW | Auto-cert for membrane.primals.eco (design doc shipped, implementation pending) |

### songbird

| Item | Priority | What |
|------|----------|------|
| TURN relay production | LOW | `songbird-turn-client` shipped — integration with mesh relay for multi-gate NAT traversal pending operational testing |

### biomeOS

| Item | Priority | What |
|------|----------|------|
| `primal.list` implementation | ~~MEDIUM~~ **RESOLVED** | Shipped in biomeOS v3.65 (May 20). Schema aligned with primalSpring Wave 20 spec. |
| `nest.store` signal dispatch | LOW | lithoSpore R5 — ferment transcript handoff collapse via signal routing |
| `spore.instantiate` | LOW | lithoSpore R7 — atomic VM provisioning for guideStones |

### toadStool

| Item | Priority | What |
|------|----------|------|
| Sandbox `working_dir` production | LOW | S263 workload spec shipped. Operational deployment pending. |

### loamSpine

| Item | Priority | What |
|------|----------|------|
| Public chain anchor | LOW | Chain anchoring spec (Bitcoin/Ethereum/RFC 3161) shipped. Implementation pending. Unblocks WS-3 provenance verification outside trust boundary. |

### sourDough

| Item | Priority | What |
|------|----------|------|
| Version drift | LOW | Cargo.toml says 0.1.0, docs/CHANGELOG say 0.3.0. Align when releasing. |

---

## Spring Evolution Horizons

### wetSpring (V182 — evolution leader)

| Item | Priority | What |
|------|----------|------|
| WS-11 re-measurement | HIGH | v3 calibration deployed. Re-run Barrick 2009 with v3 to measure updated parity. |
| Tenaillon 2016 full run | HIGH | 264 genomes, ~200 GB. Batch 0 done (5/5 clones). Full run is the next major science milestone. |
| WS-9 L3 parity | MEDIUM | L1/L2 done. L3 (local vs IPC-composed) pending live trio. |

### hotSpring / healthSpring / groundSpring / airSpring

| Item | Priority | What |
|------|----------|------|
| Eukaryotic migration | MEDIUM | `EUKARYOTIC_VALIDATION_MIGRATION.md` published in primalSpring. Use `primalspring::validation::numeric::NumericValidator` + `bridge_into` for prokaryotic experiment absorption. wetSpring V182 is the reference implementation. |

### projectFOUNDATION

| Item | Priority | What |
|------|----------|------|
| FN-1 BLAKE3 backfill | MEDIUM | 10/25 sources hashed. 15 need manual fetch. |
| Foundation validate elevation | LOW | Migrate from bash to `CompositionContext` + Rust crates |

### projectNUCLEUS

| Item | Priority | What |
|------|----------|------|
| Forgejo Actions CI | MEDIUM | Porting GitHub Actions → Forgejo for sovereignty |
| cellMembrane Nest expansion | LOW | Tower → Nest Atomic on VPS (CM-1/2/3/4 RESOLVED, infra deployment pending) |

---

## What primalSpring Is Doing While You Evolve

We're solving local debt:
- Deprecated API sunset (`probe_primal`, `AtomicHarness`, `RunningAtomic`)
- Method coverage re-baseline (456-method inverse drift)
- Legacy binary removal (`validate_all`, `primalspring_guidestone`)
- SP-2 deploy status telemetry

When you push, we'll pull, ingest, validate, and update tracking docs.

---

## How to Hand Back

1. Push to your repo's `main`
2. `notify-sporeprint.yml` fires automatically (sporePrint SP-1 auto-merges validated content)
3. If you have upstream asks or gap reports, drop a `wateringHole/handoffs/` markdown
4. primalSpring will pull, run scenarios, and file audit responses

**No blockers from primalSpring. All infrastructure is green. Evolve at your cadence.**
