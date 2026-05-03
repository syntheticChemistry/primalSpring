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
> **Current phase: INTERSTADIAL** — stadial gate cleared April 16, 2026.
> All 13 primals have reached modern async Rust parity:
> - [x] `async-trait` eliminated from all `Cargo.toml` and `.rs` files — **13/13**
> - [x] Enum dispatch replaces `Box<dyn Trait>` for finite implementors — **13/13**
> - [x] `cargo deny check bans` passes — **13/13**
> - [x] Edition 2024, `deny.toml` enforced — **13/13**
> - [x] Ring lockfile ghost: Cargo v4 artifact, never compiled — **13/13 PASS**
>
> Standards are now **permanent interstadial invariants** — regressions are rejected.
> See `wateringHole/STADIAL_PARITY_GATE_APR16_2026.md` for the full specification.
>
> **primalSpring own stadial (April 16–30, 2026)**: primalSpring stadial pass complete.
> `deny.toml` license fix, exp094 license inheritance, `#[allow(` → `#[expect(` with reasons,
> `cargo clippy` **0 warnings** (all 62 resolved), `cargo fmt` **0 violations** (76 fixed),
> **561 tests passing**. `Arc<dyn ValidationSink>` justified (open extensibility + generic `NdjsonSink<W>`).
> Experiment ID registry updated (67 → 75). All local debt eliminated.
>
> Downstream springs may resume absorption.
>
> **Last updated**: 2026-05-02 (PM) — **Phase 3: 12/13 primals implement `btsp.negotiate`. Only NestGate remains.**
>
> **primalSpring local quality gate**: `cargo clippy` 0 warnings, `cargo fmt` 0 violations,
> 563 tests (561 + 2 ignored integration), all compositions validated.
>
> **BTSP Phase 3 near-complete** (May 2, 2026 PM):
> 12 of 13 primals now implement `btsp.negotiate` server-side.
> NestGate is the sole remaining primal (team needs more time).
>
> **Phase 3 — full encrypted framing (negotiate + HKDF + ChaCha20-Poly1305 AEAD on wire)**:
> - BearDog: **FULL** — returns `chacha20-poly1305` + server_nonce (live confirmed)
> - rhizoCrypt S59: **FULL** — encrypted loop in `serve_after_handshake` + 16 tests
> - barraCuda Sprint 51: **FULL** — transport-layer intercept + typed NegotiateError
> - petalTongue: **FULL** — both framed + JSON-line paths
> - toadStool S215: **FULL** — JSON-line relay + encrypted framing
> - sweetGrass v0.7.29: **FULL** — transport refactor, UDS+TCP encrypted frame loop
> - Songbird Wave 184: **FULL** — binary-framed + NDJSON paths, BearDog key export, 28 tests
> - Squirrel: **FULL** — encrypted frame loop in jsonrpc_server, handshake key from verify, HKDF compatible
> - skunkBat: **FULL** — handshake key stored in registry, run_encrypted_frame_loop wired, E2E test
> - biomeOS: **FULL** — encrypted framing wired into connection loop, handle_encrypted_stream + try_phase3_negotiate, 16MB frame guard
>
> **Phase 3 — crypto ready, wire framing not yet connected (transport upgrade pending)**:
> - coralReef: **CRYPTO-READY** — full HKDF + ChaCha20 + zeroize + SessionKeys + encrypt/decrypt in btsp_negotiate.rs (631 LOC, 10 tests), handshake_key extracted from BearDog, keys derived and stored via take_negotiated_keys(). Wire gap: unix_jsonrpc.rs still calls process_newline_reader_writer — no post-negotiate encrypted frame loop yet
>
> **Phase 3 — negotiate handler wired, null cipher blocks ionic compositions**:
> - loamSpine: **IONIC-BOND-BLOCKING** — handler wired + 4 tests, returns `cipher: "null"`. Blocks ionic/weak bond compositions (healthSpring enclave, cross-family federation, anchoring pipeline). Already does Blake3 locally for integrity. Resolution: Pattern B — accept `session_key` from BearDog verify, add hkdf+chacha20poly1305+zeroize, wire encrypted frame loop. See `wateringHole/CRYPTO_CONSUMPTION_HIERARCHY.md` Part 7
>
> **Phase 3 — code on disk, not yet compiled into module tree**:
> - NestGate: **MODULE-PENDING** — `btsp_phase3/mod.rs` (505 LOC, 20 tests) + `transport.rs` (509 LOC, 8 async tests) exist on disk with full HKDF + ChaCha20 + SessionKeys + run_encrypted_frame_loop + try_phase3_negotiate. Missing: `pub mod btsp_phase3;` in `rpc/mod.rs`, `hkdf` + `zeroize` deps in workspace Cargo.toml. Wire gap: unix_socket_server / isomorphic_ipc not yet calling transport functions
>
> **All previous upstream gaps RESOLVED**:
> - PG-45/46/47/48, GAP-06/12 — all closed
>
> **Remaining items**:
> - PG-54: adaptive sensor polling — **DEFERRED-BY-DESIGN**
> - GAP-06: Squirrel `discovery.register` naming — cosmetic
> - GAP-18/19/20: discovery/family resolution — mitigated
>
> **Zero local debt.** primalSpring is the Phase 3 composition reference.
>
> All 10 primals running UDS-only. `ss -tlnp | grep plasmidBin` returns **empty**.
> 7 primals modified (BearDog, Songbird, Squirrel, ToadStool, rhizoCrypt, sweetGrass, loamSpine)
> to make TCP opt-in via explicit `--port` flag. Same biomeOS graph deploys on any hardware/arch.
> TCP is opt-in only for Songbird federation (`--port 8080` enables covalent mesh).
>
> **Cross-Architecture Pixel Deployment (April 14–15, updated April 20)**: **15/15 exp096 checks PASS.**
> biomeOS-managed Tower (BearDog + Songbird) runs on Pixel 8a (aarch64/GrapheneOS/Titan M2).
> All critical composition gaps RESOLVED:
> - BearDog: protocol auto-detection on TCP (peek first byte: `{` = JSON-RPC, else BTSP)
> - biomeOS: TCP cascade in `primal_start_capability`, `tcp_port_registry`, TCP-aware socket wiring
> - Songbird: `tcp://` scheme parsing in IPC endpoint discovery
> - Neural API `capability.call` routes crypto/genetic/security/beacon to BearDog over TCP
> Previous 4 failures now resolved: 3 reporting gaps fixed, HSM/Titan M2 cfg-gated upstream (beardog Session 43)
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
> **BTSP Phase 2 ECOSYSTEM CASCADE (April 9–16, escalated Phase 45c)**: *Historically 11/13 at
> mid-cascade, now* **13/13 BTSP-authenticated** *(fully converged April 24, 2026)*.
> JSON-line BTSP auto-detection and full handshake relay wired into ToadStool, barraCuda, coralReef, NestGate, Squirrel during
> Phase 45c. Songbird Wave 133→156, ToadStool S198→S203q, barraCuda Sprint 39→44,
> coralReef Iter 78→84+, rhizoCrypt S31→S43, sweetGrass wired.
> skunkBat Phase 2 COMPLETE (v0.1.0 — `PeekedStream` UDS peek + BearDog v0.9.0 alignment).
> ~~Remaining 2 upstream: petalTongue, loamSpine~~ — **ALL RESOLVED** (April 24, 2026).
> 13/13 BTSP authenticated. See CHANGELOG Phase 45c final convergence fixes.
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
"managed" — they are debt. Songbird has `ring` 0.17.14 as a transitive lockfile
ghost (via `rustls-webpki` optional dep; NOT compiled in any build config).
Blocked on upstream `rustls-rustcrypto` crates.io release. See Songbird `deny.toml`.

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
| BearDog | **no** | **PASS** | **no** | **Clean** (Wave 55: hickory-resolver removed, ring+async-trait gone from lockfile) |
| Songbird | yes | **PASS** | **no** | Lockfile artifact — Songbird IS the TLS provider |
| petalTongue | yes | **PASS** | **no** | Delegate HTTP to Songbird, eliminate `reqwest` |
| NestGate | yes | **PASS** | **no** | Lockfile artifact — vendored rustls-rustcrypto |
| loamSpine | yes | **PASS** | **no** | Lockfile artifact — hickory optional dep |
| Squirrel | no | **PASS** | **no** | Clean |
| toadStool | yes | **PASS** | **no** | Lockfile artifact — Cargo v4 optional dep |
| biomeOS | no | **PASS** | **no** | Clean |
| rhizoCrypt | no | **PASS** | **no** | Clean |
| barraCuda | no | **PASS** | **no** | Clean |
| coralReef | no | **PASS** | **no** | Clean |
| skunkBat | no | **PASS** | **no** | Clean |

**13/13 pass `cargo deny check bans`. 0/13 compile ring. 5 carry ring as Cargo v4
lockfile artifact (BearDog eliminated ring from lockfile in Wave 55). Not actionable
ecosystem debt.**

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
| toadStool | **0** | **No** | **COMPLETE** (S203r–S203t: 158→0, 32 traits → enum dispatch + RPITIT, dep removed + banned) |
| BearDog | **0** | **No** | **COMPLETE** (Wave 53–55: 49→0, 22 traits → 18 enum dispatch types, RPITIT, dep removed + lockfile clean, banned in deny.toml) |

**13/13 primals at zero. Stadial gate CLEARED. async-trait fully eliminated ecosystem-wide.**

BearDog Wave 53 created 18 enum dispatch types: `MethodHandlerKind`, `BondPersistenceBackend`,
`HsmKeyProviderBackend`, `HsmProviderBackend`, `UniversalCryptoBackend`, `CryptoProviderBackend`,
`KeyManagementBackend`, `ServiceDiscoveryBackend`, `KeystoreTransportBackend`,
`AttestationTransportBackend`, `HealthMetricsTransportBackend`, `IpcHandlerBackend`,
`PlatformListenerBackend`, `StorageBackend`, `EncryptionKeyBackend`, `AuditLoggerBackend`,
`Ctap2TransportBackend`, `HidDeviceBackend`. Wave 55 eliminated `async-trait` from `Cargo.lock`
entirely (removed hickory-resolver, tarpc, opentelemetry transitive chains).

**Resolution patterns used ecosystem-wide**:
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
| armv7 cross-arch build: `usize` overflow in cpu.rs and universal/capabilities | primalSpring v0.9.17 genomeBin | **RESOLVED** (S174 — `CpuBackend::MAX_ALLOC` gated via `#[cfg(target_pointer_width)]`; `Capabilities::memory_bandwidth` evolved from `usize` to `u64`. `cargo check --workspace --target armv7-unknown-linux-gnueabihf` passes) |

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
| **Songbird** | Wave 134-151: `capability.resolve`, `inference.*` canonical, CI-01 `cargo deny`, **SB-02 ring-crypto removed**, **SB-03 sled eliminated**, canonical constants, **PG-37 (Phase 45) capability-first routing**: `ipc.resolve` now has primal-name fallback when capability lookup fails + `ipc.resolve_by_name` alias + `name` param alias, 7,380 tests | SB-02, SB-03, capability.resolve, inference namespace, CI-01, **PG-37** | QUIC/TLS evolution, transitive `ring` in lockfile (not compiled) |
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
- ~~**Songbird**: `songbird-orchestrator` has 3 `E0308` type mismatches~~ **RESOLVED** (Wave 152 — compiles clean with `--tests`, 7,380 tests passing)
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

BC-01–BC-08 **ALL RESOLVED**. barraCuda is a full ecobin primal with 32 JSON-RPC
methods over UDS (tensor.matmul, tensor.create, stats.mean, compute.dispatch,
noise.perlin2d, fhe.ntt, etc.). The remaining composition gap is **spring-side**:
springs still link barraCuda as a Rust library (path/git dep) instead of calling
the ecobin primal over IPC. See "Cross-Spring Composition Gaps" below.

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
| ~~SD-01~~ | ~~Missing `deny.toml`~~ | ~~Low~~ | **RESOLVED** (April 30) — `deny.toml` added at repo root + scaffolded into generated primals |
| SD-02 | musl cross-compilation | Low | Open — binary builds not yet wired for ecoBin |
| SD-03 | genomeBin signing | Low | Open — sequoia-openpgp not implemented |

**Compliance** (v0.2.0-dev — 3aca9ec): clippy **CLEAN** (`all` + `pedantic` + `nursery`), fmt **PASS**, `forbid(unsafe_code)` at workspace level, `deny.toml` **PRESENT** (license + ecoBin-style C-sys bans), SPDX AGPL-3.0-or-later in Cargo.toml. **247 tests, 0 failures** (unit + integration + e2e + doctests), coverage 95%+. Edition 2024, workspace lints centralized. Zero `TODO`/`FIXME`/`HACK`/`unimplemented!` in source.

**v0.2.0 scaffold evolution** (April 30): Scaffold now generates **core + server** dual-crate layout.
Generated primals include: `ci.yml` + `notify-plasmidbin.yml` CI workflows, `deny.toml`,
JSON-RPC 2.0 server with capability wire (`health.liveness`, `health.readiness`,
`capabilities.list`), first-byte peek routing (`0x7B` = JSON-RPC, else BTSP placeholder),
XDG-compliant socket naming (`$XDG_RUNTIME_DIR/biomeos/{primal}-{fid}.sock`).
`sourdough-core` now provides canonical `PeekedStream`/`peek_protocol` for ecosystem reference.
Templates split into `templates/{mod,core,server,infra}.rs` for maintainability.
CONVENTIONS updated: JSON-RPC 2.0 primary IPC, tarpc secondary.

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

**sourDough** — ~~`deny.toml` missing~~ **RESOLVED**. musl build, genomeBin signing still open. v0.2.0 scaffold evolution shipped (server crate, CI, capability wire, peek, socket naming). Deferred: SD-02 musl, SD-03 signing.

### Primals With Tractable Local Fixes

**biomeOS** — BM-10: method translation **RESOLVED**. BM-11: ToadStool dual-socket
**RESOLVED** (`prefers_jsonrpc` + `.jsonrpc.sock` sibling check). **All tractable biomeOS gaps resolved.**

**ToadStool** — TS-01: coralReef discovery **RESOLVED** (`capability.discover("shader")` Tier 1).
Compute socket resolution fully functional via BM-11 (`prefers_jsonrpc` flag + `.jsonrpc.sock`
sibling preference). **All tractable ToadStool gaps resolved.**

**Songbird** — Wave 146-152: stadial dyn audit, mock isolation, hardcoded elimination,
dead feature removal, PG-21 persistent NDJSON, PG-37 capability-first routing, dead deps
removed (slab, wasi), yaml feature stripped, env-dependent test fixed, 7,380 tests.
SB-02: `ring` lockfile ghost — blocked on upstream `rustls-rustcrypto` release (not compiled).
SB-03: **RESOLVED** (Wave 135 — `sled` fully eliminated from workspace and Cargo.lock).
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
methods; esotericWebb needs 6 more. See `graphs/downstream/downstream_manifest.toml` (ludospring entry).

**esotericWebb** — Garden/composition (not a primal). Binary NOT in plasmidBin. Transport
needs UDS negotiation. See `graphs/downstream/downstream_manifest.toml` (esotericwebb entry).

---

## Priority Order

**0 HIGH blockers. 0 MEDIUM (all resolved). 6 LOW. Zero runtime blockers.** (sourDough SD-02/03 deferred, skunkBat GAP-28 RESOLVED)

**High**: ~~PLASMIBIN-STALE~~ **RESOLVED** (April 10 — full musl-static rebuild, 12/12 ecoBin).

**Medium** (degrades composition/experiment quality):
1. ~~**BTSP-BARRACUDA-WIRE**~~ **RESOLVED UPSTREAM** (Sprint 48) — full 7-step BTSP relay implemented since Sprint 44h-44i. tarpc keyed-cipher enforcement added. 26 BTSP compliance tests.
2. ~~**IONIC-RUNTIME**~~ **RESOLVED UPSTREAM** (BearDog Wave 76) — `crypto.sign_contract` confirmed wired and tested since Wave 42.

**Resolved this session (April 10 NUCLEUS patterns)**:
- ~~**NESTGATE-UDS**~~ **RESOLVED** — `--socket` CLI flag added and wired through dispatch → `NESTGATE_SOCKET` env var → `SocketConfig` tier-1 resolution. C5 now PASS (5/5).
- ~~**TS-UDS-JSONRPC**~~ **RESOLVED** — JSON-RPC gets dedicated `compute.jsonrpc.sock` socket, separate from tarpc `compute.sock`. Legacy symlinks for both protocols. `--socket` CLI flag wired to `run_server_main`.
- ~~**NEURAL-API-DOUBLE-PREFIX**~~ **RESOLVED** (prior session) — `capability.call` strips leading domain prefix from operation parameter.
- **BTSP-CLIENT** — primalSpring BTSP client handshake implemented (`btsp_handshake.rs`), integrated into `Transport::connect()` with auto-detection via `security_mode_from_env()`.

**Stadial Debt** (blocks parity gate — must resolve before next interstadial):
3. **SB-02** — `ring` in Songbird `Cargo.lock` (lockfile ghost only — not compiled; blocked on upstream `rustls-webpki` release)
4. ~~**SB-03**~~ **RESOLVED** (Wave 135 — `sled` fully eliminated from workspace and Cargo.lock)
5. **PT-09** — petalTongue Phase 2 stub (warn-only, no enforcement)
6. ~~**PT-DOMAINS**~~ **RESOLVED** (April 10 — `ui.sock` + `interaction.sock` symlinks added)
7. ~~**CR-03**~~ **RESOLVED** (Iter 78 — `guard_connection()` with real BearDog RPC, degraded when absent)
8. ~~**BC-GPU-PANIC (BC-05)**~~ **RESOLVED** (Sprint 39 — `Auto::new()` → `Err`, health `Degraded`)
9. ~~**EXP091-REGISTRY**~~ **RESOLVED** (April 10 — `get_family_id()` → `self.family_id`; socket alias mapping)
10. ~~**EXP-TCP-UDS**~~ — exp085/exp090 use TCP by design (crypto lifecycle, LAN probe). Ports env-configurable via `BEARDOG_PORT`/`SONGBIRD_PORT`. Not a gap — UDS experiments use `CompositionContext`
11. ~~**BTSP-E2E**~~ **RESOLVED** (April 14 — `AtomicHarness` now generates deterministic BTSP seed via HKDF-SHA256, injects `FAMILY_SEED` env on all child primals, uses `PrimalClient::connect_btsp` for BTSP-model primals. BearDog socket timeout unblocked for exp061-068)

**Deferred** (later development cycle):
- ~~**SD-01**~~ **RESOLVED** — sourDough `deny.toml` added
- **SD-02/03** — sourDough musl, genomeBin signing
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
| sourDough | **CLEAN** | **PASS** | **YES** ↑ | `-or-later` | 2024 | **PASS (247)** ↑ | FAIL | — | NONE |
| coralReef | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (4,506)** ↑ | **PASS** ↑↑ | **PASS** ↑↑ | **L2** ↑ |
| bingoCube | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | PASS | N/A | N/A | NONE |
| skunkBat | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (171)** | **PASS** ↑↑ | **PASS** ↑↑ | **L2** ↑ |

**Legend**: ↑ = improved since last audit. BTSP P1 = socket naming + insecure guard. BTSP P2 = handshake on accept/client. Wire = Capability Wire Standard level.

### Compliance Evolution (April 9 — BTSP Phase 2 ecosystem cascade)

**BTSP Phase 2→3 rollout — 13/13 authenticated (CONVERGED).** All capabilities BTSP-authenticated
after Phase 45c full convergence. **Tower: 100%. Node: 100%. Nest: 100%. Provenance: 100%.**
primalSpring itself: clippy ZERO warnings, fmt PASS, all tests PASS.

1. **Songbird**: **BTSP Phase 2 COMPLETE** ↑↑ (Wave 133, **NDJSON wire-format Wave 153**) — `perform_server_handshake()` (length-prefix) + `perform_server_handshake_ndjson()` (JSON-line) in `ipc/btsp.rs`; first-line auto-detect: `"protocol":"btsp"` → NDJSON BTSP handshake then NDJSON JSON-RPC; BearDog delegation via `SecurityRpcClient`. `BtspClient` + connection managers.
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

**Node Atomic**: FULLY OPERATIONAL. coralReef shader capabilities work (11 GPU archs).
ToadStool alive with BTSP auto-detect — raw JSON-RPC health PASS (LD-04 resolved).
barraCuda ALIVE (LD-05 resolved) — JSON-RPC transport with 32 methods including
`tensor.matmul`, `tensor.create`, `stats.mean`, `compute.dispatch` (LD-10 resolved,
Sprint 42). All Node primals reachable over UDS.

**Nest Atomic**: FULLY OPERATIONAL. NestGate storage roundtrip PASS (LD-03
resolved). sweetGrass health PASS. loamSpine ALIVE and health PASS (LD-09
resolved). rhizoCrypt ALIVE on UDS (LD-06 resolved, S37).

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

### Remaining Open Upstream Gaps (refreshed April 18)

| Priority | Gap | Owner | Status |
|----------|-----|-------|--------|
| ~~Medium~~ | ~~`storage.retrieve` for large/streaming tensors~~ | ~~NestGate~~ | **RESOLVED** (Session 50) — streaming registry + semantic router dispatch |
| ~~Medium~~ | ~~Cross-spring persistent storage IPC~~ | ~~NestGate~~ | **RESOLVED** (Session 50) — `shared` namespace + `fetch_external` wired to all dispatch paths |
| Medium | Songbird coverage 72%→90% | Songbird | **73.4%** ↑ (Wave 179: +92 tests → 7,784). I/O-heavy paths hard to cover |
| Medium | coralReef coverage ~65%→90% | coralReef | **~65%** (Iter 88: 4,639 tests). CR-04 typed errors **RESOLVED**. Coverage still gap |
| Low | 29 shader absorption candidates | barraCuda | neuralSpring pipeline (submit PRs per shader) |
| Low | Batched `OdeRK45F64` for Richards PDE | barraCuda | airSpring-specific |
| Low | IPC timing for `shader.compile` | coralReef | Deployment timing |
| ~~Medium~~ | ~~`crypto.sign_contract` (ionic bond negotiation)~~ | ~~BearDog~~ | **RESOLVED** (Wave 78 confirmed) — `IonicBondHandler` dispatches `crypto.sign_contract` + `crypto.verify_contract`, wired since Wave 42 |
| Medium | `compute.dispatch` standardization | toadStool | **RESOLVED** — S203 `DISPATCH_WIRE_CONTRACT.md` + PG-31 fix (JSON-RPC routing wired) |
| Medium | ~~18~~ ~12 barraCuda IPC surface gaps | barraCuda | **PARTIAL** (Sprint 49: +6 methods → 56 total). `eigh` enhanced. ~12 remaining |
| Low | BTSP Phase 3 (encrypted post-handshake channel) | All primals | Deferred — Phase 2 NULL cipher operational everywhere |
| ~~Low~~ | ~~Squirrel provider registration protocol~~ | ~~Squirrel~~ | **RESOLVED** — `provider.register`/`list`/`deregister` RPC + lying stubs eliminated |
| ~~Low~~ | ~~`storage.fetch_external` (cross-spring data)~~ | ~~NestGate~~ | **RESOLVED** (Session 50) — wired in semantic router + isomorphic IPC + unix dispatch |
| ~~Low~~ | ~~loamSpine provenance chain for guideStone receipts~~ | ~~loamSpine~~ | **RESOLVED** — self-contained provenance receipts (`CommitSessionResponse` + `get_provenance_chain()`) |
| Low | Genetics three-tier awareness in primals | All primals | **primalSpring RPC client aligned** (April 15). BearDog has `genetic.*` RPCs. ecoPrimal `genetics::rpc` now matches BearDog's actual API. No primal has consumed `GeneticSecurityMode` or `MitoBeacon`/`NuclearGenetics` types yet — adoption awaits ecoPrimal ≥0.10.0 |
| Low | skunkBat thymic selection impl | skunkBat + BearDog | **PARTIAL** — BearDog lineage integration done (`RemoteLineageVerifier` → `lineage.list`/`verify`). BTSP delegation to `btsp.server.verify`. Full thymic model still spec/design |
| Low | skunkBat composable primitives IPC | skunkBat + biomeOS | 5 domains defined, Neural API registration pending |
| ~~Low~~ | ~~`PeekedStream`/`PrefixedStream` convergence~~ | ~~skunkBat + BearDog~~ | **RESOLVED** — sourDough v0.2.0 provides canonical `PeekedStream` in `sourdough-core`; existing impls functionally equivalent; new primals use canonical |
| Low | toadStool coverage 83.6%→90% | toadStool | S203p pushed toward, not yet at target |
| ~~Low~~ | ~~BearDog async-trait 49→0~~ | ~~BearDog~~ | **RESOLVED** (Wave 78) — 0 in production, dep removed from workspace |
| ~~Low~~ | ~~sweetGrass async-trait 22→0~~ | ~~sweetGrass~~ | **RESOLVED** — 22→0, crate removed from all 7 Cargo.toml. `cargo deny check` in CI |
| ~~Low~~ | ~~skunkBat async-trait 14→0~~ | ~~skunkBat~~ | **RESOLVED** (April 30) — 14→0, fully eliminated. 205 tests |

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
~~`sourdough-core` path coupling (GAP-28)~~ **RESOLVED** (April 30, 2026):
Commit `ef821eb` internalized 6 types (`PrimalLifecycle`, `PrimalState`, `PrimalError`,
`PrimalHealth`, `HealthReport`, `HealthStatus`, `CommonConfig`) into `primal_foundation`
module in `skunk-bat-core`. Zero cross-repo path deps. `needs_sibling` removed from
`plasmidBin/sources.toml`. Post-GAP-28 evolution: self-knowledge consolidation, platform
util dedup, config-driven bind, deny+doc CI gates. Tests: **205**. Coverage: **~90%**.
async-trait: **0** (was 14 — fully eliminated).
~~`PeekedStream` / `PrefixedStream` convergence~~: skunkBat and BearDog each implemented
independently — sourDough now provides canonical `PeekedStream` in `sourdough-core` for
future primals. Existing impls are functionally equivalent; convergence is low priority.

### First-byte peek UDS standardization (cross-primal)

**ALL BTSP-enforcing primals now have UDS peek**: NestGate, BearDog, Songbird,
coralReef, petalTongue, **skunkBat** (v0.1.0 `PeekedStream`).
**This cross-cutting gap is CLOSED.** Two implementation patterns exist:
BearDog uses `PrefixedStream`; skunkBat uses `PeekedStream`. Should converge
to a single shared utility in `sourdough-core`.

### Class 4 ecosystem-wide: async-trait migration

**ALL 13 primals at zero** (stadial gate cleared April 16, 2026):
biomeOS(0), barraCuda(0), Squirrel(0), loamSpine(0), NestGate(0), coralReef(0),
Songbird(0), petalTongue(0), rhizoCrypt(0), toadStool(0, S203s RPITIT + enum dispatch),
BearDog(0, W56 serde_yaml eliminated), sweetGrass(0, RPITIT + enum dispatch),
skunkBat(14, non-blocking — threat/recon trait interfaces).

## Cross-Spring Composition Gaps (Evaporation — April 17, 2026)

Four springs have entered NUCLEUS composition testing and reported gaps back:

### Common Cross-Primal Protocol Gaps

| Gap | Reported By | Owner | Status |
|-----|-------------|-------|--------|
| **Ionic bond negotiation** — `crypto.sign_contract` not yet wired for cross-tower compositions | hotSpring (GAP-HS-005 evolution), healthSpring (dual-tower ionic), wetSpring (provenance cross-spring) | **BearDog team** | **OPEN** — propose→accept→seal lifecycle works; cross-family contract signing not yet exposed as IPC method |
| **BTSP Phase 3 encrypted channel** — post-handshake cipher negotiation (`btsp.negotiate` + ChaCha20Poly1305) | hotSpring, healthSpring, neuralSpring, wetSpring | **BearDog + all primals** | **DEFERRED** — Phase 2 NULL cipher operational everywhere; Phase 3 awaiting HSM integration path |
| **toadStool `compute.dispatch` standardization** — springs need consistent dispatch envelope for GPU compute | hotSpring, wetSpring, neuralSpring | **toadStool team** | **RESOLVED** (S203 `DISPATCH_WIRE_CONTRACT.md`) but spring-side adoption incomplete |
| **Squirrel provider registration** — `inference.register_provider` needed for springs with local models | neuralSpring, healthSpring, wetSpring | **Squirrel team** | **PARTIAL** — wire tests exist; production registration path in progress |
| **NestGate `storage.fetch_external`** — cross-spring data retrieval for composition pipelines | wetSpring, healthSpring | **NestGate team** | **PARTIAL** — method exists but delegated via Tower Atomic; cross-spring routing via biomeOS needed |
| **barraCuda IPC migration** — springs still link barraCuda as a Rust library (path/git dep) for domain math; need to rewire to the barraCuda ecobin's 32 JSON-RPC methods over UDS. **Spring-side actively in progress**: hotSpring (9 probes), healthSpring (2/11 IPC via math_dispatch), neuralSpring V133 (IpcMathClient, 9 methods wired), wetSpring V145 (5 primals). | All delta springs | **Each spring team** (primalSpring documents the pattern) | **IN PROGRESS** — 4/4 delta springs building IPC clients; no spring uses `primalspring::composition` yet |
| **barraCuda JSON-RPC surface gaps** — neuralSpring V133 documents 18 `barracuda::` library calls with no 1:1 JSON-RPC method: `eigh_householder_qr`, `pearson_correlation`, `chi_squared_statistic`, `empirical_spectral_density`, `shannon_from_frequencies`, `solve_f64_cpu`, `esn_v2::*`, `nn::SimpleMlp`, `belief_propagation_chain`, `graph_laplacian`, `numerical_hessian`, `boltzmann_sampling`, `nautilus::*`, `fit_linear`. These block full Level 5 for neuralSpring. Priority: `linalg.eigh`, `stats.pearson`, `stats.chi_squared`, `stats.shannon` (most-used science paths). | neuralSpring V133 (explicit), other springs (potential) | **barraCuda team** | **OPEN** — hand-back from neuralSpring Gap 11; barraCuda surface expansion needed |

### Per-Spring Composition Status (Delta Season — April 19, 2026)

| Spring | Version | guideStone Level | Composition Tier | Evidence | Blockers |
|--------|---------|-----------------|-----------------|----------|----------|
| **hotSpring** | v0.6.32 | 5 (certified) | Level 5 | `hotspring_guidestone` (bare+NUCLEUS), 64/64 suites, `primalspring::composition` API | Exit-code 2 semantics; P2–P4 partial in code |
| **healthSpring** | V54 | 2 (props documented) | Level 5 | `healthspring_guidestone` (bare.rs/domain.rs), modular split, feature-gated | P3 (CHECKSUMS), ionic bridge, bare_exit_code doc bug |
| **neuralSpring** | V134 | 2 (props documented) | Level 5 | `neuralspring_guidestone` (4-phase), `GUIDESTONE_PROPERTIES.md`, feature-gated | P3 (CHECKSUMS), 18 barraCuda method gaps, machine-readable provenance |
| **wetSpring** | V147 | 3 (bare works) | Level 5 | `wetspring_guidestone` (B0/B1+N0–N3), strict exit 2, expanded N2 (linalg/spectral/stats) | Live NUCLEUS for N1–N3, PG-12 legacy convergence |
| **ludoSpring** | V45 | 3 (bare works) | Level 5 | `ludospring_guidestone` (15 bare + 15 IPC), per wateringHole handoff | Local checkout stale |

### Patterns Handed Back (Evaporation → primalSpring)

Springs discovered and refined these patterns during composition testing.
primalSpring absorbs for standardization:

1. **Three-tier validation structure** (Tier 1: LOCAL_CAPABILITIES, Tier 2: IPC-wired, Tier 3: Full NUCLEUS) — all four delta springs converged independently. Now documented in `PRIMALSPRING_COMPOSITION_GUIDANCE.md`. **ABSORBED.**
2. **Honest skip with reason** — `check_or_skip("primal not running: {reason}")` pattern used by all active springs. Already in primalSpring's `validation/mod.rs`. **ABSORBED.**
3. **Niche self-knowledge** — healthSpring's `niche.rs` with `CAPABILITIES` + `CONSUMED_CAPABILITIES` + `COST_ESTIMATES`. Pattern matches primalSpring's own niche. **ABSORBED.**
4. **Provenance registry** — wetSpring's `provenance_registry.rs` for tracking IPC provenance chains. Candidate for absorption into ecoPrimal validation library.
5. **Dual-tower ionic bridge** — healthSpring's pattern for patient-data / analytics separation. Already has standalone proto-nucleate: `healthspring_enclave_proto_nucleate.toml`. **ABSORBED.**
6. **`capability_to_primal()` public API** — all three springs built their own domain→primal mapping. Now public in `primalspring::composition`. **ABSORBED Apr 18, 2026.**
7. **`method_to_capability_domain()` routing** — hotSpring's handoff asked for canonical JSON-RPC method→domain resolution. Now in `primalspring::composition`. **ABSORBED Apr 18, 2026.**
8. **`validate_liveness()` preamble** — all three springs wrote the same health-check-all-then-exit(2) preamble. Now canonical in `primalspring::composition::validate_liveness()`. **ABSORBED Apr 18, 2026.**
9. **`validate_primal_proof` binary convention** — hotSpring, wetSpring, ludoSpring all name their Level 5 harness `validate_primal_proof` or `validate_primal_parity`. Canonical pattern documented in PRIMALSPRING_COMPOSITION_GUIDANCE.md. **ABSORBED Apr 18, 2026.**
10. **Feature-gated IPC routing** — healthSpring's `math_dispatch.rs` uses `#[cfg(feature = "primal-proof")]` to toggle between library and IPC paths. Good pattern for gradual migration. **DOCUMENTED** — not yet absorbed as library code; springs own their dispatch layer.
11. **`IpcMathClient` typed dispatch** — neuralSpring V133's `ipc_dispatch.rs` provides a full typed IPC client mirroring the library `Dispatcher` interface. Same math surface, different transport. **DOCUMENTED Apr 18, 2026** — pattern parallel to `CompositionContext`, spring-specific by design.
12. **`deny.toml` stadial ban list** — neuralSpring V133 first to enforce the full stadial parity gate via `cargo deny` bans (`ring`, `openssl-sys`, `async-trait`, `rustls`, `ed25519-dalek`, `cmake`, `cc`). Template for all springs. **DOCUMENTED Apr 18, 2026.**
13. **18-method barraCuda gap hand-back** — neuralSpring's IPC migration audit discovered 18 library calls with no JSON-RPC equivalent. These are the first concrete hand-back to barraCuda for surface expansion. **TRACKED** — added to ecosystem-level gaps above.
14. **IPC clients are validation windows** — all four delta springs built independent IPC clients (`NucleusContext`, `IpcMathClient`, `math_dispatch`, `rpc_call`). None depend on the `primalspring` crate. This is **by design**: the IPC client is temporary tooling to prove the math works through NUCLEUS. The permanent deployable is the **guideStone binary** (pure IPC, no library dep on `barracuda`). Springs keep their lib dep for Level 2 parity comparison; the guideStone deploys without it. **RESOLVED** — framed as guideStone pattern in `GUIDESTONE_COMPOSITION_STANDARD.md`.
15. **guideStone as universal composition pattern** — hotSpring-guideStone-v0.7.0 proved all 5 properties (deterministic, traceable, self-verifying, env-agnostic, tolerance-documented) with cross-substrate parity and NUCLEUS additive layer. Extracted as ecosystem standard. **ABSORBED Apr 18, 2026** — see `wateringHole/GUIDESTONE_COMPOSITION_STANDARD.md`.
16. **`checksums` module for P3** — every spring's guideStone audit identified P3 (self-verifying) as a universal blocker because no CHECKSUMS infrastructure existed. primalSpring now provides `primalspring::checksums::{generate_manifest, verify_manifest, blake3_file}`. **ABSORBED Apr 19, 2026.**
17. **Explicit bare-only exit(2) pattern** — wetSpring V147 showed the cleanest bare-only exit semantics: `if v.exit_code() == 0 { 2 } else { 1 }`, reserving exit 0 for full NUCLEUS certification only. Adopted by `primalspring_guidestone`. **ABSORBED Apr 19, 2026.**
18. **`linalg.*` and `spectral.*` routing** — wetSpring V147 reported PG-10: `method_to_capability_domain()` didn't handle these prefixes, forcing springs to manually pass `"tensor"`. Fixed in composition module. **ABSORBED Apr 19, 2026.**
19. **Modular bare/domain split** — healthSpring V54's `bare.rs` / `domain.rs` split for guideStone binaries is a clean pattern: properties that need no primals live in `bare.rs`, IPC-dependent checks live in `domain.rs`. **DOCUMENTED Apr 19, 2026** — not absorbed as library code; springs own their layout.
20. **Feature-gated `guidestone`** — all active springs (healthSpring, neuralSpring, wetSpring, ludoSpring) use `required-features = ["guidestone"]` with `primalspring` as an optional dep. Canonical pattern for guideStone binaries. **DOCUMENTED Apr 19, 2026.**

### guideStone Readiness (April 20, 2026)

| Spring | guideStone Readiness | Evidence | Blockers |
|--------|---------------------|----------|----------|
| **primalSpring v0.9.17** | **4 — NUCLEUS guideStone works** | **187/187 ALL PASS** (13/13 BTSP, 8 cellular BTSP-enforced) against live plasmidBin NUCLEUS (12 primals). P3 CHECKSUMS (BLAKE3, 18 files). 41/41 bare. Layers 0–7 validated. biomeOS v3.25 absorbed. *(was 67/67 at v0.9.15)* | Cross-substrate parity for Level 5 |
| hotSpring v0.6.32 | **5 — Certified** | `hotspring_guidestone`: bare + NUCLEUS, all 5 properties, `primalspring::composition` API. Cross-substrate parity (Python/CPU/GPU). | P2–P4 checks are partial (metadata checks, not deep validation). Exit-code 2 semantics inconsistent with `exit_code_skip_aware()`. |
| healthSpring V54 | **2 — Properties documented** | `healthspring_guidestone` modular binary (bare.rs/domain.rs). P1/P2/P4/P5 validated. `primalspring::composition` API. Feature-gated `guidestone`. | **P3 missing** (CHECKSUMS); `bare_exit_code` doc comment contradicts implementation; dual-tower ionic bridge untested in guideStone |
| neuralSpring V134 | **2 — Properties documented** | `neuralspring_guidestone`: 4-phase binary (bare/liveness/parity/additive). `GUIDESTONE_PROPERTIES.md` documents all 5. `primalspring::composition` API. Feature-gated. | **P3 missing** (CHECKSUMS); machine-readable provenance; Gap 11 (18 barraCuda surface methods) |
| wetSpring V147 | **3 — Bare guideStone works** | `wetspring_guidestone`: B0/B1 bare + N0–N3 NUCLEUS layers. Strict exit semantics (0=full, 2=bare, 1=fail). 9/9 bare checks pass. Expanded N2 (linalg, spectral, stats). Feature-gated. | **PG-10** (spectral/linalg routing — RESOLVED in primalSpring). PG-11 manifest drift. Live NUCLEUS for N1–N3. |
| ludoSpring V45 | **3 — Bare guideStone works** | `ludospring_guidestone`: 15 bare + 15 domain IPC checks. Readiness 3 per wateringHole handoff. | Local checkout stale (pre-V45); needs fresh pull. |
| airSpring v0.10.0 | **0 — Not started** | Pre-delta; 90.56% coverage | No IPC client or guideStone yet |
| groundSpring V124 | **0 — Not started** | Pre-delta; 92% coverage | No IPC client or guideStone yet |

Readiness levels: 0=not started, 1=validation exists, 2=properties documented,
3=bare guideStone works, 4=NUCLEUS guideStone works, 5=certified.
See `wateringHole/GUIDESTONE_COMPOSITION_STANDARD.md` for the full standard.

### guideStone-Discovered Gaps (April 19, 2026)

| ID | Gap | Reporter | Owner | Status |
|----|-----|----------|-------|--------|
| **PG-10** | `method_to_capability_domain()` did not route `spectral.*` or `linalg.*` methods — springs had to pass `"tensor"` manually. | wetSpring V147 | primalSpring | **RESOLVED** — `linalg` and `spectral` prefixes now route to `"tensor"` |
| **PG-11** | `downstream_manifest.toml` field drift — guideStone readiness and binary fields missing for some springs. | wetSpring V147, neuralSpring V134 | primalSpring | **RESOLVED** — manifest updated for all active springs |
| **PG-12** | Exp403-style legacy IPC surfaces should converge to guideStone pattern. Springs maintaining both old-style `validate_primal_proof` and new `guideStone` binaries. | wetSpring V147 | Each spring team | **OPEN** — springs should migrate; old binaries remain as historical |
| **PG-13** | **P3 (self-verifying) universal blocker** — no spring has CHECKSUMS infrastructure. Every guideStone except hotSpring cannot certify P3. | All springs | primalSpring | **RESOLVED** — `primalspring::checksums` module added with `generate_manifest()` / `verify_manifest()` |
| **PG-14** | Exit code 2 semantics inconsistency — `exit_code_skip_aware()` returns 0 when bare passes and liveness is all-skipped (`passed > 0`). Springs document exit 2 for bare-only. | hotSpring V0.6.32, audit | primalSpring | **RESOLVED** — `primalspring_guidestone` now uses explicit `if v.exit_code() == 0 { 2 } else { 1 }` pattern. `exit_code_skip_aware()` retained for backward compat; guideStones should use the explicit pattern. |
| **PG-15** | `healthspring_guidestone` `bare_exit_code` doc comment says "2 if any bare check failed" but code returns 2 when bare **succeeded**. | healthSpring V54 audit | healthSpring team | **OPEN** — fix the doc comment |
| **PG-16** | Capability discovery does not check family-aware socket names (`{cap}-{family}.sock`). Only checks `{cap}.sock` (bare). Blocks tensor/dag/ledger/attribution/commit discovery when `FAMILY_ID` is set. | Live NUCLEUS validation | primalSpring | **RESOLVED** — family-aware tier added to `discover_by_capability()` |
| **PG-17** | `start_primal.sh` uses `serve` for provenance trio (rhizocrypt, loamspine, sweetgrass) but correct subcommand is `server`. | Live NUCLEUS launch | plasmidBin | **RESOLVED** — fixed in `start_primal.sh` |
| **PG-18** | Squirrel's `--bind` flag rejected; script passed `--bind` alongside `--port`. | Live NUCLEUS launch | plasmidBin | **RESOLVED** — removed `--bind` from squirrel case in `start_primal.sh` |
| **PG-19** | BearDog requires `BEARDOG_FAMILY_SEED` or `.family.seed` in production mode. `nucleus_launcher.sh` does not provide it by default. | Live NUCLEUS launch | plasmidBin | **RESOLVED** — `plasmidBin/nucleus_launcher.sh` now has `resolve_family_seed()` with 4-tier fallback: `BEARDOG_FAMILY_SEED` → `FAMILY_SEED` → `$SOCKET_DIR/.family.seed` → auto-generate from `/dev/urandom`. Seed persisted to `.family.seed` and exported as both `FAMILY_SEED` and `BEARDOG_FAMILY_SEED`. |
| **PG-20** | CoralReef `shader.compile.capabilities` returns `supported_archs` not `capabilities` array. guideStone expected `capabilities`. | Live NUCLEUS validation | primalSpring / coralReef | **RESOLVED** — guideStone updated to accept `supported_archs` |
| **PG-21** | Songbird and petalTongue speak HTTP on UDS. primalSpring's IPC client sends raw JSON-RPC. Protocol mismatch causes `ProtocolError`, not connection failure. | Live NUCLEUS validation | primalSpring | **RESOLVED** — `is_protocol_error()` added, treated as SKIP-with-reachable. **Root cause fixed upstream (Songbird Wave 148)**: orchestrator's `handle_ndjson_session` was single-shot (broke after one request); now persistent. Songbird UDS is NDJSON, not HTTP. Springs should now see full multi-request sessions without SKIP. |
| **PG-22** | Validation graphs (`crypto_negative_validate.toml`, `nucleus_atomics_validate.toml`) had invalid TOML: duplicate `[graph.node.operation]` tables. | Bare guideStone validation | primalSpring | **RESOLVED** — migrated to `[graph.nodes.operation]` (per-entry) |
| **PG-23** | Meta-tier fragment nodes (squirrel, petaltongue) had no `order` field, defaulting to 0. Caused duplicate order values when resolved into profiles. | Bare guideStone validation | primalSpring | **RESOLVED** — added explicit orders 10/11/12 to `meta_tier.toml` |

### Cross-Spring Consolidated Gaps (April 20, 2026)

Gaps reported by downstream springs during guideStone Level 3–4 validation.
These are **upstream primal evolution requests** — the springs cannot resolve
them locally. Ordered by ecosystem impact (how many springs are blocked).

| ID | Gap | Primal | Reporter(s) | Severity | Status |
|----|-----|--------|-------------|----------|--------|
| **PG-24** | **6 missing JSON-RPC methods**: `stats.variance`, `stats.correlation`, `linalg.solve`, `linalg.eigenvalues`, `spectral.fft`, `spectral.power_spectrum`. | barraCuda | wetSpring PG-13, healthSpring §19, neuralSpring Gap 11 | **CRITICAL** | **RESOLVED** — Sprint 44 (c7d7d83): all 6 wired + `tensor.matmul_inline` (32→39 methods) |
| **PG-25** | `activation.fitts` formula variant: Shannon formulation. | barraCuda | ludoSpring V46 | **HIGH** | **RESOLVED** — Sprint 44: corrected to `log₂(D/W + 1)` per MacKenzie 1992 |
| **PG-26** | `activation.hick` formula variant. | barraCuda | ludoSpring V46 | **HIGH** | **RESOLVED** — Sprint 44: confirmed already uses `log₂(N)` (no change needed) |
| **PG-27** | `noise.perlin3d` lattice invariant at origin. | barraCuda | ludoSpring V46 | **MEDIUM** | **RESOLVED** — Sprint 44: confirmed already returns 0.0 (no change needed) |
| **PG-28** | Response schema inconsistency: some methods return `{"result": N}`, others `{"mean": N}`. | barraCuda | ludoSpring V46, hotSpring | **MEDIUM** | **RESOLVED** — Sprint 44: all scalar methods now include `"result"` key |
| **PG-29** | `tensor.matmul` handle-based API friction. | barraCuda | wetSpring PG-17, ludoSpring V46 | **MEDIUM** | **RESOLVED** — Sprint 44: `tensor.matmul_inline` added as convenience path |
| **PG-30** | Squirrel BTSP-only socket: plain JSON-RPC clients get connection reset. | Squirrel | wetSpring PG-14 | **MEDIUM** | **RESOLVED** — Squirrel 0497648: first-byte peek auto-detects `{` → plain JSON-RPC fallback |
| **PG-31** | ToadStool `compute.dispatch` not registered on JSON-RPC socket. | ToadStool | wetSpring PG-15 | **MEDIUM** | **RESOLVED** — S174 (de4f330): `compute.dispatch` registered as literal route + added to `DIRECT_JSONRPC_METHODS` + `SemanticMethodRegistry` |
| **PG-32** | rhizoCrypt capability-based discovery (no manifest published). | rhizoCrypt | ludoSpring V46 | **MEDIUM** | **RESOLVED** — S43.7 (7702b67): `publish_manifest()` on startup writes `rhizocrypt.json` with UDS path + 28 capabilities; `unpublish_manifest()` on shutdown |
| **PG-33** | loamSpine startup panic: `block_on` inside async runtime. | loamSpine | ludoSpring V46 | **HIGH** | **RESOLVED** — loamSpine d34100f: `std::thread::spawn` + `oneshot` replaces `spawn_blocking` |
| **PG-34** | biomeOS `biomeos-types` missing `secret` module. | biomeOS | wetSpring V148 | **MEDIUM** | **RESOLVED** — v3.18 (4a43206): `secret.rs` tracked (was gitignored by `*secret*` glob) + `next_tcp_port()` trial-binds to skip occupied ports + post-spawn `register_spawned_primal()` for capability auto-registration |
| **PG-35** | BearDog connection reset without BTSP. | BearDog | hotSpring, wetSpring, ludoSpring | **LOW** | **RESOLVED** — Wave 56 (353c65f): first-byte `{` auto-detect, cleartext bypass for JSON-RPC clients documented |
| **PG-36** | `stats.std_dev` N-1 vs N convention. | barraCuda | wetSpring PG-16 | **LOW** | **RESOLVED** — Sprint 44: convention metadata included in response |

| **PG-38** | barraCuda `activation.fitts` default is Shannon (log₂(D/W+1)), not classic Fitts (log₂(2D/W)). `activation.hick` default is log₂(n), not log₂(n+1). | barraCuda | ludoSpring V53 (GAP-11) | **LOW** | **DOCUMENTED** — Both are intentional variants with explicit `variant`/`include_no_choice` params. Cross-spring callers must pass `variant: "fitts"` for classic Fitts and `include_no_choice: true` for textbook Hick. Not a bug — a convention mismatch. barraCuda IPC surface docs explain both. |

**Impact summary (April 21 re-audit, updated April 25)**: **All 13 cross-spring gaps RESOLVED.** PG-19 (nucleus_launcher.sh seed auto-gen) RESOLVED — `resolve_family_seed()` added to `plasmidBin/nucleus_launcher.sh`. PG-38 DOCUMENTED — barraCuda Fitts/Hick variant defaults are Shannon/log₂(n) by design; callers use explicit params for classic formulas. barraCuda Sprint 44 resolved PG-24–PG-29. Squirrel resolved PG-30 (BTSP auto-detect). ToadStool S174 resolved PG-31 (`compute.dispatch` registered). rhizoCrypt S43.7 resolved PG-32 (manifest-based discovery with 28 capabilities). loamSpine resolved PG-33 (startup panic). biomeOS v3.18 resolved PG-34 (secret module + port trial-bind + auto-registration). BearDog resolved PG-35 (cleartext bypass). barraCuda resolved PG-36 (std_dev convention). Zero blocking upstream gaps remain — downstream springs are fully unblocked for primal composition validation.

### Absorbed Patterns (April 20, 2026)

Patterns independently invented by 2+ springs, now absorbed into primalSpring:

| Pattern | Absorbed From | Into | API |
|---------|--------------|------|-----|
| `call_or_skip` pipeline helper | ludoSpring V46, healthSpring V56 | `primalspring::composition::call_or_skip()` | Returns `Option<Value>`, chains pipeline steps |
| `is_skip_error` classification | ludoSpring V46, hotSpring, wetSpring | `primalspring::composition::is_skip_error()` | Unified absent/protocol/transport skip |
| Domain-as-local-composition | healthSpring V56 | Documented in guideStone standard | Domain functions compose from IPC-proven primitives |
| Tolerance hierarchy | hotSpring v0.6.32 | `primalspring::tolerances` + guideStone standard v1.2.0 | Named constants with ordering invariant |
| BLAKE3 checksums for P3 | hotSpring v0.7.0 (origin) | `primalspring::checksums` | All 5 springs at L3+ adopted it |
| `v.section()` structured output | neuralSpring V135 | Documented pattern | `ValidationResult::section()` for machine-readable grouping |

---

## Gap Refinement — v0.9.17 / Phase 45 (April 20, 2026)

### Resolved Locally

| Gap | Was | Fixed | Validated |
|-----|-----|-------|-----------|
| barraCuda `tensor.matmul` returns nested 2D array | guidestone FAIL | `validate_parity_vec` flattens nested arrays | 86/86 guidestone PASS |
| BearDog `crypto.sign` expects base64-encoded message | guidestone FAIL (symbol 95 at offset 10) | Encode raw message with standard base64 before sending | `crypto:ed25519_sign` PASS |
| barraCuda `math.dot`/`math.l2_norm` don't exist | exp068 3/6 | Rewired to `stats.mean`, `stats.variance`, `activation.fitts` | exp068 6/6 PASS |
| exp067 `live_tower_health` uses wrong discovery key | exp067 18/19 SKIP | Query `security`/`crypto` capabilities instead of `health.liveness` | exp067 18/19 (1 expected SKIP) |
| Capability symlinks not created by `start_primal.sh` | Manual `ln -sf` required for discovery | `create_capability_symlinks()` runs post-launch | 12 capability categories auto-linked |
| Webb expects `game.record_action`, `game.push_scene`, `game.query_vertices` | Not implemented in ludoSpring | Wired in barracuda IPC handler with session DAG tracking | 22/22 IPC tests pass |

### Upstream Gaps — ALL RESOLVED (Audit April 21, 2026)

All 8 upstream gaps flagged in Phase 45 have been **resolved by the primal teams**.
Each primal was pulled and audited against the specific debt item.

| Primal | Gap (was) | Resolution | Verified In |
|--------|-----------|------------|-------------|
| **BearDog** | `crypto.sign` didn't expose `public_key` | Wave 62 (BD-PG-01): `public_key` (standard base64) in sign response. Roundtrip test added. | `asymmetric_tests.rs` |
| **BearDog** | Mixed URL-safe / standard base64 | Wave 62 (BD-PG-02): Ed25519 outputs standardized to standard base64. Verify accepts legacy. | `asymmetric_tests.rs` |
| **rhizoCrypt** | BTSP connection reset on `health.check` | S45.1: First-byte `{` auto-detect on UDS. `UNAUTHENTICATED_METHODS` allowlist. | `newline.rs` tests |
| **sweetGrass** | Same BTSP connection reset | `PeekedStream` first-byte auto-detect on UDS + TCP. | `uds/tests/autodetect.rs` |
| **loamSpine** | Socket naming / `discover_by_capability("ledger")` | v0.9.16 (GAP-MATRIX-12): `{primal}-{family}.sock` adopted. `ledger.sock` symlink at runtime. | STATUS.md: PASS |
| **Songbird** | Routes by capability only, name fails | Wave 151 (PG-37): capability-first with primal-name fallback. `ipc.resolve_by_name` alias. | `ipc_registry.rs` |
| **barraCuda** | Handle-based tensor ops fail without GPU | Sprint 44c: CPU fallback for 7 handle-based ops. `"backend": "cpu"` on headless. | `TENSOR_WIRE_CONTRACT.md` |
| **biomeOS** | HTTP-on-UDS, JSON-RPC probes get HTTP 400 | v3.22: Dual-protocol auto-detect on UDS. First byte `{` → NDJSON handler. | `unix_server.rs` |

---

### PG-39: Graph Schema Incompatibility (primalSpring vs biomeOS)

**Status:** MITIGATED locally — upstream alignment pending
**Component:** primalSpring cell graphs, biomeOS deploy parser, cell_launcher.sh
**Discovered:** April 26, 2026 (ludoSpring V53 audit)

primalSpring cell graphs use `[[graph.nodes]]` with `name`, `binary`, `by_capability`,
`order`, `spawn`, `security_model` fields. biomeOS deploy parser (`neural_graph::Graph`)
expects either `[[nodes]]` with `id` + `[nodes.primal]` + `[nodes.operation]`, or
`[[graph.nodes]]` via `convert_deployment_node` which reads `id`, `capability`,
`config.primal`.

Key mismatches:
- `name` (primalSpring) vs `id` (biomeOS) — biomeOS requires `id`
- `binary` field — biomeOS has no binary field, resolves via `by_capability`
- `security_model` — primalSpring-only, biomeOS ignores
- `order` / `spawn` — primalSpring-only, biomeOS uses `depends_on` DAG

**Local mitigation (April 26):**
- `primalspring_guidestone` cell validator reads `name` OR `id` for node identity
- `cell_launcher.sh` parser handles both `[[graph.nodes]]` and `[[nodes]]` formats
- ludoSpring maintains biomeOS-compatible cell graph (`[[nodes]]` + `id`) alongside

**Upstream action:** biomeOS team to consider accepting primalSpring's `[[graph.nodes]]`
with `name` + `binary` fields in `convert_deployment_node`, or primalSpring to migrate
all cell graphs to biomeOS `[[nodes]]` format. Recommend the latter since biomeOS is
the runtime orchestrator.

---

**Open upstream gaps as of April 26, 2026: PG-44 (coralReef Phase D — LOW).**
**Resolved this cycle: PG-42 (toadStool S204), PG-43 (petalTongue PG-43 commit), PG-39 (biomeOS v3.26).**
**Also resolved: bearDog lineage IPC (Wave 69), barraCuda 18-method expansion (Sprint 45), rhizoCrypt DID alignment (S48), nestgate streaming storage (Session 46), biomeOS cellular deploy (v3.27).**

---

## Graphics Node Gaps — Phase 45c (April 26, 2026)

Three upstream evolutions identified for petalTongue as full graphics system.
These are architectural evolution requests, not bugs.

### PG-42: toadStool Display Phase 2 — petalTongue Integration

**Status:** RESOLVED — toadStool S204 (`d2a327be6`)
**Component:** toadStool display backend (`crates/runtime/display/`)

`display.present`, `display.subscribe_input`, `display.poll_events` now wired
in `ipc/dispatch.rs` with `DisplayClient` wrapper in `client/operations.rs`.
Tests cover all three methods. **Remaining**: `display.composite` (multi-layer)
and `transport.bridge` (external process gateway) are not yet implemented.

### PG-43: petalTongue Texture Primitive

**Status:** RESOLVED (core) — petalTongue commit `94f6068`
**Component:** `petal-tongue-scene` SceneGraph `Primitive` enum

`Texture` variant added to `Primitive` with `texture_id`, dimensions, UV rect,
opacity, tint. `TextureRegistry` in visualization state. IPC methods
`visualization.texture.upload` (base64 RGBA8) and `visualization.texture.attach`
(placeholder — real memfd pending toadStool Phase 2 compositing). `From<Sprite>`
bridge maps game sprites to `Primitive::Texture`. **Remaining**: egui
`TextureResolver` for live pixel display, overlay mode (Wayland layer-shell),
real shared-memory attach path.

**Handoff:** `wateringHole/handoffs/PETALTONGUE_TEXTURE_PRIMITIVE_OVERLAY_HANDOFF_APR26_2026.md`

### PG-44: coralReef Phase D — Mixed Command Streams

**Status:** UPSTREAM — coralReef team
**Component:** `coral-gpu`, toadStool dispatch
**Priority:** LOW (longest pole, CPU path works for moderate frame rates)

toadStool `NEXT_STEPS.md` lists Phase D (draw + compute + framebuffer mixed
command streams) as planned but blocked on coralReef/FECS. Needed for
zero-copy GPU compute → display (e.g., barraCuda physics → framebuffer
without CPU readback). The CPU-based path works today for dashboards and
moderate-complexity visualization.

**Handoff:** `wateringHole/handoffs/CORALREEF_PHASE_D_MIXED_COMMANDS_HANDOFF_APR26_2026.md`

---

### Scorecard After Full Resolution

| Component | Phase 45 Start | After Local Fix | Expected After Revalidation |
|-----------|---------------|-----------------|----------------------------|
| guidestone | 84/86 (2 FAIL) | **86/86 PASS** (6 SKIP) | 86/86 PASS (≤3 SKIP — BTSP primals now probed) |
| exp067 | 18/19 (1 SKIP) | **18/19** | 18/19 (1 SKIP — ludoSpring not deployed as primal) |
| exp068 | 3/6 (3 FAIL) | **6/6 PASS** | 6/6 PASS |
| exp072 | 24/31 | **24/31** | ≥27/31 (biomeOS JSON-RPC + tensor CPU fallback now work) |
| ludoSpring game.* | 12 methods | **15 methods** | 15 methods |

---

---

## TTT Composition Validation — April 27, 2026

Full NUCLEUS Tic-Tac-Toe composition (`FAMILY_ID=ttt`) exercised all primals
from plasmidBin in a capability-routed game loop. This is the first interactive
composition to route through the full NUCLEUS stack by capability discovery.

### E2E Results: 10/10 PASS (after protocol corrections)

| Primal | Method | Result | Notes |
|--------|--------|--------|-------|
| beardog | `health.liveness` | PASS | |
| beardog | `crypto.sign` | PASS | Ed25519, returns signature + public_key |
| songbird | `health.liveness` | PASS | |
| toadstool | `health.liveness` | PASS | Needs longer socat timeout (>5s) |
| barraCuda | `tensor.create` | PASS | Returns tensor_id for downstream ops |
| barraCuda | `tensor.matmul` | PASS | Requires `lhs_id`/`rhs_id` (not inline data) |
| loamSpine | `spine.create` | PASS | Requires `name` + `owner` + `description` |
| loamSpine | `entry.append` | PASS | `entry_type` is struct variant (e.g. `{"MetadataUpdate":{"field":"...","value":"..."}}`) + `committer` |
| petalTongue | `visualization.render.scene` | PASS | SceneGraph with nodes map + root_id |
| petalTongue | `interaction.subscribe` | PASS | Requires `subscriber_id` |
| petalTongue | `interaction.poll` | PASS | Requires `subscriber_id` (not session_id) |
| petalTongue | `proprioception.get` | PASS | Returns full ProprioceptionSnapshot |

### Gaps Discovered

#### PG-45: rhizoCrypt UDS — No JSON-RPC Response (GAP-06 reconfirmed)

**Status:** UPSTREAM — rhizoCrypt team
**Component:** `rhizocrypt` UDS server
**Priority:** MEDIUM

rhizoCrypt binds its socket (`rhizocrypt-ttt.sock`) but does not respond to any
JSON-RPC including `health.liveness`. The socket accepts connections but returns
no data. This blocks DAG session creation for provenance recording in all
interactive compositions. Confirmed across both `nucleus01` and `ttt` families.

**Impact:** Game move DAG recording disabled; provenance chain incomplete.

#### PG-46: toadStool — Slow Socket Response / BTSP Blocking

**Status:** UPSTREAM — toadStool team
**Component:** `toadstool` UDS server
**Priority:** LOW

toadStool socket (`toadstool-ttt.sock`, permissions `srw-------`) sometimes
returns empty on short timeouts. With `timeout 10 -t5 socat` it responds
correctly. This may be BTSP handshake overhead or slow initial connection
negotiation. The restrictive socket permissions (`0600` vs `0660` for others)
suggest a different socket creation path.

**Impact:** Transient — works with longer timeouts. Composition scripts should
use ≥10s timeout for toadStool.

#### PG-47: barraCuda — `stats.entropy` Method Not Found

**Status:** UPSTREAM — barraCuda team
**Component:** `barracuda` method registry
**Priority:** LOW

`stats.entropy` returns "Unknown method". barraCuda supports `tensor.create`,
`tensor.matmul`, `stats.mean`, `stats.std_dev`, `stats.variance`, etc. but
does not have a general entropy calculation. Composition scripts should compute
entropy from tensor data or skip.

**Impact:** Workaround available (compute from raw tensor values).

#### PG-48: petalTongue plasmidBin Binary — winit Threading Panic

**Status:** UPSTREAM — petalTongue team
**Component:** `petaltongue` musl binary in plasmidBin
**Priority:** HIGH

The plasmidBin musl binary panics on `live` mode with:
```
Initializing the event loop outside of the main thread
```
The locally-built `target/release/petaltongue` works correctly (eframe/winit
runs on thread 01). The musl binary routes the event loop through
`tokio-rt-worker` threads, causing the panic. This blocks plasmidBin-only
deployments from using live desktop mode.

**Workaround:** Use locally-built petaltongue binary for `live` mode.
**Fix:** petalTongue musl build needs `--features ui` with correct winit
backend (X11 `any_thread` or ensure main-thread dispatch).

#### Protocol Notes for Composition Scripts

- **beardog `crypto.verify`**: expects valid base64 public_key (32 bytes Ed25519), not string
- **barraCuda `tensor.matmul`**: two-step — `tensor.create` first, then `tensor.matmul` with `lhs_id`/`rhs_id`
- **loamSpine `entry.append`**: `entry_type` must be struct variant enum (e.g. `{"MetadataUpdate":{...}}`), requires `committer` field
- **petalTongue `interaction.poll`**: requires prior `interaction.subscribe` with `subscriber_id`
- **sweetGrass**: responds to health but needs longer timeout; has limited composition API surface
- **DISPLAY**: Must match actual X server (`:1` on this system, not `:0`)

### Files Created

- `tools/ttt_nucleus.sh` — NUCLEUS launcher for game cell
- `tools/ttt_composition.sh` — game loop routing through NUCLEUS by capability
- `graphs/cells/tictactoe_cell.toml` — pure composition cell graph

### Template Value

This composition validates the pattern for all future interactive compositions
(ludoSpring, esotericWebb). The capability discovery, graceful fallback, and
multi-primal routing patterns are directly reusable.

---

### Cross-Spring Convergence Gaps (Phase 46 — April 27, 2026)

Gaps surfaced by 5 springs deploying the composition template library. These
represent ecosystem-wide patterns, not single-spring issues.

#### PG-49: Composition Library socat Dependency — RESOLVED LOCAL

**Status:** RESOLVED — primalSpring local fix
**Component:** `nucleus_composition_lib.sh`, `composition_nucleus.sh`
**Priority:** HIGH (blocked 4 springs)
**Reported by:** wetSpring (PG-20/21), healthSpring (#27)

`send_rpc()` and `health_check()` hardcoded `socat` for UDS transport. Springs
without `socat` installed could not run compositions.

**Fix:** Transport fallback chain: socat → python3 socket → nc (ncat/netcat).
Both `nucleus_composition_lib.sh` and `composition_nucleus.sh` updated.

#### PG-50: PRIMAL_LIST Missing nestgate and squirrel — RESOLVED LOCAL

**Status:** RESOLVED — primalSpring local fix
**Component:** `composition_nucleus.sh`
**Priority:** MEDIUM
**Reported by:** healthSpring (#26), neuralSpring (Gap 14)

Default `PRIMAL_LIST` omitted `nestgate` (storage) and `squirrel` (AI), causing
compositions that needed storage or AI to require manual env overrides.

**Fix:** Default `PRIMAL_LIST` now includes all 10 primals: beardog, songbird,
nestgate, squirrel, toadstool, barracuda, rhizocrypt, loamspine, sweetgrass,
petaltongue. Added `EXTRA_PRIMALS` env var for additional primals beyond defaults.

#### PG-51: Songbird Crypto Provider Discovery Failure — RESOLVED UPSTREAM

**Status:** RESOLVED — Songbird wave173 (April 27, 2026)
**Component:** Songbird startup / BearDog socket resolution
**Priority:** MEDIUM
**Reported by:** healthSpring (#24), wetSpring (PG-22)

Songbird now has family-scoped BearDog discovery: `security-{fid}.sock` →
`beardog-{fid}.sock` (legacy) → `BEARDOG_SOCKET` env fallback. CLI flag
`--security-socket` is canonical with `--beardog-socket` as alias.

**Verification:** Pull songbird, rebuild, and confirm no symlink workaround needed.

#### PG-52: Provenance Trio UDS Empty Responses — RESOLVED UPSTREAM

**Status:** RESOLVED — all three primals evolved (April 27, 2026)
**Component:** Trio JSON-RPC handlers on UDS
**Priority:** HIGH (cross-spring: 4 springs affected)
**Reported by:** healthSpring (#23), wetSpring (PG-18), neuralSpring (Gap 14), hotSpring (PG-45 family)

Root cause identified independently by all three teams:
- **rhizoCrypt (S49):** Liveness gate routed all `{`-prefixed JSON to `handle_liveness_connection`,
  blocking `dag.*` methods. Fix: plain JSON-RPC on UDS now routes to `handle_newline_connection`
  (full handler). PG-52 repro test: `test_plain_jsonrpc_data_methods_on_btsp_uds`.
- **loamSpine:** Double-`BufReader` on post-BTSP paths caused empty reads. Fix: removed duplicate
  buffering. UDS trio lifecycle test: `uds_trio_lifecycle_create_append_seal`.
  Note: plasmidBin binary needs rebuild to pick up fix.
- **sweetGrass:** EOF without trailing newline treated as I/O error in `detect_protocol`.
  Fix: EOF is valid line-end; `Unknown` protocol returns JSON-RPC error instead of silent close.
  7 new UDS tests including `braid.create` roundtrip. Callers need `\n`-terminated requests
  and >=10s read timeout.

**Verification:** Pull all three, rebuild plasmidBin binaries, reharvest. This also resolves
PG-06 and PG-45.

#### PG-53: petalTongue Server Mode Proprioception — RESOLVED UPSTREAM

**Status:** RESOLVED — petalTongue (April 27, 2026)
**Component:** `proprioception.get` in `server` mode
**Priority:** LOW
**Reported by:** healthSpring (#25)

New handler `system/proprioception.rs` returns complete JSON with `frame_rate`,
`active_scenes`, `total_frames`, `user_interactivity`, `mode`, `uptime_secs`,
and `window` fields. Dispatched via `dispatch.rs`. Tests:
`proprioception_get_server_mode_returns_zero_fps`.

**Note:** Production `UnixSocketServer` always sets `rendering_awareness: Some(...)`,
so the "0 fps server" shape may only appear in test paths. Monitor in live compositions.

#### PG-54: Adaptive Sensor Polling — LOCAL (future)

**Status:** DEFERRED — composition library enhancement
**Component:** `nucleus_composition_lib.sh`
**Priority:** LOW
**Reported by:** neuralSpring (Gap 14), hotSpring (tick model)

Fixed `POLL_INTERVAL` (default 0.5s) doesn't suit all domains. Interactive
games need ~16ms (60Hz), physics simulations need convergence-driven ticks,
agent loops need event-driven polling.

**Future:** Allow domain hooks to specify tick mode (fixed, adaptive, event-driven).

---

*Resolved gaps, compliance matrices, and historical evolution snapshots are in
[`PRIMAL_GAPS_RESOLVED_HISTORY.md`](PRIMAL_GAPS_RESOLVED_HISTORY.md).*
