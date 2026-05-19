# Upstream Gaps — River Delta (Springs) (May 19, 2026)

**Date:** May 19, 2026
**From:** primalSpring (coordination spring)
**To:** wetSpring, all delta springs, RootPulse team
**Priority:** Sweep — all remaining open spring-level gaps
**License:** AGPL-3.0-or-later

---

## Context

wetSpring's Barrick 2009 is SEALED — first E2E ecosystem proof complete.
Tenaillon 2016 (264 clones, 590 GB) is queued. This blurb collects every
remaining open gap at the spring / river delta layer.

wetSpring requested single-wave ingestion rather than piecemeal.

---

## wetSpring (2 items — own debt)

### WS-9: Cross-Tier Parity (L3 pending)

**Priority:** MEDIUM

L1 (breseq 0.40.1) vs L2 (sovereign Rust) parity is documented for
Barrick 2009: 486 sovereign vs 569 breseq (0.85 ratio), 0 position-level
overlap due to coordinate representation mismatch.

**Remaining:**
- [ ] L2 vs L3 (local vs IPC-composed via live trio) — requires live
  rhizoCrypt + loamSpine + sweetGrass deployment
- [ ] Position-level matching with ±5bp window tolerance
- [ ] Adaptive frequency threshold per generation (early gens over-call
  in repeats, late gens under-call sub-clonal sweeps)
- [ ] lithoSpore `expected_values.json` contract for Barrick 2009

### WS-11: Variant Caller Parity (Active Calibration)

**Priority:** HIGH — directly affects lithoSpore braid quality

Sovereign pipeline over-calls vs breseq baseline:
- v0 (permissive): ~34,000 variants/clone (8,500x)
- v1 (refined): ~60-78 variants/clone (15-19x)

**Remaining:**
- [ ] Quality-weighted binomial model in `SnpCallingF64` shader
- [ ] Mapper-level deduplication of repetitive region alignments
- [ ] Per-generation frequency threshold calibration
- [ ] Cross-validate against breseq polymorphism mode for late-generation
  sub-clonal sweeps

---

## wetSpring-reported, routes upstream (4 items)

### WS-1: Ionic Contract Negotiation

**Owner:** primalSpring Track 4
**Priority:** HIGH

Bonding metadata is declared and `GET /api/v1/system/composition` exposes
it, but no automated protocol exists for establishing, modifying,
terminating, or verifying ionic bonds. External researchers cannot
self-service bond creation.

**Ask:** primalSpring defines the negotiation protocol (handshake,
contract serialization, mutual verification) in Track 4. This is
architectural — needs a spec before implementation.

### WS-2: Cross-Spring Data Exchange (RootPulse semantic function)

**Owner:** biomeOS (orchestration) + provenance trio (rhizoCrypt, loamSpine, sweetGrass)
**Priority:** HIGH

NestGate stores locally but no protocol exists for another spring's
NUCLEUS to pull provenance-wrapped data subsets. No differential sync.
Each spring operates as a data silo. RootPulse is the semantic function
that should handle this — it's a NeuralAPI composition across the
provenance trio, not a standalone service.

**Ask:** biomeOS defines a `rootpulse.sync` or equivalent NeuralAPI
composition graph that orchestrates cross-spring provenance exchange via
the trio. Minimum viable: one spring can request a braid subset from
another spring's NestGate with provenance continuity through
`signal.dispatch` → trio pipeline.

### WS-3: Public Chain Anchor

**Owner:** loamSpine team
**Priority:** MEDIUM

The provenance trio produces DAG sessions (rhizoCrypt), ledger commits
(loamSpine), and semantic braids (sweetGrass). None are anchored to a
public verifiable ledger. Provenance is verifiable only within the
ecosystem trust boundary.

**Ask:** loamSpine explores public timestamping options (RFC 3161 TSA,
blockchain anchor, or similar). Spec-level — no implementation pressure.

### WS-4: petalTongue Client-Side WASM

**Owner:** petalTongue team
**Priority:** MEDIUM

All grammar-of-graphics rendering requires a live HPC connection via
petalTongue RPC. No offline rendering, no client-side interactivity
beyond Plotly.js.

**Ask:** petalTongue scopes a WASM compilation target for the grammar
engine. Phase 3 of web deployment. No timeline pressure — Plotly.js
remains functional.

---

## Tenaillon 2016 Prerequisites (cross-team)

Tenaillon is wetSpring's next major dataset (264 clones, 590 GB, 312
accessions, 524 FASTQs). These are the cross-team prerequisites:

| Prerequisite | Owner | Status |
|-------------|-------|--------|
| `compute.fan_out` DAG-aware dispatch | toadStool | Shipped (S254) |
| Consumer socket pattern (connect-probe) | wetSpring | Absorbed (Wave 23) |
| GPU API `submit_and_map` alignment | barraCuda + coralReef | **OPEN** (CG-3, see primal blurb) |
| SRA download (590 GB) | wetSpring | Workspace ready, download queued |
| Variant caller v2 (binomial model) | wetSpring | **OPEN** (WS-11) |

---

## Summary

| # | Gap | Owner | Priority | Status |
|---|-----|-------|----------|--------|
| WS-1 | Ionic contract negotiation | primalSpring Track 4 | HIGH | Needs spec |
| WS-2 | Cross-spring RootPulse exchange | biomeOS + trio (rhizoCrypt, loamSpine, sweetGrass) | HIGH | Not started |
| WS-3 | Public chain anchor | loamSpine | MEDIUM | Not started |
| WS-4 | Client WASM renderer | petalTongue | MEDIUM | Not started |
| WS-9 | Cross-tier parity (L3) | wetSpring | MEDIUM | L1/L2 done |
| WS-11 | Variant caller calibration | wetSpring | HIGH | Active |
