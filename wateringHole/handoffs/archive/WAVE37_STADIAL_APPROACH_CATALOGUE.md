# Wave 37 — Stadial Approach Catalogue

**Date**: May 21, 2026
**From**: primalSpring Wave 37 (748 tests, 445 methods, 45 scenarios)
**Purpose**: Catalogue remaining work visible from local for upstream/downstream teams as we approach stadial gates.

---

## Completed This Wave (37)

| Item | Impact |
|------|--------|
| **WS-1 Ionic bond runtime** | `IonicContractRegistry` — full state machine, metering, TTL, provenance seal. Unblocks ionic matrix cell + flockGate cross-family dispatch. |
| **SP-4 Sovereign publish** | `publish_sporeprint.sh` — NestGate `content.put` pipeline for sporePrint content. E2E requires live NestGate. |
| **Metrics sweep (Wave 36)** | 456→445 methods, 744→748 tests, 43→45 scenarios corrected across 14 repos. guidestone→UniBin. musl→genomeBin. Phase 0.5→1. |
| **Handoff fossilization** | 22 superseded handoffs archived across 5 springs. Queue now clean (7 living handoffs across all springs). |

---

## Upstream Work Needed (from primals/gardens)

### bearDog
- **E2E ionic bond signing**: `crypto.ionic_bond.propose` / `verify_proposal` / `seal` — primalSpring types exist, bearDog crypto implementation needed for live E2E.
- **ACME renewal daemon**: Phase 3 sovereign TLS renewal for cellMembrane S1 cutover.

### songbird
- **`capability.call` remote dispatch**: Cross-gate TCP routing for ionic bond calls. WS-1 client exists but needs songbird WAN relay integration.
- **NAT field test**: Residential NAT traversal via STUN/TURN for flockGate bootstrap.

### toadStool
- **`compute.fan_out` at scale**: Tenaillon 2016 batch (590 GB) needs fan_out across strandGate compute nodes.
- **`max_guest_load` yield semantics**: Designed but not wired for flockGate power-cycle-aware scheduling.

### biomeOS
- **`nest.sync` live orchestration**: 6-node graph shipped (v3.64) but E2E wiring pending. Unblocks WS-2.
- **`capability.call` → songbird**: Remote gate dispatch for cross-LAN capability sharing.

### loamSpine
- **WS-3 public chain anchor**: `PUBLIC_TIMESTAMPING.md` spec exists, RFC 3161 variant added, implementation open.

### petalTongue
- **WS-4 client-side WASM**: Grammar rendering in browser without live HPC. Not started.

### nestgate
- **aarch64-musl segfault**: Exit 139 on ARM. Blocks `nucleus-aarch64-mixed-tcp` cell.

### projectNUCLEUS
- **westGate Nest deploy**: First LAN expansion target. Hardware ready, deployment not executed.
- **Sovereign DNS (knot-dns)**: Channel 1 planned, not started.
- **Forgejo Actions CI**: Migration from GitHub Actions. Design exists.

---

## Downstream Work (from springs)

### wetSpring (highest pressure)
- **WS-11 re-measurement**: Variant caller calibration + Tenaillon batch 0 MAPQ.
- **WS-9 L3 cross-tier parity**: Live trio on deployed Nest.
- **V182 UniBin audit response**: 345 scenarios, sporePrint surface. Pending review.

### healthSpring
- **Track 4 ionic wiring**: Can now consume WS-1 `IonicContractRegistry` for `healthspring_unibin`.
- **`NestGate storage.egress_fence`**: Wire-name reconciliation.

### hotSpring
- **K80 cross-gen path**: Sovereign warm handoff for incoming K80 GPU.
- **GPC boundary CE validation**: May 19 handoff pending consumption.

### neuralSpring
- **B3/B4 ML surrogates**: lithoSpore modules 3+4 integration. Ready for absorption.

### ludoSpring
- **projectNUCLEUS workload**: `ludospring-metalforge-parity.toml` for V76 composition.

### groundSpring
- **Docs debt + lithoSpore integration**: V145 handoff still living.

---

## Glacial Shift Gate Criteria (6-point)

| # | Criterion | Status | Blocker |
|---|-----------|--------|---------|
| 1 | S1–S4 shadow cutover (7-day gate) | S1-S3 LIVE, S4 READY | Shadow period not started |
| 2 | Multi-gate LAN mesh (3+ gates) | 2/9 validated | westGate deploy needed |
| 3 | cellMembrane Nest on VPS | Tooling ready | Not deployed on VPS |
| 4 | flockGate WAN validation | Config ready | Not deployed, NAT untested |
| 5 | Sovereign DNS (knot-dns) | Planned | Not started |
| 6 | Cloudflare removal | Caddy→BearDog shadow live | Not cut over |

**Next wave priorities**: westGate deploy (criterion 2), S4 shadow start (criterion 1), cellMembrane VPS deploy (criterion 3).

---

## Living Handoff Queue (post-fossilization)

| Spring | Handoff | Status |
|--------|---------|--------|
| healthSpring | V64Y River Delta Gap Response | Open asks (Track 4 now answered by WS-1) |
| airSpring | Wave 20 PM LithoSpore Absorption | Reference |
| ludoSpring | V76 Downstream Absorption | Open (projectNUCLEUS workload) |
| groundSpring | V145 Docs Debt | Open (lithoSpore follow-through) |
| wetSpring | Upstream Asks River Delta | Open (WS-1 answered, WS-2/3/4 pending) |
| wetSpring | V182 UniBin Wave 28 | Pending review |
| neuralSpring | V169 B3/B4 ML Surrogates | Open (lithoSpore modules) |
| hotSpring | GPC Boundary CE Validate | Open |
| hotSpring | Sovereign Driver Rotation Exp 211 | Informational |
