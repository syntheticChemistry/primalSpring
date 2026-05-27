#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# DEPRECATED (Wave 54): Use Rust: primalspring registry --check all
#
# JH-0: Method gate wiring validator
#
# Scans primal dispatcher binaries for MethodGate::check integration and
# validates the exempt whitelist consistency. Advisory-only (does not fail
# CI) — reports which dispatchers have the gate wired vs which still
# accept unauthenticated calls.
#
# Usage: tools/check_method_gate.sh [--strict]
#   --strict: exit 1 if any dispatcher lacks the gate (off by default)

set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

STRICT=0
if [[ "${1:-}" == "--strict" ]]; then
    STRICT=1
fi

BIN_DIR="ecoPrimal/src/bin"
GATE_PATTERN="MethodGate"
GATE_CHECK="dispatch_request_gated\|MethodGate::check\|method_gate::"
UNWIRED=0
WIRED=0
TOTAL=0

echo "=== Method Gate Wiring Report (JH-0) ==="
echo ""

for server_file in $(find "$BIN_DIR" -name "server.rs" -o -name "dispatch.rs" 2>/dev/null | sort); do
    bin_name=$(echo "$server_file" | sed 's|.*/bin/||' | cut -d/ -f1)
    ((TOTAL++)) || true

    if grep -q "$GATE_PATTERN" "$server_file" 2>/dev/null; then
        echo "  [GATED]   $bin_name ($server_file)"
        ((WIRED++)) || true
    else
        echo "  [OPEN]    $bin_name ($server_file)"
        ((UNWIRED++)) || true
    fi
done

echo ""

# Check whitelist consistency — extract PUBLIC_METHODS from method_gate.rs
GATE_SRC="ecoPrimal/src/ipc/method_gate.rs"
if [[ -f "$GATE_SRC" ]]; then
    PUBLIC_COUNT=$(grep -c '"[a-z]' <<< "$(grep -A 50 'const PUBLIC_METHODS' "$GATE_SRC" | grep '"')" 2>/dev/null || echo "0")
    PREFIX_COUNT=$(grep -c '"[a-z]' <<< "$(grep -A 10 'const PUBLIC_METHOD_PREFIXES' "$GATE_SRC" | grep '"')" 2>/dev/null || echo "0")
    echo "Whitelist: $PUBLIC_COUNT explicit methods + $PREFIX_COUNT prefix patterns"
else
    echo "WARNING: $GATE_SRC not found — cannot validate whitelist"
fi

echo ""
echo "Summary: $WIRED/$TOTAL dispatchers gated, $UNWIRED open"

if [[ $STRICT -eq 1 && $UNWIRED -gt 0 ]]; then
    echo ""
    echo "FAIL: --strict mode and $UNWIRED dispatcher(s) lack the method gate"
    exit 1
fi

if [[ $UNWIRED -gt 0 ]]; then
    echo "ADVISORY: $UNWIRED dispatcher(s) still accept unauthenticated calls"
fi

exit 0
