# primalSpring v0.7.0 — Hardware Validation Handoff

**Date**: 2026-03-22
**From**: primalSpring v0.7.0
**To**: All primal teams, biomeOS, spring teams
**Status**: Hardware validated — blockers identified for full cross-arch deployment

---

## Summary

primalSpring validated the ecosystem against physical hardware: Pixel 8a
(aarch64 via ADB), three USB spores (biomeOS1, LiveSpore, ColdSpore), and
SoloKey 2 security key. This handoff documents what works, what's broken,
and what each team needs to evolve.

## Hardware Inventory

| Device | Type | Architecture | Mount / Connection |
|--------|------|--------------|-------------------|
| Pixel 8a (akita) | Mobile compute | aarch64 (arm64-v8a) | ADB USB-C debug mode |
| USB1 biomeOS (sda) | ext4, 14.6G | x86_64 primals | `/media/eastgate/biomeOS1` |
| LiveSpore (sdc) | EFI+boot+root, 14.6G | x86_64 primals | `/media/eastgate/biomeOS-livespor` |
| ColdSpore (sdb) | FAT32, 14.6G | x86_64 primals + vault | `/media/eastgate/BEA6-BBCE` |
| SoloKey 2 | FIDO2 / HSM | N/A | USB HID (1209:beee) |

## What Works

### Pixel 8a (aarch64)

All 5 core primals execute on the Pixel from static aarch64 binaries:

| Primal | Version | Linking | Status |
|--------|---------|---------|--------|
| beardog | 0.9.0 | ELF aarch64, static, stripped | Runs, self-initializes HSM/crypto/genetics |
| songbird | 3.33.0 | ELF aarch64, static, stripped | Runs |
| squirrel | 0.1.0 | ELF aarch64, static, stripped | Runs |
| toadstool | 0.1.0 | ELF aarch64, static, stripped | Runs |
| nestgate | 2.1.0 | ELF aarch64, static, stripped | Runs |

**primalSpring itself**: cross-compiles to `aarch64-unknown-linux-musl` (2.99 MB
static binary) and runs on the Pixel reporting v0.7.0.

The Pixel retains its February Dark Forest infrastructure: `.family.seed`,
`.known_beacons.json` (v3.1.0 evolved-genetic-v2), beacon identity
(`pixel_tower`), lineage to eastGate tower, NAT traversal config, and
23 `.genome` files in plasmidBin (v4.1 Universal Bootstrap format).

### USB Spores (x86_64)

| Primal | USB1 | LiveSpore | ColdSpore |
|--------|------|-----------|-----------|
| beardog 0.9.0 | PASS | PASS | PASS |
| songbird 3.33.0 | PASS | PASS | PASS |
| squirrel 0.1.0 | PASS | PASS | PASS |
| toadstool 0.1.0 | PASS | PASS | PASS |
| biomeos 0.1.0 | PASS | PASS | PASS |
| **nestgate** | **SEGFAULT** | **SEGFAULT** | **SEGFAULT** |

### Host Validation (eastGate x86_64)

- **Unit + integration tests**: 10/10 pass, 2 doc-tests pass
- **validate_all**: 47/49 experiments pass (133.7s)
- Failed: exp060 (neural-api-server socket timeout), exp061 (squirrel socket timeout)

## What's Broken

### 1. nestgate USB Binary (All Teams)

The nestgate binary on all three USBs is a corrupt static build (4.7 MB,
statically linked, segfaults on entry). The `plasmidBin/primals/nestgate`
(5.3 MB, dynamically linked) works fine on the host.

**Action**: Rebuild nestgate as `x86_64-unknown-linux-musl` static-pie and
update all USB spore copies.

### 2. BearDog Android Socket Binding (BearDog Team)

BearDog v0.9.0 fails to bind Unix filesystem sockets on Android due to
SELinux restrictions. The February deployments used abstract sockets
(`BEARDOG_ABSTRACT_SOCKET` env var) which worked. The current v0.9.0
socket resolution logic doesn't honor this codepath correctly.

**Error**: `Failed to bind socket on Unix (filesystem): /data/local/tmp/...`

The primals all self-initialize (HSM, crypto, genetics, BTSP provider)
successfully — only the IPC transport layer fails.

**Action**: Restore abstract socket support in beardog's socket resolver.
Android requires `@`-prefixed abstract namespace sockets or TCP fallback.

### 3. Squirrel Socket Startup (Squirrel Team)

Squirrel times out during socket establishment (15s on host, tested via
primalSpring exp061). The Tower (beardog + songbird + neural-api-server) starts
fine, but Squirrel never creates its socket file.

**Action**: Debug squirrel server socket creation. The harness sets
`SQUIRREL_SOCKET` env var and expects the socket to appear within 15s.

### 4. neural-api-server Socket Readiness (biomeOS Team)

`neural-api-server` (biomeOS binary) spawns but its socket never appears
within 45s (primalSpring exp060). The binary starts and runs — it just
doesn't create a Unix socket in the expected location.

**Action**: Verify neural-api-server `neural-api` subcommand creates a socket
at the path specified by `BIOMEOS_SOCKET` or the default nucleation path.

## What's Missing

### 1. No aarch64 Binaries in Ecosystem plasmidBin

`ecoPrimals/plasmidBin/primals/` only contains x86_64 binaries. The
`sources.toml` declares `*-aarch64-musl` assets for all primals but none
have been built and pinned.

**Action (all primal teams)**: Cross-compile with
`cargo build --release --target aarch64-unknown-linux-musl` and pin to
`plasmidBin/primals/`. primalSpring proves this works — 2.99 MB static binary.

### 2. No .genome Self-Extracting Archives

Zero `.genome` packages exist anywhere in the ecosystem filesystem. The
`wateringHole/GENOMEBIN_ARCHITECTURE_STANDARD.md` defines the format
(shell wrapper + compressed tar payload with per-arch ecoBins), and
`wateringHole/genomeBin/deploy.sh` exists, but no actual `.genome` files
have been produced.

The Pixel retains 23 `.genome` files from January/February (v4.1 format)
in `/data/local/tmp/plasmidBin/` — these are the only real genomeBin artifacts.

**Action (sourDough team)**: Run the genomeBin packager to produce `.genome`
self-extractors from the versioned binaries in `wateringHole/genomeBin/primals/`.

### 3. Mixed Linking in wateringHole/genomeBin

The `wateringHole/genomeBin/primals/` directory has both architectures but
mixed linking quality:

| Binary | x86_64 | aarch64 |
|--------|--------|---------|
| toadstool | static-pie (GOOD) | dynamic /lib/ld-linux-aarch64.so.1 |
| squirrel | static-pie (GOOD) | dynamic /lib/ld-linux-aarch64.so.1 |
| biomeos-musl | static-pie (GOOD) | static (GOOD) |
| beardog | dynamic | dynamic /system/bin/linker64 (Android-only) |
| songbird | dynamic | dynamic /lib/ld-linux-aarch64.so.1 |
| nestgate | dynamic | dynamic /lib/ld-linux-aarch64.so.1 |

The aarch64 binaries linked against `/lib/ld-linux-aarch64.so.1` won't run
on Android (needs `/system/bin/linker64`) or any static-only target. Only
`biomeos-aarch64-linux-musl` is properly statically linked.

**Action**: All primal teams should target `aarch64-unknown-linux-musl` for
static linking. See the ecoBin/genomeBin Evolution Guidance document for
the recommended cross-compile workflow.

### 4. No biomeOS Orchestrator on Pixel

The Pixel has a `biomeos/` directory with deploy graphs and primal binaries,
but no biomeOS orchestrator executable. Graph-driven atomic deployment
requires biomeOS to execute the TOML deploy graphs.

**Action (biomeOS team)**: Cross-compile biomeOS for aarch64-linux-musl and
push to Pixel for graph-driven deployment testing.

## Cross-Compile Proof (primalSpring)

primalSpring demonstrates the full cross-compile pipeline:

```bash
# All 21 Rust cross-compile targets are installed on eastGate
rustup target list --installed | grep aarch64
# aarch64-apple-darwin, aarch64-linux-android, aarch64-unknown-linux-gnu,
# aarch64-unknown-linux-musl, ...

# Cross-compile primalSpring for Pixel
cargo build --release --target aarch64-unknown-linux-musl -p primalspring
# 6.87s, 2.99 MB, ELF aarch64, statically linked

# Push and verify
adb push target/aarch64-unknown-linux-musl/release/primalspring_primal /data/local/tmp/
adb shell "/data/local/tmp/primalspring_primal --version"
# primalspring 0.7.0

# Full workspace cross-compile (49 experiments + server + validate_all)
cargo build --release --target aarch64-unknown-linux-musl --workspace
# 1.27s (incremental), all binaries produced
```

Every primal team can follow this pattern. The key requirement is
`aarch64-unknown-linux-musl` for static linking — no glibc, no dynamic
loader, runs on any aarch64 Linux including Android via ADB.

## Rust Cross-Compile Targets Installed on eastGate

```
aarch64-apple-darwin        aarch64-unknown-linux-gnu
aarch64-apple-ios           aarch64-unknown-linux-musl
aarch64-apple-ios-sim       armv7-linux-androideabi
aarch64-linux-android       armv7-unknown-linux-gnueabihf
aarch64-pc-windows-msvc     armv7-unknown-linux-musleabihf
i686-linux-android          riscv64gc-unknown-linux-gnu
i686-unknown-linux-gnu      wasm32-unknown-unknown
x86_64-apple-darwin         x86_64-linux-android
x86_64-apple-ios            x86_64-pc-windows-gnu
x86_64-pc-windows-msvc      x86_64-unknown-linux-gnu
x86_64-unknown-linux-musl
```

---

**License**: AGPL-3.0-or-later
