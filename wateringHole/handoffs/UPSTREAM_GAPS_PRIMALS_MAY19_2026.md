# Upstream Gaps — Primal Teams (May 19, 2026)

**Date:** May 19, 2026
**From:** primalSpring (coordination spring)
**To:** biomeOS, barraCuda, coralReef, songbird, bearDog teams
**Priority:** Sweep — all remaining open primal-level gaps
**License:** AGPL-3.0-or-later

---

## Context

14/14 primals are stale-socket-clean. Wave 22 (stadial gate) and Wave 24
(shadow run execution) are issued. Barrick 2009 is SEALED. This blurb
collects every remaining open gap that lives at the primal layer.

**Action**: each team resolves or explicitly deprioritizes their items,
then responds via wateringHole handoff.

---

## biomeOS (2 items)

### R5: `nest.store` signal dispatch

**Priority:** MEDIUM

biomeOS signal graph framework covers 5 tiers (lifecycle, capability,
resource, composition, braid) across 16 graphs. `nest.store` is not yet
wired as a signal-dispatch target — storing content in nestGate via a
signal graph node requires a manual `capability.call` rather than a
first-class signal route.

**Ask:** Add `nest.store` to signal dispatch routing so composition graphs
can trigger content storage as a graph node without explicit capability
call indirection.

### R7: `spore.instantiate` atomic VM provisioning

**Priority:** LOW

`spore.instantiate` route exists in biomeOS v3.61 but is not wired to a
live lithoSpore VM provisioning backend. Currently returns a scaffold
response.

**Ask:** Wire to lithoSpore's VM provisioning API when lithoSpore Tier 3
is ready, or explicitly mark as deferred-to-stadial.

---

## barraCuda / coralReef (1 item)

### Composition Gap 3: GPU API alignment (`submit_and_map`)

**Priority:** OPEN — blocks sovereign HMMA execution path

The `submit_and_map` API for GPU workload submission is not aligned
between barraCuda's `TensorSession` and coralReef's WGSL codegen output.
The HMMA (half-precision matrix multiply-accumulate) execution path
requires coralReef to emit compatible dispatch calls.

**Ask:** barraCuda + coralReef coordinate on a shared `submit_and_map`
contract. wetSpring's sovereign pipeline is the first consumer (Tenaillon
2016 will stress this path with 264-clone batch dispatch).

---

## songbird / biomeOS (1 item)

### Composition Gap 8: Cross-gate dispatch via songbird

**Priority:** OPEN (Phase 2) — songbird still evolving

Cross-gate `capability.call` routing through songbird's relay is not yet
wired in biomeOS. Current: single-gate compositions only. The covalent
mesh pattern (exp073) defines the target architecture but songbird's relay
layer needs biomeOS integration for multi-gate dispatch.

**Ask:** songbird + biomeOS coordinate on relay-mediated `capability.call`
forwarding. Shadow S2 (NAT traversal) provides the transport; this gap is
about the routing layer above it.

---

## bearDog (1 item)

### Shadow S1: `beardog-acme` crate

**Priority:** HIGH — blocks TLS shadow cutover

TLS termination shadow is LIVE (10ms vs 120ms). ACME integration path
is designed (`specs/ACME_TLS_INTEGRATION_PATH.md`). The crate itself
is not yet implemented.

**Ask:** Implement `beardog-acme` — HTTP-01 challenge handler, cert
storage in `$BEARDOG_DATA_DIR/acme/`, hot-reload via `Arc<ServerConfig>`
swap, renewal daemon (12h check, 30-day-before-expiry). This is the last
gate before TLS cutover.

---

## Summary

| # | Gap | Owner | Priority | Blocks |
|---|-----|-------|----------|--------|
| R5 | `nest.store` signal dispatch | biomeOS | MEDIUM | Composition graph content storage |
| R7 | `spore.instantiate` VM provisioning | biomeOS | LOW | lithoSpore Tier 3 |
| CG-3 | GPU API alignment (`submit_and_map`) | barraCuda + coralReef | OPEN | Tenaillon 2016 GPU path |
| CG-8 | Cross-gate dispatch via songbird | songbird + biomeOS | OPEN | Multi-gate compositions |
| S1 | `beardog-acme` auto-cert crate | bearDog | HIGH | TLS shadow cutover |
