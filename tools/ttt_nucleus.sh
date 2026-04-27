#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# ttt_nucleus.sh — Launch a full NUCLEUS composition for Tic-Tac-Toe
#
# Starts primals from plasmidBin in dependency order with correct socket
# naming for discover_by_capability ({capability}-{FAMILY_ID}.sock).
#
# Usage:
#   ./tools/ttt_nucleus.sh start    # launch NUCLEUS + petalTongue live
#   ./tools/ttt_nucleus.sh stop     # graceful shutdown
#   ./tools/ttt_nucleus.sh status   # health check all primals
#
# After start, run:
#   ./tools/ttt_composition.sh      # start the game loop

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
ECO_ROOT="$(cd "$PROJECT_ROOT/../.." && pwd)"

FAMILY_ID="${FAMILY_ID:-ttt}"
SOCKET_DIR="${XDG_RUNTIME_DIR:-/tmp}/biomeos"
PID_DIR="/tmp/nucleus-ttt-pids"
PLASMID_BIN="${ECOPRIMALS_PLASMID_BIN:-$ECO_ROOT/infra/plasmidBin}"
BIN_DIR="$PLASMID_BIN/primals"

export FAMILY_ID
export BEARDOG_FAMILY_SEED="${BEARDOG_FAMILY_SEED:-$(head -c 32 /dev/urandom | xxd -p | tr -d '\n')}"
export NODE_ID="${NODE_ID:-$(hostname)}"
export BEARDOG_NODE_ID="${BEARDOG_NODE_ID:-$NODE_ID}"

log() { echo "[ttt-nucleus] $(date +%H:%M:%S) $*"; }
err() { echo "[ttt-nucleus] ERROR: $*" >&2; }
ok()  { echo "[ttt-nucleus] OK: $*"; }

wait_for_socket() {
    local sock="$1" timeout="${2:-10}" elapsed=0
    while [[ $elapsed -lt $timeout ]]; do
        [[ -S "$sock" ]] && return 0
        sleep 0.5
        elapsed=$((elapsed + 1))
    done
    return 1
}

health_check() {
    local sock="$1" method="${2:-health.liveness}"
    local payload="{\"jsonrpc\":\"2.0\",\"method\":\"$method\",\"id\":1}"
    if command -v socat &>/dev/null; then
        echo "$payload" | timeout 3 socat - "UNIX-CONNECT:$sock" 2>/dev/null
    elif command -v python3 &>/dev/null; then
        python3 -c "
import socket,sys
s=socket.socket(socket.AF_UNIX,socket.SOCK_STREAM);s.settimeout(3)
try:
    s.connect('$sock');s.sendall(b'$payload\n');d=s.recv(65536);sys.stdout.buffer.write(d)
except: pass
finally: s.close()
" 2>/dev/null
    elif command -v nc &>/dev/null; then
        echo "$payload" | timeout 3 nc -U "$sock" 2>/dev/null
    fi
}

save_pid() {
    mkdir -p "$PID_DIR"
    echo "$2" > "$PID_DIR/$1.pid"
}

find_binary() {
    local name="$1"
    if [[ -x "$BIN_DIR/$name" ]]; then
        echo "$BIN_DIR/$name"
        return
    fi
    local release="$ECO_ROOT/primals/$name/target/release/$name"
    [[ -x "$release" ]] && echo "$release" && return
    which "$name" 2>/dev/null || true
}

start_primal() {
    local name="$1" binary="$2"; shift 2
    local logfile="/tmp/nucleus-ttt-${name}.log"
    log "starting $name..."
    setsid "$binary" "$@" > "$logfile" 2>&1 &
    local pid=$!
    disown "$pid" 2>/dev/null || true
    save_pid "$name" "$pid"
    sleep 1
    if ! kill -0 "$pid" 2>/dev/null; then
        err "$name failed to start. Check $logfile"
        return 1
    fi
    log "$name started (pid=$pid)"
}

# Socket path helpers
sock() { echo "$SOCKET_DIR/$1-${FAMILY_ID}.sock"; }

cmd_start() {
    log "============================================"
    log "  NUCLEUS Tic-Tac-Toe Composition"
    log "  family_id:  $FAMILY_ID"
    log "  socket_dir: $SOCKET_DIR"
    log "  bin_dir:    $BIN_DIR"
    log "============================================"
    mkdir -p "$SOCKET_DIR"

    echo "$BEARDOG_FAMILY_SEED" > "$SOCKET_DIR/.family.seed"
    chmod 600 "$SOCKET_DIR/.family.seed"

    # ── Phase 1: Tower Atomic (BearDog + Songbird) ──
    log "── Phase 1: Tower Atomic ──"
    local beardog_bin songbird_bin
    beardog_bin="$(find_binary beardog)"
    songbird_bin="$(find_binary songbird)"

    if [[ -n "$beardog_bin" ]]; then
        start_primal beardog "$beardog_bin" server \
            --socket "$(sock beardog)" \
            --family-id "$FAMILY_ID" || { err "beardog required"; return 1; }
        wait_for_socket "$(sock beardog)" 10 || err "beardog socket timeout"
    else
        err "beardog binary not found"; return 1
    fi

    if [[ -n "$songbird_bin" ]]; then
        SONGBIRD_SECURITY_PROVIDER="$(sock beardog)" \
        SONGBIRD_DISCOVERY_MODE="disabled" \
        BTSP_PROVIDER_SOCKET="$(sock beardog)" \
            start_primal songbird "$songbird_bin" server \
                --socket "$(sock songbird)" \
                --beardog-socket "$(sock beardog)" || { err "songbird required"; return 1; }
        wait_for_socket "$(sock songbird)" 10 || err "songbird socket timeout"
    else
        err "songbird binary not found"; return 1
    fi

    # ── Phase 2: Compute (ToadStool + barraCuda) ──
    log "── Phase 2: Compute Services ──"
    local toadstool_bin barracuda_bin
    toadstool_bin="$(find_binary toadstool)"
    barracuda_bin="$(find_binary barracuda)"

    if [[ -n "$toadstool_bin" ]]; then
        TOADSTOOL_SOCKET="$(sock toadstool)" \
        TOADSTOOL_FAMILY_ID="$FAMILY_ID" \
        TOADSTOOL_SECURITY_WARNING_ACKNOWLEDGED="1" \
        NESTGATE_SOCKET="$(sock nestgate)" \
            start_primal toadstool "$toadstool_bin" server || log "WARN: toadstool failed"
        wait_for_socket "$(sock toadstool)" 8 || log "WARN: toadstool socket not ready"
    else
        log "WARN: toadstool binary not found"
    fi

    if [[ -n "$barracuda_bin" ]]; then
        BARRACUDA_FAMILY_ID="$FAMILY_ID" \
        BEARDOG_SOCKET="$(sock beardog)" \
        SONGBIRD_SOCKET="$(sock songbird)" \
            start_primal barracuda "$barracuda_bin" server || log "WARN: barracuda failed"
        wait_for_socket "$SOCKET_DIR/barracuda-${FAMILY_ID}.sock" 8 || \
            wait_for_socket "$SOCKET_DIR/math-${FAMILY_ID}.sock" 5 || \
            log "WARN: barracuda socket not ready"
        if [[ ! -e "$SOCKET_DIR/barracuda-${FAMILY_ID}.sock" && -S "$SOCKET_DIR/math-${FAMILY_ID}.sock" ]]; then
            ln -sf "math-${FAMILY_ID}.sock" "$SOCKET_DIR/barracuda-${FAMILY_ID}.sock" 2>/dev/null || true
        fi
    else
        log "WARN: barracuda binary not found"
    fi

    # ── Phase 3: Provenance Trio ──
    log "── Phase 3: Provenance Trio ──"
    local rhizocrypt_bin loamspine_bin sweetgrass_bin
    rhizocrypt_bin="$(find_binary rhizocrypt)"
    loamspine_bin="$(find_binary loamspine)"
    sweetgrass_bin="$(find_binary sweetgrass)"

    if [[ -n "$rhizocrypt_bin" ]]; then
        RHIZOCRYPT_SOCKET="$(sock rhizocrypt)" \
        BIOMEOS_SOCKET_DIR="$SOCKET_DIR" \
        BEARDOG_SOCKET="$(sock beardog)" \
        BTSP_PROVIDER_SOCKET="$(sock beardog)" \
            start_primal rhizocrypt "$rhizocrypt_bin" server || log "WARN: rhizocrypt failed"
        wait_for_socket "$(sock rhizocrypt)" 12 || \
            wait_for_socket "$SOCKET_DIR/rhizocrypt.sock" 4 || \
            log "WARN: rhizocrypt socket not ready"
    else
        log "WARN: rhizocrypt binary not found"
    fi

    if [[ -n "$loamspine_bin" ]]; then
        LOAMSPINE_SOCKET="$(sock loamspine)" \
        BIOMEOS_SOCKET_DIR="$SOCKET_DIR" \
        BEARDOG_SOCKET="$(sock beardog)" \
        RHIZOCRYPT_SOCKET="$(sock rhizocrypt)" \
        BTSP_PROVIDER_SOCKET="$(sock beardog)" \
        BIOMEOS_FAMILY_ID="$FAMILY_ID" \
            start_primal loamspine "$loamspine_bin" server || log "WARN: loamspine failed"
        wait_for_socket "$(sock loamspine)" 8 || log "WARN: loamspine socket not ready"
    else
        log "WARN: loamspine binary not found"
    fi

    if [[ -n "$sweetgrass_bin" ]]; then
        SWEETGRASS_SOCKET="$(sock sweetgrass)" \
        BIOMEOS_SOCKET_DIR="$SOCKET_DIR" \
        BEARDOG_SOCKET="$(sock beardog)" \
        BTSP_PROVIDER_SOCKET="$(sock beardog)" \
            start_primal sweetgrass "$sweetgrass_bin" server || log "WARN: sweetgrass failed"
        wait_for_socket "$(sock sweetgrass)" 8 || log "WARN: sweetgrass socket not ready"
    else
        log "WARN: sweetgrass binary not found"
    fi

    # ── Phase 4: petalTongue (LIVE mode) ──
    # petalTongue live must NOT use setsid — winit requires the main thread
    # for the event loop. We background it directly instead.
    log "── Phase 4: petalTongue (live UI) ──"
    local petaltongue_bin
    # Prefer local build: plasmidBin musl binary has winit threading issues
    petaltongue_bin="$ECO_ROOT/primals/petalTongue/target/release/petaltongue"
    [[ -x "$petaltongue_bin" ]] || petaltongue_bin="$(find_binary petaltongue)"

    if [[ -x "$petaltongue_bin" ]]; then
        local pt_logfile="/tmp/nucleus-ttt-petaltongue.log"
        log "starting petaltongue (live, no setsid)..."
        DISPLAY="${DISPLAY:-:1}" \
        PETALTONGUE_SOCKET="$(sock petaltongue)" \
        FAMILY_ID="$FAMILY_ID" \
        BEARDOG_FAMILY_SEED="$BEARDOG_FAMILY_SEED" \
        AWAKENING_ENABLED=false \
            "$petaltongue_bin" live --socket "$(sock petaltongue)" > "$pt_logfile" 2>&1 &
        local pt_pid=$!
        save_pid petaltongue "$pt_pid"
        sleep 2
        if ! kill -0 "$pt_pid" 2>/dev/null; then
            err "petaltongue failed to start. Check $pt_logfile"
            return 1
        fi
        log "petaltongue started (pid=$pt_pid)"
        wait_for_socket "$(sock petaltongue)" 10 || err "petaltongue socket timeout"
    else
        err "petaltongue binary not found"; return 1
    fi

    # ── Capability domain symlinks ──
    log "── Creating capability aliases ──"
    local -A domain_map=(
        [security]="beardog-${FAMILY_ID}.sock"
        [crypto]="beardog-${FAMILY_ID}.sock"
        [discovery]="songbird-${FAMILY_ID}.sock"
        [compute]="toadstool-${FAMILY_ID}.sock"
        [tensor]="barracuda-${FAMILY_ID}.sock"
        [provenance]="rhizocrypt-${FAMILY_ID}.sock"
        [dag]="rhizocrypt-${FAMILY_ID}.sock"
        [ledger]="loamspine-${FAMILY_ID}.sock"
        [attribution]="sweetgrass-${FAMILY_ID}.sock"
        [visualization]="petaltongue-${FAMILY_ID}.sock"
    )
    for domain in "${!domain_map[@]}"; do
        local target="${domain_map[$domain]}"
        local alias_path="$SOCKET_DIR/${domain}-${FAMILY_ID}.sock"
        if [[ -S "$SOCKET_DIR/$target" ]] && [[ ! -e "$alias_path" ]]; then
            ln -sf "$target" "$alias_path" 2>/dev/null && \
                log "  ${domain}-${FAMILY_ID}.sock -> $target" || true
        fi
    done

    # ── Health summary ──
    log "── NUCLEUS Health Check ──"
    local healthy=0 total=0
    for primal in beardog songbird toadstool barracuda rhizocrypt loamspine sweetgrass petaltongue; do
        total=$((total + 1))
        local s="$(sock "$primal")"
        if [[ -S "$s" ]]; then
            local resp
            resp=$(health_check "$s" 2>/dev/null || true)
            if echo "$resp" | grep -q '"alive"\|"ok"\|"healthy"'; then
                ok "$primal: healthy"
                healthy=$((healthy + 1))
            elif [[ -n "$resp" ]]; then
                log "$primal: responding (non-standard) — $resp"
                healthy=$((healthy + 1))
            else
                log "WARN: $primal socket exists but no health response"
            fi
        else
            log "WARN: $primal socket not found at $s"
        fi
    done
    log "── Result: $healthy/$total primals healthy ──"
    ok "NUCLEUS ready. Run: ./tools/ttt_composition.sh"
}

cmd_stop() {
    log "Stopping NUCLEUS Tic-Tac-Toe..."
    for name in petaltongue sweetgrass loamspine rhizocrypt barracuda toadstool songbird beardog; do
        local pidfile="$PID_DIR/$name.pid"
        if [[ -f "$pidfile" ]]; then
            local pid
            pid=$(cat "$pidfile")
            if kill -0 "$pid" 2>/dev/null; then
                kill "$pid" 2>/dev/null && log "stopped $name (pid=$pid)" || true
            fi
            rm -f "$pidfile"
        fi
    done
    rm -f "$SOCKET_DIR"/*-${FAMILY_ID}.sock 2>/dev/null || true
    ok "NUCLEUS stopped"
}

cmd_status() {
    log "── NUCLEUS Status (family=$FAMILY_ID) ──"
    for primal in beardog songbird toadstool barracuda rhizocrypt loamspine sweetgrass petaltongue; do
        local s="$(sock "$primal")"
        if [[ -S "$s" ]]; then
            local resp
            resp=$(health_check "$s" 2>/dev/null || true)
            if [[ -n "$resp" ]]; then
                ok "$primal: $(echo "$resp" | head -c 120)"
            else
                log "$primal: socket exists, no response"
            fi
        else
            log "$primal: not running"
        fi
    done
}

case "${1:-start}" in
    start)  cmd_start ;;
    stop)   cmd_stop ;;
    status) cmd_status ;;
    restart) cmd_stop; sleep 2; cmd_start ;;
    *) err "Usage: $0 {start|stop|status|restart}"; exit 1 ;;
esac
