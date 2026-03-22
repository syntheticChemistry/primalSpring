# Primal Capability Status — Consolidated Audit

**Date**: 2026-03-22
**From**: primalSpring v0.7.0
**Consolidates**: 6 capability audits from v0.3.5 (March 18, 2026), filtered
against v0.7.0 validated state
**License**: AGPL-3.0-or-later

---

## Purpose

This document replaces the 6 individual primal capability audits from
March 18, 2026 (now archived). It distills the **still-open** items from
those audits after accounting for everything primalSpring v0.7.0 has since
validated: capability-based discovery, graph overlays, deploy graph
validation, Nest/Node/NUCLEUS gates, Squirrel cross-primal wiring,
and hardware cross-arch deployment.

Items are grouped by primal. Each item is a concrete evolution target.

---

## BearDog

| # | Open Item | Impact |
|---|-----------|--------|
| 1 | Register `health.liveness`, `health.readiness`, `capabilities.list` as JSON-RPC methods (only `ping`/`health`/`status` surface exists) | Ecosystem probes expect these standard names |
| 2 | Restore abstract socket support for Android (v0.9.0 regression — `BEARDOG_ABSTRACT_SOCKET` codepath broken) | Blocks all Pixel IPC |
| 3 | Replace hardcoded `/tmp/neural-api.sock` in neural registration with standard 5-tier discovery | Identity coupling |
| 4 | Replace `peer_id: "songbird-nat0"` default with capability-based peer resolution | Identity coupling |
| 5 | Drop `register_with_legacy_songbird()` in favor of capability-based registration | Dead pattern |
| 6 | Add bare-name aliases (or pre-route) for crypto methods (`x25519_*`, `chacha20_*`, `sign_ed25519`, etc.) to standard `crypto.*` names | Method name fragmentation |

---

## biomeOS

| # | Open Item | Impact |
|---|-----------|--------|
| 1 | Replace `DirectBeardogCaller` with `NeuralApiCapabilityCaller` in enrollment, dark forest beacon, lineage derivation (keep direct only for pre-Neural-API bootstrap) | 12+ production bypasses of Neural API |
| 2 | Replace identity-based discovery helpers (`discover_beardog_socket`, `discover_songbird_socket`) with capability-based discovery at each callsite | Identity coupling across many modules |
| 3 | Complete capability translation registry for `genetic.*`, `lineage.*`, and associated `crypto.*` methods | Incomplete routing |
| 4 | Remove hardcoded primal rosters (`orchestrator`, `beacon_verification`, `primal_coordinator`, etc.) in favor of graph/capability-driven lists | Roster coupling |
| 5 | Fix neural-api-server socket readiness (exp060 timeout — server spawns but never creates socket) | Blocks graph-driven Tower atomic deployment |

---

## NestGate

| # | Open Item | Impact |
|---|-----------|--------|
| 1 | Register `health.liveness`, `health.readiness`, `capabilities.list` as JSON-RPC aliases | Ecosystem probe compatibility |
| 2 | Extend socket resolution from 4-tier to 5-tier (add `PRIMAL_SOCKET` env var tier) | Discovery standard alignment |
| 3 | Rebuild as `x86_64-unknown-linux-musl` static-pie (USB binary segfaults, dynamic build fails on Android) | ecoBin compliance |

---

## Songbird

| # | Open Item | Impact |
|---|-----------|--------|
| 1 | Register `health.liveness`, `health.readiness`, `capabilities.list` as JSON-RPC aliases | Ecosystem probe compatibility |
| 2 | Route BearDog crypto calls (`songbird-tor-protocol`, `songbird-orchestrator`, `songbird-nfc`, `songbird-sovereign-onion`) through Neural API instead of direct `BeardogCryptoClient` | Identity coupling across 4+ subsystems |
| 3 | Drop identity-based socket tiers (`BEARDOG_*`, `beardog.sock`); keep capability-oriented tiers; add Neural API as tier 0 | Discovery standard alignment |
| 4 | Fix 12th subsystem (11/12 UP in v3.33.0 — identify and fix the failing one) | Full Tower utilization |

---

## Squirrel

| # | Open Item | Impact |
|---|-----------|--------|
| 1 | Replace BearDog-oriented socket fallbacks in auth (`capability_crypto`, `security_provider_client`, `delegated_jwt_client`) with capability-based discovery | Identity coupling in auth path |
| 2 | Evolve `DEPENDENCIES` in `niche.rs` from primal names to capability-oriented deployment metadata | Compile-time identity knowledge |
| 3 | Fix socket startup timeout (exp061 — Squirrel never creates its socket file within 15s) | Blocks AI composition validation |

---

## ToadStool

| # | Open Item | Impact |
|---|-----------|--------|
| 1 | Register `health.liveness`, `capabilities.list` as JSON-RPC methods | Ecosystem probe compatibility |
| 2 | Evolve showcase examples off primal names and fixed socket paths toward `discover_by_capability` patterns | Showcase code teaches bad patterns |
| 3 | Rename integration crates from primal names (`integration/beardog`, `integration/nestgate`) to capability-oriented names (`security-provider`, `storage-provider`) | Identity coupling in module structure |

---

## Cross-Primal Standards (Expectations for All)

These are the **ecosystem expectations** that every primal should meet.
primalSpring validates compliance via its integration tests and experiments.

### Required JSON-RPC Surface

Every primal must register these methods:

| Method | Response | Purpose |
|--------|----------|---------|
| `health.liveness` | `{"status": "alive"}` | Am I running? |
| `health.readiness` | `{"status": "ready", ...}` | Am I ready to serve? |
| `health.check` | `{"status": "healthy", ...}` | Full health with details |
| `capabilities.list` | `[{"name": "...", ...}]` | What can I do? |

### Required Build Targets

| Target | Linking | Purpose |
|--------|---------|---------|
| `x86_64-unknown-linux-musl` | static-pie, stripped | Desktop/server/USB |
| `aarch64-unknown-linux-musl` | static, stripped | Pixel/ARM server |

Both targets must be pinned to `ecoPrimals/plasmidBin/primals/`.

### Required Cargo Profile

```toml
[profile.release]
strip = true
lto = true
```

### Discovery Standard (5-Tier)

1. `{PRIMAL}_SOCKET` explicit env var
2. `$XDG_RUNTIME_DIR/biomeos/{primal}.sock`
3. `/tmp/biomeos/{primal}.sock`
4. Manifest / socket-registry file
5. Neural API capability query

On Android, abstract sockets (`@biomeos/{primal}`) replace tiers 2-3.

### Socket Naming

Primals must not embed other primals' names in their socket resolution.
Use capability-based discovery: ask "who provides `crypto.sign`?" not
"where is beardog's socket?".

---

## Tracking

When a primal team resolves an item, they should:
1. Create a handoff in their spring's `wateringHole/handoffs/` referencing this doc
2. primalSpring will validate the fix via its integration tests and experiments
3. This document will be updated to mark the item resolved

---

**Archived audits** (March 18, 2026, v0.3.5):
`wateringHole/handoffs/archive/BEARDOG_CAPABILITY_AUDIT_MAR18_2026.md`
`wateringHole/handoffs/archive/BIOMEOS_CAPABILITY_AUDIT_MAR18_2026.md`
`wateringHole/handoffs/archive/NESTGATE_CAPABILITY_AUDIT_MAR18_2026.md`
`wateringHole/handoffs/archive/SONGBIRD_CAPABILITY_AUDIT_MAR18_2026.md`
`wateringHole/handoffs/archive/SQUIRREL_CAPABILITY_AUDIT_MAR18_2026.md`
`wateringHole/handoffs/archive/TOADSTOOL_CAPABILITY_AUDIT_MAR18_2026.md`
