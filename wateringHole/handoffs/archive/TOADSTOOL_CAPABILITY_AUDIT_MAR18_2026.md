# ToadStool Capability-Based Compliance Audit

**From**: primalSpring coordination (v0.3.5)  
**To**: ToadStool team  
**Date**: 2026-03-18  
**Severity**: Low — mostly showcase examples need evolving

## Executive Summary

ToadStool has the best socket discovery implementation (proper 5-tier), a `SemanticMethodRegistry` for method routing, and capability-based science domain routing. Main issues are in showcase examples and missing standard health methods.

## Fixes Needed

### 1. Register `health.liveness` and `capabilities.list`

Not found in JSON-RPC handlers. Add both.

### 2. Evolve showcase examples

Showcase code hardcodes primal names and socket paths — these are teaching material that should model capability-based patterns:

| Showcase | Coupling | Evolution |
|---|---|---|
| `01-capability-discovery` | `"bearDog"`, `"nestGate"`, `"songBird"` literals | `discover_by_capability()` |
| `01-songbird-registration` | `toadstool.jsonrpc.sock`, `coordination.sock` | Capability discovery |
| `02-beardog-secured-compute` | `security.sock` | `discover_by_capability("security")` |
| `03-nestgate-artifact-storage` | `storage.sock` | `discover_by_capability("storage")` |

### 3. Named integration crates

`crates/integration/beardog` and `crates/integration/nestgate` are named by primal. Consider renaming to `integration/security-provider` and `integration/storage-provider` to reflect capability, not identity.

## What's Good

- 5-tier socket discovery (matches ecosystem standard)
- `SemanticMethodRegistry` for method routing
- `get_socket_path_for_capability(ECOLOGY_CAPABILITY)` — already capability-based in science domains
- `forbid(unsafe_code)` in core
- No `todo!()` or `unimplemented!()` in production
