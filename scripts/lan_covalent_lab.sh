#!/usr/bin/env bash
# lan_covalent_lab.sh — Run the LAN covalent mesh validation suite
#
# Orchestrates a two-gate LAN test:
#   1. Validates that the local Tower Atomic is healthy
#   2. Runs exp073 (LAN covalent mesh) against a remote gate
#   3. Runs exp074 (cross-gate per-primal health) against the same gate
#   4. Runs exp090 (LAN probe / mesh discovery) for topology mapping
#
# Usage:
#   ./scripts/lan_covalent_lab.sh <remote-host> [family-id]
#
# Examples:
#   ./scripts/lan_covalent_lab.sh 192.168.1.100
#   ./scripts/lan_covalent_lab.sh brothers-gate.local my-family-id
#
# Prerequisites:
#   - Local gate: NUCLEUS running (beardog + songbird at minimum)
#   - Remote gate: NUCLEUS running with --port flags (TCP enabled)
#   - Same FAMILY_SEED on both gates (shared secret for BTSP)
#   - plasmidBin experiments built: cargo build --workspace
#
# Environment overrides:
#   BEARDOG_PORT           — local BearDog TCP port (default: 9100)
#   SONGBIRD_PORT          — local Songbird TCP port (default: 9200)
#   REMOTE_BEARDOG_PORT    — remote BearDog TCP port (default: 9100)
#   REMOTE_SONGBIRD_PORT   — remote Songbird TCP port (default: 9200)

set -euo pipefail

REMOTE_HOST="${1:-}"
FAMILY_ID="${2:-${FAMILY_ID:-8ff3b864a4bc589a}}"

if [ -z "$REMOTE_HOST" ]; then
    echo "Usage: $0 <remote-host> [family-id]"
    echo ""
    echo "Examples:"
    echo "  $0 192.168.1.100"
    echo "  $0 brothers-gate.local my-family-seed-hex"
    echo ""
    echo "Ensure both gates are running NUCLEUS with the same FAMILY_SEED."
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

BEARDOG_PORT="${BEARDOG_PORT:-9100}"
SONGBIRD_PORT="${SONGBIRD_PORT:-9200}"
REMOTE_BEARDOG_PORT="${REMOTE_BEARDOG_PORT:-$BEARDOG_PORT}"
REMOTE_SONGBIRD_PORT="${REMOTE_SONGBIRD_PORT:-$SONGBIRD_PORT}"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║         LAN Covalent Lab — Two-Gate Validation              ║"
echo "╠══════════════════════════════════════════════════════════════╣"
echo "║  Remote gate:    $REMOTE_HOST"
echo "║  Family ID:      $FAMILY_ID"
echo "║  Local BearDog:  localhost:$BEARDOG_PORT"
echo "║  Local Songbird: localhost:$SONGBIRD_PORT"
echo "║  Remote BearDog: $REMOTE_HOST:$REMOTE_BEARDOG_PORT"
echo "║  Remote Songbird:$REMOTE_HOST:$REMOTE_SONGBIRD_PORT"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Phase 0: Quick local health check
echo "══ Phase 0: Local Gate Health ══"
"$SCRIPT_DIR/validate_remote_gate.sh" localhost || {
    echo ""
    echo "Local gate is not fully healthy. Start NUCLEUS first:"
    echo "  FAMILY_ID=$FAMILY_ID FAMILY_SEED=<shared-seed> beardog server --port $BEARDOG_PORT &"
    echo "  FAMILY_ID=$FAMILY_ID songbird server --port $SONGBIRD_PORT &"
    exit 1
}
echo ""

# Phase 1: Remote gate health
echo "══ Phase 1: Remote Gate Health (exp074) ══"
export REMOTE_GATE_HOST="$REMOTE_HOST"
export REMOTE_BEARDOG_PORT
export REMOTE_SONGBIRD_PORT
export FAMILY_ID

cd "$PROJECT_DIR"
if cargo run --bin exp074_cross_gate_health 2>&1; then
    echo "  exp074: PASS"
else
    echo "  exp074: FAIL (remote gate not fully reachable)"
    echo "  Continuing with available validations..."
fi
echo ""

# Phase 2: LAN mesh + beacon + genetics (exp073)
echo "══ Phase 2: LAN Covalent Mesh (exp073) ══"
if cargo run --bin exp073_lan_covalent_mesh 2>&1; then
    echo "  exp073: PASS"
else
    echo "  exp073: FAIL (some covalent mesh checks failed)"
fi
echo ""

# Phase 3: LAN topology probe (exp090)
echo "══ Phase 3: LAN Topology Probe (exp090) ══"
export BEARDOG_PORT
export SONGBIRD_PORT
export NODE_ID="${NODE_ID:-$(hostname)}"

if cargo run --bin exp090_tower_atomic_lan_probe 2>&1; then
    echo "  exp090: PASS"
else
    echo "  exp090: FAIL (some LAN probe checks failed)"
fi
echo ""

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  LAN Covalent Lab — Complete                                ║"
echo "╚══════════════════════════════════════════════════════════════╝"
