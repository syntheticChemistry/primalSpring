#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# primalSpring composition validation.
#
# Thin wrapper around validate_local_lab.sh that runs the Phase 22
# E2E composition experiments (exp085–exp088) against a benchScale lab.
# For local-only mode (primals already running), runs experiments directly.
#
# Usage:
#   ./scripts/validate_composition.sh                          # local (primals already running)
#   ./scripts/validate_composition.sh --topology tower         # benchScale 2-node tower lab
#   ./scripts/validate_composition.sh --topology storytelling  # full storytelling stack
#   ./scripts/validate_composition.sh --topology nucleus       # 3-node NUCLEUS
#   TOWER_HOST=10.0.0.5 ./scripts/validate_composition.sh     # remote tower

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$WORKSPACE_ROOT"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

TOPOLOGY=""
TOWER_HOST="${TOWER_HOST:-127.0.0.1}"
KEEP_LAB=false
SKIP_BUILD=false
FAILURES=0
SKIPPED=0
PASSED=0

step() { printf "\n${YELLOW}═══ %s ═══${NC}\n" "$1"; }
pass() { printf "${GREEN}✓ %s${NC}\n" "$1"; PASSED=$((PASSED + 1)); }
fail() { printf "${RED}✗ %s${NC}\n" "$1"; FAILURES=$((FAILURES + 1)); }
skip() { printf "${YELLOW}⊘ %s${NC}\n" "$1"; SKIPPED=$((SKIPPED + 1)); }

COMPOSITION_EXPERIMENTS=(
    "exp085_beardog_crypto_lifecycle:primalspring-exp085:BearDog Crypto Lifecycle"
    "exp086_genetic_identity_e2e:primalspring-exp086:Genetic Identity E2E"
    "exp087_neural_api_routing_e2e:primalspring-exp087:Neural API Routing E2E"
    "exp088_storytelling_composition:primalspring-exp088:Storytelling Composition"
)

usage() {
    cat << EOF
Usage: $0 [options]

Composition validation for primalSpring Phase 22+ E2E experiments.

Modes:
  No flags              Run experiments against locally running primals
  --topology <name>     Spin up a benchScale lab, deploy, validate, teardown

Options:
  --topology <name>     tower | storytelling | nucleus | <yaml-name>
  --host <addr>         Override TOWER_HOST for remote primals
  --keep                Keep benchScale lab alive after validation
  --skip-build          Skip cargo build step
  --help                Show this help

Examples:
  $0                                    # Quick local validation
  $0 --topology tower                   # 2-node Docker tower lab
  $0 --topology storytelling --keep     # Storytelling stack, keep lab
  TOWER_HOST=10.0.0.5 $0               # Remote tower

EOF
    exit 0
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --topology)   TOPOLOGY="$2"; shift 2 ;;
        --host)       TOWER_HOST="$2"; shift 2 ;;
        --keep)       KEEP_LAB=true; shift ;;
        --skip-build) SKIP_BUILD=true; shift ;;
        --help)       usage ;;
        *)            echo "Unknown option: $1"; usage ;;
    esac
done

# ── Topology-based mode: delegate to validate_local_lab.sh ──────────────────

if [ -n "$TOPOLOGY" ]; then
    TOPO_MAP=""
    case "$TOPOLOGY" in
        tower)        TOPO_MAP="ecoprimals-tower-2node-tcp" ;;
        storytelling) TOPO_MAP="ecoprimals-storytelling-tower" ;;
        nucleus)      TOPO_MAP="ecoprimals-nucleus-3node" ;;
        *)            TOPO_MAP="$TOPOLOGY" ;;
    esac

    step "Delegating to validate_local_lab.sh (topology: $TOPO_MAP)"

    EXTRA_ARGS=""
    $KEEP_LAB && EXTRA_ARGS="--keep"

    EXP_BINS=""
    for entry in "${COMPOSITION_EXPERIMENTS[@]}"; do
        IFS=':' read -r bin _pkg _desc <<< "$entry"
        [ -n "$EXP_BINS" ] && EXP_BINS="$EXP_BINS,"
        EXP_BINS="$EXP_BINS$bin"
    done

    exec "$WORKSPACE_ROOT/scripts/validate_local_lab.sh" \
        --topology "$TOPO_MAP" \
        --experiments "$EXP_BINS" \
        $EXTRA_ARGS
fi

# ── Local mode: run experiments directly against running primals ─────────────

export TOWER_HOST

step "Build composition experiments"
if [ "$SKIP_BUILD" != true ]; then
    BUILD_BINS=""
    for entry in "${COMPOSITION_EXPERIMENTS[@]}"; do
        IFS=':' read -r bin _pkg _desc <<< "$entry"
        BUILD_BINS="$BUILD_BINS --bin $bin"
    done
    if cargo build $BUILD_BINS 2>&1; then
        pass "experiments built"
    else
        fail "experiment build failed"
        exit 1
    fi
else
    skip "build (--skip-build)"
fi

for entry in "${COMPOSITION_EXPERIMENTS[@]}"; do
    IFS=':' read -r bin pkg desc <<< "$entry"
    step "$desc"
    if timeout 30 cargo run -p "$pkg" --bin "$bin" 2>&1; then
        pass "$desc"
    else
        exit_code=$?
        if [ $exit_code -eq 124 ]; then
            fail "$desc (timeout)"
        else
            fail "$desc (exit $exit_code)"
        fi
    fi
done

printf "\n"
step "COMPOSITION VALIDATION RESULT"
printf "  Passed:  %d\n" "$PASSED"
printf "  Failed:  %d\n" "$FAILURES"
printf "  Skipped: %d\n" "$SKIPPED"

if [ "$FAILURES" -eq 0 ]; then
    printf "${GREEN}Composition validation passed${NC}\n"
    exit 0
else
    printf "${RED}%d composition check(s) failed${NC}\n" "$FAILURES"
    exit 1
fi
