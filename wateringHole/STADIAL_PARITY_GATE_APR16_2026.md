# Stadial Parity Gate — April 16, 2026

**From**: primalSpring (coordination + composition spring)
**To**: All primal teams, all spring teams
**Phase**: STADIAL — parity gate in effect
**License**: AGPL-3.0-or-later

---

## What Is a Stadial?

The ecosystem evolves in glacial phases:

- **Glacial** — archived, fossilized. Old code/docs move to `fossilRecord/`. Dead patterns.
- **Stadial** — cold period. Parity gate. All primals reach modern standard before
  the next phase of feature evolution. No downstream springs absorb until the gate clears.
- **Interstadial** — warm period. Feature development, composition expansion, spring absorption.

**We are in a stadial.** Downstream springs are paused until all 13 primals clear the gate.

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

A `Cargo.lock` stanza for a deprecated crate is **not "managed"** — it is debt.

**`ring` lockfile ghost — root cause identified (April 16)**:
`rustls-rustcrypto v0.0.2-alpha` depends on `rustls-webpki ^0.102` without
`default-features = false`. `rustls-webpki 0.102.x` defaults to `["std", "ring"]`.
This puts ring in `Cargo.lock` even though it is **never compiled** (verified:
`cargo tree -i ring` = empty, `cargo deny check bans` = PASS in all 13 primals).

**Fix pattern**: Vendor `rustls-rustcrypto` per NestGate's approach —
`rustls-webpki = { version = "0.103.12", default-features = false }`.
This eliminates the ring default. Propagate to all primals that use
`rustls-rustcrypto`.

| Ghost | Status | Action |
|-------|--------|--------|
| `ring` in `Cargo.lock` | 6 primals (lockfile artifact, not compiled) | Vendor `rustls-rustcrypto` with NestGate pattern |
| `sled` in `Cargo.lock` | sweetGrass only (loamSpine resolved) | Remove from default features |
| `reqwest` in `Cargo.lock` | petalTongue (Squirrel resolved) | Verify dev-only or eliminate |

**Note**: `ring` lockfile ghosts are lower priority than Class 4 dyn/async-trait
elimination because ring is never compiled and deny checks pass. Clean lockfiles
remain the standard, but the real runtime gate is dyn elimination.

### 3. Edition 2024 + `deny.toml` Enforced

All primals must be Edition 2024, have `deny.toml` with C/FFI bans active, and
pass `cargo deny check bans`.

### 4. No "Managed" or "Acceptable" Exceptions

Previous gap registry entries that said "Acceptable — does not affect ecoBin binary"
or "Managed via deny.toml" are reclassified as **stadial debt**. The lockfile must
be as clean as the build.

---

## Current Audit (April 16, 2026)

### async-trait + dyn Debt

| Primal | `async-trait` dep | `#[async_trait]` attrs | `dyn` usages (code) | Status |
|--------|:-----------------:|:----------------------:|:--------------------:|--------|
| Songbird | **No** | 0 | ~365 | **INTERSTADIAL-READY** (dyn is non-trait-object) |
| Squirrel | **No** | 0 | ~677 | Verify dyn is non-trait-object |
| biomeOS | **No** | 0 | ~159 | **INTERSTADIAL-READY** |
| petalTongue | **No** | 0 | ~18 | **INTERSTADIAL-READY** |
| NestGate | **No** | 0 | ~694 | Verify dyn is non-trait-object |
| rhizoCrypt | **No** | 0 | ~18 | **INTERSTADIAL-READY** |
| loamSpine | **No** | 0 | ~51 | **INTERSTADIAL-READY** (pending sled) |
| barraCuda | **No** | 0 | ~109 | **INTERSTADIAL-READY** |
| coralReef | **No** | 0 | ~137 | **INTERSTADIAL-READY** |
| skunkBat | **No** | 0 | ~18 | **INTERSTADIAL-READY** |
| sweetGrass | **No** | 0 | 2 | **INTERSTADIAL-READY** |
| BearDog | **Yes** | ~49 | ~776 | **STADIAL DEBT** |
| toadStool | **Yes** | ~158 | ~540 | **STADIAL DEBT** |

**11/13 primals have eliminated `async-trait` dep.** Two remain.

### Lockfile Ghost Debt

| Primal | `ring` in lock | `sled` debt | Other ghosts |
|--------|:--------------:|:-----------:|:------------:|
| sweetGrass | dev-only | feature-gated | `libsqlite3-sys` (phantom) |
| BearDog | **yes** | no | — |
| Songbird | **yes** | no | — |
| Squirrel | **yes** | no | `reqwest` |
| petalTongue | **yes** | no | `reqwest` |
| NestGate | **yes** | no | — |
| loamSpine | no | **yes** | `libsqlite3-sys` |
| skunkBat | no | no | — |
| biomeOS | no | no | — |
| rhizoCrypt | no | no | — |
| barraCuda | no | no | — |
| coralReef | no | no | — |
| toadStool | no | no | — |

**7/13 primals have clean lockfiles.** Six have `ring` ghosts, two have `sled` debt.

---

## Per-Primal Stadial Debt Summary

### Primals with ZERO stadial debt (interstadial-ready)

**rhizoCrypt, barraCuda, coralReef, skunkBat, biomeOS, sweetGrass** — no
async-trait, Edition 2024, deny.toml enforced. sweetGrass cleared Apr 16 via
enum dispatch (6 backend enums, `QueryEngine<S>` generic). Remaining `dyn`
usage is non-trait-object (recursive futures, dispatch tables) and does not
block the gate. sweetGrass lockfile ghosts are dev-dep or phantom only.

### Primals with lockfile-only debt

**Songbird, Squirrel, petalTongue, NestGate, loamSpine** — no `async-trait` dep,
but lockfile ghost stanzas remain. Resolution: trace the transitive puller via
`cargo tree -i ring --edges normal` and swap or remove.

### Primals with full Class 4 + lockfile debt

**BearDog** (49 attrs, ring in lock),
**toadStool** (158 attrs, clean lock). These are the stadial blockers.

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

## Upstream Cross-Talk During Stadial

The cross-talk rules from `UPSTREAM_CROSSTALK_AND_DOWNSTREAM_ABSORPTION.md` remain
in effect. Additionally during the stadial:

1. **No new `dyn Trait` introductions** — any PR adding `Box<dyn T>` or `Arc<dyn T>`
   for a finite-implementor trait is rejected
2. **No new `async-trait` usages** — any PR adding `#[async_trait]` is rejected
3. **Lockfile hygiene** — PRs must not introduce new ghost stanzas for banned crates
4. **Feature evolution is paused** — new capabilities, new RPCs, new bonding modes
   wait for the interstadial. Bug fixes and debt resolution only.

---

## When Does the Stadial End?

The stadial clears when **all 13 primals** satisfy:

- [ ] `async-trait` crate not in any `Cargo.toml`
- [ ] Zero `#[async_trait]` attributes in `.rs` files
- [ ] Zero `Box<dyn Trait>` / `Arc<dyn Trait>` for finite-implementor traits
- [ ] Zero `ring` / `sled` / `openssl` stanzas in `Cargo.lock`
- [ ] `cargo deny check bans` passes
- [ ] Edition 2024

primalSpring will track progress in `docs/PRIMAL_GAPS.md` and issue interstadial
clearance when the gate criteria are met.

---

**License**: AGPL-3.0-or-later
