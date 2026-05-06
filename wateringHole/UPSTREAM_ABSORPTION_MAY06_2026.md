# Upstream Absorption Round — May 6, 2026

**Date**: May 6, 2026
**From**: primalSpring v0.9.24 (post-Discovery Escalation Hierarchy)
**To**: All primal teams, spring teams, deployment consumers
**License**: AGPL-3.0-or-later

---

## What Was Pulled

13 of 13 NUCLEUS primals pulled and reviewed. All code changes verified
with `cargo check`, `cargo clippy`, `cargo test` — zero warnings, zero failures.

| Primal | Version/Wave | Key Changes |
|--------|-------------|-------------|
| BearDog | W86-88 | TCP IPC port aligned to **9100** (was 9900), metrics to **9190**. crossterm 0.29 (mio dedup). Typed config errors |
| Songbird | W189-190 | **`socket` field on `ipc.resolve`** for primalSpring tier-1 discovery. Hardcoded literal cleanup, robust endpoint parsing (`parse_endpoint()` for IPv6) |
| NestGate | S54 | **Wire Standard L3**: `protocol` + `transport` on all four `capabilities.list` surfaces. `consumed_capabilities` naming aligned to `capability_registry.toml` |
| loamSpine | Gap 9 | **Hex string acceptance** for `ContentHash`/`EntryHash` (`[u8; 32]`). Optional `committer` in `AppendEntryRequest`. 14 new tests, 1,504 total |
| rhizoCrypt | S60 | **`parse_hash32` dual-format** (hex + byte array) on all DAG/Merkle handlers. Duplicate tempfile dev-dep removed. 1,573 tests |
| sweetGrass | v0.7.30 | BTSP **double-response fix** (`ServerHello` missing `session_id`). **Whitespace-tolerant TCP autodetect** (`detect_protocol` skips leading ASCII whitespace). `--http-port` flag |
| petalTongue | — | TCP port aligned to **9900**. Discovery escalation hierarchy docs (5-tier). Last 2 hardcoded primal names evolved |
| coralReef | Iter 91 | **Zero-alloc `Cow` aperture names**, `write_rpc_line` prealloc, Vec capacity hints. Marker-byte BufReader consumption fix for post-negotiate I/O |
| barraCuda | Sprint 53 | `submit_and_poll` → **`submit_and_map<T>`** breaking change documented. Discovery escalation hierarchy docs |
| biomeOS | v3.43 | **`registry_queries` reads live Neural API format** (`primary_endpoint` + `primals[].name`) with fallback for legacy `primary_socket` + `provider` |
| Squirrel | — | Already up to date (local) |
| skunkBat | — | Already up to date (local), TCP port corrected 9750→9140 |
| toadStool | S222-S223 | btsp module split (negotiate.rs + relay.rs), sandbox working_dir, env expansion, sleep elimination. 22,833 workspace tests |

---

## What primalSpring Already Handles

These upstream changes were reviewed against primalSpring code — all already aligned:

1. **Songbird `socket` field** — `CompositionContext::discover()` (tier 1) reads `socket`
   first, then `native_endpoint`, then `endpoint`. Aligned since `f7856d1`.

2. **biomeOS v3.43 discovery schema** — `discover_by_capability()` reads `primary_endpoint`
   first (falling back to `primary_socket`), and parses `primals[].name` for resolved
   primal identity. No code change needed.

3. **NestGate Wire Standard L3** — `extract_capability_names()` handles the `methods` key
   (Format F) which is what L3's `capabilities.list` returns. `protocol` and `transport`
   fields are preserved in the JSON response for downstream consumers.

4. **Provenance trio Gap 9** — primalSpring's composition layer sends JSON values through
   `serde_json::json!()` which produces byte arrays. Both loamSpine and rhizoCrypt now
   accept hex strings AND byte arrays, so interop works regardless of sender format.

5. **Port alignment** — all `TCP_FALLBACK_*_PORT` constants in `tolerances/mod.rs` match
   upstream:

   | Primal | primalSpring | Upstream | Status |
   |--------|-------------|----------|--------|
   | BearDog | 9100 | 9100 (W86) | ✓ |
   | Songbird | 9200 | 9200 | ✓ |
   | Squirrel | 9300 | 9300 | ✓ |
   | toadStool | 9400 | 9400 | ✓ |
   | NestGate | 9500 | 9500 | ✓ |
   | rhizoCrypt | 9601 | operator config | ✓ |
   | loamSpine | 9700 | operator config | ✓ |
   | coralReef | 9730 | — | ✓ |
   | barraCuda | 9740 | 9740 (Sprint 53) | ✓ |
   | skunkBat | 9140 | 9140 (DEFAULT_PORT) | ✓ (was 9750) |
   | biomeOS | 9800 | — | ✓ |
   | sweetGrass | 9850 | 9850 | ✓ |
   | petalTongue | 9900 | 9900 | ✓ |

---

## Gaps Closed This Round

### Provenance Trio Gap 9 — Hex/Byte Hash Acceptance

**RESOLVED** on both sides:

- **loamSpine** (v0.9.16): `ContentHash` / `EntryHash` (`[u8; 32]`) deserialize from
  JSON byte arrays, 64-char hex strings (optional `0x` prefix), or raw bytes.
  Handoff: `LOAMSPINE_GAP9_HEX_ACCEPTANCE_HANDOFF_MAY05_2026.md`.
- **rhizoCrypt** (S60): `parse_hash32` helper accepts both hex strings and JSON byte
  arrays. All DAG/Merkle handlers updated.

The trio interop mismatch (rhizoCrypt returns hex, loamSpine expected bytes) is eliminated.

### Phase 3 Client-Server Interop

The Phase 58 handoff noted that "most servers still respond in plaintext after negotiation."
This is now **stale** — every primal that pushed has shipped full encrypted framing:

- BearDog W81: encrypted frame I/O after negotiate
- Songbird W184: binary-framed + NDJSON paths
- NestGate S52: transport hardening (errors propagate)
- sweetGrass v0.7.30: double-response fix (ServerHello session_id)
- coralReef Iter 90: marker-byte BufReader fix
- barraCuda Sprint 51b: `buf_reader.into_inner()` fix for pipelined bytes

**toadStool** (S222-S223) confirmed Phase 3 FULL — btsp module split into negotiate.rs +
relay.rs, sandbox working_dir hardening, env expansion, 22,833 workspace tests.

---

## fieldMouse Reclassification

fieldMouse is a **deployment class** (biomeOS chimeras — smallest deployable units
for edge and IoT), not a primal organism. Removed from:

- `primal_names.rs` — no `FIELDMOUSE` constant
- `env_keys.rs` — no `FIELDMOUSE_PORT`
- `tolerances/mod.rs` — no `TCP_FALLBACK_FIELDMOUSE_PORT`
- `deploy/profiles.rs` — not in port/env lookup tables

NUCLEUS primal count: **13** (BearDog, Songbird, toadStool, NestGate, barraCuda,
coralReef, Squirrel, rhizoCrypt, loamSpine, sweetGrass, petalTongue, skunkBat, biomeOS).
fieldMouse sits alongside these as a deployment architecture pattern.

---

## Remaining Upstream Debt

### toadStool — RESOLVED

toadStool S222-S223 absorbed. Phase 3 confirmed FULL. Wire Standard L3 on `capabilities.list`
(`protocol: jsonrpc-2.0`, `transport: ["uds","tcp"]`) confirmed in identity.rs handler.
Discovery escalation docs present in CONTEXT.md (tiers 1–4; tier 5 intentionally excluded
for UDS-first local IPC). `DEBT.md` shows legacy items RESOLVED. L3 status banner still
says "partial" in NEXT_STEPS.md — cosmetic staleness vs actual code surface.

### skunkBat — Port Fix

TCP_FALLBACK_SKUNKBAT_PORT corrected from 9750 to **9140** to match skunkBat's actual
`DEFAULT_PORT` constant. This was a day-one misalignment that would have caused tier-5
TCP probing to miss a default-config skunkBat instance.

### Cross-Primal

- **BufReader post-negotiate buffering**: barraCuda (Sprint 51b) and coralReef (Iter 90)
  both independently found and fixed the same class of bug — leftover bytes in
  `BufReader` after BTSP negotiate. Any primal that wraps its accept stream in
  `BufReader` before negotiate should audit for this.

- **Whitespace-tolerant TCP detection**: sweetGrass's `detect_protocol` now skips leading
  ASCII whitespace before classifying frames. Other primals with TCP multiplexers
  should consider the same tolerance.

- **Songbird tier-1 registration**: petalTongue docs note that tier-1 Songbird
  `ipc.resolve` is still "future" on their side. Primals that want to be discoverable
  via tier-1 should register with Songbird when TCP is active.

---

**License**: AGPL-3.0-or-later
