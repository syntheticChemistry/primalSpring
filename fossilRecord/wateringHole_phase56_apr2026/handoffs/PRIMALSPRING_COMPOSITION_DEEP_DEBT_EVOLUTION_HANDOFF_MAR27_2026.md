# SPDX-License-Identifier: AGPL-3.0-or-later

# primalSpring — Composition Deep Debt + Evolution Guidance

**Date**: March 27, 2026
**From**: primalSpring Phase 17 (gen4 Deployment Evolution)
**To**: All primal teams, all spring teams, biomeOS
**Context**: Live composition experiments (exp060–080), cross-gate Pixel deployment,
NestGate integration, biomeOS substrate validation

---

## What This Is

primalSpring experiments validate compositions — primals working together. When
compositions break, it exposes deep debt in individual primals that unit tests
miss. This handoff catalogs every integration issue found during live deployment
and provides concrete evolution guidance for each primal team.

---

## Protocol Evolution (ALL PRIMALS)

### 1. Health Method Naming — Converge to `health.check`

**Problem found**: Different primals use different health method names.
- BearDog: `health.check` (returns `{status, version, timestamp}`)
- Songbird: HTTP GET `/health` (returns `OK`) AND `health.check` (returns `{status, version, uptime_seconds}`)
- NestGate: `health` (returns `{status, version}`) — **missing `.check` suffix**
- Squirrel: `health.check` (returns `{alive, timestamp, version}`)
- biomeOS neural-api: `capability.list` works, but `health.check` returns "Method not found"

**Impact**: Composition health probes need per-primal branching logic. biomeOS
can't use a single `health.check` sweep across all routed primals.

**Recommendation**: All primals should respond to `health.check` with at minimum:
```json
{"status": "healthy", "version": "x.y.z", "primal": "slug"}
```
Songbird's HTTP `/health` is fine as additional surface, but JSON-RPC `health.check`
must also work for composition orchestration.

### 2. Storage API — `family_id` Required Field

**Problem found**: NestGate storage methods (`storage.store`, `storage.retrieve`,
`storage.list`, `storage.delete`) require `family_id` as a mandatory parameter.
Callers without a family ID context get a validation error.

**Impact**: Any primal wanting to use NestGate for persistent state needs to
know its family context. This is correct from a sovereignty perspective, but
the error message should guide callers.

**Recommendation**: NestGate should default `family_id` to the socket's own
family scope when the caller is connected via a family-scoped socket
(`nestgate-{family_id}.sock`). This eliminates redundant `family_id` params
for local callers while keeping it mandatory for cross-family access.

### 3. Nested Key Paths — Directory Auto-Creation

**Problem found**: NestGate `storage.store` with keys containing `/` separators
(`test/primalspring/hello`) fails with "No such file or directory" because the
filesystem backend doesn't auto-create intermediate directories.

**Impact**: Any structured key namespace (common pattern: `{domain}/{primal}/{key}`)
silently fails.

**Recommendation**: NestGate filesystem backend should `create_dir_all` for key
path parents before writing. Flat keys already work correctly.

### 4. Birdsong Beacon API — Required Fields

**Problem found**: Songbird `birdsong.generate_encrypted_beacon` requires three
fields: `node_id`, `family_id`, `capabilities`. Omitting any returns
"missing field" error with only the first missing field named.

**Impact**: Callers iteratively discover required fields via error messages.

**Recommendation**: Either document the full param schema in a `birdsong.schema`
method, or return all missing fields in a single error. Consider making
`capabilities` optional (default to the node's registered capabilities).

---

## Transport Evolution (CROSS-GATE PRIMALS)

### 5. Abstract Socket Support — Ecosystem Decision Needed

**Current state**:
- Squirrel: abstract socket `@squirrel` (Linux-only, no filesystem path)
- BearDog: abstract socket on Android (SELinux blocks filesystem sockets)
- biomeOS: routes to filesystem sockets only
- primalSpring `PrimalClient`: filesystem sockets only

**Problem**: biomeOS cannot route to Squirrel. primalSpring had to implement
custom `SocketAddr::from_abstract_name` + `UnixStream::connect_addr` in exp077.

**Decision required**: Either:
1. biomeOS supports abstract socket addresses in its routing table, OR
2. Squirrel adds filesystem socket fallback alongside abstract socket, OR
3. Both — abstract as optimization, filesystem as standard interface

**Recommendation**: Option 3. Abstract sockets are faster (no filesystem inode)
but non-portable. Filesystem socket should be the ecosystem standard for routing;
abstract socket as a local optimization when both ends are on the same host.

### 6. TCP JSON-RPC — Newline Termination

**Problem found**: BearDog TCP JSON-RPC requires `\n`-terminated requests.
Raw `nc` probes without trailing newline hang silently. This is the correct
behavior per newline-delimited JSON-RPC, but it catches callers off guard.

**Recommendation**: Document newline termination requirement in BearDog's
capability registration metadata. Consider responding with a timeout error
instead of hanging when no newline arrives within 5s.

### 7. Cross-Gate BearDog Discovery — `BEARDOG_SOCKET` env

**Problem found**: Pixel Songbird tried abstract socket `@biomeos_beardog` for
BearDog crypto operations (beacon generation). BearDog on Pixel only binds TCP.
Required explicit `BEARDOG_SOCKET=tcp:127.0.0.1:9100` to fix.

**Recommendation**: Songbird should try `BEARDOG_SOCKET` env var first, then
abstract socket, then filesystem socket, then fail with an actionable error
message listing what was tried. This mirrors the 5-tier discovery pattern.

---

## Schema Evolution (BIOMEOS + SPRINGS)

### 8. Graph Schema Reconciliation

**Problem found**: primalSpring `DeployGraph` TOML schema uses:
```toml
[graph]
name = "..."
[[graph.node]]
```

biomeOS deploy graphs use:
```toml
[graph]
id = "..."
[[nodes]]
```

exp079 had to validate biomeOS graphs via `graph.list` API instead of parsing
TOML directly, because the schemas are incompatible.

**Recommendation**: Converge on a single graph schema. biomeOS as the substrate
should define the canonical schema; primalSpring should adopt it. Alternatively,
`primalSpring::deploy::load_graph` should accept both schemas with a version field.

### 9. Capability Wire Format — 4 Formats Still Active

**Problem found**: `extract_capability_names` in primalSpring handles 4 different
capability wire formats (A: `{capabilities: [{method: "..."}]}`,
B: `{capabilities: ["..."]}`, C: `{result: {capabilities: [...]}}`,
D: `{methods: [...]}`). All 4 are encountered in live compositions.

**Recommendation**: New primals should emit Format B (simplest):
```json
{"capabilities": ["health.check", "crypto.sign", "storage.store"]}
```
biomeOS should normalize all formats to B when routing capability queries.

---

## NestGate-Specific Evolution

### 10. Musl Static Build Segfault

**Problem**: `cargo build --release --target x86_64-unknown-linux-musl` produces
a binary that segfaults immediately. Dynamic release build works fine. This blocks
plasmidBin ecoBin compliance and Pixel deployment.

**Root cause (likely)**: jsonrpsee/tokio runtime initialization under musl.
Similar issues documented in tokio-rs/tokio#4941.

**Recommendation**: Investigate `ring` vs `rustls` TLS backend, or test with
`--cfg tokio_unstable` and `RUSTFLAGS="-C target-feature=-crt-static"`.
Alternatively, build with `cross` using a musl Docker image that links musl libc
at build time.

### 11. `storage.list` Empty Response

**Problem found**: `storage.list` returns `{keys: []}` immediately after
`storage.store` succeeds. The stored key is retrievable but not enumerable.

**Recommendation**: Fix the filesystem backend's list implementation to scan
the family directory and return stored keys. This is critical for any primal
that needs to discover what's in NestGate.

---

## Deep Debt Patterns Exposed by Compositions

### For ALL Primals

| Pattern | What Compositions Exposed | Action |
|---------|---------------------------|--------|
| **Health method diversity** | 4 different health APIs across 5 primals | Converge to `health.check` JSON-RPC |
| **Socket type fragmentation** | Abstract, filesystem, TCP — no universal discovery | 5-tier discovery + filesystem as standard |
| **Error message quality** | "missing field" errors reveal one field at a time | Return all validation errors in a single response |
| **Graph schema drift** | primalSpring vs biomeOS TOML schemas incompatible | Single canonical schema, version field |
| **Capability format drift** | 4 wire formats in the wild | Converge on Format B, biomeOS normalizes |

### For Spring Teams

| Pattern | What to Absorb from primalSpring |
|---------|----------------------------------|
| **`tolerances/` module** | Named constants with provenance docs for all timeouts/thresholds |
| **`primal_names::*` constants** | Zero string literals for primal identifiers in production code |
| **Builder validation** | `ValidationResult::new(title).with_provenance(src, date).run(sub, \|v\| {...})` |
| **Honest scaffolding** | `check_skip("reason")` when dependency unavailable — never fake a pass |
| **4-format capability parsing** | `extract_capability_names` handles all ecosystem wire formats |
| **Deploy graph overlay model** | Tier-independent primals compose via `merge_graphs()` |
| **Cross-gate validation** | TCP fallback with `tolerances::DEFAULT_*_PORT` constants |

---

## Experiment Reference

| Exp | What Broke / What Worked | Primal(s) Affected |
|-----|--------------------------|-------------------|
| 060 | biomeOS `deploy` acted as stub; fell back to `start_primal.sh` | biomeOS |
| 063 | BearDog abstract socket regression on Android/GrapheneOS | BearDog |
| 066 | NestGate storage round-trip works (13/13 gates) | NestGate |
| 068 | Full NUCLEUS 16/16 with live NestGate | NestGate, all Tower |
| 075 | biomeOS neural-api routes crypto.generate_keypair correctly | biomeOS, BearDog |
| 076 | Pixel Songbird birdsong failed until BEARDOG_SOCKET set explicitly | Songbird, BearDog |
| 076 | Birdsong beacon requires node_id + family_id + capabilities (undocumented) | Songbird |
| 077 | Squirrel abstract socket bypasses all standard discovery | Squirrel |
| 077 | biomeOS cannot route to abstract socket primals | biomeOS, Squirrel |
| 079 | biomeOS graph schema differs from primalSpring schema | biomeOS |
| 079 | All 7 sibling spring deploy graphs load via graph.list API | biomeOS |
| 080 | Cross-spring ecology pipeline works structurally | biomeOS |

---

**License**: AGPL-3.0-or-later
