#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# primalSpring Temporal Cascade — eastGate → gates
#
# Orchestrates the temporal cascade from eastGate to NUCLEUS gates:
#   1. Pulls from golgiBody (Forgejo) to local
#   2. Verifies local build health
#   3. Triggers sync_diverge graph via Neural API to detect drift
#   4. Pushes updated state to gates via rsync/ssh
#
# primalSpring owns this mechanism per Wave 157a blurb.
#
# Usage:
#   ./scripts/temporal_cascade.sh [--dry-run] [--gates GATE1,GATE2,...]
#
# Environment:
#   NEURAL_API_SOCKET  — Neural API socket (default: /run/user/1000/biomeos/biomeos-neural.sock)
#   CASCADE_GATES      — Comma-separated gate list (default: all NUCLEUS gates)
#   PRIMALSPRING_ROOT  — Workspace root (auto-detected)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
NEURAL_SOCKET="${NEURAL_API_SOCKET:-/run/user/1000/biomeos/biomeos-neural.sock}"
DRY_RUN=false
GATES="${CASCADE_GATES:-sporeGate,blueGate,southGate,ironGate,strandGate,westGate}"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

info()  { echo -e "${CYAN}[cascade]${NC} $1"; }
pass()  { echo -e "${GREEN}  ✓${NC} $1"; }
fail()  { echo -e "${RED}  ✗${NC} $1"; }
warn()  { echo -e "${YELLOW}  !${NC} $1"; }

while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run) DRY_RUN=true; shift ;;
        --gates)  GATES="$2"; shift 2 ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

echo ""
echo "═══════════════════════════════════════════════════════"
echo "  primalSpring Temporal Cascade — eastGate"
echo "  $(date '+%Y-%m-%d %H:%M:%S')"
echo "═══════════════════════════════════════════════════════"
echo ""

if $DRY_RUN; then
    warn "DRY RUN — no mutations will be performed"
    echo ""
fi

# ── Phase 1: Cascade from golgiBody ─────────────────────────────────
info "Phase 1: Cascade from golgiBody (git.primals.eco)"

cd "$PROJECT_DIR"
BEFORE=$(git rev-parse HEAD)
git fetch origin main --quiet 2>/dev/null
LOCAL=$(git rev-parse HEAD)
REMOTE=$(git rev-parse origin/main)

if [[ "$LOCAL" == "$REMOTE" ]]; then
    pass "Already up to date ($LOCAL)"
    DIVERGED=false
else
    if $DRY_RUN; then
        warn "Would pull: $LOCAL → $REMOTE"
    else
        git pull origin main --quiet 2>/dev/null
    fi
    pass "Pulled: $(git log --oneline -1)"
    DIVERGED=true
fi

# ── Phase 2: Build health ────────────────────────────────────────────
info "Phase 2: Build health check"

if cargo check --workspace --quiet 2>/dev/null; then
    pass "Workspace compiles cleanly"
else
    fail "Build failed — cascade ABORTED"
    exit 1
fi

# ── Phase 3: Neural API sync check ──────────────────────────────────
info "Phase 3: Neural API sync divergence check"

neural_rpc() {
    local method=$1
    local params=${2:-'{}'}
    local msg="{\"jsonrpc\":\"2.0\",\"method\":\"${method}\",\"params\":${params},\"id\":1}"
    printf '\xec\x00%s\n' "$msg" | socat -t5 - UNIX-CONNECT:"$NEURAL_SOCKET" 2>/dev/null
}

if [[ -S "$NEURAL_SOCKET" ]]; then
    HEALTH=$(neural_rpc "health.check")
    if echo "$HEALTH" | grep -q '"alive"'; then
        pass "Neural API alive"
        
        # Try to execute sync_diverge graph
        EXEC=$(neural_rpc "graph.execute" '{"graph_id":"sync_diverge"}')
        if echo "$EXEC" | grep -q '"execution_id"'; then
            EID=$(echo "$EXEC" | python3 -c "import sys,json; print(json.load(sys.stdin).get('result',{}).get('execution_id','?'))" 2>/dev/null)
            pass "sync_diverge initiated: $EID"
            sleep 1
            STATUS=$(neural_rpc "graph.status" "{\"execution_id\":\"$EID\"}")
            STATE=$(echo "$STATUS" | python3 -c "import sys,json; print(json.load(sys.stdin).get('result',{}).get('state','?'))" 2>/dev/null)
            info "  graph state: $STATE"
        else
            warn "sync_diverge not executable (graph not in runtime_graphs)"
        fi
    else
        warn "Neural API not responding — sync check skipped"
    fi
else
    warn "Neural API socket not found — sync check skipped"
fi

# ── Phase 4: Gate cascade ────────────────────────────────────────────
info "Phase 4: Gate cascade status"

IFS=',' read -ra GATE_LIST <<< "$GATES"
for gate in "${GATE_LIST[@]}"; do
    gate_lower=$(echo "$gate" | tr '[:upper:]' '[:lower:]')
    if ssh -o ConnectTimeout=3 -o BatchMode=yes "$gate_lower" "echo ok" 2>/dev/null; then
        pass "$gate: reachable"
    else
        warn "$gate: unreachable (SSH)"
    fi
done

# ── Summary ──────────────────────────────────────────────────────────
echo ""
echo "═══════════════════════════════════════════════════════"
AFTER=$(git rev-parse HEAD)
if [[ "$BEFORE" != "$AFTER" ]]; then
    COMMITS=$(git log --oneline "$BEFORE..$AFTER" | wc -l)
    echo "  Cascaded: $COMMITS new commits"
else
    echo "  No changes to cascade"
fi
echo "  Gates: ${#GATE_LIST[@]} configured"
echo "  Neural API: $(test -S "$NEURAL_SOCKET" && echo 'active' || echo 'offline')"
echo "═══════════════════════════════════════════════════════"
echo ""
