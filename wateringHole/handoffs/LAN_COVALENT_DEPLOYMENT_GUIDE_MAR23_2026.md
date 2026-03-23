# LAN Covalent Deployment Guide — Phase 14

**Date**: March 23, 2026
**From**: primalSpring v0.7.0
**To**: All gate operators, biomeOS team
**Scope**: Deploy NUCLEUS on LAN gates, validate Songbird mesh + BirdSong beacon exchange

---

## Prerequisites

### Hardware

From the basement HPC inventory (HARDWARE.md):

| Gate | Role | Ready |
|------|------|-------|
| eastGate | Dev tower (i9-12900, RTX 4070, Akida) | YES — development host |
| northGate | Flagship (i9-14900K, RTX 5090, 192GB) | YES — 10G NIC installed |
| strandGate | Bio (dual EPYC 64c, 256GB ECC) | YES — 10G NIC installed |
| westGate | Cold storage (76TB ZFS) | YES — 10G NIC installed |
| southGate | Gaming + compute (5800X3D) | YES — 10G NIC installed |
| biomeGate | HBM2 bench (TR 3970X, Titan V x2) | YES |
| flockGate | Brother's node (i9-13900K) | FUTURE — remote covalent |
| Pixel 8a | Mobile (aarch64) | BLOCKED — BearDog abstract socket |

### Software

On each gate:
- Rust toolchain with `x86_64-unknown-linux-musl` target
- `musl-tools` package (`apt install musl-tools`)
- Network connectivity (1G or 10G ethernet)

### Genetics

All gates must share the same `.family.seed` for covalent bonding.
The canonical seed is on ColdSpore USB (BEA6-BBCE) at `/biomeOS/.family.seed`.
Family ID: `8ff3b864a4bc589a`.

---

## Step 1: Build Static Binaries

On eastGate (or any gate with all source repos):

```bash
cd /home/eastgate/Development/ecoPrimals/primalSpring
./scripts/build_ecosystem_musl.sh
```

This builds all 6 core primals as `x86_64-unknown-linux-musl` static PIE binaries
into `/tmp/primalspring-deploy/primals/x86_64/`.

For aarch64 (Pixel): `./scripts/build_ecosystem_musl.sh --aarch64`

## Step 2: Prepare USB Spore

```bash
./scripts/prepare_spore_payload.sh /media/$USER/biomeOS1 --arch x86_64
```

This copies binaries, deploy graphs, launch profiles, and deployment scripts
to the USB mount point. For a bootable LiveSpore, use biomeOS's script:

```bash
cd /home/eastgate/Development/ecoPrimals/phase2/biomeOS
sudo ./scripts/create_livespore.sh /dev/sdX livespore-2026
```

## Step 3: Deploy to Remote Gate

### Option A: USB transfer

1. Plug USB into remote gate
2. Mount: `mount /dev/sda1 /mnt/spore`
3. Copy: `cp -r /mnt/spore/primals /tmp/biomeos-deploy/`
4. Copy genetics: `cp /mnt/spore/.family.seed /tmp/biomeos-deploy/`

### Option B: LAN transfer (from eastGate)

Using biomeOS's deploy script:

```bash
cd /home/eastgate/Development/ecoPrimals/phase2/biomeOS/livespore-usb/x86_64/scripts
./deploy_to_gate.sh <gate-ip> <node-id>
```

Or manual rsync:

```bash
rsync -avz /tmp/primalspring-deploy/ user@northgate:/tmp/biomeos-deploy/
```

## Step 4: Start NUCLEUS on Each Gate

On each gate, start Tower Atomic first:

```bash
export FAMILY_ID=8ff3b864a4bc589a
export NODE_ID=northgate  # unique per gate
export XDG_RUNTIME_DIR=/run/user/$(id -u)

# Start BearDog (security)
/tmp/biomeos-deploy/primals/x86_64/beardog-x86_64-linux-musl server &
sleep 2

# Start Songbird (discovery + mesh)
BEARDOG_SOCKET="$XDG_RUNTIME_DIR/biomeos/beardog-$FAMILY_ID.sock" \
SONGBIRD_MESH_ENABLED=true \
/tmp/biomeos-deploy/primals/x86_64/songbird-x86_64-linux-musl server &
sleep 2

# Verify Tower is live
echo '{"jsonrpc":"2.0","method":"health.liveness","params":{},"id":1}' | \
  socat - UNIX-CONNECT:$XDG_RUNTIME_DIR/biomeos/beardog-$FAMILY_ID.sock
```

Then expand to full NUCLEUS:

```bash
# ToadStool (compute)
TOADSTOOL_FAMILY_ID=$FAMILY_ID \
TOADSTOOL_NODE_ID=$NODE_ID \
TOADSTOOL_SECURITY_WARNING_ACKNOWLEDGED=1 \
/tmp/biomeos-deploy/primals/x86_64/toadstool-x86_64-linux-musl --port 0 &

# NestGate (storage)
NESTGATE_SOCKET="$XDG_RUNTIME_DIR/biomeos/nestgate-$FAMILY_ID.sock" \
NESTGATE_FAMILY_ID=$FAMILY_ID \
/tmp/biomeos-deploy/primals/x86_64/nestgate-x86_64-linux-musl daemon --socket-only --dev &

# Squirrel (AI)
SQUIRREL_SOCKET="$XDG_RUNTIME_DIR/biomeos/squirrel-$FAMILY_ID.sock" \
/tmp/biomeos-deploy/primals/x86_64/squirrel-x86_64-linux-musl &
```

Or use biomeOS's deploy graph orchestrator:

```bash
biomeos atomic deploy graphs/multi_node/basement_hpc_covalent.toml \
  --env FAMILY_ID=8ff3b864a4bc589a \
  --env NODE_ID=northgate \
  --env XDG_RUNTIME_DIR=/run/user/$(id -u)
```

## Step 5: Validate

### From any gate, probe a remote gate:

```bash
./scripts/validate_remote_gate.sh <gate-ip>
```

### Cross-gate mesh validation (primalSpring exp073):

```bash
REMOTE_GATE_HOST=<gate-ip> \
REMOTE_SONGBIRD_PORT=9200 \
REMOTE_BEARDOG_PORT=9100 \
FAMILY_ID=8ff3b864a4bc589a \
cargo run --release --bin exp073_lan_covalent_mesh
```

### Full NUCLEUS health (primalSpring exp074):

```bash
REMOTE_GATE_HOST=<gate-ip> cargo run --release --bin exp074_cross_gate_health
```

## Step 6: BirdSong Beacon Exchange

Once two gates have Tower Atomic running with `SONGBIRD_MESH_ENABLED=true`:

1. Songbird should auto-discover peers via UDP multicast (239.255.0.1:4200)
2. `mesh.peers` should return at least 1 peer
3. BirdSong beacons can be generated and exchanged:

```bash
# On gate A: generate beacon
echo '{"jsonrpc":"2.0","method":"birdsong.generate_encrypted_beacon","params":{"family_id":"8ff3b864a4bc589a","node_id":"northgate","capabilities":["security","discovery","compute"],"device_type":"tower"},"id":1}' | \
  socat - UNIX-CONNECT:$XDG_RUNTIME_DIR/biomeos/songbird-$FAMILY_ID.sock

# Copy the encrypted_beacon value to gate B for decryption
# On gate B: decrypt
echo '{"jsonrpc":"2.0","method":"birdsong.decrypt_beacon","params":{"encrypted_beacon":"<paste>"},"id":1}' | \
  socat - UNIX-CONNECT:$XDG_RUNTIME_DIR/biomeos/songbird-$FAMILY_ID.sock
```

The security model: even on a "trusted" LAN, BirdSong beacons are encrypted.
Family/work networks may have different trust levels, and public LANs exist.
Treating all LANs as potentially hostile is the correct security posture.

---

## Pixel Deployment (Phase 14.1)

### Current Blockers

1. **BearDog abstract socket**: v0.9.0 can't bind filesystem sockets on Android
   (SELinux). Needs `BEARDOG_ABSTRACT_SOCKET` or TCP fallback restored.
2. **No biomeOS orchestrator for aarch64**: graph-driven deployment requires biomeOS.

### Workaround Path

Deploy Songbird-only on Pixel (Songbird can bind TCP directly):

```bash
# Build aarch64 binaries
./scripts/build_ecosystem_musl.sh --aarch64

# Push to Pixel
adb push /tmp/primalspring-deploy/primals/aarch64/songbird-aarch64-linux-musl /data/local/tmp/songbird
adb shell chmod +x /data/local/tmp/songbird

# Start on Pixel (TCP mode, no Unix socket needed)
adb shell "FAMILY_ID=8ff3b864a4bc589a /data/local/tmp/songbird server --port 9200"
```

### Cross-Device Validation

```bash
PIXEL_SONGBIRD_HOST=<pixel-ip> \
PIXEL_SONGBIRD_PORT=9200 \
cargo run --release --bin exp063_pixel_tower_rendezvous
```

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Socket not found | Check `$XDG_RUNTIME_DIR/biomeos/` — sockets are `{primal}-{family_id}.sock` |
| Mesh no peers | Ensure `SONGBIRD_MESH_ENABLED=true` and UDP 4200 not firewalled |
| Connection refused | Check primal is running: `ps aux \| grep beardog` |
| Permission denied | Socket perms: `ls -la $XDG_RUNTIME_DIR/biomeos/` |
| STUN fails | STUN requires outbound UDP to public servers — check NAT/firewall |

---

## Gate Name → NODE_ID Mapping

From HARDWARE.md:

| Gate | NODE_ID | IP (assign) |
|------|---------|-------------|
| eastGate | `eastgate` | (dev host) |
| northGate | `northgate` | TBD |
| southGate | `southgate` | TBD |
| strandGate | `strandgate` | TBD |
| biomeGate | `biomegate` | TBD |
| westGate | `westgate` | TBD |
| swiftGate | `swiftgate` | TBD |
| kinGate | `kingate` | TBD |
| flockGate | `flockgate` | Brother's house (remote) |

Fill in IPs once 10G cables are connected or use hostname resolution.

---

**License**: AGPL-3.0-or-later
