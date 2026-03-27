# plasmidBin Cross-Hardware Dark Forest Deployment Handoff

**Date**: March 27, 2026
**Version**: plasmidBin 3.2.0
**Scope**: Three-node cross-network deployment with BirdSong + Dark Forest

---

## Summary

This handoff documents the evolution of plasmidBin from a static binary repository
into a cross-hardware, cross-network deployment system with Dark Forest beacon
discovery. The system now supports:

- **Multi-architecture** (x86_64 + aarch64 musl-static binaries)
- **ADB deployment** to Pixel/GrapheneOS with abstract socket IPC
- **Bootstrap deployment** for fresh Linux machines (gamer friend pattern)
- **Dark Forest beacons** (two-seed genetic trust: mitobeacon + nuclear DNA)
- **Multi-node mesh validation** with cross-gate beacon exchange verification

---

## What Changed

### New Scripts

| Script | Purpose |
|--------|---------|
| `deploy_pixel.sh` | Deploy aarch64 primals to Pixel via ADB, abstract sockets, TCP listen |
| `bootstrap_gate.sh` | Self-contained gate bootstrap for fresh machines (curl + run) |
| `validate_mesh.sh` | Multi-node mesh validation with BirdSong exchange testing |

### Evolved Scripts

| Script | Changes |
|--------|---------|
| `harvest.sh` | `--arch` flag, `HARVEST_MAP_AARCH64`, `primals/aarch64/` directory, cross-strip |
| `deploy_gate.sh` | `--dark-forest`, `--beacon-seed`, `--mode bootstrap` |
| `validate_gate.sh` | `--birdsong` (beacon probe), `--mesh` (peer visibility) |
| `build_ecosystem_musl.sh` | aarch64 linker config, `--harvest` now routes both arches |

### New Directories

| Path | Purpose |
|------|---------|
| `plasmidBin/primals/aarch64/` | aarch64-linux-musl primal binaries |
| `plasmidBin/springs/aarch64/` | aarch64-linux-musl spring binaries |

### Updated Manifests

- `manifest.toml`: aarch64-linux-musl entries in `[primals.*]` `arch` arrays and `[binaries.*]` sections
- `checksums.toml`: Will be populated when aarch64 binaries are harvested

---

## Three-Node Topology

```
devGate (x86_64, basement)
    ├── beardog :9100  (UDS + TCP)
    ├── songbird :9200 (HTTP + UDS)
    └── toadstool :9400
        │
        │ iPhone hotspot LAN
        ▼
pixelGate (aarch64, GrapheneOS)
    ├── beardog :9100  (abstract @biomeos_beardog + TCP)
    └── songbird :9200 (TCP --listen)
        │
        │ Internet / NAT traversal
        ▼
flockGate (x86_64, Ubuntu 24.04.3, i9-13900K)
    ├── beardog :9100  (UDS + TCP)
    ├── songbird :9200 (HTTP + UDS)
    └── toadstool :9400
```

Deploy graph: `primalSpring/graphs/multi_node/three_node_covalent_cross_network.toml`

---

## Trust Model: Two-Seed Genetics

Per `DARK_FOREST_BEACON_GENETICS_STANDARD.md`:

| Seed | Analog | Shared? | Controls |
|------|--------|---------|----------|
| **Beacon Seed** (mitobeacon) | Mitochondrial DNA | Yes | Discovery — who can *find* family |
| **Lineage Seed** (nuclear DNA) | Nuclear DNA | No (unique per device) | Authorization — what a node can *do* |

**Backward compatibility**: `BEARDOG_FAMILY_SEED` alone derives both seeds.
Two-seed mode (`BEARDOG_BEACON_SEED` + `BEARDOG_LINEAGE_SEED`) is the evolution target.

**Key principle**: Grandma can hear cousin (shared mitobeacon), but can't access
cousin's bank (different lineage permissions).

---

## Deployment Procedures

### devGate (local, already running)

```bash
# Already validated in previous session
./validate_gate.sh localhost --birdsong
```

### pixelGate (ADB over USB)

```bash
# Build aarch64 binaries
./scripts/build_ecosystem_musl.sh --aarch64
./plasmidBin/harvest.sh --arch aarch64

# Deploy to Pixel
./plasmidBin/deploy_pixel.sh --dark-forest --beacon-seed ~/.beacon.seed

# Validate via ADB port forwarding
./plasmidBin/validate_gate.sh localhost --birdsong

# Validate via hotspot LAN (find Pixel IP first)
adb shell 'ip addr show wlan0 | grep inet'
./plasmidBin/validate_gate.sh <pixel-ip> --birdsong
```

### flockGate (bootstrap via RustDesk)

```bash
# Generate bootstrap command
./plasmidBin/deploy_gate.sh --mode bootstrap --family-id <id> --dark-forest

# Remote user pastes:
curl -sL https://raw.githubusercontent.com/ecoPrimals/plasmidBin/main/bootstrap_gate.sh | \
    bash -s -- --family-id <id> --dark-forest

# Remote user opens firewall:
sudo ufw allow 9100/tcp
sudo ufw allow 9200/tcp

# Validate from dev machine:
./plasmidBin/validate_gate.sh <flock-public-ip> --birdsong
```

### Full mesh validation

```bash
./plasmidBin/validate_mesh.sh \
    --gates "devGate=localhost,pixelGate=<pixel-ip>,flockGate=<flock-ip>" \
    --birdsong-exchange
```

---

## BirdSong Dark Forest Flow

1. All nodes share `.beacon.seed` (mitobeacon), distributed out-of-band
2. BearDog loads beacon seed, derives ChaCha20-Poly1305 encryption keys
3. Songbird broadcasts Dark Forest beacons (fully encrypted — observers see noise)
4. Only nodes with matching beacon genetics can decrypt and discover peers
5. After discovery, lineage verification (nuclear DNA challenge-response) determines permissions
6. Bonding policy enforces capability scope (e.g., flockGate gets `compute.*` only)

Environment variables controlling Dark Forest:

| Variable | Effect |
|----------|--------|
| `SONGBIRD_DARK_FOREST=true` | Fully encrypted beacons (no plaintext family_id) |
| `SONGBIRD_AUTO_DISCOVERY=true` | Peer discovery enabled |
| `SONGBIRD_SECURITY_PROVIDER=beardog` | Delegate crypto to BearDog |
| `BEARDOG_SOCKET=<path>` | BearDog IPC for birdsong.encrypt/decrypt |
| `BEARDOG_FAMILY_SEED=<path>` | Backward-compat single seed (derives both) |

---

## Pixel/GrapheneOS Considerations

- **Abstract sockets**: BearDog uses `--abstract` flag for SELinux-safe `@biomeos_beardog` IPC
- **TCP for IPC**: Songbird uses `--listen 0.0.0.0:9200` since filesystem UDS is SELinux-restricted
- **ADB port forwarding**: `deploy_pixel.sh` sets up `adb forward tcp:PORT tcp:PORT` automatically
- **Binary path**: `/data/local/tmp/plasmidBin/primals/` (standard ADB push destination)
- **Cross-strip**: `aarch64-linux-gnu-strip` used for stripping aarch64 binaries on x86_64 host

---

## Absorbable Patterns for Primal Teams

### BearDog

- Two-seed mode (`BEARDOG_BEACON_SEED` + `BEARDOG_LINEAGE_SEED`) should be the primary API
- `BEARDOG_FAMILY_SEED` backward compat can remain but deprecation path should be documented
- `--abstract` flag works; consider auto-detecting Android and defaulting to abstract
- `beacon.*` RPC methods (generate, encrypt, decrypt, try_decrypt_any) are the foundation

### Songbird

- `SONGBIRD_DARK_FOREST` controls beacon format; default should evolve toward encrypted
- `mesh.auto_discover` works on LAN (multicast); WAN needs explicit `mesh.connect`
- `birdsong.generate_encrypted_beacon` and `birdsong.decrypt_beacon` are the key APIs
- Consider adding `--dark-forest` CLI flag (currently env-only)
- `mesh.peers` should include node arch/platform metadata for heterogeneous meshes

### biomeOS

- `deploy_pixel.sh` and `bootstrap_gate.sh` should evolve into `biomeos deploy --target pixel`
- The deploy graph format (`three_node_covalent_cross_network.toml`) should be executable by biomeOS
- Bootstrap pattern (curl + run) could become a biomeOS LiveSpore USB variant

---

## Open Items

| Item | Priority | Status | Notes |
|------|----------|--------|-------|
| Primal CLI standardization | P0 | Blocked on primal teams | See CLI audit table above |
| Fix NestGate `--help` segfault | P0 | Blocked on nestgate team | Exit 139 on `--help` and `daemon --help` |
| Build + harvest aarch64 binaries | High | Ready | Toolchain ready, `build_ecosystem_musl.sh --aarch64` |
| Test ADB deploy to real Pixel | High | Ready | `deploy_pixel.sh` + `start_primal.sh` created |
| Test bootstrap on flockGate | High | Ready | `bootstrap_gate.sh` + `seed_workflow.sh` created |
| BearDog seed lifecycle CLI | High | Workaround | `seed_workflow.sh` bridges gap; BearDog needs native support |
| Songbird `--dark-forest` flag | Medium | Env workaround | Currently env-only, silent fallback |
| BirdSong beacon exchange across gates | Medium | Untested | `validate_mesh.sh --birdsong-exchange` tests this |
| NAT traversal (STUN+punch) testing | Medium | Untested | For flockGate without port forwarding |
| GitHub Releases for aarch64 | Medium | Ready | `harvest.sh --release --arch aarch64` |
| LiveSpore USB with aarch64 | Low | Framework only | `phase2/biomeOS/livespore-usb/aarch64/` |

---

## Cross-Deployment Retrospective: CLI Audit

### Primal CLI Inconsistency Matrix

This is the #1 friction point discovered during cross-deployment. Each primal
uses different flags for the same concept, forcing deploy scripts to maintain
per-primal `case` blocks:

| Concept | BearDog | Songbird | Squirrel | ToadStool | NestGate |
|---------|---------|----------|----------|-----------|----------|
| **TCP bind** | `--listen addr:port` | `--port PORT` | `--port P --bind ADDR` | `--port PORT` | SEGFAULTS |
| **Unix socket** | `--socket PATH` | `--socket PATH` | `-s/--socket PATH` | `--socket PATH` | env only |
| **Family ID** | `--family-id ID` | env `SONGBIRD_FAMILY_ID` | env only | `--family-id ID` | env only |
| **Abstract socket** | `--abstract` | N/A | N/A | N/A | N/A |
| **Dark Forest** | N/A | env only (`SONGBIRD_DARK_FOREST`) | N/A | N/A | N/A |
| **Server mode** | `server` | `server` | `server` | `server` / `capabilities` | `daemon` |
| **Health check protocol** | raw TCP JSON-RPC | HTTP `/health` | HTTP JSON-RPC POST | HTTP `/health` | raw TCP JSON-RPC |

**Impact**: Deploy scripts (`deploy_gate.sh`, `deploy_pixel.sh`, `bootstrap_gate.sh`)
each maintain 60+ line case blocks that will break when any primal changes its CLI.

### Recommended Primal CLI Standard

Every primal server mode should accept:

```
--listen <addr:port>     TCP binding (unified meaning)
--socket <path>          Unix domain socket
--family-id <id>         Family identity
--abstract               Abstract socket (Android/SELinux)
--dark-forest            Enable Dark Forest mode
```

BearDog already has the best CLI. Other primals should converge toward it.

### Primal-Specific Action Items

| Primal | Action | Priority |
|--------|--------|----------|
| **NestGate** | Fix `--help` segfault (exit 139). CLI parser is broken. | P0 |
| **BearDog** | Add seed lifecycle CLI: `beardog seed generate`, `beardog seed export --base64`, `beardog seed verify` | P1 |
| **Songbird** | Add `--dark-forest` CLI flag. Validate beardog-socket if dark-forest is set. | P1 |
| **Songbird** | Standardize: `--listen addr:port` instead of `--port PORT` | P2 |
| **Squirrel** | Standardize: `--listen addr:port` instead of `--port + --bind` | P2 |
| **ToadStool** | Standardize: `--listen addr:port` instead of `--port only` | P2 |
| **All** | Health endpoint: report protocol (raw TCP vs HTTP) in `capabilities.list` response | P2 |
| **All** | Accept `--family-id` as CLI flag (not env-only) | P2 |

---

## Friction Map

### High Friction (Blocks Deployment)

| Issue | Impact | Resolution |
|-------|--------|------------|
| Primal CLI inconsistency | Per-primal case blocks in 3+ deploy scripts | Primals standardize on common flags |
| NestGate `--help` segfaults | Can't verify NestGate CLI | Fix NestGate CLI parser |
| No seed generation CLI | Can't programmatically create trust seeds | `seed_workflow.sh` bridges the gap; BearDog needs `beardog seed generate/export` |

### Medium Friction (Workarounds Exist)

| Issue | Impact | Workaround |
|-------|--------|------------|
| Dark Forest env-only | 4-6 env vars per primal, silent fallback to plaintext | `start_primal.sh` sets them; primals need `--dark-forest` flag |
| 3 health protocols | `validate_gate.sh` brute-forces 3 protocols | Multi-protocol prober works; primals should report protocol |
| No aarch64 binaries built yet | Pixel deployment untested | Toolchain ready, `build_ecosystem_musl.sh --aarch64` needed |

### Low Friction (Design Decisions Needed)

| Issue | Notes |
|-------|-------|
| NAT traversal untested | flockGate behind NAT, needs STUN/punch or port forwarding |
| Two-seed mode not in CLIs | Primals accept env vars but no `--beacon-seed`/`--lineage-seed` |
| biomeOS doesn't consume deploy graphs | Deploy graph TOML format is expressive but manual |

---

## New Tools Created (Retrospective Phase)

### `start_primal.sh` — Unified Primal Startup Wrapper

Absorbs all per-primal CLI differences. Maps generic flags (`--tcp-port`, `--socket`,
`--family-id`, `--dark-forest`, `--abstract`) to primal-specific flags. Single source
of truth for the CLI audit map.

```bash
./start_primal.sh beardog --tcp-port 9100 --socket /tmp/beardog.sock --family-id abc123
./start_primal.sh songbird --tcp-port 9200 --dark-forest --beardog-socket /tmp/beardog.sock
./start_primal.sh toadstool --capabilities-only
```

When primals standardize their CLIs, the per-primal case blocks in this script shrink
to a single generic case — and this script becomes trivially thin.

### `seed_workflow.sh` — Dark Forest Seed Lifecycle

Manages the two-seed trust model using BearDog's crypto primitives (entropy, key
generate, derive, export). Full lifecycle:

```bash
./seed_workflow.sh init --family-name "eastgate-family"          # Create root + beacon
./seed_workflow.sh add-node --node-id devgate                     # Lineage for dev
./seed_workflow.sh add-node --node-id pixel                       # Lineage for Pixel
./seed_workflow.sh add-node --node-id flockgate                   # Lineage for flockGate
./seed_workflow.sh export --format base64                         # For RustDesk paste
./seed_workflow.sh distribute --node-id flockgate                 # Deploy instructions
./seed_workflow.sh verify                                         # Integrity check
```

Seeds directory layout:
```
~/.config/biomeos/family/
├── family.key          (PROTECT — never distribute)
├── .beacon.seed        (mitobeacon — share with family)
├── family_id           (8-char identifier)
├── nodes/
│   ├── devgate.lineage.seed
│   ├── pixel.lineage.seed
│   └── flockgate.lineage.seed
└── exports/
    ├── beacon.b64
    └── flockgate.lineage.b64
```

### `fetch.sh` — Fixed aarch64 Layout

Downloads now correctly route to `primals/aarch64/` on aarch64 machines, matching
`harvest.sh`'s directory layout. x86_64 remains flat (backward compatible).

---

## What Worked Well

- **musl static binaries** are genuinely portable — one binary, any Linux box, zero deps
- **build_ecosystem_musl.sh** pipeline was already aarch64-ready (flags, targets, linker)
- **GitHub Releases** (v2026.03.25) work for `fetch.sh --all`
- **DARK_FOREST_BEACON_GENETICS_STANDARD** two-seed model is architecturally sound
- **Deploy graph TOML format** is expressive and human-readable

## What Was Harder Than Expected

- **Primal CLI inconsistency** is the dominant friction (see audit table above)
- **`beardog --listen` vs `songbird --port`** naming collision caused first live bug
- **NestGate segfaults** on `--help` — can't even discover its interface
- **No seed generation workflow** — the entire trust model depended on undocumented manual process
- **Dark Forest is 4-6 env vars** with silent fallback to plaintext if misconfigured

---

## Files Modified/Created

**New files (retrospective phase):**
- `plasmidBin/start_primal.sh` — unified primal startup wrapper
- `plasmidBin/seed_workflow.sh` — Dark Forest seed lifecycle management

**New files (deployment phase):**
- `plasmidBin/deploy_pixel.sh`
- `plasmidBin/bootstrap_gate.sh`
- `plasmidBin/validate_mesh.sh`
- `plasmidBin/primals/aarch64/.gitkeep`
- `plasmidBin/springs/aarch64/.gitkeep`
- `primalSpring/graphs/multi_node/three_node_covalent_cross_network.toml`

**Modified files:**
- `plasmidBin/fetch.sh` — aarch64 layout fix (route to `primals/aarch64/`)
- `plasmidBin/harvest.sh` — multi-arch support
- `plasmidBin/deploy_gate.sh` — Dark Forest, beacon seed, bootstrap mode
- `plasmidBin/validate_gate.sh` — BirdSong + mesh probes
- `plasmidBin/manifest.toml` — aarch64 entries
- `plasmidBin/README.md` — comprehensive update
- `primalSpring/scripts/build_ecosystem_musl.sh` — aarch64 linker config, harvest routing
