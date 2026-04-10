#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# NUCLEUS Launcher — starts the ecoPrimals composition stack in dependency order.
#
# Phases:
#   0: biomeOS Neural API (orchestration substrate)
#   1: Tower Atomic — BearDog (crypto) + Songbird (discovery)
#   2: NestGate (persistence) + Squirrel (AI bridge)
#   3: petalTongue (visualization) + ludoSpring (game science)
#   4: esotericWebb (narrative engine — the composed product)
#
# Usage:
#   ./tools/nucleus_launcher.sh start     # launch full stack
#   ./tools/nucleus_launcher.sh stop      # graceful shutdown
#   ./tools/nucleus_launcher.sh status    # health check all primals
#   ./tools/nucleus_launcher.sh restart   # stop + start
#
# Environment:
#   NUCLEUS_BIN_DIR     — directory containing primal binaries (default: auto-detect)
#   NUCLEUS_FAMILY_ID   — family ID for socket naming (default: auto-detect)
#   NUCLEUS_LOG_LEVEL   — tracing log level (default: info)
#   OLLAMA_ENDPOINT     — Ollama URL for AI narration (default: http://localhost:11434)
#   BIOMEOS_GRAPHS_DIR  — biomeOS graphs directory (default: auto-detect)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
ECO_ROOT="$(dirname "$(dirname "$PROJECT_ROOT")")"

SOCKET_DIR="/run/user/$(id -u)/biomeos"
NUCLEUS_LOG_LEVEL="${NUCLEUS_LOG_LEVEL:-info}"
OLLAMA_ENDPOINT="${OLLAMA_ENDPOINT:-http://localhost:11434}"
PID_DIR="/tmp/nucleus-pids"

detect_bin_dir() {
    if [[ -n "${NUCLEUS_BIN_DIR:-}" ]]; then
        echo "$NUCLEUS_BIN_DIR"
        return
    fi
    local deploy_dir
    deploy_dir=$(ls -d /tmp/nucleus-deploy-*/bin 2>/dev/null | head -1)
    if [[ -n "$deploy_dir" ]]; then
        echo "$deploy_dir"
        return
    fi
    echo ""
}

detect_family_id() {
    if [[ -n "${NUCLEUS_FAMILY_ID:-}" ]]; then
        echo "$NUCLEUS_FAMILY_ID"
        return
    fi
    local existing
    existing=$(ls "$SOCKET_DIR"/neural-api-*.sock 2>/dev/null | head -1 | sed 's/.*neural-api-//;s/\.sock//')
    if [[ -n "$existing" ]]; then
        echo "$existing"
        return
    fi
    echo "nucleus01"
}

detect_graphs_dir() {
    if [[ -n "${BIOMEOS_GRAPHS_DIR:-}" ]]; then
        echo "$BIOMEOS_GRAPHS_DIR"
        return
    fi
    local biome_graphs="$ECO_ROOT/primals/biomeOS/graphs"
    if [[ -d "$biome_graphs" ]]; then
        echo "$biome_graphs"
        return
    fi
    echo ""
}

BIN_DIR="$(detect_bin_dir)"
FAMILY_ID="$(detect_family_id)"
GRAPHS_DIR="$(detect_graphs_dir)"

log() { echo "[nucleus] $(date +%H:%M:%S) $*"; }
err() { echo "[nucleus] ERROR: $*" >&2; }

wait_for_socket() {
    local sock="$1" timeout="${2:-10}" elapsed=0
    while [[ $elapsed -lt $timeout ]]; do
        if [[ -S "$sock" ]] || ss -xlp 2>/dev/null | grep -q "$sock"; then
            return 0
        fi
        sleep 0.5
        elapsed=$((elapsed + 1))
    done
    return 1
}

wait_for_abstract() {
    local name="$1" timeout="${2:-10}" elapsed=0
    while [[ $elapsed -lt $timeout ]]; do
        if ss -xlp 2>/dev/null | grep -q "@${name}"; then
            return 0
        fi
        sleep 0.5
        elapsed=$((elapsed + 1))
    done
    return 1
}

health_check() {
    local sock="$1" method="${2:-health.liveness}"
    echo "{\"jsonrpc\":\"2.0\",\"method\":\"$method\",\"id\":1}" | \
        timeout 3 socat - "UNIX-CONNECT:$sock" 2>/dev/null
}

save_pid() {
    local name="$1" pid="$2"
    mkdir -p "$PID_DIR"
    echo "$pid" > "$PID_DIR/$name.pid"
}

read_pid() {
    local name="$1"
    local pidfile="$PID_DIR/$name.pid"
    if [[ -f "$pidfile" ]]; then
        cat "$pidfile"
    fi
}

find_binary() {
    local name="$1"
    if [[ -n "$BIN_DIR" && -x "$BIN_DIR/$name" ]]; then
        echo "$BIN_DIR/$name"
        return
    fi
    local target="$ECO_ROOT/primals/$name/target/release/$name"
    if [[ -x "$target" ]]; then
        echo "$target"
        return
    fi
    local garden_target="$ECO_ROOT/gardens/$2/target/release/$name"
    if [[ -x "$garden_target" ]]; then
        echo "$garden_target"
        return
    fi
    which "$name" 2>/dev/null || true
}

start_primal() {
    local name="$1" binary="$2"
    shift 2
    local args=("$@")

    if pgrep -f "$name.*(server|serve|web)" >/dev/null 2>&1; then
        log "$name already running, skipping"
        return 0
    fi

    local logfile="/tmp/nucleus-${name}.log"
    log "starting $name..."
    nohup "$binary" "${args[@]}" > "$logfile" 2>&1 &
    local pid=$!
    save_pid "$name" "$pid"
    sleep 1

    if ! kill -0 "$pid" 2>/dev/null; then
        err "$name failed to start. Check $logfile"
        return 1
    fi
    log "$name started (pid=$pid)"
    return 0
}

cmd_start() {
    log "Starting NUCLEUS composition stack"
    log "  bin_dir:   ${BIN_DIR:-<release targets>}"
    log "  family_id: $FAMILY_ID"
    log "  socket_dir: $SOCKET_DIR"
    mkdir -p "$SOCKET_DIR"

    # Phase 0: biomeOS Neural API
    local biomeos_bin
    biomeos_bin="$(find_binary biomeos biomeOS)"
    if [[ -z "$biomeos_bin" ]]; then
        err "biomeos binary not found"
        return 1
    fi
    local neural_sock="$SOCKET_DIR/neural-api-${FAMILY_ID}.sock"
    if ! pgrep -f "biomeos.*neural-api.*${FAMILY_ID}" >/dev/null 2>&1; then
        local biomeos_args=(neural-api --socket "$neural_sock" --family-id "$FAMILY_ID" --log-level "$NUCLEUS_LOG_LEVEL")
        [[ -n "$GRAPHS_DIR" ]] && biomeos_args+=(--graphs-dir "$GRAPHS_DIR")
        nohup "$biomeos_bin" "${biomeos_args[@]}" > /tmp/nucleus-biomeos.log 2>&1 &
        save_pid biomeos $!
        log "biomeOS Neural API starting (pid=$!)"
        wait_for_socket "$neural_sock" 10 || { err "biomeOS socket timeout"; return 1; }
    else
        log "biomeOS Neural API already running"
    fi

    # Phase 1: Tower — BearDog + Songbird
    local beardog_bin songbird_bin
    beardog_bin="$(find_binary beardog beardog)"
    songbird_bin="$(find_binary songbird songbird)"
    local beardog_sock="$SOCKET_DIR/beardog-${FAMILY_ID}.sock"
    local songbird_sock="$SOCKET_DIR/songbird-${FAMILY_ID}.sock"

    if [[ -n "$beardog_bin" ]]; then
        start_primal beardog "$beardog_bin" server --socket "$beardog_sock" --family-id "$FAMILY_ID" || true
        wait_for_socket "$beardog_sock" 8 || log "WARN: beardog socket not ready"
    else
        log "WARN: beardog binary not found, skipping"
    fi

    if [[ -n "$songbird_bin" ]]; then
        start_primal songbird "$songbird_bin" server --socket "$songbird_sock" --beardog-socket "$beardog_sock" --port 9200 || true
        wait_for_socket "$songbird_sock" 8 || log "WARN: songbird socket not ready"
    else
        log "WARN: songbird binary not found, skipping"
    fi

    # Phase 2: NestGate + Squirrel
    local nestgate_bin squirrel_bin
    nestgate_bin="$(find_binary nestgate nestgate)"
    squirrel_bin="$(find_binary squirrel squirrel)"
    local nestgate_sock="$SOCKET_DIR/nestgate-${FAMILY_ID}.sock"

    if [[ -n "$nestgate_bin" ]]; then
        start_primal nestgate "$nestgate_bin" server --socket "$nestgate_sock" --family-id "$FAMILY_ID" || true
        wait_for_socket "$nestgate_sock" 8 || log "WARN: nestgate socket not ready"
    else
        log "WARN: nestgate binary not found, skipping"
    fi

    if [[ -n "$squirrel_bin" ]]; then
        if ! pgrep -f "squirrel.*server" >/dev/null 2>&1; then
            log "starting squirrel (with Ollama at $OLLAMA_ENDPOINT)..."
            nohup env \
                LOCAL_AI_ENDPOINT="$OLLAMA_ENDPOINT" \
                OLLAMA_ENDPOINT="$OLLAMA_ENDPOINT" \
                MCP_DEFAULT_MODEL="${MCP_DEFAULT_MODEL:-llama3.2:3b}" \
                "$squirrel_bin" server > /tmp/nucleus-squirrel.log 2>&1 &
            save_pid squirrel $!
            log "squirrel started (pid=$!)"
            wait_for_abstract squirrel 8 || log "WARN: squirrel socket not ready"
        else
            log "squirrel already running"
        fi
    else
        log "WARN: squirrel binary not found, skipping"
    fi

    # Phase 3: petalTongue + ludoSpring
    local petaltongue_bin ludospring_bin
    petaltongue_bin="$(find_binary petaltongue petalTongue)"
    [[ -z "$petaltongue_bin" ]] && petaltongue_bin="$ECO_ROOT/primals/petalTongue/target/release/petaltongue"
    ludospring_bin="$(find_binary ludospring ludoSpring)"
    [[ -z "$ludospring_bin" ]] && ludospring_bin="$ECO_ROOT/springs/ludoSpring/target/release/ludospring"

    if [[ -x "$petaltongue_bin" ]]; then
        # Start web mode for HTTP dashboard
        start_primal petaltongue "$petaltongue_bin" web || true
        # Start server mode for IPC (proprioception)
        local pt_ipc_sock="$SOCKET_DIR/petaltongue-ipc.sock"
        if ! [[ -S "$pt_ipc_sock" ]]; then
            log "starting petaltongue IPC server..."
            nohup env PETALTONGUE_SOCKET="$pt_ipc_sock" "$petaltongue_bin" server > /tmp/nucleus-petaltongue-ipc.log 2>&1 &
            save_pid petaltongue-ipc $!
            log "petaltongue IPC started (pid=$!)"
            wait_for_socket "$pt_ipc_sock" 8 || log "WARN: petaltongue IPC socket not ready"
        else
            log "petaltongue IPC already running"
        fi
    else
        log "WARN: petaltongue binary not found, skipping"
    fi

    if [[ -x "$ludospring_bin" ]]; then
        start_primal ludospring "$ludospring_bin" server || true
    else
        log "WARN: ludospring binary not found, skipping"
    fi

    # Phase 4: esotericWebb
    local esotericwebb_bin
    esotericwebb_bin="$(find_binary esotericwebb esotericWebb)"
    [[ -z "$esotericwebb_bin" ]] && esotericwebb_bin="$ECO_ROOT/gardens/esotericWebb/target/release/esotericwebb"

    if [[ -x "$esotericwebb_bin" ]]; then
        start_primal esotericwebb "$esotericwebb_bin" serve || true
    else
        log "WARN: esotericwebb binary not found, skipping"
    fi

    log "NUCLEUS stack launch complete"
    echo ""
    cmd_status
}

cmd_stop() {
    log "Stopping NUCLEUS stack..."
    local primals=(esotericwebb ludospring petaltongue squirrel nestgate songbird beardog biomeos)
    for name in "${primals[@]}"; do
        local pid
        pid="$(read_pid "$name")"
        if [[ -n "$pid" ]] && kill -0 "$pid" 2>/dev/null; then
            log "stopping $name (pid=$pid)"
            kill "$pid" 2>/dev/null || true
        else
            pkill -f "$name.*(server|serve|web|neural-api)" 2>/dev/null || true
        fi
    done
    sleep 2
    log "NUCLEUS stack stopped"
}

cmd_status() {
    log "NUCLEUS Stack Status"
    echo "──────────────────────────────────────────────"
    printf "%-16s %-8s %-50s\n" "PRIMAL" "STATUS" "SOCKET"
    echo "──────────────────────────────────────────────"

    local checks=(
        "biomeOS|neural-api-${FAMILY_ID}|graph.list"
        "BearDog|beardog-${FAMILY_ID}|health.liveness"
        "Songbird|songbird-${FAMILY_ID}|health.liveness"
        "NestGate|nestgate-${FAMILY_ID}|health.liveness"
        "ludoSpring|ludospring-${FAMILY_ID}|health.check"
        "esotericWebb|esotericwebb-${FAMILY_ID}|webb.liveness"
    )

    for entry in "${checks[@]}"; do
        IFS='|' read -r display sock method <<< "$entry"
        local full="$SOCKET_DIR/${sock}.sock"
        if [[ -S "$full" ]]; then
            local resp
            resp=$(health_check "$full" "$method" 2>/dev/null || true)
            if [[ -n "$resp" ]] && echo "$resp" | grep -q '"result"'; then
                printf "%-16s \033[32m%-8s\033[0m %s\n" "$display" "ALIVE" "$full"
            else
                printf "%-16s \033[33m%-8s\033[0m %s\n" "$display" "ERROR" "$full"
            fi
        else
            printf "%-16s \033[31m%-8s\033[0m %s\n" "$display" "DOWN" "(no socket)"
        fi
    done

    # Squirrel uses abstract socket
    if ss -xlp 2>/dev/null | grep -q "@squirrel"; then
        printf "%-16s \033[32m%-8s\033[0m %s\n" "Squirrel" "ALIVE" "@squirrel (abstract)"
    else
        printf "%-16s \033[31m%-8s\033[0m %s\n" "Squirrel" "DOWN" "(no socket)"
    fi

    # petalTongue check via process
    if pgrep -f "petaltongue.*web" >/dev/null 2>&1; then
        printf "%-16s \033[32m%-8s\033[0m %s\n" "petalTongue" "ALIVE" "http://localhost:3000"
    else
        printf "%-16s \033[31m%-8s\033[0m %s\n" "petalTongue" "DOWN" "(not running)"
    fi

    echo "──────────────────────────────────────────────"

    # Ollama check
    if curl -sf http://localhost:11434/api/tags >/dev/null 2>&1; then
        printf "%-16s \033[32m%-8s\033[0m %s\n" "Ollama" "ALIVE" "$OLLAMA_ENDPOINT"
    else
        printf "%-16s \033[31m%-8s\033[0m %s\n" "Ollama" "DOWN" "$OLLAMA_ENDPOINT"
    fi
    echo ""
}

cmd_restart() {
    cmd_stop
    sleep 2
    cmd_start
}

case "${1:-status}" in
    start)   cmd_start   ;;
    stop)    cmd_stop    ;;
    status)  cmd_status  ;;
    restart) cmd_restart ;;
    *)       echo "Usage: $0 {start|stop|status|restart}"; exit 1 ;;
esac
