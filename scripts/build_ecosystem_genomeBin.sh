#!/usr/bin/env bash
# build_ecosystem_genomeBin.sh — Build all primals across the full genomeBin target matrix
#
# Implements the ecoBin Architecture Standard's three-tier cross-compilation matrix:
#
#   Tier 1 (MUST):   Linux musl static — x86_64, aarch64, armv7
#   Tier 2 (SHOULD): macOS (check-only), Windows (mingw), Android (NDK)
#   Tier 3 (NICE):   RISC-V (musl), WASM (check-only)
#
# Usage:
#   ./scripts/build_ecosystem_genomeBin.sh --tier1           # Linux musl static (x86_64 + aarch64 + armv7)
#   ./scripts/build_ecosystem_genomeBin.sh --tier2           # macOS + Windows + Android
#   ./scripts/build_ecosystem_genomeBin.sh --tier3           # RISC-V + WASM
#   ./scripts/build_ecosystem_genomeBin.sh --all             # All tiers
#   ./scripts/build_ecosystem_genomeBin.sh --target <triple> # Single target
#   ./scripts/build_ecosystem_genomeBin.sh --harvest         # Build + harvest into plasmidBin
#
# Output: /tmp/primalspring-deploy/primals/{target-triple}/
#
# Prerequisites:
#   musl-tools, aarch64-linux-gnu-gcc, arm-linux-gnueabihf-gcc,
#   x86_64-w64-mingw32-gcc, riscv64-linux-gnu-gcc, Android NDK r25c
#   rustup target add <all targets>

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SPRING_ROOT="$(dirname "$SCRIPT_DIR")"
ECO_ROOT="$(dirname "$(dirname "$SPRING_ROOT")")"
STAGING="/tmp/primalspring-deploy"

# ── Target definitions ────────────────────────────────────────────────────────

TIER1_TARGETS=(
    "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-musl"
    "armv7-unknown-linux-musleabihf"
)

TIER2_TARGETS=(
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-gnu"
    "aarch64-linux-android"
)

TIER3_TARGETS=(
    "riscv64gc-unknown-linux-musl"
    "wasm32-wasip1"
)

# Targets where we can only cargo-check (no linker available for full build)
CHECK_ONLY_TARGETS=(
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "wasm32-wasip1"
)

# ── Linker configuration ─────────────────────────────────────────────────────
# Cargo needs these env vars to find the right cross-linker per target.
# ~/.cargo/config.toml has aarch64-musl and android already; we set the rest.

configure_linkers() {
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER="${CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER:-aarch64-linux-gnu-gcc}"
    export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER="${CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER:-arm-linux-gnueabihf-gcc}"
    export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="${CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER:-x86_64-w64-mingw32-gcc}"
    export CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_MUSL_LINKER="${CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_MUSL_LINKER:-riscv64-linux-gnu-gcc}"

    local ndk_base="${ANDROID_NDK_HOME:-/opt/android-ndk-r25c}"
    local ndk_toolchain="$ndk_base/toolchains/llvm/prebuilt/linux-x86_64/bin"
    if [[ -d "$ndk_toolchain" ]]; then
        export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="${CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER:-$ndk_toolchain/aarch64-linux-android28-clang}"
    fi
}

# ── Primal source directories ────────────────────────────────────────────────

PRIMALS=(
    "primals/beardog"
    "primals/songbird"
    "primals/nestgate"
    "primals/toadStool"
    "primals/squirrel"
    "primals/biomeOS"
    "primals/barraCuda"
    "primals/coralReef"
    "primals/rhizoCrypt"
    "primals/loamSpine"
    "primals/sweetGrass"
    "primals/petalTongue"
    "primals/skunkBat"
)

PRIMAL_NAMES=(beardog songbird nestgate toadstool squirrel biomeos barracuda coralreef rhizocrypt loamspine sweetgrass petaltongue skunkbat)

# ── Argument parsing ─────────────────────────────────────────────────────────

BUILD_TIER1=false
BUILD_TIER2=false
BUILD_TIER3=false
DO_HARVEST=false
EXPLICIT_TARGETS=()

for arg in "$@"; do
    case "$arg" in
        --tier1)    BUILD_TIER1=true ;;
        --tier2)    BUILD_TIER2=true ;;
        --tier3)    BUILD_TIER3=true ;;
        --all)      BUILD_TIER1=true; BUILD_TIER2=true; BUILD_TIER3=true ;;
        --harvest)  DO_HARVEST=true ;;
        --target)   ;; # next arg is the target triple
        --help|-h)
            echo "Usage: $0 [--tier1] [--tier2] [--tier3] [--all] [--target <triple>] [--harvest]"
            echo ""
            echo "Tiers:"
            echo "  --tier1   MUST:   ${TIER1_TARGETS[*]}"
            echo "  --tier2   SHOULD: ${TIER2_TARGETS[*]}"
            echo "  --tier3   NICE:   ${TIER3_TARGETS[*]}"
            echo "  --all     All tiers"
            echo "  --target  Single target triple"
            echo "  --harvest Build + harvest into plasmidBin"
            exit 0
            ;;
        *)
            # Catch --target VALUE pattern
            if [[ "${prev_arg:-}" == "--target" ]]; then
                EXPLICIT_TARGETS+=("$arg")
            fi
            ;;
    esac
    prev_arg="$arg"
done

# Collect all targets to build
TARGETS=()
$BUILD_TIER1 && TARGETS+=("${TIER1_TARGETS[@]}")
$BUILD_TIER2 && TARGETS+=("${TIER2_TARGETS[@]}")
$BUILD_TIER3 && TARGETS+=("${TIER3_TARGETS[@]}")
TARGETS+=("${EXPLICIT_TARGETS[@]}")

if [[ ${#TARGETS[@]} -eq 0 ]]; then
    echo "ERROR: No targets specified. Use --tier1, --tier2, --tier3, --all, or --target <triple>"
    echo "Run $0 --help for usage."
    exit 1
fi

# Deduplicate
readarray -t TARGETS < <(printf '%s\n' "${TARGETS[@]}" | sort -u)

# ── Counters ─────────────────────────────────────────────────────────────────

build_passed=0
build_failed=0
check_passed=0
check_failed=0
skipped=0

log_ok()    { echo "  [OK]    $1"; }
log_check() { echo "  [CHECK] $1"; }
log_fail()  { echo "  [FAIL]  $1"; }
log_skip()  { echo "  [SKIP]  $1"; }

# ── Check if target is check-only ────────────────────────────────────────────

is_check_only() {
    local target="$1"
    for co in "${CHECK_ONLY_TARGETS[@]}"; do
        [[ "$target" == "$co" ]] && return 0
    done
    return 1
}

# ── Verify linker exists for a target ─────────────────────────────────────────

has_linker() {
    local target="$1"
    case "$target" in
        x86_64-unknown-linux-musl)
            return 0 ;; # native, musl-tools
        aarch64-unknown-linux-musl)
            command -v aarch64-linux-gnu-gcc >/dev/null 2>&1 ;;
        armv7-unknown-linux-musleabihf)
            command -v arm-linux-gnueabihf-gcc >/dev/null 2>&1 ;;
        x86_64-pc-windows-gnu)
            command -v x86_64-w64-mingw32-gcc >/dev/null 2>&1 ;;
        aarch64-linux-android)
            local ndk_base="${ANDROID_NDK_HOME:-/opt/android-ndk-r25c}"
            [[ -x "$ndk_base/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android28-clang" ]] ;;
        riscv64gc-unknown-linux-musl)
            command -v riscv64-linux-gnu-gcc >/dev/null 2>&1 ;;
        x86_64-apple-darwin|aarch64-apple-darwin)
            return 1 ;; # no osxcross
        wasm32-wasip1)
            return 0 ;; # no linker needed for wasm
        *)
            return 1 ;;
    esac
}

# ── Binary extension for target ──────────────────────────────────────────────

binary_ext() {
    case "$1" in
        *-windows-*) echo ".exe" ;;
        wasm32-*)    echo ".wasm" ;;
        *)           echo "" ;;
    esac
}

# ── Build or check a single primal for a single target ───────────────────────

build_target() {
    local src_dir="$1"
    local target="$2"
    local out_dir="$STAGING/primals/$target"
    local log_base="/tmp/genomeBin_$(basename "$src_dir")_${target}"

    if [[ ! -d "$src_dir" ]]; then
        log_skip "$(basename "$src_dir") @ $target (source not found)"
        ((skipped++)) || true
        return
    fi

    local name
    name="$(basename "$src_dir")"

    # Phase 1: cargo check (fast validation for all targets)
    echo -n "  $name @ $target ... "
    if ! cargo check --release --target "$target" --manifest-path "$src_dir/Cargo.toml" 2>"${log_base}_check.log"; then
        log_fail "$name @ $target: cargo check failed (see ${log_base}_check.log)"
        ((check_failed++)) || true
        return
    fi
    ((check_passed++)) || true

    # Phase 2: if check-only target or no linker, stop here
    if is_check_only "$target"; then
        log_check "$name @ $target: check passed (check-only target)"
        return
    fi

    if ! has_linker "$target"; then
        log_check "$name @ $target: check passed (no linker for full build)"
        return
    fi

    # Phase 3: full build
    mkdir -p "$out_dir"

    if cargo build --release --target "$target" --manifest-path "$src_dir/Cargo.toml" 2>"${log_base}_build.log"; then
        local bin_dir="$src_dir/target/$target/release"
        local ext
        ext=$(binary_ext "$target")
        local copied=0

        for bin in "$bin_dir"/*; do
            [[ -f "$bin" ]] && [[ -x "$bin" ]] || continue
            local bn
            bn="$(basename "$bin")"
            case "$bn" in
                *.d|*.rlib|*.rmeta|*.so|*.dll|*.dylib|*.a|build-script-*) continue ;;
            esac
            # Verify it's a real binary (ELF, PE, or Mach-O)
            local file_type
            file_type=$(file "$bin" 2>/dev/null) || true
            if echo "$file_type" | grep -qE "ELF|PE32|Mach-O|WebAssembly"; then
                cp "$bin" "$out_dir/$bn"
                ((copied++)) || true
            fi
        done

        if [[ "$copied" -gt 0 ]]; then
            log_ok "$name @ $target: $copied binaries"
            ((build_passed++)) || true
        else
            log_fail "$name @ $target: built but no binaries found"
            ((build_failed++)) || true
        fi
    else
        log_fail "$name @ $target: build failed (see ${log_base}_build.log)"
        ((build_failed++)) || true
    fi
}

# ── Main ─────────────────────────────────────────────────────────────────────

configure_linkers

echo "=================================================================="
echo "  primalSpring Ecosystem genomeBin Build"
echo "=================================================================="
echo "Source root: $ECO_ROOT"
echo "Staging:     $STAGING"
echo "Targets:     ${TARGETS[*]}"
echo ""

mkdir -p "$STAGING/primals"

for target in "${TARGETS[@]}"; do
    echo ""
    echo "── $target ──────────────────────────────────────"
    if is_check_only "$target"; then
        echo "   (check-only — no linker available for full build)"
    elif ! has_linker "$target"; then
        echo "   (check-only — linker not found)"
    fi
    echo ""

    # Build primalSpring itself (primalspring_primal)
    build_target "$SPRING_ROOT" "$target"

    # Build each primal
    for i in "${!PRIMALS[@]}"; do
        src="${ECO_ROOT}/${PRIMALS[$i]}"
        build_target "$src" "$target"
    done
done

echo ""
echo "=================================================================="
echo "  genomeBin Build Summary"
echo "=================================================================="
echo "  Build passed:  $build_passed"
echo "  Build failed:  $build_failed"
echo "  Check passed:  $check_passed"
echo "  Check failed:  $check_failed"
echo "  Skipped:       $skipped"
echo ""

# List staging output
for target in "${TARGETS[@]}"; do
    local_dir="$STAGING/primals/$target"
    if [[ -d "$local_dir" ]] && [[ "$(ls -A "$local_dir" 2>/dev/null)" ]]; then
        echo "$target binaries:"
        ls -lhS "$local_dir/" 2>/dev/null | tail -n +2
        echo ""
    fi
done

echo "Staging directory: $STAGING"

if [[ "$build_failed" -gt 0 ]]; then
    echo ""
    echo "WARNING: $build_failed builds failed. Check logs in /tmp/genomeBin_*.log"
fi

if [[ "$check_failed" -gt 0 ]]; then
    echo "WARNING: $check_failed cargo checks failed. These primals have target-incompatible deps."
fi

# ── Harvest ──────────────────────────────────────────────────────────────────

if $DO_HARVEST; then
    HARVEST_SCRIPT="$ECO_ROOT/infra/plasmidBin/harvest.sh"
    if [[ -x "$HARVEST_SCRIPT" ]]; then
        echo ""
        echo "=== Harvesting into plasmidBin ==="
        for target in "${TARGETS[@]}"; do
            local_dir="$STAGING/primals/$target"
            if [[ -d "$local_dir" ]] && [[ "$(ls -A "$local_dir" 2>/dev/null)" ]]; then
                echo "Harvesting $target ..."
                "$HARVEST_SCRIPT" --source "$local_dir" --arch "$target"
            fi
        done
    else
        echo ""
        echo "WARNING: harvest.sh not found at $HARVEST_SCRIPT"
        echo "  Harvest manually or check plasmidBin path."
    fi
fi

# Non-zero exit if any builds failed
if [[ "$build_failed" -gt 0 ]]; then
    exit 1
fi
