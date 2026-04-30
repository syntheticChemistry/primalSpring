# Primal Capability Status — Consolidated Audit

**Date**: 2026-03-22 (updated March 22 evening — post primal evolution review)
**From**: primalSpring v0.7.0
**Consolidates**: 6 capability audits from v0.3.5 (March 18, 2026), cross-
referenced against primal evolution through March 22 (biomeOS v2.67,
BearDog v0.9.0, Squirrel alpha.17, Songbird wave60)
**License**: AGPL-3.0-or-later

---

## Purpose

This document tracks the capability compliance state of each primal against
ecosystem standards. Items marked RESOLVED have been verified in the primal's
codebase or handoffs. Items marked OPEN still need work.

---

## BearDog (v0.9.0)

| # | Item | Status | Notes |
|---|------|--------|-------|
| 1 | `health.liveness`, `health.readiness`, `capabilities.list` JSON-RPC | **RESOLVED** | Implemented in `beardog-tunnel` handlers (health.rs, capabilities.rs) |
| 2 | Android abstract socket support | **EVOLVED** | Old `BEARDOG_ABSTRACT_SOCKET` env replaced by `platform/android.rs` module using `@biomeos_{primal}` namespace. Needs Pixel re-test. |
| 3 | `/tmp/neural-api.sock` in neural registration | **REDUCED** | 5-tier Neural discovery in `neural_registration.rs`, but `/tmp/neural-api.sock` remains as last-resort fallback |
| 4 | `peer_id: "songbird-nat0"` default | OPEN | Identity coupling |
| 5 | `register_with_legacy_songbird()` | OPEN | Dead pattern |
| 6 | `crypto.*` method naming | **RESOLVED** | Standard `crypto.*` is primary namespace (~91 methods); `beardog.crypto.*` aliases for Tor/onion legacy |

---

## biomeOS (v2.67)

| # | Item | Status | Notes |
|---|------|--------|-------|
| 1 | `DirectBeardogCaller` in production | **REDUCED** | Enrollment prefers `NeuralApiCapabilityCaller` when socket exists; direct retained only for bootstrap. Caller-agnostic `load_lineage()` free functions in v2.67. |
| 2 | Identity-based discovery helpers | **RESOLVED** | v2.66: 5 callsites evolved to `discover_capability_socket()` with taxonomy keys |
| 3 | `genetic.*`/`lineage.*` translations | **RESOLVED** | v2.66: taxonomy aliases, default primal fix, translations in `defaults.rs` |
| 4 | Hardcoded primal rosters | **RESOLVED** | v2.67: dynamic `primals.len()`, `primal_names` constants, lowercase convention |
| 5 | Neural API socket readiness (exp060) | **LIKELY RESOLVED** | v2.66: `serve()` binds socket before bootstrap/translation loading. Needs primalSpring re-test. |
| 6 | `capability.list` vs `capabilities.list` | **NAMING DRIFT** | biomeOS uses `capability.list`; primalSpring expects `capabilities.list`. See naming drift section below. |

---

## NestGate

| # | Item | Status | Notes |
|---|------|--------|-------|
| 1 | `health.liveness`, `health.readiness`, `capabilities.list` | OPEN | Not yet reviewed against latest NestGate code |
| 2 | Socket resolution 4→5 tier | OPEN | |
| 3 | USB binary segfault / static musl rebuild | OPEN | |

---

## Songbird (v0.2.1-wave60)

| # | Item | Status | Notes |
|---|------|--------|-------|
| 1 | `health.liveness`, `health.readiness`, `capabilities.list` aliases | OPEN | Still uses `health` and `primal.capabilities` names. Added "ecosystem standard method aliases" but these three not confirmed. |
| 2 | Neural API for BearDog crypto (4+ crates) | OPEN | `songbird-http-client` has `RoutingMode::NeuralApi`; tor/orchestrator/nfc/onion still direct `BeardogCryptoClient` |
| 3 | Identity-based discovery tiers | OPEN | Orchestrator still uses 7-tier with `BEARDOG_*`/`beardog.sock` names |
| 4 | 12th subsystem (11/12 UP) | OPEN | Not identified which subsystem fails |

Quality improvements (not in original audit):
- 9,969 tests, 0 clippy, all files under 1000 LOC
- Federation mock→real state, zero-copy evolution, 25 fuzz tests
- 30 workspace crates, ~72% coverage (target 90%)

---

## Squirrel (v0.1.0-alpha.17)

| # | Item | Status | Notes |
|---|------|--------|-------|
| 1 | BearDog auth socket fallbacks | OPEN | `capability_crypto.rs`, `security_provider_client.rs` still enumerate hardcoded paths |
| 2 | `DEPENDENCIES` in `niche.rs` names primals | OPEN | |
| 3 | Socket startup timeout (exp061) | **DIAGNOSED** | Squirrel's `UniversalListener` binds abstract socket first (`\0squirrel`), not filesystem. primalSpring harness waits for filesystem `.sock` — fix by setting explicit `SQUIRREL_SOCKET` path in launch profile. **Actionable from primalSpring side.** |
| 4 | `capability.list` vs `capabilities.list` | **NAMING DRIFT** | Uses `capability.list` (biomeOS convention). See naming drift section below. |

Quality improvements (not in original audit):
- 5,775 tests, 0 clippy, 977 max LOC, ~73% coverage
- BYOB graph types + graph.parse/validate RPC (alpha.15)
- `UniversalListener` with abstract+filesystem+TCP transport
- deny.toml ecoBin bans, cross-compile config for aarch64

---

## ToadStool

| # | Item | Status | Notes |
|---|------|--------|-------|
| 1 | `health.liveness`, `capabilities.list` | OPEN | Not yet reviewed against latest ToadStool code |
| 2 | Showcase examples off primal names | OPEN | |
| 3 | Integration crate naming | OPEN | |

---

## ToadStool

| # | Open Item | Impact |
|---|-----------|--------|
| 1 | Register `health.liveness`, `capabilities.list` as JSON-RPC methods | Ecosystem probe compatibility |
| 2 | Evolve showcase examples off primal names and fixed socket paths toward `discover_by_capability` patterns | Showcase code teaches bad patterns |
| 3 | Rename integration crates from primal names (`integration/beardog`, `integration/nestgate`) to capability-oriented names (`security-provider`, `storage-provider`) | Identity coupling in module structure |

---

## Ecosystem Naming Drift: `capability.list` vs `capabilities.list`

A real integration issue discovered during this review:

| Primal | Method Name |
|--------|-------------|
| BearDog v0.9.0 | `capabilities.list` |
| biomeOS v2.67 | `capability.list` |
| Squirrel alpha.17 | `capability.list` |
| primalSpring v0.7.0 | probes `capabilities.list` |
| Songbird wave60 | `primal.capabilities` |

Three different names for the same operation. The `SEMANTIC_METHOD_NAMING_STANDARD.md`
should be updated to canonicalize one form. Until then, all probing code
(primalSpring's `extract_capability_names()`) should accept all known aliases.

**Recommendation**: Adopt `capabilities.list` as canonical (matches BearDog,
which is the most compliance-driven implementation). biomeOS and Squirrel
should add `capabilities.list` as an alias for `capability.list`.

---

## Cross-Primal Standards (Expectations for All)

These are the **ecosystem expectations** that every primal should meet.
primalSpring validates compliance via its integration tests and experiments.

### Required JSON-RPC Surface

Every primal must register these methods (and common aliases):

| Method | Aliases | Response | Purpose |
|--------|---------|----------|---------|
| `health.liveness` | `ping`, `health` | `{"status": "alive"}` | Am I running? |
| `health.readiness` | | `{"status": "ready", ...}` | Am I ready to serve? |
| `health.check` | `status`, `check` | `{"status": "healthy", ...}` | Full health with details |
| `capabilities.list` | `capability.list`, `primal.capabilities` | `[{"name": "...", ...}]` | What can I do? |

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
