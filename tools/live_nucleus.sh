#!/usr/bin/env bash
# tools/live_nucleus.sh — Start a minimal NUCLEUS + push demo scenes to petalTongue
#
# Launches the essential tower (beardog + songbird) plus petalTongue in live
# mode from plasmidBin binaries, then pushes a Grammar of Graphics demo scene
# via JSON-RPC to the petalTongue socket.
#
# Usage:
#   ./tools/live_nucleus.sh                      # full NUCLEUS + demo scene
#   ./tools/live_nucleus.sh --minimal            # beardog + songbird + petaltongue only
#   ./tools/live_nucleus.sh --scene-only         # skip NUCLEUS, push scene to existing petaltongue
#   ./tools/live_nucleus.sh --stop               # stop all launched primals
#
# Requires: ECOPRIMALS_PLASMID_BIN or auto-detect from repo layout

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SPRING_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

MINIMAL=false
SCENE_ONLY=false
STOP=false
SOCKET="${PETALTONGUE_SOCKET:-/tmp/petaltongue.sock}"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

log()  { echo -e "${CYAN}[live_nucleus]${NC} $*"; }
ok()   { echo -e "${GREEN}[live_nucleus]${NC} $*"; }
warn() { echo -e "${YELLOW}[live_nucleus]${NC} $*"; }
err()  { echo -e "${RED}[live_nucleus]${NC} $*" >&2; }

while [[ $# -gt 0 ]]; do
    case "$1" in
        --minimal)    MINIMAL=true; shift ;;
        --scene-only) SCENE_ONLY=true; shift ;;
        --stop)       STOP=true; shift ;;
        --socket)     SOCKET="$2"; shift 2 ;;
        *)            err "Unknown option: $1"; exit 1 ;;
    esac
done

find_plasmid_bin() {
    if [[ -n "${ECOPRIMALS_PLASMID_BIN:-}" ]] && [[ -d "$ECOPRIMALS_PLASMID_BIN" ]]; then
        echo "$ECOPRIMALS_PLASMID_BIN"
        return
    fi
    local candidates=(
        "$SPRING_ROOT/../../../infra/plasmidBin"
        "$SPRING_ROOT/../../infra/plasmidBin"
    )
    for c in "${candidates[@]}"; do
        if [[ -d "$c/primals" ]]; then
            echo "$(cd "$c" && pwd)"
            return
        fi
    done
    err "Cannot find plasmidBin. Set ECOPRIMALS_PLASMID_BIN."
    exit 1
}

if $STOP; then
    log "Stopping launched primals..."
    for p in petaltongue songbird beardog toadstool barracuda coralreef nestgate rhizocrypt loamspine sweetgrass squirrel; do
        pkill -f "primals/.*$p" 2>/dev/null && ok "stopped $p" || true
    done
    exit 0
fi

if ! $SCENE_ONLY; then
    PLASMID_BIN="$(find_plasmid_bin)"
    log "Using plasmidBin at: $PLASMID_BIN"
    source "$PLASMID_BIN/ports.env"

    export FAMILY_ID="${FAMILY_ID:-live-demo}"
    export BEARDOG_FAMILY_SEED="${BEARDOG_FAMILY_SEED:-$(head -c 32 /dev/urandom | xxd -p)}"
    export NODE_ID="${NODE_ID:-$(hostname)}"
    export BEARDOG_NODE_ID="${BEARDOG_NODE_ID:-$NODE_ID}"

    find_binary() {
        local name="$1"
        local bin
        bin=$(find "$PLASMID_BIN/primals" -name "$name" -type f -executable 2>/dev/null | head -1)
        if [[ -z "$bin" ]]; then
            bin="$PLASMID_BIN/primals/$name"
        fi
        echo "$bin"
    }

    start_primal() {
        local name="$1"; shift
        local bin
        bin="$(find_binary "$name")"
        if [[ ! -x "$bin" ]]; then
            warn "binary not found: $name (skipping)"
            return 1
        fi
        log "starting $name..."
        "$bin" "$@" &
        sleep 0.5
        ok "$name started (pid $!)"
    }

    start_primal beardog || { err "beardog required"; exit 1; }
    start_primal songbird || { err "songbird required"; exit 1; }

    if ! $MINIMAL; then
        start_primal toadstool || warn "toadstool unavailable"
        start_primal barracuda || warn "barracuda unavailable"
        start_primal nestgate || warn "nestgate unavailable"
    fi

    log "starting petaltongue in live mode..."
    PT_BIN="$(find_binary petaltongue)"
    if [[ -x "$PT_BIN" ]]; then
        export PETALTONGUE_MODE=live
        export PETALTONGUE_SOCKET="$SOCKET"
        export AWAKENING_ENABLED=false
        "$PT_BIN" --mode live &
        PT_PID=$!
        ok "petaltongue started in live mode (pid $PT_PID)"
    else
        err "petaltongue binary not found"
        exit 1
    fi

    log "waiting for petaltongue socket..."
    for i in $(seq 1 20); do
        if [[ -S "$SOCKET" ]]; then
            ok "petaltongue socket ready at $SOCKET"
            break
        fi
        sleep 0.5
    done
    if [[ ! -S "$SOCKET" ]]; then
        warn "petaltongue socket not found after 10s, pushing scene anyway"
    fi
fi

log "pushing demo scenes to petaltongue..."
"$SCRIPT_DIR/push_demo_scene.sh" --socket "$SOCKET"

ok "live NUCLEUS + demo scenes ready"
ok "petaltongue live window should be displaying IPC scenes"
log "press Ctrl+C to stop all, or run: $0 --stop"
wait
