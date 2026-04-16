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
| `sled` in `Cargo.lock` | sweetGrass only | Remove from default features |
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

**Ring reclassified**: ring in `Cargo.lock` is a Cargo v4 artifact — optional deps
are included in the lockfile even when their feature is not enabled. Ring is never
compiled. `cargo deny check bans` PASSES for all 13 primals. See PRIMAL_GAPS.md
for full root cause analysis.

| Primal | `ring` in lock | Compiled? | `sled` debt | Other | `cargo deny` |
|--------|:--------------:|:---------:|:-----------:|:-----:|:------------:|
| sweetGrass | yes (artifact) | **no** | feature-gated | — | **PASS** |
| BearDog | yes (artifact) | **no** | no | — | **PASS** |
| Songbird | yes (artifact) | **no** | no | — | **PASS** |
| Squirrel | **no** | **no** | no | — | **PASS** |
| petalTongue | yes (artifact) | **no** | no | `reqwest` (runtime — delegate to Songbird) | **PASS** |
| NestGate | yes (artifact) | **no** | no | — | **PASS** |
| loamSpine | yes (artifact) | **no** | no | — | **PASS** |
| skunkBat | no | **no** | no | — | **PASS** |
| biomeOS | no | **no** | no | — | **PASS** |
| rhizoCrypt | no | **no** | no | — | **PASS** |
| barraCuda | no | **no** | no | — | **PASS** |
| coralReef | no | **no** | no | — | **PASS** |
| toadStool | no | **no** | no | — | **PASS** |

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
