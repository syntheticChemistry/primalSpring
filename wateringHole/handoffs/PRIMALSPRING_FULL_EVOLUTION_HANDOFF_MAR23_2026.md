# primalSpring Full Evolution Handoff — All Teams

**Date**: March 23, 2026
**Version**: v0.7.0 Phase 12.2
**From**: primalSpring
**To**: All primal teams, all spring teams, biomeOS, wateringHole

---

## Executive Summary

primalSpring validates ecosystem coordination itself — the composition, discovery,
bonding, and emergent behavior that biomeOS and the Neural API produce when primals
work together. After 12 phases of evolution (from scaffolding through deep ecosystem
absorption), the spring has converged on patterns absorbed from all 7 sibling springs
and is ready for the next frontier: live multi-node validation.

| Metric | Value |
|--------|-------|
| Tests | 360 (unit + integration + doc-tests + proptest) |
| Experiments | 51 across 9 tracks |
| Deploy graphs | 22 (18 single-node + 4 multi-node federation) |
| Gates | 87/87 |
| Proptest fuzz | 22 cross-cutting IPC property tests |
| RPC endpoints | 17 methods |
| Capabilities | 37 |
| MCP tools | 8 (typed, JSON Schema) |
| clippy | 0 warnings (pedantic + nursery) |
| unsafe | `forbid` workspace-wide |
| C deps | 0 (ecoBin v3.0 compliant, `deny.toml` enforced) |
| missing_docs | `deny` — all public items documented |

---

## Patterns Every Team Should Absorb

### 1. normalize_method() — Prefix-Agnostic JSON-RPC Dispatch

Wire into your server dispatch so `primalspring.health.check` and `health.check`
resolve identically. Handles all 9 ecosystem prefixes.

```rust
let method = normalize_method(raw_method);
match method {
    "health.check" => handle_health_check(params),
    // ...
}
```

**Why**: Any ecosystem caller can invoke your methods with or without the primal
prefix. Eliminates "method not found" failures from cross-spring calls.

### 2. NdjsonSink — Machine-Readable CI Output

```rust
let mut sink = NdjsonSink::stdout();
sink.record("my_check", CheckOutcome::Pass);
```

Each check emits one JSON line. Independently parseable by log aggregators,
CI dashboards, and cross-process tooling.

### 3. OnceLock-Cached Runtime Probes

```rust
static BEARDOG_PROBE: OnceLock<bool> = OnceLock::new();
fn beardog_reachable() -> bool {
    *BEARDOG_PROBE.get_or_init(|| discover::discover_primal("beardog").is_some())
}
```

Probes once per process, caches the result. Safe for parallel `#[test]` execution.
Eliminates flaky race conditions from repeated discovery.

### 4. validate_release.sh — Release Quality Gate

```bash
#!/usr/bin/env bash
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo deny check
cargo test --workspace  # with test count floor
cargo doc --workspace --no-deps
```

The test count floor prevents accidentally deleting tests from passing CI.

### 5. IpcError::is_recoverable()

Broader than `is_retriable()`. Includes transient failures AND server-reported
errors that may resolve if the primal restarts. Use for circuit breaker open/close
decisions. Use `is_retriable()` for immediate retry decisions.

### 6. Transport Enum (Unix + Tcp)

```rust
let transport = connect_transport("unix:/run/biomeos/beardog.sock")?;
let transport = connect_transport("tcp:127.0.0.1:9401")?;
let response = transport.call(&request)?;
```

Callers don't need to know the underlying socket type.

### 7. deny(missing_docs) Workspace-Wide

All public items documented. New public items without docs fail compilation.
Consider upgrading from `warn` to `deny` when your documentation is complete.

---

## Per-Team Guidance

### BearDog Team

**Your primals that primalSpring validates**: BearDog is the Tower security
primal — presence is validated in every atomic composition.

**Absorb from primalSpring**:
- `normalize_method()` — you have 91+ JSON-RPC methods; callers will send both
  prefixed and bare method names
- `OnceLock` probes — with 14,201 tests, cached probes will save significant CI time
- `validate_release.sh` — formalize your pre-release gate with test count floor

**What primalSpring needs from you**:
- Fix abstract socket regression on Android (SELinux blocks filesystem sockets)
- aarch64 musl binary in plasmidBin for Pixel deployment

### Songbird Team

**Your primals that primalSpring validates**: Songbird is the Tower mesh/discovery
primal. Validated in all atomic compositions and central to multi-node bonding
(BirdSong encrypted beacons, STUN NAT traversal).

**Absorb from primalSpring**:
- `Transport` enum — Songbird's mesh spans Unix and TCP; unified transport
  simplifies your networking layer
- `normalize_method()` — accept prefixed calls from biomeOS and other callers
- BondType/STUN tier validation — primalSpring has structural validators for
  your STUN sovereignty-first escalation; use as test reference

### ToadStool Team

**Your primals that primalSpring validates**: ToadStool is the Node Atomic compute
primal — GPU dispatch, dual-protocol, 4 workload types.

**Absorb from primalSpring**:
- `check_relative()` / `check_abs_or_rel()` — for numeric validation of GPU
  compute results (tolerances near zero where absolute is better)
- `NdjsonSink` — machine-readable output for your compute benchmarks
- `OnceLock` probes — cached GPU availability checks in parallel tests

### NestGate Team

**Your primals that primalSpring validates**: NestGate is the Nest Atomic storage
primal — ZFS fallback, model cache, cross-site replication.

**Absorb from primalSpring**:
- Data federation pipeline (exp072) — 7-phase NestGate replication with trio
  provenance; use as integration test reference
- `Transport` enum — NestGate replication will span TCP for cross-site
- `IpcError::is_recoverable()` — storage operations need broader recovery semantics

**What primalSpring needs from you**:
- Fix corrupt static USB build (segfault on spore deployment)

### Squirrel Team

**Your primals that primalSpring validates**: Squirrel is the cross-primal AI
coordinator — MCP tools, context creation, AI queries via Neural API.

**Absorb from primalSpring**:
- `normalize_method()` — Squirrel routes to multiple primals; method normalization
  prevents "method not found" failures
- full_overlay.toml — reference deploy graph for Tower + Nest + Node + Squirrel
  (all capability domains composed)
- `OnceLock` probes — with 6,720 tests, cached probes are essential

### biomeOS Team

**primalSpring's relationship to you**: biomeOS is primalSpring's primary test
subject. primalSpring validates that biomeOS correctly orchestrates primal
compositions via deploy graphs.

**Absorb from primalSpring**:
- 22 deploy graph TOMLs — all with `by_capability`, topologically validated
- Graph bonding metadata (`[graph.metadata]` + `[graph.bonding_policy]`)
- STUN tier config for sovereignty-first NAT traversal
- BondingPolicy enforcement patterns for federated sharing

### Provenance Trio (sweetGrass, rhizoCrypt, loamSpine)

**Absorb from primalSpring**:
- `ipc::provenance` module — full RootPulse pipeline via `capability.call`
- Launch profiles in `config/primal_launch_profiles.toml`
- provenance_overlay.toml deploy graph

**Known gaps to fix**:
- rhizoCrypt: TCP-only (no Unix socket, ignores `RHIZOCRYPT_SOCKET` env var)
- loamSpine: runtime panic in infant_discovery (nested block_on)
- Event type wire format mismatch (struct variants, not strings)
- braid.create / pipeline.attribute param schemas differ

### hotSpring Team

**Absorb from primalSpring**:
- `check_relative()` / `check_abs_or_rel()` — you originated these patterns;
  primalSpring's implementation handles the near-zero edge case
- `normalize_method()` — for your ember + glowplug RPC dispatch
- `validate_release.sh` — formalize with your 848+ test count

### groundSpring Team

**Absorb from primalSpring**:
- `normalize_method()` — you pioneered this; primalSpring handles all 9 ecosystem
  prefixes (you may want to converge)
- `NdjsonSink` — converged with your pattern; use the common implementation
- `OnceLock` probes — replace repeated GPU cache checks

### neuralSpring Team

**Absorb from primalSpring**:
- `is_recoverable()` — broader than `is_retriable()` for Neural API client
  retry/circuit-breaker logic
- `OnceLock` probes — cached model availability for your 1,380+ test suite
- `Transport` enum — Neural API spans both Unix and TCP

### wetSpring Team

**Absorb from primalSpring**:
- `Transport` enum — cross-platform IPC for your 354+ biology simulation binaries
- `is_recoverable()` — refined retry decisions for long-running simulation sockets
- `NdjsonSink` — converged streaming output pattern

### airSpring Team

**Absorb from primalSpring**:
- `Transport` enum — you originated this; primalSpring's version adds
  `connect_transport()` flexible address parsing
- `missing_docs = "deny"` — match your documentation discipline
- `check_abs_or_rel()` — GPU compute tolerances near zero

### healthSpring Team

**Absorb from primalSpring**:
- `check_abs_or_rel()` — combines your absolute and relative tolerance patterns
- `Transport` enum — converged with your IPC pattern
- `validate_release.sh` — formalize your 863-test pre-release gate

### ludoSpring Team

**Absorb from primalSpring**:
- `normalize_method()` — for your 24 IPC capabilities
- `NdjsonSink` — machine-readable output for game telemetry validation
- `validate_release.sh` — test count floor for your 394+ tests

---

## What Remains

| Phase | Focus | Blocker |
|-------|-------|---------|
| 13 | Emergent E2E (RootPulse + coralForge live) | biomeOS + trio running together |
| 14 | Live multi-node (basement HPC, friend remote) | 2+ machines, STUN traversal |
| 15 | Bonding live (covalent mesh, ionic contracts) | Multi-machine NUCLEUS |
| 16 | Cross-spring integration | Multiple springs deployed |
| 17 | Showcase patterns (Track 7 end-to-end) | Mined patterns validated |
| 18 | Anchoring + Economics (BTC/ETH, NFTs) | sweetGrass anchoring live |
| 19 | biomeOS self-composition | Runtime graph composition |

---

## Quality Standards Achieved

- **Zero TODO/FIXME/HACK in production** — all markers resolved
- **Zero clippy warnings** (pedantic + nursery) — code is idiomatic
- **Zero unsafe** — workspace-level `forbid(unsafe_code)`
- **Zero C dependencies** — pure Rust, ecoBin v3.0 compliant
- **Zero missing docs** — all public items documented, `deny(missing_docs)`
- **Zero `#[allow()]` in production** — all suppressions use `#[expect(reason)]`
- **22 proptest fuzz tests** — property-based testing across IPC pipeline
- **Honest scaffolding** — experiments skip honestly, never fake a pass

---

## Key Architecture Decisions

1. **Capability-first, not primal-first**: Deploy graphs use `by_capability`.
   Callers ask for capabilities, not primal identities. Discovery is runtime.
2. **Graphs as source of truth**: TOMLs define compositions. `topological_waves()`
   computes startup ordering. `graph_required_capabilities()` extracts the roster.
3. **Zero compile coupling to orchestrated primals**: All coordination via
   JSON-RPC 2.0 over Unix sockets. No Cargo dependency on any primal crate.
4. **Graceful degradation**: Experiments honestly skip when providers aren't
   running. Circuit breakers with exponential backoff for transient failures.
5. **Chemistry-inspired bonding**: Covalent (strong, trusted LAN), Metallic
   (homogeneous fleet), Ionic (contractual, metered), Weak (zero-trust
   read-only), OrganoMetalSalt (mixed).

---

**License**: AGPL-3.0-or-later
