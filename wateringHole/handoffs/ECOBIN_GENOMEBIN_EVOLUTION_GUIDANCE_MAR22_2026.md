# ecoBin / genomeBin Evolution Guidance

**Date**: 2026-03-22
**From**: primalSpring v0.7.0 (hardware validation)
**To**: All primal teams, sourDough, biomeOS
**Status**: Guidance — based on hardware validation findings

---

## The Problem

primalSpring's hardware validation (March 22, 2026) revealed that while the
ecoBin and genomeBin *standards* are well-defined in wateringHole, the actual
*artifacts* don't comply:

1. **Zero .genome self-extracting archives exist** — only raw ELF binaries
2. **Mixed linking** — some static-pie, some dynamically linked, some debug
3. **Missing architectures** — plasmidBin has only x86_64, no aarch64
4. **Individual solutions** — each primal team builds differently; no unified pipeline

## Current State vs Standard

### ecoBin Standard (wateringHole/ECOBIN_ARCHITECTURE_STANDARD.md)

| Requirement | Status |
|-------------|--------|
| Pure Rust, zero C deps | Most primals comply (deny.toml enforced per-spring) |
| Static musl linking | **PARTIAL** — toadstool, squirrel, biomeos-musl are static; beardog, songbird, nestgate are dynamic |
| Cross-compile x86_64 + aarch64 minimum | **PARTIAL** — wateringHole/genomeBin has both arches but mixed quality |
| PIE required | Most comply (pie or static-pie) |
| Stripped | **PARTIAL** — neural-api-server, songbird, petaltongue ship with debug_info |

### genomeBin Standard (wateringHole/GENOMEBIN_ARCHITECTURE_STANDARD.md)

| Requirement | Status |
|-------------|--------|
| Self-extracting shell+tar archive | **ZERO exist** in active ecosystem |
| Per-arch ecoBins embedded | N/A (no .genome files produced) |
| System detection + install logic | N/A |
| Health validation post-install | N/A |
| GPG signature (optional) | N/A |

## The Opportunity: Abstract the Cross-Compile

Rust's `cargo` already solves the hardest problem — a single `Cargo.toml`
produces correct binaries for any target triple. The ecoBin standard
describes the *desired output*. What's missing is the *automated pipeline*
that bridges `cargo build` to `.genome` packaging.

### What Rust Gives Us for Free

```bash
# One command, any architecture
cargo build --release --target aarch64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
cargo build --release --target armv7-unknown-linux-musleabihf
cargo build --release --target wasm32-unknown-unknown
```

The `musl` targets produce fully static binaries with zero runtime
dependencies. primalSpring proved this: 2.99 MB static aarch64 binary
runs on Android Pixel without any shared libraries.

### What We Should Build: `cargo genome`

Rather than each primal team manually running cross-compile + strip + package,
the ecosystem should evolve a `cargo genome` workflow (likely in sourDough):

```
cargo genome build
  ├── cargo build --release --target x86_64-unknown-linux-musl
  ├── cargo build --release --target aarch64-unknown-linux-musl
  ├── strip both binaries
  ├── create payload.tar.gz (ecoBins/ + configs/ + scripts/)
  ├── prepend deployment wrapper (system detection, install, health check)
  ├── produce primal.genome (self-extracting)
  ├── blake3sum + sha256sum
  └── update manifest.toml + checksums.toml
```

This is the genomeBin standard's "Creating a genomeBin" section automated.
sourDough already has the scaffolding role — it should own this toolchain.

### Evolution Path

| Phase | What | Who |
|-------|------|-----|
| 1. Static musl everywhere | All primal teams add `aarch64-unknown-linux-musl` to CI | All teams |
| 2. Strip debug info | Release builds use `strip = true` in `[profile.release]` | All teams |
| 3. Pin dual-arch to plasmidBin | `plasmidBin/primals/{name}-{arch}` | All teams |
| 4. `cargo genome` prototype | sourDough implements the packager | sourDough team |
| 5. genomeBin CI | Automated .genome production on release tags | sourDough + CI |
| 6. genomeBin deployment | biomeOS `deploy` command uses .genome self-extractors | biomeOS team |

### Immediate Action: Profile.release Strip

Every primal's root `Cargo.toml` should include:

```toml
[profile.release]
strip = true
lto = true
```

This is trivial and immediately shrinks binaries and removes debug info.
primalSpring already produces stripped static binaries via musl.

## Why This Matters More Than Individual Solutions

The current approach — each team building for their own host, occasionally
cross-compiling when someone remembers — doesn't scale. The ecosystem has
14 primals and 8 springs. Each needs at minimum x86_64 + aarch64. That's
44+ binaries to produce, package, sign, and distribute.

The genomeBin self-extractor model solves this by:
1. **One artifact per primal** — contains all architectures
2. **System detection at deploy time** — the .genome script picks the right binary
3. **Health validation built in** — post-install `doctor` mode runs automatically
4. **Update/rollback built in** — the wrapper handles versioning

This is the `sourDough` promise: scaffolding that makes the ecoBin → genomeBin
pipeline automatic. primalSpring's hardware validation proves the Rust
cross-compile foundation works — the packaging layer is what's missing.

## Android-Specific Guidance

For Pixel / Android deployment, the key requirements are:

1. **Target**: `aarch64-unknown-linux-musl` (NOT `aarch64-linux-android`)
   - musl produces static binaries that run on Android via ADB shell
   - The Android NDK target adds unnecessary complexity for server binaries
2. **Socket IPC**: Use abstract sockets (`@name` in Linux socket namespace)
   - Android SELinux blocks filesystem Unix sockets in `/data/local/tmp/`
   - Abstract sockets work without filesystem permission issues
   - BearDog used to support this via `BEARDOG_ABSTRACT_SOCKET` env var
3. **Working directory**: Must `cd /data/local/tmp/` before running
   - The root filesystem is read-only; audit logs, state files need writable CWD
4. **Environment**: Set `FAMILY_ID`, `NODE_ID`, `{PRIMAL}_SOCKET` explicitly
   - No XDG_RUNTIME_DIR on Android; fall through to explicit env vars

## Checksums and Provenance

The `wateringHole/genomeBin/checksums.toml` should be regenerated whenever
binaries are updated. Current checksums reference stale builds. The BLAKE3
algorithm is correct (pure Rust, faster than SHA-256). The workflow:

```bash
b3sum primals/beardog/v0.9.0/beardog-x86_64-linux > checksums for that entry
```

sweetGrass (PROV-O provenance) should eventually track the full build chain:
source commit → cargo build → strip → package → sign → distribute.

---

**License**: AGPL-3.0-or-later
