# Phase 58 Composition Handoff — skunkBat NUCLEUS + Guidestone Hardening

**Date**: May 3, 2026
**From**: primalSpring v0.9.24 (Phase 58)
**To**: All primal teams, spring teams, deployment consumers
**License**: AGPL-3.0-or-later

---

## What Happened in Phase 58

### 1. skunkBat Wired as 13th NUCLEUS Primal

skunkBat matured from lib-only to full standalone binary (GAP-28 resolved) and is
now formally integrated into the NUCLEUS composition:

- **Meta-tier** (like biomeOS, Squirrel, petalTongue) — enhances rather than
  provides core domain capability
- **Capabilities**: `defense`, `recon`, `threat`, `lineage`
- **`required = false`** — NUCLEUS functions without skunkBat but gains defense
  observability when present
- **Depends on**: BearDog (crypto keys), Songbird (discovery)
- **TCP fallback port**: 9140 (corrected May 6 — was 9750)

**What primal teams need to do**: Nothing. skunkBat discovery is automatic via
Songbird `capability.discover`. Primals that want defense observability can call
`defense.baseline_observe` or `recon.metadata_scan` — these are optional
composition enrichments, not requirements.

**What spring teams need to do**: If your deploy graphs hard-code primal counts,
update from 12 to 13. If you use `nucleus_desktop_cell.toml` or
`nucleus_composition_lib.sh`, you already have skunkBat. The `fetch_primals.sh`
script now includes skunkBat in `NUCLEUS_PRIMALS`.

### 2. Guidestone Hardening

Three classes of guidestone failure were resolved:

**BTSP alias routing** — Capability aliases (e.g., `shader.sock`) pointed to
symlinks that may not have BTSP listeners active. New `resolve_btsp_socket`
helper in the composition layer now prefers family-scoped sockets
(`coralreef-default.sock`) over plain aliases. Primals don't need to change
anything — this is a primalSpring-side fix.

**Flex key resolution** — barraCuda's JSON-RPC responses use varying key names
(`result`, `mean`, `data`, `matrix`). The composition layer now tries multiple
candidate keys via `call_f64_flex` / `call_array_flex`. Other primals with
non-standard response shapes benefit automatically.

**Desktop cell health** — The `nucleus_desktop_cell.toml` graph now includes a
`validate_cell` node with explicit health check targets for all 13 primals.

**Squirrel reconnect** — If a proactive BTSP upgrade probe fails (Squirrel
closes the connection on unexpected binary data), the composition layer now
re-establishes the cleartext client rather than leaving a broken handle.

### 3. plasmidBin CI Hub Architecture

plasmidBin is documented as the sole paid GitHub Actions repository in the
ecosystem. Key architecture decisions now formalized:

- **Per-primal concurrency**: each primal gets its own concurrency group, so
  pushes to different primals build in parallel
- **Serialized consolidation**: `checksums.toml` + `manifest.toml` updates
  run with `cancel-in-progress: false` to prevent data loss
- **Signing roadmap**: BLAKE3 checksums (now) → Ed25519 detached signatures →
  BearDog-derived keys → Sigstore-compatible manifest
- **Future distribution channels**: GitHub Releases (now), CDN, OCI registry,
  apt/deb, Nix flake — all via `manifest.toml` `mirror_url` extensibility
- **Fetch contract**: `{primal}-{target}.tar.gz` + `{primal}-{target}.blake3`
  naming standard preserved across all channels

---

## Composition Patterns for Downstream Absorption

### NUCLEUS via Neural API (biomeOS)

The recommended deployment path for springs:

```
biomeos nucleus --mode desktop --family-id $FAMILY_ID
```

This launches all 13 primals, establishes BTSP Phase 3 channels where supported,
and registers capabilities via Songbird. Springs connect through Neural API:

```
neural_api.capability.discover { "capability": "tensor" }
→ { "primal": "barracuda", "socket": "/run/ecoPrimals/barracuda-default.sock" }
```

Springs never import primal crates. All interaction is JSON-RPC 2.0 over UDS/TCP.

### Bonding Models

| Bond | Transport | Trust | Use Case |
|------|-----------|-------|----------|
| **Covalent** | UDS (same machine) | Family seed, BTSP AEAD | Local NUCLEUS |
| **Ionic** | TCP (same LAN) | Mito-Beacon discovery, BTSP negotiate | Cross-gate |
| **Weak** | TCP (internet) | Certificate + BTSP, no family trust | Federation |
| **Metallic** | Any | Pool keys, shared compute | HPC/batch |

skunkBat participates in all bond types as a passive observer — it monitors
connection patterns, validates lineage, and flags anomalies. It does not gate
connections.

### Capability Routing Priority

1. Family-scoped socket (`primal-{FAMILY_ID}.sock`) — always preferred
2. Songbird discovery (`capability.discover`) — canonical runtime path
3. Capability alias symlink (`shader.sock`) — legacy, still works
4. TCP fallback (`tolerances::TCP_FALLBACK_*_PORT`) — last resort

---

## Remaining Upstream Debt

### Phase 3 Client-Server Interop

All 13 primals advertise `chacha20-poly1305` via `btsp.negotiate`, but most
servers still respond in plaintext after negotiation. The primalSpring client
derives SessionKeys and sends encrypted frames — servers need to do the same.

**For primal teams**: When you're ready to ship full transport encryption:
1. After `btsp.negotiate` returns cipher + server_nonce, derive SessionKeys
   (see `primalSpring/ecoPrimal/src/ipc/btsp.rs` for the reference)
2. Wrap all post-negotiate I/O with `[4B len][12B nonce][ciphertext + tag]`
3. The framing is identical for UDS and TCP

This is the last major ecosystem-wide frontier. No primal is blocked — the
null-cipher fallback works. Ship when architecturally ready.

### NestGate JWT Gate

NestGate still requires `NESTGATE_JWT_SECRET` in socket-only NUCLEUS mode.
The evolution path is `NESTGATE_AUTH_MODE=beardog` which skips JWT when BTSP
is the auth layer. This is documented in NestGate's KNOWN_ISSUES.

---

## What We Learned

1. **Family-scoped sockets are the truth** — capability aliases are convenient
   but unreliable for BTSP because symlinks may point to stale or wrong
   listeners. Always prefer `{primal}-{FAMILY_ID}.sock`.

2. **Response format diversity is normal** — different primals return results
   under different JSON keys. Composition layers should be tolerant parsers
   (`result` || `mean` || `data` || `value`).

3. **Meta-tier primals are optional enhancers** — biomeOS, Squirrel,
   petalTongue, and skunkBat follow `required = false` semantics. The NUCLEUS
   functions without them but gains orchestration, AI, UI, and defense when
   they're present.

4. **plasmidBin is the CI chokepoint by design** — centralizing binary
   distribution in one paid repo simplifies billing, signing, and channel
   evolution. Individual primal repos stay on free-tier CI for lint/test.

5. **Reconnect-on-failure** — when probing primals for capability upgrades
   (e.g., BTSP), always re-establish the base client if the probe fails.
   Don't leave broken handles in the connection pool.

---

**License**: AGPL-3.0-or-later
