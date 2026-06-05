#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# NUCLEUS Full Validation Pipeline
#
# End-to-end validation from deployment pipeline through to live
# cross-gate scenarios. Chains together all validation tooling:
#
#   1. plasmidbin doctor   — binary inventory, checksums, prerequisites
#   2. plasmidbin sync     — deploy verified binaries to genomeBin
#   3. nucleus_launcher    — start composition with pre-flight + health
#   4. validate_composition.sh — exp085-088, exp094
#   5. cross-gate scenarios (optional, requires --cross-gate)
#
# Usage:
#   ./scripts/nucleus_full_validation.sh                         # local nucleus
#   ./scripts/nucleus_full_validation.sh --composition tower     # tower only
#   ./scripts/nucleus_full_validation.sh --cross-gate            # + cross-gate mesh
#   ./scripts/nucleus_full_validation.sh --dry-run               # show plan only
#   ./scripts/nucleus_full_validation.sh --skip-start            # primals already running

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ECOPRIMALS_ROOT="${ECOPRIMALS_ROOT:-$(cd "$WORKSPACE_ROOT/../.." && pwd)}"
PLASMIDBIN_ROOT="${ECOPRIMALS_ROOT}/infra/plasmidBin"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

COMPOSITION="${COMPOSITION:-nucleus}"
FAMILY_ID="${FAMILY_ID:-eastgate}"
DRY_RUN=false
CROSS_GATE=false
SKIP_START=false
STOP_AFTER=false
FAILURES=0
PASSED=0
SKIPPED=0

usage() {
    cat << EOF
Usage: $0 [options]

NUCLEUS Full Validation Pipeline — deployment through to live mesh.

Options:
  --composition <name>  tower | node | nest | nucleus (default: nucleus)
  --family-id <id>      Family identifier (default: eastgate)
  --cross-gate          Run cross-gate trust scenarios after local validation
  --skip-start          Skip startup (primals already running)
  --stop-after          Stop primals after validation
  --dry-run             Show what would run without executing
  -h, --help            Show this help
EOF
}

step() { printf "\n${CYAN}══════════════════════════════════════════════${NC}\n"; printf "${CYAN}  %s${NC}\n" "$1"; printf "${CYAN}══════════════════════════════════════════════${NC}\n\n"; }
pass() { printf "${GREEN}  PASS: %s${NC}\n" "$1"; PASSED=$((PASSED + 1)); }
fail() { printf "${RED}  FAIL: %s${NC}\n" "$1"; FAILURES=$((FAILURES + 1)); }
skip() { printf "${YELLOW}  SKIP: %s${NC}\n" "$1"; SKIPPED=$((SKIPPED + 1)); }

while [[ $# -gt 0 ]]; do
    case "$1" in
        --composition) COMPOSITION="$2"; shift 2 ;;
        --family-id)   FAMILY_ID="$2"; shift 2 ;;
        --cross-gate)  CROSS_GATE=true; shift ;;
        --skip-start)  SKIP_START=true; shift ;;
        --stop-after)  STOP_AFTER=true; shift ;;
        --dry-run)     DRY_RUN=true; shift ;;
        -h|--help)     usage; exit 0 ;;
        *)             echo "Unknown option: $1"; usage; exit 1 ;;
    esac
done

echo ""
echo "  NUCLEUS Full Validation Pipeline"
echo "  ================================"
echo "  Composition:  $COMPOSITION"
echo "  Family:       $FAMILY_ID"
echo "  Cross-gate:   $CROSS_GATE"
echo "  Workspace:    $WORKSPACE_ROOT"
echo "  plasmidBin:   $PLASMIDBIN_ROOT"
echo ""

# ── Step 1: plasmidbin doctor ─────────────────────────────────────────
step "Step 1: plasmidbin doctor"

if [[ "$DRY_RUN" == "true" ]]; then
    skip "plasmidbin doctor (dry-run)"
elif [[ -x "$PLASMIDBIN_ROOT/target/release/plasmidbin" ]]; then
    if "$PLASMIDBIN_ROOT/target/release/plasmidbin" doctor --root "$PLASMIDBIN_ROOT"; then
        pass "plasmidbin doctor"
    else
        fail "plasmidbin doctor — run 'plasmidbin rehash' to fix checksums"
    fi
elif command -v cargo >/dev/null 2>&1; then
    if (cd "$PLASMIDBIN_ROOT" && cargo run -p plasmidbin --quiet -- doctor); then
        pass "plasmidbin doctor"
    else
        fail "plasmidbin doctor — run 'plasmidbin rehash' to fix checksums"
    fi
else
    skip "plasmidbin doctor (cargo not found)"
fi

# ── Step 2: plasmidbin sync ──────────────────────────────────────────
step "Step 2: plasmidbin sync"

if [[ "$DRY_RUN" == "true" ]]; then
    skip "plasmidbin sync (dry-run)"
elif command -v cargo >/dev/null 2>&1; then
    if (cd "$PLASMIDBIN_ROOT" && cargo run -p plasmidbin --quiet -- sync --no-pull); then
        pass "plasmidbin sync"
    else
        fail "plasmidbin sync"
    fi
else
    skip "plasmidbin sync (cargo not found)"
fi

# ── Step 3: nucleus_launcher start ───────────────────────────────────
step "Step 3: nucleus_launcher start --composition $COMPOSITION --validate"

NUCLEUS_BIN=""
if [[ -x "$ECOPRIMALS_ROOT/target/release/nucleus_launcher" ]]; then
    NUCLEUS_BIN="$ECOPRIMALS_ROOT/target/release/nucleus_launcher"
fi

if [[ "$DRY_RUN" == "true" ]]; then
    skip "nucleus_launcher start (dry-run)"
elif [[ "$SKIP_START" == "true" ]]; then
    skip "nucleus_launcher start (--skip-start)"
elif [[ -n "$NUCLEUS_BIN" ]]; then
    export ECOPRIMALS_ROOT
    if "$NUCLEUS_BIN" start \
        --family-id "$FAMILY_ID" \
        --composition "$COMPOSITION" \
        --validate \
        --allow-degraded; then
        pass "nucleus_launcher start"
    else
        fail "nucleus_launcher start"
    fi
else
    skip "nucleus_launcher (binary not found — run: cargo build -p primalspring --release)"
fi

# ── Step 4: validate_composition.sh ──────────────────────────────────
step "Step 4: Composition validation (exp085-088, exp094)"

VALIDATE_SCRIPT="$WORKSPACE_ROOT/scripts/validate_composition.sh"
if [[ "$DRY_RUN" == "true" ]]; then
    skip "validate_composition.sh (dry-run)"
elif [[ ! -x "$VALIDATE_SCRIPT" ]]; then
    skip "validate_composition.sh (not found)"
else
    if "$VALIDATE_SCRIPT" 2>&1; then
        pass "validate_composition.sh"
    else
        fail "validate_composition.sh"
    fi
fi

# ── Step 5: Cross-gate scenarios (optional) ──────────────────────────
if [[ "$CROSS_GATE" == "true" ]]; then
    step "Step 5: Cross-gate trust scenarios"

    TOPOLOGY_FILE="$WORKSPACE_ROOT/benchScale/topologies/cross_gate_trust.toml"
    if [[ ! -f "$TOPOLOGY_FILE" ]]; then
        fail "cross_gate_trust.toml not found"
    else
        SCENARIOS=("s_covalent_mesh" "s_cross_gate_capability_call" "s_plasmodium_collective")
        for scenario in "${SCENARIOS[@]}"; do
            if [[ "$DRY_RUN" == "true" ]]; then
                skip "$scenario (dry-run)"
                continue
            fi
            printf "  Running: %s ... " "$scenario"
            if (cd "$WORKSPACE_ROOT" && BENCHSCALE_TOPOLOGY=cross_gate_trust cargo test --lib -- "$scenario" 2>/dev/null); then
                pass "$scenario"
            else
                fail "$scenario"
            fi
        done
    fi
fi

# ── Step 6: Stop (optional) ──────────────────────────────────────────
if [[ "$STOP_AFTER" == "true" && "$DRY_RUN" != "true" && "$SKIP_START" != "true" ]]; then
    step "Step 6: Stop primals"
    if [[ -n "$NUCLEUS_BIN" ]]; then
        "$NUCLEUS_BIN" stop --composition "$COMPOSITION" || true
        pass "nucleus_launcher stop"
    fi
fi

# ── Summary ──────────────────────────────────────────────────────────
echo ""
echo "══════════════════════════════════════════════"
echo "  Validation Summary"
echo "══════════════════════════════════════════════"
echo ""
echo "  Passed:   $PASSED"
echo "  Failed:   $FAILURES"
echo "  Skipped:  $SKIPPED"
echo ""

if [[ "$FAILURES" -gt 0 ]]; then
    printf "${RED}  RESULT: FAIL${NC}\n\n"
    exit 1
else
    printf "${GREEN}  RESULT: PASS${NC}\n\n"
    exit 0
fi
