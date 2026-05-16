# Upstream Pattern Escalation — May 15, 2026

Patterns evolved by downstream products (CATHEDRAL, projectNUCLEUS) that need
upstream adoption, canonicalization, or primal team action. As downstream
evolves towards glacial gates, more delta spring and upstream patterns will
emerge that need refinement.

---

## Tier 1: Primal Blockers (downstream waiting on upstream)

These block concrete downstream deliverables. Tracked as UB-1 through UB-4
in `docs/PRIMAL_GAPS.md`.

### UB-1: Songbird TURN Client Library — SHIPPED

**Owner**: Songbird team
**Status**: SHIPPED (Wave 205, May 15, 2026)

Songbird delivered the `songbird-turn-client` crate (RFC 5766 TURN allocation +
channel-bind + refresh). `primal.announce` wired. lithoSpore's discovery chain
(env → UDS → TURN → standalone) can now wire the TURN leg for geo-delocalized
validation.

**Remaining**: lithoSpore needs to wire `songbird-turn-client` into
`litho_core::discovery::rpc_call()` for TURN-relayed JSON-RPC connections.
This is a lithoSpore integration task, not an upstream blocker.

### UB-2: BearDog FIDO2/CTAP2 Support — SHIPPED

**Owner**: BearDog team
**Status**: SHIPPED (Wave 103, May 15, 2026)

BearDog delivered `fido2.rs` (487 lines) in `beardog-tunnel` providing the
FIDO2/CTAP2 IPC surface. Wave 104 followed with a deep debt sweep aligning
root docs and self-knowledge endpoints.

**Remaining**: lithoSpore needs to wire `beardog.fido2.authenticate` into
the `liveSpore.json` witness signature path. The primal-side surface is live.

### UB-3: genomeBin Tier 3 USB Packaging — RESOLVED

**Owner**: plasmidBin / primalSpring
**Status**: SHIPPED (May 15, 2026)

`plasmidBin/stage_usb.sh` now exports all primal binaries + metadata into a
self-contained USB-ready directory. Supports `--arch`, `--composition`,
`--verify`, `--dry-run`. Output follows canonical genomeBin layout
(`primals/<full-triple>/`) with `manifest.toml`, `checksums.toml`, `ports.env`,
and `VERSION` provenance metadata.

Also fixed: `fetch.sh` `detect_arch()` triple mismatch — now uses full
`x86_64-unknown-linux-musl` triples matching `checksums.toml` keys.

**Remaining**: lithoSpore's `litho assemble` should call `stage_usb.sh` or
adopt the same layout contract for Tier 3 binary resolution.

### UB-4: sporePrint Pipeline Wiring — RESOLVED

**Owner**: primalSpring / sporePrint (not NestGate — downstream ownership)
**Status**: SHIPPED (May 15, 2026)

sporePrint's `auto-refresh.yml` content job now detects `liveSpore.json` at
repo root, copies it to `static/lab/guidestone/liveSpore.json` (served at
`primals.eco/lab/guidestone/liveSpore.json`), and optionally runs `litho verify`
as defense-in-depth. New content page at `content/guidestone/live_spore_feed.md`
documents the feed endpoint and producer setup.

**Remaining**: GuideStone producers (lithoSpore) need to add
`notify-sporeprint.yml` with `content: "true"` and `type: "guidestone"` to
their workflows, and ensure `liveSpore.json` is at repo root.

---

## Tier 2: Patterns to Canonicalize (primalSpring -> wateringHole)

Patterns that downstream evolved independently and should become ecosystem
standards so all springs and gardens adopt them consistently.

### Discovery Chain Standard

lithoSpore evolved a `DiscoveryPath` enum matching primalSpring's 5-tier
`CompositionContext::discover()` but in a simpler consumer-oriented form:

```
env ($CAPABILITY_PORT) -> UDS (discovery.sock) -> TURN ($SONGBIRD_TURN_SERVER) -> Standalone
```

Key elements worth canonicalizing:
- **`probe_operating_mode()`**: detect mode before validation, record in provenance
- **Env var names**: `$PRIMAL_HOST`, `$CAPABILITY_PORT`, `$SONGBIRD_TURN_SERVER`,
  `$SONGBIRD_TURN_DISCOVERY_PORT` — ensure all springs use the same names
- **Graceful degradation**: every leg returns `Option<T>`, never panics, never
  silently fails. Missing primals -> `Skip`, not `Fail`.

**Action**: Publish as a wateringHole standard: "Downstream Discovery Chain
Convention" — env var names, fallback order, provenance recording format.

### Bash-to-Rust Elevation Pattern

lithoSpore replaced all 8 shell scripts with pure Rust CLI subcommands and
documented the specific external command replacements:

| Bash pattern | Rust replacement |
|-------------|-----------------|
| `date` | `chrono` crate |
| `hostname` | `/etc/hostname` read |
| `id` / `whoami` | `/proc/self/status` parse |
| `curl` / `wget` | `ureq` crate (no TLS C deps) |
| `sha256sum` / `blake3sum` | `blake3` crate (`pure` feature, no `cc`) |
| `cp -r` / `rsync` | `std::fs` + `walkdir` |
| `ln -s` | `std::os::unix::fs::symlink` (+ `copy` fallback on Windows) |
| Subshell pipelines | In-process function calls |

primalSpring still has 13 scripts in `scripts/` and 24 files in `tools/`.
Not all need elevation (lab/deployment tooling benefits from shell), but the
pattern for "when and how to elevate" should be documented for the glacial push.

**Action**: Document in wateringHole which script categories benefit from
Rust elevation (validation, assembly, fetch) vs which stay as shell (lab
orchestration, chaos injection, container builds).

### Module lib.rs In-Process Dispatch

lithoSpore refactored 7 separate module binaries into a single `litho` CLI
that calls each module's `run_validation()` directly (no subprocesses). This
mirrors primalSpring's UniBin pattern (absorbed experiments -> `build_registry()`
-> in-process dispatch).

**Action**: Document as the canonical "spore/guidestone composition pattern"
in wateringHole. Any garden building a Targeted GuideStone should follow this
pattern: expose `lib.rs::run_validation()`, compose in a unified CLI, support
argv[0] symlink detection for entry points.

### Cross-Platform Deployment Matrix

lithoSpore validated across a deployment matrix that should become standard
for all ecosystem artifacts targeting glacial USB delivery:

| Platform | Binary | Build target | Test method |
|----------|--------|-------------|-------------|
| Linux x86_64 | musl-static (5.1 MB) | `x86_64-unknown-linux-musl` | Native + Alpine chroot + Ubuntu airgap |
| Linux aarch64 | musl-static | `aarch64-unknown-linux-musl` | Cross-compile + Pi validation |
| Windows x86_64 | litho.exe (7.9 MB) | `x86_64-pc-windows-gnu` | Wine 11 |
| Read-only FS | Same musl-static | Same | Mount with `ro` flag |

Key `#[cfg]` patterns for cross-platform:
- `COMPUTERNAME` env var on Windows vs `/etc/hostname` on Unix
- `%TEMP%` vs `std::env::temp_dir()` (handled by std)
- `std::os::unix::fs::symlink` guarded by `#[cfg(unix)]` with `fs::copy` fallback

**Action**: Publish deployment matrix template in wateringHole for springs
building ecoBin artifacts.

---

## Tier 3: River Delta (Springs) — Adoption Actions

Patterns that springs in the river delta should absorb for glacial convergence.

### All Springs

1. **Adopt `primal.announce`** (biomeOS v3.57) — replace separate 3-call
   registration with single atomic RPC. Use `ctx.announce()` convenience API
   (primalSpring v0.9.26+) for automatic fallback.
2. **Declare signal-tier membership** in announce payload
3. **Validate against 451 methods** — ensure niche counts match
   `config/capability_registry.toml`
4. **Adopt `ctx.dispatch()` for composed workflows** — replace multi-call
   sequences with signal dispatch where an atomic signal exists. See
   `wateringHole/SIGNAL_ADOPTION_STANDARD.md` for migration guide.
5. **Test sovereignty track** — run `primalspring validate --track sovereignty`
   against membrane deployments when applicable
6. **Adopt routing config schema** — any spring doing membrane work must
   conform to `config/routing_config_reference.toml`

### Signal Adoption Expectations for Primals

Primals participating in signal graphs MUST respond to the capabilities
referenced in their graph nodes. The 14 signal graphs in
`graphs/signals/*.toml` define which primals participate in which signals.

`s_signal_dispatch_parity` validates all 14 signals against live biomeOS.
Any `-32601` response for a capability that a graph expects is flagged as
an **UPSTREAM GAP**. Primal teams should:

1. Verify their methods are registered in `config/capability_registry.toml`
2. Verify their socket is discoverable by biomeOS
3. Verify they respond to `primal.announce` (even if just echoing OK)
4. Run `primalspring validate --scenario signal-dispatch-parity` to confirm

### Specific Spring Actions

| Spring | Action | Priority |
|--------|--------|----------|
| neuralSpring | ML surrogates for lithoSpore modules 3, 4, 6 (B3/B4/B6) | MEDIUM (additive) |
| hotSpring | FECS stability pattern -> document as sovereignty warm-handoff standard | LOW |
| wetSpring | B7 genomics -> lithoSpore module 6 maintenance | LOW (already integrated) |
| groundSpring | Statistical method APIs for remaining lithoSpore papers (B6-B9) | MEDIUM |
| healthSpring | B5 (symbiont PK/PD) -> lithoSpore module 8 candidate | LOW |

### Foundation

| Action | Priority |
|--------|----------|
| Run `litho fetch --all` + backfill `blake3` in `data/sources/*.toml` | HIGH |
| Review Thread 1 WCM validation logs — flip `validated` where justified | MEDIUM |
| Document Thread 5 ML sources as `source_type = "internal"` | LOW |
| Wire real spring output for Anderson/enviro workloads (or mark `synthetic = true`) | LOW |

---

## Glacial Gate Horizon

As downstream approaches glacial gates, expect these patterns to surface:

- **Sovereignty telemetry standardization**: membrane_telemetry.sh patterns
  will evolve into a primal-level telemetry reporting standard
- **Multi-gate federation**: idle_compute_federation and friend_remote_covalent
  graph patterns will need upstream bonding model evolution
- **Forgejo Actions CI parity**: as Forgejo becomes primary, CI workflows
  need to be portable between GitHub Actions and Forgejo Actions
- **Content-aware routing evolution**: new backend types and match predicates
  will emerge as the membrane handles more traffic patterns
- **USB creation certification**: primalSpring may need a new certification
  layer (L9?) that validates Targeted GuideStone USB artifacts structurally
