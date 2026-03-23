#!/usr/bin/env bash
# prepare_spore_payload.sh — Assemble a USB spore deployment payload
#
# Takes musl-built binaries from the staging area and assembles a complete
# deployment payload directory suitable for copying to a USB spore.
#
# Usage:
#   ./scripts/prepare_spore_payload.sh /path/to/output [--arch x86_64|aarch64|both]
#
# The output directory will contain:
#   primals/         — static musl binaries
#   graphs/          — all 22 primalSpring deploy graphs (incl. multi-node)
#   config/          — launch profiles
#   scripts/         — deploy_to_gate.sh, start_tower.sh, validate_remote_gate.sh
#   .family.seed     — genetics (copied from existing seed if available)
#
# Prerequisites: run build_ecosystem_musl.sh first

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SPRING_ROOT="$(dirname "$SCRIPT_DIR")"
ECO_ROOT="$(dirname "$SPRING_ROOT")"
STAGING="/tmp/primalspring-deploy"
ARCH="x86_64"

if [ $# -lt 1 ]; then
    echo "Usage: $0 <output-dir> [--arch x86_64|aarch64|both]"
    exit 1
fi

OUTPUT="$1"
shift

for arg in "$@"; do
    case "$arg" in
        --arch) shift; ARCH="${1:-x86_64}" ;;
        x86_64|aarch64|both) ARCH="$arg" ;;
    esac
done

echo "=== Preparing Spore Payload ==="
echo "Output:  $OUTPUT"
echo "Arch:    $ARCH"
echo "Staging: $STAGING"
echo ""

mkdir -p "$OUTPUT"/{primals,graphs,config,scripts}

copy_arch_binaries() {
    local arch="$1"
    local src="$STAGING/primals/$arch"
    if [ -d "$src" ]; then
        echo "Copying $arch binaries..."
        mkdir -p "$OUTPUT/primals/$arch"
        cp "$src"/* "$OUTPUT/primals/$arch/" 2>/dev/null || echo "  (no $arch binaries found in staging)"
        chmod +x "$OUTPUT/primals/$arch"/* 2>/dev/null || true
    else
        echo "WARNING: No $arch binaries in $src. Run build_ecosystem_musl.sh first."
    fi
}

case "$ARCH" in
    both)
        copy_arch_binaries "x86_64"
        copy_arch_binaries "aarch64"
        ;;
    *)
        copy_arch_binaries "$ARCH"
        ;;
esac

echo "Copying deploy graphs..."
cp -r "$SPRING_ROOT/graphs/"*.toml "$OUTPUT/graphs/" 2>/dev/null || true
if [ -d "$SPRING_ROOT/graphs/multi_node" ]; then
    mkdir -p "$OUTPUT/graphs/multi_node"
    cp "$SPRING_ROOT/graphs/multi_node/"*.toml "$OUTPUT/graphs/multi_node/" 2>/dev/null || true
fi
echo "  $(find "$OUTPUT/graphs" -name '*.toml' | wc -l) deploy graphs"

echo "Copying launch profiles..."
cp "$SPRING_ROOT/config/primal_launch_profiles.toml" "$OUTPUT/config/" 2>/dev/null || true

echo "Copying deployment scripts..."
cp "$SCRIPT_DIR/validate_remote_gate.sh" "$OUTPUT/scripts/" 2>/dev/null || true
chmod +x "$OUTPUT/scripts/"*.sh 2>/dev/null || true

BIOMEOS_SCRIPTS="$ECO_ROOT/phase2/biomeOS/livespore-usb/x86_64/scripts"
if [ -d "$BIOMEOS_SCRIPTS" ]; then
    for script in deploy_to_gate.sh start_tower.sh start_node.sh start_nest.sh deploy_atomic.sh deploy_cross_arch.sh; do
        if [ -f "$BIOMEOS_SCRIPTS/$script" ]; then
            cp "$BIOMEOS_SCRIPTS/$script" "$OUTPUT/scripts/"
        fi
    done
    chmod +x "$OUTPUT/scripts/"*.sh 2>/dev/null || true
    echo "  biomeOS deployment scripts included"
fi

SEED_PATHS=(
    "/media/$USER/BEA6-BBCE/biomeOS/.family.seed"
    "$ECO_ROOT/phase2/biomeOS/livespore-usb/.beacon_seeds/.family.seed"
    "$HOME/.config/biomeos/.family.seed"
)
for seed in "${SEED_PATHS[@]}"; do
    if [ -f "$seed" ]; then
        cp "$seed" "$OUTPUT/.family.seed"
        chmod 600 "$OUTPUT/.family.seed"
        echo "Genetics: .family.seed copied from $seed"
        break
    fi
done
if [ ! -f "$OUTPUT/.family.seed" ]; then
    echo "WARNING: No .family.seed found. Deployment will need manual genetics setup."
fi

echo ""
echo "=== Payload Ready ==="
echo "  $(find "$OUTPUT/primals" -type f 2>/dev/null | wc -l) binaries"
echo "  $(find "$OUTPUT/graphs" -name '*.toml' 2>/dev/null | wc -l) deploy graphs"
echo "  $(find "$OUTPUT/scripts" -name '*.sh' 2>/dev/null | wc -l) scripts"
echo ""
echo "To deploy to USB: cp -r $OUTPUT/* /media/<usb-mount>/"
echo "To deploy to gate: $OUTPUT/scripts/deploy_to_gate.sh <gate-ip> <node-id>"
