# Primal Gap Registry

Structured inventory of known gaps per primal that block or degrade composable deployments.
Each entry links to the composition that exposes it and proposes a fix path.

> **Scope**: NUCLEUS primals only (the 10 core primals + 3 compute/ecosystem primals).
> Downstream springs and gardens (ludoSpring, esotericWebb, etc.) own their own debt
> and are NOT tracked here. See `graphs/downstream/` for proto-nucleate patterns.
> Springs/gardens do NOT have binaries in plasmidBin — only primals do.
>
> **Evolution Model — Glacial/Stadial/Interstadial (April 16, 2026)**:
> The ecosystem evolves in phases borrowed from glacial geology:
> - **Glacial** — archived, fossilized. Old docs/code moved to `fossilRecord/`. Dead patterns.
> - **Stadial** — cold period, parity gate. **All primals must reach modern parity before
>   the next interstadial of feature evolution.** No downstream absorption until the gate clears.
> - **Interstadial** — warm period, active feature development, composition expansion,
>   spring absorption.
>
> **Current phase: STADIAL** — parity gate in effect. Gate criteria:
> 1. `dyn` dispatch + `async-trait` eliminated (Class 4 — see below)
> 2. Zero ghost debt in lockfiles (no transitive `ring`, no stale `Cargo.lock` stanzas)
> 3. All primals Edition 2024, modern async Rust, `deny.toml` enforced
> 4. No "managed" or "acceptable" exceptions for deprecated patterns
>
> The stadial clears when **13/13 primals meet all gate criteria**. Only then do
> downstream springs begin their next absorption pass.
>
> **Last updated**: 2026-04-16 — **FULL NUCLEUS REVALIDATION: 12/12 ALIVE, 19/19 PASS, 0 FAIL, 0 SKIP.**
> All 10 primals running UDS-only. `ss -tlnp | grep plasmidBin` returns **empty**.
> 7 primals modified (BearDog, Songbird, Squirrel, ToadStool, rhizoCrypt, sweetGrass, loamSpine)
> to make TCP opt-in via explicit `--port` flag. Same biomeOS graph deploys on any hardware/arch.
> TCP is opt-in only for Songbird federation (`--port 8080` enables covalent mesh).
>
> **Cross-Architecture Pixel Deployment (April 14–15)**: **14/15 exp096 checks PASS.**
> biomeOS-managed Tower (BearDog + Songbird) runs on Pixel 8a (aarch64/GrapheneOS/Titan M2).
> All critical composition gaps RESOLVED:
> - BearDog: protocol auto-detection on TCP (peek first byte: `{` = JSON-RPC, else BTSP)
> - biomeOS: TCP cascade in `primal_start_capability`, `tcp_port_registry`, TCP-aware socket wiring
> - Songbird: `tcp://` scheme parsing in IPC endpoint discovery
> - Neural API `capability.call` routes crypto/genetic/security/beacon to BearDog over TCP
> Remaining 4 failures: 3 reporting gaps (capabilities_count, transport_security, generation echo) + 1 expected (HSM/Titan M2)
>
> **Live validation (April 10 — NUCLEUS polish, session 2)**:
> - TCP ports: **0** (was 12 across 8 primals)
> - UDS sockets: **25** active in `/run/user/$UID/biomeos/`
> - C1-C7 compositions: **37/38 (97%)** — single partial: C2 `ai.query` (Ollama provider config)
> - **13/13 critical experiments ALL PASS** (exp001/002/003/004/051/069/075/077/079/089/091/092/093)
> - All 10 primals healthy over UDS (`health.liveness` OK), all `ALIVE` in launcher status
>
> **Squirrel AI provider chain (April 10)**:
> - Squirrel rebuilt with `deprecated-adapters` feature + 3 fixes:
>   1. `discovery.rs`: Accept biomeOS `primary_endpoint` field (not just `primary_socket`),
>      strip `unix://` prefix — Neural API → Songbird discovery now works.
>   2. `router.rs`: Don't register broken `local-ai` provider with HTTP URL as socket path;
>      `UniversalAiAdapter` only works with Unix sockets.
>   3. `openai.rs`: Read `OPENAI_DEFAULT_MODEL` env var (was hardcoded `gpt-4`);
>      handle OpenAI-compatible error responses before parsing as success.
> - Provider chain: Squirrel → OpenAI adapter → Songbird `http.request` → Ollama `/v1/` → tinyllama-cpu.
> - Created `tinyllama-cpu` Ollama model alias with `num_gpu=0` for CUDA-free inference.
> - C2 `ai.query` now passes (was the only C1–C7 failure).
>
> **Socket resolution evolution (April 10)**:
> - `resolve_primal_socket_with()` now has 4-tier fallback: env var → domain alias
>   (`.jsonrpc.sock` preferred) → `{primal}-{family}.sock` → `{primal}.sock` (plain).
> - Primals without `--socket` (loamSpine, sweetGrass, petalTongue) now reachable
>   via plain socket fallback — biomeOS finds `loamspine.sock` when
>   `loamspine-default.sock` doesn't exist.
> - ToadStool JSON-RPC forwarding fixed: prefers `compute-default.jsonrpc.sock`
>   over tarpc `compute-default.sock` for `capability.call`.
> - `NeuralBridge::discover()` now checks both `neural-api-{family}.sock` and
>   `biomeos-{family}.sock` — experiments find biomeOS regardless of socket naming.
>
> **biomeOS registry routing fix (April 10, completed April 15)**:
> - Root cause: `defaults.rs`, `mod.rs` (`load_from_config`), and `translation_startup.rs`
>   called `biomeos_core::family_discovery::get_family_id()` instead of using the server's
>   `--family-id` value. When `--family-id nucleus01` was passed, downstream code still
>   resolved to `"default"` sockets, causing storage/dag/spine/braid routes to fail.
> - Fix (April 15, `ad4d4490`): Added `load_defaults_for_family()` and
>   `load_from_config_for_family()` to thread the server's `family_id` through all
>   translation loading. `NeuralApiServer::load_translations_on_startup` now uses
>   `self.family_id` for defaults, config, and domain registration.
> - Graph executor: `ExecutionReport` now carries `completed_nodes` and `failed_nodes`
>   vectors, and `ExecutionStatus` in `graph.status` reports per-node success/failure.
> - Validated: `exp091` routing matrix **12/12** (up from 8/12). NestGate UDS bypass resolved (April 15).
>
> **NUCLEUS deployment patterns (April 10)**:
> - ToadStool: JSON-RPC socket separated from tarpc (`compute.jsonrpc.sock` vs `compute.sock`),
>   `--socket` CLI flag wired through to `run_server_main`, legacy symlinks for both protocols.
> - NestGate: `--socket` CLI flag added to `Commands::Server`, wired through dispatch to set
>   `NESTGATE_SOCKET` env var, feeding into `SocketConfig::from_environment()` tier-1 resolution.
> - primalSpring: BTSP client handshake module (`btsp_handshake.rs`) with HKDF-SHA256 key
>   derivation + HMAC-SHA256 challenge response matching BearDog's `crypto.rs`. Auto-detection
>   in `Transport::connect()` via `security_mode_from_env()`. Both rebuilt to plasmidBin.
>
> **BTSP Phase 2 ECOSYSTEM CASCADE (April 9–16)**: **13/13** primals enforce handshake on UDS
> accept. Songbird Wave 133→145, ToadStool S198→S203q, barraCuda Sprint 39 ↑, coralReef Iter 78→83,
> rhizoCrypt S31→S43, loamSpine, sweetGrass all wired. petalTongue Phase 2 COMPLETE (Sprint 8).
> skunkBat Phase 2 COMPLETE (v0.1.0 — `PeekedStream` UDS peek + BearDog v0.9.0 alignment).
> coralReef Phase 2 COMPLETE (Iter 78+83 — jsonrpsee removed, pure NDJSON/TCP+UDS).
> **BearDog is the sole handshake provider,
> not a consumer — its status as "already complete" is by design.**
>
> **Capability Wire Standard v1.0 (April 8)**: Convergence target defined. Flat `methods`
> array + `primal` + `version` MUST fields. 8/13 primals at L2+ (BearDog L2, Songbird L3,
> NestGate L3, ToadStool L3, Squirrel L2, rhizoCrypt L3, loamSpine L2/L3, sweetGrass L3).
> barraCuda L2. petalTongue L2/L3. coralReef L2 ↑. skunkBat L2 ↑. sourDough/bingoCube: NONE (CLI tools).
>
> **plasmidBin (April 10)**: ~~`doctor.sh --quick` reports 9/11 DYNAMIC~~ **RESOLVED** —
> full `--target x86_64-unknown-linux-musl` rebuild. 12/12 static, stripped, ecoBin compliant.
>
> **Trio witness evolution (April 7)**: `WireAttestationRef` → `WireWitnessRef`.
> Self-describing `kind`/`encoding`/`algorithm`/`tier`/`context` fields. Trio harvested
> to plasmidBin (glibc → musl). See `wateringHole/handoffs/PRIMALSPRING_TRIO_WITNESS_HARVEST_HANDOFF_APR07_2026.md`.

---

## Portability Debt Registry (April 11, 2026)

Cross-cutting non-portable dependencies that violate the ecoBin "pure Rust, universal
binary" principle. Organized by severity class. Each class follows the same resolution
pattern: **identify → centralize in one primal → delegate via IPC → ban in consumers**.

### Class 1: C Crypto — SOLVED (Tower Atomic Delegation)

`ring` (C/ASM crypto) blocked musl cross-compile and single-target builds. Solution:
BearDog provides RustCrypto in-process (pure Rust). Other primals delegate crypto
to BearDog via JSON-RPC IPC. `deny.toml` bans `ring`, `openssl`, `aws-lc-sys`
ecosystem-wide. This established the delegation pattern.

| Primal | Had | Replaced With | Pattern |
|--------|-----|---------------|---------|
| Songbird | `ring` (C/ASM TLS) | `rustls_rustcrypto` + BearDog IPC | Tower Atomic delegation |
| NestGate | `aws-lc-rs` / `ring` | System `curl` (TLS) + BearDog IPC (crypto) | Delegation + system bridge |
| barraCuda | Banned in deny.toml | Never had — preemptive ban | Policy |
| Squirrel | `libloading` (FFI) | Removed (alpha.46) | Direct elimination |

**Class 1 COMPLETE (April 11, hardened April 16)**: NestGate NG-08 **RESOLVED** —
eliminated `reqwest`, switched to `ureq` + `rustls-no-provider` + `rustls-rustcrypto`.
`cargo tree -i ring` returns empty across all 13 primals. **13/13 primals are ring-free
in builds.** **Stadial policy (April 16)**: ghost entries in `Cargo.lock` are no longer
"managed" — they are debt. Songbird still has `ring` 0.17.14 as a transitive lockfile
stanza; this must be eliminated (trace puller, swap or remove).

### Class 2: GPU/Vulkan Dynamic Linking — RESOLVED (Node Atomic Delegation)

The same class of problem as ring but for compute hardware. The dependency chain:

```
wgpu 28.0.0  →  wgpu-hal 28.0.1  →  ash 0.38.0 (Vulkan bindings)
                                   →  metal 0.33.0 (Apple)
                                   →  windows-rs (DX12)
                                   →  renderdoc-sys

ash 0.38.0   →  libloading 0.8.9  →  dlopen(libvulkan.so.1)  ← FAILS on musl-static
```

**Why musl-static breaks**: musl's `dlopen` implementation cannot load glibc-linked
shared objects. `libvulkan.so.1` (and all GPU ICDs) require glibc. Therefore ecoBin
musl-static binaries can **never** access GPU hardware through the wgpu path.
This is not a bug — it's a fundamental incompatibility between static linking and
dynamic GPU driver loading.

**Affected primals** (compile-time wgpu dependency):

| Primal | wgpu Version | Feature-Gated? | Impact |
|--------|-------------|----------------|--------|
| barraCuda | 28.0.0 | `gpu` feature (default ON) | ecoBin binary always CPU-only |
| toadStool | 22.0.0 | `wgpu` feature (optional) | GPU features unavailable in ecoBin |
| petalTongue | via eframe/egui | Inherent to GUI | Headless mode avoids; acceptable |

**Existing abstractions (partial solutions)**:

| Abstraction | Location | Status | What It Does |
|------------|----------|--------|--------------|
| `GpuBackend` trait | `barraCuda/device/backend.rs` | Done | Backend-agnostic compute interface (9 required methods) |
| `WgpuDevice` | `barraCuda/device/wgpu_backend.rs` | Done | Implements `GpuBackend` via wgpu (needs dlopen — non-portable) |
| `SovereignDevice` | `barraCuda/device/sovereign_device.rs` | Wired | Implements `GpuBackend` via IPC to coralReef+toadStool (portable) |
| `CpuExecutor` | `barraCuda/unified_hardware/cpu_executor.rs` | Done | Native Rust CPU math execution |
| `cpu-shader` + `naga-exec` | `barracuda-naga-exec` crate | **Done** | Default-on (BC-08 resolved Sprint 40). Interprets WGSL shaders on CPU via naga IR |
| `Auto::new()` | `barraCuda/device/mod.rs` | **Done** | 3-tier fallback: GPU → CPU software rasterizer → SovereignDevice IPC → `Err` (BC-07 resolved Sprint 41) |
| `coral-gpu` | `coralReef/crates/coral-gpu/` | In progress | Sovereign GPU compute — replaces wgpu for compute. No wgpu dependency in production |

**The resolution pattern (Node Atomic Delegation)** mirrors Tower Atomic:

| Tower (SOLVED) | Node (TO SOLVE) |
|----------------|-----------------|
| BearDog: pure Rust crypto | barraCuda: pure Rust math (WGSL) |
| Songbird: TLS via BearDog IPC | barraCuda: GPU via toadStool+coralReef IPC |
| Consumer delegates crypto | Consumer delegates compute dispatch |
| `deny.toml` bans `ring` | Future: `deny.toml` bans direct `wgpu` in consumers |

**Gaps to close** (mapped to BC-06/07/08):

- **BC-06**: Architectural constraint — document, don't fix musl. ecoBin = CPU-only for wgpu path.
- ~~**BC-07**: Wire `SovereignDevice` into `Auto::new()` fallback chain~~ **RESOLVED** (Sprint 41) — `Auto::new()` returns `DiscoveredDevice` with 3-tier fallback (wgpu GPU → wgpu CPU → SovereignDevice IPC → Err).
- ~~**BC-08**: Make `cpu-shader` feature default-on~~ **RESOLVED** (Sprint 40) — `cpu-shader` in `default = ["gpu", "domain-models", "cpu-shader"]`.

**Target state**: barraCuda computes on **any** hardware:
1. wgpu GPU (development, glibc hosts with GPU) — fastest
2. SovereignDevice IPC (NUCLEUS deployment, coralReef+toadStool available) — GPU via IPC
3. cpu-shader/naga-exec (ecoBin, Docker, no peers) — CPU WGSL interpretation
4. Scalar Rust (absolute minimum, no naga) — native f64 fallback

### Class 3: Remaining C Surfaces — PARTIAL

| ID | Primal | Dependency | Severity | Production? | Status |
|----|--------|-----------|----------|-------------|--------|
| NG-08 | NestGate | `ring` v0.17.14 via `rustls` → `reqwest` | **RESOLVED** | Eliminated: `reqwest` → `ureq` + `rustls-rustcrypto` | Session 42 — `cargo tree -i ring` empty, `cargo deny check bans` PASS |
| CR-01 | coralReef | Missing `deny.toml` C/FFI ban list | **RESOLVED** | Iter 79 — full ecoBin v3 ban list added: `ring`, `openssl`, `native-tls`, `aws-lc-sys`, `cmake`, `pkg-config`, etc. |
| CR-02 | coralReef | `cudarc` (CUDA FFI) | Low | Feature-gated (`cuda`) | Acceptable — sovereign path (`coral-gpu`) is pure Rust |
| SG-01 | sweetGrass | `ring` via testcontainers → bollard → rustls | Low | **No** — dev-deps only | Acceptable — does not affect ecoBin binary |
| SB-02 | Songbird | `ring-crypto` opt-in feature | Low | **No** — opt-in, not default | Acceptable — default path uses `rustls_rustcrypto` |
| PT-12 | petalTongue | eframe/egui/glow (OpenGL/Vulkan GUI) | Low | Only in GUI mode | Acceptable — headless (`PETALTONGUE_HEADLESS=true`) avoids |
| TS-03 | toadStool | `wgpu`/`ash`/`vulkano`/`wasmtime`/`esp-idf-sys` | Low | All feature-gated | Acceptable — core crate does not require wgpu by default |
| BD-01 | bearDog | `ndk-sys`/`security-framework-sys` | Low | Target-gated (Android/macOS) | Acceptable — Linux ecoBin unaffected |

### Ring Lockfile Ghost — Definitive Root Cause Analysis (April 16, 2026)

**Summary**: Ring appears in 6 primal lockfiles. It is **never compiled**, never
linked, never flagged by `cargo deny check bans`. It is a Cargo lockfile v4 artifact.

#### Why ring is in `Cargo.lock`

Cargo v4 lockfiles include optional dependencies even when their feature is not
enabled. Any crate in the dep tree that lists `ring` as an optional dep causes ring
to appear in `Cargo.lock`, regardless of whether the `ring` feature is activated.

Packages that list ring as optional (per-primal):

| Primal | Packages with `ring` as optional dep |
|--------|--------------------------------------|
| Songbird | `hickory-proto 0.24`, `rustls 0.23`, `rustls-webpki 0.102+0.103`, `x509-parser 0.16` |
| sweetGrass | `rustls 0.23`, `rustls-webpki 0.103` + `rustls-native-certs` (non-optional, dev-dep chain) |
| petalTongue | `rustls 0.23`, `rustls-webpki 0.103` (via `reqwest → hyper-rustls`) |
| loamSpine | `hickory-net 0.26`, `hickory-proto 0.26` |
| BearDog | `hickory-proto 0.24`, `x509-parser 0.16` |
| NestGate | `rustls 0.23`, `rustls-webpki 0.103`, `x509-parser 0.17` |

**Ring is NOT in the resolve graph** for BearDog (confirmed: `cargo metadata` shows
zero resolve nodes for `ring@0.17`). For the other 5, ring appears in resolve metadata
but with no active feature path — `cargo tree -i ring` returns empty for all.

#### Why vendoring doesn't eliminate the lockfile entry

NestGate vendors `rustls-rustcrypto` with `rustls-webpki = { version = "0.103.12",
default-features = false }`. This prevents ring from being a *default* feature of
webpki, but ring remains in the lockfile because `rustls-webpki 0.103.12` still
lists ring as an *optional* dep. Cargo v4 includes optional deps in the lockfile
regardless of activation.

#### Definitive assessment

**Ring cannot be removed from `Cargo.lock`** without eliminating it as an optional dep
from all upstream crates (`rustls`, `rustls-webpki`, `hickory-proto`, `x509-parser`).
This requires upstream changes to the Rust TLS/DNS ecosystem — not actionable at the
primal level.

#### Stadial gate reclassification

The ring lockfile ghost is **not a stadial gate criterion**. The actual criteria:

1. `cargo deny check bans` passes (ring not compiled) — **all 13 primals PASS**
2. No direct `ring` dep in any primal `Cargo.toml` — **all 13 primals PASS**
3. No feature flag enables `ring` in any primal — **all 13 primals PASS**

The lockfile text is cosmetic. The deny check is the enforcement.

#### Tower Atomic delegation pattern (active resolution)

**petalTongue** is the one primal where the ring chain comes from an actual runtime
dep (`reqwest → hyper-rustls → rustls → ring`). While ring isn't compiled (feature
not enabled), `reqwest` itself represents an architectural concern: individual primals
should not maintain their own HTTP/TLS stack.

**Fix**: petalTongue delegates outbound HTTP/TLS to Songbird (tower atomic TLS
provider) via IPC. This eliminates `reqwest` entirely and with it the entire rustls
chain. BearDog provides crypto operations. No primal except Songbird and BearDog
should carry TLS or crypto dependencies.

| Primal | `ring` in lockfile | `cargo deny` PASS | Compiled | Action |
|--------|:------------------:|:-----------------:|:--------:|--------|
| sweetGrass | yes | **PASS** | **no** | Lockfile artifact — cosmetic |
| BearDog | yes | **PASS** | **no** | Lockfile artifact — not even in resolve |
| Songbird | yes | **PASS** | **no** | Lockfile artifact — Songbird IS the TLS provider |
| petalTongue | yes | **PASS** | **no** | Delegate HTTP to Songbird, eliminate `reqwest` |
| NestGate | yes | **PASS** | **no** | Lockfile artifact — vendored rustls-rustcrypto |
| loamSpine | yes | **PASS** | **no** | Lockfile artifact — hickory optional dep |
| Squirrel | no | **PASS** | **no** | Clean |
| toadStool | no | **PASS** | **no** | Clean |
| biomeOS | no | **PASS** | **no** | Clean |
| rhizoCrypt | no | **PASS** | **no** | Clean |
| barraCuda | no | **PASS** | **no** | Clean |
| coralReef | no | **PASS** | **no** | Clean |
| skunkBat | no | **PASS** | **no** | Clean |

**13/13 pass `cargo deny check bans`. 0/13 compile ring. Ring lockfile ghost is a
Cargo v4 artifact, not actionable ecosystem debt.**

### Class 4: `dyn` Dispatch + `async-trait` — DEPRECATED (Stadial Gate)

**Policy (April 16, 2026)**: `dyn` dispatch and `async-trait` are **ecosystem-deprecated**,
following the same lifecycle as `ring` in Class 1. There are no "dyn ceilings" or
"object-safety exceptions" — every `Box<dyn Trait>` / `Arc<dyn Trait>` with a finite
implementor set is replaced by enum dispatch. Every `#[async_trait]` is replaced by
native `async fn` in traits (RPITIT, Edition 2024). The `async-trait` crate is removed
from `Cargo.toml`. This is a **stadial parity gate** — no downstream springs absorb
until all primals reach modern async Rust parity.

**Resolution pattern (same as Class 1)**:
**audit → enumerate implementors → create dispatch enum → migrate to native AFIT →
drop `async-trait` dep → ban in `deny.toml`.**

**Ecosystem-wide modernization matrix**:

| Primal | `#[async_trait]` | `async-trait` dep | Status |
|--------|:----------------:|:-----------------:|--------|
| Songbird | **0** | **No** | **COMPLETE** (Wave 145: 141→0) |
| Squirrel | **0** | **No** | **COMPLETE** (228→0, ring+reqwest lockfile ghosts eliminated) |
| biomeOS | **0** | **No** | **COMPLETE** (72→0) |
| petalTongue | **0** | **No** | **COMPLETE** (Sprint 8: 47→0, dyn elimination) |
| NestGate | **0** | **No** | **COMPLETE** |
| rhizoCrypt | **0** | **No** | **COMPLETE** (S43) |
| loamSpine | **0** | **No** | **COMPLETE** (sled+sqlite backends removed) |
| barraCuda | **0** | **No** | **COMPLETE** |
| coralReef | **0** | **No** | **COMPLETE** (Iter 83: jsonrpsee removed) |
| skunkBat | **0** | **No** | **COMPLETE** (Phase 44: 14→0, generics+RPITIT, dep removed) |
| sweetGrass | **0** | **No** | **COMPLETE** (stadial pass: BraidBackend enum dispatch, RPITIT, dep removed) |
| toadStool | **~158** | Yes | **STADIAL DEBT** — 32 traits, all with finite implementors, enum dispatch feasible |
| BearDog | **49** | Yes | **STADIAL DEBT** — ~18 traits, most with ≤6 implementors, 2 intentionally open |

**11/13 primals at zero.** Two remain: toadStool (158), BearDog (49).

**Resolution guidance**:
- `Box<dyn Trait>` / `Arc<dyn Trait>` with finite implementors → **enum dispatch**
- `#[async_trait]` on trait def → **native `async fn`** or `fn ... -> impl Future<...> + Send`
- `#[async_trait]` on impl block → **remove** (native async works on concrete types)
- `Box<dyn Error>` → `thiserror` enum or `anyhow::Error`
- `#[allow(...)]` → `#[expect(..., reason = "...")]`
- Drop `async-trait` from Cargo.toml once all usages in the crate are removed
- For traits with genuinely **unbounded** implementors (plugin registries where
  external crates may impl): use generics + monomorphization at construction site,
  or `ErasedProvider`-style type erasure without `async-trait`

**Why this is a gate, not a nice-to-have**: `async-trait` desugars to
`Pin<Box<dyn Future>>` — heap allocation per async call. Native async fn compiles
to zero-cost state machines. For IPC-heavy primals, this is measurable overhead.
Removing dyn dispatch enables monomorphization → smaller, faster ecoBins. And
critically: `async-trait` pulls `syn` (proc-macro), inflating compile times
across the entire dependency graph.

---

## Cross-Spring Upstream Gap Synthesis (April 11, 2026)

Consolidated from April 11 handoffs across all 7 science springs. These are gaps
that multiple springs independently report as blocking their composition evolution.
Each maps to a specific primal team for resolution.

### Recurring Blockers (reported by 3+ springs)

| Gap | Affected Springs | Owner | Status |
|-----|-----------------|-------|--------|
| **BearDog BTSP server endpoint** — springs need `btsp.server.*` RPC surface | hotSpring, healthSpring, neuralSpring, ludoSpring | **BearDog team** | **RESOLVED** — `btsp.server.create_session`, `.verify`, `.negotiate`, `.status` wired with `BtspSessionStore` (session_store.rs). Legacy `btsp.session.*` aliases maintained. Springs can now connect |
| **Ionic bond runtime** — `crypto.ionic_bond` / cross-family GPU lease / data egress fence | hotSpring (GAP-HS-005), healthSpring (§2), ludoSpring | **BearDog team** | **RESOLVED** — Wave 42: `crypto.ionic_bond.seal` completes propose→accept→seal lifecycle with real Ed25519 verification at each step. Proposal TTL enforcement on accept. In-memory only by design — persistent bonds via NestGate/loamSpine. 100 JSON-RPC methods |
| **Canonical inference namespace** — springs accept `inference.*` / `model.*` / `ai.*` inconsistently | healthSpring (§4), neuralSpring (Gap 1), ludoSpring (GAP-10) | **primalSpring + Squirrel + neuralSpring** | **RESOLVED** — Songbird Wave 134 declares `inference.*` as canonical with `model.*` / `ai.*` absorption aliases |
| ~~**TensorSession adoption** — fused multi-op GPU pipelines; springs defer because API unstable~~ | hotSpring (GAP-HS-027), healthSpring, wetSpring | **barraCuda team** | **RESOLVED** — Sprint 40: renamed to `BatchGuard`, migration guide published in `BREAKING_CHANGES.md` (§TensorSession/BatchGuard Migration Guide). Sprint 42: `tensor.batch.submit` IPC method wired (fused multi-op pipeline over JSON-RPC). Spring-side adoption is coordination work |
| **Provenance trio IPC stability** — trio endpoints panic, TCP-only, or unreachable | wetSpring (PG-02), ludoSpring, healthSpring | **rhizoCrypt + loamSpine + sweetGrass teams** | **RESOLVED** — All three now have TCP_NODELAY + flush-after-write on all TCP/UDS paths. rhizoCrypt (S33-34): TCP_NODELAY+flush, +31 tests, feature narrowing. loamSpine: dedicated UDS transport (uds.rs), constants centralization, 8×5 concurrent load test. sweetGrass: BTSP mock BearDog tests, Postgres error-path coverage, module splits. Trio IPC is stable |
| **NestGate storage IPC** — `storage.retrieve` / persistent cross-spring data | wetSpring (PG-04), neuralSpring (Gap 5), healthSpring | **NestGate team** | **RESOLVED** — `storage.store` + `storage.retrieve` implemented on UDS JSON-RPC. Family-scoped socket symlinks (`storage[-{fid}].sock` → `nestgate[-{fid}].sock`) for capability discovery. Integration tests cover socket-level storage round-trips. Springs can discover and use via standard IPC |
| **`capability.resolve` / capability-first discovery** — springs want to route by capability, not primal name | wetSpring (PG-03), healthSpring (§3), all springs | **biomeOS + Songbird** | **RESOLVED** — Songbird Wave 134 implements `capability.resolve` (single best endpoint), `lifecycle.validate_consumed`, `lifecycle.composition`, canonical `ipc.discover` aliases, and `inference.*` canonical namespace |

### Per-Primal Upstream Tasks (from spring handoffs)

**barraCuda** (reported by: hotSpring, neuralSpring, groundSpring, airSpring, ludoSpring)

| Task | Source | Priority |
|------|--------|----------|
| ~~BC-07: Wire `SovereignDevice` into `Auto::new()` fallback~~ | primalSpring benchScale audit | ~~Medium~~ **RESOLVED** (Sprint 41) |
| ~~BC-08: Make `cpu-shader` default-on~~ | primalSpring benchScale audit | ~~Medium~~ **RESOLVED** (Sprint 40) |
| ~~`TensorSession` stabilization for spring adoption~~ | hotSpring GAP-HS-027, healthSpring | ~~Medium~~ **RESOLVED** (Sprint 40 rename + migration guide in BREAKING_CHANGES.md, Sprint 42 `tensor.batch.submit` IPC) |
| `plasma_dispersion` feature-gate bug (`domain-lattice` required) | neuralSpring Gap 9 | Low |
| 29 shader absorption candidates from neuralSpring | neuralSpring Gap 10 | Low |
| ~~RAWR GPU kernel (currently CPU-only `stats::rawr_mean`)~~ | groundSpring | ~~Low~~ **RESOLVED** — `RawrWeightedMeanGpu` + `rawr_weighted_mean_f64.wgsl` GPU shader in `barracuda/src/ops/`. CPU `rawr_mean` in `stats/bootstrap.rs`. Both paths working |
| Batched `OdeRK45F64` for Richards PDE | airSpring evolution_gaps | Low |

**coralReef** (reported by: neuralSpring, hotSpring, healthSpring)

| Task | Source | Priority |
|------|--------|----------|
| CR-01: Add `deny.toml` C/FFI ban list | primalSpring portability audit | **RESOLVED** (Iter 79 — deny.toml with ecoBin v3 C/FFI ban, cudarc behind feature gate) |
| Multi-stage ML pipeline support via `shader.compile.wgsl` | neuralSpring handoff | **RESOLVED** (Iter 80+ — 6 end-to-end pipeline composition tests, CompilationInfo IPC) |
| IPC timing for `shader.compile` in deployment | neuralSpring, healthSpring | Low |

**toadStool** (reported by: wetSpring, neuralSpring, airSpring)

| Task | Source | Priority |
|------|--------|----------|
| Stable `compute.dispatch.submit` / `compute.execute` IPC | wetSpring PG-05, neuralSpring | **RESOLVED** (S199) |
| Pipeline scheduling for ordered dispatch | neuralSpring handoff | **RESOLVED** (S199 — `compute.dispatch.pipeline.submit` with DAG validation, topological execution, status) |

**NestGate** (reported by: wetSpring, neuralSpring, healthSpring)

| Task | Source | Priority |
|------|--------|----------|
| NG-08: Eliminate `ring` from production build | primalSpring portability audit | **RESOLVED** (Session 43 — reqwest→ureq 3.3 + rustls-rustcrypto, ring/openssl/aws-lc-rs fully eliminated) |
| `storage.retrieve` for large/streaming tensors | neuralSpring, wetSpring PG-04 | Medium |
| Cross-spring persistent storage IPC | healthSpring, wetSpring | Medium |

**BearDog** (reported by: hotSpring, healthSpring, neuralSpring, ludoSpring)

| Task | Source | Priority |
|------|--------|----------|
| BTSP server endpoint (`btsp.server.*`) | healthSpring §10, hotSpring GAP-HS-006 | **RESOLVED** (Wave 36 — `btsp.server.create_session`, `.verify`, `.negotiate`, `.status`) |
| Ionic bond runtime (`crypto.ionic_bond`) | hotSpring GAP-HS-005, healthSpring §2 | **RESOLVED** (Wave 42 — propose→accept→seal with Ed25519, proposal TTL) |
| Signed capability announcements | neuralSpring handoff | **RESOLVED** (Wave 45 — SA-01: Ed25519 signed attestation on discover + capability.register) |

**Squirrel** (reported by: neuralSpring, healthSpring, ludoSpring)

| Task | Source | Priority |
|------|--------|----------|
| `inference.register_provider` wire method | neuralSpring Gap 1 | **RESOLVED** (alpha.49 — 5 wire tests, real handler path) |
| Stable ecoBin binary for composition deployments | healthSpring §9 | **RESOLVED** (alpha.49 — 3.5MB static-pie, stripped, BLAKE3, zero host paths) |
| SQ-04: `--bind` flag / `SQUIRREL_IPC_HOST` for Docker TCP | primalSpring benchScale exp077 | **RESOLVED** (alpha.52 — `--bind` CLI flag, `SQUIRREL_BIND`/`SQUIRREL_IPC_HOST` env vars, default `127.0.0.1`, Docker uses `--bind 0.0.0.0`) |

**biomeOS / Songbird** (reported by: wetSpring, healthSpring, ludoSpring)

| Task | Source | Priority |
|------|--------|----------|
| `capability.resolve` single-step routing | wetSpring PG-03, healthSpring §3 | Medium |
| Deploy-time `consumed_capabilities` completeness check | wetSpring V143 handoff | Low |
| `lifecycle.composition` for live dashboards | ludoSpring handoff | Low |

### Primal Evolution Summary (April 11, cross-primal review)

| Primal | Key Evolution Since Handoff | Resolved Gaps | Remaining |
|--------|---------------------------|---------------|-----------|
| **barraCuda** | Sprint 39-41: **BC-07 RESOLVED** — `Auto::new()` returns `DiscoveredDevice` with 3-tier fallback (wgpu GPU → wgpu CPU → SovereignDevice IPC). BC-06 documented (README deployment matrix). TensorSession migration guide in BREAKING_CHANGES.md. Capability-based naming (no hardcoded primal names) | BC-05, BC-06, BC-07, BC-08, TensorSession | Sovereign pipeline readback, DF64 NVK verification, coverage →90% |
| **coralReef** | Iter 79-79c: **CR-04 RESOLVED** (Wave 4 complete, zero `Result<_, String>` in production driver), **CR-05 RESOLVED** (cpu_exec.rs deleted), deny.toml bans, IPC latency doc, `#[allow]` audit, 4,467 tests | CR-01, CR-04, CR-05 | Transitive libc (deferred until mio→rustix, mio#1735) |
| **BearDog** | Wave 34-35: **Real Ed25519 signing** on ionic bond propose+accept, placeholder elimination, real `/proc` metrics, self-knowledge module split, BTSP server live | BTSP server, ionic bond signatures (real Ed25519 verify) | Bond persistence (NestGate/loamSpine), HSM/BTSP Phase 3 signing |
| **NestGate** | Session 35-42: NG-08 ring eliminated (ureq + rustls-rustcrypto), **storage.store/retrieve on UDS** with family-scoped symlinks, ZFS bridge (7 `zfs.*` methods, GAP-MATRIX-04), BTSP Phase 1+2 (server handshake wired), Wire L3 capabilities.list + identity.get, `fetch_external` → Tower Atomic, `#[serial]` eliminated, 11,856 tests ~80% cov | NG-08, storage IPC, ZFS bridge, BTSP Phase 2 | Doc drift (57 methods in STATUS vs 41 in code const), `data.*` capability inconsistency, coverage 80→90%, 181 deprecated APIs to clean |
| **toadStool** | S199-202: pipeline dispatch stable, capability-based naming (`coral_reef_available` → `shader_compiler_available`), +46 tests, dispatch refactor | PG-05 (dispatch IPC), pipeline scheduling (S199) | D-COVERAGE-GAP (83.6→90%), V4L2 ioctl, async/dyn markers |
| **Songbird** | Wave 134-136: `capability.resolve`, `inference.*` canonical, CI-01 `cargo deny`, **SB-02 ring-crypto removed**, **SB-03 sled eliminated**, canonical constants | SB-02, SB-03, capability.resolve, inference namespace, CI-01 | QUIC/TLS evolution, transitive `ring` in lockfile (not compiled) |
| **Provenance Trio** | **All three now have TCP_NODELAY + flush-after-write.** rhizoCrypt S33-34: +31 tests, feature narrowing, primal-agnostic naming, BTSP types module, service_types split. 1,502 tests ~93% cov. loamSpine: dedicated UDS transport (uds.rs), constants.rs centralization, 8×5 concurrent load test. 1,442 tests ~92% cov, **178** source files, **stadial gate** (sled + sqlite storage out; hickory-resolver 0.26). sweetGrass: BTSP mock BearDog test pattern, Postgres error-path tests (no Docker), module splits (braids/health/config), sled clone reduction. 1,315 tests ~87% cov | Trio IPC stability (TCP_NODELAY+flush), constants centralization, BTSP types | sweetGrass Postgres full-path (needs Docker CI), sweetGrass coverage 87→90% |
| **biomeOS** | v3.01-3.03: **`capability.resolve` implemented** (single-step routing), **`lifecycle.composition`** dashboard, **`consumed_capabilities` validation** in graph loader, full **`inference.*` routing** (7 methods incl `register_provider`), anyhow evolution, `#[expect]` migration, hot-path clone elimination. 7,749 tests | capability.resolve, lifecycle.composition, inference.*, consumed_capabilities | Songbird mesh state, gate2/Pixel deploy validation |
| **petalTongue** | Sprint 5: **PT-06 RESOLVED** (push delivery wired on server startup), 9 new test modules (IPC handlers, provenance trio, engine, animation, audio, SVG, neural graph, primal details), anyhow removed from all production deps, `#[expect]` migration, self-knowledge constants gated, hot-path allocation reduction. ~2,277 tests ~90% cov. **BTSP Phase 2 WIRED** (Apr 15): real BearDog handshake delegation on UDS+TCP, TCP first-byte peek for biomeOS bypass | PT-06 (push delivery activated), PT-08 (BTSP Phase 1), **PT-09 (BTSP Phase 2 WIRED)** | 6 files >700 LOC |

### Full Ecosystem Revalidation (April 12, 2026)

**ecoBin Harvest**: All 13 primals rebuilt as musl-static x86_64, harvested to plasmidBin.

| Primal | ecoBin | Size | Tests (lib+bins) | Pass | Fail | Status |
|--------|--------|------|-----------------|------|------|--------|
| **barraCuda** | static-pie, stripped | 6.8M | 3,849 | 3,835 | 14 | ESN v2 model + tensor scalar failures (99.6% pass) |
| **coralReef** | static-pie, stripped | 6.5M | 25 | 25 | 0 | CLEAN |
| **BearDog** | static-pie, stripped | 7.2M | 409 | 408 | 1 | Minor (99.8% pass) |
| **Songbird** | static-pie, stripped | 17M | — | — | — | Compile error in `songbird-orchestrator` test (3 type mismatches). Binary builds fine |
| **NestGate** | static, stripped | 7.9M | 2,175 | 2,172 | 3 | Minor (99.9% pass) |
| **toadStool** | static-pie, stripped | 11M | 178 | 178 | 0 | CLEAN |
| **Squirrel** | static-pie, stripped | 4.5M | 666 | 666 | 0 | CLEAN |
| **biomeOS** | static, stripped | 13M | 22 | 22 | 0 | CLEAN |
| **rhizoCrypt** | static-pie, stripped | 5.6M | 875 | 873 | 2 | Minor (99.8% pass) |
| **loamSpine** | static-pie, stripped | 4.6M | 6 | 6 | 0 | CLEAN |
| **sweetGrass** | static-pie, stripped | 5.8M | 57 | 57 | 0 | CLEAN |
| **petalTongue** | static-pie, stripped | 26M | 173 | 172 | 0 | CLEAN (1 ignored) |
| **skunkBat** | static-pie, stripped | 2.2M | 84 | 81 | 3 | Minor (96.4% pass) |
| **primalSpring** | static-pie, stripped | 1.9M | 431 | 426 | 5 | Minor (98.8% pass) |

**Note**: Test counts above are `--lib --bins` only (unit + lib tests). Full `--all-targets` counts are higher
(e.g. barraCuda 3,849 here vs ~8,000+ with integration tests; see per-primal docs for full counts).

**Test failures to investigate**:
- **barraCuda**: 14 ESN v2 model tests + tensor scalar ops — likely numerical precision or initialization
- **Songbird**: `songbird-orchestrator` has 3 `E0308` type mismatches in test code — needs test update
- **primalSpring**: 5 failures in composition experiments — likely stale expected values after primal evolution
- **Others**: 1-3 failures each, minor, not blocking deployment

### Spring Evolution Status (April 12, 2026)

Springs do NOT ship binaries to plasmidBin. "Niche Defined" means the spring has
a `NICHE_*` composition entry in `plasmidBin/ports.env` for its primal requirements.

| Spring | Version | Stage | Deploy Graphs | Tests | barraCuda | deny.toml | Niche Defined? |
|--------|---------|-------|---------------|-------|-----------|-----------|----------------|
| **hotSpring** | v0.6.32 | composing | 1 (QCD deploy) | 4,422+ | 0.3.11 (git rev) | **Missing** | Yes — niche-hotspring |
| **neuralSpring** | v0.1.0 / S181 | composing | 1 (inference deploy) | many | 0.3.11 (path) | Weak (no bans) | Yes — niche-neuralspring |
| **wetSpring** | V143 | composing | 7 (deploy + workflows) | 1,950 | 0.3.11 (pinned) | Good (openssl banned) | Yes — niche-wetspring |
| **healthSpring** | V52 / 0.8.0 | composing | 7 (deploy + workflows) | 985+ | 0.3.11 (rev pin) | Good (ring exception for rustls) | Yes — niche-healthspring |
| **airSpring** | v0.10.0 | composing | 5 (deploy + pipelines) | 1,364 | 0.3.11 (path) | Present | Yes — niche-airspring |
| **groundSpring** | V124 | composing | 6 (deploy + validation) | many | 0.3.11 (path) | Present | Yes — niche-groundspring |
| **ludoSpring** | V41 | composing | (via primalSpring) | — | (via barraCuda) | — | Yes — pure composition |

### Spring deny.toml Compliance

| Spring | deny.toml? | `ring` banned? | `openssl` banned? | Notes |
|--------|-----------|---------------|-------------------|-------|
| hotSpring | **No** | N/A | N/A | **Gap: needs deny.toml** |
| neuralSpring | Yes (weak) | **No** | **No** | Only license/advisory checks; **no C/FFI bans** |
| wetSpring | Yes | **No** | **Yes** | Bans openssl + sys crates; ring not explicitly banned |
| healthSpring | Yes | **Exception** | **Yes** | ring allowed as rustls wrapper; explicit evolution note |
| airSpring | Yes | Unknown | Unknown | Present but not fully audited |
| groundSpring | Yes | Unknown | Unknown | Present but not fully audited |

### Composition Validation Library Evolution (April 12, 2026)

primalSpring now provides a **composition parity validation toolkit** so downstream
springs can validate their domain science as primal compositions without understanding
primal internals. This is the bridge from "Rust validation" to "primal composition
validation" — Level 5 on the maturity ladder.

**New modules and APIs** (ecoPrimal v0.8.0+):

| Module | What It Provides |
|--------|-----------------|
| `composition::CompositionContext` | Capability-keyed IPC client set — abstracts socket discovery, primal names, JSON-RPC responses |
| `composition::validate_parity` | One-call scalar comparison: local baseline vs primal composition result |
| `composition::validate_parity_vec` | Element-wise vector comparison for tensor/array results |
| `validation::check_composition_parity` | Lower-level: user-supplied extractor closure for custom response schemas |
| `ipc::client::call_extract_f64` | Typed extraction: call + extract scalar by key from JSON-RPC result |
| `ipc::client::call_extract_vec_f64` | Typed extraction: call + extract array by key from JSON-RPC result |
| `ipc::client::call_extract<T>` | Generic typed extraction via `DeserializeOwned` |
| `tolerances::CPU_GPU_PARITY_TOL` | Named f64 tolerance for CPU vs GPU divergence (1e-10) |
| `tolerances::WGSL_SHADER_TOL` | Named f64 tolerance for f32 shader vs f64 baseline (1e-6) |
| `tolerances::STOCHASTIC_SEED_TOL` | Named f64 tolerance for seeded PRNG divergence (1e-6) |
| `tolerances::DF64_PARITY_TOL` | Named f64 tolerance for df64 emulated precision (1e-14) |

**AtomicType alignment** (corrected):
- `Node` now includes barraCuda + coralReef (5 primals, was 3) with `tensor` + `shader` capabilities
- `FullNucleus` now includes 11 primals (added barraCuda, coralReef, petalTongue) with `visualization` capability
- `Nest` unchanged (storage-focused, no compute)

**Remaining upstream gaps for composition validation**:

| Gap | Owner | What Springs Need | Status |
|-----|-------|-------------------|--------|
| `tensor.matmul` / `tensor.dot` response schema | barraCuda | Standardized result key (`"value"` or `"result"`?) for typed extraction | **RESOLVED** — Sprint 42: `TENSOR_WIRE_CONTRACT.md` v1.0.0. Category 1 (tensor-producing): `result_id` + `shape`. Category 2 (scalar): `value`. Category 3 (batch): `tensor.batch.submit` with aliased ops |
| `shader.compile` response schema | coralReef | Standardized result format for shader compilation output | **RESOLVED** — Iter 80: `SHADER_COMPILE_WIRE_CONTRACT.md`. `binary` (base64) + `size` + `arch` + `status` + `info` (gpr_count, instr_count, shared_mem_bytes, workgroup_size) |
| `compute.dispatch` result schema | toadStool | Standardized result format for dispatch outcomes | **RESOLVED** — S203: `DISPATCH_WIRE_CONTRACT.md`. Standard envelope: `{domain, operation, job_id, status, output, error, metadata}` for all 8 dispatch variants |
| BatchGuard / TensorSession adoption | barraCuda | Fused multi-op pipeline results via IPC (not just per-op calls) | **RESOLVED** — Sprint 42: `tensor.batch.submit` with aliased op chaining (create → matmul → relu → readback in one IPC round-trip) |
| Primal capability method catalog | all primals | Centralized registry of which primal provides which method with response schema | **PARTIAL** — wire contracts now exist for tensor/shader/dispatch; remaining: crypto, storage, discovery schemas |

**What this means for springs**: At the composition validation level (Level 5),
springs have **no local math** — all computation delegates to primals via IPC.
Springs use `CompositionContext::from_live_discovery_with_fallback()` (preferred)
or `from_live_discovery()` + `validate_parity()` to confirm that primal compositions
produce results matching the original Python baselines. The `_with_fallback` variant
tries UDS first, then probes TCP ports via `{PRIMAL}_PORT` env vars — enabling
validation against both UDS and TCP (container, cross-arch) deployments.
The spring's own Rust code (Levels 2-4) served its purpose: it evolved
the upstream primals and is now fossil record. When a primal isn't running, checks
degrade to `SKIP` (honest, not faked). **There are no spring binaries at this level.**

**What this means for gardens**: esotericWebb and future gardens are pure compositions
of primals via biomeOS — graph-as-product. They use ludoSpring math (now in barraCuda),
wetSpring biology (now in primals), etc. Gardens never ship their own binaries.
Downstream validates upstream: paper → Python → Rust → ecoPrimals.

### Composition Elevation Sprint Priorities (April 13, 2026)

Current season: **Mountain → Spring transition**. Primals are stabilizing;
primalSpring is proving composition parity. See `ECOSYSTEM_EVOLUTION_CYCLE.md`
in `infra/wateringHole/` for the full water-cycle model.

**primalSpring — Phase 34 (composition elevation)**:

| # | Sprint Item | Depends On | Status |
|---|-------------|------------|--------|
| 1 | **Tower composition parity**: launch BearDog + Songbird, call `crypto.hash` + `discovery.resolve`, compare against known values | Nothing — schemas stable | **IN PROGRESS** |
| 2 | **Nest composition parity**: add NestGate + provenance trio, call `storage.store` + `storage.retrieve` round-trip, verify data integrity | Nothing — storage IPC stable | **IN PROGRESS** |
| 3 | **Node composition parity**: add barraCuda + coralReef + toadStool, call `tensor.matmul` / `tensor.dot`, compare against Python baseline | Wire contracts delivered (Sprint 42 / Iter 80 / S203) | **UNBLOCKED** |
| 4 | **Full NUCLEUS parity**: combine Tower + Node + Nest, run cross-atomic composition (encrypt → compute → store → retrieve → verify) | Items 1-3 | **IN PROGRESS** |
| 5 | **Chimera compositions**: multi-niche compositions via biomeOS graph execution | biomeOS v3.04 `nucleus_composition_e2e.rs` + Item 4 | **CLOSER** |
| 6 | **Downstream proto-nucleate parity harness**: template experiment for springs to plug in their Python baseline and validate composition | Item 4 + spring response schema docs | **FUTURE** |

**Upstream primal sprint targets (composition enablement)**:

| Primal | Sprint | Composition Enablement Task | Status |
|--------|--------|----------------------------|--------|
| barraCuda | Sprint 42 | `TENSOR_WIRE_CONTRACT.md` v1.0.0 — 3 response categories, batch pipeline | **DELIVERED** |
| coralReef | Iter 80 | `SHADER_COMPILE_WIRE_CONTRACT.md` — compile + multi-device + capabilities | **DELIVERED** |
| toadStool | S203 | `DISPATCH_WIRE_CONTRACT.md` — standard envelope for all dispatch variants | **DELIVERED** |
| biomeOS | v3.04 | `nucleus_composition_e2e.rs` — TOML parsing + topological sort + multi-phase execution | **DELIVERED** |
| BearDog | Wave 36 | Ionic bond lifecycle (propose → accept → seal with real Ed25519) | **DELIVERED** |
| Songbird | Wave 137 | `capability.resolve` wiring, capability-based naming | **DELIVERED** |
| NestGate | Session 43 | Compliance audit, deep debt evolution | **DELIVERED** |
| Squirrel | alpha.49 | ecoBin compliance, inference wire test | **DELIVERED** |

---

## biomeOS

| ID | Gap | Status |
|----|-----|--------|
| BM-01 | `graph.deploy` routing | **RESOLVED** (v2.79 — `graph.execute`) |
| BM-02 | `health.liveness` on Neural API | **RESOLVED** (v2.81) |
| BM-03 | `unix://` prefix on `capability.discover` | **RESOLVED** (v2.79 — `strip_unix_uri`) |
| BM-04 | Late primal registration invisible | **RESOLVED** (v2.81 — `topology.rescan` + lazy discovery) |
| BM-05 | Multi-shape probe response | **RESOLVED** (v2.81) |
| BM-06 | `discover_capability` lacks domain prefix matching | **RESOLVED** (v2.92 — `try_prefix_lookup` + `capability_to_provider_fallback` last resort). Deploy graphs also include bare domain aliases as belt-and-suspenders. |
| BM-07 | Registry stores `{primal}-{hash}.sock` instead of live sockets | **RESOLVED** (April 10 — `get_family_id()` → `self.family_id` in defaults, config, domain bridge; socket alias for toadstool→compute, nestgate→storage) |
| BM-08 | Socket resolution misses primals without `--socket` flag | **RESOLVED** (April 10 — plain `{primal}.sock` fallback in `resolve_primal_socket_with()` for loamSpine, sweetGrass, petalTongue) |
| BM-09 | `capability.call` forwards to tarpc socket instead of JSON-RPC | **RESOLVED** (April 10 — `.jsonrpc.sock` preferred over `.sock` for domain aliases in socket resolution) |
| BM-11 | ToadStool dual-socket: `build_socket_path` + `resolve_primal_socket` lack explicit JSON-RPC preference | **RESOLVED** (April 10 — `prefers_jsonrpc` flag in `socket.rs`, `.jsonrpc.sock` sibling check in `path_builder.rs`, stale socket cleanup in launcher) |

**Compliance** (v3.13 — April 14): clippy **CLEAN**, fmt **PASS**, **7,695+ tests PASS** ↑, `deny(unsafe_code)` workspace + `forbid` per-crate, `deny.toml` present, SPDX present. Zero `#[allow(`. **BTSP Phase 1 COMPLETE** (v2.98). **BTSP Phase 2 COMPLETE** ↑↑ — `btsp_client.rs` expanded to 524+ lines: full server-side handshake (`server_handshake()`) wired into Neural API UDS listener (`handle_connection_with_btsp`), enforce vs warn-only modes, graceful fallback for raw JSON-RPC clients. Wire types: `ClientHello/ServerHello/ChallengeResponse/HandshakeComplete`. BearDog delegation. **v3.10–v3.13 evolution**: hardcoded primal names → capability constants, `learn_from_event` implemented, topology uses live health probes (not hardcoded "healthy"), `capability.call` prefers Tower Atomic relay, recursive `graph.list`, BTSP handshake failure warnings with socket path, `BIOMEOS_BIND_ADDRESS` for TCP-only bootstrap, `capability.rs` split. **Discovery compliance: COMPLETE**.

---

## petalTongue

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| PT-01 | Socket at non-standard path | **RESOLVED** — `biomeos/petaltongue.sock` |
| PT-02 | No live push to browser | **RESOLVED** — SSE `/api/events` |
| PT-03 | `motor_tx` not wired in server mode | **RESOLVED** — drain channel wired |
| PT-04 | No `ExportFormat::Html` in headless CLI | Low | Partially — `ExportFormat::Html` exists in headless path + IPC; needs product validation |
| PT-05 | `visualization.showing` returns false | **RESOLVED** — `RenderingAwareness` auto-init in `UnixSocketServer` |
| PT-06 | `callback_method` poll-only dispatch | **RESOLVED** | Sprint 5 — `UnixSocketServer::new()` now spawns push delivery and assigns `callback_tx` on `RpcHandlers` at startup. `callback_sender()` exposed for UI consumers. Test asserts wiring on construction. Intentionally push-free in non-server modes (headless/TUI/web) |
| PT-07 | No external event source in server mode | **RESOLVED** — periodic discovery refresh wired |
| PT-08 | No BTSP Phase 1 (`BIOMEOS_INSECURE` guard) | **RESOLVED** ↑ — `btsp.rs` module: `validate_insecure_guard()`, family-scoped sockets, domain symlinks |
| PT-09 | BTSP Phase 2 (handshake integration) | Low | Phase 2 stub — `handshake_policy` logs warning, connections accepted without handshake |
| PT-10 | `--socket` CLI flag missing | **RESOLVED** | April 10 — `--socket` flag added to `Commands::Server`, plumbed via `UnixSocketServer::with_socket_path()` |
| PT-11 | Only `visualization` domain symlink | **RESOLVED** | April 10 — now creates `visualization.sock`, `ui.sock`, `interaction.sock` symlinks (create+drop) |

**Compliance** (v1.6.6+ — April 10): clippy **CLEAN**, fmt **PASS**, `forbid(unsafe_code)` per-crate, `deny.toml` present, SPDX present. Zero `todo!`/`unimplemented!`/`FIXME`. Tests **ALL PASS**. **BTSP Phase 1 COMPLETE** ↑↑. **BTSP Phase 2 STUB** — `handshake_policy` logs but does not enforce. **`--socket` CLI flag** wired via `with_socket_path()`. **Domain symlinks**: `visualization`, `ui`, `interaction`. **Capability Wire Standard L2/L3**.

---

## barraCuda

BC-01–BC-05 **RESOLVED**. New architectural gaps BC-06–BC-08 identified during benchScale
NUCLEUS deployment validation (April 11). Math is universal — these gaps block
barraCuda from fulfilling its role as a hardware-agnostic math primal.

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| BC-01 | Fitts formula variant | **RESOLVED** (Sprint 25 — `variant` param, Shannon default) |
| BC-02 | Hick formula off-by-one | **RESOLVED** (Sprint 25 — `include_no_choice` param) |
| BC-03 | Perlin3D lattice | **RESOLVED** (Sprint 25 — proper gradients + quintic fade) |
| BC-04 | No plasmidBin binary | **RESOLVED** (April 1 harvest, 4.5M static-pie stripped) |
| BC-05 | `barracuda server` panics without GPU | **RESOLVED** (Sprint 39 — `Auto::new()` returns `Err`, server starts with `device = None`, health reports `Degraded`. Stale binary in plasmidBin was pre-Sprint 39; refreshed April 11) |
| BC-06 | musl-static binary can't access GPU | **RESOLVED** (documented) | Sprint 41 — Constraint documented in README (Deployment Modes matrix) and CONTEXT.md. ecoBin musl-static binaries run CPU-only via wgpu path. GPU access in NUCLEUS via SovereignDevice IPC (BC-07) or cpu-shader (BC-08). This is architectural, not a bug |
| BC-07 | No toadStool→coralReef IPC delegation | **RESOLVED** | Sprint 41 — `Auto::new()` now returns `DiscoveredDevice` with full 3-tier fallback: wgpu GPU → wgpu CPU → SovereignDevice IPC (via `sovereign_available()` + `SovereignDevice::with_auto_device()`). Requires `sovereign-dispatch` feature + live peers. `BarraCudaPrimal` holds `DiscoveredDevice`, health reports `sovereign_ipc` |
| BC-08 | No pure-CPU scalar fallback | **RESOLVED** | Sprint 40 — `cpu-shader` feature now **default-on** in `crates/barracuda/Cargo.toml` (`default = ["gpu", "domain-models", "cpu-shader"]`). ecoBin binaries now include naga-exec CPU math. All batch ops have `#[cfg(feature = "cpu-shader")]` paths active by default |

**Compliance** (Sprint 39 — April 10): clippy **CLEAN** (`-D warnings`, pedantic + nursery), fmt **PASS**, `deny.toml` present (bans openssl/native-tls/ring/aws-lc-sys), zero `todo!`/`unimplemented!`/`FIXME`. **4,422 tests PASS** (nextest CI). `#![forbid(unsafe_code)]` on `barracuda` + `barracuda-core`. **BTSP Phase 1 COMPLETE**. **BTSP Phase 2 COMPLETE** ↑↑ — `guard_connection()` implements full 6-step handshake relay: `ClientHello` → `btsp.session.create` → `ServerHello` → `ChallengeResponse` → `btsp.session.verify` → `HandshakeComplete`. Capability-based crypto provider discovery (`crypto-{fid}.sock` → `crypto.sock` → `*.json` scan). All 3 accept loops guarded (Unix, TCP, tarpc). Legacy/non-BTSP clients degrade gracefully (2s timeout). **Capability Wire Standard L2**. Nextest `gpu-serial` extended to stress/gpu profiles. **Note**: `BufReader` lifetime gap between handshake phases (edge-case for fast/coalescing clients); post-handshake stream encryption not yet applied.

---

## Squirrel

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| SQ-01 | Abstract-only socket | **RESOLVED** (alpha.25b — `UniversalListener`) |
| SQ-02 | `LOCAL_AI_ENDPOINT` not wired into `AiRouter` | **RESOLVED** (alpha.27 — step 1.5 discovery, `resolve_local_ai_endpoint()`) |
| SQ-03 | `deprecated-adapters` feature flag docs | **RESOLVED** — documented in `CURRENT_STATUS.md` feature-gates table; intentional retention until v0.3.0 with migration path to `UniversalAiAdapter` + `LOCAL_AI_ENDPOINT` |
| SQ-04 | `--port` TCP bind hardcoded to `127.0.0.1` | **RESOLVED** (alpha.52) — `--bind` CLI flag + `SQUIRREL_BIND` / `SQUIRREL_IPC_HOST` env vars. Default `127.0.0.1` (secure). Docker: `--bind 0.0.0.0`. Parity with barraCuda BC-09 `resolve_bind_host()` pattern |

**Compliance** (alpha.52 — April 14): Zero `todo!`/`unimplemented!`/`FIXME` in non-test code. fmt **PASS**. clippy **PASS**. **7,203 tests PASS** (22 workspace members). `deny.toml` present. Workspace `forbid(unsafe_code)`. **BTSP Phase 1 COMPLETE** (alpha.44). **BTSP Phase 2 COMPLETE** ↑↑ — `btsp_handshake.rs` (627 LOC) implements full server-side handshake on UDS accept with BearDog delegation (`btsp.session.create`, `btsp.session.verify`). `maybe_handshake()` called in both abstract+filesystem UDS accept paths in `jsonrpc_server.rs`. Length-prefixed wire framing per standard. `is_btsp_required()` checks `FAMILY_ID` + `BIOMEOS_INSECURE`. Provider discovery: env → manifest scan → well-known `beardog-{fid}.sock`. **BTSP Phase 3 deferred** — `cipher = "null"` after verify; full cipher negotiation via `btsp.negotiate` pending. **SQ-04 RESOLVED** ↑ — `--bind` CLI flag + `SQUIRREL_BIND` / `SQUIRREL_IPC_HOST` env vars. Default `127.0.0.1` (secure). Docker: `--bind 0.0.0.0`. Parity with barraCuda BC-09 `resolve_bind_host()` pattern. **Capability Wire Standard L2**. Smart refactoring: 9 large files split (alpha.52), session/mod.rs/transport/client.rs/context_state.rs/api.rs all under 600 LOC. Dependency purge: pprof/openai/libloading/hostname removed, flate2 → pure Rust backend. **Inference provider bridge** ↑ — `inference.complete`/`embed`/`models` wire methods dispatched via `handlers_inference.rs`, bridging ecoPrimal wire standard to `AiRouter`. Capability-first naming (toadstool→compute, songbird→discovery stems). **Genetics awareness**: `genetic_families` optional wire field; no three-tier type consumption yet — awaits ecoPrimal ≥0.10.0.

---

## songBird

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| SB-01 | `health.liveness` canonical name | **RESOLVED** (wave89-90) |
| SB-02 | CLI `ring-crypto` opt-in feature | **RESOLVED** | Wave 135 — `ring-crypto` feature removed entirely. No direct `ring` in any manifest. Default uses `rustls_rustcrypto`. Note: `ring` remains in `Cargo.lock` as transitive via `rustls`/`rustls-webpki` — not compiled in default build |
| SB-03 | `sled` in orchestrator/sovereign-onion/tor | **RESOLVED** | Wave 135 — `sled` fully eliminated from workspace and Cargo.lock. No `sled` in any manifest |

**Compliance** (Wave 133 — April 9 wave 3): clippy **CLEAN**, fmt **PASS**. `forbid(unsafe_code)` per-crate, `deny.toml` present, SPDX present. **BTSP Phase 1 COMPLETE**. **BTSP Phase 2 COMPLETE** ↑↑ — `perform_server_handshake()` in `ipc/btsp.rs`, wired into UDS accept loop (`connection.rs` branches on `btsp_active`), BearDog delegation via `SecurityRpcClient`. `BtspClient` + `btsp_client.rs`. Length-prefixed frames after handshake. **Capability Wire Standard L3**.

---

## NestGate

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| NG-01 | IPC storage backend inconsistency | **RESOLVED** ↑ — `SemanticRouter::new()` enforces `FileMetadataBackend` in production; `InMemoryMetadataBackend` only used in tests/ephemeral. NG-01 compliance: file backend mandatory when `FAMILY_ID` set |
| NG-02 | Session API inconsistency | **RESOLVED** — `semantic_router/session.rs` added; `SemanticRouter::call_method` dispatches `session.save`/`load`/`list`/`delete` |
| NG-03 | `data.*` handlers delegation | **RESOLVED** ↑ — `data.*` wildcard delegation replaces hardcoded NCBI/NOAA/IRIS stubs. Returns structured `NotImplemented` with `discovery.query` redirect. Explicitly excluded from `capabilities.list`. Tested in `data_wildcard_returns_delegation_not_implemented` |
| NG-04 | C dependency (`aws-lc-rs`/`ring`) | **RESOLVED** — `ring` eliminated, TLS delegated to system `curl` |
| NG-05 | Crypto crates not fully delegated | **RESOLVED** — `nestgate-security` zero crypto deps, all via BearDog IPC `CryptoDelegate` |

| NG-06 | `--socket` CLI flag not wired in `Commands::Server` | **RESOLVED** | April 10 — `--socket` flag added to `Commands::Server`, sets `NESTGATE_SOCKET` env var before `run_daemon`, feeds into `SocketConfig::from_environment()` tier-1 resolution |
| NG-07 | aarch64-musl segfault | **RESOLVED** | Static-PIE + musl ≤1.2.2 crash in `_start_c/dlstart.c`. Fixed: `-C relocation-model=static` in `.cargo/config.toml` for both x86_64 and aarch64 targets |
| NG-08 | `ring` v0.17.14 in production via `rustls` default crypto | **RESOLVED** | April 11 — NestGate eliminated `reqwest` entirely, switched to `ureq` with `rustls-no-provider` + `rustls-rustcrypto`. `cargo tree -i ring` now returns "nothing to print". **13/13 primals ring-free.** |

**Compliance** (Session 43n — April 14): Clippy **CLEAN**, fmt **PASS**, **11,819 tests PASS** ↑. `forbid(unsafe_code)` per-crate + workspace `deny`. `deny.toml` present. SPDX present. **BTSP Phase 1 COMPLETE**. **BTSP Phase 2 COMPLETE** ↑↑ — `btsp_server_handshake.rs` implements full server-side handshake wired into **both** UDS listener paths. Delegates to BearDog `btsp.session.create/verify/negotiate`. `is_btsp_required()` guard. **Session 43n evolution**: Semantic router streaming parity (5 storage streaming methods). Event-driven connection lifecycle (`select!` idle timeout, `connection.closing` notification). Deep debt: zero `dyn Error`, zero `async-trait` in production. `fetch_external` delegated through Tower Atomic (biomeOS `capability.call`), direct TLS removed from nestgate-rpc. **Capability Wire Standard L3**.

---

## rhizoCrypt

All gaps **RESOLVED**.

| ID | Gap | Status |
|----|-----|--------|
| RC-01 | TCP-only transport | **RESOLVED** (v0.14.0-dev s23 — `--unix`, `UdsJsonRpcServer`, `biomeos/` path) |
| RC-02 | Witness wire evolution | **RESOLVED** (v0.14.0-dev — `WireWitnessRef`: kind/evidence/encoding/algorithm/tier/context) |

**Compliance** (S31 — April 9 wave 3): clippy clean, fmt clean, `deny(unsafe_code)` + `forbid`, `deny.toml` present, tests pass. **BTSP Phase 1 COMPLETE**. **BTSP Phase 2 COMPLETE** ↑↑ — new `rhizo-crypt-rpc/src/btsp/` module (framing.rs, server.rs, types.rs): `BtspServer::accept_handshake` implements full 4-step handshake, wired into UDS accept loop (`serve_inner` → `handle_uds_connection`). **Local crypto** (HKDF, X25519, HMAC-SHA256) — does NOT delegate to BearDog (self-sovereign approach). Client handshake in `btsp/handshake.rs`. **Capability Wire Standard L3**.

---

## loamSpine

All gaps **RESOLVED**.

| ID | Gap | Status |
|----|-----|--------|
| LS-03 | Panic on startup | **RESOLVED** (v0.9.15 — infant discovery fails gracefully) |
| LS-04 | Witness wire evolution | **RESOLVED** (v0.9.16 — `WireWitnessRef` in `trio_types.rs`, witnesses on wire summaries) |
| LS-05 | `--socket` CLI flag missing | **RESOLVED** | April 10 — `--socket` flag added to `Command::Server`, passed directly to `run_server` (no env mutation, respects `forbid(unsafe_code)`) |

**Compliance** (0.9.16+ — April 14): clippy clean, fmt **PASS**, `forbid(unsafe_code)` workspace, `deny.toml` present, **1,442 tests PASS**. **BTSP Phase 1 COMPLETE**. **BTSP Phase 2 COMPLETE** ↑↑ — `perform_server_handshake()` in `crates/loam-spine-core/src/btsp.rs`, wired into UDS accept loop. **BTSP decoupled from BearDog identity** ↑ — `beardog_client.rs` → `provider_client.rs` (any security provider can serve BTSP sessions). **`JsonRpcCryptoSigner` / `JsonRpcCryptoVerifier`** — production `Signer`/`Verifier` wire adapters (JSON-RPC to the configured security provider). `provenance.commit` → `session.commit` alias wired (primalSpring benchScale compat). `certificate.get` capability added. Named constants, `Arc<str>`, `.into()` modernization. **Capability Wire Standard L2/L3**. **Stadial parity gate (April 16, 2026):** sled + sqlite/rusqlite storage backends removed; **hickory-resolver** 0.26; lockfile ghosts eliminated (`sled`, `libsqlite3-sys`, `rusqlite`, `instant`, `fxhash`); **`cargo deny`** bans + advisories **PASS**; **178** Rust source files; **0** `#[async_trait]` in-tree; **0** clippy warnings; Edition **2024**; transitive **`async-trait`** via **hickory-net** only (upstream); **`ring`** only in optional features; **`dyn` audit** 72 total (non-blocking).

---

## toadStool

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| TS-01 | coralReef discovery not pure capability-based | Low | **RESOLVED** — `VisualizationClient` (shader client) uses `capability.discover("shader")` Tier 1, then filesystem fallback Tiers 0/2/3. No 6-step pattern remains. |
| TS-02 | `compute.sock` tarpc-only; JSON-RPC probes fail | **RESOLVED** | April 10 — `jsonrpc_socket` now `compute.jsonrpc.sock` (separate from tarpc `compute.sock`). Legacy symlinks: `toadstool.jsonrpc.sock` → `compute.jsonrpc.sock` |
| TS-03 | `--socket` CLI flag parsed but not wired | **RESOLVED** | April 10 — `socket_override` param added to `run_server_main`, wired through dispatch. Overrides `get_socket_path()` resolution |
| TS-04 | `ollama.*`/`inference.*` semantic mappings advertised but not dispatched | **RESOLVED** | April 10 — Removed from `mappings_extended.rs`. Inference is Squirrel's domain via ecoPrimal wire standard. ToadStool is compute substrate, not model serving. |

**Compliance** (S203i — April 14): Clippy **CLEAN**, fmt **PASS**. 21,600+ tests **PASS**. `deny.toml` present. **BTSP Phase 1 COMPLETE**. **BTSP Phase 2 COMPLETE** ↑↑ — `BtspServer::accept_handshake` wired into JSON-RPC Unix accept and tarpc accept, feature-gated behind `btsp` feature + env check. `BtspClient` in `toadstool_common::btsp`. Fuzz targets (`fuzz_btsp_framing.rs`). **S203e–S203i evolution**: test extraction from 52 production files, TCP idle timeout (resolves exp082 half-open), BTSP auto-detect (LD-04: binary vs text first byte on accept), `compute.execute` direct route, pipeline methods in `capabilities.list`, network centralization, async GPU discovery. **Capability Wire Standard L3**. **Socket separation COMPLETE** — JSON-RPC and tarpc bind distinct sockets.

---

## sweetGrass

All gaps **RESOLVED**. TCP JSON-RPC added, `cargo-deny`, `forbid(unsafe)`.

| ID | Gap | Status |
|----|-----|--------|
| SG-01 | Witness wire evolution | **RESOLVED** (v0.7.27 — `Witness` type, `EcoPrimalsAttributes.witnesses`, kind/evidence/encoding) |
| SG-02 | `--socket` CLI flag missing | **RESOLVED** | April 10 — `--socket` flag added to `Commands::Server`, plumbed via `start_uds_listener_at()` / `cleanup_socket_at()` |

**Compliance** (v0.7.27+ — April 10): clippy clean, fmt clean, `deny(unsafe_code)` workspace + `forbid` per-crate, `deny.toml` present, tests pass. **BTSP Phase 1 COMPLETE**. **BTSP Phase 2 COMPLETE** ↑↑ — new `sweet-grass-service/src/btsp/` module (mod.rs, protocol.rs, server.rs): `perform_server_handshake()` wired into UDS accept (`handle_uds_connection_btsp` in `uds.rs`) + TCP (`tcp_jsonrpc.rs`). **Delegates to BearDog** (`btsp.session.create/verify/negotiate`). Client: `perform_handshake()` in `sweet-grass-integration/src/btsp/protocol.rs`. **`--socket` CLI flag** wired. **Capability Wire Standard L3**.

---

## sourDough

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| SD-01 | Missing `deny.toml` | Low | Open — cargo-deny not configured |
| SD-02 | musl cross-compilation | Low | Open — binary builds not yet wired for ecoBin |
| SD-03 | genomeBin signing | Low | Open — sequoia-openpgp not implemented |

**Compliance** (v0.1.0 — f1cc802): clippy **CLEAN** (`all` + `pedantic` + `nursery`), fmt **PASS**, `forbid(unsafe_code)` at workspace level, `deny.toml` **MISSING**, SPDX AGPL-3.0-or-later in Cargo.toml. **239 tests, 0 failures** (unit + integration + e2e + doctests), coverage 96%+. Edition 2024, workspace lints centralized. Zero `TODO`/`FIXME`/`HACK`/`unimplemented!` in source. **Discovery compliance: NEAR-CLEAN** — 1 BearDog string in CLI genomebin.rs (cosmetic). Scaffold independence confirmed: generated primals have no runtime dependency on sourDough.

---

## coralReef

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| CR-01 | No BTSP Phase 1 (`BIOMEOS_INSECURE` guard) | **RESOLVED** ↑ — `validate_insecure_guard()` in glowplug, core, ember; called from `main.rs` |
| CR-02 | No `capabilities.list` with flat `methods` array | **RESOLVED** ↑ — `capability.list` + `identity.get` with flat `methods` (uses singular `capability.list` not `capabilities.list`) |
| CR-03 | BTSP Phase 2 (handshake) | **RESOLVED** ↑↑ — `guard_connection()` (renamed from `gate_connection`) in all 3 crates: BearDog delegation via `btsp.session.create`, capability-based crypto socket discovery, `BtspOutcome` enum. Async in core/glowplug, blocking in ember. Degraded mode when provider missing. |
| CR-04 | Typed errors (`Result<_, String>` in driver) | **RESOLVED** | Iter 79b — Wave 4 complete: `BootTrace::from_mmiotrace` → `Result<Self, ChannelError>`, `ChannelAllocDiag.result` → `Result<u32, DriverError>`. Zero `Result<_, String>` remaining in coral-driver production code. Test harness still uses `String` errors (acceptable) |
| CR-05 | `cpu_exec.rs` dead code | **RESOLVED** | Iter 79b — File deleted (365 lines removed). Was orphaned stub not in module tree |

**Compliance** (Iter 80 — April 14): clippy **CLEAN** (pedantic + nursery, 0 warnings), fmt **PASS**, `forbid(unsafe_code)` on coralreef-core + nak-ir-proc + stubs, `deny.toml` present (bans wildcards, yanked-deny). **4,506 tests, 0 failures**, ~153 ignored (HW-gated). SPDX present. **0 files over 1000 LOC**. `coral-driver` opts out of workspace `unsafe_code = "deny"` (ioctl/mmap/MMIO required). **BTSP Phase 1 COMPLETE**. **BTSP Phase 2 COMPLETE** ↑↑ — `guard_connection()` calls `btsp.session.create` on real UDS, degrades when provider absent. Wired into Unix JSON-RPC, TCP newline, tarpc accept paths. **Iter 79–80 evolution**: `--bind` flag + `CORALREEF_IPC_HOST` for network-facing deployments. Feature-gate VFIO constructors. `#[must_use]` dispatch audit. 6 multi-stage ML pipeline composition tests. Hot-path alloc elimination. `engine_regs` module extraction. `Display` zero-alloc. **Capability Wire Standard L2** ↑ — `capability.list` + `identity.get` with flat `methods`. tarpc `Result<_, String>` → `TarpcCompileError`.

---

## bearDog

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| BD-01 | `crypto.verify_ed25519` does not accept `encoding` hint | **RESOLVED** ↑ — Wave 33: per-field `message_encoding`, `signature_encoding`, `public_key_encoding` + global `encoding` default. Semantic aliases `crypto.ed25519.sign`/`crypto.ed25519.verify` added. Tests cover hex/mixed encodings. |

**Compliance** (Wave 50 — April 14): clippy clean, fmt clean, `forbid(unsafe_code)` at workspace level, `deny.toml` present. SPDX present. **Coverage 90.51%** (llvm-cov). **14,784 tests, 0 failures.** **0 files over 1000 LOC**. `#[allow(` 193→75 (62% reduction), `#[expect(reason` 361→476. **BTSP Phase 2+3 COMPLETE**. **Capability Wire Standard L2**. **TS-01 RESOLVED** ↑ — `transport_security` in `capabilities.list` and `discover_capabilities` (btsp_required, btsp_version, cleartext_available). BTSP rejection now sends JSON-RPC -32600 error (not silent drop). **`genetic.*` RPCs** serve three-tier genetics: `derive_lineage_key`, `derive_lineage_beacon_key`, `mix_entropy`, `generate_lineage_proof`, `verify_lineage`. **Dynamic `ipc.register`** with orchestration registry (non-blocking + heartbeat). **Standalone startup** (`standalone-{uuid}` on missing `NODE_ID`). TCP transport skip when `--port`/`--listen` not passed (UDS-only default). **BD-01 RESOLVED**.

---

## BTSP Secure-by-Default Compliance (April 9, 2026)

Per `BTSP_PROTOCOL_STANDARD.md` v1.0: All primals MUST implement socket naming
alignment (Phase 1) and BTSP handshake (Phase 2) when `FAMILY_ID` is set.

### Phase 1: Socket Naming + BIOMEOS_INSECURE Guard

| Primal | Socket Naming | INSECURE Guard | Family-Scoped | Domain-Based | Status |
|--------|:---:|:---:|:---:|:---:|--------|
| BearDog | PASS | PASS | PASS | PASS (`security`) | **COMPLETE** |
| Songbird | PASS | PASS | PASS | partial (`network`) | **COMPLETE** |
| biomeOS | PASS | PASS | PASS | — | **COMPLETE** (v2.98) |
| NestGate | PASS | PASS | PASS | PASS (`storage`) | **COMPLETE** |
| ToadStool | PASS | PASS | PASS | pending | **COMPLETE** (S192) |
| Squirrel | PASS | PASS | PASS | PASS (`ai`) | **COMPLETE** (alpha.44) |
| rhizoCrypt | PASS | PASS | PASS | PASS (`dag`) | **COMPLETE** (S29) |
| loamSpine | PASS | PASS | PASS | PASS (`permanence`) | **COMPLETE** (v0.9.16) |
| sweetGrass | PASS | PASS | PASS | partial | **COMPLETE** (v0.7.27) |
| barraCuda | PASS | PASS | PASS | PASS (`math`) | **COMPLETE** (Sprint 31) |
| petalTongue | PASS ↑ | PASS ↑ | PASS ↑ | PASS (`visualization`) | **COMPLETE** ↑↑ |
| coralReef | PASS ↑ | PASS ↑ | PASS ↑ | PASS (`shader`/`device`) | **COMPLETE** ↑↑ |
| skunkBat | PASS ↑ | PASS ↑ | PASS ↑ | — | **COMPLETE** ↑↑ |

### Phase 2: BTSP Handshake Integration

| Primal | Handshake on Accept | Handshake Client | Status |
|--------|:---:|:---:|--------|
| BearDog | **YES** (`perform_server_handshake`) | **YES** (reference impl) | **COMPLETE** — Wave 31 |
| Songbird | **YES** ↑↑ (`perform_server_handshake`) | **YES** (`BtspClient`) | **COMPLETE** ↑↑ — Wave 133 |
| biomeOS | **YES** (`handle_connection_with_btsp`) | **YES** (`btsp_client.rs`) | **COMPLETE** — v3.00 |
| NestGate | **YES** (`btsp_server_handshake.rs`) | **YES** (`btsp_client.rs`) | **COMPLETE** — both UDS paths |
| ToadStool | **YES** ↑↑ (`BtspServer::accept_handshake`) | **YES** ↑ (`BtspClient`) | **COMPLETE** ↑↑ — S198 |
| Squirrel | **YES** (`btsp_handshake.rs`) | **YES** | **COMPLETE** — alpha.46+ |
| rhizoCrypt | **YES** ↑↑ (`BtspServer::accept_handshake`) | **YES** (`btsp/handshake.rs`) | **COMPLETE** ↑↑ — S31 (local crypto) |
| loamSpine | **YES** ↑↑ (`perform_server_handshake`) | mock only | **COMPLETE** ↑↑ — BearDog delegation |
| sweetGrass | **YES** ↑↑ (`perform_server_handshake`) | **YES** (`btsp/protocol.rs`) | **COMPLETE** ↑↑ — BearDog delegation |
| barraCuda | **YES** ↑↑ (`guard_connection`) | **YES** (BearDog relay) | **COMPLETE** ↑↑ — Sprint 39 (full 6-step relay: ClientHello→create→ServerHello→ChallengeResponse→verify→Complete) |
| petalTongue | **YES** ↑↑ (Sprint 8) | **YES** (BearDog delegation) | **COMPLETE** ↑↑ — Sprint 8 (real delegation, RPITIT) |
| coralReef | **YES** ↑↑ (`guard_connection`) | **YES** (BearDog session.create) | **COMPLETE** ↑↑ — Iter 78 (real UDS RPC to BearDog, session_id parsed, degraded when provider absent) |
| skunkBat | **YES** ↑↑ (`PeekedStream`) | **YES** (BearDog v0.9.0) | **COMPLETE** ↑↑ — v0.1.0 (UDS first-byte peek, BearDog handshake alignment) |

**Phase 2 ecosystem cascade (April 9–16)**: **13/13** primals now enforce BTSP handshake on
incoming UDS connections: BearDog, Songbird, biomeOS, NestGate, ToadStool, Squirrel,
rhizoCrypt, loamSpine, sweetGrass, barraCuda (Sprint 39), coralReef (Iter 78),
**petalTongue** ↑ (Sprint 8), **skunkBat** ↑ (v0.1.0).
**Tower Atomic: 100%.** **Node Atomic: 100%.** **NUCLEUS: 100%.** **All primals: 100%.**
**Note**: Full challenge-response + encrypted framing (Phase 3) not yet applied to
post-handshake streams in barraCuda or coralReef.

**plasmidBin validation (April 10)**: Full musl-static rebuild confirms all BTSP Phase 1+2
code is now in the deployed binaries. Previous plasmidBin binaries (Apr 8) predated the
cascade and had no handshake enforcement. 12/12 ecoBin compliant.

---

## Capability Wire Standard Compliance (April 9, 2026)

Per `PRIMAL_CAPABILITY_WIRE_STANDARD_APR08_2026.md` v1.0: every primal's
`capabilities.list` MUST include `primal`, `version`, `methods` (flat array).

| Primal | Wire Level | `methods` flat | `identity.get` | `provided_capabilities` | Notes |
|--------|:---:|:---:|:---:|:---:|---|
| BearDog | **L2** | YES | YES | YES (Format E groups) | Reference impl |
| Songbird | **L3** | YES | YES | YES | `capability_tokens.rs` |
| biomeOS | consumer | parses all | probes peers | — | 5-format adaptive parser |
| NestGate | **L3** | YES | YES | YES | `model_cache_handlers.rs` |
| ToadStool | **L3** | YES | YES | YES | `handler/core.rs` |
| Squirrel | **L2** | YES | YES | partial | `handlers_capability.rs` |
| rhizoCrypt | **L3** | YES | YES | YES | `niche.rs` — full composable |
| loamSpine | **L3** | YES | YES | YES | Full L3: 37 methods, 10 capability groups, bond-ledger, self-knowledge compliant, **stadial-gate compliant**, 178 source files (April 16, 2026) |
| sweetGrass | **L3** | YES | YES | YES | Full composable |
| barraCuda | **L2** | YES | YES | partial | Sprint 31 |
| petalTongue | **L2/L3** | YES | YES | partial | `system.rs` |
| coralReef | **L2** ↑ | YES ↑ | YES | partial | `capability.list` (singular) + `identity.get` |
| skunkBat | **L2** ↑ | YES ↑ | YES ↑ | partial | New JSON-RPC server, both `capability.list` + `capabilities.list` |
| sourDough | **NONE** | NO | NO | NO | Scaffolding tool, not IPC primal |
| bingoCube | **NONE** | NO | NO | NO | CLI tool, not IPC primal |

---

## plasmidBin Binary Inventory (April 10, 2026 — full musl rebuild)

All 12 x86_64 primals rebuilt with `--target x86_64-unknown-linux-musl` and stripped.
**12/12 ecoBin compliant** — zero dynamic library dependencies, no interpreter.

| Binary | Size | Linkage | ecoBin | Build Date | BTSP P1+P2 in Binary? |
|--------|------|---------|:---:|------------|:---:|
| beardog | 7.4M | **STATIC** | YES | Apr 10 | **YES** |
| songbird | 17M | **STATIC** | YES | Apr 10 | **YES** |
| nestgate | 7.9M | **STATIC** | YES | Apr 10 | **YES** |
| squirrel | 3.4M | **STATIC** | YES | Apr 10 | **YES** |
| toadstool | 11M | **STATIC** | YES | Apr 10 | **YES** |
| petaltongue | 29M | **STATIC** | YES | Apr 10 | Phase 1 (Phase 2 stub) |
| biomeos | 17M | **STATIC** | YES | Apr 10 | **YES** (BM-07 fix) |
| rhizocrypt | 5.7M | **STATIC** | YES | Apr 10 | **YES** |
| loamspine | 4.4M | **STATIC** | YES | Apr 10 | **YES** |
| sweetgrass | 8.9M | **STATIC** | YES | Apr 10 | **YES** |
| barracuda | 4.7M | **STATIC** | YES | Apr 10 | **YES** ↑ (full 6-step relay) |
| skunkbat | 2.2M | **STATIC** | YES | Apr 10 | Phase 1 only |

**aarch64** (5 binaries): beardog, songbird, squirrel, toadstool static+stripped; biomeos static NOT stripped.

**PLASMIBIN-STALE RESOLVED.** All x86_64 binaries now include BTSP Phase 1+2 code
from the April 9 ecosystem cascade. musl-static compliance: 12/12 (was 2/11).

---

## Per-Primal Team Handoff (April 10, 2026)

Copy-paste blurbs for team assignment. Grouped by priority.

### Primals Needing Dedicated Team Evolution

**barraCuda** — BTSP Phase 2 **COMPLETE** ↑↑ (Sprint 39 — full 6-step handshake relay).
GPU panic **RESOLVED** (BC-05 — `Auto::new()` returns `Err`, health `Degraded`).
`fault_injection` test SIGSEGV persists (nextest `gpu-serial` workaround). 4,422 tests.
**Remaining**: `BufReader` lifetime edge-case in handshake relay, post-handshake stream
encryption (Phase 3), musl rebuild with Sprint 39 changes. **Effort: low.**

**coralReef** — BTSP Phase 2 **COMPLETE** ↑↑ (Iter 78 — `guard_connection()` with real
BearDog RPC in all 3 crates, degraded mode when provider absent). 7 large files split
into modules, typed driver errors Waves 1–3. 4,459 tests. `cpu_exec.rs` exists but is
dead code (not wired into `service/mod.rs`). Wire Standard L2 (singular `capability.list`).
**Remaining**: CR-04 typed errors Wave 4+ (~20 HW functions still `Result<_, String>`),
CR-05 `cpu_exec.rs` wiring, crypto socket discovery path alignment across crates, musl
rebuild with Iter 78 changes. **Effort: low-medium.**

### Deferred (later development cycle)

~~**skunkBat**~~ — **BTSP Phase 2 RESOLVED** (v0.1.0: `PeekedStream` UDS peek + BearDog v0.9.0 alignment).
Remaining: thymic selection impl (blocked on BearDog `lineage.list`), composable primitives IPC
registration (blocked on biomeOS Neural API), coverage 89.6%→90%, `PeekedStream` convergence.

**sourDough** — `deny.toml` missing, musl build, genomeBin signing. Scaffolding CLI tool, not IPC primal. Deferred to later cycle.

### Primals With Tractable Local Fixes

**biomeOS** — BM-10: method translation **RESOLVED**. BM-11: ToadStool dual-socket
**RESOLVED** (`prefers_jsonrpc` + `.jsonrpc.sock` sibling check). **All tractable biomeOS gaps resolved.**

**ToadStool** — TS-01: coralReef discovery **RESOLVED** (`capability.discover("shader")` Tier 1).
Compute socket resolution fully functional via BM-11 (`prefers_jsonrpc` flag + `.jsonrpc.sock`
sibling preference). **All tractable ToadStool gaps resolved.**

**Songbird** — Wave 146-147: stadial dyn audit, mock isolation, hardcoded elimination,
dead feature removal. SB-02: `ring` lockfile ghost — **STADIAL DEBT** (Wave 146 analysis
in progress, not yet eliminated from `Cargo.lock`). SB-03: `sled` no longer in lockfile.
Discovery abstraction layer refactored (adapters enum dispatch). `deny.toml` hardened.

**petalTongue** — PT-10 `--socket` **RESOLVED**, PT-11 domain symlinks **RESOLVED** (`ui`, `interaction`, `visualization`).
Remaining: PT-04 HTML export (partial), PT-06 push delivery (`callback_tx` not activated), PT-09 BTSP Phase 2 stub.
**Effort: low-medium. Functional for NUCLEUS.**

**NestGate** — aarch64-musl segfault **RESOLVED** (static-PIE + musl ≤1.2.2 root cause;
`-C relocation-model=static` in `.cargo/config.toml` for both x86_64 and aarch64 targets).
All gaps resolved. **Reference standard alongside BearDog.**

**loamSpine** — LS-03 startup crash **RESOLVED** (v0.9.15 — infant discovery graceful
degradation). No `--socket` CLI flag (uses plain socket fallback). Connection closes after
first response — primalSpring now calls `capabilities()` before `health_check()` as
workaround. **Effort: trivial (connection reuse would be nice but not blocking).**

### Reference Standard Primals (Working Well)

**BearDog** — Gold standard. Zero-port, BTSP Phase 2+3 complete, 14,593 tests, 90.51%
coverage, all files under 1000 LOC, dynamic `ipc.register`, standalone startup. Only minor:
`btsp.negotiate` vs `btsp.session.negotiate` metadata inconsistency.

**Songbird** — Zero-port default, federation opt-in via `--port`. BTSP Phase 2 complete,
Wire Standard L3. The gateway model for all external communication.

**Squirrel** — Zero-port, BTSP Phase 2 complete, AI provider chain fully operational
(Squirrel → OpenAI adapter → Songbird → Ollama). 7,203 tests. Wire Standard L2.
**Inference provider bridge**: `inference.complete`/`embed`/`models` via ecoPrimal wire standard.

**biomeOS** — Orchestration substrate. BTSP Phase 2 complete, 7,724 tests, registry routing
fixed (BM-07/08/09 + April 15 family-id propagation), BM-10 method translation + BM-11
ToadStool dual-socket **RESOLVED**. Graph executor now reports per-node errors in
`graph.status`. **All composition gaps resolved.** `exp091` 12/12 pass, `exp094` 19/19 pass.

**NestGate** — 11,856 tests, BTSP Phase 2 complete, Wire Standard L3. `--socket` wired.
Fully functional on x86_64.

**ToadStool** — 21,600 tests, BTSP Phase 2 complete, Wire Standard L3. Socket separation
complete (JSON-RPC vs tarpc). `--socket` wired.

**Provenance Trio (rhizoCrypt + loamSpine + sweetGrass)** — All three BTSP Phase 2 complete,
Wire Standard L2/L3. Witness wire (`WireWitnessRef`) fully standardized. rhizoCrypt uses
local crypto (self-sovereign), loamSpine/sweetGrass delegate to BearDog.

### Downstream (NOT in this registry — reference only)

**ludoSpring** — Spring (not a primal). Binary NOT in plasmidBin. IPC surface: 8 `game.*`
methods; esotericWebb needs 6 more. See `graphs/downstream/ludospring_proto_nucleate.toml`.

**esotericWebb** — Garden/composition (not a primal). Binary NOT in plasmidBin. Transport
needs UDS negotiation. See `graphs/downstream/esotericwebb_proto_nucleate.toml`.

---

## Priority Order

**0 HIGH blockers. 2 MEDIUM. 7 LOW. Zero runtime blockers.** (sourDough + skunkBat deferred)

**High**: ~~PLASMIBIN-STALE~~ **RESOLVED** (April 10 — full musl-static rebuild, 12/12 ecoBin).

**Medium** (degrades composition/experiment quality):
1. **BTSP-BARRACUDA-WIRE** — barraCuda `guard_connection()` does session creation but not full X25519 challenge-response on client stream
2. **IONIC-RUNTIME** — Ionic bond propose→accept→seal needs BearDog `crypto.sign_contract`

**Resolved this session (April 10 NUCLEUS patterns)**:
- ~~**NESTGATE-UDS**~~ **RESOLVED** — `--socket` CLI flag added and wired through dispatch → `NESTGATE_SOCKET` env var → `SocketConfig` tier-1 resolution. C5 now PASS (5/5).
- ~~**TS-UDS-JSONRPC**~~ **RESOLVED** — JSON-RPC gets dedicated `compute.jsonrpc.sock` socket, separate from tarpc `compute.sock`. Legacy symlinks for both protocols. `--socket` CLI flag wired to `run_server_main`.
- ~~**NEURAL-API-DOUBLE-PREFIX**~~ **RESOLVED** (prior session) — `capability.call` strips leading domain prefix from operation parameter.
- **BTSP-CLIENT** — primalSpring BTSP client handshake implemented (`btsp_handshake.rs`), integrated into `Transport::connect()` with auto-detection via `security_mode_from_env()`.

**Stadial Debt** (blocks parity gate — must resolve before next interstadial):
3. **SB-02** — `ring` in Songbird `Cargo.lock` (trace + eliminate transitive puller)
4. **SB-03** — `sled` default-on in Songbird orchestrator/sovereign-onion (remove from defaults)
5. **PT-09** — petalTongue Phase 2 stub (warn-only, no enforcement)
6. ~~**PT-DOMAINS**~~ **RESOLVED** (April 10 — `ui.sock` + `interaction.sock` symlinks added)
7. ~~**CR-03**~~ **RESOLVED** (Iter 78 — `guard_connection()` with real BearDog RPC, degraded when absent)
8. ~~**BC-GPU-PANIC (BC-05)**~~ **RESOLVED** (Sprint 39 — `Auto::new()` → `Err`, health `Degraded`)
9. ~~**EXP091-REGISTRY**~~ **RESOLVED** (April 10 — `get_family_id()` → `self.family_id`; socket alias mapping)
10. ~~**EXP-TCP-UDS**~~ — exp085/exp090 use TCP by design (crypto lifecycle, LAN probe). Ports env-configurable via `BEARDOG_PORT`/`SONGBIRD_PORT`. Not a gap — UDS experiments use `CompositionContext`
11. ~~**BTSP-E2E**~~ **RESOLVED** (April 14 — `AtomicHarness` now generates deterministic BTSP seed via HKDF-SHA256, injects `FAMILY_SEED` env on all child primals, uses `PrimalClient::connect_btsp` for BTSP-model primals. BearDog socket timeout unblocked for exp061-068)

**Deferred** (later development cycle):
- **SD-01/02/03** — sourDough `deny.toml`, musl, genomeBin signing
- ~~**SKUNKBAT-BTSP-P2**~~ **RESOLVED** — v0.1.0: `PeekedStream` UDS peek + BearDog v0.9.0 alignment

---

## Guideline Compliance Matrix (April 9, 2026)

| Primal | Clippy | Fmt | `deny.toml` | License | Edition | Tests | BTSP P1 | BTSP P2 | Wire |
|--------|--------|-----|-------------|---------|---------|-------|---------|---------|------|
| biomeOS | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (7,801)** ↑ | **PASS** | **PASS** ↑↑ | consumer |
| BearDog | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (14,787)** ↑ | **PASS** | **PASS** | **L2** |
| Songbird | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (7,359)** ↑ | **PASS** | **PASS** ↑↑ | **L3** |
| NestGate | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (11,856)** | **PASS** | **PASS** | **L3** |
| petalTongue | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (6,100)** ↑ | **PASS** ↑↑ | **PASS** ↑↑ | **L2** |
| Squirrel | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (7,160)** ↑ | **PASS** | **PASS** ↑↑ | **L3** ↑ |
| toadStool | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (21,700)** ↑ | **PASS** | **PASS** ↑↑ | **L3** |
| sweetGrass | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (1,560)** | **PASS** | **PASS** ↑↑ | **L3** |
| rhizoCrypt | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (1,507)** | **PASS** | **PASS** ↑↑ | **L3** |
| loamSpine | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (1,442)** | **PASS** | **PASS** ↑↑ | **L2** |
| barraCuda | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (4,393)** ↑ | **PASS** | **PASS** ↑↑ | **L2** |
| sourDough | **CLEAN** | **PASS** | **MISSING** | `-or-later` | 2024 | **PASS (239)** | FAIL | — | NONE |
| coralReef | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (4,506)** ↑ | **PASS** ↑↑ | **PASS** ↑↑ | **L2** ↑ |
| bingoCube | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | PASS | N/A | N/A | NONE |
| skunkBat | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (171)** | **PASS** ↑↑ | **PASS** ↑↑ | **L2** ↑ |

**Legend**: ↑ = improved since last audit. BTSP P1 = socket naming + insecure guard. BTSP P2 = handshake on accept/client. Wire = Capability Wire Standard level.

### Compliance Evolution (April 9 — BTSP Phase 2 ecosystem cascade)

**BTSP Phase 2 rollout COMPLETE.** **13/13** primals enforce full handshake on accept.
All 13 primals have Phase 1 + Phase 2 (guard + socket naming + handshake on accept).
**Tower Atomic: 100%. Node Atomic: 100%. NUCLEUS: 100%. All primals: 100%.**
primalSpring itself: clippy ZERO warnings, fmt PASS, all tests PASS.

1. **Songbird**: **BTSP Phase 2 COMPLETE** ↑↑ (Wave 133) — `perform_server_handshake()` in `ipc/btsp.rs`, wired into UDS accept loop, BearDog delegation via `SecurityRpcClient`. `BtspClient` + connection managers.
2. **ToadStool**: **BTSP Phase 2 COMPLETE** ↑↑ (S198) — `BtspServer::accept_handshake` on JSON-RPC Unix + tarpc paths, feature-gated. `BtspClient`. Fuzz targets (`fuzz_btsp_framing.rs`).
3. **barraCuda**: **BTSP Phase 2 COMPLETE** ↑↑ (Sprint 39) — `guard_connection()` full 6-step handshake relay in all 3 accept loops. BearDog delegation via capability-based `crypto` socket discovery. Legacy clients degrade (2s timeout).
4. **rhizoCrypt**: **BTSP Phase 2 COMPLETE** ↑↑ (S31) — `BtspServer::accept_handshake` in UDS accept. Local crypto (self-sovereign — HKDF/X25519/HMAC-SHA256, no BearDog delegation).
5. **loamSpine**: **BTSP Phase 2 COMPLETE** ↑↑ — `perform_server_handshake` in UDS accept, BearDog delegation (`btsp.session.create/verify/negotiate`). Mock tests.
6. **sweetGrass**: **BTSP Phase 2 COMPLETE** ↑↑ — `perform_server_handshake` in UDS + TCP accept, BearDog delegation. Client `perform_handshake` in integration crate.
7. **petalTongue**: **BTSP Phase 2 COMPLETE** ↑↑ (Apr 15) — real BearDog handshake delegation: `perform_server_handshake` in UDS+TCP accept, length-prefixed framing, `btsp.session.create/verify/negotiate` provider client. TCP first-byte peek (`{` → plain JSON-RPC for biomeOS). `BtspHandshakeConfig::from_env()` for production gating.
8. **coralReef**: **BTSP Phase 2 COMPLETE** ↑↑ (Iter 78) — `guard_connection()` in all 3 crates (async core/glowplug, blocking ember). Real UDS RPC to BearDog `btsp.session.create`. Degraded mode when provider absent. **Wire Standard L2** ↑ (`capability.list` + flat `methods`). 7 large files split into modules, typed driver errors (Waves 1–3).
9. **skunkBat**: **BTSP Phase 2 COMPLETE** ↑↑ (Apr 15) — real BearDog handshake delegation: `perform_server_handshake` in TCP+UDS accept, length-prefixed framing, provider client for `btsp.session.create/verify/negotiate`. TCP first-byte peek. `BtspHandshakeConfig::from_env()`.
10. **BearDog**: Wave 33 — **BD-01 RESOLVED** (per-field encoding hints + semantic aliases). 90.51% coverage. 14,593+ tests. `#[allow(` 193→75. `runtime.rs` 1244→360 LOC. Dynamic `ipc.register`. Standalone startup (`standalone-{uuid}`). 0 files over 1000 LOC. Minor: `btsp.negotiate` vs `btsp.session.negotiate` metadata inconsistency.
11. **Squirrel/biomeOS/NestGate**: Phase 2 complete (wave 2, unchanged).

---

---

## Class 5: Live NUCLEUS Deployment Gaps (April 12, 2026)

Discovered during `exp094_composition_parity` against a running NUCLEUS stack
(all ecoBins from plasmidBin, `nucleus_launcher.sh start`).

### Revalidation (April 12 — post-upstream evolution)

All 5 core primals pulled, rebuilt, tests run, ecoBins refreshed to plasmidBin.
Upstream claims: LD-03 resolved (NestGate Session 43), LD-04 resolved (ToadStool
S203b), LD-05 resolved (barraCuda Sprint 42).

**exp094 result: 19/19 PASS, 0 FAIL, 0 SKIP** — ALL PASS. Full NUCLEUS composition validated.

| ID | Primal | Gap | Status |
|----|--------|-----|--------|
| ~~**LD-01**~~ | BearDog | `crypto.hash` expects base64 `data` param | **RESOLVED** — `CompositionContext::hash_bytes()` handles encoding round-trip. `crypto_hash_nonempty` PASS, `crypto_hash_deterministic` PASS |
| ~~**LD-02**~~ | Songbird | `ipc.resolve` expects `primal_id` not `capability` | **RESOLVED** (wire) — Songbird Wave 137b accepts `capability` param. **NEW GAP LD-08**: Songbird still returns "Primal not found" for `beardog`/`toadstool`/`nestgate` — primals need runtime `ipc.register` with Songbird for resolve to work |
| ~~**LD-03**~~ | NestGate | UDS single-shot connection | **RESOLVED UPSTREAM** — NestGate Session 43 keep-alive. `storage_roundtrip_match` PASS (put + get works). Health check PASS |
| ~~**LD-04**~~ | ToadStool | UDS connection: BTSP framing only | **RESOLVED** — ToadStool S203d: `handle_btsp_connection` auto-detects plain-text vs BTSP binary via first-byte inspection. Raw JSON-RPC connections degrade gracefully. `compute_dispatch_alive` PASS, `health.liveness` responds to raw JSON-RPC |
| ~~**LD-05**~~ | barraCuda | Internal `Address in use` on startup | **RESOLVED** — Sprint 42 phase 2: eliminated TCP sidecar in UDS mode. Root cause: `nucleus_launcher.sh` passed `--unix barracuda-nucleus01.sock` conflicting with barraCuda's own socket+symlink creation (`math-{family}.sock` + `barracuda-{family}.sock` → symlink). Launcher updated to omit `--unix`. barraCuda ALIVE |
| ~~**LD-06**~~ | rhizoCrypt | Socket naming / TCP-only | **MITIGATED** — launcher alias sweep. rhizoCrypt still TCP-only (ports 9400/9401), no UDS socket. `dag` capability SKIP in exp094 |
| ~~**LD-07**~~ | All primals | Health format inconsistency | **RESOLVED** — `CompositionContext::health_check()` normalizes. BearDog, Songbird, NestGate, Squirrel, sweetGrass all PASS |
| ~~**LD-08**~~ | Songbird | `ipc.resolve` returns "Primal not found" | **RESOLVED** — Two-part fix: (1) Songbird Wave 138 scans `$XDG_RUNTIME_DIR/biomeos/*.sock` at startup, probes primals via `identity.get` + `capabilities.list`. (2) `nucleus_launcher.sh` Phase 5 seeds Songbird registry via `ipc.register` after all primals start. `resolve_security`, `resolve_compute`, `resolve_storage` all PASS |
| ~~**LD-09**~~ | loamSpine | Port 8080 conflict on startup | **RESOLVED** — loamSpine LD-09 commit: TCP transports (tarpc + JSON-RPC HTTP) now opt-in via `--port`/`--tarpc-port` flags or `LOAMSPINE_*_PORT` env vars. UDS socket unconditional. loamSpine ALIVE in NUCLEUS |
| ~~**LD-10**~~ | barraCuda | UDS socket uses tarpc, not JSON-RPC | **RESOLVED** — barraCuda Sprint 42 phase 5: `fix(LD-10): replay consumed BTSP guard line to JSON-RPC handler`. JSON-RPC now works on UDS. `stats.mean`, `stats.weighted_mean`, `capabilities.list` all respond. `tensor_stats_mean` parity check PASS |

### NUCLEUS Stack Status (April 13 revalidation — **12/12 ALIVE, 19/19 PASS, 0 FAIL, 0 SKIP**)

| Primal | Socket | Health | IPC Verified | Notes |
|--------|--------|--------|--------------|-------|
| **BearDog** | `beardog-nucleus01.sock` | ALIVE | `crypto.hash` PASS (BLAKE3 base64), deterministic PASS | Gold standard |
| **Songbird** | `songbird-nucleus01.sock` | ALIVE | `rpc.discover` PASS (67 methods), `ipc.resolve` PASS (Phase 5 registry seeding) | LD-08 RESOLVED |
| **ToadStool** | `toadstool-nucleus01.sock` | ALIVE | `health.liveness` PASS, `compute_dispatch_alive` PASS | LD-04 RESOLVED (BTSP auto-detect) |
| **barraCuda** | `math-nucleus01.sock` + symlink | ALIVE | `stats.mean` PASS, `capabilities.list` PASS (32 methods) | LD-05 RESOLVED, LD-10 RESOLVED (JSON-RPC works) |
| **coralReef** | `shader.sock` | ALIVE | `shader.compile.capabilities` PASS (11 GPU archs) | Fully functional |
| **NestGate** | `nestgate-nucleus01.sock` | ALIVE | `storage.put` + `storage.get` roundtrip PASS | Keep-alive RESOLVED (LD-03) |
| **Squirrel** | `squirrel-nucleus01.sock` | ALIVE | Health PASS | AI provider chain |
| **sweetGrass** | `sweetgrass-nucleus01.sock` | ALIVE | Health PASS | Provenance |
| **rhizoCrypt** | `rhizocrypt-nucleus01.sock` | ALIVE | `health.liveness` PASS (UDS) | LD-06 RESOLVED (S37 UDS unconditional) |
| **loamSpine** | `loamspine-nucleus01.sock` | ALIVE | `health.liveness` PASS (JSON-RPC) | LD-09 RESOLVED (TCP opt-in) |
| **petalTongue** | `petaltongue-nucleus01.sock` | ALIVE | Socket active | `--socket` CLI added (Sprint 6) |

### Cross-Atomic Pipeline (April 13 — **ALL PASS**)

**Tower Atomic**: FULLY OPERATIONAL. Health, crypto hash (base64 round-trip),
method catalog (67 methods), capability resolution via Songbird (`ipc.resolve`
PASS for security, compute, storage after Phase 5 registry seeding).

**Node Atomic**: OPERATIONAL. coralReef shader capabilities work (11 GPU archs).
ToadStool alive with BTSP auto-detect — raw JSON-RPC health PASS (LD-04 resolved).
barraCuda ALIVE (LD-05 resolved) — tarpc transport, `tensor.dot` gracefully SKIP
pending JSON-RPC bridge (LD-10).

**Nest Atomic**: FULLY OPERATIONAL. NestGate storage roundtrip PASS (LD-03
resolved). sweetGrass health PASS. loamSpine ALIVE and health PASS (LD-09
resolved). rhizoCrypt TCP-only (SKIP — low priority).

**Full NUCLEUS cross-atomic pipeline**: **PASS** — hash (Tower/BearDog) → store
(Nest/NestGate) → retrieve (Nest/NestGate) → verify matches. End-to-end
composition integrity confirmed across all 3 atomics.

### Remaining Blockers for Full Composition

| Priority | Gap | Owner | What Springs Need |
|----------|-----|-------|-------------------|
| ~~**High**~~ | ~~LD-05: barraCuda internal AddrInUse~~ | ~~barraCuda team~~ | **RESOLVED** — launcher `--unix` override removed; barraCuda manages own sockets |
| ~~**High**~~ | ~~LD-04: ToadStool BTSP-only socket~~ | ~~primalSpring + ToadStool~~ | **RESOLVED** — S203d BTSP auto-detect; raw JSON-RPC works |
| ~~**Medium**~~ | ~~LD-08: Songbird resolve has no registrations~~ | ~~Songbird + all primals~~ | **RESOLVED** — Wave 138 auto-discovery + Phase 5 launcher seeding |
| ~~**Medium**~~ | ~~LD-09: loamSpine port 8080 conflict~~ | ~~loamSpine team~~ | **RESOLVED** — TCP opt-in, UDS unconditional |
| ~~**Low**~~ | ~~LD-10: barraCuda tarpc-only UDS~~ | ~~barraCuda team~~ | **RESOLVED** — Sprint 42 phase 5 replays BTSP guard line to JSON-RPC handler |
| ~~**Low**~~ | ~~LD-06: rhizoCrypt TCP-only~~ | ~~rhizoCrypt team~~ | **RESOLVED** — S37: UDS unconditional, TCP opt-in. `rhizocrypt_alive` PASS |

---

## Post-Pull Resolution Wave (April 13, 2026 — Phase 41)

After pulling all upstream primals and reviewing commit evolution, the following
gaps moved to RESOLVED. NestGate needs more time (no new commits).

| Gap | Primal | Resolved In | How |
|-----|--------|-------------|-----|
| `inference.register_provider` wire method | Squirrel | alpha.49 | 5 wire tests, real handler path |
| Stable ecoBin binary | Squirrel | alpha.49 | 3.5MB static-pie, stripped, BLAKE3, zero host paths |
| Ionic bond lifecycle (`crypto.ionic_bond`) | BearDog | Wave 42 | `seal` step: propose→accept→seal with Ed25519, proposal TTL |
| BTSP server endpoint (`btsp.server.*`) | BearDog | Wave 36 | `create_session`, `verify`, `negotiate`, `status` wired |
| `health.check` accepts empty params | loamSpine | deep debt pass | `#[serde(default)]` on `include_details`, null→{} normalization |
| `EVENT_TYPE_REFERENCE.md` for domain springs | rhizoCrypt | S40 | Canonical 27-variant spec in rhizoCrypt repo |
| `capability.call` gate routing | biomeOS | v3.05 | Explicit error on unregistered gate, `gate="local"` support |
| `--port` in api/nucleus modes | biomeOS | v3.05 | TCP listener alongside UDS for mobile/cross-gate |
| biomeOS DOWN during testing | biomeOS | v3.05 | Neural API co-launch in Nucleus Full mode |
| LD-10 BTSP guard line consumed | barraCuda | Sprint 42 | Replay consumed line in `BtspOutcome::Degraded` |
| LD-05 TCP AddrInUse co-deployment | barraCuda | Sprint 42 | Eliminated TCP sidecar in UDS mode |
| BC-09 `--port` Docker TCP bind | barraCuda | Sprint 42 | `resolve_bind_host()` respects `BARRACUDA_IPC_HOST` for cross-container TCP |

### Remaining Open Upstream Gaps

| Priority | Gap | Owner | Status |
|----------|-----|-------|--------|
| ~~**High**~~ | ~~NG-08: Eliminate `ring` from production build~~ | ~~NestGate~~ | **RESOLVED** — Session 43: reqwest→ureq 3.3 + rustls-rustcrypto, pure Rust TLS |
| ~~Medium~~ | ~~BC-07: `SovereignDevice` into `Auto::new()` fallback~~ | ~~barraCuda~~ | **RESOLVED** — Sprint 41: 3-tier fallback (wgpu GPU → CPU → SovereignDevice IPC) |
| ~~Medium~~ | ~~BC-08: `cpu-shader` default-on~~ | ~~barraCuda~~ | **RESOLVED** — Sprint 40: default feature, ecoBin computes without wgpu |
| ~~Medium~~ | ~~CR-01: `deny.toml` C/FFI ban list~~ | ~~coralReef~~ | **RESOLVED** — Iter 79: ecoBin v3 ban list, cudarc behind feature gate |
| ~~Medium~~ | ~~Multi-stage ML pipeline `shader.compile.wgsl`~~ | ~~coralReef~~ | **RESOLVED** — 6 end-to-end pipeline composition tests, CompilationInfo IPC |
| ~~Low~~ | ~~Signed capability announcements~~ | ~~BearDog~~ | **RESOLVED** — Wave 45: SA-01, Ed25519 signed attestation |
| ~~Low~~ | ~~`plasma_dispersion` feature-gate bug~~ | ~~barraCuda~~ | **RESOLVED** — Sprint 40: corrected to dual feature gate |
| Medium | `storage.retrieve` for large/streaming tensors | NestGate | OPEN |
| Medium | Cross-spring persistent storage IPC | NestGate | OPEN |
| ~~Medium~~ | ~~`TensorSession`/`BatchGuard` adoption by springs~~ | ~~barraCuda~~ | **RESOLVED** — Sprint 40: renamed, migration guide published in `BREAKING_CHANGES.md` (§TensorSession/BatchGuard Migration Guide). `tensor.batch.submit` IPC method wired (Sprint 42). Spring-side adoption is coordination work |
| Low | 29 shader absorption candidates | barraCuda | neuralSpring pipeline (they submit PRs per shader) |
| ~~Low~~ | ~~RAWR GPU kernel (CPU-only)~~ | ~~barraCuda~~ | **RESOLVED** — `RawrWeightedMeanGpu` + `rawr_weighted_mean_f64.wgsl` GPU shader already exist in `barracuda/src/ops/`. CPU `rawr_mean` also available in `stats/bootstrap.rs`. Both paths working |
| Low | Batched `OdeRK45F64` for Richards PDE | barraCuda | airSpring-specific (single-trajectory loop sufficient for now) |
| Low | IPC timing for `shader.compile` | coralReef | Deployment timing |

## Post-Pull Resolution Wave (April 14, 2026 — Phase 42)

Pulled all upstream primals. biomeOS, NestGate, loamSpine, toadStool, coralReef,
BearDog received new commits. Squirrel reviewed locally (alpha.52). barraCuda,
Songbird, petalTongue, rhizoCrypt, sweetGrass already up to date.

### Key Upstream Evolution

| Primal | Version | Tests | What Changed |
|--------|---------|-------|--------------|
| **BearDog** | Wave 50 | 14,784 | **TS-01**: `transport_security` in `capabilities.list` (btsp_required, btsp_version, cleartext_available). BTSP rejection sends JSON-RPC -32600 error. Deep debt (Wave 49: workspace deps, large file refactor). TCP skip when `--port` not passed. |
| **biomeOS** | v3.13 | 7,695+ | Hardcoded primal names → capability constants. `learn_from_event` implemented. Topology uses live health probes. Composition forwarding via Tower Atomic relay. Recursive `graph.list`. |
| **NestGate** | Session 43n | 11,819 | Semantic router streaming parity (5 storage streaming methods). Event-driven connection lifecycle (`select!` idle timeout). Deep debt: zero `dyn Error`, zero `async-trait`. |
| **loamSpine** | 0.9.16+ | 1,442 | **Stadial gate cleared** — sled + sqlite removed; `bincode` → `rmp-serde` (RUSTSEC-2025-0141 eliminated); biomeOS doc refs 29→0 (self-knowledge compliant); **hickory-resolver** 0.26; lockfile clean except upstream **hickory-net** `async-trait`. |
| **toadStool** | S203i | 21,600+ | Test extraction from 52 production files. TCP idle timeout (exp082 half-open fix). BTSP auto-detect (LD-04). `compute.execute` direct route. Pipeline methods in capabilities. |
| **coralReef** | Iter 80 | 4,506 | `--bind` flag + `CORALREEF_IPC_HOST` for network-facing. Feature-gate VFIO constructors. `#[must_use]` audit. Multi-stage ML pipeline tests (6 new). |
| **Squirrel** | alpha.52 | 7,203 | **SQ-04 RESOLVED**: `--bind` CLI + `SQUIRREL_BIND` / `SQUIRREL_IPC_HOST`. Smart refactoring (9 large files split). Capability-first naming (toadstool→compute stem). `hostname` dep removed. BTSP Phase 2 complete, Phase 3 (cipher negotiation) deferred. |

### New Resolutions

| Gap | Primal | Resolved In | How |
|-----|--------|-------------|-----|
| TS-01 Transport security advertisement | BearDog | Wave 48 | `transport_security` block in `capabilities.list` + `discover_capabilities` — btsp_required, version, cleartext_available. Programmatic BTSP detection for biomeOS and AtomicHarness |
| SQ-04 `--bind` TCP bind hardcoded | Squirrel | alpha.52 | `--bind` CLI flag + `SQUIRREL_BIND` / `SQUIRREL_IPC_HOST` env vars. Default `127.0.0.1`. Docker: `--bind 0.0.0.0` |
| BTSP provider coupling | loamSpine | 0.9.16+ | BTSP module decoupled from BearDog identity (`beardog_client.rs` → `provider_client.rs`). Any security provider can serve BTSP sessions |
| TCP idle timeout (exp082 half-open) | toadStool | S203h | Resolves benchScale half-open connection finding from chaos substrate experiments |
| Composition forwarding gaps | biomeOS | v3.12–v3.13 | Tower Atomic relay for `capability.call`, recursive `graph.list`, BTSP handshake failure warnings with socket path |

### Post-Pull Resolution Wave (April 16, 2026 — Phase 44)

Pulled all primals except NestGate (still evolving). Reviewed local pushes for
skunkBat and Squirrel. Massive async-trait progress: **9/13 primals at zero** (was 6/13).

| Gap | Primal | Resolved In | How |
|-----|--------|-------------|-----|
| async-trait elimination | Songbird | Wave 145 | 141→0 across 17 crates, full dyn→static dispatch |
| async-trait elimination | petalTongue | Sprint 8 | 47→0, RPITIT throughout |
| async-trait elimination | rhizoCrypt | S43 | Crate removed, `ProtocolAdapter` uses manual `BoxFuture` |
| async-trait elimination | coralReef | Iter 83 | jsonrpsee removed, pure NDJSON/TCP dispatch |
| dyn→static dispatch | Songbird | Wave 144 | `PeerConnection` enum (6 types), `BtspProviderImpl`, `SecurityProviderImpl` |
| Content distribution federation | Songbird | Wave 143 | `discovery.content_peers`, `ContentAnnouncementStore` with TTL |
| syn compile surface | BearDog | Wave 52 | `async-trait` dep removed from 5 crates |
| BTSP Phase 2 UDS peek | skunkBat | v0.1.0 | `PeekedStream` custom wrapper, BearDog v0.9.0 alignment |
| Monitoring real impl | toadStool | S203o | `toadstool_sysmon` + rustix `statvfs`, real workload ID |
| Storage real behavior | toadStool | S203o | RPC failure → `StorageStatus::LocalOnly` |
| Env interning complete | toadStool | S203p | All `TOADSTOOL_*` → `socket_env::*` (~55 constants) |
| Resource estimator/optimizer | toadStool | S203p | Topological sort, diamond DAG, cost/allocation tests |
| capabilities.list L2→L3 | Squirrel | latest | Composable group descriptions from `niche::CAPABILITY_GROUP_DESCRIPTIONS` |
| Security hardcoding→capability | Squirrel | latest | `SECURITY_SERVICE_ID`, `supports_security_provider`, agnostic naming |
| Entity module refactor | sweetGrass | latest | `entity.rs` → `entity/mod.rs` + `entity/tests.rs` (803→483 LOC) |
| jsonrpsee removal | coralReef | Iter 83 | Pure NDJSON/TCP, dropped jsonrpsee + transitive async-trait/hyper/tower |
| VFIO/nvidia test extraction | coralReef | Iter 82 | `registers_tests.rs`, `nvidia_headers_tests.rs`, firmware parser split |
| Fractal compute refactor | biomeOS | v3.17 | `fractal/mod.rs` + `leaf.rs` + `parent.rs`, enum-dispatch |
| Dep pruning + manifest | biomeOS | v3.17 | tokio removed from biomeos-types, placeholder features removed |
| Crypto model authoritative | rhizoCrypt | S43 | `specs/CRYPTO_MODEL.md` — BearDog delegation canonical |
| cli_mode refactor | petalTongue | Sprint 8 | `gather.rs`, `output.rs`, `types.rs`, `tests.rs` module split |

### Remaining Open Upstream Gaps (refreshed April 16)

| Priority | Gap | Owner | Status |
|----------|-----|-------|--------|
| Medium | `storage.retrieve` for large/streaming tensors | NestGate | OPEN (NestGate still evolving) |
| Medium | Cross-spring persistent storage IPC | NestGate | OPEN (NestGate still evolving) |
| Medium | Songbird coverage 72%→90% | Songbird | **72.29%** — main remaining quality debt |
| Medium | coralReef coverage ~65%→90% | coralReef | **~65%** — significant gap |
| Low | 29 shader absorption candidates | barraCuda | neuralSpring pipeline (submit PRs per shader) |
| Low | Batched `OdeRK45F64` for Richards PDE | barraCuda | airSpring-specific |
| Low | IPC timing for `shader.compile` | coralReef | Deployment timing |
| Low | BTSP Phase 3 (encrypted post-handshake channel) | All primals | Deferred — Phase 2 NULL cipher operational everywhere |
| Low | Genetics three-tier awareness in primals | All primals | **primalSpring RPC client aligned** (April 15). BearDog has `genetic.*` RPCs. ecoPrimal `genetics::rpc` now matches BearDog's actual API. No primal has consumed `GeneticSecurityMode` or `MitoBeacon`/`NuclearGenetics` types yet — adoption awaits ecoPrimal ≥0.10.0 |
| Low | skunkBat thymic selection impl | skunkBat + BearDog | Blocked on BearDog `lineage.list` + `btsp.session.verify` IPC |
| Low | skunkBat composable primitives IPC | skunkBat + biomeOS | 5 domains defined, Neural API registration pending |
| Low | `PeekedStream`/`PrefixedStream` convergence | skunkBat + BearDog | Two independent impls — consolidate to `sourdough-core` |
| Low | toadStool coverage 83.6%→90% | toadStool | S203p pushed toward, not yet at target |
| Low | BearDog async-trait 49→0 | BearDog | Continuing syn elimination |
| Low | sweetGrass async-trait 22→0 | sweetGrass | 5 object-safe traits constrain further reduction |
| Low | skunkBat async-trait 14→0 | skunkBat | Threat/recon traits |

### Genetics Posture (April 15, 2026 — RPC client aligned)

primalSpring's `ecoPrimal::genetics` module defines the three-tier model:

| Tier | Type | Where Implemented | Primal Awareness |
|------|------|------------------|-----------------|
| 1 | `MitoBeacon` | ecoPrimal + BearDog (`genetic.derive_lineage_beacon_key`) | **ecoPrimal RPC client aligned** (April 15). BearDog serves RPC; no primal consumes yet |
| 2 | `NuclearGenetics` | ecoPrimal + BearDog (`genetic.derive_lineage_key`, `mix_entropy`, `verify_lineage`) | **ecoPrimal RPC client aligned** (April 15). BearDog serves RPC; no primal consumes yet |
| 3 | `GeneticTag` | ecoPrimal (`from_legacy_family_seed()`) | Bridge for legacy `FAMILY_SEED` — all primals still use flat seed |

**April 15 — Genetics RPC client alignment**: `ecoPrimal::genetics::rpc` param/response types
realigned to BearDog's actual JSON-RPC surface. `DeriveLineageKeyParams` now sends
`{our_family_id, peer_family_id, context, lineage_seed}` (was fictional `{domain, generation}`).
`LineageKeyResult` expects `{key}` (was `{lineage_key, generation, parent_hash}`).
`MixEntropyParams` sends `{tier3_human, tier2_supervised, tier1_machine}` (was `{tiers: [...]}`).
`VerifyLineageParams` sends `{lineage_proof}` (was `{proof}`). All encodings corrected (base64
for keys/proofs, hex for beacon keys). exp096 params also aligned.

**Note**: BearDog's `generate_lineage_proof` / `verify_lineage` do not yet support generational
provenance — the proof is a static commitment given the same lineage_seed + family ID pair.
Generation tracking remains local to `NuclearGenetics`. Upstream BearDog evolution needed for
full verifiable lineage chains.

**Next evolution**: As primals pull ecoPrimal ≥0.10.0, they can adopt `mito_beacon_from_env()`
instead of `family_seed_from_env()`. BearDog's `transport_security` advertisement (TS-01)
provides the programmatic hook for biomeOS/AtomicHarness to negotiate BTSP tier. loamSpine's
provider decoupling (`provider_client.rs`) sets the pattern for other primals to follow.

---

## Next Evolution Targets (April 16, 2026)

Refreshed after full upstream pull + code review of all primals. Massive progress since
April 15 — 20+ gaps resolved upstream. Remaining items below validated by code inspection.

### Resolved Since April 15

| Gap | Primal | Evidence |
|-----|--------|----------|
| BTSP Phase 3 server negotiate | BearDog | `btsp.server.negotiate` + ChaCha20Poly1305 session crypto (Wave 42+51) |
| UDS first-byte peek | BearDog | `read_exact` + `PrefixedStream` in production UDS path (Wave 51, `c6b7f11d0`) |
| UDS first-byte peek | Songbird | `handle_connection_with_peek` via `BufReader::fill_buf()` (`464dc04f0`) |
| UDS first-byte peek | coralReef | `BufReader::fill_buf()` + `guard_from_first_byte` (`a5c95df`) |
| UDS first-byte peek | petalTongue | `BufReader::fill_buf()` on UDS read half (`1f8721e`) |
| UDS first-byte peek | skunkBat | `PeekedStream` custom wrapper on UDS — auto-detect JSON-RPC vs BTSP (v0.1.0) |
| BTSP Phase 2 real enforcement | petalTongue | BearDog delegation via `btsp.session.create/verify/negotiate` (`1f8721e`) |
| BTSP Phase 2 real enforcement | skunkBat | BearDog v0.9.0 handshake alignment, `PeekedStream` on UDS (v0.1.0) |
| BTSP Phase 3 stream encryption | barraCuda | ChaCha20Poly1305 AEAD + BtspFrameReader/Writer (`6284469e`) |
| BufReader lifetime edge-case | barraCuda | Single BufReader for handshake, writes via `get_mut()` (`6284469e`) |
| Genetic RPC → chain proofs | BearDog | `LineageProofManager` wired into RPC handlers with `chain_id` dispatch + Blake3 fallback (Wave 51) |
| Ring elimination | BearDog | Not in Cargo.lock, banned in deny.toml (Wave 51) |
| syn compile surface reduction | BearDog | Wave 52: `async-trait` dep removed from 5 crates, `syn` surface reduced |
| Bond persistence trait | BearDog | `BondPersistence` + `InMemoryBondPersistence` + `with_persistence()` (Wave 51) |
| Graph-level genetics_tier | biomeOS | `GraphMetadata.genetics_tier: Option<GeneticsTier>` parsed + enforced (`674627bb`) |
| Deploy class auto-resolution | biomeOS | `resolve_composition()` infers from node capabilities (`674627bb`) |
| capability.call routing contract | biomeOS | `specs/CAPABILITY_CALL_ROUTING_CONTRACT.md` formalized |
| async-trait elimination | biomeOS | **0** remaining (was 72→43→0) (`580a9458`) |
| Fractal compute refactor | biomeOS | `fractal/mod.rs` + `leaf.rs` + `parent.rs`, enum-dispatch, `ResourceInfo::zeroed()` (v3.17) |
| Dep pruning + manifest hygiene | biomeOS | tokio removed from biomeos-types, placeholder features removed (v3.17) |
| Bond ledger persistence | loamSpine | Dedicated spine + in-memory index, `bonding.ledger.store/retrieve/list` (`8e1067f`) |
| Crypto signing via IPC | loamSpine | `JsonRpcCryptoSigner/Verifier` delegates to BearDog UDS (`8f508b7`) |
| Streaming storage | NestGate | `store_stream`, `store_stream_chunk`, `retrieve_stream`, `retrieve_stream_chunk` (Session 43p) |
| Doc drift (method counts) | NestGate | STATUS reconciled: 51 UDS, 23 HTTP, 42 semantic (Session 43q) |
| `data.*` delegation stub | NestGate | Removed from router entirely, tests guard against re-introduction (Session 43q) |
| async-trait + Box\<dyn Error\> | NestGate | **0 / 0** in production |
| SigningClient wire alignment | rhizoCrypt | `crypto.sign_ed25519` / `crypto.verify_ed25519` field names aligned (`17973d0`) |
| Crypto model decision | rhizoCrypt | `specs/CRYPTO_MODEL.md` — BearDog delegation canonical (`1046e6f`) |
| async-trait elimination | rhizoCrypt | **0** — crate removed, `ProtocolAdapter` uses manual `BoxFuture` (S43) |
| Files >700 LOC | petalTongue | Zero production files >680 LOC (`cf7d264`) |
| async-trait elimination | petalTongue | **0** — Sprint 8: 47→0, crate removed, RPITIT throughout |
| async-trait elimination | Songbird | **0** — Wave 145: 141→0 across 17 crates, full dyn→static dispatch |
| dyn→static dispatch | Songbird | Wave 144: `PeerConnection` enum (6 types), `BtspProviderImpl`, `SecurityProviderImpl`, `ConsentStorage`/`TaskStorage` enums |
| Content distribution federation | Songbird | Wave 143: `discovery.content_peers`, `ContentAnnouncementStore` with TTL, topic-based announce |
| async-trait elimination | Squirrel | **0** — 228→0 complete, dep removed |
| capabilities.list L2→L3 | Squirrel | Composable group descriptions from `niche::CAPABILITY_GROUP_DESCRIPTIONS` |
| Security hardcoding→capability | Squirrel | `SECURITY_SERVICE_ID`, `supports_security_provider`, agnostic naming |
| sweetGrass coverage 87→90% | sweetGrass | **91.7%** with Postgres, **90.4%** without (`34736bb`) |
| sweetGrass entity refactor | sweetGrass | `entity.rs` → `entity/mod.rs` + `entity/tests.rs` (803→483 LOC) |
| Squirrel coverage 86→90% | Squirrel | **90.1%** region coverage, 7,160 tests |
| deny.toml ring ban | toadStool | Uncommented and active (S203l) |
| Monitoring real implementations | toadStool | S203o: `toadstool_sysmon` + rustix `statvfs`, real workload ID |
| Storage real behavior | toadStool | S203o: RPC failure → `StorageStatus::LocalOnly` (not fake success) |
| Env interning complete | toadStool | S203p: all `TOADSTOOL_*` → `socket_env::*` (~55 constants) |
| Resource estimator/optimizer | toadStool | S203p: topological sort, diamond DAG, cost/allocation/bottleneck tests |
| Shader absorption audit | barraCuda | **18/18** per-shader verified (`3cdfa221`) |
| Postgres multi-statement DDL | sweetGrass | `raw_sql()` for simple query protocol (`bf7190e`) |
| jsonrpsee removal | coralReef | Iter 83: pure NDJSON/TCP, dropped jsonrpsee + transitive async-trait/hyper/tower |
| VFIO/nvidia test modules | coralReef | Iter 82: `registers_tests.rs`, `nvidia_headers_tests.rs`, firmware parser split |

### async-trait Scorecard (April 16 — refreshed)

| Primal | Before | Now | Status |
|--------|:---:|:---:|--------|
| biomeOS | 72 | **0** | **COMPLETE** |
| barraCuda | unknown | **0** | **COMPLETE** |
| Squirrel | 228 | **0** | **COMPLETE** (dep removed) |
| loamSpine | unknown | **0** | **COMPLETE** |
| NestGate | 587 Box\<dyn Error\> | **0 / 0** | **COMPLETE** (both) |
| Songbird | 141 | **0** | **COMPLETE** (Wave 145: dep removed from 17 crates) |
| petalTongue | 47 | **0** | **COMPLETE** (Sprint 8: RPITIT throughout) |
| rhizoCrypt | 6 | **0** | **COMPLETE** (S43: crate removed) |
| coralReef | unknown | **0** | **COMPLETE** (Iter 83: jsonrpsee removed) |
| BearDog | ~115 | **49** | -57%, syn surface reduced (Wave 52) |
| toadStool | 320 | **~158** | -50%, dyn-ceiling (32 dyn-dispatched traits, all `NOTE(async-dyn)`) |
| sweetGrass | 34 | **22** | 5 object-safe traits documented, rest migrated |
| skunkBat | unknown | **14** | Threat/recon traits, integrations |

### BearDog

~~BTSP Phase 3 server negotiate~~ **RESOLVED** (Wave 42+51).
~~UDS first-byte peek~~ **RESOLVED** (Wave 51) — `read_exact` + `PrefixedStream` in
production UDS path. JSON-RPC detected via `0x7B`, BTSP otherwise.
~~Genetic RPC chain proofs~~ **RESOLVED** — `LineageProofManager` wired with `chain_id` dispatch + Blake3 fallback.
~~Ring elimination~~ **RESOLVED** — not in Cargo.lock, banned in deny.toml.
~~Bond persistence trait~~ **RESOLVED** — `BondPersistence` + `InMemoryBondPersistence` + `with_persistence()` (Wave 51).
**NestGate/loamSpine wiring not yet implemented** — loamSpine bond ledger is ready upstream.
~~syn compile surface~~ Wave 52: `async-trait` dep removed from 5 crates.
HSM/Titan M2: StrongBox/mobile profiles expanded, Android `generate_key` present.
**Titan M2 not explicitly wired** as named backend for `crypto.generate_keypair`.
async-trait: **49** (was ~115). Coverage: **90.51%**. Tests: **14,787+**. 100 JSON-RPC methods.

### Songbird

~~UDS first-byte peek~~ **RESOLVED** (`464dc04f0`).
~~async-trait~~ **COMPLETE** — Wave 145: **141→0**, dep removed from 17 crates.
~~dyn→static dispatch~~ Wave 144: `PeerConnection` enum (6 types), `BtspProviderImpl`,
`SecurityProviderImpl`, `ConsentStorage`/`TaskStorage` enums. **7 trait impls converted.**
Content distribution federation (Wave 143): `discovery.content_peers` + `ContentAnnouncementStore`
with TTL, topic-based announce. Deep seeder/leecher networking not yet wired.
`ring` in `Cargo.lock` — still present (0.17.14 via rustls chain). **STADIAL DEBT** —
"managed" status revoked. Must trace the transitive puller and eliminate or swap it.
Ghost lockfile entries are not acceptable; the lockfile must be clean like the build.
Mito-beacon provider implemented with graceful fallback — depends on BearDog `beacon.*`.
Coverage: **72.29%** (target 90% — main remaining debt). Tests: **7,359**.

### NestGate

~~Doc drift~~ **RESOLVED** — STATUS now says 51 UDS, 23 HTTP, 42 semantic, matching code.
~~`data.*` stub~~ **RESOLVED** — removed from router, tests guard against re-introduction.
~~Streaming storage~~ **RESOLVED** — 4 chunk RPC methods implemented.
~~async-trait~~ **0.** ~~Box\<dyn Error\>~~ **0** in production.
~176 deprecated APIs remain (down from ~195). Coverage 82.06% → 90% target.
Tests: **8,534** (lib), **~11,800** (full). Vendored `rustls-rustcrypto` for WebPKI fixes.

### biomeOS

~~genetics_tier~~ **RESOLVED.** ~~Deploy class auto-resolution~~ **RESOLVED.**
~~capability.call routing contract~~ **RESOLVED.**
~~async-trait~~ **0** (was 72→43→0). **COMPLETE.**
~~Fractal compute~~ v3.17: `fractal/mod.rs` + `leaf.rs` + `parent.rs`, enum-dispatch, `ResourceInfo::zeroed()`.
~~Dep pruning~~ v3.17: tokio removed from biomeos-types, placeholder features removed.
Tick-loop scheduling (60Hz) remains the only major open item.
Coverage: **≥90%** (region/function/line). Tests: **7,801**.

### toadStool

async-trait: **~158** remaining (32 dyn-dispatched traits with `NOTE(async-dyn)` markers,
all justified by object safety). Further reduction requires trait redesign.
~~deny.toml ring ban~~ **RESOLVED** — active (S203l).
~~Monitoring stubs~~ S203o: `toadstool_sysmon` + rustix `statvfs`, real workload ID.
~~Storage fake success~~ S203o: RPC failure → `StorageStatus::LocalOnly`.
~~Env interning~~ S203p: all `TOADSTOOL_*` → `socket_env::*` (~55 constants).
~~Resource estimator/optimizer~~ S203p: topological sort, diamond DAG, cost/allocation tests.
V4L2 ioctl safe wrappers implemented (8 ioctls via rustix 1.x).
Real edge discovery (USB sysfs, Bluetooth) and scheduler queuing (`UniversalJobQueue`).
Coverage **83.6%** → 90% target. Tests: **21,700+**.

### barraCuda

~~BTSP Phase 3~~ **RESOLVED.** ~~BufReader~~ **RESOLVED.** ~~plasma_dispersion~~ Clean.
~~Shader absorption~~ **18/18** verified per-shader audit.
**async-trait: 0. Fully clean.** Tests: **4,393**.

### Squirrel

~~Coverage 86→90%~~ **RESOLVED** — **90.1%** region coverage.
~~async-trait~~ **COMPLETE** — 228→0, dep removed.
~~capabilities.list L2→L3~~ Composable group descriptions from `niche::CAPABILITY_GROUP_DESCRIPTIONS`.
~~Security hardcoding~~ `SECURITY_SERVICE_ID`, `supports_security_provider`, agnostic naming.
Three-tier genetics: prep/annotations only, blocked on ecoPrimal ≥0.10.0.
Content curation: blocked on NestGate content-addressed storage API.
Tests: **7,160**. Coverage: **90.1%** region / **89.6%** line.

### petalTongue

~~BTSP Phase 2~~ **RESOLVED** (real delegation). ~~UDS peek~~ **RESOLVED.**
~~Files >700 LOC~~ **RESOLVED.**
~~async-trait~~ **COMPLETE** — Sprint 8: 47→0, crate removed, RPITIT throughout.
`cli_mode` refactored: `gather.rs`, `output.rs`, `types.rs`, `tests.rs` module.
CHANGELOG doc drift: still says "stub" — should reflect real delegation.
Coverage: **~90%** line. Tests: **6,100+**.

### Provenance Trio

~~rhizoCrypt SigningClient alignment~~ **RESOLVED** (`crypto.sign_ed25519` / `verify_ed25519`).
~~rhizoCrypt crypto model~~ **DECIDED** — BearDog delegation (`specs/CRYPTO_MODEL.md` authoritative).
~~rhizoCrypt async-trait~~ **COMPLETE** (S43: crate removed, `ProtocolAdapter` uses manual `BoxFuture`).
~~loamSpine bond ledger~~ **RESOLVED.** ~~loamSpine crypto delegation~~ **RESOLVED**
(`JsonRpcCryptoSigner/Verifier`).
~~sweetGrass coverage~~ **91.7%** (target exceeded). ~~Postgres DDL~~ **RESOLVED** (`raw_sql()`).
~~sweetGrass entity refactor~~ `entity.rs` → `entity/mod.rs` + `entity/tests.rs` (803→483 LOC).
sweetGrass NestGate store backend implemented.
rhizoCrypt: DID vs raw public_key semantic gap still open. Coverage: **93.88%**. Tests: **1,507**.
loamSpine: **0** `#[async_trait]` in-tree; **178** source files. **Stadial gate cleared**
(sled + sqlite out). Coverage: **~90.9%**. Tests: **1,442**. **Regression**: `ring`
appeared in `Cargo.lock` via hickory-resolver 0.26 upgrade — needs trace.
~~sweetGrass async-trait~~ **COMPLETE** — stadial pass: `BraidBackend` enum dispatch
(Memory/Redb/Postgres/Sled/NestGate), `SigningClientKind`/`PrimalDiscoveryKind` enums,
all trait methods converted to native RPITIT. `async-trait` dep fully removed.
Tests: **1,560**. Remaining lockfile debt: `ring` + `sled` ghost stanzas.
~~loamSpine sled/sqlite~~ **REMOVED** (`ec19ea0`): `sled` and `sqlite` backends deleted.
~~Squirrel lockfile ghosts~~ **ELIMINATED** (`169768a8`): ring and reqwest removed from
`Cargo.lock`. Squirrel is fully interstadial-ready.

### coralReef

~~UDS first-byte peek~~ **RESOLVED.**
~~async-trait~~ **COMPLETE** — Iter 83: jsonrpsee removed, pure NDJSON/TCP dispatch.
`primal-rpc-client` gains `TcpLine` / `UnixLine` transports.
VFIO channel register tests + nvidia header tests extracted (Iter 82).
Transitive libc deferred (mio→rustix upstream). Coverage: **~65%**. Tests: **4,506**.

### skunkBat

~~BTSP Phase 2~~ **RESOLVED** — v0.1.0: `PeekedStream` custom UDS wrapper for first-byte
auto-detect (JSON-RPC `0x7B` → biomeOS bypass, else BTSP handshake). BearDog v0.9.0 IPC
surface alignment. Full JSON-RPC 2.0 compliance (batch + notifications).
Thymic selection model: design spec complete (`THYMIC_SELECTION_SPEC.md`), implementation
blocked on BearDog `lineage.list` + `btsp.session.verify` IPC availability.
Composable primitives: 5 domains (`baseline.*`, `metadata.*`, `response.*`, `lineage.*`,
`health.*`) defined in spec, IPC registration via biomeOS Neural API pending.
`PeekedStream` / `PrefixedStream` convergence: skunkBat and BearDog each implemented
independently — should consolidate into shared `sourdough-core` utility.
async-trait: **14** (threat/recon traits). Coverage: **89.6%**. Tests: **171**.

### First-byte peek UDS standardization (cross-primal)

**ALL BTSP-enforcing primals now have UDS peek**: NestGate, BearDog, Songbird,
coralReef, petalTongue, **skunkBat** (v0.1.0 `PeekedStream`).
**This cross-cutting gap is CLOSED.** Two implementation patterns exist:
BearDog uses `PrefixedStream`; skunkBat uses `PeekedStream`. Should converge
to a single shared utility in `sourdough-core`.

### Class 4 ecosystem-wide: async-trait migration

**9 primals at zero**: biomeOS(0), barraCuda(0), Squirrel(0), loamSpine(0),
NestGate(0+0), coralReef(0), Songbird(0), petalTongue(0), rhizoCrypt(0).
Remaining: toadStool(~158, dyn-ceiling) > BearDog(49) > sweetGrass(22, object-safety) >
skunkBat(14).

---

*Resolved gaps, compliance matrices, and historical evolution snapshots are in
[`PRIMAL_GAPS_RESOLVED_HISTORY.md`](PRIMAL_GAPS_RESOLVED_HISTORY.md).*
