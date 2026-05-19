# Upstream Gaps — Downstream (Products + Foundation) (May 19, 2026)

**Date:** May 19, 2026
**From:** primalSpring (coordination spring)
**To:** projectNUCLEUS, projectFOUNDATION, petalTongue, songbird teams
**Priority:** Sweep — all remaining open downstream gaps
**License:** AGPL-3.0-or-later

---

## Context

Wave 24 (shadow run execution) is issued and absorbed by projectNUCLEUS.
Shadow matrix is operational: S1 TLS LIVE, S2-S4 READY. This blurb
collects every remaining open gap at the product and foundation layers.

---

## projectNUCLEUS (3 items — shadow run blockers)

### Stale Clone: Pull bearDog

**Priority:** IMMEDIATE — phantom gaps

projectNUCLEUS flagged 2 gaps that are already resolved:
1. `specs/ACME_TLS_INTEGRATION_PATH.md` missing — **exists on disk** (Wave 105)
2. `deny.toml` ring `wrappers = []` — **already `["rustls", "rustls-webpki"]`** (Wave 105)

**Ask:** `git pull` bearDog to pick up Waves 105-106. This clears 2 of
your 5 flagged blockers immediately.

### Shadow S2: Songbird Relay Deployment

**Priority:** HIGH — blocks S2 shadow

songbird's TURN client crate is shipped (Wave 205). The relay node
itself is not yet deployed on cellMembrane infrastructure.

**Ask:** projectNUCLEUS + songbird coordinate relay node deployment on
the sovereign VPS. Same machine as bearDog TLS endpoint is the natural
location. Once deployed, `songbird_nat_parity.sh` can begin dual-path
traffic measurement.

### Shadow S3: petalTongue Static Asset Parity

**Priority:** MEDIUM — blocks S3 cutover

petalTongue shipped compression (gzip + brotli), security headers,
HTTP tracing, and custom 404 handling in its latest push. Systematic
parity testing against GitHub Pages is not yet done.

**Ask:** projectNUCLEUS runs the `nestgate_content_parity.sh` script
against the staging subdomain once content mirror is deployed. Checklist:

- [ ] CSS/JS/image MIME types match GitHub Pages
- [ ] Cache-Control headers match
- [ ] Compression ratios comparable
- [ ] 404 handling matches (custom 404.html)
- [ ] TTFB within 1.5× of GitHub Pages p95

---

## projectFOUNDATION (2 items)

### FN-1: BLAKE3 Backfill

**Priority:** MEDIUM

Thread 1 (WCM — World Complexity Map) has 0/24 data sources validated.
All other validatable threads (2, 6, 7) are fully validated.

**Ask:** backfill BLAKE3 hashes for Thread 1 WCM sources. This is
mechanical — download each source, compute hash, update thread index.

### FN-5: Thread 1 WCM Validation

**Priority:** MEDIUM — downstream of FN-1

Once BLAKE3 hashes are backfilled, Thread 1 WCM needs the same CI
validation pipeline that Threads 2/6/7 already pass.

**Ask:** extend the thread-index CI validation to cover Thread 1 after
FN-1 lands.

---

## lithoSpore Integration (tracking)

| Item | Status |
|------|--------|
| Tier 2: 7/7 modules PASS (75/75 checks, 117 tests) | **DONE** |
| Cross-tier parity: 7/7 MATCH | **DONE** |
| Tier 3: USB packaging (`stage_usb.sh`) | **SHIPPED** |
| Barrick 2009 USB artifacts from wetSpring | **SEALED** — handoff May 19 |
| Tier 3: live wire to VM provisioning | Awaiting R7 (biomeOS `spore.instantiate`) |

---

## Summary

| # | Gap | Owner | Priority | Blocks |
|---|-----|-------|----------|--------|
| — | Stale bearDog clone | projectNUCLEUS | IMMEDIATE | Phantom gap clearance |
| S2 | Songbird relay deployment | projectNUCLEUS + songbird | HIGH | S2 shadow |
| S3 | petalTongue asset parity test | projectNUCLEUS | MEDIUM | S3 cutover |
| FN-1 | BLAKE3 backfill (Thread 1 WCM) | projectFOUNDATION | MEDIUM | Thread validation |
| FN-5 | Thread 1 WCM CI validation | projectFOUNDATION | MEDIUM | Thread coverage |
