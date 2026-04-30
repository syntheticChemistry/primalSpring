# Squirrel Capability-Based Compliance Audit

**From**: primalSpring coordination (v0.3.5)  
**To**: Squirrel team  
**Date**: 2026-03-18  
**Severity**: Low — best capability compliance of all NUCLEUS primals

## Executive Summary

Squirrel is the gold standard for capability compliance: implements `health.liveness`, `health.readiness`, and `capability.list`. Has a `capability_registry.toml` as source of truth. 5-tier socket discovery. Main issues are BearDog socket fallback paths in auth code.

## Fixes Needed

### 1. Replace BearDog socket fallbacks in auth

| Location | Pattern | Evolution |
|---|---|---|
| `core/auth/capability_crypto.rs:88-93` | Well-known paths: `security.sock`, `crypto.sock`, `beardog.sock` | `discover_by_capability("crypto")` |
| `core/auth/security_provider_client.rs:50` | Default: `/var/run/beardog/crypto.sock` | `discover_by_capability("security")` |
| `core/auth/delegated_jwt_client.rs:84-85` | Fallback: `/var/run/crypto/provider.sock` | `discover_by_capability("crypto")` |

### 2. Evolve `DEPENDENCIES` in niche.rs

**Current** (`niche.rs:177-199`):
```
("beardog", true, ...), ("songbird", true, ...), ("toadstool", false, ...)
```

This is deployment metadata with primal names. Consider expressing as capabilities:
```
("security", true, ...), ("discovery", true, ...), ("compute", false, ...)
```

### 3. Deploy graph primal names

`graphs/squirrel_ai_niche.toml` uses node names like `beardog`, `songbird`. These are correct as graph node identifiers, but `by_capability` should be the primary routing mechanism.

## What's Good (Model for Other Primals)

- `health.liveness` and `health.readiness` implemented
- `capability.list` implemented
- `capability_registry.toml` as source of truth
- 5-tier socket discovery
- `forbid(unsafe_code)` in core crates
- `unwrap_used = "deny"`, `expect_used = "deny"` in workspace
- Strong `niche.rs` self-knowledge: `PRIMAL_ID`, `CAPABILITIES`, `CONSUMED_CAPABILITIES`, `COST_ESTIMATES`
