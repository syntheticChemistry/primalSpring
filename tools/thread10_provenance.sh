#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# Thread 10 Provenance Capture — primalSpring
#
# Runs primalspring validate and writes provenance artifacts to
# the projectFOUNDATION validation folder for Thread 10.
#
# Usage:
#   ./tools/thread10_provenance.sh [--tier rust|live|both]
#
# Environment:
#   FOUNDATION_ROOT   Override path to projectFOUNDATION (default: auto-detect)
#   PRIMALSPRING_ROOT Override spring root (default: script directory parent)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SPRING_ROOT="${PRIMALSPRING_ROOT:-$(dirname "$SCRIPT_DIR")}"

FOUNDATION_ROOT="${FOUNDATION_ROOT:-${SPRING_ROOT}/../../gardens/foundation}"
if [ ! -d "$FOUNDATION_ROOT" ]; then
    echo "warning: projectFOUNDATION not found at $FOUNDATION_ROOT"
    echo "  Set FOUNDATION_ROOT to override"
fi

TODAY="$(date +%Y-%m-%d)"
PROVENANCE_DIR="${FOUNDATION_ROOT}/validation/primalSpring/${TODAY}"

TIER_ARG=""
if [ "${1:-}" = "--tier" ] && [ -n "${2:-}" ]; then
    TIER_ARG="--tier $2"
fi

UNIBIN="${SPRING_ROOT}/ecoPrimal/target/release/primalspring_unibin"
if [ ! -x "$UNIBIN" ]; then
    UNIBIN="${SPRING_ROOT}/ecoPrimal/target/release/primalspring"
fi
if [ ! -x "$UNIBIN" ]; then
    UNIBIN="${SPRING_ROOT}/ecoPrimal/target/debug/primalspring_unibin"
fi
if [ ! -x "$UNIBIN" ]; then
    UNIBIN="${SPRING_ROOT}/ecoPrimal/target/debug/primalspring"
fi

if [ ! -x "$UNIBIN" ]; then
    echo "error: primalspring binary not found. Build with:"
    echo "  cd ${SPRING_ROOT}/ecoPrimal && cargo build --release"
    exit 1
fi

echo "=== Thread 10 Provenance Capture ==="
echo "Spring:     primalSpring"
echo "Binary:     $UNIBIN"
echo "Output:     $PROVENANCE_DIR"
echo "Date:       $TODAY"
echo ""

mkdir -p "$PROVENANCE_DIR"

# shellcheck disable=SC2086
"$UNIBIN" validate $TIER_ARG \
    --format json \
    --provenance-dir "$PROVENANCE_DIR" \
    | tee "${PROVENANCE_DIR}/validate.stdout" || true

echo ""
echo "=== Provenance artifacts ==="
ls -la "$PROVENANCE_DIR/" 2>/dev/null || echo "(none)"
