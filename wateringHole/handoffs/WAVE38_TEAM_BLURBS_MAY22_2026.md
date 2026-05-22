# Wave 38 — Team Evolution Blurbs

**Date**: May 22, 2026
**From**: primalSpring (Wave 38, 748 tests, 445 methods)
**Context**: Shadow data is live for S1-S3. Ecosystem at zero debt (13/13).
Critical path is now operational deployment, not code. Teams should ingest
operational data as it flows and evolve accordingly.

---

## bearDog

**Shadow data in**: S1 TLS shadow LIVE — BearDog :8443 at ~10ms vs
Cloudflare ~120ms tunnel. You're winning by 12x.

**Action items**:
1. Wire `crypto.ionic_bond.propose` / `verify_proposal` / `seal` methods.
   primalSpring has the `IonicContractRegistry` types + state machine
   (WS-1, Wave 37). You provide the Ed25519 signing envelope. This
   unblocks live ionic bonds between deployed gates.
2. ACME renewal daemon — Phase 3 of sovereign TLS. Caddy currently
   handles renewal on cellMembrane. BearDog needs to own it for S1
   formal cutover. This is the last piece before Cloudflare removal.
3. Review `content.*` scope in session tokens — SP-4 `content.put`
   pipeline is built and will hit your token validation in production.

**Pull**: primalSpring for `bonding::ionic` + `bonding::ionic_runtime` types.

---

## songbird

**Shadow data in**: S2 NAT relay LIVE — TURN :3478 at 100% reachable for
3+ days running. Cloudflared tunnel is the commercial baseline.

**Action items**:
1. `capability.call` remote dispatch over TCP — cross-gate routing so
   ionic bond calls can traverse LAN/WAN. Your existing TURN relay is
   the transport; wire the JSON-RPC routing layer on top.
2. NAT field test — when flockGate deploys, you need residential NAT
   traversal proven under real conditions (CGNAT, double-NAT, mobile
   tethering). STUN/TURN chain from Wave 213 is the foundation.
3. Shadow data: monitor TURN relay uptime and latency metrics. The 7-day
   100% reachable gate is the formal cutover criterion.

**Pull**: primalSpring for `validation::shadow` comparison pattern.

---

## toadStool

**No immediate shadow data, but operational horizon approaching.**

**Action items**:
1. `compute.fan_out` at scale — Tenaillon 2016 batch is 590 GB across
   multiple clones. When strandGate (64c EPYC) deploys, this is the
   first real distributed workload. Design the fan_out graph now.
2. `max_guest_load` yield semantics — flockGate runs on family hardware
   with natural power cycles. Your scheduler needs to yield gracefully
   when guest load exceeds threshold and resume when idle.
3. S267 sandbox `working_dir` is production — make sure all new dispatch
   paths inherit it.

---

## biomeOS

**Action items**:
1. `nest.sync` live orchestration — the 6-node graph (v3.64) is shipped
   but untested end-to-end. With westGate deploying as Nest data tier,
   you'll have real infrastructure to orchestrate against. This is WS-2.
2. `capability.call` → songbird — route cross-gate capability dispatch
   through songbird TCP relay. This is the composition layer that makes
   multi-gate LAN mesh actually functional.
3. Registry is 445 methods (was 452). If you have any hardcoded counts,
   update.

---

## loamSpine

**Action items**:
1. WS-3 public chain anchor — low urgency until trio data volume
   justifies anchoring, but the spec is ready (`PUBLIC_TIMESTAMPING.md`).
   `AnchorTarget::Rfc3161Tsa` variant exists. When you're ready, wire
   the RFC 3161 TSA client.
2. Your 43 methods include `anchor.publish_batch` + `anchor.verify` —
   these are the surface that primalSpring validates. Any changes, push
   a handoff.

---

## nestgate

**Shadow data in**: S3 content shadow LIVE — NestGate + petalTongue at
67ms TTFB vs GitHub Pages 111ms. You're winning.

**Action items**:
1. **aarch64-musl segfault** (Exit 139) — blocks ARM deployment cells.
   Not blocking LAN mesh (x86 gates), but will block swiftGate and any
   ARM expansion. Investigate when bandwidth allows.
2. cellMembrane Nest deploy is imminent (`deploy_membrane.sh
   --composition nest`). Your binary will be the first new primal on VPS
   after Tower. Make sure S68 ZFS storage detector works on DO volumes.
3. SP-4 `content.put` will start hitting you in production. Validate the
   base64 + BLAKE3 ingest path.

---

## petalTongue

**Shadow data in**: S3 content shadow — you're the frontend. 67ms TTFB
sovereign vs 111ms commercial.

**Action items**:
1. WS-4 client-side WASM — future work, not blocking glacial shift. When
   ready, grammar rendering without live HPC connection.
2. S3 shadow parity is proven. Continue monitoring TTFB metrics for the
   7-day formal cutover gate.

---

## squirrel

**No open pressure. CLEAN.**

Continue inference dispatch evolution. When biomeOS wires `capability.call`
→ songbird for cross-gate, your AI composition paths will need to handle
remote inference routing.

---

## Provenance trio (rhizoCrypt + loamSpine + sweetGrass)

**Trio is the first expansion on cellMembrane VPS** (Nest composition).
`deploy_membrane.sh --composition nest` deploys all three alongside
nestgate. When Nest goes live on VPS, your methods will be exercised in
cross-boundary production for the first time.

Ensure `dag.*`, `spine.*`, and `braid.*` aliases are all clean — GAP-36
resolution from S68/v0.9.16/v0.7.35 will be validated under real load.

---

## Spring Teams — Quick Reference

| Spring | Pull | Key item | Priority |
|--------|------|----------|----------|
| **wetSpring** | primalSpring | WS-11 re-measurement (2/5 Tenaillon clones), WS-9 L3 parity, registry now 445 | HIGH |
| **healthSpring** | primalSpring | `IonicContractRegistry` available (WS-1) — wire into Track 4 ionic bridge | MEDIUM |
| **hotSpring** | — | K80 cross-gen path, GPC boundary CE (May 19 handoff open) | MEDIUM |
| **neuralSpring** | — | B3/B4 ML surrogates for lithoSpore modules 3+4 | MEDIUM |
| **ludoSpring** | — | `ludospring-metalforge-parity.toml` for projectNUCLEUS | LOW |
| **groundSpring** | — | Docs debt + lithoSpore integration (V145 handoff) | LOW |
| **airSpring** | primalSpring | Registry synced to 445. No open pressure. | — |

---

## Garden Teams

**projectNUCLEUS**: westGate NUCLEUS deploy is your top operational
priority. Hardware is ready. This moves the multi-gate criterion from
2/9 → 3/9. After that: cellMembrane Nest on VPS, then sovereign DNS.

**lithoSpore**: Test fixture registry updated to 445 (Wave 38). B3/B4
neural surrogates are the next science integration target from
neuralSpring. USB artifact pipeline continues to work.

---

*Push evolution back as you ingest. We absorb on the next pass.*
