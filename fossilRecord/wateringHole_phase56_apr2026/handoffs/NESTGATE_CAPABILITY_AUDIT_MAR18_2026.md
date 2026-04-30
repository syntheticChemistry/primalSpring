# NestGate Capability-Based Compliance Audit

**From**: primalSpring coordination (v0.3.5)  
**To**: NestGate team  
**Date**: 2026-03-18  
**Severity**: Medium

## Executive Summary

NestGate is clean on dependencies (pure Rust, `forbid(unsafe_code)`) but has coupling to beardog and songbird via hardcoded socket paths, and is missing the standard health/capability methods.

## Fixes Needed

### 1. Register `health.liveness` and `capabilities.list`

**Current**: Exposes `health.check`, `health.metrics`, `health.version`, `health.protocols`  
**Missing**: `health.liveness`, `health.readiness`, `capabilities.list`

### 2. Replace BearDog socket defaults

| Location | Pattern | Evolution |
|---|---|---|
| `nestgate-api/src/transport/config.rs:59-62` | Default `security_provider: /tmp/beardog-{family_id}.sock` | `discover_by_capability("security")` |
| `nestgate-api/src/transport/security.rs:90-94` | Fallback glob: `/tmp/beardog-{family}-*.sock` | Capability discovery |
| `nestgate-core/src/capability_discovery.rs:259` | Default: `/primal/songbird` | `discover_by_capability("discovery")` |
| `nestgate-core/src/capability_discovery.rs:280-346` | `discover_songbird_ipc()` hardcodes bootstrap | Capability-based discovery |

### 3. Extend socket resolution to 5 tiers

**Current**: 4-tier (`NESTGATE_SOCKET` → `BIOMEOS_SOCKET_DIR` → XDG → temp)  
**Standard**: 5-tier (add `PRIMAL_SOCKET` or `/run/user/{uid}/biomeos/` tier)

## What's Good

- `forbid(unsafe_code)` workspace-wide
- No C dependencies (uses `uzers` not `libc`)
- Proper semantic method names (`storage.store`, `storage.retrieve`, etc.)
- `CAPABILITY_MAPPINGS.md` documents provided/required capabilities
- No `todo!()` in production
