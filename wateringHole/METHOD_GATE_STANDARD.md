# Method Gate Standard — Ecosystem Pre-Dispatch Authorization (JH-0)

**Version**: 1.0
**Date**: May 7, 2026
**Author**: primalSpring (reference implementation)
**Status**: Active — primalSpring v0.9.25+

## Overview

Every primal's JSON-RPC dispatcher MUST include a pre-dispatch authorization
gate that classifies methods into **Public** and **Protected**, checks caller
identity and/or capability tokens for protected methods, and rejects
unauthorized calls with a standard error code.

This addresses JH-0 (Critical): in multi-user compositions (JupyterHub,
shared ironGate), any localhost process could previously call any primal
method without authentication.

## Architecture

```
Connection → Extract CallerContext → MethodGate::check → dispatch_request
                                          │
                                    ┌─────┴─────┐
                                    │ Public     │ Protected
                                    │ → allow    │ → check token
                                    └────────────┘
                                          │
                                    ┌─────┴─────┐
                                    │ Valid      │ Invalid/missing
                                    │ → allow    │ → PERMISSION_DENIED
                                    └────────────┘
```

## Error Codes

| Code     | Name               | When |
|----------|--------------------|------|
| `-32001` | `PERMISSION_DENIED`| Caller identity established but lacks scope for the method |
| `-32000` | `UNAUTHORIZED`     | Caller identity could not be established |
| `-32002` | `NOT_READY`        | Primal not yet initialized |

These are in the JSON-RPC 2.0 server-defined range (-32000 to -32099).

## Exempt Whitelist (Public Methods)

These methods are always accessible without a token. The whitelist is
intentionally small — only introspection and liveness probes:

| Method              | Purpose |
|---------------------|---------|
| `health.*`          | Liveness and readiness probes |
| `identity.get`      | Primal self-identification |
| `capabilities.list` | Capability advertisement |
| `capability.list`   | Alias for above |
| `lifecycle.status`  | Running/stopped status |
| `auth.check`        | Is the caller authenticated? |
| `auth.mode`         | Current enforcement mode |
| `auth.peer_info`    | Peer credential introspection |

Everything else is **Protected**.

## Enforcement Modes

| Mode          | Behavior |
|---------------|----------|
| `Permissive`  | Log violations but allow all calls (default for backward compat) |
| `Enforced`    | Reject unauthenticated calls to protected methods with `-32001` |

Resolved from `PRIMALSPRING_AUTH_MODE` env var (or primal-specific equivalent).
Default: `Permissive`.

## Caller Context

### Peer Credentials (Unix Sockets)

On Unix domain sockets, the kernel provides `SO_PEERCRED` — the caller's
PID, UID, and GID. This is the caller identity available today without
any token infrastructure.

**Note**: `std::os::unix::net::UnixStream::peer_cred()` is currently behind
the unstable `peer_credentials_unix_socket` feature gate. Primals that
`#![forbid(unsafe_code)]` should defer to `rustix` or wait for stabilization.
The gate pattern works with bearer tokens alone.

### Bearer Token

Capability tokens (ionic tokens, issued by BearDog `auth.issue_ionic`)
are carried in the JSON-RPC request and verified by the gate. Token
verification is a trait/interface — the gate works without it (using
only peer credentials) until BearDog ships `auth.verify_ionic`.

## Implementation Guide

### 1. Add error codes to your protocol types

```rust
pub const PERMISSION_DENIED: i64 = -32_001;
pub const UNAUTHORIZED: i64 = -32_000;
```

### 2. Create a method gate module

Key types:
- `MethodAccessLevel` (`Public` / `Protected`)
- `classify_method(method: &str) -> MethodAccessLevel`
- `CallerContext` (bearer token, peer credentials, connection origin)
- `MethodGate` (enforcement mode, `check()` method)

### 3. Wire into your dispatcher

```
handle_connection(stream)
    → CallerContext::from_unix_stream(stream)
    → for each request:
        → gate.check(method, caller)
        → if Ok: dispatch_request(line)
        → if Err: return PERMISSION_DENIED response
```

### 4. Register auth methods

Add `auth.check`, `auth.mode`, `auth.peer_info` to your capabilities.
These are handled in the gated dispatch layer (not in the main dispatch
table) because they need access to the gate and caller context.

### 5. Add guidestone / validation support

If your primal has a validation binary, add checks for:
- `security:method_gate:wired` — does `auth.mode` respond?
- `security:method_gate:mode` — what enforcement mode?
- `security:method_gate:peer_info` — does peer extraction work?
- `security:method_gate:whitelist` — do public methods pass through?

## Reference Implementation

primalSpring v0.9.25+ — see:
- `ecoPrimal/src/ipc/method_gate.rs` — gate module
- `ecoPrimal/src/ipc/protocol.rs` — error codes
- `ecoPrimal/src/ipc/error.rs` — `PermissionDenied` variant
- `ecoPrimal/src/bin/primalspring_primal/server.rs` — wiring
- `ecoPrimal/src/certification/` — validation (formerly `bin/primalspring_guidestone/layers/btsp.rs`)
- `tools/check_method_gate.sh` — CI validator

## Adoption Path

1. **primalSpring** (done) — reference implementation, permissive default
2. **BearDog** — add `auth.issue_ionic` / `auth.verify_ionic` token issuance
3. **All primals** — adopt the gate pattern, start in permissive mode
4. **Compositions** — flip to enforced mode per-composition when all primals
   in the composition have the gate wired
5. **Step 2b** — BTSP auth inside the encrypted tunnel, replacing PAM
