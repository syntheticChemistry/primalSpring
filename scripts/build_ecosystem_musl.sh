#!/usr/bin/env bash
# build_ecosystem_musl.sh — Build all core primals as musl static binaries
#
# Iterates over ecoPrimals primal source directories and builds each for
# x86_64-unknown-linux-musl and (optionally) aarch64-unknown-linux-musl.
#
# Usage:
#   ./scripts/build_ecosystem_musl.sh                 # x86_64 only
#   ./scripts/build_ecosystem_musl.sh --aarch64       # both architectures
#   ./scripts/build_ecosystem_musl.sh --aarch64-only  # aarch64 only
#   ./scripts/build_ecosystem_musl.sh --harvest       # build + harvest into plasmidBin
#
# Output: /tmp/primalspring-deploy/primals/{x86_64,aarch64}/
#
# Prerequisites:
#   - musl-tools (apt install musl-tools)
#   - aarch64-linux-musl-gcc for aarch64 cross-compile
#   - rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SPRING_ROOT="$(dirname "$SCRIPT_DIR")"
ECO_ROOT="$(dirname "$SPRING_ROOT")"
STAGING="/tmp/primalspring-deploy"

X86_TARGET="x86_64-unknown-linux-musl"
ARM_TARGET="aarch64-unknown-linux-musl"

BUILD_X86=true
BUILD_ARM=false
DO_HARVEST=false

# aarch64-unknown-linux-musl needs a cross-linker.
# aarch64-linux-gnu-gcc works for pure Rust (Rust provides musl runtime).
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER="${CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER:-aarch64-linux-gnu-gcc}"

for arg in "$@"; do
    case "$arg" in
        --aarch64)      BUILD_ARM=true ;;
        --aarch64-only) BUILD_ARM=true; BUILD_X86=false ;;
        --harvest)      DO_HARVEST=true ;;
        --help|-h)
            echo "Usage: $0 [--aarch64] [--aarch64-only] [--harvest]"
            exit 0
            ;;
    esac
done

PRIMALS=(
    "primals/beardog"
    "primals/songbird"
    "primals/nestgate"
    "primals/toadstool"
    "primals/squirrel"
    "primals/biomeOS"
)

PRIMAL_NAMES=(beardog songbird nestgate toadstool squirrel biomeos)

passed=0
failed=0
skipped=0

log_ok()   { echo "  [OK]   $1"; }
log_fail() { echo "  [FAIL] $1"; }
log_skip() { echo "  [SKIP] $1"; }

build_target() {
    local src_dir="$1"
    local target="$2"
    local arch_label="$3"
    local out_dir="$STAGING/primals/$arch_label"

    mkdir -p "$out_dir"

    if [ ! -d "$src_dir" ]; then
        log_skip "$src_dir (directory not found)"
        ((skipped++)) || true
        return
    fi

    local name
    name="$(basename "$src_dir")"

    echo "  Building $name for $target ..."
    if cargo build --release --target "$target" --manifest-path "$src_dir/Cargo.toml" 2>/tmp/build_${name}_${arch_label}.log; then
        local bin_dir="$src_dir/target/$target/release"
        local copied=0
        for bin in "$bin_dir"/*; do
            [ -f "$bin" ] && [ -x "$bin" ] && [ ! -d "$bin" ] || continue
            local bn
            bn="$(basename "$bin")"
            # Skip build artifacts that aren't real binaries
            case "$bn" in
                *.d|*.rlib|*.rmeta|*.so|build-script-*|*.a) continue ;;
            esac
            # Verify it's actually an ELF binary
            if file "$bin" | grep -q "ELF"; then
                cp "$bin" "$out_dir/${bn}-${arch_label}-linux-musl"
                ((copied++)) || true
            fi
        done
        if [ "$copied" -gt 0 ]; then
            log_ok "$name ($arch_label): $copied binaries"
            ((passed++)) || true
        else
            log_fail "$name ($arch_label): built but no binaries found"
            ((failed++)) || true
        fi
    else
        log_fail "$name ($arch_label): build failed (see /tmp/build_${name}_${arch_label}.log)"
        ((failed++)) || true
    fi
}

echo "=== primalSpring Ecosystem musl Build ==="
echo "Source root: $ECO_ROOT"
echo "Staging:     $STAGING"
echo ""

mkdir -p "$STAGING/primals"

# Build primalSpring itself
echo "--- primalSpring ---"
if $BUILD_X86; then
    build_target "$SPRING_ROOT" "$X86_TARGET" "x86_64"
fi
if $BUILD_ARM; then
    build_target "$SPRING_ROOT" "$ARM_TARGET" "aarch64"
fi

# Build each primal
for i in "${!PRIMALS[@]}"; do
    src="${ECO_ROOT}/${PRIMALS[$i]}"
    echo "--- ${PRIMAL_NAMES[$i]} ---"
    if $BUILD_X86; then
        build_target "$src" "$X86_TARGET" "x86_64"
    fi
    if $BUILD_ARM; then
        build_target "$src" "$ARM_TARGET" "aarch64"
    fi
done

echo ""
echo "=== Build Summary ==="
echo "  Passed:  $passed"
echo "  Failed:  $failed"
echo "  Skipped: $skipped"
echo ""

if $BUILD_X86 && [ -d "$STAGING/primals/x86_64" ]; then
    echo "x86_64 binaries:"
    ls -lh "$STAGING/primals/x86_64/" 2>/dev/null | tail -n +2
fi
if $BUILD_ARM && [ -d "$STAGING/primals/aarch64" ]; then
    echo ""
    echo "aarch64 binaries:"
    ls -lh "$STAGING/primals/aarch64/" 2>/dev/null | tail -n +2
fi

echo ""
echo "Staging directory: $STAGING"

if [ "$failed" -gt 0 ]; then
    echo "WARNING: $failed builds failed. Check logs in /tmp/build_*.log"
    exit 1
fi

if $DO_HARVEST; then
    HARVEST_SCRIPT="$ECO_ROOT/plasmidBin/harvest.sh"
    if [ -x "$HARVEST_SCRIPT" ]; then
        echo ""
        echo "=== Harvesting into plasmidBin ==="
        if $BUILD_X86 && [ -d "$STAGING/primals/x86_64" ]; then
            "$HARVEST_SCRIPT" --source "$STAGING/primals/x86_64" --arch x86_64
        fi
        if $BUILD_ARM && [ -d "$STAGING/primals/aarch64" ]; then
            "$HARVEST_SCRIPT" --source "$STAGING/primals/aarch64" --arch aarch64
        fi
    else
        echo ""
        echo "WARNING: harvest.sh not found at $HARVEST_SCRIPT"
        echo "  Run from ecoPrimals/ or ensure plasmidBin/harvest.sh exists."
        exit 1
    fi
fi
