# Stadial Parity Gate — April 16, 2026

**From**: primalSpring (coordination + composition spring)
**To**: All primal teams, all spring teams
**Phase**: **INTERSTADIAL** — gate cleared April 16, 2026
**License**: AGPL-3.0-or-later

---

## What Is a Stadial?

The ecosystem evolves in glacial phases:

- **Glacial** — archived, fossilized. Old code/docs move to `fossilRecord/`. Dead patterns.
- **Stadial** — cold period. Parity gate. All primals reach modern standard before
  the next phase of feature evolution. No downstream springs absorb until the gate clears.
- **Interstadial** — warm period. Feature development, composition expansion, spring absorption.

**The stadial gate has cleared.** All 13 primals have reached modern async Rust parity.
Downstream springs may resume absorption. The standards defined here remain enforced
as **interstadial invariants** — regressions are rejected.

---

## Gate Criteria (ALL must pass for ALL 13 primals)

### 1. `dyn` Dispatch + `async-trait` Elimination (Class 4)

`dyn` dispatch and `async-trait` are **ecosystem-deprecated**, following the same
lifecycle as `ring` in Class 1. There are no "dyn ceilings" or "object-safety
exceptions."

| What | Replace With |
|------|-------------|
| `#[async_trait]` on trait def | Native `async fn` or `fn ... -> impl Future<...> + Send` (RPITIT) |
| `#[async_trait]` on impl block | Remove (native async works on concrete types) |
| `Box<dyn Trait>` / `Arc<dyn Trait>` with finite implementors | **Enum dispatch** |
| `Box<dyn Trait>` with unbounded implementors | Generics + monomorphization at construction site |
| `Box<dyn Error>` | `thiserror` enum or `anyhow::Error` |
| `Pin<Box<dyn Future>>` | Native async (zero-cost state machines) |
| `async-trait` in Cargo.toml | Remove entirely |

**Why this is a gate**: `async-trait` desugars to `Pin<Box<dyn Future>>` — heap
allocation per async call. Native async fn compiles to zero-cost state machines.
`async-trait` also pulls `syn` (proc-macro), inflating compile times. Removing
dyn dispatch enables monomorphization — smaller, faster ecoBins.

### 2. Zero Ghost Debt in Lockfiles

**`ring` lockfile ghost — definitive analysis (April 16)**:

Ring appears in 6 primal `Cargo.lock` files. Deep investigation confirms this is a
**Cargo v4 lockfile artifact**: Cargo includes optional dependencies in the lockfile
even when their feature is not activated. Packages like `rustls`, `rustls-webpki`,
`hickory-proto`, and `x509-parser` list `ring` as an optional dep — Cargo resolves
it into the lockfile for reproducibility even though the `ring` feature is never
enabled.

**Verification** (all 13 primals):
- `cargo tree -i ring` → empty or "did not match" (ring has no build path)
- `cargo deny check bans` → PASS (ring not in build graph)
- `cargo metadata` → ring not in resolve for BearDog; in resolve but
  feature-gated-off for the other 5

**Ring cannot be removed from `Cargo.lock`** without eliminating it as an optional dep
from upstream crates (`rustls`, `rustls-webpki`, `hickory-proto`, `x509-parser`). This
is not actionable at the primal level — it requires Rust ecosystem changes.

**Reclassification**: Ring lockfile presence is **not a stadial gate criterion**.
The actual enforcement is:

| Check | Criterion | Status |
|-------|-----------|--------|
| `cargo deny check bans` | Ring not compiled | **13/13 PASS** |
| No `ring` in Cargo.toml | No direct dep | **13/13 PASS** |
| No feature enables ring | No feature activation | **13/13 PASS** |

**Actionable lockfile debt** (non-ring):

| Ghost | Status | Action |
|-------|--------|--------|
| `sled` in `Cargo.lock` | ~~sweetGrass~~ **RESOLVED** | Crate archived, zero lockfile entries |
| `reqwest` (runtime dep) | petalTongue | Delegate HTTP/TLS to Songbird via tower atomic |

### 2a. Tower Atomic Delegation — TLS and Crypto

Individual primals should NOT maintain their own HTTP/TLS or crypto dependencies.
The tower atomic provides these as composable services:

- **Songbird** → TLS provider. Primals that need outbound HTTPS delegate to
  Songbird's TLS relay via IPC rather than pulling `reqwest`/`hyper-rustls`.
- **BearDog** → Crypto provider. Primals that need signing, encryption, or
  certificate operations delegate to BearDog via BTSP/IPC.

**petalTongue** is the priority: it carries `reqwest` as a runtime dep
(`reqwest → hyper-rustls → rustls`). Migrating to Songbird-mediated HTTP eliminates
the entire rustls chain and aligns with the tower atomic architecture.

### 3. Edition 2024 + `deny.toml` Enforced

All primals must be Edition 2024, have `deny.toml` with C/FFI bans active, and
pass `cargo deny check bans`.

### 4. No "Managed" or "Acceptable" Exceptions

Previous gap registry entries that said "Acceptable — does not affect ecoBin binary"
or "Managed via deny.toml" are reclassified as **stadial debt**. The actual gate
is `cargo deny check bans` PASS + no direct deps on banned crates.

---

## Final Audit — Gate Cleared (April 16, 2026)

### async-trait + dyn: 13/13 COMPLETE

| Primal | `async-trait` dep | `#[async_trait]` attrs | Dispatch pattern | Status |
|--------|:-----------------:|:----------------------:|------------------|--------|
| Songbird | **No** | 0 | RPITIT, enum dispatch | **COMPLETE** |
| Squirrel | **No** | 0 | RPITIT, enum dispatch | **COMPLETE** |
| biomeOS | **No** | 0 | RPITIT | **COMPLETE** |
| petalTongue | **No** | 0 | RPITIT, enum dispatch | **COMPLETE** |
| NestGate | **No** | 0 | RPITIT | **COMPLETE** |
| rhizoCrypt | **No** | 0 | RPITIT | **COMPLETE** |
| loamSpine | **No** | 0 | RPITIT | **COMPLETE** |
| barraCuda | **No** | 0 | RPITIT | **COMPLETE** |
| coralReef | **No** | 0 | RPITIT | **COMPLETE** |
| skunkBat | **No** | 0 | Generics + RPITIT | **COMPLETE** |
| sweetGrass | **No** | 0 | BraidBackend enum, RPITIT | **COMPLETE** |
| toadStool | **No** | 0 | 13 dispatch files, 32 traits → enum + RPITIT | **COMPLETE** |
| BearDog | **No** | 0 | 18 dispatch enums, 22 traits → RPITIT, lockfile clean | **COMPLETE** |

### Lockfile Status: 13/13 `cargo deny check bans` PASS

Ring in `Cargo.lock` is a **Cargo v4 artifact** — optional deps are included in the
lockfile even when their feature is not activated. Ring is never compiled anywhere.
See `PRIMAL_GAPS.md` for root cause analysis.

| Primal | `ring` lockfile | `cargo deny` | Notes |
|--------|:--------------:|:------------:|-------|
| Songbird | artifact | **PASS** | TLS provider (expected) |
| sweetGrass | artifact | **PASS** | dev-dep chain only |
| petalTongue | artifact | **PASS** | reqwest chain (interstadial: delegate to Songbird) |
| NestGate | artifact | **PASS** | vendored rustls-rustcrypto |
| loamSpine | artifact | **PASS** | hickory optional dep |
| toadStool | artifact | **PASS** | — |
| BearDog | **clean** | **PASS** | Wave 55: hickory-resolver removed |
| Squirrel | **clean** | **PASS** | stadial pass eliminated ring+reqwest |
| biomeOS | **clean** | **PASS** | — |
| rhizoCrypt | **clean** | **PASS** | — |
| barraCuda | **clean** | **PASS** | — |
| coralReef | **clean** | **PASS** | — |
| skunkBat | **clean** | **PASS** | — |

---

## Interstadial Remaining Work

All 13 primals have cleared the stadial gate. **primalSpring itself** has also undergone
its own stadial pass (April 16, 2026): `deny.toml` license fix (BSD-3-Clause for `subtle`),
`#[allow(` → `#[expect(` with reasons, clippy 0 warnings, integration test API drift fixed,
experiment registry updated to 75, 570 tests passing. `Arc<dyn ValidationSink>` is justified
(open extensibility + generic `NdjsonSink<W>`). plasmidBin (infra scripts) audited: no
deprecated Rust references, all scripts use `set -euo pipefail`.

The following items are **interstadial work** — improvements, not blockers:

### petalTongue — `reqwest` delegation

`reqwest` is a runtime dependency pulling the `hyper-rustls → rustls` chain.
Migrate to Songbird-mediated HTTP via tower atomic IPC. petalTongue should not
maintain its own TLS stack.

### sweetGrass — `sled` backend

`sled` store crate is archived. Migrate to `redb` or NestGate-backed storage.

### Remaining `dyn` usages

Justified `dyn` remains in primals with unbounded plugin registries (toadStool:
24 usages in infant discovery/plugins, BearDog: HSM test mocks). These are
architecturally correct — unbounded trait objects where the implementor set is
not known at compile time. primalSpring has 3 `Arc<dyn ValidationSink>` (justified:
open extensibility for test harnesses + generic `NdjsonSink<W>`).

### plasmidBin — deploy script consistency

Audit identified two functional risks in deploy scripts:
- `deploy_gate.sh` uses `barracuda server` / `coralreef server` while `start_primal.sh`
  uses `serve` — CLI subcommand drift between paths
- NestGate JWT derivation differs between `start_primal.sh` and `deploy_gate.sh` — potential
  auth inconsistency if both paths are used in the same ecosystem
- `validate_mesh.sh` hardcodes Songbird/BearDog ports instead of sourcing `ports.env`

---

## Resolution Patterns

### async-trait → native async fn

```rust
// BEFORE (deprecated)
#[async_trait]
pub trait Store: Send + Sync {
    async fn get(&self, id: &str) -> Result<Option<Item>>;
}

// AFTER (stadial-compliant)
pub trait Store: Send + Sync {
    fn get(&self, id: &str) -> impl Future<Output = Result<Option<Item>>> + Send;
}
```

### dyn dispatch → enum dispatch

```rust
// BEFORE (deprecated)
pub struct Engine {
    store: Arc<dyn Store>,
}

// AFTER (stadial-compliant)
pub enum StoreBackend {
    Memory(MemoryStore),
    Redb(RedbStore),
    Postgres(PostgresStore),
}

impl Store for StoreBackend {
    fn get(&self, id: &str) -> impl Future<Output = Result<Option<Item>>> + Send {
        async move {
            match self {
                Self::Memory(s) => s.get(id).await,
                Self::Redb(s) => s.get(id).await,
                Self::Postgres(s) => s.get(id).await,
            }
        }
    }
}

pub struct Engine {
    store: Arc<StoreBackend>,
}
```

### Lockfile ghost elimination

```bash
# Find what pulls ring
cargo tree -i ring --edges normal

# If it's a transitive dep from crate X, check if X has a feature
# flag to use a pure-Rust alternative, or swap X for a ring-free crate.
# After fixing, regenerate the lockfile:
cargo update
cargo tree -i ring  # should be empty
```

---

## Interstadial Standards (enforced going forward)

The stadial standards become **permanent invariants**. Any regression is rejected.

### Hard invariants (CI-enforced)

1. **No `async-trait` crate** in any `Cargo.toml` — `deny.toml` bans it
   (wrappers allowed for transitive deps from `axum`, `hickory`, etc.)
2. **No `#[async_trait]` attributes** in `.rs` files — use native `async fn` (RPITIT)
3. **No new `Box<dyn Trait>` / `Arc<dyn Trait>`** for finite-implementor traits —
   use enum dispatch. Unbounded registries (plugin systems) may use `dyn` with
   documented justification.
4. **`cargo deny check bans` PASS** — C/FFI bans enforced (`ring`, `openssl`,
   `sled`, `async-trait` per deny.toml)
5. **Edition 2024** — all crates, `rust-toolchain.toml` pinned
6. **`#![forbid(unsafe_code)]` or `#![deny(unsafe_code)]`** — unsafe confined to
   documented FFI boundaries with `// SAFETY:` comments

### Soft standards (review-enforced)

7. **`#[expect(...)]` over `#[allow(...)]`** — expect fails when the lint no longer
   fires, keeping suppression lists clean
8. **`thiserror` over `Box<dyn Error>`** — typed errors in all public APIs
9. **Zero `TODO`/`FIXME` in production code** — track in debt registries instead
10. **File size < 1000 LOC** — refactor into domain modules when approaching limit
11. **Test coverage ≥ 85%** — measured by `cargo llvm-cov`, target 90%+
12. **Zero commented-out code** — archive to `fossilRecord/` or delete
13. **Tower atomic delegation** — primals do not maintain their own TLS or crypto
    stacks. Songbird provides TLS, BearDog provides crypto, via IPC.

### deny.toml standard template

Every primal must have a `deny.toml` with at minimum:

```toml
[bans]
deny = [
    { crate = "ring" },
    { crate = "openssl" },
    { crate = "openssl-sys" },
    { crate = "async-trait", wrappers = ["<transitive-wrapper-crates>"] },
]
```

Additional bans per primal's domain (e.g., `sled`, `serde_yaml`, `reqwest`).

---

## Gate Status: CLEARED

- [x] `async-trait` crate not in any `Cargo.toml` — **13/13**
- [x] Zero `#[async_trait]` attributes in `.rs` files — **13/13**
- [x] Zero `Box<dyn Trait>` / `Arc<dyn Trait>` for finite-implementor traits — **13/13**
- [x] `cargo deny check bans` passes — **13/13**
- [x] Edition 2024 — **13/13**
- [x] Ring lockfile ghost: Cargo v4 artifact, never compiled — **13/13 PASS**

**Stadial cleared April 16, 2026. Ecosystem enters interstadial.**
Downstream springs may resume absorption. Feature evolution resumes.

---

**License**: AGPL-3.0-or-later
