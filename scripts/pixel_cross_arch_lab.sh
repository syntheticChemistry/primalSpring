#!/usr/bin/env bash
# pixel_cross_arch_lab.sh — Cross-architecture bonding lab with Pixel/GrapheneOS
#
# Detects a connected Pixel via ADB, sets up port forwarding for NUCLEUS
# TCP services, and runs the cross-architecture validation suite.
#
# Architecture:
#   Eastgate (x86_64) ←── ADB USB / LAN TCP ──→ Pixel (aarch64, GrapheneOS, Titan M2)
#
# The Pixel runs NUCLEUS primals (BearDog + Songbird at minimum) built for
# aarch64-unknown-linux-musl. This script handles:
#   1. ADB device detection and connection verification
#   2. TCP port forwarding (ADB forward or direct LAN)
#   3. Running exp096 (cross-arch bonding) and exp076 (neural routing)
#   4. Optional: deploying aarch64 binaries to Pixel via ADB push
#
# Usage:
#   ./scripts/pixel_cross_arch_lab.sh                    # ADB USB (auto-detect)
#   ./scripts/pixel_cross_arch_lab.sh --lan 192.168.1.50 # Direct LAN (no ADB)
#   ./scripts/pixel_cross_arch_lab.sh --deploy           # Deploy aarch64 binaries first
#   ./scripts/pixel_cross_arch_lab.sh --help
#
# Prerequisites:
#   - ADB installed and Pixel connected via USB (or --lan with Pixel IP)
#   - NUCLEUS primals running on the Pixel (BearDog + Songbird with --port)
#   - For --deploy: aarch64 binaries in /tmp/primalspring-deploy/primals/aarch64/
#     (run: ./scripts/build_ecosystem_musl.sh --aarch64)
#
# Port mapping (ADB forward):
#   localhost:19100 → Pixel:9100 (BearDog)
#   localhost:19200 → Pixel:9200 (Songbird)
#   localhost:19300 → Pixel:9300 (NestGate)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

MODE="adb"
PIXEL_LAN_IP=""
DO_DEPLOY=false
PIXEL_DEPLOY_DIR="/data/local/tmp/nucleus"

# Pixel-side ports (what the primals listen on inside the Pixel)
PIXEL_INTERNAL_BEARDOG=9100
PIXEL_INTERNAL_SONGBIRD=9200
PIXEL_INTERNAL_NESTGATE=9300

# Host-side forwarded ports (what exp096/exp076 connect to)
HOST_BEARDOG=19100
HOST_SONGBIRD=19200
HOST_NESTGATE=19300

FAMILY_ID="${FAMILY_ID:-pixel-cross-arch-lab}"

for arg in "$@"; do
    case "$arg" in
        --lan)
            MODE="lan"
            ;;
        --deploy)
            DO_DEPLOY=true
            ;;
        --help|-h)
            head -35 "$0" | grep '^#' | sed 's/^# \?//'
            exit 0
            ;;
        *)
            if [ "$MODE" = "lan" ] && [ -z "$PIXEL_LAN_IP" ]; then
                PIXEL_LAN_IP="$arg"
            fi
            ;;
    esac
done

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Pixel Cross-Architecture Bonding Lab                       ║"
echo "║  x86_64 (Eastgate) ↔ aarch64 (Pixel/GrapheneOS + Titan M2) ║"
echo "╠══════════════════════════════════════════════════════════════╣"
echo "║  Mode:      $MODE"
echo "║  Family ID: $FAMILY_ID"
if [ "$MODE" = "lan" ]; then
echo "║  Pixel IP:  $PIXEL_LAN_IP"
fi
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# ── ADB Detection ─────────────────────────────────────────────────────────

detect_adb_pixel() {
    if ! command -v adb &>/dev/null; then
        echo "ERROR: adb not found. Install Android platform-tools."
        echo "  sudo apt install android-tools-adb"
        exit 1
    fi

    echo "══ ADB Device Detection ══"
    local devices
    devices=$(adb devices 2>/dev/null | grep -v "List of devices" | grep -v "^$" || true)

    if [ -z "$devices" ]; then
        echo "  No ADB devices found."
        echo ""
        echo "  Ensure:"
        echo "    1. Pixel is connected via USB"
        echo "    2. USB debugging is enabled (Settings → System → Developer options)"
        echo "    3. GrapheneOS authorized this computer for ADB"
        exit 1
    fi

    local device_id
    device_id=$(echo "$devices" | head -1 | awk '{print $1}')
    local device_state
    device_state=$(echo "$devices" | head -1 | awk '{print $2}')

    echo "  Device:  $device_id"
    echo "  State:   $device_state"

    if [ "$device_state" != "device" ]; then
        echo "  ERROR: Device not ready (state: $device_state)"
        echo "  Check USB connection and authorization."
        exit 1
    fi

    local device_model
    device_model=$(adb -s "$device_id" shell getprop ro.product.model 2>/dev/null || echo "unknown")
    local device_arch
    device_arch=$(adb -s "$device_id" shell getprop ro.product.cpu.abi 2>/dev/null || echo "unknown")
    local device_os
    device_os=$(adb -s "$device_id" shell getprop ro.build.display.id 2>/dev/null || echo "unknown")

    echo "  Model:   $device_model"
    echo "  Arch:    $device_arch"
    echo "  OS:      $device_os"
    echo ""

    PIXEL_DEVICE_ID="$device_id"
}

# ── ADB Port Forwarding ──────────────────────────────────────────────────

setup_adb_forwarding() {
    echo "══ ADB Port Forwarding ══"

    adb -s "$PIXEL_DEVICE_ID" forward "tcp:$HOST_BEARDOG" "tcp:$PIXEL_INTERNAL_BEARDOG" 2>/dev/null || true
    echo "  localhost:$HOST_BEARDOG → Pixel:$PIXEL_INTERNAL_BEARDOG (BearDog)"

    adb -s "$PIXEL_DEVICE_ID" forward "tcp:$HOST_SONGBIRD" "tcp:$PIXEL_INTERNAL_SONGBIRD" 2>/dev/null || true
    echo "  localhost:$HOST_SONGBIRD → Pixel:$PIXEL_INTERNAL_SONGBIRD (Songbird)"

    adb -s "$PIXEL_DEVICE_ID" forward "tcp:$HOST_NESTGATE" "tcp:$PIXEL_INTERNAL_NESTGATE" 2>/dev/null || true
    echo "  localhost:$HOST_NESTGATE → Pixel:$PIXEL_INTERNAL_NESTGATE (NestGate)"

    echo ""
}

cleanup_adb_forwarding() {
    if [ -n "${PIXEL_DEVICE_ID:-}" ]; then
        echo ""
        echo "══ Cleaning up ADB port forwards ══"
        adb -s "$PIXEL_DEVICE_ID" forward --remove "tcp:$HOST_BEARDOG" 2>/dev/null || true
        adb -s "$PIXEL_DEVICE_ID" forward --remove "tcp:$HOST_SONGBIRD" 2>/dev/null || true
        adb -s "$PIXEL_DEVICE_ID" forward --remove "tcp:$HOST_NESTGATE" 2>/dev/null || true
        echo "  Forwards removed."
    fi
}

# ── Binary Deployment ─────────────────────────────────────────────────────

deploy_aarch64_binaries() {
    echo "══ Deploying aarch64 Binaries to Pixel ══"

    local staging="/tmp/primalspring-deploy/primals/aarch64"
    if [ ! -d "$staging" ]; then
        echo "  ERROR: No aarch64 binaries found at $staging"
        echo "  Run: ./scripts/build_ecosystem_musl.sh --aarch64"
        exit 1
    fi

    local bin_count
    bin_count=$(find "$staging" -type f -executable 2>/dev/null | wc -l)
    echo "  Source: $staging ($bin_count binaries)"
    echo "  Target: $PIXEL_DEPLOY_DIR"

    adb -s "$PIXEL_DEVICE_ID" shell "mkdir -p $PIXEL_DEPLOY_DIR" 2>/dev/null || true

    for bin in "$staging"/*; do
        [ -f "$bin" ] && [ -x "$bin" ] || continue
        local name
        name=$(basename "$bin")
        echo "  pushing $name ..."
        adb -s "$PIXEL_DEVICE_ID" push "$bin" "$PIXEL_DEPLOY_DIR/$name" 2>/dev/null
        adb -s "$PIXEL_DEVICE_ID" shell "chmod +x $PIXEL_DEPLOY_DIR/$name" 2>/dev/null
    done

    echo "  Deployed $bin_count binaries to Pixel."
    echo ""
}

# ── Quick Connectivity Check ──────────────────────────────────────────────

check_pixel_connectivity() {
    local host="$1"
    local port="$2"
    local label="$3"

    if timeout 3 bash -c "echo >/dev/tcp/$host/$port" 2>/dev/null; then
        printf "  %-12s  LIVE  (%s:%s)\n" "$label" "$host" "$port"
        return 0
    else
        printf "  %-12s  DOWN  (%s:%s)\n" "$label" "$host" "$port"
        return 1
    fi
}

# ── Run Experiments ───────────────────────────────────────────────────────

run_experiments() {
    local pixel_host="$1"
    local bd_port="$2"
    local sb_port="$3"
    local ng_port="$4"

    echo "══ Pre-flight: Pixel Connectivity ══"
    local reachable=0
    check_pixel_connectivity "$pixel_host" "$bd_port" "BearDog" && ((reachable++)) || true
    check_pixel_connectivity "$pixel_host" "$sb_port" "Songbird" && ((reachable++)) || true
    check_pixel_connectivity "$pixel_host" "$ng_port" "NestGate" || true
    echo ""

    if [ "$reachable" -eq 0 ]; then
        echo "ERROR: No Pixel primals reachable."
        echo ""
        echo "Ensure NUCLEUS is running on the Pixel:"
        echo "  beardog server --port $PIXEL_INTERNAL_BEARDOG &"
        echo "  songbird server --port $PIXEL_INTERNAL_SONGBIRD &"
        exit 1
    fi

    cd "$PROJECT_DIR"

    # Phase 1: Cross-arch bonding (exp096)
    echo "══ Phase 1: Cross-Architecture Bonding (exp096) ══"
    export PIXEL_HOST="${pixel_host/localhost/127.0.0.1}"
    export PIXEL_BEARDOG_PORT="$bd_port"
    export PIXEL_SONGBIRD_PORT="$sb_port"
    export PIXEL_NESTGATE_PORT="$ng_port"
    export FAMILY_ID

    if cargo run --bin exp096_pixel_cross_arch_bonding 2>&1; then
        echo "  exp096: COMPLETE"
    else
        echo "  exp096: COMPLETED WITH ERRORS (some checks may have failed)"
    fi
    echo ""

    # Phase 2: Neural routing (exp076)
    echo "══ Phase 2: Cross-Gate Neural Routing (exp076) ══"
    export PIXEL_BEARDOG_TCP="${pixel_host}:${bd_port}"
    export PIXEL_SONGBIRD_PORT="$sb_port"

    if cargo run --bin exp076_cross_gate_neural_routing 2>&1; then
        echo "  exp076: COMPLETE"
    else
        echo "  exp076: COMPLETED WITH ERRORS"
    fi
    echo ""
}

# ── Main Flow ─────────────────────────────────────────────────────────────

PIXEL_DEVICE_ID=""

if [ "$MODE" = "adb" ]; then
    detect_adb_pixel

    if $DO_DEPLOY; then
        deploy_aarch64_binaries
    fi

    setup_adb_forwarding
    trap cleanup_adb_forwarding EXIT

    run_experiments "localhost" "$HOST_BEARDOG" "$HOST_SONGBIRD" "$HOST_NESTGATE"

elif [ "$MODE" = "lan" ]; then
    if [ -z "$PIXEL_LAN_IP" ]; then
        echo "ERROR: --lan requires a Pixel IP address."
        echo "  Usage: $0 --lan 192.168.1.50"
        exit 1
    fi

    echo "══ LAN Mode: Direct TCP to $PIXEL_LAN_IP ══"
    echo ""

    run_experiments "$PIXEL_LAN_IP" "$PIXEL_INTERNAL_BEARDOG" "$PIXEL_INTERNAL_SONGBIRD" "$PIXEL_INTERNAL_NESTGATE"
fi

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Pixel Cross-Architecture Lab — Complete                    ║"
echo "╚══════════════════════════════════════════════════════════════╝"
