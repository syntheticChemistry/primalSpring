# Wave 38 — Upstream Evolution Blurb

**Date**: May 22, 2026
**From**: primalSpring (748 tests, 445 methods, 45 scenarios, Wave 38)
**To**: All primal teams, spring teams, garden teams
**Purpose**: Current state + remaining work for ingestion as teams evolve with operational data.

---

## Ecosystem State (May 22)

**13/13 primals at zero debt.** 267+ PASS, 0 FAIL across projectNUCLEUS
validation. Software parity is the best it has ever been. The critical path
has shifted from Rust capability shipping to **physical deployment and
sovereignty cutover**.

Waves 36–38 swept the entire ecosystem: metric alignment (445 methods, 748
tests, 45 scenarios synchronized across 14 repos), 452→445 downstream
registry sync (wetSpring, airSpring, lithoSpore), 22 superseded handoffs
archived, PLASMINBIN→PLASMIDBIN typo fix, guidestone→UniBin references
updated, `build_ecosystem_musl.sh` → `build_ecosystem_genomeBin.sh` everywhere.

**Key protocol deliverables now available:**
- **WS-1 Ionic Bond Runtime** — `IonicContractRegistry` with full state
  machine (propose→accept→meter→modify→terminate→seal), TTL enforcement,
  policy checks, provenance sealing. 12 unit tests. Ready for downstream
  consumption.
- **SP-4 Sovereign Publish** — `tools/publish_sporeprint.sh` publishes
  content to sporePrint via NestGate `content.put` over UDS. E2E requires
  live NestGate instance.

---

## Per-Team Remaining Work

Ingest what applies. Deprioritize or defer what doesn't fit your current
wave. Items marked **(operational)** require physical deployment; items
marked **(code)** are implementable from local.

### bearDog
- **(code)** `crypto.ionic_bond.propose` / `verify_proposal` / `seal` —
  primalSpring types exist in `bonding::ionic_runtime`. Wire the crypto
  signing side. Unblocks live ionic E2E.
- **(operational)** ACME renewal daemon — sovereign TLS renewal for
  cellMembrane S1 cutover. Phase 3 of escalation ladder.

### songbird
- **(code)** `capability.call` remote dispatch — TCP routing for cross-gate
  ionic bond calls. Relay integration with existing TURN infrastructure.
- **(operational)** NAT field test — residential NAT traversal via
  STUN/TURN for flockGate bootstrap. Real-world conditions, not lab.

### toadStool
- **(code)** `compute.fan_out` at scale — Tenaillon 2016 batch (590 GB)
  across strandGate compute nodes when deployed.
- **(code)** `max_guest_load` yield semantics — power-cycle-aware
  scheduling for flockGate distributed compute.

### biomeOS
- **(code)** `nest.sync` live orchestration — 6-node graph shipped (v3.64),
  E2E wiring pending. Unblocks WS-2 cross-spring data exchange.
- **(code)** `capability.call` → songbird — remote gate dispatch for
  cross-LAN capability sharing.

### loamSpine
- **(code)** WS-3 public chain anchor — `PUBLIC_TIMESTAMPING.md` spec
  exists, `AnchorTarget::Rfc3161Tsa` variant added. Implementation open.
  Low urgency until trio data volume justifies anchoring.

### petalTongue
- **(code)** WS-4 client-side WASM — grammar rendering in browser without
  live HPC. Future work, not blocking glacial shift.

### nestgate
- **(code)** aarch64-musl segfault — Exit 139 on ARM. Blocks
  `nucleus-aarch64-mixed-tcp` cell. Not blocking LAN mesh (x86).

### projectNUCLEUS
- **(operational)** westGate NUCLEUS deploy — first LAN expansion. Hardware
  ready. 76TB ZFS makes it the natural Nest data tier.
- **(operational)** Sovereign DNS (knot-dns) — Channel 1 on cellMembrane.
- **(operational)** cellMembrane Nest deploy — `deploy_membrane.sh
  --composition nest` is ready to execute on VPS.

---

## Per-Spring Horizons

### wetSpring (highest downstream pressure)
- **WS-11**: Variant caller re-measurement — v2 deployed (V180), 2/5
  Tenaillon clones validated. Complete the re-measurement.
- **WS-9**: L3 cross-tier parity — L1/L2 done. Requires deployed Nest.
- **V182 UniBin audit**: 345 scenarios. Pending review.
- Registry is now synced to 445 (Wave 38). CI cross-sync surfaces updated.

### healthSpring
- **Track 4 ionic wiring**: `IonicContractRegistry` is available (WS-1).
  Wire into `healthspring_unibin` for bond lifecycle management.
- **NestGate `storage.egress_fence`**: Wire-name reconciliation.

### hotSpring
- **K80 cross-gen**: Sovereign warm handoff when K80 arrives.
- **GPC boundary CE validation**: May 19 handoff still open.

### neuralSpring
- **B3/B4 ML surrogates**: lithoSpore modules 3+4 need neural
  approximations. Ready for absorption.

### ludoSpring
- **projectNUCLEUS workload**: `ludospring-metalforge-parity.toml` for V76.

### groundSpring
- **Docs debt + lithoSpore integration**: V145 handoff still living.

### airSpring
- Registry synced to 445. Wave 20 PM absorption reference still living.
  No open pressure.

---

## Glacial Shift Gate — 6 Criteria

The shift from interstadial to stadial requires these operational gates.
Software is ready. Deployment is the bottleneck.

| # | Gate | Status | Next action |
|---|------|--------|-------------|
| 1 | S1–S4 shadow cutover (7-day) | S1-S3 LIVE, S4 code ready | **Start S4 shadow period** |
| 2 | Multi-gate LAN mesh (3+) | 2/9 (ironGate + eastGate) | **Deploy westGate** |
| 3 | cellMembrane Nest on VPS | Tooling shipped | **Execute on VPS** |
| 4 | flockGate WAN validation | Config ready | Deploy + NAT field test |
| 5 | Sovereign DNS (knot-dns) | Planned | Start on cellMembrane |
| 6 | Cloudflare removal | All shadows live | Formal cutover after S4 |

**Priority order**: westGate → S4 shadow → cellMembrane Nest → flockGate →
DNS → Cloudflare cutover.

---

## How to Consume This

1. **Pull primalSpring** — Wave 38 has the latest metric alignment, ionic
   bond runtime, and publish pipeline.
2. **Check your registry sync** — if your CI cross-syncs against the
   canonical registry, target is **445** (was 452 at Wave 20, corrected
   Wave 36).
3. **Pick up items that fit your wave** — don't block on items marked
   (operational) unless you're deploying.
4. **Push evolution back** — as you ingest operational data and evolve,
   push downstream. We'll pull and absorb on the next pass.

Approaching stadial gates. Review and refinement passes will increase.
